# Row lineage-hint case corpus

This directory seeds the reviewer-facing corpus for row-level lineage
hints on explorer, quick-open, search, symbol-jump, docs-search,
cross-repo, graph-overlay, AI-citation, and support / export
reference surfaces.

Companion material:

- [`../drift_regeneration_manifest.yaml`](../drift_regeneration_manifest.yaml)
  — the full safe-edit posture corpus every row hint joins against.
- [`../../../docs/generated/lineage_hint_packet.md`](../../../docs/generated/lineage_hint_packet.md)
  — contract this corpus seeds.
- [`../../../artifacts/generated/explorer_search_rows/`](../../../artifacts/generated/explorer_search_rows/)
  — concrete `row_lineage_hint_record` examples cited by case id.
- [`../../../artifacts/generated/viewer_fallback_examples/`](../../../artifacts/generated/viewer_fallback_examples/)
  — full posture records each row hint projects from.

## Files

- `lineage_hint_manifest.yaml`
  Stable case ids plus the shared row-hint vocabulary every surface
  reads. Surfaces cite the ids here; they never invent surface-local
  row-origin tokens.

## Rules

1. Consumers join on `case_id`. A row hint that drifts from its case
   id's posture binding is non-conforming.
2. A new case must exercise at least one new combination of artifact
   class, origin class, provenance state, default edit posture,
   rebuild intent, override policy, viewer fallback state, or row
   surface class that existing cases do not already cover.
3. Row hint cases do not duplicate the posture record inline. They
   carry the opaque `artifact_posture_ref` and the opaque
   `generated_artifact_lineage_ref` when available; consumers resolve
   the full record for override provenance, structured-viewer
   fallback, and per-surface projection detail.
4. Non-canonical rows keep `do_not_imply_canonical_source = true`.
   A case that silently promotes a generated, mirrored, imported, or
   preview artifact to canonical source is rejected.
