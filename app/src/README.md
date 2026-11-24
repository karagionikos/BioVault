# app/src: the interface

React + TypeScript: the screens, the map renderer and the theme. It talks to the core only through the commands and events of [`biovault-tauri`](../../core/biovault-tauri/README.md).

## Planned contents

| Area | What | Phase | Spec |
|---|---|---|---|
| Screens | Inbox, Search, Paper (with processing), Vault, Trash, Settings (stats, export, about; devices in P2) | P1 | [07 · Other screens](../../docs/07-map-and-interface.md#other-screens-p1-unless-noted), [02 · Screens](../../docs/02-features-and-flows.md#screens) |
| Map renderer | d3-force layout in a Web Worker + PixiJS drawing; layers, focus mode, stable positions, Tidy up | P1 | [07](../../docs/07-map-and-interface.md) |
| Theme | The one file every visual reads from | P1 | [07 · Design brief](../../docs/07-map-and-interface.md#design-brief) |
| Generated types | TypeScript types generated from the Rust command and event types. Never edited by hand | P1 | [04](../../docs/04-architecture.md#core--interface-boundary) |
| Timelapse | Replays the map's growth at current positions | P3 | [07](../../docs/07-map-and-interface.md#timelapse-p3-d74) |
| Sketch canvas | perfect-freehand, strokes stored as point lists | P3 | [07](../../docs/07-map-and-interface.md#sketch-canvas-p3-d67-d75) |

Home screens (D58): the desktop opens on the map with side panels. The phone opens on the Inbox, with tabs Inbox · Search · Map · Vault.

## Map rules to know before writing code

- **Positions are data.** Write `(x, y, pinned)` only when the simulation settles or a drag ends, never per frame.
- New dots settle near what they link to. Pinned dots never move unless you move them.
- Filtered-out dots dim; they don't vanish.
- Target: 1,000 dots + 3,000 links at 60 fps on the desktop, smooth on the phone.

## Tests

Vitest + Testing Library for the logic-heavy pieces: merge display, processing rules, map state ([09](../../docs/09-testing-and-release.md#test-layers-d75)).
