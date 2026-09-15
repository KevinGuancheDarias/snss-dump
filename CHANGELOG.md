# snss-dump changelog
v0.2.0-rc1 (latest) First CI-released build: tagged, cross-compiled, and zipped x64 artifacts for Windows and Linux.
==============================================================================================================================
* __New:__ GitHub Actions `Release` workflow (`.github/workflows/release.yml`) that fires on a pushed `v*` tag and builds native `x86_64-pc-windows-msvc` and `x86_64-unknown-linux-gnu` release binaries in a matrix
* __New:__ release packaging attaches one zip per platform as a release asset (`snss-dump-x86_64-pc-windows-msvc.zip`, `snss-dump-x86_64-unknown-linux-gnu.zip`), Linux binary stripped before zipping
* __New:__ `AGENTS.md` with project overview, build/run instructions, the hard invariants (read-only, no output truncation, thin-wrapper, stable exit codes), and the do-not list for agents working on this repo
* __New:__ `CHANGELOG.md` at the repo root (owge-style `vX.Y.Z (latest|date)` + `* __Type:__` bullets)

v0.1.0 (initial)
================
* __New:__ `snss-dump` — a thin read-only CLI that resolves a Chrome `User Data` / profile / `Sessions` directory and decodes the newest `Session_*`, the previous `Session_*`, and the newest `Tabs_*` SNSS files
* __New:__ prints the tabs Chrome would restore per window as a Markdown table (full, untruncated scheme-included URLs + titles), selectable via `current` (default) / `last` / `recently-closed` / `all`
* __New:__ decoding, record replay, and nav-command parsing delegated to the `snss-core` crate (imported as `snss`); this repo only adds profile-dir resolution, CLI args, and Markdown rendering
* __New:__ stable user-facing exit codes — `0` printed a match, `1` error / no source matched, `2` usage error
