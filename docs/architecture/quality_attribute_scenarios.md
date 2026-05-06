# Quality-attribute scenario library

Aureline’s architecture and UI/UX specs define journey SLOs, contract gates, and degradation posture. This
library turns those bars into **named, mechanically consumable scenarios** so benchmark, QE, and
design-review lanes can cite **scenario IDs** instead of re-stating prose expectations.

Authoritative machine-readable sources:

- `artifacts/architecture/quality_scenario_rows.yaml` — scenario row register (fields, anchors, targets).
- `artifacts/qe/quality_scenario_to_lane_map.yaml` — linkage to benchmark slices, QE lanes, and review lanes.
- `fixtures/qe/quality_scenario_cases/` — worked case stubs that point at the current fixture/corpus anchors.

## How to cite scenarios

Use the stable `qas:*` identifier (for example, `qas:startup.warm_first_paint`) in:

- benchmark reports and benchmark waiver packets,
- QE lane evidence summaries,
- design/accessibility/visual review checklists,
- exception and waiver packets.

Consumers MUST cite the ID and MAY quote the title; consumers MUST NOT paraphrase a bar while omitting the
ID, because the ID is how tooling joins evidence, waivers, and drift.

## Scenario row shape

Each row in `artifacts/architecture/quality_scenario_rows.yaml` records:

- **Actor** — who initiates the stimulus (user, admin, extension, remote peer).
- **Stimulus** — the triggering event or intent.
- **Environment** — the declared posture (warm/cold, large repo, restricted mode, offline, etc).
- **Protected journey / sequence anchors** — journey-row refs, protected-path refs, and/or critical-sequence refs.
- **Target response** — SLI + thresholds and the canonical source reference.
- **Measure window** — what is measured (a named metric window), plus the canonical ledger reference.
- **Degradation posture** — required fail-soft behavior and disclosure rules.
- **Evidence lanes** — the primary evidence lane plus supporting lanes that prove the bar.
- **Waiver authority** — which decision forum can approve an exception when a claim or release would widen.

## Scenario index

The full set is in `artifacts/architecture/quality_scenario_rows.yaml`. The index below is a quick navigation
aid, not the source of truth.

| Scenario ID | Title | Posture | Primary anchor |
|---|---|---|---|
| `qas:startup.warm_first_paint` | Warm startup to first paint | Release-blocking | `journey_row:path.shell.launch` |
| `qas:startup.warm_first_useful_chrome` | Warm startup to first useful chrome | Release-blocking | `journey_row:path.shell.first_useful_chrome` |
| `qas:command_palette.open_first_ranked_results` | Command palette open to first ranked results | Release-blocking | `journey_row:path.command_palette.open` |
| `qas:editor.placeholder_open_to_editable_buffer` | Placeholder file open to editable buffer | Release-blocking | `journey_row:path.editor.placeholder_open` |
| `qas:editor.first_accepted_edit_reflected` | First accepted edit reflected on screen | Release-blocking | `journey_row:path.editor.first_useful_edit` |
| `qas:editor.save_to_durable_completion` | Save request to durable completion | Release-blocking | `journey_row:path.editor.save` |
| `qas:workspace.restore_to_delivered_level` | Restore prompt to delivered restore level | Release-blocking | `journey_row:path.workspace.restore` |
| `qas:onboarding.start_center_first_useful_edit` | Start Center first run to first useful edit | Release-blocking | `journey_row:path.onboarding.start_center_first_useful_edit` |
| `qas:remote.attach_or_reconnect_usable_session` | Attach/reconnect to remote workspace (usable session) | Release-blocking | `journey_row:remote_reconnect_to_usable_session` |
| `qas:tooling.rerun_last_task_or_test_dispatch` | Rerun last task/test dispatch to first lifecycle event | Release-blocking | `journey_row:rerun_last_task_or_test_dispatch` |
| `qas:ai.explain_selected_failure_useful_answer` | AI explain selected failure to useful answer | Release-blocking | `journey_row:ai_explain_selected_failure` |
| `qas:repo_open.large_repo_first_navigation` | Open large repo to first useful navigation | Release-blocking | `seq.large_repo.first_navigation` |
| `qas:preview.safe_preview_trust_classes` | Safe preview trust classes and content integrity | Release-blocking | `fixtures/preview/` |
| `qas:editor.save_conflict_resolution` | Save conflict resolution without silent overwrite | Release-blocking | TAD save sequence + `ff.vfs_save_conflict_handling` |
| `qas:ai.multi_file_patch_approval` | AI multi-file patch with explicit approval | Release-blocking | `seq.ai.multi_file_change_approval` |
| `qas:collab.join_follow_shared_debug` | Collaboration join and follow shared debug moment | Release-blocking | `seq.collab.shared_debug_follow` |
| `qas:release.update_rollback_path` | Update/rollback preserves a working build and user state | Release-blocking | PRD lifecycle + fixtures |
| `qas:extensions.quarantine_after_crash_or_revocation` | Extension quarantine after crash loops or revocation | Release-blocking | PRD/TAD quarantine rules + fixtures |
| `qas:recovery.restore_after_failure` | Restore-after-failure preserves truth and avoids silent replay | Release-blocking | restore + recovery ladder fixtures |

