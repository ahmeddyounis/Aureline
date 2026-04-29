# Diagnostic remap, import, and delta fixtures

These YAML fixtures exercise the contract frozen in
[`/docs/language/diagnostic_freshness_and_delta_contract.md`](../../../docs/language/diagnostic_freshness_and_delta_contract.md)
and the boundary schemas:

- [`/schemas/language/diagnostic_remap_state.schema.json`](../../../schemas/language/diagnostic_remap_state.schema.json)
- [`/schemas/language/sarif_import_record.schema.json`](../../../schemas/language/sarif_import_record.schema.json)
- [`/schemas/language/diagnostic_delta.schema.json`](../../../schemas/language/diagnostic_delta.schema.json)

Each fixture is one concrete record. The corpus uses opaque refs and
reviewable summaries only; it does not include raw source text, raw
SARIF bodies, raw logs, raw command lines, raw paths, raw provider
payloads, or secret material.

| Fixture | Schema | Scenario |
|---|---|---|
| `remapped_inline_disclosed.yaml` | `diagnostic_remap_state` | A remapped imported finding may still render inline because the current range is disclosed as remapped and original context is available. |
| `needs_remap_after_branch_switch.yaml` | `diagnostic_remap_state` | A finding remains visible after a branch switch, but no inline range may be shown until remap review runs. |
| `sarif_provider_import_record.yaml` | `sarif_import_record` | A managed CI SARIF-like scan imports with tool/rule-pack identity, target scope, baseline family, severity mapping, and imported authority. |
| `current_imported_baseline_delta.yaml` | `diagnostic_delta` | Current live, imported, baseline, and suppressed findings compare through one delta packet. |
| `support_export_stale_delta.yaml` | `diagnostic_delta` | Support replay preserves stale, remap-needed, and unsupported-anchor rows without upgrading them to current live truth. |
