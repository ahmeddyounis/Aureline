# Fixtures: finalize-qualification-rows-for-desktop-local-remote-helper

These fixtures document the stable qualification-matrix proof packet for
desktop local, remote/helper, provider-linked, state/schema, and
accessibility surfaces. The canonical source of truth is the seeded packet
produced by
`aureline_remote::finalize_qualification_rows_for_desktop_local_remote_helper::seeded_qualification_matrix_page()`.

## Files

| File | Content |
|------|---------|
| `page.json` | Full `QualificationMatrixPage` proof packet (stable, zero defects) |
| `rows.json` | All 22 `QualificationMatrixRow` records |
| `defects.json` | Empty defect list (clean stable packet) |
| `summary.json` | `QualificationMatrixSummary` with counts and overall qualification |
| `support_export.json` | `QualificationMatrixSupportExport` envelope for support/diagnostics |
| `drills/missing_row_preview.json` | Drill: missing `desktop_local:local_oss` row narrows to `preview` |
| `drills/raw_material_withdrawn.json` | Drill: raw private material on `desktop_local:managed` row withdraws packet |
| `drills/no_local_core_continuity_beta.json` | Drill: `remote_helper:managed` row without local-core continuity narrows to `beta` |

## Required rows

All 22 required rows must be present for a stable claim:

**Surface × profile (16 rows):**
`desktop_local:local_oss`, `desktop_local:self_hosted`, `desktop_local:managed`, `desktop_local:air_gapped`,
`remote_helper:local_oss`, `remote_helper:self_hosted`, `remote_helper:managed`, `remote_helper:air_gapped`,
`provider_linked:local_oss`, `provider_linked:self_hosted`, `provider_linked:managed`, `provider_linked:air_gapped`,
`state_schema:local_oss`, `state_schema:self_hosted`, `state_schema:managed`, `state_schema:air_gapped`.

**Accessibility features (6 rows):**
`accessibility:keyboard`, `accessibility:screen_reader`, `accessibility:ime_grapheme_bidi`,
`accessibility:zoom`, `accessibility:high_contrast`, `accessibility:reduced_motion`.

## Schema

`schemas/enterprise/finalize-qualification-rows-for-desktop-local-remote-helper.schema.json`

## Contract ref

`remote:qualification_matrix:desktop:v1`
