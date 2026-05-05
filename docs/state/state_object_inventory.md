# State-object inventory, authority owner, schema-evolution path, and corruption-routing matrix

This document freezes one shared inventory of every persisted state
object Aureline manages, the authority that owns each object, the
schema-evolution path each object travels when producer and target
schema versions diverge, and the corruption-routing posture each object
takes when its on-disk body fails integrity validation. It exists so
clear-cache, repair, restore, downgrade, support-export, and reboot
flows resolve the same answer for the same object instead of each
surface inventing its own "is this safe to rebuild / repair / roll
back" story.

Persisted state is a governed product contract, not an accidental
by-product of implementation. Before any later persistence,
migration, clear-data, or Project-Doctor surface lands, the inventory
here declares — for every object the product holds on disk, in a
signed bundle, or in the OS credential store — **who authors it**,
**how the product is allowed to evolve its schema**, **whether a
backup is mandatory before a destructive migration runs**, and
**which corruption-routing posture governs the object when integrity
validation fails.**

This contract is normative. Where it disagrees with the PRD, TAD,
TDD, UI/UX spec, or Design-System style guide quotations cited in §10,
those documents win and this document MUST be updated in the same
change. Where this document disagrees with a downstream clear-cache,
migration, Project-Doctor, support-export, or repair surface minting a
parallel vocabulary, this document wins and the surface is
non-conforming.

## Companion artifacts

- [`/artifacts/state/state_objects.yaml`](../../artifacts/state/state_objects.yaml)
  — machine-readable state-object inventory with one row per object
  binding authority owner, location row, schema-evolution posture,
  backward / downgrade readability, backup-before-migrate rule, and
  corruption-routing posture.
- [`/artifacts/state/corruption_routing_matrix.yaml`](../../artifacts/state/corruption_routing_matrix.yaml)
  — machine-readable corruption-routing matrix that freezes the six
  corruption-posture classes and the decision-table rows every
  detector maps an object onto.
- [`/fixtures/state/migration_and_corruption_cases/`](../../fixtures/state/migration_and_corruption_cases/)
  — worked fixtures covering schema-evolution cases, backup-before-
  migrate enforcement, and each corruption-posture route.
- [`/docs/state/profile_and_state_map.md`](./profile_and_state_map.md)
  — portable-profile contract and Appendix-F-style state-map rows this
  inventory quotes rather than renames.
- [`/docs/state/config_and_state_path_map.md`](./config_and_state_path_map.md)
  and
  [`/artifacts/state/path_level_seed_map.yaml`](../../artifacts/state/path_level_seed_map.yaml)
  — path-level selectors for support, backup, clear-data,
  portability, low-disk, and policy tooling.
- [`/docs/state/migration_and_restore_playbook.md`](./migration_and_restore_playbook.md)
  — shared state migration / restore playbook and the fidelity-label
  vocabulary this inventory re-exports.
- [`/docs/state/durable_state_compatibility_contract.md`](./durable_state_compatibility_contract.md)
  — durable-state compatibility-window matrix and restore-after-
  downgrade packet that re-export this inventory's authority owner,
  schema-evolution posture, backup-before-migrate rule, and
  corruption posture verbatim into the closed cross-surface family
  rows (`user_authored_durable_state`,
  `workspace_authored_durable_state`, `cache_or_index_state`,
  `public_schemas_or_interfaces`, `portable_state_packages`,
  `generated_or_structured_artifacts`) Help, Support Center,
  Migration Center, and the shiproom dashboard resolve against.
- [`/docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md)
  and
  [`/artifacts/runtime/storage_classes.yaml`](../../artifacts/runtime/storage_classes.yaml)
  — storage-class vocabulary, pin-source vocabulary, and clear-cache
  protection classes this inventory re-exports for disposable state.
- [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md)
  — recovery-ladder rung ids the corruption-routing matrix's
  `repair_flow` posture binds to.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  — support-bundle contract this inventory names for evidence /
  support state.
- [`/schemas/state/restore_provenance.schema.json`](../../schemas/state/restore_provenance.schema.json)
  — restore-provenance record whose preserved-prior-artifact refs the
  backup-before-migrate rule populates.

## Why freeze this now

Each later persistence surface — sync engine, migration executor,
policy-bundle distribution, clear-data review, Project-Doctor,
support-bundle exporter, benchmark council, repair flow — otherwise
re-invents "what kind of state is this, who owns it, can I rebuild or
delete it, and how do I evolve its schema without losing user work?"
Without the inventory, clear-cache accidentally drops the recovery
journal, a migration rewrites a user-authored body without a backup,
or a detector quarantines a disposable cache as if it were durable
user truth.

The state-object inventory pins one authority owner, one schema-
evolution posture, one backup-before-migrate rule, and one corruption-
routing posture per object, so later surfaces compose over the same
row rather than re-classifying state ad hoc.

## Scope

- Enumerate every persisted state object the M0 product contract
  holds: user-authored durable state, workspace-authored durable
  state, derived disposable state, recovery journals, audit / trust /
  security state, and evidence / support state.
- Freeze one `state_object_inventory_row` shape that every row in
  `state_objects.yaml` validates against and every later persistence
  surface reads.
- Freeze the six-value `corruption_posture_class` vocabulary —
  `block_feature_only`, `rebuild_automatically`, `open_with_warning`,
  `repair_flow`, `backup_rollback`, `fail_closed_for_privileged_operations`
  — and pin exactly one posture per object.
- Freeze the schema-evolution-posture vocabulary — `frozen_no_evolution`,
  `additive_minor_only`, `migrating_with_equivalence_map`,
  `replay_only_no_schema`, `signed_epoch_replacement`,
  `content_addressed_immutable` — and pin exactly one posture per
  object.
- Freeze the backward / downgrade-readability vocabulary —
  `downgrade_reads_fully`, `downgrade_reads_with_fallback`,
  `downgrade_requires_export_only`, `downgrade_refused` — so a
  target-version mismatch resolves through one shared label.
- Pin the backup-before-migrate rule: destructive migrations of
  user-authored durable state MUST preserve the pre-migration body by
  opaque ref before writing the translated result, and the preserved
  ref MUST flow into the restore-provenance record.
- Link each row to a downstream consumer surface — clear-data review,
  Project Doctor, repair flow, migration center, support bundle,
  benchmark council, restore surface, signed-bundle distribution — so
  later UI does not mint a parallel selector.

## Out of scope

- Implementing every migration flow or corruption repair. The
  inventory and matrix freeze the contract; the engines that execute
  it land in later milestones.
- The concrete on-disk encoding, byte layout, or encryption envelope
  for any specific object. Those live with each artifact's dedicated
  contract.
- Final product copy for corruption banners, repair-flow labels, or
  downgrade dialogs. Copy lives with the shell interaction-safety
  contract; this document pins the closed sets the copy resolves
  against.
- The full Project-Doctor finding catalogue. The matrix reserves the
  repair-rung binding; the finding catalogue is a later packet.

## 1. Authority-owner vocabulary

Every state object in the inventory resolves to exactly one authority
owner. The set re-exports the closed vocabulary frozen in
[`/docs/state/profile_and_state_map.md`](./profile_and_state_map.md)
§1:

- `user_authored_durable_truth` — the user typed it or accepted it
  through a preview. Losing it requires an explicit delete.
- `user_owned_recovery_state` — the product held it on the user's
  behalf so a crash or restart does not lose work. Local, but
  user-owned; clear requires preview.
- `admin_or_control_artifact` — signed bundle, policy cache, offline
  entitlement, or trust-root update authored by admin / control
  authority. The user may inspect a decision trace but cannot edit or
  clear the payload.
- `disposable_derived_cache` — fully regenerable from other truth.
  Clear is allowed at any time.

Rules (frozen):

1. Every inventory row MUST name exactly one authority owner.
2. A surface that silently reclassifies an object (for example,
   treating a recovery journal as a disposable cache so that a
   blanket clear path can target it) is non-conforming.
3. A row MAY name a secondary authority hint under
   `secondary_authority_hint` when the artifact is handed off between
   authorities (for example, a trust approval recorded against a
   signed policy-bundle epoch). The primary owner still drives clear
   and corruption posture.

## 2. Object category vocabulary

Every inventory row MUST declare exactly one `object_category`. The
set is closed:

- `user_authored_durable_state` — what the user typed, accepted, or
  configured. Primary content of a portable profile. Examples: user
  settings, keybindings, snippets, themes, command aliases, AI preset
  selection, terminal preferences, extension selection, machine
  settings.
- `workspace_authored_durable_state` — what the workspace declares as
  shared truth. Examples: workspace manifest, workset manifests,
  tasks and launch configs, extension recommendations, extension
  lockfile.
- `derived_disposable_state` — regenerable from other truth. Examples:
  index cache, object store, execution-context cache, render atlases,
  logs / traces past retention.
- `recovery_journal_state` — held on the user's behalf to survive a
  crash or restart. Examples: dirty-buffer recovery journal,
  session-restore state, local history, deferred-intent outbox, AI
  memory metadata, terminal restore metadata, sync metadata, conflict
  journal.
- `audit_trust_security_state` — signed admin artifacts, trust
  approvals, secrets, audit trails. Examples: admin policy bundle,
  policy-bundle cache, trust approvals, auth session secrets,
  long-lived credentials, admin audit log.
- `evidence_support_state` — policy-bounded evidence and support
  artifacts. Examples: support bundles, crash envelopes, review
  packets, incident bundles, benchmark results, release-evidence
  packets.

Rules (frozen):

1. A state object MUST resolve to exactly one category.
2. Adding a new category is breaking and requires a new decision row.
3. Adding a new object to an existing category is additive-minor and
   MUST bump `schema_version` on `state_objects.yaml`.

## 3. Schema-evolution-posture vocabulary

Every inventory row MUST declare exactly one schema-evolution posture.
The set is closed:

- `frozen_no_evolution` — the schema is frozen at M0; evolution
  requires a new decision row and a schema-version bump of the
  containing artifact. Used for narrow, stable shapes where churn
  would threaten cross-surface compatibility.
- `additive_minor_only` — fields may be added (minor-additive) but
  not removed, repurposed, or narrowed. Removing or narrowing is
  breaking and requires a migration path.
- `migrating_with_equivalence_map` — schema meaning can change across
  versions, but every breaking translation MUST be declared in an
  equivalence map and carry a rollback checkpoint. The producer
  preserves the pre-translation body for compare / export before
  apply.
- `replay_only_no_schema` — the body is not a schema payload; it is a
  replay log (mutation journal, recovery journal, outbox). Evolution
  happens by replaying into a new-format buffer, not by translating
  stored records.
- `signed_epoch_replacement` — the authority issues a new signed
  epoch; migration is "accept new epoch or keep last-known-good." No
  in-place translation of the old epoch's body.
- `content_addressed_immutable` — the body is identified by its
  digest; evolution is additive only (a new digest supersedes the
  old, the old stays reachable by ref). Used for object-store blobs,
  evidence bundles, benchmark results.

Rules (frozen):

1. Every row MUST name exactly one schema-evolution posture.
2. `frozen_no_evolution` and `content_addressed_immutable` forbid
   in-place rewrites; any surface that mutates the body under these
   postures is non-conforming.
3. `migrating_with_equivalence_map` REQUIRES the row to populate
   `backup_before_migrate_rule = backup_required_before_destructive_migration`.
4. `replay_only_no_schema` rows MAY downgrade to `open_with_warning`
   when replay fails on a corrupted frame; they MUST NOT claim
   `rebuild_automatically` against authority they do not own.

## 4. Backward / downgrade-readability vocabulary

Every inventory row MUST declare exactly one
`backward_downgrade_readability` label describing how an older target
build reads a newer producer's body. The set is closed:

- `downgrade_reads_fully` — older targets read the newer body with
  no loss of meaning. Reserved for strictly additive, opaque-to-older
  fields that carry default semantics when absent.
- `downgrade_reads_with_fallback` — older targets read the newer body
  but fall back to a documented default for fields they do not
  understand. The target emits a compatibility note.
- `downgrade_requires_export_only` — the newer body is not readable
  on the older target; the only supported path is to export the
  artifact on the newer target and hand the export to the older
  target through the declared import route.
- `downgrade_refused` — the authority refuses older targets outright.
  Used for signed admin artifacts whose epoch the older target cannot
  validate.

Rules (frozen):

1. Every row MUST name exactly one readability label.
2. `downgrade_refused` is reserved for `admin_or_control_artifact`
   rows whose epoch signatures bind the admissible target range.
3. `downgrade_reads_fully` is not allowed on
   `migrating_with_equivalence_map` rows; the translation path makes
   full downgrade unsafe by construction.

## 5. Backup-before-migrate rule vocabulary

Every inventory row MUST declare exactly one backup-before-migrate
rule. The set is closed:

- `backup_required_before_destructive_migration` — a destructive
  migration (in-place rewrite, schema-meaning change, translation
  through an equivalence map) MUST preserve the pre-migration body
  by opaque ref before writing the translated result. The preserved
  ref flows into the restore-provenance record.
- `backup_optional_user_offered` — the object is replayable or
  trivially rebuildable; the migration MAY offer an opt-in snapshot
  but is not required to preserve one.
- `backup_not_applicable_content_addressed` — content-addressed
  immutable bodies supersede by new digest; the old digest remains
  reachable by ref, so a separate backup is redundant.
- `backup_not_applicable_disposable` — the object is disposable
  derived state; any "migration" is a rebuild from authoritative
  truth.
- `backup_handled_by_authority` — the admin / control authority holds
  the backup posture; the client does not create local copies of
  signed bundles or trust roots.

Rules (frozen):

1. Every row MUST name exactly one backup-before-migrate rule.
2. Every row whose `object_category` is `user_authored_durable_state`
   or `workspace_authored_durable_state` with
   `schema_evolution_posture = migrating_with_equivalence_map` MUST
   carry `backup_required_before_destructive_migration`.
3. `backup_not_applicable_disposable` is only admissible for
   `derived_disposable_state` rows.
4. `backup_not_applicable_content_addressed` is only admissible for
   `content_addressed_immutable` rows.
5. A migration that rewrites a `backup_required_before_destructive_migration`
   body without preserving the prior artifact ref is non-conforming.

## 6. Corruption-posture vocabulary

Every inventory row MUST declare exactly one corruption posture. The
posture routes what a detector does when the object's integrity
validation fails. The set is closed:

- `block_feature_only` — the feature whose surface depends on the
  object is disabled or degraded; the rest of the product continues.
  The surface renders a typed unavailability note. Used when the
  object is not required for editor correctness (for example, a
  stale workspace-truth projection cache, a terminal preference
  overlay).
- `rebuild_automatically` — the detector discards the corrupted body
  and rebuilds from authoritative truth. Admissible only when the
  authority for rebuild lies elsewhere (a disposable cache, a
  content-addressed derived blob with known-good producer). Never
  admissible for `user_authored_durable_truth` or
  `user_owned_recovery_state`.
- `open_with_warning` — the product opens against the object with a
  warning banner; the user may proceed with degraded fidelity but
  cannot silently overwrite the corrupted body. Used for partially
  replayable recovery journals and for layout snapshots.
- `repair_flow` — the product routes the user through an explicit
  repair flow (recovery-ladder rung, Project Doctor finding). The
  user sees what was preserved, what is lost, and what a repair run
  will do before it runs.
- `backup_rollback` — the detector refuses the corrupted body and
  restores the preserved prior artifact named by the
  backup-before-migrate ref. Admissible only for rows carrying
  `backup_required_before_destructive_migration`.
- `fail_closed_for_privileged_operations` — privileged operations
  (signing, apply-policy, managed-sync publish, release cut,
  admin-console apply) refuse to run until the object's integrity
  is re-established. Non-privileged editor operations continue under
  `open_with_warning` semantics. Used for admin policy bundles,
  trust approvals, policy-bundle cache, and release-evidence packs.

Rules (frozen):

1. Every row MUST declare exactly one corruption posture.
2. A row whose authority is `user_authored_durable_truth` or
   `user_owned_recovery_state` MUST NOT declare
   `rebuild_automatically`. Rebuilding user truth is not the
   product's call.
3. A row whose authority is `admin_or_control_artifact` MUST declare
   `fail_closed_for_privileged_operations` or `backup_rollback`; a
   silent rebuild of an admin artifact is non-conforming.
4. `repair_flow` rows MUST name a `repair_rung_ref` binding the row
   to a recovery-ladder rung id.
5. `backup_rollback` rows MUST carry
   `backup_before_migrate_rule = backup_required_before_destructive_migration`.

## 7. Consumer-surface reservation

Every inventory row MUST name at least one downstream consumer surface
the row projects to. The set is closed:

- `clear_data_review` — clear-cache / clear-data review sheet.
- `project_doctor` — Project Doctor findings surface.
- `repair_flow` — explicit repair-flow affordance (recovery-ladder
  rung entry).
- `migration_center` — migration-session preview / apply surface.
- `support_bundle` — support-bundle exporter.
- `benchmark_council` — benchmark governance packet.
- `restore_surface` — entry / restore result surface.
- `signed_bundle_distribution` — signed-bundle distribution path
  (managed sync, admin distribution, policy-epoch refresh).

Rules (frozen):

1. Every row MUST declare `consumer_surfaces` with length >= 1. A
   row with no consumer is non-conforming.
2. Every row whose `corruption_posture` is `repair_flow` MUST include
   `repair_flow` in `consumer_surfaces` and bind a `repair_rung_ref`.
3. Every row whose `corruption_posture` is
   `fail_closed_for_privileged_operations` MUST include
   `signed_bundle_distribution` or `project_doctor` in
   `consumer_surfaces`.

## 8. Inventory rows (overview)

The table below is the reviewer-facing overview. The machine-readable
row set, including full field coverage, lives in
[`/artifacts/state/state_objects.yaml`](../../artifacts/state/state_objects.yaml)
and is the source of truth for every later surface.

| State object | Category | Authority | Schema evolution | Downgrade | Backup-before-migrate | Corruption posture |
|---|---|---|---|---|---|---|
| `user_global_settings` | user-authored durable | `user_authored_durable_truth` | `migrating_with_equivalence_map` | `downgrade_reads_with_fallback` | `backup_required_before_destructive_migration` | `backup_rollback` |
| `keybindings` | user-authored durable | `user_authored_durable_truth` | `migrating_with_equivalence_map` | `downgrade_reads_with_fallback` | `backup_required_before_destructive_migration` | `backup_rollback` |
| `snippets` | user-authored durable | `user_authored_durable_truth` | `additive_minor_only` | `downgrade_reads_fully` | `backup_required_before_destructive_migration` | `open_with_warning` |
| `themes_and_design_tokens` | user-authored durable | `user_authored_durable_truth` | `migrating_with_equivalence_map` | `downgrade_reads_with_fallback` | `backup_required_before_destructive_migration` | `backup_rollback` |
| `command_aliases` | user-authored durable | `user_authored_durable_truth` | `additive_minor_only` | `downgrade_reads_fully` | `backup_required_before_destructive_migration` | `open_with_warning` |
| `ui_presets_and_layout_defaults` | user-authored durable | `user_authored_durable_truth` | `additive_minor_only` | `downgrade_reads_fully` | `backup_required_before_destructive_migration` | `open_with_warning` |
| `extension_selection_inventory` | user-authored durable | `user_authored_durable_truth` | `migrating_with_equivalence_map` | `downgrade_reads_with_fallback` | `backup_required_before_destructive_migration` | `backup_rollback` |
| `ai_preset_selection` | user-authored durable | `user_authored_durable_truth` | `additive_minor_only` | `downgrade_reads_with_fallback` | `backup_required_before_destructive_migration` | `open_with_warning` |
| `terminal_preferences` | user-authored durable | `user_authored_durable_truth` | `additive_minor_only` | `downgrade_reads_fully` | `backup_required_before_destructive_migration` | `open_with_warning` |
| `machine_specific_settings` | user-authored durable | `user_authored_durable_truth` | `additive_minor_only` | `downgrade_reads_fully` | `backup_required_before_destructive_migration` | `open_with_warning` |
| `profile_library_index` | user-authored durable | `user_authored_durable_truth` | `additive_minor_only` | `downgrade_reads_fully` | `backup_required_before_destructive_migration` | `block_feature_only` |
| `workspace_manifest` | workspace-authored durable | `user_authored_durable_truth` | `migrating_with_equivalence_map` | `downgrade_reads_with_fallback` | `backup_required_before_destructive_migration` | `repair_flow` |
| `workset_manifest` | workspace-authored durable | `user_authored_durable_truth` | `additive_minor_only` | `downgrade_reads_fully` | `backup_required_before_destructive_migration` | `repair_flow` |
| `tasks_and_launch_configs` | workspace-authored durable | `user_authored_durable_truth` | `additive_minor_only` | `downgrade_reads_with_fallback` | `backup_required_before_destructive_migration` | `open_with_warning` |
| `extension_recommendations` | workspace-authored durable | `user_authored_durable_truth` | `additive_minor_only` | `downgrade_reads_fully` | `backup_optional_user_offered` | `block_feature_only` |
| `extension_lockfile` | workspace-authored durable | `user_authored_durable_truth` | `additive_minor_only` | `downgrade_reads_with_fallback` | `backup_required_before_destructive_migration` | `repair_flow` |
| `index_cache` | derived disposable | `disposable_derived_cache` | `frozen_no_evolution` | `downgrade_reads_fully` | `backup_not_applicable_disposable` | `rebuild_automatically` |
| `object_store` | derived disposable | `disposable_derived_cache` | `content_addressed_immutable` | `downgrade_reads_fully` | `backup_not_applicable_content_addressed` | `rebuild_automatically` |
| `execution_context_cache` | derived disposable | `disposable_derived_cache` | `frozen_no_evolution` | `downgrade_reads_fully` | `backup_not_applicable_disposable` | `rebuild_automatically` |
| `logs_and_traces` | derived disposable | `disposable_derived_cache` | `additive_minor_only` | `downgrade_reads_fully` | `backup_not_applicable_disposable` | `rebuild_automatically` |
| `interactive_hot_cache` | derived disposable | `disposable_derived_cache` | `frozen_no_evolution` | `downgrade_reads_fully` | `backup_not_applicable_disposable` | `rebuild_automatically` |
| `knowledge_cache` | derived disposable | `disposable_derived_cache` | `frozen_no_evolution` | `downgrade_reads_fully` | `backup_not_applicable_disposable` | `rebuild_automatically` |
| `prebuild_environment_cache` | derived disposable | `disposable_derived_cache` | `content_addressed_immutable` | `downgrade_reads_fully` | `backup_not_applicable_content_addressed` | `rebuild_automatically` |
| `dirty_buffer_recovery_journal` | recovery journal | `user_owned_recovery_state` | `replay_only_no_schema` | `downgrade_reads_with_fallback` | `backup_optional_user_offered` | `open_with_warning` |
| `session_restore_state` | recovery journal | `user_owned_recovery_state` | `additive_minor_only` | `downgrade_reads_with_fallback` | `backup_optional_user_offered` | `open_with_warning` |
| `local_history` | recovery journal | `user_owned_recovery_state` | `replay_only_no_schema` | `downgrade_reads_with_fallback` | `backup_optional_user_offered` | `open_with_warning` |
| `deferred_intent_outbox` | recovery journal | `user_owned_recovery_state` | `replay_only_no_schema` | `downgrade_reads_with_fallback` | `backup_optional_user_offered` | `repair_flow` |
| `ai_memory_metadata` | recovery journal | `user_owned_recovery_state` | `additive_minor_only` | `downgrade_reads_with_fallback` | `backup_optional_user_offered` | `open_with_warning` |
| `terminal_restore_metadata` | recovery journal | `user_owned_recovery_state` | `additive_minor_only` | `downgrade_reads_with_fallback` | `backup_optional_user_offered` | `open_with_warning` |
| `sync_metadata` | recovery journal | `user_owned_recovery_state` | `additive_minor_only` | `downgrade_reads_with_fallback` | `backup_optional_user_offered` | `repair_flow` |
| `conflict_journal` | recovery journal | `user_owned_recovery_state` | `replay_only_no_schema` | `downgrade_reads_with_fallback` | `backup_optional_user_offered` | `repair_flow` |
| `admin_policy_bundle` | audit / trust / security | `admin_or_control_artifact` | `signed_epoch_replacement` | `downgrade_refused` | `backup_handled_by_authority` | `fail_closed_for_privileged_operations` |
| `policy_bundle_cache` | audit / trust / security | `admin_or_control_artifact` | `signed_epoch_replacement` | `downgrade_refused` | `backup_handled_by_authority` | `fail_closed_for_privileged_operations` |
| `trust_approvals` | audit / trust / security | `user_owned_recovery_state` | `additive_minor_only` | `downgrade_reads_with_fallback` | `backup_required_before_destructive_migration` | `fail_closed_for_privileged_operations` |
| `auth_session_secrets` | audit / trust / security | `user_owned_recovery_state` | `frozen_no_evolution` | `downgrade_reads_fully` | `backup_handled_by_authority` | `fail_closed_for_privileged_operations` |
| `long_lived_credentials` | audit / trust / security | `user_owned_recovery_state` | `frozen_no_evolution` | `downgrade_reads_fully` | `backup_handled_by_authority` | `fail_closed_for_privileged_operations` |
| `admin_audit_log` | audit / trust / security | `admin_or_control_artifact` | `content_addressed_immutable` | `downgrade_reads_fully` | `backup_handled_by_authority` | `fail_closed_for_privileged_operations` |
| `support_bundles` | evidence / support | `user_owned_recovery_state` | `content_addressed_immutable` | `downgrade_reads_fully` | `backup_not_applicable_content_addressed` | `block_feature_only` |
| `crash_envelopes` | evidence / support | `user_owned_recovery_state` | `content_addressed_immutable` | `downgrade_reads_fully` | `backup_not_applicable_content_addressed` | `block_feature_only` |
| `review_packets` | evidence / support | `admin_or_control_artifact` | `content_addressed_immutable` | `downgrade_reads_fully` | `backup_handled_by_authority` | `fail_closed_for_privileged_operations` |
| `benchmark_results` | evidence / support | `admin_or_control_artifact` | `content_addressed_immutable` | `downgrade_reads_fully` | `backup_handled_by_authority` | `fail_closed_for_privileged_operations` |
| `release_evidence_packs` | evidence / support | `admin_or_control_artifact` | `content_addressed_immutable` | `downgrade_reads_fully` | `backup_handled_by_authority` | `fail_closed_for_privileged_operations` |
| `incident_bundles` | evidence / support | `admin_or_control_artifact` | `content_addressed_immutable` | `downgrade_reads_fully` | `backup_handled_by_authority` | `fail_closed_for_privileged_operations` |

Rules (frozen):

1. A surface that reads or writes an object MUST quote the object's
   authority, category, schema-evolution posture, downgrade
   readability, backup-before-migrate rule, and corruption posture
   verbatim from this inventory.
2. Every acceptance-listed M0 state artifact (layout, profile,
   settings, journals, caches, policy bundles, evidence packs,
   support bundles, benchmark results) is covered by at least one row
   above. The machine artifact is authoritative; this table is the
   reviewer projection.
3. A new state object is additive-minor and MUST bump
   `schema_version` on `state_objects.yaml`. Reclassifying an
   existing object — swapping authority, category, schema-evolution
   posture, or corruption posture — is breaking and requires a new
   decision row.

## 9. Corruption-routing matrix (overview)

The corruption-routing matrix pins the decision rule for each posture
class. The reviewer-facing table below is the summary; the
machine-readable matrix lives at
[`/artifacts/state/corruption_routing_matrix.yaml`](../../artifacts/state/corruption_routing_matrix.yaml).

| Corruption posture | Detector action | Authority classes it may govern | Required consumer | Forbidden shortcut |
|---|---|---|---|---|
| `block_feature_only` | disable or degrade the dependent feature; render typed unavailability; continue other work | any | `project_doctor` or `repair_flow` surface with a follow-up action | silently ignoring the fault |
| `rebuild_automatically` | discard the corrupted body and rebuild from authoritative truth | `disposable_derived_cache` only | `clear_data_review` (cleanup lane) | rebuilding `user_*` authority |
| `open_with_warning` | open with a warning banner at degraded fidelity; no silent overwrite | `user_authored_durable_truth`, `user_owned_recovery_state`, `disposable_derived_cache` | `restore_surface` or `project_doctor` | claiming `exact` fidelity |
| `repair_flow` | route to an explicit repair-flow rung; show preserved, lost, and repair-effect | `user_authored_durable_truth`, `user_owned_recovery_state` | `repair_flow` with a `repair_rung_ref` | running repair without preview |
| `backup_rollback` | refuse corrupted body; restore preserved prior artifact by ref | authority classes with `backup_required_before_destructive_migration` | `restore_surface` (compare/export) | rewriting over the corrupted body |
| `fail_closed_for_privileged_operations` | refuse privileged ops (sign, apply policy, sync publish, release cut) until integrity re-established; non-privileged editing continues with warning | `admin_or_control_artifact`, or `user_owned_recovery_state` acting as trust / secret artifact | `signed_bundle_distribution` or `project_doctor` | allowing a privileged op to proceed |

Rules (frozen):

1. A detector that selects a posture not listed above is
   non-conforming.
2. A row that projects to two postures for the same detector event is
   non-conforming; ambiguity resolves through the inventory row's
   declared posture.
3. A posture that crosses an authority-boundary rule (for example,
   `rebuild_automatically` on user truth) is non-conforming and the
   schema / artifact validator rejects the row.

## 10. Reference rows

- PRD §10.15 (diagnostics), §10.22 (support export), §10.23 (recovery
  ladder), §12.4 (portable profile), §12.4.1 (portable profile
  artifact rules), and §22.6.1 (signed policy bundle).
- TAD §8.10 (fault domain and supervisor), §21.10 (profile sync,
  snapshot, backup, restore), §24.2.2 (recovery rungs), §24.2.3
  (checkpoint and reversal), §24.4 (repair preview), Appendix F
  (configuration and state map), Appendix BD (profile, settings sync,
  and conflict matrix), and Appendix BG (policy bundle, offline
  entitlement, and admin auditability matrix).
- TDD §9 (persistence stores), §11 (recovery and journals), §14
  (support / evidence bundles).
- UI / UX §22.20 (Support Center), §23.26 (Doctor surface), §24
  (Clear data review).
- ADR-0001 (identity modes and workspace trust), ADR-0003 (buffer,
  undo, large-file, and mutation-journal checkpoint model), ADR-0006
  (VFS, save, cache identity, and root capability), ADR-0007 (secret
  broker, credential handle, trust store, and redaction), ADR-0008
  (settings definition and effective-configuration resolver).
- `docs/state/profile_and_state_map.md` §1 (authority vocabulary) and
  §6 (Appendix-F-style rows) — re-exported by this inventory.
- `docs/state/migration_and_restore_playbook.md` §1–§5 — re-exported
  fidelity, downgrade, and preserved-prior-artifact vocabulary.
- `docs/runtime/storage_classes_and_gc.md` and
  `artifacts/runtime/storage_classes.yaml` — clear-cache and low-disk
  protection classes this inventory quotes for disposable state.
- `docs/support/recovery_ladder_packet.md` — recovery rung ids bound
  by `repair_flow` corruption posture.
- `docs/support/support_bundle_contract.md` — support-bundle contract
  for evidence / support rows.

## 11. Linking from architecture and supportability materials

The state-object inventory and corruption-routing matrix are linked
from:

- Portable-profile and Appendix-F-style state map — the inventory
  rows project through the same location / authority / redaction
  vocabulary.
- Migration and restore playbook — backup-before-migrate rule rows
  populate preserved-prior-artifact refs in the restore-provenance
  record.
- Storage-classes and GC contract — the inventory pins the
  corruption posture for every disposable-cache class that contract
  enumerates.
- Recovery-ladder packet — `repair_flow` rows bind to the packet's
  rung ids.
- Support-bundle contract — evidence / support rows pin how a
  support exporter quotes the authority, schema, and corruption
  posture of each included artifact.
- Signed-bundle and admin-audit contracts — audit / trust / security
  rows pin `fail_closed_for_privileged_operations` as the routing
  for every privileged operation that reads admin-authored state.

Later clear-cache, repair, restore, migration, support-export, and
signed-bundle distribution tooling reads this document as the shared
seed and does not retrofit a parallel inventory or matrix.
