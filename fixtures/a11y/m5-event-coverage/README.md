# M5 Event-Class Coverage Fixtures

These fixtures are valid, export-safe event-coverage catalogs that exercise the
auto-narrowing behavior the canonical support export keeps green. Each one keeps a
coverage row for every governed event family, the shared, announcement, and coverage
vocabulary sets intact, and the conformance-review, consumer-projection, and
release-posture invariants satisfied — the difference is which family narrows and why.
They are minted from the same seed builder as the canonical export by
`aureline_shell_m5_event_coverage`.

## proof_stale_narrowed.json

The AI/patch-review family's assistive-tech proof has gone stale. The family narrows
from Stable to Beta and keeps its `proof_stale` downgrade trigger, so the gap is
disclosed while every event stays present with its grammar class, concise identity,
blocked/degraded disclosure, and reopenable durable fallback intact. Demonstrates that
stale proof narrows the claim rather than hiding the family.

## bridge_unavailable_narrowed.json

The terminal-boundary family's OS accessibility bridge is unavailable. The family
narrows from Stable to Preview, drops its `non_visual_fidelity` to
`degraded_accessible`, and keeps its `bridge_unavailable` downgrade trigger. Its
`event:terminal.boundary-unavailable` event still narrates the unavailable reason on a
reopenable notification-center entry. Demonstrates that when the bridge is gone the
boundary events still disclose their unavailable reason, rather than the meaning being
dropped.
