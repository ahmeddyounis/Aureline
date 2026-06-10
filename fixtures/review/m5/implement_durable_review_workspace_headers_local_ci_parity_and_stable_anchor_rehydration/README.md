# Durable Review Header, Local-CI Parity, and Anchor Rehydration Fixtures

These fixtures are valid, export-safe packets that exercise the labeling and
narrowing behavior the canonical support export keeps green. Each one keeps the
trust-review and consumer-projection invariants satisfied and proof freshness
valid — the difference is which states are narrowed and why.

## stale_base_anchor_drift.json

The header is on a stale base (`stale_base`) and its approval has been reset on
base change; the anchor rehydrated as `orphaned_flagged` after the base change and
carries an explicit drift label. Demonstrates that stale-base and anchor-drift
states are labeled rather than hidden, and that a header's durable anchor still
has a rehydration record even when the anchor is orphaned.

## ci_parity_offline.json

CI status is unavailable because the workspace is offline, so the required check
lanes report `unavailable_offline` and the local results stay advisory. The
header remains `current` with its anchor rehydrated exactly. Demonstrates that
parity is explicit even when CI has not reported — the verdict says
`unavailable_offline` rather than implying a pass.
