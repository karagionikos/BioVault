# biovault-core

The library: BioVault's data, rules and logic, shared by the Windows and Android builds. It's a plain Rust crate with **no Tauri dependency** (D68).

## Planned modules

From [04 · Core modules](../../docs/04-architecture.md#core-modules). Each becomes a module under `src/`.

| Module | Responsibility | Phase |
|---|---|---|
| `vault` | Create, update and delete entities; derive paper status; Trash and 30-day purge; enforce the rules below | P1 |
| `oplog` | Turn every mutation into an HLC-stamped operation; apply local and remote ops deterministically | P1 records · P2 exchanges |
| `sources` | Clients for PubMed, Europe PMC, OpenAlex, Crossref and Unpaywall: rate-limited, cached, polite | P1 |
| `identity` | Normalize identifiers; merge one paper's records from several sources | P1 |
| `labels` | Derive evidence labels from source metadata ("unknown" when unsure) | P1 |
| `search` | Vault search: keyword (SQLite FTS5) and meaning (fingerprints, compared in memory) | P1 |
| `ml` | Load the SPECTER2 variants; compute fingerprints; run the "related" score | P1 |
| `suggest` | Category suggestions and learned mappings; related suggestions; decision logging | P1 |
| `sync` | Discovery, pairing, encrypted sessions, protocol, blob transfer, version checks | P2 |
| `export` | JSON (full), Markdown with `[[links]]`, BibTeX/RIS; JSON import | P1 |

Specs: data in [03](../../docs/03-data-model.md), sync in [05](../../docs/05-sync-spec.md), sources and labels in [06](../../docs/06-sources-and-search.md), ML in [08](../../docs/08-ml.md).

## Invariants the code must hold

- IDs are UUIDv7. `created_at` and `updated_at` are hybrid-logical-clock stamps. Deletes are soft: `deleted_at` is set ([03](../../docs/03-data-model.md#base-rules-for-every-synced-entity)).
- Changes are field-level operations, never whole-entity overwrites.
- Paper status is **derived**: `processed` ⇔ at least one live finding with a non-empty takeaway **and** at least one category. It's recomputed locally and never synced as a decision.
- A `takeaway` block can never be AI-written (`ai_generated` must be false). This is enforced here, not in the UI (D41).
- A category move that would create a cycle is refused, both when you edit and when an op is applied.
- Every suggestion shown writes an append-only `suggestion_event` with a feature snapshot (D66).
- API keys come from device-local settings, never from code (D49).

## Tests that live here

- Unit tests: status derivation, the AI-takeaway ban, category cycles, identity merge, label derivation, HLC, op apply, and the JSON export round-trip.
- Source-client tests against recorded API responses (fixtures), never live servers ([09](../../docs/09-testing-and-release.md#test-layers-d75)).
