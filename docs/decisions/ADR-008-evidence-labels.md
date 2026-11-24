# ADR-008 · No approved-journal list: evidence labels + filters

**Status:** Accepted · 2026-11-23 · Decisions D6, D24, D25

## Context
The original idea was "trustworthy publishers". But a respected publisher doesn't guarantee a trustworthy study, and strict lists would block good work in smaller field journals (biomechanics, sports science). Much drug and physiology research is animal/in-vitro or preprint.

## Decision
- No whitelist. Every paper carries **evidence labels** in scientific terms: study type, human/animal/in-vitro, peer-reviewed vs preprint, year, retraction/correction status (sample size when available).
- Animal/in-vitro studies and preprints are **included, labeled, filterable**.
- Feed defaults (P3) are stricter than search.
- **Retraction alerts** (P3) re-check saved papers; withdrawn preprints flagged like retractions.

## Consequences
- You stay in control; nothing is hidden by default in search.
- Labels come from heuristics over source metadata — "unknown" when unsure; manual correction allowed and marked.
