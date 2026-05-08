//! Piece-tree buffer prototype with grouped undo/redo.
//!
//! This is a prototype, not the production buffer engine. It validates
//! the contract frozen in `docs/adr/0003-buffer-undo-large-file.md`:
//!
//! - a piece-tree representation (append-only source bytes, append-
//!   only edit buffer, ordered piece list) so the original bytes are
//!   never copied and edits are cheap and local;
//! - snapshots that can be read without mutating active buffer state;
//! - grouped undo/redo transactions carrying a frozen undo-class id,
//!   an originator, an optional label, the parent snapshot, and the
//!   produced snapshot;
//! - a distinction between compensatable groups (whose inverse is a
//!   forward transaction) and only-revertible groups (whose inverse
//!   depends on the pre-transaction snapshot), so later editor layers
//!   can reason about which redo survives a divergent edit;
//! - named hook counters that match the protected-hot-path vocabulary
//!   in the ADR.
//!
//! The piece list is a `Vec<Piece>`; the production buffer replaces
//! this with a balanced index without changing the public API.

use std::ops::Range;
use std::sync::Arc;

use super::class::{CompensationPosture, UndoClass};
use super::hooks::HookCounters;

/// Snapshot-identifier newtype. Monotonic per-buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SnapshotId(pub u64);

/// Transaction-identifier newtype. Monotonic per-buffer, assigned at
/// commit time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TransactionId(pub u64);

/// Undo-group newtype. Monotonic per-buffer, assigned at `begin`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UndoGroupId(pub u64);

/// Checkpoint-handle newtype. Monotonic per-buffer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CheckpointHandle(pub u64);

/// Errors the buffer surfaces back to callers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BufferError {
    /// The offset or range fell outside the buffer.
    OutOfBounds { offset: usize, len: usize },
    /// Range end precedes start.
    InvertedRange { start: usize, end: usize },
    /// A mutation was attempted outside an open transaction when the
    /// caller asked the buffer to refuse the auto-wrap.
    NoOpenTransaction,
    /// A second transaction was requested while one was already open.
    /// The prototype does not support nested transactions.
    TransactionAlreadyOpen,
    /// A named-group class was begun without the label the ADR
    /// requires.
    MissingLabelForNamedGroup { class_id: &'static str },
    /// The journal refused to store this transaction's inverse
    /// because it exceeded the per-buffer cap. Fires the
    /// `journal_inverse_rejected` hook.
    InverseTooLarge { bytes: usize, cap: usize },
}

impl std::fmt::Display for BufferError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfBounds { offset, len } => {
                write!(f, "offset {offset} out of bounds (buffer len {len})")
            }
            Self::InvertedRange { start, end } => {
                write!(f, "inverted range: start {start} > end {end}")
            }
            Self::NoOpenTransaction => f.write_str("no open transaction"),
            Self::TransactionAlreadyOpen => f.write_str("a transaction is already open"),
            Self::MissingLabelForNamedGroup { class_id } => {
                write!(f, "class {class_id:?} requires a human-readable label")
            }
            Self::InverseTooLarge { bytes, cap } => {
                write!(
                    f,
                    "stored inverse is {bytes} bytes, exceeding the per-buffer cap of {cap} bytes"
                )
            }
        }
    }
}

impl std::error::Error for BufferError {}

/// A read-only view of the buffer at one version. Snapshots are
/// values, not locks — holding one does not block edits.
#[derive(Debug, Clone)]
pub struct Snapshot {
    id: SnapshotId,
    version: u64,
    content: Arc<Vec<u8>>,
}

impl Snapshot {
    pub fn id(&self) -> SnapshotId {
        self.id
    }
    pub fn version(&self) -> u64 {
        self.version
    }
    pub fn len(&self) -> usize {
        self.content.len()
    }
    pub fn is_empty(&self) -> bool {
        self.content.is_empty()
    }
    pub fn as_bytes(&self) -> &[u8] {
        &self.content
    }
    /// Lossless UTF-8 read. Returns `None` if the snapshot is not
    /// valid UTF-8 (the production buffer gates this at the encoding
    /// boundary; the prototype surfaces the failure to the caller).
    pub fn as_str(&self) -> Option<&str> {
        std::str::from_utf8(&self.content).ok()
    }
}

/// Configuration a caller passes when opening a transaction.
#[derive(Debug, Clone)]
pub struct TransactionSpec {
    pub class: UndoClass,
    /// Stable originator identifier (e.g. `user_keystroke`,
    /// `command:rename_local`, `save_participant:format_on_save`,
    /// `vfs_external_change`).
    pub originator: String,
    /// Human-readable label. Required for named-group classes.
    pub label: Option<String>,
}

impl TransactionSpec {
    pub fn new(class: UndoClass, originator: impl Into<String>) -> Self {
        Self {
            class,
            originator: originator.into(),
            label: None,
        }
    }
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }
}

/// What `commit` returns to the caller.
#[derive(Debug, Clone)]
pub struct CommittedInfo {
    pub transaction_id: TransactionId,
    pub class_id: &'static str,
    pub compensation_posture: CompensationPosture,
    pub undo_group_id: UndoGroupId,
    pub parent_snapshot: SnapshotId,
    pub produced_snapshot: SnapshotId,
    pub operation_count: usize,
    pub inserted_bytes: usize,
    pub removed_bytes: usize,
}

/// Outcome of an `undo` or `redo` step.
#[derive(Debug, Clone)]
pub struct UndoOutcome {
    pub transaction_id: TransactionId,
    pub class_id: &'static str,
    pub compensation_posture: CompensationPosture,
    pub undo_group_id: UndoGroupId,
    pub produced_snapshot: SnapshotId,
}

// ---------------------------------------------------------------------------
// Internal representation.
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Source {
    Original,
    Append,
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    source: Source,
    start: usize,
    len: usize,
}

#[derive(Debug, Clone)]
struct EditOp {
    /// The byte range that was replaced. `range.len()` is the number
    /// of removed bytes; `inserted.len()` is the number of inserted
    /// bytes.
    range: Range<usize>,
    removed: Vec<u8>,
    inserted: Vec<u8>,
}

#[derive(Debug, Clone)]
struct CommittedGroup {
    transaction_id: TransactionId,
    class: UndoClass,
    originator: String,
    label: Option<String>,
    undo_group_id: UndoGroupId,
    parent_snapshot: SnapshotId,
    produced_snapshot: SnapshotId,
    operations: Vec<EditOp>,
    /// For `OnlyRevertible` classes: the piece list and total length
    /// of the parent snapshot. Undo restores this wholesale.
    parent_pieces: Option<(Vec<Piece>, usize)>,
}

impl CommittedGroup {
    fn inserted_bytes(&self) -> usize {
        self.operations.iter().map(|op| op.inserted.len()).sum()
    }
    fn removed_bytes(&self) -> usize {
        self.operations.iter().map(|op| op.removed.len()).sum()
    }
    fn inverse_storage_bytes(&self) -> usize {
        let op_bytes = self.inserted_bytes() + self.removed_bytes();
        let snap_bytes = self
            .parent_pieces
            .as_ref()
            .map(|(_, total)| *total)
            .unwrap_or(0);
        op_bytes + snap_bytes
    }
}

struct OpenTransaction {
    class: UndoClass,
    originator: String,
    label: Option<String>,
    undo_group_id: UndoGroupId,
    parent_snapshot_id: SnapshotId,
    parent_version: u64,
    // Rollback state captured before any mutation.
    rollback_pieces: Vec<Piece>,
    rollback_total_len: usize,
    rollback_append_len: usize,
    operations: Vec<EditOp>,
}

// ---------------------------------------------------------------------------
// Configuration.
// ---------------------------------------------------------------------------

/// Knobs exposed to the prototype caller. Production defaults live
/// in workspace policy; the prototype exposes them here so benches
/// and tests can vary them without editing buffer internals.
#[derive(Debug, Clone)]
pub struct BufferConfig {
    /// Hard cap on the stored inverse size (per committed group).
    /// Exceeding this cap fires `journal_inverse_rejected` and the
    /// transaction rolls back. Matches the ADR rule that large-file
    /// journals MAY reject transactions rather than silently truncate.
    pub inverse_cap_bytes: usize,
}

impl Default for BufferConfig {
    fn default() -> Self {
        Self {
            // Generous default for the prototype; benches override it
            // to exercise the rejection path deterministically.
            inverse_cap_bytes: 64 * 1024 * 1024,
        }
    }
}

// ---------------------------------------------------------------------------
// Buffer.
// ---------------------------------------------------------------------------

pub struct Buffer {
    original: Arc<Vec<u8>>,
    append: Vec<u8>,
    pieces: Vec<Piece>,
    total_len: usize,
    version: u64,

    next_transaction_id: u64,
    next_group_id: u64,
    next_snapshot_id: u64,
    next_checkpoint_handle: u64,

    latest_snapshot_id: SnapshotId,

    journal: Vec<CommittedGroup>,
    redo_stack: Vec<CommittedGroup>,

    open: Option<OpenTransaction>,

    counters: HookCounters,
    config: BufferConfig,
}

impl Buffer {
    pub fn new() -> Self {
        Self::with_config(BufferConfig::default())
    }

    pub fn with_config(config: BufferConfig) -> Self {
        Self::from_bytes_with_config(&[], config)
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        Self::from_bytes_with_config(bytes, BufferConfig::default())
    }

    pub fn from_str(text: &str) -> Self {
        Self::from_bytes(text.as_bytes())
    }

    pub fn from_bytes_with_config(bytes: &[u8], config: BufferConfig) -> Self {
        let original = Arc::new(bytes.to_vec());
        let total_len = original.len();
        let pieces = if total_len == 0 {
            Vec::new()
        } else {
            vec![Piece {
                source: Source::Original,
                start: 0,
                len: total_len,
            }]
        };
        let mut counters = HookCounters::default();
        counters.buffer_open += 1;
        let snapshot_id = SnapshotId(0);
        Self {
            original,
            append: Vec::new(),
            pieces,
            total_len,
            version: 0,
            next_transaction_id: 1,
            next_group_id: 1,
            next_snapshot_id: 1,
            next_checkpoint_handle: 1,
            latest_snapshot_id: snapshot_id,
            journal: Vec::new(),
            redo_stack: Vec::new(),
            open: None,
            counters,
            config,
        }
    }

    // -- Read API --------------------------------------------------------

    pub fn len(&self) -> usize {
        self.total_len
    }

    pub fn is_empty(&self) -> bool {
        self.total_len == 0
    }

    pub fn version(&self) -> u64 {
        self.version
    }

    /// Materialise the current buffer contents into a new vector.
    pub fn contents(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(self.total_len);
        for piece in &self.pieces {
            out.extend_from_slice(self.slice(*piece));
        }
        out
    }

    /// Take a snapshot of the current buffer state. Fires
    /// `snapshot_create`. Does not mutate active buffer state beyond
    /// incrementing the counter; callers may hold the snapshot while
    /// edits continue.
    pub fn snapshot(&mut self) -> Snapshot {
        self.counters.snapshot_create += 1;
        let id = SnapshotId(self.next_snapshot_id);
        self.next_snapshot_id += 1;
        self.latest_snapshot_id = id;
        Snapshot {
            id,
            version: self.version,
            content: Arc::new(self.contents()),
        }
    }

    /// Record a checkpoint. Fires `checkpoint_create`. In the
    /// production buffer this pairs with a durable recovery-journal
    /// write; in the prototype it only fires the hook and returns a
    /// monotonic handle.
    pub fn create_checkpoint(&mut self) -> CheckpointHandle {
        self.counters.checkpoint_create += 1;
        let h = CheckpointHandle(self.next_checkpoint_handle);
        self.next_checkpoint_handle += 1;
        h
    }

    pub fn hook_counters(&self) -> &HookCounters {
        &self.counters
    }

    pub fn journal_len(&self) -> usize {
        self.journal.len()
    }

    pub fn redo_len(&self) -> usize {
        self.redo_stack.len()
    }

    pub fn has_open_transaction(&self) -> bool {
        self.open.is_some()
    }

    /// Iterate the committed journal newest-last.
    pub fn journal(&self) -> JournalView<'_> {
        JournalView {
            inner: self.journal.iter(),
        }
    }

    // -- Transaction API -------------------------------------------------

    /// Open a new transaction. The returned handle records subsequent
    /// insert/delete/replace calls under one `undo_group_id`. The
    /// transaction MUST be committed or aborted before a new one is
    /// opened; dropping the handle without calling `commit` aborts.
    pub fn begin(&mut self, spec: TransactionSpec) -> Result<Transaction<'_>, BufferError> {
        if self.open.is_some() {
            return Err(BufferError::TransactionAlreadyOpen);
        }
        if spec.class.is_named_group() && spec.label.is_none() {
            return Err(BufferError::MissingLabelForNamedGroup {
                class_id: spec.class.class_id(),
            });
        }
        let group_id = UndoGroupId(self.next_group_id);
        self.next_group_id += 1;
        let parent_snapshot_id = self.latest_snapshot_id;
        self.open = Some(OpenTransaction {
            class: spec.class,
            originator: spec.originator,
            label: spec.label,
            undo_group_id: group_id,
            parent_snapshot_id,
            parent_version: self.version,
            rollback_pieces: self.pieces.clone(),
            rollback_total_len: self.total_len,
            rollback_append_len: self.append.len(),
            operations: Vec::new(),
        });
        if spec.class.is_named_group() {
            self.counters.undo_group_open += 1;
        }
        Ok(Transaction {
            buffer: self,
            finished: false,
        })
    }

    /// Convenience: open a single-op `text_edit` transaction, apply
    /// one insert, and commit. Mirrors what a single keystroke looks
    /// like when the editor has no coalescing state to extend.
    pub fn insert(
        &mut self,
        offset: usize,
        text: &str,
        originator: impl Into<String>,
    ) -> Result<CommittedInfo, BufferError> {
        let mut tx = self.begin(TransactionSpec::new(UndoClass::TextEdit, originator))?;
        tx.insert(offset, text)?;
        Ok(tx.commit()?)
    }

    /// Convenience: single-op `text_edit` delete-and-commit.
    pub fn delete(
        &mut self,
        range: Range<usize>,
        originator: impl Into<String>,
    ) -> Result<CommittedInfo, BufferError> {
        let mut tx = self.begin(TransactionSpec::new(UndoClass::TextEdit, originator))?;
        tx.delete(range)?;
        Ok(tx.commit()?)
    }

    /// Convenience: single-op `text_edit` replace-and-commit.
    pub fn replace(
        &mut self,
        range: Range<usize>,
        text: &str,
        originator: impl Into<String>,
    ) -> Result<CommittedInfo, BufferError> {
        let mut tx = self.begin(TransactionSpec::new(UndoClass::TextEdit, originator))?;
        tx.replace(range, text)?;
        Ok(tx.commit()?)
    }

    // -- Undo / redo -----------------------------------------------------

    /// Undo the most-recent committed group. Returns `None` if the
    /// journal is empty. Fires `undo_apply` and, for named groups,
    /// `undo_group_close`.
    pub fn undo(&mut self) -> Option<UndoOutcome> {
        if self.open.is_some() {
            return None;
        }
        let group = self.journal.pop()?;
        self.apply_inverse(&group);
        self.counters.undo_apply += 1;
        if group.class.is_named_group() {
            self.counters.undo_group_close += 1;
        }
        self.version += 1;
        let produced = SnapshotId(self.next_snapshot_id);
        self.next_snapshot_id += 1;
        self.latest_snapshot_id = produced;
        let outcome = UndoOutcome {
            transaction_id: group.transaction_id,
            class_id: group.class.class_id(),
            compensation_posture: group.class.compensation_posture(),
            undo_group_id: group.undo_group_id,
            produced_snapshot: produced,
        };
        self.redo_stack.push(group);
        Some(outcome)
    }

    /// Redo the most-recently-undone group. Returns `None` if the
    /// redo stack is empty. Fires `redo_apply`.
    pub fn redo(&mut self) -> Option<UndoOutcome> {
        if self.open.is_some() {
            return None;
        }
        let group = self.redo_stack.pop()?;
        self.apply_forward(&group);
        self.counters.redo_apply += 1;
        self.version += 1;
        let produced = SnapshotId(self.next_snapshot_id);
        self.next_snapshot_id += 1;
        self.latest_snapshot_id = produced;
        let outcome = UndoOutcome {
            transaction_id: group.transaction_id,
            class_id: group.class.class_id(),
            compensation_posture: group.class.compensation_posture(),
            undo_group_id: group.undo_group_id,
            produced_snapshot: produced,
        };
        self.journal.push(group);
        Some(outcome)
    }

    // -- Internals -------------------------------------------------------

    fn slice(&self, piece: Piece) -> &[u8] {
        let buf: &[u8] = match piece.source {
            Source::Original => &self.original,
            Source::Append => &self.append,
        };
        &buf[piece.start..piece.start + piece.len]
    }

    /// Locate the piece containing `offset`. Returns
    /// `(piece_index, offset_within_piece)`. If `offset == total_len`,
    /// returns `(pieces.len(), 0)`.
    fn locate(&self, offset: usize) -> (usize, usize) {
        let mut cum = 0usize;
        for (i, p) in self.pieces.iter().enumerate() {
            if offset < cum + p.len {
                return (i, offset - cum);
            }
            cum += p.len;
        }
        (self.pieces.len(), 0)
    }

    /// Split a piece at internal offset, returning the piece index
    /// whose start is exactly `offset`. No-op if `offset` is already
    /// at a piece boundary.
    fn split_at(&mut self, offset: usize) -> usize {
        let (idx, within) = self.locate(offset);
        if within == 0 {
            return idx;
        }
        let p = self.pieces[idx];
        let left = Piece {
            source: p.source,
            start: p.start,
            len: within,
        };
        let right = Piece {
            source: p.source,
            start: p.start + within,
            len: p.len - within,
        };
        self.pieces[idx] = left;
        self.pieces.insert(idx + 1, right);
        idx + 1
    }

    /// Extract the bytes within `range` as a fresh `Vec<u8>`.
    fn extract_range(&self, range: Range<usize>) -> Vec<u8> {
        let mut out = Vec::with_capacity(range.end - range.start);
        let mut cum = 0usize;
        for piece in &self.pieces {
            let piece_start = cum;
            let piece_end = cum + piece.len;
            cum = piece_end;
            if piece_end <= range.start {
                continue;
            }
            if piece_start >= range.end {
                break;
            }
            let local_start = range.start.saturating_sub(piece_start);
            let local_end = (range.end - piece_start).min(piece.len);
            let s = self.slice(*piece);
            out.extend_from_slice(&s[local_start..local_end]);
        }
        out
    }

    fn apply_op_in_open(
        &mut self,
        range: Range<usize>,
        inserted: &[u8],
    ) -> Result<(), BufferError> {
        if self.open.is_none() {
            return Err(BufferError::NoOpenTransaction);
        }
        if range.start > range.end {
            return Err(BufferError::InvertedRange {
                start: range.start,
                end: range.end,
            });
        }
        if range.end > self.total_len {
            return Err(BufferError::OutOfBounds {
                offset: range.end,
                len: self.total_len,
            });
        }
        let removed = self.extract_range(range.clone());
        // Split at both ends first (end first so start index stays
        // valid in the `start == end` case).
        let end_idx = self.split_at(range.end);
        let start_idx = self.split_at(range.start);
        // end_idx might need recomputing if the start split happened
        // inside the same original piece — recompute by length.
        let end_idx = {
            let mut cum = 0usize;
            let mut found = self.pieces.len();
            for (i, p) in self.pieces.iter().enumerate() {
                if cum == range.end {
                    found = i;
                    break;
                }
                if cum > range.end {
                    break;
                }
                cum += p.len;
            }
            if cum == range.end {
                found
            } else {
                end_idx
            }
        };
        // Remove the middle pieces.
        self.pieces.drain(start_idx..end_idx);
        // Insert new piece for inserted bytes.
        if !inserted.is_empty() {
            let append_start = self.append.len();
            self.append.extend_from_slice(inserted);
            self.pieces.insert(
                start_idx,
                Piece {
                    source: Source::Append,
                    start: append_start,
                    len: inserted.len(),
                },
            );
        }
        self.total_len = self.total_len - removed.len() + inserted.len();
        let op = EditOp {
            range,
            removed,
            inserted: inserted.to_vec(),
        };
        self.open.as_mut().unwrap().operations.push(op);
        Ok(())
    }

    fn commit_open(&mut self) -> Result<CommittedInfo, BufferError> {
        // Peek at the open transaction and compute the would-be
        // inverse size without moving `open` out. If the cap rejects
        // the commit, we restore the rollback state captured at
        // begin and leave version/ids unchanged.
        let (inverse_bytes, would_produce_parent_snap_copy) = {
            let open = self.open.as_ref().ok_or(BufferError::NoOpenTransaction)?;
            let op_bytes: usize = open
                .operations
                .iter()
                .map(|op| op.inserted.len() + op.removed.len())
                .sum();
            let posture = open.class.compensation_posture();
            let snap_bytes = if posture == CompensationPosture::OnlyRevertible {
                open.rollback_total_len
            } else {
                0
            };
            (
                op_bytes + snap_bytes,
                posture == CompensationPosture::OnlyRevertible,
            )
        };
        if inverse_bytes > self.config.inverse_cap_bytes {
            self.counters.journal_inverse_rejected += 1;
            let open = self.open.take().unwrap();
            // Restore pre-transaction state wholesale. The rollback
            // pieces reference the original+append bytes; append
            // bytes added during the transaction are truncated so
            // no stale piece refers past the rollback mark.
            self.pieces = open.rollback_pieces;
            self.total_len = open.rollback_total_len;
            self.append.truncate(open.rollback_append_len);
            if open.class.is_named_group() {
                self.counters.undo_group_close += 1;
            }
            return Err(BufferError::InverseTooLarge {
                bytes: inverse_bytes,
                cap: self.config.inverse_cap_bytes,
            });
        }

        let open = self.open.take().unwrap();
        let parent_snapshot = open.parent_snapshot_id;
        let posture = open.class.compensation_posture();
        let parent_pieces = if would_produce_parent_snap_copy {
            Some((open.rollback_pieces, open.rollback_total_len))
        } else {
            None
        };
        let transaction_id = TransactionId(self.next_transaction_id);
        self.next_transaction_id += 1;
        self.version += 1;
        let produced_snapshot = SnapshotId(self.next_snapshot_id);
        self.next_snapshot_id += 1;
        self.latest_snapshot_id = produced_snapshot;
        let class = open.class;
        let group = CommittedGroup {
            transaction_id,
            class,
            originator: open.originator,
            label: open.label,
            undo_group_id: open.undo_group_id,
            parent_snapshot,
            produced_snapshot,
            operations: open.operations,
            parent_pieces,
        };
        let operation_count = group.operations.len();
        let inserted_bytes = group.inserted_bytes();
        let removed_bytes = group.removed_bytes();
        let info = CommittedInfo {
            transaction_id,
            class_id: class.class_id(),
            compensation_posture: posture,
            undo_group_id: group.undo_group_id,
            parent_snapshot,
            produced_snapshot,
            operation_count,
            inserted_bytes,
            removed_bytes,
        };
        self.counters.transaction_apply += 1;
        if class.fires_text_edit_apply() {
            self.counters.text_edit_apply += 1;
        }
        if class.is_named_group() {
            self.counters.undo_group_close += 1;
        }
        // Per the ADR: only-revertible redo pins to an exact
        // snapshot lineage, so a divergent commit drops those redo
        // entries. Compensatable redo MAY survive.
        if !self.redo_stack.is_empty() {
            self.redo_stack
                .retain(|g| g.class.compensation_posture() == CompensationPosture::Compensatable);
        }
        self.journal.push(group);
        Ok(info)
    }

    fn abort_open(&mut self) -> Result<(), BufferError> {
        let open = self.open.take().ok_or(BufferError::NoOpenTransaction)?;
        // Restore pieces, total_len, append buffer.
        self.pieces = open.rollback_pieces;
        self.total_len = open.rollback_total_len;
        self.append.truncate(open.rollback_append_len);
        if open.class.is_named_group() {
            self.counters.undo_group_close += 1;
        }
        Ok(())
    }

    /// Low-level piece-list edit used by undo/redo and inverse-cap
    /// rollback. Updates `total_len` and `append`; does not update
    /// journal state or counters.
    fn apply_raw(&mut self, range: Range<usize>, inserted: &[u8]) {
        let end_idx = self.split_at(range.end);
        let start_idx = self.split_at(range.start);
        let end_idx = {
            let mut cum = 0usize;
            let mut found = self.pieces.len();
            for (i, p) in self.pieces.iter().enumerate() {
                if cum == range.end {
                    found = i;
                    break;
                }
                if cum > range.end {
                    break;
                }
                cum += p.len;
            }
            if cum == range.end {
                found
            } else {
                end_idx
            }
        };
        let removed_len: usize = self.pieces[start_idx..end_idx].iter().map(|p| p.len).sum();
        self.pieces.drain(start_idx..end_idx);
        if !inserted.is_empty() {
            let append_start = self.append.len();
            self.append.extend_from_slice(inserted);
            self.pieces.insert(
                start_idx,
                Piece {
                    source: Source::Append,
                    start: append_start,
                    len: inserted.len(),
                },
            );
        }
        self.total_len = self.total_len - removed_len + inserted.len();
    }

    /// Undo a committed group by restoring the pre-transaction
    /// state. Only-revertible uses the stored snapshot; compensatable
    /// applies the operation inverses in reverse order.
    fn apply_inverse(&mut self, group: &CommittedGroup) {
        match group.class.compensation_posture() {
            CompensationPosture::OnlyRevertible => {
                let (pieces, total_len) = group
                    .parent_pieces
                    .as_ref()
                    .cloned()
                    .expect("only_revertible group must carry parent_pieces");
                self.pieces = pieces;
                self.total_len = total_len;
            }
            CompensationPosture::Compensatable => {
                for op in group.operations.iter().rev() {
                    let inverse_range = op.range.start..op.range.start + op.inserted.len();
                    self.apply_raw(inverse_range, &op.removed);
                }
            }
        }
    }

    /// Redo a committed group by re-applying the forward operations
    /// in order. For only-revertible groups this relies on the piece
    /// list being byte-equivalent to the original pre-transaction
    /// state; redo after a divergent commit is not supported (the
    /// ADR requires such redo stacks to be dropped, which is what
    /// `commit_open` does).
    fn apply_forward(&mut self, group: &CommittedGroup) {
        for op in &group.operations {
            self.apply_raw(op.range.clone(), &op.inserted);
        }
    }
}

impl Default for Buffer {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Transaction handle.
// ---------------------------------------------------------------------------

pub struct Transaction<'a> {
    buffer: &'a mut Buffer,
    finished: bool,
}

impl Transaction<'_> {
    pub fn insert(&mut self, offset: usize, text: &str) -> Result<(), BufferError> {
        self.buffer
            .apply_op_in_open(offset..offset, text.as_bytes())
    }

    pub fn delete(&mut self, range: Range<usize>) -> Result<(), BufferError> {
        self.buffer.apply_op_in_open(range, &[])
    }

    pub fn replace(&mut self, range: Range<usize>, text: &str) -> Result<(), BufferError> {
        self.buffer.apply_op_in_open(range, text.as_bytes())
    }

    pub fn commit(mut self) -> Result<CommittedInfo, BufferError> {
        self.finished = true;
        self.buffer.commit_open()
    }

    pub fn abort(mut self) {
        self.finished = true;
        let _ = self.buffer.abort_open();
    }
}

impl Drop for Transaction<'_> {
    fn drop(&mut self) {
        if !self.finished {
            let _ = self.buffer.abort_open();
        }
    }
}

// ---------------------------------------------------------------------------
// Journal view.
// ---------------------------------------------------------------------------

/// Read-only view of one committed transaction in the journal.
#[derive(Debug, Clone, Copy)]
pub struct JournalEntry<'a> {
    inner: &'a CommittedGroup,
}

impl<'a> JournalEntry<'a> {
    pub fn transaction_id(&self) -> TransactionId {
        self.inner.transaction_id
    }
    pub fn class_id(&self) -> &'static str {
        self.inner.class.class_id()
    }
    pub fn class(&self) -> UndoClass {
        self.inner.class
    }
    pub fn compensation_posture(&self) -> CompensationPosture {
        self.inner.class.compensation_posture()
    }
    pub fn originator(&self) -> &'a str {
        &self.inner.originator
    }
    pub fn label(&self) -> Option<&'a str> {
        self.inner.label.as_deref()
    }
    pub fn undo_group_id(&self) -> UndoGroupId {
        self.inner.undo_group_id
    }
    pub fn parent_snapshot(&self) -> SnapshotId {
        self.inner.parent_snapshot
    }
    pub fn produced_snapshot(&self) -> SnapshotId {
        self.inner.produced_snapshot
    }
    pub fn operation_count(&self) -> usize {
        self.inner.operations.len()
    }
    pub fn inserted_bytes(&self) -> usize {
        self.inner.inserted_bytes()
    }
    pub fn removed_bytes(&self) -> usize {
        self.inner.removed_bytes()
    }
    pub fn carries_parent_snapshot_copy(&self) -> bool {
        self.inner.parent_pieces.is_some()
    }
}

pub struct JournalView<'a> {
    inner: std::slice::Iter<'a, CommittedGroup>,
}

impl<'a> Iterator for JournalView<'a> {
    type Item = JournalEntry<'a>;
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|g| JournalEntry { inner: g })
    }
}

impl ExactSizeIterator for JournalView<'_> {
    fn len(&self) -> usize {
        self.inner.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_buffer_is_empty_and_counts_buffer_open() {
        let b = Buffer::new();
        assert_eq!(b.len(), 0);
        assert!(b.is_empty());
        assert_eq!(b.contents(), Vec::<u8>::new());
        assert_eq!(b.hook_counters().buffer_open, 1);
    }

    #[test]
    fn from_str_materialises_original_bytes() {
        let b = Buffer::from_str("hello");
        assert_eq!(b.len(), 5);
        assert_eq!(b.contents(), b"hello");
    }

    #[test]
    fn single_insert_replace_delete() {
        let mut b = Buffer::from_str("hello");
        b.insert(5, ", world", "user_keystroke").unwrap();
        assert_eq!(b.contents(), b"hello, world");
        b.replace(0..5, "HELLO", "command:upper").unwrap();
        assert_eq!(b.contents(), b"HELLO, world");
        b.delete(5..7, "command:trim").unwrap();
        assert_eq!(b.contents(), b"HELLOworld");
    }

    #[test]
    fn snapshot_is_stable_across_edits() {
        let mut b = Buffer::from_str("hello");
        let snap = b.snapshot();
        b.insert(5, "!", "user_keystroke").unwrap();
        assert_eq!(snap.as_bytes(), b"hello");
        assert_eq!(b.contents(), b"hello!");
        assert_eq!(b.hook_counters().snapshot_create, 1);
    }

    #[test]
    fn multi_op_transaction_is_atomic_on_abort() {
        let mut b = Buffer::from_str("hello");
        let before = b.contents();
        let before_version = b.version();
        let before_journal = b.journal_len();
        {
            let mut tx = b
                .begin(TransactionSpec::new(
                    UndoClass::StructuralEdit,
                    "command:sort_lines",
                ))
                .unwrap();
            tx.insert(0, "A").unwrap();
            // After the first insert the buffer is `Ahello` (len 6).
            // Appending at offset 6 is valid.
            tx.insert(6, "Z").unwrap();
            // Drop without commit => abort.
        }
        assert_eq!(b.contents(), before);
        assert_eq!(b.journal_len(), before_journal);
        assert_eq!(b.version(), before_version);
        assert!(!b.has_open_transaction());
    }

    #[test]
    fn undo_and_redo_text_edit() {
        let mut b = Buffer::from_str("hello");
        b.insert(5, "!", "user_keystroke").unwrap();
        assert_eq!(b.contents(), b"hello!");
        let u = b.undo().unwrap();
        assert_eq!(u.class_id, "text_edit");
        assert_eq!(u.compensation_posture, CompensationPosture::Compensatable);
        assert_eq!(b.contents(), b"hello");
        b.redo().unwrap();
        assert_eq!(b.contents(), b"hello!");
    }

    #[test]
    fn multi_cursor_transaction_is_one_undo_group() {
        let mut b = Buffer::from_str("a\nb\nc");
        // Simulate three cursors inserting `,` — apply right-to-left
        // so earlier insertions do not shift later offsets.
        let mut tx = b
            .begin(TransactionSpec::new(
                UndoClass::MultiCursorTextEdit,
                "user_keystroke:multi_cursor",
            ))
            .unwrap();
        tx.insert(5, ",").unwrap();
        tx.insert(3, ",").unwrap();
        tx.insert(1, ",").unwrap();
        let info = tx.commit().unwrap();
        assert_eq!(info.class_id, "multi_cursor_text_edit");
        assert_eq!(info.operation_count, 3);
        assert_eq!(b.contents(), b"a,\nb,\nc,");
        // One undo reverts all three commas in one step.
        b.undo().unwrap();
        assert_eq!(b.contents(), b"a\nb\nc");
        // Redo re-applies all three atomically.
        b.redo().unwrap();
        assert_eq!(b.contents(), b"a,\nb,\nc,");
    }

    #[test]
    fn only_revertible_redo_dropped_on_divergent_commit() {
        let mut b = Buffer::from_str("one\ntwo\n");
        // Multi-file refactor (only_revertible) followed by undo +
        // divergent text_edit — the multi-file refactor's redo entry
        // MUST be dropped.
        {
            let mut tx = b
                .begin(
                    TransactionSpec::new(
                        UndoClass::RefactorMultiFile,
                        "command:rename_symbol_workspace",
                    )
                    .with_label("Rename across workspace"),
                )
                .unwrap();
            tx.replace(0..3, "ONE").unwrap();
            tx.commit().unwrap();
        }
        assert_eq!(b.contents(), b"ONE\ntwo\n");
        b.undo().unwrap();
        assert_eq!(b.contents(), b"one\ntwo\n");
        assert_eq!(b.redo_len(), 1);
        // Divergent edit (compensatable).
        b.insert(3, "!", "user_keystroke").unwrap();
        // Redo stack for only_revertible group is gone.
        assert_eq!(b.redo_len(), 0);
    }

    #[test]
    fn compensatable_redo_survives_another_compensatable_commit() {
        let mut b = Buffer::from_str("abc");
        b.insert(3, "X", "user_keystroke").unwrap(); // commit 1, compensatable
        b.insert(4, "Y", "user_keystroke").unwrap(); // commit 2, compensatable
        b.undo().unwrap(); // undoes commit 2 -> redo stack [commit2]
        assert_eq!(b.contents(), b"abcX");
        b.insert(4, "Z", "user_keystroke").unwrap(); // divergent commit 3, compensatable
                                                     // Per ADR: compensatable classes MAY be redone after a
                                                     // divergent edit. The prototype preserves the redo entry.
        assert_eq!(b.redo_len(), 1);
        b.redo().unwrap(); // replay commit 2 at its recorded offset
                           // The redo reinserts "Y" at offset 4 on top of the divergent
                           // state "abcXZ", producing "abcXYZ". Exact byte-by-byte
                           // reconstruction of the pre-undo state is not promised for
                           // compensatable redo-after-divergence; the contract is that
                           // the recorded operation is reapplied.
        assert_eq!(b.contents(), b"abcXYZ");
        assert_eq!(b.hook_counters().redo_apply, 1);
    }

    #[test]
    fn named_group_requires_label() {
        let mut b = Buffer::new();
        let err = b
            .begin(TransactionSpec::new(
                UndoClass::RefactorSingleFile,
                "command:rename_local",
            ))
            .unwrap_err();
        assert!(matches!(err, BufferError::MissingLabelForNamedGroup { .. }));
    }

    #[test]
    fn sequential_open_abort_reopen_is_stable() {
        // The borrow checker prevents two `Transaction` handles
        // aliasing the same buffer, so `TransactionAlreadyOpen` is a
        // defensive check. What we CAN exercise directly is the
        // open/abort-drop/reopen sequence — the invariant that
        // leaving a transaction via Drop restores the buffer to a
        // state where a new transaction can begin cleanly.
        let mut b = Buffer::from_str("hi");
        {
            let mut tx = b
                .begin(TransactionSpec::new(UndoClass::TextEdit, "user_keystroke"))
                .unwrap();
            tx.insert(0, "X").unwrap();
        } // drop without commit -> abort
        assert!(!b.has_open_transaction());
        assert_eq!(b.contents(), b"hi");
        // Reopen and commit succeeds.
        let mut tx = b
            .begin(TransactionSpec::new(UndoClass::TextEdit, "user_keystroke"))
            .unwrap();
        tx.insert(0, "Y").unwrap();
        tx.commit().unwrap();
        assert_eq!(b.contents(), b"Yhi");
    }

    #[test]
    fn repeated_open_close_cycles_are_stable() {
        let mut b = Buffer::from_str("seed");
        const CYCLES: usize = 64;
        for i in 0..CYCLES {
            let offset = b.len();
            let mut tx = b
                .begin(TransactionSpec::new(UndoClass::TextEdit, "user_keystroke"))
                .unwrap();
            tx.insert(offset, ".").unwrap();
            tx.commit().unwrap();
            assert_eq!(b.len(), 4 + i + 1);
        }
        // Journal grew monotonically and counters match commit count.
        assert_eq!(b.hook_counters().transaction_apply, CYCLES as u64);
        assert_eq!(b.hook_counters().text_edit_apply, CYCLES as u64);
        assert_eq!(b.journal_len(), CYCLES);
        // Undo every commit; journal empties; redo fills.
        for _ in 0..CYCLES {
            b.undo().unwrap();
        }
        assert_eq!(b.journal_len(), 0);
        assert_eq!(b.redo_len(), CYCLES);
        assert_eq!(b.contents(), b"seed");
        // Redo every commit; state restored.
        for _ in 0..CYCLES {
            b.redo().unwrap();
        }
        let expected: Vec<u8> = b"seed"
            .iter()
            .copied()
            .chain(std::iter::repeat(b'.').take(CYCLES))
            .collect();
        assert_eq!(b.contents(), expected);
        assert_eq!(b.hook_counters().undo_apply, CYCLES as u64);
        assert_eq!(b.hook_counters().redo_apply, CYCLES as u64);
        // Abort-only cycles also leave no journal residue.
        for _ in 0..16 {
            let mut tx = b
                .begin(TransactionSpec::new(UndoClass::TextEdit, "user_keystroke"))
                .unwrap();
            tx.insert(0, "XX").unwrap();
            tx.abort();
        }
        // Abort does not touch the journal or the redo stack.
        assert_eq!(b.journal_len(), CYCLES);
        assert_eq!(b.redo_len(), 0);
        assert_eq!(b.contents(), expected);
    }

    #[test]
    fn snapshot_create_does_not_mutate_observable_state() {
        let mut b = Buffer::from_str("hello");
        let len_before = b.len();
        let version_before = b.version();
        let contents_before = b.contents();
        let snap = b.snapshot();
        assert_eq!(b.len(), len_before);
        assert_eq!(b.version(), version_before);
        assert_eq!(b.contents(), contents_before);
        assert_eq!(snap.version(), version_before);
        assert_eq!(snap.as_bytes(), contents_before.as_slice());
    }

    #[test]
    fn inverse_cap_rejection_rolls_back() {
        let mut b = Buffer::from_bytes_with_config(
            b"abc",
            BufferConfig {
                inverse_cap_bytes: 4, // tiny cap
            },
        );
        let before = b.contents();
        let before_version = b.version();
        let err = b.insert(3, "0123456789", "user_keystroke").unwrap_err();
        assert!(matches!(err, BufferError::InverseTooLarge { .. }));
        assert_eq!(b.contents(), before);
        assert_eq!(b.version(), before_version);
        assert_eq!(b.hook_counters().journal_inverse_rejected, 1);
        assert_eq!(b.journal_len(), 0);
    }

    #[test]
    fn save_participant_group_fires_named_group_hooks() {
        let mut b = Buffer::from_str("a=1\nb=2\n");
        let mut tx = b
            .begin(
                TransactionSpec::new(UndoClass::SaveParticipantGroup, "command:save")
                    .with_label("Save + format + organise imports"),
            )
            .unwrap();
        tx.replace(0..3, "a = 1").unwrap();
        tx.replace(6..9, "b = 2").unwrap();
        tx.commit().unwrap();
        let c = b.hook_counters();
        assert_eq!(c.undo_group_open, 1);
        assert_eq!(c.undo_group_close, 1);
        assert_eq!(c.transaction_apply, 1);
        assert_eq!(c.text_edit_apply, 0); // save_participant_group does not fire it
    }
}
