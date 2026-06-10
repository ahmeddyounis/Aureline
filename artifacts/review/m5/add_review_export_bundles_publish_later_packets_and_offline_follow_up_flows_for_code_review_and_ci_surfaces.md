# Review/Export Bundles, Publish-Later Packets, and Offline Follow-Up Flows for Code Review and CI Surfaces

- Packet: `review-export-bundle:stable:0001`
- Schema: `schemas/review/add-review-export-bundles-publish-later-packets-and-offline-follow-up-flows-for-code-review-and-ci-surfaces.schema.json`
- Support export: `artifacts/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces/support_export.json`
- Contract doc: `docs/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces.md`
- Fixtures: `fixtures/review/m5/add_review_export_bundles_publish_later_packets_and_offline_follow_up_flows_for_code_review_and_ci_surfaces/`
- Producer: `aureline_review::current_review_export_bundle_export`

## Coverage

- **Export bundles** name each source a bundle is gathered from
  (`review_thread_bundle`, `ci_run_bundle`, `pipeline_bundle`,
  `mixed_review_ci_bundle`, `generic_bundle`, `unknown_scope_provider_owned`) and
  disclose whether the bundle supports publish-later, offline replay, and a
  redacted export, so an export can never claim a capability the bundle does not
  have. `unknown_scope_provider_owned` is never flattened into a known scope.
- **Bundle export rows** carry, per export, the durable review anchor, the bundle
  id, a redaction-aware subject label, an attention block, and an actor
  attribution with an audit row, so an export is always anchored, attributable,
  and honest about what it carries across the boundary. An unknown scope, an
  unverified/untrusted/unknown trust class, a stale/unknown freshness, a
  partial/unredacted/unknown redaction, an offline/reconnecting/unknown
  connectivity, a held/discarded/blocked/unknown disposition, or a blocked export
  each require an explicit attention reason.
- **Provenance and redaction** record, per export, the scope class, the trust
  class (`first_party_trusted`, `provider_verified`, `provider_unverified`,
  `untrusted_external`, `unknown_trust_provider_owned`), the truth freshness
  (`fresh_current_truth`, `stale_prior_truth`, `unknown_freshness_provider_owned`),
  a source label, and the redaction class (`fully_redacted_safe`, `metadata_only`,
  `partial_redaction_review_required`, `unredacted_blocked`,
  `unknown_redaction_provider_owned`), so an export can never hide which source it
  bundles, how fresh that source is, or how redacted the exported bytes are.
- **Publish-later and offline follow-up** record, per export, the publish state
  (`held_draft`, `queued_to_publish`, `scheduled_publish`, `published`,
  `publish_blocked`, `unknown_publish_provider_owned`), the connectivity class
  (`online`, `offline_queued`, `reconnecting`, `unknown_connectivity_provider_owned`),
  and the follow-up disposition (`no_pending_follow_up`, `replay_on_reconnect`,
  `hold_for_review`, `discarded`, `blocked_pending_truth`,
  `unknown_disposition_provider_owned`). A publish is a read-only draft unless an
  attributable `queued_to_publish` / `scheduled_publish` / `published` cites a
  `publish_ref`, a stale truth narrows the publish
  (`blocked_stale_truth_review_required`) rather than shipping a possibly-wrong
  state, and an offline, reconnecting, or unknown surface can never be
  pre-authorized to replay.

## Trust guardrails

The `trust_review` block encodes the hard invariants — all must hold for the
packet to validate: the bundle source provenance is disclosed; the bundle source
trust class is explicit; the redaction class, truth freshness, publish
disposition, and follow-up action are all disclosed; a publish is read-only unless
an attributable publish is cited; every export is anchored and attributable; no
publish or follow-up creates hidden publish scope; a stale truth narrows the
publish; an offline replay waits for reconnect authority; downgrade narrows the
claim instead of hiding the lane; and stale or underqualified rows block
promotion.

Proof freshness SLO is 168 hours with automatic narrowing on stale proof. The
supported downgrade triggers are `proof_stale`, `policy_blocked`,
`publish_attribution_missing`, `truth_stale`, `bundle_redaction_unverified`,
`bundle_trust_unknown`, `offline_replay_unauthorized`, `follow_up_unattributed`,
`trust_narrowing`, and `upstream_dependency_narrowed`.

## Boundary

Raw export bytes, raw bundle payloads, raw provider payloads, raw log bodies, raw
artifact bytes, raw absolute paths, raw author email addresses, credentials, and
live provider responses never cross this boundary. The packet carries only
metadata, bundle capabilities, scope classes, trust classes, freshness classes,
redaction classes, publish states, connectivity classes, disposition classes,
blocked classes, reviewable labels, and contract references. Every export,
provenance, redaction, publish disposition, and follow-up stays attributable and
reviewable before any publish or replay effect fires.
