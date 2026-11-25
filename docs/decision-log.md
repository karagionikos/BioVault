# BioVault (working name) — Decision Log

Running record of the blueprint interview. Updated every round; synced to the BioVault project at topic boundaries.
Legend: ✅ decided · 🟡 provisional (my proposal, awaiting confirmation) · ❓ open · ⚠️ risk / flag · 💡 idea parked · ❌ dropped

---

## Project one-liner (draft v4)
A personal research vault for life-science and health research, running on desktop and phone: it helps its owner remember what they read, what they took from it, and how findings across papers connect — surfacing papers through in-app search and a topic feed, and organizing saves into an evolving, categorized evidence map. Research only: it never tells anyone what to do.

## Decisions

### Topic 1 — Vision, purpose & positioning (Round 1)
- ✅ D1 Motivation: personal interest in reading research; core pain = findings get noted down inconsistently (or not at all) and then forgotten.
- ✅ D2 User #1 is the builder himself. No revenue plans. Goal: functional and fun. Scaling to real users is a "maybe, later".
- ✅ D3 Standalone new project — earlier app ideas' decisions do NOT carry over.
- ✅ D4 Core loop: **Collect & connect** is the heart, fed by **Discover**, with a light **Understand** layer (summary + evidence info per paper). **Apply** is out — permanently (see D22).
- ✅ D5 Differentiator: a *personal evidence map* — saved papers, connected, with evidence info visible.
- ✅ D6 Evidence info shown in scientific terms — no simplified grades or lay rewrites.
- ✅ D7 Door to real users kept open only where free (account-ready data model). No sharing/social/moderation, no compliance work until a deliberate decision to go public.

### Topic 2 — User #1: reading habits & the forgetting problem (Rounds 2–3)
- ✅ D8 Discovery channels: PubMed, ClinicalTrials.gov, OpenFDA, journals (e.g. The Lancet), Google Scholar; Reddit and podcasts as leads.
- ✅ D9 Reads mainly on **desktop**, secondarily on phone. Abstract first; full text when it seems useful.
- ✅ D10 Notes today are scattered across phone and paper.
- ✅ D11 What gets forgotten: key findings, own takeaway, and connections between papers. Actively cross-references papers for connected findings.
- ✅ D12 Retrieval: (1) search, (2) "you saved something related before" prompt when saving. (Unprompted resurfacing later dropped — D57.)
- ✅ D13 Interests: curiosity-driven, broad — neuroscience, drug-related studies, kinesiology, psychology, muscle physiology, biomechanics, and more.
- ✅ D14 Fun = building + look & feel + exploring the map; **building** is the one to protect.
- ✅ D15 Working name **BioVault**; "biohacking" dropped as the framing.
- ✅ D16 Categories emerge from saves; no pre-built taxonomy tree.
- ✅ D17 Tech choices may favor learning value, guarded by an early "usable on my real papers" milestone.
- ✅ D18 Platform: one codebase running on desktop (web) and phone, built and tested desktop-first. Phone: quick saves, reviewing takeaways, prompts. Full graph on desktop, simpler view on phone.
- ✅ D19 Saved research items = papers that can be searched and found in the app.
- ✅ D20 User-added content: links, screenshots, etc. (shape → D27).
- ✅ D21 Connections: hybrid. Papers hold one or more findings (key result + own takeaway); links can join whole papers or specific findings; every link has a type (supports / contradicts / same mechanism / extends / related) and an optional note. Graph shows papers by default, expands into findings. Automatic links later, as suggestions kept visibly separate.
- ✅ D22 No self-experiment features — by principle. Research only; applying findings is the user's own choice outside the app.

### Topic 3 — Content: sources, trust & what goes in (Round 4, 2026-11-23)
- ✅ D23 In-app search: **Europe PMC** (life sciences, preprints, OA full text) + **OpenAlex** (all fields, citations, DOI lookup). User also wants **PubMed** and **Google Scholar** → see Q13b. Trials: registration number + link-out on papers that report a trial; ClinicalTrials.gov search is a later candidate. OpenFDA: not searchable; save FDA pages as user-added links.
- ✅ D24 Trust: **no approved-journal list.** Every paper carries evidence labels (study type, human/animal/in-vitro, peer-reviewed vs preprint, year, retraction/correction status; sample size via extraction → Topic 5). Feed defaults stricter (peer-reviewed only) than search (everything, labeled). **Retraction alerts**: saved papers re-checked against Crossref/Retraction Watch data.
- ✅ D25 Animal/in-vitro studies and preprints: **included, labeled, filterable.** Withdrawn preprints flagged like retractions.
- ✅ D26 Full text: **phase 1** = abstract + details in-app, "Read" opens best free copy (Unpaywall) or publisher page. **Later (committed by end of roadmap)** = in-app OA reader + attach own PDFs to any paper, stored privately.
- ✅ D27 User-added content: **attached by default** (to a paper or finding) **or standalone as a "lead"** — a claim/link waiting for its paper. Leads look different on the map, carry no evidence labels, and resolve when a paper is linked.
- ✅ D28 Papers search can't find: paste DOI / PubMed ID / URL → resolved automatically; no identifier → manual entry marked "details unverified".
- ✅ D29 (Q13b) Search sources: **PubMed (direct, free API key) + Europe PMC + OpenAlex**, one search box, results merged so each paper appears once (matched by DOI / PubMed ID), source chips to narrow. **Google Scholar outside the app**: "Open in Google Scholar" button runs the current query in the browser; finds come in by pasting DOI/URL.

### Topic 4 — Categories & the graph (Round 5, 2026-11-23)
- ✅ D30 Categories: **nested, several per paper** (e.g. Neuroscience › Neuroplasticity › BDNF). Also seen as more functional and visually appealing.
- ✅ D31 Category assignment: **suggested** from OpenAlex topics and MeSH terms; one tap to accept / rename / ignore. Suggestions **learn** from past mappings (OpenAlex topic → user category). Nothing filed silently.
- ✅ D32 Map content: **toggleable layers.** Base = papers + typed links, colored by category. Optional layers: category/keyword hubs, findings, leads, automatic suggestions (citations, similarity; dashed). Each link type has its own line style.
- ✅ D33 Map views: **Global view is the signature view** — "fun to watch the graph grow as your knowledge does". 🟡 (my proposal) Local neighborhood = a focus mode opened by tapping a node, not a separate destination; category = filter. On phone, global view shows dots + colors, labels appear on zoom.
- ✅ D34 Scale target: ~1,000 saved papers and a few thousand links over the first year or two (confirmed realistic).
- ✅ D35 Link directions: supports / extends are one-way; contradicts / same mechanism / related are two-way. Five fixed types in v1; custom types later if needed. (No objection.)
- ✅ D36 Leads drawn as a different shape; fade once resolved. (No objection.)
- 🟡 D37 (my call) Every paper, finding, lead, category and link stores when it was created/changed — enables a future "growth replay" of the map at no extra cost.

### Topic 5 — Interests, recommendations & AI (Round 6, 2026-11-23)
- ✅ D38 Interest model: **explicit + learned + explainable.** Followed topics/keywords/authors/saved searches + user categories decide what enters the feed; saves and links re-rank it; every item shows *why* it's there. Content-based (single user).
- ✅ D39 "Related" = **metadata signals + meaning-similarity (text embeddings), combined into one score.** Embeddings also cover the user's own findings and takeaways; enables search-by-meaning inside the vault. Powers: on-save "related" prompt, feed ranking, dashed suggestion links. Embedding model runs locally, free.
- ✅ D40 **No built-in generative AI** (no running costs wanted). Generative features (detail extraction, key-result drafts, link-type suggestions, ask-your-vault) only ever arrive through a bring-your-own-model connector.
- ✅ D41 **Principle: AI never writes the user's takeaway** — writing it is what makes it stick. (Stated in Round 6; no objection.)
- ✅ D42 **Local app, no hosting.** Running cost target: €0. 🟡 Refines D18: the desktop side becomes an installed desktop app rather than a website (→ Topic 9).
- ✅ D43 **Bring-your-own-model connector** for users who want AI (shape & timing → Q29).

### Topic 8 — Devices, sync, backup & AI connector (Round 7, 2026-11-23)
- ✅ D44 Devices: **Android** phone, **Windows** desktop. (Free, permanent sideloading on Android; iOS out of scope unless BioVault goes public.)
- ✅ D45 **Peer-to-peer sync between equal devices** — no home base; desktop and phone are equal full replicas, both can create and edit; each works fully offline and they merge changes when they meet.
- ✅ D46 Sync over the **local network only**: any mix of connections on the same home network (desktop on Ethernet ↔ phone on Wi-Fi included). **No VPN, now or later.**
- ✅ D47 Backups/export: **manual export only** — the two equal replicas are seen as each other's backup. (Safety net + formats → Q28b.)
- ✅ D48 BYO-model connector: **one OpenAI-compatible setting** (server address + key + model name; covers local runners like Ollama/LM Studio and most hosted providers). Built **after the core is solid.** All AI features off by default.
- ✅ D49 API keys (NCBI, OpenAlex) stored in app settings, not in code. (No objection.)
- ✅ D50 Desktop side = installed Windows app (confirms D42 refinement of D18).
- 🟡 D51 (my calls) Sync mechanics: automatic discovery on the home network; first-time pairing by scanning a QR code shown on the other device; encrypted connection; fallback = enter the other device's address. Merge rules: own text (takeaways, notes) edited on both sides → keep both versions and ask; other fields → latest edit wins; links and category memberships → merged.
- ✅ D52 (Q27b) **Same compact model on both devices**; whichever device saves a paper computes its fingerprint, which syncs like any other data. Both devices must run the identical model. (Model choice → Q27c.)
- ✅ D53 (Q28b) **30-day trash** on both devices (deleted papers/findings/links/categories restorable). Manual export in **JSON** (full, restorable), **Markdown with [[links]]** (Obsidian) and **BibTeX/RIS** (Zotero). No automatic backups.

### Topic 6 — First-version scope, screens & key flows (Round 8, 2026-11-23)
- ✅ D54 **First version** (desktop first, then Android + sync): merged search (PubMed + Europe PMC + OpenAlex), "Open in Google Scholar", add by DOI/PubMed ID/URL, manual entry; paper page (abstract, evidence labels, "Read" link); findings + takeaways, typed links, leads, attachments; nested categories with suggestions; global map + focus mode + layers (base, findings, leads); "related" prompt; vault search by keyword and meaning; Android app + LAN sync; manual export + 30-day trash.
- ✅ D55 **Next**: topic feed with "why"; retraction alerts; suggestion layer (citations/similar, dashed) + keyword-hub layer; BYO-model connector; in-app OA reader + attach own PDFs.
- ✅ D56 **Later (parked)**: path view, timeline layout, ClinicalTrials.gov search, custom link types. (Growth replay promoted — D74.)
- ❌ D57 **Dropped altogether**: browser extension; unprompted resurfacing.
- ✅ D58 Home screens: **desktop = global map** with side panels (search, inbox, categories; paper panel on click). **Phone = Inbox**, tabs: Inbox · Search · Map · Vault.
- ✅ D59 **Save → Inbox → Process.** Saving drops a paper into the Inbox; processing = at least one finding with own takeaway + a category; the "related" prompt appears during processing. Unprocessed papers = faint dots on the map; processing turns them into full dots with links — the map grows with understanding, not bookmarks.
- ✅ D60 **Visual design is the user's to do later** (nothing designed yet). 🟡 (my call) The blueprint leaves the visual identity open and lists the visual elements any design must cover (layers, link line styles, faint vs full dots, lead shape, category colors, phone zoom behavior).

### Topic 8 follow-up (Round 9, 2026-11-23)
- ✅ D61 (Q27c) Findings and takeaways are always written in **English** (terminology). Wants to build **his own linking model**; fallback = **SPECTER2**. (Feasible shape → Q27d.)

### Topic 7 — Safety, licensing & naming (Round 9, 2026-11-23)
- ✅ D62 (Q33) **Public, source-available, all rights reserved** — code viewable on GitHub, no open-source license granted. May go private later depending on goals.
- 🟡 D63 (my calls on D62) README carries a copyright notice + "no license is granted" statement. Known consequences: GitHub's terms let other users view and fork a public repo on GitHub; going private later doesn't recall existing public forks or local clones. Avoid GPL/AGPL dependencies (they'd conflict with all-rights-reserved distribution); keep attribution notices for Apache/MIT dependencies (e.g. SPECTER2).
- ✅ D64 (Q34) **Keep "BioVault"**; rename later only if needed. Name kept in one place in code.
- ✅ D65 Research-only notice in About + first launch; good-citizen API use (app name + contact email where asked, rate limits, local caching); nothing shared → copyright stays simple; no analytics/telemetry/crash reports leave devices; model licenses noted in repo. (Proposed in Round 9; no objection.)

### Follow-ups (Round 10, 2026-11-23)
- ✅ D66 (Q27d) **Own link model, built from scratch, on top of SPECTER2** (option C). SPECTER2 reads papers; the own model decides "would I link these?" from fingerprint similarity + shared-detail signals, trained on own links and accept/dismiss decisions. Replaces the hand-tuned score of D39. Arrives in **Next**; v1 ships SPECTER2 + hand-tuned score and **records every accept/dismiss from day one**. No SPECTER2 fine-tuning planned. Predicting link *type* (supports vs contradicts) is out of scope for it.
- ✅ D67 (Q35) **Sketching** in findings: freehand + drawing over attached screenshots; sketch blocks live inside findings (findings = block stacks from day one); drawn with a finger on the phone and a mouse on desktop; stored as pen strokes; arrives in **Next**. **Sketches are notes only** — never used for links, "related" suggestions, fingerprints or search.

### Topic 9 — Tech architecture & stack (Round 11, 2026-11-24)
- ✅ D68 (Q36) **Tauri 2: Rust core + React/TypeScript interface**, one codebase for Windows + Android. Safeguards: the Rust core stays independent of the UI (Flutter via flutter_rust_bridge is the fallback UI); **Spike 0** before features proves (1) the app runs on both devices, (2) SPECTER2 fingerprints a paper on the phone in reasonable time, (3) devices discover each other on the LAN and exchange a message.
- ✅ D69 (Q37) **Own sync engine**: operation log stamped with hybrid logical clocks, exchanged between devices; merge rules per D51/D53; category moves that would create a cycle are rejected. Precise merge spec + a **two-device sync simulation test harness** are mandatory.
- ✅ D70 (Q38) Own link model **trained in Python on the desktop** (user has Python experience), exported as a small model file run by the Rust core on both devices; the model file syncs like data. Hand-written training math (Rust) = optional later exercise.
- ✅ D71 (my calls, Round 11; no objection) SQLite on both devices inside the Rust core + built-in full-text search; no vector database (in-memory comparison at this scale); SPECTER2 as ONNX, 8-bit, two variants (papers / short text & queries), inference engine (ONNX Runtime via ort, or pure-Rust tract) chosen in Spike 0; search-result identity merge DOI → PubMed ID → PMC ID → OpenAlex ID → title+year; pairing = desktop shows QR, phone scans (Tauri mobile barcode-scanner plugin); monorepo core/ app/ ml/ docs/.

### Topic 9 continued (Round 12, 2026-11-24)
- ✅ D72 (Q39) **Own map renderer**: layout physics with d3-force, drawing with PixiJS (both MIT). Full control over faint/full dots, lead shapes, per-type line styles, growth animation.
- ✅ D73 (Q40) **Stable, shared map**: dot positions saved and synced like data (same map on both devices, day to day); new dots settle near what they link to; dragging pins a dot; **"Tidy up"** button re-runs the layout on demand.
- ✅ D74 **"Graph view timelapse" button** wanted (promotes the parked growth replay to a real feature; phase → Q42). 🟡 (my call) Replay reveals dots and lines in the order they were created, at their current positions, showing faint → full as papers get processed; play/pause, speed and a scrubber.
- ✅ D75 (my calls, Round 12; no objection) Sketch canvas built in-house on perfect-freehand (MIT), strokes as point lists, screenshot as background layer (no tldraw, no Excalidraw). Testing: Rust unit tests; mandatory sync simulator; API clients against saved sample responses; light UI component tests + manual per-release checklist on both devices; link model must beat the hand-tuned score on held-out links before replacing it. Windows = NSIS setup .exe; Android = signed APK via USB/file; signing key generated once, kept out of the repo, backed up. No auto-updater in v1. Version check before every sync. All colors/sizes/line styles in one theme file.

### Topic 10 — Time, roadmap & success (Round 13, 2026-11-24)
- ✅ D76 (Q41) **No timeframe, no target date, no rush — enjoyment first.** Weekly hours limited and variable; busy periods expected but work continues through them. → Plan in hours per phase, not calendar dates.
- ✅ D77 (Q42) **Phase plan accepted**: Phase 0 Spike 0 → Phase 1 desktop vault (M1 "it remembers for me") → Phase 2 Android + sync (M2 "one vault, two devices") → Phase 3 Next, in order: timelapse, feed + retraction alerts, suggestion layer + keyword hubs, own link model, sketching, in-app OA reader + own PDFs, BYO-model connector → Phase 4 Later. Development will lean heavily on AI agents, so real hours will likely come in under the estimates. 🟡 (my call) Blueprint written "agent-ready": every feature has "done when" criteria; the sync merge rules and link-model evaluation are marked human-owned (agents assist, you design and review).
- ✅ D78 (Q43) **Success = (a) + (b):** app-counted measures (share of saved papers processed, links per processed paper, share of "related" suggestions accepted) + zero lost edits across syncs (simulator and real use). 🟡 (my call) Shown in a small private Stats section in Settings (Phase 1; sync health added in Phase 2).
- ✅ D79 Blueprint format: zip of linked Markdown docs with Mermaid diagrams, laid out to drop into the repo's docs/ folder; also saved to the BioVault project. Self-review as an outside developer before handover. (Proposed Round 13; no objection.)

### Repo setup (2026-11-24)
- ✅ D80 **Cargo workspace root at the repository root.** One root `Cargo.toml` (virtual workspace) lists `core/biovault-core`, `core/biovault-tauri`, `core/sync-sim` and `app/src-tauri`: one `Cargo.lock`, one `target/`, `cargo test --workspace` from the root. `core/` holds the Rust crates. Refines D71 (blueprint 04 had drawn `core/` itself as the workspace).
- ✅ D81 **Initial repository = hygiene files + planned folders holding READMEs only.** Root: README (copyright + no-license notice), `.gitignore`, `.gitattributes`, `THIRD_PARTY_NOTICES.md`, `deny.toml` (GPL/AGPL license check); `core/`, `app/`, `ml/`, `models/` hold only READMEs; this blueprint and log in `docs/`. No code or config until Spike 0 (throwaway branch) and P1 step 1.1. CI, agent instructions (AGENTS.md / CLAUDE.md), issue/PR templates and editor/format config deferred — Nikos adds them himself later. The license check also allows **Unicode-3.0** and **CDLA-Permissive-2.0**, permissive licenses that Tauri and ort's build tooling pull in (copyright applies to BioVault's own code).
- ✅ D82 (2026-11-25) **Tooling deferred by D81 added:** GitHub Actions CI (`.github/workflows/ci.yml`), `AGENTS.md` + `CLAUDE.md` (CLAUDE.md imports AGENTS.md), issue forms keyed to F-IDs with "done when" boxes, a PR checklist mirroring the Definition of done ([09](09-testing-and-release.md#definition-of-done-any-feature)), and editor/format config for VS Code. CI runs `cargo fmt --check`, clippy (`-D warnings`) and `cargo test --workspace` on a **Windows** runner (the desktop target, no Linux WebKit libraries needed), `cargo deny check licenses`, and in `app/` the npm scripts typecheck, lint, format:check, test and licenses; each job switches on when `Cargo.toml` / `app/package.json` exists. **ESLint** (Nikos's choice over oxlint, which Vite's template now ships) with typescript-eslint, which supports TypeScript only up to 6.0, so TypeScript is pinned to `~6.0` until it supports 7. Prettier is scoped to `app/` so the blueprint's Markdown keeps its style. Line endings: LF in every checkout (`.gitattributes` `eol=lf`), which rustfmt, Prettier and `.editorconfig` all expect.
- ✅ D83 (2026-11-25) **No outside contributions.** Pull requests from anyone but Nikos aren't accepted: with all rights reserved and no contributor agreement, their code can't be merged into BioVault (follows from D62, D63). Stated in the README (§ Copyright and license) and at the top of the PR template. Issues stay open.

## Open questions
None blocking. Items deliberately left for later are listed in the blueprint's roadmap (Open items section).

## Facts gathered (recon)
- Europe PMC: free REST API; all PubMed abstracts + open-access full text + preprints from 30+ servers; flags withdrawn/removed preprints; links to Unpaywall.
- OpenAlex: all-fields index with citations; free API key ($1/day free usage) required since Feb 2026; DOI lookups free. Topics: 4-level hierarchy (4 domains → 26 fields → ~252 subfields → ~4,500 topics), auto-assigned per paper.
- PubMed E-utilities: free; 3 requests/s without key, 10/s with a free key.
- Google Scholar: no official API; terms prohibit automated querying; aggressively blocks scrapers; only paid third-party scraping services exist.
- Crossref REST API: includes Retraction Watch retraction/correction data; full dataset also downloadable as CSV.
- Unpaywall: free REST API by DOI (email parameter) returning the best open-access copy / PDF URL.
- Embedding models: **SPECTER2** (AllenAI) — BERT-base + adapters, trained on 6M citation triplets across 23 fields, input = title + abstract, Apache-2.0, English. **EmbeddingGemma** (Google, Sept 2025) — 308M params, built for on-device use, <200MB RAM when quantized, 100+ languages, output truncatable 768→128 dims, open weights under Gemma terms.
- Name check: OpenMined runs a "BioVault" — a privacy-first genomic analysis mobile app and collaborative research network.
- GitHub: a public repo with no license = default copyright (all rights reserved), but GitHub's terms let other users view and fork it on GitHub. Making a public repo private later splits existing public forks into their own network (they stay public); local clones remain. choosealicense.com suggests a copyright notice + "no license" statement in the README.
- Frameworks (Nov 2026): **Tauri 2** stable since Oct 2024 (2.11 line mid-2026); Tauri team states not all desktop plugins are ported to mobile yet; official mobile barcode-scanner plugin exists. **Flutter** 3.41 (Feb 2026), Windows desktop in production since 2022; flutter_rust_bridge v2 supports Android + Windows. **Compose Multiplatform** 1.11+ supports Android and Windows 10.
- Inference in Rust: **ort** wraps ONNX Runtime and also offers pure-Rust backends (ort-candle, ort-tract); **tract** is a pure-Rust ONNX runtime used for Android/WASM targets.
- CRDTs: **Loro** (Rust, 1.0 Oct 2024, stable format) offers movable tree, LWW map, text (Fugue).
- Map rendering: **Sigma.js** v4 (MIT) — WebGL, built on graphology, pan/zoom/multi-touch built in, custom looks via WebGL node/edge programs. **PixiJS** v8 (MIT) — WebGL/WebGPU 2D renderer with Canvas fallback and multi-touch.
- Sketching: **tldraw** SDK ≥4.0 (Sept 2025) needs a license key in production (commercial, or free hobby for non-commercial) and sends anonymous usage info → unsuitable. **perfect-freehand** (MIT) turns input points into natural pen strokes.
- Tauri distribution: Windows = .msi (WiX, Windows-only build) or NSIS -setup.exe; Android = signed APK/AAB with own keystore (keep private, never in the public repo); Tauri updater plugin is desktop-only and needs signing keys + a hosted endpoint.
- Apple: free Personal Team can install own apps on own iPhone, but they expire after 7 days (max 3 apps per device); Apple Developer Program is $99/year. iOS builds possible from Windows via cloud build services (e.g. Expo).

## Risks & flags
- ✅ F1 Publisher-as-trust — resolved by D24
- ✅ F2 Evidence-level tension — resolved by D25
- ⚠️ F3 Advice boundary — further reduced by D22 (→ Topic 7, light)
- ✅ F4 Paywalls — handled by D26
- ✅ F5 Graph on a phone screen — handled by D18 + D33
- ✅ F6 Private vault vs personalization — resolved by D42 (all interest data stays on-device)
- ⚠️ F7 Crowded space — downgraded; "why not Zotero + Obsidian?" kept as a benchmark
- ✅ F8 Mobile-vs-desktop contradiction — resolved by D18
- ✅ F9 Self-experimentation tension — resolved by D22
- ⚠️ F10 Building-first risk — mitigated by M1 milestone (desktop vault in daily use before sync work) (D77)
- ⚠️ F11 Sample size isn't in metadata; extraction needs AI → only via BYO model (D40, D43); otherwise label shows "n/a"
- ✅ F12 Google Scholar — handled outside the app (D29)
- ✅ F14 AI-drafted takeaways — ruled out by D41
- ✅ F13 Overlapping indexes — handled by identity merge (D71)
- ✅ F15 Local app vs two devices — resolved by D45/D46 (peer-to-peer LAN sync)
- ✅ F16 iPhone build limits — not applicable (Android, D44)
- ✅ F17 Replicas aren't backups — mistakes covered by the 30-day trash (D53)
- ⚠️ F19 Guest Wi-Fi / "client isolation" router settings block device-to-device traffic; some networks block auto-discovery (→ fallback in D51)
- ⚠️ F20 Embeddings from different models can't be compared — every device must use the identical model; changing model = recompute vault (→ Q27c)
- ⚠️ F21 Equal-peer, two-way sync is the hardest engineering piece of the project (→ Topic 9/10 sequencing)
- ⚠️ F22 Name collision with OpenMined's BioVault — accepted for now; revisit before any public release (D64)
- ⚠️ F23 Only search queries and paper lookups leave the device — but they do reveal interests to NCBI / EBI / OpenAlex / Crossref / Unpaywall
- ⚠️ F24 Own link model needs training data first (a few hundred links / accept-dismiss decisions) — handled by logging from day one (D66)
- ⚠️ F25 Losing the Android signing key = can't update the app in place; reinstalling wipes the phone's copy of the vault (desktop copy survives; resync)
- ⚠️ F26 Two devices running different app/data versions could half-sync → version check before every sync
- ⚠️ F18 API keys (NCBI, OpenAlex) live inside a local app — fine for one user; each user brings their own if ever public

## Competitive landscape (recon 2026-11-22)
| Product | What it covers | Gap vs this idea |
|---|---|---|
| R Discovery | Mobile app, personalized paper feed from chosen topics, library, summaries | General academic; no personal graph |
| ResearchRabbit / Litmaps / Connected Papers | Citation/similarity graph maps from seed papers, collections, alerts | Citation-based, not your own typed connections |
| Consensus | AI search + synthesis over 200M+ papers, study-design filters, mobile app | Question-answering tool, not a personal library/graph |
| Examine | Graded evidence for supplements/interventions, research feed on chosen topics | Editorial database, not your own vault |

## Ideas parked
- 💡 Highlight text in a PDF to create a finding
- 💡 ClinicalTrials.gov as a searchable source
- 💡 Path view ("how does A connect to B?") and timeline layout
