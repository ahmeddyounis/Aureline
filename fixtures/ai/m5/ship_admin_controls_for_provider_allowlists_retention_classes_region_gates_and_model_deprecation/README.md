# Admin Controls Fixtures

## blocked_control_narrows.json

An admin-control catalogue captured after a workspace region gate was overridden
by a higher-tier organisation policy.

The `region-gate-eu` control stays at Stable: it pins managed routes to the
`eu-west` and `eu-central` regions, denies any route outside the gate, carries an
admin-approval gate, is audited, is actively enforced, and its checkpoint rollback
is verified.

The `region-gate-apac-blocked` control demonstrates narrowing: it pins APAC routes
but is `blocked_by_higher_policy` because the organisation-wide EU pin overrides
it. Because the control is no longer live, it claims `held` rather than any public
lane, its rollback posture is `not_applicable`, and both downgrade rules narrow to
`unavailable` — so a stale proof or a provider outage keeps it out of the claimed
lanes entirely.

This fixture is the canonical example of an admin control that narrows its claim
instead of presenting an unenforced public qualification, and it round-trips
through `AdminControlPacket::validate` with no violations.
