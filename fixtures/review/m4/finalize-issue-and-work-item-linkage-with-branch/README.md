# Fixtures: Finalize issue and work-item linkage with branch

These fixtures exercise the work-item linkage finalization packet that binds review workspace, stabilization, work-item detail surfaces, status-transition sheets, offline-handoff continuities, branch/review links, publish-later continuities, commands, support export, and inspection into a single coherent view.

## Files

| Fixture | Scenario |
|---|---|
| `finalized_current_all_surfaces_present.json` | All surfaces present: provider-authoritative and local-draft detail surfaces, transition sheet with previewed side effects, offline handoff continuity, previewable branch and review links, publish-later continuity. |
| `finalized_offline_handoff_only.json` | Offline-handoff-only finalization: no detail surfaces, no branch/review links, but offline handoff continuity survives restart, reconnect, and export/import. |
| `finalized_partial_work_item_scope.json` | Partial work-item scope: only one detail surface present, missing transition sheets, offline handoff, branch/review links, and publish-later continuities. |
| `finalized_stale_provider_overlay.json` | Stale provider overlay: detail surface is stale within grace, branch/review links are not previewable, transition sheet blocks confirm action. |

## Fixture format

Each fixture is a JSON object with:

- `record_kind`: `"work_item_linkage_finalization_case"`
- `schema_version`: `1`
- `case_name`: descriptive name
- `seed_fixture_ref`: path to the alpha seed fixture
- `beta_workspace_input`: input to `ReviewWorkspaceBetaPacket::from_seed_packet`
- `landing_input`: input to `LandingCandidatePacket::from_workspace_packet`
- `stabilization_input`: input to `ReviewStabilizationPacket::from_workspace_and_landing_packets`
- `linkage_finalization_input`: input to `WorkItemLinkageFinalizationPacket::from_workspace_and_stabilization_packets`
- `expected`: boolean and count assertions for the inspection record
