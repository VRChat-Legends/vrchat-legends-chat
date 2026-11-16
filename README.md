<div align="center">

# VRChat Legends Chat

**[VRChat Legends](https://vrchatlegends.com) Messages in its own window — for Windows and Linux.**

Friends, groups, voice and video calls, bots and reactions, in a native app that signs you in with your VRChat Legends account.

</div>

---

## Download

Grab the installer for your system from the [latest release](https://github.com/VRChat-Legends/vrchat-legends-chat/releases/latest):

| System | File |
| --- | --- |
| **Windows** | `VRChat.Legends.Chat_x64-setup.exe` (installs for your user, no admin needed) |
| **Linux** (Debian / Ubuntu) | `.deb` |
| **Linux** (Fedora / RHEL) | `.rpm` |
| **Linux** (anything) | `.AppImage` — mark it executable and run it |

The app **updates itself**: it checks for a newer release on launch, installs it, and asks you to restart.

## What it is

A small Rust app ([Tauri 2](https://tauri.app)) that opens `vrchatlegends.com/messages` in its own window. The site recognises the app and shows only the chat — no site header, banners or footer. Links that leave the chat (someone's profile, an outside site, a link in a message) open in your normal web browser.

You sign in the usual way: VRChat Legends password, or Discord, GitHub or Twitch. (Google may refuse to sign in inside apps; use one of the others.)

## Auto-updates

Updates are served from this repo's GitHub releases and are **cryptographically signed** — the app only installs an update signed with the project's private key, so a fake or tampered update is rejected.

Rolling out a new version:

1. Bump `version` in [`src-tauri/tauri.conf.json`](src-tauri/tauri.conf.json).
2. Commit, then push a matching tag:
   ```bash
   git tag v0.1.1
   git push origin v0.1.1
   ```
3. GitHub Actions builds, signs and creates a **draft** release with the installers and `latest.json`.
4. Open the [Releases](https://github.com/VRChat-Legends/vrchat-legends-chat/releases) page and **publish** it. Everyone's app picks it up from there.

> The repository (or at least its releases) must be **public** so the app and its updates can be downloaded without signing in to GitHub.

### One-time setup for signing

The release workflow needs two repository secrets (**Settings → Secrets and variables → Actions**):

| Secret | Value |
| --- | --- |
| `TAURI_SIGNING_PRIVATE_KEY` | the updater private key |
| `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` | its password |

The matching **public** key is already in `tauri.conf.json`. Keep the private key safe — losing it means you can no longer ship updates that existing installs will accept.

## Building it yourself

Needs [Rust](https://rustup.rs) and Node 20+. On Windows also the "Desktop development with C++" tools from Visual Studio Build Tools; on Linux `libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf`.

```bash
npm install
npm run dev     # run it
npm run build   # make installers in src-tauri/target/release/bundle
```

App icons are made from `app-icon.png` with `npm run icons`.

## Layout

```
shell/                  loading page shown before the site loads
src-tauri/
  src/main.rs           the whole app: window, link handling, auto-update
  tauri.conf.json       app name, icons, bundle and updater settings
  capabilities/         window permissions
.github/workflows/      the build-and-release workflow
```

## License

See [LICENSE](LICENSE). This is proprietary software: the source is here so the project can be built and audited, but it is **not** open source — you may not copy, redistribute or make your own versions of it.
