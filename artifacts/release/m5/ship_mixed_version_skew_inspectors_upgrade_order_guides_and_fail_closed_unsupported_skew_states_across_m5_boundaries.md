# M5 Boundary Skew-Inspector Register Artifact Companion

This file is the artifact-level companion document for the checked-in M5 mixed-version boundary skew-inspector register.

- **Canonical JSON**: `artifacts/release/m5/ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries.json`
- **Schema**: `schemas/compat/m5-boundary-skew-inspectors.schema.json`
- **Typed consumer**: `crates/aureline-release/src/ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries/mod.rs`
- **Validation capture**: `artifacts/release/captures/ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries_validation_capture.json`
- **Companion doc**: `docs/m5/ship_mixed_version_skew_inspectors_upgrade_order_guides_and_fail_closed_unsupported_skew_states_across_m5_boundaries.md`
- **Generator**: `tools/regenerate_m5_boundary_skew_inspectors.py`

The register is the single source of truth for the runtime skew inspectors bound to the M5 boundary-crossing flows — helper/agent attach, extension/runtime load, workspace-state import/restore, and provider snapshot/open. For each boundary it records the version skew inspected, the fail-closed verdict reported before the gated mutating-or-privileged action, the helper/agent/host/schema/provider downgrade subject, the structured upgrade-order guide, the stable claim backed, and the promotion verdict. All downstream surfaces ingest it directly. Regenerate it with `python3 tools/regenerate_m5_boundary_skew_inspectors.py` from the repository root after changing the inspectors.
