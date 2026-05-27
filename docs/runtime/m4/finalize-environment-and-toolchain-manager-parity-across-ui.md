# Environment + toolchain manager and execution-context inspector parity — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
environment + toolchain manager and execution-context inspector parity
truth packet. The cross-tool boundary schema lives at
[`schemas/runtime/finalize_environment_and_toolchain_manager_parity_across_ui_truth.schema.json`](../../../schemas/runtime/finalize_environment_and_toolchain_manager_parity_across_ui_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-runtime/src/finalize_environment_and_toolchain_manager_parity_across_ui/`](../../../crates/aureline-runtime/src/finalize_environment_and_toolchain_manager_parity_across_ui/),
and the checked-in stable packet at
[`artifacts/runtime/m4/finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.json`](../../../artifacts/runtime/m4/finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.json).

The packet pins one boundary truth that the editor run surface,
terminal pane, task panel, CLI/headless inspector
(`aureline env inspect`), support export, release proof index,
Help/About proof card, and the conformance dashboard all read.
Surfaces MUST NOT mint local copies, fork their own runtime semantics,
or paraphrase inspector posture; they project the packet verbatim.

## Lanes (closed vocabulary)

- `local_lane` — local-host execution.
- `remote_helper_lane` — SSH and remote-agent attach.
- `container_lane` — ad-hoc containers and devcontainers.
- `managed_lane` — managed workspaces, remote workspace VMs,
  prebuild runtimes, and the AI sandbox.

Adding or removing a lane is a vocabulary change that requires bumping
the schema and updating the Rust contract, the artifact, the fixture
corpus, and this document together.

## Row classes (closed vocabulary)

- `inspector_parity_quality` — the lane headline. Required at
  `launch_stable` for any lane that claims the M4 grade.
- `inspector_field_admission` — one row per resolved inspector field
  (`interpreter`, `sdk`, `shell`, `container_target`, `remote_target`,
  `activator`, `trust_state`, `policy_source`). All eight required
  for any `launch_stable` lane.
- `parity_surface_binding` — one row per parity surface (`ui`,
  `cli_headless`, `help_about`, `support_export`). All four required
  for any `launch_stable` lane.
- `recovery_admission` — one row per structured recovery posture
  (`reconnect`, `restore_no_rerun`, `blocked_target`,
  `degraded_helper`, `artifact_provenance`). All five required for
  any `launch_stable` lane. The `restore_no_rerun` row MUST set
  `restore_preserves_no_rerun: true`.
- `toolchain_manager_admission` — admits environment + toolchain
  manager identity. Required for every `launch_stable` lane and MUST
  surface a non-empty `toolchain_manager_id_binding`.
- `lineage_admission` — binds the stable `execution_context_id` (or
  equivalent lineage object) into event streams, support packets,
  approval tickets, and evidence exports. Required for every
  `launch_stable` lane and MUST surface a non-empty
  `execution_context_id_binding`.
- `known_limit`, `downgrade_automation` — disclosed gap rows. Each
  must carry its disclosure ref.

## Support classes (closed vocabulary)

`launch_stable` is the M4 grade. `launch_stable_below`,
`beta_grade_only`, `preview_only`, and `unsupported` are the precise
narrowed labels; each narrowed row MUST surface a disclosure ref.
`support_unbound` never qualifies for stable promotion.

## Inspector fields (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish an
`inspector_field_admission` row for each of:

| field token | what the inspector resolves |
|---|---|
| `interpreter` | Resolved interpreter (e.g. Python interpreter, Node binary). |
| `sdk` | Resolved SDK / language toolchain. |
| `shell` | Resolved shell binding (login shell, devcontainer shell). |
| `container_target` | Resolved container target (ad-hoc container or devcontainer). |
| `remote_target` | Resolved remote target (SSH host, remote helper, workspace VM). |
| `activator` | Resolved activator (env-manager, devcontainer build, capsule activator). |
| `trust_state` | Resolved trust state (policy epoch, capability envelope). |
| `policy_source` | Resolved policy source (which layer wins the trust decision). |

A missing field auto-narrows the lane below `launch_stable` with a
typed `missing_inspector_field_coverage` finding.

## Parity surfaces (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a
`parity_surface_binding` row for each of:

- `ui` — editor UI (run surface, terminal pane, task panel,
  debug-prep, request workspace, AI tool, artifact view).
- `cli_headless` — `aureline env inspect` and headless launch.
- `help_about` — Help / About proof card.
- `support_export` — support / export bundle.

A missing surface auto-narrows the lane below `launch_stable` with a
typed `missing_parity_surface_binding_coverage` finding.

## Recovery postures (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a
`recovery_admission` row for each structured recovery posture:

| state token | meaning |
|---|---|
| `reconnect` | Reattach / reconnect posture (helper online, target reachable). |
| `restore_no_rerun` | Restore brought metadata back without rerunning tasks, reattaching debuggers, or reusing a drifted target. |
| `blocked_target` | Requested target is blocked by trust, policy, or capability. |
| `degraded_helper` | Helper / remote agent reports degraded capabilities. |
| `artifact_provenance` | Produced artifact survives reattach and support export with its target identity intact. |

## Lineage and `execution_context_id`

A `lineage_admission` row MUST be present on every `launch_stable`
lane with a non-empty `execution_context_id_binding`. Surfaces (event
streams, support packets, approval tickets, evidence exports) carry
the same lineage id so a "why this target?" question always resolves
to the same execution-context object.

## Environment + toolchain manager identity

A `toolchain_manager_admission` row MUST be present on every
`launch_stable` lane with a non-empty `toolchain_manager_id_binding`
(e.g. `pyenv`, `rye`, `volta`, `nvm`, `cargo-rustup`, `mise`). The
inspector resolves the binding identically across UI, CLI/headless,
Help/About, and support/export surfaces so reviewers see the same
manager identity regardless of where they ask.

## Restore / reconnect honesty

A `recovery_admission` row binding `restore_no_rerun` MUST be present
on every `launch_stable` lane with `restore_preserves_no_rerun: true`.
Reopening a surface MAY restore metadata, panes, and transcripts, but
it MUST NOT silently rerun tasks, reattach debuggers, or reuse a
drifted target.

## Consumer projections (required)

Every packet MUST carry a projection for each of:

- `editor_run_surface`
- `terminal_pane`
- `task_panel`
- `cli_headless`
- `support_export`
- `release_proof_index`
- `help_about`
- `conformance_dashboard`

Each projection MUST preserve the packet id and the nine vocabularies
verbatim (`preserves_lane_vocabulary`,
`preserves_row_class_vocabulary`,
`preserves_support_class_vocabulary`,
`preserves_inspector_field_vocabulary`,
`preserves_parity_surface_vocabulary`,
`preserves_recovery_state_vocabulary`,
`preserves_known_limit_vocabulary`,
`preserves_downgrade_automation_vocabulary`,
`preserves_evidence_class_vocabulary`). A projection that collapses
any vocabulary auto-narrows the packet below `launch_stable`.

## Validator findings

The validator emits one or more findings (`info` / `warning` /
`blocker`) per gap. A `blocker` always demotes the packet to
`blocks_stable`; a `warning` demotes it to `narrowed_below_stable`.
The closed finding vocabulary covers missing identity, missing lane
coverage, missing inspector-field / parity-surface / recovery-state
coverage, missing toolchain-manager admission, missing lineage
admission, unbound support / known-limit / downgrade-automation /
evidence bindings, missing or collapsed disclosure refs, raw source
material / secrets / ambient authority leaks, missing or drifted
consumer projections, and promotion-state mismatch. See
[`mod.rs`](../../../crates/aureline-runtime/src/finalize_environment_and_toolchain_manager_parity_across_ui/mod.rs)
for the full list.

## Auto-narrowing

When any required row is missing or any binding is unbound, the
packet is demoted automatically with a typed finding kind. This is the
honesty contract: no lane silently inherits adjacent green claims, and
no surface paraphrases inspector posture into free-form prose.

## See also

- Reviewer artifact: [`artifacts/runtime/m4/finalize-environment-and-toolchain-manager-parity-across-ui.md`](../../../artifacts/runtime/m4/finalize-environment-and-toolchain-manager-parity-across-ui.md)
- Generator: [`tools/regenerate_finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.py`](../../../tools/regenerate_finalize_environment_and_toolchain_manager_parity_across_ui_truth_packet.py)
- Execution-context resolver truth packet: [`docs/runtime/m4/stabilize-execution-context-resolver.md`](./stabilize-execution-context-resolver.md)
- Beta layer (lane manifest, ticket-drift evaluator): [`crates/aureline-runtime/src/execution_context/beta.rs`](../../../crates/aureline-runtime/src/execution_context/beta.rs)
- Seed resolver: [`crates/aureline-runtime/src/execution_context/mod.rs`](../../../crates/aureline-runtime/src/execution_context/mod.rs)
