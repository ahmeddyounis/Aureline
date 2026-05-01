# Notification privacy / dedupe cases

Worked fixtures for [`/docs/ux/notification_privacy_dedupe_audit.md`](../../../docs/ux/notification_privacy_dedupe_audit.md),
[`/artifacts/ux/badge_class_review.yaml`](../../../artifacts/ux/badge_class_review.yaml),
and [`/artifacts/ux/quiet_hours_override_matrix.yaml`](../../../artifacts/ux/quiet_hours_override_matrix.yaml).

These fixtures are review-side bindings: each one cites the upstream
records (`notification_event_record`, `event_lineage_record`,
`activity_event_envelope_record`, `notification_suppression_record`,
`badge_class_review_row_record`, `quiet_hours_override_row_record`)
that prove the audit checklist or override row is mechanically
satisfied. The fixtures do not mint new vocabulary; every axis is a
re-export of the upstream schemas.

## Audit-row cases

- `grouped_burst_dedupe_audit.yaml` — one grouped burst with
  `dedupe_key_scheme = grouped_burst_id`; per-surface envelopes share
  one `canonical_event_id` and one `grouped_burst_id`; release on
  mode exit emits one digest.
- `repeated_failure_coalescing_audit.yaml` — ten retries against one
  pipeline collapse via `subsystem_plus_object_plus_phase`; the badge
  increments `failed_runs` once; tier-rise records
  `repeated_dedupe_threshold_crossed`.
- `lock_screen_safe_summary_audit.yaml` — a workspace-trust review
  payload renders as `lock_screen_safe_scoped`; forbidden label kinds
  are stripped and recorded on `privacy_redaction.stripped_label_kinds`;
  reopen routes through the in-product trust review canvas.
- `companion_fanout_audit.yaml` — one event fanned out to desktop
  and companion clients; cross-client siblings collapse via
  `cross_client_canonical_event_id`; companion action's
  `effect_class = read_only_reopen_attention_item`.
- `forbidden_shortcut_audit.yaml` — an OS notification action for a
  high-risk force-publish refuses to complete; the thirteen forbidden
  shortcut classes are enumerated; `effect_class =
  mutation_via_review_path_only` with a non-null `review_sheet_ref`.

## Quiet-hours override cases

- `security_advisory_quiet_hours_break.yaml` — secret-broker
  credential compromise breaks `mode_quiet_hours_user`;
  `tier_critical_safety`; OS payload `lock_screen_safe_generic`;
  reopen denies until revalidation.
- `trust_downgrade_quiet_hours_break.yaml` — workspace trust state
  flips to restricted under `mode_focus_mode_user`;
  `tier_blocking_trust`; reopen denies until trust review canvas
  re-entry.
- `approval_expiry_quiet_hours_break.yaml` — review-apply approval
  ticket about to expire under `mode_do_not_disturb_user`;
  `tier_blocking_trust`; OS payload `lock_screen_safe_scoped`.
- `route_warning_quiet_hours_break.yaml` — provider-handoff route
  becomes unsafe under `mode_presentation`; `tier_blocking_trust`;
  reopen denies until provider grant review canvas re-entry.
