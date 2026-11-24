# models/: model files (never committed)

On your machine this folder holds the model files the app runs. In the repository it holds only this README and, from Spike 0, a fetch script and a checksum list. Weights never go into git (D63, D75). The root `.gitignore` blocks everything here except `README.md`, `fetch.*` and `SHA256SUMS`, and blocks `*.onnx` and other weight formats anywhere in the repository.

## What lands here

| File (planned) | What it does | Made by |
|---|---|---|
| SPECTER2 proximity, int8 ONNX | Fingerprints papers, findings and leads | [`ml/`](../ml/README.md) export |
| SPECTER2 adhoc query, int8 ONNX | Fingerprints vault search queries (might be dropped for the phone; Spike 0 decides) | `ml/` export |
| Tokenizer files | The WordPiece tokenizer for the Rust `tokenizers` crate | `ml/` export |
| Your link model (P3) | Ranks how likely you are to link two items | `ml/` training |

The export in Spike 0 sets the exact file names.

## How files get here

- `fetch.*` (Spike 0) places the files and verifies them against `SHA256SUMS`. The int8 files come from your own export, so where the script fetches them from is still open: rerun the export from `ml/`, or download a copy you store somewhere.
- The app accepts a model only when its checksum matches.

## Rules

- Both devices must run **identical** model files. Switching models means re-fingerprinting the vault, coordinated by a `model_registry` op ([08](../docs/08-ml.md), F20).
- SPECTER2 is Apache-2.0. When the app bundles it, ship its license and credit AllenAI ([THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md)).
