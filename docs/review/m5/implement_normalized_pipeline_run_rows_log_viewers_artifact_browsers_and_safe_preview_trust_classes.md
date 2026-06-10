# Normalized Pipeline Run Rows, Log Viewers, Artifact Browsers, and Safe-Preview Trust Classes

This document is the contract for the M5 packet that projects CI / pipeline /
check / build / deploy / release runs into the in-product pipeline viewer without
ever pulling raw provider bytes across the support boundary. The packet is the
canonical M5 control source for this lane: the pipeline viewer, runs panel, log
pane, artifact browser, review-workspace header, CLI/headless output,
diagnostics, Help/About, and support exports ingest the checked-in packet rather
than cloning status text.

- Record kind: `normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes`
- Schema: [`schemas/review/implement-normalized-pipeline-run-rows-log-viewers-artifact-browsers-and-safe-preview-trust-classes.schema.json`](../../../schemas/review/implement-normalized-pipeline-run-rows-log-viewers-artifact-browsers-and-safe-preview-trust-classes.schema.json)
- Canonical support export: [`artifacts/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes/support_export.json`](../../../artifacts/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes/support_export.json)
- Summary artifact: [`artifacts/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes.md`](../../../artifacts/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes.md)
- Fixtures: [`fixtures/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes/`](../../../fixtures/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes/)
- Producer: `aureline_review::current_pipeline_viewer_export`

## Pillars

### Normalized pipeline run rows

Each `run_rows[]` row binds a run to its `target_identity_label` (what the run is
for), a `durable_anchor_id` that ties the run to its review anchor, a
`pipeline_label`, a `run_status`, a `freshness` class, a `trigger_attribution_label`,
a `run_control_authority` class, and a human-readable `status_summary`. A
non-green `run_status` must carry at least one entry in `attention_reasons`, so a
run is never read as benign when it failed, was cancelled, needs action, timed
out, or returned an unrecognised provider-owned status. Every `run_id` must
appear in at least one log viewer row.

| Field | Source contract |
| --- | --- |
| run identity, normalized status, freshness | [`schemas/ci/pipeline_run_row.schema.json`](../../../schemas/ci/pipeline_run_row.schema.json) |
| rerun / cancel authority and attribution | [`schemas/ci/run_control_review.schema.json`](../../../schemas/ci/run_control_review.schema.json) |
| log stream state and viewer | [`schemas/ci/log_view.schema.json`](../../../schemas/ci/log_view.schema.json) |
| artifact kind, safe-open path, retention | [`schemas/ci/pipeline_artifact_card.schema.json`](../../../schemas/ci/pipeline_artifact_card.schema.json) |
| safe-preview trust class | [`schemas/security/trust_class.schema.json`](../../../schemas/security/trust_class.schema.json) |

### Log viewers

Each `log_views[]` row records a `run_id`, a `view_id`, a redaction-aware
`log_label`, a `stream_state` (`live_streaming`, `completed_replay`,
`partial_retained`, `unavailable`), a `safe_preview_trust_class`, a
`safe_open_path`, and a `truncation_label`. A `partial_retained` or `unavailable`
log must carry a non-empty `truncation_label` and must resolve a `safe_open_path`
that does not depend on live bytes (`open_in_safe_preview_metadata_only`,
`download_only_no_in_product_open`, or `denied_no_open_path`), so a truncated log
is labeled rather than presented as complete. Every `run_id` must correspond to a
run row.

### Artifact browsers

Each `artifact_cards[]` row records a `run_id`, an `artifact_id`, a
redaction-aware `artifact_label`, an `artifact_kind`, a `safe_preview_trust_class`,
a `safe_open_path`, a `freshness` class, a `size_disclosure_label`, and a
`retention_label`. Opaque-byte artifacts (`binary_executable`, `container_image`,
`archive`) must resolve a download-only or denied open path, and any artifact
whose freshness is `degraded_cached`, `stale`, or `unverified` must narrow its
open path off live bytes. Every `run_id` must correspond to a run row.

### Safe-preview trust classes

`safe_preview_trust_class` uses the frozen architecture spellings verbatim
(`RawText`, `SanitizedRich`, `TrustedLocalActive`, `IsolatedRemoteActive`). Logs
and artifacts arrive from a provider boundary, so `TrustedLocalActive` is never
admissible on a log or artifact row; the schema and the Rust validator both
reject it.

## Track invariant

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate:

- `run_status_explicit` and `run_status_never_overstated` — statuses are explicit
  and non-green runs always carry their attention reasons.
- `safe_preview_trust_class_explicit` and `active_content_never_trusted_local` —
  every log and artifact shows a trust class and provider-boundary content never
  resolves `TrustedLocalActive`.
- `log_truncation_labeled_not_hidden` and `artifact_retention_labeled_not_hidden`
  — truncation and retention are labeled rather than silently absorbed.
- `freshness_explicit_and_narrows_open_path` — freshness is shown and degraded
  freshness narrows the safe-open path off live bytes.
- `rerun_cancel_authority_explicit_and_attributable`, `no_hidden_write_scope`,
  `downgrade_narrows_instead_of_hides`, and
  `stale_or_underqualified_blocks_promotion`.

## Downgrade and freshness

`proof_freshness` carries the SLO (168 hours) and last-refresh timestamp; when
proof goes stale `auto_narrow_on_stale` narrows the lane. The supported downgrade
triggers are `proof_stale`, `policy_blocked`, `run_status_unverified`,
`safe_preview_trust_narrowed`, `log_truncation_unlabeled`,
`artifact_retention_expired`, `run_control_authority_revoked`, `trust_narrowing`,
`scope_expansion_unqualified`, and `upstream_dependency_narrowed`. The
[fixtures](../../../fixtures/review/m5/implement_normalized_pipeline_run_rows_log_viewers_artifact_browsers_and_safe_preview_trust_classes/)
show a run whose logs and artifacts expired while offline, and a deployment run
whose provider-owned status could not be normalized; both remain valid because
narrowing is explicit, not hidden.

## Boundary

Raw run, log, and artifact bodies, raw provider payloads, raw URLs, raw absolute
paths, credentials, and live provider responses never cross this boundary. The
packet carries only metadata, normalized run statuses, log stream states,
artifact kinds, safe-preview trust classes, safe-open paths, and contract
references. Every rerun, cancel, or download action stays read-only or
attributable and reviewable.
