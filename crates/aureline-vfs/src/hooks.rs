//! Protected-hot-path and observability-only hook counters for
//! the VFS and save pipeline.
//!
//! Hook ids match the names frozen in
//! `docs/adr/0006-vfs-save-cache-identity.md` § Protected-hot-path
//! hooks. The prototype counts; a production build replaces the
//! struct with a telemetry seam behind the same names.
//!
//! Counts only (no wall-clock latencies) so the emitted save-plan
//! records stay byte-stable across hosts. The benchmark lab layers
//! timing on top of these counters when it scores against the
//! protected-hot-path budgets the ADR freezes.

/// Per-workspace hook-fire counters.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct HookCounters {
    /// Fires once per root attached to the workspace.
    pub vfs_root_attach: u64,
    /// Fires once per root detached.
    pub vfs_root_detach: u64,
    /// Fires when the VFS resolves a presentation path to a
    /// canonical filesystem object.
    pub vfs_canonicalize: u64,
    /// Fires when two presentation paths are detected to resolve
    /// to the same canonical object and are converged onto one
    /// buffer authority.
    pub vfs_alias_converge: u64,
    /// Fires when a primary or fallback watcher emits an event.
    pub vfs_watcher_event: u64,
    /// Observability-only: fires when `watcher_health` transitions.
    pub vfs_watcher_health_changed: u64,
    /// Fires when the VFS observes a canonical-object change from
    /// an external source (compare-before-write mismatch).
    pub vfs_external_change_detected: u64,
    /// Fires when the save pipeline stages a buffer snapshot
    /// against a save-target token.
    pub vfs_save_stage: u64,
    /// Fires when a save participant starts running on staged
    /// content. The prototype models a single text-normalisation
    /// participant per successful save.
    pub vfs_save_participant_run: u64,
    /// Fires when a save participant fails; the pipeline halts.
    pub vfs_save_participant_failed: u64,
    /// Fires when the VFS re-reads the target's generation token
    /// and compares it to the save-target token.
    pub vfs_save_compare_before_write: u64,
    /// Fires when compare-before-write or a conditional-write
    /// precondition yields a conflict.
    pub vfs_save_conflict: u64,
    /// Fires when an `atomic_replace` save commits via rename.
    pub vfs_save_atomic_commit: u64,
    /// Fires when an `in_place_write` save commits.
    pub vfs_save_in_place_commit: u64,
    /// Fires when a `conditional_remote_write` save commits.
    pub vfs_save_remote_conditional_commit: u64,
    /// Observability-only: fires when a save-mode is `blocked` for
    /// the attempted target.
    pub vfs_save_blocked: u64,
    /// Fires when the VFS writes the save manifest post-commit.
    pub vfs_save_manifest_record: u64,
    /// Observability-only: fires when a rename plan is produced.
    pub vfs_rename_plan_previewed: u64,
    /// Fires when a rename commits. Unused in the prototype; the
    /// rename-plan surface lands separately.
    pub vfs_rename_commit: u64,
    /// Fires when a cache consumer requests a cache entry. Unused
    /// in the prototype; named so downstream lanes do not invent a
    /// synonym.
    pub vfs_cache_lookup: u64,
    /// Fires when a cache entry is invalidated because an
    /// `input_digest_set` member changed. Unused in the prototype.
    pub vfs_cache_invalidate: u64,
    /// Observability-only: fires when a `durable` cache
    /// supersession check fails and a rebuild begins. Unused in
    /// the prototype.
    pub vfs_cache_rebuild: u64,
    /// Observability-only: fires when a save commits under a
    /// degraded class.
    pub vfs_degraded_guarantee_declared: u64,
}

impl HookCounters {
    /// Ordered `(hook_id, count)` pairs — deterministic iteration
    /// order so harness JSON is byte-stable.
    pub fn entries(&self) -> [(&'static str, u64); 23] {
        [
            ("vfs_root_attach", self.vfs_root_attach),
            ("vfs_root_detach", self.vfs_root_detach),
            ("vfs_canonicalize", self.vfs_canonicalize),
            ("vfs_alias_converge", self.vfs_alias_converge),
            ("vfs_watcher_event", self.vfs_watcher_event),
            (
                "vfs_watcher_health_changed",
                self.vfs_watcher_health_changed,
            ),
            (
                "vfs_external_change_detected",
                self.vfs_external_change_detected,
            ),
            ("vfs_save_stage", self.vfs_save_stage),
            ("vfs_save_participant_run", self.vfs_save_participant_run),
            (
                "vfs_save_participant_failed",
                self.vfs_save_participant_failed,
            ),
            (
                "vfs_save_compare_before_write",
                self.vfs_save_compare_before_write,
            ),
            ("vfs_save_conflict", self.vfs_save_conflict),
            ("vfs_save_atomic_commit", self.vfs_save_atomic_commit),
            ("vfs_save_in_place_commit", self.vfs_save_in_place_commit),
            (
                "vfs_save_remote_conditional_commit",
                self.vfs_save_remote_conditional_commit,
            ),
            ("vfs_save_blocked", self.vfs_save_blocked),
            ("vfs_save_manifest_record", self.vfs_save_manifest_record),
            ("vfs_rename_plan_previewed", self.vfs_rename_plan_previewed),
            ("vfs_rename_commit", self.vfs_rename_commit),
            ("vfs_cache_lookup", self.vfs_cache_lookup),
            ("vfs_cache_invalidate", self.vfs_cache_invalidate),
            ("vfs_cache_rebuild", self.vfs_cache_rebuild),
            (
                "vfs_degraded_guarantee_declared",
                self.vfs_degraded_guarantee_declared,
            ),
        ]
    }
}
