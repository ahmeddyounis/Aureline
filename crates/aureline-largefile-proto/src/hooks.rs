//! Large-file path hook counters.
//!
//! Hook ids match the names frozen in
//! `docs/adr/0003-buffer-undo-large-file.md` § Protected-hot-path
//! hooks. The prototype counts; the production build replaces
//! the struct with a telemetry seam behind the same names.
//!
//! Counts only (no wall-clock latencies) so harness output is
//! byte-stable across hosts. The benchmark lab layers timing on
//! top of these counters when it scores against the ADR's
//! protected-hot-path budgets.

/// Per-large-file-buffer hook-fire counters.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HookCounters {
    /// Fires once per successful open.
    pub buffer_open: u64,
    /// Fires when the buffer enters large-file mode.
    pub large_file_mode_enter: u64,
    /// Fires when the buffer leaves large-file mode (explicit
    /// operator downgrade). The prototype exposes this even though
    /// leaving the mode is out of scope for the first prototype.
    pub large_file_mode_exit: u64,
    /// Fires when a save participant rebases or aborts because
    /// the on-disk file changed mid-flight.
    pub save_participant_rebase: u64,
    /// Fires when the journal refuses a transaction whose stored
    /// inverse exceeds the per-buffer cap.
    pub journal_inverse_rejected: u64,
    /// Fires when the buffer performs a paged read. Observability
    /// only; not a protected-hot-path hook but useful for the
    /// benchmark lab. Named here so lanes do not invent synonyms.
    pub paged_read: u64,
    /// Fires when a classification decision is recorded for a
    /// buffer, regardless of outcome. Observability only.
    pub classification_recorded: u64,
}

impl HookCounters {
    /// Ordered `(hook_id, count)` pairs — deterministic iteration
    /// order so harness JSON is byte-stable.
    pub fn entries(&self) -> [(&'static str, u64); 7] {
        [
            ("buffer_open", self.buffer_open),
            ("large_file_mode_enter", self.large_file_mode_enter),
            ("large_file_mode_exit", self.large_file_mode_exit),
            ("save_participant_rebase", self.save_participant_rebase),
            ("journal_inverse_rejected", self.journal_inverse_rejected),
            ("paged_read", self.paged_read),
            ("classification_recorded", self.classification_recorded),
        ]
    }
}
