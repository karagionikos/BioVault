# core/: Rust crates

Everything BioVault *knows* lives here, written once for both devices: storage, sources, search, sync and on-device ML. The crates are members of the Cargo workspace whose root is the repository's `Cargo.toml` (D80).

| Crate | What it is | Depends on |
|---|---|---|
| [`biovault-core/`](biovault-core/README.md) | The library: all data, rules and logic. **No Tauri dependency.** | — |
| [`biovault-tauri/`](biovault-tauri/README.md) | Thin layer that exposes the core as Tauri commands and events | `biovault-core`, Tauri |
| [`sync-sim/`](sync-sim/README.md) | Sync simulator: 2–3 in-memory devices, random edits and syncs, convergence checks | `biovault-core` |

The app binary itself is [`app/src-tauri`](../app/src-tauri/README.md). It's also a workspace member and depends on `biovault-tauri`.

## Rules

1. **The core doesn't know about the UI.** `biovault-core` never depends on Tauri, so the interface could move to Flutter (via flutter_rust_bridge) without touching it ([04](../docs/04-architecture.md#the-rule-that-protects-the-project-the-core-doesnt-know-about-the-ui), D68).
2. **One write path.** Every change to data goes through `vault` → `oplog`. Nothing else writes to SQLite.
3. **Ops from the first build.** The op log records every mutation from P1's first build, although sync arrives in P2. The desktop vault then syncs later without a migration ([05](../docs/05-sync-spec.md#operation)).
4. **Human-owned:** sync merge rules, conflict detection, the simulator's assertions, and the rule that AI never writes a takeaway ([10](../docs/10-roadmap.md#working-with-ai-agents-d77)).
5. **Gates:** `cargo test --workspace` on every change, and the simulator for any sync change ([09](../docs/09-testing-and-release.md#test-layers-d75)).
6. **Licenses:** no GPL/AGPL. Run `cargo deny check licenses` from the root ([`deny.toml`](../deny.toml)). Give every BioVault crate `publish = false`; the check then skips our own, deliberately unlicensed crates.

## When code arrives

Spike 0 (a throwaway branch) proves Rust + ONNX on Android and LAN discovery ([04](../docs/04-architecture.md#spike-0--prove-the-risky-parts-first-d68)). P1 step 1.1 then creates the real crates, the SQLite schema and migrations, and op-log recording ([10](../docs/10-roadmap.md)).
