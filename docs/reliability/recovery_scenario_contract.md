# Recovery-scenario card, safe-first-action matrix, and destructive-risk labeling contract

This document freezes the cross-surface contract every reliability,
support, repair, and recovery surface uses when it answers the same
two questions about a large-failure recovery posture:

1. **Which seeded scenario family is the user actually in — and what
   remains safe right now?**
2. **What is the safe first action, what destructive-risk class does
   the recommended action carry, and which repair classes are
   forbidden for this scenario?**

The recovery-scenario card is the **shared inspectable body** that
Project Doctor, the repair-transaction preview, the support-intake
scenario picker, the export-before-reset path, the recovery-ladder
packet, the CLI listing, and the evidence packet project into one
typed row a reviewer can read mechanically. It is not a diagnosis
engine, not a repair runner, and not an automated reset path. It is
the contract those surfaces MUST conform to so recovery copy stays
**scenario-coded** rather than collapsing into one generic
troubleshooting narrative.

The machine-readable schema lives at:

- [`/schemas/recovery/recovery_scenario_card.schema.json`](../../schemas/recovery/recovery_scenario_card.schema.json)

The closed scenario-family, affected-scope, safe-remainder,
first-action, verb, destructive-risk, and reversibility vocabularies
plus the safe-first-action matrix rows live at:

- [`/artifacts/recovery/safe_first_action_matrix.yaml`](../../artifacts/recovery/safe_first_action_matrix.yaml)

Worked fixtures live under:

- [`/fixtures/recovery/recovery_scenario_cases/`](../../fixtures/recovery/recovery_scenario_cases/)

This contract composes with — and never re-defines — the recovery,
support, and reliability rules frozen elsewhere:

- [`/docs/reliability/continuity_status_card_contract.md`](./continuity_status_card_contract.md)
  — continuity-status card, recovery-promise classes, restore-target
  inventory, and verification posture.
- [`/docs/reliability/local_history_contract.md`](./local_history_contract.md)
  — local-history entry, group, and clear-scope vocabulary.
- [`/docs/reliability/local_history_restore_preview_contract.md`](./local_history_restore_preview_contract.md)
  — visible snapshot card, restore preview, and retention/export card.
- [`/docs/reliability/autosave_journal_and_guided_replay_contract.md`](./autosave_journal_and_guided_replay_contract.md)
  — dirty-buffer journal, guided replay, and journal-reset.
- [`/docs/state/restore_provenance_and_placeholder_contract.md`](../state/restore_provenance_and_placeholder_contract.md)
  — restore-provenance, compatibility-restore downgrade, and
  missing-dependency placeholder card.
- [`/docs/support/project_doctor_packet.md`](../support/project_doctor_packet.md)
  — Project Doctor packet, finding catalog, and probe-family rows.
- [`/docs/support/project_doctor_probe_contract.md`](../support/project_doctor_probe_contract.md)
  — probe-family contract and finding-code rules.
- [`/docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md)
  — repair-transaction record, preview, and approved-repair vocabulary.
- [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md)
  — recovery-ladder rung packet and rollback expectation rows.
- [`/docs/support/support_intake_and_escalation_contract.md`](../support/support_intake_and_escalation_contract.md)
  — support-intake scenario picker, escalation-packet completeness, and
  delivery posture.
- [`/docs/support/object_handoff_packet.md`](../support/object_handoff_packet.md)
  — object-handoff packet and audience-scoped redaction.
- [`/docs/release/update_and_rollback_contract.md`](../release/update_and_rollback_contract.md)
  — update apply/rollback path and previous-install rollback candidate.
- [`/docs/admin/org_admin_seat_and_fleet_contract.md`](../admin/org_admin_seat_and_fleet_contract.md)
  — admin seat, entitlement, and fleet-recovery rules.
- [`/docs/state/state_object_inventory.md`](../state/state_object_inventory.md)
  — authority class, schema-evolution posture, and backup-before-
  migrate rule for every persisted state object.
- [`/docs/deployment/drill_catalog_seed.md`](../deployment/drill_catalog_seed.md)
  — continuity, disaster-recovery, and impairment drill catalog.

This contract is normative. Where it disagrees with the PRD, TAD,
TDD, UI/UX spec, or one of the upstream contracts above, those
documents win and this contract plus the schema MUST be updated in
the same change.

## Why freeze this now

Large-failure recovery is the moment when copy drift hurts the user
most. Without one frozen card per scenario family:

- a reset banner says `Try resetting your profile to fix this` while
  the actual failure is a derived workspace index — and the user
  loses profile-wide settings the rebuild never needed to touch;
- a control-plane outage is rendered with a `Reauthorize sync` button
  while the managed control-plane is unreachable, walking the user
  toward a destructive policy refresh that cannot succeed;
- a failed update offers `Rebuild caches` first while a verified
  rollback to the previous install is the actual safe path;
- a device replacement opens with `Restore from sync` while the
  authoritative backup is the only covering promise that carries
  evidence packets and unsaved workspace bytes;
- a destructive `Reset workspace` button appears before the surface
  has shown the user what remains safe and what convenience export
  exists.

The recovery-scenario card forecloses these patterns by treating each
seeded large-failure family as a separately inspectable card with a
typed safe-remainder list, a typed recommended first action, and a
typed destructive-risk note. Once the boundary is named, every
recovery surface stays scenario-coded, comparable, and exportable.

## Scope

Frozen here:

- one `recovery_scenario_card_record` shape that every reliability,
  support, repair, and recovery surface emits when it routes the
  user to a large-failure recovery path;
- the closed eight-class **scenario-family** vocabulary
  (`profile_corruption`, `workspace_index_corruption`, `failed_update`,
  `control_plane_outage`, `device_replacement`, `seat_loss`,
  `credential_store_unreadable`,
  `mirror_or_offline_bundle_unavailable`);
- the closed nine-class **affected-scope** vocabulary the card uses
  to label what the failure touches without inventing a private
  "everything broke" scope;
- the closed fourteen-class **safe-remainder** vocabulary, with at
  least one entry required on every card so the user is never told
  the system is broken without a named anchor;
- the closed twelve-class **first-action** vocabulary, the closed
  six-class **verb** vocabulary, and the bound wording rules that
  distinguish `investigate`, `export`, `checkpoint`, `rebuild`,
  `restore`, and `reset` so copy stays precise about risk and
  reversibility;
- the closed seven-class **destructive-risk** vocabulary and the
  cross-field invariant that gates
  `destructive_user_authored_no_undo_export_required` behind an
  export-before-reset pre-action;
- the **safe-first-action matrix** (one row per scenario family)
  binding preferred first-diagnosis target, approved repair classes,
  forbidden repair classes, and escalation-packet minimums to each
  family;
- typed **linkage** rules from the card into Project Doctor (probe
  family + finding-code refs), the repair-transaction preview, the
  support-intake scenario picker, and the export-before-reset path;
- **honesty invariants** — closed scenario family, named safe
  remainder, destructive action gated by export, no generic reset
  collapse, typed linkage refs.

Out of scope:

- the diagnosis engines themselves (Project Doctor probes, watcher
  scans, helper-attach probes);
- the repair runners (repair-transaction apply, restore runner,
  rollback runner);
- automated reset flows or "one-click recovery" UI;
- byte-level verification, signature schemes, or content-addressable
  storage internals;
- final UI rendering, copy localization, or visual layout of the
  recovery dashboard.

## 1. Record model

One record per (scenario family, generated-at) pair. Every recovery
surface reads exactly the fields below and no others.

| Field | Purpose |
|---|---|
| `card_id` | Stable id. Project Doctor, repair previews, support bundles, evidence packets, recovery-ladder packets, and CLI output cite it. |
| `generated_at` | Producer-local monotonic timestamp. The card never re-reads system wall-clock from this field. |
| `scenario_family_class` | One of the closed eight scenario families. Cards never collapse two families into one row. |
| `title` / `summary` | Short, redaction-aware text. Never embeds raw paths, raw provider payloads, or raw credentials. |
| `deployment_profile_scope_class` | Profile / deployment posture (`individual_local`, `self_hosted`, `air_gapped`, `managed_tenant`, `cross_plane_failover_pending`). |
| `affected_scopes[]` | Closed affected-scope classes. Bounded so the card cannot silently expand into an undocumented blast radius. |
| `safe_remainder[]` | Closed safe-remainder classes. At least one entry is required. |
| `recommended_first_action` | Action class, verb, reversibility, destructive-risk class, optional covering promise, optional pre-actions, summary. |
| `destructive_risk_note` | The honesty label for the recommended action. Risk class + reversibility + optional pre-actions + optional covering promise + summary. |
| `approved_repair_classes[]` (optional) | Repair classes the matrix permits for this family. Cards cite, never invent. |
| `forbidden_repair_classes[]` (optional) | Repair classes the matrix forbids for this family. The two sets MUST NOT intersect. |
| `linkage` | Typed refs into Project Doctor, repair-transaction preview, support intake, and export-before-reset. |
| `honesty_invariants` | Const guarantees the card cannot silently waive. |

## 2. Scenario families

Eight closed families. Every card resolves to exactly one. They
classify the *large-failure recovery posture* the user is in, not
diagnostic intake routing — the support-intake scenario family
(see `docs/support/support_intake_and_escalation_contract.md`) is
adjacent and a card MAY cite one intake row as a linkage hint, but
no card collapses two recovery families into one row.

| Family | What it means |
|---|---|
| `profile_corruption` | Profile or durable-settings store has lost integrity. Workspace bytes and evidence remain readable; profile schema, key index, or settings file is drifted/missing/hash-mismatched. |
| `workspace_index_corruption` | Knowledge cache, search index, or other derived workspace cache is corrupt. User-authored bytes are intact; only disposable derived state is affected. |
| `failed_update` | An install or update applied incompletely, leaving build-identity skew, helper skew, or update-applied-but-not-runnable state. A previous-install rollback candidate may be available. |
| `control_plane_outage` | Managed control-plane is unreachable, cross-plane failover is pending, or managed-policy fetch is failing. Cached managed policy keeps trust posture stable; local work continues. |
| `device_replacement` | A new or replacement device is hydrating from authoritative sources. Workspace, profile, evidence, and layout all need rehydration. |
| `seat_loss` | The managed-tenant entitlement has been revoked or the seat is deactivated. The user retains local-only convenience exports; cloud-bound paths are gone. |
| `credential_store_unreadable` | The OS-level credential store / keyring backend is unreadable, mismatched, or unlocked failed. Read-only diagnosis first. |
| `mirror_or_offline_bundle_unavailable` | Docs/extension/toolchain mirror or offline bundle is missing, signature-mismatched, or stale beyond policy. Workspace bytes remain readable. |

## 3. Affected scopes

Closed nine-class vocabulary. The first four mirror the
restore-target classes in
[`/artifacts/recovery/backup_checkpoint_classes.yaml`](../../artifacts/recovery/backup_checkpoint_classes.yaml);
the remaining five cover control-plane and supply-chain scopes that
are not restore targets but ARE affected by large-failure recovery.

- `workspace`
- `profile`
- `evidence`
- `layout`
- `credentials`
- `managed_control_plane`
- `update_install_chain`
- `mirror_or_offline_bundle`
- `remote_runtime_or_helper`

A card lists every affected scope; surfaces never collapse the list
into a generic "everything broke" claim. The schema bounds the array
length so a card cannot silently grow new scope classes.

## 4. Safe remainder

Closed fourteen-class vocabulary. At least one value is required on
every card so the user is never told the system is broken without a
named anchor of what remains inspectable, exportable, or operable.

- `workspace_bytes_local` — user-authored workspace bytes on local disk.
- `local_history_lane` — local-history lane is healthy and capturing.
- `autosave_journal` — autosave / dirty-buffer journal is intact.
- `authoritative_backup_local` — verified authoritative backup is
  reachable on this device or local-secondary storage.
- `convenience_export_local` — a recent convenience export
  (portable-state package, support bundle, patch export, evidence
  packet copy) is on disk.
- `mirror_cache_bytes_readable` — signed mirror cache is readable.
- `offline_bundle_readable` — offline-bundle artifact is readable.
- `profile_settings_readable` — profile-wide durable settings are
  inspectable even when other recovery sources are not.
- `evidence_packets_readable` — evidence packets remain readable.
- `layout_recoverable_from_checkpoint` — window topology can be
  restored from the latest local-history checkpoint.
- `credentials_intact` — credential store is unlocked and operable.
- `managed_policy_cached_locally` — most-recent managed policy is
  cached locally; trust posture is stable.
- `previous_install_rollback_available` — the previous install is on
  disk as a rollback candidate.
- `remote_session_resumable` — a remote runtime / helper session can
  be resumed without re-attach.

## 5. First-action vocabulary

Closed twelve-class vocabulary. Every card recommends exactly one
first action. The matrix (§9) constrains which actions are allowed
for which family; `continue_local_work` is permitted only when the
scenario does not threaten the workspace, profile, evidence, or
layout the user is currently in.

- `investigate_with_project_doctor`
- `run_repair_transaction_preview`
- `export_now_before_change`
- `capture_local_checkpoint`
- `import_offline_bundle`
- `reauthorize_sync_after_review`
- `rebuild_disposable_state_only`
- `restore_from_authoritative_backup`
- `restore_from_sync_replica`
- `escalate_to_support_with_evidence`
- `defer_to_admin_seat_recovery`
- `continue_local_work`

## 6. Verb vocabulary and wording rules

Six verbs. Each binds one user-visible meaning, one allowed
reversibility range, and one allowed destructive-risk range so a
card never says "reset" when it means "rebuild", or "restore" when
it means "investigate".

| Verb | Meaning (short) | Allowed reversibility | Allowed destructive risk |
|---|---|---|---|
| `investigate` | Read-only diagnostic. Reads logs, runs probes, renders evidence; never mutates state. | `exact_undo` | `non_destructive_read_only`, `non_destructive_writes_local_evidence_only` |
| `export` | Write a local-only convenience export. Never changes authoritative state; never silently leaves the device. | `exact_undo`, `compensating_action` | `non_destructive_writes_local_evidence_only` |
| `checkpoint` | Capture a local-history / autosave / workspace-authority checkpoint a later step can reverse. Required before any rebuild/restore/reset of workspace bytes or layout. | `exact_undo`, `checkpoint_restore` | `non_destructive_writes_local_evidence_only`, `writes_user_owned_recovery_state` |
| `rebuild` | Recreate disposable derived state (knowledge cache, artifact cache, prebuild env, search index) from authoritative sources. Never touches user-authored durable state, profile-wide settings, evidence, or credentials. | `regeneration` | `writes_disposable_state_only` |
| `restore` | Bring a target back from `authoritative_backup`, `sync_replica`, or `local_checkpoint`. MUST cite exactly one covering recovery-promise class. `mirror_cache` and `convenience_export` MUST NOT cover a restore. | `checkpoint_restore`, `compensating_action` | `mutates_workspace_bytes_with_checkpoint`, `mutates_profile_state_with_checkpoint_and_export` |
| `reset` | Destroy or re-initialize durable state without an authoritative restore source. MUST NOT run before an export and a checkpoint pre-action are listed (or the no-undo-export-only path has been explicitly authorized). | `no_undo_export_only` | `destructive_user_authored_no_undo_export_required` |

The full wording rule body — including each verb's
`forbidden_combinations` set — is re-exported verbatim in
[`/artifacts/recovery/safe_first_action_matrix.yaml`](../../artifacts/recovery/safe_first_action_matrix.yaml)`#wording_rules`.
A surface that says "reset" when it means "rebuild" — or "restore"
when it means "investigate" — is non-conforming.

## 7. Destructive-risk classes

Closed seven-class vocabulary. Every recommended action carries one
class so the surface never collapses "rebuild a cache" and "reset
the profile" into one generic destructive label.

- `non_destructive_read_only` — pure read; no writes anywhere.
- `non_destructive_writes_local_evidence_only` — writes only the
  local evidence lane (audit, evidence packet, support-bundle index).
- `writes_disposable_state_only` — recreates derived disposable state
  (caches, indexes, prebuilds). Never touches durable state.
- `writes_user_owned_recovery_state` — writes only into local-history
  / autosave / workspace-authority checkpoint state owned by the user.
- `mutates_workspace_bytes_with_checkpoint` — restores workspace bytes;
  a workspace-authority checkpoint MUST precede the action so it
  remains reversible.
- `mutates_profile_state_with_checkpoint_and_export` — restores
  profile-wide durable state; a checkpoint AND an export-before-change
  pre-action MUST precede the action.
- `destructive_user_authored_no_undo_export_required` — destroys
  durable user-authored state. Reset-only verb. Export pre-action is
  required; no-undo acknowledgement is required.

The schema enforces:

1. Only the verb `reset` may carry
   `destructive_user_authored_no_undo_export_required`.
2. Only `rebuild_disposable_state_only` may carry
   `writes_disposable_state_only`.
3. Read-only `investigate` MUST carry one of the two non-destructive
   read classes.
4. A card whose `destructive_risk_note.risk_class` is
   `destructive_user_authored_no_undo_export_required` MUST set
   `no_undo_acknowledgement_required: true`, list
   `export_now_before_change` in `requires_pre_action[]`, and set
   `linkage.export_before_reset.export_required: true` with a typed
   `export_artifact_ref` and `redaction_class`.

## 8. Recommended first action

The card's `recommended_first_action` block is the single typed
projection every surface reads:

- `action_class` — one of the twelve first-action classes.
- `verb_class` — one of the six verbs.
- `reversibility_class` — one of the five reversibility classes
  (re-exported from
  `schemas/support/recovery_action.schema.json#reversal_class`).
- `destructive_risk_class` — one of the seven destructive-risk
  classes.
- `covering_promise_class` (required when the action is
  `restore_from_authoritative_backup` or
  `restore_from_sync_replica`). MUST be drawn from
  `authoritative_backup`, `sync_replica`, or `local_checkpoint`;
  `mirror_cache` and `convenience_export` are rejected by the schema.
- `must_precede[]` (optional, required for `verb_class: reset`) —
  pre-actions the user MUST complete first. Drawn from
  `export_now_before_change`, `capture_local_checkpoint`,
  `import_offline_bundle`, `investigate_with_project_doctor`.
- `summary` — short, redaction-aware text.

## 9. Safe-first-action matrix

One row per scenario family. Each row binds:

- `preferred_first_diagnosis_target` — `probe_family_class` plus a
  finding-code-pattern summary the surface uses to find the right
  Project Doctor probe.
- `recommended_first_action_class` — the matrix's recommended action
  for the family (the card MAY refine when fixture context demands,
  but MUST stay inside the family's allowed set).
- `approved_repair_classes[]` — repair classes the matrix permits.
- `forbidden_repair_classes[]` — repair classes the matrix forbids.
  The card's `linkage.repair_transaction_preview.approved_repair_class`
  MUST NOT appear in this set.
- `forbidden_first_actions[]` — first-action classes that are
  explicitly disallowed for the family (for example, restoring from a
  sync replica during seat loss).
- `escalation_packet_minimum` — minimum escalation-packet shape:
  required finding-code pattern, required evidence-class floor,
  whether a `repair_transaction_ref` is required when a repair is
  proposed, whether a `pre_action_checkpoint_ref` is required,
  whether a `pre_action_export_ref` is required, and a completeness
  outcome floor (`complete`, `complete_with_typed_unknowns`, or
  `incomplete_refused_export`).
- `rationale` — short prose explaining why this is the safe row.

The matrix rows live in
[`/artifacts/recovery/safe_first_action_matrix.yaml`](../../artifacts/recovery/safe_first_action_matrix.yaml)`#safe_first_action_rows`
and are re-exported verbatim into the schema's enum-typed allowed
sets. Adding a row is additive-minor and must bump the matrix
`schema_version`; repurposing a row is breaking and requires a new
decision row in `artifacts/governance/decision_index.yaml`.

## 10. Linkage rules

Every card MUST cite typed linkage refs. Free-form prose linkage is
non-conforming.

### 10.1 Project Doctor

`linkage.project_doctor` carries:

- `probe_family_class` — drawn from the closed
  `probe_family_vocabulary` re-exported from
  `fixtures/support/scenario_matrix.yaml`.
- `doctor_finding_code_refs[]` (optional) — opaque finding-code
  refs the surface should consult first.
- `scenario_matrix_row_ref` (optional) — opaque ref to the matching
  matrix row.
- `summary` — short, redaction-aware text.

A card MUST cite one probe family. Free-text "look at logs" linkage
is non-conforming.

### 10.2 Repair-transaction preview

`linkage.repair_transaction_preview` carries:

- `preview_required` — boolean. When `true`, the card recommends a
  repair the user can apply; the repair-transaction contract requires
  a preview record before any apply.
- `repair_transaction_ref`, `repair_preview_ref` (optional) —
  opaque refs.
- `approved_repair_class` (optional) — one of the closed
  approved-repair classes from
  `schemas/support/scenario_picker.schema.json#approved_repair_class`.
  MUST NOT appear in the matching matrix row's
  `forbidden_repair_classes[]`.
- `summary` — short, redaction-aware text.

The repair-transaction preview is the only path through which a card
proposes a repair. Free-text repair recommendations are non-conforming.

### 10.3 Support intake

`linkage.support_intake` carries:

- `scenario_picker_row_ref` — opaque ref to the support-intake
  scenario-picker row the card composes with.
- `escalation_packet_ref` (optional) — opaque ref.
- `object_handoff_packet_ref` (optional) — opaque ref.
- `completeness_outcome_floor` (optional) — drawn from the closed
  `completeness_outcome_class` vocabulary
  (`complete`, `complete_with_typed_unknowns`,
  `incomplete_refused_export`). MUST be at least the matrix-row
  floor.
- `summary` — short, redaction-aware text.

### 10.4 Export-before-reset

`linkage.export_before_reset` carries:

- `export_required` — boolean. MUST be `true` when the
  recommended action carries
  `destructive_user_authored_no_undo_export_required` risk OR when
  the matrix row's `pre_action_export_ref` requirement is set.
- `export_artifact_ref` (required when `export_required: true`) —
  opaque ref to the convenience export.
- `redaction_class` (required when `export_required: true`) — drawn
  from the closed `data_class_boundary_class` vocabulary.
- `local_only_by_default` — const `true`. Convenience exports never
  silently leave the device.
- `summary` — short, redaction-aware text.

### 10.5 Continuity-status, recovery-action, and matrix-row refs

Optional but recommended:

- `linkage.continuity_status_card_ref` — opaque ref to the
  `continuity_status_card_record` the scenario card composes with.
- `linkage.recovery_action_record_ref` — opaque ref to the
  `recovery_action_record` the scenario card composes with.
- `linkage.matrix_row_ref` — opaque ref to the matrix row in
  `artifacts/recovery/safe_first_action_matrix.yaml#safe_first_action_rows`
  that governs this family.

## 11. Honesty invariants

Every card MUST carry the `honesty_invariants` block with five
const-`true` fields:

- `scenario_family_is_closed: true` — the card resolves to exactly
  one closed scenario family and never collapses two families into
  one row.
- `safe_remainder_named: true` — the card lists at least one
  `safe_remainder` value; the user is never told the system is
  broken without a named anchor.
- `destructive_action_after_export_only: true` — any
  `destructive_user_authored_no_undo_export_required` action is
  gated behind an `export_now_before_change` pre-action.
- `no_generic_reset_collapse: true` — the card never proposes a
  generic reset; reset is verb-coded with a specific scope and
  pre-action set.
- `linkage_is_typed: true` — linkage refs are typed (probe family,
  repair-transaction ref, scenario-picker row ref, export artifact
  ref) rather than free-form prose.

These are const guarantees in the schema. Any surface that emits a
card without them is non-conforming.

## 12. Surface rules

Apply to every surface that renders, logs, exports, or reasons about
recovery-scenario records.

1. **No surface invents a private scenario family.** Every consumer
   resolves to one of the closed eight families; surfaces do not
   render a parallel "generic reset" or "broken state" family.
2. **One verb per recommended action.** Cards do not say "reset and
   rebuild" or "restore and reset" in one breath. If two verbs are
   needed, two cards or one card plus a typed pre-action is the
   shape; free-form chained verbs are non-conforming.
3. **No destructive action before safe remainder is shown.** A
   destructive recommendation MUST be preceded — in card layout
   order — by the `safe_remainder` row and the export-before-reset
   linkage. The schema enforces this for the no-undo class.
4. **Restore requires an authoritative covering promise.** A card
   whose first action is `restore_from_authoritative_backup` or
   `restore_from_sync_replica` MUST cite `authoritative_backup`,
   `sync_replica`, or `local_checkpoint` as `covering_promise_class`.
   `mirror_cache` and `convenience_export` are rejected by the
   schema.
5. **Rebuild is disposable-state only.** A card whose first action
   is `rebuild_disposable_state_only` MUST NOT name `workspace`,
   `profile`, `evidence`, `layout`, `credentials`, or
   `managed_control_plane` as the rebuild target. Rebuild is
   regeneration, never restoration.
6. **Reset is gated.** The verb `reset` MUST NOT appear in any card
   before an export and a checkpoint pre-action are listed. Reset is
   the only verb permitted to carry the
   `destructive_user_authored_no_undo_export_required` class.
7. **Forbidden repair classes are enforced.** A card MUST NOT cite
   a repair transaction whose `approved_repair_class` appears in
   the matching matrix row's `forbidden_repair_classes[]`. Surfaces
   that bypass this rule by emitting a free-text repair are
   non-conforming.
8. **Control-plane outage forbids destructive first actions.** Cards
   in the `control_plane_outage` family MUST NOT recommend any first
   action whose verb is `reset`, `restore`, or `rebuild` while the
   managed control-plane is unreachable.
9. **One scenario per card.** Two scenarios on one device emit two
   cards; a card never silently mixes families.
10. **Linkage stays typed.** Every card cites typed refs for Project
    Doctor, repair-transaction preview, support intake, and the
    export-before-reset path. Free-text linkage prose is
    non-conforming.

## 13. Composition with adjacent contracts

- The continuity-status card describes recoverability *posture*;
  the recovery-scenario card describes recovery *guidance for one
  large failure*. The scenario card cites the continuity-status card
  by `continuity_status_card_ref`; it never re-derives recovery-
  promise truth.
- The local-history contract owns the local-history lane. The
  scenario card cites `local_history_lane` and
  `layout_recoverable_from_checkpoint` as safe-remainder anchors but
  never re-defines local-history vocabulary.
- The autosave-journal contract owns dirty-buffer recovery. The
  scenario card cites `autosave_journal` as a safe-remainder anchor
  but never re-creates the journal vocabulary.
- The Project Doctor packet contract owns finding codes and probe
  families. The scenario card cites a `probe_family_class` and
  optional `doctor_finding_code_refs[]`; it never invents a parallel
  finding taxonomy.
- The repair-transaction contract owns repair-preview, apply, and
  rollback. The scenario card cites a `repair_transaction_ref` /
  `repair_preview_ref`; it never carries repair payload bodies.
- The support-intake / escalation contract owns scenario picker and
  escalation-packet completeness. The scenario card cites a
  `scenario_picker_row_ref` and a completeness floor; it never
  re-defines packet shape.
- The recovery-ladder packet contract owns the rung sequence. The
  scenario card composes with one or more rungs by way of the
  `recovery_action_record_ref`; it never replaces the ladder.
- The update-and-rollback contract owns previous-install rollback.
  The `failed_update` family cites `previous_install_rollback_available`
  as a safe-remainder anchor; the scenario card never re-derives
  rollback eligibility.
- The admin seat / fleet contract owns seat lifecycle. The
  `seat_loss` family cites `defer_to_admin_seat_recovery` only when
  the matrix row authorizes it; the scenario card never re-derives
  entitlement state.
- The drill-catalog seed owns recovery drill rows. Each drill row
  exercises one scenario family; the scenario card and the drill
  outcome compare against each other without translation.

## 14. Acceptance

- Recovery guidance is **scenario-coded**, not generic troubleshooting
  prose. The eight closed families plus the matrix rows force every
  surface to resolve to a specific recovery posture.
- Destructive actions never appear before the user can see what
  remains safe and what evidence/export path exists. The schema
  rejects `destructive_user_authored_no_undo_export_required` without
  `export_now_before_change` as a required pre-action and a typed
  export artifact ref.
- The fixtures under
  [`/fixtures/recovery/recovery_scenario_cases/`](../../fixtures/recovery/recovery_scenario_cases/)
  cover at least: profile corruption, failed update, device
  replacement, and control-plane outage; together they exercise the
  read-only investigate verb, the rebuild verb, the restore verb,
  the export-then-rollback path, and the
  `continue_local_work` first action.

## 15. Changing this vocabulary

- **Additive-minor** changes (new scenario family, new affected
  scope, new safe-remainder class, new first-action class, new
  destructive-risk class, new safe-first-action matrix row) land in
  this document, the schema, the matrix yaml, and the fixtures in
  the same change. The change must cite the motivating fixture or
  packet.
- **Repurposing** a scenario family, verb, destructive-risk class,
  or honesty invariant is **breaking**. It opens a new decision row
  and supersedes the relevant section of this document.
- The schema is the boundary. Any surface that adds a private field,
  collapses two families, or emits a card without the
  `honesty_invariants` block is non-conforming.

## Source anchors

- `.t2/docs/Aureline_PRD.md` §5.24, §5.53 — recoverability and
  continuity claims; large-failure guidance must be scenario-coded.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §9.6, §9.7,
  Appendix CP — control-plane / data-plane separation, recovery
  posture, and the local-history / checkpoint matrix.
- `.t2/docs/Aureline_Technical_Design_Document.md` — Project Doctor,
  repair-transaction, and support-intake component designs.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §15 — recovery and
  restore preview surfaces; destructive actions follow safe-remainder
  and export-before-reset rendering.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` §1202 — session
  restore, autosave, and crash-loop recovery preserve unsaved local
  content; copy stays scenario-coded.

## Linked artifacts

- Schema:
  [`schemas/recovery/recovery_scenario_card.schema.json`](../../schemas/recovery/recovery_scenario_card.schema.json).
- Closed-vocabulary catalog and safe-first-action matrix:
  [`artifacts/recovery/safe_first_action_matrix.yaml`](../../artifacts/recovery/safe_first_action_matrix.yaml).
- Worked-example fixtures:
  [`fixtures/recovery/recovery_scenario_cases/`](../../fixtures/recovery/recovery_scenario_cases/).
- Continuity-status card contract:
  [`docs/reliability/continuity_status_card_contract.md`](./continuity_status_card_contract.md).
- Local-history contract:
  [`docs/reliability/local_history_contract.md`](./local_history_contract.md).
- Restore-preview contract:
  [`docs/reliability/local_history_restore_preview_contract.md`](./local_history_restore_preview_contract.md).
- Autosave-journal contract:
  [`docs/reliability/autosave_journal_and_guided_replay_contract.md`](./autosave_journal_and_guided_replay_contract.md).
- Project Doctor packet contract:
  [`docs/support/project_doctor_packet.md`](../support/project_doctor_packet.md).
- Repair-transaction contract:
  [`docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md).
- Recovery-ladder packet contract:
  [`docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md).
- Support-intake / escalation contract:
  [`docs/support/support_intake_and_escalation_contract.md`](../support/support_intake_and_escalation_contract.md).
- Object-handoff packet contract:
  [`docs/support/object_handoff_packet.md`](../support/object_handoff_packet.md).
- Update-and-rollback contract:
  [`docs/release/update_and_rollback_contract.md`](../release/update_and_rollback_contract.md).
- Admin seat / fleet contract:
  [`docs/admin/org_admin_seat_and_fleet_contract.md`](../admin/org_admin_seat_and_fleet_contract.md).
- State-object inventory:
  [`docs/state/state_object_inventory.md`](../state/state_object_inventory.md).
- Drill catalog seed:
  [`docs/deployment/drill_catalog_seed.md`](../deployment/drill_catalog_seed.md).
