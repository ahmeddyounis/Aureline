# Browser/Provider Handoff Continuity for Review, CI, Logs, and Artifact Deep Links

Status: canonical M5 review-lane contract. The checked-in implementation,
fixtures, schema, and proof packet produced by this lane are canonical; later
product, help, and support surfaces consume them rather than re-describing the
state manually.

- Crate module: `aureline-review` →
  `ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links`
- Producer: `aureline_review::current_handoff_continuity_export`
- Packet type: `HandoffContinuityPacket` (`record_kind =
  ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links`,
  `schema_version = 1`)
- Boundary schema:
  `schemas/review/ship-browser-provider-handoff-continuity-for-review-ci-logs-and-artifact-deep-links.schema.json`
- Support export:
  `artifacts/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links/support_export.json`
- Fixtures:
  `fixtures/review/m5/ship_browser_provider_handoff_continuity_for_review_ci_logs_and_artifact_deep_links/`

## Purpose

This lane keeps a handoff from an in-product review, CI, log, or artifact surface
out to a browser tab or a provider web surface continuous and attributable: the
jump never loses its durable review anchor, never hides which destination, host,
or provider it lands on, never hides how fresh the underlying truth is, never
hides the safe-preview trust class, and never opens write scope unless an
attributable handoff is cited. It binds four pillars into one export-safe truth
packet that the review workspace header, merge-queue panel, pipeline run viewer,
log viewer, artifact browser, handoff action, command palette, CLI / headless
output, support exports, and diagnostics all project identically.

It builds on, and references by id, the remote-preview route contract
(`schemas/review/add-remote-preview-route-lifecycle-expiry-target-identity-and-preview-runtime-trust-disclosure.schema.json`),
the merge-queue / CI-status contract
(`schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json`),
the pipeline run / log / artifact safe-preview contract
(`schemas/review/implement-normalized-pipeline-run-rows-log-viewers-artifact-browsers-and-safe-preview-trust-classes.schema.json`),
and the trust-class vocabulary (`schemas/security/trust_class.schema.json`).

## Records

### Handoff target row

Each handoff target row names a stable `target_id` (which handoffs reference), a
`target_class` (`review_thread`, `ci_pipeline`, `ci_run`, `log_viewer`,
`artifact_deep_link`, `generic_target`, or `unknown_target_provider_owned`), a
`target_label`, three capability flags (`supports_deep_link`,
`supports_safe_preview`, `supports_provider_handoff`), a coverage label, and a
disclosure label. The unknown target class is never flattened into a known
target. Every handoff references an existing target row.

### Handoff continuity row

Each handoff row names its durable review anchor (`durable_anchor_id`), the
`target_id` it leaves from, and a redaction-aware subject label. It carries:

- `target_identity` — a `HandoffTargetIdentity` (`destination_class`,
  `trust_class`, `host_label`, `provider_label`, `identity_disclosed`). The
  identity must be disclosed: a non-empty host label, a non-empty provider label,
  and `identity_disclosed` set true. An `unknown_destination_provider_owned`
  destination, or a `provider_unverified`/`untrusted_external`/unknown trust
  class, each require an attention reason.
- `deep_link` — a `DeepLinkDisclosure` (`link_class`, `freshness_class`,
  `link_disclosed`, `link_label`). The link must be disclosed: a non-empty link
  label and `link_disclosed` set true. An `unanchored_link`/unknown link class, or
  a `stale_prior_truth`/unknown freshness, each require an attention reason.
- `safe_preview` — a `SafePreviewDisclosure` (`preview_class`,
  `preview_disclosed`, `preview_label`). The preview must be disclosed: a
  non-empty preview label and `preview_disclosed` set true. An
  `unsafe_preview_blocked`/`preview_unsupported`/unknown preview class requires an
  attention reason.
- `handoff_action` — a `HandoffAction` (`action_kind`, `action_disclosed`,
  `read_only`, `action_label`, `handoff_ref`). The action must be disclosed: a
  non-empty action label and `action_disclosed` set true. The `read_only` flag
  must match the action kind — `open_in_product`, `copy_deep_link`,
  `reveal_target_local`, and `unsupported_no_continuity` are read-only;
  `open_in_browser_handoff` and `open_in_provider_handoff` are not read-only and
  must cite a `handoff_ref`. An `unsupported_no_continuity` action must be blocked.
- `blocked_class` — `not_blocked` or one of the blocked reasons, including
  `blocked_no_durable_anchor`, `blocked_stale_truth_review_required`,
  `blocked_untrusted_target`, and `blocked_unsafe_preview`. A blocked action
  carries at least one attention reason.
- `actor_attribution_label` and `audit_row_ref` — both required and non-empty, so
  every handoff and its action is attributable and lands an audit row.

## Invariants

`HandoffContinuityPacket::validate` returns a stable list of
`HandoffContinuityViolation` tokens. The packet is canonical only when the list is
empty. The enforced invariants are:

- `wrong_record_kind` / `wrong_schema_version` / `missing_identity` — record kind,
  schema version, and identity fields are correct and present.
- `missing_source_contracts` — the schema, doc, remote-preview, merge-queue,
  pipeline, and trust-class refs are all present.
- `target_rows_missing` / `target_row_incomplete` — at least one target row, each
  with its required fields.
- `handoff_rows_missing` / `handoff_row_incomplete` — at least one handoff row,
  each with its required fields.
- `orphan_target_reference` — every handoff references an existing target row.
- `target_identity_undisclosed` — every handoff discloses its target host /
  provider identity.
- `deep_link_undisclosed` — every handoff discloses its deep link.
- `safe_preview_undisclosed` — every handoff discloses its safe-preview class.
- `handoff_action_undisclosed` — every handoff discloses its action.
- `handoff_action_read_only_mismatch` — the read-only flag matches the action
  kind.
- `handoff_ref_missing` — an `open_in_browser_handoff` / `open_in_provider_handoff`
  action cites a handoff ref.
- `unsupported_handoff_not_blocked` — an `unsupported_no_continuity` action is
  blocked.
- `attribution_missing` — every handoff carries an actor attribution and audit row.
- `attention_reason_missing` — an unknown destination, an unverified/untrusted/
  unknown trust class, an unanchored/unknown link, a stale/unknown freshness, an
  unsafe/unsupported preview, or a blocked action carries at least one attention
  reason.
- `downgrade_triggers_missing` / `consumer_surfaces_missing` — both lists are
  non-empty.
- `trust_review_incomplete` / `consumer_projection_incomplete` /
  `proof_freshness_incomplete` — the review, projection, and proof blocks hold.
- `raw_boundary_material_in_export` — the export carries no forbidden boundary
  material.

## Downgrade behavior

The `downgrade_triggers` list names the conditions that narrow this lane below its
claimed qualification: `proof_stale`, `policy_blocked`,
`handoff_attribution_missing`, `truth_stale`, `deep_link_unanchored`,
`target_identity_undisclosed`, `target_trust_unknown`, `safe_preview_unsupported`,
`trust_narrowing`, and `upstream_dependency_narrowed`. Proof freshness carries an
SLO (168 hours) and an automatic-narrow flag, so stale or underqualified rows
narrow the claim before publication rather than overstating it.

## Boundary

Raw deep-link URLs, raw host names, raw provider payloads, raw log bodies, raw
artifact bytes, raw absolute paths, raw author email addresses, credentials, and
live provider responses never cross this boundary. The packet is metadata-only:
target capabilities, target classes, destination classes, trust classes, link
classes, freshness classes, safe-preview classes, action kinds, blocked classes,
reviewable labels, and contract references. Every handoff, target identity, deep
link, safe preview, and action stays attributable and reviewable before any
handoff or upstream effect fires.
