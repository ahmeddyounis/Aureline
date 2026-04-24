# Rust workspace

## Row binding

- Archetype row id: `archetype_row:rust_workspace`
- Archetype id: `rust_workspace_self_host`
- Initial support class: `supported`
- Target support class: `certified`
- Inclusion target: `foundations`
- Compatibility row: `compat_row:certification.launch_archetype_matrix`
- Skew register: `skew_register:certification.launch_archetype_matrix`

## Representative stack

Cargo workspace with declared crates, in-repo tests, clippy, and
rustfmt against the repo-pinned toolchain. The reference workspace
slices the live repo so the corpus tracks real workspace shape rather
than a vendored copy.

## Required-mode rationale

- `local_only` — Cargo build, test, debug, rename, and large-workspace
  indexing are fully covered on a developer machine against the
  repo-pinned toolchain.

## Evidence already on file

- Reference workspace: `refws.small_rust_self_host_slice`
  ([fixture](../../workspaces/reference/small_rust_self_host_slice.json)).
- Corpus scenarios:
  `workflow.startup_rust_self_host_slice`,
  `workflow.first_useful_edit_rust_self_host`,
  `vfs.rust_workspace_enumeration`.
- Toolchain pin: `rust_toolchain_pinned_v1` from the repo-pinned
  toolchain in `rust-toolchain.toml`.

## Open evidence questions

- Stand up a certified-archetype report that names hardware
  (`ref.arm64.macos15.apple_silicon_14in` and at least one Linux
  laptop), the toolchain pin, and the platform-profile row.
- Publish a claim-manifest row for the Rust workspace before any
  top-level "supports Rust workspace" wording is admissible.
- Decide whether the certified row requires Windows coverage at first
  stable; the inventory currently lists `windows_x86_64` in
  `platform_dimensions` but no Windows-specific corpus scenario is
  seeded.
