# Materialize the canonical .ipynb document model, stable cell IDs, attachments, and no-kernel editability

## Purpose

This document describes the M05-012 canonical `.ipynb` document model that keeps the notebook surface honest about canonical source truth, cell identity, attachment survival, and editing without a kernel.

## Principles

- `.ipynb` stays canonical unless the user explicitly converts formats.
- Stable cell IDs survive reorder, diff, review, and comment anchoring where the format allows.
- Attachments preserve unknown namespaces and are not silently externalized.
- Notebooks must open, remain searchable, and be editable even when no compatible kernel is available.
- Unknown metadata, extension namespaces, cell IDs, and attachments survive round-trip open/save whenever the upstream format allows.
- Document trust, workspace trust, kernel availability, and output trust are distinct state labels and may differ without collapsing into one generic status.

## Model overview

### NotebookDocument

The top-level document record binds:

- `document_id` — stable opaque notebook identity.
- `document_path_token_ref` — VFS canonical path-identity token.
- `document_uri` — display/export URI.
- `nbformat_major` / `nbformat_minor` — format version read from the file.
- `canonical_preservation_class` — whether `.ipynb` is canonical, export-only, or has no paired text representation.
- `cell_id_stability_class` — whether stable IDs are required, minted on first save, or unavailable.
- `metadata_survival_class` — which metadata namespaces must survive round-trip.
- `no_kernel_editability_class` — the editability posture when no kernel is available.
- `cells` — ordered list of [`NotebookCell`] records.
- `local_state_overlay` — ephemeral UI state that must not rewrite canonical structure.

### NotebookCell

Each cell carries:

- `cell_id` — stable opaque cell identity.
- `cell_type` — `code`, `markdown`, or `raw`.
- `cell_source_ref` — opaque ref to the source body (raw bytes never appear inline).
- `cell_metadata_survival_class` — metadata survival posture for this cell.
- `attachment_refs` — opaque refs to [`NotebookAttachment`] records.
- `unknown_vendor_namespaces_present` — list of vendor namespaces detected (raw bodies never appear).
- `output_lineage_refs` — opaque refs to output-lineage records.

### NotebookAttachment

Each attachment carries:

- `attachment_id` — stable opaque attachment identity.
- `owner_cell_ref` — opaque ref to the owning cell.
- `mime_class` — closed-vocabulary MIME class token.
- `digest` — content digest.
- `size_bytes` — size in bytes.
- `preview_class` — how the attachment is previewed without exposing raw bytes.

### NotebookLocalStateOverlay

UI convenience state that must not rewrite canonical notebook structure without an explicit save/apply path:

- `selected_cell_id_ref` — currently selected cell.
- `output_collapsed_cell_id_refs` — cells whose output is collapsed.
- `source_folded_cell_id_refs` — cells whose source is folded.
- `scroll_anchor_cell_id_ref` — scroll anchor.
- `pinned_viewer_cell_id_refs` — pinned viewers.

## Closed vocabularies

| Vocabulary | Variants | Location |
|---|---|---|
| `NotebookCellType` | `code`, `markdown`, `raw` | `aureline-notebook` crate |
| `NotebookCanonicalPreservationClass` | `canonical_ipynb_preserved`, `export_only_no_round_trip`, `no_paired_text_representation` | `aureline-notebook` crate |
| `NotebookCellIdStabilityClass` | `stable_cell_id_required`, `stable_cell_id_minted_on_first_save`, `cell_id_stability_not_available_raw_json_fallback_only` | `aureline-notebook` crate |
| `NotebookMetadataSurvivalClass` | `survival_required_jupyter_and_aureline_namespaces`, `survival_required_vendor_namespaces`, `survival_recommended_unknown_vendor_namespaces` | `aureline-notebook` crate |
| `NotebookNoKernelEditabilityClass` | `editable_searchable_reviewable`, `editable_readonly_kernel_required_for_execution`, `editable_with_degraded_preview` | `aureline-notebook` crate |
| `NotebookAttachmentPreviewClass` | `inline_preview`, `thumbnail_preview`, `icon_preview`, `no_preview` | `aureline-notebook` crate |

## Schema

The boundary schema lives at:

```
/schemas/notebook/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability.schema.json
```

## Checked-in artifact

The typed document-model packet is checked in at:

```
artifacts/notebook/m5/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability.json
```

It is embedded in the `aureline-notebook` crate via `include_str!` so consumers and CI agree on every row without a cargo build in CI.

## Fixtures

Worked fixture cases live under:

```
fixtures/notebook/m5/materialize_the_canonical_ipynb_document_model_stable_cell_ids_attachments_and_no_kernel_editability/
```

Cases cover:
- `clean_notebook` — canonical `.ipynb` with stable cell IDs and inline preview attachment.
- `no_kernel_editable` — notebook editable without a kernel, with derived paired-text export.
- `attachments_with_unknown_metadata` — attachments and unknown vendor metadata that must survive round-trip.

## Integration

The `aureline-notebook` crate exposes:

- `NotebookDocument`, `NotebookCell`, `NotebookAttachment`, `NotebookLocalStateOverlay`
- `NotebookDocumentModelPacket` and `current_notebook_document_model_packet()`
- Validation methods on every record that return typed `DocumentModelFinding` findings
- Closed vocabularies with `as_str()` tokens and `ALL` arrays

## Risks and downgrade behavior

- If the embedded JSON artifact is missing or malformed, `current_notebook_document_model_packet()` returns an error and CI must treat the lane as underqualified.
- If a fixture case fails validation, the corpus is incomplete and promotion should narrow the claim.
- Unknown vendor namespaces must survive round-trip; stripping them silently is a protocol violation.
- No-kernel editability is required; any code path that blocks open or edit without a kernel is non-conforming.
