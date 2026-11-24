# app/: the Tauri 2 app

The one app that runs on the Windows desktop and the Android phone: a React + TypeScript interface in the system WebView, on top of the Rust core ([ADR-003](../docs/decisions/ADR-003-tauri-rust-react.md)).

| Folder | What lives there |
|---|---|
| [`src/`](src/README.md) | The interface: screens, the map renderer, the theme |
| [`src-tauri/`](src-tauri/README.md) | The app crate (a member of the root Cargo workspace): Tauri config, permissions, icons, Windows and Android bundling |

The files that make this folder a Vite + React + TypeScript project (`package.json`, `index.html`, `vite.config.ts`, `tsconfig.json`) arrive with Spike 0 on its branch, and with P1 on `main`.

## Rules

- **The interface never writes to the database.** It calls core commands and listens to events ([04](../docs/04-architecture.md#core--interface-boundary)).
- **One theme file.** Everything visual (colors, sizes, fonts, line styles, animation timings) reads from it, so your future design drops in without code changes (D75, [07 · Design brief](../docs/07-map-and-interface.md#design-brief)).
- **English interface.** The desktop's core flows (search, save, process, link) are fully usable by keyboard.
- **npm license check (D63).** Once `package.json` exists, mark it `"private": true, "license": "UNLICENSED"` and add a script that fails on any license outside the policy. This was tested on 2026-11-24: it rejects a GPL dependency and passes the planned front-end stack.

  ```json
  "scripts": {
    "licenses": "license-checker-rseidelsohn --production --excludePrivatePackages --onlyAllow \"MIT;Apache-2.0;BSD-2-Clause;BSD-3-Clause;ISC;Zlib;MPL-2.0\""
  }
  ```

  Install the checker as a dev dependency (`npm i -D license-checker-rseidelsohn`). `--excludePrivatePackages` skips BioVault's own unlicensed package. List every new dependency in [THIRD_PARTY_NOTICES.md](../THIRD_PARTY_NOTICES.md).

## Building and shipping

From [09 · Building & installing](../docs/09-testing-and-release.md#building--installing):

| Target | Output | Install |
|---|---|---|
| Windows | NSIS `-setup.exe` | Run the installer |
| Android | Signed release APK | Copy to the phone, or `adb install -r` over USB |

- No auto-updater in v1.
- **The Android release keystore never enters this repository.** Generate it once at the start of P2, keep it outside the repo, and keep an offline backup (F25). See [`src-tauri/`](src-tauri/README.md#never-commit).
