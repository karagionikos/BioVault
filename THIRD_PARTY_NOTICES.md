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

Spike 0 (throwaway branch). When you add a dependency or a model that ships in the app, add a row here:

| Component | Version | License | Used for | Attribution / notice |
|---|---|---|---|---|
| Tauri (`tauri`, `tauri-build`), incl. its placeholder app icons | 2.12.7 · 2.7.6 | Apache-2.0 OR MIT | App shell | Tauri Programme within The Commons Conservancy |
| `@tauri-apps/api` | 2.12.5 | Apache-2.0 OR MIT | Interface ↔ Rust calls | Tauri Programme within The Commons Conservancy |
| React + react-dom | 19.3.1 | MIT | Interface | Meta Platforms, Inc. and affiliates |
| `ort` + ONNX Runtime (prebuilt static library, downloaded at build time by `ort-sys`) | 2.0.0-rc.15 · 1.31.0 | ort: MIT OR Apache-2.0 · ONNX Runtime: MIT | Model inference (Spike 0 pick) | pyke.io (ort); Microsoft Corporation (ONNX Runtime). ONNX Runtime bundles further components (protobuf, abseil, re2, …) listed in its `ThirdPartyNotices.txt`: ship that file with the app |
| `tokenizers` | 0.22.2 | Apache-2.0 | WordPiece tokenization | Hugging Face, Inc. |
| `serde_json` | 1.0.154 | MIT OR Apache-2.0 | Reading the sample-abstract fixture | Erick Tryzelaar, David Tolnay |
| **SPECTER2**: `allenai/specter2_base` + adapters `allenai/specter2` (proximity), `allenai/specter2_adhoc_query`, exported to int8 ONNX | — | **Apache-2.0**: ship its license text and credit AllenAI | Fingerprints | Allen Institute for AI (AI2) |

## Planned components (named in the blueprint; not yet included)

Licenses as published on crates.io, npm and Hugging Face on 2026-11-24. Re-check the exact version when you adopt one, then move its row up.

### Shipped inside the app

| Component | Ecosystem | License | Role | Blueprint |
|---|---|---|---|---|
| `tauri-plugin-opener` · `@tauri-apps/plugin-opener` | Rust · npm | Apache-2.0 OR MIT | Open links in the system browser | [04](docs/04-architecture.md#platform-specifics) |
| `tauri-plugin-barcode-scanner` · `@tauri-apps/plugin-barcode-scanner` | Rust · npm | Apache-2.0 OR MIT | QR pairing (phone scans) | [05](docs/05-sync-spec.md#discovery--pairing) |
| d3-force 3 | npm | **ISC** | Map layout | [07](docs/07-map-and-interface.md#renderer-d72) |
| PixiJS (`pixi.js` 8) | npm | MIT | Map drawing | [07](docs/07-map-and-interface.md#renderer-d72) |
| perfect-freehand 1 | npm | MIT | Sketch strokes (P3) | [07](docs/07-map-and-interface.md#sketch-canvas-p3-d67-d75) |
| SQLite, bundled (e.g. via `rusqlite`) | Rust | SQLite: public domain · crate: MIT | Database + FTS5 | [03](docs/03-data-model.md) |
| `snow` 0.10 (Noise) **or** `rustls` 0.23 (TLS) | Rust | snow: Apache-2.0 OR MIT · rustls: Apache-2.0 OR ISC OR MIT | Encrypted sync sessions | [05](docs/05-sync-spec.md#discovery--pairing) |
| `ts-rs` 12 **or** `specta` 2 (rc) | Rust | MIT | TypeScript types generated from Rust | [04](docs/04-architecture.md#core--interface-boundary) |
| `uuid` 1 (feature `v7`) | Rust | Apache-2.0 OR MIT | UUIDv7 identifiers | [03](docs/03-data-model.md) |

Note: crates.io's newest `tauri` is a 3.0 alpha. BioVault targets **Tauri 2** (ADR-003): pin `2`.

### Development only (not shipped, no notice needed)

| Tool | License |
|---|---|
| TypeScript 6 (`~6.0` until typescript-eslint supports 7, D82) · Vite 8 · Vitest 5 · Testing Library (`@testing-library/react` 16) · `@tauri-apps/cli` 2 | Apache-2.0 · MIT · MIT · MIT · Apache-2.0 OR MIT |
| `@vitejs/plugin-react` 6 · `@types/react` · `@types/react-dom` · `@types/node` | MIT (all) |
| `proptest` 1 (sync simulator) | MIT OR Apache-2.0 |
| ESLint 10 · `@eslint/js` · typescript-eslint 8 · `eslint-plugin-react-hooks` · `eslint-plugin-react-refresh` · `globals` · `eslint-config-prettier` · Prettier 3 (lint and format) | MIT (all) |
| `cargo-deny` · `license-checker-rseidelsohn` (license checks) | MIT OR Apache-2.0 · BSD-3-Clause |
| Python in `ml/`: PyTorch · transformers · adapters · onnx · onnxruntime · scikit-learn · skl2onnx · onnxmltools | BSD-style (PyPI lists several permissive licenses) · Apache-2.0 · Apache-2.0 · Apache-2.0 · MIT · BSD-3-Clause · Apache-2.0 · Apache-2.0 |
