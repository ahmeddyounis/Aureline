# Execution-plane certification truth fixtures

This directory contains fixture cases consumed by the integration test.

| fixture | what it proves |
|---|---|
| `baseline_stable.json` | Baseline stable posture: all three lanes (local, remote_helper, enterprise_network) publish an execution_plane_certification_quality row at launch_stable plus full surface-binding coverage, target admission, full route-state admission coverage, restore-rerun-honesty, full reconnect-state admission coverage, full degraded-helper-state admission coverage, full artifact-provenance-state admission coverage, and lineage admission binding execution_context_id; every row binds support, known limit, downgrade automation, and evidence classes; narrowed rows carry disclosure refs; and all eight required consumer projections preserve the packet verbatim. |
| `launch_stable_with_unbound_evidence_blocks_stable.json` | A launch_stable quality row with evidence_unbound is refused. |
| `missing_route_admission_for_launch_stable_blocks_stable.json` | The local_lane dropping its blocked_target route admission triggers missing_route_admission. |
| `missing_reconnect_admission_for_launch_stable_blocks_stable.json` | The remote_helper_lane dropping its restore_no_rerun reconnect admission triggers missing_reconnect_admission. |
| `missing_degraded_helper_admission_for_launch_stable_blocks_stable.json` | The enterprise_network_lane dropping its helper_skew degraded-helper admission triggers missing_degraded_helper_admission. |
| `missing_artifact_provenance_admission_for_launch_stable_blocks_stable.json` | The local_lane dropping its provenance_missing artifact-provenance admission triggers missing_artifact_provenance_admission. |
| `lineage_admission_missing_execution_context_id_blocks_stable.json` | Dropping the execution_context_id_binding on the local_lane lineage row triggers lineage_admission_missing_execution_context_id and missing_lineage_admission. |
| `narrowed_row_missing_disclosure_ref_blocks_stable.json` | A launch_stable_below row without a disclosure ref triggers narrowed_row_missing_disclosure_ref and downgrade_automation_missing_disclosure_ref. |
| `projection_collapses_route_state_vocabulary_blocks_stable.json` | A Help/About projection that collapses the route-state vocabulary triggers route_state_vocabulary_collapsed, plus missing_consumer_projection and consumer_projection_drift. |
| `raw_source_material_blocks_stable.json` | Admitting raw command lines or process environment bytes past the boundary triggers raw_source_material_present. |
