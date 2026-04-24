# Notebook round-trip, trust, kernel, and diff/merge worked cases

Conformance corpus for
[`docs/adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md`](../../../docs/adr/0022-notebook-document-model-kernel-transport-trust-and-diff-merge.md).
Each `.ipynb` file is a minimal valid nbformat-4 document carrying
Aureline-owned metadata under the closed `aureline.*` namespace defined
in [`schemas/notebook/notebook_metadata_aureline.schema.json`](../../../schemas/notebook/notebook_metadata_aureline.schema.json)
and binds to one scenario the successor ADR MUST pass before promoting
the seed from `Proposed` to `Accepted`.

The cases are intentionally tiny. They are not demos; they are the
vocabulary the kernels-and-trust matrix at
[`artifacts/notebook/kernels_and_trust_matrix.yaml`](../../../artifacts/notebook/kernels_and_trust_matrix.yaml)
reads.

## Rules

- Every case MUST cite at least one invariant from the ADR and one
  acceptance claim from the kernels-and-trust matrix.
- Raw kernel-protocol frames, raw output bytes beyond tiny
  placeholder text, raw widget state bytes, and raw URLs MUST NOT
  appear. Outputs are either absent, minimal placeholder text, or a
  widget MIME bundle with opaque refs.
- Stable `id` fields on every cell (nbformat 4.5+) are required on
  every case except those explicitly exercising the
  `cell_id_stability_not_available_raw_json_fallback_only` path.
- Aureline metadata keys outside the closed namespace in the schema
  are non-conforming.
- Milestone slugs MUST NOT appear in any id, path, or field.

## Cases

| File | Scenario |
|---|---|
| `canonical_ipynb_preserved_with_stable_cell_ids.ipynb` | Baseline canonical `.ipynb` with stable cell ids, `kernel_trust_inherited_from_document_trust`, no outputs. |
| `unknown_vendor_metadata_round_trip.ipynb` | Unknown vendor namespaces (`colab`, `vscode`, `nteract`) MUST round-trip through open / save / diff / merge. |
| `captured_outputs_from_prior_session.ipynb` | Outputs carry `output_trust_captured_from_prior_session`; reopen MUST NOT auto-rerun. |
| `widget_gated_output_default_denied.ipynb` | Widget output defaults to `widget_trust_denied_by_default`; static render-only fallback applies. |
| `orphaned_output_no_kernel_binding.ipynb` | Kernelspec unresolved; outputs are `orphaned_no_kernel_binding`; document stays editable. |
| `paired_text_export_script_derived.ipynb` | Paired `.py` export is derived; `aureline.paired_text_export_ref` cites it; canonical-direction is `notebook->export_script:v1`. |
| `no_kernel_available_still_editable.ipynb` | No kernel resolvable; every cell carries `execution_unavailable_without_kernel` chip; document remains editable, reviewable, searchable. |
| `raw_json_fallback_missing_cell_ids.ipynb` | Legacy document with no cell ids; diff surface falls back to raw JSON with visible chip `fallback_cell_id_stability_not_available`. |

## Boundary schema

Every `aureline.*` metadata namespace in these fixtures is valid under
[`schemas/notebook/notebook_metadata_aureline.schema.json`](../../../schemas/notebook/notebook_metadata_aureline.schema.json)
for the fields it cites. The schema is the authority; these fixtures
follow it.
