# 01 · Vision & scope

## Why BioVault exists

- **The pain (D1, D10, D11):** findings get written down inconsistently — phone notes, paper, or not at all — and then forgotten. What's lost is threefold: *what the paper found*, *your own takeaway*, and *how it connects to other papers*. Cross-referencing papers for connected findings is something you actively do; today it lives only in your head.
- **The fix:** a single place where every paper you care about gets a finding, a takeaway and links — and where those come back when you need them: through **search** and through a **"you saved something related before"** prompt while processing (D12).
- **Why build it rather than use Zotero + Obsidian:** because building it is the fun part (D14), and because neither gives you an evidence map with typed links between findings, evidence labels on every node, and meaning-based "related" suggestions across your own takeaways. The Zotero + Obsidian pair stays a useful benchmark for *what not to reinvent* — hence the exports to both formats (D53).

## Who it's for

**User #1 is you** (D2): a curiosity-driven reader of life-science research — neuroscience, drug-related studies, kinesiology, psychology, muscle physiology, biomechanics and more (D13). You read mostly on desktop, sometimes on your phone; abstract first, full text when it earns it (D9). You write findings in English (D61). You find research through PubMed, ClinicalTrials.gov, OpenFDA, journals, Google Scholar — and leads through Reddit and podcasts (D8).

**Future users** are a maybe (D2). The design keeps that door open only where it's free (D7): an account-ready data model, per-user API keys, no hard-coded single user. Because there is no server, more users would cost nothing — each runs their own copy.

## Positioning

The **personal evidence map** (D5): your saved papers and findings, connected by typed links you made, with evidence information visible on every node, in a map that keeps its shape and grows as you understand more.

| Existing product | What it does | Why BioVault is different |
|---|---|---|
| R Discovery | Mobile feed of papers from chosen interests, library, summaries | No personal graph; feed-first, not memory-first |
| ResearchRabbit / Litmaps / Connected Papers | Citation/similarity maps from seed papers | Maps of *citations*, not of *your* typed connections between findings |
| Consensus | AI search + synthesis over 200M+ papers | Question-answering, not a personal library |
| Examine | Graded evidence for supplements & interventions | Editorial database, not your vault |

## Goals (v1 = Phases 1–2)

1. Every paper you read can be saved, processed and found again in under a minute.
2. Findings, takeaways and typed links between papers/findings, visible on a map.
3. "Related" suggestions that surface your earlier papers while you process a new one.
4. One vault on desktop and phone, fully offline-capable, syncing over the home network with **zero lost edits**.
5. €0 running cost; nothing but searches and lookups leaves your devices.

## Non-goals

- Cloud sync, accounts, hosting, VPN access (D42, D46).
- Built-in generative AI (D40). Optional bring-your-own-model connector comes later (D48).
- Sharing, social features, collaboration (D7).
- iOS (D44) — only if BioVault goes public.
- Pre-built biohacking taxonomy (D16); approved-journal lists (D24).
- Browser extension; unprompted resurfacing (D57 — dropped altogether).

## Anti-goals — failure even if it technically works

- **It becomes a bookmark graveyard.** If papers pile up unprocessed, the original problem is back. (Guarded by Save → Inbox → Process and faint dots, D59.)
- **It thinks for you.** AI-written takeaways would quietly bring back the forgetting (D41).
- **It loses your work.** A sync bug that drops an edit is the worst possible failure (D69, mandatory simulator).
- **It tells people what to do.** Research only (D22).
- **It never ships.** Building infrastructure forever without using the app (F10). Guarded by milestone M1: desktop vault in daily use before sync work begins (D77).

## Success measures (D78)

Measured by the app itself, shown in **Settings → Stats** (my call):

| Measure | What it tells you | Healthy direction |
|---|---|---|
| Share of saved papers processed | Whether the Inbox is becoming a graveyard | High and stable |
| Links per processed paper | Whether you're connecting, not just collecting | Rising over time |
| "Related" suggestions accepted ÷ shown | Whether suggestions are useful (and training data for your link model) | Rising once the link model lands |
| Lost or duplicated edits across syncs | Sync correctness | **Always zero** (simulator + real use) |

## Constraints

| Constraint | Source |
|---|---|
| Windows desktop + Android phone, one codebase | D18, D44, D68 |
| Local app, no hosting, €0/month | D42 |
| Sync only on the local network, equal peers | D45, D46 |
| Findings in English | D61 |
| Source-available, all rights reserved; avoid GPL/AGPL dependencies | D62, D63 |
| No timeframe; limited, variable weekly hours; enjoyment first | D76 |
| Heavy use of AI coding agents | D77 |
