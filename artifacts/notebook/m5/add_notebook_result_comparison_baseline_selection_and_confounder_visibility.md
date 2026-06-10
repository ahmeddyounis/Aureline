# Artifact: Add notebook result comparison, baseline selection, and confounder visibility

## Packet

- **Path**: `artifacts/notebook/m5/add_notebook_result_comparison_baseline_selection_and_confounder_visibility.json`
- **Schema version**: 1
- **Record kind**: `notebook_result_comparison_packet`
- **As of**: 2026-06-09T00:00:00Z

## Closed vocabularies

### Comparison modes

- `cell_aware` — per-cell source and output comparison
- `output_aware` — output-only comparison
- `summary_only` — high-level equivalence check
- `raw_json_fallback` — structural JSON diff fallback

### Comparison outcomes

- `equivalent` — baseline and current match
- `different` — baseline and current differ
- `baseline_missing` — baseline run is missing or unavailable
- `current_missing` — current run is missing or not yet executed
- `incomparable` — comparison mode or scope prevents valid comparison

### Comparison scopes

- `full_notebook` — compare all cells
- `selected_cells` — compare a user-selected subset
- `active_cell` — compare only the currently active cell

### Baseline sources

- `last_successful_run` — most recent successful execution
- `pinned_experiment` — explicitly pinned experiment run
- `tagged_commit` — baseline tied to a version-control tag
- `manual_upload` — user-uploaded baseline artifact
- `workspace_snapshot` — baseline from a workspace snapshot

### Baseline selection states

- `selected` — baseline is active and valid
- `stale` — baseline is outdated but still present
- `unavailable` — baseline cannot be resolved
- `ambiguous` — multiple baselines match criteria
- `explicit_none` — user chose no baseline

### Confounder classes

- `environment_drift` — environment fingerprint diff
- `dependency_change` — package or kernel dependency diff
- `dataset_change` — dataset card or source diff
- `hardware_difference` — CPU/GPU/memory topology diff
- `kernel_restart` — runtime restart between runs
- `runtime_parameter_change` — execution parameter diff
- `unspecified` — confounder detected but not classified

### Confounder visibility classes

- `visible` — confounder is shown to the reviewer
- `suppressed` — confounder is hidden by policy or redaction
- `unknown` — visibility cannot be determined
- `not_applicable` — no confounder applies to this comparison

## Invariants

1. A result comparison MUST carry non-empty `baseline_run_ref` and `current_run_ref`.
2. When `outcome_class` is `different`, `confounder_refs` MUST contain at least one ref.
3. When `selection_state` is `selected` or `stale`, `baseline_run_ref` MUST be non-empty.
4. When `selection_state` is `unavailable` or `explicit_none`, `baseline_run_ref` MUST be `null`.
5. A confounder visibility record MUST carry a non-empty `document_id_ref`.

## Downstream consumers

- `crates/aureline-notebook` — canonical record definitions and validators
- `crates/aureline-review` — review workspace result comparison integration
- `crates/aureline-collab` — collaboration baseline share-scope integration
- `docs/notebook/m5/add_notebook_result_comparison_baseline_selection_and_confounder_visibility.md` — human-readable spec
