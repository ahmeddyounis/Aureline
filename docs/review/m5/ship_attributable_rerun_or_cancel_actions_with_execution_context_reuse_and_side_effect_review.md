# Attributable Rerun / Cancel Actions, Execution-Context Reuse, and Side-Effect Review

Status: canonical M5 review-lane contract. The checked-in implementation,
fixtures, schema, and proof packet produced by this lane are canonical; later
product, help, and support surfaces consume them rather than re-describing the
state manually.

- Crate module: `aureline-review` →
  `ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review`
- Producer: `aureline_review::current_rerun_cancel_review_export`
- Packet type: `RerunCancelReviewPacket` (`record_kind =
  ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review`,
  `schema_version = 1`)
- Boundary schema:
  `schemas/review/ship-attributable-rerun-or-cancel-actions-with-execution-context-reuse-and-side-effect-review.schema.json`
- Support export:
  `artifacts/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review/support_export.json`
- Fixtures:
  `fixtures/review/m5/ship_attributable_rerun_or_cancel_actions_with_execution_context_reuse_and_side_effect_review/`

## Purpose

This lane lets a reviewer rerun or cancel a CI / pipeline / deployment run from
inside the product without ever firing an upstream effect that is
unattributable, that silently reuses a stale execution context, or that mutates
external state before its side effects have been reviewed. It binds three pillars
into one export-safe truth packet that the runs panel, run-control menu,
side-effect review sheet, review workspace header, command palette, CLI /
headless output, support exports, diagnostics, and Help / About all project
identically.

It builds on, and references by id, the frozen run-control review contract
(`schemas/ci/run_control_review.schema.json`), the pipeline-run-row contract
(`schemas/ci/pipeline_run_row.schema.json`), the execution-context contract
(`schemas/runtime/execution_context.schema.json`), and the trust-class vocabulary
(`schemas/security/trust_class.schema.json`).

## Records

### Rerun / cancel action row

Each action row names the run it acts on (`run_id`), its durable review anchor
(`durable_anchor_id`), and a redaction-aware target identity. It carries:

- `control_class` — one of `rerun_workflow`, `rerun_failed_jobs`,
  `rerun_single_job`, `rerun_single_step`, `cancel_workflow`, `cancel_single_job`,
  or `unknown_control_provider_owned`. The unknown control is never flattened
  into a known one and must carry at least one attention reason.
- `target_scope` — one of `single_step_only`, `single_job_only`,
  `failed_jobs_only`, `entire_workflow_run`, `entire_check_run`,
  `entire_deployment_run`, or `entire_release_run`. A known control must resolve
  to its fixed scope (`rerun_workflow`/`cancel_workflow` → `entire_workflow_run`,
  `rerun_failed_jobs` → `failed_jobs_only`, `rerun_single_job`/`cancel_single_job`
  → `single_job_only`, `rerun_single_step` → `single_step_only`).
- `mutation_mode` — one of `publish_now`, `open_in_provider`, or
  `deferred_publish`. The local-only `local_draft` mode is intentionally absent:
  rerun and cancel reach upstream provider state. `publish_now` must cite an
  `approval_ticket_ref`, `open_in_provider` must cite a `browser_handoff_ref`, and
  `deferred_publish` must cite a `deferred_queue_ref`.
- `blocked_class` — `not_blocked` or one of the blocked reasons, including
  `blocked_context_reuse_stale_review_required` and
  `blocked_side_effect_unacknowledged`. A blocked action carries at least one
  attention reason.
- `actor_attribution_label` and `audit_row_ref` — both required and non-empty,
  so every rerun or cancel is attributable and lands an audit row.
- `effect_summary` — four reviewable strings: what upstream effect will fire,
  where it lands, under whose authority, and what audit row will land. Every
  consumer surface projects the same summary for the same action id.

### Execution-context reuse

Each action records the `execution_context_id` it would reuse or fork from, a
`context_reuse_decision` (`reuse_identical_context`, `reuse_with_pinned_inputs`,
`reuse_with_refreshed_secrets`, `fork_new_context`, `provider_default_context`,
`unknown_context_provider_owned`), and a `context_freshness`
(`authoritative_live`, `warm_cached`, `degraded_cached`, `stale`, `unverified`).
When the decision reuses an existing recorded context and the freshness is
degraded, stale, or unverified, the action must carry a non-empty
`context_staleness_label` so the reuse is flagged and reviewed rather than
silently replayed.

### Side-effect review

Each side-effect row binds to an action by `action_id` and records a typed
`side_effect_class`, an `acknowledgment_requirement`, and a disclosure label.
Every action carries at least one side-effect row. Any non-inert side effect —
anything other than `no_external_side_effect`, including
`unknown_side_effect_provider_owned` — must require an acknowledgment stronger
than `no_ack_required` (`requires_explicit_confirmation`,
`requires_approval_ticket`, `requires_browser_handoff`, `requires_deferred_queue`,
or `denied_no_safe_action`) before the control can fire.

## Invariants

`RerunCancelReviewPacket::validate` returns a stable list of
`RerunCancelViolation` tokens. The packet is canonical only when the list is
empty. The enforced invariants are:

- `wrong_record_kind` / `wrong_schema_version` / `missing_identity` — record kind,
  schema version, and identity fields are correct and present.
- `missing_source_contracts` — the schema, doc, run-control, pipeline-run,
  execution-context, and trust-class refs are all present.
- `action_rows_missing` / `action_row_incomplete` — at least one action row, each
  with its required fields.
- `effect_summary_incomplete` — the four reviewable effect strings are present.
- `control_scope_mismatch` — a known control resolves to its fixed target scope.
- `attribution_missing` — every action carries an actor attribution and audit row.
- `mutation_grant_ref_missing` — each mutation mode cites the grant it requires.
- `context_reuse_stale_unflagged` — a degraded reused context carries a staleness
  label.
- `attention_reason_missing` — an unknown control, unknown context, or blocked
  action carries at least one attention reason.
- `action_missing_side_effect_review` — every action has at least one side-effect
  row.
- `orphan_row_reference` — a side-effect row references an existing action.
- `side_effect_rows_missing` / `side_effect_row_incomplete` — at least one
  side-effect row, each with its required fields.
- `side_effect_unacknowledged` — a non-inert side effect requires an explicit
  acknowledgment.
- `downgrade_triggers_missing` / `consumer_surfaces_missing` — both lists are
  non-empty.
- `trust_review_incomplete` / `consumer_projection_incomplete` /
  `proof_freshness_incomplete` — the review, projection, and proof blocks hold.
- `raw_boundary_material_in_export` — the export carries no forbidden boundary
  material.

## Downgrade behavior

The `downgrade_triggers` list names the conditions that narrow this lane below
its claimed qualification: `proof_stale`, `policy_blocked`,
`action_attribution_missing`, `context_reuse_stale`, `side_effect_unreviewed`,
`run_control_authority_revoked`, `trust_narrowing`, `scope_expansion_unqualified`,
and `upstream_dependency_narrowed`. Proof freshness carries an SLO (168 hours) and
an automatic-narrow flag, so stale or underqualified rows narrow the claim before
publication rather than overstating it.

## Boundary

Raw run, log, and artifact bodies, raw provider payloads, raw URLs, raw absolute
paths, raw author email addresses, credentials, and live provider responses never
cross this boundary. The packet is metadata-only: control classes, target scopes,
mutation modes, reuse decisions, context freshness, side-effect classes,
acknowledgment requirements, reviewable effect summaries, and contract
references.
