# Update-ready review cases

Worked fixtures for the update-ready review contract:

- `normal_signed_update.yaml` - verified in-place stable update with no
  known extension impact and a visible rollback path.
- `side_by_side_channel_change.yaml` - staged Preview side-by-side
  install with separate state roots and uncertified forecast rows.
- `blocked_policy_update.yaml` - update artifacts verify, but policy
  blocks apply and restart.
- `migration_required_update.yaml` - signed update generates migration
  tasks and blocks restart until review.
- `rollback_ready_emergency.yaml` - emergency last-known-good path with
  break-glass refs and rollback-ready forecast rows.

Each case embeds an `update_ready_review_record` and the referenced
`extension_impact_forecast_record` rows.
