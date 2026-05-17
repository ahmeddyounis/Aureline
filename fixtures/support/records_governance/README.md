# Records-governance packet fixtures

Each fixture is one `records_governance_packet_record` envelope
matching the boundary schema at
[`/schemas/support/record_class.schema.json`](../../../schemas/support/record_class.schema.json).
The fixtures exercise every value of the closed `artifact_class`
vocabulary:

| Fixture | Artifact class | Record class | Hold state | Destruction caveat |
| ------- | -------------- | ------------ | ---------- | ------------------ |
| `local_only_workspace_state.json` | `local_only` | `durable_workspace_state` | `none` | `none` |
| `managed_copy_index.json` | `managed_copy` | `managed_copy_index_entry` | `none` | `retained_subset_remains` |
| `held_support_bundle.json` | `held` | `support_bundle_archive` | `on_hold` | `legal_hold_prevents` |
| `queued_for_delete_offboarding.json` | `queued_for_delete` | `offboarding_exit_packet` | `none` | `provider_backlog` |
| `deleted_support_bundle_archive.json` | `deleted` | `support_bundle_archive` | `none` | `none` |
| `retained_destruction_receipt.json` | `retained_for_evidence` | `destruction_receipt_record` | `none` | `receipt_retained` |
| `export_only_usage_packet.json` | `export_only` | `entitlement_usage_export_packet` | `none` | `none` |

These fixtures are also the canonical replay set for
`crates/aureline-support/tests/records_governance_beta.rs`. Adding a
new acceptance class requires both a fixture and a typed branch in
the evaluator.
