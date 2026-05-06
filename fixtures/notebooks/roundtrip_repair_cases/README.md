# Structured round-trip repair packet cases

Worked YAML fixtures for:

- [`docs/notebooks/structured_roundtrip_repair_contract.md`](../../../docs/notebooks/structured_roundtrip_repair_contract.md)
- [`schemas/notebooks/roundtrip_repair_packet.schema.json`](../../../schemas/notebooks/roundtrip_repair_packet.schema.json)

Each file is a single `structured_round_trip_repair_packet_record` and is
intended to be privacy-safe: refs, hashes, and typed vocabulary only.
Raw notebook JSON bodies, raw cell source bytes, raw outputs, raw widget
state, raw kernel protocol frames, raw paths, raw URLs, and raw
credential material do not appear.

## Cases

| File | Scenario |
|---|---|
| `unknown_fields_preserved_under_namespace_compare_first.yaml` | Unknown fields observed in a user-owned manifest; candidate repair preserves them under a declared namespace with raw preservation + compare-first gating. |
| `decoder_fallback_policy_redacted_refuse_rewrite.yaml` | Decode recovery required but raw bytes are policy-redacted; lossy repair is refused because compare/preservation requirements cannot be met. |
| `widget_downgrade_strip_widget_state_compare_first.yaml` | Widget runtime/trust downgrade; candidate strips widget state for compatibility and requires compare-first + checkpoint. |
| `manifest_normalization_warn_allow_apply.yaml` | Manifest normalization candidate that is metadata-only loss with explicit disclosure; warn-then-apply gate with raw preservation. |
| `notebook_output_stripping_compare_first_rerun_required.yaml` | Notebook output stripping candidate for recovery/size reduction; requires compare-first and declares rerun requirement. |
| `partial_open_recovery_missing_adapter_refuse_rewrite.yaml` | Partial-open context with missing adapters/runtime; repair is refused because round-trip impact cannot be proven. |
| `downgrade_compatibility_drop_unsupported_fields_compare_first.yaml` | Downgrade compatibility candidate that drops unsupported fields; compare-first + raw preservation is mandatory and loss is enumerated. |

