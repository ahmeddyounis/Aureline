# History-Surgery Sessions and First Consumers

- Map: `git-history-session-first-consumers:0001`
- Repository: `repo-ref:main`
- Sessions: 5 / Consumer bindings: 35

## Sessions

- **conflict_session** (`conflict-0001`): state `active_awaiting_resolution`, unresolved 2, actions [continue, abort, skip]
- **sequence_edit_session** (`sequence-0001`): state `running`, unresolved 0, actions [continue, abort, skip, edit_sequence]
- **stash_shelf_entry** (`stash-0001`): state `captured_unapplied`, unresolved 0, actions [apply, pop, drop, create_branch]
- **publish_ref_update_proposal** (`publish-0001`): state `ready_to_publish`, unresolved 0, actions [publish, withdraw]
- **recovery_checkpoint** (`checkpoint-0001`): state `captured_ready_to_restore`, unresolved 0, actions [restore, prune]

## Consumer bindings

- **desktop** → `conflict-0001`: actionable [continue, abort, skip], network_mutation false, recovery_visible true
- **review** → `conflict-0001`: actionable [continue, abort, skip], network_mutation false, recovery_visible true
- **search** → `conflict-0001`: actionable [], network_mutation false, recovery_visible true
- **ai_context** → `conflict-0001`: actionable [], network_mutation false, recovery_visible true
- **cli_headless** → `conflict-0001`: actionable [continue, abort, skip], network_mutation false, recovery_visible true
- **support_export** → `conflict-0001`: actionable [], network_mutation false, recovery_visible true
- **provider_overlay** → `conflict-0001`: actionable [], network_mutation false, recovery_visible true
- **desktop** → `sequence-0001`: actionable [continue, abort, skip, edit_sequence], network_mutation false, recovery_visible true
- **review** → `sequence-0001`: actionable [continue, abort, skip, edit_sequence], network_mutation false, recovery_visible true
- **search** → `sequence-0001`: actionable [], network_mutation false, recovery_visible true
- **ai_context** → `sequence-0001`: actionable [], network_mutation false, recovery_visible true
- **cli_headless** → `sequence-0001`: actionable [continue, abort, skip, edit_sequence], network_mutation false, recovery_visible true
- **support_export** → `sequence-0001`: actionable [], network_mutation false, recovery_visible true
- **provider_overlay** → `sequence-0001`: actionable [], network_mutation false, recovery_visible true
- **desktop** → `stash-0001`: actionable [apply, pop, drop, create_branch], network_mutation false, recovery_visible true
- **review** → `stash-0001`: actionable [apply, pop, drop, create_branch], network_mutation false, recovery_visible true
- **search** → `stash-0001`: actionable [], network_mutation false, recovery_visible true
- **ai_context** → `stash-0001`: actionable [], network_mutation false, recovery_visible true
- **cli_headless** → `stash-0001`: actionable [apply, pop, drop, create_branch], network_mutation false, recovery_visible true
- **support_export** → `stash-0001`: actionable [], network_mutation false, recovery_visible true
- **provider_overlay** → `stash-0001`: actionable [], network_mutation false, recovery_visible true
- **desktop** → `publish-0001`: actionable [publish, withdraw], network_mutation true, recovery_visible true
- **review** → `publish-0001`: actionable [publish, withdraw], network_mutation true, recovery_visible true
- **search** → `publish-0001`: actionable [], network_mutation false, recovery_visible true
- **ai_context** → `publish-0001`: actionable [], network_mutation false, recovery_visible true
- **cli_headless** → `publish-0001`: actionable [publish, withdraw], network_mutation true, recovery_visible true
- **support_export** → `publish-0001`: actionable [], network_mutation false, recovery_visible true
- **provider_overlay** → `publish-0001`: actionable [], network_mutation false, recovery_visible true
- **desktop** → `checkpoint-0001`: actionable [restore, prune], network_mutation false, recovery_visible true
- **review** → `checkpoint-0001`: actionable [restore, prune], network_mutation false, recovery_visible true
- **search** → `checkpoint-0001`: actionable [], network_mutation false, recovery_visible true
- **ai_context** → `checkpoint-0001`: actionable [], network_mutation false, recovery_visible true
- **cli_headless** → `checkpoint-0001`: actionable [restore, prune], network_mutation false, recovery_visible true
- **support_export** → `checkpoint-0001`: actionable [], network_mutation false, recovery_visible true
- **provider_overlay** → `checkpoint-0001`: actionable [], network_mutation false, recovery_visible true
