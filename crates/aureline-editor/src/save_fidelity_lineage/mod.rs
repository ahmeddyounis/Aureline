//! Source-fidelity save lineage: the editor's governed, export-safe projection
//! of the staged-save participant contract.
//!
//! The staged save coordinator in `aureline-workspace` runs save participants
//! (formatter, organize-imports, linter/code-action, scanner, AI apply) on
//! staged content, preserves encoding / BOM / line-ending / final-newline
//! posture, performs compare-before-write against the pinned save target, and
//! emits a [`SaveParticipantRiskReview`]. That risk review is the canonical
//! truth source for what a save attempt did.
//!
//! This module ingests that live record verbatim — it never clones status text
//! — together with the open-time [`SourceFidelityRecord`], and projects one
//! governed lineage record per save posture that:
//!
//! 1. pins each participant to a fixed hot-save-path **stage**
//!    (`format -> organize_imports -> lint -> code_action_apply -> scan ->
//!    validate_after_apply`) so participant ordering is explicit and provable;
//! 2. classifies each fix action as one of `safe_inline`, `preview_required`,
//!    `multi_file`, `generated_scope`, or `semantically_broad`, and enforces
//!    checkpoint-plus-preview once an action crosses the safe-inline threshold;
//!    and
//! 3. maps each participant to an export-safe **recovery action** (`exact_undo`,
//!    `compensation`, `regeneration`, `checkpoint_restore`, or `none_no_write`)
//!    so the user can always see how a committed or refused save reverses before
//!    any destructive cleanup.
//!
//! The projected record auto-narrows below Stable with a named reason whenever
//! it cannot prove the contract on the claimed posture (a participant failed,
//! ordering was not canonical, preview-plus-checkpoint was not enforced, or the
//! open-time encoding cannot round-trip). It excludes raw source, raw patches,
//! and raw tool logs, so it is safe for support export.

use aureline_workspace::save::{
    BomStateDetected, DetectedEncoding, DetectionSource, ExecutableIntent, FileEffectSummary,
    FinalNewlineDetected, NewlineModeDetected, SaveParticipantClass, SaveParticipantFixSafetyClass,
    SaveParticipantOutputOrigin, SaveParticipantRiskEntry, SaveParticipantRiskOutcomeClass,
    SaveParticipantRiskReview, SaveParticipantRunStateClass, SourceFidelityAdjustment,
    SourceFidelityRecord, SourceFidelityRewriteClass,
};
use serde::{Deserialize, Serialize};

/// Schema version for the save-fidelity lineage record.
pub const SAVE_FIDELITY_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Schema reference for the save-fidelity lineage record.
pub const SAVE_FIDELITY_LINEAGE_SCHEMA_REF: &str =
    "schemas/editor/save_fidelity_lineage.schema.json";

/// Stable record-kind tag for the save-fidelity lineage record.
pub const SAVE_FIDELITY_LINEAGE_RECORD_KIND: &str = "save_fidelity_lineage_record";

/// Fixed hot-save-path stage a save participant belongs to.
///
/// The stage ordering is the canonical contract for the order quality
/// participants run on the save path. Lineage entries keep the order the
/// coordinator actually ran them in; the record reports whether that order was
/// canonical so an out-of-order pipeline narrows below Stable instead of
/// silently committing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SaveParticipantStage {
    /// Formatter / pretty-printer normalization.
    Format,
    /// Import organizer source action.
    OrganizeImports,
    /// Linter quality pass slot (auto-fixing linters map here).
    Lint,
    /// Language-service / extension code-action apply (including AI apply).
    CodeActionApply,
    /// Read-only scanner audit after content is settled.
    Scan,
    /// Read-only validation after apply.
    ValidateAfterApply,
    /// Participant could not be sequenced and must be reviewed.
    Unsequenced,
}

impl SaveParticipantStage {
    /// Returns the stable string vocabulary for this stage.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Format => "format",
            Self::OrganizeImports => "organize_imports",
            Self::Lint => "lint",
            Self::CodeActionApply => "code_action_apply",
            Self::Scan => "scan",
            Self::ValidateAfterApply => "validate_after_apply",
            Self::Unsequenced => "unsequenced",
        }
    }

    /// Returns the canonical hot-save-path ordering rank for this stage.
    pub const fn order_rank(self) -> u8 {
        match self {
            Self::Format => 0,
            Self::OrganizeImports => 1,
            Self::Lint => 2,
            Self::CodeActionApply => 3,
            Self::Scan => 4,
            Self::ValidateAfterApply => 5,
            Self::Unsequenced => 6,
        }
    }

    /// Maps a participant class to its canonical save-path stage.
    pub const fn for_participant_class(class: SaveParticipantClass) -> Self {
        match class {
            SaveParticipantClass::Formatter => Self::Format,
            SaveParticipantClass::OrganizeImports => Self::OrganizeImports,
            SaveParticipantClass::CodeAction | SaveParticipantClass::AiApply => {
                Self::CodeActionApply
            }
            SaveParticipantClass::ScannerReadOnly => Self::Scan,
            SaveParticipantClass::ValidationAfterApply => Self::ValidateAfterApply,
            SaveParticipantClass::ParticipantUnknownRequiresReview => Self::Unsequenced,
        }
    }
}

/// Classification of the fix a save participant applies to staged content.
///
/// Anything other than [`FixActionClass::SafeInline`] crosses the declared
/// threshold and demands preview-plus-checkpoint before durable mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FixActionClass {
    /// Deterministic local text edit bounded to the staged file/range.
    SafeInline,
    /// Whole-file rewrite, representation change, or external-change conflict.
    PreviewRequired,
    /// Touches more than the visible file (multi-file or workspace-wide).
    MultiFile,
    /// Touches generated artifacts and must route through regeneration lineage.
    GeneratedScope,
    /// Heuristic, AI, or otherwise unproven output requiring broad review.
    SemanticallyBroad,
}

impl FixActionClass {
    /// Returns the stable string vocabulary for this fix-action class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeInline => "safe_inline",
            Self::PreviewRequired => "preview_required",
            Self::MultiFile => "multi_file",
            Self::GeneratedScope => "generated_scope",
            Self::SemanticallyBroad => "semantically_broad",
        }
    }

    /// Returns true when this fix action must be previewed before commit.
    pub const fn requires_preview(self) -> bool {
        !matches!(self, Self::SafeInline)
    }

    /// Returns true when this fix action must be checkpointed before commit.
    pub const fn requires_checkpoint(self) -> bool {
        !matches!(self, Self::SafeInline)
    }
}

/// Why preview was or was not required for a participant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreviewReason {
    /// Edit is a safe inline patch; no preview is required.
    NotRequiredSafeInline,
    /// Output rewrites the whole file or changes representation truth.
    WholeFileOrRepresentation,
    /// Output may touch files outside the visible target.
    MultiFileScope,
    /// Output touches generated artifacts.
    GeneratedScope,
    /// Output is heuristic, AI, or otherwise semantically broad.
    SemanticallyBroad,
}

impl PreviewReason {
    /// Returns the stable string vocabulary for this preview reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequiredSafeInline => "not_required_safe_inline",
            Self::WholeFileOrRepresentation => "whole_file_or_representation",
            Self::MultiFileScope => "multi_file_scope",
            Self::GeneratedScope => "generated_scope",
            Self::SemanticallyBroad => "semantically_broad",
        }
    }

    const fn for_fix_action(class: FixActionClass) -> Self {
        match class {
            FixActionClass::SafeInline => Self::NotRequiredSafeInline,
            FixActionClass::PreviewRequired => Self::WholeFileOrRepresentation,
            FixActionClass::MultiFile => Self::MultiFileScope,
            FixActionClass::GeneratedScope => Self::GeneratedScope,
            FixActionClass::SemanticallyBroad => Self::SemanticallyBroad,
        }
    }
}

/// How a committed or refused participant effect can be reversed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryActionClass {
    /// The buffer undo stack reverses the edit exactly.
    ExactUndo,
    /// A compensating action is required (multi-file / cross-target effect).
    Compensation,
    /// Re-running the generator restores the generated companion.
    Regeneration,
    /// Restoring a checkpoint reverses a broad or whole-file mutation.
    CheckpointRestore,
    /// Nothing durable was written, so no recovery is required.
    NoneNoWrite,
}

impl RecoveryActionClass {
    /// Returns the stable string vocabulary for this recovery action.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactUndo => "exact_undo",
            Self::Compensation => "compensation",
            Self::Regeneration => "regeneration",
            Self::CheckpointRestore => "checkpoint_restore",
            Self::NoneNoWrite => "none_no_write",
        }
    }
}

/// Named reason a lineage record narrows below Stable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageNarrowReason {
    /// A save participant failed before producing proven output.
    ParticipantFailed,
    /// A participant ran outside the canonical hot-save-path ordering.
    ParticipantOrderingViolation,
    /// A threshold-crossing action committed without preview-plus-checkpoint.
    PreviewOrCheckpointNotEnforced,
    /// The open-time encoding cannot be proven to round-trip on save.
    SourceFidelityUnprovable,
}

impl LineageNarrowReason {
    /// Returns the stable string vocabulary for this narrow reason.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParticipantFailed => "participant_failed",
            Self::ParticipantOrderingViolation => "participant_ordering_violation",
            Self::PreviewOrCheckpointNotEnforced => "preview_or_checkpoint_not_enforced",
            Self::SourceFidelityUnprovable => "source_fidelity_unprovable",
        }
    }
}

/// Stable-qualification posture for a lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageStableQualification {
    /// Whether the record proves the contract on the claimed posture.
    pub qualified: bool,
    /// Stable lifecycle label: `stable` or `narrowed_below_stable`.
    pub level: String,
    /// Named reasons the record narrowed below Stable, when not qualified.
    pub narrow_reasons: Vec<LineageNarrowReason>,
}

/// Open-time source-fidelity posture ingested from a [`SourceFidelityRecord`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceFidelitySummary {
    /// Encoding chosen at open time.
    pub detected_encoding: DetectedEncoding,
    /// Why that encoding was selected.
    pub detection_source: DetectionSource,
    /// Whether a BOM was present at open.
    pub bom_state_detected: BomStateDetected,
    /// Dominant newline mode detected at open.
    pub newline_mode_detected: NewlineModeDetected,
    /// Whether the file ended with a newline terminator at open.
    pub final_newline_detected: FinalNewlineDetected,
    /// Executable-bit intent captured at open, when available.
    pub executable_intent: ExecutableIntent,
    /// Whether the open-time encoding can round-trip on save.
    pub round_trip_provable: bool,
}

impl From<&SourceFidelityRecord> for SourceFidelitySummary {
    fn from(record: &SourceFidelityRecord) -> Self {
        Self {
            detected_encoding: record.detected_encoding,
            detection_source: record.detection_source,
            bom_state_detected: record.bom_state_detected,
            newline_mode_detected: record.newline_mode_detected,
            final_newline_detected: record.final_newline_detected,
            executable_intent: record.executable_intent,
            round_trip_provable: record.detected_encoding != DetectedEncoding::UnknownBinaryLike,
        }
    }
}

/// One participant row in the save-fidelity lineage.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveParticipantLineageEntry {
    /// Position in the order the coordinator ran participants (0-based).
    pub order_index: u8,
    /// Canonical hot-save-path stage for this participant.
    pub stage: SaveParticipantStage,
    /// Stable participant id.
    pub participant_id: String,
    /// Participant family.
    pub participant_class: SaveParticipantClass,
    /// Run state captured by the risk review.
    pub run_state_class: SaveParticipantRunStateClass,
    /// Fix-action classification.
    pub fix_action_class: FixActionClass,
    /// Whether preview was required before commit.
    pub preview_required: bool,
    /// Why preview was or was not required.
    pub preview_reason: PreviewReason,
    /// Whether a checkpoint was required before commit.
    pub checkpoint_required: bool,
    /// How this participant's effect can be reversed.
    pub recovery_action_class: RecoveryActionClass,
    /// Declared file effects before the participant ran.
    pub declared_file_effect_summary: FileEffectSummary,
    /// Actual file effects after staged execution, when available.
    pub actual_file_effect_summary: Option<FileEffectSummary>,
    /// Human-readable disclosure.
    pub visible_disclosure: String,
}

/// Governed, export-safe save-fidelity lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SaveFidelityLineageRecord {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub save_fidelity_lineage_schema_version: u32,
    /// Schema reference.
    pub schema_ref: String,
    /// Stable lineage id.
    pub lineage_id: String,
    /// Save packet this lineage belongs to.
    pub save_packet_ref: String,
    /// Risk review this lineage is projected from.
    pub risk_review_ref: String,
    /// Overall save outcome (ingested from the risk review).
    pub outcome_class: SaveParticipantRiskOutcomeClass,
    /// Open-time source-fidelity posture.
    pub source_fidelity: SourceFidelitySummary,
    /// Source-fidelity adjustments detected on staged output.
    pub source_fidelity_adjustments: Vec<SourceFidelityAdjustment>,
    /// Optional external-change event ref.
    pub external_change_event_ref: Option<String>,
    /// Optional checkpoint ref protecting the save.
    pub checkpoint_ref: Option<String>,
    /// Whether preview-plus-checkpoint was enforced for every committed
    /// threshold-crossing action.
    pub preview_and_checkpoint_enforced: bool,
    /// Whether participants ran in canonical hot-save-path order.
    pub participant_order_canonical: bool,
    /// Ordered participant lineage rows.
    pub entries: Vec<SaveParticipantLineageEntry>,
    /// Stable-qualification posture with named narrow reasons.
    pub stable_qualification: LineageStableQualification,
    /// Whether support export may include this record without raw source.
    pub raw_payload_excluded: bool,
    /// Human-readable summary.
    pub summary: String,
}

impl SaveFidelityLineageRecord {
    /// Returns true when the record is metadata-safe for support export.
    pub fn is_support_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.schema_ref == SAVE_FIDELITY_LINEAGE_SCHEMA_REF
            && self.record_kind == SAVE_FIDELITY_LINEAGE_RECORD_KIND
    }

    /// Returns true when the record proves the contract on the claimed posture.
    pub fn is_stable_qualified(&self) -> bool {
        self.stable_qualification.qualified
    }
}

/// Classifies the fix a participant applies to staged content.
fn classify_fix_action(entry: &SaveParticipantRiskEntry) -> FixActionClass {
    let effect = entry
        .actual_file_effect_summary
        .as_ref()
        .unwrap_or(&entry.declared_file_effect_summary);

    // Heuristic / AI / unprovable output is the broadest class and wins first.
    if matches!(
        entry.participant_class,
        SaveParticipantClass::AiApply | SaveParticipantClass::ParticipantUnknownRequiresReview
    ) || matches!(
        entry.output_origin_class,
        SaveParticipantOutputOrigin::AiSuggestion
            | SaveParticipantOutputOrigin::HeuristicFallback
            | SaveParticipantOutputOrigin::OriginUnknownRequiresReview
    ) || entry.fix_safety_class == SaveParticipantFixSafetyClass::FixSafetyUnknownRequiresReview
    {
        return FixActionClass::SemanticallyBroad;
    }

    // Generated-artifact scope routes through regeneration lineage.
    if entry.fix_safety_class == SaveParticipantFixSafetyClass::GeneratedCompanionUpdate
        || entry.source_fidelity_rewrite_class
            == SourceFidelityRewriteClass::GeneratorRegenerationWrite
        || entry.output_origin_class == SaveParticipantOutputOrigin::GeneratedLineage
        || effect.generated_artifacts_touched > 0
    {
        return FixActionClass::GeneratedScope;
    }

    // Anything reaching beyond the visible file is multi-file scope.
    if effect.files_touched > 1
        || effect.may_touch_outside_visible_file
        || effect.protected_paths_touched > 0
        || entry.fix_safety_class == SaveParticipantFixSafetyClass::WorkspaceWidePreviewRequired
    {
        return FixActionClass::MultiFile;
    }

    // Whole-file rewrite, representation change, external-change conflict, or a
    // policy block on a single file still demands preview before commit.
    if effect.whole_file_rewrite
        || entry.source_fidelity_rewrite_class.is_whole_file_rewrite()
        || matches!(
            entry.fix_safety_class,
            SaveParticipantFixSafetyClass::WholeFileRewriteDisclosed
                | SaveParticipantFixSafetyClass::ExternalChangeConflictRequiresReview
                | SaveParticipantFixSafetyClass::PolicyBlocked
        )
    {
        return FixActionClass::PreviewRequired;
    }

    FixActionClass::SafeInline
}

/// Maps a participant to the recovery action that reverses its effect.
fn recovery_action_for(
    fix_action: FixActionClass,
    run_state: SaveParticipantRunStateClass,
    committed: bool,
) -> RecoveryActionClass {
    match run_state {
        SaveParticipantRunStateClass::Planned
        | SaveParticipantRunStateClass::Failed
        | SaveParticipantRunStateClass::HeldForReview
        | SaveParticipantRunStateClass::BlockedBeforeRun => {
            return RecoveryActionClass::NoneNoWrite;
        }
        SaveParticipantRunStateClass::Ran => {}
    }

    // A participant that ran on staged content but never committed reverses
    // exactly in-buffer: no durable bytes moved.
    if !committed {
        return RecoveryActionClass::ExactUndo;
    }

    match fix_action {
        FixActionClass::SafeInline => RecoveryActionClass::ExactUndo,
        FixActionClass::MultiFile => RecoveryActionClass::Compensation,
        FixActionClass::GeneratedScope => RecoveryActionClass::Regeneration,
        FixActionClass::PreviewRequired | FixActionClass::SemanticallyBroad => {
            RecoveryActionClass::CheckpointRestore
        }
    }
}

/// Projects a governed save-fidelity lineage record from a live risk review and
/// the open-time source-fidelity record.
///
/// The projection is deterministic and ingests the risk review verbatim: it
/// adds the ordered participant contract, fix-action classification, preview /
/// checkpoint enforcement, recovery mapping, and stable qualification without
/// re-deriving any save outcome.
pub fn project_save_fidelity_lineage(
    lineage_id: impl Into<String>,
    risk_review: &SaveParticipantRiskReview,
    source_fidelity: &SourceFidelityRecord,
) -> SaveFidelityLineageRecord {
    let committed = risk_review.outcome_class == SaveParticipantRiskOutcomeClass::Committed;

    let mut entries = Vec::with_capacity(risk_review.participant_entries.len());
    let mut last_rank: Option<u8> = None;
    let mut order_canonical = true;
    let mut threshold_crosser_committed = false;

    for (index, risk_entry) in risk_review.participant_entries.iter().enumerate() {
        let stage = SaveParticipantStage::for_participant_class(risk_entry.participant_class);
        let rank = stage.order_rank();
        if let Some(previous) = last_rank {
            if rank < previous {
                order_canonical = false;
            }
        }
        last_rank = Some(rank);

        let fix_action = classify_fix_action(risk_entry);
        let preview_required = fix_action.requires_preview();
        let checkpoint_required = fix_action.requires_checkpoint();
        let recovery_action_class =
            recovery_action_for(fix_action, risk_entry.run_state_class, committed);

        if committed
            && preview_required
            && risk_entry.run_state_class == SaveParticipantRunStateClass::Ran
        {
            threshold_crosser_committed = true;
        }

        entries.push(SaveParticipantLineageEntry {
            order_index: u8::try_from(index).unwrap_or(u8::MAX),
            stage,
            participant_id: risk_entry.participant_id.clone(),
            participant_class: risk_entry.participant_class,
            run_state_class: risk_entry.run_state_class,
            fix_action_class: fix_action,
            preview_required,
            preview_reason: PreviewReason::for_fix_action(fix_action),
            checkpoint_required,
            recovery_action_class,
            declared_file_effect_summary: risk_entry.declared_file_effect_summary.clone(),
            actual_file_effect_summary: risk_entry.actual_file_effect_summary.clone(),
            visible_disclosure: risk_entry.visible_disclosure.clone(),
        });
    }

    let source_fidelity_summary = SourceFidelitySummary::from(source_fidelity);

    // Preview-plus-checkpoint is enforced unless a threshold-crossing action
    // actually committed durable bytes without a checkpoint backing it.
    let preview_and_checkpoint_enforced =
        !(threshold_crosser_committed && risk_review.checkpoint_ref.is_none());

    let mut narrow_reasons = Vec::new();
    if risk_review.outcome_class == SaveParticipantRiskOutcomeClass::ParticipantFailed {
        narrow_reasons.push(LineageNarrowReason::ParticipantFailed);
    }
    if !order_canonical {
        narrow_reasons.push(LineageNarrowReason::ParticipantOrderingViolation);
    }
    if !preview_and_checkpoint_enforced {
        narrow_reasons.push(LineageNarrowReason::PreviewOrCheckpointNotEnforced);
    }
    if !source_fidelity_summary.round_trip_provable {
        narrow_reasons.push(LineageNarrowReason::SourceFidelityUnprovable);
    }

    let qualified = narrow_reasons.is_empty();
    let stable_qualification = LineageStableQualification {
        qualified,
        level: if qualified {
            "stable".to_owned()
        } else {
            "narrowed_below_stable".to_owned()
        },
        narrow_reasons,
    };

    let summary = build_summary(risk_review.outcome_class, &stable_qualification, &entries);

    SaveFidelityLineageRecord {
        record_kind: SAVE_FIDELITY_LINEAGE_RECORD_KIND.to_owned(),
        save_fidelity_lineage_schema_version: SAVE_FIDELITY_LINEAGE_SCHEMA_VERSION,
        schema_ref: SAVE_FIDELITY_LINEAGE_SCHEMA_REF.to_owned(),
        lineage_id: lineage_id.into(),
        save_packet_ref: risk_review.save_packet_ref.clone(),
        risk_review_ref: risk_review.save_participant_risk_review_id.clone(),
        outcome_class: risk_review.outcome_class,
        source_fidelity: source_fidelity_summary,
        source_fidelity_adjustments: risk_review.source_fidelity_adjustments.clone(),
        external_change_event_ref: risk_review.external_change_event_ref.clone(),
        checkpoint_ref: risk_review.checkpoint_ref.clone(),
        preview_and_checkpoint_enforced,
        participant_order_canonical: order_canonical,
        entries,
        stable_qualification,
        raw_payload_excluded: true,
        summary,
    }
}

fn build_summary(
    outcome: SaveParticipantRiskOutcomeClass,
    qualification: &LineageStableQualification,
    entries: &[SaveParticipantLineageEntry],
) -> String {
    let ran = entries
        .iter()
        .filter(|entry| entry.run_state_class == SaveParticipantRunStateClass::Ran)
        .count();
    if qualification.qualified {
        format!(
            "Save lineage proven Stable: {ran} participant(s) ran, outcome {outcome}.",
            outcome = outcome.as_str()
        )
    } else {
        let reasons: Vec<&str> = qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        format!(
            "Save lineage narrowed below Stable (outcome {outcome}): {reasons}.",
            outcome = outcome.as_str(),
            reasons = reasons.join(", ")
        )
    }
}

/// Renders the export-safe human-readable lines for a lineage record.
///
/// This is the shared projection consumed by the editor save-status surface,
/// the headless CLI emitter, Help/About, and support export, so they never
/// clone status text from each other.
pub fn save_fidelity_lineage_lines(record: &SaveFidelityLineageRecord) -> Vec<String> {
    let mut lines = Vec::new();
    lines.push(format!(
        "Save fidelity lineage — {} ({})",
        record.outcome_class.as_str(),
        record.stable_qualification.level
    ));
    lines.push(format!(
        "packet={} review={}",
        record.save_packet_ref, record.risk_review_ref
    ));
    lines.push(format!(
        "encoding={} bom={} newline={} final_newline={} round_trip_provable={}",
        record.source_fidelity.detected_encoding.as_str(),
        record.source_fidelity.bom_state_detected.as_str(),
        record.source_fidelity.newline_mode_detected.as_str(),
        record.source_fidelity.final_newline_detected.as_str(),
        record.source_fidelity.round_trip_provable,
    ));
    if !record.source_fidelity_adjustments.is_empty() {
        let adjustments: Vec<&str> = record
            .source_fidelity_adjustments
            .iter()
            .map(|adjustment| adjustment.as_str())
            .collect();
        lines.push(format!(
            "source_fidelity_adjustments: {}",
            adjustments.join(", ")
        ));
    }
    lines.push(format!(
        "ordering_canonical={} preview_and_checkpoint_enforced={} checkpoint={}",
        record.participant_order_canonical,
        record.preview_and_checkpoint_enforced,
        record.checkpoint_ref.as_deref().unwrap_or("none"),
    ));
    if let Some(event) = &record.external_change_event_ref {
        lines.push(format!("external_change_event: {event}"));
    }

    lines.push("Participants (run order):".to_owned());
    for entry in &record.entries {
        lines.push(format!(
            "  {idx}. [{stage}] {id} — {run_state} | fix={fix} preview={preview}({reason}) checkpoint={checkpoint} recovery={recovery}",
            idx = entry.order_index,
            stage = entry.stage.as_str(),
            id = entry.participant_id,
            run_state = entry.run_state_class.as_str(),
            fix = entry.fix_action_class.as_str(),
            preview = entry.preview_required,
            reason = entry.preview_reason.as_str(),
            checkpoint = entry.checkpoint_required,
            recovery = entry.recovery_action_class.as_str(),
        ));
    }

    if !record.stable_qualification.qualified {
        let reasons: Vec<&str> = record
            .stable_qualification
            .narrow_reasons
            .iter()
            .map(|reason| reason.as_str())
            .collect();
        lines.push(format!("Narrowed below Stable: {}", reasons.join(", ")));
    }

    lines.push(record.summary.clone());
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use aureline_workspace::save::{
        SaveParticipantCheckpointPolicyClass, SaveParticipantReviewTriggerClass,
        SaveParticipantRiskDeclaration,
    };

    fn source_fidelity_utf8() -> SourceFidelityRecord {
        SourceFidelityRecord {
            detected_encoding: DetectedEncoding::Utf8,
            detection_source: DetectionSource::Utf8Heuristic,
            bom_state_detected: BomStateDetected::Absent,
            newline_mode_detected: NewlineModeDetected::Lf,
            final_newline_detected: FinalNewlineDetected::Present,
            executable_intent: ExecutableIntent::NonExecutable,
        }
    }

    fn formatter_declaration() -> SaveParticipantRiskDeclaration {
        SaveParticipantRiskDeclaration {
            participant_id: "rustfmt".to_owned(),
            participant_class: SaveParticipantClass::Formatter,
            output_origin_class: SaveParticipantOutputOrigin::ExactRule,
            fix_safety_class: SaveParticipantFixSafetyClass::SafeLocalTextEdit,
            declared_file_effect_summary: FileEffectSummary::safe_single_file(),
            source_fidelity_rewrite_class: SourceFidelityRewriteClass::TargetedContentPatch,
            review_trigger_classes: vec![SaveParticipantReviewTriggerClass::NotRequired],
            checkpoint_policy_class: SaveParticipantCheckpointPolicyClass::RecoveryJournal,
            reviewed_ticket_ref: None,
            visible_disclosure: "rustfmt formats the staged buffer.".to_owned(),
        }
    }

    fn organize_imports_declaration() -> SaveParticipantRiskDeclaration {
        SaveParticipantRiskDeclaration {
            participant_id: "organize-imports".to_owned(),
            participant_class: SaveParticipantClass::OrganizeImports,
            output_origin_class: SaveParticipantOutputOrigin::ExactRule,
            fix_safety_class: SaveParticipantFixSafetyClass::SafeLocalTextEdit,
            declared_file_effect_summary: FileEffectSummary::safe_single_file(),
            source_fidelity_rewrite_class: SourceFidelityRewriteClass::TargetedContentPatch,
            review_trigger_classes: vec![SaveParticipantReviewTriggerClass::NotRequired],
            checkpoint_policy_class: SaveParticipantCheckpointPolicyClass::RecoveryJournal,
            reviewed_ticket_ref: None,
            visible_disclosure: "organizes imports in the staged buffer.".to_owned(),
        }
    }

    #[test]
    fn safe_inline_pipeline_is_stable_and_recovers_with_exact_undo() {
        let mut review = SaveParticipantRiskReview::open(
            "review:safe",
            "save_packet:safe",
            None,
            vec![formatter_declaration(), organize_imports_declaration()],
        );
        review.record_actual_effect("rustfmt", FileEffectSummary::safe_single_file());
        review.record_actual_effect("organize-imports", FileEffectSummary::safe_single_file());
        review.mark_committed();

        let record =
            project_save_fidelity_lineage("lineage:safe", &review, &source_fidelity_utf8());

        assert!(record.is_stable_qualified());
        assert!(record.is_support_export_safe());
        assert!(record.participant_order_canonical);
        assert!(record.preview_and_checkpoint_enforced);
        assert_eq!(record.entries.len(), 2);
        assert_eq!(record.entries[0].stage, SaveParticipantStage::Format);
        assert_eq!(
            record.entries[1].stage,
            SaveParticipantStage::OrganizeImports
        );
        for entry in &record.entries {
            assert_eq!(entry.fix_action_class, FixActionClass::SafeInline);
            assert_eq!(entry.recovery_action_class, RecoveryActionClass::ExactUndo);
            assert!(!entry.preview_required);
        }
    }

    #[test]
    fn out_of_order_pipeline_narrows_below_stable() {
        // organize-imports (rank 1) before formatter (rank 0) is non-canonical.
        let review = SaveParticipantRiskReview::open(
            "review:order",
            "save_packet:order",
            None,
            vec![organize_imports_declaration(), formatter_declaration()],
        );

        let record =
            project_save_fidelity_lineage("lineage:order", &review, &source_fidelity_utf8());

        assert!(!record.participant_order_canonical);
        assert!(!record.is_stable_qualified());
        assert!(record
            .stable_qualification
            .narrow_reasons
            .contains(&LineageNarrowReason::ParticipantOrderingViolation));
    }

    #[test]
    fn whole_file_rewrite_requires_preview_and_maps_to_checkpoint_restore() {
        let declaration = SaveParticipantRiskDeclaration::whole_file_rewrite(
            "prettier-whole-file",
            SaveParticipantClass::Formatter,
            SaveParticipantOutputOrigin::ImportedConfig,
            4096,
            "prettier rewrites the whole file.",
        );
        let review = SaveParticipantRiskReview::open(
            "review:whole",
            "save_packet:whole",
            Some("checkpoint:whole".to_owned()),
            vec![declaration],
        );

        let record =
            project_save_fidelity_lineage("lineage:whole", &review, &source_fidelity_utf8());

        let entry = &record.entries[0];
        assert_eq!(entry.fix_action_class, FixActionClass::PreviewRequired);
        assert!(entry.preview_required);
        assert!(entry.checkpoint_required);
        assert_eq!(
            entry.preview_reason,
            PreviewReason::WholeFileOrRepresentation
        );
        // Held for review before mutation -> no durable write -> none_no_write.
        assert_eq!(
            entry.recovery_action_class,
            RecoveryActionClass::NoneNoWrite
        );
        assert_eq!(
            record.outcome_class,
            SaveParticipantRiskOutcomeClass::ReviewRequiredBeforeMutation
        );
    }

    #[test]
    fn participant_failure_narrows_below_stable() {
        let mut review = SaveParticipantRiskReview::open(
            "review:fail",
            "save_packet:fail",
            None,
            vec![formatter_declaration()],
        );
        review.mark_participant_failed("rustfmt");

        let record =
            project_save_fidelity_lineage("lineage:fail", &review, &source_fidelity_utf8());

        assert!(!record.is_stable_qualified());
        assert!(record
            .stable_qualification
            .narrow_reasons
            .contains(&LineageNarrowReason::ParticipantFailed));
    }

    #[test]
    fn unknown_binary_encoding_narrows_source_fidelity() {
        let mut fidelity = source_fidelity_utf8();
        fidelity.detected_encoding = DetectedEncoding::UnknownBinaryLike;
        let mut review = SaveParticipantRiskReview::open(
            "review:binary",
            "save_packet:binary",
            None,
            vec![formatter_declaration()],
        );
        review.record_actual_effect("rustfmt", FileEffectSummary::safe_single_file());
        review.mark_committed();

        let record = project_save_fidelity_lineage("lineage:binary", &review, &fidelity);

        assert!(!record.source_fidelity.round_trip_provable);
        assert!(record
            .stable_qualification
            .narrow_reasons
            .contains(&LineageNarrowReason::SourceFidelityUnprovable));
    }

    #[test]
    fn lines_render_ordered_participants_and_narrow_reason() {
        let review = SaveParticipantRiskReview::open(
            "review:lines",
            "save_packet:lines",
            None,
            vec![organize_imports_declaration(), formatter_declaration()],
        );
        let record =
            project_save_fidelity_lineage("lineage:lines", &review, &source_fidelity_utf8());
        let lines = save_fidelity_lineage_lines(&record);
        assert!(lines
            .iter()
            .any(|line| line.contains("Participants (run order):")));
        assert!(lines
            .iter()
            .any(|line| line.contains("Narrowed below Stable")));
    }
}
