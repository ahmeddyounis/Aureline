# Filesystem mutation-lineage matrix report

Artifact packet:
[`artifacts/state/filesystem_mutation_lineage_matrix.json`](./filesystem_mutation_lineage_matrix.json)

Schema:
[`schemas/state/filesystem_mutation_lineage_matrix.schema.json`](../../schemas/state/filesystem_mutation_lineage_matrix.schema.json)

Reviewer doc:
[`docs/state/filesystem_mutation_lineage_matrix.md`](../../docs/state/filesystem_mutation_lineage_matrix.md)

Fixture manifest:
[`fixtures/state/filesystem_mutation_lineage_matrix/manifest.yaml`](../../fixtures/state/filesystem_mutation_lineage_matrix/manifest.yaml)

## Summary

The packet freezes one matrix for:

- canonical filesystem identity versus generated/provider/imported identity,
- watch fidelity and save fallback posture,
- mutation-journal and undo honesty,
- corruption routing by class, and
- deferred-intent / reconnect reconciliation posture.

## Packet rows

| Row | Canonical filesystem identity | Watch guarantee | Writable save target | Mutation journal | Deferred / reconcile exposure |
| --- | --- | --- | --- | --- | --- |
| `notebook_document` | yes | yes | yes | yes | no |
| `notebook_output_artifact` | no | no | no | yes | no |
| `request_workspace_document` | yes | yes | yes | yes | yes |
| `request_response_snapshot` | no | no | no | no | no |
| `database_export_artifact` | no | no | no | yes | no |
| `profiler_trace_artifact` | no | no | yes | yes | no |
| `preview_output_artifact` | no | no | no | yes | no |
| `sync_packet_artifact` | no | no | yes | yes | yes |
| `provider_local_draft` | no | no | yes | yes | yes |
| `infrastructure_overlay_document` | no | no | no | no | yes |
| `imported_archive_capture` | no | no | no | no | no |

## Frozen invariants

1. Presentation path, logical identity, canonical target, alias set, and save target remain distinct when the root can express them.
2. Degraded watch or save semantics are visible before or during mutation.
3. Material mutations emit one attributable journal entry with an explicit undo class.
4. Corruption degrades by class and row instead of forcing a whole-app reset.
5. Deferred managed work requires revalidation or review and never replays invisibly after reconnect or policy refresh.
