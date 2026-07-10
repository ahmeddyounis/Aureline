# M5 Protected-Path Governance Component Matrix Fixtures

These fixtures are valid, export-safe matrix packets that exercise the downgrade
behavior the canonical support export keeps green. Each one keeps every component
present, trust-review and consumer-projection invariants satisfied, proof freshness
valid, and the enforcement distinction, governance-state vocabulary, escalation
boundary, and backup-coverage fallback populated on every row — the difference is
which components are narrowed and why.

## ownership_card_backup_missing_narrowed.json

The ownership card is narrowed to Beta because owner backup coverage is missing for
the guarded path; the card labels the path `backup_missing`, keeps the primary owner
shown, and never presents the path as covered. Demonstrates the
`owner_coverage_backup_missing` downgrade trigger narrowing an ownership claim rather
than hiding the gap. The DRI-registry row and merge-readiness strip stay at their
baseline Beta and Preview maturities.

## merge_control_banner_held.json

The merge-control banner is held pending upstream merge-control graduation. Held
components do not require evidence packets; no banner claim is offered while held,
and each merge blocker stays named rather than collapsed into a generic warning. The
protected-path row, ownership card, approver matrix, review-pack summary,
public-surface diff card remain Stable and the DRI-registry row and merge-readiness
strip remain Beta and Preview.
