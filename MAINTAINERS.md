# Maintainers

Notes for the people who build and release VRChat Legends Chat. Users do not need this.

## What it is

A small Rust app (Tauri 2) that opens `vrchatlegends.com/messages` in its own window. The site recognises the app (it adds `VRCLDesktop` to the browser name and sets `window.__VRCL_DESKTOP__`) and shows only the chat. Links that leave the chat open in the normal web browser. See `src-tauri/src/main.rs`.

## Build on your own machine

Needs [Rust](https://rustup.rs) and Node 20+. On Windows also the "Desktop development with C++" tools from Visual Studio Build Tools; on Linux `libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`.

```bash
npm install
npm run dev     # run it
npm run build   # make installers in src-tauri/target/release/bundle
```

App icons are made from `app-icon.png` with `npm run icons`.

## Release a new version

1. Bump `version` in `src-tauri/tauri.conf.json`.
2. Commit, then push a matching tag:
   ```bash
   git tag v0.1.1
   git push origin v0.1.1
   ```
3. GitHub Actions builds, signs and creates a draft release with the installers and `latest.json`.
4. Open the Releases page and publish it. Everyone's app updates to the latest published, non-prerelease release.

The repository (or at least its releases) must be public so the app and its updates can be downloaded without signing in to GitHub.

## Signing secrets (one time)

The release workflow needs two repository secrets (Settings, then Secrets and variables, then Actions):

| Secret | Value |
| --- | --- |
| `TAURI_SIGNING_PRIVATE_KEY` | the updater private key |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | its password |

The matching public key is already in `tauri.conf.json`. Keep the private key safe: losing it means existing installs will no longer accept updates.

## Layout

```
shell/                  loading page shown before the site loads
src-tauri/
  src/main.rs           the whole app: window, link handling, auto-update
  tauri.conf.json       app name, icons, bundle and updater settings
  capabilities/         window permissions
.github/workflows/      the build and release workflow
```
