# Run Lineage, Rerun Review, And Interruption Recovery

This document is the reviewer-facing landing page for beta run-lineage
records. The runtime implementation lives at
[`/crates/aureline-runtime/src/run_lineage/`](../../../crates/aureline-runtime/src/run_lineage/).
The boundary schemas live at
[`/schemas/runtime/run_summary.schema.json`](../../../schemas/runtime/run_summary.schema.json)
and
[`/schemas/runtime/rerun_review.schema.json`](../../../schemas/runtime/rerun_review.schema.json).

The beta promise is one durable language from launch through retry, export,
and post-failure diagnosis:

- run headers preserve run id, attempt id, initiator, target, boundary,
  toolchain, environment capsule, policy epoch, build or commit identity, and
  start/finish state;
- durable job rows remain inspectable after look-away, sleep/resume, window
  switch, or runtime restart, with retry, exact rerun, current-context rerun,
  and details actions still present;
- artifact detail sheets keep producing run and attempt identity, freshness,
  retention class, redaction class, compatible viewer, and raw fallback;
- rerun review sheets always distinguish `Rerun exactly` from
  `Rerun with current context` when code, config, toolchain, locale, policy,
  secrets, branch, target, trust, environment, or workspace scope may differ;
- stale and imported rows preserve old evidence instead of replacing it with
  current-context rerun output.

## Records

| Record | Purpose |
| --- | --- |
| `run_summary_card_record` | Header or summary card for one run and latest attempt |
| `durable_job_row_record` | Activity-center scale row that survives surface dismissal and restore paths |
| `run_artifact_detail_sheet_record` | Artifact details with producing-run lineage and viewer/export posture |
| `rerun_review_sheet_record` | Exact-versus-current rerun review before dispatch |
| `run_history_support_export_record` | Export packet used by support, CLI/headless, and parity tests |

## Interruption Taxonomy

The controlled interruption vocabulary is:

| Token | Meaning |
| --- | --- |
| `user_cancel` | User cancelled the run |
| `remote_disconnect` | Remote helper, route, or target disconnected |
| `thermal_pause` | Host or target paused because of thermal pressure |
| `policy_block` | Policy denied, blocked, or revoked execution |
| `auth_expiry` | Credential or delegated auth expired |
| `process_crash` | Process or adapter crashed |
| `lost_source_map` | Source-map lineage could not be resolved |
| `truncated_log` | Logs were truncated by policy, transport, or retention |
| `stale_import` | Imported evidence is stale relative to current truth |
| `manual_replay_requirement` | Automatic replay is unavailable; manual replay is required |

These tokens are exported as a taxonomy and rendered directly on current run
rows whenever they apply, so interrupted work never exists only as toast text
or transient logs.

## Rerun Review

`RerunReviewSheet::from_comparison` consumes the existing
`RerunTargetComparison` produced by the rerun loop and adds non-target drift
rows for code, config, locale, branch, and secret-handle changes. The sheet
always includes two explicit mode options:

- `rerun_exactly` preserves the exact captured execution-context reference;
- `rerun_with_current_context` dispatches against the freshly resolved current
  context.

Any changed drift row sets `requires_review_before_dispatch`. The old/current
relationship is recorded as `stale_prior_evidence` rather than overwriting old
logs, diagnostics, validation summaries, or artifact links.

## Fixtures And Artifacts

Checked-in scenarios live under
[`/fixtures/runtime/m3/run_lineage_and_interruptions/`](../../../fixtures/runtime/m3/run_lineage_and_interruptions/).
The closed vocabulary packet and support-export projection live under
[`/artifacts/runtime/m3/run_history_packets/`](../../../artifacts/runtime/m3/run_history_packets/).

The seeded support export covers:

1. `current_local_passed_sleep_resume`
2. `remote_disconnect_current_context_review`
3. `auth_expiry_stale_evidence`
4. `stale_import_manual_replay`

## Acceptance

- Users can tell whether a rerun reproduces the original context exactly or
  uses current workspace/context.
- Durable history keeps logs, diagnostics, validation summaries, and evidence
  links attributable without widening sensitive payload retention.
- Interruption tokens remain visible in UI rows, activity rows, CLI/headless
  output, and support exports.
- Artifact sheets preserve producing-run lineage, freshness, redaction state,
  and raw fallback.
- Sleep/resume, disconnect, auth drift, and runtime restarts keep completed or
  failed work in durable rows instead of transient notification-only state.
