# M5 Dynamic-Surface Assistive-Tech Certification Drill Fixtures

These fixtures are valid, export-safe certification packets that exercise the stale-proof,
regression, and waiver drills the canonical certification keeps green. Each one keeps a
certification row for every claimed dynamic surface family, the certification and shared
vocabulary sets intact, one certification per proof dimension, and the conformance-review,
consumer-projection, and release-posture invariants satisfied — the difference is which
surface degrades, on which dimension, and whether it auto-narrows, blocks, or is waived.
They are minted from the same seed builder as the canonical support export by
`aureline_shell_m5_dynamic_a11y_certification`.

## stale_proof_retest_pending.json

The dense collection's stale-proof downgrade evidence has fallen out of its freshness SLO.
The `stale_proof_downgrade` dimension is `stale` with a `proof_stale` cause, so the surface
status is `retest_pending` (yellow) and the gate is `auto_narrowed` to `beta`. The exact
stale-proof cause is named per dimension. Stale proof narrows but never blocks — the lane
keeps shipping at the reduced claim. Demonstrates auto-narrowing on stale proof.

## regression_blocked.json

The terminal canvas's OS accessibility bridge has gone stale and partial, regressing both
the `bridge_health` and `non_visual_summaries` dimensions. With no waiver these are unhandled
blocking regressions, so the surface status is `degraded` (red), the gate is `blocked`, the
effective claim is `held`, and the packet-level release gate blocks Stable promotion. The
cause is named, not hidden. Demonstrates the row a release/public-truth run fails on for a
screen-reader / bridge regression.

## waived_narrowed.json

The dense collection's `announcement_coverage` dimension carries a blocking regression that
is accepted under an active, disclosed waiver (scope, owner, expiry, and the `preview` claim
it accepts). The waived regression no longer blocks promotion — the surface ships
`auto_narrowed` to `preview` — while its true status stays `degraded` (red) and the cause is
named as `waived`. The dashboard names the active waiver. Demonstrates a disclosed waiver
that ships a narrowed claim without hiding the regression.
