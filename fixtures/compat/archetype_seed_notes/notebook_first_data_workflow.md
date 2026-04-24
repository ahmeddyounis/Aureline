# Notebook-first data workflow

## Row binding

- Archetype row id: `archetype_row:notebook_first_data_workflow`
- Archetype id: `notebook_first_data_workflow`
- Initial support class: `experimental`
- Target support class: `supported`
- Inclusion target: `first_beta`
- Compatibility row: `compat_row:certification.launch_archetype_matrix`
- Skew register: `skew_register:certification.launch_archetype_matrix`

## Representative stack

Jupyter-notebook-first data workflow on a local Python kernel with a
paired test or script module, version-controlled notebooks, and no
remote kernel as the baseline. The row consciously pairs with
`archetype_row:python_service_or_data_app` through the
`notebook_handoff` workflow so notebook-specific concerns stay on this
row instead of leaking into the Python service archetype.

## Required-mode rationale

- `notebook_kernel_local_only` — the row's first-stable promise is a
  local kernel only. Remote kernels and managed compute land on
  separate future rows so the supported claim does not silently
  inherit a runtime mode it has not been measured against.

## Evidence already on file

- Reference workspace: `reservation:fixtures/workspaces/reference/notebook_first_data_workflow_archetype_seed.json`.
- Corpus scenarios: `reservation:archetype.notebook_first_data_workflow_first_open`,
  `reservation:workflow.first_useful_edit_notebook_first_data_workflow`.
- Structured-artifact review surface presets:
  `cell_id_stability_required=true`,
  `metadata_filter_preset=preserve_jupyter_and_aureline_namespaces_only`,
  and `output_handling_preset=ignore_outputs_by_default_with_opt_in_inclusion`
  apply to any notebook fixture this row materialises.

## Open evidence questions

- Materialise the reservation slot for the seed notebook workspace
  before any graduation step. The fixture must respect the cell-aware
  compare presets above and must not vendor a real third-party
  notebook with embedded outputs.
- Decide whether the supported row covers paired text export
  (`.ipynb` plus a `.py` companion) as required evidence or as an
  optional supplementary scenario.
- Capture which cell-execution semantics the supported row promises
  (kernel-managed only, no remote-execution promise) so claim wording
  cannot widen past the measured baseline.
