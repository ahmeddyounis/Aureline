# M5 Experiment Component Surface Certification (M05-1019)

This is the **closing surface-certification capstone** for the B120 experiment-component lane. Where
the frozen matrix (`schemas/ui/m5-experiment-component-matrix.schema.json`) defines the eight
reusable **experiment-run-row**, **dataset-provenance-card**, **artifact-lineage-panel**,
**run-comparison-table**, **environment-fingerprint-card**, **compare-guard-banner**,
**sensitivity-sharing-banner**, and **result-summary-card** components, the M05-1013..1016 primitive
lanes narrow each one, the M05-1017 consumer lane proves they are reusable across the claimed
notebook-run-history / task-test-eval / review-evidence / compare-view / companion-summary /
CLI-headless-export / support-export consumers, and the M05-1018 accessibility / auto-narrowing
capstone certifies keyboard / screen-reader / CLI / export parity per family, this capstone
**certifies that the shared experiment-component truth holds on every claimed M5 notebook-adjacent
and data-workflow surface** — and auto-narrows any surface that cannot sustain it.

- **Module:**
  `crates/aureline-notebook/src/certify_experiment_run_row_dataset_provenance_card_artifact_lineage_panel_run_comparison_table_environment_fingerprint_card_compare_guard_banner_sensitivity_sharing_banner_and_result_summary_card_truth_on_claimed_m5_surfaces/`
- **Boundary schema:** `schemas/ui/m5-experiment-component-certification.schema.json`
- **Release proof:** `artifacts/release/m5-experiment-component-certification/`
  (`support_export.json`, `matrix.csv`, `report.md`)
- **Fixtures:** `fixtures/ui/m5-experiment-component-certification/`
- **Canonical bundle every row cites:** `artifacts/release/m5-experiment-component-proof/support_export.json`
  (the frozen experiment-component matrix release proof — the canonical M5 evidence index entry for
  this lane)

## What is certified

The packet is keyed on the **surface** a user reviews, compares, shares, or escalates a result on —
not on component family or primitive lane. Eight claimed surfaces are certified exactly once:

| Surface | Meaning |
| --- | --- |
| `notebook_experiment_run` | The notebook experiment-run surface (run history / run rows). |
| `experiment_dashboard` | The experiment dashboard. |
| `run_comparison` | The run-comparison (compare-view) surface. |
| `data_catalog` | The data-catalog surface (dataset provenance / data lanes). |
| `artifact_lineage` | The artifact-lineage surface. |
| `review_evidence` | The review-evidence (review workspace) surface. |
| `support_export` | The support / export bundle. |
| `cli_headless` | The CLI / headless surface. |

Each surface is scored on **six truth axes**: `visual`, `keyboard`, `screen_reader`, `export`
(always-on), `degraded_state`, and `provenance_and_comparability`. Every one of the eight frozen
component families is certified on at least one surface.

## The invariant

**A degraded axis must produce a visible claim narrowing.** A surface that keeps an
`exact_comparable_result` / `reviewable_result` claim while one of its truth axes is not current —
the artifact lineage is stale, the comparison evidence is incomplete, the environment fingerprint is
only partially captured, or a dataset sensitivity class blocks the raw preview — is over-claiming and
**blocks (red)**. A surface that discloses the reduction by narrowing its result claim (with a bound
reason and a frozen downgrade trigger) is honestly **yellow**. A surface with full parity delivers
its claim (**green**).

The result-claim ladder, strongest first: `exact_comparable_result` (7) > `reviewable_result` (6) >
`partial_fingerprint_projection` (5) > `incomparable_runs_projection` (4) >
`guard_blocked_projection` (3) > `stale_lineage_projection` (2) > `unprovenanced_data_projection`
(1) > `blocked_preview_projection` (0). Certification may only **narrow** a claim, never strengthen
it.

### Experiment-lineage preservation

Experiment truth never loses lineage: a narrowed surface always preserves its **run-origin /
dataset-provenance / lineage / export-scope** lineage continuity rather than dropping it between an
experiment run row, a lineage panel, and an exported result summary. Dropping lineage blocks the
surface (`LineageDropped`).

### No unproven parity, no raw payload by default

No certified surface may **imply an apples-to-apples comparison without parity evidence**: a metric
delta never reads as a fair baseline unless parity is proven (`ApplesToApplesImpliedWithoutParity`).
No certified surface may **expose raw production-like data by default**: previews stay metadata-only
and raw payloads are opt-in (`RawPayloadExposedByDefault`).

### Always-on export parity

The `export` axis must always stay certified, so support and automation can reconstruct the same
run / dataset / lineage / comparison / fingerprint / sensitivity / summary truth from the same
component identity the user saw. Export must offer text / JSON / Markdown reconstruction and prohibit
a raw-payload-only export.

## The four auto-narrow conditions

The seed packet certifies four green surfaces (full parity, claim delivered) and four yellow
surfaces — one for each spec auto-narrow condition (incomplete comparison evidence, sensitivity-
blocked raw preview, stale artifact lineage, or partially captured environment fingerprint):

| Surface | Claimed → Certified | Binding axis | Trigger |
| --- | --- | --- | --- |
| `run_comparison` | `exact_comparable_result` → `incomparable_runs_projection` | `provenance_and_comparability` | `comparability_overstated` |
| `data_catalog` | `exact_comparable_result` → `blocked_preview_projection` | `provenance_and_comparability` | `sensitivity_class_unstated` |
| `artifact_lineage` | `exact_comparable_result` → `stale_lineage_projection` | `degraded_state` | `cached_state_hidden` |
| `cli_headless` | `exact_comparable_result` → `partial_fingerprint_projection` | `degraded_state` | `environment_fingerprint_unstated` |

No surface hides drift (red), no surface implies apples-to-apples without parity, no surface exposes
a raw payload by default, and no surface drops lineage.

## Metadata-only boundary

The packet is metadata-only: typed class tokens, opaque refs, booleans, and redacted labels. Raw
dataset payloads, captured output bytes, model weights, raw production-like data, and
credential-bearing material never cross this boundary (`RawExperimentPayloadInExport`).

## Regenerating the artifacts

The checked-in export is byte-aligned with the in-code seed builder
(`seeded_m5_experiment_component_certification_packet`). A drift test fails if they diverge. To
regenerate after an intentional change:

```
GEN_EXPERIMENT_CERT_ARTIFACTS=1 cargo test -p aureline-notebook --lib \
  -- certify_experiment_run_row generate_artifacts
```

Then re-run the suite:

```
cargo test -p aureline-notebook --lib -- certify_experiment_run_row
```
