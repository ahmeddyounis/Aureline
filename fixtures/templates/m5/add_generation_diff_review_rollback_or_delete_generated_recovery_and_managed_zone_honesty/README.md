# Generation Diff Review, Recovery, and Managed-Zone Honesty Fixtures

These fixtures are valid, export-safe generation-recovery packets that exercise
the downgrade behavior the canonical support export keeps green. Each keeps
every canonical row present, the review and consumer-projection invariants
satisfied, and proof freshness valid — the difference is which row is narrowed
and why. They are regenerated from the canonical builder via
`cargo run -p aureline-templates --example dump_generation_recovery`.

## lineage_unknown_blocked.json

The clean first-party generated row's scaffold lineage can no longer be
resolved, so its managed zone narrows to `zone_unknown_review_required`, its diff
to `diff_unavailable_review_required`, its overwrite to
`overwrite_blocked_lineage_unknown`, and its recovery to
`recovery_blocked_lineage_unknown`. The row is withdrawn from recovery and gains
the `lineage_unknown` downgrade trigger. It is labeled and blocked rather than
hidden or silently overwritten. The mixed, bridge, and imported rows are
unchanged.

## authored_protection_quarantined.json

The mixed authored/generated row's authored-content protection could not be
verified, so its `delete_generated_only` recovery is downgraded to
`quarantine_generated`, it is withdrawn from recovery, and it gains the
`authored_protection_unverified` downgrade trigger. Authored content is never
deleted on an unverified protection. The clean, bridge, and imported rows are
unchanged.
