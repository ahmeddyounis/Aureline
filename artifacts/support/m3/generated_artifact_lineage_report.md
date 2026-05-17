# Generated-artifact lineage and drift-state baseline report

This artifact is the reviewer-facing baseline rendering of the
generated-artifact lineage report produced by the
[`generated_lineage`](../../../crates/aureline-reactive-state/src/generated_lineage/mod.rs)
module from the protected corpus under
[`/fixtures/state/generated_artifacts_beta/`](../../../fixtures/state/generated_artifacts_beta/).
It records the consumer surface, artifact family, lineage class,
drift state, default edit posture, downgrade label, and open-gap
classes for every generated-artifact beta surface decision in the
corpus. The report stays metadata-safe: it never carries raw payload
bytes, raw private material, or ambient authority, and every row is
drawn from the closed lineage-packet vocabularies.

Schema: `schemas/state/generated_artifact.schema.json`
(record kind `generated_artifact_lineage_report_record`, version 1).
Reviewer doc: [`docs/state/m3/generated_artifact_lineage_beta.md`](../../../docs/state/m3/generated_artifact_lineage_beta.md).
Corpus manifest:
[`fixtures/state/generated_artifacts_beta/manifest.yaml`](../../../fixtures/state/generated_artifacts_beta/manifest.yaml).

## Matrix rows

| Packet ID | Surface | Artifact ref | Family | Lineage | Drift | Edit posture | Downgrade | Open gaps |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `lineage:ai_context:build_output:stale_generated` | `ai_context` | `workspace:demo/target/debug/greet` | `build_output` | `generated_from_local_source` | `stale_generated` | `read_only_generated` | `yellow_drift_pending` | `regen_pending` |
| `lineage:ai_context:preview:aligned` | `ai_context` | `workspace:demo/.aureline/preview/docs/readme.html` | `preview_render` | `previewed_from_local_source` | `aligned` | `regenerate_only` | `none` | `none` |
| `lineage:review:build_output:source_drifted` | `review` | `workspace:demo/target/debug/greet` | `build_output` | `generated_from_local_source` | `source_drifted` | `read_only_generated` | `yellow_drift_pending` | `regen_pending` |
| `lineage:review:lockfile:aligned` | `review` | `workspace:demo/Cargo.lock` | `lockfile` | `regenerable_lockfile_artifact` | `aligned` | `regenerate_only` | `none` | `none` |
| `lineage:search:build_output:aligned` | `search` | `workspace:demo/target/debug/greet` | `build_output` | `generated_from_local_source` | `aligned` | `read_only_generated` | `none` | `none` |
| `lineage:search:build_output:imported_external` | `search` | `workspace:demo/vendor/acme/libacme.so` | `build_output` | `imported_external_artifact` | `imported_no_local_source` | `imported_read_only` | `none` | `none` |
| `lineage:support_export:notebook:aligned` | `support_export` | `workspace:demo/notebooks/analysis.ipynb#cell-2:out` | `notebook_output` | `derived_from_run_artifact` | `aligned` | `transient_run_artifact` | `none` | `none` |
| `lineage:support_export:run_result:aligned` | `support_export` | `workspace:demo/.aureline/runs/test/2026-05-16T09-50/junit.xml` | `run_result_artifact` | `derived_from_run_artifact` | `aligned` | `transient_run_artifact` | `none` | `none` |

## Per-consumer-surface summary

| Consumer surface | Packets | Aligned | Drift | Imported | Downgrade required |
| --- | --- | --- | --- | --- | --- |
| `search` | 2 | 1 | 0 | 1 | 0 |
| `review` | 2 | 1 | 1 | 0 | 1 |
| `ai_context` | 2 | 1 | 1 | 0 | 1 |
| `support_export` | 2 | 2 | 0 | 0 | 0 |

## Per-artifact-family summary

| Artifact family | Packets | Aligned | Drift | Downgrade required |
| --- | --- | --- | --- | --- |
| `build_output` | 4 | 2 | 2 | 2 |
| `lockfile` | 1 | 1 | 0 | 0 |
| `preview_render` | 1 | 1 | 0 | 0 |
| `notebook_output` | 1 | 1 | 0 | 0 |
| `run_result_artifact` | 1 | 1 | 0 | 0 |

## Per-lineage-class summary

| Lineage class | Packets | Aligned | Drift | Downgrade required |
| --- | --- | --- | --- | --- |
| `generated_from_local_source` | 3 | 1 | 2 | 2 |
| `regenerable_lockfile_artifact` | 1 | 1 | 0 | 0 |
| `previewed_from_local_source` | 1 | 1 | 0 | 0 |
| `derived_from_run_artifact` | 2 | 2 | 0 | 0 |
| `imported_external_artifact` | 1 | 1 | 0 | 0 |

(Imported rows count under `aligned` for the family / lineage summaries
because `imported_no_local_source` is the healthy state for the
`imported_external_artifact` lane; the consumer-surface summary breaks
imported out separately so support reviewers can audit imported rows
as a class.)

## Open gaps

- `lineage:ai_context:build_output:stale_generated` (`regen_pending`):
  generated build output is older than the configured freshness
  window; AI context should regenerate before quoting it as a derived
  fact.
- `lineage:review:build_output:source_drifted` (`regen_pending`):
  source manifest changed since the last build; reviewers must
  regenerate before treating the build output as truth.

## Safety baseline

- `raw_payload_excluded = true` on every packet and on the report.
- `raw_private_material_excluded = true` on every packet and on the
  report.
- `ambient_authority_excluded = true` on every packet and on the
  report.
- `destructive_resets_present = false` on every packet.
- `preserves_user_authored_files = true` on every packet and on every
  evidence-export projection.
- Every `evidence_export` projection preserves the artifact-family
  label, lineage label, drift-state label, edit-posture label,
  consumer-surface label, generator identity, and source refs so the
  in-product chrome and the exported packet quote the same truth.

## Out of scope

- Live measurement of generator latency or throughput.
- Cross-tenant ticket routing — the report is consumed locally by
  the support-export pipeline and the chrome.
- Adding new consumer surfaces, artifact families, lineage classes,
  drift states, edit postures, downgrade labels, open-gap classes,
  generator kinds, or source kinds without updating the schema, the
  Rust module, the reviewer doc, this report, and the protected
  corpus together.
