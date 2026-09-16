<a href="https://www.warp.dev">
    <img width="1024" alt="Warp Agentic Development Environment product preview" src="https://github.com/user-attachments/assets/9976b2da-2edd-4604-a36c-8fd53719c6d4" />
</a>

<p align="center">
  <a href="https://www.warp.dev">Website</a>
  ·
  <a href="https://www.warp.dev/code">Code</a>
  ·
  <a href="https://www.warp.dev/agents">Agents</a>
  ·
  <a href="https://www.warp.dev/terminal">Terminal</a>
  ·
  <a href="https://www.warp.dev/drive">Drive</a>
  ·
  <a href="https://docs.warp.dev">Docs</a>
  ·
  <a href="https://www.warp.dev/blog/how-warp-works">How Warp Works</a>
</p>

> [!NOTE]
> OpenAI is the founding sponsor of the new, open-source Warp repository, and the new agentic management workflows are powered by GPT models.

<h1></h1>

## About

[Warp](https://www.warp.dev) is an agentic development environment, born out of the terminal. Use Warp's built-in coding agent, or bring your own CLI agent (Claude Code, Codex, Gemini CLI, and others).

> [!WARNING]
> **This repository is an unofficial local-only fork** of the AGPL Warp client ([ThePeppy/warp](https://github.com/ThePeppy/warp)). It is not affiliated with Warp.dev. Official Warp server, hosted auth, Warp Drive backend, and Oz remain proprietary and are **not** in this repo. This fork keeps local terminal features and skips Warp account login / paid membership. Keep the AGPL and MIT license files when redistributing.

## Local-only macOS fork

This fork compiles with the `local_only` cargo feature so the app starts as a
local terminal **without** Warp account login, registration, or a paid plan.

`local_only` enables `skip_login` (an existing compile-time hook that installs
an onboarded local user and fails authenticated Warp cloud requests) and hides
account / billing / upgrade / hosted-AI UI. Cloud-only features degrade: they
are hidden, disabled, or no-op — they must not block a local shell.

### What still works locally

- Local shells, tabs, panes, and session restoration
- Local settings, themes, keybindings, and the settings file
- Local completions that do not need Warp cloud
- Local MCP servers and third-party CLI agents you run yourself (Claude Code, Codex, Gemini CLI, …)

### What is hidden or no-op

- Sign in / Sign up / Log out / Upgrade / Manage subscription / billing
- Warp Drive cloud sync, teams, referrals, shared blocks
- Hosted Warp Agent / Oz / cloud environments (they require a Warp account)

Residual cloud code still exists in the tree (GraphQL clients, Firebase hooks,
telemetry types). In `local_only` builds those calls fail closed instead of
prompting for credentials. Do not invent Warp server credentials.

### Building with `local_only`

`./script/run` and the OSS bundle scripts enable `local_only` automatically:

```bash
./script/bootstrap   # once, on a Mac with Xcode (not required if you use CI)
./script/run         # cargo features include local_only
```

If you invoke cargo yourself:

```bash
cargo run --bin warp-oss --features gui,local_only
# or, to produce a .app on macOS:
./script/macos/bundle --channel oss --arch aarch64 --selfsign --features local_only
```

To build the official login-gated client from this tree, omit `local_only`
(and do not use `./script/run` as-is).

### GitHub Actions: Apple Silicon artifact (no local Xcode)

You do **not** need Xcode on your personal Mac. Use the workflow
[`.github/workflows/build-macos-arm64.yml`](.github/workflows/build-macos-arm64.yml):

1. On GitHub: **Actions → Build macOS Apple Silicon (local-only) → Run workflow**.
2. Leave the profile at `release-lto` (or pick `dev` for a faster unsigned debug build).
3. When the job finishes, download the `WarpOss-macos-arm64` artifact (zipped `.app`).
   A `.dmg` is uploaded as `WarpOss-macos-arm64-dmg` when `create-dmg` succeeds.
4. Optional: set **create_github_release** and a **release_tag** to attach the
   same files to a GitHub Release.

The workflow runs on `macos-15` (Apple Silicon), calls the repo's
`script/macos/bundle` with `--channel oss --arch aarch64 --selfsign`, and does
not need Apple Developer certificates. Official notarization secrets are unused.

### Opening an unsigned / ad-hoc signed app on macOS

Gatekeeper will block the first launch of an app that is not Developer ID
signed and notarized:

1. Unzip the artifact (or open the DMG) and copy `WarpOss.app` to `/Applications`
   or another folder you control.
2. **Right-click** `WarpOss.app` → **Open** → confirm **Open**.
3. Or: System Settings → Privacy & Security → Open Anyway.

`xattr -d com.apple.quarantine WarpOss.app` also clears the download quarantine
if you trust the artifact you just built.

## Installation

You can [download Warp](https://www.warp.dev/download) and [read our docs](https://docs.warp.dev/) for platform-specific instructions.

## Licensing

Warp's UI framework (the `warpui_core` and `warpui` crates) are licensed under the [MIT license](LICENSE-MIT).

The rest of the code in this repository is licensed under the [AGPL v3](LICENSE-AGPL).

## Open Source & Contributing

Warp's client codebase is open source and lives in this repository. We welcome community contributions and have designed a lightweight workflow to help new contributors get started. For the full contribution flow, read our [CONTRIBUTING.md](CONTRIBUTING.md) guide.

### Issue to PR

Before filing, [search existing issues](https://github.com/warpdotdev/warp/issues?q=is%3Aissue+is%3Aopen+sort%3Areactions-%2B1-desc) for your bug or feature request. If nothing exists, [file an issue](https://github.com/warpdotdev/warp/issues/new/choose) using our templates. Security vulnerabilities should be reported privately as described in [CONTRIBUTING.md](CONTRIBUTING.md#reporting-security-issues).

Once filed, a Warp maintainer reviews the issue and may apply a readiness label: [`ready-to-spec`](https://github.com/warpdotdev/warp/issues?q=is%3Aissue+is%3Aopen+label%3Aready-to-spec) signals the design is open for contributors to spec out, and [`ready-to-implement`](https://github.com/warpdotdev/warp/issues?q=is%3Aissue+is%3Aopen+label%3Aready-to-implement) signals the design is settled and code PRs are welcome. Anyone can pick up a labeled issue — mention **@oss-maintainers** on an issue if you'd like it considered for a readiness label.

### Building the Repo Locally

To build and run Warp from source:

```bash
./script/bootstrap   # platform-specific setup
./script/run         # build and run Warp
./script/presubmit   # fmt, clippy, and tests
```

See [WARP.md](WARP.md) for the full engineering guide, including coding style, testing, and platform-specific notes.

## Joining the Team

Interested in joining the team? See our [open roles](https://www.warp.dev/careers).

## Support and Questions

1. See our [docs](https://docs.warp.dev/) for a comprehensive guide to Warp's features.
2. Join our [Slack Community](https://go.warp.dev/join-preview) to connect with other users and get help from the Warp team.
3. Try our [Preview build](https://www.warp.dev/download-preview) to test the latest experimental features.
4. Mention **@oss-maintainers** on any issue to escalate to the team — for example, if you encounter problems with the automated agents.

## Code of Conduct

We ask everyone to be respectful and empathetic. Warp follows the [Code of Conduct](CODE_OF_CONDUCT.md). To report violations, email warp-coc at warp.dev.

## Open Source Dependencies

We'd like to call out a few of the [open source dependencies](https://docs.warp.dev/help/licenses) that have helped Warp to get off the ground:

* [Tokio](https://github.com/tokio-rs/tokio)
* [NuShell](https://github.com/nushell/nushell)
* [Fig Completion Specs](https://github.com/withfig/autocomplete)
* [Warp Server Framework](https://github.com/seanmonstar/warp)
* [Alacritty](https://github.com/alacritty/alacritty)
* [Hyper HTTP library](https://github.com/hyperium/hyper)
* [FontKit](https://github.com/servo/font-kit)
* [Core-foundation](https://github.com/servo/core-foundation-rs)
* [Smol](https://github.com/smol-rs/smol)
