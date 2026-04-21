//! Hook counters for the buffer prototype.
//!
//! Structural counts only (no wall-clock latencies) so harness output
//! is byte-stable across hosts. The benchmark lab layers timing on
//! top of these counters; the prototype never claims a budget.
//!
//! Hook names are frozen in
//! `docs/adr/0003-buffer-undo-large-file.md` § Protected-hot-path
//! hooks. Lanes MUST NOT invent alternative names for the same
//! measurement.

/// Per-buffer hook-fire counters.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HookCounters {
    pub buffer_open: u64,
    pub text_edit_apply: u64,
    pub transaction_apply: u64,
    pub snapshot_create: u64,
    pub checkpoint_create: u64,
    pub undo_group_open: u64,
    pub undo_group_close: u64,
    pub undo_apply: u64,
    pub redo_apply: u64,
    pub journal_inverse_rejected: u64,
}

impl HookCounters {
    /// Ordered list of `(hook_id, count)` pairs — deterministic
    /// iteration order so harness JSON is byte-stable.
    pub fn entries(&self) -> [(&'static str, u64); 10] {
        [
            ("buffer_open", self.buffer_open),
            ("text_edit_apply", self.text_edit_apply),
            ("transaction_apply", self.transaction_apply),
            ("snapshot_create", self.snapshot_create),
            ("checkpoint_create", self.checkpoint_create),
            ("undo_group_open", self.undo_group_open),
            ("undo_group_close", self.undo_group_close),
            ("undo_apply", self.undo_apply),
            ("redo_apply", self.redo_apply),
            ("journal_inverse_rejected", self.journal_inverse_rejected),
        ]
    }
}
