# ADR-006 · Save → Inbox → Process

**Status:** Accepted · 2026-11-23 · Decisions D41, D59

## Context
The original problem is forgetting. Saving a paper isn't remembering it; writing your takeaway is. But forcing a takeaway at save time makes quick saves on the phone painful.

## Decision
Saving puts a paper in the **Inbox**. A paper becomes **processed** when it has ≥1 finding with a non-empty takeaway **and** ≥1 category. The "related" prompt appears during processing. Inbox papers are **faint dots**; processed papers are **full dots** with their links.

## Alternatives considered
- Takeaway required at save: blocks quick capture.
- Takeaways optional, no Inbox: quietly recreates the bookmark graveyard.

## Consequences
- The map grows with understanding, not bookmarks.
- "Share processed" becomes the primary health measure in Stats.
- Status is derived deterministically on each device — no sync conflicts over it.
