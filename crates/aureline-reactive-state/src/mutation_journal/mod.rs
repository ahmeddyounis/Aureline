//! Mutation-journal beta projection.
//!
//! This module is the canonical loader, validator, projector, and
//! reporter for the mutation-journal beta. The beta groups material
//! multi-file or tool-driven writes into one journal entry and binds
//! each entry to:
//!
//! - one closed [`SourceLane`] (`ai_assistant`,
//!   `interactive_refactor`, `automated_tooling`, `manual_save`,
//!   `migration_replay`, `restore_pipeline`) so support and incident
//!   pipelines can attribute every grouped write to where it came
//!   from;
//! - one closed [`ActorClass`] (`human_user`, `ai_agent`,
//!   `automated_tool`, `system_service`, `unknown_actor`) and one
//!   closed [`AuthorityClass`] mirroring the reactive-state authority
//!   labels so the journal records who wrote on whose behalf and
//!   which authority owned the write;
//! - one closed [`EntryKind`] describing the shape of the write
//!   (`single_file_write`, `multi_file_write`, `directory_rename`,
//!   `metadata_write`, `derived_artifact_write`);
//! - one closed [`RecoveryClass`] stating whether recovery is
//!   `exact_undo`, `compensation`, `regeneration`,
//!   `checkpoint_restore`, or `requires_user_resolution`;
//! - one closed [`AttributionState`] (`attributed`,
//!   `partially_attributed`, `unattributed`) the evaluator
//!   cross-checks against the declared actor / authority / source
//!   lane;
//! - one closed [`ReplayabilityState`] (`replay_ready`,
//!   `replay_with_compensation`, `regenerate_only`,
//!   `requires_manual_inspection`) so support and recovery surfaces
//!   know whether the entry is replayable as-is, replayable with
//!   compensation, must be regenerated, or needs an operator;
//! - one [`SupportExportProjection`] declaring the metadata-safe
//!   baseline (no raw payload bytes, no raw private material, no
//!   ambient authority) so support and incident packets quote the
//!   audit fields without reconstructing the diff;
//! - one closed [`DowngradeLabel`] drawn from the mutation-journal
//!   vocabulary so a failing row downgrades without inventing new
//!   tokens.
//!
//! Bound to the boundary schema at
//! [`/schemas/state/mutation_journal.schema.json`](../../../../schemas/state/mutation_journal.schema.json),
//! the reviewer doc at
//! [`/docs/state/m3/mutation_journal_beta.md`](../../../../docs/state/m3/mutation_journal_beta.md),
//! and the baseline report at
//! [`/artifacts/support/m3/mutation_journal_beta_report.md`](../../../../artifacts/support/m3/mutation_journal_beta_report.md).

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for a mutation-journal case record.
pub const MUTATION_JOURNAL_CASE_RECORD_KIND: &str = "mutation_journal_case_record";

/// Stable record-kind tag for the mutation-journal report record.
pub const MUTATION_JOURNAL_REPORT_RECORD_KIND: &str = "mutation_journal_report_record";

/// Frozen schema version for the mutation-journal records.
pub const MUTATION_JOURNAL_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MUTATION_JOURNAL_SCHEMA_REF: &str = "schemas/state/mutation_journal.schema.json";

/// Repo-relative path of the reviewer doc.
pub const MUTATION_JOURNAL_DOC_REF: &str = "docs/state/m3/mutation_journal_beta.md";

/// Repo-relative path of the baseline report.
pub const MUTATION_JOURNAL_REPORT_REF: &str =
    "artifacts/support/m3/mutation_journal_beta_report.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const MUTATION_JOURNAL_CORPUS_DIR: &str = "fixtures/state/mutation_journal_beta";

/// Repo-relative path of the protected corpus manifest.
pub const MUTATION_JOURNAL_CORPUS_MANIFEST_REF: &str =
    "fixtures/state/mutation_journal_beta/manifest.yaml";

/// Closed source-lane vocabulary. Every grouped journal entry must
/// declare exactly one source lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceLane {
    AiAssistant,
    InteractiveRefactor,
    AutomatedTooling,
    ManualSave,
    MigrationReplay,
    RestorePipeline,
}

impl SourceLane {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AiAssistant => "ai_assistant",
            Self::InteractiveRefactor => "interactive_refactor",
            Self::AutomatedTooling => "automated_tooling",
            Self::ManualSave => "manual_save",
            Self::MigrationReplay => "migration_replay",
            Self::RestorePipeline => "restore_pipeline",
        }
    }
}

/// Source lanes the corpus must seed at least one case for. Mirrors
/// the spec's call-out of AI, refactor, and tooling writes.
pub const REQUIRED_SOURCE_LANES: [SourceLane; 3] = [
    SourceLane::AiAssistant,
    SourceLane::InteractiveRefactor,
    SourceLane::AutomatedTooling,
];

/// Closed actor-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActorClass {
    HumanUser,
    AiAgent,
    AutomatedTool,
    SystemService,
    UnknownActor,
}

impl ActorClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanUser => "human_user",
            Self::AiAgent => "ai_agent",
            Self::AutomatedTool => "automated_tool",
            Self::SystemService => "system_service",
            Self::UnknownActor => "unknown_actor",
        }
    }

    pub const fn is_known(self) -> bool {
        !matches!(self, Self::UnknownActor)
    }
}

/// Closed authority-class vocabulary. Mirrors the
/// `AuthorityLabel` used by the reactive-views beta so the journal
/// can record which authority owned each grouped write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityClass {
    WorkspaceVfs,
    BufferEditor,
    DerivedKnowledge,
    Execution,
    PolicyEntitlement,
    ProviderOverlay,
}

impl AuthorityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceVfs => "workspace_vfs",
            Self::BufferEditor => "buffer_editor",
            Self::DerivedKnowledge => "derived_knowledge",
            Self::Execution => "execution",
            Self::PolicyEntitlement => "policy_entitlement",
            Self::ProviderOverlay => "provider_overlay",
        }
    }
}

/// Closed entry-kind vocabulary describing the shape of one grouped
/// journal entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EntryKind {
    SingleFileWrite,
    MultiFileWrite,
    DirectoryRename,
    MetadataWrite,
    DerivedArtifactWrite,
}

impl EntryKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleFileWrite => "single_file_write",
            Self::MultiFileWrite => "multi_file_write",
            Self::DirectoryRename => "directory_rename",
            Self::MetadataWrite => "metadata_write",
            Self::DerivedArtifactWrite => "derived_artifact_write",
        }
    }
}

/// Closed recovery-class vocabulary; the journal states whether a
/// grouped entry can be recovered through exact undo, compensation,
/// regeneration, checkpoint restore, or only by user resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryClass {
    ExactUndo,
    Compensation,
    Regeneration,
    CheckpointRestore,
    RequiresUserResolution,
}

impl RecoveryClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactUndo => "exact_undo",
            Self::Compensation => "compensation",
            Self::Regeneration => "regeneration",
            Self::CheckpointRestore => "checkpoint_restore",
            Self::RequiresUserResolution => "requires_user_resolution",
        }
    }
}

/// Recovery classes the corpus must seed at least one case for.
pub const REQUIRED_RECOVERY_CLASSES: [RecoveryClass; 4] = [
    RecoveryClass::ExactUndo,
    RecoveryClass::Compensation,
    RecoveryClass::Regeneration,
    RecoveryClass::CheckpointRestore,
];

/// Closed attribution-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttributionState {
    Attributed,
    PartiallyAttributed,
    Unattributed,
}

impl AttributionState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Attributed => "attributed",
            Self::PartiallyAttributed => "partially_attributed",
            Self::Unattributed => "unattributed",
        }
    }
}

/// Closed replayability-state vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplayabilityState {
    ReplayReady,
    ReplayWithCompensation,
    RegenerateOnly,
    RequiresManualInspection,
}

impl ReplayabilityState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReplayReady => "replay_ready",
            Self::ReplayWithCompensation => "replay_with_compensation",
            Self::RegenerateOnly => "regenerate_only",
            Self::RequiresManualInspection => "requires_manual_inspection",
        }
    }
}

/// Closed consumer-surface vocabulary. Each case declares the
/// primary consumer that quotes the journal entry, and the corpus
/// must cover at least one case per required consumer surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    IncidentPacket,
    SupportBundle,
    DoctorProbe,
    CrashReport,
    RecoveryLadder,
}

impl ConsumerSurface {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IncidentPacket => "incident_packet",
            Self::SupportBundle => "support_bundle",
            Self::DoctorProbe => "doctor_probe",
            Self::CrashReport => "crash_report",
            Self::RecoveryLadder => "recovery_ladder",
        }
    }
}

/// Consumer surfaces the corpus must cover.
pub const REQUIRED_CONSUMER_SURFACES: [ConsumerSurface; 5] = [
    ConsumerSurface::IncidentPacket,
    ConsumerSurface::SupportBundle,
    ConsumerSurface::DoctorProbe,
    ConsumerSurface::CrashReport,
    ConsumerSurface::RecoveryLadder,
];

/// Closed downgrade-label vocabulary; a failing row downgrades using
/// one of these labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DowngradeLabel {
    /// No downgrade applied; the row passes outright.
    None,
    /// Red — the beta row is blocked until attribution or recovery
    /// truth is restored.
    RedBlocksBetaRow,
    /// Yellow — at least one of {actor, authority, source_lane}
    /// could not be attributed at write time.
    YellowPartialAttribution,
    /// Yellow — the journal does not know which recovery class
    /// applies and is awaiting operator resolution.
    YellowRecoveryClassUnknown,
    /// The entry degrades to checkpoint restore because finer
    /// recovery options were not preserved.
    DegradedToCheckpointRestoreOnly,
    /// The protected corpus is stale; release candidate cannot
    /// promote until it is restored.
    StaleCorpusBlocksReleaseCandidate,
}

impl DowngradeLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RedBlocksBetaRow => "red_blocks_beta_row",
            Self::YellowPartialAttribution => "yellow_partial_attribution",
            Self::YellowRecoveryClassUnknown => "yellow_recovery_class_unknown",
            Self::DegradedToCheckpointRestoreOnly => "degraded_to_checkpoint_restore_only",
            Self::StaleCorpusBlocksReleaseCandidate => "stale_corpus_blocks_release_candidate",
        }
    }

    pub const fn is_healthy(self) -> bool {
        matches!(self, Self::None)
    }
}

/// Closed open-gap class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenGapClass {
    None,
    AttributionPending,
    CompensationClassPending,
    ReplayPipelinePending,
    SupportExportPending,
}

impl OpenGapClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::AttributionPending => "attribution_pending",
            Self::CompensationClassPending => "compensation_class_pending",
            Self::ReplayPipelinePending => "replay_pipeline_pending",
            Self::SupportExportPending => "support_export_pending",
        }
    }
}

/// One open-gap row attached to a mutation-journal case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenGapEntry {
    pub gap_class: OpenGapClass,
    pub summary: String,
}

/// Metadata-safe support-export projection. The journal preserves
/// the closed-vocabulary audit fields and the canonical path refs so
/// support and incident packets can explain what changed without
/// re-reading raw diffs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportExportProjection {
    pub includes_entry_id: bool,
    pub includes_source_lane: bool,
    pub includes_actor_class: bool,
    pub includes_authority_class: bool,
    pub includes_recovery_class: bool,
    pub includes_replayability_state: bool,
    pub includes_affected_paths: bool,
    pub raw_payload_excluded: bool,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub preserves_user_authored_files: bool,
}

impl SupportExportProjection {
    pub const fn metadata_safe_baseline() -> Self {
        Self {
            includes_entry_id: true,
            includes_source_lane: true,
            includes_actor_class: true,
            includes_authority_class: true,
            includes_recovery_class: true,
            includes_replayability_state: true,
            includes_affected_paths: true,
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            preserves_user_authored_files: true,
        }
    }
}

/// Safety baseline pinned on every mutation-journal case and on the
/// report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseSafety {
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub destructive_resets_present: bool,
    pub preserves_user_authored_files: bool,
}

impl CaseSafety {
    pub const fn metadata_safe_baseline() -> Self {
        Self {
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_resets_present: false,
            preserves_user_authored_files: true,
        }
    }
}

/// Companion refs quoted on each mutation-journal case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseReferences {
    pub doc_ref: String,
    pub schema_ref: String,
    pub report_ref: String,
}

/// One mutation-journal case record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationJournalCase {
    pub schema_version: u32,
    pub record_kind: String,
    pub entry_id: String,
    pub title: String,
    pub source_lane: SourceLane,
    pub actor_class: ActorClass,
    pub authority_class: AuthorityClass,
    pub entry_kind: EntryKind,
    pub group_size: u32,
    pub affected_paths: Vec<String>,
    pub recovery_class: RecoveryClass,
    pub attribution_state: AttributionState,
    pub replayability_state: ReplayabilityState,
    pub consumer_surface: ConsumerSurface,
    pub support_export: SupportExportProjection,
    pub downgrade_label: DowngradeLabel,
    #[serde(default)]
    pub open_gaps: Vec<OpenGapEntry>,
    pub safety: CaseSafety,
    pub references: CaseReferences,
    pub captured_at: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reviewer_summary: Option<String>,
}

/// One fixture-bound entry in the corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationJournalCorpusEntry {
    pub fixture_ref: String,
    pub case: MutationJournalCase,
}

/// Mutation-journal corpus loaded from checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationJournalCorpus {
    pub entries: Vec<MutationJournalCorpusEntry>,
}

/// One row in the report matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReportMatrixRow {
    pub entry_id: String,
    pub consumer_surface: ConsumerSurface,
    pub source_lane: SourceLane,
    pub actor_class: ActorClass,
    pub authority_class: AuthorityClass,
    pub entry_kind: EntryKind,
    pub group_size: u32,
    pub affected_path_count: u32,
    pub recovery_class: RecoveryClass,
    pub attribution_state: AttributionState,
    pub replayability_state: ReplayabilityState,
    pub downgrade_label: DowngradeLabel,
    pub open_gap_classes: Vec<OpenGapClass>,
}

impl ReportMatrixRow {
    fn from_case(case: &MutationJournalCase) -> Self {
        let mut open_gap_classes: Vec<OpenGapClass> =
            case.open_gaps.iter().map(|gap| gap.gap_class).collect();
        if open_gap_classes.is_empty() {
            open_gap_classes.push(OpenGapClass::None);
        }
        Self {
            entry_id: case.entry_id.clone(),
            consumer_surface: case.consumer_surface,
            source_lane: case.source_lane,
            actor_class: case.actor_class,
            authority_class: case.authority_class,
            entry_kind: case.entry_kind,
            group_size: case.group_size,
            affected_path_count: case.affected_paths.len() as u32,
            recovery_class: case.recovery_class,
            attribution_state: case.attribution_state,
            replayability_state: case.replayability_state,
            downgrade_label: case.downgrade_label,
            open_gap_classes,
        }
    }
}

/// Per-source-lane summary row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceLaneSummaryRow {
    pub source_lane: SourceLane,
    pub case_count: u32,
    pub attributed_count: u32,
    pub partially_attributed_count: u32,
    pub unattributed_count: u32,
    pub downgrade_required_count: u32,
}

/// Per-recovery-class summary row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryClassSummaryRow {
    pub recovery_class: RecoveryClass,
    pub case_count: u32,
    pub replay_ready_count: u32,
    pub replay_with_compensation_count: u32,
    pub regenerate_only_count: u32,
    pub requires_manual_inspection_count: u32,
}

/// Metadata-safe mutation-journal report record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MutationJournalReport {
    pub schema_version: u32,
    pub record_kind: String,
    pub report_id: String,
    pub captured_at: String,
    pub doc_ref: String,
    pub schema_ref: String,
    pub corpus_manifest_ref: String,
    pub raw_payload_excluded: bool,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub required_source_lanes: Vec<SourceLane>,
    pub required_recovery_classes: Vec<RecoveryClass>,
    pub required_consumer_surfaces: Vec<ConsumerSurface>,
    pub matrix_rows: Vec<ReportMatrixRow>,
    pub source_lane_summaries: Vec<SourceLaneSummaryRow>,
    pub recovery_class_summaries: Vec<RecoveryClassSummaryRow>,
}

impl MutationJournalReport {
    pub fn is_export_safe(&self) -> bool {
        if !self.raw_payload_excluded
            || !self.raw_private_material_excluded
            || !self.ambient_authority_excluded
        {
            return false;
        }
        if self.matrix_rows.is_empty() {
            return false;
        }
        if self.source_lane_summaries.is_empty() {
            return false;
        }
        if self.recovery_class_summaries.is_empty() {
            return false;
        }
        true
    }
}

/// One validation violation emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationJournalViolation {
    pub check_id: String,
    pub subject_ref: String,
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationJournalValidationReport {
    pub violations: Vec<MutationJournalViolation>,
}

impl fmt::Display for MutationJournalValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} mutation-journal violation(s)",
            self.violations.len()
        )
    }
}

impl Error for MutationJournalValidationReport {}

/// Mutation-journal evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct MutationJournalEvaluator;

impl MutationJournalEvaluator {
    pub const fn new() -> Self {
        Self
    }

    pub fn validate_case(
        &self,
        case: &MutationJournalCase,
    ) -> Result<(), MutationJournalValidationReport> {
        let violations = validate_case(case);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(MutationJournalValidationReport { violations })
        }
    }

    pub fn validate_corpus(
        &self,
        corpus: &MutationJournalCorpus,
    ) -> Result<(), MutationJournalValidationReport> {
        let violations = validate_corpus(corpus);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(MutationJournalValidationReport { violations })
        }
    }

    pub fn report(
        &self,
        report_id: impl Into<String>,
        captured_at: impl Into<String>,
        corpus: &MutationJournalCorpus,
    ) -> Result<MutationJournalReport, MutationJournalValidationReport> {
        self.validate_corpus(corpus)?;
        let mut matrix_rows: Vec<ReportMatrixRow> = corpus
            .entries
            .iter()
            .map(|entry| ReportMatrixRow::from_case(&entry.case))
            .collect();
        matrix_rows.sort_by(|a, b| a.entry_id.cmp(&b.entry_id));

        let source_lane_summaries = REQUIRED_SOURCE_LANES
            .iter()
            .map(|lane| summarize_source_lane(corpus, *lane))
            .collect();
        let recovery_class_summaries = REQUIRED_RECOVERY_CLASSES
            .iter()
            .map(|cls| summarize_recovery_class(corpus, *cls))
            .collect();

        Ok(MutationJournalReport {
            schema_version: MUTATION_JOURNAL_SCHEMA_VERSION,
            record_kind: MUTATION_JOURNAL_REPORT_RECORD_KIND.to_owned(),
            report_id: report_id.into(),
            captured_at: captured_at.into(),
            doc_ref: MUTATION_JOURNAL_DOC_REF.to_owned(),
            schema_ref: MUTATION_JOURNAL_SCHEMA_REF.to_owned(),
            corpus_manifest_ref: MUTATION_JOURNAL_CORPUS_MANIFEST_REF.to_owned(),
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            required_source_lanes: REQUIRED_SOURCE_LANES.to_vec(),
            required_recovery_classes: REQUIRED_RECOVERY_CLASSES.to_vec(),
            required_consumer_surfaces: REQUIRED_CONSUMER_SURFACES.to_vec(),
            matrix_rows,
            source_lane_summaries,
            recovery_class_summaries,
        })
    }
}

fn summarize_source_lane(
    corpus: &MutationJournalCorpus,
    source_lane: SourceLane,
) -> SourceLaneSummaryRow {
    let mut row = SourceLaneSummaryRow {
        source_lane,
        case_count: 0,
        attributed_count: 0,
        partially_attributed_count: 0,
        unattributed_count: 0,
        downgrade_required_count: 0,
    };
    for entry in &corpus.entries {
        if entry.case.source_lane != source_lane {
            continue;
        }
        row.case_count += 1;
        match entry.case.attribution_state {
            AttributionState::Attributed => row.attributed_count += 1,
            AttributionState::PartiallyAttributed => row.partially_attributed_count += 1,
            AttributionState::Unattributed => row.unattributed_count += 1,
        }
        if !entry.case.downgrade_label.is_healthy() {
            row.downgrade_required_count += 1;
        }
    }
    row
}

fn summarize_recovery_class(
    corpus: &MutationJournalCorpus,
    recovery_class: RecoveryClass,
) -> RecoveryClassSummaryRow {
    let mut row = RecoveryClassSummaryRow {
        recovery_class,
        case_count: 0,
        replay_ready_count: 0,
        replay_with_compensation_count: 0,
        regenerate_only_count: 0,
        requires_manual_inspection_count: 0,
    };
    for entry in &corpus.entries {
        if entry.case.recovery_class != recovery_class {
            continue;
        }
        row.case_count += 1;
        match entry.case.replayability_state {
            ReplayabilityState::ReplayReady => row.replay_ready_count += 1,
            ReplayabilityState::ReplayWithCompensation => row.replay_with_compensation_count += 1,
            ReplayabilityState::RegenerateOnly => row.regenerate_only_count += 1,
            ReplayabilityState::RequiresManualInspection => {
                row.requires_manual_inspection_count += 1
            }
        }
    }
    row
}

fn validate_corpus(corpus: &MutationJournalCorpus) -> Vec<MutationJournalViolation> {
    let mut violations = Vec::new();

    if corpus.entries.is_empty() {
        push_violation(
            &mut violations,
            "corpus.empty",
            MUTATION_JOURNAL_CORPUS_DIR,
            "corpus must contain at least one mutation-journal case",
        );
        return violations;
    }

    let mut entry_ids = BTreeSet::new();
    let mut fixture_refs = BTreeSet::new();
    let mut seen_source_lanes: BTreeSet<SourceLane> = BTreeSet::new();
    let mut seen_recovery_classes: BTreeSet<RecoveryClass> = BTreeSet::new();
    let mut seen_consumer_surfaces: BTreeSet<ConsumerSurface> = BTreeSet::new();
    let mut seen_partial_attribution = false;
    let mut seen_unattributed = false;

    for entry in &corpus.entries {
        if !fixture_refs.insert(entry.fixture_ref.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_fixture_ref",
                &entry.fixture_ref,
                "fixture_ref must be unique within the corpus",
            );
        }
        let case = &entry.case;
        if !entry_ids.insert(case.entry_id.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_entry_id",
                &case.entry_id,
                "entry_id must be unique within the corpus",
            );
        }
        seen_source_lanes.insert(case.source_lane);
        seen_recovery_classes.insert(case.recovery_class);
        seen_consumer_surfaces.insert(case.consumer_surface);
        match case.attribution_state {
            AttributionState::PartiallyAttributed => seen_partial_attribution = true,
            AttributionState::Unattributed => seen_unattributed = true,
            AttributionState::Attributed => {}
        }
        violations.extend(validate_case(case));
    }

    for lane in REQUIRED_SOURCE_LANES {
        if !seen_source_lanes.contains(&lane) {
            push_violation(
                &mut violations,
                "corpus.required_source_lane_missing",
                lane.as_str(),
                format!(
                    "corpus must seed at least one case for source_lane = {}",
                    lane.as_str()
                ),
            );
        }
    }
    for cls in REQUIRED_RECOVERY_CLASSES {
        if !seen_recovery_classes.contains(&cls) {
            push_violation(
                &mut violations,
                "corpus.required_recovery_class_missing",
                cls.as_str(),
                format!(
                    "corpus must seed at least one case for recovery_class = {}",
                    cls.as_str()
                ),
            );
        }
    }
    for surface in REQUIRED_CONSUMER_SURFACES {
        if !seen_consumer_surfaces.contains(&surface) {
            push_violation(
                &mut violations,
                "corpus.required_consumer_surface_missing",
                surface.as_str(),
                format!(
                    "corpus must seed at least one case for consumer_surface = {}",
                    surface.as_str()
                ),
            );
        }
    }
    if !seen_partial_attribution {
        push_violation(
            &mut violations,
            "corpus.partially_attributed_case_missing",
            MUTATION_JOURNAL_CORPUS_DIR,
            "corpus must seed at least one case with attribution_state = partially_attributed",
        );
    }
    if !seen_unattributed {
        push_violation(
            &mut violations,
            "corpus.unattributed_case_missing",
            MUTATION_JOURNAL_CORPUS_DIR,
            "corpus must seed at least one case with attribution_state = unattributed",
        );
    }

    violations
}

fn validate_case(case: &MutationJournalCase) -> Vec<MutationJournalViolation> {
    let mut violations = Vec::new();
    let target = case.entry_id.as_str();

    if case.schema_version != MUTATION_JOURNAL_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "case.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if case.record_kind != MUTATION_JOURNAL_CASE_RECORD_KIND {
        push_violation(
            &mut violations,
            "case.record_kind",
            target,
            format!("record_kind must be {MUTATION_JOURNAL_CASE_RECORD_KIND}"),
        );
    }
    if case.entry_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "case.entry_id",
            target,
            "entry_id must be non-empty",
        );
    }
    if case.title.trim().is_empty() {
        push_violation(
            &mut violations,
            "case.title",
            target,
            "title must be non-empty",
        );
    }
    if case.captured_at.trim().is_empty() {
        push_violation(
            &mut violations,
            "case.captured_at",
            target,
            "captured_at must be non-empty",
        );
    }

    validate_group(&mut violations, target, case);
    validate_attribution(&mut violations, target, case);
    validate_replayability(&mut violations, target, case);
    validate_outcome_and_downgrade(&mut violations, target, case);
    validate_support_export(&mut violations, target, &case.support_export);
    validate_open_gaps(&mut violations, target, &case.open_gaps);
    validate_safety(&mut violations, target, &case.safety);
    validate_references(&mut violations, target, &case.references);

    violations
}

fn validate_group(
    violations: &mut Vec<MutationJournalViolation>,
    target: &str,
    case: &MutationJournalCase,
) {
    if case.group_size == 0 {
        push_violation(
            violations,
            "case.group.size_zero",
            target,
            "group_size must be at least 1",
        );
    }
    if case.affected_paths.is_empty() {
        push_violation(
            violations,
            "case.group.affected_paths_empty",
            target,
            "affected_paths must list at least one repo-relative path",
        );
    } else {
        let mut seen: BTreeSet<&String> = BTreeSet::new();
        for path in &case.affected_paths {
            if path.trim().is_empty() {
                push_violation(
                    violations,
                    "case.group.affected_path_empty",
                    target,
                    "affected_paths entries must be non-empty",
                );
            }
            if !seen.insert(path) {
                push_violation(
                    violations,
                    "case.group.affected_path_duplicate",
                    target,
                    format!("affected_paths entries must be unique (duplicate: {path})"),
                );
            }
        }
    }
    match case.entry_kind {
        EntryKind::SingleFileWrite => {
            if case.group_size != 1 {
                push_violation(
                    violations,
                    "case.group.single_file_group_size",
                    target,
                    "entry_kind = single_file_write requires group_size = 1",
                );
            }
            if case.affected_paths.len() != 1 {
                push_violation(
                    violations,
                    "case.group.single_file_path_count",
                    target,
                    "entry_kind = single_file_write requires exactly one affected_path",
                );
            }
        }
        EntryKind::MultiFileWrite => {
            if case.group_size < 2 {
                push_violation(
                    violations,
                    "case.group.multi_file_group_size",
                    target,
                    "entry_kind = multi_file_write requires group_size >= 2",
                );
            }
            if case.affected_paths.len() < 2 {
                push_violation(
                    violations,
                    "case.group.multi_file_path_count",
                    target,
                    "entry_kind = multi_file_write requires at least two affected_paths",
                );
            }
        }
        EntryKind::DirectoryRename
        | EntryKind::MetadataWrite
        | EntryKind::DerivedArtifactWrite => {}
    }
}

fn validate_attribution(
    violations: &mut Vec<MutationJournalViolation>,
    target: &str,
    case: &MutationJournalCase,
) {
    match case.attribution_state {
        AttributionState::Attributed => {
            if !case.actor_class.is_known() {
                push_violation(
                    violations,
                    "case.attribution.attributed_requires_known_actor",
                    target,
                    "attribution_state = attributed requires actor_class != unknown_actor",
                );
            }
        }
        AttributionState::PartiallyAttributed => {
            // The partial state is informational; we just require an
            // open_gap entry below.
        }
        AttributionState::Unattributed => {
            if case.actor_class.is_known() {
                push_violation(
                    violations,
                    "case.attribution.unattributed_requires_unknown_actor",
                    target,
                    "attribution_state = unattributed requires actor_class = unknown_actor",
                );
            }
        }
    }
}

fn validate_replayability(
    violations: &mut Vec<MutationJournalViolation>,
    target: &str,
    case: &MutationJournalCase,
) {
    match case.replayability_state {
        ReplayabilityState::ReplayReady => {
            if case.attribution_state != AttributionState::Attributed {
                push_violation(
                    violations,
                    "case.replayability.replay_ready_requires_attributed",
                    target,
                    "replayability_state = replay_ready requires attribution_state = attributed",
                );
            }
            if matches!(
                case.recovery_class,
                RecoveryClass::CheckpointRestore | RecoveryClass::RequiresUserResolution
            ) {
                push_violation(
                    violations,
                    "case.replayability.replay_ready_recovery_class",
                    target,
                    "replayability_state = replay_ready requires recovery_class in {exact_undo, compensation, regeneration}",
                );
            }
        }
        ReplayabilityState::ReplayWithCompensation => {
            if case.recovery_class != RecoveryClass::Compensation {
                push_violation(
                    violations,
                    "case.replayability.replay_with_compensation_requires_compensation",
                    target,
                    "replayability_state = replay_with_compensation requires recovery_class = compensation",
                );
            }
            if case.attribution_state != AttributionState::Attributed {
                push_violation(
                    violations,
                    "case.replayability.replay_with_compensation_requires_attributed",
                    target,
                    "replayability_state = replay_with_compensation requires attribution_state = attributed",
                );
            }
        }
        ReplayabilityState::RegenerateOnly => {
            if case.recovery_class != RecoveryClass::Regeneration {
                push_violation(
                    violations,
                    "case.replayability.regenerate_only_requires_regeneration",
                    target,
                    "replayability_state = regenerate_only requires recovery_class = regeneration",
                );
            }
        }
        ReplayabilityState::RequiresManualInspection => {
            let recovery_ok = matches!(
                case.recovery_class,
                RecoveryClass::CheckpointRestore | RecoveryClass::RequiresUserResolution
            );
            let attribution_ok = matches!(
                case.attribution_state,
                AttributionState::PartiallyAttributed | AttributionState::Unattributed
            );
            if !recovery_ok && !attribution_ok {
                push_violation(
                    violations,
                    "case.replayability.requires_manual_inspection_signal",
                    target,
                    "replayability_state = requires_manual_inspection requires recovery_class in {checkpoint_restore, requires_user_resolution} or attribution_state in {partially_attributed, unattributed}",
                );
            }
        }
    }
}

fn validate_outcome_and_downgrade(
    violations: &mut Vec<MutationJournalViolation>,
    target: &str,
    case: &MutationJournalCase,
) {
    let aligned_pass = case.attribution_state == AttributionState::Attributed
        && matches!(
            case.recovery_class,
            RecoveryClass::ExactUndo | RecoveryClass::Compensation | RecoveryClass::Regeneration
        )
        && matches!(
            case.replayability_state,
            ReplayabilityState::ReplayReady
                | ReplayabilityState::ReplayWithCompensation
                | ReplayabilityState::RegenerateOnly
        );

    match case.downgrade_label {
        DowngradeLabel::None => {
            if !aligned_pass {
                push_violation(
                    violations,
                    "case.outcome.aligned_requires_clean_triple",
                    target,
                    "downgrade_label = none requires attribution_state = attributed, recovery_class in {exact_undo, compensation, regeneration}, and replayability_state != requires_manual_inspection",
                );
            }
            if case
                .open_gaps
                .iter()
                .any(|gap| gap.gap_class != OpenGapClass::None)
            {
                push_violation(
                    violations,
                    "case.outcome.aligned_must_not_record_open_gap",
                    target,
                    "aligned rows must not declare any open_gap with a non-none gap_class",
                );
            }
        }
        DowngradeLabel::RedBlocksBetaRow => {
            if case.attribution_state != AttributionState::Unattributed {
                push_violation(
                    violations,
                    "case.outcome.red_requires_unattributed",
                    target,
                    "downgrade_label = red_blocks_beta_row requires attribution_state = unattributed",
                );
            }
        }
        DowngradeLabel::YellowPartialAttribution => {
            if case.attribution_state != AttributionState::PartiallyAttributed {
                push_violation(
                    violations,
                    "case.outcome.yellow_partial_requires_partial",
                    target,
                    "downgrade_label = yellow_partial_attribution requires attribution_state = partially_attributed",
                );
            }
        }
        DowngradeLabel::YellowRecoveryClassUnknown => {
            if case.recovery_class != RecoveryClass::RequiresUserResolution {
                push_violation(
                    violations,
                    "case.outcome.yellow_recovery_unknown_requires_user_resolution",
                    target,
                    "downgrade_label = yellow_recovery_class_unknown requires recovery_class = requires_user_resolution",
                );
            }
        }
        DowngradeLabel::DegradedToCheckpointRestoreOnly => {
            if case.recovery_class != RecoveryClass::CheckpointRestore {
                push_violation(
                    violations,
                    "case.outcome.degraded_checkpoint_requires_checkpoint",
                    target,
                    "downgrade_label = degraded_to_checkpoint_restore_only requires recovery_class = checkpoint_restore",
                );
            }
        }
        DowngradeLabel::StaleCorpusBlocksReleaseCandidate => {}
    }

    if !case.downgrade_label.is_healthy() {
        let has_open_gap = case
            .open_gaps
            .iter()
            .any(|gap| gap.gap_class != OpenGapClass::None);
        if !has_open_gap {
            push_violation(
                violations,
                "case.outcome.non_aligned_must_record_open_gap",
                target,
                "downgraded rows must record at least one open_gap with a non-none gap_class",
            );
        }
    }
}

fn validate_support_export(
    violations: &mut Vec<MutationJournalViolation>,
    target: &str,
    support: &SupportExportProjection,
) {
    if !support.raw_payload_excluded {
        push_violation(
            violations,
            "case.support_export.raw_payload_excluded",
            target,
            "support_export.raw_payload_excluded must be true",
        );
    }
    if !support.raw_private_material_excluded {
        push_violation(
            violations,
            "case.support_export.raw_private_material_excluded",
            target,
            "support_export.raw_private_material_excluded must be true",
        );
    }
    if !support.ambient_authority_excluded {
        push_violation(
            violations,
            "case.support_export.ambient_authority_excluded",
            target,
            "support_export.ambient_authority_excluded must be true",
        );
    }
    if !support.preserves_user_authored_files {
        push_violation(
            violations,
            "case.support_export.preserves_user_authored_files",
            target,
            "support_export.preserves_user_authored_files must be true",
        );
    }
    if !(support.includes_entry_id
        && support.includes_source_lane
        && support.includes_actor_class
        && support.includes_authority_class
        && support.includes_recovery_class
        && support.includes_replayability_state
        && support.includes_affected_paths)
    {
        push_violation(
            violations,
            "case.support_export.must_preserve_audit_fields",
            target,
            "support_export must include entry_id, source_lane, actor_class, authority_class, recovery_class, replayability_state, and affected_paths so support and incident packets can explain what changed",
        );
    }
}

fn validate_open_gaps(
    violations: &mut Vec<MutationJournalViolation>,
    target: &str,
    gaps: &[OpenGapEntry],
) {
    let mut seen = BTreeSet::new();
    for gap in gaps {
        if gap.summary.trim().is_empty() {
            push_violation(
                violations,
                "case.open_gaps.summary",
                target,
                "open_gaps.summary must be non-empty",
            );
        }
        if !seen.insert(gap.gap_class) {
            push_violation(
                violations,
                "case.open_gaps.duplicate_gap_class",
                target,
                format!("duplicate open_gap_class {}", gap.gap_class.as_str()),
            );
        }
    }
}

fn validate_safety(
    violations: &mut Vec<MutationJournalViolation>,
    target: &str,
    safety: &CaseSafety,
) {
    if !safety.raw_private_material_excluded {
        push_violation(
            violations,
            "case.safety.raw_private_material_excluded",
            target,
            "raw_private_material_excluded must be true",
        );
    }
    if !safety.ambient_authority_excluded {
        push_violation(
            violations,
            "case.safety.ambient_authority_excluded",
            target,
            "ambient_authority_excluded must be true",
        );
    }
    if safety.destructive_resets_present {
        push_violation(
            violations,
            "case.safety.destructive_resets_present",
            target,
            "destructive_resets_present must be false",
        );
    }
    if !safety.preserves_user_authored_files {
        push_violation(
            violations,
            "case.safety.preserves_user_authored_files",
            target,
            "preserves_user_authored_files must be true",
        );
    }
}

fn validate_references(
    violations: &mut Vec<MutationJournalViolation>,
    target: &str,
    refs: &CaseReferences,
) {
    if refs.doc_ref != MUTATION_JOURNAL_DOC_REF {
        push_violation(
            violations,
            "case.references.doc_ref",
            target,
            format!("references.doc_ref must pin {MUTATION_JOURNAL_DOC_REF}"),
        );
    }
    if refs.schema_ref != MUTATION_JOURNAL_SCHEMA_REF {
        push_violation(
            violations,
            "case.references.schema_ref",
            target,
            format!("references.schema_ref must pin {MUTATION_JOURNAL_SCHEMA_REF}"),
        );
    }
    if refs.report_ref != MUTATION_JOURNAL_REPORT_REF {
        push_violation(
            violations,
            "case.references.report_ref",
            target,
            format!("references.report_ref must pin {MUTATION_JOURNAL_REPORT_REF}"),
        );
    }
}

fn push_violation(
    violations: &mut Vec<MutationJournalViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(MutationJournalViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

/// Loads a YAML-encoded [`MutationJournalCase`].
pub fn load_mutation_journal_case(yaml: &str) -> Result<MutationJournalCase, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Returns the checked-in mutation-journal beta corpus.
pub fn current_mutation_journal_corpus() -> Result<MutationJournalCorpus, serde_yaml::Error> {
    let entries = CASE_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<MutationJournalCase>(yaml).map(|case| {
                MutationJournalCorpusEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    case,
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(MutationJournalCorpus { entries })
}

/// Returns the set of fixture refs the corpus loads, in declaration
/// order.
pub fn current_mutation_journal_fixture_refs() -> impl Iterator<Item = &'static str> {
    CASE_FIXTURES.iter().map(|(fixture_ref, _)| *fixture_ref)
}

const CASE_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/state/mutation_journal_beta/ai_multifile_extract_method_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/mutation_journal_beta/ai_multifile_extract_method_case.yaml"
        )),
    ),
    (
        "fixtures/state/mutation_journal_beta/interactive_refactor_rename_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/mutation_journal_beta/interactive_refactor_rename_case.yaml"
        )),
    ),
    (
        "fixtures/state/mutation_journal_beta/automated_formatter_pass_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/mutation_journal_beta/automated_formatter_pass_case.yaml"
        )),
    ),
    (
        "fixtures/state/mutation_journal_beta/ai_bulk_paste_checkpoint_restore_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/mutation_journal_beta/ai_bulk_paste_checkpoint_restore_case.yaml"
        )),
    ),
    (
        "fixtures/state/mutation_journal_beta/ai_partial_attribution_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/mutation_journal_beta/ai_partial_attribution_case.yaml"
        )),
    ),
    (
        "fixtures/state/mutation_journal_beta/tooling_unattributed_case.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/mutation_journal_beta/tooling_unattributed_case.yaml"
        )),
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn aligned_case() -> MutationJournalCase {
        MutationJournalCase {
            schema_version: MUTATION_JOURNAL_SCHEMA_VERSION,
            record_kind: MUTATION_JOURNAL_CASE_RECORD_KIND.to_owned(),
            entry_id: "journal:test:aligned".to_owned(),
            title: "Aligned test entry".to_owned(),
            source_lane: SourceLane::AiAssistant,
            actor_class: ActorClass::AiAgent,
            authority_class: AuthorityClass::BufferEditor,
            entry_kind: EntryKind::MultiFileWrite,
            group_size: 3,
            affected_paths: vec![
                "src/a.rs".to_owned(),
                "src/b.rs".to_owned(),
                "src/c.rs".to_owned(),
            ],
            recovery_class: RecoveryClass::ExactUndo,
            attribution_state: AttributionState::Attributed,
            replayability_state: ReplayabilityState::ReplayReady,
            consumer_surface: ConsumerSurface::SupportBundle,
            support_export: SupportExportProjection::metadata_safe_baseline(),
            downgrade_label: DowngradeLabel::None,
            open_gaps: vec![],
            safety: CaseSafety::metadata_safe_baseline(),
            references: CaseReferences {
                doc_ref: MUTATION_JOURNAL_DOC_REF.to_owned(),
                schema_ref: MUTATION_JOURNAL_SCHEMA_REF.to_owned(),
                report_ref: MUTATION_JOURNAL_REPORT_REF.to_owned(),
            },
            captured_at: "2026-05-16T00:00:00Z".to_owned(),
            reviewer_summary: None,
        }
    }

    #[test]
    fn aligned_case_validates() {
        MutationJournalEvaluator::new()
            .validate_case(&aligned_case())
            .expect("aligned test case must validate");
    }

    #[test]
    fn refuses_aligned_with_downgrade_label() {
        let mut case = aligned_case();
        case.downgrade_label = DowngradeLabel::YellowPartialAttribution;
        let err = MutationJournalEvaluator::new()
            .validate_case(&case)
            .expect_err("aligned with downgrade must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.outcome.yellow_partial_requires_partial"));
    }

    #[test]
    fn refuses_replay_ready_with_checkpoint_restore() {
        let mut case = aligned_case();
        case.recovery_class = RecoveryClass::CheckpointRestore;
        case.downgrade_label = DowngradeLabel::DegradedToCheckpointRestoreOnly;
        case.replayability_state = ReplayabilityState::ReplayReady;
        case.open_gaps.push(OpenGapEntry {
            gap_class: OpenGapClass::CompensationClassPending,
            summary: "compensation pending".into(),
        });
        let err = MutationJournalEvaluator::new()
            .validate_case(&case)
            .expect_err("replay_ready with checkpoint_restore must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.replayability.replay_ready_recovery_class"));
    }

    #[test]
    fn refuses_unattributed_with_known_actor() {
        let mut case = aligned_case();
        case.attribution_state = AttributionState::Unattributed;
        case.downgrade_label = DowngradeLabel::RedBlocksBetaRow;
        case.replayability_state = ReplayabilityState::RequiresManualInspection;
        case.open_gaps.push(OpenGapEntry {
            gap_class: OpenGapClass::AttributionPending,
            summary: "needs attribution".into(),
        });
        let err = MutationJournalEvaluator::new()
            .validate_case(&case)
            .expect_err("unattributed with known actor must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.attribution.unattributed_requires_unknown_actor"));
    }

    #[test]
    fn refuses_multi_file_with_single_path() {
        let mut case = aligned_case();
        case.affected_paths = vec!["only/one.rs".to_owned()];
        case.group_size = 1;
        let err = MutationJournalEvaluator::new()
            .validate_case(&case)
            .expect_err("multi_file with one path must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.group.multi_file_path_count"));
    }

    #[test]
    fn refuses_destructive_reset() {
        let mut case = aligned_case();
        case.safety.destructive_resets_present = true;
        let err = MutationJournalEvaluator::new()
            .validate_case(&case)
            .expect_err("destructive reset must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.safety.destructive_resets_present"));
    }

    #[test]
    fn refuses_support_export_dropping_audit_fields() {
        let mut case = aligned_case();
        case.support_export.includes_recovery_class = false;
        let err = MutationJournalEvaluator::new()
            .validate_case(&case)
            .expect_err("dropped audit fields must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "case.support_export.must_preserve_audit_fields"));
    }

    #[test]
    fn checked_in_corpus_loads_and_validates() {
        let corpus = current_mutation_journal_corpus().expect("checked-in corpus must parse");
        MutationJournalEvaluator::new()
            .validate_corpus(&corpus)
            .expect("checked-in corpus must validate");
        for lane in REQUIRED_SOURCE_LANES {
            assert!(
                corpus
                    .entries
                    .iter()
                    .any(|entry| entry.case.source_lane == lane),
                "checked-in corpus must seed a case for source_lane = {}",
                lane.as_str()
            );
        }
        for cls in REQUIRED_RECOVERY_CLASSES {
            assert!(
                corpus
                    .entries
                    .iter()
                    .any(|entry| entry.case.recovery_class == cls),
                "checked-in corpus must seed a case for recovery_class = {}",
                cls.as_str()
            );
        }
    }

    #[test]
    fn checked_in_report_is_export_safe() {
        let corpus = current_mutation_journal_corpus().unwrap();
        let report = MutationJournalEvaluator::new()
            .report("report:test", "2026-05-16T00:00:00Z", &corpus)
            .expect("report must build");
        assert!(report.is_export_safe());
        assert_eq!(report.matrix_rows.len(), corpus.entries.len());
        assert_eq!(
            report.source_lane_summaries.len(),
            REQUIRED_SOURCE_LANES.len()
        );
        assert_eq!(
            report.recovery_class_summaries.len(),
            REQUIRED_RECOVERY_CLASSES.len()
        );
    }
}
