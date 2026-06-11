# M5 Maintenance-Truth Register Artifact Companion

This file is the artifact-level companion document for the checked-in M5 maintenance-truth register.

- **Canonical JSON**: `artifacts/release/m5/add_backport_and_hotfix_rules_proof_freshness_automation_and_help_about_truth_updates_for_m5_lanes.json`
- **Schema**: `schemas/governance/add_backport_and_hotfix_rules_proof_freshness_automation_and_help_about_truth_updates_for_m5_lanes.schema.json`
- **Typed consumer**: `crates/aureline-release/src/add_backport_and_hotfix_rules_proof_freshness_automation_and_help_about_truth_updates_for_m5_lanes/mod.rs`
- **Validation capture**: `artifacts/release/captures/add_backport_and_hotfix_rules_proof_freshness_automation_and_help_about_truth_updates_for_m5_lanes_validation_capture.json`
- **Generator**: `gen_maintenance_truth_register.py`

The register is the single source of truth for the M5 maintenance lanes (supported-line backport rules, emergency hotfix rules, proof-freshness/evidence-expiry automation, and the Help/About truth surfaces those lanes publish), the per-lane maintenance scorecard, the disclosed support posture and trust tier of each, owner manifests, downgrade automation, and the promotion verdict. All downstream surfaces ingest it directly. Regenerate it with `python3 gen_maintenance_truth_register.py` from the repository root after changing the maintenance-truth lanes.
