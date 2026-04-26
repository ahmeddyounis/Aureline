# First-run, open, import, restore, and migration-result object model

This document freezes the cross-surface object model every first-run,
Start Center, project-entry, template / prebuild, workspace-switch,
session-restore, crash-recovery, and migration-handoff surface uses
when it names **what kind of entry the user is making**, **which
target is being acquired**, **which restore level a recovered
session promises**, **which missing-target state is being repaired**,
**which checkpoint-linked recovery class is available**, and **which
migration-result outcome per imported item the user should trust**.

Switching, acquisition, restore, and migration are first-class
launch surfaces. This model freezes their shared vocabulary before
the Start Center, migration center, and recovery ladder are
implemented, so later UX retrofits do not have to reconcile
competing field names across entry flows, recent-work lists, restore
prompts, rollback screens, and migration reports.

The machine-readable schema lives at:

- [`/schemas/workspace/entry_and_restore_result.schema.json`](../../schemas/workspace/entry_and_restore_result.schema.json)

The companion fixtures live under:

- [`/fixtures/workspace/entry_restore_examples/`](../../fixtures/workspace/entry_restore_examples/)

The companion workspace-layout contract lives at:

- [`/docs/workspace/layout_serialization_contract.md`](./layout_serialization_contract.md)

The shared migration and restore playbook lives at:

- [`/docs/state/migration_and_restore_playbook.md`](../state/migration_and_restore_playbook.md)

The dedicated migration-center companion contract lives at:

- [`/docs/migration/migration_center_object_model.md`](../migration/migration_center_object_model.md)

The concrete recent-work row, restore-card, and switcher-row
anatomy companion lives at:

- [`/docs/ux/recent_work_and_restore_card_contract.md`](../ux/recent_work_and_restore_card_contract.md)

This contract is normative. Where it disagrees with the PRD, TAD,
TDD, or UI/UX spec quotations cited in §10, those documents win and
this document MUST be updated in the same change. Where this
document disagrees with a downstream Start Center, migration-center,
continuity, or recovery-ladder surface's mint of its own fields,
this document wins and the surface is non-conforming.

## Why freeze this now

Entry, restore, and migration surfaces tell the same user five
different stories about the same object unless the object is frozen.
A Start Center recent-work card calls the entry a "folder"; the
command-palette row calls it a "project"; OS drag-and-drop calls it
a "target"; the restore prompt calls it a "session"; the migration
report calls it an "import scope." Users cannot reason about safety,
reversibility, or next-step recovery across that drift. Freezing the
record shape — target kind, trust / target state, restore level,
missing-target state, recovery class, migration outcome — is the
prerequisite for honest first-run and switching UX.

Migration is the same hazard in a different coat: if the importer
calls an item "translated" and the rollback screen calls the same
item "applied," and the post-import validator calls it "needs
review," the user cannot tell which of the three is authoritative.
The migration-result taxonomy here makes the outcome per item
exactly one closed value that every surface quotes.

## Scope

- Freeze one `project_entry_action_record` that covers the frozen
  entry verbs **Open**, **Clone**, **Import**, **Add root**,
  **Restore**, **Resume**, and **StartFromSnapshot** across Start
  Center, command palette, OS file association, drag-and-drop,
  deep link, template / prebuild handoff, and CLI / headless entry
  surfaces.
- Freeze one `recent_work_entry_record` that every Start Center,
  workspace-switcher, palette-recents, and CLI-recents surface
  reads so trust, target, restore availability, and safe recovery
  actions are named once.
- Freeze one `restore_prompt_record` that every session-restore,
  crash-recovery, continuity-card, and topology-adjust surface
  reads so restore levels, missing-dependency placeholders, and
  checkpoint-linked recovery actions are named once.
- Freeze one `migration_result_record` whose per-item outcome set
  is closed — `exact`, `translated`, `approximated`, `skipped`,
  `blocked`, `needs_review`, `rollback_available` — and reserves
  typed slots for category-specific parity scores, equivalence-map
  rows, post-import validation outcome references, and next-step
  decisions (`roll_back_import`, `keep_imported_state`,
  `adopt_recommended_bundle`, `review_unsupported_items`).
- Reuse ADR-0001 `trust_state` without redefinition and quote
  ADR-0006 filesystem-identity layers by reference rather than
  re-inventing local object identity.

## Out of scope

- The full Start Center, migration-center, recovery-ladder, or
  first-run implementation. The vocabulary freeze lands here; the
  surfaces that render the records are later milestones.
- The concrete wire format or local-disk encoding for recent-work
  lists, session-restore manifests, or migration reports. The
  schema here is the cross-tool record shape; Rust types in the
  eventual crate are the schema of record once the crate lands.
- Final copy / microcopy for entry verbs, restore-fidelity
  labels, or migration outcome labels. Copy lives with the shell
  interaction-safety contract; this document pins the closed sets
  the copy resolves against.
- The connected-provider / browser-handoff approval-ticket flow.
  Imports from a connected provider carry an ADR-0010 ticket by
  reference; this document does not redefine the ticket.

## 1. Project-entry action record

Every user intent to enter, acquire, or rehydrate work emits
exactly one `project_entry_action_record` on commit. The shell, OS
file association, drag-and-drop handler, deep-link resolver,
command-palette row, CLI / headless entry point, and template /
prebuild handoff all resolve through the same record; downstream
surfaces never mint a parallel shape.

### 1.1 Entry verb

Exactly one `entry_verb` per record. The set is closed:

- `open` — inspect or begin work on an existing local file, folder,
  repository root, workspace, or workset manifest. No remote
  materialization, no import extraction, no rollback checkpoint.
- `clone` — materialize a remote repository locally. Authoring
  trust, dependency restore, and task execution remain later
  inspectable steps; clone alone never implies trust.
- `import` — inspect, extract, or apply a transferred artifact
  (competitor keymap, settings, profile, handoff packet, portable
  state package). Always pairs with a `migration_result_record`
  when the import actually writes items.
- `add_root` — widen an active workspace with a new folder or
  repository. Inherits the active workspace authority and re-
  evaluates per-root trust; never inherits trust from sibling
  roots.
- `restore` — rehydrate a prior session, workspace checkpoint, or
  remembered layout. Always pairs with a `restore_prompt_record`
  that names the restore level and missing-dependency posture.
- `resume` — reattach to a session that is still live (remote
  workspace, managed workspace, companion handoff). Differs from
  `restore` in that no dehydration occurred; authority re-
  evaluation is required even when the session bytes survived.
- `start_from_snapshot` — open a template, prebuild, warm-start
  snapshot, or certified-archetype scaffold. Always discloses
  setup actions, side-effect envelope, and bypass path; never
  collapses into `open` just because the snapshot landed on
  local disk.

Rules (frozen):

1. The seven verbs are not interchangeable. A surface that
   collapses any two into a single generic `get_started` label is
   non-conforming.
2. `open` never implies trust widening, dependency install, or
   task execution. `clone` never implies trust. `import` never
   implies in-place mutation of the source tool. `restore` never
   implies re-running mutating commands. `resume` never implies
   reuse of expired authority. `start_from_snapshot` never hides
   side effects behind generic language.
3. Every `entry_verb` carries a `target_kind` from §1.2. A surface
   that emits a verb without a resolved target kind denies with
   `target_kind_unresolved` rather than silently classify.

### 1.2 Target kind

What is being acquired / opened. The set is closed:

- `local_file`
- `local_folder`
- `local_repo_root`
- `workspace_manifest`
- `workset_manifest`
- `remote_repository`
- `ssh_workspace`
- `container_workspace`
- `devcontainer_workspace`
- `managed_cloud_workspace`
- `portable_state_package`
- `handoff_packet`
- `competitor_config_root`
- `template_or_prebuild_snapshot`
- `review_or_work_item_deep_link`
- `recovery_checkpoint`

Rules (frozen):

1. Every `project_entry_action_record` names exactly one
   `target_kind`. A surface that observes an unresolved target
   denies with `target_kind_unresolved`; silent classification is
   non-conforming.
2. `local_file`, `local_folder`, `local_repo_root`,
   `workspace_manifest`, `workset_manifest`, and
   `recovery_checkpoint` carry a full ADR-0006 filesystem-identity
   record by reference; no parallel identity is minted.
3. `remote_repository`, `ssh_workspace`, `container_workspace`,
   `devcontainer_workspace`, and `managed_cloud_workspace` carry a
   typed `remote_target_descriptor` (host / protocol / auth mode
   / credential handle ref). Raw credentials never appear.
4. `portable_state_package`, `handoff_packet`,
   `competitor_config_root`, and `template_or_prebuild_snapshot`
   carry a typed `artifact_descriptor` (artifact class, schema
   version, producer identity, signature state). The descriptor
   is quoted by id; raw bytes never appear.
5. `review_or_work_item_deep_link` carries a typed
   `deep_link_descriptor` (origin, referenced object id, target
   kind the link resolves into). Deep links never elevate trust.

### 1.3 Resulting mode

What Aureline will actually materialize on commit. The set is
closed:

- `single_file`
- `folder`
- `repo_root`
- `workspace_candidate`
- `workspace_with_roots`
- `workset_slice`
- `inspect_only`
- `clone_then_review`
- `clone_then_open`
- `clone_then_add`
- `clone_only`
- `extract_then_review`
- `compare_before_restore`
- `apply_to_active_workspace`
- `open_prebuild_with_setup_actions`
- `open_prebuild_minimal`
- `resume_live_session`
- `restore_last_session`
- `restore_from_checkpoint`

Rules (frozen):

1. The resulting mode is named before commit. A surface that
   hides whether Aureline will write to disk, contact a remote
   host, or execute setup actions is non-conforming.
2. `inspect_only` and `compare_before_restore` never write files
   outside a labelled non-durable staging area.
3. `open_prebuild_with_setup_actions` always carries a typed
   `side_effect_envelope` (setup actions, networked actions,
   expected time class, cleanup posture, bypass path).

### 1.4 Trust / admission posture

- `trust_state` — reuses ADR-0001 identity-mode vocabulary:
  `trusted`, `restricted`, `pending_evaluation`. Entry surfaces
  never bypass trust merely because the action originated outside
  the Aureline shell.
- `admission_class` — `admitted`, `trust_review_required`,
  `policy_blocked`, `needs_repair`, `needs_reconnect`,
  `needs_reauth`. Every non-`admitted` value routes to a typed
  repair hook named in §1.7.
- `authority_reevaluation_required` — boolean. `true` on
  `resume`, `restore`, and on `open` of any workspace whose last
  session held expired remote authority; `false` only when the
  authority is verifiably live.

### 1.5 Destination disposition

Only present when the verb may materialize bytes on disk.

- `destination_disposition` — one of `no_write`,
  `write_to_labelled_staging`, `write_to_user_destination`,
  `write_to_durable_workspace`, `write_to_admin_owned_root`. The
  shell renders this verbatim; a surface that silently promotes
  `write_to_labelled_staging` to `write_to_durable_workspace` is
  non-conforming.
- `collision_class` — `no_collision`, `reuse_existing`,
  `add_existing`, `clone_elsewhere`, `reveal_only`,
  `destination_blocked`. Every non-`no_collision` value MUST
  appear in a destination-collision sheet before commit.

### 1.6 Source of entry

- `source_surface` — one of `start_center`, `command_palette`,
  `quick_open`, `os_file_association`, `drag_and_drop`,
  `deep_link`, `cli`, `headless_automation`,
  `workspace_switcher`, `recent_work_activation`,
  `template_or_prebuild_card`, `migration_center`,
  `crash_recovery_screen`, `companion_handoff_return`.
- The source surface is recorded so later migration between
  surfaces preserves intent; a recent-work activation from the
  palette and from Start Center resolve through the same record
  shape.

### 1.7 Next-step decision hooks

Every record reserves typed next-step decision hooks. A surface
that wants to continue, repair, or reverse the entry quotes these
values verbatim; free-form action labels are non-conforming.

- `continue_in_restricted_mode`
- `review_trust_and_open`
- `open_minimal`
- `set_up_later`
- `review_archetype_match`
- `compare_before_restore`
- `open_without_restore`
- `safe_mode`
- `reconnect_required`
- `reauth_required`
- `locate_missing_target`
- `remove_from_recents`
- `review_migration_report`
- `roll_back_import`
- `keep_imported_state`
- `adopt_recommended_bundle`
- `review_unsupported_items`

Rules (frozen):

1. Every record carries at least one next-step decision hook.
   A record that lists none is denied with
   `next_step_decision_hook_missing`.
2. The four migration-handoff decisions listed in the task spec —
   `roll_back_import`, `keep_imported_state`,
   `adopt_recommended_bundle`, `review_unsupported_items` — are
   reserved slots every post-import surface reads; a migration
   surface that renders parallel labels is non-conforming.
3. Decision hooks never imply that the user has already accepted
   a destructive path. They are the typed options the shell
   offers, not an audit record of what ran.

## 2. Recent-work entry record

Every Start Center, workspace-switcher, palette-recents, OS jump
list, and CLI-recents surface reads one
`recent_work_entry_record` per row. Surfaces render labels and
chips from these fields; free-text subtitles that contradict the
record are non-conforming.

### 2.1 Identity

- `recent_work_id` — stable opaque id. Reusing an id after
  removal is forbidden.
- `presentation_label` — human-readable workspace / project name.
  Redaction-aware.
- `presentation_subtitle` — redacted path or target subtitle. Raw
  absolute paths never appear under redaction policy.
- `target_kind` — re-exported from §1.2 without modification.
- `pinned` — boolean. Pinned rows survive retention pressure
  that would otherwise drop recent rows.

### 2.2 Target / remote descriptor

- `filesystem_identity_ref` — required when `target_kind` is a
  local kind.
- `remote_target_descriptor_ref` — required when `target_kind` is
  `remote_repository`, `ssh_workspace`, `container_workspace`,
  `devcontainer_workspace`, or `managed_cloud_workspace`.
- `artifact_descriptor_ref` — required when `target_kind` is
  `portable_state_package`, `handoff_packet`,
  `competitor_config_root`, or `template_or_prebuild_snapshot`.

### 2.3 Target state

The closed set a recent-work row renders in its state chip:

- `reachable`
- `stale_metadata`
- `missing_target`
- `moved_target_detected`
- `remote_unreachable`
- `authority_expired`
- `locked_by_other_instance`
- `policy_blocked`
- `quarantined`
- `mode_downgraded`
- `unknown`

Rules (frozen):

1. A row whose target state is anything other than `reachable`
   MUST advertise the safe recovery actions listed in §2.6. A
   surface that offers a single generic `Open anyway` button for
   all non-`reachable` states is non-conforming.
2. `unknown` is allowed only on rows the canonical owner has not
   been re-reached for verification; the chip MUST read
   `unknown` rather than collapse into `reachable`.

### 2.4 Portability class

- `portability_class` — one of `local_only`, `synced`,
  `imported`, `provider_linked`, `stale`. The Start Center chip
  reads this verbatim so local-only vs synced vs imported vs
  provider-linked entries stay distinguishable.

### 2.5 Trust / restore availability

- `trust_state` — ADR-0001 `trusted` / `restricted` /
  `pending_evaluation`.
- `restore_availability` — `exact`, `compatible`, `layout_only`,
  `evidence_only`, `none`. Matches the closed set the
  `restore_prompt_record` uses; a row that promises a higher
  level than the prompt will deliver is non-conforming.
- `last_opened_at` — monotonic timestamp of last authoritative
  open. Opaque format.

### 2.6 Safe recovery actions

The closed set of recovery actions a recent-work row may offer.
Free-form action labels are non-conforming.

- `open`
- `open_in_new_window`
- `open_restricted`
- `locate_missing_target`
- `reconnect`
- `reauth`
- `open_read_only_cached_view`
- `retry_later`
- `compare_before_restore`
- `open_without_restore`
- `unpin`
- `pin`
- `remove_from_recents`
- `reveal_in_explorer`

Every row exposes at least one recovery action consistent with
its target state and restore availability.

### 2.7 Reserved references

Every row reserves typed refs so later surfaces attach without
rewriting rows:

- `parity_score_refs[]` — per-category parity scores where the
  recent entry arrived through a migration import.
- `equivalence_map_row_refs[]` — per-item equivalence rows from
  the source migration.
- `post_import_validation_refs[]` — outcome refs from the last
  post-import validation.
- `bundle_adoption_ref` — optional reference to an adopted
  workflow bundle.
- `recovery_checkpoint_refs[]` — checkpoint-linked recovery
  class instances available for this row (see §4).

## 3. Restore prompt record

Every session-restore, crash-recovery, topology-adjust, continuity
card, and "restored previous session" affordance emits one
`restore_prompt_record`. The shell surface quotes this record
verbatim; there is no parallel restore-level vocabulary.

### 3.1 Restore level

The closed set (matches UI/UX spec §6.10 so diagnostics, crash
screens, support bundles, and docs stay aligned):

- `exact_restore`
- `compatible_restore`
- `layout_only`
- `recovered_drafts`
- `evidence_only`
- `no_restore`

Rules (frozen):

1. A surface that promises a higher restore level than the
   record declares is non-conforming.
2. `exact_restore` forbids silent re-execution of mutating
   commands, notebook cells, debug attach, AI tool calls,
   publish / promote actions, or remote mutations. The shell
   reads this as layout + dirty-buffer rehydration only.
3. `recovered_drafts` is always paired with a
   `dirty_buffer_summary` and a "compare / discard / open
   journal" action set.

### 3.2 Missing target state

The closed set every restore surface names before the user
commits:

- `fully_present`
- `missing_extension_host`
- `quarantined_extension`
- `missing_remote_target`
- `authority_expired_for_remote`
- `missing_toolchain`
- `missing_profile`
- `missing_devcontainer`
- `missing_managed_workspace`
- `topology_changed`
- `schema_migrated`
- `binary_or_extension_version_changed`
- `corrupt_restorable_state`
- `policy_blocked_restore`

Rules (frozen):

1. Every `restore_prompt_record` carries at least one
   `missing_target_state` entry. `fully_present` is the only
   value that may appear alone.
2. `corrupt_restorable_state` MUST pair with a typed
   quarantine / export action; a surface that silently discards
   corrupt state is non-conforming.

### 3.3 Session-scoped execution posture

The closed set for terminals, task runs, debug sessions,
notebook kernels, preview servers, and remote tunnels:

- `transcript_restored_not_rerun`
- `session_ended`
- `reconnect_available`
- `rerun_required`
- `context_unavailable`
- `live_session_continued`

`live_session_continued` is reachable only when the runtime
actually survived; inferring it from absence of evidence is
forbidden.

### 3.4 Checkpoint-linked recovery class

Every prompt reserves typed refs for §4 recovery classes. The
shell renders the class-verb verbatim; `Undo` is forbidden on
compensating or regenerate-class recovery.

## 4. Checkpoint-linked recovery classes

Every restore, rollback, or next-step-after-migration surface
names exactly one recovery class per offered action. The set is
closed and orthogonal to the §3 restore level.

- `exact_rollback_from_checkpoint` — restores the last-known-good
  state recorded at a durable checkpoint without compensating
  side effects.
- `compensating_rollback` — rolls back imported or migrated state
  by running a compensating action; the user is told `Revert with
  compensation` rather than `Undo`.
- `regenerate_from_canonical_source` — for generated-artifact
  drift or template / prebuild re-materialization; pairs with a
  `generated_artifact_lineage_record`.
- `restore_from_session_checkpoint` — rehydrate a saved session /
  layout checkpoint.
- `restore_from_migration_checkpoint` — restore the migration
  import-commit checkpoint (pre-apply snapshot of affected
  settings / profiles / manifests).
- `restore_from_settings_backup` — restore a user-profile /
  keymap / snippet backup emitted before an automatic migration.
- `restore_from_recovery_journal` — rehydrate dirty buffers from
  the ADR-0003 recovery journal.
- `restore_from_local_history_snapshot` — rehydrate a file /
  group from local-history.
- `manual_recovery_required` — no automatic class applies; the
  shell exposes the narrowest safe repair plus an export path.

Rules (frozen):

1. Every recovery action cited by a restore prompt, migration
   result, or recent-work row names exactly one recovery class
   from this set. Free-form recovery verbs are non-conforming.
2. The recovery class determines the user-visible verb (`Undo`
   only for `exact_rollback_from_checkpoint`; `Revert with
   compensation` for `compensating_rollback`; `Regenerate` for
   `regenerate_from_canonical_source`; `Restore` for the
   `*_checkpoint`, `*_journal`, `*_snapshot`, and
   `*_settings_backup` classes; `Manual recovery` for
   `manual_recovery_required`).
3. Every non-`manual_recovery_required` class carries a
   `checkpoint_ref` resolved by the mutation-journal
   `checkpoint_kind` vocabulary (ADR-0003 / ADR-0006). Parallel
   checkpoint vocabularies are non-conforming.

## 5. Migration-result record

Every import flow that writes items (competitor config import,
portable-state restore, profile / keymap adoption, bundle handoff)
emits exactly one `migration_result_record`. The migration center,
post-import validation card, support bundle, and any "Adopt
recommended bundle" / "Roll back import" surface read this record
verbatim.

### 5.1 Per-item outcome

The closed set (every imported item carries exactly one):

- `exact` — translated with preserved meaning and no
  approximation.
- `translated` — mapped to a semantic equivalent; confidence is
  documented per row but the item is considered applied.
- `approximated` — partial / capability-based mapping; the user
  is told the item is approximate and not an exact parity claim.
- `skipped` — intentionally not applied (policy, user choice,
  unsupported by target); paired with a typed `skip_reason`.
- `blocked` — could not be applied because policy or trust
  forbade it; paired with a typed `block_reason`.
- `needs_review` — applied but requires user review before it
  rides as trusted workflow (e.g., keybinding conflict,
  extension capability mapping uncertainty).
- `rollback_available` — item was applied but the pre-apply
  checkpoint is retained and a typed rollback class is offered.
  Not mutually exclusive with `translated` / `approximated` /
  `needs_review`.

Rules (frozen):

1. Every imported item names exactly one `outcome`. `skipped`
   MUST carry a typed `skip_reason` (`user_deselected`,
   `policy_excludes`, `unsupported_by_target`,
   `redundant_with_existing`, `source_missing`,
   `lossy_mapping_refused`). Free-form skip strings are
   non-conforming.
2. Every `blocked` item MUST carry a typed `block_reason`
   (`workspace_trust_excludes`, `admin_policy_excludes`,
   `extension_effective_permission_excludes`,
   `connected_provider_policy_excludes`, `signature_mismatch`,
   `schema_version_unsupported`). Free-form block strings are
   non-conforming.
3. `rollback_available` is a companion axis: every imported item
   MAY additionally carry `rollback_available` when the
   migration retained a pre-apply checkpoint. The next-step
   decision `roll_back_import` reads this axis; a surface that
   offers rollback without the checkpoint reference is non-
   conforming.
4. `needs_review` MUST name the review hook (`keybinding
   conflict`, `extension capability uncertain`, `launch config
   semantics differ`, `settings migration lossy`). Every
   `needs_review` item carries at least one
   `post_import_validation_ref` pointing at the validator that
   flagged it.

### 5.2 Parity score refs (per category)

The migration result reserves a typed slot for per-category
parity scores. The UI / UX spec Appendix DP freezes the category
set; this schema reserves the slots so scorecards attach without
re-defining fields.

- `parity_score_refs[]` — each entry names a
  `category` (`keymap`, `theme_and_visuals`, `settings`,
  `tasks_and_run_configs`, `launch_debug`,
  `extensions_and_providers`, `snippets_and_templates`, `other`),
  a `score` (integer 0–100), a `confidence_class` (`high`,
  `medium`, `low`, `insufficient_evidence`), and an optional
  `evidence_ref`.
- Category-specific parity is mandatory; a single aggregate
  score that hides weak categories is non-conforming.

### 5.3 Equivalence-map rows

- `equivalence_map_row_refs[]` — per-item equivalence rows
  naming `source_id` (opaque), `target_id` (opaque),
  `mapping_basis` (`exact_identity`, `semantic_equivalent`,
  `capability_based`, `name_heuristic`, `user_override`), and a
  `caveat` string (≤ 512 graphemes, redaction-aware).
- `name_heuristic` is allowed only when no stronger basis exists
  and the row is flagged for user review.

### 5.4 Post-import validation refs

- `post_import_validation_refs[]` — each entry names a
  `validator_class` (`keybinding_conflict`,
  `missing_extension_suggestion`, `launch_config_sanity`,
  `cli_headless_smoke`, `workflow_smoke`,
  `settings_schema_migration`, `bundle_readiness`) and an
  `outcome_class` (`passed`, `passed_with_warnings`,
  `failed_recoverable`, `failed_blocking`,
  `skipped_not_applicable`). Free-form outcome strings are
  non-conforming.

### 5.5 Reserved next-step decision hooks

Every migration result reserves — at minimum — the four next-step
decisions named in the task spec. A migration-center surface that
renders parallel labels is non-conforming:

- `roll_back_import`
- `keep_imported_state`
- `adopt_recommended_bundle`
- `review_unsupported_items`

Additional hooks from §1.7 MAY be cited; the four above MUST be
available whenever the outcome mix admits them (e.g.,
`roll_back_import` requires at least one
`rollback_available` item or a migration-level rollback
checkpoint).

## 6. Worked examples

Each example references a companion fixture under
[`/fixtures/workspace/entry_restore_examples/`](../../fixtures/workspace/entry_restore_examples/).

### 6.1 Open local folder

The user opens a local folder from the Start Center. One
`project_entry_action_record` with `entry_verb = open`,
`target_kind = local_folder`, `resulting_mode = folder`,
`destination_disposition = no_write`,
`admission_class = admitted`, `authority_reevaluation_required =
false`, and a single next-step decision hook
`review_archetype_match`.

See [`open_local_folder.json`](../../fixtures/workspace/entry_restore_examples/open_local_folder.json).

### 6.2 Clone remote repository

The user clones `github.com/acme/payments` with
`clone_then_review`. The record names
`remote_target_descriptor_ref` (protocol, auth mode,
credential-handle ref), `resulting_mode = clone_then_review`,
`destination_disposition = write_to_user_destination`,
`collision_class = no_collision`,
`authority_reevaluation_required = true`, and next-step hooks
`review_trust_and_open` and `set_up_later`. Trust is not widened
by the clone.

See [`clone_remote_repo.json`](../../fixtures/workspace/entry_restore_examples/clone_remote_repo.json).

### 6.3 Import VS Code settings / keybindings

The user imports competitor config from a VS Code install. The
entry action record emits `entry_verb = import`,
`target_kind = competitor_config_root`,
`resulting_mode = apply_to_active_workspace`. A companion
`migration_result_record` lists per-item outcomes — two
`exact`, three `translated`, one `approximated`, one
`skipped` with `unsupported_by_target`, one `needs_review`
with a `keybinding_conflict` validator ref, and a migration-
level `rollback_available` — plus category parity scores for
`keymap`, `theme_and_visuals`, `tasks_and_run_configs`, and
`extensions_and_providers`. Reserved next-step hooks are all
four migration decisions.

See [`vs_code_settings_import.json`](../../fixtures/workspace/entry_restore_examples/vs_code_settings_import.json)
and [`vs_code_settings_import_result.json`](../../fixtures/workspace/entry_restore_examples/vs_code_settings_import_result.json).

### 6.4 Restore last session after crash

The user reopens after a crash. `restore_prompt_record` with
`restore_level = compatible_restore` (one extension was updated
across the restart), `missing_target_states =
[binary_or_extension_version_changed,
missing_extension_host]`, session-scoped execution posture
`transcript_restored_not_rerun` for terminal panes and
`rerun_required` for a task pane, and checkpoint-linked
recovery classes `restore_from_session_checkpoint` plus
`restore_from_recovery_journal` for dirty buffers. Next-step
decision hooks: `open_without_restore` and `safe_mode`.

See [`restore_last_session.json`](../../fixtures/workspace/entry_restore_examples/restore_last_session.json).

### 6.5 Resume managed cloud workspace

The user reopens a managed cloud workspace that is still live.
Entry verb `resume`, target kind `managed_cloud_workspace`,
resulting mode `resume_live_session`,
`authority_reevaluation_required = true` (managed authority
ticket MUST be re-evaluated even when the session bytes
survived). Next-step hooks: `reauth_required`,
`continue_in_restricted_mode`.

See [`resume_managed_workspace.json`](../../fixtures/workspace/entry_restore_examples/resume_managed_workspace.json).

### 6.6 Start from template / prebuild snapshot

The user opens a TS web-app template from the Start Center.
Entry verb `start_from_snapshot`, target kind
`template_or_prebuild_snapshot`, resulting mode
`open_prebuild_with_setup_actions`, a `side_effect_envelope`
naming the setup actions (Node toolchain detect, dependency
install, devcontainer attach) with time / connectivity class
and bypass path. The bypass path carries `open_prebuild_minimal`;
next-step hooks include `open_minimal` and `set_up_later`.

See [`start_from_prebuild.json`](../../fixtures/workspace/entry_restore_examples/start_from_prebuild.json).

### 6.7 Recent-work row with missing target

A Start Center recent-work row for a repository whose folder was
moved on disk. `target_state = missing_target`, restore
availability `layout_only`, safe recovery actions
`locate_missing_target` and `remove_from_recents`. The row
preserves the stale-but-inspectable metadata without claiming
`reachable`.

See [`recent_work_missing_target.json`](../../fixtures/workspace/entry_restore_examples/recent_work_missing_target.json).

## 7. Surface rules

These rules apply to every surface that renders, logs, exports,
or reasons about the records defined in §1 – §5.

1. **One canonical record shape.** No surface mints private
   entry-verb, target-kind, restore-level, missing-target,
   recovery-class, or migration-outcome vocabularies. Downstream
   surfaces read the record by id.
2. **Verbs stay distinct across surfaces.** `Open`, `Clone`,
   `Import`, `Add root`, `Restore`, `Resume`, and
   `StartFromSnapshot` render as separate rows / buttons /
   commands in Start Center, command palette, OS file
   association, drag-and-drop, deep link, CLI, and headless
   entry. A `Get started` collapse is non-conforming.
3. **Trust / admission never widens silently.** An entry verb
   may not produce trust widening, dependency restore, hook
   execution, or secret projection as a side effect. Those are
   separate reviewed steps.
4. **Restore-level language is canonical.** `Exact restore`,
   `Compatible restore`, `Layout only`, `Recovered drafts`, and
   `Evidence only` render verbatim. Diagnostics, crash screens,
   support bundles, and docs reuse them unchanged.
5. **Missing-target states carry actions.** A `missing_target`,
   `moved_target_detected`, `remote_unreachable`,
   `authority_expired`, `policy_blocked`, `quarantined`, or
   `corrupt_restorable_state` row MUST expose at least one
   typed safe recovery action.
6. **Migration outcomes are closed and per-item.** A migration
   UI that collapses `approximated` into `translated` or hides
   `skipped` / `blocked` items is non-conforming. Category
   parity scores remain separable; a single aggregate score is
   insufficient.
7. **Next-step decisions are typed.** Free-form action labels
   on entry, restore, and migration surfaces are non-conforming.
   The four migration-handoff hooks — `roll_back_import`,
   `keep_imported_state`, `adopt_recommended_bundle`,
   `review_unsupported_items` — are reserved on every migration
   result that admits them.
8. **Support parity.** Every record shape exports through the
   support bundle, crash manifest, and migration-report families
   with the same fields it renders in chrome. Redaction is the
   only way to hide a field.

## 8. Changing this vocabulary

- **Additive-minor** changes (new `target_kind`, new
  `resulting_mode`, new `missing_target_state`, new
  `checkpoint_linked_recovery_class`, new migration
  `skip_reason` / `block_reason`, new
  `post_import_validation` `validator_class`, new
  `next_step_decision_hook`) land here and in the companion
  schema in the same change. The change MUST cite the motivating
  fixture or packet.
- **Repurposing** an existing verb, target kind, restore level,
  missing-target state, recovery class, or migration outcome is
  breaking. It opens a new decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- The PRD / TAD / TDD / UI-UX spec wins on any disagreement with
  the quotations in §10; this document and the schema are updated
  in the same change.

## 9. Acceptance

- The model distinguishes `Open`, `Clone`, `Import`, `Restore`,
  `Resume`, `StartFromSnapshot` (and `Add root`) unambiguously
  via the closed `entry_verb` set and the typed `target_kind` /
  `resulting_mode` companions.
- Recent-work and restore records expose `target_kind`,
  `trust_state`, `target_state`, `restore_availability`, and
  typed safe recovery actions instead of free text.
- The migration-result taxonomy covers the seven outcomes named
  in the task spec — `exact`, `translated`, `approximated`,
  `skipped`, `blocked`, `needs_review`, and `rollback_available`
  — and reserves category-specific parity scores, equivalence-
  map rows, post-import validation refs, and the four next-step
  decisions `roll_back_import`, `keep_imported_state`,
  `adopt_recommended_bundle`, `review_unsupported_items`.
- The schema at
  [`/schemas/workspace/entry_and_restore_result.schema.json`](../../schemas/workspace/entry_and_restore_result.schema.json)
  validates the seven worked-example fixtures under
  [`/fixtures/workspace/entry_restore_examples/`](../../fixtures/workspace/entry_restore_examples/).
- The object model is rich enough that later migration
  scorecards, validation packets, and recovery-ladder rungs can
  attach by reference without redefining entry or restore
  results.
- The object model is referenced by Start Center, continuity,
  migration-center, and recovery-ladder planning docs via the
  linked artifacts in §11.

## 10. Source anchors

- `.t2/docs/Aureline_PRD.md:284` — first-run onboarding is an
  identified launch risk, not a polish afterthought.
- `.t2/docs/Aureline_PRD.md:1293` — recovery-journal and
  autosave truth: "Forward-readable where practical; bounded
  retention window".
- `.t2/docs/Aureline_PRD.md:1300` — "crash recovery must degrade
  gracefully from 'exact session restore' to 'recover dirty
  buffers' to 'open clean with preserved evidence,' never
  directly to silent loss".
- `.t2/docs/Aureline_PRD.md:1703` — "repeated startup failure
  should offer a clear recovery ladder: safe mode → extension
  quarantine → cache reset → workspace restricted mode".
- `.t2/docs/Aureline_PRD.md:2680` — "Any destructive config or
  workspace migration must create a checkpoint that can be
  restored automatically if startup fails".
- `.t2/docs/Aureline_PRD.md:3311` — "first-run onboarding should
  detect existing tools and offer a dry-run import with a diff
  preview and rollback option".
- `.t2/docs/Aureline_PRD.md:4670` — required migration source
  scope: VS Code settings, keybindings, snippets, tasks, launch
  configs, themes, selected extensions.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:3244` —
  "Preview and parity scoring — show a dry-run diff, unsupported
  items, behavior changes, and an estimated workflow-parity
  score before apply".
- `.t2/docs/Aureline_Technical_Design_Document.md:9147` —
  migration / import parity fixture corpus, dry-run diff packet,
  rollback drill, compatibility scorecard snapshot.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:762` — Start Center
  primary actions: `Open folder`, `Open workspace`, `Clone
  repository`, `Restore last session`, `Import from…`.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:784` — recent-work
  row fields (workspace/project name, path, kind icon,
  last-opened timestamp, trust state, restore availability, pin
  / remove / locate / reconnect actions).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:836` — restore-class
  taxonomy (Exact session restore, Context restore with
  placeholders, Dirty-buffer recovery, Checkpoint rollback,
  Evidence-only recovery).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:866` —
  restore-fidelity rules and the `Exact restore` / `Compatible
  restore` / `Layout only` / `Recovered drafts` / `Evidence
  only` controlled terms.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:1068` — Project
  entry, clone / import / open, and workspace-admission flows.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:9579` — Migration
  equivalence maps, post-import validation, and competitor
  handoff UX (including the four required next-step decisions
  `Roll back import`, `Keep imported state`, `Adopt recommended
  bundle`, `Review unsupported items`).
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:17008` — Appendix DP
  migration report, parity score, and equivalence-map templates
  (source discovery, imported item row, parity scoreboard,
  compatible extension mapping, unsupported-item banner,
  post-import validation summary).
- `.t2/docs/Aureline_Milestones_Document.md:1023` — Start
  Center and primary shell entry points MUST keep `Open`,
  `Clone`, `Import`, `Restore`, and `Recent work` distinct with
  a no-account path to useful local work.
- `.t2/docs/Aureline_Milestones_Document.md:1544` — entry verbs
  stay distinct across Start Center, command surfaces, OS file
  association, drag-and-drop, and CLI / headless entry.

## 11. Linked artifacts

- ADR (identity modes, trust vocabulary):
  [`docs/adr/0001-identity-modes.md`](../adr/0001-identity-modes.md).
- ADR (filesystem identity, save pipeline, cache identity):
  [`docs/adr/0006-vfs-save-cache-identity.md`](../adr/0006-vfs-save-cache-identity.md).
- ADR (buffer, recovery journal, undo classes):
  [`docs/adr/0003-buffer-undo-large-file.md`](../adr/0003-buffer-undo-large-file.md).
- ADR (connected-provider / browser-handoff tickets):
  [`docs/adr/0010-connected-provider-browser-handoff-approval-ticket.md`](../adr/0010-connected-provider-browser-handoff-approval-ticket.md).
- Mutation-journal and generated-artifact lineage vocabulary:
  [`docs/workspace/mutation_lineage_model.md`](./mutation_lineage_model.md).
- Filesystem-identity vocabulary (cross-surface):
  [`docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md).
- Entry / restore / migration schema:
  [`schemas/workspace/entry_and_restore_result.schema.json`](../../schemas/workspace/entry_and_restore_result.schema.json).
- Worked-example fixtures:
  [`fixtures/workspace/entry_restore_examples/`](../../fixtures/workspace/entry_restore_examples/).
- Onboarding / first-useful-work / migration measurement plan
  (reads the entry-restore records as the canonical record shape
  for measurement):
  [`docs/product/onboarding_measurement_plan.md`](../product/onboarding_measurement_plan.md).
- Start Center, workspace-switcher, and open-flow disclosure
  contract (wraps `recent_work_entry_record`,
  `restore_prompt_record`, and the §1 entry verbs with a frozen
  disclosure posture and zone vocabulary):
  [`docs/ux/start_center_contract.md`](../ux/start_center_contract.md)
  and
  [`schemas/ux/start_center_surface.schema.json`](../../schemas/ux/start_center_surface.schema.json).
  Task-success corpus seed:
  [`artifacts/product/task_success_corpus_seed.yaml`](../../artifacts/product/task_success_corpus_seed.yaml).
  No-account switching scoreboard seed:
  [`artifacts/product/no_account_switching_scoreboard_seed.yaml`](../../artifacts/product/no_account_switching_scoreboard_seed.yaml).
