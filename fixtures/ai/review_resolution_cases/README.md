# AI review-assist resolution-memory worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/ai/review_resolution_memory_contract.md`](../../../docs/ai/review_resolution_memory_contract.md)
and the schemas at
[`/schemas/ai/review_resolution_memory.schema.json`](../../../schemas/ai/review_resolution_memory.schema.json)
and
[`/schemas/ai/local_review_findings_store.schema.json`](../../../schemas/ai/local_review_findings_store.schema.json).

Every file is a multi-document YAML stream. The first document is a
`__fixture__` prelude summarising the scenario, the contract sections
it exercises, and the record kinds it produces. The remaining
documents are individual `review_resolution_record`,
`review_resolution_state_transition_record`,
`review_resolution_material_diff_change_record`,
`review_resolution_audit_event_record`,
`local_review_findings_store_record`,
`local_review_findings_export_packet_record`, and
`local_review_findings_store_audit_event_record` instances that
conform to the schemas.

No fixture embeds raw outbound text, raw inline-suggestion patch
bodies, raw check-annotation payloads, raw diff bodies, raw
absolute paths, raw branch / commit URLs, raw provider URLs, raw
provider-thread URLs, raw author identity strings, raw notebook
cell text, raw URLs, raw API keys, raw OAuth tokens, raw mTLS
material, raw model weights, or raw embeddings. Every such field
is an opaque ref or a redaction-aware reviewable label.

## Cases

- [`local_only_review.yaml`](./local_only_review.yaml) — A reviewer
  runs the AI review-assist pack against a `selected_diff_in_local_workspace`
  scope on a workspace with no provider overlay. The finding is
  kept locally with `resolution_state_class = open` and
  `publish_eligibility_class = not_applicable_local_only_no_publish_proposed`.
  The local store row preserves the analyzed scope, diff fingerprints,
  provider/model identity, and policy context. The export packet
  records `export_records_local_only_no_publish_proposed`.
- [`published_finding.yaml`](./published_finding.yaml) — A reviewer
  publishes an evidence-backed finding to a hosted thread comment.
  The resolution row records `resolution_state_class =
  published_to_review_destination` with
  `publish_eligibility_class = publish_eligible_provider_write_admitted`
  and an explicit `user_action_publish_to_destination` transition.
  The export packet records `export_records_publish_was_performed`.
- [`suppressed_org_policy.yaml`](./suppressed_org_policy.yaml) — An
  admin policy bundle suppresses a finding class for the workspace.
  The resolution row records `resolution_state_class =
  suppressed_by_policy` with
  `resolution_source_class = admin_policy_bundle` and
  `reopen_eligibility_class = reopen_admitted_admin_only`. The local
  store row's `storage_authority_class = admin_or_control_artifact`
  and `delete_posture_class = delete_denied_class_immutable`.
- [`provider_outage.yaml`](./provider_outage.yaml) — The reviewer
  attempts to publish a finding through a hosted destination but
  the provider write is unavailable due to an evaluator outage. The
  publish-to-review sheet refuses the action; the resolution row
  records `resolution_state_class = rerun_recommended` with
  `publish_eligibility_class = publish_blocked_outdated_lifecycle`.
  The local store and export packet preserve the finding for later
  reattempt; the export packet records
  `export_records_publish_blocked_kept_local`.
- [`changed_diff_with_stale_findings.yaml`](./changed_diff_with_stale_findings.yaml)
  — Two prior findings exist on a hosted-review object. After the
  author force-pushes a new head, the scope-freshness watcher fires
  a `base_or_head_changed_after_run_resolution_outdated` verdict on
  one and a `provider_overlay_refreshed_resolution_rerun_recommended`
  verdict on the other. Both resolution rows are forced into
  `outdated_diff_changed` / `rerun_recommended` with
  `publish_eligibility_class = publish_blocked_outdated_lifecycle`;
  no fresh visual chip is admitted while the analyzed scope drifts.
