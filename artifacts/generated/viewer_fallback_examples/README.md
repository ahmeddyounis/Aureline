# Generated-artifact posture examples

Reviewer-facing `artifact_edit_posture_record` examples consumed by the
generated-artifact safe-edit policy and the drift/regeneration corpus.

These files do not replace the full lineage record. They show the compact
posture every search/open/review/AI/export/support surface should
preserve.

Coverage in this seed:

- `codegen_clean_regenerate.json`
  generated source sibling in sync; canonical source remains the default
  mutation path.
- `structured_lockfile_stale_generated.json`
  stale lockfile with declared safe structured edit ranges plus rebuild
  intent.
- `generated_unknown_source_diverged.json`
  generated file already diverged from its generator and can no longer
  resolve its canonical source.
- `preview_snapshot_mock_provenance.json`
  preview/render snapshot backed by mock data; compare-only and source
  redirect posture stays explicit.
- `mirrored_docs_pack_drifted.json`
  mirrored docs pack no longer matches upstream; mirror refresh replaces
  ad hoc local edits.
- `notebook_output_structured_viewer_fallback.json`
  structured viewer remains useful, but edit round-trip is not proven so
  raw fallback and canonical-source paths remain visible.

Rules:

1. Search/open/review/AI/export/support projections quote the same
   `artifact_origin_class`, `provenance_state`, and
   `default_edit_posture`.
2. Any non-canonical row keeps `do_not_imply_canonical_source = true`.
3. A row in `diverged_from_generator` state carries active override
   provenance rather than hiding the override in prose.
