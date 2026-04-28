# Breadcrumb Projection Examples

These records validate against
[`schemas/navigation/breadcrumb_segment.schema.json`](../../../schemas/navigation/breadcrumb_segment.schema.json).
They exercise renderer-facing breadcrumb projection behavior while
pointing back to the durable navigation trail model.

| Fixture | Coverage |
| --- | --- |
| `file_path_current_file.json` | Path-only breadcrumb with root, folder, current file, copy/reveal actions, and no overflow. |
| `mixed_symbol_overflow.json` | Mixed path-plus-symbol breadcrumb where older folders overflow before the active file and leaf symbol lose clarity. |
| `stale_symbol_provider_unavailable.json` | Symbol breadcrumb with stale container state and unavailable leaf disclosure while preserving keyboard reachability and inspect actions. |
