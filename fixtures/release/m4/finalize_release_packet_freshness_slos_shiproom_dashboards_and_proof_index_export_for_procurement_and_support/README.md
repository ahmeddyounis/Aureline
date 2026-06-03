# M04-183 Fixture Cases

Negative fixture cases for the release-packet freshness SLOs, shiproom dashboards,
and proof-index export register.

## Cases

- `narrowing_row_not_narrowed.json` — A narrowed-stale row keeps a Stable effective label.
- `held_on_breached_packet.json` — A current row rides a proof packet whose SLO state is breached.
- `claim_label_ceiling_mismatch.json` — A row records a claim label wider than the stable claim manifest publishes (cross-artifact check).
- `stale_alarm_without_narrowing.json` — A row with a stale-report alarm does not narrow below the cutline.
- `downgrade_pending_not_narrowed.json` — A row with downgrade propagation pending does not narrow below the cutline.
