# M5 Public-Interface Diff-Report Register Artifact Companion

This file is the artifact-level companion document for the checked-in M5 public-interface diff-report register.

- **Canonical JSON**: `artifacts/release/m5/implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts.json`
- **Schema**: `schemas/compat/m5-public-interface-diff-reports.schema.json`
- **Typed consumer**: `crates/aureline-release/src/implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts/mod.rs`
- **Validation capture**: `artifacts/release/captures/implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts_validation_capture.json`
- **Companion doc**: `docs/m5/implement_public_interface_diff_reports_compatibility_windows_support_class_caveats_and_successor_deprecation_packets_for_changed_stable_facing_m5_contracts.md`
- **Generator**: `tools/regenerate_m5_public_interface_diff_reports.py`

The register is the single source of truth for the public-interface diff reports of changed stable-facing M5 contracts — wire/state schemas, CLI/headless outputs, exported packets, SDK/runtime contracts, and compatibility bridges. For each changed contract it records the public-interface diff (added, removed, and changed surface plus the reader/writer compatibility review), the compatibility window, the support-class caveat, and — for a deprecated contract — the successor/deprecation packet naming the owner, replacement path, alias map, removal checkpoint and horizon, migration, and rollback implications, plus the stable claim backed and the promotion verdict. All downstream surfaces ingest it directly. Regenerate it with `python3 tools/regenerate_m5_public_interface_diff_reports.py` from the repository root after changing the reports.
