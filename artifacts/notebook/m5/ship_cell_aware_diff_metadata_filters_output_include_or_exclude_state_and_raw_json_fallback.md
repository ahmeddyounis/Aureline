# Artifact: Ship cell-aware diff, metadata filters, output include or exclude state, and raw JSON fallback

## Packet

- **Path**: `artifacts/notebook/m5/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback.json`
- **Schema version**: 1
- **Record kind**: `notebook_diff_packet`
- **As of**: 2026-06-09T00:00:00Z

## Closed vocabularies

### Diff modes

- `cell_aware` — default structured review
- `metadata_focused` — metadata/namespace drift analysis
- `output_aware` — figure/table/output review
- `raw_json_fallback` — escape hatch when semantic review is unavailable

### Cell change classes

- `cell_added`
- `cell_removed`
- `cell_reordered`
- `cell_type_changed`
- `source_changed`
- `metadata_changed`
- `output_changed`
- `execution_changed`
- `unchanged`

### Output change classes

- `output_added`
- `output_removed`
- `output_updated`
- `output_unchanged`

### Metadata filter states

- `all_visible`
- `official_only`
- `aureline_only`
- `unknown_hidden`

### Output include states

- `included`
- `excluded`
- `collapsed`

### Raw JSON fallback reasons

- `parse_error`
- `unsupported_version`
- `extension_mismatch`
- `explicit_user_choice`
- `corrupt_structure`

### Merge resolution classes

- `base`
- `ours`
- `theirs`
- `result`
- `unresolved`

## Invariants

1. A review session in `raw_json_fallback` mode MUST carry a
   `raw_json_fallback_ref`.
2. A review session NOT in `raw_json_fallback` mode MUST NOT carry a
   `raw_json_fallback_ref`.
3. Metadata filters are review conveniences; `unknown_namespaces_preserved_on_save`
   MUST be `true` when the filter hides unknown namespaces.
4. Output include/exclude state MUST remain visible on every review surface.
5. Raw JSON fallback MUST include a human-readable `fallback_explanation`.

## Downstream consumers

- `crates/aureline-notebook` — canonical record definitions and validators
- `crates/aureline-review` — diff/review surface integration
- `crates/aureline-collab` — collaboration anchor and share-scope integration
- `docs/notebook/m5/ship_cell_aware_diff_metadata_filters_output_include_or_exclude_state_and_raw_json_fallback.md` — human-readable spec
