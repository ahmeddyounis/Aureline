# Notebook output viewer truth cases

Worked YAML fixtures for
[`docs/notebooks/output_viewer_truth_contract.md`](../../../docs/notebooks/output_viewer_truth_contract.md).

Each case pairs:

- a `notebook_output_viewer_state_record` from
  [`schemas/notebooks/output_viewer_state.schema.json`](../../../schemas/notebooks/output_viewer_state.schema.json);
- a `notebook_output_include_policy_record` from
  [`schemas/notebooks/output_include_policy.schema.json`](../../../schemas/notebooks/output_include_policy.schema.json).

The fixtures intentionally carry refs, labels, hashes, and policy ids
only. Raw notebook JSON bodies, raw cell source, raw output bytes, raw
widget state, raw kernel protocol frames, raw URLs, raw hostnames, and
raw credential material do not appear.

## Cases

| File | Scenario |
|---|---|
| `live_dataframe_virtualized.yaml` | Live dataframe output bound to the current kernel, row-virtualized, user-frozen, and exported as a typed truncated report preview. |
| `captured_media_detached_support.yaml` | Captured image output from a prior session rendered through a media proxy and included in support as a detached artifact reference. |
| `stale_variable_explorer_metadata_only.yaml` | Variable explorer row whose source cell changed after capture, forcing stale metadata-only support inclusion. |
| `replayed_truncated_raw_fallback.yaml` | Captured output replayed without kernel execution and summarized through a raw/textual fallback with truncation disclosure. |
| `widget_blocked_export_omitted.yaml` | Widget output blocked by default trust, rendered as a static placeholder, and excluded from export. |
| `orphaned_output_metadata_only.yaml` | Output whose producing kernel session cannot be resolved, preserved as orphaned metadata only. |
| `intentionally_omitted_support_capture.yaml` | User intentionally omits a heavy output from support capture while preserving provenance and omission impact. |
