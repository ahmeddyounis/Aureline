# M5 Field-Readiness Register Artifact Companion

This file is the artifact-level companion document for the checked-in M5 field-readiness register.

- **Canonical JSON**: `artifacts/release/m5/implement_support_bundle_schema_expansion_feature_family_export_packets_and_field_readiness_drills_for_m5_surfaces.json`
- **Schema**: `schemas/governance/implement_support_bundle_schema_expansion_feature_family_export_packets_and_field_readiness_drills_for_m5_surfaces.schema.json`
- **Typed consumer**: `crates/aureline-release/src/implement_support_bundle_schema_expansion_feature_family_export_packets_and_field_readiness_drills_for_m5_surfaces/mod.rs`
- **Validation capture**: `artifacts/release/captures/implement_support_bundle_schema_expansion_feature_family_export_packets_and_field_readiness_drills_for_m5_surfaces_validation_capture.json`
- **Generator**: `gen_field_readiness_register.py`

The register is the single source of truth for the M5 field-readiness surfaces (the support-bundle schema expansion, the feature-family export packets, the field-readiness drills, and the operator escalation runbook those surfaces lean on), the per-surface readiness scorecard, the disclosed support posture and trust tier of each, owner manifests, downgrade automation, and the promotion verdict. All downstream surfaces ingest it directly. Regenerate it with `python3 gen_field_readiness_register.py` from the repository root after changing the field-readiness surfaces.
