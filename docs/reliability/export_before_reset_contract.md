# Export-before-reset checklist, verification-result, and unsupported-class disclosure contract

This document freezes the cross-surface contract every reliability,
support, repair, and recovery surface uses when it answers the same
three questions **before** a drastic destructive recovery action runs:

1. **What user-owned artifact will be deleted, retained, or remain
   externally recoverable after the reset — and which classes are
   unsupported or policy-excluded from the export step?**
2. **What is the per-class export verification result — Verified,
   Partial, Blocked by policy, Unsupported class, or User declined —
   and what is the stable follow-up guidance for each?**
3. **Which named scenario family does the reset belong to, and how does
   the surface link the checklist into the support bundle, the repair-
   transaction record, and the restore-destination review so a drastic
   repair can be reconstructed later?**

The export-before-reset checklist is the **shared inspectable body**
that Project Doctor, the recovery-scenario card, the repair-transaction
preview, the restore-destination review, the support-intake escalation
packet, the recovery-ladder packet, the CLI listing, and the evidence
packet project into one typed row a reviewer can read mechanically. It
is not a reset runner, an export transport, or a UI rendering plan; it
is the contract those surfaces MUST conform to so a factory reset, a
profile reset, or any other drastic destructive recovery action never
fires before what will be deleted, retained, externally recoverable, or
unsupported has been frozen, verified, and recorded.

The machine-readable schemas live at:

- [`/schemas/recovery/export_before_reset_checklist.schema.json`](../../schemas/recovery/export_before_reset_checklist.schema.json)
- [`/schemas/recovery/export_verification_result.schema.json`](../../schemas/recovery/export_verification_result.schema.json)

Worked fixtures live under:

- [`/fixtures/recovery/export_before_reset_cases/`](../../fixtures/recovery/export_before_reset_cases/)

This contract composes with — and never re-defines — the recovery,
support, state, and reliability rules frozen elsewhere:

- [`/docs/reliability/recovery_scenario_contract.md`](./recovery_scenario_contract.md)
  — recovery-scenario family, safe-first-action matrix, destructive-
  risk class, and `export_before_reset` linkage.
- [`/docs/reliability/continuity_status_card_contract.md`](./continuity_status_card_contract.md)
  — recovery-promise class and restore-target inventory.
- [`/docs/reliability/corruption_rescue_compare_contract.md`](./corruption_rescue_compare_contract.md)
  — corrupt-artifact class, value posture, and quarantined-copy
  preservation.
- [`/docs/state/restore_destination_review_contract.md`](../state/restore_destination_review_contract.md)
  — restore-destination outcome, retained-vs-overwritten classes, and
  checkpoint-before-overwrite gate.
- [`/docs/state/portable_state_package_contract.md`](../state/portable_state_package_contract.md)
  — portable-state manifest, redaction manifest, and import posture.
- [`/docs/state/state_object_inventory.md`](../state/state_object_inventory.md)
  — authority class, schema-evolution posture, and corruption-routing
  matrix.
- [`/docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md)
  — support-bundle schema, redaction class, and recovery-ladder step.
- [`/docs/support/support_bundle_preview_contract.md`](../support/support_bundle_preview_contract.md)
  — preview-item identity, data-class tag, and per-item deselection.
- [`/docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md)
  — repair-transaction record and approved-repair vocabulary.
- [`/docs/support/support_intake_and_escalation_contract.md`](../support/support_intake_and_escalation_contract.md)
  — support-intake scenario picker and escalation-packet completeness.
- [`/docs/admin/org_admin_seat_and_fleet_contract.md`](../admin/org_admin_seat_and_fleet_contract.md)
  — admin seat lifecycle and fleet-recovery rules.

This contract is normative. Where it disagrees with the PRD, TAD, TDD,
UI/UX spec, or one of the upstream contracts above, those documents win
and this contract plus the schemas MUST be updated in the same change.
Where a downstream reset, repair, support-export, migration, or
recovery surface mints a parallel checklist, verification-result,
external-recoverability, or unsupported-class vocabulary, this contract
wins and the surface is non-conforming.

## Why freeze this now

Drastic destructive recovery is the moment panic-loss hurts the user
most. Without one frozen checklist:

- a `Reset workspace` button advertises a generic `This will erase all
  data` warning that hides which evidence packets remain externally
  recoverable, which credentials need re-issuance, and which machine-
  local assets cannot be exported at all;
- a factory reset deletes the user's local-history lane while a
  managed-tenant authoritative backup is still verified — the user
  panics, restores from the wrong source, and silently overwrites a
  recent draft;
- a profile reset is offered with `Export profile` greyed out because
  policy excludes secret-bearing keys from convenience exports, and the
  user clicks reset believing the export was completed;
- an evidence-packet class is silently omitted from the export step
  because its body is policy-bound, and the user discovers post-reset
  that the support escalation has nothing to attach;
- a checkpoint-class artifact (the local-history checkpoint index) is
  unsupported by the convenience-export transport, and the surface
  hides this behind a single `Export complete` summary instead of
  declaring `Unsupported class`;
- the export verification result is not recorded in any typed form, so
  the support engineer who reconstructs the reset later cannot tell
  whether the user clicked through `Verified`, `Partial`, or
  `User declined`;
- the reset is later re-routed through a generic warning rather than a
  scenario-specific review and the same lossy copy is reused
  identically for `profile_corruption`, `failed_update`, `seat_loss`,
  and `device_replacement`.

The export-before-reset checklist forecloses these patterns by treating
the export step as one inspectable row per user-owned class, a typed
verification result with stable follow-up guidance, an explicit list of
unsupported and policy-excluded classes, and a typed reset-review
language block that distinguishes deleted, retained, externally
recoverable, and not-exported state instead of relying on generic
caution copy. Once the boundary is named, every drastic recovery stays
scenario-coded, comparable, exportable, and reconstructible.

## Scope

Frozen here:

- one `export_before_reset_checklist_record` shape that every
  reliability, support, repair, and recovery surface emits before a
  drastic destructive recovery action runs;
- one `export_verification_result_record` shape that records the per-
  class export verification result and the typed reset-authorization
  state the action gate consumes;
- the closed eight-class **scenario-family** vocabulary re-exported
  from `schemas/recovery/recovery_scenario_card.schema.json` so a
  checklist always resolves to one named scenario and never to a
  generic reset story;
- the closed eight-class **reset-kind** vocabulary
  (`factory_reset`, `profile_reset`, `workspace_reset`, `layout_reset`,
  `evidence_index_reset`, `credential_store_reset`,
  `mirror_or_offline_bundle_reset`, `sync_replica_reset`) so every
  drastic action declares the exact scope it intends to destroy;
- the closed eleven-class **artifact-class** vocabulary covering every
  user-owned, policy-bound, machine-local, or recovery-state artifact
  the export step might cover (`workspace_state`, `profile_defaults`,
  `layout_topology`, `evidence_packets`, `local_history_lane`,
  `autosave_journal`, `credentials`, `seat_or_license_token`,
  `machine_local_assets`, `caches`, `install_chain_metadata`);
- the closed seven-class **value-posture** vocabulary re-exported from
  `schemas/recovery/corruption_rescue_sheet.schema.json` so the
  preservation gate uses the same posture vocabulary corruption-rescue
  uses (`user_authored_durable`, `user_owned_recovery_state`,
  `derived_disposable_only`, `mixed_user_and_derived`,
  `policy_bound_evidence`, `local_forensics_only`,
  `install_chain_metadata`);
- the closed six-class **external-recoverability** vocabulary
  (`externally_recoverable_authoritative_backup`,
  `externally_recoverable_sync_replica`,
  `externally_recoverable_managed_admin_seat`,
  `externally_recoverable_offline_bundle`,
  `locally_recoverable_only_no_external_source`,
  `not_recoverable_after_reset`) so every row names whether the class
  remains recoverable elsewhere after the reset, or names the absence
  explicitly;
- the closed six-class **export-action** vocabulary
  (`export_required`, `export_recommended`, `export_offered`,
  `export_unsupported`, `export_blocked_by_policy`, `export_declined_by_user`)
  binding the row to the verification-result class it is allowed to
  reach;
- the closed five-class **verification-result** vocabulary
  (`verified`, `partial`, `blocked_by_policy`, `unsupported_class`,
  `user_declined`) plus the closed seven-class **follow-up-guidance**
  vocabulary
  (`proceed_to_reset_acknowledged`,
  `retry_export_after_policy_review`,
  `skip_unsupported_class_acknowledged`,
  `defer_reset_until_user_returns`,
  `escalate_to_admin_seat_recovery`,
  `escalate_to_support_with_evidence`,
  `abort_reset`);
- the closed five-class **post-reset-state** vocabulary
  (`will_be_deleted`, `will_be_retained`,
  `externally_recoverable_after_reset`, `not_exported_unsupported`,
  `not_exported_user_declined`) the reset-review language block uses to
  distinguish deleted, retained, externally recoverable, and not-
  exported classes instead of a generic caution string;
- the closed seven-class **reset-authorization-state** vocabulary
  (`authorized_for_reset`, `blocked_pending_user_acknowledgement`,
  `blocked_pending_admin_seat`, `blocked_by_policy`,
  `blocked_unsupported_class`, `declined_by_user`,
  `aborted_by_user`) the destructive-action gate consumes verbatim;
- typed **linkage** rules from the checklist into the recovery-
  scenario card, the support bundle, the repair-transaction record,
  the restore-destination review, the continuity-status card, and the
  Project Doctor probe family so a drastic repair can be reconstructed
  later;
- a glossary block defining `externally_recoverable` versus
  `merely_local` artifacts so the surface vocabulary is not free-form
  prose;
- **honesty invariants** — closed scenario family, named reset kind,
  named post-reset state per row, named external-recoverability per
  row, unsupported and policy-excluded classes explicit (never hidden
  inside a generic warning), typed verification result per row, typed
  follow-up guidance per non-verified row, typed linkage refs.

Out of scope:

- the reset runner, factory-reset transport, profile-reset executor,
  or any other destructive runtime;
- the export transport itself (file write, encryption, redaction
  filter, signature backend);
- byte-level integrity checking, signature schemes, or content-
  addressable storage internals;
- automated retry of policy-blocked exports;
- final UI rendering, copy localization, or visual layout of the reset
  banner, the export-preview card, or the verification dashboard.

## 1. Record model — export-before-reset checklist

One checklist record per (recovery-scenario card, reset-kind) pair.
Every recovery surface reads exactly the fields below and no others.

| Field | Purpose |
|---|---|
| `checklist_id` | Stable id. Project Doctor, repair previews, support bundles, evidence packets, recovery-ladder packets, restore-destination reviews, and CLI output cite it. |
| `generated_at` | Producer-local monotonic timestamp. The checklist never re-reads system wall-clock from this field. |
| `scenario_family_class` | One of the closed eight recovery-scenario families re-exported from the recovery-scenario card. The checklist always resolves to one family; export-before-reset is never family-free. |
| `reset_kind_class` | One of the closed eight reset kinds. Surfaces never collapse `factory_reset` and `profile_reset` into one row. |
| `title` / `summary` | Short, redaction-aware text. Never embeds raw paths, raw provider payloads, or raw credentials. |
| `deployment_profile_scope_class` | Profile / deployment posture (`individual_local`, `self_hosted`, `air_gapped`, `managed_tenant`, `cross_plane_failover_pending`). |
| `checklist_rows[]` | One row per reviewable artifact class. Bounded by the closed artifact-class vocabulary so the checklist cannot silently expand into an undocumented blast radius. |
| `unsupported_classes[]` | Every artifact class whose export is `export_unsupported` is also enumerated here, with a closed unsupported-reason class. The list is **never empty** when at least one row is unsupported, even if the row also appears under `checklist_rows[]`. |
| `policy_exclusions[]` | Every artifact class whose export is `export_blocked_by_policy` is enumerated here with a typed policy ref and a redaction class. |
| `reset_review_language` | Typed block carrying short summaries for the four post-reset states: `will_be_deleted`, `will_be_retained`, `externally_recoverable_after_reset`, and `not_exported`. The reset surface MUST NOT collapse the four into one warning. |
| `verification_result_ref` | Opaque ref to the matching `export_verification_result_record`. Required when the reset is gated on the export step (`reset_authorization_state_class` other than `authorized_for_reset` is admitted only when the verification result is non-null). |
| `glossary` | Const-true block declaring the `externally_recoverable` vs `merely_local` distinction. Surfaces never substitute a private prose definition. |
| `linkage` | Typed refs into recovery-scenario card, support bundle, repair transaction, restore-destination review, continuity-status card, Project Doctor, and admin-seat recovery. |
| `honesty_invariants` | Const guarantees the checklist cannot silently waive. |

Rules (frozen):

1. A single record covers exactly one reset action on one scope; a
   factory-reset that would touch two profile scopes emits two records,
   one per scope.
2. Every artifact class enumerated under `checklist_rows[]` MUST resolve
   to exactly one of the eleven closed classes. A surface that mints
   `private_state`, `recovery_blob`, or another parallel class is
   non-conforming.
3. `unsupported_classes[]` and `policy_exclusions[]` are enumerated
   even when the matching row already appears in `checklist_rows[]`.
   The redundancy is intentional: a reviewer scanning only the
   unsupported list MUST see every class that cannot be exported, and a
   reviewer scanning only the policy list MUST see every class that the
   active policy excludes from convenience exports.
4. The `reset_review_language` block is the **only** structured copy
   the reset surface renders before confirm. Free-form
   `This is destructive` strings or generic `All data will be erased`
   prose are non-conforming on their own.

## 2. Reset-kind vocabulary

Eight closed reset kinds. Adding a kind is additive-minor and must
update this document, the schema, and the fixtures in the same change.

| Kind | What it deletes |
|---|---|
| `factory_reset` | Every user-owned class on this device returns to first-run state. Authoritative backup, sync replica, and managed-admin-seat sources remain externally recoverable; the local-history lane and autosave journal are deleted. |
| `profile_reset` | Profile defaults, keybindings, snippets, themes, and credentials are returned to defaults. Workspace bytes and evidence remain. |
| `workspace_reset` | Workspace state (workspace-authority body, workset metadata, dirty-buffer journal, local-history lane segments scoped to this workspace) returns to first-open state. Profile and evidence remain. |
| `layout_reset` | Window-topology snapshot, stable pane ids, tab/group topology, monitor-affinity hints return to defaults. Workspace bytes, profile, evidence remain. |
| `evidence_index_reset` | Evidence-packet index and support-evidence rows are wiped on this device. Authoritative-backup copies remain externally recoverable. |
| `credential_store_reset` | Local credential-store entries are wiped. Sync replica and managed-admin-seat re-issuance paths remain. |
| `mirror_or_offline_bundle_reset` | Mirror cache or offline bundle bytes return to first-fetch state. The user's authored bytes are not in scope. |
| `sync_replica_reset` | Local mirror of a sync replica is invalidated and re-hydrated. Authoritative source remains. |

Rules (frozen):

1. Reset kind is independent of scenario family. A `factory_reset`
   may belong to `device_replacement`, `seat_loss`, or
   `profile_corruption`; the matrix in §11 binds compatible
   combinations.
2. The reset-kind set is closed. A surface that mints
   `partial_reset`, `quick_clean`, or `cleanup` is non-conforming.

## 3. Artifact-class vocabulary

Eleven closed artifact classes. Each `checklist_rows[]` row resolves to
exactly one class. The first six mirror the reviewable-class taxonomy
in [`/docs/state/restore_destination_review_contract.md`](../state/restore_destination_review_contract.md);
the remaining five cover authentication, install-chain, and disposable-
state scopes that are reviewable in the export step but never reviewed
by the destination-review row set.

- `workspace_state` — workspace-authority bodies, dirty-buffer journals,
  active worksets, trusted roots.
- `profile_defaults` — settings, keybindings, snippets, themes,
  extension inventory.
- `layout_topology` — window-topology snapshots, stable pane ids,
  tab/group topology, monitor-affinity hints.
- `evidence_packets` — restore-provenance evidence refs, transcripts,
  snapshots, support-evidence indices.
- `local_history_lane` — the local-history lane bodies and metadata
  scoped to this device.
- `autosave_journal` — dirty-buffer / autosave journal segments scoped
  to this device.
- `credentials` — the OS-level credential store entries for this user.
- `seat_or_license_token` — managed-tenant entitlement, seat token, or
  license envelope held on this device.
- `machine_local_assets` — display hints, machine-unique handles,
  local-only credentials slots, OS-bound paths.
- `caches` — disposable derived state. Caches default to
  `derived_disposable_only` posture and `export_not_offered_metadata_only`
  guidance.
- `install_chain_metadata` — update-applied metadata and
  previous-install rollback candidate metadata.

Rules (frozen):

1. Every artifact class is reviewed exactly once per record. A class
   that appears in two checklist rows is non-conforming.
2. A row's `value_posture` is the producer's claim about the
   destination body. `mixed_user_and_derived` posture forces the row's
   `export_action_class` into the `export_required` or
   `export_recommended` half of the vocabulary; `derived_disposable_only`
   posture forces it into the `export_not_offered_metadata_only` /
   `export_unsupported` half.
3. A row whose `artifact_class` is `caches` MUST set
   `value_posture = derived_disposable_only` and
   `export_action_class = export_not_offered_metadata_only` unless the
   row is also enumerated under `unsupported_classes[]` with reason
   `class_is_disposable_derived`.
4. A row whose `artifact_class` is `evidence_packets` and whose
   `value_posture` is `policy_bound_evidence` MUST land in
   `policy_exclusions[]` when the active policy blocks convenience
   export. The schema enforces this with a conditional rule.

## 4. Per-row checklist fields

Each `checklist_rows[]` row carries:

- `row_id` — opaque stable id.
- `artifact_class` — one of the eleven closed classes from §3.
- `value_posture` — one of the seven closed value postures.
- `external_recoverability_class` — one of the six closed classes from
  §6. The row never relies on free-form prose to claim a class is
  externally recoverable.
- `external_source_ref` — opaque ref to the authoritative backup, sync
  replica, managed admin-seat record, or offline bundle that covers the
  class after reset. Required when `external_recoverability_class` is
  one of the four `externally_recoverable_*` classes; null when the
  class is `locally_recoverable_only_no_external_source` or
  `not_recoverable_after_reset`.
- `export_action_class` — one of the six closed export actions from
  §7.
- `export_artifact_ref` — opaque ref to the convenience export
  produced for this class. Required when `export_action_class` is
  `export_required`, `export_recommended`, or `export_offered` AND the
  matching `verification_result_class` is `verified` or `partial`.
- `redaction_class` — one of the four closed data-class boundary
  classes (`metadata_only`, `environment_adjacent`, `code_adjacent`,
  `high_risk`).
- `estimated_size` — bytes plus precision class
  (`exact_preflight`, `estimated_preflight`, `bounded_upper`,
  `unknown_until_build`). Re-export of the portable-state manifest
  vocabulary.
- `verification_result_class` — one of the five closed verification-
  result classes from §8.
- `follow_up_guidance_class` — required when `verification_result_class`
  is anything other than `verified`. Drawn from the seven-class
  follow-up-guidance vocabulary.
- `post_reset_state_class` — one of the five closed post-reset states
  from §10. The row never says `unknown` or `partially_lost`; if the
  class is unsupported, the row says `not_exported_unsupported`.
- `summary` — short, redaction-aware text the surface renders for the
  row before confirm.

Rules (frozen):

1. The row label set is closed. A surface that mints `partial_export`,
   `best_effort`, or `applied_with_caveat` for the row is
   non-conforming.
2. `export_required` rows whose `verification_result_class` is anything
   other than `verified` MUST carry a non-null
   `follow_up_guidance_class`. Free-form `Try again` prose is
   non-conforming.
3. `external_recoverability_class = not_recoverable_after_reset` MUST
   force the `post_reset_state_class` to `will_be_deleted` AND the
   reset-authorization gate to `blocked_pending_user_acknowledgement`
   (or `aborted_by_user`) until the user records the no-undo
   acknowledgement.

## 5. Unsupported-class disclosure

Every artifact class whose `export_action_class` is `export_unsupported`
MUST appear under `unsupported_classes[]` with:

- `unsupported_class` — the artifact class id.
- `unsupported_reason_class` — one of the closed five reason classes:
  - `transport_does_not_carry_class` — the convenience-export transport
    does not admit this class (machine-bound credentials, OS-level
    keyring entries, machine-local-asset addenda).
  - `class_is_disposable_derived` — the class is derived disposable
    state; export is not the right protection.
  - `class_is_authority_owned_remote` — authoritative source for the
    class is the managed control-plane or admin seat; the local body
    is a replica, not the source of truth.
  - `class_is_install_chain_metadata` — install-chain metadata is
    re-derivable from the previous-install rollback candidate, not
    exportable.
  - `class_is_machine_local_unique` — the class binds to a machine
    fingerprint and cannot travel.
- `redaction_class` — closed data-class boundary class.
- `summary` — redaction-aware text.

Rules (frozen):

1. The unsupported list is **never empty** when at least one
   `checklist_rows[]` row carries `export_action_class =
   export_unsupported`. Surfaces that hide an unsupported class behind
   a single `Export complete` summary are non-conforming.
2. `unsupported_reason_class` is closed. A reason that does not match
   one of the five values is non-conforming.
3. The reset-review language block (§10) MUST list every unsupported
   class under `not_exported.unsupported_classes[]` so the user sees
   the absence in the reset banner before confirm.

## 6. External-recoverability vocabulary

Six closed classes. Every checklist row resolves to exactly one. The
distinction `externally_recoverable` vs `merely_local` is fixed by the
glossary block and re-exported into the schema.

- `externally_recoverable_authoritative_backup` — the class is covered
  by a verified authoritative backup (workspace bytes, profile,
  evidence). After reset, the user can restore from that backup. Source
  ref points at the backup record.
- `externally_recoverable_sync_replica` — the class is covered by a
  sync replica (profile-only, never workspace authority). After reset,
  the user can re-hydrate from the replica.
- `externally_recoverable_managed_admin_seat` — the class is owned by
  the managed control-plane / admin seat (seat token, license
  envelope). After reset, the admin re-issues.
- `externally_recoverable_offline_bundle` — the class is reproducible
  from a signed offline bundle (mirror, docs pack, toolchain bundle).
- `locally_recoverable_only_no_external_source` — no external source
  exists for the class. The convenience export written by this
  checklist is the **only** copy. If the user declines the export, the
  class becomes `not_recoverable_after_reset`.
- `not_recoverable_after_reset` — the class is destroyed by the reset
  with no recovery path. Reset is gated on a typed no-undo
  acknowledgement.

Rules (frozen):

1. `evidence_packets` MUST resolve to
   `externally_recoverable_authoritative_backup` when an authoritative
   backup is present; sync replicas and local checkpoints never carry
   evidence bodies.
2. `seat_or_license_token` MUST resolve to
   `externally_recoverable_managed_admin_seat` when the deployment
   profile is `managed_tenant` and the seat is active. When the seat
   is revoked, the class resolves to `not_recoverable_after_reset` and
   the reset-authorization state advances to
   `blocked_pending_admin_seat`.
3. `caches` MUST resolve to `locally_recoverable_only_no_external_source`
   (regenerable from authoritative workspace bytes) or
   `not_recoverable_after_reset` (no authoritative source). Caches
   never claim `externally_recoverable_*`.

## 7. Export-action vocabulary

Six closed classes. Every checklist row resolves to exactly one.

- `export_required` — the row carries durable user-authored bytes or
  user-owned recovery state; the export step is required before
  confirm.
- `export_recommended` — the row carries mixed user-and-derived bytes
  or evidence whose authoritative source is reachable but the user
  benefits from a local convenience copy.
- `export_offered` — the row carries portable settings the user MAY
  export but is not required to.
- `export_unsupported` — the row's class is unsupported by the
  convenience-export transport. Enumerated under
  `unsupported_classes[]`.
- `export_blocked_by_policy` — the row's class is admitted by the
  transport but the active policy blocks the export. Enumerated under
  `policy_exclusions[]`.
- `export_declined_by_user` — the user explicitly declined the export
  for this row. Reset is gated on a typed `decline_acknowledgement_ref`.

Rules (frozen):

1. `export_required` and `export_recommended` rows whose
   `verification_result_class` is `verified` MUST carry a non-null
   `export_artifact_ref`.
2. `export_unsupported` MUST also appear in `unsupported_classes[]`
   with a closed reason class.
3. `export_blocked_by_policy` MUST also appear in `policy_exclusions[]`
   with a typed policy ref.
4. `export_declined_by_user` MUST set `verification_result_class` to
   `user_declined` and `follow_up_guidance_class` to one of
   `proceed_to_reset_acknowledged`, `defer_reset_until_user_returns`,
   or `abort_reset`.

## 8. Verification-result vocabulary

Five closed verification-result classes. Every checklist row resolves
to exactly one.

- `verified` — the export artifact was written, its checksum matches,
  the redaction class is honoured, and the row is safe to count as
  preserved.
- `partial` — the export artifact was written but at least one
  selected sub-section verified short of the producer claim
  (size-bounded, schema-pending, or one nested ref unresolved). Reset
  is gated on user acknowledgement of the partial.
- `blocked_by_policy` — the active policy refused the export at
  preflight. The row appears under `policy_exclusions[]`.
- `unsupported_class` — the convenience-export transport does not
  carry this class. The row appears under `unsupported_classes[]`.
- `user_declined` — the user explicitly declined the export. Reset is
  gated on the typed decline acknowledgement.

Rules (frozen):

1. The verification-result set is closed. A surface that emits
   `complete_with_warnings`, `best_effort`, or `try_again` is
   non-conforming.
2. The follow-up guidance for each non-`verified` row is enumerated in
   §9. Free-form `Try again` prose is non-conforming.

## 9. Follow-up guidance vocabulary

Seven closed follow-up-guidance classes. Every non-`verified` row
carries exactly one. The mapping below is the safe baseline; the
recovery-scenario card MAY refine but never invent.

| Verification result | Allowed follow-up guidance |
|---|---|
| `verified` | (none — no follow-up required) |
| `partial` | `proceed_to_reset_acknowledged`, `defer_reset_until_user_returns`, `escalate_to_support_with_evidence`, `abort_reset` |
| `blocked_by_policy` | `retry_export_after_policy_review`, `defer_reset_until_user_returns`, `escalate_to_admin_seat_recovery`, `abort_reset` |
| `unsupported_class` | `skip_unsupported_class_acknowledged`, `escalate_to_support_with_evidence`, `abort_reset` |
| `user_declined` | `proceed_to_reset_acknowledged`, `defer_reset_until_user_returns`, `abort_reset` |

The schema enforces the mapping with a conditional `if/then` block per
verification-result class.

## 10. Reset-review language block

The block carries one short summary per closed post-reset state and
binds each row to exactly one of the five states. It is the **only**
structured reset-review copy admitted by this contract; surfaces never
substitute a generic caution string.

The block carries:

- `will_be_deleted` — array of artifact-class ids that will be deleted.
  Required when at least one row's `post_reset_state_class` is
  `will_be_deleted`.
- `will_be_retained` — array of artifact-class ids that will be
  retained on this device after the reset. Required when at least one
  row's `post_reset_state_class` is `will_be_retained`.
- `externally_recoverable_after_reset[]` — array of one
  `recoverable_row` per class that will remain externally recoverable
  after the reset. Each row carries the artifact class, the external
  source class, and a short summary so the user reads "Your
  authoritative backup still covers your evidence packets" rather than
  "Some data may be recoverable elsewhere."
- `not_exported.unsupported_classes[]` — array of artifact-class ids
  that the export step did not carry because the class is unsupported.
  Required when at least one row's `verification_result_class` is
  `unsupported_class`.
- `not_exported.policy_excluded_classes[]` — array of artifact-class
  ids that the export step did not carry because policy excluded them.
  Required when at least one row's `verification_result_class` is
  `blocked_by_policy`.
- `not_exported.user_declined_classes[]` — array of artifact-class
  ids that the export step did not carry because the user declined.
- `summary` — redaction-aware text summarizing the four buckets in one
  sentence. The summary MUST distinguish deleted, retained, externally
  recoverable, and not-exported state in human terms; vague phrasing
  such as "best-effort" or "partial reset" is non-conforming on its
  own.

Rules (frozen):

1. Every checklist row's `post_reset_state_class` MUST appear in the
   matching block field. A row whose class is `will_be_deleted` but is
   omitted from `will_be_deleted[]` is non-conforming.
2. The block MUST NOT collapse the four buckets into one warning. A
   surface that renders only `summary` without rendering the four
   bucket arrays is non-conforming.
3. Display copy MAY render the block as four sections under one
   "Before resetting" header; the closed machine set is fixed.

## 11. Scenario-family / reset-kind matrix

The matrix below is the safe baseline. The matrix is enforced by a
schema rule: every (scenario, reset-kind) pair the matrix declares
`forbidden` is rejected by the schema.

| Scenario family | Allowed reset kinds | Forbidden reset kinds |
|---|---|---|
| `profile_corruption` | `profile_reset`, `factory_reset` | `workspace_reset`, `evidence_index_reset`, `mirror_or_offline_bundle_reset` |
| `workspace_index_corruption` | (none — rebuild only; reset is forbidden because the destructive action would touch user-authored bytes) | every reset kind |
| `failed_update` | `factory_reset` (only after rollback failed) | `profile_reset`, `workspace_reset`, `evidence_index_reset` |
| `control_plane_outage` | (none — destructive action is forbidden during outage; consult `recovery_scenario_card` rule §12.8) | every reset kind |
| `device_replacement` | `factory_reset` (on the abandoned device only) | every reset kind on the new device |
| `seat_loss` | `factory_reset`, `profile_reset`, `credential_store_reset` | `evidence_index_reset` (evidence stays exportable to the user-owned local bundle), `mirror_or_offline_bundle_reset` |
| `credential_store_unreadable` | `credential_store_reset` | every other reset kind (no user-authored bytes are touched) |
| `mirror_or_offline_bundle_unavailable` | `mirror_or_offline_bundle_reset`, `sync_replica_reset` | every other reset kind |

Adding a row is additive-minor and must update this document, the
schema, and the fixtures in the same change. Repurposing a row is
breaking and requires a new decision row in
`artifacts/governance/decision_index.yaml`.

## 12. Linkage rules

Every checklist MUST cite typed linkage refs. Free-form prose linkage
is non-conforming.

### 12.1 Recovery-scenario card

`linkage.recovery_scenario_card` carries:

- `card_ref` — opaque ref to the
  `recovery_scenario_card_record` the checklist composes with.
- `matrix_row_ref` — opaque ref to the safe-first-action matrix row
  for the scenario family.
- `summary` — short, redaction-aware text.

A checklist MUST cite exactly one card ref. Free-text "see the recovery
panel" linkage is non-conforming.

### 12.2 Support bundle

`linkage.support_bundle` carries:

- `support_bundle_ref` — opaque ref to the
  `support_bundle_record` the checklist's exports are bundled into.
  The support-bundle preview surface composes over this ref.
- `support_bundle_preview_ref` — opaque ref to the
  `support_bundle_preview_record`.
- `redaction_class` — one of the four closed data-class boundary
  classes; MUST be at least the highest redaction class of any
  attached row.
- `summary` — short, redaction-aware text.

A checklist whose `verification_result_class` is `verified` for any
`export_required` row MUST cite a non-null `support_bundle_ref` so a
support engineer can reconstruct the export later.

### 12.3 Repair-transaction record

`linkage.repair_transaction` carries:

- `repair_transaction_ref` — opaque ref to the
  `repair_transaction_record` the reset action runs under.
- `repair_preview_ref` — opaque ref to the matching repair-preview
  record.
- `approved_repair_class` — drawn from the closed approved-repair
  vocabulary in
  `schemas/support/scenario_picker.schema.json#approved_repair_class`.
  MUST NOT appear in the matching scenario family's
  `forbidden_repair_classes[]`.
- `summary` — short, redaction-aware text.

The reset is the only path through which a destructive action runs;
the checklist MUST cite a repair-transaction ref so the action is
governed by the repair-transaction contract.

### 12.4 Restore-destination review

`linkage.restore_destination_review` carries:

- `review_ref` — opaque ref to the
  `state_restore_destination_review_record` produced when the user
  later restores from the convenience export. Required when at least
  one `export_required` row is `verified` and the export bundle could
  be re-imported on a new device.
- `summary` — short, redaction-aware text.

The destination-review record is the surface a future support engineer
or admin uses to reconstruct the reset.

### 12.5 Continuity-status, Project Doctor, and admin-seat refs

Optional but recommended:

- `linkage.continuity_status_card_ref` — opaque ref to the
  `continuity_status_card_record` the checklist composes with.
- `linkage.project_doctor.probe_family_class` — drawn from the closed
  probe-family vocabulary. Required when the scenario family routes
  through Project Doctor.
- `linkage.admin_seat_recovery_ref` — opaque ref to the admin-seat
  recovery record. Required when the scenario family is `seat_loss`
  and the reset kind is `factory_reset` or `profile_reset`.

## 13. Honesty invariants

Every checklist MUST carry the `honesty_invariants` block with seven
const-`true` fields:

- `scenario_family_is_closed: true` — the checklist resolves to exactly
  one closed scenario family and never collapses two families into one
  row.
- `reset_kind_is_closed: true` — the checklist resolves to exactly one
  closed reset kind.
- `unsupported_classes_explicit: true` — every unsupported class is
  enumerated under `unsupported_classes[]` with a closed reason; no
  unsupported class is hidden inside a generic warning.
- `policy_exclusions_explicit: true` — every policy-excluded class is
  enumerated under `policy_exclusions[]` with a typed policy ref.
- `external_recoverability_per_row: true` — every checklist row carries
  one closed external-recoverability class; the user is never told the
  class is "maybe recoverable" without a named source.
- `reset_review_language_distinguishes_four_buckets: true` — the
  reset-review block separately enumerates deleted, retained,
  externally recoverable, and not-exported classes; no surface
  collapses the four into one caution string.
- `linkage_is_typed: true` — linkage refs are typed (recovery-scenario
  card ref, support-bundle ref, repair-transaction ref, restore-
  destination review ref, probe-family class) rather than free-form
  prose.

These are const guarantees in the schema. Any surface that emits a
checklist without them is non-conforming.

## 14. Glossary

A const-true block re-exported by the schema:

- `externally_recoverable` — an artifact class is externally
  recoverable after a reset when at least one of the following sources
  exists, is reachable from the user's identity, and carries the same
  authority class as the local body: an authoritative backup, a sync
  replica (for profile-shaped state only), a managed-admin-seat
  re-issuance path (for seat or license tokens only), or a signed
  offline bundle (for mirror or docs-pack-shaped state only).
- `merely_local` — an artifact class is merely local when no source
  above exists. The convenience export written by this checklist is
  the only copy; if the export is unsupported, blocked, or declined,
  the class becomes `not_recoverable_after_reset`.

Surfaces never substitute a private prose definition for either term.

## 15. Record model — export-verification-result

One verification-result record per checklist. The record carries:

| Field | Purpose |
|---|---|
| `verification_id` | Stable id. Cited from `checklist.verification_result_ref`. |
| `checklist_id_ref` | Opaque ref back to the matching checklist. |
| `generated_at` | Producer-local monotonic timestamp. |
| `overall_result_class` | One of the closed five verification-result classes. The overall result is the lowest-success result across all checklist rows: a single `unsupported_class` or `blocked_by_policy` row pulls the overall result out of `verified`. |
| `verification_rows[]` | One row per checklist row, binding `row_id_ref` to `result_class` and `follow_up_guidance_class`. |
| `reset_authorization_state_class` | One of the closed seven reset-authorization-state classes. The destructive action gate consumes this verbatim. |
| `decline_acknowledgement_ref` | Opaque ref to the typed acknowledgement the user recorded for any `user_declined` or `not_recoverable_after_reset` row. |
| `summary` | Short, redaction-aware text. |
| `honesty_invariants` | Const guarantees the verification-result record cannot silently waive. |

Rules (frozen):

1. The `verification_rows[]` array carries exactly one row per
   checklist row. A surface that omits a row is non-conforming.
2. `overall_result_class = verified` is admitted only when every row's
   `result_class` is `verified` AND no row's `export_action_class` is
   `export_unsupported` or `export_blocked_by_policy`.
3. `reset_authorization_state_class = authorized_for_reset` is
   admitted only when the overall result is `verified` OR when every
   non-verified row carries a typed acknowledgement (no-undo,
   skip-unsupported, or decline-acknowledgement).

## 16. Reset-authorization-state vocabulary

Seven closed classes. The destructive-action gate consumes exactly one.

- `authorized_for_reset` — every required acknowledgement is recorded;
  the reset action gate is open.
- `blocked_pending_user_acknowledgement` — at least one row carries
  `not_recoverable_after_reset`, `partial`, `unsupported_class`, or
  `user_declined` and the user has not yet recorded the typed
  acknowledgement.
- `blocked_pending_admin_seat` — at least one row carries
  `externally_recoverable_managed_admin_seat` but the seat is revoked
  or unreachable; the admin must re-issue before the reset proceeds.
- `blocked_by_policy` — at least one row carries
  `export_blocked_by_policy` and the active policy refused the export
  outright; the reset is blocked until the policy review path
  completes.
- `blocked_unsupported_class` — at least one row carries
  `export_unsupported` AND the matching follow-up guidance is
  `escalate_to_support_with_evidence` AND the user has not yet skipped
  the unsupported class.
- `declined_by_user` — the user declined the reset entirely. The
  authorization state is terminal until a new checklist is generated.
- `aborted_by_user` — the user aborted the reset after the gate
  opened. Authorization is terminal; a fresh checklist is required for
  any retry.

The verification-result record's
`reset_authorization_state_class` is the **only** field the reset
runner reads to decide whether the destructive action may run.

## 17. Honesty invariants — verification result

Every verification-result record MUST carry the `honesty_invariants`
block with five const-`true` fields:

- `every_row_has_typed_result: true` — every row carries one closed
  verification-result class.
- `non_verified_row_has_typed_followup: true` — every non-`verified`
  row carries one closed follow-up-guidance class.
- `unsupported_or_policy_row_visible_in_overall: true` — a single
  unsupported or policy-blocked row pulls the overall result out of
  `verified`; the overall result never silently upgrades.
- `reset_authorization_is_typed: true` — the reset-authorization state
  is one of the seven closed classes; the gate never reads a free-form
  prose flag.
- `linkage_back_to_checklist_is_typed: true` — `checklist_id_ref` is a
  typed opaque ref to the matching checklist.

## 18. Surface rules

Apply to every surface that renders, logs, exports, or reasons about
export-before-reset records.

1. **No surface invents a private reset kind or scenario family.**
   Every consumer resolves to one of the eight closed reset kinds and
   one of the eight closed scenario families.
2. **Unsupported and policy-excluded classes are visible in two
   places.** Every unsupported class appears under
   `checklist_rows[]` AND `unsupported_classes[]`; every policy-
   excluded class appears under `checklist_rows[]` AND
   `policy_exclusions[]`. Surfaces that hide either set behind a
   single banner string are non-conforming.
3. **The reset-review language block is structured.** Surfaces render
   the four buckets (`will_be_deleted`, `will_be_retained`,
   `externally_recoverable_after_reset`,
   `not_exported.{unsupported,policy,user_declined}_classes`)
   separately. Free-form `This is destructive` strings without the
   four buckets are non-conforming.
4. **Linkage stays typed.** Every checklist cites typed refs for the
   recovery-scenario card, the support bundle, the repair transaction,
   and (when a destination apply could later happen) the restore-
   destination review.
5. **Reset is gated by the verification-result record.** A reset
   runner that reads any field other than
   `reset_authorization_state_class` to decide whether the destructive
   action may run is non-conforming.
6. **Drastic actions never reuse one generic warning.** A
   `factory_reset` banner that is identical to a `profile_reset`
   banner is non-conforming. Banner copy MUST cite the reset kind, the
   scenario family, and the four reset-review buckets.

## 19. Composition with adjacent contracts

- The recovery-scenario card describes the recovery posture; this
  checklist describes the export-before-reset gate for one drastic
  action under that posture. The checklist cites the scenario card by
  `card_ref`; it never re-derives recovery-promise truth.
- The corruption-rescue compare sheet handles the *suspect-state
  preservation* gate (quarantined-copy preservation before rebuild /
  replace / discard); this checklist handles the *export-before-reset*
  gate. A drastic reset that involves a corrupt artifact MAY cite the
  corruption-rescue sheet via `linkage.recovery_scenario_card.card_ref`
  resolving to the same scenario family.
- The restore-destination review describes a *restore* apply; this
  checklist describes the export step that precedes a *reset*. The
  two compose: a future restore from the convenience export this
  checklist produced is reviewed by a destination-review record.
- The support-bundle contract owns the bundle body; this checklist
  cites the bundle by ref and never re-defines the bundle vocabulary.
- The support-bundle preview contract owns per-item preview rows; this
  checklist cites the preview by ref.
- The repair-transaction contract owns repair-preview, apply, and
  rollback. The reset runs through the repair transaction; this
  checklist is the export-before-reset gate that the transaction
  consumes.
- The admin seat / fleet contract owns seat lifecycle. The
  `seat_or_license_token` row's external-recoverability resolves
  through the admin-seat contract; this checklist never re-derives
  entitlement state.

## 20. Acceptance

- Factory-reset and similarly broad destructive actions require a
  checklist that names what will be deleted, what is retained, and
  what is recoverable elsewhere. The schema rejects any checklist
  whose `reset_review_language` block omits the four buckets when
  matching rows are present.
- Unsupported or policy-excluded classes remain explicit rather than
  hidden inside generic warnings. The schema requires every
  `export_unsupported` row to also appear under
  `unsupported_classes[]` and every `export_blocked_by_policy` row to
  also appear under `policy_exclusions[]`.
- Broad destructive recovery actions can later be routed through
  scenario-specific review rather than reusing one generic reset
  warning. The closed scenario-family / reset-kind matrix in §11
  forces every checklist to declare its specific (family, kind) pair.
- The fixtures under
  [`/fixtures/recovery/export_before_reset_cases/`](../../fixtures/recovery/export_before_reset_cases/)
  cover at least: local-only reset, policy-blocked export, verified
  export-before-reset, and unsupported evidence class.

## 21. Changing this vocabulary

- **Additive-minor** changes (new artifact class, new reset kind, new
  external-recoverability class, new export-action class, new
  verification-result class, new follow-up-guidance class, new
  post-reset state class, new reset-authorization state class, new
  scenario / reset-kind matrix row) land in this document, the
  schemas, and the fixtures in the same change. The change must cite
  the motivating fixture or packet.
- **Repurposing** an existing artifact class, reset kind, verification
  result, follow-up guidance, post-reset state, or reset-authorization
  state is **breaking**. It opens a new decision row and supersedes
  the relevant section of this document.
- The schemas are the boundary. Any surface that adds a private field,
  collapses two reset kinds, or emits a checklist without the
  `honesty_invariants` block is non-conforming.

## Source anchors

- `.t2/docs/Aureline_PRD.md` §5.24, §5.53 — recoverability and
  continuity claims; drastic destructive actions must be scenario-
  coded and gated on a reviewable export step.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §9.6, §9.7,
  Appendix CP — control-plane / data-plane separation, recovery
  posture, and the local-history / checkpoint matrix.
- `.t2/docs/Aureline_Technical_Design_Document.md` — Project Doctor,
  repair-transaction, support-bundle, and reset path component
  designs.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §15 — recovery and
  restore preview surfaces; destructive actions follow the
  export-before-reset gate.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` §1202 — session
  restore, autosave, and crash-loop recovery preserve unsaved local
  content; copy stays scenario-coded.

## Linked artifacts

- Schema (checklist):
  [`schemas/recovery/export_before_reset_checklist.schema.json`](../../schemas/recovery/export_before_reset_checklist.schema.json).
- Schema (verification result):
  [`schemas/recovery/export_verification_result.schema.json`](../../schemas/recovery/export_verification_result.schema.json).
- Worked-example fixtures:
  [`fixtures/recovery/export_before_reset_cases/`](../../fixtures/recovery/export_before_reset_cases/).
- Recovery-scenario card contract:
  [`docs/reliability/recovery_scenario_contract.md`](./recovery_scenario_contract.md).
- Continuity-status card contract:
  [`docs/reliability/continuity_status_card_contract.md`](./continuity_status_card_contract.md).
- Corruption-rescue compare contract:
  [`docs/reliability/corruption_rescue_compare_contract.md`](./corruption_rescue_compare_contract.md).
- Restore-destination review contract:
  [`docs/state/restore_destination_review_contract.md`](../state/restore_destination_review_contract.md).
- Portable-state package contract:
  [`docs/state/portable_state_package_contract.md`](../state/portable_state_package_contract.md).
- Support-bundle contract:
  [`docs/support/support_bundle_contract.md`](../support/support_bundle_contract.md).
- Support-bundle preview contract:
  [`docs/support/support_bundle_preview_contract.md`](../support/support_bundle_preview_contract.md).
- Repair-transaction contract:
  [`docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md).
- Support-intake / escalation contract:
  [`docs/support/support_intake_and_escalation_contract.md`](../support/support_intake_and_escalation_contract.md).
- Admin seat / fleet contract:
  [`docs/admin/org_admin_seat_and_fleet_contract.md`](../admin/org_admin_seat_and_fleet_contract.md).
