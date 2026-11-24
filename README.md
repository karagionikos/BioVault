# BioVault

A personal research vault for life-science and health research. It's one app on a Windows desktop and an Android phone: it works fully offline, syncs the two devices over the home network, and turns the papers you read into a **personal evidence map** of papers, the findings you took from them, and the typed links between them.

> **Research only.** BioVault organizes what published research says. It never tells anyone what to do.

## Status

**Blueprint done, no code yet.** This first commit sets up the repository: hygiene files, the planned folders (each with a README saying what goes there), and the full blueprint in [`docs/`](docs/README.md).

Next is **Spike 0**, a throwaway branch that proves the risky parts before any feature work: Tauri on Android, SPECTER2 on the phone, LAN discovery, and a WebGL map. After it comes P1 step 1.1, the real skeleton. See the [roadmap](docs/10-roadmap.md).

## Repository layout

| Path | What lives there |
|---|---|
| [`core/`](core/README.md) | Rust crates: `biovault-core` (all logic, no Tauri), `biovault-tauri` (command/event wrapper), `sync-sim` (sync simulator) |
| [`app/`](app/README.md) | The Tauri 2 app: React + TypeScript interface in [`src/`](app/src/README.md), app crate and platform config in [`src-tauri/`](app/src-tauri/README.md) |
| [`ml/`](ml/README.md) | Python: SPECTER2 export, the reader evaluation set, link-model training |
| [`models/`](models/README.md) | Model files, **never committed**. Only a fetch script and checksums live in git |
| [`docs/`](docs/README.md) | The blueprint (00–11), the [decision log](docs/decision-log.md) (D1–D81) and the [ADRs](docs/decisions/) |
| `Cargo.toml` *(arrives with code)* | Cargo workspace root over `core/*` and `app/src-tauri` (D80) |
| [`deny.toml`](deny.toml) | Rust license check that rejects GPL/AGPL (`cargo deny check licenses`) |
| [`THIRD_PARTY_NOTICES.md`](THIRD_PARTY_NOTICES.md) | License policy and every third-party component's license |

## Rules that protect the project

From the blueprint, for anyone (human or agent) who writes code here:

1. **`biovault-core` never depends on Tauri.** The interface stays swappable ([04](docs/04-architecture.md#the-rule-that-protects-the-project-the-core-doesnt-know-about-the-ui)).
2. **Every data change goes through `vault` → `oplog`.** The interface never writes to the database.
3. **AI never writes a takeaway.** This is enforced in the core, not the UI (D41).
4. **No sync change merges unless the sync simulator passes.** Sync merge logic and link-model evaluation are human-owned ([10](docs/10-roadmap.md#working-with-ai-agents-d77)).
5. **No GPL/AGPL dependencies.** Check every new one ([`deny.toml`](deny.toml), [`THIRD_PARTY_NOTICES.md`](THIRD_PARTY_NOTICES.md)).
6. **Some things never enter this repository:** model weights, the Android signing key, `.env` files, and anything exported from your vault. The repository is public ([`.gitignore`](.gitignore)).

## Copyright and license

Copyright © 2026 Karageorgis Nikos (karagionikos). All rights reserved.

**No license is granted.** The source is public so that it can be read; that doesn't make it open source. Apart from what GitHub's Terms of Service allow on GitHub itself (viewing and forking), you may not copy, modify, distribute or use any part of it without written permission.

Third-party components keep their own licenses: see [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).
