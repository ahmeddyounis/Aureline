# M5 Bridge and Announcement Drill Fixtures

These fixtures are valid, export-safe diagnostics reports that exercise the
bridge/announcement/visual drills the canonical support export keeps green. Each one keeps
a diagnostics row for every protected surface family, the shared and diagnostics
vocabulary sets intact, one check per diagnostic class, and the conformance-review,
consumer-projection, and release-posture invariants satisfied — the difference is which
surface degrades, how, and whether it auto-narrows or fails the release gate. They are
minted from the same seed builder as the canonical export by
`aureline_shell_m5_dynamic_a11y_diagnostics`.

## bridge_unavailable_narrowed.json

The editor canvas's OS accessibility bridge is unavailable. The surface auto-narrows from
Stable to Beta, drops its non-visual fidelity to `degraded_accessible`, marks its
`bridge_health` and `missing_semantic_node` checks `auto_narrowed`, discloses the degraded
state, and keeps its `bridge_unavailable` downgrade trigger. The narrowing is honest, so
the per-surface gate passes and the report-level release gate stays green — the surface
keeps shipping at the narrowed claim rather than disappearing. Demonstrates auto-narrowing.

## bridge_regression_blocked.json

The terminal canvas's bridge mapping has gone stale and partial, but the surface still
over-claims Stable. The `bridge_health` and `missing_semantic_node` checks are unhandled
blocking regressions, so the per-surface gate and the report-level release gate both
block. Demonstrates the row a release/public-truth run fails on for a bridge regression.

## announcement_spam_blocked.json

The dense collection's live region floods past its announcement budget
(`within_budget: false`). The `announcement_rate` and `coalescing_violation` checks are
unhandled blocking regressions, so the gate blocks. The bridge stays healthy, so no
degraded bridge state is claimed. Demonstrates the row a release/public-truth run fails on
for announcement spam.

## visual_regression_blocked.json

The review/diff surface regresses under forced colors. The `high_contrast_regression`
check is an unhandled blocking regression (the gate blocks for a contrast breakage); the
`reduced_motion_regression` check also regresses but is advisory, so it is recorded
without gating. Demonstrates that zoom/contrast/motion breakage fails the gate while
advisory findings do not.
