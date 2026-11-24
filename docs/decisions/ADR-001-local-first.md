# ADR-001 · Local-first: no hosting, €0 running cost

**Status:** Accepted · 2026-11-23 · Decisions D42, D45, D65

## Context
BioVault is a personal tool with no revenue plans. The vault holds private reading notes. A server would add cost, maintenance and a privacy surface.

## Decision
BioVault is a **local app**. All data lives in SQLite on each device. There is no server, no account and no telemetry. The only outbound traffic is searches and lookups to public paper databases, plus LAN sync between your own devices.

## Alternatives considered
- Hosted backend with free-tier sync (e.g. a BaaS): rejected — it's hosting, and it puts the vault on someone else's machine.
- Cloud-folder sync (Drive/iCloud/Syncthing): rejected in favor of direct LAN sync (see ADR-002).

## Consequences
- €0/month; privacy by construction; scaling to more users costs nothing (each runs their own copy).
- No access from outside the home network; no web version.
- Backups are your responsibility: manual export + 30-day Trash (D53).
