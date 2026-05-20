# Project-entry and workspace-admission report (M3 beta)

This report is the published switching-proof summary for the M3 project-entry and
workspace-admission beta rows. It is regenerated from
[`fixtures/workspace/m3/project_entry_and_admission/manifest.json`](../../../fixtures/workspace/m3/project_entry_and_admission/manifest.json)
and replayed by `cargo test -p aureline-qe --test project_entry_admission_conformance`.
The conformance suite asserts that this report names every drill id, so the report
cannot drift from the corpus it summarizes.

Every entry verb keeps a distinct, reviewable, and honest landing: Open, Clone,
Import, Add root, Restore, and recent-work reopen never collapse into a generic
open, never silently grant trust, never run setup, repo tasks, or hooks, and never
auto-trust or auto-install. The first landing surface always explains why it was
chosen, preserves the Blocking now / Recommended soon / Optional later readiness
grouping, and offers a safe next action. Cross-surface parity preserves the verb
and review model across Start Center, command palette, drag-and-drop, OS open-with,
protocol-handler deep links, CLI/headless, and the workspace switcher, with the
deep-link surface always requiring deep-link intent review.

## Status legend

| Status | Meaning |
| --- | --- |
| Verified | Resulting mode, source access, first landing, readiness grouping, and trust posture all proven by a passing drill. |
| Restricted | Admission is policy-blocked or restricted-mode; the row lands safely but commit stays gated behind an explicit, reviewed choice. |
| Partial | Entry is proven but a claimed surface or follow-on is unproven; treat as Partial in switching proof. |
| Retest pending | Evidence is stale or the builder changed; re-prove before any beta claim cites the row. |

Switching proof, migration docs, and claim-manifest rows must follow the lowest
row status in this report.

## Marketed switching rows (positive drills)

| Drill id | Row | Resulting mode | Source access | First landing | Status |
| --- | --- | --- | --- | --- | --- |
| `open.single_file.os_open` | OS open-with hands a single file to the editor | single_file | local_filesystem | file_editor_with_root_cues | Verified |
| `open.local_folder.start_center` | Start Center opens a local folder | folder | local_filesystem | generic_shell_with_diagnostics | Verified |
| `open.repo_with_nested_candidates.cli` | CLI open of a repo root with nested workspace candidates | repo_root | local_filesystem | generic_shell_with_diagnostics | Verified |
| `open.multi_root_workspace.workspace_switcher` | Workspace switcher reopens a multi-root workspace | workspace_with_roots | local_filesystem | generic_shell_with_diagnostics | Verified |
| `clone.clone_only.start_center` | Start Center clone without opening afterward | clone_only | direct_online | post_clone_handoff | Verified |
| `clone.clone_then_open.command_palette` | Command palette clone-and-open (credential-bearing URL) | clone_then_open | direct_online | post_clone_handoff | Verified |
| `clone.mirror_first.command_palette` | Command palette clone through an internal mirror | clone_then_review | mirror_first | post_clone_handoff | Verified |
| `clone.offline_bundle.cli` | CLI clone from an offline snapshot bundle | clone_then_review | offline_snapshot | post_clone_handoff | Verified |
| `clone.air_gapped.cli` | CLI clone from air-gapped transfer media | clone_then_review | air_gapped_media | post_clone_handoff | Verified |
| `clone.duplicate_destination.cli` | CLI clone whose destination matches a prior clone | clone_then_review | direct_online | post_clone_handoff | Verified |
| `clone.policy_blocked_destination.cli` | CLI clone into a policy-blocked destination | clone_then_review | direct_online | post_clone_handoff | Restricted |
| `import.portable_state.inspect_only.command_palette` | Inspect-only import of a portable-state package | inspect_only | direct_online | import_compare_or_restore_sheet | Verified |
| `import.portable_state.extract_then_review.start_center` | Import that extracts to labelled staging for review | extract_then_review | direct_online | import_compare_or_restore_sheet | Verified |
| `import.competitor_config.apply.command_palette` | Import that applies a competitor config to the active workspace | apply_to_active_workspace | direct_online | import_compare_or_restore_sheet | Verified |
| `add_root.into_active_workspace.workspace_switcher` | Workspace switcher adds a root to the active workspace | workspace_with_roots | local_filesystem | generic_shell_with_diagnostics | Verified |
| `restore.recent_work_reopen.start_center` | Start Center reopens the last session from recent work | restore_last_session | direct_online | restored_layout_with_placeholders | Verified |
| `resume.live_session.workspace_switcher` | Workspace switcher resumes a live managed cloud session | resume_live_session | direct_online | restored_layout_with_placeholders | Verified |
| `deep_link.review_incident.protocol_handler` | Protocol-handler deep link arrives at a review or incident object | repo_root | direct_online | linked_review_incident_or_work_item | Verified |
| `deep_link.clone_managed_open.protocol_handler` | Protocol-handler deep link clones and opens a managed repository | clone_then_open | direct_online | post_clone_handoff | Verified |

## Rejected regressions (negative drills)

Each negative drill builds a valid entry record, applies one typed tamper, and
proves the entry contract rejects it with a specific finding.

| Drill id | Tamper | Required finding |
| --- | --- | --- |
| `negative.clone_grants_trust` | clone_grants_trust | clone must defer trust, dependencies, and tasks |
| `negative.clone_exposes_credentials` | clone_exposes_credentials | clone remote label must not expose credentials |
| `negative.import_writes_before_review` | import_writes_before_review | import must defer durable write and state rehydration |
| `negative.import_inspect_only_advertises_write` | import_inspect_advertises_write | inspect-only import must advertise no write |
| `negative.collision_skips_explicit_choice` | collision_skips_explicit_choice | destination collision requires explicit choice |
| `negative.surface_parity_drift` | surface_parity_drift | surface parity drift on |
| `negative.failure_repair_drops_inputs` | failure_repair_drops_inputs | failed entry repair state must preserve inputs and redacted diagnostics |
| `negative.route_auto_trust` | route_auto_trust | auto_trust_allowed must remain false |
| `negative.route_auto_install` | route_auto_install | auto_install_allowed must remain false |
| `negative.review_sheet_mismatch` | review_sheet_mismatch | review sheet kind does not match entry verb |

## Redaction guarantees

Fixtures carry only typed labels and `~/`-style placeholders; the runner scans
each fixture and the built record for forbidden raw-content tokens (private keys,
absolute home paths, cloud keys, bearer tokens). The
`clone.clone_then_open.command_palette` row carries a credential-bearing source URL
on purpose and proves the built record never reproduces the credential marker.
