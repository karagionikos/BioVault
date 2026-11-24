# ADR-003 · Tauri 2 + Rust core + React/TypeScript interface

**Status:** Accepted · 2026-11-24 · Decisions D18, D44, D50, D68

## Context
One codebase must target Windows desktop and Android. The hard logic (storage, sync, search merge, on-device ML) should exist once. You know React/TypeScript and Python; building is the fun part.

## Decision
**Tauri 2** app shell; **Rust core library** (`biovault-core`, no Tauri dependency) for all logic; **React + TypeScript** interface in the system WebView. A **Spike 0** proves Android viability, on-phone SPECTER2, LAN discovery and WebGL performance before any feature work.

## Alternatives considered
- **Flutter** (+ Rust core via flutter_rust_bridge): most mature single-UI option for Android + Windows; kept as the fallback interface.
- **Compose Multiplatform:** native Android feel; desktop on the JVM; new language; thinner graph tooling.
- **Electron + separate Android app:** two codebases; rejected.

## Consequences
- Familiar UI stack; Rust is new (deliberate learning).
- Tauri's mobile side is younger; some native bits may need small Kotlin plugins (e.g. multicast lock).
- Because the core is UI-independent, switching the interface to Flutter would keep most of the work.
