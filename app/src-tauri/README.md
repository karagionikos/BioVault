# app/src-tauri: the app crate

The Tauri application crate: the binary that ships on Windows and Android. It's a member of the root Cargo workspace (D80) and depends on [`biovault-tauri`](../../core/biovault-tauri/README.md). It holds configuration and bundling, not logic.

## Planned contents

| What | Notes |
|---|---|
| `Cargo.toml`, `src/main.rs`, `src/lib.rs` | Entry points. `lib.rs` carries the mobile entry point and registers `biovault-tauri`'s commands. Set `publish = false` |
| `tauri.conf.json` | App identity, windows, bundle settings (Windows NSIS, Android) |
| `capabilities/` | Tauri 2 permission files. Grant only what the interface needs |
| `icons/` | App icons, generated with `tauri icon` once the design exists |
| `gen/android/` | Android project written by `tauri android init`. **Commit it**: you edit its Gradle files, e.g. `app/build.gradle.kts` for release signing |
| `gen/schemas/` | Generated capability schemas. Git-ignored |

## Platform pieces to verify (Spike 0 / P2)

| Need | Plan | Ref |
|---|---|---|
| QR scanning on the phone | Tauri's official barcode-scanner plugin | [05](../../docs/05-sync-spec.md#discovery--pairing) |
| Opening links | Tauri opener plugin → system browser | [04](../../docs/04-architecture.md#platform-specifics) |
| mDNS discovery on Android | Hold a `WifiManager.MulticastLock`, likely via a small Kotlin plugin | [04](../../docs/04-architecture.md#platform-specifics) |
| Receiving URLs shared from other apps | Android share-target intent, likely via a small Kotlin plugin | [02 · F-20](../../docs/02-features-and-flows.md#phase-2--android--sync-milestone-m2-one-vault-two-devices) |

Where those Kotlin plugins live is still open. Decide when Spike 0 needs the first one.

## Never commit

- **The release keystore** (`*.jks`) and **`gen/android/keystore.properties`**, which holds its password. Tauri's signing setup reads that file; keep the keystore itself outside the repo, with an offline backup (F25). The root `.gitignore` blocks both as a safety net.
- Build outputs: `.apk`, `.aab`, installers.
