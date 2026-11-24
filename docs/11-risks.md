# 11 · Risks

Likelihood / impact: **H**igh, **M**edium, **L**ow. "F##" refers to flags in the [decision log](decision-log.md).

| ID | Risk | L | I | Mitigation | Phase |
|---|---|---|---|---|---|
| R-01 | **Sync bug silently loses or duplicates edits** (F21) | M | **H** | Own engine with explicit merge rules; mandatory property-based simulator; human-owned review; 30-day Trash; JSON export | P2 |
| R-02 | Tauri's Android side is immature for what we need (plugins, WebView quirks) | M | H | Spike 0 first; core independent of UI; Flutter fallback for the interface only | P0 |
| R-03 | SPECTER2 too slow or too large on the phone | M | M | Spike 0 measures; 8-bit quantization; proximity-only fallback; fingerprints sync, so each paper is computed once | P0 |
| R-04 | **Endless setup — the app never gets used** (F10) | M | H | M1 milestone (desktop vault in daily use) before any sync work; each P1 step usable on its own | P1 |
| R-05 | Inbox turns into a bookmark graveyard (anti-goal) | M | H | Save → Inbox → Process; faint dots; "share processed" in Stats | P1 |
| R-06 | Home network blocks device discovery (guest Wi-Fi, client isolation, no multicast) (F19) | M | M | Manual address entry fallback; clear error message naming the likely cause | P2 |
| R-07 | Android signing key lost (F25) | L | M | Generated once, stored outside repo, offline backup | P2 |
| R-08 | OpenAlex usage-based pricing: frequent searches exhaust the free daily allowance | M | L | Cache searches 24 h; prefer free DOI lookups; if exceeded, OpenAlex search pauses for the day and PubMed + Europe PMC carry on | P1 |
| R-09 | Source APIs change, rate-limit or go down | M | M | Per-source isolation; fixtures in tests; graceful degradation; caching | P1 |
| R-10 | Devices end up with different embedding models (F20) | L | H | Model registry op + checksums; switch only when files verified; full re-fingerprint on change | P1–P2 |
| R-11 | Different app/schema versions half-sync (F26) | M | H | Version check in HELLO; refuse with "update the other device first" | P2 |
| R-12 | A GPL/AGPL dependency slips in, conflicting with all-rights-reserved (D63) | M | M | `cargo-deny` + npm license check; THIRD_PARTY_NOTICES | All |
| R-13 | "BioVault" name collision (OpenMined's genomics app) (F22) | — | L | Fine while personal; revisit before going public; name kept in one place | Later |
| R-14 | Search queries reveal interests to external services (F23) | H | L | Documented; no other traffic; no telemetry | All |
| R-15 | Your link model never beats the hand-tuned score | M | L | v1 score stays; replacement only on evidence; learning value regardless | P3 |
| R-16 | Evidence-label heuristics are wrong for some papers | M | M | "Unknown" when unsure; show the source of each label; manual correction marked "edited by you" | P1 |
| R-17 | Sample size unavailable without AI (F11) | H | L | Accepted: shows "n/a"; optional extraction via BYO model in P3 | P3 |
| R-18 | Agent-written code in critical paths that you don't fully understand | M | H | Human-owned list; review; tests before merge | All |
| R-19 | Map becomes unreadable at 1,000 dots | M | M | Level-of-detail rules, filters that dim instead of hide, focus mode, keyword hubs off by default | P1 |
