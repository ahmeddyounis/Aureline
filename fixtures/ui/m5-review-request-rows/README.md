# Review-Request Row Fixtures

These fixtures are valid, export-safe review-request row packets that exercise the
labeling and narrowing behavior the canonical support export keeps green. Each one
keeps the trust-review and consumer-projection invariants satisfied, covers the
local-estimate / provider-backed / offline-exported backing kinds so the three are
distinguishable from the row alone, and keeps proof freshness valid — the
difference is which states are narrowed and why.

## provider_stale_local_continue.json

A provider-backed pull request whose provider truth has gone `provider_stale` on a
`stale_base`. The row still claims hosted status (`claims_provider_backed: true`) —
the stale provider is **not** flattened into a local estimate — but it preserves a
`local_continue_fallback` and offers `refresh_provider_truth` plus
`continue_local_review` so ordinary triage never forces raw-provider navigation.

## browser_handoff_placeholder.json

A `browser_handoff_placeholder` row whose hosted status lives only behind a
provider deep link. It carries an explicit `browser_handoff_boundary` and never
pretends hosted status exists (`claims_provider_backed: false`). The packet also
includes a live provider-backed row, a local estimate, and an offline export so the
placeholder reads distinctly against the other three backing kinds.
