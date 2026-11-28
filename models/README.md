# models/: model files (never committed)

On your machine this folder holds the model files the app runs. In the repository it holds only this README and, from Spike 0, a fetch script and a checksum list. Weights never go into git (D63, D75). The root `.gitignore` blocks everything here except `README.md`, `fetch.*` and `SHA256SUMS`, and blocks `*.onnx` and other weight formats anywhere in the repository.

## What lands here

| File (planned) | What it does | Made by |
|---|---|---|
| `specter2-proximity-int8.onnx` (132.6 MB) | Fingerprints papers, findings and leads | [`ml/`](../ml/README.md) export |
| `specter2-adhoc_query-int8.onnx` (132.6 MB) | Fingerprints vault search queries (kept on the phone too, D84) | `ml/` export |
| `tokenizer.json` | The WordPiece tokenizer for the Rust `tokenizers` crate | `ml/` export |
| Your link model (P3) | Ranks how likely you are to link two items | `ml/` training |

[`ml/src/export_specter2.py`](../ml/src/export_specter2.py) writes all three and `SHA256SUMS` (Spike 0). Checksums only repeat with the same package versions ([`ml/requirements.txt`](../ml/requirements.txt)).

## How files get here

- `fetch.*` (Spike 0) places the files and verifies them against `SHA256SUMS`. The int8 files come from your own export, so where the script fetches them from is still open: rerun the export from `ml/`, or download a copy you store somewhere.
- The app accepts a model only when its checksum matches.

## Rules

- Both devices must run **identical** model files. Switching models means re-fingerprinting the vault, coordinated by a `model_registry` op ([08](../docs/08-ml.md), F20).
- SPECTER2 is Apache-2.0. When the app bundles it, ship its license and credit AllenAI ([THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md)).
