# M5 Diagnostic-Truth Lane Fixtures

## row_downgrades_on_unlinked_quality_session.json

An auto-downgrade regression fixture for the M5 diagnostic-truth lane matrix.
Every claimed M5 diagnostic-producing surface — notebook cells, framework packs,
request / API tooling, data tooling, preview runtime, package lane, language
provider, editor-structural guard, and imported scanner — carries its source
kind, imported-versus-live origin class, freshness class, anchor-remap state,
collection-completeness class, cluster-meaning class, and the governing
quality-session outcome.

The data-tooling row claims `beta`, but no governing quality session yet binds
its mutating fix routes (`quality_session_outcome_class` is absent). Because a
claimed row may not outrun identified evidence, the row auto-downgrades to
`effective` `held`, records an `unlinked_quality_session` downgrade trigger, and
carries a precise degraded label rather than a generic provider error. Every
other row identifies a source, origin, proven freshness, remap state, collection
completeness, and a quality-session outcome, so its effective qualification
equals its claim.

The imported-scanner row keeps its `imported_snapshot` origin, `imported_static`
anchors, and `imported_snapshot_set` completeness explicit, and never lets
imported evidence read as live local truth; the preview-runtime row discloses its
`contextual` remap; the package-lane row discloses its `unmapped` range; the
notebook row keeps its `partial_visible_scan` collection visible; and the
language-provider row's `exact_duplicate` clustering preserves each member's
source, freshness, and remap provenance. Every row keeps anchor remap
append-only, preserves target / environment / policy refs, and routes mutating
fixes through the typed quality-action proposal contract.

The fixture validates against
`schemas/quality/m5-diagnostic-truth-lane.schema.json` and is byte-identical to
the checked support export at
`artifacts/m5/diagnostics/freeze-packet/support_export.json`.
