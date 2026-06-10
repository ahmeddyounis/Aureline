# Review/Export Bundles, Publish-Later Packets, and Offline Follow-Up Flows for Code Review and CI Surfaces

Status: canonical M5 review-lane contract. The checked-in implementation,
fixtures, schema, and proof packet produced by this lane are canonical; later
product, help, and support surfaces consume them rather than re-describing the
state manually.

- Crate module: `aureline-review` →
  `add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces`
- Producer: `aureline_review::current_review_export_bundle_export`
- Packet type: `ReviewExportBundlePacket` (`record_kind =
  add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces`,
  `schema_version = 1`)
- Boundary schema:
  `schemas/review/add-review-export-bundles-publish-later-packets-and-offline-follow-up-flows-for-code-review-and-ci-surfaces.schema.json`
- Support export:
  `artifacts/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces/support_export.json`
- Fixtures:
  `fixtures/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces/`

## Purpose

This lane lets a code-review or CI surface gather its state into an export-safe
bundle, queue a publish for later, and queue follow-up actions while offline —
without ever losing the durable review anchor, hiding the bundle's provenance,
hiding how fresh the bundled truth is, hiding the export redaction class, or
opening hidden publish scope. A publish stays a read-only draft unless an
attributable publish is cited, a stale truth narrows the publish rather than
shipping a possibly-wrong state, and an offline follow-up is held for review
rather than auto-firing on reconnect. It binds four pillars into one export-safe
truth packet that the review workspace header, merge-queue panel, pipeline run
viewer, export bundle panel, publish-later queue, offline follow-up tray, command
palette, CLI / headless output, support exports, and diagnostics all project
identically.

It builds on, and references by id, the browser/provider handoff continuity
contract
(`schemas/review/ship-browser-provider-handoff-continuity-for-review-ci-logs-and-artifact-deep-links.schema.json`),
the merge-queue / CI-status contract
(`schemas/review/add-merge-queue-readiness-stale-base-invalidation-and-approval-recomputation-flows.schema.json`),
the pipeline run / log / artifact safe-preview contract
(`schemas/review/implement-normalized-pipeline-run-rows-log-viewers-artifact-browsers-and-safe-preview-trust-classes.schema.json`),
and the trust-class vocabulary (`schemas/security/trust_class.schema.json`).

## Records

### Export bundle row

Each export bundle row names a stable `bundle_id` (which exports reference), a
`scope_class` (`review_thread_bundle`, `ci_run_bundle`, `pipeline_bundle`,
`mixed_review_ci_bundle`, `generic_bundle`, or `unknown_scope_provider_owned`), a
`bundle_label`, three capability flags (`supports_publish_later`,
`supports_offline_replay`, `supports_redacted_export`), a coverage label, and a
disclosure label. The unknown scope class is never flattened into a known scope.
Every export references an existing bundle row.

### Bundle export row

Each export row names its durable review anchor (`durable_anchor_id`), the
`bundle_id` it is gathered from, and a redaction-aware subject label. It carries:

- `provenance` — a `BundleProvenance` (`scope_class`, `trust_class`,
  `freshness_class`, `source_label`, `identity_disclosed`). The provenance must be
  disclosed: a non-empty source label and `identity_disclosed` set true. An
  `unknown_scope_provider_owned` scope, a
  `provider_unverified`/`untrusted_external`/unknown trust class, or a
  `stale_prior_truth`/unknown freshness, each require an attention reason.
- `redaction` — a `BundleRedactionDisclosure` (`redaction_class`,
  `redaction_disclosed`, `redaction_label`). The redaction must be disclosed: a
  non-empty redaction label and `redaction_disclosed` set true. A
  `partial_redaction_review_required`/`unredacted_blocked`/unknown redaction class
  requires an attention reason.
- `publish_disposition` — a `PublishDisposition` (`publish_state`,
  `publish_disclosed`, `read_only`, `publish_label`, `publish_ref`). The
  disposition must be disclosed: a non-empty publish label and `publish_disclosed`
  set true. The `read_only` flag must match the publish state — `held_draft`,
  `publish_blocked`, and `unknown_publish_provider_owned` are read-only;
  `queued_to_publish`, `scheduled_publish`, and `published` are not read-only and
  must cite a `publish_ref`. A `publish_blocked` state must be blocked.
- `follow_up_action` — a `FollowUpAction` (`connectivity_class`,
  `disposition_class`, `action_disclosed`, `replay_ready`, `action_label`). The
  action must be disclosed: a non-empty action label and `action_disclosed` set
  true. `replay_ready` is reserved for an `online` surface — an `offline_queued`,
  `reconnecting`, or unknown surface can never be pre-authorized to replay. An
  `offline_queued`/`reconnecting`/unknown connectivity, or a
  `hold_for_review`/`discarded`/`blocked_pending_truth`/unknown disposition, each
  require an attention reason.
- `blocked_class` — `not_blocked` or one of the blocked reasons, including
  `blocked_stale_truth_review_required`, `blocked_untrusted_bundle`,
  `blocked_policy_forbids_publish`, `blocked_unredacted_export`, and
  `blocked_offline_no_replay_authority`. A blocked export carries at least one
  attention reason.
- `actor_attribution_label` and `audit_row_ref` — both required and non-empty, so
  every export and its action is attributable and lands an audit row.

## Invariants

`ReviewExportBundlePacket::validate` returns a stable list of
`ReviewExportBundleViolation` tokens. The packet is canonical only when the list
is empty. The enforced invariants are:

- `wrong_record_kind` / `wrong_schema_version` / `missing_identity` — record kind,
  schema version, and identity fields are correct and present.
- `missing_source_contracts` — the schema, doc, handoff-continuity, merge-queue,
  pipeline, and trust-class refs are all present.
- `bundle_rows_missing` / `bundle_row_incomplete` — at least one bundle row, each
  with its required fields.
- `export_rows_missing` / `export_row_incomplete` — at least one export row, each
  with its required fields.
- `orphan_bundle_reference` — every export references an existing bundle row.
- `bundle_provenance_undisclosed` — every export discloses its source provenance.
- `bundle_redaction_undisclosed` — every export discloses its redaction class.
- `publish_disposition_undisclosed` — every export discloses its publish
  disposition.
- `publish_read_only_mismatch` — the read-only flag matches the publish state.
- `publish_ref_missing` — a `queued_to_publish` / `scheduled_publish` / `published`
  disposition cites a publish ref.
- `publish_blocked_not_marked` — a `publish_blocked` disposition is blocked.
- `follow_up_undisclosed` — every export discloses its follow-up action.
- `offline_replay_without_authority` — a `replay_ready` follow-up is reserved for
  an `online` surface.
- `attribution_missing` — every export carries an actor attribution and audit row.
- `attention_reason_missing` — an unknown scope, an unverified/untrusted/unknown
  trust class, a stale/unknown freshness, a partial/unredacted/unknown redaction,
  an offline/reconnecting/unknown connectivity, a held/discarded/blocked/unknown
  disposition, or a blocked export carries at least one attention reason.
- `downgrade_triggers_missing` / `consumer_surfaces_missing` — both lists are
  non-empty.
- `trust_review_incomplete` / `consumer_projection_incomplete` /
  `proof_freshness_incomplete` — the review, projection, and proof blocks hold.
- `raw_boundary_material_in_export` — the export carries no forbidden boundary
  material.

## Downgrade behavior

The `downgrade_triggers` list names the conditions that narrow this lane below its
claimed qualification: `proof_stale`, `policy_blocked`,
`publish_attribution_missing`, `truth_stale`, `bundle_redaction_unverified`,
`bundle_trust_unknown`, `offline_replay_unauthorized`, `follow_up_unattributed`,
`trust_narrowing`, and `upstream_dependency_narrowed`. Proof freshness carries an
SLO (168 hours) and an automatic-narrow flag, so stale or underqualified rows
narrow the claim before publication rather than overstating it.

## Boundary

Raw export bytes, raw bundle payloads, raw provider payloads, raw log bodies, raw
artifact bytes, raw absolute paths, raw author email addresses, credentials, and
live provider responses never cross this boundary. The packet is metadata-only:
bundle capabilities, scope classes, trust classes, freshness classes, redaction
classes, publish states, connectivity classes, disposition classes, blocked
classes, reviewable labels, and contract references. Every export, provenance,
redaction, publish disposition, and follow-up stays attributable and reviewable
before any publish or replay effect fires.
