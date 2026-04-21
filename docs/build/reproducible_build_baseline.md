# Reproducible-build baseline

This document is the authoritative description of what the Aureline workspace
build is expected to reproduce deterministically today, what is still
provisional, and how contributors and CI are expected to get to a clean
build from a fresh clone.

It is intentionally narrow: the goal is to establish a baseline so that
later benchmark, release, provenance, and clean-room work can consume a
stable foundation. Full release signing, SBOM emission, and clean-room
infrastructure are out of scope here.

## Goals

1. **One bootstrap command** takes a clean clone to a state where every
   seeded crate builds.
2. **One build command** produces the same artifact identity for the same
   `(commit, toolchain, target)` triple, modulo the provisional fields
   called out below.
3. **Every build emits a machine-readable identity record** so downstream
   tooling (provenance, crash symbolication, governance validators) can
   converge on one exact build identity, per the technical architecture
   requirement that the artifact graph point at a single build identity.

## Supported dev hosts

| Host                         | Target triple                    | Tier for the baseline                                                  |
|------------------------------|----------------------------------|------------------------------------------------------------------------|
| macOS 13+ on Apple silicon   | `aarch64-apple-darwin`           | Tier 1 — primary developer platform.                                   |
| macOS 13+ on Intel           | `x86_64-apple-darwin`            | Tier 1 — fully supported.                                              |
| Linux (glibc 2.31+) on x86_64| `x86_64-unknown-linux-gnu`       | Tier 1 — primary CI platform.                                          |
| Linux on aarch64             | `aarch64-unknown-linux-gnu`      | Tier 2 — builds must pass; developer tooling is best-effort.           |
| Windows 10/11 on x86_64      | `x86_64-pc-windows-msvc`         | Tier 3 / provisional — compile parity is a target; no determinism SLA. |

Tier-1 targets are covered by the pinned toolchain's `targets` list and are
expected to build from a clean clone. Tier-2 must build; Tier-3 (Windows)
is a best-effort compile-parity objective for the baseline and is excluded
from the determinism contract below until a Windows build lane is stood up.

## Pinned inputs

The pin for every build is the tuple of files below. Changing any of them
is a deliberate act and must update this document in the same change.

| File                                             | What it pins                                                                                |
|--------------------------------------------------|---------------------------------------------------------------------------------------------|
| [`rust-toolchain.toml`](../../rust-toolchain.toml) | Rust channel, profile, components (`rustfmt`, `clippy`, `rust-src`), and target matrix.     |
| [`.cargo/config.toml`](../../.cargo/config.toml)   | Workspace-level rustflags, net retry/fetch behavior, and future-incompatibility reporting.  |
| [`Cargo.toml`](../../Cargo.toml)                   | Workspace topology, `rust-version` (MSRV), lint profile, and release profile settings.      |
| [`Cargo.lock`](../../Cargo.lock)                   | Resolved dependency graph. Committed even while the workspace has no external deps.         |

Format and lint tool versions are pinned transitively: `rustfmt` and
`clippy` ship with the pinned toolchain channel, so bumping the channel is
the only supported way to bump them.

Benchmark helper tooling is intentionally not pinned yet. `aureline-bench`
is seeded empty; the first bench task to add a helper (e.g. `criterion`)
must add it here as a pinned input and in
`artifacts/governance/dependency_register.yaml` in the same change that
introduces the dependency.

## The one bootstrap command

From a clean clone:

```sh
./tools/build/bootstrap.sh
```

This script:

1. Verifies `rustup` is present (and refers the developer to
   <https://rustup.rs> if not).
2. Installs the pinned toolchain channel and required components
   (`rustfmt`, `clippy`, `rust-src`).
3. Checks that the active `rustc --version` matches the pin.
4. Runs `cargo fetch --locked` so the dependency graph in `Cargo.lock` is
   materialized. With no external deps today this is effectively a no-op,
   but it becomes load-bearing the moment a dependency is added.
5. Runs `cargo metadata --no-deps` as a smoke check that every seeded crate
   is resolvable and that the workspace manifest is well-formed.

Offline variant (`./tools/build/bootstrap.sh --offline`) skips the rustup
and `cargo fetch` steps for air-gapped mirrors. The caller is responsible
for pre-provisioning the toolchain and cache.

## The one build command

```sh
./tools/build/build.sh            # dev profile
./tools/build/build.sh --release  # release profile
```

CI runs the same command via `ci/build.sh`, which only adds deterministic
environment exports (`TZ=UTC`, `LC_ALL=C`, `CARGO_TERM_COLOR=never`,
`CARGO_NET_RETRY=3`, `SOURCE_DATE_EPOCH` pinned to the commit time) and
redirects the build identity record to an artifact directory.

Every invocation:

- Exports `SOURCE_DATE_EPOCH` (defaulting to `git log -1 --pretty=%ct`) so
  any embedded build timestamp is derived from the commit and not from
  wall-clock time. This matches the release-engineering rule in the PRD
  that build pipelines should record `SOURCE_DATE_EPOCH`-style timestamp
  normalization as part of provenance.
- Builds the workspace with `cargo build --locked --workspace --all-targets`
  so the lockfile is authoritative; a missing or stale `Cargo.lock` fails
  the build rather than silently regenerating it.
- Writes `target/{debug|release}/build_identity.json` conforming to
  [`schemas/build/build_identity.schema.json`](../../schemas/build/build_identity.schema.json).

## Build identity record

The record captures the small set of inputs needed to reattach a given
binary to a clean-room rebuild, a crash packet, or an SBOM entry later:

- `commit`, `commit_short`, `dirty`
- `toolchain_channel`, `rustc_version`, `cargo_version`
- `host_triple`, `target_triple`, `profile`
- `workspace_version`
- `source_date_epoch`, `build_timestamp_utc`

`print_build_identity.sh` is a separate script so governance tooling can
invoke it without running a full build.

## Determinism contract

For a given `(commit, toolchain_channel, target_triple, profile)` on a
Tier-1 host, rerunning `./tools/build/build.sh` must produce an identical
`build_identity.json` except for the fields explicitly marked provisional
below.

**Fixed (must match on rebuild):**

- `commit`, `commit_short`
- `toolchain_channel`, `rustc_version`, `cargo_version`
- `host_triple`, `target_triple`, `profile`
- `workspace_version`
- `source_date_epoch`, `build_timestamp_utc`
- `schema_version`

**Provisional (allowed to differ in M0):**

- `dirty` — only differs if the working tree is actually dirty; release
  lanes must reject dirty trees, dev builds tolerate them.
- Contents of compiled binaries under `target/` — byte-for-byte
  reproducibility of the binaries themselves is a later milestone; the
  baseline only guarantees identity-record reproducibility.
- Tier-3 (Windows) builds — the determinism contract does not apply until
  a Windows build lane is stood up.

## What is deliberately not in the baseline

- **Byte-identical artifact rebuild.** The baseline emits a stable
  identity, pins toolchains, and normalizes timestamps; it does not yet
  strip every source of non-determinism from compiled output (e.g. linker
  input ordering, build-script side effects). That is tracked under later
  clean-room / provenance work.
- **Release signing and transparency.** Out of scope; only the identity
  surface that a future signing pipeline will consume is established here.
- **SBOM and license review.** The workspace has no external dependencies
  yet; dependency-level provenance now lands through the dependency
  review policy and the canonical dependency/import registers when the
  first external crate or imported asset is introduced.
- **Remote build cache policy.** Deliberately absent: remote caches must
  not substitute for rebuildability, so the baseline does not even enable
  one.

## Upgrade discipline

Bumping the Rust toolchain, adding a format/lint tool, or introducing the
first benchmark helper all share the same discipline:

1. Change the pin (`rust-toolchain.toml`, `.cargo/config.toml`, or a new
   pinned tool manifest) in the same commit that updates this document.
2. Refresh `Cargo.lock` in the same commit if the dependency graph moves.
3. Note in the commit message why the bump is happening. Do not reference
   internal planning IDs.

Any future validation tool that wants to assert the baseline is intact
should read `rust-toolchain.toml`, `.cargo/config.toml`, `Cargo.lock`, and
the build identity record; those four files are the contract.
