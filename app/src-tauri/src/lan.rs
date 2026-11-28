//! Spike 0: devices find each other with mDNS (`_biovault._tcp.local.`) and exchange one
//! message over a Noise XX session on TCP (docs/05-sync-spec.md § Discovery & pairing).
//! No Tauri in here: in P2 this moves into biovault-core's `sync` module.
//! Interface-agnostic on purpose: the listener takes every IPv4 interface and mdns-sd
//! advertises on each one, so Ethernet ↔ Wi-Fi and Wi-Fi ↔ Wi-Fi take the same path.

use std::{
    collections::HashSet,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    time::{Duration, Instant},
};

use mdns_sd::{ResolvedService, ServiceDaemon, ServiceEvent, ServiceInfo};
use snow::{Builder, TransportState};

const SERVICE: &str = "_biovault._tcp.local.";
const NOISE: &str = "Noise_XX_25519_ChaChaPoly_BLAKE2s";
const FIND_FOR: Duration = Duration::from_secs(10);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
const IO_TIMEOUT: Duration = Duration::from_secs(10);
/// Android reports no error while the local-network permission is denied: the network just looks empty.
/// shortcut: a hint instead of checking the permission; F-21 checks it and links to the setting.
const NOT_FOUND: &str = if cfg!(target_os = "android") {
    "no other BioVault found within 10 s. Is \"Nearby devices\" allowed for BioVault (Settings → Apps → BioVault → Permissions)?"
} else {
    "no other BioVault found on the network within 10 s"
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone)]
pub struct Lan {
    mdns: ServiceDaemon,
    private: Vec<u8>,
    /// First 4 bytes of this device's public key, in hex. The peer shows the same id.
    pub id: String,
    pub name: String,
    pub port: u16,
}

impl Lan {
    /// Listens on a free TCP port, advertises it with mDNS and answers each incoming message
    /// on a background thread; `log` gets one line per incoming connection.
    /// shortcut: a new key per launch and every peer accepted; P2 pairs and pins keys (F-21).
    pub fn start(log: impl Fn(String) + Send + 'static) -> Result<Self> {
        let keys = Builder::new(NOISE.parse()?).generate_keypair()?;
        let id = hex(&keys.public[..4]);
        let name = format!("BioVault {} {id}", std::env::consts::OS);
        // shortcut: IPv4 only; IPv6 link-local addresses need scope ids.
        let listener = TcpListener::bind(("0.0.0.0", 0))?;
        let port = listener.local_addr()?.port();

        let mdns = ServiceDaemon::new()?;
        let host = format!("biovault-{id}.local.");
        let properties = [("id", id.as_str())];
        let info = ServiceInfo::new(SERVICE, &name, &host, "", port, &properties[..])?;
        mdns.register(info.enable_addr_auto())?;

        let (private, me) = (keys.private.clone(), name.clone());
        std::thread::spawn(move || {
            for stream in listener.incoming() {
                let line = match stream
                    .map_err(Into::into)
                    .and_then(|s| answer(s, &private, &me))
                {
                    Ok(got) => format!(
                        "Received “{}” from {} at {} · replied · code {}",
                        got.text, got.peer, got.from, got.code
                    ),
                    Err(e) => format!("Incoming connection failed: {e}"),
                };
                log(line);
            }
        });
        Ok(Self {
            mdns,
            private: keys.private,
            id,
            name,
            port,
        })
    }

    /// Finds another BioVault on the network, sends it a message and returns its reply.
    pub fn find_and_send(&self) -> Result<String> {
        let events = self.mdns.browse(SERVICE)?;
        let deadline = Instant::now() + FIND_FOR;
        let (mut tried, mut failures, mut last) = (HashSet::new(), Vec::new(), None);
        let connected = 'search: loop {
            let left = deadline.saturating_duration_since(Instant::now());
            let Ok(event) = events.recv_timeout(left) else {
                break None;
            };
            let ServiceEvent::ServiceResolved(found) = event else {
                continue;
            };
            if found.get_property_val_str("id") == Some(self.id.as_str()) {
                continue;
            }
            // Every new address re-resolves the service, and IPv4 and IPv6 arrive in separate
            // packets, so an event may hold only IPv6 (the listener is IPv4-only). A cached entry
            // of an app that restarted since refuses the connection. Either way: keep looking.
            for ip in found.get_addresses_v4() {
                let addr = SocketAddr::from((ip, found.port));
                if tried.insert(addr) {
                    match TcpStream::connect_timeout(&addr, CONNECT_TIMEOUT) {
                        Ok(stream) => {
                            break 'search Some((instance(&found).to_owned(), addr, stream));
                        }
                        Err(e) => failures.push(format!("{addr}: {e}")),
                    }
                }
            }
            last = Some(found);
        };
        self.mdns.stop_browse(SERVICE)?;

        let Some((name, addr, stream)) = connected else {
            let Some(found) = last else {
                return Err(NOT_FOUND.into());
            };
            let (name, addrs) = (instance(&found), found.get_addresses());
            let failures = failures.join("; ");
            return Err(
                format!("found {name} at {addrs:?} but could not connect ({failures})").into(),
            );
        };
        let got = talk(stream, &self.private, &format!("Hello from {}", self.name))?;
        Ok(format!(
            "Found {name} at {addr} · got back “{}” from {} · code {}",
            got.text, got.peer, got.code
        ))
    }
}

/// The instance name, e.g. "BioVault windows d9528e72".
fn instance(found: &ResolvedService) -> &str {
    found
        .fullname
        .trim_end_matches(SERVICE)
        .trim_end_matches('.')
}

/// What one side learned from an exchange.
struct Exchange {
    text: String,
    peer: String,
    /// Six digits from the handshake hash: equal on both screens only if no one sat in between.
    code: String,
    from: SocketAddr,
}

/// Initiator: handshake, send `text`, read the reply.
fn talk(mut stream: TcpStream, private: &[u8], text: &str) -> Result<Exchange> {
    let (mut noise, peer, code) = handshake(&mut stream, private, true)?;
    send(&mut stream, &mut noise, text)?;
    let reply = recv(&mut stream, &mut noise)?;
    Ok(Exchange {
        text: reply,
        peer,
        code,
        from: stream.peer_addr()?,
    })
}

/// Responder: handshake, read one message, reply.
fn answer(mut stream: TcpStream, private: &[u8], me: &str) -> Result<Exchange> {
    let (mut noise, peer, code) = handshake(&mut stream, private, false)?;
    let text = recv(&mut stream, &mut noise)?;
    send(&mut stream, &mut noise, &format!("Hello back from {me}"))?;
    Ok(Exchange {
        text,
        peer,
        code,
        from: stream.peer_addr()?,
    })
}

/// Noise XX: three messages, after which both sides know each other's static key.
fn handshake(
    stream: &mut TcpStream,
    private: &[u8],
    initiator: bool,
) -> Result<(TransportState, String, String)> {
    stream.set_read_timeout(Some(IO_TIMEOUT))?;
    stream.set_write_timeout(Some(IO_TIMEOUT))?;
    let builder = Builder::new(NOISE.parse()?).local_private_key(private)?;
    let mut noise = if initiator {
        builder.build_initiator()?
    } else {
        builder.build_responder()?
    };
    let mut buf = vec![0; u16::MAX.into()];
    while !noise.is_handshake_finished() {
        if noise.is_my_turn() {
            let n = noise.write_message(&[], &mut buf)?;
            write_frame(stream, &buf[..n])?;
        } else {
            noise.read_message(&read_frame(stream)?, &mut buf)?;
        }
    }
    let peer = hex(&noise.get_remote_static().ok_or("peer sent no static key")?[..4]);
    let hash: [u8; 4] = noise.get_handshake_hash()[..4].try_into()?;
    let code = format!("{:06}", u32::from_be_bytes(hash) % 1_000_000);
    Ok((noise.into_transport_mode()?, peer, code))
}

fn send(stream: &mut TcpStream, noise: &mut TransportState, text: &str) -> Result<()> {
    let mut buf = vec![0; text.len() + 16]; // + the ChaChaPoly tag
    let n = noise.write_message(text.as_bytes(), &mut buf)?;
    write_frame(stream, &buf[..n])
}

fn recv(stream: &mut TcpStream, noise: &mut TransportState) -> Result<String> {
    let frame = read_frame(stream)?;
    let mut buf = vec![0; frame.len()];
    let n = noise.read_message(&frame, &mut buf)?;
    buf.truncate(n);
    Ok(String::from_utf8(buf)?)
}

/// Noise frames go on the wire with a 2-byte big-endian length (Noise caps a message at 65,535 bytes).
fn write_frame(stream: &mut TcpStream, frame: &[u8]) -> Result<()> {
    stream.write_all(&u16::try_from(frame.len())?.to_be_bytes())?;
    stream.write_all(frame)?;
    Ok(())
}

fn read_frame(stream: &mut TcpStream) -> Result<Vec<u8>> {
    let mut len = [0; 2];
    stream.read_exact(&mut len)?;
    let mut frame = vec![0; u16::from_be_bytes(len).into()];
    stream.read_exact(&mut frame)?;
    Ok(frame)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypted_exchange_over_loopback() {
        let keys = || {
            Builder::new(NOISE.parse().unwrap())
                .generate_keypair()
                .unwrap()
        };
        let (a, b) = (keys(), keys());
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let addr = listener.local_addr().unwrap();
        let b_private = b.private.clone();
        let server = std::thread::spawn(move || {
            answer(listener.accept().unwrap().0, &b_private, "B").unwrap()
        });

        let at_a = talk(TcpStream::connect(addr).unwrap(), &a.private, "hi").unwrap();
        let at_b = server.join().unwrap();

        assert_eq!(at_b.text, "hi");
        assert_eq!(at_a.text, "Hello back from B");
        assert_eq!(at_a.peer, hex(&b.public[..4]));
        assert_eq!(at_b.peer, hex(&a.public[..4]));
        assert_eq!(at_a.code, at_b.code);
    }
}
