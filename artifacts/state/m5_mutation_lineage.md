# M5 mutation-lineage packet

The canonical M5 mutation-lineage packet is implemented in
[`crates/aureline-reactive-state/src/m5_mutation_lineage/mod.rs`](../../crates/aureline-reactive-state/src/m5_mutation_lineage/mod.rs)
and serialized to
[`artifacts/state/m5_mutation_lineage.json`](./m5_mutation_lineage.json).

It is the checked-in truth source for:

- cross-surface history inspection in
  [`crates/aureline-shell/src/m5_mutation_history_inspector/mod.rs`](../../crates/aureline-shell/src/m5_mutation_history_inspector/mod.rs)
- metadata-safe support export in
  [`crates/aureline-support/src/m5_mutation_lineage/mod.rs`](../../crates/aureline-support/src/m5_mutation_lineage/mod.rs)
- fixture replay in
  [`crates/aureline-reactive-state/tests/m5_mutation_lineage.rs`](../../crates/aureline-reactive-state/tests/m5_mutation_lineage.rs)

## Frozen evidence

The packet proves:

- one entry per material mutation across notebook, request, data,
  preview, sync, repair, provider, workflow, profiler, AI-evidence,
  and incident surfaces
- one visible reversal vocabulary:
  `exact`, `grouped_exact`, `compensate`, `regenerate`, `manual`,
  `audit_only`
- one lineage-thread join model that keeps follow-on evidence and repair
  work attributable to the original checkpoint chain
- one support-export grouping model that preserves file count,
  artifact class, and automation or policy influence without raw
  payload bodies

## Fixture corpus

The fixture corpus under
[`fixtures/state/m5_mutation_lineage/`](../../fixtures/state/m5_mutation_lineage/)
pins five lineage-root scenarios:

| Fixture | Lineage root | Primary surface | Highest risk reversal |
| --- | --- | --- | --- |
| `notebook_execution.json` | notebook execution | notebook document | `audit_only` |
| `request_batch.json` | request batch | request workspace | `grouped_exact` |
| `preview_publish.json` | preview publish | preview output | `regenerate` |
| `provider_sync.json` | provider sync and repair | sync packet | `manual` |
| `incident_capture.json` | profiler and incident | profiler trace | `audit_only` |

## Export posture

Every support-export row produced from this packet keeps:

- `raw_payload_excluded = true`
- `raw_private_material_excluded = true`
- `ambient_authority_excluded = true`
- `single_lineage_thread_preserved = true`

The export remains explanatory without widening authority or embedding
the mutated content.
