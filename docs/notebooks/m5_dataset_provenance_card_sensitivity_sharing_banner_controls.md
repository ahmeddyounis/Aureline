# M5 dataset provenance cards and sensitivity / sharing banners

The dataset provenance card and the sensitivity / sharing banner are two of the eight governed
experiment components frozen by the
[M5 experiment-component matrix](m5_experiment_component_matrix.md). This lane implements those
two families as two co-equal control vectors in one export-safe packet,
[`DatasetProvenanceCardSensitivitySharingBannerControlsPacket`](../../crates/aureline-notebook/src/implement_dataset_provenance_cards_and_sensitivity_sharing_banners_with_snapshot_sample_redaction_and_local_remote_location_truth_across_claimed_m5_data_lanes/mod.rs),
so a claimed M5 notebook, experiment-dashboard, comparison, data-catalog, share-review, or CLI
surface can project a dataset card and a sharing banner that keep data-bearing results
**metadata-first, privacy-safe, and location-aware before preview, compare, or share** — never
inferred, and never exposing raw production-like data by default.

## What the resolvers decide

The module has two derived resolvers so the honesty of each component is computed, never
asserted.

### `resolve_dataset_provenance`

Given a dataset card's source class and provenance state, the resolver derives a **location
class** and a **provenance class**:

- `local_file` / `synthetic_data` / `redacted_sample` → `local_data` (local)
- `tracked_dataset` / `remote_snapshot` → `remote_data` (must carry an explicit remote note),
  not local
- `unknown_source` → `location_unknown` (must carry an explicit unknown-location note), not
  local
- `provenance_complete` → `provenanced`, `version_pinned` → `pinned` (both fully provenanced)
- `provenance_partial` → `partially_provenanced` (must carry an explicit partial note), not
  fully provenanced
- `provenance_missing` / `version_drifted` / `access_restricted` → `unprovenanced` (must carry
  an explicit unprovenanced note), not fully provenanced

A user can therefore always tell whether the data is **local, remote, or of unknown location**
and whether it is fully provenanced before trusting a downstream compare or share; a remote,
unknown-location, or unprovenanced dataset can never read as a fully-provenanced local dataset.

### `resolve_share_scope`

Given a banner's sensitivity class and share scope state, the resolver derives a **share
disposition**:

- `summary_only` / `summary_plus_metadata` → `metadata_safe` (metadata-only)
- `evidence_included` → `evidence_scoped`
- `raw_payload_included` → `raw_exposed` (must carry an explicit raw-payload warning)
- `redacted_share` → `redacted` (must carry an explicit redaction note)
- `share_blocked` → `blocked` (must carry an explicit blocked note)

A `confidential`, `regulated`, or `production_like` sensitivity is high-sensitivity and must
carry an explicit sensitivity warning. Raw data is therefore **never implied by default**; a
raw-payload share is only ever the result of an explicit, warned choice, and the metadata-only,
local-safe alternative stays visible before any share.

## Dataset identity, sampling, redaction, and share safety

- **Dataset identity and provenance** — every dataset card names its dataset / table, its
  source class, its version / snapshot / partition, its row / file count or estimate, its
  sample / truncation state, its sensitivity / redaction state, and its local-versus-remote
  location, so what data a result was built on stays **always explicit**.
- **Open / inspect / export** — every dataset card offers the mandatory `open_dataset`,
  `inspect_provenance`, and `export_metadata` actions (metadata-first export), plus
  `open_deep_link`, `compare_datasets`, and `copy_dataset_id` as appropriate.
- **Sensitivity and share scope** — every banner names its sensitivity class, its share scope
  state, its share class, its blocked destinations, its metadata-only-versus-raw-payload choice,
  its copy / export policy, and its local-safe alternatives.
- **Review / share-metadata-only** — every banner offers the mandatory `review_share_scope` and
  `share_metadata_only` actions, plus `export_metadata_only`, `open_deep_link`,
  `copy_local_safe_reference`, and `block_share` as appropriate.
- **Stable deep links** — every next step names a stable `run_object`, `notebook_location`,
  `dataset_catalog_anchor`, or `docs_anchor` deep link with a resolvable reference. A component
  that offers a deep-link action must name a resolvable kind, so a next step is never an
  ephemeral overlay or hidden route.

## Hard invariants

Every component keeps five bools `false`, and validation flags any that is `true`:

- `masks_provenance_or_sensitivity_state` — dataset provenance and sensitivity posture stay
  visible.
- `hides_dataset_location_or_provenance` — where the data lives and how it is provenanced stay
  explicit.
- `exposes_raw_payload_by_default` — a raw payload is never included in a share by default.
- `implies_apples_to_apples_without_parity` — a comparison is never implied comparable without
  parity evidence.
- `invents_alternate_state_label` — no surface invents a second word for a governed source,
  provenance, sensitivity, or share state.

Cards and banners reuse Aureline's existing privacy, redaction, retention, and share-class
vocabulary instead of inventing data-specific exceptions; cached, offline, and local-only state
stays visible.

## Coverage

The checked-in support export exercises every location class, every provenance class, every
dataset source class, and every dataset provenance state across the six seeded dataset cards,
and every share disposition, every sensitivity class, and every share scope state across the
six seeded sharing banners.

## Source of truth and artifacts

- Boundary schema: [`schemas/ui/m5-dataset-provenance-card-sensitivity-sharing-banner-controls.schema.json`](../../schemas/ui/m5-dataset-provenance-card-sensitivity-sharing-banner-controls.schema.json)
- Support export: [`artifacts/release/m5-dataset-provenance-card-sensitivity-sharing-banner-proof/support_export.json`](../../artifacts/release/m5-dataset-provenance-card-sensitivity-sharing-banner-proof/support_export.json)
- Matrix CSV: [`artifacts/release/m5-dataset-provenance-card-sensitivity-sharing-banner-proof/matrix.csv`](../../artifacts/release/m5-dataset-provenance-card-sensitivity-sharing-banner-proof/matrix.csv)
- Design report: [`artifacts/design/m5-dataset-provenance-card-sensitivity-sharing-banner.md`](../../artifacts/design/m5-dataset-provenance-card-sensitivity-sharing-banner.md)
- Scenario fixtures: [`fixtures/ui/m5-dataset-provenance-card-sensitivity-sharing-banner-controls/`](../../fixtures/ui/m5-dataset-provenance-card-sensitivity-sharing-banner-controls/)

Regenerate every artifact and fixture from the single seed with the headless emitter:

```sh
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_dataset_provenance_card_sensitivity -- support-export
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_dataset_provenance_card_sensitivity -- csv
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_dataset_provenance_card_sensitivity -- report
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_dataset_provenance_card_sensitivity -- fixture-dataset-card-remote
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_dataset_provenance_card_sensitivity -- fixture-sharing-banner-raw-payload
cargo run -q -p aureline-notebook --bin aureline_notebook_m5_dataset_provenance_card_sensitivity -- validate
```
