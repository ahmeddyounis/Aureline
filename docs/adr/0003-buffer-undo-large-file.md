# ADR 0003 — Buffer, undo/redo, and large-file editing

- **Decision id:** D-0002 (see `artifacts/governance/decision_index.yaml#D-0002`)
- **Status:** Accepted
- **Decision date:** 2026-04-19
- **Freeze deadline:** 2026-07-01
- **Owner:** `@ahmeddyounis`
- **Backup owner:** `null` (covered by waiver `single-maintainer-backup` in `artifacts/governance/ownership_matrix.yaml#waivers`)
- **Forum:** architecture_council
- **Related requirement ids:** none

## Context

The buffer is the editor's text core. It owns the bytes the user is
editing, the undo journal that lets every mutation be reversed, the
snapshot lineage that diffs and background analysis quote, the save
pipeline that returns those bytes to disk, and the large-file mode
that keeps oversized or hostile files from poisoning the normal path.
Whichever shape this core takes becomes the floor under every later
editor command, every save participant, every refactor, every AI
apply flow, and every replay path the mutation journal will feed.

The freeze matters because later work cannot land honestly on top of
an unfrozen text core: refactor flows cannot promise "previewable and
reversible" without knowing what an undo group actually contains;
save participants cannot promise "rebase or abort" without knowing
the snapshot semantics they rebase against; the mutation journal
cannot replay anything without a stable undo-class taxonomy; and the
large-file controller cannot draw a line between "normal" and
"reduced-capability" without a concrete switch condition. An
unfrozen core also keeps source-fidelity claims aspirational — every
new flow can argue for its own encoding, BOM, or newline behaviour.

This ADR closes `D-0002` (buffer and editor-core persistence model)
ahead of its `2026-07-01` freeze so the buffer prototype, the
large-file controller, the save pipeline, and the eventual
mutation-journal work can start instrumenting against concrete hook
names and a single undo-class vocabulary rather than a moving target.

## Decision

Aureline freezes one buffer model, one snapshot and undo lineage,
one undo-class taxonomy, one large-file fallback path, one set of
source-fidelity rules, and the protected-hot-path hooks that govern
them. All are stated in terms of contracts, hook names, and class
ids rather than specific crates so dependency refresh is a hygiene
change, not a re-litigation.

### Buffer model

- **Primary representation.** A piece-tree buffer (a piece-table
  derivative with a balanced index over append and edit pieces).
  Source bytes are kept immutable; edits become append-piece
  references. Insert and delete are cheap and local; the original
  bytes are never copied for the lifetime of the buffer.
- **Coordinate system.** The buffer exposes a stable mapping between
  byte offsets, line starts, grapheme clusters, and protocol
  coordinates (UTF-16 code units for LSP-class consumers, UTF-8
  byte offsets for native consumers). Coordinate translation is a
  buffer responsibility, not a per-consumer chore.
- **Encoding boundary.** Internally the buffer is UTF-8. The decode
  step at open time and the encode step at save time are the only
  places encoding is converted; the editor command graph never sees
  non-UTF-8 bytes.
- **Selections and multicursor.** Selections are a buffer concept,
  not a view concept. Multicursor edits are one transaction over
  N selection ranges; they MUST collapse to one undo group (see
  below) so undo cannot half-revert a multicursor operation.
- **Dirty state.** Dirty / clean is per-buffer and is computed
  against the most recent saved snapshot, not against the
  on-disk file. An external change does not silently mark the
  buffer dirty; it raises an external-change event handled below.

### Snapshot and checkpoint semantics

- **Versioned snapshots.** Every committed mutation produces a
  monotonically increasing buffer version. Snapshots are cheap
  (the piece-tree is structurally shared) and are the unit that
  diff, background analysis, search, decoration, accessibility,
  and preview surfaces quote. A snapshot is a value, not a lock —
  holding one does not block edits.
- **Checkpoint.** A checkpoint is a snapshot plus a durable record
  in the recovery journal. Checkpoints are taken automatically on:
  save, on entering / leaving large-file mode, before a destructive
  configuration or workspace migration, and at the start of any
  multi-file mutation that would otherwise lose intermediate state
  on crash. Lanes MAY request additional checkpoints; they MAY NOT
  remove the automatic ones.
- **Recovery journal.** A separate, append-only on-disk record
  pairs each checkpoint with the bytes the user has not yet saved.
  Crash recovery replays the journal forward from the last
  checkpoint; recovery never silently discards user bytes.
- **Snapshot retention.** Snapshots that no live consumer holds and
  that are not the head of an undo / redo chain or a checkpoint
  MAY be reclaimed. Reclamation is best-effort and never blocks
  the input path.

### Undo / redo and transaction grouping

- **Transaction.** A transaction is the atomic unit of mutation. A
  transaction has: a class (see taxonomy below), an originator
  (user, command, refactor, AI apply, save participant, import), an
  optional human-readable label, the snapshot it ran against, and
  the snapshot it produced. Transactions cannot be partially
  applied; failure rolls back to the pre-transaction snapshot.
- **Undo group.** An undo group is one or more transactions that
  undo / redo treat as a single user step. Coalescing rules:
  - Single-character typing into the same cursor coalesces into one
    group until a word boundary, a caret jump, a selection change,
    or a configurable idle window passes.
  - Multicursor edits in one keystroke are one group across all
    cursors.
  - A command that opens a named undo group (refactor, AI apply,
    multi-file rename, formatter, import) collects every
    transaction it emits into that single named group; the group
    closes when the command resolves (apply, abort, or revert).
  - A save and its save participants run inside one group so an
    undo after save reverses both the participant edits and the
    save back to the pre-participant state where possible.
- **Single undo journal per buffer.** The journal is per-buffer and
  is the source of truth for that buffer's history. Cross-buffer
  multi-file groups (see taxonomy) live in a workspace-level
  history that references each buffer's journal.
- **Compensation vs revert.** A class is *compensatable* when its
  effect can be reversed by a forward operation that is itself a
  legal transaction (a delete reversed by an insert of the same
  bytes at the same offset). A class is *only revertible* when the
  reverse depends on a stored snapshot (a refactor across N files
  whose forward operation cannot be expressed as a small inverse).
  Compensatable classes MAY be redone after a divergent edit;
  only-revertible classes pin redo to an exact snapshot lineage,
  and a divergent edit drops the redo stack for that group.

### Undo-class taxonomy

The taxonomy is the vocabulary the undo journal, the workspace-level
history, the recovery journal, and the eventual mutation-journal
contract all share. Class ids and the compensatable / revertible
posture are frozen here; per-class field definitions live in
`artifacts/architecture/undo_class_rows.yaml`. A new class requires a
new row in that file and an ADR amendment.

| Class id                       | Scope                                                                 | Originator examples                                | Compensation posture |
|--------------------------------|-----------------------------------------------------------------------|----------------------------------------------------|----------------------|
| `text_edit`                    | Local insert / delete / replace inside one buffer                     | Keystroke, paste, single-cursor command            | compensatable        |
| `multi_cursor_text_edit`       | One transaction over N selection ranges in one buffer                 | Multicursor type, multicursor delete               | compensatable        |
| `structural_edit`              | Whole-line, block, indent, sort, reformat range inside one buffer     | Move-line, reindent, sort-lines, range format      | compensatable        |
| `refactor_single_file`         | Single-file refactor with a named group                               | Local rename, extract method, inline variable      | compensatable        |
| `refactor_multi_file`          | Refactor that mutates two or more buffers / files                     | Cross-file rename, organise imports across project | only revertible      |
| `formatter_run`                | Formatter or pretty-printer run inside save or on demand              | Format-on-save, format-document                    | compensatable        |
| `save_participant_group`       | Save plus the participants that ran in the same save pipeline         | Format-on-save + organise imports + AI apply       | only revertible      |
| `imported_change`              | Change ingested from outside the editor without a typed transaction   | External rewrite reconciled, settings import       | only revertible      |
| `machine_generated_change`     | Edit produced by a non-human originator and applied with attribution  | AI apply, code-action quick-fix, generator output  | only revertible      |
| `migration_change`             | Schema, settings, or workspace-state migration with a checkpoint      | Settings file migration, workspace-state upgrade   | only revertible      |
| `external_reload`              | Buffer adopted on-disk bytes after an external modification           | VFS-detected change, clean-buffer auto-reload      | only revertible      |
| `decode_recovery_change`       | Buffer adopted a recovered decoded form after a decode failure        | Encoding override, BOM correction, byte-recovery   | only revertible      |

Rules across the taxonomy:

- Every transaction MUST declare a class id from this table; an
  untyped mutation is a contract violation.
- A class's compensation posture is invariant; lanes MAY NOT label
  a multi-file refactor as compensatable to enable shorter redo
  chains.
- An undo group MAY contain transactions of different classes; the
  group's compensation posture is the most restrictive of its
  members (any only-revertible member makes the whole group
  only-revertible).
- The mutation-journal lane (when it lands) consumes this taxonomy
  unchanged; introducing a new class is a journal-schema change,
  not an unannounced taxonomy extension.

### Source-fidelity rules

- **Encoding detection at open.** Detection runs once at open: BOM
  → declared encoding metadata → heuristic (UTF-8, then platform
  default with confidence threshold) → user override. The detected
  encoding is sticky for the lifetime of the buffer and is part
  of the saved snapshot.
- **BOM preservation.** A file opened with a BOM is saved with the
  same BOM. A file opened without a BOM is saved without one.
  Adding or removing a BOM requires an explicit user or workspace
  policy action; no save participant MAY change BOM state silently.
- **Newline preservation.** The dominant newline mode (`\n`, `\r\n`,
  `\r`) detected at open is preserved on save. Mixed-newline files
  preserve the dominant mode and surface the mix in the status
  surface; conversion requires an explicit command.
- **Final-newline state.** Whether the file ends with a final
  newline is preserved. Adding or removing a final newline is an
  explicit command, not a side effect of formatters or save
  participants.
- **Decode failure and recovery handoff.** If decode fails, the
  buffer opens in **decode-recovery state**: the original on-disk
  bytes are preserved verbatim, the buffer surfaces a recoverable
  banner (override encoding, treat as binary, open in large-file
  mode), and editing is gated until recovery resolves. The raw
  bytes are never destroyed by the failed decode. A successful
  recovery emits a `decode_recovery_change` transaction so the
  journal records the resolution.
- **Generated and read-only posture.** Files marked generated (by
  detection, attribute, or workspace policy) open in a
  *advisory-read-only* state: editing is permitted but a banner
  warns that the file is generated; save participants that would
  rewrite the file (formatters, import organisers) are off by
  default for generated files. OS-level read-only files open in
  *enforced-read-only* state: editing is denied; save is denied;
  the buffer still produces snapshots so diff and search work.
- **Save target constraints.** The save pipeline writes to the
  *canonical* save target (the resolved on-disk path the file was
  opened from), not to whichever alias the user typed. Symlink
  targets, case-insensitive name variants, Unicode-normalisation
  variants, and overlay aliases are surfaced when they would
  affect the save destination; saving to an unexpected alias
  requires explicit confirmation. Executable bits, OS read-only
  flags, and symlink intent are preserved across save where the
  platform supports them.
- **Atomic save pipeline.** The default save sequence is: encode
  → write to a same-directory temp file → `fsync` the temp file →
  atomic rename over the canonical target → `fsync` the parent
  directory where required by the host filesystem. On filesystems
  where rename safety is ambiguous, the buffer falls back to a
  journal-mediated save and surfaces the fallback mode in the
  status surface. The save pipeline is one undo group.
- **External modification handling.** When the VFS reports the
  on-disk bytes changed out of band, the buffer raises an
  external-change event. Clean buffers MAY auto-reload by policy;
  dirty buffers route to an explicit diff / merge / choose flow
  and never silently overwrite either side. Auto-reload commits an
  `external_reload` transaction so the journal is honest about
  the source of the new bytes.
- **Round-trip safety.** Save participants operate on a *staged*
  copy of the buffer; if the on-disk file changes mid-flight, the
  participant rebases or aborts and the user is notified. Bytes
  outside the participant's edited ranges are preserved verbatim
  on round trip.

### Large-file mode

- **What it is.** A reduced-capability buffer mode for files whose
  size, classification, or system-resource profile would violate
  the input budget or the memory budget of the normal path.
- **Switch conditions (entering large-file mode).** A file enters
  large-file mode at open time when **any** of the following hold:
  1. **Size threshold.** The file's on-disk size exceeds the
     workspace policy's `large_file_size_threshold` (default
     `100 MiB`; configurable down to `1 MiB` and up to host RAM).
  2. **Resource pressure.** Loading the file at the size threshold
     would push estimated buffer RSS above the resource manager's
     soft budget for the editor service class.
  3. **Classification.** The file is detected as binary,
     minified beyond a complexity threshold, or matches a
     workspace-policy "large-file pack" rule.
  4. **Decode posture.** The file failed normal decoding and the
     user chose "open in large-file mode" from the
     decode-recovery surface.
  5. **Operator override.** The user explicitly opened the file
     in large-file mode.
  These are evaluated in order; the first match wins. The chosen
  trigger is recorded with the buffer so support bundles can
  explain why the mode applied.
- **What large-file mode keeps.** Read, navigate, search-within
  (subject to a streaming search variant), select, copy, and save
  with the source-fidelity rules above. The accessibility tree
  remains populated for the visible viewport. The file MAY be
  opened read-only by policy.
- **What large-file mode reduces or denies.**
  - Multi-cursor edits across the whole file are denied; viewport-
    bounded multi-cursor remains.
  - Full-file reflow, full-file syntax parsing, full-file diagnostics,
    and full-file decoration are denied; viewport-bounded variants
    apply.
  - Save participants that rewrite the whole file (full-file
    formatters, import organisers, AI apply over the whole file)
    are denied. Save participants that operate on the edited range
    only are permitted.
  - Indexing and background analysis are denied; on-demand,
    cursor-local lookups are permitted.
  - Whole-file load into RAM is denied; the buffer is paged through
    an mmap-backed reader.
  - Undo / redo coalescing windows shorten and per-edit storage
    bounds tighten so the journal cannot grow unbounded; the
    journal MAY reject a transaction whose stored inverse exceeds
    a per-buffer cap, surfacing the rejection rather than silently
    truncating history.
- **Switch conditions (leaving large-file mode).** A buffer leaves
  large-file mode only when the user explicitly downgrades it
  (after, for example, splitting the file or trimming it) and the
  switch conditions above no longer apply. Mode transitions emit a
  checkpoint.
- **Backing store.** The large-file backing store is an
  mmap-backed paged reader with a paged-rope-class write overlay.
  The reader exposes the same coordinate system as the normal
  buffer; consumers are not expected to special-case large-file
  buffers beyond the reduced-capability list above.

### Performance and memory assumptions (must-measure list)

The following are the assumptions prototypes MUST measure. They are
not numeric budgets here — those live in the benchmark lab. They are
the list of measurements that gates "prototype is reviewable against
this ADR".

- **Open latency** for a representative source file under the size
  threshold: cold and warm.
- **Open latency** at and above the size threshold: cold and warm,
  reporting which switch condition triggered large-file mode.
- **Apply latency** for a single-character text edit (`text_edit`)
  in a normal-path buffer of representative size.
- **Apply latency** for a multicursor edit (`multi_cursor_text_edit`)
  across N cursors at representative N values.
- **Apply latency** for a multi-file refactor (`refactor_multi_file`)
  spanning representative file counts.
- **Undo / redo latency** for each class id, including the journal
  rebuild path after a divergent edit drops a redo stack.
- **Save pipeline duration** for the atomic save and for the
  journal-mediated fallback, including a save-participant group.
- **Snapshot creation cost** under a realistic edit cadence.
- **Memory footprint** of the buffer plus the journal at
  representative file sizes, plus the high-water mark across
  large-file mode entry, edit, and exit.
- **Recovery duration** from the most recent checkpoint plus the
  journal forward to the last user keystroke.
- **External-change reconciliation latency** for clean and dirty
  buffers, including the diff / merge handoff cost.

A prototype that does not report against this list is not
reviewable against this ADR.

### Protected-hot-path hooks

The buffer exposes the following named hooks. They are the canonical
instrumentation surface for the buffer prototype, the save pipeline,
the large-file controller, and the eventual mutation-journal work;
no lane MAY invent alternative names for the same measurement.

| Hook id                          | Fires when                                                                                                            | Protected hot-path budget |
|----------------------------------|-----------------------------------------------------------------------------------------------------------------------|---------------------------|
| `buffer_open`                    | A buffer transitions from "loading" to "ready" in either normal or large-file mode                                    | yes                       |
| `text_edit_apply`                | A `text_edit` or `multi_cursor_text_edit` transaction commits                                                          | yes                       |
| `transaction_apply`              | Any transaction (any class id) commits to the journal                                                                 | yes                       |
| `snapshot_create`                | A versioned snapshot is materialised for a consumer                                                                   | yes                       |
| `checkpoint_create`              | A checkpoint is written to the recovery journal                                                                       | yes                       |
| `undo_group_open`                | A named undo group opens                                                                                              | no (observability only)   |
| `undo_group_close`               | A named undo group closes (apply, abort, or revert)                                                                   | no (observability only)   |
| `undo_apply`                     | Undo replays one undo group                                                                                           | yes                       |
| `redo_apply`                     | Redo replays one undo group                                                                                           | yes                       |
| `save_pipeline_run`              | The save pipeline runs (atomic or journal-mediated)                                                                   | yes                       |
| `save_participant_rebase`        | A save participant rebases or aborts because the on-disk file changed mid-flight                                      | yes                       |
| `external_change_detected`       | The VFS reports an out-of-band on-disk change for an open buffer                                                      | yes                       |
| `external_change_merge`          | The buffer adopts external bytes (auto-reload or merged choice)                                                       | yes                       |
| `decode_recovery_open`           | A buffer opens in decode-recovery state                                                                               | no (observability only)   |
| `decode_recovery_resolve`        | A decode-recovery state resolves (override encoding, treat as binary, large-file mode)                                | yes                       |
| `large_file_mode_enter`          | A buffer enters large-file mode (any switch condition)                                                                | yes                       |
| `large_file_mode_exit`           | A buffer leaves large-file mode                                                                                       | yes                       |
| `journal_recovery_replay`        | Crash recovery replays the journal forward from the last checkpoint                                                   | yes                       |
| `journal_inverse_rejected`       | The journal rejects a transaction whose stored inverse exceeds the per-buffer cap                                     | no (observability only)   |

The benchmark lab reports every hot-path hook against its protected
budget on the claimed corpora; non-hot-path hooks are
observability-only and do not gate release.

### Non-goals at this decision

Out of scope until a superseding decision row opens:

- Collaborative / multi-author editing semantics. CRDTs remain a
  collaboration-and-session protocol layered above this buffer
  contract; this ADR does not freeze their merge model.
- Block / structured-document buffers (notebook outputs, rich
  decorations as authoritative bytes). Structured documents that
  must round-trip MUST sit on top of this buffer; their
  unknown-key preservation rules live with their own ADRs.
- Distributed undo across machines or sessions.
- Block-level deduplication of buffer contents across files.
- Encrypted-at-rest buffer storage beyond what the host
  filesystem provides.
- A user-facing history UI; this ADR freezes the history's
  semantics, not the surface that visualises it.

These lines move only by opening a new decision row, not by editing
this ADR.

### Tradeoff table

The structured tradeoff rows live in
`artifacts/architecture/undo_class_rows.yaml` (per-class fields,
compensation posture, journal schema reservations). The headline
buffer tradeoff summary:

| Axis                                         | Chosen stack                                                                       | Best rejected alternative                                                | Why chosen wins                                                                                           |
|----------------------------------------------|------------------------------------------------------------------------------------|--------------------------------------------------------------------------|-----------------------------------------------------------------------------------------------------------|
| **Edit-path performance**                    | Piece tree with cheap append-piece inserts and structural snapshot sharing         | Gap buffer                                                               | Gap buffer collapses on scattered edits and large files; piece tree is the AD-003 floor                   |
| **Large-file behaviour**                     | Mmap-backed paged reader with paged-rope write overlay and a frozen reduced list   | Same buffer for all file sizes                                            | Whole-file load violates memory and input budgets; AD-004 already commits to a separate fallback path     |
| **Undo correctness across classes**          | Frozen taxonomy with explicit compensatable / only-revertible posture per class    | One opaque undo stack across every originator                            | Opaque undo hides cross-file and machine-generated changes the source docs require to be attributable     |
| **Source fidelity**                          | Sticky encoding / BOM / newline detection plus explicit-only conversion            | Inferred conversions on save                                              | Save-time inference loses round-trip safety the source docs make a release-bearing obligation             |
| **Save safety**                              | Temp + fsync + atomic rename, journal-mediated fallback where rename is unsafe     | Direct overwrite                                                         | Direct overwrite loses crash safety; the source docs require atomic save where the filesystem allows it   |
| **Recovery semantics**                       | Per-buffer journal + checkpoints + workspace-level multi-file group references     | In-memory undo only                                                       | In-memory undo loses everything on crash; the recovery journal is what makes the journey budget honest    |
| **Mutation-journal reuse**                   | Same class taxonomy and group semantics consumed by the journal                    | Journal invents its own class set                                         | Two taxonomies drift; one taxonomy is the only way the journal can replay history honestly                |

Each row carries reopen triggers in the YAML (for example: a
benchmark finding that `text_edit_apply` exceeds its budget on a
representative file size reopens the buffer-model row).

### Decision-example fixtures

A small corpus of decision-example fixtures lives under
`fixtures/text/buffer_decision_cases/`. They are short, reviewable
scenarios (typing, multicursor, multi-file refactor, format-on-save,
AI apply, external modification, decode recovery, BOM / newline
preservation, large-file open, large-file constrained edit) used by
the buffer prototype, the save pipeline, the large-file controller,
and the recovery work to anchor the hook names and the undo classes
above to concrete inputs and observable outcomes. They are not a
test suite; they are the language the ADR's hook list and undo
taxonomy refer to.

## Consequences

- **Frozen:** the buffer model (piece tree with paged-rope
  large-file overlay), the snapshot and checkpoint semantics, the
  undo / redo grouping rules, the undo-class taxonomy and per-class
  compensation posture, the source-fidelity rules, the large-file
  switch conditions and reduced-capability list, and the protected-
  hot-path hook names.
- **Frozen:** the save pipeline's ordering (encode → temp-write →
  fsync → atomic rename, with a journal-mediated fallback) and the
  rule that one save plus its participants is one undo group.
- **Frozen:** decode failure preserves the original on-disk bytes
  verbatim. No flow MAY discard raw bytes to "fix" a decode error.
- **Permitted:** crates that own buffer storage, the journal, the
  large-file backing store, the recovery journal, and the save
  pipeline MAY refresh implementations as long as the contracts,
  hook names, and undo classes above are unchanged.
- **Permitted:** the workspace MAY tighten or loosen the
  `large_file_size_threshold` within the bounds named above; it
  MAY NOT remove the other switch conditions or widen the reduced-
  capability list silently.
- **Follow-up:** the buffer prototype instruments every hot-path
  hook before claiming latency budgets. The benchmark lab
  stabilises traces against representative file sizes, multicursor
  N values, and multi-file refactor counts. The mutation-journal
  lane consumes the undo-class taxonomy unchanged. The
  workspace-level multi-file history is a follow-on contract that
  references each buffer's journal; its schema is reserved in
  `artifacts/architecture/undo_class_rows.yaml` and lands as its
  own decision row when needed.
- **Follow-up:** a save-participant ADR MAY narrow specific
  participants above this floor (which participants run, in what
  order, with what telemetry) but cannot widen the rejected-
  alternatives list.
- **Follow-up:** a recovery-journal-format ADR MAY freeze the
  on-disk encoding of the recovery journal; until it lands, the
  journal is a private implementation detail of the buffer crate.
- **Ratifies:** the undo-class taxonomy becomes the vocabulary the
  mutation-journal lane, the AI apply flow, the refactor lanes,
  the multi-file rename lane, and the settings-import lane all
  use when they describe what they emitted.

## Alternatives considered

- **Gap buffer everywhere (the cheapest single-data-structure
  option).** Use a gap buffer as the only buffer model. Rejected:
  performs well only near the cursor on small files; collapses
  under scattered edits and large files. The source documents
  already classify this as "fine for small editors; not enough
  for a flagship IDE", and adopting it would force a parallel
  large-file path with a different coordinate system.
- **Pure rope without piece-table append history.** Use a balanced
  rope as the steady-state representation. Rejected: ropes are
  excellent for very large text and persistent snapshots, but
  the piece-table append model is what makes save participants,
  formatters, and AI apply preserve original bytes outside the
  edit ranges. Pure rope would require a parallel "original-bytes
  side table" that the piece tree gives us natively.
- **Same buffer for all file sizes (no large-file mode).** Load
  every file into the normal buffer. Rejected: violates the
  memory budget and the input-latency budget on representative
  large files, and would force the editor to either lock up at
  open time or load files lazily into the same data structure
  with a hidden divergence in semantics — exactly the drift the
  large-file mode exists to prevent.
- **One opaque undo stack across every originator.** Treat all
  mutations as one anonymous sequence. Rejected: hides what
  multi-file refactors, AI apply flows, save participants, and
  imported changes did; breaks the source documents'
  "previewable, attributable, and undoable" obligation; and
  leaves the mutation-journal lane with no taxonomy to consume.
- **Per-flow ad-hoc history (each command invents its own undo
  semantics).** Let refactor, AI apply, multi-file rename, and
  save participants each maintain their own history beside the
  buffer. Rejected: drift across flows, no single undo button,
  no single recovery journal, and no shared vocabulary for the
  mutation journal. The named-undo-group rule is the floor that
  prevents this.
- **Inferred encoding / BOM / newline conversions on save.**
  Convert files opportunistically when saving (for example,
  rewrite CRLF to LF in a "Linux project"). Rejected: collapses
  round-trip safety, loses BOM state, and contradicts the source
  documents' source-fidelity obligations. Conversions are
  explicit-only.
- **Direct overwrite save (no temp file, no fsync, no atomic
  rename).** Save by writing directly to the canonical path.
  Rejected: loses crash safety on every host where rename is
  the safe primitive; the source documents commit to atomic save
  where the filesystem allows it.
- **In-memory undo only (no recovery journal).** Keep undo state
  purely in process memory. Rejected: a crash discards every
  unsaved transaction and every undo / redo step; the
  large-file mode in particular cannot promise recovery without
  a journal.
- **Defer to a later milestone.** Leave `D-0002` open and let the
  narrowing default apply on `2026-07-01`. Rejected: the
  narrowing default (commit to the piece tree alone and defer the
  large-file path) is strictly narrower than the frozen posture
  here and would leave the save pipeline, the large-file
  controller, the recovery work, and the mutation-journal lane
  without a contract to instrument against during the most
  expensive months of pre-implementation work.

The `D-0002` default-if-unresolved narrowing would have committed
the project to a piece-tree buffer for standard editable files and
deferred the mmap + paged rope path behind an explicit fitness
function. Accepting this ADR replaces that narrowing with the
frozen buffer model, snapshot and checkpoint semantics, undo /
redo grouping rules, undo-class taxonomy, source-fidelity rules,
large-file mode, and hook list above; the narrowed posture does
not apply.

## Reopen triggers

Each of the following MUST open a new decision row (not an edit of
this ADR):

- a representative corpus exceeds the protected budget on
  `text_edit_apply`, `transaction_apply`, `undo_apply`,
  `save_pipeline_run`, or `journal_recovery_replay` and the fix
  would require swapping the buffer model or the backing store;
- a save-participant or AI apply flow needs an undo class that is
  neither compensatable nor only-revertible (i.e. would force a
  third compensation posture);
- a host filesystem becomes common enough that the atomic-rename
  pipeline must be replaced rather than fall back to journal-
  mediated save;
- collaborative / multi-author editing enters product scope and
  forces the buffer to expose CRDT-class semantics directly
  rather than as a layered protocol;
- the workspace-level multi-file history needs to mutate buffer
  history rather than reference it;
- a regulatory or compliance requirement forces the recovery
  journal to be encrypted-at-rest or signed.

## Platform-specific risk notes

- **macOS.** APFS rename is well-behaved; the risk lane is
  case-insensitive defaults and Unicode normalisation differences
  on save targets. The save-target alias surface is the trace
  anchor.
- **Windows.** NTFS rename across handles, antivirus scanners
  holding file handles open mid-save, and OneDrive / Files-on-
  Demand placeholder files are the dominant risks. The save
  pipeline's journal-mediated fallback covers the rename-unsafe
  path; `save_participant_rebase` and the save-target surface are
  the trace anchors.
- **Linux.** ext4, xfs, and btrfs all behave well for the atomic
  rename; the risk lane is fuse-mounted filesystems and
  network-mounted homes where rename atomicity is filesystem-
  specific. Fontless edit (no display server) MUST still produce
  a usable buffer; the buffer crate does not require a renderer.
- **All platforms.** Network filesystems frequently degrade
  rename safety; the journal-mediated save path and the save-
  target surface are the first-class path for those environments,
  not a silent regression.

## Benchmark-measurement expectations

- Every protected-hot-path hook reports latency to the benchmark
  lab on representative corpora (small, medium, large, and
  large-file-mode files; multicursor at representative N values;
  multi-file refactors at representative file counts) and on
  representative host filesystems.
- The benchmark lab's reproducibility pack for buffer claims
  names the file corpus, the originator (user, refactor, AI
  apply, save participant), the undo class id, the snapshot
  cadence, and the large-file mode trigger at measurement time.
- A benchmark result that crosses a protected budget on a
  claimed corpus is a `red` lane state; repeated `yellow` on the
  same hook forces a scope correction per the milestone-
  scorecard rules.

## Source anchors

- `.t2/docs/Aureline_PRD.md:144` — gap-buffer / piece-tree / rope /
  mmap comparison and the recommendation row.
- `.t2/docs/Aureline_PRD.md:148` — "mmap + paged reader … necessary
  fallback for multi-GB files".
- `.t2/docs/Aureline_PRD.md:150` — "piece-table-derived piece tree
  for standard editable files, with large-file mode backed by mmap
  + paged rope-like chunks".
- `.t2/docs/Aureline_PRD.md:707` — "every automated edit path —
  refactorings, quick fixes, AI changes, formatter actions — must
  be previewable, attributable, and undoable".
- `.t2/docs/Aureline_PRD.md:854` — "large-file mode: mmap-backed
  paged reader with read-only and constrained-edit modes".
- `.t2/docs/Aureline_PRD.md:856` — "undo/redo: operation log with
  coalescing and transaction grouping".
- `.t2/docs/Aureline_PRD.md:1373` — Section 5.29 "Text model,
  Unicode correctness, and source fidelity".
- `.t2/docs/Aureline_PRD.md:1382` — "save operations preserve
  detected encoding, BOM state, and dominant line-ending mode by
  default".
- `.t2/docs/Aureline_PRD.md:1383` — "atomic save pipeline … temp-
  write + fsync + atomic rename when the filesystem semantics make
  it safe".
- `.t2/docs/Aureline_PRD.md:1385` — "external modification
  handling … dirty buffers must use explicit diff/merge/choose
  flows".
- `.t2/docs/Aureline_PRD.md:1387` — "binary, generated, minified,
  or multi-GB files open in limited/read-only modes by default".
- `.t2/docs/Aureline_PRD.md:1390` — "decoding failures must never
  destroy the original on-disk byte sequence".
- `.t2/docs/Aureline_PRD.md:1391` — "save participants … operate
  on a staged buffer and rebase or abort if the on-disk file
  changed mid-flight".
- `.t2/docs/Aureline_PRD.md:1392` — "line endings, final-newline
  presence, and BOM state should be visible in the status surface".
- `.t2/docs/Aureline_PRD.md:1394` — structured documents preserve
  unknown keys and extension namespaces on round trip.
- `.t2/docs/Aureline_PRD.md:2680` — "any destructive config or
  workspace migration must create a checkpoint that can be
  restored automatically if startup fails".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:402` —
  AD-003 "primary text model … piece-table / piece-tree".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:403` —
  AD-004 "large-file path … mmap-backed paged reader with limited
  edit mode".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1396` —
  "Aureline uses a piece-table / piece-tree text model for normal
  editable source files".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1409` —
  "transaction-grouped undo/redo".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1410` —
  "versioned snapshots for diff, background analysis, and
  preview".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1411` —
  "efficient mapping between byte offsets, line starts, grapheme
  clusters, and protocol coordinates".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1421` —
  "save operations preserve encoding, BOM state, final newline,
  and dominant line endings by default".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1424` —
  "decoding failures never discard original bytes".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1432` —
  "perform temp-write + fsync + atomic rename where safe".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1444` —
  Section 11.4 "Large-file mode".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1446` —
  large-file mode trigger by "size, content classification, and
  system constraints".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1481` —
  "buffer store | piece tree, snapshots, undo journal".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1485` —
  "large-file controller | classification and degraded
  capabilities".
- `.t2/docs/Aureline_Technical_Architecture_Document.md:1681` —
  "buffer / editor authority … text buffers, selections, dirty
  state, undo groups, large-file mode, view state handles".
- `.t2/docs/Aureline_Technical_Design_Document.md:1239` — Section
  7.1.8 "Large-file mode".
- `.t2/docs/Aureline_Technical_Design_Document.md:1355` — "broad-
  scope apply paths create named undo groups and, when they cross
  files or protected classes, durable review artifacts and
  checkpoints".
- `.t2/docs/Aureline_Technical_Design_Document.md:5359` — "user-
  visible mutations register named undo groups where supported;
  AI apply, extension refactor, multi-file replace, and settings
  import create grouped history entries with source attribution".
- `.t2/docs/Aureline_Technical_Design_Document.md:5545` — "broad
  semantic writes use preview/apply/revert language, named undo
  groups, and checkpoint affordances consistent with AI, refactor,
  and import flows".

## Linked artifacts

- Decision register row: `artifacts/governance/decision_index.yaml#D-0002`
- RFC: none.
- Undo-class taxonomy (machine form):
  `artifacts/architecture/undo_class_rows.yaml`.
- Decision-example fixtures:
  `fixtures/text/buffer_decision_cases/`.
- Affected lanes: `crates/aureline-buffer`, `crates/aureline-text`,
  `crates/aureline-vfs` (external-change events and save target
  resolution), `crates/aureline-bench`,
  `artifacts/governance/ownership_matrix.yaml#scorecard_lane_index:benchmark_lab`,
  `artifacts/governance/ownership_matrix.yaml#scorecard_lane_index:release_evidence`.

## Supersession history

First acceptance. No supersession.
