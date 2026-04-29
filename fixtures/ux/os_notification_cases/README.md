# OS Notification Cases

Worked fixtures for [`/docs/ux/os_notification_and_quiet_hours_contract.md`](../../../docs/ux/os_notification_and_quiet_hours_contract.md)
and [`/schemas/ux/notification_suppression_record.schema.json`](../../../schemas/ux/notification_suppression_record.schema.json).

These cases cover suppression audit, lock-screen privacy, exact-reopen
linkage, no-bypass on high-risk shortcuts, and desktop summary
affordances:

## `notification_suppression_record` cases

- `quiet_hours_held_suppression_audit.json` — a held OS notification
  during `mode_quiet_hours_user`, releasing as one grouped digest on
  mode exit while preserving the durable activity-center linkback and
  the same `canonical_event_id`.
- `lock_screen_redacted_to_audit_only_under_privacy_mode.json` — a
  workspace-trust review denied lock-screen delivery under
  `mode_privacy_mode` with `policy_forbidden_on_lock_screen`; the
  in-product attention item still renders, the lock-screen surface
  receives no payload, and reopen requires unlock plus review.
- `admin_narrowed_durable_only.json` — admin policy narrows OS-level
  surfaces to durable-only delivery via `held_admin_suppression`;
  the durable-job row remains the authoritative render path.
- `grouped_burst_dedupe_collapse_audit.json` — a retry collapses onto
  an existing grouped burst with `not_delivered_collapsed` and an
  audit-only release step.

## `privacy_safe_payload_rule_record` cases

- `lock_screen_safe_generic_payload_rule.json` — frozen rule for the
  category-only payload class shared by lock-screen, companion, and
  system-notification surfaces, with the closed allowed-label,
  allowed-action, and forbidden-shortcut sets.
- `in_product_only_payload_rule.json` — frozen rule for the in-product
  payload class; lock-screen, companion, and system-notification
  surfaces are excluded by construction.

## `desktop_summary_affordance_record` cases

- `dock_taskbar_progress_read_only_mirror.json` — dock / taskbar
  progress affordance mirroring a durable build job;
  `effect_class = read_only_progress_mirror`, progress derived from
  the durable-job envelope, mutation classes forbidden.
- `lock_screen_quick_action_read_only_reopen.json` — lock-screen quick
  action for a collaboration mention; `effect_class =
  read_only_reopen_attention_item`, scoped privacy payload, mutation
  classes forbidden.
- `os_action_high_risk_review_sheet_no_bypass.json` — OS notification
  action for a high-risk force-publish shortcut; `effect_class =
  mutation_via_review_path_only` with a non-null `review_sheet_ref`,
  every destructive / irreversible / bypass class explicitly
  forbidden.
