//! Restore-prompt projection for the live shell.
//!
//! The restore prompt is the protected pre-rehydration surface shown after
//! abnormal termination, after a deliberate restore activation, or whenever
//! the shell needs to disclose what session restore would actually do. It is
//! the canonical truth source for honest resume and missing-target recovery
//! and it is the single place restore-related action grammar is admitted.
//!
//! Inputs are the canonical [`RestoreProposal`] from the recovery crate. The
//! prompt does not invent counts or downgrade reasons; it projects what the
//! proposal already knows.
//!
//! Two honesty invariants ride on every prompt:
//!
//! 1. **No silent rerun.** `auto_rerun_forbidden` is always `true`. Side-
//!    effectful surfaces (terminals, debuggers, notebook kernels, AI panels,
//!    remote sessions) are skeletoned, not replayed.
//! 2. **Missing dependencies are explicit.** Downgrade triggers from the
//!    proposal map to a `missing_dependency_count`; safe-mode and
//!    clear-journal paths are always offered when the prompt has anything
//!    to disclose.

use std::path::Path;

use serde::{Deserialize, Serialize};

use aureline_recovery::session_restore::proposal::{
    RestoreProposal, RestoreProposalCounts, RestoreProposalPlanKind,
};
use aureline_recovery::session_restore::records::{DowngradeTriggerClass, RestoreClass};

pub mod placeholders;
pub mod provenance;

/// Canonical command id used for the safe-mode path in restore prompts.
pub const RESTORE_SAFE_MODE_COMMAND_ID: &str = "cmd:workspace.enter_safe_mode";

/// Canonical command id used for the clear-journal path in restore prompts.
pub const RESTORE_CLEAR_JOURNAL_COMMAND_ID: &str = "cmd:workspace.clear_recovery_journal";

/// Canonical command id used for the open-clean path in restore prompts.
pub const RESTORE_OPEN_CLEAN_COMMAND_ID: &str = "cmd:workspace.open_clean";

/// Canonical command id used for the restore-now path in restore prompts.
pub const RESTORE_NOW_COMMAND_ID: &str = "cmd:workspace.restore_from_checkpoint";

/// Schema version exported with [`RestorePromptRecord`].
pub type RestorePromptSchemaVersion = u32;

/// Stable choice keys exported by the restore prompt.
///
/// The keys reuse the action grammar in
/// `docs/ux/crash_loop_and_restore_fidelity_contract.md` so prompt rendering
/// stays aligned across the live shell, diagnostics packets, and exported
/// fixtures.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestorePromptChoiceKey {
    /// Apply the proposal (skeletons only; no auto-rerun).
    RestoreNow,
    /// Skip the prompt for this launch; preserve all evidence.
    SkipOnce,
    /// Open a clean shell instead of rehydrating.
    OpenClean,
    /// Compare a recovered draft against the on-disk target.
    CompareToDisk,
    /// Inspect the recovery journal without applying anything.
    OpenJournal,
    /// Enter safe mode (extensions disabled, no auto-restore, no remote).
    SafeMode,
    /// Clear the recovery journal after explicit confirmation.
    ClearJournal,
    /// Open the local recovery logs.
    OpenLogs,
    /// Export the retained evidence packets.
    ExportEvidence,
}

impl RestorePromptChoiceKey {
    /// Stable string used in records, fixtures, and surface a11y exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoreNow => "restore_now",
            Self::SkipOnce => "skip_once",
            Self::OpenClean => "open_clean",
            Self::CompareToDisk => "compare_to_disk",
            Self::OpenJournal => "open_journal",
            Self::SafeMode => "safe_mode",
            Self::ClearJournal => "clear_journal",
            Self::OpenLogs => "open_logs",
            Self::ExportEvidence => "export_evidence",
        }
    }
}

/// Implication of a choice on the local journal and on durable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreJournalImplication {
    /// Skeleton restoration only; no journal cleared, no live attach.
    RestoreSkeletonOnly,
    /// No mutation for this launch; evidence preserved.
    NoChangeForThisLaunch,
    /// Evidence retained for later inspection or export.
    EvidenceRetained,
    /// Compare-only routes never write or discard.
    CompareOnly,
    /// Inspection-only routes do not apply or remove journal entries.
    InspectOnly,
    /// Safe-mode entry records the recovery checkpoint.
    SafeModeEntered,
    /// Journal cleared after the user confirms the destructive action.
    JournalClearedAfterConfirmation,
    /// Export routes copy evidence without touching live state.
    ExportOnly,
}

impl RestoreJournalImplication {
    /// Stable string used in records, fixtures, and a11y exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RestoreSkeletonOnly => "restore_skeleton_only",
            Self::NoChangeForThisLaunch => "no_change_for_this_launch",
            Self::EvidenceRetained => "evidence_retained",
            Self::CompareOnly => "compare_only",
            Self::InspectOnly => "inspect_only",
            Self::SafeModeEntered => "safe_mode_entered",
            Self::JournalClearedAfterConfirmation => "journal_cleared_after_confirmation",
            Self::ExportOnly => "export_only",
        }
    }
}

/// One offered choice exported by the restore prompt.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorePromptChoice {
    pub choice_key: RestorePromptChoiceKey,
    pub enabled: bool,
    pub forbidden_reason: String,
    pub requires_confirmation: bool,
    pub journal_implication: RestoreJournalImplication,
}

/// Counts surfaced by the restore prompt before any pane is hydrated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RestorePromptCounts {
    pub windows: usize,
    pub tab_groups: usize,
    pub tabs: usize,
    pub dirty_buffer_count: usize,
    pub missing_dependency_count: usize,
    pub evidence_packet_count: usize,
    pub recovery_packet_count: usize,
    pub terminal_count: usize,
    pub transient_task_count: usize,
}

impl RestorePromptCounts {
    fn from_proposal_counts(
        counts: &RestoreProposalCounts,
        missing_dependency_count: usize,
    ) -> Self {
        Self {
            windows: counts.windows,
            tab_groups: counts.tab_groups,
            tabs: counts.tabs,
            dirty_buffer_count: counts.dirty_buffer_journals,
            missing_dependency_count,
            evidence_packet_count: counts.evidence_packets,
            recovery_packet_count: counts.recovery_packets,
            terminal_count: counts.terminals,
            transient_task_count: counts.transient_tasks,
        }
    }
}

/// Canonical pre-rehydration restore prompt projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestorePromptRecord {
    pub record_kind: String,
    pub restore_prompt_schema_version: RestorePromptSchemaVersion,
    pub prior_run_abnormal: bool,
    pub auto_rerun_forbidden: bool,
    pub restore_class: String,
    pub counts: RestorePromptCounts,
    pub safe_mode_command_id: String,
    pub clear_journal_command_id: String,
    pub open_clean_command_id: String,
    pub restore_now_command_id: String,
    pub summary_line: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    pub downgrade_triggers: Vec<String>,
    pub choices: Vec<RestorePromptChoice>,
}

impl RestorePromptRecord {
    /// True when the prompt has nothing actionable to disclose.
    pub fn is_empty(&self) -> bool {
        self.counts.windows == 0
            && self.counts.tab_groups == 0
            && self.counts.tabs == 0
            && self.counts.dirty_buffer_count == 0
            && self.counts.evidence_packet_count == 0
            && self.counts.recovery_packet_count == 0
            && self.counts.terminal_count == 0
            && self.counts.transient_task_count == 0
            && self.counts.missing_dependency_count == 0
    }
}

/// Materializes a restore prompt projection from a [`RestoreProposal`].
///
/// The projection inherits proposal counts, restore class, and notes; it adds
/// the missing-dependency count derived from `downgrade_triggers`, and it
/// always includes the safe-mode and clear-journal command ids so the live
/// surface, diagnostics packet, and exported fixture share one truth.
pub fn materialize_restore_prompt(proposal: &RestoreProposal) -> RestorePromptRecord {
    let missing_dependency_count = proposal
        .downgrade_triggers
        .iter()
        .filter(|trigger| is_missing_dependency_trigger(**trigger))
        .count();

    let counts =
        RestorePromptCounts::from_proposal_counts(&proposal.counts, missing_dependency_count);
    let restore_class = restore_class_label(proposal.restore_class).to_string();
    let summary_line = proposal.summary_line();
    let downgrade_triggers = proposal
        .downgrade_triggers
        .iter()
        .map(|trigger| downgrade_trigger_label(*trigger).to_string())
        .collect();

    let has_layout_or_drafts = !proposal.is_empty();
    let has_dirty_buffers = proposal.has_dirty_buffers();
    let has_journal =
        proposal.counts.dirty_buffer_journals > 0 || proposal.counts.recovery_packets > 0;
    let has_evidence = proposal.counts.evidence_packets > 0
        || matches!(proposal.restore_class, RestoreClass::EvidenceOnly);
    let has_compare_target = proposal
        .dirty_buffer_entries
        .iter()
        .any(|entry| entry.presentation_hint.is_some());
    let has_terminals_or_tasks = proposal.pane_plans.iter().any(|plan| {
        matches!(
            plan.plan_kind,
            RestoreProposalPlanKind::BlockedSideEffectful
        )
    });

    let choices = vec![
        RestorePromptChoice {
            choice_key: RestorePromptChoiceKey::RestoreNow,
            enabled: has_layout_or_drafts,
            forbidden_reason: if has_layout_or_drafts {
                "none".to_string()
            } else {
                "no_restorable_state".to_string()
            },
            requires_confirmation: false,
            journal_implication: RestoreJournalImplication::RestoreSkeletonOnly,
        },
        RestorePromptChoice {
            choice_key: RestorePromptChoiceKey::SkipOnce,
            enabled: has_layout_or_drafts,
            forbidden_reason: if has_layout_or_drafts {
                "none".to_string()
            } else {
                "no_prompt_present".to_string()
            },
            requires_confirmation: false,
            journal_implication: RestoreJournalImplication::NoChangeForThisLaunch,
        },
        RestorePromptChoice {
            choice_key: RestorePromptChoiceKey::OpenClean,
            enabled: true,
            forbidden_reason: "none".to_string(),
            requires_confirmation: false,
            journal_implication: RestoreJournalImplication::EvidenceRetained,
        },
        RestorePromptChoice {
            choice_key: RestorePromptChoiceKey::CompareToDisk,
            enabled: has_compare_target,
            forbidden_reason: if has_compare_target {
                "none".to_string()
            } else {
                "no_compare_target".to_string()
            },
            requires_confirmation: false,
            journal_implication: RestoreJournalImplication::CompareOnly,
        },
        RestorePromptChoice {
            choice_key: RestorePromptChoiceKey::OpenJournal,
            enabled: has_journal,
            forbidden_reason: if has_journal {
                "none".to_string()
            } else {
                "no_journal_entries".to_string()
            },
            requires_confirmation: false,
            journal_implication: RestoreJournalImplication::InspectOnly,
        },
        RestorePromptChoice {
            choice_key: RestorePromptChoiceKey::SafeMode,
            enabled: true,
            forbidden_reason: "none".to_string(),
            requires_confirmation: false,
            journal_implication: RestoreJournalImplication::SafeModeEntered,
        },
        RestorePromptChoice {
            choice_key: RestorePromptChoiceKey::ClearJournal,
            enabled: has_journal,
            forbidden_reason: if has_journal {
                "none".to_string()
            } else {
                "no_journal_entries".to_string()
            },
            requires_confirmation: true,
            journal_implication: RestoreJournalImplication::JournalClearedAfterConfirmation,
        },
        RestorePromptChoice {
            choice_key: RestorePromptChoiceKey::OpenLogs,
            enabled: true,
            forbidden_reason: "none".to_string(),
            requires_confirmation: false,
            journal_implication: RestoreJournalImplication::InspectOnly,
        },
        RestorePromptChoice {
            choice_key: RestorePromptChoiceKey::ExportEvidence,
            enabled: has_evidence,
            forbidden_reason: if has_evidence {
                "none".to_string()
            } else {
                "no_evidence_packets".to_string()
            },
            requires_confirmation: false,
            journal_implication: RestoreJournalImplication::ExportOnly,
        },
    ];

    let _ = has_dirty_buffers;
    let _ = has_terminals_or_tasks;

    RestorePromptRecord {
        record_kind: "restore_prompt_record".to_string(),
        restore_prompt_schema_version: 1,
        prior_run_abnormal: proposal.prior_run_abnormal,
        auto_rerun_forbidden: proposal.auto_rerun_forbidden,
        restore_class,
        counts,
        safe_mode_command_id: RESTORE_SAFE_MODE_COMMAND_ID.to_string(),
        clear_journal_command_id: RESTORE_CLEAR_JOURNAL_COMMAND_ID.to_string(),
        open_clean_command_id: RESTORE_OPEN_CLEAN_COMMAND_ID.to_string(),
        restore_now_command_id: RESTORE_NOW_COMMAND_ID.to_string(),
        summary_line,
        notes: proposal.notes.clone(),
        downgrade_triggers,
        choices,
    }
}

/// Writes a restore-prompt record to `<recovery_root>/restore_prompt_latest.json`.
pub fn write_restore_prompt_log(
    recovery_root: &Path,
    record: &RestorePromptRecord,
) -> Result<(), String> {
    std::fs::create_dir_all(recovery_root)
        .map_err(|err| format!("create recovery root failed: {err}"))?;
    let path = recovery_root.join("restore_prompt_latest.json");
    let json = serde_json::to_string_pretty(record)
        .map_err(|err| format!("serialize restore prompt failed: {err}"))?;
    std::fs::write(&path, json).map_err(|err| format!("write {} failed: {err}", path.display()))?;
    Ok(())
}

/// Renders the restore prompt as a single-line status string.
pub fn restore_prompt_status_line(record: &RestorePromptRecord) -> String {
    format!(
        "restore_prompt: class={class}; windows={windows}; tabs={tabs}; \
         dirty_buffers={drafts}; missing_deps={missing}; safe_mode={safe}; \
         clear_journal={clear}",
        class = record.restore_class,
        windows = record.counts.windows,
        tabs = record.counts.tabs,
        drafts = record.counts.dirty_buffer_count,
        missing = record.counts.missing_dependency_count,
        safe = record.safe_mode_command_id,
        clear = record.clear_journal_command_id,
    )
}

const fn restore_class_label(class: RestoreClass) -> &'static str {
    match class {
        RestoreClass::ExactRestore => "exact_restore",
        RestoreClass::CompatibleRestore => "compatible_restore",
        RestoreClass::LayoutOnly => "layout_only",
        RestoreClass::RecoveredDrafts => "recovered_drafts",
        RestoreClass::EvidenceOnly => "evidence_only",
        RestoreClass::NoRestore => "no_restore",
    }
}

const fn downgrade_trigger_label(trigger: DowngradeTriggerClass) -> &'static str {
    match trigger {
        DowngradeTriggerClass::SchemaTranslationRequired => "schema_translation_required",
        DowngradeTriggerClass::SchemaMeaningChanged => "schema_meaning_changed",
        DowngradeTriggerClass::MissingExtensionDependency => "missing_extension_dependency",
        DowngradeTriggerClass::MissingRemoteSession => "missing_remote_session",
        DowngradeTriggerClass::MissingRemoteAuthority => "missing_remote_authority",
        DowngradeTriggerClass::UnsupportedDisplayTopology => "unsupported_display_topology",
        DowngradeTriggerClass::ExcludedSecretMaterial => "excluded_secret_material",
        DowngradeTriggerClass::ExcludedLiveHandle => "excluded_live_handle",
        DowngradeTriggerClass::WorkspaceManifestConflict => "workspace_manifest_conflict",
        DowngradeTriggerClass::PolicyNarrowing => "policy_narrowing",
        DowngradeTriggerClass::ManualRepairRequired => "manual_repair_required",
        DowngradeTriggerClass::ProducerSchemaDowngradeRefused => {
            "producer_schema_downgrade_refused"
        }
    }
}

const fn is_missing_dependency_trigger(trigger: DowngradeTriggerClass) -> bool {
    matches!(
        trigger,
        DowngradeTriggerClass::MissingExtensionDependency
            | DowngradeTriggerClass::MissingRemoteSession
            | DowngradeTriggerClass::MissingRemoteAuthority
            | DowngradeTriggerClass::UnsupportedDisplayTopology
            | DowngradeTriggerClass::WorkspaceManifestConflict
            | DowngradeTriggerClass::ManualRepairRequired
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    use aureline_recovery::crash_journal::{
        CrashJournalCaptureInput, CrashJournalStore, ObjectClass,
    };
    use aureline_recovery::session_restore::records::{
        ProducerBuildStamp, SurfaceClass, SurfaceRole, WindowRole,
    };
    use aureline_recovery::session_restore::{
        SessionRestoreCaptureInput, SessionRestoreStore, TabGroupCaptureInput, TabItemCaptureInput,
    };

    fn producer() -> ProducerBuildStamp {
        ProducerBuildStamp {
            producer_name: "aureline-shell-restore-test".to_string(),
            producer_version: "0.0.0".to_string(),
            producer_channel: None,
            producer_platform_class: None,
            producer_instance_handle: None,
        }
    }

    fn capture_one_layout(store: &mut SessionRestoreStore) {
        let input = SessionRestoreCaptureInput {
            workspace_ref: "ws-restore-prompt".to_string(),
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
                ordered_tabs: vec![TabItemCaptureInput {
                    tab_id: "tab-edit-router".to_string(),
                    tab_label: Some("router.ts".to_string()),
                    pinned: false,
                    dirty_badge_visible: true,
                    surface_role: SurfaceRole::Editor,
                    surface_class: SurfaceClass::TextEditor,
                    restore_metadata: None,
                }],
                active_tab_id: Some("tab-edit-router".to_string()),
            }],
            emitted_at: "mono:test:00001".to_string(),
            notes: None,
        };
        store.capture(input).expect("capture");
    }

    fn capture_one_dirty_buffer(store: &mut CrashJournalStore) {
        let input = CrashJournalCaptureInput {
            journal_id: "journal:ws-restore-prompt".to_string(),
            workspace_ref: "ws-restore-prompt".to_string(),
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
    fn empty_proposal_yields_honest_no_restore_prompt() {
        let dir = tempfile::tempdir().expect("tempdir");
        let session_store = SessionRestoreStore::new(dir.path());
        let crash_store = CrashJournalStore::new(dir.path());
        let proposal = RestoreProposal::build(&session_store, &crash_store, false).expect("build");

        let prompt = materialize_restore_prompt(&proposal);
        assert!(prompt.is_empty());
        assert!(prompt.auto_rerun_forbidden);
        assert_eq!(prompt.restore_class, "no_restore");
        assert_eq!(prompt.safe_mode_command_id, RESTORE_SAFE_MODE_COMMAND_ID);
        assert_eq!(
            prompt.clear_journal_command_id,
            RESTORE_CLEAR_JOURNAL_COMMAND_ID
        );

        // Restore-now must be disabled with a typed reason when nothing to do.
        let restore_now = prompt
            .choices
            .iter()
            .find(|choice| choice.choice_key == RestorePromptChoiceKey::RestoreNow)
            .expect("restore_now offered");
        assert!(!restore_now.enabled);
        assert_eq!(restore_now.forbidden_reason, "no_restorable_state");

        // Open clean and safe mode must always be reachable.
        let open_clean = prompt
            .choices
            .iter()
            .find(|choice| choice.choice_key == RestorePromptChoiceKey::OpenClean)
            .expect("open_clean offered");
        assert!(open_clean.enabled);
        let safe_mode = prompt
            .choices
            .iter()
            .find(|choice| choice.choice_key == RestorePromptChoiceKey::SafeMode)
            .expect("safe_mode offered");
        assert!(safe_mode.enabled);
    }

    #[test]
    fn recovered_drafts_proposal_lights_compare_journal_safe_clear() {
        let dir = tempfile::tempdir().expect("tempdir");
        let mut session_store = SessionRestoreStore::new(dir.path());
        let mut crash_store = CrashJournalStore::new(dir.path());

        capture_one_layout(&mut session_store);
        capture_one_dirty_buffer(&mut crash_store);

        let proposal = RestoreProposal::build(&session_store, &crash_store, true).expect("build");
        let prompt = materialize_restore_prompt(&proposal);

        assert!(!prompt.is_empty());
        assert!(prompt.auto_rerun_forbidden);
        assert_eq!(prompt.restore_class, "recovered_drafts");
        assert_eq!(prompt.counts.dirty_buffer_count, 1);
        assert_eq!(prompt.counts.windows, 1);

        for required in [
            RestorePromptChoiceKey::RestoreNow,
            RestorePromptChoiceKey::SkipOnce,
            RestorePromptChoiceKey::CompareToDisk,
            RestorePromptChoiceKey::OpenJournal,
            RestorePromptChoiceKey::ClearJournal,
        ] {
            let choice = prompt
                .choices
                .iter()
                .find(|c| c.choice_key == required)
                .unwrap_or_else(|| panic!("choice {} missing", required.as_str()));
            assert!(choice.enabled, "{} must be enabled", required.as_str());
        }

        // Clear-journal must require explicit confirmation.
        let clear_journal = prompt
            .choices
            .iter()
            .find(|c| c.choice_key == RestorePromptChoiceKey::ClearJournal)
            .expect("clear_journal offered");
        assert!(clear_journal.requires_confirmation);

        let line = restore_prompt_status_line(&prompt);
        assert!(line.contains("class=recovered_drafts"));
        assert!(line.contains("dirty_buffers=1"));
        assert!(line.contains(RESTORE_SAFE_MODE_COMMAND_ID));
        assert!(line.contains(RESTORE_CLEAR_JOURNAL_COMMAND_ID));
    }

    #[test]
    fn fixture_round_trips_restore_prompt_record() {
        let fixture_path = Path::new(env!("CARGO_MANIFEST_DIR")).join(
            "../../fixtures/ux/restore_and_deeplink_cases/restore_prompt_recovered_drafts.json",
        );
        let payload = std::fs::read_to_string(&fixture_path).expect("fixture must read");
        let record: RestorePromptRecord =
            serde_json::from_str(&payload).expect("fixture must parse");

        assert_eq!(record.record_kind, "restore_prompt_record");
        assert!(record.auto_rerun_forbidden);
        assert_eq!(record.restore_class, "recovered_drafts");
        assert!(record
            .choices
            .iter()
            .any(|c| matches!(c.choice_key, RestorePromptChoiceKey::SafeMode)));
        assert!(record
            .choices
            .iter()
            .any(|c| matches!(c.choice_key, RestorePromptChoiceKey::ClearJournal)));
    }
}
