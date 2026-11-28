# ml/: Python model work

Python on the desktop (D70): preparing SPECTER2 for the app, checking its quality, and (in P3) designing and training your own link model. Nothing here ships in the app except the exported model files. Those go to [`models/`](../models/README.md) and are never committed.

> **This repository is public.** Training data exported from your vault (`links.csv`, `suggestion_events.csv`, `features.parquet`) and the reader evaluation set show what you read and how you connect it. Keep them in `ml/data/`, which is git-ignored, and never commit them.

## 1 · SPECTER2 export (Spike 0, then P1)

From [08 §1](../docs/08-ml.md#1--specter2-on-both-devices-d52-d61-d71):

1. Load `allenai/specter2_base` with the **proximity** adapter (`allenai/specter2`), and again with the **adhoc query** adapter (`allenai/specter2_adhoc_query`).
2. Merge or activate the adapter, then export to ONNX.
3. Apply dynamic int8 quantization.
4. Verify that quantized and original fingerprints have cosine ≥ 0.99 on a sample.
5. Record SHA-256 checksums for `models/SHA256SUMS`.

Expect about 100–150 MB per variant. If two are too heavy for the phone, v1 uses proximity for queries too (Spike 0 decides). Both devices must run the identical files (F20).

## 2 · Reader evaluation set (P1)

From [08](../docs/08-ml.md#evaluation-set-for-the-reader-my-call): about 50 of your own papers where you already know which ones connect. It answers which text recipe works best for findings, whether quantization hurt quality, and later how your link model compares. It doubles as a regression test whenever a model changes.

## 3 · Your own link model (P3)

> **Human-owned:** feature design, evaluation and the replace-or-not decision are yours ([08 §4](../docs/08-ml.md#4--your-own-link-model-p3-d66-d70)).

- Start at about 200 links and 300 logged decisions.
- Data: Settings → Developer → *Export training data*, saved into `ml/data/`.
- Models in order: logistic regression → gradient-boosted trees → a small neural network (PyTorch). Export to ONNX.
- Evaluation: time-based split (newest ~20% held out); precision@5, recall@10 and MRR, computed against the hand-tuned v1 score on the same split.
- Replace v1 only if the model is better on at least two metrics and not clearly worse on any.

## Suggested layout

```
ml/
├─ notebooks/   # exploration and evaluation
├─ src/         # export, quantization and training code
├─ data/        # exported vault data and the evaluation set (git-ignored)
└─ artifacts/   # exported models before they move to models/ (git-ignored)
```

Spike 0 used a plain venv with pinned versions (the export's checksums depend on them):

```powershell
py -3.14 -m venv ml/.venv
ml/.venv/Scripts/python -m pip install -r ml/requirements.txt --extra-index-url https://download.pytorch.org/whl/cpu
ml/.venv/Scripts/python ml/src/export_specter2.py
```

The export downloads about 450 MB from Hugging Face and needs about 3 GB of disk while it runs. Python libraries run only on your desktop and don't ship. Anything whose code or weights end up in the app must pass the license policy in [THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md).
