# M5 Release-Center Visibility Register Artifact Companion

This file is the artifact-level companion document for the checked-in M5 release-center visibility register.

- **Canonical JSON**: `artifacts/release/m5/ship_release_center_visibility_for_m5_trains_channel_profile_rollout_controls_and_narrow_or_broaden_decisions.json`
- **Schema**: `schemas/governance/ship_release_center_visibility_for_m5_trains_channel_profile_rollout_controls_and_narrow_or_broaden_decisions.schema.json`
- **Typed consumer**: `crates/aureline-release/src/ship_release_center_visibility_for_m5_trains_channel_profile_rollout_controls_and_narrow_or_broaden_decisions/mod.rs`
- **Validation capture**: `artifacts/release/captures/ship_release_center_visibility_for_m5_trains_channel_profile_rollout_controls_and_narrow_or_broaden_decisions_validation_capture.json`
- **Generator**: `gen_release_center_visibility_register.py`

The register is the single source of truth for the M5 release-center visibility surfaces (the release-center view of the feature trains, the channel rollout controls, the profile rollout controls, and the narrow-or-broaden decision surface those controls drive), the per-surface readiness scorecard, the disclosed rollout posture and trust tier of each, owner manifests, downgrade automation, and the promotion verdict. All downstream surfaces ingest it directly. Regenerate it with `python3 gen_release_center_visibility_register.py` from the repository root after changing the release-center visibility surfaces.
