# Generated-artifact timeline proof packet

The canonical generated-artifact timeline packet is implemented in
[`crates/aureline-generated/src/generated_timeline/mod.rs`](../../crates/aureline-generated/src/generated_timeline/mod.rs)
and serialized to
[`artifacts/generated/generated-timeline-packet.json`](./generated-timeline-packet.json).

It is the checked-in truth source for:

- the reviewer contract in
  [`docs/generated/generated-history.md`](../../docs/generated/generated-history.md)
- the boundary schema at
  [`schemas/generated/generated-timeline-entry.schema.json`](../../schemas/generated/generated-timeline-entry.schema.json)
- fixture replay in
  [`crates/aureline-generated/tests/generated_timeline.rs`](../../crates/aureline-generated/tests/generated_timeline.rs)
- the fixture corpus under
  [`fixtures/generated/timeline/`](../../fixtures/generated/timeline/)

## What the packet models

For one generated artifact, the packet carries a timeline of typed entries.
Each entry records how its bytes were captured — full snapshot,
metadata-plus-reference, regenerated candidate, or omitted — plus its
redaction class and its lineage links (generator identity, canonical source,
divergence state, and reversible-checkpoint lineage). One engine stamps the
restore fidelity, the exact-byte-continuity claim, the byte-provenance
explanation, the compare basis, and the restore availability the entry may
present.

## Described entries

| Entry | Class | Capture | Redaction | Divergence | Restore fidelity | Exact continuity |
| --- | --- | --- | --- | --- | --- | --- |
| `generated.timeline.scaffolded_project_full_snapshot` | scaffolded_project | `full_snapshot` | `none` | `in_sync` | `exact_snapshot` | yes |
| `generated.timeline.notebook_output_metadata_plus_reference` | notebook_output | `metadata_plus_reference` | `none` | `in_sync` | `compatible_regeneration` | no |
| `generated.timeline.framework_codegen_regenerated_candidate` | framework_codegen | `regenerated_candidate` | `none` | `drifting` | `compatible_regeneration` | no |
| `generated.timeline.preview_derivative_omitted_bytes` | preview_derivative | `omitted_bytes` | `size_capped` | `in_sync` | `evidence_only` | no |
| `generated.timeline.request_artifact_redacted_snapshot` | request_artifact | `full_snapshot` | `secrets_redacted` | `in_sync` | `compatible_regeneration` | no |
| `generated.timeline.support_packet_reference_source_missing` | support_packet | `metadata_plus_reference` | `none` | `source_missing` | `evidence_only` | no |
| `generated.timeline.ai_assisted_edit_full_snapshot` | ai_assisted_edit | `full_snapshot` | `none` | `in_sync` | `exact_snapshot` | yes |

## The frozen guardrail

Exact generated-byte continuity is claimed only when the timeline captured a
full, unredacted snapshot. A metadata-plus-reference, regenerated-candidate,
omitted, or redacted capture never lets restore or compare claim exact byte
continuity. The request-artifact entry proves the redaction case: a full
snapshot with secrets redacted is no longer faithful, so it narrows to a
compatible regeneration and drops the exact-continuity claim. The
support-packet entry proves the missing-source case: a reference with no
canonical source can no longer be regenerated, so it falls to evidence only.

## Surface bindings

Every binding ingests the same packet id
(`generated.generated_timeline.v1`) and consumes the entry outcome:

- `history_timeline` — `crates/aureline-history/src/local_history/mod.rs`
- `compare_view` — `crates/aureline-review/src/change_inspector/mod.rs`
- `restore_preview` — `crates/aureline-recovery/src/lib.rs`
- `support_export` — `crates/aureline-support/src/generated_lineage/mod.rs`
