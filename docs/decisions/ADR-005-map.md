# ADR-005 · Own map renderer and a stable, shared map

**Status:** Accepted · 2026-11-24 · Decisions D32, D33, D72, D73, D74

## Context
The global map is the signature view: "fun to watch the graph grow as your knowledge does". The visual design doesn't exist yet. BioVault is a memory tool — spatial memory matters.

## Decision
- **Own renderer:** d3-force layout in a Web Worker + PixiJS drawing.
- **Stable, shared map:** node positions are data, saved and synced; new dots settle near what they link to; drag to pin; **Tidy up** on demand.
- **Timelapse** (P3) replays creation history at current positions.

## Alternatives considered
- **Sigma.js:** fast, WebGL, interactions built in — but unusual designs require low-level shader programs; would likely be replaced once the design exists.
- Re-laying out on every open: rejected — destroys "where things are" memory.

## Consequences
- More initial work than a ready-made library; full visual freedom for the future design.
- Positions generate sync ops — written only on settle/drag.
- Timelapse is cheap because timestamps are recorded from day one.
