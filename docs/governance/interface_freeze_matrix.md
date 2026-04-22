# Interface-freeze matrix

This document is the human-readable companion to
[`/artifacts/governance/interface_freeze_matrix.yaml`](../../artifacts/governance/interface_freeze_matrix.yaml).
The YAML file is the canonical machine-readable source; this page
explains how to read the rows and why they exist.

Companion documents:

- [`./interface_freeze_guide.md`](./interface_freeze_guide.md) —
  short downstream citation guide for task specs, ADRs, and packet
  updates.
- [`./control_artifact_index.md`](./control_artifact_index.md) —
  canonical governance index row for the matrix.
- [`/artifacts/milestones/M0_architecture_pack/packet_index.yaml`](../../artifacts/milestones/M0_architecture_pack/packet_index.yaml)
  and
  [`/artifacts/milestones/M0_architecture_pack/coverage_and_freeze_exceptions.yaml`](../../artifacts/milestones/M0_architecture_pack/coverage_and_freeze_exceptions.yaml)
  — source packet that named the carry-forward gates and freeze
  exceptions this matrix consolidates.

## Purpose

The milestone plan already says which decisions must be locked before
implementation broadens. What it did not have was one canonical table
that downstream work could cite directly. This matrix closes that gap.

Each row answers five questions in one place:

1. Is the contract `frozen`, `provisional`, or `blocked`?
2. Who owns it?
3. What is actually frozen right now?
4. Which downstream work is still narrowed or blocked by it?
5. When must the row be re-reviewed?

## Status meaning

| Status | Meaning |
|---|---|
| `frozen` | The contract is settled enough that downstream work should cite the row, not restate the contract prose. |
| `provisional` | A narrowed subset is usable now, but missing scope still blocks or narrows downstream work. |
| `blocked` | No implementation-broadening work should treat the row as settled; the contract is still missing. |

## Row classes

| Row class | Use |
|---|---|
| `core_m1_implementation_blocker` | Current implementation work should cite these rows directly. |
| `later_surface_architecture_seed` | These rows are visible now so later surfaces do not get invented ad hoc while core implementation proceeds. |

## Core implementation blockers

| Row | Status | Current state | Next review |
|---|---|---|---|
| `renderer_text_model` | `frozen` | Renderer, shaping fallback, invalidation, and hot-path hooks are accepted and stable enough for implementation. | 2026-05-15 |
| `buffer_strategy_and_source_fidelity` | `frozen` | Buffer, large-file, undo, and source-fidelity rules are accepted and should be reused as-is. | 2026-05-15 |
| `ipc_event_and_reactive_truth` | `frozen` | Internal RPC and subscription envelopes are frozen; packet-local variants should stop here. | 2026-05-15 |
| `workspace_identity_save_recovery_lineage` | `provisional` | Save semantics and filesystem identity are frozen, but lineage and compare/restore remain narrowed. | 2026-05-15 |
| `benchmark_hardware_corpus_and_fit_catalog` | `provisional` | Corpus and protected metrics exist; the unresolved gap is the approved hardware baseline. | 2026-05-06 |
| `repo_topology_and_work_package_ownership` | `provisional` | Package layering and ownership are explicit, but the shell home and backup coverage still need follow-up. | 2026-05-06 |
| `command_descriptor_and_invocation_schema` | `frozen` | Command ids, invocation packets, preview classes, and disabled reasons are settled. | 2026-05-15 |
| `settings_ids_and_effective_configuration` | `frozen` | Stable setting ids, scope precedence, and effective-setting records are settled. | 2026-05-15 |
| `entry_restore_and_migration_result_model` | `frozen` | Entry verbs, restore levels, and migration-result outcomes are settled. | 2026-05-15 |
| `design_token_and_component_state_vocabulary` | `frozen` | Core token, state, and layer names are frozen for shell implementation. | 2026-05-15 |
| `attention_routing_and_activity_taxonomy` | `frozen` | Durable work, interruptibility, quiet-hours, and reopen semantics are frozen. | 2026-05-15 |
| `embedded_surface_boundary_and_auth_handoff` | `frozen` | Embedded-boundary and system-browser-first auth handoff rules are accepted. | 2026-05-15 |
| `operator_truth_search_safety_and_install_review` | `frozen` | Search truth, interaction-safety, policy-simulation, and install-review vocabulary is frozen. | 2026-05-15 |
| `docs_route_and_exact_build_truth` | `provisional` | Docs-pack and route truth are seeded, but exact-build joins and clean-room confidence are still open. | 2026-05-06 |

## Later-surface architecture seeds

| Row | Status | Current state | Next review |
|---|---|---|---|
| `ai_provider_context_assembly_and_route_truth` | `provisional` | AI context, provider handoff, route/spend, and taint rules are frozen; provider arbitration is not. | 2026-06-15 |
| `collaboration_session_and_presence_contracts` | `blocked` | Only visible hooks exist; no collaboration session or presence contract is frozen. | 2026-07-15 |
| `review_change_stack_and_hosted_merge_policy` | `blocked` | Review lineage seeds exist, but hosted review and change-stack policy are still missing. | 2026-07-15 |
| `package_data_api_surface_boundaries` | `blocked` | Boundary and route seeds exist, but no dedicated package/data/API contract has landed. | 2026-07-15 |
| `scaffolding_and_generated_change_contracts` | `provisional` | Scaffolding is typed as a runtime and mutation class, but preview/rollback semantics still need a dedicated contract. | 2026-06-15 |
| `incident_intake_and_workspace_packet` | `provisional` | Severity and incident packet identity are frozen; live response tooling is still deferred. | 2026-06-15 |
| `companion_surface_projection_and_boundary_contracts` | `provisional` | Companion slot, scope, docs/help, and attention narrowing rules are seeded, but no single companion contract exists yet. | 2026-06-15 |

## Gate coverage

The YAML file includes an explicit `gate_row_map` so milestone gates can
be checked mechanically. The current map covers:

- the transition gate into the prototype milestone;
- the architecture-freeze row from the critical-path freeze calendar;
- the first shell/editor/workspace integration checkpoint;
- the M0 design-freeze checkpoint for operator-truth vocabulary;
- the late-M0 docs/route/rebuild checkpoint;
- the late-M0 onboarding/visual/interruptibility/embedded-boundary checkpoint;
- the M0 exit-gate language that says prototype work must not build on
  hidden operational debt.

If a new gate is added in the milestone document, extend the matrix in
the same change. If a row changes status, update the gate map and the
row together.
