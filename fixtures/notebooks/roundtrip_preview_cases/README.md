# Structured round-trip preview sheet cases

Worked YAML fixtures for:

- [`docs/notebooks/notebook_trust_and_roundtrip_preview_contract.md`](../../../docs/notebooks/notebook_trust_and_roundtrip_preview_contract.md)
- [`schemas/notebooks/roundtrip_preview.schema.json`](../../../schemas/notebooks/roundtrip_preview.schema.json)

Each file is a single `structured_round_trip_preview_sheet_record` and is
intended to be privacy-safe: refs, hashes, and typed vocabulary only.
Raw notebook JSON bodies, raw cell source bytes, raw outputs, raw widget
state, raw kernel protocol frames, raw paths, raw URLs, and raw
credential material do not appear.

## Cases

| File | Scenario |
|---|---|
| `notebook_cell_lossless_roundtrip_local_kernel.yaml` | Fully trusted notebook cell edit preview with lossless round-trip; local kernel available. |
| `notebook_output_representation_loss_requires_rerun.yaml` | Notebook cell change that normalizes output representation; rerun required to refresh derived outputs; output trust remains captured. |
| `widget_downgraded_by_trust_static_fallback.yaml` | Widget output preview where widget trust is denied/suppressed; preview is a tombstone/static fallback with safe next actions. |
| `json_manifest_lossy_metadata_only_normalised.yaml` | JSON manifest structured preview normalizes formatting/ordering only; metadata-only loss with warning gate. |
| `yaml_manifest_comments_dropped_compare_first.yaml` | YAML manifest structured preview drops comments; compare-first review and raw preservation required. |
| `env_file_duplicate_keys_lossy_structural.yaml` | `.env` structured preview cannot preserve duplicate keys/order; structural loss requires compare-first and raw preservation. |
| `lockfile_structured_edit_policy_blocked_regenerate_first.yaml` | Lockfile edit attempt is policy-blocked; regenerate-first safe action is surfaced; apply refused. |
| `unsupported_attachment_roundtrip_unavailable_refuse.yaml` | Unsupported/unknown structured element makes risk unknown; round-trip unavailable and apply refused (tombstone preview). |
| `mixed_local_remote_kernel_trust_axes_distinct.yaml` | Mixed trust posture: document trusted, kernel unavailable (remote), output captured; preview keeps trust axes distinct. |

