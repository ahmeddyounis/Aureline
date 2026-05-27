# Harden environment-capsule resolution — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
environment-capsule resolution truth packet. The cross-tool boundary
schema lives at
[`schemas/runtime/harden_environment_capsule_resolution_truth.schema.json`](../../../schemas/runtime/harden_environment_capsule_resolution_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-runtime/src/harden_environment_capsule_resolution/`](../../../crates/aureline-runtime/src/harden_environment_capsule_resolution/),
and the checked-in stable packet at
[`artifacts/runtime/m4/harden_environment_capsule_resolution_truth_packet.json`](../../../artifacts/runtime/m4/harden_environment_capsule_resolution_truth_packet.json).

The packet pins one boundary truth that the editor run surface,
terminal pane, task panel, CLI/headless inspector
(`aureline env inspect`), Project Doctor, support export, release proof
index, Help/About proof card, and the conformance dashboard all read.
Surfaces MUST NOT mint local copies, fork their own capsule semantics,
or paraphrase capsule posture; they project the packet verbatim.

## Track invariant

Capsules, templates, and prebuilds are accelerators and descriptors,
not hidden authorities. Declarative inputs (devcontainer, Nix, Compose,
shell/SDK descriptors, and Aureline environment manifests) resolve into
one typed environment-capsule object whose identity, fingerprint, and
invalidation reasons are inspectable. No stale environment may present
itself as current truth.

## Lanes (closed vocabulary)

- `devcontainer_lane` — `.devcontainer/devcontainer.json`,
  `Dockerfile`, and OCI refs consumed by the devcontainer build.
- `nix_lane` — `flake.nix`, `shell.nix`, and pinned channels.
- `compose_lane` — `compose.yaml` / `docker-compose.yaml` services
  and startup ordering.
- `shell_sdk_lane` — `.tool-versions`, `.python-version`, `.nvmrc`,
  login shell descriptors, and SDK manager descriptors.
- `template_prebuild_lane` — Aureline environment manifests,
  requested templates, and prebuild artifacts.

Adding or removing a lane is a vocabulary change that requires bumping
the schema and updating the Rust contract, the artifact, the fixture
corpus, and this document together.

## Row classes (closed vocabulary)

- `capsule_resolution_quality` — the lane headline. Required at
  `launch_stable` for any lane that claims the M4 grade.
- `capsule_field_admission` — one row per typed capsule field
  (`host_or_base_image_identity`, `target_plan`,
  `resolved_toolchain_locks`, `projected_environment_variables`,
  `secret_references`, `writable_mount_model`,
  `service_startup_ordering`, `trust_network_posture`, `provenance`).
  All nine required for any `launch_stable` lane.
- `prebuild_fingerprint_admission` — one row per fingerprint
  component (`commit_or_tree_identity`, `capsule_hash`,
  `platform_arch`, `policy_epoch`, `extension_lock_digest`,
  `critical_toolchain_digest`). All six required for any
  `launch_stable` lane.
- `invalidation_reason_admission` — one row per visible invalidation
  reason (`cold_path`, `partially_warm_path`, `fingerprint_mismatch`,
  `untrusted_template_metadata`, `blocked_hook`, `stale_prebuild`).
  All six required for any `launch_stable` lane.
- `materialized_identity_admission` — binds the requested
  template/capsule/prebuild artifact identity, the materialized
  runtime instance identity, and attests `no_silent_prebuild_reuse`.
  Required for every `launch_stable` lane and MUST surface non-empty
  `requested_artifact_identity_binding` and
  `materialized_runtime_identity_binding` values.
- `project_doctor_finding_admission` — one row per Project Doctor
  finding code (`wrong_interpreter`, `stale_prebuild`,
  `blocked_activator`, `drifted_toolchain`,
  `untrusted_template_metadata`). All five required for any
  `launch_stable` lane.
- `known_limit`, `downgrade_automation` — disclosed gap rows. Each
  must carry its disclosure ref.

## Support classes (closed vocabulary)

`launch_stable` is the M4 grade. `launch_stable_below`,
`beta_grade_only`, `preview_only`, and `unsupported` are the precise
narrowed labels; each narrowed row MUST surface a disclosure ref.
`support_unbound` never qualifies for stable promotion.

## Typed capsule fields (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a
`capsule_field_admission` row for each of:

| field token | what the resolver produces |
|---|---|
| `host_or_base_image_identity` | Devcontainer base image, Compose service image, or host kernel descriptor. |
| `target_plan` | Which capsule + target + service ordering Aureline plans to materialize. |
| `resolved_toolchain_locks` | Interpreter version, SDK version, and package-manager pins. |
| `projected_environment_variables` | Typed env names + provenance (no raw values). |
| `secret_references` | Vault refs, OS keychain refs (never raw secret bodies). |
| `writable_mount_model` | Declared mounts, scopes, and write authority. |
| `service_startup_ordering` | Compose dependency graph and hook ordering as named refs. |
| `trust_network_posture` | Egress class, ingress class, trusted host list, declared network policy. |
| `provenance` | Why this capsule was chosen (source inputs, precedence ladder, hint origin). |

A missing field auto-narrows the lane below `launch_stable` with a
typed `missing_capsule_field_coverage` finding.

## Prebuild fingerprint (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a
`prebuild_fingerprint_admission` row for each of:

- `commit_or_tree_identity` — Git commit hash or repo tree hash.
- `capsule_hash` — Digest of the typed capsule object.
- `platform_arch` — OS family, CPU arch, libc.
- `policy_epoch` — Capability + trust envelope version.
- `extension_lock_digest` — Installed extension set lock.
- `critical_toolchain_digest` — Interpreter, SDK, and package
  manager digests.

Reuse may happen only when every component matches. A missing
component auto-narrows the lane below `launch_stable` with a typed
`missing_prebuild_fingerprint_coverage` finding.

## Visible invalidation reasons (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish an
`invalidation_reason_admission` row for each structured reason:

| reason token | meaning |
|---|---|
| `cold_path` | No matching prebuild; capsule materialized from scratch. |
| `partially_warm_path` | Partial match; warm capsule with drifted layers re-derived. |
| `fingerprint_mismatch` | Commit/tree, capsule hash, or platform/arch drift. |
| `untrusted_template_metadata` | Reuse refused until the template is reviewed. |
| `blocked_hook` | Repository hook would have to run; cold path runs only after reviewer approval. |
| `stale_prebuild` | Reused prebuild detected as stale; fallback to cold path with a visible reason. |

A missing reason auto-narrows the lane below `launch_stable` with a
typed `missing_invalidation_reason_coverage` finding. Repository hooks
are never executed merely because a descriptor exists.

## Requested vs. materialized identity

A `materialized_identity_admission` row MUST be present on every
`launch_stable` lane with non-empty
`requested_artifact_identity_binding` and
`materialized_runtime_identity_binding` values plus
`no_silent_prebuild_reuse: true`. Surfaces show both the requested
template/capsule/prebuild artifact identity (declared environment
intent) and the materialized runtime instance identity (where work
actually ran). Equating the two without the no-silent-reuse attestation
is refused; warmed snapshots are never treated as authoritative.

## Project Doctor finding codes (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a
`project_doctor_finding_admission` row for each finding code:

| finding token | meaning |
|---|---|
| `wrong_interpreter` | Wrong interpreter resolved (e.g. system Python instead of the declared pyenv version). |
| `stale_prebuild` | Stale prebuild reused on an outdated fingerprint. |
| `blocked_activator` | Activator was blocked by trust, policy, or capability. |
| `drifted_toolchain` | Drifted toolchain (interpreter, SDK, or package manager) on a reused capsule. |
| `untrusted_template_metadata` | Template metadata was untrusted; capsule selection refused. |

A missing finding auto-narrows the lane below `launch_stable` with a
typed `missing_project_doctor_finding_coverage` finding.

## Consumer projections (required)

Every packet MUST carry a projection for each of:

- `editor_run_surface`
- `terminal_pane`
- `task_panel`
- `cli_headless`
- `project_doctor`
- `support_export`
- `release_proof_index`
- `help_about`
- `conformance_dashboard`

Each projection MUST preserve the packet id and the nine vocabularies
verbatim (`preserves_lane_vocabulary`,
`preserves_row_class_vocabulary`,
`preserves_support_class_vocabulary`,
`preserves_capsule_field_vocabulary`,
`preserves_prebuild_fingerprint_vocabulary`,
`preserves_invalidation_reason_vocabulary`,
`preserves_project_doctor_finding_vocabulary`,
`preserves_known_limit_vocabulary`,
`preserves_downgrade_automation_vocabulary`,
`preserves_evidence_class_vocabulary`). A projection that collapses
any vocabulary auto-narrows the packet below `launch_stable`.

## Validator findings

The validator emits one or more findings (`info` / `warning` /
`blocker`) per gap. A `blocker` always demotes the packet to
`blocks_stable`; a `warning` demotes it to `narrowed_below_stable`.
The closed finding vocabulary covers missing identity, missing lane
coverage, missing capsule-field / prebuild-fingerprint /
invalidation-reason / project-doctor-finding coverage, missing
materialized-identity admission, unbound support / known-limit /
downgrade-automation / evidence bindings, missing or collapsed
disclosure refs, raw source material / secrets / ambient authority
leaks, missing or drifted consumer projections, silent prebuild reuse,
and promotion-state mismatch. See
[`mod.rs`](../../../crates/aureline-runtime/src/harden_environment_capsule_resolution/mod.rs)
for the full list.

## Auto-narrowing

When any required row is missing or any binding is unbound, the
packet is demoted automatically with a typed finding kind. This is the
honesty contract: no lane silently inherits adjacent green claims, no
warmed snapshot is treated as authoritative truth, and no surface
paraphrases capsule posture into free-form prose.

## See also

- Reviewer artifact: [`artifacts/runtime/m4/harden-environment-capsule-resolution.md`](../../../artifacts/runtime/m4/harden-environment-capsule-resolution.md)
- Generator: [`tools/regenerate_harden_environment_capsule_resolution_truth_packet.py`](../../../tools/regenerate_harden_environment_capsule_resolution_truth_packet.py)
- Alpha + beta capsule resolvers: [`crates/aureline-runtime/src/capsule_resolver/`](../../../crates/aureline-runtime/src/capsule_resolver/)
- Inspector parity truth packet: [`docs/runtime/m4/finalize-environment-and-toolchain-manager-parity-across-ui.md`](./finalize-environment-and-toolchain-manager-parity-across-ui.md)
- Execution-context resolver truth packet: [`docs/runtime/m4/stabilize-execution-context-resolver.md`](./stabilize-execution-context-resolver.md)
