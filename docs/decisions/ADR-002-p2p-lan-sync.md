# ADR-002 · Equal-peer LAN sync with an own sync engine

**Status:** Accepted · 2026-11-23/24 · Decisions D45, D46, D51, D69

## Context
One vault must live on a Windows desktop and an Android phone, with no hosting (ADR-001). You read mostly on desktop but save and review on the phone; either device may be offline.

## Decision
- Desktop and phone are **equal full replicas**; neither is a home base.
- They sync **only on the local network** (any mix of Ethernet and Wi-Fi), discovered via mDNS, paired once by QR code, over encrypted, mutually authenticated sessions. No VPN, now or later.
- Sync uses **our own engine**: an operation log stamped with hybrid logical clocks, exchanged by version vectors, with explicit per-field merge rules (LWW for scalars; observed-remove sets for memberships; **keep both versions** for concurrent edits of your own text; deletions to Trash).

## Alternatives considered
- **CRDT library (Loro/Automerge):** proven merge math, movable trees — but their text types blend concurrent edits character-by-character, the opposite of "keep both and ask". Rejected; Loro's tree algorithm remains useful reading.
- Desktop-as-hub: rejected — devices are equals.

## Consequences
- The hardest engineering piece of the project; **human-owned**; a property-based sync simulator is mandatory.
- Works only at home; sync waits until both devices share a network.
- Learning value is high: HLCs, version vectors, convergence testing.
