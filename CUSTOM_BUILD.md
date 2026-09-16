# Local-first fork (offline / no Warp account)

This repository is a modified [AGPL](LICENSE-AGPL) fork of the open-source Warp *client*. Warp's servers, hosted auth, Drive backend, and Oz orchestration are **not** in this repo and remain proprietary. This fork does **not** reimplement those APIs or invent credentials.

It strips client-side account, login, and billing gates so the local terminal works without a Warp membership.

## What works locally

- App launch straight into the terminal (no login / signup wall)
- Local shells, panes, tabs, editor, keybindings, and other on-device features
- No subscription, trial, or upgrade nag UI for core local use

## What still needs Warp servers (hidden or no-op)

These cannot be reimplemented from this client tree. They are disabled or fail-open rather than bricking the UI:

- Warp account login, SSO, Firebase anonymous users
- Subscription / billing / request credits
- Warp Drive cloud sync
- Hosted Warp AI (agent harness that talks to `app.warp.dev`)
- Oz / cloud agents / cloud environments
- Settings sync, referrals, teams, Stripe portal

You can still run third-party CLI agents (Claude Code, Codex, Gemini CLI, …) *inside* the local terminal; those use *your* tooling, not Warp's hosted backend.

## Local-offline flag

Mode is on by default for the `warp-oss` / OSS channel of this fork.

| Switch | Effect |
|--------|--------|
| Cargo feature `local_offline` | Compile-time enable (wired into OSS bundles) |
| Feature flag `LocalOffline` | Enabled by `app/src/bin/oss.rs` for OSS |
| `WARP_LOCAL_OFFLINE=1` | Force on |
| `WARP_LOCAL_OFFLINE=0` | Force off (restores production `app.warp.dev` URLs; login walls return) |

When on, the client:

1. Installs a synthetic local user (`local@localhost`) so the UI treats you as signed in
2. Points GraphQL / RTC / Oz URLs at `127.0.0.1:0` so missing servers fail immediately instead of calling Warp production
3. Skips Firebase, user refresh, Drive/workspace pollers, and login-gated modals
4. Hides billing, teams, referrals, Drive, Oz environment, and upgrade UI

Do **not** point this build at Warp production auth or invent tokens. That is out of scope and not supported.

## Hardening (round 2)

`local_offline` remains the single switch. When it is on, leftover production clients are redirected, skipped, or no-op’d so a missing Warp server cannot brick the terminal or revive a login wall.

### Disabled when `local_offline` is on

| Area | Behavior |
|------|----------|
| Auth / billing GraphQL | `ChannelState` getters return `http://127.0.0.1:0` (and empty Firebase key) |
| RTC / Drive websocket | `ws://127.0.0.1:0/...`; listener never starts |
| Oz / cloud agents | `http://127.0.0.1:0`; launch / credit / upgrade modals skipped |
| Settings sync | Cloud preference retry loop and `sync()` no-op |
| Telemetry / Rudderstack | Collector does not start; batches are dropped; destinations empty |
| Sentry / crash reporting | Init skipped; DSN empty |
| Auto-update / force-update | Polling skipped; releases URL empty; menu items hidden |
| Account / upgrade UI | Avatar menu, Refer a Friend, invite CTAs, billing banners, credits alerts hidden or no-op |

`ChannelState::new()` also rewrites production channel configs (dev/preview/stable binaries) when `WARP_LOCAL_OFFLINE=1`, so the env override is enough even if you are not running `warp-oss`.

Network failures fail open: GraphQL/HTTP errors against the dead-end do not log you out or show a signup wall. The synthetic local user stays signed in.

Branding (AGPL-safe): About, window title, and the app menu use the label **Warp OSS local**. `LICENSE-AGPL`, copyright notices, and `THIRD_PARTY_LICENSES` are unchanged. This is a modified Warp client, not an official Warp release.

### Remaining network I/O (intentional)

These are **not** Warp production auth/billing/telemetry:

- User-initiated help links (`docs.warp.dev`, privacy policy, Slack community) if you click them
- Third-party CLI agents you run *inside* the local terminal (Claude Code, Codex, …) using *your* tooling
- Local filesystem, shells, fonts, and package-manager checks (for example Homebrew) that do not call `app.warp.dev`
- MCP OAuth you configure yourself (no bundled Warp MCP client secrets in OSS)

If a leftover caller still builds a URL, getters force it to `127.0.0.1:0` rather than `app.warp.dev`.

### Restoring production contact

```bash
WARP_LOCAL_OFFLINE=0 ./script/run
```

That turns the flag off: GraphQL/RTC/Oz point at Warp production again, login walls return, and telemetry/Sentry/autoupdate follow the channel config. Do this only if you intend to use official Warp servers.

## Build a macOS Apple Silicon app without installing Xcode

GitHub-hosted `macos-15` runners have Xcode. You only need a browser.

1. Open the repo on GitHub → **Actions** → **Build macOS arm64**
2. Click **Run workflow** and select this branch (optional profile: `release` default, `release-lto` heavier, `dev` fastest/debug)
3. When the run finishes, download the **WarpOss-macos-arm64-unsigned** artifact
4. Unzip `WarpOss-macos-arm64-unsigned.zip` to get `WarpOss.app`

The workflow file is [`.github/workflows/build-macos-arm64.yml`](.github/workflows/build-macos-arm64.yml). It also runs on pushes to `master`.

### Gatekeeper (unsigned / ad-hoc)

The first cut is **unsigned**. macOS will warn that the app is from an unidentified developer.

Workarounds (pick one):

```bash
# Clear quarantine after unzipping
xattr -cr /path/to/WarpOss.app
open /path/to/WarpOss.app
```

Or Finder: Control-click `WarpOss.app` → **Open** → confirm.

You need an Apple Developer ID to notarize; this fork does not require it.

### Limitations of the CI artifact

- Apple Silicon (`arm64`) only — not Intel
- Unsigned: no notarization, no official auto-update
- Default profile is `release` (not `release-lto`) so GitHub-hosted runners stay within time/memory. The app is optimized but not fully LTO-thin.
- Full macOS link only works on the Actions runner (or a Mac with Xcode). This Linux cloud VM cannot produce the `.app`.

## Build from source on a Mac

```bash
./script/bootstrap
# OSS binary defaults to local-offline
./script/run
# Or a distributable unsigned bundle:
./script/bundle --channel oss --arch aarch64 --nouniversal --nosign
```

`script/run` uses `warp-oss` when the private `warp-channel-config` tool is unavailable (typical for this public fork).

## License

This is a modified version of Warp's AGPL-licensed client. Keep `LICENSE-AGPL`, `LICENSE-MIT`, and bundled `THIRD_PARTY_LICENSES.txt`. Do not remove copyright notices.
