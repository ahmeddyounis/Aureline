# M5 Normalized Diagnostic-Record Fixtures

## normalized_record_set.json

A conformance fixture for the M5 normalized diagnostic-record set
(`NormalizedDiagnosticRecordSetPacket`). It carries one normalized finding per M5
finding surface — notebook cell, framework pack, request/API tooling, data
tooling, preview runtime, package lane, language provider, editor-structural
guard, and imported scanner — each reusing the canonical v1 diagnostic record
rather than a second diagnostic store.

Every entry carries a reopen handle for the editor, Problems, review,
CLI/headless, AI evidence, and support export that resolves to the same canonical
diagnostic id; a stable-identity family whose observations all resolve to the same
id and anchor family across ordinary refresh, adapter refresh, surface hop, and
presentational change; and any suppression / baseline join reflected on the
record's own refs. The imported-scanner entry keeps its `imported_snapshot`
origin, `imported_static` anchors, and a governed, release-visible suppression
join; the framework-pack entry carries a compatible baseline join; the
preview-runtime entry discloses its `contextual` remap; and the package-lane entry
discloses its `unmapped` range.

The data-tooling entry is the auto-downgrade case: it omits the AI-evidence reopen
handle, so it auto-downgrades from `beta` to `held` with a `missing_reopen_surface`
trigger and a precise degraded label rather than a generic provider error. Every
other entry's effective qualification equals its claim.

The fixture validates against
[`schemas/quality/diagnostic-record.schema.json`](../../../../schemas/quality/diagnostic-record.schema.json)
and is byte-identical to the checked support export at
[`artifacts/m5/diagnostics/diagnostic-record-proof/support_export.json`](../../../../artifacts/m5/diagnostics/diagnostic-record-proof/support_export.json).
