# M5 Project-Entry Component Consumers

- Packet: `m5-project-entry-component-consumers:stable:0001`
- As of: `2026-07-06T00:00:00Z`
- Rows: 22 across 5 consumer groups and 10 / 10 frozen families
- Entry verbs with stable command ids: 6
- Families reused across groups: 7

## Rows

- **consumer:start-center:quick-action-open** — surface=start_center_home group=start_center_palette family=start_center_quick_action_card verb=open command=cmd:workspace.open.target authority=full label_parity=preserved handoff=none
- **consumer:start-center:recent-work-row** — surface=start_center_home group=start_center_palette family=recent_work_row verb=open_recent command=cmd:start_center.open_recent authority=full label_parity=preserved handoff=none
- **consumer:open-recent:recent-work-row** — surface=open_recent_list group=start_center_palette family=recent_work_row verb=open_recent command=cmd:start_center.open_recent authority=full label_parity=preserved handoff=none
- **consumer:start-center:restore-prompt** — surface=start_center_home group=start_center_palette family=restore_prompt_card verb=restore command=cmd:workspace.restore_from_checkpoint authority=full label_parity=preserved handoff=none
- **consumer:start-center:workspace-switcher** — surface=start_center_home group=start_center_palette family=workspace_switcher_entry verb=resume command=cmd:remote.open_session authority=full label_parity=preserved handoff=none
- **consumer:palette:entry-chooser-open** — surface=command_palette group=start_center_palette family=entry_chooser_row verb=open command=cmd:workspace.open.target authority=full label_parity=preserved handoff=none
- **consumer:palette:entry-chooser-clone** — surface=command_palette group=start_center_palette family=entry_chooser_row verb=clone command=cmd:workspace.clone_repository authority=full label_parity=preserved handoff=none
- **consumer:system-open:entry-chooser-open** — surface=system_open_file_association group=system_open_intake family=entry_chooser_row verb=open command=cmd:workspace.open.target authority=full label_parity=preserved handoff=none
- **consumer:system-open:entry-review-open** — surface=system_open_file_association group=system_open_intake family=entry_review_sheet verb=open command=cmd:workspace.open.target authority=review_required label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:drag-drop:destination-collision-clone** — surface=drag_and_drop_intake group=system_open_intake family=destination_collision_sheet verb=clone command=cmd:workspace.clone_repository authority=review_required label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:deep-link:entry-review-open** — surface=protocol_deep_link group=deep_link_handoff family=entry_review_sheet verb=open command=cmd:workspace.open.target authority=review_required label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:deep-link:review-link-handoff** — surface=protocol_deep_link group=deep_link_handoff family=post_entry_handoff_card verb=open command=cmd:workspace.open.target authority=read_only label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:browser-mobile:entry-chooser-clone** — surface=browser_mobile_handoff group=deep_link_handoff family=entry_chooser_row verb=clone command=cmd:workspace.clone_repository authority=inspect_only label_parity=disclosed_narrowed handoff=browser_readonly
- **consumer:cli:entry-review-import** — surface=cli_entry group=cli_headless family=entry_review_sheet verb=import command=cmd:workspace.import.bundle authority=full label_parity=preserved handoff=none
- **consumer:cli:archetype-readiness-open** — surface=cli_entry group=cli_headless family=archetype_readiness_row verb=open command=cmd:workspace.open.target authority=export_only label_parity=disclosed_narrowed handoff=handoff_packet
- **consumer:headless:admission-checkpoint-open** — surface=headless_automation group=cli_headless family=admission_checkpoint_card verb=open command=cmd:workspace.open.target authority=inspect_only label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:support-export:recent-work-row** — surface=support_export_replay group=support_diagnostics_docs family=recent_work_row verb=open_recent command=cmd:start_center.open_recent authority=export_only label_parity=disclosed_narrowed handoff=handoff_packet
- **consumer:support-export:post-entry-handoff-import** — surface=support_export_replay group=support_diagnostics_docs family=post_entry_handoff_card verb=import command=cmd:workspace.import.bundle authority=export_only label_parity=disclosed_narrowed handoff=handoff_packet
- **consumer:admin-diagnostics:workspace-switcher** — surface=admin_diagnostics group=support_diagnostics_docs family=workspace_switcher_entry verb=resume command=cmd:remote.open_session authority=read_only label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:admin-diagnostics:destination-collision-clone** — surface=admin_diagnostics group=support_diagnostics_docs family=destination_collision_sheet verb=clone command=cmd:workspace.clone_repository authority=read_only label_parity=disclosed_narrowed handoff=desktop_shell
- **consumer:help-docs:entry-chooser-open** — surface=help_center_docs group=support_diagnostics_docs family=entry_chooser_row verb=open command=cmd:workspace.open.target authority=read_only label_parity=disclosed_narrowed handoff=none
- **consumer:help-docs:restore-prompt** — surface=help_center_docs group=support_diagnostics_docs family=restore_prompt_card verb=restore command=cmd:workspace.restore_from_checkpoint authority=read_only label_parity=disclosed_narrowed handoff=none
