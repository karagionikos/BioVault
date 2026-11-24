# Third-party notices

BioVault's own code is all rights reserved (see [README](README.md)). Third-party components keep their own licenses. This file lists every component BioVault includes, its license, and any attribution that license requires.

## License policy (D63)

| | Licenses |
|---|---|
| **Allowed**, with attribution | MIT · Apache-2.0 · BSD-2-Clause · BSD-3-Clause · ISC · Zlib · MPL-2.0 (file-level copyleft) · Unicode-3.0 · CDLA-Permissive-2.0 |
| **Never** | GPL and AGPL, any version. They conflict with all-rights-reserved distribution. |
| Anything else | Decide case by case. If you allow it, add it to [`deny.toml`](deny.toml) with a reason and note it here. |

**Checks**
- Rust: `cargo deny check licenses` from the repository root, configured in [`deny.toml`](deny.toml).
- npm: a license checker run against production dependencies once `app/package.json` exists (command in [`app/README.md`](app/README.md#rules)).

## Included components

**None yet.** The repository holds no code, no dependencies and no models.

When you add a dependency or a model that ships in the app, add a row here:

| Component | Version | License | Used for | Attribution / notice |
|---|---|---|---|---|

## Planned components (named in the blueprint; not yet included)

Licenses as published on crates.io, npm and Hugging Face on 2026-11-24. Re-check the exact version when you adopt one, then move its row up.

### Shipped inside the app

| Component | Ecosystem | License | Role | Blueprint |
|---|---|---|---|---|
| Tauri 2 (`tauri` 2.12, `@tauri-apps/api` 2.12) | Rust · npm | Apache-2.0 OR MIT | App shell | [04](docs/04-architecture.md), [ADR-003](docs/decisions/ADR-003-tauri-rust-react.md) |
| `tauri-plugin-opener` · `@tauri-apps/plugin-opener` | Rust · npm | Apache-2.0 OR MIT | Open links in the system browser | [04](docs/04-architecture.md#platform-specifics) |
| `tauri-plugin-barcode-scanner` · `@tauri-apps/plugin-barcode-scanner` | Rust · npm | Apache-2.0 OR MIT | QR pairing (phone scans) | [05](docs/05-sync-spec.md#discovery--pairing) |
| React + react-dom 19 | npm | MIT | Interface | [04](docs/04-architecture.md) |
| d3-force 3 | npm | **ISC** | Map layout | [07](docs/07-map-and-interface.md#renderer-d72) |
| PixiJS (`pixi.js` 8) | npm | MIT | Map drawing | [07](docs/07-map-and-interface.md#renderer-d72) |
| perfect-freehand 1 | npm | MIT | Sketch strokes (P3) | [07](docs/07-map-and-interface.md#sketch-canvas-p3-d67-d75) |
| SQLite, bundled (e.g. via `rusqlite`) | Rust | SQLite: public domain · crate: MIT | Database + FTS5 | [03](docs/03-data-model.md) |
| **SPECTER2**: `allenai/specter2_base` + adapters `allenai/specter2` (proximity), `allenai/specter2_adhoc_query` | Model | **Apache-2.0**: ship its license text and credit AllenAI | Fingerprints | [08](docs/08-ml.md), [ADR-004](docs/decisions/ADR-004-ml-approach.md) |
| `ort` 2.0 (rc) + ONNX Runtime, **or** `tract-onnx` 0.23 (Spike 0 picks one) | Rust (+ native library for ort) | ort: MIT OR Apache-2.0 · ONNX Runtime: MIT · tract: MIT OR Apache-2.0 | Model inference | [04](docs/04-architecture.md) |
| `tokenizers` 0.23 | Rust | Apache-2.0 | WordPiece tokenization | [08](docs/08-ml.md) |
| `snow` 0.10 (Noise) **or** `rustls` 0.23 (TLS) | Rust | snow: Apache-2.0 OR MIT · rustls: Apache-2.0 OR ISC OR MIT | Encrypted sync sessions | [05](docs/05-sync-spec.md#discovery--pairing) |
| `ts-rs` 12 **or** `specta` 2 (rc) | Rust | MIT | TypeScript types generated from Rust | [04](docs/04-architecture.md#core--interface-boundary) |
| `uuid` 1 (feature `v7`) | Rust | Apache-2.0 OR MIT | UUIDv7 identifiers | [03](docs/03-data-model.md) |

Note: crates.io's newest `tauri` is a 3.0 alpha. BioVault targets **Tauri 2** (ADR-003): pin `2`.

### Development only (not shipped, no notice needed)

| Tool | License |
|---|---|
| TypeScript 7 · Vite 8 · Vitest 5 · Testing Library (`@testing-library/react` 16) · `@tauri-apps/cli` 2 | Apache-2.0 · MIT · MIT · MIT · Apache-2.0 OR MIT |
| `proptest` 1 (sync simulator) | MIT OR Apache-2.0 |
| `cargo-deny` · `license-checker-rseidelsohn` (license checks) | MIT OR Apache-2.0 · BSD-3-Clause |
| Python in `ml/`: PyTorch · transformers · adapters · onnx · onnxruntime · scikit-learn · skl2onnx · onnxmltools | BSD-style (PyPI lists several permissive licenses) · Apache-2.0 · Apache-2.0 · Apache-2.0 · MIT · BSD-3-Clause · Apache-2.0 · Apache-2.0 |
