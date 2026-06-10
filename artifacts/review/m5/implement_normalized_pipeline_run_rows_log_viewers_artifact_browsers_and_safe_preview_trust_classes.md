# Normalized Pipeline Run Rows, Log Viewers, Artifact Browsers, and Safe-Preview Trust Classes

- Packet: `pipeline-viewer:stable:0001`
- Schema: `schemas/review/implement-normalized-pipeline-run-rows-log-viewers-artifact-browsers-and-safe-preview-trust-classes.schema.json`
- Support export: `artifacts/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes/support_export.json`
- Contract doc: `docs/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes.md`
- Fixtures: `fixtures/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes/`
- Producer: `aureline_review::current_pipeline_viewer_export`

## Coverage

- **Normalized pipeline run rows** carry the target identity (what the run is
  for), the durable review anchor id, the pipeline label, the normalized run
  status, the freshness class, the trigger attribution, and the rerun/cancel
  authority. A non-green status (`failed`, `cancelled`, `action_required`,
  `timed_out`, `unknown`) must carry at least one attention reason, so the viewer
  never reads a run as benign when it is not, and `unknown` is never flattened
  into `failed` or `succeeded`.
- **Log viewer** rows record, per log, the stream state (`live_streaming`,
  `completed_replay`, `partial_retained`, `unavailable`), the safe-preview trust
  class, and the safe-open path. A partial or unavailable log must carry a
  non-empty truncation label and must not resolve an open path that depends on
  live bytes.
- **Artifact browser** rows record, per artifact, the artifact kind, the
  safe-preview trust class, the safe-open path, the freshness, a size disclosure,
  and a retention label. Opaque-byte artifacts (binary, container, archive) and
  degraded/stale/unverified freshness both narrow the open path off any
  in-product render.
- **Safe-preview trust classes** are drawn verbatim from the frozen architecture
  vocabulary (`RawText`, `SanitizedRich`, `TrustedLocalActive`,
  `IsolatedRemoteActive`). Because pipeline content arrives from a provider
  boundary, `TrustedLocalActive` is never admissible on a log or artifact row.

## Trust guardrails

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate: run status is explicit and never overstated; safe-preview
trust class is shown for every log and artifact; provider-boundary content never
resolves `TrustedLocalActive`; log truncation and artifact retention are labeled
rather than hidden; freshness is explicit and narrows the open path when
degraded; rerun/cancel authority is explicit and every action stays attributable;
no surface creates hidden write scope; downgrade narrows the claim instead of
hiding the lane; and stale or underqualified rows block promotion.

Proof freshness SLO is 168 hours with automatic narrowing on stale proof. The
supported downgrade triggers are `proof_stale`, `policy_blocked`,
`run_status_unverified`, `safe_preview_trust_narrowed`, `log_truncation_unlabeled`,
`artifact_retention_expired`, `run_control_authority_revoked`, `trust_narrowing`,
`scope_expansion_unqualified`, and `upstream_dependency_narrowed`.

## Boundary

Raw run, log, and artifact bodies, raw provider payloads, raw URLs, raw absolute
paths, credentials, and live provider responses never cross this boundary. The
packet carries only metadata, normalized run statuses, log stream states,
artifact kinds, safe-preview trust classes, safe-open paths, and contract
references. Every rerun, cancel, or download action stays read-only or
attributable and reviewable.
