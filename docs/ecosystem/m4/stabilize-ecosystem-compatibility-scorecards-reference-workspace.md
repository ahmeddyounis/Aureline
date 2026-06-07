# Ecosystem compatibility scorecards and reference-workspace linkage

Stable ecosystem claims now resolve through one machine-readable scorecard row
instead of per-surface prose. The canonical schema is
`schemas/ecosystem/compatibility_scorecard.schema.json`, and the first checked-in
fixtures live under
`fixtures/ecosystem/m4/stabilize-ecosystem-compatibility-scorecards-reference-workspace/`.

## What the canonical row carries

- claimed and effective parity band
- bridge parity and evidence freshness
- evidence source reference
- supported deployment and runtime profile rows
- reference-workspace ids and lineage refs
- known-gap state and linked guidance
- linked workflow bundles and imported-user handoff bundles

## Downgrade behavior

- stale or expired evidence narrows the row to `retest_pending`
- partial or approximate bridge parity narrows the row to `limited`
- unsupported bridge parity narrows the row to `unsupported`
- expired reference-workspace certification narrows the row to `retest_pending`
- missing evidence or missing certification narrows the row to `preview`

## Consumer surfaces

The first protected consumers are:

- marketplace card
- migration center report
- bridge detail view
- bundle detail view
- support export
- release claim manifest

Each consumer projection keeps the canonical row ref and mirrors its effective
parity band, freshness state, evidence source, and reference-workspace linkage.
