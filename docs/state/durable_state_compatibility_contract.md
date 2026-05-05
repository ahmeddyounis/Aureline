# Durable-state compatibility window, backup-before-migrate matrix, and restore-after-downgrade packet contract

This document freezes one matrix that answers four questions for every
stable-bearing durable-state artifact family the product carries:

- **how far back can an older target read a newer producer's body** —
  the backward-readable window;
- **what does a forward (newer target reading older body) reader
  promise** — the forward-read expectations;
- **what backup is mandatory before a destructive migration runs** —
  the backup-before-migrate rule;
- **what does the surface do when the target sits below the producer**
  — the rollback or downgrade behavior, the corruption or repair
  path, and the migration-policy owner.

The matrix is the **single source of truth** Help, Support Center,
Migration Center, and the shiproom dashboard panel resolve their
per-family answers against. It re-exports the upstream
state-object inventory, migration-and-restore playbook, and
portable-state package contracts verbatim so a stable label cannot
quietly imply something the per-object rows do not promise.

The machine-readable schemas live at:

- [`/schemas/state/compatibility_window_row.schema.json`](../../schemas/state/compatibility_window_row.schema.json)
- [`/schemas/state/restore_after_downgrade_packet.schema.json`](../../schemas/state/restore_after_downgrade_packet.schema.json)

Worked fixtures live under:

- [`/fixtures/state/durable_state_cases/`](../../fixtures/state/durable_state_cases/)

This contract composes with:

- [`/docs/state/state_object_inventory.md`](./state_object_inventory.md)
- [`/artifacts/state/state_objects.yaml`](../../artifacts/state/state_objects.yaml)
- [`/docs/state/migration_and_restore_playbook.md`](./migration_and_restore_playbook.md)
- [`/docs/state/portable_state_package_contract.md`](./portable_state_package_contract.md)
- [`/docs/state/profile_and_state_map.md`](./profile_and_state_map.md)
- [`/docs/state/restore_artifact_family_contract.md`](./restore_artifact_family_contract.md)

This contract is normative. Where it disagrees with the PRD, TAD,
TDD, UI/UX spec, the state-object inventory, the migration-and-restore
playbook, or the portable-state package contract, those documents win
and this document plus the matrix and packet schemas MUST be updated
in the same change. Where a downstream Help, Support Center, Migration
Center, shiproom, Project Doctor, or release-evidence surface mints a
parallel backward-window, downgrade-behavior, or unsupported-state
vocabulary, this contract wins and the surface is non-conforming.

## Why freeze this now

Stable durable-state promises drift the moment each later flow tells
its own story:

- a settings migrator says "compatible" but rewrites the body without
  preserving the pre-translation artifact;
- a workspace-task migrator silently drops fields the new schema
  removed because no row admits "translated, not preserved";
- a portable-state package downgrade lands on an older target whose
  build cannot read the manifest, and the destination guesses a posture
  rather than rendering inspect-only;
- a generated-structure surface (object-store derivation, render
  atlas, structured artifact view) falls back to compare-only or
  read-only mode without naming the fallback class;
- shiproom green-lights a stable-channel widening because the
  release-evidence pack quotes a "compatible" claim that no row in the
  state-object inventory backs.

The matrix forecloses these patterns by binding each artifact family
to one closed backward-window class, one closed forward-read class,
one backup rule, one rollback / downgrade class, one corruption /
repair path, one migration-policy owner, and the closed surfaces it
feeds. Stable-bearing artifacts can no longer imply policy that has
no row.

## Scope

- Freeze one `compatibility_window_row_record` per artifact family
  covering: user-authored durable state, workspace-authored durable
  state, cache or index state, public schemas or interfaces,
  portable-state packages, and generated or structured artifacts.
- Freeze the closed `artifact_family_class`,
  `backward_readable_window_class`,
  `forward_read_expectations_class`,
  `rollback_or_downgrade_behavior_class`,
  `corruption_or_repair_path_class`,
  `migration_policy_owner_class`, and `consumer_surface_class`
  vocabularies.
- Re-export the state-object inventory's `schema_evolution_posture`,
  `backup_before_migrate_rule_class`, and
  `downgrade_readability_class` vocabularies so a row cannot disagree
  with the per-object inventory.
- Freeze one `compatibility_window_matrix_record` aggregate carrying
  every row, with at least one row per artifact family.
- Freeze one `restore_after_downgrade_packet_record` shape carrying
  original artifact identity, backup path, migrated state, downgraded
  target, the closed preserved-versus-lost field rows, the closed
  next-safe-action class, and the closed unsupported-state class.
- Freeze the `next_safe_action_class`, `field_status_class`,
  `field_class_class`, `unsupported_state_class`, and downgrade-trigger
  vocabularies the packet uses.
- Provide worked YAML fixtures for settings/profile migration,
  workspace task or launch artifact migration, portable-state package
  downgrade, and generated-structure fallback to compare-only or
  read-only mode.

## Out of scope

- Implementing migrators. The contract freezes the vocabulary and
  acceptance rules; the engines that execute migration land in later
  milestones.
- Performing mass conversion of historical artifacts. The matrix
  governs how migrations behave when they do run; bulk historical
  conversion is a separate operational packet.
- Final UI copy. Display copy may render `Backward window stable
  indefinite`, `Compatible translation with backup`, `Inspect only`,
  `Read-only mode`, and so on; the closed machine vocabularies are
  fixed.

## 1. Compatibility-window row shape

Every row MUST emit exactly one
`compatibility_window_row_record`. The record carries:

| Field | Meaning |
|---|---|
| `row_id`, `row_label` | identity and reviewable label; opaque to this schema |
| `artifact_family_class` | one of six closed families (see §2) |
| `schema_evolution_posture` | re-exported from the state-object inventory |
| `backward_readable_window` | closed window class describing how far back an older target reads a newer producer's body |
| `forward_read_expectations` | closed class describing what a newer target promises when reading an older body |
| `backup_before_migrate_rule` | re-exported from the state-object inventory |
| `rollback_or_downgrade_behavior` | closed behavior class for "target sits below producer" |
| `corruption_or_repair_path` | re-exports the inventory's six corruption postures plus three new generated-structure / read-only fallback classes |
| `migration_policy_owner` | who owns the migration policy |
| `downgrade_readability` | re-exported from the inventory's downgrade-readability vocabulary |
| `stable_label_implication` | what a stable-channel label promises for this family |
| `applicable_state_object_refs[]` | opaque refs to inventory rows the family covers (empty for non-inventory families) |
| `consumer_surfaces[]` | non-empty closed list of downstream surfaces (Help, Support Center, Migration Center, shiproom, Project Doctor, etc.) |
| `policy_summary_text_keys` | reviewable prose snippets surfaces resolve verbatim |
| `restore_packet_refs[]` | opaque refs to `restore_after_downgrade_packet_record` bodies that worked through this row |
| `notes` | reviewable prose after redaction |

Frozen rules:

1. `migrating_with_equivalence_map` rows MUST carry
   `backup_required_before_destructive_migration`. The schema
   enforces this.
2. `corruption_or_repair_path = backup_rollback` rows MUST carry
   `backup_required_before_destructive_migration`.
3. `signed_epoch_replacement` rows MUST carry
   `rollback_or_downgrade_behavior =
   downgrade_refused_by_authority_signed_epoch`,
   `downgrade_readability = downgrade_refused`, and
   `migration_policy_owner = admin_or_control_authority`.
4. `cache_or_index_state` rows MUST carry
   `migration_policy_owner = disposable_derived_cache` and a
   corruption-or-repair path drawn from
   `{rebuild_automatically, no_repair_path_disposable_only}`.
5. `user_authored_durable_state` rows MUST carry
   `backup_required_before_destructive_migration`.
6. `workspace_authored_durable_state` rows MUST carry
   `backup_required_before_destructive_migration` or
   `backup_optional_user_offered`.
7. `consumer_surfaces[]` is non-empty. A row that feeds no surface
   would be invisible and is non-conforming.
8. `policy_summary_text_keys` is mandatory. Surfaces resolve copy
   keys against this block; they MUST NOT re-author policy text.

## 2. Artifact-family rows

The closed family set is fixed:

### 2.1 `user_authored_durable_state`

User settings, keybindings, snippets, themes, command aliases,
terminal preferences, AI-preset selections, extension-selection
inventory, machine-specific settings.

| Field | Value |
|---|---|
| Schema evolution | `additive_minor_only` or `migrating_with_equivalence_map` (depends on the inventory row) |
| Backward readable window | `additive_minor_only_back_to_first_release` for additive rows; `translation_required_through_equivalence_map` for migrating rows |
| Forward read expectations | `compatible_forward_read_with_translation` for migrating rows; `forward_read_with_documented_fallback` for additive rows |
| Backup before migrate | `backup_required_before_destructive_migration` (frozen) |
| Rollback or downgrade | `rollback_to_preserved_prior_artifact` |
| Corruption or repair | inherits the per-object posture: `backup_rollback` for migrating settings/keybindings/themes/extension-selection, `open_with_warning` for additive settings/snippets/preferences |
| Migration policy owner | `user_authored_durable_truth` |
| Stable label implication | `stable_label_implies_compatible_translation_with_backup` (default) or `stable_label_implies_no_format_change` (frozen rows) |

### 2.2 `workspace_authored_durable_state`

Workspace manifests, workset manifests, tasks and launch configs,
extension recommendations, extension lockfile.

| Field | Value |
|---|---|
| Schema evolution | `additive_minor_only` or `migrating_with_equivalence_map` |
| Backward readable window | `additive_minor_only_back_to_first_release` or `translation_required_through_equivalence_map` |
| Forward read expectations | `compatible_forward_read_with_translation` for migrating rows; `forward_read_with_documented_fallback` for additive rows |
| Backup before migrate | `backup_required_before_destructive_migration` (default) or `backup_optional_user_offered` (recommendations) |
| Rollback or downgrade | `rollback_to_preserved_prior_artifact` for migrating rows; `downgrade_reads_with_documented_fallback` for additive rows |
| Corruption or repair | `repair_flow` (manifest, workset, lockfile) or `open_with_warning` (tasks/launch) |
| Migration policy owner | `workspace_authored_durable_truth` |
| Stable label implication | `stable_label_implies_compatible_translation_with_backup` |

### 2.3 `cache_or_index_state`

Index cache, object store, execution-context cache, render atlases,
logs / traces past retention, interactive hot cache, knowledge
cache, prebuild environment cache.

| Field | Value |
|---|---|
| Schema evolution | `frozen_no_evolution` or `content_addressed_immutable` or `additive_minor_only` |
| Backward readable window | `no_backward_window_content_addressed` or `additive_minor_only_back_to_first_release` |
| Forward read expectations | `forward_read_content_addressed_supersede` or `exact_forward_read` |
| Backup before migrate | `backup_not_applicable_disposable` or `backup_not_applicable_content_addressed` |
| Rollback or downgrade | `downgrade_rebuild_from_authoritative_truth` |
| Corruption or repair | `rebuild_automatically` (default) or `no_repair_path_disposable_only` |
| Migration policy owner | `disposable_derived_cache` (frozen) |
| Stable label implication | `stable_label_implies_content_addressed_supersede` |

### 2.4 `public_schemas_or_interfaces`

Public schemas, public IPC / WIT interfaces, public OpenAPI surfaces,
the schemas this product publishes that downstream consumers compose
over.

| Field | Value |
|---|---|
| Schema evolution | `additive_minor_only` (default; breaking changes require a new decision row) |
| Backward readable window | `stable_within_major_version` |
| Forward read expectations | `compatible_forward_read_with_translation` |
| Backup before migrate | `backup_handled_by_authority` (the public-interface authority owns the migration story) |
| Rollback or downgrade | `downgrade_reads_with_documented_fallback` |
| Corruption or repair | `fail_closed_for_privileged_operations` for signed surfaces; `open_with_warning` otherwise |
| Migration policy owner | `public_interface_authority` |
| Stable label implication | `stable_label_implies_no_format_change` within the major version |

### 2.5 `portable_state_packages`

`portable_state_manifest_record` packages, profile-sync snapshots,
layout snapshots, session-restore manifests, workspace-manifest
bundles, support-recovery bundles.

| Field | Value |
|---|---|
| Schema evolution | `additive_minor_only` on the manifest schema; per-section bodies follow their own rows |
| Backward readable window | `stable_within_major_version` |
| Forward read expectations | `forward_read_inspect_only_off_producing_machine` (default for `restore_compare_export` and `support_review_export`); `compatible_forward_read_with_translation` otherwise |
| Backup before migrate | `backup_required_before_destructive_migration` for the wrapped per-section bodies; `backup_handled_by_authority` for signed bundles |
| Rollback or downgrade | `downgrade_inspect_only_off_producing_machine` (re-exports the package contract's inspect-only posture for off-producing-machine destinations) or `downgrade_compare_only_no_apply` |
| Corruption or repair | `compare_only_no_apply` or `repair_flow` |
| Migration policy owner | `portable_state_package_authority` |
| Stable label implication | `stable_label_implies_compatible_translation_with_backup` |

### 2.6 `generated_or_structured_artifacts`

Generated derivations the product surfaces (structured artifact
views, derived schemas, generated documentation projections, render
atlas projections that the user inspects but does not author).

| Field | Value |
|---|---|
| Schema evolution | `frozen_no_evolution` or `content_addressed_immutable` |
| Backward readable window | `no_backward_window_content_addressed` |
| Forward read expectations | `forward_read_content_addressed_supersede` |
| Backup before migrate | `backup_not_applicable_content_addressed` |
| Rollback or downgrade | `downgrade_compare_only_no_apply` or `downgrade_read_only_mode` |
| Corruption or repair | `compare_only_no_apply` or `read_only_mode` |
| Migration policy owner | `generated_structure_producer` |
| Stable label implication | `stable_label_implies_compare_or_read_only_fallback` |

## 3. Backup-before-migrate matrix

The matrix freezes the rule per artifact family. It re-exports the
state-object inventory rule per row but binds the family-level summary
the help / support / migration-center / shiproom surfaces show.

| Family | Required backup | Preservation triggers | Forbidden shortcut |
|---|---|---|---|
| `user_authored_durable_state` | `backup_required_before_destructive_migration` | `schema_meaning_changed`, `schema_translation_required`, `destructive_migration` | rewriting in place without a preserved prior artifact ref |
| `workspace_authored_durable_state` | required for manifest/workset/lockfile/tasks; optional for recommendations | as above | dropping a manifest field without a preserved prior artifact |
| `cache_or_index_state` | not applicable | rebuild from authoritative truth | claiming `compatible` translation against a disposable cache |
| `public_schemas_or_interfaces` | handled by the public-interface authority | new major version requires a decision row | a stable-channel claim without a backward-window note |
| `portable_state_packages` | handled at the per-section body level (re-exports user / workspace rules) | per-section migration | overwriting workspace truth on import |
| `generated_or_structured_artifacts` | not applicable | content-addressed supersede | implying the body is editable when it is generated |

Frozen rules:

1. A migration that rewrites a
   `backup_required_before_destructive_migration` body without
   preserving the prior artifact ref is non-conforming, both at the
   per-object inventory layer and at this matrix layer.
2. A `restore_after_downgrade_packet_record` whose
   `original_artifact_identity.artifact_family_class` is
   `user_authored_durable_state` and whose
   `migrated_state.fidelity_label` is `compatible` or `manual_review`
   MUST carry `backup_path.backup_present = true`. The schema
   enforces this.
3. A surface that quotes a stable label without quoting one of the
   `stable_label_implication_class` values is non-conforming.

## 4. Restore-after-downgrade packet shape

Every restore-after-downgrade packet MUST emit exactly one
`restore_after_downgrade_packet_record`. The record carries:

| Block | Meaning |
|---|---|
| `original_artifact_identity` | artifact-family class, source artifact ref, source schema version, producer build, and the `compatibility_window_row_ref` that binds the packet to one matrix row |
| `backup_path` | `backup_present`, preserved prior artifact ref, preservation reason, redaction class, optional compare / export / rollback-checkpoint refs |
| `migrated_state` | migrated artifact ref, migrated schema version, fidelity label, equivalence map ref |
| `downgraded_target` | target build (schema version, channel, platform class, instance handle) and one or more typed `downgrade_triggers` |
| `preserved_versus_lost_fields[]` | one row per named field-class that crossed the boundary, with status (`preserved_exact`, `translated_meaning_changed`, `dropped_unsupported_on_target`, `replaced_with_default`, `manual_review_required`, etc.) |
| `next_safe_action` | one closed `next_safe_action_class` plus the action target ref and reason |
| `unsupported_state` | one closed `unsupported_state_class` describing what (if anything) remains unsupported on the target |
| `restore_provenance_refs[]` | optional opaque refs to the cross-artifact `state_restore_provenance_record` bodies the apply emits |

Frozen rules:

1. `backup_path.backup_present = true` requires a non-null
   `preserved_prior_artifact_ref`.
2. `backup_path.backup_present = false` requires a
   `preservation_reason` drawn from the
   `no_preservation_*` or `user_offered_optional_snapshot` set.
3. A packet with `migrated_state.fidelity_label` in
   `{compatible, manual_review}` MUST carry a non-null
   `equivalence_map_ref`.
4. A packet whose original artifact family is
   `user_authored_durable_state` and whose fidelity label is
   `compatible` or `manual_review` MUST carry
   `backup_path.backup_present = true`. The schema enforces this.
5. `downgrade_triggers[]` is non-empty. The trigger enum is
   re-exported from the migration-and-restore playbook plus the
   portable-state package contract, and adds the two
   `generated_structure_fallback_to_*` triggers.
6. `next_safe_action.action_class` is closed. A packet that recommends
   a free-form action is non-conforming.
7. `unsupported_state.unsupported_state_class` is mandatory. A packet
   whose value is `no_unsupported_state` still emits the row so help /
   support / migration-center / shiproom surfaces always render an
   answer.

## 5. Cross-surface consumption

The matrix and packet feed downstream surfaces verbatim. No surface
re-authors policy:

- **Help Center** resolves per-artifact-family copy from
  `policy_summary_text_keys` and
  `stable_label_implication_class`.
- **Support Center** resolves the per-artifact-family
  `corruption_or_repair_path` and `next_safe_action_class` from the
  matrix and the active packets, so support intake routes the same
  way for the same artifact every time.
- **Migration Center** resolves
  `backup_before_migrate_rule`,
  `rollback_or_downgrade_behavior`, and the per-field
  `field_status_class` rows so a preview shows what will be preserved,
  translated, dropped, or escalated before any apply.
- **Shiproom dashboard** resolves
  `stable_label_implication_class` and the active packets for any
  family rendered as part of beta widening, stable widening, or
  milestone close. A stable-channel widening that disagrees with the
  matrix's stable-label implication is non-conforming.
- **Project Doctor** resolves `corruption_or_repair_path` and
  `next_safe_action_class` for the family of any failing finding.

## 6. Seed cases

The seed fixtures cover the four scenarios named in the spec.

| Fixture | Scenario | Primary point |
|---|---|---|
| [`settings_profile_compatible_migration_with_backup.yaml`](../../fixtures/state/durable_state_cases/settings_profile_compatible_migration_with_backup.yaml) | settings / profile migration | user-authored durable settings translate through the equivalence map; backup is mandatory; next safe action opens compare with the preserved body |
| [`workspace_tasks_compatible_migration_with_backup.yaml`](../../fixtures/state/durable_state_cases/workspace_tasks_compatible_migration_with_backup.yaml) | workspace task or launch migration | workspace-authored durable manifest field is renamed; preview preserves the prior body; one task entry escalates to manual review |
| [`portable_state_package_downgraded_off_producing_machine.yaml`](../../fixtures/state/durable_state_cases/portable_state_package_downgraded_off_producing_machine.yaml) | portable-state package downgrade | manifest crosses to a target below the channel floor; posture downgrades to inspect-only; missing-extension dependency drops a saved-view row to a placeholder |
| [`generated_structure_fallback_to_compare_only.yaml`](../../fixtures/state/durable_state_cases/generated_structure_fallback_to_compare_only.yaml) | generated-structure fallback | derived structured artifact crosses to a target whose schema cannot read it; the surface falls back to compare-only and read-only mode without claiming a translation |

## 7. Conformance checklist

A compatibility-window row conforms when its body answers:

- Which `artifact_family_class` does it cover?
- Which `schema_evolution_posture`,
  `backward_readable_window_class`, and
  `forward_read_expectations_class` describe the family?
- Which `backup_before_migrate_rule_class` is mandatory?
- Which `rollback_or_downgrade_behavior_class` covers a target below
  the producer?
- Which `corruption_or_repair_path_class` covers integrity failure?
- Which `migration_policy_owner_class` owns the policy?
- Which `downgrade_readability_class` describes older targets?
- Which `stable_label_implication_class` does the stable-channel claim
  rest on?
- Which inventory row refs (if any) does the row cover?
- Which `consumer_surface_class` values does the row feed, and what
  prose lives in `policy_summary_text_keys`?

A restore-after-downgrade packet conforms when its body answers:

- Which `original_artifact_identity` and
  `compatibility_window_row_ref` does it bind to?
- What is the `backup_path` (present or not, with which preserved
  prior artifact ref, preservation reason, redaction class, and
  optional compare / export / rollback handles)?
- What is the `migrated_state` (migrated ref, migrated schema
  version, fidelity label, equivalence map ref)?
- What is the `downgraded_target` (target build plus the typed
  `downgrade_triggers[]`)?
- Which `preserved_versus_lost_fields[]` rows cover the boundary
  crossing, with what `field_status_class` values?
- Which `next_safe_action_class` does the packet recommend, and
  against which target ref?
- Which `unsupported_state_class` (including `no_unsupported_state`)
  applies?

If any answer requires new vocabulary, this contract and the schemas
are extended first, in the same change as the new fixture under
[`/fixtures/state/durable_state_cases/`](../../fixtures/state/durable_state_cases/).

## 8. Changing this vocabulary

- **Additive-minor** changes (a new
  `consumer_surface_class`, a new
  `next_safe_action_class`, a new `downgrade_trigger_class`, a new
  `field_class_class`, a new `field_status_class`, a new
  `unsupported_state_class`) land here and in the schemas in the same
  change. The change MUST cite a motivating fixture under
  [`/fixtures/state/durable_state_cases/`](../../fixtures/state/durable_state_cases/)
  and (when the new value rides above the per-object inventory) a
  matching row in the state-object inventory.
- **Repurposing** an existing `artifact_family_class`,
  `backward_readable_window_class`,
  `forward_read_expectations_class`,
  `rollback_or_downgrade_behavior_class`,
  `corruption_or_repair_path_class`,
  `migration_policy_owner_class`, or `stable_label_implication_class`
  is breaking and requires a governance decision row.
- The cross-references above are minimums. A row MAY feed more
  consumer surfaces than the listed minimum; widening the row to
  silently drop a required reference (for example, claiming a
  `migrating_with_equivalence_map` row that does not require a
  backup) is non-conforming.

## 9. Reference rows

- State-object inventory — per-object authority owners, schema-
  evolution postures, backup rules, corruption postures the matrix
  re-exports.
- Migration-and-restore playbook — fidelity labels, downgrade
  triggers, preserved-prior-artifact rules the packet quotes.
- Portable-state package contract — the package-level import-posture,
  redaction-manifest, and compatibility-range vocabularies the
  `portable_state_packages` row composes over.
- Profile-and-state map — portability classes, redaction classes, and
  authority-owner rows the matrix re-exports.
- Restore-artifact-family contract — the workspace-authority
  checkpoint and window-topology snapshot bodies the packet
  references via `restore_provenance_refs[]` (never inline).
