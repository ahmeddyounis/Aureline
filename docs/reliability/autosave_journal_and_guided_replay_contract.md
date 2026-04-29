# Autosave journal, guided replay, and journal-reset contract

This document freezes the crash-recovery contract for dirty-buffer
autosave journals. It gives the shell, editor, safe-mode flow, support
bundle, CLI, and future tests one shared answer to four questions:

1. What recoverable work did the journal capture?
2. Which replay choices are available, blocked, or downgraded?
3. When does repeated failure stop automatic replay and enter safe mode
   or suspect-extension quarantine?
4. How can a user reset journals without confusing that action with
   clearing caches, resetting settings, or clearing local history?

Companion artifacts:

- [`/schemas/recovery/autosave_journal_entry.schema.json`](../../schemas/recovery/autosave_journal_entry.schema.json)
  - journal entries, grouped replay records, crash sentinels, and
    explicit journal-reset records.
- [`/schemas/recovery/guided_replay_choice.schema.json`](../../schemas/recovery/guided_replay_choice.schema.json)
  - restore, inspect, discard, open-without-replay, and safe-mode
    choices with blocking and downgrade reasons.
- [`/fixtures/recovery/autosave_replay_cases/`](../../fixtures/recovery/autosave_replay_cases/)
  - worked cases for single-file recovery, grouped replay, checksum
    failure, repeated crash loops, and read-only/generated targets.

Related recovery contracts:

- [`/docs/reliability/local_history_contract.md`](./local_history_contract.md)
  distinguishes local-history checkpoints from autosave journals.
- [`/docs/reliability/local_history_restore_preview_contract.md`](./local_history_restore_preview_contract.md)
  defines compare/restore preview cards once recovered bytes have become
  local-history checkpoints.
- [`/docs/support/recovery_ladder_packet.md`](../support/recovery_ladder_packet.md)
  defines recovery-rung packet shape for safe mode, quarantine, and
  support escalation.
- [`/docs/runtime/storage_classes_and_gc.md`](../runtime/storage_classes_and_gc.md)
  defines storage classes and the ordinary-cache-clear exclusion rule.

## Source anchors

- `.t2/docs/Aureline_PRD.md:1293` - recovery journals are autosave
  snapshots, dirty-buffer journals, and crash sentinels; guided replay
  is preferred over raw dump recovery; journal reset is separate from
  settings reset.
- `.t2/docs/Aureline_PRD.md:1300` - crash recovery degrades from exact
  session restore to dirty-buffer recovery to clean open with preserved
  evidence.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1128` -
  dirty-buffer journals are user-owned recovery state and are never
  ordinary cache garbage.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1829` -
  recovery journals are forward-readable where practical and safe mode
  exposes journal reset separately.
- `.t2/docs/Aureline_Technical_Architecture_Document.md:7709` -
  dirty-buffer journals attempt guided replay with integrity checks and
  offer restore, inspect, or discard choices.
- `.t2/docs/Aureline_UX_Design_System_Style_Guide.md:1202` -
  session restore, autosave, and crash-loop recovery must preserve
  unsaved local content whenever recovery journals exist.

## Scope

Frozen here:

- the minimum record shape for a dirty-buffer autosave journal entry;
- grouped replay records for multi-file or multi-object recovery;
- crash-sentinel counters and thresholds that prevent silent replay
  loops;
- guided replay choices and their allowed, blocked, or downgraded
  states;
- journal reset as an explicit destructive action with export and
  support linkage;
- fixture coverage for the recovery scenarios that must remain
  explainable without opening raw log files.

Out of scope:

- final on-disk segment/frame encoding;
- crash uploader transport;
- byte-level diff rendering;
- exact editor merge algorithms after a restore choice has been
  admitted.

## 1. Record Model

The machine-readable boundary has four autosave-side record kinds and
one guided-choice record kind:

| Record kind | Purpose |
|---|---|
| `autosave_journal_entry` | One captured dirty buffer, virtual buffer, generated/read-only target stub, or journal frame summary. |
| `autosave_journal_group_record` | One multi-entry replay group, such as a crash involving several dirty files or notebook cells. |
| `crash_sentinel_record` | The durable strike counter that prevents repeated hidden restore or extension-host restart loops. |
| `journal_reset_record` | The explicit destructive action that removes selected journals or tombstones them after review. |
| `guided_replay_choice_record` | One advertised replay choice for a journal entry or group, including blocked and downgraded states. |

Surfaces must render recovery from these records. They must not infer
hidden replay behavior from raw journal frames, log files, crash dumps,
or private extension state.

## 2. Autosave Journal Entry

An `autosave_journal_entry` is the smallest unit of dirty-buffer
recovery. Required fields:

| Field | Purpose |
|---|---|
| `journal_entry_id` | Stable id cited by replay choices, support bundles, local-history checkpoints, and reset records. |
| `journal_id` | Stable journal container id for the crashed workspace/session authority. |
| `workspace_ref` | Opaque workspace/session authority ref. Raw absolute paths do not appear here. |
| `object_identity` | Logical document id, object class, canonical identity refs, current relation, and sanitized display hint. |
| `base_on_disk_token` | Compare-before-write token observed when the journal frame was captured. |
| `text_format` | Encoding, BOM, newline mode, decoder posture, final-newline state, and large-file posture. |
| `actor_surface` | Actor class, source class, command/session refs, and originating surface. |
| `capture_descriptor` | Capture class, capture mode, body availability, object refs, grouped member refs, and omission reason. |
| `integrity` | Checksum algorithm/ref, frame integrity state, replay integrity posture, last-good frame, and evidence refs. |
| `replay_posture` | Object-class replay posture, recommended choice, blocked choices, downgrade reasons, and checkpoint linkage. |
| `retention_posture` | Retention class, ordinary-cache/settings/local-history reset exclusions, export-before-reset posture, and expiry/pin refs. |
| `support_export` | Support-bundle inclusion state, redaction class, and export refs. |

### 2.1 Object Identity

Every entry carries both logical and storage identity:

- `logical_document_id` follows the user's buffer across rename, move,
  alias, or provider identity drift.
- `object_ref` names the concrete object or virtual buffer without
  exposing a raw path.
- `object_class` is closed: canonical file, virtual buffer, untitled
  buffer, generated artifact, managed mirror, read-only file, notebook
  cell, structured artifact, or unsupported object.
- `filesystem_identity_ref` and `canonical_identity_ref` carry refs to
  the stronger filesystem/provider identity records when known.
- `identity_relation` states whether current identity is exact, alias
  drifted, same path but different object, missing, virtual-only, or
  unknown.

If identity is not exact, restore may still be possible, but the choice
must be downgraded to a reviewed restore or inspect-only posture before
any write.

### 2.2 Base-on-Disk Token

`base_on_disk_token` is the compare-before-write anchor. It records the
strongest token available at capture time:

- `file_id_generation`
- `inode_mtime_size_hash`
- `remote_etag_revision`
- `content_hash_only`
- `missing_or_not_applicable`
- `unknown`

Restore is blocked or review-gated when the current on-disk token cannot
be matched to this base token. A replay engine may open recovered content
for inspection, but it must not silently overwrite a changed object.

### 2.3 Encoding and Newline Mode

`text_format` captures the user's buffer format rather than re-detecting
it after a crash:

- encoding label and BOM posture;
- newline mode: `lf`, `crlf`, `cr`, `mixed`, `binary_or_not_text`, or
  `unknown`;
- decoder posture: exact, lossy with raw bytes preserved, metadata only,
  binary snapshot, or unknown;
- final-newline state and large-file mode.

Restore must preserve these fields or explicitly warn when it cannot.
For lossy decode or binary-adjacent data, restore choices normally
downgrade to inspect-only or compare/review before write.

### 2.4 Actor and Surface

`actor_surface` records who or what caused the captured dirty state:
user keystroke, user command, multi-cursor command, formatter, save
participant, AI/tool apply, extension surface, remote session, external
change detector, or crash-recovery journal. It also carries source class
and command/session refs.

These fields let a recovery card explain "this was unsaved typing in an
editor" versus "this was generated by a formatter during autosave"
without replaying logs.

### 2.5 Capture and Integrity

`capture_descriptor.capture_class` is closed:

- `dirty_text_delta`
- `full_buffer_snapshot`
- `cursor_view_state`
- `save_coordination_metadata`
- `grouped_manifest`
- `metadata_only_stub`
- `evidence_only_corrupt_frame`

`integrity.frame_integrity_state` is also closed:

- `verified`
- `verified_partial`
- `checksum_mismatch`
- `truncated_frame`
- `schema_incompatible`
- `missing_frame`
- `redacted_metadata_only`
- `unverifiable`

Integrity gates replay:

- `verified` frames can offer restore when identity and policy also
  allow it.
- `verified_partial` frames can offer restore up to
  `last_good_frame_ref` and must expose the omitted tail.
- `checksum_mismatch`, `truncated_frame`, `missing_frame`, and
  `unverifiable` frames block direct restore for affected bytes and
  downgrade to inspect-only, evidence-only, or safe mode.
- `schema_incompatible` frames block automatic replay unless an
  explicit forward-reader declares compatibility.
- `redacted_metadata_only` frames can support evidence export or
  support review, but not body restore.

## 3. Guided Replay Choices

Every recovery surface advertises choices from
`guided_replay_choice_record`. Hidden choices and private replay modes
are forbidden.

| Choice | Allowed when | Blocks or downgrades |
|---|---|---|
| `restore` | Integrity is verified or safely partial, identity/base token match or review has resolved drift, policy allows body restore, and object class is writable. | Downgrade to `inspect_only` or block when checksum fails, target is generated/read-only/managed, base token drift is unresolved, policy redacts body, or repeated replay failed. |
| `inspect_only` | Any recoverable body, last-good frame, or metadata summary can be shown without writing. | Block only when policy prohibits even metadata display or the frame is unavailable with no last-good summary. |
| `discard_journal` | User confirms a journal reset for selected entries and any export-before-reset rule is satisfied. | Block when legal hold, support case pin, user pin, managed policy, or current dirty buffer protection prevents deletion. Requires destructive confirmation. |
| `open_without_replay` | User wants a clean workspace launch while keeping journals available. | Becomes the default when replay is unsafe; may be blocked only by a policy that requires safe mode after repeated failure. Journals remain retained unless separately reset. |
| `escalate_to_safe_mode` | Crash sentinel reaches a threshold, user chooses safe mode, or policy/admin recovery requires it. | Never deletes journals. It disables risky capability classes and preserves recovery evidence. If a single extension is the likely offender, quarantine is preferred before global safe mode. |

Choices carry `availability_class`:

- `allowed`
- `requires_review`
- `downgraded`
- `disabled`
- `blocked_by_integrity`
- `blocked_by_policy`
- `blocked_by_incompatible_object_class`
- `blocked_by_repeated_failure`

Blocked choices remain visible with `block_or_downgrade_reasons[]` so a
user, support engineer, or test can reconstruct why replay did or did
not happen.

## 4. Blocking and Downgrade Rules

Replay engines apply gates in this order:

1. **Integrity gate** - checksum, frame continuity, schema
   compatibility, and last-good-frame state.
2. **Policy gate** - retention, body redaction, legal hold, managed
   policy, and export-before-delete requirements.
3. **Identity gate** - logical/canonical identity, base-on-disk token,
   external drift, missing object, and alias relation.
4. **Object-class gate** - generated artifacts, read-only targets,
   managed mirrors, unsupported structured objects, and binary-adjacent
   buffers.
5. **Repeated-failure gate** - crash-sentinel thresholds and suspect
   extension evidence.

The first blocking gate sets the primary availability class; later gates
remain listed as secondary reasons. A direct restore may become
inspect-only, a grouped restore may become selected-member restore, and
an automatic replay may become open-without-replay or safe mode. These
downgrades are visible in the choice record.

## 5. Crash Sentinels and Thresholds

Crash sentinels are durable strike records scoped to workspace authority,
build identity, restore graph, and fault domain. They exist to prevent
invisible retry loops.

Default thresholds:

| Trigger | Threshold | Required transition |
|---|---|---|
| Single abnormal termination with verified journal | 1 crash before first successful post-crash launch | Show guided replay; do not auto-enter safe mode. |
| Restore replay failure | 2 failed replay attempts for the same restore graph within 15 minutes | Stop automatic replay; default to `open_without_replay` and offer `escalate_to_safe_mode`. |
| Shell startup crash loop | 3 abnormal shell exits before first interactive idle within 30 minutes, or 2 consecutive crashes while applying the same restore graph | Next launch enters safe mode and disables automatic restore replay. |
| Extension-host crash during restore | 2 crashes from the same extension/capability world during startup restore, or 3 crashes within 10 minutes | Quarantine the suspect extension before retry; if no suspect is isolated, enter safe mode. |
| Journal integrity failure | First checksum or truncation failure for an entry | Block direct restore for affected bytes; offer inspect last-good frame and evidence export. |
| Repeated integrity failure | 2 integrity failures for the same journal container within 24 hours | Mark journal evidence-only, stop automatic replay, and recommend support export. |
| Safe-mode re-entry | 2 safe-mode entries for the same workspace/build within 24 hours without a successful normal launch | Offer support escalation and keep suspect-extension quarantine active. |

Counters reset only after a successful interactive idle window with no
restore replay, extension-host, or shell supervisor failure. The idle
window is 5 minutes by default. Manual journal reset can clear journal
strike linkage only for the reset journal; it does not clear shell crash
or extension quarantine evidence.

## 6. Safe Mode and Suspect-Extension Quarantine

Safe mode is a published runtime profile. It is not a private attempt to
"try again differently." A safe-mode transition records:

- entry trigger and threshold reached;
- disabled capability classes;
- retained data classes;
- journal/group refs kept available for inspection;
- support/export refs;
- return path to normal mode.

When an extension or extension host is the likely offender, quarantine is
preferred over global safe mode. Quarantine records the suspect extension
or narrowed cohort, disables auto-reactivation, preserves journals and
dirty buffers, and exposes a reviewed re-enable path.

Neither safe mode nor quarantine may delete autosave journals, settings,
profiles, local history, support evidence, or user files.

## 7. Journal Reset Separation

Journal reset is a distinct destructive action represented by
`journal_reset_record`. It is separate from all of the following:

- settings reset;
- profile reset;
- clear caches;
- clear local history;
- support bundle deletion;
- extension state reset;
- workspace restricted-mode entry.

Required reset behavior:

1. The reset review names the selected journals/entries, affected dirty
   buffers, recoverable body availability, pins/holds, support refs, and
   export-before-reset posture.
2. The confirmation text names "recovery journal" or "autosave journal";
   generic "clear cache" language is invalid for this action.
3. Reset may delete bodies only after any required export is complete or
   explicitly declined where policy allows decline.
4. Reset writes a tombstone with reset scope, actor, confirmation,
   export/support refs, skipped entries, and reason.
5. Reset does not remove local-history checkpoints created from prior
   restores. Local history has its own clear-history action.
6. Reset does not change settings, profiles, keybindings, snippets,
   extension installation state, or cache classes.
7. Reset does not clear crash-sentinel counters for shell or extension
   failures unless those counters point only to the reset journal and no
   separate crash evidence remains.

## 8. Retention and Export

Autosave journals are local-first, user-owned recovery state. Their
default retention class is `retained_until_user_decision` after an
abnormal termination and `active_replay_window` while the shell is still
recovering.

Retention posture must state:

- ordinary cache clear excluded;
- settings reset excluded;
- local-history clear excluded;
- whether journal reset is required for deletion;
- expiry policy and pin refs;
- export-before-reset state;
- support bundle inclusion state and redaction class.

Support bundles cite journal entry ids and integrity summaries by
default. Raw recovered body content requires explicit review and may be
blocked by policy.

## 9. Fixture Expectations

The fixture set under
[`/fixtures/recovery/autosave_replay_cases/`](../../fixtures/recovery/autosave_replay_cases/)
anchors the contract with reviewable examples:

- single-file crash with verified dirty-buffer restore;
- multi-file grouped replay with selected-member downgrade;
- journal checksum failure with last-good inspect-only posture;
- repeated crash loop that enters safe mode or quarantines a suspect
  extension;
- read-only and generated targets that block direct restore but keep
  inspect/export paths.

Each fixture embeds records from the two schemas. Fixture metadata is
review-only; production consumers read the schema records themselves.

## 10. Conformance Rules

1. Recovery UI, CLI, support, and tests read schema records instead of
   raw journal frames whenever explaining recovery behavior.
2. Direct restore requires integrity, policy, identity/base token, object
   class, and repeated-failure gates to pass.
3. Inspect-only remains available whenever a safe summary or last-good
   body exists.
4. Open-without-replay keeps journals retained until the user resets
   them or retention policy expires them into tombstones.
5. Automatic restore replay stops at the crash-sentinel thresholds in
   this document.
6. Journal reset is never exposed as generic cache clear, settings reset,
   or local-history clear.
7. Safe mode and quarantine preserve journals and dirty buffers unless a
   separate, previewed destructive reset is confirmed.
