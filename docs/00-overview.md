# 00 · Overview

## One paragraph

You read research, and what you read doesn't stick: sometimes you write it down, sometimes you don't, and either way it fades. BioVault is the place where a paper you read becomes something you keep — **the finding, your takeaway in your own words, and how it connects to other papers you've read**. It searches PubMed, Europe PMC and OpenAlex in one box, lets you save papers into an Inbox, and asks you to *process* them (write a finding and takeaway, pick a category). Processed papers become full dots on a map that grows with what you've understood. Typed links — *supports, contradicts, same mechanism, extends, related* — turn that map into a personal evidence map.

## What BioVault is — and isn't

| BioVault is | BioVault is not |
|---|---|
| A personal memory tool for research | A reference manager clone (Zotero already exists) |
| Local-first: your data lives on your devices | A cloud service; there is no server |
| One app on Windows desktop + Android phone, equal peers | A phone-only or desktop-only app |
| A research tool | A health coach, protocol generator or self-experiment tracker (D22) |
| Free to run (€0/month) | Dependent on paid AI APIs (D40, D42) |
| Built for one user first, with the door open to more | A social or sharing platform |

## Principles

These are the rules that decide close calls. If a future feature breaks one, the feature loses.

1. **Writing your takeaway is the point.** AI never writes your takeaway (D41). It may, later and only if you connect your own model, draft an *objective key result* — never the takeaway.
2. **The map grows with understanding, not bookmarks.** Unprocessed papers are faint; processing makes them full (D59).
3. **Research only.** No advice, no protocols, no self-experiment features (D22).
4. **Your data, your devices.** No hosting, no telemetry, nothing leaves the device except searches and paper lookups to the public databases (D42, D65).
5. **Nothing is filed silently.** Categories, links and suggestions are always proposed and confirmed by you (D31, D21).
6. **You own the exit.** Everything exports to open formats: JSON, Markdown with `[[links]]`, BibTeX/RIS (D53).
7. **Building is the fun part.** Where it's reasonable, choose the path that's worth learning — but never at the cost of an app you actually use (D14, D17).

## Glossary

| Term | Meaning |
|---|---|
| **Paper** | A research item found through search or added by identifier (journal article or preprint). The unit you save. |
| **Inbox** | Where saved-but-unprocessed papers wait. |
| **Processed** | A paper with ≥1 finding that has a non-empty takeaway **and** ≥1 category (D59). |
| **Finding** | Something a paper showed. A stack of blocks: a *key result* (objective), your *takeaway* (your words), optional notes; later sketch blocks (D67). |
| **Takeaway** | Why the finding matters to you, in your own words. Never AI-written. |
| **Link** | A typed connection between two papers, two findings, or a paper/finding and a lead. Types: supports, contradicts, same mechanism, extends, related (D21, D35). |
| **Lead** | A claim or link waiting for its paper — e.g. "podcast said X affects Y". Drawn differently, carries no evidence labels, resolves when a paper is linked (D27). |
| **Attachment** | A link (URL) or screenshot attached to a paper, finding or lead (D20, D27). |
| **Category** | A node in your own nested category tree; a paper can sit in several (D30). |
| **Evidence labels** | Study type, human/animal/in-vitro, peer-reviewed vs preprint, year, retraction/correction status; sample size when available (D24). |
| **Fingerprint** | The SPECTER2 embedding of a paper or finding — a vector of 768 numbers used for meaning-similarity (D39, D61). |
| **"Related" prompt** | Suggestions of previously saved papers/findings that relate to the one you're processing (D12, D59). |
| **Suggestion decision** | Your accept/dismiss on a suggestion; logged from day one to train your own link model (D66). |
| **Device / peer** | One installation (desktop or phone). Both are equal full replicas (D45). |
| **Operation (op)** | One recorded change in the sync log, stamped with a hybrid logical clock (D69). |
| **Spike 0** | The throwaway feasibility experiment before any feature work (D68). |
