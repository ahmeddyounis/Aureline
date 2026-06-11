# M5 Publication-Pack Register Artifact Companion

This file is the artifact-level companion document for the checked-in M5 publication-pack register.

- **Canonical JSON**: `artifacts/release/m5/publish_docs_migration_and_known_limits_packs_for_m5_feature_families.json`
- **Schema**: `schemas/governance/publish_docs_migration_and_known_limits_packs_for_m5_feature_families.schema.json`
- **Typed consumer**: `crates/aureline-release/src/publish_docs_migration_and_known_limits_packs_for_m5_feature_families/mod.rs`
- **Validation capture**: `artifacts/release/captures/publish_docs_migration_and_known_limits_packs_for_m5_feature_families_validation_capture.json`
- **Generator**: `gen_publication_pack_register.py`

The register is the single source of truth for the M5 publication packs (the docs packs, the migration packs, the known-limits packs, and the publication index that ties them together across the feature families — notebooks, DB/API, profiler, AI, frameworks, companion, sync, and offboarding), the per-pack readiness scorecard, the disclosed support posture and trust tier of each, owner manifests, downgrade automation, and the promotion verdict. All downstream surfaces ingest it directly. Regenerate it with `python3 gen_publication_pack_register.py` from the repository root after changing the publication packs.
