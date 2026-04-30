# Corruption-rescue compare sheet, quarantined-copy preservation, and rebuild/restore/export/discard contract

This document freezes the cross-surface contract every reliability,
support, repair, and recovery surface uses when it answers the same
two questions before a destructive corruption-rescue action runs:

1. **What is the corrupt artifact, what value does it still carry,
   and what healthy candidate source covers a recovery — if any?**
2. **What is preserved (in a typed quarantined copy), what is
   replaced or discarded, and which rescue action class — inspect,
   export, rebuild, restore, replace, or discard — is the user
   actually being offered?**

The corruption-rescue compare sheet is the **shared inspectable
body** that Project Doctor, the repair-transaction preview, the
recovery-scenario card, the recovery-ladder packet, the
support-intake scenario picker, the export-before-reset path, and
the CLI listing project into one typed row a reviewer can read
mechanically. It is not a diff renderer, not a repair runner, and
not an automated reset path. It is the contract those surfaces MUST
conform to so a destructive rebuild never runs before suspect state
that may still hold user-owned data, diagnostics value, or
policy-bound evidence has been preserved as a typed quarantined
copy.

The machine-readable schemas live at:

- [`/schemas/recovery/corruption_rescue_sheet.schema.json`](../../schemas/recovery/corruption_rescue_sheet.schema.json)
- [`/schemas/recovery/quarantined_copy_record.schema.json`](../../schemas/recovery/quarantined_copy_record.schema.json)

Worked fixtures live under:

- [`/fixtures/recovery/corruption_rescue_cases/`](../../fixtures/recovery/corruption_rescue_cases/)

This contract composes with — and never re-defines — the recovery,
support, and reliability rules frozen elsewhere:

- [`/docs/reliability/recovery_scenario_contract.md`](./recovery_scenario_contract.md)
  — recovery-scenario card, safe-first-action matrix, and
  destructive-risk labeling.
- [`/docs/reliability/continuity_status_card_contract.md`](./continuity_status_card_contract.md)
  — recovery-promise class and restore-target inventory.
- [`/docs/reliability/local_history_restore_preview_contract.md`](./local_history_restore_preview_contract.md)
  — restore-preview identity-relation vocabulary and retained-state
  rendering rules.
- [`/docs/reliability/autosave_journal_and_guided_replay_contract.md`](./autosave_journal_and_guided_replay_contract.md)
  — dirty-buffer journal and journal-reset path.
- [`/docs/support/project_doctor_packet.md`](../support/project_doctor_packet.md)
  — Project Doctor finding catalog and probe-family rows.
- [`/docs/support/repair_transaction_contract.md`](../support/repair_transaction_contract.md)
  — repair-transaction preview, approved-repair vocabulary, and the
  `disposable_state_rebuild` / `guided_export_escalation` repair
  classes.
- [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md)
  — recovery-ladder rung rollback and reversal-class vocabulary.
- [`/docs/state/state_object_inventory.md`](../state/state_object_inventory.md)
  — authority class, schema-evolution posture, and backup-before-
  migrate rule for every persisted state object.
- [`/docs/state/restore_provenance_and_placeholder_contract.md`](../state/restore_provenance_and_placeholder_contract.md)
  — restore-provenance, compatibility-restore downgrade, and
  missing-dependency placeholder.
- [`/docs/release/update_and_rollback_contract.md`](../release/update_and_rollback_contract.md)
  — update apply/rollback path and previous-install rollback
  candidate.

This contract is normative. Where it disagrees with the PRD, TAD,
TDD, UI/UX spec, or one of the upstream contracts above, those
documents win and this contract plus the schemas MUST be updated in
the same change.

## Why freeze this now

Corruption-rescue copy is the moment a destructive action is one
click away. Without one frozen sheet:

- a "Rebuild profile" button regenerates a profile-store from
  defaults and silently destroys the user's keyring index and
  durable settings, because the surface treated it as a derived
  cache;
- a "Reset workspace index" path replaces a `mixed_user_and_derived`
  index whose user-side annotations were never quarantined first,
  and the user loses pinned search results and indexer overrides;
- a "Discard failed update metadata" path removes the install-chain
  evidence Doctor needed for rollback diagnosis, because the surface
  collapsed `derived` and `forensics-valuable` into one bucket;
- a `local_forensics_only` corrupt artifact is reset before its
  bytes are quarantined for the pending investigation, leaving the
  diagnosis trail with a hole;
- the user sees a single `Replace` button with no row showing what
  is kept, what is replaced, and which healthy source — if any —
  covers the recovery.

The corruption-rescue compare sheet forecloses these patterns by
treating each suspect artifact as one inspectable row carrying a
typed value-posture, a typed compare preview, a typed preservation
decision, and one verb-coded recommended action. Once the boundary
is named, every corruption-rescue surface stays
suspect-state-preserving, action-honest, and exportable.

## Scope

Frozen here:

- one `corruption_rescue_sheet_record` shape that every reliability,
  support, repair, and recovery surface emits when it routes the
  user toward a corruption-rescue path;
- one `quarantined_copy_record` shape that records the typed
  preservation receipt for the suspect artifact;
- the closed **corrupt-artifact** class vocabulary (workspace search
  index, workspace knowledge cache, profile settings store, profile
  keyring index, workspace-authority checkpoint index, local-history
  segment, evidence-packet index, update-install metadata, mirror
  cache manifest, offline-bundle manifest);
- the closed **artifact-value-posture** vocabulary
  (`user_authored_durable`, `user_owned_recovery_state`,
  `derived_disposable_only`, `mixed_user_and_derived`,
  `policy_bound_evidence`, `local_forensics_only`,
  `install_chain_metadata`) that drives the preservation gate;
- the closed **rescue-action** class, **rescue-verb** class,
  **reversibility** class, and **destructive-risk** class
  vocabularies, re-exported verbatim from the recovery-scenario
  contract so a sheet never says "reset" when it means "rebuild" or
  "restore" when it means "investigate";
- the closed **healthy-candidate-source** vocabulary
  (`covering_authoritative_backup`, `covering_sync_replica`,
  `covering_local_checkpoint`,
  `regeneration_from_authoritative_workspace_bytes`,
  `default_initialization_template`,
  `no_healthy_candidate_available`) so every sheet either names one
  authoritative source or names the absence explicitly;
- the closed **post-action-state** vocabulary
  (`kept_intact`, `retained_in_quarantine`,
  `regenerated_from_authoritative`,
  `restored_from_covering_promise`, `replaced_with_default`,
  `discarded_after_export`, `exported_only_no_change`) that the
  compare preview uses to make retained-versus-replaced state
  explicit;
- the closed **preserve-reason** vocabulary
  (`carries_user_authored_bytes`,
  `carries_unexported_user_recovery_state`,
  `carries_diagnostics_value`,
  `carries_policy_bound_evidence`,
  `mixed_user_and_derived_indeterminate`,
  `forensics_required_by_pending_investigation`,
  `ownership_indeterminate`) that drives the quarantine decision;
- the closed **artifact-origin** class, **preserved-storage** class,
  **integrity-witness** class, **retention** class,
  **clearance-owner** class, **visibility** class, and
  **inspectable-action** class vocabularies for quarantined copies;
- typed **linkage** rules from the sheet into Project Doctor (probe
  family + finding-code refs), the repair-transaction preview, the
  recovery-scenario card, the continuity-status card, the
  support-intake scenario picker, and the export-before-destructive
  path;
- **honesty invariants** — closed corrupt-artifact class, named
  artifact-value posture, preserved-copy required before any
  destructive replace or discard, healthy-candidate source named (or
  absence named explicitly), retained-versus-replaced state
  explicit, no overpromise of reversibility, typed linkage refs.

Out of scope:

- the diff renderer or compare engine itself;
- repair runners (repair-transaction apply, restore runner, rebuild
  runner, install-chain rollback runner);
- byte-level integrity checking, signature verification schemes, or
  content-addressable storage internals;
- automated quarantine retention sweepers;
- final UI rendering, copy localization, or visual layout of the
  rescue dashboard.

## 1. Record model — corruption-rescue compare sheet

One sheet per (corrupt artifact, generated-at) pair. Every recovery
surface reads exactly the fields below and no others.

| Field | Purpose |
|---|---|
| `sheet_id` | Stable id. Project Doctor, repair previews, support bundles, evidence packets, recovery-ladder packets, and CLI output cite it. |
| `generated_at` | Producer-local monotonic timestamp. The sheet never re-reads system wall-clock from this field. |
| `scenario_family_class` | One of the closed eight recovery-scenario families re-exported from `schemas/recovery/recovery_scenario_card.schema.json`. The sheet always resolves to one family; corruption-rescue is never family-free. |
| `title` / `summary` | Short, redaction-aware text. Never embeds raw paths, raw provider payloads, or raw credentials. |
| `corrupt_artifact_class` | One of the closed corrupt-artifact classes. Sheets never collapse two artifacts into one row. |
| `artifact_value_posture` | One of the seven closed value postures. Drives the preservation gate. |
| `corrupt_witness` | Typed witness for the suspect artifact: opaque identity token, integrity-witness class, redaction class, summary. |
| `healthy_candidate` | Healthy-candidate source class plus an opaque source identity token, redaction class, and summary; or `no_healthy_candidate_available` with an explicit summary. |
| `compare_preview` | Closed post-action-state rows binding affected scopes to retained-versus-replaced state. The sheet never relies on free-form prose to communicate what is kept vs replaced. |
| `preservation` | Preserved-copy decision: required-or-not, preserved-copy ref (when required), closed preserve-reason set, summary. |
| `available_actions[]` | One row per action class the matrix permits for this sheet. Each row is verb-coded and risk-coded. |
| `recommended_action` | Exactly one action drawn from `available_actions[]`. Required pre-actions and no-undo acknowledgement live here. |
| `linkage` | Typed refs into Project Doctor, repair-transaction preview, recovery-scenario card, continuity-status card, support intake, and export-before-destructive. |
| `honesty_invariants` | Const guarantees the sheet cannot silently waive. |

## 2. Corrupt-artifact classes

Closed ten-class vocabulary. Every sheet resolves to exactly one
artifact class. Adding a class is additive-minor and must update
this document, the schema, and the fixtures in the same change.

| Class | What it covers |
|---|---|
| `workspace_search_index` | Derived workspace search index. |
| `workspace_knowledge_cache` | Derived workspace knowledge / artifact cache. |
| `profile_settings_store` | Profile-wide durable settings store. |
| `profile_keyring_index` | Profile keyring index. |
| `workspace_authority_checkpoint_index` | Local workspace-authority checkpoint index. |
| `local_history_segment` | A local-history segment whose backing file lost integrity. |
| `evidence_packet_index` | Evidence packet index file. |
| `update_install_metadata` | Install-chain metadata for an applied or partly-applied update. |
| `mirror_cache_manifest` | Signed mirror-cache manifest. |
| `offline_bundle_manifest` | Offline-bundle manifest. |

## 3. Artifact value posture

Closed seven-class vocabulary. Every sheet declares one value
posture; this drives preservation. The schema and §11 invariants
enforce the gates.

| Posture | Preservation rule |
|---|---|
| `user_authored_durable` | User-typed configuration, profile-wide settings, keyring contents. Preservation REQUIRED before any `replace` or `discard`. |
| `user_owned_recovery_state` | Local-history, autosave, workspace-authority checkpoint state owned by the user. Preservation REQUIRED before any `replace` or `discard`. |
| `derived_disposable_only` | Caches, indexes, prebuild artifacts. Preservation OPTIONAL unless the sheet also lists `carries_diagnostics_value` in `preserve_reasons`. |
| `mixed_user_and_derived` | Cache or index that contains user-side annotations or pins. Preservation REQUIRED before any `replace` or `discard`. Rebuild is FORBIDDEN; restore is the path. |
| `policy_bound_evidence` | Evidence-packet bytes covered by retention or legal hold. Preservation REQUIRED. |
| `local_forensics_only` | Bytes whose remaining value is forensic — diagnosing the failure or supporting a pending investigation. Preservation REQUIRED. |
| `install_chain_metadata` | Install / update metadata. Preservation REQUIRED before any `replace` or `discard` because the previous-install rollback path depends on it. |

## 4. Rescue-verb vocabulary and wording rules

Six verbs. Each binds one user-visible meaning, one allowed
reversibility range, and one allowed destructive-risk range so a
sheet never says "reset" when it means "rebuild", or "restore" when
it means "investigate".

| Verb | Meaning | Allowed reversibility | Allowed destructive risk |
|---|---|---|---|
| `inspect` | Read-only. Reads metadata, runs probes, renders the compare preview; never mutates state. | `exact_undo` | `non_destructive_read_only` |
| `export` | Write a typed quarantined-copy export to the local-only export lane. Never changes the corrupt artifact in place; never silently leaves the device. | `exact_undo`, `compensating_action` | `non_destructive_writes_local_evidence_only` |
| `rebuild` | Recreate disposable derived state from authoritative workspace bytes. ONLY allowed when `artifact_value_posture` is `derived_disposable_only`. Never used on `mixed_user_and_derived`, user-authored, evidence, or install-chain artifacts. | `regeneration` | `writes_disposable_state_only` |
| `restore` | Bring the artifact back from `authoritative_backup`, `sync_replica`, or `local_checkpoint`. MUST cite exactly one covering recovery-promise class. `mirror_cache` and `convenience_export` MUST NOT cover a restore. | `checkpoint_restore`, `compensating_action` | `mutates_workspace_bytes_with_checkpoint`, `mutates_profile_state_with_checkpoint_and_export` |
| `replace` | Install a fresh default in place of the corrupt artifact when no covering promise exists. MUST be gated behind a quarantined-copy preservation and a no-undo-export-only acknowledgement. Source class MUST be `default_initialization_template`. | `no_undo_export_only` | `destructive_user_authored_no_undo_export_required` |
| `discard` | Remove or zero the corrupt artifact entirely without an authoritative restore source. MUST be gated behind a quarantined-copy preservation and a no-undo-export-only acknowledgement. The bytes are gone after the action; only the quarantined copy remains. | `no_undo_export_only` | `destructive_user_authored_no_undo_export_required` |

A surface that says "discard" when it means "rebuild" — or "replace"
when it means "restore" — is non-conforming. The schema enforces
each row.

## 5. Rescue-action classes

Closed eight-class vocabulary. The schema binds each action to
exactly one verb so the sheet's `available_actions[]` and
`recommended_action` cannot drift.

| Action class | Verb |
|---|---|
| `inspect_only` | `inspect` |
| `export_quarantined_copy` | `export` |
| `rebuild_disposable_state` | `rebuild` |
| `restore_from_authoritative_backup` | `restore` |
| `restore_from_sync_replica` | `restore` |
| `restore_from_local_checkpoint` | `restore` |
| `replace_with_default_after_quarantine` | `replace` |
| `discard_after_export_acknowledged` | `discard` |

## 6. Healthy-candidate sources

Closed six-class vocabulary. Every sheet either names one source or
names the absence explicitly. Free-text "we'll figure it out" is
non-conforming.

- `covering_authoritative_backup` — verified authoritative backup
  covers the artifact.
- `covering_sync_replica` — sync replica within the policy window
  covers the artifact.
- `covering_local_checkpoint` — local-history / workspace-authority
  checkpoint covers the artifact.
- `regeneration_from_authoritative_workspace_bytes` — the artifact
  is `derived_disposable_only` and regenerable from authoritative
  workspace bytes. Rebuild path.
- `default_initialization_template` — fresh default ships with the
  product. Replace path; only valid when no covering promise exists
  AND a quarantined copy has been preserved.
- `no_healthy_candidate_available` — none of the above covers the
  artifact. The sheet's recommended action MUST be `inspect_only` or
  `export_quarantined_copy`; `replace` and `discard` are forbidden
  in this state.

## 7. Compare preview

Every sheet carries a `compare_preview` block with three required
parts:

- `corrupt_witness` — opaque identity token, integrity-witness class
  (see §10.3), redaction class, short summary.
- `healthy_witness` — required when `healthy_candidate.source_class`
  is not `no_healthy_candidate_available`. Carries the source
  identity token, the source class, redaction class, and a summary.
- `post_action_state_rows[]` — one row per affected scope drawn from
  the recovery-scenario `affected_scope_class` set, plus an
  additional `derived` and `credentials` axis for sheet-local
  precision. Each row binds:
  - `scope_class` — `workspace`, `profile`, `evidence`, `layout`,
    `credentials`, or `derived`.
  - `post_action_state_class` — `kept_intact`,
    `retained_in_quarantine`, `regenerated_from_authoritative`,
    `restored_from_covering_promise`, `replaced_with_default`,
    `discarded_after_export`, `exported_only_no_change`.
  - `summary` — short, redaction-aware text.

The schema bounds the rows array length so a sheet cannot silently
expand into an undocumented blast radius.

## 8. Preservation decision

The sheet's `preservation` block is the typed gate every consumer
reads. Schema enforcement (see §11):

- `preserved_copy_required` MUST be `true` when:
  - `artifact_value_posture` is one of `user_authored_durable`,
    `user_owned_recovery_state`, `mixed_user_and_derived`,
    `policy_bound_evidence`, `local_forensics_only`, or
    `install_chain_metadata`; OR
  - `recommended_action.verb_class` is `replace` or `discard`; OR
  - `preserve_reasons` contains `carries_diagnostics_value`,
    `carries_policy_bound_evidence`,
    `forensics_required_by_pending_investigation`, or
    `mixed_user_and_derived_indeterminate`.
- When `preserved_copy_required` is `true`, `preserved_copy_ref`
  MUST be present and MUST resolve to a
  `quarantined_copy_record` produced under
  [`/schemas/recovery/quarantined_copy_record.schema.json`](../../schemas/recovery/quarantined_copy_record.schema.json).
- `preserve_reasons` MUST list at least one closed reason class when
  `preserved_copy_required` is `true`.

`preserved_copy_required: false` is permitted only when
`artifact_value_posture` is `derived_disposable_only` AND
`preserve_reasons` does NOT contain a forensics or
diagnostics-value class AND `recommended_action.verb_class` is one
of `inspect`, `export`, or `rebuild`.

## 9. Recommended action

The sheet's `recommended_action` is one row drawn from
`available_actions[]` with extra preservation-gating fields:

- `action_class`, `verb_class`, `reversibility_class`,
  `destructive_risk_class` — bound by §4 / §5.
- `covering_promise_class` — required when `verb_class` is
  `restore`. MUST be drawn from
  `authoritative_backup`, `sync_replica`, or `local_checkpoint`;
  `mirror_cache` and `convenience_export` are rejected by the
  schema.
- `must_precede[]` — required when `verb_class` is `replace`,
  `discard`, or `restore` (with mutates_* risk). Drawn from
  `export_quarantined_copy`, `capture_local_checkpoint`,
  `inspect_only`. `replace` and `discard` MUST list both
  `export_quarantined_copy` AND `capture_local_checkpoint` (the
  workspace-authority checkpoint precedes the destructive replace
  or discard).
- `no_undo_acknowledgement_required` — boolean. MUST be `true` when
  `destructive_risk_class` is
  `destructive_user_authored_no_undo_export_required`.
- `summary` — short, redaction-aware text.

## 10. Quarantined-copy record

One record per preserved suspect artifact. Every consumer reads
exactly the fields below and no others.

### 10.1 Fields

| Field | Purpose |
|---|---|
| `copy_id` | Stable id the rescue sheet, repair-preview, evidence packet, and CLI cite. |
| `captured_at` | Producer-local monotonic timestamp. |
| `origin` | Typed origin block (see §10.2). |
| `sensitivity_class` | Closed `data_class_boundary_class` (`metadata_only`, `environment_adjacent`, `code_adjacent`, `high_risk`). |
| `retention_posture` | Closed `retention_class` plus a short retention-window summary plus a closed `clearance_owner_class`. |
| `identity` | Closed `integrity_witness_class` plus an opaque `identity_token`. |
| `visibility_class` | Closed `user_visible`, `support_visible`, or `local_forensics_only`. |
| `preserve_reasons[]` | At least one closed `preserve_reason_class`. |
| `allowed_inspectable_actions[]` | Closed inspectable-action set the surface MAY offer (see §10.4). |
| `linkage` | Typed refs into the rescue sheet, repair transaction, evidence packet, and recovery-scenario card. |
| `honesty_invariants` | Const guarantees (see §10.5). |

### 10.2 Origin

`origin` is a typed block:

- `corrupt_artifact_class` — same closed vocabulary as the rescue
  sheet (§2).
- `artifact_origin_class` — closed: `workspace_local`,
  `profile_local`, `install_chain_local`, `evidence_local`,
  `mirror_or_offline_bundle_local`, `credentials_store_local`.
- `preserved_path_ref` — opaque ref. The contract NEVER carries a
  raw absolute path.
- `preserved_storage_class` — closed: `local_forensics_lane`,
  `local_quarantine_lane`, `support_bundle_attached_local`,
  `evidence_packet_attached_local`.
- `summary` — short, redaction-aware text.

### 10.3 Identity

Closed `integrity_witness_class`:

- `checksum_recorded` — a content checksum is recorded.
- `signature_recorded` — a signature over the artifact is recorded.
- `content_addressed` — the storage backend is content-addressed.
- `policy_redacted_witness` — the witness is recorded but redacted
  by policy.
- `no_integrity_witness` — no integrity witness is available; the
  copy is opaque.

`identity_token` is an opaque ref; raw hashes that may identify the
content are NOT carried. Surfaces that need the actual checksum
read it from the backing store under their own contract.

### 10.4 Inspectable actions

Closed `inspectable_action_class`:

- `inspect_metadata_only` — the surface MAY render the metadata
  card; the bytes themselves are not exposed.
- `export_to_user_export_lane` — the user MAY export the copy to
  the local-only convenience export lane.
- `export_to_support_bundle` — the support pipeline MAY attach the
  copy to a support bundle, subject to redaction class.
- `attach_to_evidence_packet` — the copy MAY be attached to an
  evidence packet for policy-bound evidence.
- `hold_for_pending_investigation` — the copy is held; no export
  path is offered until the investigation is resolved.

The schema enforces that `local_forensics_only` visibility excludes
`export_to_user_export_lane`, and that `policy_bound_evidence`
preserve reason MUST include either `attach_to_evidence_packet` or
`hold_for_pending_investigation`.

### 10.5 Honesty invariants

Every quarantined-copy record MUST carry the `honesty_invariants`
block with five const-`true` fields:

- `origin_is_typed: true` — origin uses the closed vocabulary, not
  free-form prose.
- `sensitivity_class_named: true` — sensitivity is one of the
  closed redaction classes.
- `retention_owner_named: true` — `clearance_owner_class` is set.
- `visibility_is_explicit: true` — `visibility_class` is set; the
  copy is never silently visible to support or admin.
- `never_exits_device_silently: true` — convenience exports of the
  copy never silently leave the device.

## 11. Honesty invariants — rescue sheet

Every sheet MUST carry the `honesty_invariants` block with seven
const-`true` fields:

- `corrupt_artifact_class_is_closed: true` — the sheet resolves to
  exactly one closed corrupt-artifact class and never collapses
  two artifacts into one row.
- `artifact_value_posture_named: true` — the sheet declares one
  closed value posture; the user is never told the system is
  broken without a named anchor describing what value the artifact
  still carries.
- `preserved_copy_before_destructive_action: true` — any
  `replace` or `discard` action is gated behind a typed
  quarantined-copy ref AND an `export_quarantined_copy` pre-action
  AND a `capture_local_checkpoint` pre-action AND a
  no-undo acknowledgement.
- `healthy_candidate_source_named: true` — the sheet either names
  one healthy-candidate source or names
  `no_healthy_candidate_available` explicitly.
- `retained_versus_replaced_explicit: true` — the compare preview
  carries one closed post-action-state class per affected scope.
- `no_overpromise_of_reversibility: true` — verb wording binds
  reversibility class; `replace` and `discard` are
  `no_undo_export_only` and never claim `compensating_action`.
- `linkage_is_typed: true` — linkage refs are typed (probe family,
  repair-transaction ref, recovery-scenario card ref, scenario-
  picker row ref, export artifact ref) rather than free-form prose.

These are const guarantees in the schema. Any surface that emits a
sheet without them is non-conforming.

## 12. Surface rules

Apply to every surface that renders, logs, exports, or reasons
about corruption-rescue sheets and quarantined-copy records.

1. **No surface invents a private artifact class.** Every consumer
   resolves to one of the closed ten corrupt-artifact classes;
   surfaces do not render a parallel "broken state" or "generic
   corrupt file" class.
2. **One verb per recommended action.** Sheets do not say "rebuild
   and discard" or "restore and replace" in one breath. If two
   verbs are needed, two sheets or one sheet plus a typed
   pre-action is the shape; chained verbs are non-conforming.
3. **No destructive action before quarantine.** A sheet's
   `recommended_action` whose `destructive_risk_class` is
   `destructive_user_authored_no_undo_export_required` MUST cite a
   `preserved_copy_ref` AND list `export_quarantined_copy` and
   `capture_local_checkpoint` in `must_precede[]` AND set
   `no_undo_acknowledgement_required: true`.
4. **Rebuild is disposable-state only.** A sheet whose
   `recommended_action.verb_class` is `rebuild` MUST set
   `artifact_value_posture: derived_disposable_only` AND
   `healthy_candidate.source_class:
   regeneration_from_authoritative_workspace_bytes`. Rebuild on
   `mixed_user_and_derived`, user-authored, evidence, or
   install-chain artifacts is non-conforming.
5. **Restore requires an authoritative covering promise.** A sheet
   whose `recommended_action.verb_class` is `restore` MUST cite
   `authoritative_backup`, `sync_replica`, or `local_checkpoint` as
   `covering_promise_class`. `mirror_cache` and
   `convenience_export` are rejected by the schema.
6. **Replace requires a default template AND quarantine.** A sheet
   whose `recommended_action.verb_class` is `replace` MUST cite
   `default_initialization_template` as the healthy-candidate
   source AND a quarantined-copy ref. Replace without a fresh
   default — or without a preserved copy — is non-conforming.
7. **Discard requires quarantine AND export acknowledgement.** A
   sheet whose `recommended_action.verb_class` is `discard` MUST
   cite a quarantined-copy ref AND list `export_quarantined_copy`
   AND `capture_local_checkpoint` in `must_precede[]` AND set
   `no_undo_acknowledgement_required: true`.
8. **No healthy candidate forbids destructive verbs.** When
   `healthy_candidate.source_class` is
   `no_healthy_candidate_available`, the sheet's recommended
   action MUST be `inspect_only` or `export_quarantined_copy`;
   `replace`, `discard`, `rebuild`, and `restore` are forbidden in
   this state.
9. **Local-forensics-only never silently widens visibility.** A
   quarantined-copy record whose `visibility_class` is
   `local_forensics_only` MUST NOT list
   `export_to_user_export_lane` in `allowed_inspectable_actions`.
10. **Linkage stays typed.** Every sheet cites typed refs for
    Project Doctor, repair-transaction preview, recovery-scenario
    card, support intake, and the export-before-destructive path.
    Free-text linkage prose is non-conforming.

## 13. Composition with adjacent contracts

- The recovery-scenario card describes the **family** of recovery
  the user is in. The corruption-rescue sheet is the **per-artifact
  inspectable body** that family card cites when the family is
  one of `profile_corruption`, `workspace_index_corruption`,
  `failed_update`, or any other family that can produce a corrupt
  artifact. The sheet cites the scenario card by
  `linkage.recovery_scenario_card_ref`; it never re-derives
  scenario-family truth.
- The continuity-status card describes recoverability *posture*;
  the rescue sheet projects one corrupt artifact's recovery into a
  healthy-candidate row. The sheet cites the continuity-status card
  by `linkage.continuity_status_card_ref`; it never re-derives
  recovery-promise truth.
- The local-history restore-preview contract owns identity-relation
  vocabulary for restore previews. The rescue sheet cites a
  restore-preview ref via `linkage.restore_preview_ref` when
  `verb_class` is `restore`; it never re-derives
  identity-relation classes.
- The Project Doctor packet contract owns finding codes and probe
  families. The rescue sheet cites a `probe_family_class` and
  optional `doctor_finding_code_refs[]`; it never invents a
  parallel finding taxonomy.
- The repair-transaction contract owns repair-preview, apply, and
  rollback. The rescue sheet cites a `repair_transaction_ref` /
  `repair_preview_ref`; it never carries repair payload bodies.
- The support-intake / escalation contract owns scenario picker
  and escalation-packet completeness. The rescue sheet cites a
  `scenario_picker_row_ref` and a completeness floor; it never
  re-defines packet shape.
- The recovery-ladder packet contract owns rung sequence and
  reversal-class vocabulary. The rescue sheet's
  `reversibility_class` reuses that vocabulary verbatim.
- The update-and-rollback contract owns previous-install rollback.
  The `update_install_metadata` artifact class names
  `install_chain_metadata` posture; the rescue sheet never
  re-derives rollback eligibility.
- The portable-state package and restore-provenance contracts own
  export shape. The quarantined-copy record cites a
  `support_bundle_ref` or `evidence_packet_ref` when its
  `allowed_inspectable_actions` permit attachment; it never
  carries export payload bodies.

## 14. Acceptance

- Destructive corruption repair (`replace` or `discard`) preserves
  a typed quarantined copy whenever the artifact still carries
  user-owned data, diagnostics value, or policy-bound evidence. The
  schema rejects the destructive variants without a
  `preserved_copy_ref`, an `export_quarantined_copy` pre-action, a
  `capture_local_checkpoint` pre-action, and a
  no-undo-acknowledgement.
- Compare sheets make healthy-candidate source AND
  retained-versus-replaced state explicit before action. The
  schema requires a non-empty `compare_preview.post_action_state_rows`
  list and either a typed `healthy_witness` or an explicit
  `no_healthy_candidate_available`.
- The fixtures under
  [`/fixtures/recovery/corruption_rescue_cases/`](../../fixtures/recovery/corruption_rescue_cases/)
  cover at least:
  - a corrupted workspace search index (`derived_disposable_only`,
    rebuild path),
  - a suspect profile artifact (`user_authored_durable`,
    quarantine-then-restore path),
  - a failed update metadata case (`install_chain_metadata`,
    quarantine-then-export-then-rollback path),
  - a local-forensics-only preserved copy (no healthy candidate;
    inspect-only path with a held quarantined copy).

## 15. Changing this vocabulary

- **Additive-minor** changes (new corrupt-artifact class, new
  artifact-value posture, new healthy-candidate source class, new
  post-action-state class, new preserve-reason class, new
  retention class, new visibility class, new inspectable-action
  class) land in this document, the two schemas, and the fixtures
  in the same change. The change must cite the motivating fixture
  or packet.
- **Repurposing** a corrupt-artifact class, value posture, verb,
  destructive-risk class, or honesty invariant is **breaking**. It
  opens a new decision row and supersedes the relevant section of
  this document.
- The schemas are the boundary. Any surface that adds a private
  field, collapses two artifact classes, or emits a sheet or
  quarantined-copy record without the `honesty_invariants` block is
  non-conforming.

## Source anchors

- `.t2/docs/Aureline_PRD.md` §5.24, §5.53 — recoverability and
  continuity claims; large-failure repair MUST preserve suspect
  state when it may carry user value or forensics value.
- `.t2/docs/Aureline_Technical_Architecture_Document.md` §9.6, §9.7,
  Appendix CP — control-plane / data-plane separation, recovery
  posture, and the local-history / checkpoint matrix.
- `.t2/docs/Aureline_Technical_Design_Document.md` — Project
  Doctor, repair-transaction, and support-intake component designs.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md` §15 — recovery and
  restore preview surfaces; destructive actions follow safe-
  remainder, export-before-reset, and quarantine rendering.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md` §1202 —
  session restore, autosave, and crash-loop recovery preserve
  unsaved local content; rescue copy stays verb-coded and
  preservation-gated.

## Linked artifacts

- Schema (compare sheet):
  [`schemas/recovery/corruption_rescue_sheet.schema.json`](../../schemas/recovery/corruption_rescue_sheet.schema.json).
- Schema (quarantined copy):
  [`schemas/recovery/quarantined_copy_record.schema.json`](../../schemas/recovery/quarantined_copy_record.schema.json).
- Worked-example fixtures:
  [`fixtures/recovery/corruption_rescue_cases/`](../../fixtures/recovery/corruption_rescue_cases/).
- Recovery-scenario contract:
  [`docs/reliability/recovery_scenario_contract.md`](./recovery_scenario_contract.md).
- Continuity-status card contract:
  [`docs/reliability/continuity_status_card_contract.md`](./continuity_status_card_contract.md).
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
- Update-and-rollback contract:
  [`docs/release/update_and_rollback_contract.md`](../release/update_and_rollback_contract.md).
- State-object inventory:
  [`docs/state/state_object_inventory.md`](../state/state_object_inventory.md).
