# M5 Per-Train Scorecard Register Artifact Companion

This file is the artifact-level companion document for the checked-in M5 per-train scorecard register.

- **Canonical JSON**: `artifacts/release/m5/implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_automation_for_all_m5_trains.json`
- **Schema**: `schemas/governance/implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_automation_for_all_m5_trains.schema.json`
- **Typed consumer**: `crates/aureline-release/src/implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_automation_for_all_m5_trains/mod.rs`
- **Validation capture**: `artifacts/release/captures/implement_per_feature_scorecards_owner_manifests_and_rollback_or_downgrade_automation_for_all_m5_trains_validation_capture.json`
- **Generator**: `gen_m05_123.py`

The register is the single source of truth for M5 per-train scorecards, owner manifests, rollback/downgrade automation, and the promotion verdict. All downstream surfaces ingest it directly. Regenerate it with `python3 gen_m05_123.py` from the repository root after changing the train scorecards.
