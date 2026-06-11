# Feature-Train Compatibility Register Artifact Companion

This file is the artifact-level companion document for the checked-in feature-train compatibility register.

- **Canonical JSON**: `artifacts/release/m5/implement_feature_train_compatibility_reports_provider_family_support_windows_and_change_freeze_guidance.json`
- **Schema**: `schemas/governance/implement_feature_train_compatibility_reports_provider_family_support_windows_and_change_freeze_guidance.schema.json`
- **Typed consumer**: `crates/aureline-release/src/implement_feature_train_compatibility_reports_provider_family_support_windows_and_change_freeze_guidance/mod.rs`
- **Validation capture**: `artifacts/release/captures/implement_feature_train_compatibility_reports_provider_family_support_windows_and_change_freeze_guidance_validation_capture.json`
- **Generator**: `gen_feature_train_compatibility.py`

The register is the single source of truth for feature-train compatibility lanes (core platform, AI assistant, collaboration, extensions), the per-lane compatibility-report scorecard, the disclosed provider-family support window and trust tier of each, owner manifests, change-freeze guidance, and the promotion verdict. All downstream surfaces ingest it directly. Regenerate it with `python3 gen_feature_train_compatibility.py` from the repository root after changing the feature-train lanes.
