# Local-history snapshot / index contract, attribution lineage, and clear-history scope rules

This document is the **cross-tool contract** for the local-history
recovery timeline. The local-history lane is a first-class,
attributable timeline of user-, tool-, and automation-originated
mutations that remains available even when a user never committed,
stashed, or explicitly saved an intermediate state. It is distinct
from the dirty-buffer recovery journal, autosave journals, Git
history, automation lineage, external-change records, review-state
checkpoints, and synced or provider history.

The machine-readable schema lives at:

- [`/schemas/recovery/local_history_entry.schema.json`](../../schemas/recovery/local_history_entry.schema.json)

The mutation-lineage vocabulary reused here is frozen at:

- [`/docs/workspace/mutation_lineage_model.md`](../workspace/mutation_lineage_model.md)
- [`/schemas/workspace/mutation_journal.schema.json`](../../schemas/workspace/mutation_journal.schema.json)
- [`/schemas/workspace/generated_artifact_lineage.schema.json`](../../schemas/workspace/generated_artifact_lineage.schema.json)

The state-object, storage-class, and cache-clear rules reused here
are frozen at:

- [`/artifacts/state/state_objects.yaml`](../../artifacts/state/state_objects.yaml) (`local_history` row)
- [`/docs/state/state_object_inventory.md`](../state/state_object_inventory.md)
- [`/artifacts/runtime/storage_classes.yaml`](../../artifacts/runtime/storage_classes.yaml)
- [`/docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md)

The clipboard / reopen-history / undo-group contract that local
history rides alongside is at:

- [`/docs/ux/clipboard_history_contract.md`](../ux/clipboard_history_contract.md)

TAD Appendix CP (`Local History, Timeline, and Reversible
Checkpoint Matrix`) wins on any disagreement; this document and the
schema are updated in the same change.

## Why freeze this now

Local history touches the most sensitive promise the IDE makes:
*you will not lose work*. Left implicit, each surface — timeline
lane, compare view, restore preview, clear-history review, support
bundle, AI evidence packet, activity centre — will answer *what was
this snapshot, really?* slightly differently. One surface treats
local history as indistinguishable from crash-journal replay;
another folds it into Git history; a third silently evicts it when
the user runs a generic "clear cache".

The goal here is one frozen shape so local history is visibly
**a separate truth source** on the timeline, **attributable** to one
mutation journal lineage id, **explainable** when bytes are not
captured, and **safe** against ordinary cache-clear semantics.

## Scope

- Freeze one `local_history_entry` shape that covers
  edit / save checkpoints, workspace mutation checkpoints,
  automation / AI checkpoints, external-state checkpoints,
  restore / rollback checkpoints, and two stub classes for
  omitted or policy-redacted captures.
- Freeze one `local_history_group_record` shape for named
  operations (refactor apply, AI patch, scaffolding run, migration
  import, generated-artifact refresh, automation recipe run,
  multi-file rename, import / paste group, restore group).
- Freeze one `local_history_clear_scope_record` shape for every
  scoped clear-history action.
- Reuse the ADR-0003 undo-class axis and the mutation-journal
  reversal-class axis unchanged; carry them on every entry by link
  to `mutation_journal_link.linked_id` rather than duplicating.
- Freeze the **truth-source-class** axis that visibly distinguishes
  local history from autosave journals, Git history, automation
  lineage, external-change records, review checkpoints, and
  synced / provider history on the timeline.
- Freeze the **capture-mode** axis (content-addressed snapshot,
  metadata-plus-reference only, group-manifest only, external-cause
  metadata only) with a closed omission-reason vocabulary so every
  omission or redaction is inspectable.
- Freeze the **logical-document identity** that follows a document
  across rename, move, and identity drift, plus the
  **restore-preview minimum fields** a preview must carry when
  canonical identity drifted.
- Freeze the **clear-history scope vocabulary** (`this_file`,
  `this_workspace`, `this_profile`, `this_device`) with an
  export-before-delete decision, a local-only-by-default invariant,
  and the ordinary-cache-clear exclusion rule.
- Seed worked fixtures covering typing, paste / import,
  AI apply, automation recipe, repair (decode recovery),
  external-change, a restore that emits a new attributable
  checkpoint, and a scoped clear-history action.

## Out of scope

- The final storage engine internals (content-addressable object
  store layout, on-disk segment format, GC interactions). The
  contract here is the **record shape**, not the bytes on disk.
- Diff UI polish, compare surface styling, timeline animation, and
  other rendering details. The contract fixes what the surfaces
  must read, not how they paint pixels.
- Sync / export implementation beyond the contract. Local history
  is local-first; any future sync or export engine must be an
  additive extension that respects `local_only_posture`.
- Per-generator regeneration playbooks beyond the `reference_digest`
  + `capture_mode = metadata_plus_reference_only` handshake.

## 1. Record model

Three record shapes. Every local-history surface reads exactly
these three and no others.

### 1.1 `local_history_entry`

One entry per capture, omission stub, redaction stub, or restore.
Required fields:

| Field                          | Purpose                                                                                       |
|--------------------------------|-----------------------------------------------------------------------------------------------|
| `entry_id`                     | Stable id. Support bundles, evidence packets, compare handles, and restore previews cite it. |
| `group_id` (optional)          | Present when the entry is a member of a named operation.                                      |
| `snapshot_class`               | Closed class. See §2.                                                                         |
| `captured_at`                  | Producer-local monotonic timestamp.                                                           |
| `truth_source_class`           | Always `local_history` for entries authored by this lane. See §3.                             |
| `logical_document_identity`    | Logical document id + current filesystem identity + rename/move chain. See §4.                |
| `branch_worktree_context`      | Branch / worktree / merge-rebase context at capture time. See §5.                             |
| `capture_descriptor`           | Capture mode + omission reason + body availability + object refs. See §6.                     |
| `mutation_journal_link`        | Journal entry or group id, actor class, source class, reversal class, redaction class. See §7.|
| `restore_of_ref`               | Required on `restore_rollback_checkpoint` entries. See §8.                                    |
| `retention_scope`              | Why the entry is still on the timeline. See §9.                                               |
| `local_only_posture`           | Local-only-by-default + sync exclusion + ordinary-cache-clear exclusion. See §10.             |

### 1.2 `local_history_group_record`

One group per named operation. Members are `local_history_entry`
rows by `entry_id`. The group's `snapshot_class` is usually
`workspace_mutation_checkpoint` or `automation_ai_checkpoint`;
restore groups carry `restore_rollback_checkpoint` and populate
`restore_of_ref`.

### 1.3 `local_history_clear_scope_record`

One record per scoped clear-history action. Records the scope,
target, confirmation, export-before-delete decision, removed /
stubbed / skipped counts, and the invariant that no ordinary
cache-clear flow authored the action.

## 2. Snapshot class

The closed `snapshot_class` vocabulary is:

| Class                               | Meaning                                                                                                                                                                   |
|-------------------------------------|---------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
| `edit_save_checkpoint`              | Edit / save / autosave / explicit format-on-save / selection-scoped apply.                                                                                                |
| `workspace_mutation_checkpoint`     | Multi-file refactor, bulk replace, multi-file rename, import, generated-config rewrite.                                                                                   |
| `automation_ai_checkpoint`          | AI apply, recipe replay, macro run, scaffolding step, migration / import action.                                                                                          |
| `external_state_checkpoint`         | Branch switch, worktree switch, merge / rebase conflict, external overwrite detection. Observation-only; not a mutation by Aureline.                                      |
| `restore_rollback_checkpoint`       | A compare-and-restore, revert-to-checkpoint, or cherry-pick from local history. The restore itself writes one of these, citing the entry it restored from.                 |
| `capture_omitted_stub`              | No body was captured (too large, binary, generated, managed, excluded path, quota exceeded, or unsupported filesystem semantics). The stub still appears on the timeline. |
| `policy_redacted_stub`              | Body capture was blocked by policy for secret-adjacent or high-risk artifacts. The stub still appears on the timeline with a redaction note.                              |

Restore (`restore_rollback_checkpoint`) is the only class that
**must** carry `restore_of_ref`. A restore never invisibly rewrites
the prior trail; it always creates a new attributable entry.

## 3. Truth-source class (timeline distinction)

The timeline lane renders rows from seven distinct truth sources
under their own closed `truth_source_class` label so the UI never
collapses them into one generic "edited" state:

| Truth source                    | Authored by                                                  | Shown on timeline as                                   |
|---------------------------------|--------------------------------------------------------------|--------------------------------------------------------|
| `local_history`                 | This lane.                                                   | The local-history row itself.                          |
| `autosave_journal`              | The dirty-buffer recovery / autosave journal (ADR 0003).     | Adjacent rows labeled as autosave / crash journal.     |
| `git_history`                   | Git (commits, branches, merges, rebases).                    | Adjacent rows labeled as Git commits.                  |
| `automation_lineage`            | Macro / recipe / scaffolding / migration lineage.            | Adjacent rows labeled with the originating recipe.     |
| `external_change_record`        | External-change detector (ADR 0006).                         | Adjacent rows labeled "external change".               |
| `synced_or_provider_history`    | Sync engines, provider review history, hosted evidence.      | Adjacent rows labeled with the sync / provider source. |
| `review_checkpoint`             | Review packs, request workspaces.                            | Adjacent rows labeled as review checkpoints.           |

Local-history rows MAY carry one or more
`timeline_distinction_labels` (for example,
`distinct_from_git_history` while a rebase is in progress) so the
surrounding context never misreads the row.

## 4. Logical document identity and rename / move

Every entry carries a `logical_document_identity`:

- `logical_document_id` — allocated on first capture, never
  changes, follows the document across every rename and move.
- `current_filesystem_identity` — the full filesystem-identity
  record (ADR 0006 layers 1–4) observed at capture time. Reuses
  `schemas/filesystem/save_target_token.schema.json#filesystem_identity_record`
  without redefinition.
- `canonical_identity_drift` — the drift class between the logical
  document id and the current canonical filesystem object:
  `no_drift`, `rename_detected`, `move_detected`,
  `rename_and_move_detected`, `device_inode_reuse_detected`,
  `provider_object_id_rotated`, `canonical_identity_unknown`, or
  null.
- `rename_move_history` — append-only log of creation, rename,
  move, restore-from-local-history, and reattach-after-drift
  events. Each event cites the mutation-journal entry that
  produced it.

**Restore preview minimum fields.** When canonical identity has
drifted, the restore preview MUST carry every field in
`restore_preview_required_fields` before apply:

- `source_entry_ref`
- `last_known_canonical_identity_ref`
- `current_canonical_identity_ref`
- `canonical_identity_drift`
- `rename_move_chain_ref`
- `body_availability`
- `resulting_snapshot_class` (always `restore_rollback_checkpoint`)
- `new_checkpoint_entry_ref` (the new attributable entry the
  restore will write on apply)

## 5. Branch / worktree context

Each entry captures the branch / worktree context in effect at
capture time: `no_vcs_context`, `git_branch`, `git_worktree`,
`git_detached_head`, `git_rebase_in_progress`,
`git_merge_in_progress`, `git_cherry_pick_in_progress`,
`external_vcs`, or `review_workspace_branch`. Opaque branch and
worktree ids plus base / parent commit digests are carried on the
entry so the compare and restore surfaces can label adjacent
external-state rows correctly — and so a restore across branches
is visible rather than silent.

## 6. Capture mode, omission, and redaction

`capture_descriptor.capture_mode` is closed:

- `content_addressed_snapshot` — the entry addresses one or more
  content-hashed blobs in the object store. `body_available` is
  true, `body_object_refs[]` is non-empty, and `omission_reason`
  is `not_omitted`.
- `metadata_plus_reference_only` — the entry captures identity and
  delta metadata plus a stable `reference_digest` (in-workspace
  path digest, generator output digest, upstream artifact digest,
  external revision token). `body_available` is false. Used for
  large files, binaries, generated artifacts whose canonical
  source carries the real lineage, and managed external artifacts.
- `group_manifest_only` — the entry is a group record that stores
  a member manifest but no bytes of its own. `body_available` is
  false.
- `external_cause_metadata_only` — the entry is an external-state
  checkpoint whose cause is a non-mutating event (branch switch,
  external overwrite). Body not captured.

`capture_descriptor.omission_reason` is closed:

- `not_omitted`
- `omitted_too_large`
- `omitted_binary_class_metadata_only`
- `omitted_generated_artifact_use_lineage`
- `omitted_managed_external_artifact`
- `omitted_excluded_path`
- `omitted_policy_redacted_secret_adjacent`
- `omitted_quota_exceeded`
- `omitted_unsupported_filesystem_semantics`

Every omission is user-visible on the timeline. The surface quotes
`omission_note` verbatim so a reviewer can distinguish *we did not
capture bytes because policy blocked it* from *we did not capture
bytes because the artifact is generated and the canonical source
carries the real lineage*.

`mutation_journal_link.redaction_class` mirrors the mutation-journal
redaction vocabulary (`metadata_only`, `environment_adjacent`,
`code_adjacent`, `high_risk`) and drives how support bundles, AI
evidence packets, and replay captures export the body.

## 7. Attribution lineage

Attribution is never duplicated; it is linked.

Every local-history entry carries one `mutation_journal_link`:

- `linked_kind` — `mutation_journal_entry`,
  `mutation_group_record`, or
  `no_mutation_journal_entry_external_cause`.
- `linked_id` — the mutation-journal id that every derived row
  (undo-history, reopen-history, activity centre, compare,
  support-bundle export) cites for the same operation. This is the
  single lineage id enforced by
  `docs/ux/clipboard_history_contract.md` §10.
- `actor_class` — re-exports the mutation-journal actor vocabulary
  and adds:
    - `paste_or_drop_import` (clipboard / drag-drop / import
      attribution required by the clipboard-and-reopen contract)
    - `automation_recipe_runner` (macro / recipe lineage)
    - `external_change_detector` (authored an external-state
      checkpoint)
    - `restore_rollback_runner` (authored a restore checkpoint)
- `source_class` — re-exports the mutation-journal source
  vocabulary unchanged.
- `reversal_class` — re-exports the mutation-journal reversal
  vocabulary and adds `no_reversal_external_event` for
  observation-only external-state checkpoints.
- `redaction_class` — mirrors the mutation-journal redaction set.

External-state checkpoints still carry a `linked_kind` of
`no_mutation_journal_entry_external_cause` so the timeline knows
there is no mutation behind the row — but the row is still
attributable to a named cause (branch switch, external overwrite,
merge-in-progress).

## 8. Restore as a new attributable checkpoint

> **Rule.** Restoring from local history creates a new attributable
> checkpoint instead of invisibly rewriting the prior trail.

Mechanically:

1. The user opens a restore preview on a source entry.
2. The preview populates `restore_preview_required_fields` (§4)
   including `new_checkpoint_entry_ref` — the id the restore will
   write on apply. When identity drifted, the preview quotes the
   drift class and the last relevant rename/move event verbatim.
3. On apply, the lane writes a new `local_history_entry` with
   `snapshot_class = restore_rollback_checkpoint` and
   `restore_of_ref.restored_from_entry_id` set to the source entry
   id. The previous entries remain on the timeline unchanged.
4. A group-scoped restore emits a `local_history_group_record` with
   `group_kind = restore_rollback_group`,
   `snapshot_class = restore_rollback_checkpoint`, and
   `restore_of_ref.restored_from_group_id` set to the source group
   id.

The new checkpoint is itself timeline-visible, citable by support
exports, and undoable by the same compare / restore surface.

## 9. Retention scope

`retention_scope` is closed:

- `retained_by_policy_window` — default local-history retention.
- `retained_by_explicit_user_pin` — the user pinned this entry.
- `retained_by_evidence_reference` — an evidence packet cites it.
- `retained_by_support_case_reference` — an open support case cites it.
- `retained_by_release_reference` — a release manifest cites it.
- `retained_by_review_pack_reference` — a review pack cites it.
- `stub_only_after_expiry` — body evicted; metadata stub remains.
- `stub_only_after_redaction` — body blocked by policy; metadata
  stub remains.

Eviction order is fixed by the storage-class contract: caches
first, unpinned derived artifacts second, local history last
within its declared retention window.

## 10. Local-only posture and ordinary-cache-clear exclusion

Every local-history record carries `local_only_posture`:

- `local_only_by_default: true` (const) — Local history is
  local-first. Exporters, sync engines, and support-bundle writers
  cannot silently flip this.
- `sync_exclusion` — one of `excluded_from_sync_by_default`,
  `opt_in_export_only`, or `support_bundle_by_entry_ref_only`. No
  mode silently publishes bodies; opt-in export ships structured
  JSON / Markdown with raw deltas separable from metadata per
  redaction profile.
- `ordinary_cache_clear_exclusion: true` (const) — Ordinary
  "clear caches" flows MUST NOT touch local-history entries. Only
  the class-specific clear-history flow defined in §11 may.

This mirrors the `local_history` rows in
[`artifacts/state/state_objects.yaml`](../../artifacts/state/state_objects.yaml)
and
[`artifacts/runtime/storage_classes.yaml`](../../artifacts/runtime/storage_classes.yaml)
(`user_owned_recovery_state`, `clear_requires_preview`,
`never ordinary cache GC`).

## 11. Clear-history scope rules

`local_history_clear_scope_record` freezes the scope vocabulary:

| Scope           | Target                                                         |
|-----------------|----------------------------------------------------------------|
| `this_file`     | One logical document (by `logical_document_id`).               |
| `this_workspace`| One workspace (by workspace id).                               |
| `this_profile`  | The user's profile-wide local history across workspaces.       |
| `this_device`   | The whole device's local history.                              |

No clear-history operation combines scopes silently. Each scoped
action writes one `local_history_clear_scope_record` carrying:

- `confirmation_class` — one of
  `export_before_delete_completed`,
  `export_before_delete_declined_by_user`,
  `export_before_delete_declined_by_policy`,
  `export_before_delete_not_offered_metadata_only`, or
  `metadata_stub_preserved_for_audit`.
- `export_before_delete` — structured record of whether the review
  sheet offered export, whether it completed, the produced export
  artifact ref, and the export redaction class. Export-before-delete
  is the **default offer** for every scope; `reason_not_offered`
  must be populated (`metadata_only_stub_scope`,
  `policy_blocks_export`, `admin_declined_export`) whenever the
  review declines to offer.
- `entries_removed_count`, `entries_stubbed_count`,
  `entries_skipped_due_to_retention_reference` — so the review
  sheet quotes *exactly* what the clear did, including how many
  entries remained untouched because they are pinned by an
  evidence packet, support case, release manifest, or review pack.
- `ordinary_cache_clear_origin: false` (const) — contract invariant
  that no ordinary cache-clear flow ever authored this action.

The clear-history review sheet (see
`docs/runtime/storage_classes_and_gc.md` §*User-owned recovery
state under explicit review*) is the only path to this record.

## 12. Surface rules

Apply to every surface that renders, logs, exports, or reasons
about the records defined above.

1. **No surface invents a private local-history field.** Every
   consumer reads `entry_id`, `group_id`, `snapshot_class`,
   `truth_source_class`, `capture_descriptor`,
   `mutation_journal_link.*`, `logical_document_identity`, and
   `local_only_posture` from the record; surfaces do not add
   parallel fields when they render.
2. **One lineage id per operation.** The undo-history row, the
   reopen-history row, the activity-centre durable job row, the
   local-history compare row, the support-bundle export, and the
   AI evidence packet all cite the **same**
   `mutation_journal_link.linked_id`. Minting a parallel id per
   surface is a contract violation (denies with
   `reopen_history_lineage_forked_per_surface` per the clipboard /
   reopen-history contract §10).
3. **Truth-source labels are canonical.** The timeline never
   collapses local-history rows with Git commits, autosave
   journal rows, automation lineage rows, external-change records,
   sync / provider history, or review checkpoints. Each renders
   under its own `truth_source_class` label.
4. **Restore creates a new attributable checkpoint.** A restore
   surface that rewrites the source entry, deletes it, or folds
   the restore into the source row is a contract violation.
5. **Omission and redaction are visible.** A capture omitted or
   policy-redacted for any reason MUST still render on the
   timeline as `capture_omitted_stub` or `policy_redacted_stub`
   with the reason quoted verbatim. Silent "no change" rendering
   is forbidden.
6. **Restore-preview minimum fields are mandatory under drift.**
   A restore preview that crosses a rename, move, identity drift,
   branch switch, or worktree switch MUST carry every field in
   `restore_preview_required_fields` before apply.
7. **Local-only posture is invariant.** `local_only_by_default`
   and `ordinary_cache_clear_exclusion` are constants. Any
   exporter, sync engine, or cache-clear flow that silently
   flips them is a contract violation.
8. **Clear-history is scoped and reviewable.** Every clear action
   writes one `local_history_clear_scope_record`. Ordinary
   cache-clear flows never author clear-history records; the
   `ordinary_cache_clear_origin` invariant denies that path at
   contract time.
9. **Export parity.** Support bundles, AI evidence packets, and
   replay captures reference entry ids rather than embedding raw
   bodies unless the redaction profile explicitly requires it.

## 13. Changing this vocabulary

- **Additive-minor** changes (new `snapshot_class` value, new
  `group_kind`, new `actor_class` re-export, new `retention_scope`
  reason, new `omission_reason`, new `truth_source_class` for a
  new adjacent truth source, new `timeline_distinction_label`)
  land here and in the schema in the same change. The change must
  cite the motivating fixture or packet.
- **Repurposing** an existing state, class, or reversal value is
  breaking. It opens a new decision row in
  [`artifacts/governance/decision_index.yaml`](../../artifacts/governance/decision_index.yaml)
  and supersedes the relevant section of this document.
- Appendix CP (TAD) wins on any disagreement with this document
  or the schema; this document and the schema are updated in the
  same change.

## 14. Acceptance

- Restoring from local history creates a new attributable
  checkpoint (`snapshot_class = restore_rollback_checkpoint`)
  instead of invisibly rewriting the prior trail. The
  `restore_creates_new_checkpoint` fixture demonstrates this
  explicitly.
- The fixtures under
  [`/fixtures/recovery/local_history_cases/`](../../fixtures/recovery/local_history_cases/)
  show typing, paste / import, AI apply, automation, repair, and
  external-change lineage **without** collapsing them into one
  generic "edited" state — each carries a distinct `actor_class`,
  `source_class`, and `snapshot_class`.
- Local history remains outside ordinary cache-clear semantics
  (`local_only_posture.ordinary_cache_clear_exclusion: true`) and
  can explain what was captured, omitted, or policy-redacted for
  large, binary, generated, or managed artifacts (worked through
  the `ai_apply_group` fixture's secret-redacted member and the
  `automation_recipe_group` fixture's generated-artifact
  metadata-plus-reference member).
- The
  [`clear_workspace_history_scope.json`](../../fixtures/recovery/local_history_cases/clear_workspace_history_scope.json)
  fixture demonstrates the closed
  `this_file | this_workspace | this_profile | this_device` scope
  vocabulary, export-before-delete offer and completion, and the
  `ordinary_cache_clear_origin: false` invariant.

## Source anchors

- `.t2/docs/Aureline_Technical_Architecture_Document.md:1771` —
  "12.4.2 Local history, timeline, and reversible checkpoint
  architecture".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1783` —
  timeline and checkpoint rules (content-addressed snapshot store
  plus metadata index; timeline UI must visibly distinguish local
  history, Git history, automation lineage, and external-change
  records).
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1788` —
  "local history remains local-first and is excluded from sync by
  default".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9386` —
  "Appendix CP — Local History, Timeline, and Reversible
  Checkpoint Matrix".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9522` —
  "User-owned recovery state: local history, checkpoints,
  journals, exports — never ordinary cache GC".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9531` —
  "never treat local history, journals, or current-case evidence
  as generic cache".

## Linked artifacts

- Mutation-journal contract:
  [`docs/workspace/mutation_lineage_model.md`](../workspace/mutation_lineage_model.md)
  and
  [`schemas/workspace/mutation_journal.schema.json`](../../schemas/workspace/mutation_journal.schema.json).
- Storage-class contract:
  [`docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md)
  and
  [`artifacts/runtime/storage_classes.yaml`](../../artifacts/runtime/storage_classes.yaml).
- State-object inventory:
  [`docs/state/state_object_inventory.md`](../state/state_object_inventory.md)
  and
  [`artifacts/state/state_objects.yaml`](../../artifacts/state/state_objects.yaml).
- Clipboard / reopen-history / undo-group contract:
  [`docs/ux/clipboard_history_contract.md`](../ux/clipboard_history_contract.md).
- Source-fidelity and undo packet:
  [`docs/verification/source_fidelity_and_undo_packet.md`](../verification/source_fidelity_and_undo_packet.md).
- Filesystem-identity vocabulary:
  [`docs/filesystem/filesystem_identity_vocabulary.md`](../filesystem/filesystem_identity_vocabulary.md).
- Local-history schema:
  [`schemas/recovery/local_history_entry.schema.json`](../../schemas/recovery/local_history_entry.schema.json).
- Worked-example fixtures:
  [`fixtures/recovery/local_history_cases/`](../../fixtures/recovery/local_history_cases/).
