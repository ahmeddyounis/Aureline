# Generated-artifact lineage and drift-state (beta)

This document is the reviewer-facing contract for the generated-artifact
lineage beta projection. It defines the closed vocabularies, the
acceptance contract a fixture row must satisfy, the support-export
parity rules, and the relationship to the search, review, AI context,
and support export surfaces that read the lineage record in-product.

A generated-artifact lineage packet binds **one consumer surface** to
**one artifact** under a closed artifact-family vocabulary and captures
**generator identity**, **canonical source refs**, **lineage class**,
**drift state**, and the **default edit posture** in a single
metadata-safe exportable object. The `default_edit_posture` is
**re-derived from `lineage_class`** and the `downgrade_label` is
**re-derived from `drift_state`**, so prose cannot lie about generator
identity, and a closed `downgrade_label` downgrades a failing row
without inventing new vocabulary.

## Why this exists

Before this projection, generated artifacts could masquerade as
hand-authored source: a `target/` binary, a `Cargo.lock`, a notebook
output, or an imported vendor drop could appear in search results,
review surfaces, or AI context without telling the user that it was
generated, regenerable, derived from a run, or imported. That meant:

- Search results conflated authored source with build outputs.
- Review surfaces let edits land on regenerable artifacts.
- AI context quoted run-derived outputs as if they were authored facts.
- Support exports captured raw generated payload bytes instead of
  metadata-only generator identity and source refs.

A lineage packet captures the artifact family, lineage class, drift
state, default edit posture, generator identity, and canonical source
refs in one exportable object so:

- The chrome can render the same fields the support packet carries.
- Search, review, AI context, and support export pipelines can tell
  whether an artifact is generated, mirrored, imported, or canonical
  source.
- Artifact lineage is exportable without forcing raw payload capture.

## Source contract

- Schema: [`/schemas/state/generated_artifact.schema.json`](../../../schemas/state/generated_artifact.schema.json)
  (records `generated_artifact_lineage_record`,
  `generated_artifact_lineage_report_record`, version 1).
- Crate module:
  [`/crates/aureline-reactive-state/src/generated_lineage/mod.rs`](../../../crates/aureline-reactive-state/src/generated_lineage/mod.rs).
- Support consumer:
  [`/crates/aureline-support/src/generated_lineage/mod.rs`](../../../crates/aureline-support/src/generated_lineage/mod.rs).
- Fixture corpus:
  [`/fixtures/state/generated_artifacts_beta/`](../../../fixtures/state/generated_artifacts_beta/)
  with [`manifest.yaml`](../../../fixtures/state/generated_artifacts_beta/manifest.yaml).
- Baseline report:
  [`/artifacts/support/m3/generated_artifact_lineage_report.md`](../../../artifacts/support/m3/generated_artifact_lineage_report.md).

## Closed vocabularies

### `consumer_surface`

| Token | Meaning |
| --- | --- |
| `search` | Search results, quick-open, file index hits. |
| `review` | Diff review, impact review, review seeds. |
| `ai_context` | AI context selection, context inspector, prompt evidence handoff. |
| `support_export` | Support packets, evidence exports, escalation bundles. |

### `artifact_family`

`build_output`, `lockfile`, `preview_render`, `notebook_output`,
`run_result_artifact`. The corpus must seed at least one packet per
family. Families are intentionally narrow: anything that the user did
not author by typing it falls into one of these five lanes during
beta.

### `lineage_class`

`canonical_source`, `generated_from_local_source`,
`regenerable_lockfile_artifact`, `mirrored_from_local_source`,
`imported_external_artifact`, `derived_from_run_artifact`,
`previewed_from_local_source`, `unknown_lineage`.

`canonical_source` is the baseline (hand-authored) lane and is kept in
the vocabulary so search, review, AI, and support can differentiate
generated from canonical source. The corpus does not seed
`canonical_source` rows — the projection is for non-canonical lineage.

### `drift_state`

`aligned`, `source_drifted`, `regen_pending`, `stale_generated`,
`generator_missing`, `imported_no_local_source`, `out_of_scope`.

`aligned` and `imported_no_local_source` are the two healthy states:
`aligned` means the artifact matches its generator's last-observed
inputs, and `imported_no_local_source` is the natural state for an
imported artifact that has no local source to drift against. All
other states are anomalous and must downgrade the row.

### `default_edit_posture`

`editable_canonical`, `read_only_generated`, `regenerate_only`,
`review_required_before_edit`, `imported_read_only`,
`transient_run_artifact`. The posture is **re-derived from
`lineage_class`** by the evaluator so the surface chrome and the
support packet quote the same posture without prose drift.

### `downgrade_label`

`none`, `red_blocks_beta_row`, `yellow_drift_pending`,
`yellow_generator_unknown`, `yellow_partial_coverage`,
`degraded_to_metadata_only`, `stale_corpus_blocks_release_candidate`.
Healthy rows (aligned or imported-no-local-source) must pin `none`;
every drift row must pin one of the other labels. The evaluator also
re-derives the label from `drift_state` so the mapping cannot drift
silently.

### `open_gap_class`

`none`, `regen_pending`, `generator_identity_pending`,
`source_ref_pending`, `surface_coverage_pending`, `lineage_pending`,
`evidence_export_pending`. A downgraded row must record at least one
non-none open gap; a healthy row must declare none.

### `generator_kind`

`build_system`, `package_manager`, `preview_renderer`,
`notebook_kernel`, `task_runner`, `external_import`,
`unknown_generator`. The generator identity is pinned on every packet
so support and review can name the producer without re-running it.

### `source_kind`

`local_source_file`, `local_source_manifest`, `local_notebook_cell`,
`external_import_descriptor`, `run_invocation_descriptor`,
`no_local_source`. Each packet carries at least one source ref drawn
from this vocabulary so the canonical source (or the lack of one) is
quoted in the export.

## Acceptance contract

The evaluator
[`GeneratedArtifactLineageEvaluator`](../../../crates/aureline-reactive-state/src/generated_lineage/mod.rs)
refuses a row when any of these contracts are broken:

1. `default_edit_posture` disagrees with the posture derived from
   `lineage_class`.
2. `downgrade_label` disagrees with the label derived from
   `drift_state`.
3. Aligned or imported-no-local-source rows carry a non-none
   `downgrade_label`, a non-none `open_gap_class`, or vice versa.
4. `imported_external_artifact` rows do not pin
   `drift_state = imported_no_local_source`, or non-imported rows
   pin it.
5. `canonical_source` rows pin a `drift_state` other than `aligned`.
6. `evidence_export` drops any of the preservation flags
   (`preserves_artifact_family_label`, `preserves_lineage_label`,
   `preserves_drift_state_label`, `preserves_edit_posture_label`,
   `preserves_consumer_surface_label`,
   `preserves_generator_identity`, `preserves_source_refs`).
7. `evidence_export` admits raw payload bytes, raw private material,
   or ambient authority, or drops
   `preserves_user_authored_files`.
8. The packet declares `safety.destructive_resets_present = true` or
   drops `safety.preserves_user_authored_files`, admits
   `safety.raw_payload_excluded = false`,
   `safety.raw_private_material_excluded = false`, or
   `safety.ambient_authority_excluded = false`.
9. The packet's `references` block drops the pinned doc, schema, or
   report ref.
10. The packet's `source_refs` array is empty, or any
    `source_path` is blank.

The corpus is also refused unless:

- Every required `consumer_surface` is seeded by at least one packet.
- Every required `artifact_family` is seeded by at least one packet.
- Every required `lineage_class` is seeded by at least one packet.
- At least one packet declares a non-aligned `drift_state` so the
  drift contract is exercised by a fixture.

## Support-export parity

The same packet fields the in-product chrome renders travel through
the support packet without re-running generators:

- `consumer_surface`, `artifact_family`, `lineage_class`,
  `drift_state`, and `default_edit_posture` are pinned on the packet
  record itself.
- `generator_identity` and `source_refs` are pinned on the packet so
  support and review can name the producer and the canonical source
  without re-running anything.
- `evidence_export` declares that the support packet preserves the
  artifact-family label, lineage label, drift-state label, edit-posture
  label, consumer-surface label, generator identity, and source refs.
- The packet record is metadata-only: it never carries raw payload
  bytes, raw private material, or ambient authority, and the safety
  baseline forbids destructive resets and preserves user-authored
  files.

The support-side mirror lives in
[`crates/aureline-support/src/generated_lineage/mod.rs`](../../../crates/aureline-support/src/generated_lineage/mod.rs)
and re-emits the report matrix as a typed
`generated_lineage_support_export_envelope` so support and export
pipelines audit lineage truth without re-running generators.

## Out of scope

- Live measurement of generator latency or throughput.
- Cross-tenant ticket routing. Lineage packets are consumed locally
  by the support-export pipeline and the chrome.
- Adding new consumer surfaces, artifact families, lineage classes,
  drift states, edit postures, downgrade labels, open-gap classes,
  generator kinds, or source kinds without updating the schema, the
  Rust module, this reviewer doc, the baseline report, and the
  protected corpus together.
