//! Restore-proposal builder.
//!
//! A `RestoreProposal` is the canonical pre-rehydration summary that recovery
//! surfaces show after abnormal termination. It joins the latest workspace
//! checkpoint, window-topology snapshot, and crash-journal entries into a
//! shape that is honest about what comes back as live state, what comes back
//! as a placeholder skeleton, and what is retained as evidence only.
//!
//! The builder owns three honesty invariants that downstream consumers must
//! preserve:
//!
//! 1. **Counts before rehydration.** `RestoreProposalCounts` is computed from
//!    persisted artifacts before any pane, terminal, or transient task is
//!    woken up. The numbers visible in the proposal must match what the
//!    shell will actually try to restore.
//! 2. **No silent rerun.** Side-effectful surfaces (terminals, debuggers,
//!    notebook kernels, AI panels, remote sessions) are classified as
//!    `BlockedSideEffectful` so the shell skeletons them as placeholders and
//!    requires explicit user intent before rerunning.
//! 3. **Honest restore class.** `RestoreClass` is derived from what is
//!    actually available — never speculative. Missing or corrupt frames
//!    downgrade the class and record the trigger.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::crash_journal::{
    AutosaveJournalEntryRecord, ChecksumAlgorithm, CrashJournalStore, FrameIntegrityState,
    GuidedChoiceClass, ReplayPostureClass,
};

use super::records::{
    DowngradeTriggerClass, RestoreClass, StablePaneInventoryEntry, SurfaceClass, SurfaceRole,
    TerminalPaneRestoreMetadata, WindowTopologySnapshotRecord, WorkspaceAuthorityCheckpointRecord,
};
use super::store::{SessionRestoreError, SessionRestoreStore};

/// Schema version for `RestoreProposalRecord`.
pub type RestoreProposalSchemaVersion = u32;

/// Honest classification of a single pane's restore plan.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreProposalPlanKind {
    /// Open a lightweight skeleton view; user-driven hydration completes it.
    LiveSkeleton,
    /// Show a placeholder card; no live surface is opened automatically.
    PlaceholderOnly,
    /// Retain record as evidence only; the surface is not re-opened.
    EvidenceOnly,
    /// Side-effectful surface: never auto-rerun. User must opt in explicitly.
    BlockedSideEffectful,
}

/// Counts captured from persisted artifacts before any rehydration runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RestoreProposalCounts {
    pub windows: usize,
    pub tab_groups: usize,
    pub tabs: usize,
    pub dirty_buffer_journals: usize,
    pub transient_tasks: usize,
    pub terminals: usize,
    pub evidence_packets: usize,
    pub recovery_packets: usize,
}

/// References to the persisted artifacts that back a proposal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RestoreProposalArtifactRefs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workspace_authority_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_id: Option<String>,
}

/// Per-pane plan describing how the surface returns after restore.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProposalPanePlan {
    pub pane_id: String,
    pub surface_role: SurfaceRole,
    pub surface_class: SurfaceClass,
    pub plan_kind: RestoreProposalPlanKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_metadata: Option<TerminalPaneRestoreMetadata>,
    pub note: String,
}

/// Per-buffer dirty-draft entry surfaced for review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProposalDirtyBufferEntry {
    pub journal_entry_id: String,
    pub journal_id: String,
    pub object_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<String>,
    pub replay_posture: ReplayPostureClass,
    pub frame_integrity: FrameIntegrityState,
    pub recommended_choice: GuidedChoiceClass,
}

/// Pre-rehydration restore proposal: the canonical summary surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreProposal {
    pub record_kind: String,
    pub restore_proposal_schema_version: RestoreProposalSchemaVersion,
    pub prior_run_abnormal: bool,
    pub restore_class: RestoreClass,
    pub counts: RestoreProposalCounts,
    pub artifact_refs: RestoreProposalArtifactRefs,
    pub pane_plans: Vec<RestoreProposalPanePlan>,
    pub dirty_buffer_entries: Vec<RestoreProposalDirtyBufferEntry>,
    pub downgrade_triggers: Vec<DowngradeTriggerClass>,
    pub auto_rerun_forbidden: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

/// Runtime dependencies used while applying a [`RestoreProposal`].
///
/// The runtime owns no shell state. It only provides access to the persisted
/// session-restore and crash-journal stores so execution can resolve the
/// proposal into concrete pane and dirty-buffer replay requests.
#[derive(Debug)]
pub struct RestoreRuntime<'a> {
    session_store: &'a SessionRestoreStore,
    crash_store: &'a CrashJournalStore,
}

impl<'a> RestoreRuntime<'a> {
    /// Creates a restore runtime over the active workspace recovery stores.
    pub fn new(session_store: &'a SessionRestoreStore, crash_store: &'a CrashJournalStore) -> Self {
        Self {
            session_store,
            crash_store,
        }
    }
}

/// Execution posture for one pane in a restore outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestorePaneExecutionKind {
    /// Reopen the pane as a lightweight, non-mutating shell surface.
    Reopened,
    /// Recreate the pane as a placeholder that requires user action to hydrate.
    Placeholder,
    /// Retain the pane as restore evidence without opening a live surface.
    EvidenceOnly,
    /// Keep a side-effectful surface inactive so no command or authority reruns.
    BlockedSideEffectful,
}

/// Result of executing a single pane plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorePaneOutcome {
    pub pane_id: String,
    pub surface_role: SurfaceRole,
    pub surface_class: SurfaceClass,
    pub execution_kind: RestorePaneExecutionKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restore_metadata: Option<TerminalPaneRestoreMetadata>,
    pub note: String,
}

/// Dirty-buffer body resolved from the crash journal for live replay.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreDirtyBufferReplay {
    pub journal_entry_id: String,
    pub journal_id: String,
    pub object_ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presentation_hint: Option<String>,
    pub body_object_ref: String,
    pub bytes: Vec<u8>,
    pub replay_posture: ReplayPostureClass,
    pub frame_integrity: FrameIntegrityState,
    pub recommended_choice: GuidedChoiceClass,
}

/// Typed reason a dirty-buffer entry could not be replayed automatically.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreDirtyBufferFailureKind {
    /// The proposal or journal marks the frame unsafe for automatic replay.
    ManualRepairRequired,
    /// The journal entry referenced by the proposal is missing.
    MissingJournalEntry,
    /// The journal entry has no available body object.
    MissingJournalBody,
    /// The body object checksum no longer matches the journal integrity record.
    ChecksumMismatch,
    /// The crash-journal store could not be read.
    StoreUnavailable,
}

/// Dirty-buffer entry retained for review because replay was not safe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreDirtyBufferFailure {
    pub journal_entry_id: String,
    pub journal_id: String,
    pub object_ref: String,
    pub failure_kind: RestoreDirtyBufferFailureKind,
    pub detail: String,
}

/// Outcome produced by [`RestoreProposal::execute`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreOutcome {
    pub record_kind: String,
    pub restore_class: RestoreClass,
    pub pane_outcomes: Vec<RestorePaneOutcome>,
    pub dirty_buffer_replays: Vec<RestoreDirtyBufferReplay>,
    pub dirty_buffer_failures: Vec<RestoreDirtyBufferFailure>,
    pub manual_repair_required: bool,
    pub auto_rerun_forbidden: bool,
    pub summary_line: String,
}

impl RestoreOutcome {
    /// True when execution found no panes, dirty buffers, or retained failures.
    pub fn is_empty(&self) -> bool {
        self.pane_outcomes.is_empty()
            && self.dirty_buffer_replays.is_empty()
            && self.dirty_buffer_failures.is_empty()
    }

    /// True when all replayable work completed without typed failures.
    pub fn succeeded_without_failures(&self) -> bool {
        self.dirty_buffer_failures.is_empty()
    }

    /// Number of side-effectful panes restored only as inactive surfaces.
    pub fn blocked_side_effectful_count(&self) -> usize {
        self.pane_outcomes
            .iter()
            .filter(|pane| pane.execution_kind == RestorePaneExecutionKind::BlockedSideEffectful)
            .count()
    }
}

impl RestoreProposal {
    /// Builds the canonical pre-rehydration proposal from persisted state.
    ///
    /// Reads at most one workspace-authority checkpoint, one topology
    /// snapshot, and the live crash-journal entries. Missing or corrupt
    /// records downgrade the `restore_class` and record the trigger; the
    /// builder never invents counts or plans for things it cannot read.
    pub fn build(
        session_store: &SessionRestoreStore,
        crash_store: &CrashJournalStore,
        prior_run_abnormal: bool,
    ) -> Result<Self, SessionRestoreError> {
        let mut downgrade_triggers: Vec<DowngradeTriggerClass> = Vec::new();
        let mut artifact_refs = RestoreProposalArtifactRefs::default();
        let mut pane_plans: Vec<RestoreProposalPanePlan> = Vec::new();
        let mut counts = RestoreProposalCounts::default();

        let latest_refs = session_store.latest_refs()?;
        let mut checkpoint: Option<WorkspaceAuthorityCheckpointRecord> = None;
        let mut snapshot: Option<WindowTopologySnapshotRecord> = None;

        if let Some(latest) = latest_refs.as_ref() {
            artifact_refs.checkpoint_id = Some(latest.checkpoint_id.clone());
            artifact_refs.snapshot_id = Some(latest.snapshot_id.clone());

            match session_store.load_checkpoint(&latest.checkpoint_id) {
                Ok(record) => {
                    artifact_refs.workspace_authority_ref =
                        Some(record.workspace_authority_ref.clone());
                    counts.evidence_packets = record.evidence_bundle_refs.len();
                    counts.recovery_packets = record.recovery_journal_refs.len();
                    checkpoint = Some(record);
                }
                Err(_) => {
                    downgrade_triggers.push(DowngradeTriggerClass::ManualRepairRequired);
                }
            }

            match session_store.load_window_topology_snapshot(&latest.snapshot_id) {
                Ok(record) => {
                    artifact_refs.window_id = Some(record.window_id.clone());
                    counts.windows = 1 + record.sibling_window_refs.len();
                    counts.tab_groups = record.tab_group_topology.len();
                    counts.tabs = record
                        .tab_group_topology
                        .iter()
                        .map(|group| group.ordered_tab_ids.len())
                        .sum();
                    counts.terminals = record
                        .stable_pane_id_inventory
                        .iter()
                        .filter(|pane| is_terminal_like(pane.surface_role, pane.surface_class))
                        .count();
                    counts.transient_tasks = record
                        .stable_pane_id_inventory
                        .iter()
                        .filter(|pane| is_transient_task(pane.surface_role, pane.surface_class))
                        .count();
                    pane_plans = record
                        .stable_pane_id_inventory
                        .iter()
                        .map(materialize_pane_plan)
                        .collect();
                    snapshot = Some(record);
                }
                Err(_) => {
                    downgrade_triggers.push(DowngradeTriggerClass::ManualRepairRequired);
                }
            }
        }

        let crash_entries = crash_store
            .load_entries()
            .map_err(|err| SessionRestoreError::MissingRecord(err.to_string()))?;
        let dirty_buffer_entries = collect_dirty_buffer_entries(&crash_entries);
        counts.dirty_buffer_journals = dirty_buffer_entries.len();

        if dirty_buffer_entries
            .iter()
            .any(|entry| !matches!(entry.frame_integrity, FrameIntegrityState::Verified))
            && !downgrade_triggers.contains(&DowngradeTriggerClass::ManualRepairRequired)
        {
            downgrade_triggers.push(DowngradeTriggerClass::ManualRepairRequired);
        }

        let restore_class = classify_restore_class(
            checkpoint.as_ref(),
            snapshot.as_ref(),
            &dirty_buffer_entries,
            !downgrade_triggers.is_empty(),
        );

        let notes = build_notes(
            prior_run_abnormal,
            &restore_class,
            &counts,
            !downgrade_triggers.is_empty(),
        );

        Ok(Self {
            record_kind: "restore_proposal_record".to_string(),
            restore_proposal_schema_version: 1,
            prior_run_abnormal,
            restore_class,
            counts,
            artifact_refs,
            pane_plans,
            dirty_buffer_entries,
            downgrade_triggers,
            auto_rerun_forbidden: true,
            notes: Some(notes),
        })
    }

    /// True when the proposal has nothing meaningful to restore.
    pub fn is_empty(&self) -> bool {
        self.counts.windows == 0
            && self.counts.tab_groups == 0
            && self.counts.tabs == 0
            && self.counts.dirty_buffer_journals == 0
            && self.counts.transient_tasks == 0
            && self.counts.terminals == 0
            && self.counts.evidence_packets == 0
            && self.counts.recovery_packets == 0
    }

    /// True when the proposal carries dirty drafts that require user review.
    pub fn has_dirty_buffers(&self) -> bool {
        self.counts.dirty_buffer_journals > 0
    }

    /// One-line summary suitable for status surfaces and command-runtime notes.
    pub fn summary_line(&self) -> String {
        format!(
            "restore_class={class}; windows={windows}; tab_groups={groups}; tabs={tabs}; \
             dirty_buffers={drafts}; transient_tasks={tasks}; terminals={terminals}; \
             evidence_packets={evidence}; recovery_packets={recovery}",
            class = restore_class_label(self.restore_class),
            windows = self.counts.windows,
            groups = self.counts.tab_groups,
            tabs = self.counts.tabs,
            drafts = self.counts.dirty_buffer_journals,
            tasks = self.counts.transient_tasks,
            terminals = self.counts.terminals,
            evidence = self.counts.evidence_packets,
            recovery = self.counts.recovery_packets,
        )
    }

    /// Executes this proposal against the provided recovery runtime.
    ///
    /// Execution reopens only non-mutating pane skeletons and resolves verified
    /// dirty-buffer journal bodies into replay requests. It never saves the
    /// recovered bytes and never reruns side-effectful surfaces such as
    /// terminals, debuggers, notebooks, or AI panels.
    pub fn execute(self, runtime: &mut RestoreRuntime<'_>) -> RestoreOutcome {
        let summary_line = self.summary_line();
        let mut outcome = RestoreOutcome {
            record_kind: "restore_outcome_record".to_string(),
            restore_class: self.restore_class,
            pane_outcomes: Vec::new(),
            dirty_buffer_replays: Vec::new(),
            dirty_buffer_failures: Vec::new(),
            manual_repair_required: self
                .downgrade_triggers
                .contains(&DowngradeTriggerClass::ManualRepairRequired),
            auto_rerun_forbidden: true,
            summary_line,
        };

        if self.is_empty() {
            return outcome;
        }

        if let Some(snapshot_id) = self.artifact_refs.snapshot_id.as_deref() {
            if runtime
                .session_store
                .load_pane_tree_body(snapshot_id)
                .is_err()
            {
                outcome.manual_repair_required = true;
            }
        }

        let evidence_only_restore = matches!(
            self.restore_class,
            RestoreClass::EvidenceOnly | RestoreClass::NoRestore
        );

        outcome.pane_outcomes = self
            .pane_plans
            .into_iter()
            .map(|plan| execute_pane_plan(plan, evidence_only_restore))
            .collect();

        if self.dirty_buffer_entries.is_empty() {
            return outcome;
        }

        let crash_entries = match runtime.crash_store.load_entries() {
            Ok(entries) => entries,
            Err(err) => {
                outcome.manual_repair_required = true;
                for entry in self.dirty_buffer_entries {
                    outcome.dirty_buffer_failures.push(dirty_failure_from_entry(
                        &entry,
                        RestoreDirtyBufferFailureKind::StoreUnavailable,
                        err.to_string(),
                    ));
                }
                return outcome;
            }
        };
        let crash_entries_by_id: HashMap<&str, &AutosaveJournalEntryRecord> = crash_entries
            .iter()
            .map(|entry| (entry.journal_entry_id.as_str(), entry))
            .collect();

        for entry in self.dirty_buffer_entries {
            if evidence_only_restore
                || !matches!(entry.frame_integrity, FrameIntegrityState::Verified)
            {
                outcome.manual_repair_required = true;
                outcome.dirty_buffer_failures.push(dirty_failure_from_entry(
                    &entry,
                    RestoreDirtyBufferFailureKind::ManualRepairRequired,
                    "dirty-buffer frame requires manual repair".to_string(),
                ));
                continue;
            }

            let Some(record) = crash_entries_by_id.get(entry.journal_entry_id.as_str()) else {
                outcome.dirty_buffer_failures.push(dirty_failure_from_entry(
                    &entry,
                    RestoreDirtyBufferFailureKind::MissingJournalEntry,
                    "journal entry referenced by proposal is missing".to_string(),
                ));
                continue;
            };

            if !record.capture_descriptor.body_available {
                outcome.dirty_buffer_failures.push(dirty_failure_from_entry(
                    &entry,
                    RestoreDirtyBufferFailureKind::MissingJournalBody,
                    "journal entry body is not available".to_string(),
                ));
                continue;
            }

            let Some(body_object_ref) = record.capture_descriptor.body_object_refs.first() else {
                outcome.dirty_buffer_failures.push(dirty_failure_from_entry(
                    &entry,
                    RestoreDirtyBufferFailureKind::MissingJournalBody,
                    "journal entry has no body object reference".to_string(),
                ));
                continue;
            };

            let bytes = match runtime.crash_store.read_body_object(body_object_ref) {
                Ok(bytes) => bytes,
                Err(err) => {
                    outcome.dirty_buffer_failures.push(dirty_failure_from_entry(
                        &entry,
                        RestoreDirtyBufferFailureKind::MissingJournalBody,
                        err.to_string(),
                    ));
                    continue;
                }
            };

            if matches!(
                record.integrity.checksum_algorithm,
                ChecksumAlgorithm::Blake3
            ) {
                let actual = blake3::hash(&bytes).to_hex().to_string();
                if actual != record.integrity.checksum_ref {
                    outcome.dirty_buffer_failures.push(dirty_failure_from_entry(
                        &entry,
                        RestoreDirtyBufferFailureKind::ChecksumMismatch,
                        "journal body checksum mismatch".to_string(),
                    ));
                    continue;
                }
            }

            outcome.dirty_buffer_replays.push(RestoreDirtyBufferReplay {
                journal_entry_id: entry.journal_entry_id,
                journal_id: entry.journal_id,
                object_ref: entry.object_ref,
                presentation_hint: entry.presentation_hint,
                body_object_ref: body_object_ref.clone(),
                bytes,
                replay_posture: entry.replay_posture,
                frame_integrity: entry.frame_integrity,
                recommended_choice: entry.recommended_choice,
            });
        }

        outcome
    }
}

fn execute_pane_plan(
    plan: RestoreProposalPanePlan,
    evidence_only_restore: bool,
) -> RestorePaneOutcome {
    let execution_kind = if matches!(
        plan.plan_kind,
        RestoreProposalPlanKind::BlockedSideEffectful
    ) {
        RestorePaneExecutionKind::BlockedSideEffectful
    } else if evidence_only_restore
        || matches!(plan.plan_kind, RestoreProposalPlanKind::EvidenceOnly)
    {
        RestorePaneExecutionKind::EvidenceOnly
    } else {
        match plan.plan_kind {
            RestoreProposalPlanKind::LiveSkeleton => RestorePaneExecutionKind::Reopened,
            RestoreProposalPlanKind::PlaceholderOnly => RestorePaneExecutionKind::Placeholder,
            RestoreProposalPlanKind::EvidenceOnly => RestorePaneExecutionKind::EvidenceOnly,
            RestoreProposalPlanKind::BlockedSideEffectful => {
                RestorePaneExecutionKind::BlockedSideEffectful
            }
        }
    };
    let note = match execution_kind {
        RestorePaneExecutionKind::Reopened => {
            "pane reopened as lightweight restore skeleton".to_string()
        }
        RestorePaneExecutionKind::Placeholder => {
            "pane restored as placeholder; no automatic hydration".to_string()
        }
        RestorePaneExecutionKind::EvidenceOnly => {
            "pane retained as evidence only; not reopened".to_string()
        }
        RestorePaneExecutionKind::BlockedSideEffectful => {
            "side-effectful surface restored inactive; no commands rerun".to_string()
        }
    };
    RestorePaneOutcome {
        pane_id: plan.pane_id,
        surface_role: plan.surface_role,
        surface_class: plan.surface_class,
        execution_kind,
        title_hint: plan.title_hint,
        restore_metadata: plan.restore_metadata,
        note,
    }
}

fn dirty_failure_from_entry(
    entry: &RestoreProposalDirtyBufferEntry,
    failure_kind: RestoreDirtyBufferFailureKind,
    detail: String,
) -> RestoreDirtyBufferFailure {
    RestoreDirtyBufferFailure {
        journal_entry_id: entry.journal_entry_id.clone(),
        journal_id: entry.journal_id.clone(),
        object_ref: entry.object_ref.clone(),
        failure_kind,
        detail,
    }
}

fn collect_dirty_buffer_entries(
    crash_entries: &[AutosaveJournalEntryRecord],
) -> Vec<RestoreProposalDirtyBufferEntry> {
    let mut latest_per_object: HashMap<String, &AutosaveJournalEntryRecord> = HashMap::new();
    for entry in crash_entries {
        let key = format!("{}|{}", entry.journal_id, entry.object_identity.object_ref);
        latest_per_object
            .entry(key)
            .and_modify(|current| {
                if entry.emitted_at > current.emitted_at {
                    *current = entry;
                }
            })
            .or_insert(entry);
    }
    let mut out: Vec<_> = latest_per_object
        .values()
        .map(|entry| RestoreProposalDirtyBufferEntry {
            journal_entry_id: entry.journal_entry_id.clone(),
            journal_id: entry.journal_id.clone(),
            object_ref: entry.object_identity.object_ref.clone(),
            presentation_hint: entry.object_identity.presentation_hint.clone(),
            replay_posture: entry.replay_posture.object_class_replay_posture,
            frame_integrity: entry.integrity.frame_integrity_state,
            recommended_choice: entry.replay_posture.recommended_choice_class,
        })
        .collect();
    out.sort_by(|a, b| a.journal_entry_id.cmp(&b.journal_entry_id));
    out
}

fn materialize_pane_plan(pane: &StablePaneInventoryEntry) -> RestoreProposalPanePlan {
    let plan_kind = classify_pane_plan(pane.surface_role, pane.surface_class);
    let note = match plan_kind {
        RestoreProposalPlanKind::LiveSkeleton => {
            "skeleton restored; user opens content explicitly".to_string()
        }
        RestoreProposalPlanKind::PlaceholderOnly => {
            "placeholder card; no auto-hydration".to_string()
        }
        RestoreProposalPlanKind::EvidenceOnly => {
            "retained as evidence only; not re-opened".to_string()
        }
        RestoreProposalPlanKind::BlockedSideEffectful => {
            "side-effectful surface; never auto-rerun".to_string()
        }
    };
    RestoreProposalPanePlan {
        pane_id: pane.pane_id.clone(),
        surface_role: pane.surface_role,
        surface_class: pane.surface_class,
        plan_kind,
        title_hint: pane.title_hint.clone(),
        restore_metadata: pane.restore_metadata.clone(),
        note,
    }
}

fn classify_pane_plan(role: SurfaceRole, class: SurfaceClass) -> RestoreProposalPlanKind {
    if is_terminal_like(role, class) || is_transient_task(role, class) {
        return RestoreProposalPlanKind::BlockedSideEffectful;
    }
    match role {
        SurfaceRole::Editor | SurfaceRole::Diff | SurfaceRole::Docs | SurfaceRole::Explorer => {
            RestoreProposalPlanKind::LiveSkeleton
        }
        SurfaceRole::Search
        | SurfaceRole::Problems
        | SurfaceRole::Scm
        | SurfaceRole::Test
        | SurfaceRole::Preview => RestoreProposalPlanKind::PlaceholderOnly,
        SurfaceRole::Placeholder | SurfaceRole::CustomExtension => {
            RestoreProposalPlanKind::PlaceholderOnly
        }
        SurfaceRole::Terminal
        | SurfaceRole::Debugger
        | SurfaceRole::Notebook
        | SurfaceRole::AiPanel => RestoreProposalPlanKind::BlockedSideEffectful,
    }
}

fn is_terminal_like(role: SurfaceRole, class: SurfaceClass) -> bool {
    matches!(role, SurfaceRole::Terminal) || matches!(class, SurfaceClass::TerminalView)
}

fn is_transient_task(role: SurfaceRole, class: SurfaceClass) -> bool {
    matches!(
        role,
        SurfaceRole::Debugger | SurfaceRole::Notebook | SurfaceRole::AiPanel | SurfaceRole::Test
    ) || matches!(
        class,
        SurfaceClass::DebugView
            | SurfaceClass::NotebookView
            | SurfaceClass::AiPanel
            | SurfaceClass::TestResults
    )
}

fn classify_restore_class(
    checkpoint: Option<&WorkspaceAuthorityCheckpointRecord>,
    snapshot: Option<&WindowTopologySnapshotRecord>,
    dirty_entries: &[RestoreProposalDirtyBufferEntry],
    downgraded: bool,
) -> RestoreClass {
    if downgraded
        && dirty_entries
            .iter()
            .all(|entry| !matches!(entry.frame_integrity, FrameIntegrityState::Verified))
        && !dirty_entries.is_empty()
    {
        return RestoreClass::EvidenceOnly;
    }

    let has_layout = snapshot.is_some();
    let has_dirty = !dirty_entries.is_empty();

    match (has_layout, has_dirty, checkpoint) {
        (true, true, _) => RestoreClass::RecoveredDrafts,
        (true, false, _) => RestoreClass::LayoutOnly,
        (false, true, _) => RestoreClass::RecoveredDrafts,
        (false, false, Some(_)) => RestoreClass::LayoutOnly,
        (false, false, None) => RestoreClass::NoRestore,
    }
}

fn restore_class_label(class: RestoreClass) -> &'static str {
    match class {
        RestoreClass::ExactRestore => "exact_restore",
        RestoreClass::CompatibleRestore => "compatible_restore",
        RestoreClass::LayoutOnly => "layout_only",
        RestoreClass::RecoveredDrafts => "recovered_drafts",
        RestoreClass::EvidenceOnly => "evidence_only",
        RestoreClass::NoRestore => "no_restore",
    }
}

fn build_notes(
    prior_run_abnormal: bool,
    class: &RestoreClass,
    counts: &RestoreProposalCounts,
    downgraded: bool,
) -> String {
    let header = if prior_run_abnormal {
        "prior run terminated abnormally"
    } else {
        "clean prior run"
    };
    let suffix = if downgraded {
        " (downgrade triggers present; review before restore)"
    } else {
        ""
    };
    format!(
        "{header}; class={class}; windows={windows}, tabs={tabs}, drafts={drafts}{suffix}",
        class = restore_class_label(*class),
        windows = counts.windows,
        tabs = counts.tabs,
        drafts = counts.dirty_buffer_journals,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::crash_journal::{CrashJournalCaptureInput, ObjectClass};
    use crate::session_restore::records::{ProducerBuildStamp, WindowRole};
    use crate::session_restore::store::{
        SessionRestoreCaptureInput, TabGroupCaptureInput, TabItemCaptureInput,
    };

    fn producer() -> ProducerBuildStamp {
        ProducerBuildStamp {
            producer_name: "aureline-recovery-test".to_string(),
            producer_version: "0.0.0".to_string(),
            producer_channel: None,
            producer_platform_class: None,
            producer_instance_handle: None,
        }
    }

    fn capture_one_layout(store: &mut SessionRestoreStore, with_terminal: bool) {
        let mut tabs = vec![TabItemCaptureInput {
            tab_id: "tab-edit-router".to_string(),
            tab_label: Some("router.ts".to_string()),
            pinned: false,
            dirty_badge_visible: true,
            surface_role: SurfaceRole::Editor,
            surface_class: SurfaceClass::TextEditor,
            restore_metadata: None,
        }];
        if with_terminal {
            tabs.push(TabItemCaptureInput {
                tab_id: "tab-terminal-shell".to_string(),
                tab_label: Some("zsh".to_string()),
                pinned: false,
                dirty_badge_visible: false,
                surface_role: SurfaceRole::Terminal,
                surface_class: SurfaceClass::TerminalView,
                restore_metadata: None,
            });
        }
        let input = SessionRestoreCaptureInput {
            workspace_ref: "ws-test".to_string(),
            producer_build: producer(),
            source_schema_version: "1".to_string(),
            trusted_root_refs: Vec::new(),
            active_workset_ids: Vec::new(),
            dirty_buffer_journal_identities: Vec::new(),
            recovery_journal_refs: vec!["rec-packet-001".to_string()],
            local_history_snapshot_refs: Vec::new(),
            evidence_bundle_refs: vec!["evidence-001".to_string()],
            excluded_live_authority_classes: Vec::new(),
            downgrade_triggers: Vec::new(),
            window_id: "win-primary".to_string(),
            window_role: WindowRole::Primary,
            topology_family_ref: None,
            sibling_window_refs: Vec::new(),
            tab_groups: vec![TabGroupCaptureInput {
                group_id: "tg-main".to_string(),
                ordered_tabs: tabs,
                active_tab_id: Some("tab-edit-router".to_string()),
            }],
            emitted_at: "mono:test:00001".to_string(),
            notes: None,
        };
        store.capture(input).expect("capture");
    }

    fn capture_one_dirty_buffer(store: &mut CrashJournalStore) {
        let input = CrashJournalCaptureInput {
            journal_id: "journal:ws-test".to_string(),
            workspace_ref: "ws-test".to_string(),
            logical_document_id: "ld:router".to_string(),
            object_ref: "buffer:router".to_string(),
            object_class: ObjectClass::CanonicalFile,
            presentation_hint: Some("router.ts".to_string()),
            emitted_at: "mono:test:00002".to_string(),
            bytes: b"hello world".to_vec(),
        };
        store
            .capture_minimal_full_snapshot(input)
            .expect("capture journal");
    }

    #[test]
    fn proposal_with_no_state_is_empty_no_restore() {
        let dir = tempfile::tempdir().expect("tempdir");
        let session_store = SessionRestoreStore::new(dir.path());
        let crash_store = CrashJournalStore::new(dir.path());

        let proposal = RestoreProposal::build(&session_store, &crash_store, false).expect("build");
        assert!(proposal.is_empty());
        assert!(!proposal.has_dirty_buffers());
        assert_eq!(proposal.restore_class, RestoreClass::NoRestore);
        assert!(proposal.auto_rerun_forbidden);
    }

    #[test]
    fn layout_only_when_no_dirty_buffers() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut session_store = SessionRestoreStore::new(dir.path());
        let crash_store = CrashJournalStore::new(dir.path());

        capture_one_layout(&mut session_store, false);

        let proposal = RestoreProposal::build(&session_store, &crash_store, true).expect("build");
        assert_eq!(proposal.restore_class, RestoreClass::LayoutOnly);
        assert_eq!(proposal.counts.windows, 1);
        assert_eq!(proposal.counts.tab_groups, 1);
        assert_eq!(proposal.counts.tabs, 1);
        assert_eq!(proposal.counts.dirty_buffer_journals, 0);
        assert_eq!(proposal.counts.evidence_packets, 1);
        assert_eq!(proposal.counts.recovery_packets, 1);
        assert!(!proposal.is_empty());
        assert!(proposal.prior_run_abnormal);
        assert!(proposal.auto_rerun_forbidden);
    }

    #[test]
    fn recovered_drafts_when_dirty_buffers_present() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut session_store = SessionRestoreStore::new(dir.path());
        let mut crash_store = CrashJournalStore::new(dir.path());

        capture_one_layout(&mut session_store, false);
        capture_one_dirty_buffer(&mut crash_store);

        let proposal = RestoreProposal::build(&session_store, &crash_store, true).expect("build");
        assert_eq!(proposal.restore_class, RestoreClass::RecoveredDrafts);
        assert!(proposal.has_dirty_buffers());
        assert_eq!(proposal.counts.dirty_buffer_journals, 1);
        assert_eq!(proposal.dirty_buffer_entries.len(), 1);
    }

    #[test]
    fn terminals_classified_as_blocked_side_effectful() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut session_store = SessionRestoreStore::new(dir.path());
        let crash_store = CrashJournalStore::new(dir.path());

        capture_one_layout(&mut session_store, true);

        let proposal = RestoreProposal::build(&session_store, &crash_store, false).expect("build");
        assert_eq!(proposal.counts.terminals, 1);
        let terminal_plan = proposal
            .pane_plans
            .iter()
            .find(|plan| matches!(plan.surface_role, SurfaceRole::Terminal))
            .expect("terminal plan");
        assert_eq!(
            terminal_plan.plan_kind,
            RestoreProposalPlanKind::BlockedSideEffectful
        );
    }

    #[test]
    fn drafts_only_proposal_without_layout() {
        let dir = tempfile::tempdir().expect("tempdir");
        let session_store = SessionRestoreStore::new(dir.path());
        let mut crash_store = CrashJournalStore::new(dir.path());

        capture_one_dirty_buffer(&mut crash_store);

        let proposal = RestoreProposal::build(&session_store, &crash_store, true).expect("build");
        assert_eq!(proposal.restore_class, RestoreClass::RecoveredDrafts);
        assert_eq!(proposal.counts.windows, 0);
        assert_eq!(proposal.counts.dirty_buffer_journals, 1);
        assert!(proposal.has_dirty_buffers());
    }

    #[test]
    fn fixture_cases_round_trip_into_restore_proposal_record() {
        let fixtures_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .map(|p| {
                p.join("fixtures")
                    .join("recovery")
                    .join("session_restore_cases")
            })
            .expect("derive fixtures dir");

        let cases = [
            "no_restore_first_launch.json",
            "layout_only_clean_relaunch.json",
            "recovered_drafts_after_crash.json",
            "evidence_only_corrupt_snapshot.json",
        ];

        let mut covered = std::collections::HashSet::new();
        for case in cases {
            let path = fixtures_dir.join(case);
            let bytes = std::fs::read(&path)
                .unwrap_or_else(|err| panic!("read fixture {}: {err}", path.display()));
            let proposal: RestoreProposal = serde_json::from_slice(&bytes)
                .unwrap_or_else(|err| panic!("parse fixture {}: {err}", path.display()));

            assert!(
                proposal.auto_rerun_forbidden,
                "fixture {case} must keep auto_rerun_forbidden=true"
            );
            assert_eq!(proposal.record_kind, "restore_proposal_record");
            assert_eq!(proposal.restore_proposal_schema_version, 1);

            for plan in &proposal.pane_plans {
                if matches!(
                    plan.surface_role,
                    SurfaceRole::Terminal
                        | SurfaceRole::Debugger
                        | SurfaceRole::Notebook
                        | SurfaceRole::AiPanel
                ) {
                    assert_eq!(
                        plan.plan_kind,
                        RestoreProposalPlanKind::BlockedSideEffectful,
                        "fixture {case} pane {} must block auto-rerun",
                        plan.pane_id
                    );
                }
            }

            covered.insert(proposal.restore_class);
        }

        for required in [
            RestoreClass::NoRestore,
            RestoreClass::LayoutOnly,
            RestoreClass::RecoveredDrafts,
            RestoreClass::EvidenceOnly,
        ] {
            assert!(
                covered.contains(&required),
                "fixtures must cover restore_class={:?}",
                required
            );
        }
    }
}
