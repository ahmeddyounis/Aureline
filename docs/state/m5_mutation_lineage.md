# Unified M5 mutation lineage

This document freezes the shared M5 mutation-lineage packet implemented by
[`crates/aureline-reactive-state/src/m5_mutation_lineage/mod.rs`](../../crates/aureline-reactive-state/src/m5_mutation_lineage/mod.rs),
consumed by the support-export envelope in
[`crates/aureline-support/src/m5_mutation_lineage/mod.rs`](../../crates/aureline-support/src/m5_mutation_lineage/mod.rs),
and projected into shell history rows by
[`crates/aureline-shell/src/m5_mutation_history_inspector/mod.rs`](../../crates/aureline-shell/src/m5_mutation_history_inspector/mod.rs).

The checked-in truth lives in:

- [`artifacts/state/m5_mutation_lineage.json`](../../artifacts/state/m5_mutation_lineage.json)
- [`artifacts/state/m5_mutation_lineage.md`](../../artifacts/state/m5_mutation_lineage.md)
- [`fixtures/state/m5_mutation_lineage/`](../../fixtures/state/m5_mutation_lineage/)
- [`schemas/state/m5_mutation_lineage.schema.json`](../../schemas/state/m5_mutation_lineage.schema.json)

## Frozen vocabulary

The packet closes the vocabulary for the following axes:

- `surface_class`: `notebook_document`, `notebook_output`,
  `request_workspace`, `data_export_artifact`, `preview_output`,
  `sync_packet`, `repair_transaction`, `provider_draft`,
  `workflow_bundle`, `profiler_trace`, `ai_evidence_packet`,
  `incident_action`
- `artifact_class`: `notebook_file`, `notebook_output_bundle`,
  `request_document`, `query_export`, `preview_snapshot`,
  `sync_manifest`, `repair_receipt`, `provider_draft`,
  `workflow_bundle`, `trace_capture`, `ai_evidence_packet`,
  `incident_packet`
- `reversal_class`: `exact`, `grouped_exact`, `compensate`,
  `regenerate`, `manual`, `audit_only`
- `actor_class`: `interactive_user`, `notebook_runtime`,
  `query_runner`, `preview_publisher`, `sync_engine`,
  `repair_executor`, `provider_publisher`,
  `workflow_bundle_runner`, `profiler_capture_service`,
  `ai_assistant`, `incident_responder`
- `source_class`: `human_local`, `machine_local`,
  `machine_remote_agent`, `ai_hosted_provider`,
  `policy_driven`, `imported_evidence`
- `scope_class`: `workspace`, `notebook`,
  `request_workspace`, `data_workspace`, `preview_route`,
  `sync_lane`, `repair_scope`, `provider_draft`,
  `workflow_bundle`, `performance_session`,
  `incident_workspace`
- `automation_influence`: `none`, `notebook_runtime`,
  `query_plan_automation`, `preview_publish_automation`,
  `sync_reconciliation`, `workflow_bundle`,
  `repair_transaction`, `incident_capture`,
  `ai_evidence_capture`
- `policy_influence`: `none`, `policy_checked`,
  `approval_bound`, `reauth_gate`, `incident_retention`,
  `provider_publish_rules`

## Lineage model

The packet keeps three levels distinct:

- `mutation_id`: one material mutation entry
- `group_id`: one visible mutation phase or apply step
- `lineage_root_id`: one cross-surface thread that joins follow-on
  artifacts, evidence capture, repair, and incident work back to the
  original mutation context

This separation is the key M5 addition. It lets the product keep
`AI evidence`, `repair follow-on`, `workflow bundles`, `provider draft`
work, and `incident actions` attributable to the same checkpoint lineage
without lying that every follow-on row is the same kind of undoable
mutation.

## Thread coverage

The checked-in packet proves five lineage roots:

| Lineage root | Covered surfaces | Highest visible reversal |
| --- | --- | --- |
| `lineage:m5:notebook_execution:0001` | notebook document, notebook output, AI evidence | `audit_only` |
| `lineage:m5:request_batch:0001` | request workspace, data export | `grouped_exact` |
| `lineage:m5:preview_publish:0001` | preview output | `regenerate` |
| `lineage:m5:provider_sync:0001` | sync packet, provider draft, repair transaction, workflow bundle | `manual` |
| `lineage:m5:incident_capture:0001` | profiler trace, incident action | `audit_only` |

## Contract rules

- Every material M5 mutation emits one attributable journal entry with
  actor, source, scope, checkpoint lineage, and explicit reversal class.
- `exact` language is reserved for exact rows; `grouped_exact` is the
  narrower label for grouped multi-artifact rollback. `compensate`,
  `regenerate`, `manual`, and `audit_only` remain visibly distinct.
- History inspectors aggregate by `lineage_root_id`, not by whichever
  subsystem happened to create the latest artifact.
- Support-export rows preserve `file count`, `artifact class`, and
  `automation` or `policy` influence without embedding raw payloads or
  ambient authority.
- Deferred or managed follow-on work never replays invisibly. If
  reauthentication, approval, or manual recovery is required, the same
  lineage root stays visible and the visible reversal class narrows.
