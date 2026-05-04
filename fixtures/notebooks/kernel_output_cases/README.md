# Notebook kernel-output lineage and widget-trust cases

Worked YAML fixtures for
[`docs/notebooks/kernel_output_lineage_and_widget_contract.md`](../../../docs/notebooks/kernel_output_lineage_and_widget_contract.md).

Each case carries:

- a `notebook_kernel_output_lineage_record` from
  [`schemas/notebooks/kernel_output_lineage.schema.json`](../../../schemas/notebooks/kernel_output_lineage.schema.json);
- where applicable, a `notebook_widget_trust_state_record` from
  [`schemas/notebooks/widget_trust_state.schema.json`](../../../schemas/notebooks/widget_trust_state.schema.json).

The fixtures intentionally carry refs, labels, hashes, and policy ids
only. Raw notebook JSON, raw cell source, raw output bytes, raw widget
state, raw kernel-protocol frames, raw URLs, raw hostnames, and raw
credential material do not appear.

## Cases

| File | Scenario |
|---|---|
| `live_trusted_output.yaml` | Live trusted output bound to the current local kernel session and current cell-execution run. |
| `replayed_output_after_kernel_loss.yaml` | Captured output replayed without re-executing the kernel after the prior session was lost. |
| `blocked_widget.yaml` | Widget output blocked by default trust; static fallback rendered; widget-trust record gated and offers a preview admission. |
| `queued_remote_execution.yaml` | Cell admitted to the queue against a remote-agent kernel; the remote transport is briefly unavailable so the queue state is `queued_waiting_on_kernel`. |
| `orphaned_output_preserved_in_review.yaml` | Output whose producing kernel session cannot be resolved; preserved as orphaned evidence in the review/export bundle. |
