# Portable-profile artifact and configuration / state map

This document freezes the cross-surface vocabulary every profile
export / import, managed-sync, support-bundle, restore / migration,
and remembered-state-inspector surface uses when it names **what kind
of state this is**, **which authority owns it**, **where it lives on
disk or in a signed bundle**, **whether it may travel across machines
or org boundaries**, and **how honest a restore from a given source
artifact can claim to be**.

Portability, remembered state, and restore provenance are first-class
concerns from the start, not a later UX retrofit. This document
freezes the shared vocabulary before the Start Center, migration
center, settings UI, support / export, optional-sync lane, and
remembered-state inspector are implemented, so later persistence
surfaces compose over the same fields rather than re-inventing a
parallel location / portability / fidelity dialect.

The machine-readable schema lives at:

- [`/schemas/profile/portable_profile.schema.json`](../../schemas/profile/portable_profile.schema.json)

The companion fixtures live under:

- [`/fixtures/profile/restore_provenance_examples/`](../../fixtures/profile/restore_provenance_examples/)

The companion workspace-layout contract lives at:

- [`/docs/workspace/layout_serialization_contract.md`](../workspace/layout_serialization_contract.md)

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
or UI/UX spec quotations cited in §9, those documents win and this
document MUST be updated in the same change. Where this document
disagrees with a downstream settings UI, migration-center, or
support-export surface's mint of its own fields, this document wins
and the surface is non-conforming.

## Why freeze this now

Profile, sync, and restore surfaces each tell the same user a slightly
different story unless the vocabulary is frozen. The profile UI calls
an AI preset selection "included"; the managed-sync lane calls it
"roaming"; the support-bundle exporter calls it "declared content";
the migration-center calls it "imported"; the remembered-state
inspector calls it "remembered." Users cannot reason about what
crosses a machine boundary, what a stale remote copy may overwrite, or
what a restore from a support bundle will actually do unless every
surface quotes the same record shape.

Equally, the state map has to pin where durable state lives before
persistence-specific tasks (sync engine, conflict journal, policy
bundle cache, execution-context cache, AI outbox) land. Without that
pin, each task invents its own path concept and each support /
recovery / clear-data flow reclassifies state ad hoc. The state-map
rows here reserve the location vocabulary every later artifact-
specific surface reads verbatim.

## Scope

- Freeze one `portable_profile_artifact_record` shape that describes
  the diffable, text-based `*.aureprofile.json` body a user exports /
  imports to move settings, keybindings, snippets, themes, layout
  presets, extension selections, AI preset references, and terminal
  preferences across machines without secrets, trust approvals,
  machine-unique trust anchors, or admin-policy bundles.
- Freeze one `state_map_row_record` shape that enumerates the
  location-root + location-path-concept + authority + portability
  class + redaction class + clear posture + inspector posture + sync
  posture + support-export posture for every state class the product
  holds, so later persistence tasks inherit a stable map.
- Freeze one `export_manifest_record` shape that names what an export
  actually wrote, what it declined, and what it redacted.
- Freeze one `restore_provenance_record` shape whose fidelity labels
  are closed — `exact`, `compatible`, `layout_only`, `manual_review` —
  so restore / migration surfaces never overclaim.
- Reserve the **remembered-state inspector** surface fields
  (`stable_pane_id`, `state_classes_exposed`, `export_action_available`,
  `clear_action_available`, `compare_action_available`) so later
  persistence UI does not mint a parallel shape.
- Reserve the **Appendix-F-style path rows** for profile library,
  sync metadata and conflict journals, terminal restore metadata,
  policy bundle cache, execution-context cache, extension lockfile,
  AI / outbox metadata, local history, and logs / traces so later
  artifact-specific tasks inherit a stable location vocabulary.

## Out of scope

- The full sync engine, migration executor, profile UI, or support-
  export implementation. The vocabulary freeze lands here; the
  surfaces that render and mutate the records are later milestones.
- The concrete on-disk encoding, wire format, or encryption envelope
  for a portable profile or managed-sync payload. The schema here is
  the cross-tool record shape; the eventual profile / state-map /
  migration crates' Rust types are the schema of record once they
  land.
- Final copy / microcopy for portability badges, fidelity labels, or
  sync-conflict strings. Copy lives with the shell interaction-safety
  contract; this document pins the closed sets the copy resolves
  against.
- The ADR-0007 secret broker's storage backend. Secret classes are
  quoted by reference; the portable profile never carries raw bytes.
- The ADR-0010 connected-provider / browser-handoff approval-ticket
  flow. Imports from a managed source quote the ticket by reference;
  this document does not redefine it.

## 1. State authority classes

Every state class answers **who owns this?** The set is closed:

- `user_authored_durable_truth` — the user typed it or accepted it via
  a preview. Losing it requires an explicit delete. Examples: user-
  global settings, keybindings, snippets, themes, extension selection,
  AI preset selection, terminal preferences, workspace manifest,
  workset manifest, tasks and launch configs.
- `user_owned_recovery_state` — the product held it on the user's
  behalf so a crash or restart does not lose work. It is local, but
  the user still owns it and a Clear action requires a preview.
  Examples: dirty-buffer recovery journal, session-restore state,
  local history, terminal restore metadata (redacted).
- `admin_or_control_artifact` — a signed bundle, policy cache,
  offline entitlement, or trust-root update authored by admin /
  control authority. The user may inspect a decision trace but may
  not clear or edit the payload. Examples: admin policy bundle,
  policy bundle cache, trust approvals recorded against a signed
  bundle epoch.
- `disposable_derived_cache` — fully regenerable from other truth. A
  Clear action is allowed at any time. Examples: index / cache store,
  object store, execution-context cache, AI memory metadata (the
  cache-key metadata, not any retained body), extension lockfile as
  a derived lockset, logs / traces once retention bounds hit.

Rules (frozen):

1. A state class MUST resolve to exactly one authority. A surface
   that silently reclassifies a row (for example, treating dirty-
   buffer recovery as a disposable cache) is non-conforming.
2. `user_authored_durable_truth` is the only class a portable profile
   body carries as its primary content. Every other class either
   travels as a local-only addendum, as an admin-owned bundle with
   its own signature path, or not at all.
3. A `Clear data` or `Reset` flow MUST honour the authority class. A
   clear action on `user_authored_durable_truth` requires a preview +
   rollback checkpoint; on `user_owned_recovery_state` requires a
   preview; on `admin_or_control_artifact` is denied at the surface
   and routed through the signed-bundle path; on
   `disposable_derived_cache` is allowed without preview.

## 2. Portability classes

- `portable` — carried in a `*.aureprofile.json` body and round-trips
  across machines and org boundaries.
- `portable_with_machine_addendum` — the portable body carries the
  shared preference; a companion machine-local addendum carries the
  machine-bound half (for example, a local toolchain path hint). The
  addendum is itself `local_only` and never synced by default.
- `local_only` — never crosses a machine boundary via a portable
  profile or by default sync. Machine-specific settings, the sync
  metadata / conflict journal itself, and the terminal restore
  metadata sit here.
- `shared_workspace_only` — travels inside a workspace export, not a
  user profile. Workspace manifest, workset manifest, tasks and
  launch configs sit here.
- `admin_owned` — authored by signed admin bundle; travels via the
  bundle distribution path and never via user profile.
- `excluded` — documented here explicitly so later surfaces cannot
  quietly reclassify it. Auth session secrets, long-lived
  credentials, trust approvals, dirty-buffer recovery journal, and
  admin policy bundle sit here for a user profile; auth secrets also
  sit here for every export manifest destined outside the OS
  credential store.

Rules (frozen):

1. Every row in the state map names exactly one portability class. A
   surface that silently promotes `local_only` to `portable` is non-
   conforming.
2. `portable_with_machine_addendum` always carries the addendum's
   exclusion reason on the machine-local side. The reason set is the
   frozen `exclusion_reason_id` vocabulary.
3. `excluded` rows MUST name an `exclusion_reason_id`. Free-form
   reasons are non-conforming.

## 3. Profile modes

A portable profile artifact declares exactly one mode:

- `file_portable_plain` — text-based `*.aureprofile.json`. Diffable.
  No secrets, no trust approvals, no machine-unique anchors. Default
  for user-initiated export.
- `file_portable_encrypted` — same body, encrypted at rest by a key
  the user or enterprise owns.
- `managed_sync_opt_in` — the managed-sync lane carries the body.
  Customer-managed key variants use `customer_managed_key_sync`.
- `customer_managed_key_sync` — managed sync with a customer-managed
  encryption key.
- `self_hosted_sync` — self-hosted sync service carries the body.
- `support_recovery_only` — not a portable profile at all; a support /
  recovery manifest that may include redacted excerpts under opt-in.

Rules (frozen):

1. A managed-sync mode MUST NOT carry a state class whose row marks
   `sync_posture = never_synced`. The resolver denies the write; the
   state map is the source of truth.
2. A `file_portable_plain` artifact MUST list
   `auth_session_secrets`, `long_lived_credentials`, `trust_approvals`,
   `admin_policy_bundle`, `dirty_buffer_recovery_journal`, and
   `machine_specific_settings` (unless paired as an addendum) in
   `excluded_state_classes`.

## 4. Fidelity labels

A restore-provenance record names exactly one fidelity label. The set
is closed:

- `exact` — every included state class round-tripped byte-for-byte
  modulo documented canonicalisation. No equivalence mapping, no
  user acknowledgement, no post-restore warning.
- `compatible` — at least one class translated through a declared
  equivalence-map row (quoted by reference, never inlined). A
  rollback checkpoint was created before apply. All post-restore
  validators returned `passed` or `passed_with_warnings`.
- `layout_only` — only window topology, pane layout, and visual
  tokens restored. No authored settings crossed. Reserved for
  cross-machine / cross-channel restore where the authored surface
  is still authoritative on the destination.
- `manual_review` — one or more included classes require explicit
  human review before apply. At least one validator returned
  `failed_recoverable` or `failed_blocking`, or an equivalence-map
  row is flagged for review. The restore surface surfaces the
  companion `compare_ref` affordance.

Rules (frozen):

1. A producer that labels a restore `exact` when any class required
   translation, user acknowledgement, or a rollback checkpoint is
   non-conforming.
2. `compatible` and `manual_review` MUST set `rollback_checkpoint_ref`
   and `equivalence_map_ref`. `layout_only` MAY omit both.
3. `fidelity_label` is the producer's claim about the artifact;
   `restore_level` (re-exported from the entry-restore object model)
   is the surface's decision at apply time. A surface that renders a
   higher `restore_level` than the record's `fidelity_label` admits
   is non-conforming.

## 5. Remembered-state inspector reservations

The later remembered-state inspector surface reads the same record
shape every profile / support / migration surface reads. This
document reserves:

- `stable_pane_id` — opaque, survives layout restore and window
  topology change. A stable-pane-id that changes across restore is a
  bug.
- `state_classes_exposed` — closed set of state-class ids the
  inspector may surface. Adding a class here is additive-minor.
- `export_action_available` — whether an Export action is offered
  from this inspector view.
- `clear_action_available` — whether a Clear / Reset action is
  offered. Clear actions MUST honour the row's authority class.
- `compare_action_available` — whether a Compare action against a
  portable profile or restore-provenance record is offered.

Rules (frozen):

1. A later persistence surface that mints a parallel pane-id or
   action set is non-conforming.
2. An inspector view whose `state_classes_exposed` includes
   `auth_session_secrets`, `long_lived_credentials`,
   `dirty_buffer_recovery_journal`, or `admin_policy_bundle` MUST
   render the row with metadata-only inspector posture. Raw secret
   bodies are never inspectable.

## 6. State map rows (Appendix-F-style)

The state map freezes the Appendix-F-style rows below. Every row is
expressed via one `state_map_row_record`; the columns align with the
schema's `state_map_row_record` fields.

| State class | Location root | Location path concept | Authority | Portability | Retention | Sync posture | Support export | Clear posture | Redaction |
|---|---|---|---|---|---|---|---|---|---|
| `user_global_settings` | `AURELINE_CONFIG` | `settings.jsonc` | `user_authored_durable_truth` | `portable` | `durable_until_user_deletes` | `synced_opt_in` | `included_metadata_only` | `clear_requires_preview_and_rollback` | `ui_string_only` |
| `keybindings` | `AURELINE_CONFIG` | `keybindings.jsonc` | `user_authored_durable_truth` | `portable` | `durable_until_user_deletes` | `synced_opt_in` | `included_by_default` | `clear_requires_preview_and_rollback` | `none` |
| `snippets` | `AURELINE_CONFIG` | `snippets/*` | `user_authored_durable_truth` | `portable` | `durable_until_user_deletes` | `synced_opt_in` | `opt_in_only` | `clear_requires_preview` | `ui_string_only` |
| `themes_and_design_tokens` | `AURELINE_CONFIG` | `themes/*` | `user_authored_durable_truth` | `portable` | `durable_until_user_deletes` | `synced_opt_in` | `included_metadata_only` | `clear_allowed` | `none` |
| `command_aliases` | `AURELINE_CONFIG` | `aliases.jsonc` | `user_authored_durable_truth` | `portable` | `durable_until_user_deletes` | `synced_opt_in` | `included_metadata_only` | `clear_requires_preview` | `none` |
| `ui_presets_and_layout_defaults` | `AURELINE_CONFIG` | `layout_presets/*` | `user_authored_durable_truth` | `portable` | `durable_until_user_deletes` | `synced_opt_in` | `included_metadata_only` | `clear_allowed` | `none` |
| `extension_selection_inventory` | `AURELINE_CONFIG` | `extensions.selected.jsonc` | `user_authored_durable_truth` | `portable` | `durable_until_user_deletes` | `synced_opt_in` | `included_by_default` | `clear_requires_preview_and_rollback` | `none` |
| `extension_recommendations` | `workspace_tree` | `.aureline/extensions.recommend.jsonc` | `user_authored_durable_truth` | `shared_workspace_only` | `durable_until_user_deletes` | `never_synced` | `included_by_default` | `clear_requires_preview` | `none` |
| `ai_preset_selection` | `AURELINE_CONFIG` | `ai/presets.jsonc` | `user_authored_durable_truth` | `portable` | `durable_until_user_deletes` | `synced_opt_in` | `included_metadata_only` | `clear_requires_preview` | `redact_value_preserve_shape` |
| `terminal_preferences` | `AURELINE_CONFIG` | `terminal/preferences.jsonc` | `user_authored_durable_truth` | `portable` | `durable_until_user_deletes` | `synced_opt_in` | `included_metadata_only` | `clear_allowed` | `none` |
| `workspace_manifest` | `workspace_tree` | `aureline.workspace.jsonc` | `user_authored_durable_truth` | `shared_workspace_only` | `durable_until_user_deletes` | `never_synced` | `included_by_default` | `clear_requires_preview_and_rollback` | `ui_string_only` |
| `workset_manifest` | `workspace_tree` | `.aureline/worksets/*.jsonc` | `user_authored_durable_truth` | `shared_workspace_only` | `durable_until_user_deletes` | `never_synced` | `included_by_default` | `clear_requires_preview` | `ui_string_only` |
| `tasks_and_launch_configs` | `workspace_tree` | `.aureline/tasks.jsonc`, `.aureline/launch.jsonc` | `user_authored_durable_truth` | `shared_workspace_only` | `durable_until_user_deletes` | `never_synced` | `included_by_default` | `clear_requires_preview_and_rollback` | `ui_string_only` |
| `machine_specific_settings` | `AURELINE_CONFIG` | `machine.settings.jsonc` | `user_authored_durable_truth` | `portable_with_machine_addendum` | `durable_until_user_deletes` | `never_synced` | `included_metadata_only` | `clear_requires_preview` | `redact_value_preserve_shape` |
| `profile_library_index` | `AURELINE_CONFIG` | `profiles/*.aureprofile.json` | `user_authored_durable_truth` | `portable` | `durable_until_user_deletes` | `synced_opt_in` | `included_metadata_only` | `clear_requires_preview` | `ui_string_only` |
| `sync_metadata` | `AURELINE_STATE` | `sync/*` | `user_owned_recovery_state` | `local_only` | `bounded_by_policy_or_quota` | `never_synced` | `included_metadata_only` | `clear_requires_preview` | `redact_to_class_label` |
| `conflict_journal` | `AURELINE_STATE` | `sync/conflict_journal/*` | `user_owned_recovery_state` | `local_only` | `bounded_by_policy_or_quota` | `never_synced` | `included_metadata_only` | `clear_requires_preview` | `redact_to_class_label` |
| `terminal_restore_metadata` | `AURELINE_STATE` | `terminal/restore/*` | `user_owned_recovery_state` | `local_only` | `bounded_by_policy_or_quota` | `never_synced` | `included_metadata_only` | `clear_requires_preview` | `redact_value_preserve_shape` |
| `policy_bundle_cache` | `AURELINE_STATE` | `policy/*.signed` | `admin_or_control_artifact` | `admin_owned` | `bounded_by_signed_bundle_epoch` | `admin_owned_sync_only` | `included_metadata_only` | `clear_denied_admin_owned` | `redact_to_class_label` |
| `execution_context_cache` | `AURELINE_STATE` | `contexts/*` | `disposable_derived_cache` | `local_only` | `reclaimable_at_any_time` | `never_synced` | `included_metadata_only` | `clear_allowed` | `redact_value_preserve_shape` |
| `extension_lockfile` | `workspace_tree` | `.aureline/extensions.lock.json` | `user_authored_durable_truth` | `shared_workspace_only` | `durable_until_user_deletes` | `never_synced` | `included_by_default` | `clear_requires_preview_and_rollback` | `none` |
| `ai_memory_metadata` | `AURELINE_STATE` | `ai/*` | `user_owned_recovery_state` | `local_only` | `bounded_by_policy_or_quota` | `never_synced` | `included_metadata_only` | `clear_requires_preview` | `redact_to_class_label` |
| `deferred_intent_outbox` | `AURELINE_STATE` | `outbox/*` | `user_owned_recovery_state` | `local_only` | `bounded_by_policy_or_quota` | `never_synced` | `included_metadata_only` | `clear_requires_preview` | `redact_to_class_label` |
| `local_history` | `AURELINE_STATE` | `history/*` | `user_owned_recovery_state` | `local_only` | `bounded_by_policy_or_quota` | `never_synced` | `opt_in_only` | `clear_requires_preview` | `redact_value_preserve_shape` |
| `logs_and_traces` | `AURELINE_STATE` | `logs/*` | `disposable_derived_cache` | `local_only` | `bounded_by_policy_or_quota` | `never_synced` | `included_metadata_only` | `clear_allowed` | `redact_to_class_label` |
| `object_store` | `AURELINE_STATE` | `objects/*` | `disposable_derived_cache` | `local_only` | `reclaimable_at_any_time` | `never_synced` | `excluded_by_default` | `clear_allowed` | `redact_to_class_label` |
| `index_cache` | `AURELINE_STATE` | `cache/*` | `disposable_derived_cache` | `local_only` | `reclaimable_at_any_time` | `never_synced` | `excluded_by_default` | `clear_allowed` | `redact_to_class_label` |
| `dirty_buffer_recovery_journal` | `AURELINE_STATE` | `history/recovery_journal/*` | `user_owned_recovery_state` | `local_only` | `bounded_by_policy_or_quota` | `never_synced` | `opt_in_only` | `clear_requires_preview` | `exclude_from_export` |
| `session_restore_state` | `AURELINE_STATE` | `session/*` | `user_owned_recovery_state` | `local_only` | `bounded_by_policy_or_quota` | `never_synced` | `included_metadata_only` | `clear_requires_preview` | `redact_value_preserve_shape` |
| `auth_session_secrets` | `os_credential_store` | `credential_handle/*` | `user_owned_recovery_state` | `excluded` | `durable_until_user_clears_with_preview` | `never_synced` | `excluded_always` | `clear_denied_platform_credential_store_only` | `exclude_from_export` |
| `admin_policy_bundle` | `AURELINE_POLICY` | `aureline.policy.signed` | `admin_or_control_artifact` | `admin_owned` | `bounded_by_signed_bundle_epoch` | `admin_owned_sync_only` | `included_metadata_only` | `clear_denied_admin_owned` | `redact_to_class_label` |
| `trust_approvals` | `AURELINE_STATE` | `trust/approvals/*` | `user_owned_recovery_state` | `excluded` | `durable_until_user_clears_with_preview` | `never_synced` | `excluded_by_default` | `clear_denied_live_authority` | `exclude_from_export` |
| `long_lived_credentials` | `os_credential_store` | `credential_handle/*` | `user_owned_recovery_state` | `excluded` | `durable_until_user_clears_with_preview` | `never_synced` | `excluded_always` | `clear_denied_platform_credential_store_only` | `exclude_from_export` |

Rules (frozen):

1. A surface that reads or writes to a row MUST quote the row's
   authority, portability, retention, sync posture, support-export
   posture, clear posture, and redaction verbatim. A row-specific
   "but in this surface it's different" treatment is non-conforming.
2. A new state class is additive-minor: add a `state_class_id`, add a
   row here, bump the schema version. Reclassifying an existing
   state class is breaking and requires a new decision row.
3. A later artifact-specific task (sync engine, mutation journal,
   policy bundle, AI outbox, local history, logs / traces) MUST
   resolve its storage location against the row above rather than
   minting a new path concept.

## 7. Export manifest rules

- A plain file-portable export (`file_portable_plain`) MUST list
  `auth_session_secrets`, `long_lived_credentials`, `trust_approvals`,
  `admin_policy_bundle`, `dirty_buffer_recovery_journal`, and
  `machine_specific_settings` (unless paired as an addendum) in
  `excluded_state_classes`, each with an explicit
  `exclusion_reason_id`.
- A managed-sync export (`managed_sync_opt_in`,
  `customer_managed_key_sync`, `self_hosted_sync`) MUST NOT include
  any state class whose row marks `sync_posture = never_synced`.
- A support / recovery export (`support_recovery_only`) MAY include
  redacted excerpts of `user_owned_recovery_state` under opt-in. It
  MUST NOT include raw auth secrets, long-lived credentials, or
  trust approvals. `redact_to_class_label` is the minimum redaction
  floor for `ai_memory_metadata`, `deferred_intent_outbox`, and
  `logs_and_traces` on any support export.
- Every export manifest carries an `integrity_digest_ref`. A
  manifest without a digest handle is non-conforming.

## 8. Restore-provenance rules

- `fidelity_label` is a producer claim; surfaces render their own
  `restore_level` based on the record plus the destination's
  compatibility posture. A surface that renders a higher
  `restore_level` than the record admits is non-conforming.
- `exact` MUST leave `equivalence_map_ref` and
  `rollback_checkpoint_ref` null; `compatible` and `manual_review`
  MUST populate both; `layout_only` MAY leave both null but MUST
  carry at least one `layout_restore_sanity` validator outcome.
- `post_restore_validator_outcomes` MUST contain at least one entry
  for `exact` (typically `settings_schema_migration = passed`),
  `compatible`, and `manual_review`. A `manual_review` record MUST
  list the unresolved validator classes.
- `compare_ref` and `export_ref` are reserved. A later persistence
  surface that mints a parallel compare / export handle is non-
  conforming.
- Raw workspace trust approvals, long-lived credentials, and auth
  session secrets MUST NOT appear inside a restore-provenance
  record under any fidelity label.

## 9. Reference rows

- PRD §12.4 and §12.4.1 — portable profile artifact rules.
- PRD §22.6.1 — signed policy bundle, offline entitlement, and
  admin-distribution lifecycle.
- TAD §21.10 — profile sync, snapshot, backup, and restore
  architecture.
- TAD Appendix F — configuration and state map.
- TAD Appendix BD — profile, settings sync, and conflict matrix.
- TAD Appendix BG — policy bundle, offline entitlement, and admin
  auditability matrix.
- ADR-0001 — identity modes and workspace trust.
- ADR-0003 — buffer, undo, large-file, and mutation-journal
  checkpoint model.
- ADR-0006 — VFS, save, cache identity, and root capability.
- ADR-0007 — secret broker, credential handle, trust store, and
  redaction.
- ADR-0008 — settings definition and effective-configuration
  resolver.
- `docs/workspace/entry_restore_object_model.md` — first-run, open,
  import, restore, and migration-result object model (source of the
  `restore_level` and `equivalence_map` vocabulary).

## 10. Linking from architecture and supportability materials

The state-map seed is linked from:

- [ADR-0007 redaction index](../adr/0007-secret-broker-credential-handle-trust-store-redaction.md)
  — the redaction classes this document re-exports.
- [ADR-0008 settings resolver](../adr/0008-settings-definition-and-effective-configuration-resolver.md)
  — the scope / authority vocabulary the user-authored state classes
  resolve against.
- [Entry / restore object model](../workspace/entry_restore_object_model.md)
  — the `restore_level`, `equivalence_map_ref`, and post-import /
  post-restore validator vocabulary.
- [Filesystem identity vocabulary](../filesystem/filesystem_identity_vocabulary.md)
  — the filesystem-identity layers every `workspace_tree` row
  resolves through.

Later migration tooling, support-export tooling, managed-sync
tooling, and remembered-state inspector tooling read this document
as the shared seed and do not retrofit a parallel dialect.
