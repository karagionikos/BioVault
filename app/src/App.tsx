import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { useEffect, useState } from "react";

export default function App() {
  const [platform, setPlatform] = useState("…");
  const [report, setReport] = useState("");
  const [running, setRunning] = useState(false);
  const [lanLog, setLanLog] = useState<string[]>([]);
  const [searching, setSearching] = useState(false);
  const addLan = (line: string) => setLanLog((log) => [...log, line]);

  useEffect(() => {
    invoke<string>("platform").then(setPlatform, (err) => setPlatform(`error: ${String(err)}`));
    const unlisten = listen<string>("lan", (event) => addLan(event.payload));
    return () => void unlisten.then((stop) => stop());
  }, []);

  function lan(command: "lan_listen" | "lan_send") {
    return invoke<string>(command).then(addLan, (err) => addLan(`error: ${String(err)}`));
  }

  // One search at a time: a second mDNS browse of the same service replaces the first.
  function findAndSend() {
    setSearching(true);
    addLan("Searching the network…");
    lan("lan_send").finally(() => setSearching(false));
  }

  function fingerprint() {
    setRunning(true);
    setReport("Loading the model and fingerprinting…");
    invoke<string>("fingerprint_sample")
      .then(setReport, (err) => setReport(`error: ${String(err)}`))
      .finally(() => setRunning(false));
  }

  return (
    <main>
      <h1>BioVault</h1>
      <p>Spike 0 · Rust core is running on: {platform}</p>
      <button type="button" onClick={fingerprint} disabled={running}>
        Fingerprint the sample abstract
      </button>
      <pre style={{ whiteSpace: "pre-wrap" }}>{report}</pre>
      <button type="button" onClick={() => lan("lan_listen")}>
        Listen on the network
      </button>{" "}
      <button type="button" onClick={findAndSend} disabled={searching}>
        Find a device and send a message
      </button>
      <pre style={{ whiteSpace: "pre-wrap" }}>{lanLog.join("\n")}</pre>
    </main>
  );
}
