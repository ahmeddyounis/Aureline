# Notebook collaboration contract cases

Worked YAML fixtures for
[`docs/notebooks/notebook_collaboration_contract.md`](../../../docs/notebooks/notebook_collaboration_contract.md).

Each case carries a `__fixture__` prelude plus a `records` array. Records
with `record_kind` values from
[`schemas/notebooks/cell_anchor.schema.json`](../../../schemas/notebooks/cell_anchor.schema.json)
and
[`schemas/notebooks/notebook_share_scope.schema.json`](../../../schemas/notebooks/notebook_share_scope.schema.json)
may appear together; downstream tooling discriminates by `record_kind`.

The fixtures intentionally carry opaque refs, labels, hashes, and policy
ids only. Raw notebook JSON, raw cell source, raw output payloads, raw
comment bodies, raw widget state, raw runtime frames, raw URLs, raw
hostnames, raw user identifiers, and raw credential material do not
appear.

## Cases

| File | Scenario |
|---|---|
| `comment_anchor_cross_cell_review.yaml` | Comment anchor drift finds a cross-cell candidate but preserves the original anchor until explicit review; silent cross-cell rebind is denied. |
| `output_anchor_reexecution_review.yaml` | Output anchor sees a rerun candidate and records a user-confirmed output rebind rather than silently replacing the output target. |
| `presenter_focus_captured_runtime.yaml` | Presenter focus points followers at a captured output while the remote kernel is disconnected; focus is view-only and labeled captured. |
| `redaction_before_export_share.yaml` | Export share scope completes redaction before share, omitting transient runtime state and redacting secret-bearing output. |
| `offline_review_stale_overlay.yaml` | Offline review preserves local notebook readability while collaboration overlays and runtime state are stale. |
| `shared_live_managed_kernel_scope.yaml` | A shared-live collaboration scope discloses the managed runtime boundary, redaction review, and metadata-only provider resources. |
