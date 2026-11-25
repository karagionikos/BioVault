# AGENTS.md: BioVault

BioVault is a personal research vault: one Tauri 2 app (Rust core + React/TypeScript interface) for a Windows desktop and an Android phone, offline-first, with peer-to-peer LAN sync. The full design lives in the blueprint in [`docs/`](docs/README.md).

**Status:** blueprint done, no code yet. Next is Spike 0 on a throwaway branch (never merged into `main`), then P1 step 1.1, the real skeleton ([10](docs/10-roadmap.md)).

## The unit of work

One task = one roadmap step or one feature ID (`F-04`, `1.1`). Its completion criterion is the feature's **"done when"** in [02](docs/02-features-and-flows.md), plus the **Definition of done** in [09](docs/09-testing-and-release.md#definition-of-done-any-feature). Name branches, issues and PR titles after the ID (`F-04: …`).

Before writing code, read the feature in 02, the blueprint sections it links, and the `README.md` of every folder you edit (each lists that folder's rules and planned contents). Then read whichever of these your change touches:

| Touching | Read |
|---|---|
| Entities, fields, IDs, timestamps, deletion | [03 · Data model](docs/03-data-model.md) |
| Crate boundaries, commands/events, platform specifics | [04 · Architecture](docs/04-architecture.md) |
| `oplog`, HLC, sync, pairing, merge, the simulator | [05 · Sync spec](docs/05-sync-spec.md): **human-owned**, see below |
| Source clients, identity merge, evidence labels, categories | [06 · Sources & search](docs/06-sources-and-search.md) |
| Map, screens, theme, phone layout | [07 · Map & interface](docs/07-map-and-interface.md) |
| SPECTER2, "related" score, decision logging, `ml/`, `models/` | [08 · ML](docs/08-ml.md) |
| Tests, versions, builds, signing | [09 · Testing & release](docs/09-testing-and-release.md) |
| Why something is the way it is | [Decision log](docs/decision-log.md) (D-numbers), [ADRs](docs/decisions/) |

## Rules

1. **`biovault-core` stays free of Tauri.** `biovault-tauri` is the only bridge to the interface, and it holds no business logic (D68).
2. **One write path:** every data change goes `vault` → `oplog`. The interface calls core commands and re-queries when events arrive.
3. **Takeaways are human-written.** A `takeaway` block always has `ai_generated = false`, enforced in the core (D41). The full list of invariants is in [`core/biovault-core/README.md`](core/biovault-core/README.md#invariants-the-code-must-hold).
4. **Permissive dependencies only** (MIT, Apache-2.0, BSD, ISC, Zlib, MPL-2.0; no GPL/AGPL). Every new dependency passes the license checks and gets a row in [`THIRD_PARTY_NOTICES.md`](THIRD_PARTY_NOTICES.md). A license outside the policy is the owner's decision (D63).
5. **The repository is public.** Commit code, docs, fixtures and checksums. Model weights, signing keys, `.env` files, vault data, exports and Jupyter notebook outputs stay on the machine: clear notebook outputs before you commit. API keys come from device settings (D49).
6. **Tests use recorded fixtures**, never live APIs. Sync changes also pass the simulator (`core/sync-sim`).
7. **Interface:** English text, keyboard-usable core flows on the desktop, every visual value read from the one theme file (D75), TypeScript types generated from Rust (never edited by hand).
8. **Decisions are recorded.** A change that departs from the blueprint adds the next D-number to the decision log and updates the docs that cite it.

## Human-owned areas

The owner designs these and reviews every line (D77):

- sync merge rules, conflict detection and the simulator's assertions ([05](docs/05-sync-spec.md), `core/sync-sim`);
- link-model features, the evaluation protocol and the replace-or-not decision ([08](docs/08-ml.md), F-34);
- the takeaway rule above.

Here you draft and explain; the owner decides. Mark the PR **human-owned**, explain each change in plain words, and keep every assertion as written. When the simulator fails, report the seed (proptest keeps it under `proptest-regressions/`, which is committed) and stop.

## Gates

CI ([`.github/workflows/ci.yml`](.github/workflows/ci.yml)) is the source of truth. Each part switches on once its project file exists. Run the same commands locally before you hand work back:

```powershell
# repository root (once Cargo.toml exists)
cargo fmt --all --check
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo test --workspace --locked
cargo deny check licenses
```

```powershell
# app/ (once package.json exists; the scripts are listed in app/README.md)
npm run typecheck; npm run lint; npm run format:check; npm test; npm run licenses
```

## Environment

- The owner works on Windows with PowerShell and VS Code. Files use LF and UTF-8 ([`.editorconfig`](.editorconfig), [`.gitattributes`](.gitattributes)).
- Formatting: rustfmt ([`rustfmt.toml`](rustfmt.toml)) for Rust, Prettier + ESLint in `app/`. Markdown is not auto-formatted.
- TypeScript stays on `~6.0` until typescript-eslint supports TypeScript 7 (D82).
