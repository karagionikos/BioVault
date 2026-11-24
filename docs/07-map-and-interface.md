# 07 · Map & interface

The map is BioVault's signature view (D33): you should be able to watch it grow as your knowledge does.

## Renderer (D72)

- **Layout:** d3-force simulation (link force, many-body repulsion, collision, weak category clustering force — my call) running in a **Web Worker** so the UI stays smooth.
- **Drawing:** PixiJS (WebGL/WebGPU, Canvas fallback). One retained scene graph; nodes and edges as batched sprites/graphics.
- **Level of detail:** labels appear only above a zoom threshold or on hover/focus; edges thin out when zoomed far out; off-screen culling.
- **Target:** 1,000 nodes + 3,000 edges at 60 fps on desktop and smooth on the phone (Spike 0 test 4).

## What's on the map (D32)

| Element | Base layer | Optional layer | Notes |
|---|---|---|---|
| Processed paper | ● full dot, category color | — | Size could reflect number of findings/links (design choice) |
| Inbox paper | ◌ faint dot | — | Becomes full when processed (D59) |
| Your links | Lines, **one style per type** | — | supports/extends drawn with direction |
| Findings | — | **Findings layer**: small satellites around their paper; finding-level links attach to them | Paper-level view by default (D21) |
| Leads | — | **Leads layer**: distinct shape (e.g. ◇), no evidence labels; fade when resolved | D27, D36 |
| Suggestions (P3) | — | **Suggestion layer**: dashed lines for citation/similarity candidates | Always visibly different from your links |
| Keyword hubs (P3) | — | **Keyword layer**: hub nodes for shared OpenAlex topics/MeSH | Off by default — they dominate large maps |

**Filters:** category (incl. sub-tree), study type, species, peer-reviewed, year, processed/inbox. Filtered-out dots dim rather than vanish (my call), so the map keeps its shape.

## A stable, shared map (D73)

- **Positions are data.** Each node's `(x, y, pinned)` is stored and synced (LWW). Both devices show the same map; it doesn't reshuffle between sessions.
- **New dots** start next to the centroid of what they link to (or their category's centroid; or the map's edge if unlinked), then the simulation runs briefly around them only — existing dots barely move.
- **Drag to pin** a dot where you want it. Pinned dots never move except by you.
- **Tidy up** re-runs the full layout for unpinned dots, animated, with an **Undo** for the whole tidy-up (my call).
- **Write policy:** positions are recorded when the simulation settles or a drag ends — never per frame.

## Focus mode & navigation (D33)

- Click/tap a dot → camera eases to it; its 1-step neighbors (2-step on desktop, toggle) stay bright; everything else dims. The paper panel opens beside the map (desktop) or as a bottom sheet (phone).
- Click a line → shows link type + note; buttons to open either end.
- Keyboard (desktop, my call): `/` search, `Esc` exit focus, `F` frame all, `T` tidy up.

## Phone map (D18, D33)

- Same data, same positions; viewport fits to screen.
- Dots + category colors only; labels on zoom; tap for focus; long-press a dot to start drawing a link to another dot.
- Layers and filters in a bottom sheet.

## Timelapse (P3, D74)

- **Map → Timelapse** button. The map clears and replays history **at today's positions**:
  - a paper appears (faint) at its save time, turns full at its processed time;
  - links draw at their creation time; leads appear and fade at their open/resolved times;
  - deletions disappear at their deletion time.
- Controls: play/pause, speed (1 day/s … 1 month/s), **scrubber with dates**, current date label, counts (papers, links).
- Data needed is already recorded from day one: HLC timestamps on every entity, and the op log (kept in full, [05](05-sync-spec.md)). A paper's "processed at" moment is the HLC of the op that first made it meet the processed conditions, recovered by replaying the log (D37). No historical positions are needed.

## Design brief

Visual identity is yours to design later (D60). Everything visual reads from **one theme file** (colors, sizes, fonts, line styles, animation timings) so your design drops in without code changes (D75). Any design must cover:

1. **Dot states:** processed (full), inbox (faint), selected, focused-neighbor, dimmed-by-filter, retracted (must be unmistakable).
2. **Lead shape** (open vs resolved/faded).
3. **Five link line styles** + direction markers for supports/extends; your links vs dashed suggestions (P3).
4. **Category colors** — scheme that works for a nested tree (children inherit/shade parent's hue?) and for colorblind viewing.
5. **Evidence label badges** compact enough for the paper panel and search results.
6. **Zoom behavior:** when labels appear; edge thinning; cluster legibility at 1,000 dots.
7. **Phone layouts:** Inbox list, bottom sheet paper panel, map gestures.
8. **Timelapse** look: how appearing/filling/drawing is animated.
9. **Empty states:** first launch, empty Inbox, empty map.
10. **Light/dark** (or a single deliberate theme).

## Other screens (P1 unless noted)

| Screen | Desktop | Phone |
|---|---|---|
| Inbox | Side panel list | Home tab |
| Search | Side panel → results list → paper panel | Search tab |
| Paper | Right panel | Full screen / bottom sheet |
| Processing | Inline in paper panel: finding editor + "related" prompt + category picker | Same, stacked |
| Vault | Category tree + list view with filters | Vault tab |
| Trash | Settings → Trash | Settings → Trash |
| Settings | Account-free: email, API keys, devices (P2), export, stats, about | Same |

Interface language: English (my call). Desktop is fully usable by keyboard for the core flows (search, save, process, link).

## Sketch canvas (P3, D67, D75)

- A `sketch` block inside a finding. Tools: pen (2–3 widths), eraser, a few colors from the theme, undo/redo, clear.
- Optional **background image**: any screenshot attachment of that finding/paper; drawing over it to circle the key part of a figure.
- Input: pointer events — finger on phone, mouse on desktop; pressure used if available, else velocity.
- Strokes stored as point lists (`[x, y, pressure]`, normalized to the canvas size) and rendered with perfect-freehand.
- **Notes only:** sketches are never fingerprinted, never searched, never used for links or suggestions.
- Sync: a sketch block is "your text" for conflict purposes — concurrent edits keep both versions.
