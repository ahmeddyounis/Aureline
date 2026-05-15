# Shell bootstrap matrix smoke

Integration tripwire for the desktop shell bootstrap. Verifies that every
workspace crate `aureline-shell` integrates is still wired in, and that the
shell's headless bootstrap path commits an open→edit→save sequence against a
synthetic workspace without a display.

## What the matrix proves

1. **Headless bootstrap is alive.** Spawns the `aureline_shell` binary in
   `--headless-test-edit-save` mode against a tmpdir-only workspace, drives a
   real open→edit→save cycle through buffer, editor, history, commands, and
   vfs, and asserts the committed byte sequence on disk plus the
   `outcome: committed` record in the headless report.
2. **Every integrated crate is reachable.** For each crate listed under
   `aureline-shell`'s `[dependencies]`, the matrix performs a tight
   public-API touch (one `use aureline_x::...` line and one assertion). A
   crate removed from `aureline-shell/Cargo.toml` breaks the matching `use`
   line and fails the build.
3. **Manifest matches matrix.** `shell_dependency_manifest_matches_matrix`
   re-reads `aureline-shell/Cargo.toml` at test time and fails loudly if the
   manifest no longer declares a path dependency for a crate listed in
   `INTEGRATED_CRATES`. Mirrors the static `use` lines for catastrophic
   churn (e.g. a manifest reshuffle that accidentally drops a crate).

## Run

```bash
cargo test --test shell_bootstrap_matrix
```

The harness is registered as a `[[test]]` entry under
`crates/aureline-shell/Cargo.toml`, so it builds the shell binary as a
test-time prerequisite and exports `CARGO_BIN_EXE_aureline_shell` for the
headless invocation.

## Constraints honored

- **No GPU / no display.** The headless edit/save mode short-circuits
  `run_native_shell` before window or event-loop construction.
- **No writes outside tmpdir.** The synthetic workspace, the headless
  report, and the headless log root (which the shell derives from the
  report's parent directory) all live inside a single `tempfile::TempDir`
  per test invocation.

## When to extend

Add a crate to `INTEGRATED_CRATES`, add the matching `use aureline_x::...`
line, and add a `touches_aureline_x` test the moment `aureline-shell`
gains a new workspace dependency. The matrix is the source of truth for
the integrated set.
