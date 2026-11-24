# ADR-007 · Public, source-available, all rights reserved

**Status:** Accepted · 2026-11-23 · Decisions D62, D63, D64

## Context
You want the code visible on GitHub, but not open-source; you may go private later depending on goals.

## Decision
Public repository with **no license file**; the README carries a copyright notice and "No license is granted; all rights reserved." Keep the name **BioVault** for now.

## Consequences
- Default copyright applies, but GitHub's terms let other users **view and fork** a public repo on GitHub.
- Going private later doesn't recall existing public forks or local clones — anything published stays out there.
- **No GPL/AGPL dependencies** (they conflict with all-rights-reserved distribution); permissive dependencies keep their attribution notices (THIRD_PARTY_NOTICES).
- Name collision with OpenMined's genomics "BioVault" — revisit before any public release.
