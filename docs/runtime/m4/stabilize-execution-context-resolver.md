# Stabilize execution-context resolver — M4 truth packet

This document is the reviewer-facing contract for the M4 stable
execution-context resolver truth packet. The cross-tool boundary
schema lives at
[`schemas/runtime/stabilize_execution_context_resolver_truth.schema.json`](../../../schemas/runtime/stabilize_execution_context_resolver_truth.schema.json),
the canonical Rust contract at
[`crates/aureline-runtime/src/stabilize_execution_context_resolver/`](../../../crates/aureline-runtime/src/stabilize_execution_context_resolver/),
and the checked-in stable packet at
[`artifacts/runtime/m4/stabilize_execution_context_resolver_truth_packet.json`](../../../artifacts/runtime/m4/stabilize_execution_context_resolver_truth_packet.json).

The packet pins one boundary truth that the editor run surface,
terminal pane, task panel, CLI/headless inspector
(`aureline env inspect`), support export, release proof index,
Help/About proof card, and the conformance dashboard all read.
Surfaces MUST NOT mint local copies, fork their own runtime semantics,
or paraphrase execution-context posture; they project the packet
verbatim.

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

- `execution_context_resolution_quality` — the lane headline.
  Required at `launch_stable` for any lane that claims the M4 grade.
- `surface_binding` — one row per run-capable surface
  (`terminal`, `task`, `test`, `debug`, `request_workspace`,
  `artifact`, `ai_tool`, `cli_headless`, `docs_help`,
  `support_export`).
- `target_admission` — admits requested-vs-materialized target
  identity. Required for every `launch_stable` lane.
- `state_admission` — one row per structured resolver state
  (`wrong_target`, `blocked_activator`, `stale_capsule_or_prebuild`,
  `unsupported_skew`, `reconnect_required`, `restore_no_rerun`,
  `degraded_helper`, `route_drift`). All eight required for any
  `launch_stable` lane.
- `restore_rerun_honesty` — attests that restore brings metadata,
  panes, and transcripts back without silently rerunning tasks,
  reattaching debuggers, or reusing a drifted target. Required for
  every `launch_stable` lane and MUST set `restore_preserves_no_rerun:
  true`.
- `capability_skew_admission` — admits helper capability negotiation
  / mixed-version skew posture. Required for every `launch_stable`
  lane.
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

## Surface bindings (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a `surface_binding`
row for each of:

- `terminal` — terminal pane / shell session header
- `task` — task pane / per-run header
- `test` — test-runner host
- `debug` — debug-prep / debug-run host
- `request_workspace` — managed-workspace request flow
- `artifact` — produced-artifact / output binding
- `ai_tool` — AI tool-call host
- `cli_headless` — `aureline env inspect` and headless launch
- `docs_help` — docs/help disclosures
- `support_export` — support / export bundles

A missing surface auto-narrows the lane below `launch_stable` with a
typed `missing_surface_binding_coverage` finding.

## Resolver states (required per `launch_stable` lane)

Every lane claiming `launch_stable` MUST publish a `state_admission`
row for each structured state. The resolver never falls back to
generic launch-failure copy; the eight states are the closed
vocabulary every surface reads:

| state token | meaning |
|---|---|
| `wrong_target` | Requested target disagrees with materialized target. |
| `blocked_activator` | Activator (env-manager, container, devcontainer build, …) is blocked by trust, policy, or capability. |
| `stale_capsule_or_prebuild` | Cached capsule or prebuild is stale / drifted. |
| `unsupported_skew` | Mixed-version skew falls outside the published window. |
| `reconnect_required` | Reattach / reconnect is required before the surface may dispatch. |
| `restore_no_rerun` | Restore brought metadata back without rerunning tasks, reattaching debuggers, or reusing a drifted target. |
| `degraded_helper` | Helper / remote agent reports degraded capabilities. |
| `route_drift` | Route / tunnel posture drifted relative to the stored binding. |

## Lineage and `execution_context_id`

A `lineage_admission` row MUST be present on every `launch_stable`
lane with a non-empty `execution_context_id_binding`. Surfaces (event
streams, support packets, approval tickets, evidence exports) carry
the same lineage id so a "why this target?" question always resolves
to the same execution-context object.

## Restore / reconnect honesty

A `restore_rerun_honesty` row MUST be present on every `launch_stable`
lane with `restore_preserves_no_rerun: true`. Reopening a surface MAY
restore metadata, panes, and transcripts, but it MUST NOT silently
rerun tasks, reattach debuggers, or reuse a drifted target.

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

Each projection MUST preserve the packet id and the eight vocabularies
verbatim (`preserves_lane_vocabulary`,
`preserves_row_class_vocabulary`,
`preserves_support_class_vocabulary`,
`preserves_surface_binding_vocabulary`,
`preserves_resolver_state_vocabulary`,
`preserves_known_limit_vocabulary`,
`preserves_downgrade_automation_vocabulary`,
`preserves_evidence_class_vocabulary`). A projection that collapses
any vocabulary auto-narrows the packet below `launch_stable`.

## Validator findings

The validator emits one or more findings (`info` / `warning` /
`blocker`) per gap. A `blocker` always demotes the packet to
`blocks_stable`; a `warning` demotes it to `narrowed_below_stable`.
The closed finding vocabulary covers missing identity, missing lane
coverage, missing surface or state coverage, missing target
admission, missing restore-rerun honesty, missing capability-skew
admission, missing lineage admission, unbound support / known-limit /
downgrade-automation / evidence bindings, missing or collapsed
disclosure refs, raw source material / secrets / ambient authority
leaks, missing or drifted consumer projections, and promotion-state
mismatch. See
[`mod.rs`](../../../crates/aureline-runtime/src/stabilize_execution_context_resolver/mod.rs)
for the full list.

## Auto-narrowing

When any required row is missing or any binding is unbound, the
packet is demoted automatically with a typed finding kind. This is the
honesty contract: no lane silently inherits adjacent green claims, and
no surface paraphrases execution-context state into free-form prose.

## See also

- Spec row: `.plans/M04-081.md`
- Reviewer artifact: [`artifacts/runtime/m4/stabilize-execution-context-resolver.md`](../../../artifacts/runtime/m4/stabilize-execution-context-resolver.md)
- Generator: [`tools/regenerate_stabilize_execution_context_resolver_truth_packet.py`](../../../tools/regenerate_stabilize_execution_context_resolver_truth_packet.py)
- Beta layer (lane manifest, ticket-drift evaluator): [`crates/aureline-runtime/src/execution_context/beta.rs`](../../../crates/aureline-runtime/src/execution_context/beta.rs)
- Seed resolver: [`crates/aureline-runtime/src/execution_context/mod.rs`](../../../crates/aureline-runtime/src/execution_context/mod.rs)
