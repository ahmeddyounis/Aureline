# AI review-assist worked-example corpus

This directory holds worked examples for the contract frozen in
[`/docs/ai/review_assist_publish_contract.md`](../../../docs/ai/review_assist_publish_contract.md)
and the schemas at
[`/schemas/ai/review_finding.schema.json`](../../../schemas/ai/review_finding.schema.json),
[`/schemas/ai/review_scope_selection.schema.json`](../../../schemas/ai/review_scope_selection.schema.json),
and
[`/schemas/ai/publish_to_review_sheet.schema.json`](../../../schemas/ai/publish_to_review_sheet.schema.json).

Every file is a multi-document YAML stream. The first document
is a `__fixture__` prelude summarising the scenario, the contract
sections it exercises, and the record kinds it produces. The
remaining documents are individual `review_scope_selection_record`,
`review_finding_record`,
`publish_to_review_sheet_record`,
`review_scope_selection_audit_event_record`,
`review_finding_audit_event_record`, and
`publish_to_review_audit_event_record` instances that conform to
the schemas.

No fixture embeds raw outbound text, raw inline suggestion patch
bodies, raw check annotation payloads, raw diff bodies, raw
absolute paths, raw branch / commit URLs, raw provider URLs, raw
provider thread URLs, raw author identity strings, raw notebook
cell text, or raw URLs. Every such field is an opaque ref or a
redaction-aware reviewable label.

## Cases

- [`local_diff_review_with_publish.yaml`](./local_diff_review_with_publish.yaml)
  — A reviewer pins a `selected_diff_in_local_workspace` scope
  against a local branch with no provider overlay. The AI run
  produces one `risk_or_bug_concern` finding directed by a
  `review_pack_advisory_check` with confidence
  `evidence_backed`. The finding is published as a
  `hosted_review_thread_comment` through a composite-with-overlay
  workflow with `provider_write_admitted` continuity, attribution
  `posted_as_user_with_ai_assist_disclosed`, and redaction note
  `internal_identifier_redacted` after the broker pass redacts an
  internal ticket id.
- [`hosted_review_stale_base_head_outdated.yaml`](./hosted_review_stale_base_head_outdated.yaml)
  — A reviewer runs the AI assist against a `hosted_review_object`
  scope whose base/head changed after the initial run. The new
  scope row is derived from the prior scope with lineage
  `derived_from_prior_scope_base_head_changed` and rerun reason
  `base_or_head_changed_after_run_rerun_required`. The matching
  finding lifecycle moves to `outdated_diff_changed` and routes
  through `mark_outdated_pending_rerun`; a publish attempt would
  deny with `outdated_lifecycle_must_not_admit_publish_action`.
- [`provider_write_missing_keep_local.yaml`](./provider_write_missing_keep_local.yaml)
  — The reviewer holds a hosted review object through a browser
  handoff packet but the provider write capability is not held
  on this client. The publish-to-review sheet resolves to
  `provider_write_missing_keep_local_or_export` continuity and
  `block_publish_provider_write_missing_keep_local` action; the
  finding is published locally to a
  `local_review_workspace_anchor_only` destination instead, and
  the audit stream records
  `publish_to_review_sheet_provider_write_missing_observed`.
- [`redaction_required_publication_blocked.yaml`](./redaction_required_publication_blocked.yaml)
  — A reviewer attempts to publish an inline suggested patch
  whose preview contains a credential handle that the broker pass
  flags as `redaction_required_user_must_review`. The publish
  sheet refuses `publish_to_destination` and resolves to
  `block_publish_redaction_failed_user_must_review` (after the
  user-review pass surfaces the broker's findings without
  resolving them) with redaction note
  `redaction_pass_failed_publish_blocked`; the finding remains in
  `open` lifecycle.
