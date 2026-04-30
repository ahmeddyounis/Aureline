# Restore-destination review, retained-vs-overwritten classes, and checkpoint-before-overwrite contract

This document freezes the cross-surface vocabulary every backup
restore, migration apply, imported-package open, and support-handoff
flow uses when it answers the four questions a reviewer asks **before**
durable local state is replaced:

- **what does the source claim to carry, and onto which target scope
  does the apply land** — recorded in the source identity and target
  scope of one restore-destination review;
- **which classes will be retained, overwritten, merged, recovered as
  drafts, stand in as placeholders, downgrade to compare-only, or be
  omitted altogether** — recorded as one row per class in the
  retained-vs-overwritten inventory;
- **what checkpoint covers the overwrite, when is one mandatory, when
  is one only recommended, and what acknowledges a recommended skip**
  — recorded in the checkpoint-before-overwrite gate;
- **which review-time outcome did the destination land on — exact,
  partial, incompatible schema downgrade, inspect-only imported
  package, or placeholder-only overlay — and which review actions are
  enabled to proceed, downgrade, or cancel** — recorded as the
  destination-review outcome and the closed action set.

The review is the **shared inspectable body** that every restore
explanation surface emits **before confirm**, so backup, migration, and
imported-state operations cannot overwrite durable local state without
visible scope, retained-vs-overwritten inventory, and a checkpoint path
the user can roll back to. It is not a restore engine, a migration
executor, or a UI rendering plan; it is the contract those surfaces
MUST conform to so a reviewer, support engineer, or migration tool can
explain — and undo — an apply mechanically instead of negotiating
parallel field names.

The machine-readable schemas live at:

- [`/schemas/state/restore_destination_review.schema.json`](../../schemas/state/restore_destination_review.schema.json)
- [`/schemas/state/retained_vs_overwritten_row.schema.json`](../../schemas/state/retained_vs_overwritten_row.schema.json)

Worked fixtures live under:

- [`/fixtures/state/restore_destination_cases/`](../../fixtures/state/restore_destination_cases/)

This contract composes with:

- [`/docs/state/migration_and_restore_playbook.md`](./migration_and_restore_playbook.md)
- [`/docs/state/restore_artifact_family_contract.md`](./restore_artifact_family_contract.md)
- [`/docs/state/restore_provenance_and_placeholder_contract.md`](./restore_provenance_and_placeholder_contract.md)
- [`/docs/state/portable_state_package_contract.md`](./portable_state_package_contract.md)
- [`/docs/state/profile_and_state_map.md`](./profile_and_state_map.md)
- [`/docs/state/workspace_memory_contract.md`](./workspace_memory_contract.md)
- [`/docs/state/state_object_inventory.md`](./state_object_inventory.md)

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, or one of the upstream state contracts above, those
documents win and this contract plus its schemas MUST be updated in
the same change. Where a downstream restore, migration, support-export,
or imported-state surface mints a parallel target-scope, retained,
overwritten, checkpoint, or destination-outcome vocabulary, this
contract wins and the surface is non-conforming.

## Why freeze this now

Restore-destination drift starts when each new flow — backup restore,
migration apply, imported-package open, support handoff —
defines its own way of saying "this is what your machine will look
like after the apply." Without one frozen review:

- a backup restore promises `Restore complete` while a migration apply
  quotes `Restored with adjustments` and a support handoff describes a
  `Partial recovery` outcome — and none of the three labels are in the
  schema;
- durable user state is overwritten without a visible checkpoint,
  leaving no rollback path the user can name;
- a workspace-shared manifest is silently merged through a
  compatibility translation while the user believed the destination
  body was retained;
- an inspect-only imported package surfaces a `Confirm overwrite`
  button that should not exist;
- an incompatible schema downgrade is hidden behind a generic
  `Compatible` badge with no compare/export reachable;
- a placeholder-only overlay is described as `Restored` even though
  every live surface stands in as a placeholder;
- a recommended-before-overwrite checkpoint is silently skipped with
  no typed acknowledgement;
- the destination-review record carries different field names on
  every surface and cannot be diffed mechanically.

The review forecloses these patterns by treating source identity,
target scope, schema/version note, retained-vs-overwritten classes,
checkpoint-before-overwrite, and review actions as five distinct
contracts inside one frozen body. Once the boundary is named, every
apply stays explicit, comparable, exportable, and reversible.

## Scope

- Freeze one `state_restore_destination_review_record` shape carrying
  source identity, target scope, schema/version note, retained-vs-
  overwritten rows by class, checkpoint-before-overwrite state,
  downgrade/inspect-only/partial-restore/placeholder-only outcomes,
  review-action set, restore-provenance-record link, and compare/export
  hooks.
- Freeze the closed five-class destination-review outcome —
  `exact_restore`, `partial_restore`, `incompatible_schema_downgrade`,
  `inspect_only_imported_package`, `placeholder_only_overlay` — and
  the conditional rules tying each outcome to its required handles,
  retained/overwritten rows, and checkpoint-state values.
- Freeze the closed six-class reviewable-class taxonomy covering
  `workspace_state`, `profile_defaults`, `layout_topology`,
  `evidence_packets`, `caches`, and `machine_local_assets` so every
  restore review surfaces every class in one place.
- Freeze the closed eight-class treatment vocabulary (`retained_unchanged`,
  `overwritten_with_source`, `merged_through_equivalence_map`,
  `recovered_as_drafts`, `placeholder_only`,
  `downgraded_to_compare_only`, `omitted`, `inspect_only_overlay`)
  so every class reads as either retained, overwritten by a named path,
  preserved as evidence/placeholder, or unchanged.
- Freeze the durability-posture vocabulary so the checkpoint-before-
  overwrite gate is mechanical: `durable_user_authored`,
  `authoritative_local_record`, `workspace_shared_truth`,
  `portable_settings`, `replaceable_machine_state`,
  `disposable_derived`, `evidence_metadata`.
- Freeze the closed checkpoint-requirement vocabulary
  (`mandatory_before_overwrite`, `recommended_before_overwrite`,
  `not_applicable`, `not_supported`) and the closed checkpoint-state
  vocabulary (`created`, `mandatory_pending`, `optional_skipped`,
  `not_applicable`, `not_supported`, `refused`).
- Freeze the closed nine-action review-action set so review surfaces
  cannot mint `force_apply`, `merge_with_skip`, or other
  silent-overwrite actions.
- Freeze the rule that the `plain_language_summary` on every row
  states what stays and what changes in human terms before
  `confirm_overwrite` is admitted.

## Out of scope

- The restore engine, migration executor, sync apply pipeline, or UI
  rendering of the review card. The vocabulary freeze lands here;
  production surfaces compose over it later.
- Final product copy. Display copy may render `Exact restore`,
  `Partial restore`, `Incompatible schema downgrade`,
  `Inspect only imported package`, and `Placeholder only overlay`;
  the closed machine set is fixed.
- The recursive pane-tree body, the workspace-authority checkpoint
  body, the portable-state package manifest, or the restore-provenance
  record. This review references them by opaque id.

## 1. Record boundary

Every restore-destination review record under this contract MUST
resolve every field to exactly one of the five boundaries below.
Flattening them into one payload is non-conforming.

| Boundary | What it carries | Where it lives |
|---|---|---|
| **Provenance core** | source identity, created-at, producer build, source schema version, redaction class, top-level compare/export refs, restore-provenance-record link | top-level fields of `state_restore_destination_review_record` |
| **Target scope** | scope class, workspace authority ref, profile ref, window ids, workset ids, evidence index ref, target-machine class | `target_scope` |
| **Retained-vs-overwritten inventory** | one row per reviewable class describing treatment, durability posture, checkpoint requirement, before/after refs, evidence posture, treatment reason, destination scope targets, plain-language summary | `retained_rows[]`, `overwritten_rows[]`, `placeholder_only_rows[]`, `downgraded_rows[]`, `omitted_rows[]` |
| **Checkpoint-before-overwrite gate** | state, checkpoint class, checkpoint ref, rollback-path ref, scope-target refs, skip-acknowledgement ref, refused reason | `checkpoint_before_overwrite` |
| **Review actions** | closed nine-action set with enabled, consequence-class, typed disabled-reason | `review_actions[]` |

Rules (frozen):

1. A single record MUST cover exactly one source artifact applied to
   exactly one target scope. A multi-target apply emits one record per
   target scope.
2. Every reviewable class in §3 MUST appear in exactly one of
   `retained_rows[]`, `overwritten_rows[]`, `placeholder_only_rows[]`,
   `downgraded_rows[]`, or `omitted_rows[]`. A class that is silently
   absent from every row set is non-conforming, because the reviewer
   cannot tell whether it was retained, overwritten, or excluded.
3. The `retained_rows[]` set carries every row whose treatment is
   `retained_unchanged`. A row with any other treatment is
   non-conforming inside `retained_rows[]`.
4. The `overwritten_rows[]` set carries every row whose treatment is
   `overwritten_with_source`, `merged_through_equivalence_map`, or
   `recovered_as_drafts`. A row with any other treatment is
   non-conforming inside `overwritten_rows[]`.
5. The `placeholder_only_rows[]` set carries every row whose treatment
   is `placeholder_only`. The destination authority is unchanged; the
   source pane reopens as a placeholder card with typed recovery
   actions.
6. The `downgraded_rows[]` set carries every row whose treatment is
   `downgraded_to_compare_only` or `inspect_only_overlay`. No apply
   runs for these rows; only compare/export actions are admitted.
7. The `omitted_rows[]` set carries every row whose treatment is
   `omitted`. The source carried nothing for the class; the destination
   body is untouched.
8. The compare and export refs on the top-level record cover the
   review as a whole. Per-row compare/export is reached through each
   row's own refs, not by minting parallel handles inside the action
   set.

## 2. Provenance core fields

Required fields (frozen):

- `review_id` — opaque stable id for the record.
- `source` — `artifact_family`, `source_class`, and
  `source_artifact_ref`. Re-export of the restore-provenance
  vocabulary; the review never minted parallel labels.
- `created_at`, `emitted_at` — producer-local monotonic timestamps.
- `producer_build` — producer name, version, channel, platform class,
  pseudonymous instance handle. Never a raw hostname.
- `source_schema_version` — opaque schema-version string the producer
  used.
- `redaction_class` — closed redaction-class enum reused from the
  shared portability vocabulary.
- `destination_review_outcome` — exactly one value from §6.
- `restore_provenance_record_ref` — opaque ref to the cross-artifact
  `state_restore_provenance_and_placeholder_record` this review
  composes over. Required for every outcome above `exact_restore`.
- `compare_ref`, `export_ref` — top-level handles for the review as a
  whole. Required for every outcome above `exact_restore`.
- `notes` — reviewer prose only, after the active redaction policy is
  applied. Never a place to hide a missing class, an excluded scope, or
  an outcome claim.

## 3. Reviewable classes

The closed six-class set is fixed. Display copy may render the
title-case labels shown below.

| Display label | Machine enum | Default durability posture | Notes |
|---|---|---|---|
| `Workspace state` | `workspace_state` | `workspace_shared_truth` (or `authoritative_local_record` for local-only workspaces) | Workspace-authority bodies, dirty-buffer journals, active worksets, trusted roots. |
| `Profile defaults` | `profile_defaults` | `portable_settings` (or `durable_user_authored` for portable-profile bodies) | Settings, keybindings, snippets, themes, extension inventory. |
| `Layout topology` | `layout_topology` | `replaceable_machine_state` | Window-topology snapshots, stable pane ids, tab/group topology, monitor-affinity hints. |
| `Evidence packets` | `evidence_packets` | `evidence_metadata` | Restore-provenance evidence refs, transcripts, snapshots, support-evidence indices. |
| `Caches` | `caches` | `disposable_derived` | Disposable derived state. Caches admit no checkpointable form: their default checkpoint requirement is `not_applicable`. |
| `Machine-local assets` | `machine_local_assets` | `replaceable_machine_state` (or `evidence_metadata`) | Display hints, machine-unique handles, local-only credentials slots. Carries `not_supported` for any treatment that would require a checkpoint, since the destination scope admits no checkpointable form. |

Rules (frozen):

1. Every class is reviewed exactly once per record. A class that
   appears in two row sets is non-conforming.
2. A row's `durability_posture` is the producer's claim about the
   destination body. The checkpoint-before-overwrite gate consumes
   this posture verbatim — durable_user_authored and
   authoritative_local_record posture force `mandatory_before_overwrite`
   when the treatment overwrites or merges; disposable_derived posture
   forces `not_applicable` or `not_supported`.
3. A row whose `reviewable_class` is `caches` and whose `treatment`
   overwrites or merges MUST set `checkpoint_requirement` to
   `not_applicable` and `checkpoint_ref` to null. Cache state is
   disposable; checkpoint creation is not the right protection here,
   and minting one would be misleading.
4. A row whose `reviewable_class` is `machine_local_assets` and whose
   treatment overwrites or merges MUST set `checkpoint_requirement` to
   `not_supported` unless the row's `durability_posture` is
   `evidence_metadata`. Machine-local-asset overwrites narrow the
   destination-review outcome to `inspect_only_imported_package` or
   `placeholder_only_overlay` so reviewers see the limitation.

## 4. Retained-vs-overwritten row fields

Each row in `retained_rows[]`, `overwritten_rows[]`,
`placeholder_only_rows[]`, `downgraded_rows[]`, or `omitted_rows[]`
follows the row schema at
[`/schemas/state/retained_vs_overwritten_row.schema.json`](../../schemas/state/retained_vs_overwritten_row.schema.json).

Required fields (frozen):

- `row_id` — opaque stable id.
- `reviewable_class` — exactly one of the six classes from §3.
- `treatment` — exactly one of the closed eight-class treatment values
  in §5.
- `durability_posture` — exactly one of the closed seven-class
  durability postures.
- `checkpoint_requirement` — exactly one of the closed four-class
  checkpoint-requirement values.
- `checkpoint_ref` — opaque ref to the rollback/authority checkpoint
  that protects the row's destination body. Mandatory when the
  requirement is `mandatory_before_overwrite`. Null when the
  requirement is `not_applicable`, `not_supported`, or
  `recommended_before_overwrite` with a typed
  `checkpoint_skip_acknowledgement_ref`.
- `preserved_prior_artifact_ref` — opaque ref for the destination body
  preserved before the apply. Mandatory when the row's treatment is
  `overwritten_with_source`, `merged_through_equivalence_map`, or
  `recovered_as_drafts` over a durable posture. Null when the
  treatment is `retained_unchanged`, `omitted`, or
  `inspect_only_overlay`.
- `after_apply_ref` — opaque ref for what will be present at the
  destination after apply. Null when the treatment is
  `downgraded_to_compare_only` or `inspect_only_overlay` (no apply
  will run for the row).
- `compare_ref`, `export_ref` — opaque compare/export handles for the
  row. Required when the treatment is `overwritten_with_source`,
  `merged_through_equivalence_map`, `recovered_as_drafts`,
  `downgraded_to_compare_only`, or `inspect_only_overlay`.
- `evidence_retained` — boolean.
- `treatment_reason` — exactly one closed reason class from the
  schema's `treatment_reason_class` enum. Free-form prose substitutions
  are non-conforming.
- `destination_scope_targets[]` — at least one
  `scope_target_record` row naming `scope_target_kind` (one of
  `workspace_authority`, `profile`, `window_topology`, `workset`,
  `evidence_index`, `cache_namespace`, `machine_local_asset`) and an
  opaque `scope_target_ref`. Reviewers MUST be able to resolve every
  overwrite to a named target.
- `plain_language_summary` — redaction-aware text the review surface
  renders for this row before confirm. The summary MUST state, in
  human terms, what stays and what changes; vague phrasing such as
  "partial" or "best-effort" is non-conforming on its own.

Optional fields:

- `evidence_ref` — opaque ref to retained evidence body.
- `checkpoint_skip_acknowledgement_ref` — opaque ref for the typed
  acknowledgement that recorded an explicit skip of a
  `recommended_before_overwrite` checkpoint. Required when the row's
  `checkpoint_requirement` is `recommended_before_overwrite` and
  `checkpoint_ref` is null.
- `note` — short reviewer note.

## 5. Treatment vocabulary

The closed eight-class machine set is fixed. Display copy may render
title-case labels.

| Display label | Machine enum | Meaning | Per-row obligations |
|---|---|---|---|
| `Retained unchanged` | `retained_unchanged` | the destination body is left as-is; the source did not claim authority over the class (or the destination already matches the source) | `checkpoint_requirement = not_applicable`; `checkpoint_ref`, `preserved_prior_artifact_ref`, and `after_apply_ref` are null |
| `Overwritten with source` | `overwritten_with_source` | the source body replaces the destination body | `compare_ref`, `export_ref`, `after_apply_ref` are required; durable postures force `mandatory_before_overwrite` |
| `Merged through equivalence map` | `merged_through_equivalence_map` | the source translated through a declared equivalence map and the merged body lands at the destination | `compare_ref`, `export_ref`, `after_apply_ref` required; the schema-migration note's `equivalence_map_ref` and `rollback_checkpoint_ref` MUST be non-null; durable postures force `mandatory_before_overwrite` |
| `Recovered as drafts` | `recovered_as_drafts` | dirty-buffer or local-history bodies were rehydrated as drafts; the destination keeps its prior body until the user accepts | `preserved_prior_artifact_ref`, `compare_ref`, `export_ref`, and `after_apply_ref` required; `treatment_reason = drafts_pending_review` |
| `Placeholder only` | `placeholder_only` | the source pane reopens as a placeholder card with typed recovery actions; the destination authority is unchanged | `after_apply_ref` is null; `checkpoint_requirement` is `not_applicable` or `recommended_before_overwrite` |
| `Downgraded to compare-only` | `downgraded_to_compare_only` | the source class cannot apply on this destination; only compare/export actions are admitted | `compare_ref`, `export_ref` required; `after_apply_ref` is null |
| `Omitted` | `omitted` | the source carried nothing for the class; the destination body is untouched | every checkpoint/preserved/after ref is null; `checkpoint_requirement = not_applicable` |
| `Inspect only overlay` | `inspect_only_overlay` | the package's import posture is `inspect_only`; the row exists only to surface compare/export actions | `compare_ref`, `export_ref` required; `after_apply_ref` and `checkpoint_ref` are null |

Rules (frozen):

1. The label set is closed. A surface that mints `partial_apply`,
   `best_effort_overwrite`, `applied_with_caveat`, or another parallel
   label is non-conforming.
2. `overwritten_with_source` and `merged_through_equivalence_map`
   over `durable_user_authored` or `authoritative_local_record`
   posture MUST set `checkpoint_requirement` to
   `mandatory_before_overwrite` and carry a non-null `checkpoint_ref`
   plus a non-null `preserved_prior_artifact_ref`. The schema enforces
   this with conditional `if/then` rules.
3. `recovered_as_drafts` MUST always carry a non-null
   `preserved_prior_artifact_ref` so the user can roll back to the
   on-disk body.
4. `placeholder_only` is the only treatment that is allowed for a
   `live-surface` row whose source authority cannot reopen live.
   Every placeholder row MUST resolve through the
   `restore_provenance_record_ref`'s
   `missing_dependency_placeholder_cards[]` set.

## 6. Destination-review outcome

The closed five-class destination-review outcome is fixed. Display
copy may render the title-case labels shown below.

| Display label | Machine enum | Meaning | Required row sets and checkpoint state |
|---|---|---|---|
| `Exact restore` | `exact_restore` | every reviewable class round-trips without translation, placeholder, review, or overwrite of durable user state | `overwritten_rows[]`, `placeholder_only_rows[]`, `downgraded_rows[]` are empty; `checkpoint_before_overwrite.state` is `not_applicable` |
| `Partial restore` | `partial_restore` | one or more classes were overwritten or merged through compatibility translation; placeholder rows cover incomplete dependencies | `overwritten_rows[]` is non-empty; durable rows force `mandatory_before_overwrite` checkpoints; `checkpoint_before_overwrite.state` is `created` (or `optional_skipped` for recommended-only rows) |
| `Incompatible schema downgrade` | `incompatible_schema_downgrade` | the source schema falls outside the destination's admitted range, the producer refused a downgrade, or the equivalence map is missing | `overwritten_rows[]` and `placeholder_only_rows[]` empty; `downgraded_rows[]` non-empty; `checkpoint_before_overwrite.state` is `not_applicable` or `not_supported`; `confirm_overwrite` is disabled with `compatibility_range_outside` |
| `Inspect only imported package` | `inspect_only_imported_package` | the package's import posture is `inspect_only`; the destination MUST NOT apply the package | `overwritten_rows[]` and `placeholder_only_rows[]` empty; `downgraded_rows[]` non-empty (every reviewed class lands as `inspect_only_overlay`); `checkpoint_before_overwrite.state` is `not_applicable` or `not_supported`; `confirm_overwrite` is disabled with `inspect_only_outcome` |
| `Placeholder only overlay` | `placeholder_only_overlay` | layout topology and stable pane ids land, but every live surface stands in as a placeholder; authority is unchanged | `overwritten_rows[]` is empty; `placeholder_only_rows[]` is non-empty; `checkpoint_before_overwrite.state` MAY be `not_applicable` |

Rules (frozen):

1. The label set is closed. A surface that invents `partial`,
   `best_effort`, `restored_with_adjustments`, or another parallel
   label is non-conforming.
2. `exact_restore` is forbidden once any row's treatment is anything
   other than `retained_unchanged` or `omitted`.
3. `partial_restore`, `incompatible_schema_downgrade`,
   `inspect_only_imported_package`, and `placeholder_only_overlay`
   MUST carry a non-null `restore_provenance_record_ref` so the
   reviewer can resolve the same fidelity, missing-dependency, and
   placeholder vocabulary on the cross-artifact provenance record.
4. `incompatible_schema_downgrade` and
   `inspect_only_imported_package` MUST disable `confirm_overwrite`
   with the closed `disabled_reason_class` values
   `compatibility_range_outside` or `inspect_only_outcome`. The action
   is never silently hidden — it is rendered disabled with a typed
   reason so the reviewer can see why apply is unreachable.

## 7. Checkpoint-before-overwrite gate

The checkpoint-before-overwrite gate is the only place inside this
contract that may declare an apply is safe to confirm. It consumes the
durability posture and treatment of every overwritten row and resolves
to one of six closed states.

Required fields (frozen):

- `state` — one of `created`, `mandatory_pending`, `optional_skipped`,
  `not_applicable`, `not_supported`, `refused`.
- `checkpoint_class` — one of the closed five checkpoint classes
  (`workspace_authority_checkpoint`,
  `portable_profile_rollback_checkpoint`,
  `layout_snapshot_rollback_checkpoint`,
  `evidence_packet_rollback_checkpoint`,
  `mixed_scope_rollback_checkpoint`). Even when `state` is
  `not_applicable` or `not_supported`, the row carries the class the
  gate would have used.
- `checkpoint_ref` — opaque ref to the created checkpoint. Required
  when `state` is `created`. Null in every other state.
- `rollback_path_ref` — opaque ref for the rollback path the
  checkpoint enables. Required when `state` is `created`.
- `scope_target_refs[]` — opaque refs for the scope targets the
  checkpoint covers. The set MUST cover every overwritten row whose
  `checkpoint_requirement` is `mandatory_before_overwrite`.
- `skip_acknowledgement_ref` — opaque ref to the typed
  acknowledgement the user recorded. Required when `state` is
  `optional_skipped`.
- `refused_reason` — closed enum (`user_declined`, `policy_denied`,
  `destination_unsupported`, `checkpoint_class_unsupported`).
  Required when `state` is `refused`, `not_supported`, or
  `optional_skipped`. Null otherwise.

Rules (frozen):

1. `created` is the only state that admits `confirm_overwrite`. Every
   other state disables the action with a typed reason
   (`requires_checkpoint`, `checkpoint_refused`,
   `inspect_only_outcome`, or `compatibility_range_outside`).
2. `mandatory_pending` is the default state at the moment the review
   surface opens; the user MUST advance it to `created` (by triggering
   `create_checkpoint_before_overwrite`), to `refused` (by declining),
   or to `not_applicable`/`not_supported` (when the outcome narrows
   to a class that admits no apply).
3. `optional_skipped` is admitted only when every overwritten row's
   `checkpoint_requirement` is `recommended_before_overwrite` (no
   `mandatory_before_overwrite` row is uncovered) and a typed
   acknowledgement ref accompanies the state.
4. `not_supported` narrows the destination-review outcome to
   `incompatible_schema_downgrade`, `inspect_only_imported_package`,
   or `placeholder_only_overlay` — never to `partial_restore` —
   because no apply runs over a posture the gate cannot protect.
5. `refused` requires the review surface to disable
   `confirm_overwrite` with `checkpoint_refused` and to enable
   `downgrade_to_compare_only` or `cancel_review`. A surface that
   admits an apply over a refused gate is non-conforming.
6. The checkpoint-before-overwrite gate is the only protection a
   reviewer needs to confirm an overwrite. Per-row
   `preserved_prior_artifact_ref` rows ride alongside the gate so the
   reviewer can compare class-by-class, but they do not replace the
   gate's authority over confirm.

## 8. Review-action set

The closed nine-action set is fixed.

| Action id | Consequence class | When enabled |
|---|---|---|
| `inspect` | `read_only` | always |
| `compare_with_destination` | `read_only` | always when at least one overwritten or downgraded row is present; otherwise enabled when `restore_provenance_record_ref` is non-null |
| `export_destination_before_overwrite` | `builds_export` | enabled whenever any row in `overwritten_rows[]` has `mandatory_before_overwrite` and the gate state is not yet `created`; remains enabled after `created` |
| `export_source_before_apply` | `builds_export` | enabled when the source admits export under the package's redaction class |
| `create_checkpoint_before_overwrite` | `creates_checkpoint` | enabled when `checkpoint_before_overwrite.state` is `mandatory_pending`; disabled with `destination_unsupported` when the gate state is `not_supported`; disabled with `no_overwrite_pending` when no row's treatment requires a checkpoint |
| `downgrade_to_compare_only` | `downgrades_outcome` | enabled when the outcome admits a downgrade (`partial_restore`, `placeholder_only_overlay`, or a `mandatory_pending`/`refused` gate); disabled with `no_overwrite_pending` for `exact_restore` |
| `downgrade_to_inspect_only` | `downgrades_outcome` | always enabled — a reviewer may always choose to open the package as inspect-only without applying |
| `confirm_overwrite` | `applies_overwrite` | enabled only when `checkpoint_before_overwrite.state` is `created` or every overwritten row's posture admits `not_applicable`; disabled with a typed reason in every other case |
| `cancel_review` | `cancels_without_mutation` | always |

Rules (frozen):

1. The action set is closed. A surface that mints `force_apply`,
   `merge_with_skip`, `apply_with_warnings`, or another parallel
   action is non-conforming.
2. `confirm_overwrite` MUST carry a typed `disabled_reason` from the
   shared `disabled_reason_class` set when disabled: `requires_checkpoint`,
   `checkpoint_refused`, `inspect_only_outcome`,
   `compatibility_range_outside`, `live_authority_excluded`,
   `policy_denied`, `signature_mismatch`, `destination_unsupported`,
   or `package_still_building`. Free-form reasons are non-conforming.
3. `cancel_review` MUST leave the destination workspace untouched. A
   cancel action that triggers a partial overwrite or rewrites prior
   provenance is non-conforming.
4. `create_checkpoint_before_overwrite` MUST advance the gate state to
   `created` and populate `checkpoint_ref` and `rollback_path_ref`. A
   surface that claims a checkpoint was created without populating
   the refs is non-conforming.

## 9. Cross-surface mapping

The review is reusable verbatim by every surface that explains a
restore, migration apply, imported-package open, or support handoff.

| Surface | Linkage |
|---|---|
| Backup restore review and migration apply preview | Render the closed `destination_review_outcome` label and gate `confirm_overwrite` on the checkpoint-before-overwrite state; render every retained-vs-overwritten row's `plain_language_summary` before confirm. |
| Persistence inspector and restore-provenance card | Map the review's `restore_provenance_record_ref` onto the inspector's outcome rows; row-level `compare_ref` and `export_ref` line up one-to-one with the cross-artifact handles. |
| Imported-package open dialog | Surface `inspect_only_imported_package` outcomes with `confirm_overwrite` disabled and `inspect`, `compare_with_destination`, `export_destination_before_overwrite`, and `cancel_review` enabled. |
| Support recovery handoff | Quote the review verbatim; preserved prior artifacts and checkpoint refs back the rollback path. |
| Migration handoff and managed sync | Reuse the closed destination-review outcome and treatment vocabularies in handoff manifests. |
| Docs and help flows | Reuse the title-case labels and review-action set; never invent parallel copy. |

Reviewers should be able to start from any of those surfaces and
resolve the same `review_id`, `destination_review_outcome`,
retained/overwritten row inventories, and checkpoint state.

## 10. Conformance checklist

A restore-destination review record conforms when it can answer:

- Which `source` was applied, onto which `target_scope`, and from
  which producer build and source schema version?
- Which `destination_review_outcome` does the review claim, and does
  its retained/overwritten/placeholder/downgraded/omitted row inventory
  match the conditional rules in §6?
- For every row, is the closed `treatment`, `durability_posture`,
  `checkpoint_requirement`, `treatment_reason`, and at least one
  `destination_scope_targets[]` row present, with a non-empty
  `plain_language_summary`?
- Is every row whose treatment overwrites or merges over a durable
  posture covered by a `mandatory_before_overwrite` checkpoint
  requirement and a non-null `checkpoint_ref` plus
  `preserved_prior_artifact_ref`?
- Is the checkpoint-before-overwrite gate state in `created`,
  `mandatory_pending`, `optional_skipped`, `not_applicable`,
  `not_supported`, or `refused`, and does it carry the matching
  `checkpoint_class`, `checkpoint_ref`, `rollback_path_ref`,
  `scope_target_refs[]`, `skip_acknowledgement_ref`, and
  `refused_reason`?
- Are every action in the closed nine-action set rendered with a typed
  `enabled`, `consequence_class`, and `disabled_reason`?
- For outcomes above `exact_restore`, is
  `restore_provenance_record_ref` non-null, and do `compare_ref` and
  `export_ref` resolve top-level handles for the review?

If any answer requires new vocabulary, this contract and its schemas
are extended first.

## 11. Changing this vocabulary

- **Additive-minor** changes (new reviewable class, new treatment, new
  treatment-reason class, new checkpoint class, new review action,
  new disabled reason, new scope-target kind, new target-machine
  class) land here and in the companion schemas in the same change.
  The change MUST cite the motivating fixture under
  [`/fixtures/state/restore_destination_cases/`](../../fixtures/state/restore_destination_cases/).
- **Repurposing** an existing destination-review outcome, treatment
  class, durability posture, checkpoint state, or action id is
  breaking and requires a governance decision row.
- The conditional rules in §6 and §7 stay aligned with the schema's
  `if/then` blocks. A surface that loosens a required handle without
  updating both this contract and the schemas is non-conforming.

## 12. Reference rows

- Migration-and-restore playbook — fidelity labels, downgrade
  triggers, preserved-prior-artifact rules the review quotes when it
  composes over a `state_restore_provenance_and_placeholder_record`.
- Restore-artifact-family contract — workspace-authority checkpoint
  and window-topology snapshot bodies the review references via
  `target_workspace_authority_ref` and `target_window_ids[]`.
- Restore-provenance-and-placeholder contract — the cross-artifact
  record the review composes over via `restore_provenance_record_ref`,
  including the closed missing-dependency and intentional-exclusion
  vocabularies.
- Portable-state package contract — the package manifest the source
  references; the import posture (`exact`, `compatible`, `downgraded`,
  `inspect_only`) maps verbatim onto the destination-review outcome.
- Workspace-memory contract — the excluded-live-authority floor the
  review re-exports through the cross-artifact provenance record's
  intentional-exclusion rows.
- State-object inventory and config-and-state path map — the
  authority-owner and path-class rows the
  `destination_scope_targets[]` array points at.
