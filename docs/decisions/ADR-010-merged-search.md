# ADR-010 · One search box over PubMed, Europe PMC and OpenAlex; Google Scholar outside the app

**Status:** Accepted · 2026-11-23 · Decisions D23, D29, D71

## Context
You find research via PubMed, Google Scholar, journals and more. Each index has different strengths; Google Scholar has no official API and forbids automated access.

## Decision
- One search box queries **PubMed** (own ranking/MeSH), **Europe PMC** (preprints, open-access full text) and **OpenAlex** (all fields, topics, citations) in parallel; results are merged by identity (DOI → PubMed ID → PMC ID → OpenAlex ID → fuzzy title+year), with source chips to narrow.
- **Google Scholar** stays outside: an "Open in Google Scholar" button runs the current query in the browser; finds come in by pasting a DOI/URL.
- ClinicalTrials.gov: link-out from papers now, searchable later (P4). OpenFDA: save pages as links.

## Consequences
- Broad coverage with clean deduplication — a satisfying engineering problem.
- Three clients to maintain, rate-limit and cache; OpenAlex searches consume a daily free allowance (lookups are free).
