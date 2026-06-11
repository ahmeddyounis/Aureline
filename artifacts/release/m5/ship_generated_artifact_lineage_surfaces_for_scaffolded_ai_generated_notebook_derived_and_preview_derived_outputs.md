# Generated-Artifact Lineage Register Artifact Companion

This file is the artifact-level companion document for the checked-in generated-artifact lineage register.

- **Canonical JSON**: `artifacts/release/m5/ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs.json`
- **Schema**: `schemas/governance/ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs.schema.json`
- **Typed consumer**: `crates/aureline-release/src/ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs/mod.rs`
- **Validation capture**: `artifacts/release/captures/ship_generated_artifact_lineage_surfaces_for_scaffolded_ai_generated_notebook_derived_and_preview_derived_outputs_validation_capture.json`
- **Generator**: `gen_generated_artifact_lineage_surfaces.py`

The register is the single source of truth for generated-artifact lineage surfaces (scaffolded, AI-generated, notebook-derived, preview-derived), the disclosed provenance and trust tier of each, owner manifests, rollback/downgrade automation, and the promotion verdict. All downstream surfaces ingest it directly. Regenerate it with `python3 gen_generated_artifact_lineage_surfaces.py` from the repository root after changing the lineage surfaces.
