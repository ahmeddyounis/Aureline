# Stable Dense-Collection Contract

This document describes the stable dense-collection packet implemented in
`aureline-collections`. The packet is the shared contract for filter ASTs,
saved views, query history, scope packs, column presets, scope counters,
selection state, and batch-review sheets across search, provider-backed
review/admin, package/test/diagnostics, and notebook data-grid surfaces.

## Contract

- Portable filter truth is the typed `CollectionFilterAst`; free text may be
  displayed, but source ownership, hidden narrowing, redaction posture, and
  fallback labels remain machine-readable.
- Saved views carry filter, column, sort, scope, owner, privacy, schema, and
  fallback state. They do not carry transient selection, stale provider cursors,
  raw secret material, or ambient authority.
- Scope counters preserve the governed vocabulary: `visible`, `loaded`,
  `matching`, `selected`, `approx.`, `exact`, `hidden by policy`, and
  `outside current filter`.
- Select-all controls declare whether they mean visible rows, loaded rows, all
  matching rows after deliberate expansion, an explicit identity set, or a
  provider-side query after review.
- Remote, provider-owned, destructive, and export/share batch actions require a
  `BatchReviewSheet` before execution. Sheets preserve included, excluded,
  blocked, hidden, stale, and mixed-result aftermath state per item.
- Reopening a saved view or shared link keeps captured-versus-current scope
  honest through `ReopenScopePosture`, including prior query snapshots.
- Export, support, CLI/headless, UI, and accessibility projections consume the
  same packet and may not recompute or rename count truth.

## Canonical Artifacts

- Packet: `artifacts/collections/m4/stabilize-filter-ast-saved-view-scope-pack-column-preset.json`
- Human artifact: `artifacts/collections/m4/stabilize-filter-ast-saved-view-scope-pack-column-preset.md`
- Fixtures: `fixtures/collections/m4/stabilize-filter-ast-saved-view-scope-pack-column-preset/`

## Verification

Run:

```sh
cargo test -p aureline-collections
```

The tests validate the seeded packet, the checked-in artifact, pinned vocabulary
tokens, missing notebook/data-grid coverage, and missing protected batch-review
state.
