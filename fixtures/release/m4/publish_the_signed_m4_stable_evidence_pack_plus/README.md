# M04-185 Fixture Cases

Negative fixture cases for the signed M4 stable evidence pack.

## Cases

- `narrowing_row_not_narrowed.json` — A narrowed-stale bundle keeps a Stable effective label.
- `held_on_breached_packet.json` — A signed-current bundle rides a proof packet whose SLO state is breached.
- `published_wider_than_claim.json` — A bundle records an effective label wider than its claim ceiling.
- `unsigned_without_attestation_reason.json` — An unsigned-unattested bundle lacks the `attestation_missing` gap reason.
- `missing_signoff_on_signed_row.json` — A signed-current bundle lacks owner sign-off.
