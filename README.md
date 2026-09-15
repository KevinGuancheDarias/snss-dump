# snss-dump

A small command-line tool that reads Chrome's modern session-restore files
(`User Data/<profile>/Sessions/`) and prints the tabs Chrome would restore —
as a Markdown table with full URLs — from a **stopped or running browser,
without touching the live profile**.

Chrome 125+ replaced the old text-ish `Last Tabs` / `Current Tabs` files with
a new binary format (the [`SNSS` magic](https://chromium.googlesource.com/chromium/src/+/main/components/sessions/core/command_storage_backend.cc),
"command stream" of length-prefixed records). The URLs and titles inside are
stored in the clear (UTF-8 URLs, UTF-16-LE titles), so a decoder can list them.

## What it does

Given a Chrome directory (any of):

```
.../User Data/Default            # profile dir
.../User Data                    # profile parent (single profile)
.../User Data/Default/Sessions   # the sessions dir itself
```

it resolves the `Sessions/` directory, decodes the newest `Session_*` file
("Current Session" — exactly what Chrome reopens with *On startup → continue
where you left off*), plus the previous `Session_*` ("Last Session") and the
newest `Tabs_*` file ("Recently Closed" — the closed-tab restore list), and
prints, per window:

```
| # | Title | URL |
|---|-------|-----|
| 1 | Some page title | https://example.com/path?query=full |
```

Full URIs including the scheme are preserved, nothing is truncated.

## Usage

```console
$ snss-dump <chrome-user-data-or-profile-dir> [current|last|recently-closed|all]

$ snss-dump "/mnt/c/Users/me/AppData/Local/Google/Chrome/User Data/Default" current
## Current Session — .../Default/Sessions/Session_134...
### Window 1649223459 (14)
| # | Title | URL |
...
```

- `current` (default): the session Chrome reopens on next start.
- `last`: the previous session (the superseded `Session_*` file).
- `recently-closed`: the `Tabs_*` file (tabs closed during the run, usable
  for "reopen closed tab").
- `all`: all of the above.

## Building

Rust, depends only on [`snss`](https://crates.io/crates/snss) (a panic-free
read-only SNSS decoder).

```console
$ cargo build --release
# binary: target/release/snss-dump
```

The decoder logic (file framing, record replay, nav-command decoding) is the
`snss` crate; this repo is a thin CLI around it plus profile-directory
resolution.

## Notes / limitations

- **Read-only**: the tool opens files, decodes them, and prints. It never
  writes to the profile.
- Decoding happens on file bytes as they are on disk; if Chrome is running
  and mid-write on Windows, a transient `UnreadableSource` warning can appear
  (the file is just reopened on the next run).
- Only the newest `Session_*` and `Tabs_*` files are meaningful; older
  rotated files are ignored.

## ⚠️ Disclaimer

> **This project is vibe-coded by AI with no human review.**
> It was written by an LLM agent and only **feature-tested** (it dumps the
> expected tabs on a real Chrome profile) — it has **not** been audited,
> fuzzed, or security-reviewed. Do not point it at untrusted or sensitive
> directories, do not trust its parsing blindly, and treat the output as
> best-effort. Use at your own risk.

The underlying `snss` decoder crate carries its own guarantees; this wrapper
is a convenience CLI and is untested beyond the smoke test described above.
