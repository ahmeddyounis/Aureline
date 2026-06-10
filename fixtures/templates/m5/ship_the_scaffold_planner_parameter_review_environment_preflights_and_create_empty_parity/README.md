# Scaffold Planner, Parameter Review, Preflight, and Create-Empty Parity Fixtures

These fixtures are valid, export-safe scaffold-planner packets that exercise the
downgrade behavior the canonical support export keeps green. Each keeps every
canonical plan present, the safety-review and consumer-projection invariants
satisfied, and proof freshness valid — the difference is which plan is narrowed
and why. They are regenerated from the canonical builder via
`cargo run -p aureline-scaffold --example dump_scaffold_planner`.

## parameter_unresolved_blocked.json

The ready Rust CLI plan's required parameters became unresolved, so its
parameter review narrows to `awaiting_required_input`, its readiness to
`blocked_awaiting_input`, it is withdrawn from apply, and it gains the
`required_parameter_unresolved` downgrade trigger. The plan is labeled and
blocked rather than hidden. The awaiting-input, preflight-blocked, and
create-empty plans are unchanged.

## create_empty_parity_broken.json

The create-empty workspace plan's parity with the templated flow broke, so it is
marked `parity_broken_blocked`, its readiness becomes `blocked_parity_broken`, it
is withdrawn from apply, and it gains the `create_empty_parity_broken` trigger.
The plan is labeled and blocked rather than silently downgraded to an unreviewed
shortcut. The three templated plans are unchanged.
