//! Save-participant risk review records.
//!
//! The staged save coordinator uses these records to decide whether a formatter,
//! organize-imports pass, code action, or AI apply participant can run on the
//! hot save path. The record intentionally carries only metadata: participant
//! ids, effect counts, disclosure classes, checkpoint posture, and support-safe
//! summaries.

use serde::{Deserialize, Serialize};

use super::source_fidelity::SourceFidelityAdjustment;

/// Schema version for save-participant risk records.
pub const SAVE_PARTICIPANT_RISK_SCHEMA_VERSION: u32 = 1;

/// Schema reference for save-participant risk records.
pub const SAVE_PARTICIPANT_RISK_SCHEMA_REF: &str =
    "schemas/editor/save_participant_risk.schema.json";

/// Stable record-kind tag for a save-participant risk review.
pub const SAVE_PARTICIPANT_RISK_REVIEW_RECORD_KIND: &str = "save_participant_risk_review_record";

/// Save participant family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveParticipantClass {
    /// Formatter or pretty-printer participant.
    Formatter,
    /// Import organizer or source-action participant.
    OrganizeImports,
    /// Language-service or extension code-action participant.
    CodeAction,
    /// AI apply participant with a reviewed apply plan.
    AiApply,
    /// Read-only validation after apply.
    ValidationAfterApply,
    /// Read-only scanner participant.
    ScannerReadOnly,
    /// Participant class is not known and must be reviewed.
    ParticipantUnknownRequiresReview,
}

impl SaveParticipantClass {
    /// Returns the stable string vocabulary for this participant class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Formatter => "formatter",
            Self::OrganizeImports => "organize_imports",
            Self::CodeAction => "code_action",
            Self::AiApply => "ai_apply",
            Self::ValidationAfterApply => "validation_after_apply",
            Self::ScannerReadOnly => "scanner_read_only",
            Self::ParticipantUnknownRequiresReview => "participant_unknown_requires_review",
        }
    }
}

/// Origin of participant output.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveParticipantOutputOrigin {
    /// Named exact tool rule or source action.
    ExactRule,
    /// Checked-in or imported tool configuration.
    ImportedConfig,
    /// Heuristic fallback output.
    HeuristicFallback,
    /// Generated-artifact lineage output.
    GeneratedLineage,
    /// Policy or trust decision.
    PolicyDecision,
    /// AI-suggested output.
    AiSuggestion,
    /// Read-only validation output.
    ReadOnlyValidation,
    /// Unknown origin requiring review.
    OriginUnknownRequiresReview,
}

impl SaveParticipantOutputOrigin {
    /// Returns the stable string vocabulary for this output origin.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRule => "exact_rule",
            Self::ImportedConfig => "imported_config",
            Self::HeuristicFallback => "heuristic_fallback",
            Self::GeneratedLineage => "generated_lineage",
            Self::PolicyDecision => "policy_decision",
            Self::AiSuggestion => "ai_suggestion",
            Self::ReadOnlyValidation => "read_only_validation",
            Self::OriginUnknownRequiresReview => "origin_unknown_requires_review",
        }
    }
}

/// Fix-safety class used by save participants.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveParticipantFixSafetyClass {
    /// Deterministic local text edit bounded to one staged file or range.
    SafeLocalTextEdit,
    /// Workspace-wide or multi-file mutation that requires preview.
    WorkspaceWidePreviewRequired,
    /// Generated companion update that requires lineage and review posture.
    GeneratedCompanionUpdate,
    /// Policy, trust, or read-only posture blocks mutation.
    PolicyBlocked,
    /// Whole-file rewrite has been disclosed.
    WholeFileRewriteDisclosed,
    /// On-disk target drift requires external-change review.
    ExternalChangeConflictRequiresReview,
    /// Participant cannot prove a safe class.
    FixSafetyUnknownRequiresReview,
}

impl SaveParticipantFixSafetyClass {
    /// Returns the stable string vocabulary for this fix-safety class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeLocalTextEdit => "safe_local_text_edit",
            Self::WorkspaceWidePreviewRequired => "workspace_wide_preview_required",
            Self::GeneratedCompanionUpdate => "generated_companion_update",
            Self::PolicyBlocked => "policy_blocked",
            Self::WholeFileRewriteDisclosed => "whole_file_rewrite_disclosed",
            Self::ExternalChangeConflictRequiresReview => {
                "external_change_conflict_requires_review"
            }
            Self::FixSafetyUnknownRequiresReview => "fix_safety_unknown_requires_review",
        }
    }

    /// Returns true when this class cannot auto-apply on the hot save path.
    pub const fn requires_review_or_block(self) -> bool {
        !matches!(self, Self::SafeLocalTextEdit)
    }
}

/// Review trigger class for save-participant risk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveParticipantReviewTriggerClass {
    /// No review trigger is present.
    NotRequired,
    /// Whole-file rewrite or whole-file fallback.
    WholeFileRewrite,
    /// Generated artifact would be touched.
    GeneratedArtifactImpact,
    /// Multiple files or workspace-wide state would be touched.
    MultiFileEdit,
    /// External change raced the save attempt.
    ExternalChangeRace,
    /// Policy or trust posture narrowed or blocked the write.
    PolicyOrTrustNarrowing,
    /// Provider, semantic, or freshness proof is insufficient.
    ProviderDependencyOrSemanticFreshnessGap,
    /// Source-fidelity conversion would change representation truth.
    SourceFidelityConversion,
    /// Heuristic or AI output is not safe local text.
    OutputOriginHeuristicOrAi,
    /// Unknown trigger requiring review.
    UnknownRequiresReview,
}

impl SaveParticipantReviewTriggerClass {
    /// Returns the stable string vocabulary for this review trigger.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::WholeFileRewrite => "whole_file_rewrite",
            Self::GeneratedArtifactImpact => "generated_artifact_impact",
            Self::MultiFileEdit => "multi_file_edit",
            Self::ExternalChangeRace => "external_change_race",
            Self::PolicyOrTrustNarrowing => "policy_or_trust_narrowing",
            Self::ProviderDependencyOrSemanticFreshnessGap => {
                "provider_dependency_or_semantic_freshness_gap"
            }
            Self::SourceFidelityConversion => "source_fidelity_conversion",
            Self::OutputOriginHeuristicOrAi => "output_origin_heuristic_or_ai",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Returns true when this trigger demands review before durable mutation.
    pub const fn requires_review(self) -> bool {
        !matches!(self, Self::NotRequired)
    }
}

/// Participant run state captured by the risk review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveParticipantRunStateClass {
    /// Participant is planned but has not run.
    Planned,
    /// Participant ran and produced staged output.
    Ran,
    /// Participant failed before producing output.
    Failed,
    /// Participant did not run because review is required first.
    HeldForReview,
    /// Participant did not run because policy blocked it.
    BlockedBeforeRun,
}

impl SaveParticipantRunStateClass {
    /// Returns the stable string vocabulary for this run state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Ran => "ran",
            Self::Failed => "failed",
            Self::HeldForReview => "held_for_review",
            Self::BlockedBeforeRun => "blocked_before_run",
        }
    }
}

/// Save-time rewrite class for source-fidelity disclosure.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceFidelityRewriteClass {
    /// No write is needed.
    NoWriteNeeded,
    /// Bounded text patch.
    TargetedContentPatch,
    /// Whole-file rewrite was declared before the participant ran.
    WholeFileRewriteDeclared,
    /// Whole-file rewrite fallback was detected.
    WholeFileRewriteFallback,
    /// Generator regeneration write.
    GeneratorRegenerationWrite,
    /// Merge resolution write.
    MergeResolutionWrite,
    /// No write is allowed.
    BlockedNoWrite,
}

impl SourceFidelityRewriteClass {
    /// Returns the stable string vocabulary for this rewrite class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoWriteNeeded => "no_write_needed",
            Self::TargetedContentPatch => "targeted_content_patch",
            Self::WholeFileRewriteDeclared => "whole_file_rewrite_declared",
            Self::WholeFileRewriteFallback => "whole_file_rewrite_fallback",
            Self::GeneratorRegenerationWrite => "generator_regeneration_write",
            Self::MergeResolutionWrite => "merge_resolution_write",
            Self::BlockedNoWrite => "blocked_no_write",
        }
    }

    /// Returns true when this class is a whole-file rewrite.
    pub const fn is_whole_file_rewrite(self) -> bool {
        matches!(
            self,
            Self::WholeFileRewriteDeclared | Self::WholeFileRewriteFallback
        )
    }
}

/// Checkpoint posture for a participant or risk review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveParticipantCheckpointPolicyClass {
    /// No checkpoint is required.
    NoneRequired,
    /// Recovery journal is enough.
    RecoveryJournal,
    /// Local-history checkpoint required.
    LocalHistoryCheckpoint,
    /// Workspace mutation checkpoint group required.
    WorkspaceMutationCheckpointGroup,
    /// Generated-artifact regeneration checkpoint required.
    GeneratorRegenerationCheckpoint,
    /// Metadata-only checkpoint required when body capture is blocked.
    MetadataOnlyCheckpoint,
    /// No write is allowed, so checkpointing is blocked.
    CheckpointBlockedNoWrite,
}

impl SaveParticipantCheckpointPolicyClass {
    /// Returns the stable string vocabulary for this checkpoint policy.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneRequired => "none_required",
            Self::RecoveryJournal => "recovery_journal",
            Self::LocalHistoryCheckpoint => "local_history_checkpoint",
            Self::WorkspaceMutationCheckpointGroup => "workspace_mutation_checkpoint_group",
            Self::GeneratorRegenerationCheckpoint => "generator_regeneration_checkpoint",
            Self::MetadataOnlyCheckpoint => "metadata_only_checkpoint",
            Self::CheckpointBlockedNoWrite => "checkpoint_blocked_no_write",
        }
    }
}

/// Overall outcome of the risk review.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveParticipantRiskOutcomeClass {
    /// All participants may run on the staged buffer.
    SafeToRun,
    /// Review is required before a participant mutates staged content.
    ReviewRequiredBeforeMutation,
    /// Participant ran on staged content, but durable write requires review.
    ReviewRequiredBeforeCommit,
    /// External-change rebase or abort is required.
    RebaseRequired,
    /// No durable write is allowed.
    BlockedNoWrite,
    /// Participant failed.
    ParticipantFailed,
    /// Durable write committed.
    Committed,
}

impl SaveParticipantRiskOutcomeClass {
    /// Returns the stable string vocabulary for this outcome.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeToRun => "safe_to_run",
            Self::ReviewRequiredBeforeMutation => "review_required_before_mutation",
            Self::ReviewRequiredBeforeCommit => "review_required_before_commit",
            Self::RebaseRequired => "rebase_required",
            Self::BlockedNoWrite => "blocked_no_write",
            Self::ParticipantFailed => "participant_failed",
            Self::Committed => "committed",
        }
    }
}

/// File-effect summary used for declared and actual participant effects.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FileEffectSummary {
    /// Number of files touched.
    pub files_touched: u64,
    /// Number of files created.
    pub files_created: u64,
    /// Number of files deleted.
    pub files_deleted: u64,
    /// Approximate changed bytes.
    pub changed_bytes: u64,
    /// Whether the effect rewrites the full file.
    pub whole_file_rewrite: bool,
    /// Number of generated artifacts touched.
    pub generated_artifacts_touched: u64,
    /// Number of protected paths touched.
    pub protected_paths_touched: u64,
    /// Whether the effect may touch files outside the visible target.
    pub may_touch_outside_visible_file: bool,
}

impl FileEffectSummary {
    /// Returns a no-op effect summary.
    pub const fn no_write() -> Self {
        Self {
            files_touched: 0,
            files_created: 0,
            files_deleted: 0,
            changed_bytes: 0,
            whole_file_rewrite: false,
            generated_artifacts_touched: 0,
            protected_paths_touched: 0,
            may_touch_outside_visible_file: false,
        }
    }

    /// Returns a safe single-file local edit declaration.
    pub const fn safe_single_file() -> Self {
        Self {
            files_touched: 1,
            files_created: 0,
            files_deleted: 0,
            changed_bytes: u64::MAX,
            whole_file_rewrite: false,
            generated_artifacts_touched: 0,
            protected_paths_touched: 0,
            may_touch_outside_visible_file: false,
        }
    }

    /// Returns a whole-file rewrite declaration.
    pub const fn whole_file_rewrite(changed_bytes: u64) -> Self {
        Self {
            files_touched: 1,
            files_created: 0,
            files_deleted: 0,
            changed_bytes,
            whole_file_rewrite: true,
            generated_artifacts_touched: 0,
            protected_paths_touched: 0,
            may_touch_outside_visible_file: false,
        }
    }
}

/// Participant risk declaration supplied before the participant runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveParticipantRiskDeclaration {
    /// Stable participant id.
    pub participant_id: String,
    /// Participant family.
    pub participant_class: SaveParticipantClass,
    /// Output origin.
    pub output_origin_class: SaveParticipantOutputOrigin,
    /// Fix-safety class.
    pub fix_safety_class: SaveParticipantFixSafetyClass,
    /// Declared file effects.
    pub declared_file_effect_summary: FileEffectSummary,
    /// Source-fidelity rewrite class.
    pub source_fidelity_rewrite_class: SourceFidelityRewriteClass,
    /// Review triggers declared before run.
    pub review_trigger_classes: Vec<SaveParticipantReviewTriggerClass>,
    /// Checkpoint policy for this participant.
    pub checkpoint_policy_class: SaveParticipantCheckpointPolicyClass,
    /// Optional review ticket that proves a risky mutation was already reviewed.
    pub reviewed_ticket_ref: Option<String>,
    /// Human-readable disclosure.
    pub visible_disclosure: String,
}

impl SaveParticipantRiskDeclaration {
    /// Returns a safe local declaration for legacy or simple participants.
    pub fn safe_local(participant_id: impl Into<String>) -> Self {
        let participant_id = participant_id.into();
        Self {
            participant_id: participant_id.clone(),
            participant_class: SaveParticipantClass::Formatter,
            output_origin_class: SaveParticipantOutputOrigin::ExactRule,
            fix_safety_class: SaveParticipantFixSafetyClass::SafeLocalTextEdit,
            declared_file_effect_summary: FileEffectSummary::safe_single_file(),
            source_fidelity_rewrite_class: SourceFidelityRewriteClass::TargetedContentPatch,
            review_trigger_classes: vec![SaveParticipantReviewTriggerClass::NotRequired],
            checkpoint_policy_class: SaveParticipantCheckpointPolicyClass::RecoveryJournal,
            reviewed_ticket_ref: None,
            visible_disclosure: format!("{participant_id} may edit the staged buffer locally."),
        }
    }

    /// Returns a declaration for a disclosed whole-file rewrite.
    pub fn whole_file_rewrite(
        participant_id: impl Into<String>,
        participant_class: SaveParticipantClass,
        output_origin_class: SaveParticipantOutputOrigin,
        changed_bytes: u64,
        visible_disclosure: impl Into<String>,
    ) -> Self {
        Self {
            participant_id: participant_id.into(),
            participant_class,
            output_origin_class,
            fix_safety_class: SaveParticipantFixSafetyClass::WholeFileRewriteDisclosed,
            declared_file_effect_summary: FileEffectSummary::whole_file_rewrite(changed_bytes),
            source_fidelity_rewrite_class: SourceFidelityRewriteClass::WholeFileRewriteDeclared,
            review_trigger_classes: vec![SaveParticipantReviewTriggerClass::WholeFileRewrite],
            checkpoint_policy_class: SaveParticipantCheckpointPolicyClass::LocalHistoryCheckpoint,
            reviewed_ticket_ref: None,
            visible_disclosure: visible_disclosure.into(),
        }
    }

    /// Returns true when this declaration cannot run without review.
    pub fn requires_review_before_run(&self) -> bool {
        if self.fix_safety_class == SaveParticipantFixSafetyClass::PolicyBlocked {
            return true;
        }
        if self.reviewed_ticket_ref.is_some() {
            return false;
        }
        self.fix_safety_class.requires_review_or_block()
            || self.source_fidelity_rewrite_class.is_whole_file_rewrite()
            || self
                .review_trigger_classes
                .iter()
                .any(|trigger| trigger.requires_review())
            || matches!(
                self.participant_class,
                SaveParticipantClass::AiApply
                    | SaveParticipantClass::ParticipantUnknownRequiresReview
            )
            || matches!(
                self.output_origin_class,
                SaveParticipantOutputOrigin::AiSuggestion
                    | SaveParticipantOutputOrigin::HeuristicFallback
                    | SaveParticipantOutputOrigin::OriginUnknownRequiresReview
            )
            || self.declared_file_effect_summary.whole_file_rewrite
            || self
                .declared_file_effect_summary
                .generated_artifacts_touched
                > 0
            || self.declared_file_effect_summary.protected_paths_touched > 0
            || self
                .declared_file_effect_summary
                .may_touch_outside_visible_file
    }
}

/// One participant row inside a risk review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveParticipantRiskEntry {
    /// Stable participant id.
    pub participant_id: String,
    /// Participant family.
    pub participant_class: SaveParticipantClass,
    /// Run state.
    pub run_state_class: SaveParticipantRunStateClass,
    /// Output origin.
    pub output_origin_class: SaveParticipantOutputOrigin,
    /// Fix-safety class.
    pub fix_safety_class: SaveParticipantFixSafetyClass,
    /// Declared file effects.
    pub declared_file_effect_summary: FileEffectSummary,
    /// Actual file effects after staged execution, when available.
    pub actual_file_effect_summary: Option<FileEffectSummary>,
    /// Source-fidelity rewrite class.
    pub source_fidelity_rewrite_class: SourceFidelityRewriteClass,
    /// Review triggers.
    pub review_trigger_classes: Vec<SaveParticipantReviewTriggerClass>,
    /// Checkpoint policy.
    pub checkpoint_policy_class: SaveParticipantCheckpointPolicyClass,
    /// Optional reviewed-ticket ref.
    pub reviewed_ticket_ref: Option<String>,
    /// Human-readable disclosure.
    pub visible_disclosure: String,
}

impl From<SaveParticipantRiskDeclaration> for SaveParticipantRiskEntry {
    fn from(value: SaveParticipantRiskDeclaration) -> Self {
        Self {
            participant_id: value.participant_id,
            participant_class: value.participant_class,
            run_state_class: SaveParticipantRunStateClass::Planned,
            output_origin_class: value.output_origin_class,
            fix_safety_class: value.fix_safety_class,
            declared_file_effect_summary: value.declared_file_effect_summary,
            actual_file_effect_summary: None,
            source_fidelity_rewrite_class: value.source_fidelity_rewrite_class,
            review_trigger_classes: value.review_trigger_classes,
            checkpoint_policy_class: value.checkpoint_policy_class,
            reviewed_ticket_ref: value.reviewed_ticket_ref,
            visible_disclosure: value.visible_disclosure,
        }
    }
}

/// Save-participant risk review emitted with a staged save result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveParticipantRiskReview {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub save_participant_risk_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable risk-review id.
    pub save_participant_risk_review_id: String,
    /// Save packet this risk review belongs to.
    pub save_packet_ref: String,
    /// Overall risk outcome.
    pub outcome_class: SaveParticipantRiskOutcomeClass,
    /// Optional checkpoint ref protecting the save.
    pub checkpoint_ref: Option<String>,
    /// Optional external-change event ref.
    pub external_change_event_ref: Option<String>,
    /// Source-fidelity adjustments detected on staged participant output.
    pub source_fidelity_adjustments: Vec<SourceFidelityAdjustment>,
    /// Participant rows.
    pub participant_entries: Vec<SaveParticipantRiskEntry>,
    /// Whether support export may include this record without raw source.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl SaveParticipantRiskReview {
    /// Opens a risk review for a save packet.
    pub fn open(
        save_participant_risk_review_id: impl Into<String>,
        save_packet_ref: impl Into<String>,
        checkpoint_ref: Option<String>,
        declarations: Vec<SaveParticipantRiskDeclaration>,
    ) -> Self {
        let participant_entries: Vec<_> = declarations.into_iter().map(Into::into).collect();
        let mut review = Self {
            record_kind: SAVE_PARTICIPANT_RISK_REVIEW_RECORD_KIND.to_owned(),
            save_participant_risk_schema_version: SAVE_PARTICIPANT_RISK_SCHEMA_VERSION,
            schema_ref: SAVE_PARTICIPANT_RISK_SCHEMA_REF.to_owned(),
            save_participant_risk_review_id: save_participant_risk_review_id.into(),
            save_packet_ref: save_packet_ref.into(),
            outcome_class: SaveParticipantRiskOutcomeClass::SafeToRun,
            checkpoint_ref,
            external_change_event_ref: None,
            source_fidelity_adjustments: Vec::new(),
            participant_entries,
            raw_payload_excluded: true,
            summary: "Save participants are safe to run on staged content.".to_owned(),
        };
        if review.requires_review_before_run() {
            review.mark_review_required_before_mutation();
        }
        review
    }

    /// Returns true when a participant must be reviewed before it runs.
    pub fn requires_review_before_run(&self) -> bool {
        self.participant_entries
            .iter()
            .any(entry_requires_review_before_run)
    }

    /// Marks risky participants as held for review before mutation.
    pub fn mark_review_required_before_mutation(&mut self) {
        for entry in &mut self.participant_entries {
            if entry_requires_review_before_run(entry) {
                entry.run_state_class =
                    if entry.fix_safety_class == SaveParticipantFixSafetyClass::PolicyBlocked {
                        SaveParticipantRunStateClass::BlockedBeforeRun
                    } else {
                        SaveParticipantRunStateClass::HeldForReview
                    };
            }
        }
        self.outcome_class = SaveParticipantRiskOutcomeClass::ReviewRequiredBeforeMutation;
        self.summary =
            "Save participant requires review before mutating staged content.".to_owned();
    }

    /// Records actual staged effects for a participant.
    pub fn record_actual_effect(&mut self, participant_id: &str, actual: FileEffectSummary) {
        if let Some(entry) = self
            .participant_entries
            .iter_mut()
            .find(|entry| entry.participant_id == participant_id)
        {
            entry.run_state_class = SaveParticipantRunStateClass::Ran;
            entry.actual_file_effect_summary = Some(actual.clone());
            if actual.whole_file_rewrite && !entry.declared_file_effect_summary.whole_file_rewrite {
                entry.fix_safety_class = SaveParticipantFixSafetyClass::WholeFileRewriteDisclosed;
                entry.source_fidelity_rewrite_class =
                    SourceFidelityRewriteClass::WholeFileRewriteFallback;
                add_unique(
                    &mut entry.review_trigger_classes,
                    SaveParticipantReviewTriggerClass::WholeFileRewrite,
                );
                entry.visible_disclosure =
                    "Participant output rewrites the whole file and requires review.".to_owned();
                self.outcome_class = SaveParticipantRiskOutcomeClass::ReviewRequiredBeforeCommit;
                self.summary =
                    "Participant output widened to a whole-file rewrite before commit.".to_owned();
            }
        }
    }

    /// Records participant failure.
    pub fn mark_participant_failed(&mut self, participant_id: &str) {
        if let Some(entry) = self
            .participant_entries
            .iter_mut()
            .find(|entry| entry.participant_id == participant_id)
        {
            entry.run_state_class = SaveParticipantRunStateClass::Failed;
        }
        self.outcome_class = SaveParticipantRiskOutcomeClass::ParticipantFailed;
        self.summary = "Save participant failed before durable write.".to_owned();
    }

    /// Records source-fidelity adjustments that require review before commit.
    pub fn mark_source_fidelity_adjustments(&mut self, adjustments: Vec<SourceFidelityAdjustment>) {
        for adjustment in adjustments {
            if !self.source_fidelity_adjustments.contains(&adjustment) {
                self.source_fidelity_adjustments.push(adjustment);
            }
        }
        for entry in &mut self.participant_entries {
            if entry.run_state_class == SaveParticipantRunStateClass::Ran {
                add_unique(
                    &mut entry.review_trigger_classes,
                    SaveParticipantReviewTriggerClass::SourceFidelityConversion,
                );
            }
        }
        self.outcome_class = SaveParticipantRiskOutcomeClass::ReviewRequiredBeforeCommit;
        self.summary =
            "Participant output would change source-fidelity posture before save.".to_owned();
    }

    /// Records external-change drift detected after staged participants ran.
    pub fn mark_external_change(&mut self, external_change_event_ref: impl Into<String>) {
        self.external_change_event_ref = Some(external_change_event_ref.into());
        for entry in &mut self.participant_entries {
            add_unique(
                &mut entry.review_trigger_classes,
                SaveParticipantReviewTriggerClass::ExternalChangeRace,
            );
        }
        self.outcome_class = SaveParticipantRiskOutcomeClass::RebaseRequired;
        self.summary =
            "On-disk target changed during save; rebase, compare, or abort is required.".to_owned();
    }

    /// Marks the review as blocked without a durable write.
    pub fn mark_blocked_no_write(&mut self, summary: impl Into<String>) {
        self.outcome_class = SaveParticipantRiskOutcomeClass::BlockedNoWrite;
        self.summary = summary.into();
    }

    /// Marks the review as committed.
    pub fn mark_committed(&mut self) {
        self.outcome_class = SaveParticipantRiskOutcomeClass::Committed;
        self.summary = "Save participants completed and durable write committed.".to_owned();
    }

    /// Returns true when the review is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == SAVE_PARTICIPANT_RISK_SCHEMA_REF
            && self.record_kind == SAVE_PARTICIPANT_RISK_REVIEW_RECORD_KIND
    }
}

/// Summarizes the byte-level effect of a participant on staged content.
pub fn summarize_staged_file_effect(before: &[u8], after: &[u8]) -> FileEffectSummary {
    if before == after {
        return FileEffectSummary::no_write();
    }
    let changed_bytes = changed_byte_count(before, after);
    FileEffectSummary {
        files_touched: 1,
        files_created: 0,
        files_deleted: 0,
        changed_bytes,
        whole_file_rewrite: looks_like_whole_file_rewrite(before, after),
        generated_artifacts_touched: 0,
        protected_paths_touched: 0,
        may_touch_outside_visible_file: false,
    }
}

fn changed_byte_count(before: &[u8], after: &[u8]) -> u64 {
    let paired = before
        .iter()
        .zip(after.iter())
        .filter(|(left, right)| left != right)
        .count() as u64;
    paired + before.len().abs_diff(after.len()) as u64
}

fn looks_like_whole_file_rewrite(before: &[u8], after: &[u8]) -> bool {
    if before.is_empty() {
        return !after.is_empty();
    }
    if after.is_empty() {
        return true;
    }

    let min_len = before.len().min(after.len());
    if min_len < 128 {
        return changed_byte_count(before, after) as usize >= min_len;
    }

    let mut prefix = 0usize;
    while prefix < min_len && before[prefix] == after[prefix] {
        prefix += 1;
    }

    let mut suffix = 0usize;
    while suffix < min_len.saturating_sub(prefix)
        && before[before.len() - 1 - suffix] == after[after.len() - 1 - suffix]
    {
        suffix += 1;
    }

    let preserved = prefix.saturating_add(suffix);
    preserved.saturating_mul(5) < before.len()
}

fn entry_requires_review_before_run(entry: &SaveParticipantRiskEntry) -> bool {
    if entry.fix_safety_class == SaveParticipantFixSafetyClass::PolicyBlocked {
        return true;
    }
    if entry.reviewed_ticket_ref.is_some() {
        return false;
    }
    entry.fix_safety_class.requires_review_or_block()
        || entry.source_fidelity_rewrite_class.is_whole_file_rewrite()
        || entry
            .review_trigger_classes
            .iter()
            .any(|trigger| trigger.requires_review())
        || entry.declared_file_effect_summary.whole_file_rewrite
        || entry
            .declared_file_effect_summary
            .generated_artifacts_touched
            > 0
        || entry.declared_file_effect_summary.protected_paths_touched > 0
        || entry
            .declared_file_effect_summary
            .may_touch_outside_visible_file
        || matches!(
            entry.participant_class,
            SaveParticipantClass::AiApply | SaveParticipantClass::ParticipantUnknownRequiresReview
        )
        || matches!(
            entry.output_origin_class,
            SaveParticipantOutputOrigin::AiSuggestion
                | SaveParticipantOutputOrigin::HeuristicFallback
                | SaveParticipantOutputOrigin::OriginUnknownRequiresReview
        )
}

fn add_unique<T>(items: &mut Vec<T>, item: T)
where
    T: PartialEq,
{
    if !items.contains(&item) {
        items.push(item);
    }
}
