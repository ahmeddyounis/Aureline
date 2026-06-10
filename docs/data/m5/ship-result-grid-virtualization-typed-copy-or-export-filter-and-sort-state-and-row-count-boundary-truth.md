# Result-grid virtualization, typed copy or export, filter and sort state, and row-count boundary truth

## Scope

This document describes the canonical M5 qualification packet for result-grid virtualization, typed copy or export, filter and sort state, and row-count boundary truth in Aureline.

## Truth sources

- Implementation: `crates/aureline-api/src/ship_result_grid_virtualization_typed_copy_or_export_filter_and_sort_state_and_row_count_boundary_truth/mod.rs`
- Schema: `schemas/data/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.schema.json`
- Checked-in packet: `artifacts/data/m5/ship-result-grid-virtualization-typed-copy-or-export-filter-and-sort-state-and-row-count-boundary-truth.json`
- Fixtures: `fixtures/data/m5/ship_result_grid_virtualization_typed_copy_or_export_filter_and_sort_state_and_row_count_boundary_truth/`

## Surface claims

| Surface | Claim | Displayed | Rationale |
|---|---|---|---|
| Result grid viewer | stable | stable | Virtualizes rows and columns, blocks active content, and shows truncation, export, and row-count truth. |
| Typed copy action | stable | stable | Preserves column types, discloses truncation, and carries provenance. |
| Typed export action | stable | stable | Supports CSV, JSON Lines, Parquet, and notebook handoff with truncation disclosure and provenance. |
| Filter and sort state panel | stable | stable | Shows predicate count, sort key count, and filter evaluation locus including client-side-only disclosure. |
| Row-count boundary chip | stable | stable | Discloses exact total, exact returned, or streaming unknown state so 'Showing 1000 rows' never silently hides an unknown total. |

## Downgrade rules

- All promoted surfaces have `downgrade_if_missing: true`.
- Missing proof on a stable claim narrows the surface to `preview` instead of inheriting a generic label.

## Redaction and privacy

- Result grids virtualize by default and enforce row and cell size limits.
- Active content in cells is blocked; large cells open in detail panes.
- Typed copy and export actions disclose truncation state so partial results are never silently complete.
- Export and notebook handoff carry provenance chips for downstream traceability.
- Filter and sort state panels disclose client-side-only filter locus so users do not confuse visible-row filtering with full-result filtering.
- Row-count boundary chips cite the truth class so streaming or capped results are not misread as complete.

## Verification

Run `cargo check -p aureline-api` to verify the embedded packet deserializes and validates.
