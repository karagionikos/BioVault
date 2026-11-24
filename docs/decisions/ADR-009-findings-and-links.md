# ADR-009 · Findings as block stacks, connected by typed links

**Status:** Accepted · 2026-11-22/23 · Decisions D21, D27, D35, D67

## Context
You cross-reference papers for connected *findings*. A paper-to-paper link loses which result connects; finding-only links are precise but heavy.

## Decision
- Papers hold findings; a finding is an ordered **stack of blocks** (key result, takeaway, notes; sketches in P3).
- Links join papers, findings or leads, with a **type**: supports →, extends → (one-way); contradicts ↔, same mechanism ↔, related ↔ (two-way); optional note.
- **Leads** are claims waiting for a paper; they resolve when linked.
- Sketches are notes only — never fingerprinted, searched or used for links.

## Consequences
- The typed links are what make an evidence map ("three findings support each other, one contradicts them").
- The map shows papers by default and expands into findings via a layer.
- Block stacks from day one let sketches drop in later without a data migration.
