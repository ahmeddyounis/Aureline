# Convention-Diagnostic Fixtures

These fixtures are valid, export-safe convention-diagnostic packets that exercise
the downgrade behavior the canonical support export keeps green. Each keeps every
canonical row present, the review and consumer-projection invariants satisfied,
and proof freshness valid — the difference is which diagnostic is narrowed and
why. They are regenerated from the canonical builder via
`cargo run -p aureline-templates --example dump_convention_diagnostics`.

## proving_file_unavailable_blocked.json

The high-confidence model-naming diagnostic's proving file can no longer be
disclosed, so its proving disclosure narrows to `proving_file_unavailable`, its
confidence to `confidence_unknown`, its proving-file refs are cleared, and its
downgrade banner becomes `proving_file_unavailable_banner`. The diagnostic is
withdrawn from confident display and gains the `proving_file_unavailable`
downgrade trigger. It is labeled and blocked rather than hidden or presented as a
grounded match. The exact, heuristic, suppressed, unavailable, and bridged rows
are unchanged.

## confidence_unverified_withheld.json

The exact controller-file-location diagnostic's confidence could not be verified,
so its confidence narrows to `confidence_unknown`, it gains a confidence banner,
it is withdrawn from confident display, and it gains the `confidence_degraded`
downgrade trigger. An unverified confidence is never presented as exact truth. The
high-confidence, heuristic, suppressed, proving-unavailable, and bridged rows are
unchanged.
