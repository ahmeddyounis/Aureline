# M5 Depth-Claim Manifest Artifact Companion

This file is the artifact-level companion document for the checked-in M5 depth-claim manifest.

- **Canonical JSON**: `artifacts/release/m5/freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix.json`
- **Schema**: `schemas/governance/freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix.schema.json`
- **Typed consumer**: `crates/aureline-release/src/freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix/mod.rs`
- **Validation capture**: `artifacts/release/captures/freeze_the_m5_depth_claim_manifest_feature_family_packets_and_qualification_matrix_validation_capture.json`
- **Generator**: `gen_m05_122.py`

The manifest is the single source of truth for M5 depth-claim scope, per-family qualification-matrix posture, and the promotion verdict. All downstream surfaces ingest it directly. Regenerate it with `python3 gen_m05_122.py` from the repository root after changing the family packets.
