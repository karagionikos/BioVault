# ADR-004 · SPECTER2 reader + your own link model; no built-in generative AI

**Status:** Accepted · 2026-11-23/24 · Decisions D39, D40, D41, D52, D61, D66, D70

## Context
"Related" suggestions need meaning-similarity across fields. You don't want running costs, you write findings in English, and you want to build your own linking model.

## Decision
- **SPECTER2** (8-bit ONNX) runs on **both** devices — the identical model — to fingerprint papers, findings and leads; whichever device saves an item computes its fingerprint, which then syncs.
- v1 "related" = a **hand-tuned score** combining fingerprint similarity with shared categories, topics/MeSH, citations and authors. **Every suggestion decision is logged from day one** with a feature snapshot.
- In P3, **your own link model** — designed and trained by you in Python on the desktop, exported as a small model file — replaces the hand-tuned score *only if* it beats it on held-out decisions.
- **No built-in generative AI.** An optional OpenAI-compatible bring-your-own-model connector comes after the core (P3), all features off by default.
- **AI never writes your takeaway** — enforced in the core.

## Alternatives considered
- Training a text model from scratch: unrealistic at home-GPU scale.
- Fine-tuning SPECTER2: feasible but not chosen; re-fingerprinting on every retrain.
- EmbeddingGemma: strong on-device general model, multilingual — unnecessary since findings are English.

## Consequences
- €0 cost; fully offline suggestions; ~100–300 MB model on each device.
- Sample size label is "n/a" without a connected model.
- The link model can predict *whether* things connect, not *how* (type stays yours).
