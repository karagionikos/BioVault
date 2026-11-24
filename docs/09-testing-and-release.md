# 09 · Testing & release

## Test layers (D75)

| Layer | What | Tooling (my call) | Gate |
|---|---|---|---|
| Core unit tests | vault rules (status derivation, AI-takeaway ban, category cycles), identity merge, label derivation, HLC, op apply, export round-trip | `cargo test` | Every change |
| **Sync simulator** | 2–3 simulated devices, random offline edits, random/interrupted syncs; asserts convergence, no loss, no duplicates, idempotence ([05](05-sync-spec.md#the-sync-simulator-mandatory-d69)) | `sync-sim` crate + `proptest` | **Mandatory** for any sync change and for M2 |
| Source clients | Parse and merge saved sample responses (fixtures) from each API; never hit live servers in tests | Recorded JSON/XML fixtures | Every change |
| Export round-trip | JSON export → import into empty vault → identical canonical state | `cargo test` | Every change |
| ML checks | Quantized vs original fingerprints (cosine ≥ 0.99); reader evaluation set; link model vs v1 on held-out decisions ([08](08-ml.md)) | Python + a small Rust check | Model changes |
| UI | Component tests for logic-heavy pieces (merge display, processing rules, map state) | Vitest + Testing Library | Logic changes |
| Manual device checklist | Short run-through on both devices per release (below) | — | Every release |

### Manual release checklist (both devices)
1. Search a known topic; save two papers; one via DOI paste.
2. Process one paper: finding + takeaway + category; dot turns full.
3. Create a link from the "related" prompt; dismiss one suggestion.
4. Create a lead on the phone; resolve it on the desktop.
5. Sync (P2): edit the same takeaway on both while offline → conflict shown → resolve.
6. Delete and restore a paper from Trash.
7. Export JSON; Tidy up the map; restart; positions persist.

## Versioning

- **App version:** semantic versioning (`0.x` until M2).
- **Schema version:** integer, bumped with every migration; migrations run on startup and are tested from every previous version.
- **Protocol version:** integer for the sync message format.
- Devices sync only when protocol and schema versions match (F-24).

## Building & installing

| Target | Output | How you install |
|---|---|---|
| Windows | NSIS `-setup.exe` (Tauri also supports `.msi` via WiX) | Run the installer |
| Android | Signed release APK | Copy to phone or `adb install -r` over USB |

- **No auto-updater in v1** (D75). Tauri's updater only covers desktop and needs a hosted update location.
- **Release builds** can run locally; optionally on GitHub Actions (free minutes for public repositories). Signing secrets would then live in encrypted repository secrets, never in files.

## ⚠️ The Android signing key (F25)

- Generate **one** release keystore at the start of Phase 2.
- Store it **outside** the repository, plus an offline backup (e.g. a USB drive and a password manager entry for its passwords).
- Android updates an app in place only if the new APK is signed with the same key. Lose the key → you must uninstall → the phone's copy of the vault is wiped (the desktop copy survives; re-pair and resync).

## Repository hygiene

- `README.md` carries: what BioVault is, a **copyright notice**, and **"No license is granted; all rights reserved"** (D62, D63).
- `THIRD_PARTY_NOTICES.md`: every dependency and model with its license (SPECTER2 Apache-2.0 attribution, PixiJS/d3/perfect-freehand MIT, etc.). Add a check that rejects GPL/AGPL dependencies (e.g. `cargo-deny` for Rust, a license checker for npm).
- `.gitignore`: model weights, keystores, `.env`, app data, exports.
- Model files fetched by a script with checksums (`models/fetch.*`), not committed.

## Definition of done (any feature)
1. Its "done when" criteria in [02](02-features-and-flows.md) are met.
2. Tests at the right layers exist and pass (simulator if it touches sync).
3. It works on both devices (from P2 on).
4. Decision log / docs updated if it changed a decision.
