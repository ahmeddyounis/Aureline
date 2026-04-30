# Continuity-status card, backup/checkpoint promise class, and restore-target inventory contract

This document freezes the cross-surface contract every reliability,
support, repair, migration, and recovery surface uses when it answers
three questions before a destructive repair, migration, or restore
path runs:

1. **How recoverable is the current workspace and profile right now?**
2. **Which restore sources are authoritative versus merely
   convenient?**
3. **Which restore targets — workspace, profile, evidence, layout —
   stand on their own, and which need an explicit local-safe action
   before any change?**

The card is the **shared inspectable body** that backup, checkpoint,
sync, mirror, and export sources project into one row a reviewer can
read mechanically. It is not a backup engine, a sync engine, or a
restore runner; it is the contract those surfaces MUST conform to so
recoverability claims do not blur into one ambiguous "your data is
safe" promise.

The machine-readable schema lives at:

- [`/schemas/recovery/continuity_status_card.schema.json`](../../schemas/recovery/continuity_status_card.schema.json)

The closed recovery-promise, restore-target, and local-safe guidance
vocabularies live at:

- [`/artifacts/recovery/backup_checkpoint_classes.yaml`](../../artifacts/recovery/backup_checkpoint_classes.yaml)

Worked fixtures live under:

- [`/fixtures/recovery/continuity_status_cases/`](../../fixtures/recovery/continuity_status_cases/)

This contract composes with — and never re-defines — the recovery
storage and lineage rules frozen elsewhere:

- [`/docs/reliability/local_history_contract.md`](./local_history_contract.md)
  — local-history entry, group, and clear-scope vocabulary.
- [`/docs/reliability/local_history_restore_preview_contract.md`](./local_history_restore_preview_contract.md)
  — visible snapshot card, restore preview, and retention/export card.
- [`/docs/reliability/autosave_journal_and_guided_replay_contract.md`](./autosave_journal_and_guided_replay_contract.md)
  — dirty-buffer journal, guided replay, and journal-reset.
- [`/docs/state/restore_provenance_and_placeholder_contract.md`](../state/restore_provenance_and_placeholder_contract.md)
  — restore-provenance, compatibility-restore downgrade, and
  missing-dependency placeholder card.
- [`/docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md)
  — storage classes, pin sources, and the protected-class invariants.
- [`/docs/state/state_object_inventory.md`](../state/state_object_inventory.md)
  — authority class, schema-evolution posture, and backup-before-
  migrate rule for every persisted state object.
- [`/docs/deployment/drill_catalog_seed.md`](../deployment/drill_catalog_seed.md)
  — continuity, disaster-recovery, and impairment drill catalog.

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, or one of the upstream recovery contracts above, those
documents win and this contract plus the schema MUST be updated in the
same change.

## Why freeze this now

Recoverability drift starts when each surface invents its own way of
saying "you have a backup". Without one frozen card:

- a startup banner says `Backup ready` while a support tool quotes a
  `Mirror snapshot available` and a settings page says `Sync caught
  up` — and none of the three statements are interchangeable;
- a "restore" action replays a mirror cache and silently demotes the
  user's evidence packets that the mirror never carried;
- a stale sync replica is rendered as authoritative because nothing
  on the surface tells the user the replica lag has crossed the
  policy window;
- a partial restore lands on a generic "your workspace is restored"
  banner while the profile-wide settings, evidence packets, or window
  layout were silently dropped or rebuilt from defaults;
- ordinary cache-clear flows evict mirror caches and the surface
  fails to distinguish that from clearing local-history entries —
  blurring user-owned recovery state into a disposable cache class.

The card forecloses these patterns by treating five recovery promise
classes — `authoritative_backup`, `local_checkpoint`, `sync_replica`,
`mirror_cache`, `convenience_export` — as separately inspectable
sources, and four restore-target classes — `workspace`, `profile`,
`evidence`, `layout` — as separately restorable rows. Once the
boundary is named, every recoverability claim stays explicit,
comparable, and exportable.

## Scope

Frozen here:

- one `continuity_status_card_record` shape that every reliability,
  support, repair, migration, and recovery surface emits before a
  destructive write or migration;
- the closed five-class recovery-promise vocabulary
  (`authoritative_backup`, `local_checkpoint`, `sync_replica`,
  `mirror_cache`, `convenience_export`) and what each class can and
  cannot safely replace;
- the closed four-class restore-target inventory vocabulary
  (`workspace`, `profile`, `evidence`, `layout`) so workspace bytes,
  profile-scoped state, evidence packets, and window topology stay
  separately inspectable;
- the closed `restorability_state` vocabulary
  (`ready`, `stale`, `unverified`, `missing`, `partially_restorable`)
  with required local-safe guidance on every non-`ready` row;
- closed verification, sync/replication, export-availability, and
  local-history health vocabularies so cards never carry private
  "ok"/"degraded" labels;
- the deployment / profile scope axis so a card always answers
  "recoverable for which profile under which deployment posture";
- honesty invariants — no promise blurring, no convenience export as
  authoritative, no ordinary-cache-clear demotion of recovery state,
  no missing target without a local-safe next action.

Out of scope:

- the backup engine, sync engine, mirror replicator, or restore
  runner. The card is a projection over their state, not a control
  plane for them;
- final UI rendering, copy localization, or visual layout of the
  status strip / recovery dashboard;
- byte-level verification, signature schemes, or content-addressable
  storage internals;
- long-term storage GC algorithms (those compose with the storage-
  class contract under the protected-class invariants).

## 1. Record model

One record per (deployment scope, generated-at) pair. Every
continuity surface reads exactly the fields below and no others.

| Field | Purpose |
|---|---|
| `card_id` | Stable id. Status strips, support bundles, evidence packets, recovery ladder packets, and CLI output cite it. |
| `generated_at` | Producer-local monotonic timestamp. The card never re-reads system wall-clock from this field. |
| `deployment_scope` | Profile / workspace / device / tenant scope the card describes. One scope per card. |
| `last_backup` | Most recent authoritative backup the surface knows about. See §3. |
| `last_checkpoint` | Most recent local-history or autosave checkpoint. See §3. |
| `local_history_health` | Health, retention pressure, and ordinary-cache-clear posture of the local-history lane. See §4. |
| `sync_or_replication_state` | Profile-sync / replication state, lag, and policy-window posture. See §5. |
| `export_availability` | Convenience-export availability, locality, and redaction class. See §6. |
| `restore_targets[]` | One row per restore-target class. See §7. |
| `honesty_invariants` | Const guarantees the card cannot silently waive. See §8. |

The `restore_targets[]` array is **bounded to four entries** so the
card cannot silently grow new target classes; adding a target class is
additive-minor and lands here, in
[`/artifacts/recovery/backup_checkpoint_classes.yaml`](../../artifacts/recovery/backup_checkpoint_classes.yaml),
and in the schema in the same change.

## 2. Recovery promise classes

Five closed promise classes. Every backup, checkpoint, sync, mirror,
and export source the card surfaces resolves to exactly one. Every
restore-target row cites exactly one promise class as its covering
promise; mirror caches and convenience exports are **never**
authoritative.

| Class | Can replace | Cannot replace | Notes |
|---|---|---|---|
| `authoritative_backup` | workspace, profile, evidence, layout | — | Signed, verified, content-addressable. Verification time is mandatory; an unverified backup demotes to `local_checkpoint`-equivalent semantics until verified. |
| `local_checkpoint` | workspace, layout | profile, evidence | Local-history checkpoints, autosave journals, dirty-buffer sentinels. Authoritative for in-flight workspace state and window layout but never a substitute for profile-wide durable truth or retained evidence. |
| `sync_replica` | profile, layout | workspace, evidence | Opt-in profile sync replica. May rehydrate a profile after device loss; not a substitute for unsaved workspace bytes; never carries evidence bodies. |
| `mirror_cache` | — | workspace, profile, evidence, layout | Signed mirror or offline-bundle cache of upstream artifacts. Lets the workspace continue to read known-good bytes; holds no user-authored truth; **never an authoritative restore source**. |
| `convenience_export` | — | workspace, profile, evidence, layout | Portable-state package, support bundle, patch export, evidence packet copy. Useful for off-device import and audit; **never authoritative on its own**. |

The full row contract — required verification states, typical storage
classes, and the rationale behind each "cannot replace" entry — lives
in
[`/artifacts/recovery/backup_checkpoint_classes.yaml`](../../artifacts/recovery/backup_checkpoint_classes.yaml).

## 3. Last backup and last checkpoint

`last_backup` and `last_checkpoint` describe the two most recent
authoritative recovery sources the surface knows about, on separate
rows.

`last_backup`:

- `promise_class` — MUST be `authoritative_backup`. Rendering a sync
  replica, mirror cache, or convenience export as the "last backup"
  is a contract violation.
- `presence_class` — closed:
  `absent_no_source`, `present_authoritative`, `present_unverified`,
  `present_stale`, or `scheduled_not_yet_run`.
- `last_backup_at` — monotonic timestamp of the most recent backup
  capture.
- `verification` — verification posture (state, last verified at,
  verifier ref, evidence ref, summary).

`last_checkpoint`:

- `promise_class` — fixed to `local_checkpoint` by the contract.
  Sync replicas and backups have their own rows.
- `presence_class` — closed:
  `absent_no_source`, `present_recent`, `present_stale`, or
  `present_evicted_stub_only`.
- `last_checkpoint_at` — monotonic timestamp of the most recent
  capture.
- `verification` — verification posture (typically `verified_recent`
  for content-addressed checkpoints; `verification_unavailable` for
  stub-only entries).

A card MAY carry both, one, or neither. When neither is present, the
restore-target inventory MUST flag affected rows as `missing` or
`partially_restorable` with explicit local-safe guidance.

## 4. Local-history health

`local_history_health` mirrors the local-history contract:

- `health_class` — closed: `healthy`, `retention_pressure`,
  `quota_exhausted`, `storage_unavailable`, `never_initialized`, or
  `cleared_by_user`.
- `last_capture_at` — monotonic timestamp of the most recent local-
  history entry.
- `retained_entries_count` — non-negative integer of entries still on
  the timeline (including stubs).
- `eviction_pressure_class` — `no_pressure`, `approaching_window`,
  `evicting_oldest_first`, or `stub_only_after_expiry`.
- `ordinary_cache_clear_excluded` — const `true`. Mirrors the
  local-history contract §10 invariant; ordinary cache-clear flows
  never alter local-history entries.
- `summary` — short, redaction-aware text. Never embeds raw paths,
  raw absolute file references, raw provider payloads, or raw
  credentials.

## 5. Sync or replication state

`sync_or_replication_state` is the profile-sync / replication posture
the card surfaces. Closed `state_class` vocabulary:

- `not_configured` — the user never opted into sync.
- `in_sync` — replica matches authority within a freshness window.
- `lagging_within_window` — replica is behind but inside the policy-
  declared lag window.
- `stale_outside_window` — replica lag has crossed the policy window;
  the replica MUST NOT be cited as the authoritative covering promise
  for any restore target.
- `paused_by_user` — user-paused.
- `paused_by_policy` — policy/admin-paused.
- `divergent_requires_review` — replica diverged; manual review is
  required before further sync. Like `stale_outside_window`, the
  replica is non-authoritative until reviewed.
- `sync_unavailable` — sync subsystem is unavailable on this device.

`lag_within_policy_window` is a boolean projection of the same truth
so a surface can answer "is the replica still trustworthy" without
re-deriving the policy window.

## 6. Export availability

`export_availability` describes the convenience-export posture:

- `availability_class` — closed: `export_on_disk_recent`,
  `export_on_disk_stale`, `export_offered_not_taken`,
  `export_in_progress`, `export_blocked_by_policy`,
  `export_not_offered_metadata_only`, or `export_unavailable`.
- `last_export_at` — monotonic timestamp.
- `export_artifact_refs[]` — opaque references to portable-state
  packages, support bundles, patch exports, or evidence packet copies.
- `redaction_class` — `metadata_only`, `environment_adjacent`,
  `code_adjacent`, or `high_risk` (mirrors the mutation-journal
  vocabulary).
- `local_only_by_default` — const `true`. Convenience exports never
  silently leave the device.

A convenience export MAY be surfaced to encourage an export-before-
change posture before a destructive repair or migration. It **never**
appears as an authoritative covering promise on a restore-target row.

## 7. Restore-target inventory

`restore_targets[]` is the heart of the card. Exactly one row per
restore-target class, in any order, capped at four entries. Each row
MUST carry:

- `target_class` — `workspace`, `profile`, `evidence`, or `layout`.
- `target_ref` (optional) — opaque id of the target, for support and
  evidence linkage.
- `restorability_state` — `ready`, `stale`, `unverified`, `missing`,
  or `partially_restorable`.
- `covering_promise_class` — the recovery-promise class that should
  cover this target. Constrained by §2:
  - `evidence` MUST be covered by `authoritative_backup`.
  - `profile` MUST be covered by `authoritative_backup` or
    `sync_replica`.
  - `mirror_cache` and `convenience_export` MUST NOT appear here.
- `advisory_promise_classes[]` (optional) — additional promise
  classes the card surfaces as advisory hints (for example,
  `convenience_export` to encourage an export before change).
- `verification` — verification posture for the covering promise.
  `ready` rows MUST carry `verified_recent` or
  `verified_within_policy_window`; any other verification state
  forces a non-`ready` restorability state.
- `covered_state_categories[]` — the state-object categories this
  target covers (re-exported from `artifacts/state/state_objects.yaml`
  §`object_category_vocabulary`).
- `missing_dependency_summary` — required on `missing` and
  `partially_restorable` rows. Names the typed dependency the user
  needs to resolve before the row can move toward `ready`. Composes
  with the missing-dependency placeholder contract.
- `local_safe_guidance[]` — at least one guidance class, drawn from
  `local_safe_guidance_vocabulary`. Required on every row; the user
  is never left without a local-safe next action even when the row
  is `ready`.
- `summary` — short, redaction-aware text.

### 7.1 Restorability states

| State | Meaning | Required signal |
|---|---|---|
| `ready` | Covering promise is present, verified, and authoritative. | Verification state ∈ {`verified_recent`, `verified_within_policy_window`}. |
| `stale` | Covering promise is present but the verification or replica window has expired. | Local-safe guidance MUST include `verify_backup_locally`, `re_capture_local_checkpoint`, `reauthorize_sync_after_review`, or `import_offline_bundle`. |
| `unverified` | Covering promise is present but has never been verified or verification is currently unavailable. | Demoted from any `authoritative_backup` claim until verification completes; local-safe guidance MUST include `verify_backup_locally` or `escalate_to_support_with_evidence`. |
| `missing` | Covering promise is absent. | `missing_dependency_summary` MUST name what is missing; local-safe guidance MUST include at least one path forward. |
| `partially_restorable` | Covering promise exists but cannot fully cover the target on its own. | `missing_dependency_summary` MUST name the typed gap; local-safe guidance MUST include `restore_unsupported_use_partial_recovery` or `rebuild_from_authoritative_source`. |

### 7.2 Local-safe guidance

Closed `local_safe_guidance` vocabulary:

- `continue_local_work` — the local path remains safe and authoritative.
- `export_now_before_change` — encourage a convenience export before
  the next destructive action.
- `verify_backup_locally` — verification is overdue or never ran.
- `re_capture_local_checkpoint` — local-history capture has lagged.
- `import_offline_bundle` — offline mirror import is the truthful
  recovery path.
- `reauthorize_sync_after_review` — sync replica is divergent or
  outside the policy window; review and reauthorize before relying
  on it.
- `rebuild_from_authoritative_source` — the target requires a
  rebuild from an authoritative source.
- `restore_unsupported_use_partial_recovery` — full restore is not
  available; the placeholder contract handles the gap.
- `escalate_to_support_with_evidence` — the user should produce an
  evidence packet before any further destructive action.

## 8. Honesty invariants

Every card MUST carry the `honesty_invariants` block with three
const fields:

- `ordinary_cache_clear_excluded: true` — ordinary cache-clear flows
  never alter `authoritative_backup`, `local_checkpoint`,
  `sync_replica`, or evidence sources cited on this card. Mirror
  caches MAY be evicted by class-selective clear flows; the card
  surfaces that as a restorability state on the affected rows, never
  as a silent change to the promise class.
- `no_promise_blurring: true` — the card never collapses backup,
  checkpoint, sync replica, mirror cache, and convenience export into
  one ambiguous promise. Each appears under its own field or row.
- `missing_targets_have_local_safe_guidance: true` — `missing` and
  `partially_restorable` rows always emit at least one
  `local_safe_guidance` value drawn from
  `local_safe_guidance_vocabulary`.

These are const guarantees in the schema. Any surface that emits a
card without them is non-conforming.

## 9. Surface rules

Apply to every surface that renders, logs, exports, or reasons about
the records defined above.

1. **No surface invents a private continuity field.** Every consumer
   reads `card_id`, `generated_at`, `deployment_scope`,
   `last_backup`, `last_checkpoint`, `local_history_health`,
   `sync_or_replication_state`, `export_availability`,
   `restore_targets[]`, and `honesty_invariants` from the record;
   surfaces do not add parallel "backup ok"/"sync ok" fields when
   they render.
2. **One promise class per source.** A backup, checkpoint, replica,
   mirror, or export source resolves to exactly one promise class.
   Rendering it under two classes (for example, both `local_checkpoint`
   and `sync_replica`) is a contract violation.
3. **Mirror caches and convenience exports are never authoritative.**
   The schema enforces this by rejecting them in
   `restore_targets[].covering_promise_class`. Surfaces MUST NOT
   bypass the boundary by composing an "effective covering promise"
   from advisory entries.
4. **Evidence is only covered by an authoritative backup.** A
   surface that lists `sync_replica` or `local_checkpoint` as the
   covering promise for the `evidence` row is non-conforming.
5. **Profile is not covered by a local checkpoint.** Profile-wide
   durable state lives outside the local-history lane.
6. **Stale and unverified states demote.** A `stale_outside_window`
   sync replica or `never_verified` backup MUST NOT carry a `ready`
   restorability state. The schema rejects the combination.
7. **Missing and partially restorable rows always emit guidance.**
   The user is never left without a local-safe next action even when
   no managed service is available.
8. **Ordinary cache-clear is invariant.** A surface that lets a
   generic cache-clear flow demote local-history, autosave, or
   evidence rows on this card is non-conforming.
9. **One scope per card.** Cross-scope continuity (for example, two
   profiles on one device) emits one card per scope; a card never
   silently mixes profiles.

## 10. Composition with adjacent contracts

- The local-history contract defines the underlying entry, group,
  and clear-scope records. The continuity card cites local-history
  health and local-checkpoint presence, but never re-defines the
  entry vocabulary.
- The restore-preview contract owns the per-checkpoint compare/restore
  preview. The continuity card cites the most recent checkpoint and
  delegates per-target preview to the restore-preview record.
- The autosave-journal contract owns dirty-buffer recovery. The
  continuity card surfaces local-history-health and local-checkpoint
  presence; it never re-creates the journal vocabulary.
- The restore-provenance and placeholder contract owns partial-
  restore explanation. The continuity card flags
  `partially_restorable` with `missing_dependency_summary` so a
  follow-up restore-provenance record can carry the full placeholder
  card.
- The storage-classes contract owns garbage-collection policy. The
  continuity card cites `ordinary_cache_clear_excluded: true` to
  affirm the user-owned recovery-state invariant; it does not
  re-derive GC policy.
- The drill-catalog seed owns continuity drill rows. The continuity
  card's `deployment_scope` and `restore_targets[].covering_promise_class`
  align with drill-row vocabulary so support bundles can compare a
  drill outcome against a real card without translation.

## 11. Acceptance

- Users can tell how recoverable the current workspace, profile,
  evidence, and layout targets are by reading exactly one card —
  without inferring promise truth from prose. The fixtures under
  [`/fixtures/recovery/continuity_status_cases/`](../../fixtures/recovery/continuity_status_cases/)
  demonstrate this for healthy local-only, stale sync replica,
  verified backup, mirror-only cache, and partially restorable
  profile cases.
- Recovery promises do not blur into one ambiguous promise. The
  five-class vocabulary is closed; mirror caches and convenience
  exports are rejected as covering promises; evidence is only
  covered by an authoritative backup; profile is never covered by a
  local checkpoint.
- Restore targets remain separately inspectable. Workspace, profile,
  evidence, and layout rows do not collapse into one generic
  "restore" button.
- Stale, unverified, missing, and partially restorable states all
  carry required local-safe guidance, so the user always has a next
  action that does not require a managed service.

## 12. Changing this vocabulary

- **Additive-minor** changes (new `restorability_state` value, new
  `verification_state`, new `local_safe_guidance_class`, new
  `sync_replication_state_class`, new `export_availability_class`,
  new `deployment_profile_scope_class`, or a new
  `local_history_health_class`) land in this document, the schema,
  and the closed-vocabulary artifact in the same change. The change
  must cite the motivating fixture or packet.
- **Repurposing** a recovery-promise class, restore-target class,
  honesty invariant, or restorability state is **breaking**. It
  opens a new decision row and supersedes the relevant section of
  this document.
- The schema is the boundary. Any surface that adds a private field,
  collapses two promise classes, or emits a card without the
  honesty-invariants block is non-conforming.

## Source anchors

- `.t2/docs/Aureline_PRD.md` §5.24, §5.53 — recoverability and
  continuity claims.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §9.6, §9.7
  — control-plane / data-plane separation and recovery posture.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` Appendix CP
  — Local History, Timeline, and Reversible Checkpoint Matrix.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §15 — recovery and
  restore preview surfaces.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` §1202 — session
  restore, autosave, and crash-loop recovery preserve unsaved local
  content.

## Linked artifacts

- Schema:
  [`schemas/recovery/continuity_status_card.schema.json`](../../schemas/recovery/continuity_status_card.schema.json).
- Closed vocabulary catalog:
  [`artifacts/recovery/backup_checkpoint_classes.yaml`](../../artifacts/recovery/backup_checkpoint_classes.yaml).
- Worked-example fixtures:
  [`fixtures/recovery/continuity_status_cases/`](../../fixtures/recovery/continuity_status_cases/).
- Local-history contract:
  [`docs/reliability/local_history_contract.md`](./local_history_contract.md).
- Restore-preview contract:
  [`docs/reliability/local_history_restore_preview_contract.md`](./local_history_restore_preview_contract.md).
- Autosave-journal contract:
  [`docs/reliability/autosave_journal_and_guided_replay_contract.md`](./autosave_journal_and_guided_replay_contract.md).
- Restore-provenance contract:
  [`docs/state/restore_provenance_and_placeholder_contract.md`](../state/restore_provenance_and_placeholder_contract.md).
- Storage-classes contract:
  [`docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md).
- State-object inventory:
  [`docs/state/state_object_inventory.md`](../state/state_object_inventory.md).
- Drill catalog seed:
  [`docs/deployment/drill_catalog_seed.md`](../deployment/drill_catalog_seed.md).
