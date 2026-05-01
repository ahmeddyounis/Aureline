# Checkpoint inspector, clear / export / revert controls, and scoped-deletion contract

This document freezes the cross-surface contract every diagnostics
panel, settings inspector, support-export preview, CLI text formatter,
and docs/help surface uses when it answers a single question about
restorable state outside a recovering launch:

> What does Aureline remember for this scope, what would each control
> actually touch, and which broader surface owns the effects this
> inspector deliberately does **not** execute?

Without this contract, that question collapses into one ambiguous
`Reset` button that:

- offers a single `Reset` action that silently mixes workspace-authority
  checkpoint deletion with cache eviction and profile-default rewrite;
- treats `Clear remembered state` as if it also deletes source files,
  workspace manifests, dirty-buffer journals, evidence packets, or
  unrelated profile settings;
- promises `Restore` while only window-topology snapshots are present
  and the workspace-authority checkpoint is gone;
- folds dirty-buffer journals, local-history journals, session-restore
  journals, and portable-state packages into one row labeled
  `Recovery data`;
- shows an `Export` action that writes a portable-state package without
  preserving enough provenance to explain later restore fidelity or
  omission;
- skips export-before-reset gating because the destructive action was
  scoped to "just the remembered state";
- contradicts the clear-data review sheet, support-bundle storage
  section, factory-reset checklist, and storage inspector on which
  surface actually deletes which class.

The checkpoint-inventory record is the **shared inspectable body**
every diagnostics, settings, support, and docs/help surface projects
into the same closed inventory vocabulary, the same closed scope
vocabulary, the same closed effect-breadth vocabulary, and the same
closed control vocabulary. It is **not** a backup service, **not** a
checkpoint storage backend, **not** a cleanup daemon, and **not** a
factory-reset runner. It is the contract those surfaces MUST conform
to so a user or support engineer can inspect, export, revert, or
clear remembered state mechanically — without accidentally deleting
unrelated workspace, profile, or source data, and without
masquerading a destructive action as cache cleanup.

The machine-readable schema lives at:

- [`/schemas/recovery/checkpoint_inventory.schema.json`](../../schemas/recovery/checkpoint_inventory.schema.json)
  — closed seven-class inventory vocabulary, closed five-class scope
  vocabulary, closed six-class age vocabulary, closed five-class
  effect-breadth vocabulary, closed four-class control vocabulary,
  closed five-class control-availability vocabulary, closed four-class
  linkage-target vocabulary, the disclaimer block enforcing all three
  broader-effect classes, the typed linkage refs, and the const
  honesty invariants.

Worked fixtures live under:

- [`/fixtures/recovery/checkpoint_control_cases/`](../../fixtures/recovery/checkpoint_control_cases/)

This contract composes with — and never re-defines — the recovery,
state, storage, and reliability rules frozen elsewhere:

- [`/docs/recovery/restore_chooser_contract.md`](./restore_chooser_contract.md)
  — progressive recovery-level matrix, restore-chooser state record,
  remembered-choice expiry. The inspector cites the chooser only for
  recovering launches; outside recovery, the inspector is the
  inspectable body for remembered state.
- [`/docs/state/restore_artifact_family_contract.md`](../state/restore_artifact_family_contract.md)
  — workspace-authority checkpoint and window-topology snapshot
  shapes the inventory rows reference by opaque ref.
- [`/docs/state/restore_provenance_and_placeholder_contract.md`](../state/restore_provenance_and_placeholder_contract.md)
  — restore-provenance record the inventory rows cite for the
  resulting fidelity, downgrade triggers, and missing-dependency
  placeholder explanation.
- [`/docs/state/portable_state_package_contract.md`](../state/portable_state_package_contract.md)
  — portable-state package manifest the inventory rows cite for
  exported packages, redaction class, signature state, and
  exclusions.
- [`/docs/ux/persistence_inspector_contract.md`](../ux/persistence_inspector_contract.md)
  — the broader remembered-state inspector / portable-state export
  sheet / restore-provenance card UX contract this checkpoint
  inspector specializes for the recovery family.
- [`/docs/storage/clear_data_and_low_disk_contract.md`](../storage/clear_data_and_low_disk_contract.md)
  — clear-data review sheet for cache eviction, low-disk drills, and
  workspace-storage cleanup. The inspector cites this surface as the
  contrast link when its disclaimer block names
  `affects_broader_workspace_content` or `affects_caches`; the
  inspector never executes clear-data review actions itself.
- [`/docs/reliability/export_before_reset_contract.md`](../reliability/export_before_reset_contract.md)
  — export-before-reset checklist and verification result the
  inspector cites whenever a destructive control transitions through
  `blocked_pending_export_before_reset`.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  — support-bundle record the inspector cites by opaque ref so
  destructive controls remain auditable in support evidence.
- [`/docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md)
  — chronology rules and remembered-decision waiver TTL the inspector
  reuses without redefinition.

This contract is normative for the checkpoint-inspector disclosure
posture. Where it disagrees with the PRD, TAD, TDD, UI/UX spec, or
one of the upstream contracts above, those documents win and this
contract plus its schema and fixtures MUST be updated in the same
change. Where a downstream diagnostics, settings, support-export, or
docs/help surface mints a parallel inventory, scope, effect-breadth,
or control vocabulary, this contract wins and the surface is
non-conforming.

This contract mints **no** new restore-fidelity, restore-level,
recovery-level, downgrade-trigger, redaction, export-posture,
reversibility, pre-action, recovery-class, or excluded-live-authority
value. Every closed set re-exported here is quoted by reference from
the schema that owns it.

## Why freeze this now

Cleanup-and-recovery drift starts when the moment a user wants to
"reset" remembered state collapses into one undifferentiated button.
Without an explicit family boundary:

- A diagnostics panel offers `Reset` and silently deletes the
  workspace-authority checkpoint along with the unrelated semantic-
  search cache and the user's pinned offline bundle.
- A factory-reset checklist proposes deleting "remembered state" but
  also rewrites profile defaults and clears trusted-roots state,
  obliterating non-recovery data the user never asked to drop.
- A storage cleanup pass evicts a portable-state package because the
  cache pressure ladder treats every recovery artifact as
  `disposable_derived_cache`, even when the package is the only copy.
- An export action writes a package that fails to record producer
  build, schema version, redaction class, missing dependencies, or
  resulting fidelity — so a future restore cannot explain why a
  surface reopened as placeholder or was intentionally omitted.
- A revert walks a checkpoint without an export-before-reset
  acknowledgement; the user loses dirty-buffer text the rollback did
  not cover and has no convenience export to walk back from.
- A support-bundle storage section labels every recovery artifact
  `Recovery data` and reviewers cannot tell which row is the
  authority checkpoint, which is the journal, and which is the
  evidence-only pack.

The checkpoint-inventory record forecloses these patterns by
projecting one closed seven-class inventory vocabulary, one closed
five-class scope vocabulary, one closed five-class effect-breadth
vocabulary with a const disclaimer block that always names the three
broader surfaces, one closed four-class control vocabulary, the
typed export-before-reset gate on destructive controls, and the
typed restore-provenance / portable-state-package / clear-data-review
linkage every consumer reads.

## Scope

Frozen here:

- one `inventory_item_class` closed seven-class vocabulary —
  `workspace_authority_checkpoint`, `window_topology_snapshot`,
  `dirty_buffer_recovery_journal`, `local_history_journal`,
  `session_restore_journal`, `portable_state_package`,
  `evidence_only_recovery_pack` — naming the artifact families the
  inspector exposes;
- one `scope_class` closed five-class vocabulary — `current_window`,
  `current_workspace`, `current_machine`, `profile_local`,
  `all_windows_for_workspace` — the inspector uses for its overall
  scope and for every per-control target scope;
- one `age_class` closed six-class vocabulary —
  `current_session`, `this_workspace_session`,
  `within_recent_window`, `older_than_recent_window`,
  `expired_pending_gc`, `unknown_no_timestamp` — used by inventory
  rows to disclose recency without re-exporting wall-clock time
  semantics;
- one `fidelity_label_class` re-exported from
  `/schemas/state/workspace_authority_checkpoint.schema.json#restore_class`
  so the inspector quotes Exact restore, Compatible restore, Layout
  only, Recovered drafts, Evidence only, and No restore verbatim;
- one `inventory_item_record` shape carrying the artifact ref, scope,
  age, producer build, source schema version, fidelity label,
  downgrade triggers, excluded live-authority classes, redaction /
  export posture, optional size / checksum / signature state, the
  rollback / preserved-prior-artifact refs, the restore-provenance
  ref, and the typed control ids that act on the row;
- one `control_class` closed four-class vocabulary —
  `inspect`, `export`, `revert`, `clear` — and the per-class
  effect-breadth, reversibility, evidence-retention, and linkage
  rules;
- one `effect_breadth_class` closed five-class vocabulary —
  `non_destructive_read_only`, `removes_remembered_state_only`,
  `affects_broader_workspace_content`, `affects_profile_settings`,
  `affects_caches` — where only the first two are admissible on an
  executable inspector control and the last three exist solely as
  contrast disclaimers naming broader surfaces the inspector links
  by ref;
- one `control_availability_class` closed six-class vocabulary
  including the inspector-scoped
  `blocked_pending_export_before_reset` value that disables a
  destructive control until the linked export-before-reset checklist
  resolves to verified;
- one `review_language_block` shape with one
  `removes_remembered_state_only_summary` and a const-three
  `broader_effect_disclaimers[]` that always names all three broader
  effect classes, the surface that owns them, and the link a user
  follows to actually trigger the broader effect;
- one `linkage_block` shape carrying typed refs to restore-provenance
  records, export-before-reset checklists, clear-data review records,
  portable-state package records, and a support-bundle candidate;
- one `honesty_invariants` block with const-true guarantees that the
  inventory class is closed, scope is explicit before any control
  runs, effect breadth is typed, the four control classes render
  distinctly, destructive controls link export-before-reset, clear
  and revert never reach broader workspace / profile / cache content,
  broader effects are routed by ref, and linkage is typed.

Out of scope:

- the persistence engine, sync execution, checkpoint storage layout,
  GC daemon, or restore runtime — the vocabulary freeze lands here;
  production surfaces compose over it later;
- the actual backup service or hosted restore endpoint;
- final user-facing copy / microcopy or visual layout — pinned by the
  UX style guide and shell-zone contract;
- changing the upstream `workspace_authority_checkpoint_record`,
  `window_topology_snapshot_record`, `portable_state_package_record`,
  `restore_provenance_record`, `clear_data_review_record`, or
  `export_before_reset_checklist_record` shapes;
- profile-reset and factory-reset acceptance gates — those ride the
  export-before-reset contract and the recovery-scenario card.

## 1. Record model

The inspector emits one record shape:

| Record | Purpose |
|---|---|
| `checkpoint_inventory_record` | One per inspector instance. Names the inspector's overall scope, the producer build, the typed inventory rows for restorable artifacts in scope, the typed control rows that act on those items, the review-language block separating remembered-state-only effects from broader workspace / profile / cache effects, the linkage refs to restore provenance / export-before-reset / clear-data review / portable-state package / support bundle, and the const honesty invariants. |

A given inspector instance emits at most one
`checkpoint_inventory_record`. Multiple inspector scopes (e.g. one
inspector per workspace authority across a multi-workspace
diagnostics panel) emit one record each, scoped to their authority.

## 2. Inventory-item vocabulary

Seven closed classes. Every inventoried row resolves to exactly one.
The set is re-exported into the schema's `inventory_item_class` enum.

| Class | What it covers | Default fidelity ceiling |
|---|---|---|
| `workspace_authority_checkpoint` | Authoritative restorable body for workspace authority — dirty-buffer/journal identity, trusted roots, active worksets, related evidence refs. | `exact_restore` (downgrades to `compatible_restore`, `layout_only`, `recovered_drafts`, or `evidence_only` per the upstream packet). |
| `window_topology_snapshot` | Window-local view body — pane-tree schema version, stable pane ids, tab/group topology, visible inspectors, focus chain, presentation/follow state, monitor-affinity hints. | `compatible_restore` (never claims `exact_restore`; layout-only is the floor when authority is missing). |
| `dirty_buffer_recovery_journal` | Recovery journal carrying unsaved bytes for the workspace. | `recovered_drafts` (never `exact_restore`). |
| `local_history_journal` | Local-history snapshot lane scoped to the workspace. | `recovered_drafts` or `compatible_restore`. |
| `session_restore_journal` | Session-restore manifest carrying tab/file/cursor identity for a previous session. | `compatible_restore` or `layout_only`. |
| `portable_state_package` | Portable-state package manifest — selected artifact classes, redaction, exclusions, checksum, signature, package preview. | `exact_restore` only when the package was produced from an authority checkpoint with no downgrade triggers; otherwise `compatible_restore`, `layout_only`, `recovered_drafts`, or `evidence_only`. |
| `evidence_only_recovery_pack` | Crash envelope, forensic packets, support-bundle candidate, and other inspectable / exportable evidence. | `evidence_only` or `no_restore`; never `exact_restore`, `compatible_restore`, `layout_only`, or `recovered_drafts`. |

Rules (frozen):

1. **Exactly one class per row.** A row that lists two "current"
   classes (`workspace_authority_checkpoint` AND
   `portable_state_package`) is non-conforming. Multiple rows MAY
   share the same artifact ref via the upstream restore-provenance
   record — the inspector still names each class on its own row.
2. **Class binds the fidelity ceiling.** The schema's `allOf` block
   forbids `window_topology_snapshot`, `dirty_buffer_recovery_journal`,
   `local_history_journal`, `session_restore_journal`, and
   `evidence_only_recovery_pack` from claiming `exact_restore`, and
   forbids `evidence_only_recovery_pack` from claiming anything other
   than `evidence_only` or `no_restore`.
3. **Empty inventory is non-conforming.** A workspace with nothing to
   show MUST emit one `evidence_only_recovery_pack` row whose
   `summary` names that the workspace has no remembered state — so
   the user is never told there is nothing without a named anchor.

## 3. Scope vocabulary

Five closed classes. The inspector cites exactly one for its overall
scope, and every control cites exactly one for its `target_scope`.
The set is re-exported into the schema's `scope_class` enum.

| Class | Meaning |
|---|---|
| `current_window` | The active window only. Window-topology snapshot is the typical inventory row at this scope. |
| `current_workspace` | The workspace authority and every artifact bound to it. Workspace-authority checkpoint, dirty-buffer journals, local-history journals, and session-restore journals at this scope. |
| `current_machine` | All workspaces and profile data on this device. Portable-state packages destined for the local machine, machine-local evidence-only recovery packs. |
| `profile_local` | Profile-scoped state for the current user identity (settings, profile sync snapshot, portable profile body). |
| `all_windows_for_workspace` | Every window currently open against the same workspace authority. Window-topology snapshots and per-window placeholder cards at this scope. |

Rules:

1. **Scope is explicit before any control runs.** A control whose
   `target_scope_class` is not present is non-conforming. The
   inspector renders the scope class and ref alongside the control
   label.
2. **Cross-scope rerouting is disallowed.** A control whose
   `target_scope_class` is `current_workspace` MUST NOT mutate
   `profile_local` rows or `current_machine` rows. Surfaces that
   bypass by quietly extending the deletion set are non-conforming.
3. **Scope class is preserved in evidence.** Support bundles,
   export-before-reset checklists, and restore-provenance cards
   referencing this inspector cite the scope class verbatim; the
   inspector never collapses scope into prose.

## 4. Inventory item record

Each inventory row carries the typed fields below. Surfaces that mint
parallel field names (e.g. `kind`, `family`, `tier`) are
non-conforming.

### 4.1 Required fields

| Field | Purpose |
|---|---|
| `item_id` | Stable opaque id for cross-surface linkage. |
| `item_class` | One of the seven `inventory_item_class` values (§2). |
| `item_artifact_ref` | Opaque ref into the underlying artifact (workspace_authority_ref, snapshot id, journal_id, package id, evidence-pack id). |
| `scope_class` | One of the five `scope_class` values (§3). |
| `scope_ref` | Opaque ref to the bound workspace, window, profile, or machine. |
| `age_class` | One of the six `age_class` values (§4.3). |
| `producer_build` | Producer name, version, and optional channel / platform / instance handle. |
| `source_schema_version` | Opaque schema-version string the producer used. |
| `fidelity_label_class` | One of the six `fidelity_label_class` values (§2). |
| `downgrade_triggers[]` | Typed reasons the row's fidelity narrowed. MUST be empty when `fidelity_label_class` is `exact_restore`. |
| `excluded_live_authority_classes[]` | Live-authority classes the underlying artifact intentionally left out. |
| `redaction_class` | Re-exported `redaction_class` from the restore-fidelity vocabulary. |
| `export_posture` | Re-exported `export_posture` from the restore-fidelity vocabulary. |
| `available_control_ids[]` | Opaque ids of the controls targeting this row. Every row MUST be reachable by at least one control (the `inspect` control at minimum). |
| `summary` | Short redaction-aware text restating the typed classes for the user. |

### 4.2 Optional fields

| Field | Purpose |
|---|---|
| `last_written_at` | Producer-local monotonic timestamp. Opaque to the contract; the surface never re-reads system wall-clock from this field. The `age_class` is the user-facing recency signal. |
| `size_estimate_bytes` | Optional byte estimate. Null when the producer cannot estimate without rebuilding the artifact body. |
| `checksum_state` | Optional checksum state — `unchecked`, `verified`, `mismatch`, `unsupported_destination`, or null. |
| `signature_state` | Optional signature state — `unsigned`, `signed_unverified`, `signed_verified`, `unsupported_destination`, or null. |
| `restore_provenance_ref` | Opaque ref to the matching restore-provenance record explaining what reopened live, what reopened as placeholder, and what was intentionally excluded. |
| `rollback_checkpoint_ref` | Opaque ref to the rollback-checkpoint that precedes any destructive migration the row is bound to. |
| `preserved_prior_artifact_refs[]` | Opaque refs to prior artifacts preserved before mutation. |

### 4.3 Age vocabulary

Closed six-class set. Surfaces render exactly one per row.

- `current_session` — written or rebound during the current
  session.
- `this_workspace_session` — written during the active workspace
  session but not the current restore turn.
- `within_recent_window` — within the recent-history retention
  window.
- `older_than_recent_window` — outside the recent-history retention
  window but still retained.
- `expired_pending_gc` — eligible for garbage collection in a
  future cleanup cycle; not yet removed.
- `unknown_no_timestamp` — recency cannot be derived (e.g. the
  producer schema does not carry the timestamp). Rendered as
  "Unknown" rather than as "Just now" or "Old" by inference.

## 5. Controls

The inspector exposes four control classes. The set is re-exported
into the schema's `control_class` enum and is exhaustive: a control
that is not one of these four is non-conforming.

| Class | Effect breadth | Reversibility | Required linkage |
|---|---|---|---|
| `inspect` | `non_destructive_read_only` | `exact_undo` | None beyond the row's `restore_provenance_ref`. |
| `export` | `non_destructive_read_only` | `exact_undo` | `linked_portable_state_package_ref` (the package produced). |
| `revert` | `removes_remembered_state_only` | `checkpoint_restore` or `compensating_action` | `linked_workspace_authority_checkpoint_ref` (the authority checkpoint walked); `linked_export_before_reset_ref` whenever availability transitions through `blocked_pending_export_before_reset`. |
| `clear` | `removes_remembered_state_only` | `checkpoint_restore`, `regeneration`, or `no_undo_export_only` | `linked_export_before_reset_ref` whenever availability transitions through `blocked_pending_export_before_reset`. |

Rules (frozen):

1. **No generic `Reset` button.** A control that collapses
   `inspect` / `export` / `revert` / `clear` into one button — or
   that hides the chosen class behind a single `Reset` label — is
   non-conforming. The schema enforces presence of an `inspect`
   control and the per-class effect-breadth / reversibility
   constraints.
2. **Inspect and export are non-destructive.** `inspect` and
   `export` always carry `effect_breadth_class:
   non_destructive_read_only` and `reversibility_class: exact_undo`
   and retain evidence after the action. Surfaces that imply
   inspecting or exporting deletes anything are non-conforming.
3. **Revert walks an authority checkpoint.** `revert` MUST cite a
   `linked_workspace_authority_checkpoint_ref` and resolve to
   `removes_remembered_state_only` effect breadth. A revert that
   reaches profile defaults, cache eviction, or workspace source
   bytes is non-conforming; that surface lives elsewhere.
4. **Clear removes only the row.** `clear` MUST resolve to
   `removes_remembered_state_only`. The schema forbids clear from
   carrying `affects_broader_workspace_content`,
   `affects_profile_settings`, or `affects_caches`. A surface that
   wants those broader effects routes the user by ref to the surface
   that owns them (§7).
5. **Action labels reflect the class.** The rendered confirm label
   names the class verbatim (e.g. `Export portable-state package`,
   `Revert to checkpoint`, `Clear remembered window topology`). A
   control that reads `Reset` without naming the class is
   non-conforming.
6. **Evidence retention is the default.** `evidence_retained_after_action`
   is `true` for every `inspect` and `export` control, and `true`
   for every `clear` and `revert` control whose target list does not
   include an evidence row. When the cleared row IS evidence (e.g.
   clearing an `evidence_only_recovery_pack`), the surface MUST
   disclose the loss in its summary copy and confirm-label so the
   user is not led to believe evidence persisted.

### 5.1 Availability

Closed six-class vocabulary. The inspector renders the availability
class and the user-facing reason verbatim.

- `available` — the control is enabled.
- `disabled_until_probe` — a non-blocking probe must complete first
  (e.g. checksum verify before export).
- `hidden_not_applicable` — the control does not apply to the
  inspector's scope or to the row's class.
- `blocked_by_policy` — policy forbids the action; the
  policy-narrowing rationale rides the row's
  `excluded_live_authority_classes[]` and the
  `linked_clear_data_review_ref` when applicable.
- `unavailable_missing_evidence` — the underlying artifact body or
  evidence anchor is missing; the action cannot run.
- `blocked_pending_export_before_reset` — the control is destructive
  and the linked export-before-reset checklist has not yet resolved
  to `verified`. The schema requires the control to cite a
  `linked_export_before_reset_ref` and to render `enabled: false` in
  this state.

## 6. Review-language block

Every inventory record carries one `review_language_block`. Free-text
"will affect remembered state" prose is non-conforming.

### 6.1 Required fields

- `removes_remembered_state_only_summary` — short redaction-aware
  text naming what the inspector's destructive controls actually
  touch. The schema constrains the inspector to
  `removes_remembered_state_only` for every executable destructive
  control, so this summary is the user-facing restatement.
- `broader_effect_disclaimers[]` — exactly three rows naming the
  three broader-effect classes (`affects_broader_workspace_content`,
  `affects_profile_settings`, `affects_caches`), the
  `linkage_target_class` of the surface that owns the broader
  effect, the opaque ref to that surface, and a one-line summary.
  All three rows MUST be present every time the inspector renders;
  the schema's `allOf` block enforces presence of each effect
  breadth.
- `remembered_state_definition_const` — const `true`. The surface
  uses the contract's remembered-state definition (the seven
  inventory item classes, §2) and never substitutes a private prose
  definition.
- `broader_surface_routing_const` — const `true`. Broader workspace
  content, profile settings, and caches are routed by ref to the
  surface that owns them and never executed inline by the
  checkpoint inspector.

### 6.2 Linkage targets

Closed four-class vocabulary, re-exported into the schema's
`linkage_target_class` enum:

- `clear_data_review` — paired with
  `affects_broader_workspace_content` and `affects_caches`. The user
  follows this link to the clear-data review sheet that owns those
  broader effects.
- `factory_or_profile_reset_checklist` — paired with
  `affects_profile_settings` when a full profile reset is the
  intended user goal. The user follows this link to the
  export-before-reset checklist that authorizes profile reset or
  factory reset.
- `profile_settings_inspector` — paired with
  `affects_profile_settings` when the user is editing profile
  defaults rather than removing remembered state.
- `storage_inspector` — paired with `affects_caches` when the user
  is reviewing cache pressure and storage pinning rather than
  deleting recovery state.

## 7. Linkage

Every inventory record carries one `linkage_block`. Free-form prose
linkage is non-conforming.

### 7.1 Required fields

- `restore_provenance_refs[]` — opaque refs to
  `/schemas/state/restore_provenance_record.schema.json` records
  explaining the resulting fidelity, missing dependencies,
  intentional exclusions, and rollback handles for the inventoried
  rows. Empty list is allowed when no restore has occurred against
  the inventoried rows; the inspector still emits the field so
  consumers do not invent a parallel ref shape.
- `export_before_reset_checklist_refs[]` — opaque refs to
  `/schemas/recovery/export_before_reset_checklist.schema.json`
  records gating destructive controls.
- `clear_data_review_refs[]` — opaque refs to
  `/schemas/storage/clear_data_review.schema.json` records used by
  the disclaimer block as the contrast link for
  `affects_broader_workspace_content` and `affects_caches`.
- `portable_state_package_refs[]` — opaque refs to
  `/schemas/state/portable_state_package.schema.json` records
  produced by inventory `export` controls.
- `support_bundle_candidate_ref` — opaque ref to a staged support
  bundle. Null when no bundle is staged.

### 7.2 Rules

1. **Same scope across surfaces.** A diagnostics panel, settings
   inspector, support-export preview, and docs/help example that
   cite the same `inventory_id` MUST reflect the same scope class,
   the same inventory rows, and the same control rows. A
   support-export preview that names a different scope than the
   diagnostics panel is non-conforming.
2. **Retained-evidence ids are stable.** Evidence ids cited in
   `restore_provenance_refs[]`, `portable_state_package_refs[]`,
   and `support_bundle_candidate_ref` MUST match the ids cited in
   the rows' `restore_provenance_ref` and the controls'
   `linked_portable_state_package_ref`. A record with two divergent
   ref sets is non-conforming.
3. **Export-before-reset is gated, not bypassed.** A destructive
   control whose `availability` transitions through
   `blocked_pending_export_before_reset` MUST cite a
   `linked_export_before_reset_ref`. Skipping the linkage and
   rendering `available: true` on a destructive control without a
   verified checklist is non-conforming.
4. **Clear-data review is referenced, not embedded.** The inspector
   cites a `clear_data_review_ref` only as a contrast link from the
   disclaimer block. The inspector never executes clear-data review
   actions, never re-mints `storage_class_id` rows, and never
   contradicts the clear-data review sheet's lost / rebuilt /
   retained / pinned / blocked projection.

## 8. Honesty invariants

Every inventory record MUST carry the `honesty_invariants` block
with eight const-`true` fields:

- `inventory_class_is_closed: true` — every row resolves to one of
  the seven closed classes; remembered state is never rendered as
  one opaque blob.
- `scope_is_explicit: true` — both the inspector's overall scope
  and every control's target scope are typed before any control
  runs.
- `effect_breadth_is_typed: true` — every control resolves to one
  of the five closed effect-breadth values; free-text "minor
  cleanup" is forbidden.
- `controls_distinguish_inspect_export_clear_revert: true` — the
  four control classes render distinctly. No generic `Reset`
  collapse.
- `destructive_controls_link_export_before_reset: true` — any
  control whose availability transitions through
  `blocked_pending_export_before_reset` cites a verified
  export-before-reset checklist by ref.
- `clear_actions_remove_remembered_state_only: true` — `clear` and
  `revert` controls never reach broader workspace content, profile
  settings, or caches inline; the schema enforces the effect
  breadth on every executable inspector control.
- `broader_effects_routed_by_ref: true` —
  `affects_broader_workspace_content`, `affects_profile_settings`,
  and `affects_caches` appear only in the disclaimer block as
  ref-linked contrast.
- `linkage_is_typed: true` — restore provenance, export-before-reset,
  clear-data review, portable-state package, and support-bundle
  candidate refs are typed opaque ids; free-text linkage prose is
  non-conforming.

These are const guarantees in the schema. Any surface that emits an
inventory record without them is non-conforming.

## 9. Surface rules

Apply to every surface that renders, logs, exports, or reasons about
checkpoint-inventory records.

1. **No private inventory class.** Every consumer resolves to one
   of the seven closed classes; surfaces do not render a parallel
   "Recovery data" or "Other" tier.
2. **One scope per record.** The inspector instance has exactly one
   `inspector_scope_class`; multi-scope diagnostics emit one record
   per scope.
3. **No generic `Reset` button.** The four control classes render
   distinctly with class-aware confirm labels.
4. **No claim above the row's fidelity.** A row that promises
   `exact_restore` while the underlying artifact's `restore_class`
   is `compatible_restore` is non-conforming. The inspector quotes
   the upstream fidelity verbatim.
5. **Evidence retained on inspect / export.** `inspect` and
   `export` controls always retain evidence. Surfaces that imply
   inspecting consumed evidence are non-conforming.
6. **Clear / revert are scoped to remembered state.** A `clear` or
   `revert` control whose effect breadth is anything other than
   `removes_remembered_state_only` is rejected by the schema.
7. **Broader surfaces are linked, not executed.** The disclaimer
   block always names all three broader-effect classes; the user
   follows the linked ref to actually trigger broader effects.
8. **Destructive controls are gated by export-before-reset.** A
   destructive control whose availability is
   `blocked_pending_export_before_reset` stays disabled until the
   linked checklist resolves to `verified`.
9. **Provenance is preserved on export.** Every `export` control's
   produced package carries enough provenance — producer build,
   source schema version, redaction class, included artifact
   classes, exclusions, downgrade triggers, missing dependencies —
   to explain a later restore's fidelity or omission.
10. **Support-bundle linkage stays opaque.** Raw paths, raw
    credentials, raw URLs, raw provider payloads, raw command
    lines, and raw terminal scrollback never appear in an
    inventory record.

## 10. Composition with adjacent contracts

- **Restore artifact family contract** owns the
  `workspace_authority_checkpoint_record` and
  `window_topology_snapshot_record` shapes. The inspector cites
  these by ref; it never re-derives the authority body or the pane
  tree.
- **Restore provenance and placeholder contract** owns the
  `restore_provenance_record` shape. The inspector cites
  `restore_provenance_refs[]`; restore explanation lives there.
- **Portable-state package contract** owns the
  `portable_state_package_record`. Inventory `export` controls
  produce a package by ref; the package carries its own redaction,
  signature, and exclusion fields and the inspector quotes them
  rather than redefining them.
- **Persistence inspector contract** owns the broader
  remembered-state inspector / portable-state export sheet /
  restore-provenance card UX surface. The checkpoint inspector
  specializes that family for the recovery artifacts and inherits
  the persistence-inspector invariants (no opaque blob, no
  cross-scope deletion by implication, no overstated
  verification).
- **Restore-chooser contract** owns the recovering-launch chooser
  and the five progressive recovery levels. The checkpoint
  inspector is the inspectable body for remembered state OUTSIDE a
  recovering launch; recovering launches use the chooser, not the
  inspector.
- **Clear-data review and low-disk contract** owns the cache /
  workspace storage cleanup review sheet and the low-disk banner.
  The checkpoint inspector cites this contract as the contrast
  link when its disclaimer names `affects_broader_workspace_content`
  or `affects_caches`; the inspector never executes clear-data
  review actions.
- **Export-before-reset contract** owns the destructive-action
  checklist and verification result. The inspector cites
  `linked_export_before_reset_ref` whenever a destructive control
  transitions through `blocked_pending_export_before_reset`.
- **Support-bundle contract** owns bundle records. The inspector
  cites `support_bundle_candidate_ref`; bundle bodies are not
  redefined here.
- **Governance record-state model** owns chronology rules. The
  inspector cites `last_written_at` as an opaque monotonic
  timestamp and quotes the `age_class` enum for the user-facing
  recency signal; timezone or skew rules are never re-derived.

## 11. Acceptance

- **Inventory stays distinct.** The seven `inventory_item_class`
  values render verbatim across diagnostics, settings, support-export,
  CLI text, and docs/help. No surface collapses workspace authority
  checkpoints, window-topology snapshots, journals, portable-state
  packages, and evidence-only recovery packs into one row.
- **Scope is explicit before deletion.** Every `clear` and `revert`
  control names its `target_scope_class` and `target_scope_ref`
  before it can run. The schema rejects controls that omit either
  field.
- **Effect breadth is typed.** Inspector clear and revert controls
  resolve to `removes_remembered_state_only`; the disclaimer block
  always names the three broader effects with surface-routing refs
  so users see the contrast every time.
- **Destructive controls cite export-before-reset.** A control whose
  availability transitions through
  `blocked_pending_export_before_reset` cites a verified
  export-before-reset checklist by ref. Silent bypass is
  non-conforming.
- **Exports preserve provenance.** Every `export` control yields a
  portable-state package record carrying producer build, source
  schema version, redaction class, included artifact classes,
  exclusions, downgrade triggers, and missing dependencies — enough
  for a later restore to explain its fidelity or omission.
- **Fixtures.** The fixtures under
  [`/fixtures/recovery/checkpoint_control_cases/`](../../fixtures/recovery/checkpoint_control_cases/)
  cover at least: workspace-scoped inspect of a workspace-authority
  checkpoint with full inspect / export / revert / clear control
  surface; window-scoped clear of a window-topology snapshot
  contrasted against broader profile / cache surfaces; profile-local
  export of a portable-state package preserving provenance for
  later restore; and an evidence-only recovery pack with a
  destructive clear blocked pending export-before-reset.

## 12. Changing this contract

- **Additive-minor** changes (new `inventory_item_class`, new
  `scope_class`, new `age_class`, new `effect_breadth_class`, new
  `control_class`, new `control_availability_class`, new
  `linkage_target_class`) land in this document, the schema, and at
  least one fixture in the same change. The change must cite the
  motivating fixture or packet.
- **Repurposing** an existing inventory class, scope class, effect
  breadth, control class, availability class, linkage target, or
  honesty invariant is **breaking**. It opens a new decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section here.
- The schema is the boundary. Any surface that adds a private
  field, collapses two control classes, weakens the disclaimer
  block to fewer than three rows, or relaxes the
  export-before-reset gate on a destructive control is
  non-conforming.

## 13. Source anchors

- `.t2/docs/Aureline_PRD.md` §5.25 — crash recovery and remembered
  state MUST degrade gracefully without silent loss; clear and
  reset paths name what they touch and never silently delete
  unrelated workspace, profile, or source data.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` line 2257–2296 — users
  and support can inspect, export, clear, or ignore restore state
  without deleting unrelated workspace, profile, or evidence
  state; restore summaries name how many windows / dirty buffers /
  transient tasks / notebooks / terminals / remote sessions /
  evidence packets were found before rehydration.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §9.6 / §9.7
  / Appendix CP — control-plane / data-plane separation, recovery
  posture, and the local-history / checkpoint matrix.
- `.t2/docs/Aureline_Milestones_Document.md` line 1023 — Start
  Center keeps `Open`, `Clone`, `Import`, `Restore`, and
  `Recent work` distinct; restore prompts name the recovery class,
  resulting mode, and primary actions rather than one generic
  reopen CTA.

## 14. Linked artifacts

- Checkpoint-inventory schema:
  [`/schemas/recovery/checkpoint_inventory.schema.json`](../../schemas/recovery/checkpoint_inventory.schema.json).
- Worked-example fixtures:
  [`/fixtures/recovery/checkpoint_control_cases/`](../../fixtures/recovery/checkpoint_control_cases/).
- Restore artifact family (source of truth for workspace-authority
  checkpoints and window-topology snapshots):
  [`/docs/state/restore_artifact_family_contract.md`](../state/restore_artifact_family_contract.md).
- Restore-provenance and placeholder contract (source of truth for
  resulting fidelity, missing dependencies, and intentional
  exclusion explanation):
  [`/docs/state/restore_provenance_and_placeholder_contract.md`](../state/restore_provenance_and_placeholder_contract.md).
- Portable-state package contract (source of truth for export
  manifests):
  [`/docs/state/portable_state_package_contract.md`](../state/portable_state_package_contract.md).
- Persistence inspector contract (source of truth for the broader
  remembered-state inspector / portable-state export sheet /
  restore-provenance card UX surface):
  [`/docs/ux/persistence_inspector_contract.md`](../ux/persistence_inspector_contract.md).
- Restore-chooser contract (source of truth for the recovering-launch
  chooser and the five progressive recovery levels):
  [`/docs/recovery/restore_chooser_contract.md`](./restore_chooser_contract.md).
- Clear-data review and low-disk contract (source of truth for the
  cache / workspace storage cleanup review and the low-disk banner;
  cited by the inspector as the contrast link for
  `affects_broader_workspace_content` and `affects_caches`):
  [`/docs/storage/clear_data_and_low_disk_contract.md`](../storage/clear_data_and_low_disk_contract.md).
- Export-before-reset contract (source of truth for the
  destructive-action checklist and verification result; cited by the
  inspector whenever a destructive control transitions through
  `blocked_pending_export_before_reset`):
  [`/docs/reliability/export_before_reset_contract.md`](../reliability/export_before_reset_contract.md).
- Support-bundle contract (source of truth for bundle records):
  [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md).
- Governance record-state and policy-simulation models (source of
  truth for chronology rules):
  [`/docs/governance/record_state_and_policy_simulation_models.md`](../governance/record_state_and_policy_simulation_models.md).
