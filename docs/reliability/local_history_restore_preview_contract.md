# Local-history snapshot card, restore preview, and retention/export contract

This document freezes the inspectable contract that sits above the
local-history timeline record. It gives recovery surfaces one shared
answer to three questions before any restore or clear-history write:

1. What kind of checkpoint am I viewing?
2. What object, range, file, or checkpoint group would a restore touch?
3. What may be retained, exported, redacted, or cleared?

Companion artifacts:

- [`/schemas/recovery/local_history_snapshot_class.schema.json`](../../schemas/recovery/local_history_snapshot_class.schema.json)
  - user-facing snapshot-class cards for local-history rows.
- [`/schemas/recovery/restore_preview.schema.json`](../../schemas/recovery/restore_preview.schema.json)
  - restore-preview records for inspect, range/hunk restore,
  whole-file restore, grouped restore, and export-only recovery.
- [`/schemas/recovery/local_history_retention_card.schema.json`](../../schemas/recovery/local_history_retention_card.schema.json)
  - retention/export cards and clear-history scope selectors.
- [`/fixtures/recovery/restore_preview_cases/`](../../fixtures/recovery/restore_preview_cases/)
  - worked JSON cases covering exact identity, same-path drift,
  alias/canonical drift, generated targets, managed mirrors,
  current-object-missing, grouped restore, and retention/export review.
- [`/docs/reliability/local_history_contract.md`](./local_history_contract.md)
  - underlying local-history entry, group, and clear-scope record.

This contract is a projection, not a second storage model. A surface
reads the underlying `local_history_entry` or `local_history_group_record`,
then emits one or more of the records defined here so the user, support
bundle, CLI, and evidence packet see the same checkpoint class, target
identity, action availability, redaction posture, and future checkpoint
lineage.

## Source anchors

- `.t2/docs/Aureline_Technical_Architecture_Document.md:1771` -
  local history is a first-class attributable timeline.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1783` -
  compare and restore use the same review surfaces as Git, refactor,
  and structured-artifact review; restore goes through save,
  journaling, and external-change checks.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:9386` -
  file, group, automation, external-state, and pruned/redacted
  timelines have distinct compare, restore, retention, and export rules.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:15383` -
  local-history checkpoint rows must show time, actor, scope, mutation
  class, and compare action.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:15395` -
  restore preview cards must show identity, drift, restore modes, and
  that restore creates a new checkpoint.
- `.t2/docs/Aureline_UI_UX_Spec_Document.md:15761` -
  storage cleanup must name protected local-history and support evidence
  exclusions.

## Scope

Frozen here:

- visible snapshot classes that a checkpoint row or card renders;
- actor and source fields that preserve who or what produced a
  checkpoint without re-reading every mutation-journal body;
- export metadata on snapshot cards so support, evidence, and patch
  exports do not infer redaction or locality from prose;
- restore-preview identity fields for exact object identity,
  same-path/different-object drift, alias/canonical drift, generated
  targets, managed mirrors, and missing current objects;
- restore scope granularity for inspect-only, selected range, selected
  hunks, whole file, grouped checkpoint, and export-only operations;
- grouped-checkpoint implications before apply;
- retention/export cards naming retention class, redaction posture,
  support-bundle inclusion state, local-only/exported state, and
  clear-history scope selectors.

Out of scope:

- byte-level diff rendering;
- final visual layout;
- content-addressable storage internals;
- restore algorithms, merge algorithms, and long-term storage GC.

## 1. Visible snapshot classes

The underlying `snapshot_class` in the local-history entry remains the
storage and lineage class. The visible snapshot card adds the class a
reviewer sees on a timeline row, diff header, restore preview, support
bundle, or CLI listing.

| Visible class | Meaning | Typical underlying classes |
|---|---|---|
| `autosaved_typing_bucket` | A short-lived bucket of typing, paste, selection edit, or autosave captures. | `edit_save_checkpoint` |
| `save_boundary` | A deliberate save or formatter/save-participant boundary that can be compared as a file state. | `edit_save_checkpoint`, `workspace_mutation_checkpoint` |
| `pre_destructive_checkpoint` | A checkpoint created before overwrite, reload-with-discard, repair, migration, refactor, package, or other destructive-looking action. | `workspace_mutation_checkpoint`, `automation_ai_checkpoint`, `restore_rollback_checkpoint` |
| `tool_generated_checkpoint` | A checkpoint produced by AI apply, codegen, recipe, macro, migration, scaffold, or generated-artifact refresh. | `automation_ai_checkpoint`, `workspace_mutation_checkpoint`, `capture_omitted_stub`, `policy_redacted_stub` |
| `crash_restore_capture` | A crash, journal, repair, or local-history restore capture. The row must distinguish source capture from the new restore checkpoint. | `external_state_checkpoint`, `restore_rollback_checkpoint`, `edit_save_checkpoint` |

Each card carries:

- `actor` - actor class, display label, stable actor ref, and whether
  the actor is user, automation, AI/tool, external detector, repair, or
  restore runner;
- `source` - source class, truth source class, origin surface, and
  source refs such as mutation-journal entry id, group id, command id,
  external-change event id, crash journal id, or support/evidence ref;
- `checkpoint_identity` - entry/group ids, logical document ids,
  branch/worktree context, capture mode, and group membership;
- `export_metadata` - redaction class, retention class, support-bundle
  inclusion state, local/exported state, allowed export actions, export
  refs, and local-only invariants.

Cards must not use planning ids, raw absolute paths, raw file bodies, raw
diff hunks, raw prompts, raw credentials, or opaque provider payloads as
labels. Cards use opaque refs plus redaction-aware display text.

## 2. Restore preview record

A restore preview is required before any restore writes bytes, clears a
target, replaces a generated artifact, or applies a grouped checkpoint.
The preview record answers four questions in structured fields:

| Question | Required fields |
|---|---|
| What is the source? | `source_checkpoint`, visible snapshot class, source entry/group refs, body availability, redaction class. |
| What is the current target? | `target_object`, current object state, filesystem identity refs, generated/mirror posture, managed constraints. |
| What drift exists? | `identity_comparison.object_identity_relation` plus booleans for exact identity, same-path/different-object, alias/canonical drift, and current-object-missing. |
| What can be done? | `restore_scope`, `action_availability[]`, overwrite/preserve summary, grouped implications, and `new_checkpoint_on_apply`. |

The closed identity relation vocabulary is:

- `exact_object_identity` - source-time and current identity name the
  same canonical object.
- `same_path_different_object` - the presentation path matches but the
  canonical object differs, for example inode reuse or provider object
  rotation.
- `alias_canonical_drift` - the target is reachable through an alias,
  symlink, case variant, mount, provider alias, or canonical path drift.
- `generated_target` - the current object is generated or derived; the
  preview must show the canonical source/regeneration relation before a
  direct restore can be offered.
- `managed_mirror` - the target is a managed mirror or provider-owned
  copy; direct writes are blocked unless an owning adapter exposes a
  reviewed promotion path.
- `current_object_missing` - the logical document or group member no
  longer resolves to a current object; the preview can inspect/export or
  restore-as-new only when policy and scope allow.

`restore_scope.granularity_class` is closed:

- `inspect_only`
- `selected_range`
- `selected_hunks`
- `whole_file`
- `grouped_checkpoint`
- `export_only`

`action_availability[]` is the only place a surface advertises actions.
The closed action vocabulary is:

- `inspect_only`
- `restore_selected_range`
- `restore_selected_hunks`
- `restore_whole_file`
- `restore_grouped_checkpoint`
- `export_as_patch`
- `export_as_evidence`

A disabled action remains visible with `availability_class` and
`disabled_reason_class`; hidden actions make the preview
non-reconstructable. A restore action that is allowed must name
`new_checkpoint_on_apply` so the user can see the new attributable
checkpoint that will be written if they continue.

## 3. Grouped checkpoint implications

Grouped checkpoints are not a file-sized action disguised as a group.
When `grouped_checkpoint_implications.applies_to_group` is true, the
preview must expose:

- source group id and group kind;
- total member count, affected member count, and unavailable member
  count;
- whether the selected scope restores the whole group, selected
  members, selected hunks across members, or export-only evidence;
- member implication rows with each member's target relation and action
  availability;
- whether applying the preview creates a new group record and member
  restore entries;
- whether any member is blocked by generated-source, managed-mirror,
  missing-object, policy, or body-unavailable constraints.

The preview must not summarize a multi-file restore as "restore all" if
any member is inspect-only, export-only, or blocked.

## 4. Generated and managed target constraints

Generated and managed files keep their source-of-truth posture visible
in the restore preview.

Generated targets carry `generated_target_relation`:

- artifact origin class;
- provenance state;
- canonical source refs;
- regeneration action ref;
- whether direct restore would diverge from the generator;
- whether restore is blocked, requires redirect to canonical source,
  or is allowed only as an export/evidence action.

Managed mirrors carry `managed_mirror_constraint`:

- mirror owner class;
- provider or mirror identity ref;
- local write posture;
- promotion or refresh action ref;
- support/export posture;
- why direct restore is blocked when blocked.

These fields are visible in preview, not discovered after restore.

## 5. Retention and export card

Retention/export cards are rendered from a local-history entry, group,
or preview so the user can inspect lifecycle and export posture without
opening the storage engine.

Every card carries:

- `retention_class` - the same retention reason family as the
  local-history entry (`retained_by_policy_window`,
  `retained_by_explicit_user_pin`, evidence/support/release/review
  references, or stub-only retention);
- `redaction_posture` - metadata, environment-adjacent, code-adjacent,
  or high-risk body posture, plus whether the current display is full,
  summary-only, redacted, local-only, or prohibited;
- `support_bundle_inclusion_state` - excluded by default, included by
  entry ref, review required, retained local-only, exported in bundle,
  or prohibited;
- `local_export_state` - local-only, exported as patch, exported as
  evidence, exported in support bundle, managed copy only, or export
  blocked;
- `clear_history_scope_selectors[]` - one selector for each allowed or
  blocked clear-history scope (`this_file`, `this_workspace`,
  `this_profile`, `this_device`) with target refs, counts, warning
  classes, and export-before-delete posture.

The clear-history selector is not the clear operation. The clear
operation remains the `local_history_clear_scope_record` defined in the
local-history contract. The selector lets a review surface say what would
be affected, what would be stubbed, what would be skipped because of
retention refs, and which export-before-delete path is available before
the clear is confirmed.

## 6. Surface rules

1. Snapshot cards, restore previews, and retention/export cards use the
   schemas named above. Surfaces must not add private action or drift
   keys.
2. Restore previews must show identity relation, target state, generated
   and managed constraints, scope granularity, and new-checkpoint
   lineage before any write.
3. Restore actions create a new attributable checkpoint or group record.
   A preview that claims a restore can rewrite the source entry is
   non-conforming.
4. Inspect-only and export actions remain available whenever metadata is
   available, even if body restore is blocked by redaction, missing
   object, generated-source, managed-mirror, or policy constraints.
5. Clear-history scopes are explicit and separate. A card that combines
   file, workspace, profile, or device scopes without separate selectors
   is non-conforming.
6. Support bundles and evidence packets cite entry, group, preview,
   retention-card, or export artifact refs rather than embedding raw
   bodies unless a reviewed redaction profile explicitly allows it.

## 7. Fixture expectations

The fixtures under
[`/fixtures/recovery/restore_preview_cases/`](../../fixtures/recovery/restore_preview_cases/)
are reviewable without a UI. They must show:

- at least one card for each visible snapshot class;
- at least one preview for exact object identity;
- one preview where the same presentation path resolves to a different
  current object;
- one preview with alias/canonical drift and grouped checkpoint
  implications;
- one preview that exposes generated-target constraints before any
  direct write;
- one preview that exposes managed-mirror and current-object-missing
  constraints;
- one retention/export card with support-bundle and clear-history scope
  selectors.

Adding a new action, identity relation, clear scope, redaction posture,
or support-bundle inclusion state is additive only when this document,
the relevant schema, and at least one fixture update in the same change.
