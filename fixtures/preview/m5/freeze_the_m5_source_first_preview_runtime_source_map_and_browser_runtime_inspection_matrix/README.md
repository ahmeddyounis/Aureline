# M5 Source-First Preview / Browser-Runtime Inspection Matrix Fixtures

## support_export_row_narrows_on_unidentified_mapping_quality.json

An auto-narrowing drill fixture for the source-first preview / browser-runtime
inspection matrix. Every claimed M5 preview/runtime surface — source-first
framework preview, visual-surface mapping, browser-runtime inspection, device or
simulator preview, full-stack preview loop, embedded webview preview, visual-edit
transform, and support/export projection — carries its preview-session class,
source-sync state, target kind, mapping-quality class, browser-runtime attach
depth, and round-trip capability.

The support/export projection row claims `beta`, but its mapping-quality class is
not yet identified (`mapping_quality` is absent). Because a claimed row may not
outrun identified evidence, the row auto-narrows to `effective` `held`, records an
`unidentified_mapping_quality` narrowing trigger, and carries a precise degraded
label rather than a generic provider error. Every other row identifies all three
of its source-sync state, target kind, and mapping-quality class, so its effective
qualification equals its claim. The browser-runtime inspection row is
`runtime_only_no_source` and so declares `runtime_backed` true while never claiming
to be saved source state, and the inspect-only and source-only-fallback rows stay
non-write-capable.

The fixture validates against
`schemas/preview/freeze-the-m5-source-first-preview-preview-runtime-source-map-and-browser-runtime-inspection-matrix.schema.json`.
