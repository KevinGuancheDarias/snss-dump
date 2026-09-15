# AGENTS.md

Guidance for AI coding agents working in this repository.

## What this project is

`snss-dump` is a thin **read-only CLI** that decodes Chrome's SNSS session-restore
files (`User Data/<profile>/Sessions/Session_*`, `Tabs_*`) and prints the tabs
Chrome would restore as a Markdown table. The actual decoding (file framing,
record replay, nav-command parsing) lives in the external [`snss-core`
crate](https://crates.io/crates/snss-core) — this repo is *only* profile-dir
resolution, CLI arg handling, and Markdown rendering.

## Layout

- `src/main.rs` — the entire program. No modules, no tests.
- `Cargo.toml` — single dependency: package `snss-core` (its library crate is imported as `snss`). Rust edition 2024.
- Binary name: `snss-dump` (`target/release/snss-dump`).

## Build & run

```console
$ cargo build --release        # or: cargo run
$ ./target/release/snss-dump <chrome-user-data-or-profile-dir> [current|last|recently-closed|all]
```

No test suite, no lints config, no CI. Verify with `cargo build` (warnings:
`cargo build --release 2>&1`) and, if a real Chrome profile is available,
smoke-run it — this tool is feature-tested manually only.

## Hard invariants (do not break)

1. **Read-only.** Never write to the Chrome profile. Only open files, decode,
   print. Any feature must not create/modify/delete profile files.
2. **No output truncation.** Full URLs (with scheme) and titles must pass
   through verbatim; the Markdown escapes only `|` → `\|`.
3. **Thin wrapper.** Do not re-implement SNSS decoding here; use the
   `snss` crate API (the `snss-core` package on crates.io). Do not add
   dependencies. If the decoder needs fixing, fix it in the `snss-core`
   crate (separate repo).
4. **Exit codes are user-facing signal.** `0` = printed something,
   `1` = error/no-match, `2` = usage error. Preserve these.
5. **`current` is the default** source selection (`[current|last|recently-closed|all]`
   is optional).

## Conventions

- std-only outside `snss-core`; no `unwrap()`/`expect()` on I/O or user input
  (use `Result`/`eprintln!` + exit, matching existing style).
- Keep it single-file unless the program genuinely outgrows it.
- Errors are printed with `eprintln!` to stderr, results to stdout
  (so output is pipe-able).
- Rust edition 2024 idioms are fine.

## Do NOT do

- Don't add a test framework, CI, or dependency just to "improve" the repo.
- Don't add features (e.g. HTML/JSON output, watching the profile, parsing
  other Chrome data) without an explicit user request.
- Don't "clean up" the disclaimer or vibe-coded nature of the project in the
  README — it's intentional.
- Don't point it at untrusted directories when testing; treat profiles as
  local user data.
