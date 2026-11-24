# biovault-tauri

A thin layer that exposes `biovault-core` to the React interface as Tauri **commands** and **events**. No business logic lives here: if a function decides something about the vault, it belongs in the core.

## The boundary

From [04 · Core ↔ interface boundary](../../docs/04-architecture.md#core--interface-boundary):

- **Commands** (UI → core): request/response. The TypeScript types are generated from the Rust types with `ts-rs` or `specta` (not chosen yet).
- **Events** (core → UI): `vault_changed {entities}`, `search_results {batch}`, `sync_status`, `suggestions_ready`. The UI re-queries what it shows; it never assumes.
- Long jobs (fingerprinting, sync, search fan-out) run on background threads in the core. This crate forwards their progress as events.

## What it may depend on

`biovault-core` and Tauri. The dependency only points one way: `biovault-core` must never depend on this crate, or on Tauri.
