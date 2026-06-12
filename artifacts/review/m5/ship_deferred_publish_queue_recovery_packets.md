# Deferred publish queue recovery packet

Packet id: `deferred-publish-recovery:stable:0001`

Surface: Deferred publish queue recovery packets

## Queue rows

- `provider_object:work_item:eng-90:link-review`: `draft_only` / `not_applicable_local_draft` / `not_applicable_draft_or_published` / `not_applicable`
- `provider_object:work_item:eng-84:update`: `queued_for_publish` / `provider_refresh_required_before_replay` / `manual_retry_after_provider_recovery` / `provider_version_review_required`
- `provider_object:release:fleet-0001:v1.2.3`: `blocked` / `current_target_identity_required` / `no_automatic_replay_across_changed_boundaries` / `export_or_discard_only`
- `provider_object:incident:sev-245:annotation`: `blocked` / `current_target_identity_required` / `retry_after_redaction_review` / `export_or_discard_only`
- `provider_object:work_item:eng-242:transition`: `stale_target` / `provider_refresh_required_before_replay` / `retry_after_freshness_refresh` / `provider_version_review_required`
- `provider_object:work_item:eng-242:comment-retry`: `conflict_review_required` / `current_target_identity_required` / `retry_after_conflict_reconcile` / `compare_and_reconcile_before_replay`
- `provider_object:work_item:eng-242:published`: `published` / `fresh_within_grace_window` / `not_applicable_draft_or_published` / `not_applicable`

## Support export

- rows: 7
- consumers: queue_panel, activity_center, support_export, incident_workspace, browser_companion
- raw provider material excluded: true
