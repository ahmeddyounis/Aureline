//! Compare, restore, and export timeline projection for local history.
//!
//! The projection keeps local-history compare and restore surfaces honest
//! about what can be restored exactly, what is compatible, what is layout
//! only, and what is evidence only. Support exports consume the same enum
//! values so timeline cards and exported packets do not drift apart.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::ActorLineageClass;

/// Stable record-kind tag for a local-history timeline case fixture.
pub const LOCAL_HISTORY_TIMELINE_CASE_RECORD_KIND: &str = "local_history_timeline_alpha_case";

/// Stable record-kind tag for a local-history timeline packet.
pub const LOCAL_HISTORY_TIMELINE_PACKET_RECORD_KIND: &str = "local_history_timeline_alpha_packet";

/// Stable record-kind tag for a local-history timeline report.
pub const LOCAL_HISTORY_TIMELINE_REPORT_RECORD_KIND: &str = "local_history_timeline_alpha_report";

/// Frozen schema version for the timeline alpha records.
pub const LOCAL_HISTORY_TIMELINE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the timeline alpha boundary schema.
pub const LOCAL_HISTORY_TIMELINE_SCHEMA_REF: &str =
    "schemas/state/local_history_timeline_alpha.schema.json";

/// Repo-relative path of the reviewer document.
pub const LOCAL_HISTORY_TIMELINE_DOC_REF: &str = "docs/state/m3/local_history_timeline_alpha.md";

/// Repo-relative path of the baseline support report.
pub const LOCAL_HISTORY_TIMELINE_REPORT_REF: &str =
    "artifacts/support/m3/local_history_timeline_alpha_report.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const LOCAL_HISTORY_TIMELINE_CORPUS_DIR: &str = "fixtures/recovery/m3/local_history_timeline";

/// Repo-relative path of the protected fixture corpus manifest.
pub const LOCAL_HISTORY_TIMELINE_CORPUS_MANIFEST_REF: &str =
    "fixtures/recovery/m3/local_history_timeline/manifest.yaml";

/// Fidelity labels that every protected timeline corpus must cover.
pub const REQUIRED_TIMELINE_FIDELITY_LABELS: [LocalHistoryTimelineFidelityLabel; 4] = [
    LocalHistoryTimelineFidelityLabel::Exact,
    LocalHistoryTimelineFidelityLabel::Compatible,
    LocalHistoryTimelineFidelityLabel::LayoutOnly,
    LocalHistoryTimelineFidelityLabel::EvidenceOnly,
];

/// Action classes that every timeline row must expose.
pub const REQUIRED_TIMELINE_ACTION_CLASSES: [LocalHistoryTimelineActionClass; 3] = [
    LocalHistoryTimelineActionClass::Compare,
    LocalHistoryTimelineActionClass::Restore,
    LocalHistoryTimelineActionClass::Export,
];

/// Closed fidelity-label vocabulary for local-history compare, restore, and export rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryTimelineFidelityLabel {
    /// The current target matches the captured object and bytes can be restored exactly.
    Exact,
    /// The target is compatible, but restore requires review because identity or schema changed.
    Compatible,
    /// Only layout or placement state can be restored; live execution is not resumed.
    LayoutOnly,
    /// Only evidence can be reopened or exported; no restore action may claim live state.
    EvidenceOnly,
}

impl LocalHistoryTimelineFidelityLabel {
    /// Returns the stable schema token for this fidelity label.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Compatible => "compatible",
            Self::LayoutOnly => "layout_only",
            Self::EvidenceOnly => "evidence_only",
        }
    }

    /// Returns the user-facing label paired with the schema token.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::Exact => "Exact",
            Self::Compatible => "Compatible",
            Self::LayoutOnly => "Layout only",
            Self::EvidenceOnly => "Evidence only",
        }
    }
}

/// Restore-level vocabulary paired with the timeline fidelity label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryTimelineRestoreLevel {
    /// Exact restore can write the captured bytes against the same object identity.
    ExactRestore,
    /// Compatible restore can write after review against a compatible target.
    CompatibleRestore,
    /// Layout-only restore can rehydrate placement or pane state without live sessions.
    LayoutOnly,
    /// Evidence-only recovery can reopen metadata, logs, or compare artifacts only.
    EvidenceOnly,
}

impl LocalHistoryTimelineRestoreLevel {
    /// Returns the stable schema token for this restore level.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactRestore => "exact_restore",
            Self::CompatibleRestore => "compatible_restore",
            Self::LayoutOnly => "layout_only",
            Self::EvidenceOnly => "evidence_only",
        }
    }
}

/// Surface producing or consuming the local-history timeline projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryTimelineConsumerSurface {
    /// In-product local-history timeline.
    TimelineUi,
    /// Restore preview or recovery card.
    RestorePreview,
    /// Metadata-safe support export.
    SupportExport,
    /// Headless command or fixture runner.
    CliHeadless,
}

impl LocalHistoryTimelineConsumerSurface {
    /// Returns the stable schema token for this consumer surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TimelineUi => "timeline_ui",
            Self::RestorePreview => "restore_preview",
            Self::SupportExport => "support_export",
            Self::CliHeadless => "cli_headless",
        }
    }
}

/// Action class rendered on a local-history timeline row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryTimelineActionClass {
    /// Open a compare or diff view for this checkpoint.
    Compare,
    /// Restore from this checkpoint when permitted.
    Restore,
    /// Export this checkpoint as patch or evidence.
    Export,
}

impl LocalHistoryTimelineActionClass {
    /// Returns the stable schema token for this action class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Compare => "compare",
            Self::Restore => "restore",
            Self::Export => "export",
        }
    }
}

/// Availability state for one timeline action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryTimelineActionAvailability {
    /// Action is available without an extra review step.
    Available,
    /// Action is available only after a review or confirmation surface.
    ReviewRequired,
    /// Restore is disabled; compare remains available.
    DisabledCompareOnly,
    /// Restore is disabled; export/evidence remains available.
    DisabledExportOnly,
}

impl LocalHistoryTimelineActionAvailability {
    /// Returns true when this state can write a restore checkpoint.
    pub const fn can_restore(self) -> bool {
        matches!(self, Self::Available | Self::ReviewRequired)
    }
}

/// Target posture used to derive the timeline fidelity label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryTimelineTargetPosture {
    /// Current target is the same canonical object captured by local history.
    SameObjectCurrent,
    /// Current target is the same logical document but requires compatible review.
    CompatibleLogicalDocument,
    /// Only layout or placement metadata is available.
    LayoutMetadataOnly,
    /// Only evidence metadata is available.
    EvidenceMetadataOnly,
}

/// Compare basis used by the row before a restore or export action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryTimelineCompareBasis {
    /// Byte snapshot comparison is available.
    ByteSnapshot,
    /// A compatible schema or object comparison is available.
    CompatibleSnapshot,
    /// Layout topology comparison is available.
    LayoutTopology,
    /// Evidence-manifest comparison is available.
    EvidenceManifest,
}

/// Resumption posture shown when a restore opens a non-file surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalHistoryTimelineResumptionPosture {
    /// The row concerns only file or metadata restore.
    NotApplicable,
    /// Context can be restored, but live execution is not resumed.
    ContextRestoredNotLive,
    /// Only static evidence is reopened.
    StaticEvidenceOnly,
    /// Authority was verified without hidden rerun or privilege reuse.
    AuthorityVerifiedNoRerun,
}

/// Metadata-safe support projection carried on each timeline row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineSupportExportProjection {
    /// True when support export includes the timeline row id.
    pub includes_row_id: bool,
    /// True when support export includes source checkpoint refs.
    pub includes_checkpoint_refs: bool,
    /// True when support export includes the fidelity label.
    pub includes_fidelity_label: bool,
    /// True when support export includes compare, restore, and export actions.
    pub includes_action_vocabulary: bool,
    /// True when raw local-history payloads are excluded.
    pub raw_payload_excluded: bool,
    /// True when private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when live authority or privilege handles are excluded.
    pub live_authority_excluded: bool,
}

impl LocalHistoryTimelineSupportExportProjection {
    /// Returns the metadata-safe support/export baseline.
    pub const fn metadata_safe_baseline() -> Self {
        Self {
            includes_row_id: true,
            includes_checkpoint_refs: true,
            includes_fidelity_label: true,
            includes_action_vocabulary: true,
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            live_authority_excluded: true,
        }
    }

    /// Returns true when the projection can cross a support-export boundary.
    pub const fn is_export_safe(&self) -> bool {
        self.includes_row_id
            && self.includes_checkpoint_refs
            && self.includes_fidelity_label
            && self.includes_action_vocabulary
            && self.raw_payload_excluded
            && self.raw_private_material_excluded
            && self.live_authority_excluded
    }
}

/// No-rerun guard rendered by restore and support surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineNoRerunGuard {
    /// Visible posture for live-session or evidence recovery.
    pub resumption_posture: LocalHistoryTimelineResumptionPosture,
    /// True only when a live session actually survived or was explicitly reconnected.
    pub live_session_resumed: bool,
    /// True only when a privileged run actually survived under current authority.
    pub privileged_run_resumed: bool,
    /// True when the UI must show a rerun-required label.
    pub rerun_required_label_visible: bool,
    /// True when the UI must show an evidence-only label.
    pub evidence_only_label_visible: bool,
    /// Redaction-aware explanation for the row.
    pub summary: String,
}

/// One compare, restore, or export action on a timeline row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineAction {
    /// Action class rendered on the row.
    pub action_class: LocalHistoryTimelineActionClass,
    /// Canonical command id for invoking the action.
    pub command_id: String,
    /// Availability class for the action.
    pub availability_class: LocalHistoryTimelineActionAvailability,
    /// Fidelity label shown beside the action.
    pub fidelity_label: LocalHistoryTimelineFidelityLabel,
    /// Restore level paired with the action label.
    pub restore_level: LocalHistoryTimelineRestoreLevel,
    /// True when invoking this action writes a new local-history checkpoint.
    pub writes_new_checkpoint: bool,
    /// Optional support-export row ref for this action.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_export_ref: Option<String>,
}

/// One visible local-history timeline row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineRow {
    /// Stable row id for timeline and support joins.
    pub row_id: String,
    /// Compact, redaction-aware label.
    pub display_label: String,
    /// Surface this row is projected for.
    pub consumer_surface: LocalHistoryTimelineConsumerSurface,
    /// Source local-history entry ref.
    pub source_entry_ref: String,
    /// Optional local-history group ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_group_ref: Option<String>,
    /// Optional mutation-journal ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mutation_journal_ref: Option<String>,
    /// Actor lineage class inherited from the local-history row.
    pub actor_lineage_class: ActorLineageClass,
    /// Snapshot class token inherited from the local-history row.
    pub snapshot_class: String,
    /// Target posture used to derive the fidelity label.
    pub target_posture: LocalHistoryTimelineTargetPosture,
    /// Compare basis available to the user.
    pub compare_basis: LocalHistoryTimelineCompareBasis,
    /// Shared fidelity label for the row and all visible actions.
    pub fidelity_label: LocalHistoryTimelineFidelityLabel,
    /// Restore level paired with the fidelity label.
    pub restore_level: LocalHistoryTimelineRestoreLevel,
    /// True when the captured body is available locally.
    pub body_available_locally: bool,
    /// New checkpoint ref that restore will write when restore is available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_checkpoint_on_restore_ref: Option<String>,
    /// Visible compare, restore, and export actions.
    pub actions: Vec<LocalHistoryTimelineAction>,
    /// Guard that prevents hidden rerun or privilege-resume claims.
    pub no_rerun_guard: LocalHistoryTimelineNoRerunGuard,
    /// Metadata-safe support-export projection for this row.
    pub support_export: LocalHistoryTimelineSupportExportProjection,
}

/// One fixture case in the protected timeline corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineCase {
    /// Integer schema version for the case.
    pub schema_version: u32,
    /// Stable discriminator for the case.
    pub record_kind: String,
    /// Opaque case id.
    pub case_id: String,
    /// Human-readable case title.
    pub title: String,
    /// The row under test.
    pub timeline_row: LocalHistoryTimelineRow,
    /// Fixture and artifact refs this case binds to.
    pub references: LocalHistoryTimelineReferences,
    /// Timestamp for the fixture case.
    pub captured_at: String,
}

/// Companion refs quoted by each timeline case.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineReferences {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Baseline report ref.
    pub report_ref: String,
}

/// One fixture-bound entry in the local-history timeline corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineCorpusEntry {
    /// Repo-relative fixture path.
    pub fixture_ref: String,
    /// Parsed case.
    pub case: LocalHistoryTimelineCase,
}

/// Checked-in timeline fixture corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineCorpus {
    /// Corpus entries loaded from fixtures.
    pub entries: Vec<LocalHistoryTimelineCorpusEntry>,
}

/// Packet consumed by timeline, restore-preview, support, and headless surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineAlphaPacket {
    /// Stable discriminator for this packet.
    pub record_kind: String,
    /// Integer schema version for this alpha packet.
    pub schema_version: u32,
    /// Opaque packet id.
    pub packet_id: String,
    /// Producer timestamp for this projection.
    pub produced_at: String,
    /// Consumer surface for this projection.
    pub consumer_surface: LocalHistoryTimelineConsumerSurface,
    /// Timeline rows included in the packet.
    pub rows: Vec<LocalHistoryTimelineRow>,
    /// True when raw local-history payloads are excluded.
    pub raw_payload_excluded: bool,
    /// True when private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when live authority or privilege handles are excluded.
    pub live_authority_excluded: bool,
}

impl LocalHistoryTimelineAlphaPacket {
    /// Creates a timeline packet from already-validated rows.
    pub fn new(
        packet_id: impl Into<String>,
        produced_at: impl Into<String>,
        consumer_surface: LocalHistoryTimelineConsumerSurface,
        rows: Vec<LocalHistoryTimelineRow>,
    ) -> Self {
        Self {
            record_kind: LOCAL_HISTORY_TIMELINE_PACKET_RECORD_KIND.to_owned(),
            schema_version: LOCAL_HISTORY_TIMELINE_SCHEMA_VERSION,
            packet_id: packet_id.into(),
            produced_at: produced_at.into(),
            consumer_surface,
            rows,
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            live_authority_excluded: true,
        }
    }

    /// Validates export safety and fidelity/action coverage on this packet.
    pub fn validate(&self) -> Result<(), LocalHistoryTimelineValidationReport> {
        let violations = validate_packet(self);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(LocalHistoryTimelineValidationReport { violations })
        }
    }
}

/// One report matrix row emitted for the timeline corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineReportRow {
    /// Case id that produced this row.
    pub case_id: String,
    /// Timeline row id.
    pub row_id: String,
    /// Shared row/action fidelity label.
    pub fidelity_label: LocalHistoryTimelineFidelityLabel,
    /// Restore level paired with the fidelity label.
    pub restore_level: LocalHistoryTimelineRestoreLevel,
    /// Target posture covered by the fixture.
    pub target_posture: LocalHistoryTimelineTargetPosture,
    /// Compare basis covered by the fixture.
    pub compare_basis: LocalHistoryTimelineCompareBasis,
    /// Restore action availability.
    pub restore_action_availability: LocalHistoryTimelineActionAvailability,
    /// True when support export includes the row safely.
    pub support_export_safe: bool,
    /// True when the row claims a live session resumed.
    pub live_session_resumed: bool,
    /// True when the row claims privileged execution resumed.
    pub privileged_run_resumed: bool,
}

/// Per-fidelity summary emitted by the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineFidelitySummaryRow {
    /// Fidelity label summarized by this row.
    pub fidelity_label: LocalHistoryTimelineFidelityLabel,
    /// Number of cases carrying this label.
    pub case_count: u32,
    /// Number of rows whose restore action can write.
    pub restore_available_count: u32,
    /// Number of rows that are export-only.
    pub export_only_count: u32,
    /// Number of rows that claim live or privileged session resumption.
    pub resumption_claim_count: u32,
}

/// Metadata-safe report for the checked-in timeline corpus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalHistoryTimelineReport {
    /// Integer schema version for the report.
    pub schema_version: u32,
    /// Stable discriminator for this report.
    pub record_kind: String,
    /// Opaque report id.
    pub report_id: String,
    /// Report timestamp.
    pub captured_at: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
    /// Corpus manifest ref.
    pub corpus_manifest_ref: String,
    /// True when raw local-history payloads are excluded.
    pub raw_payload_excluded: bool,
    /// True when private material is excluded.
    pub raw_private_material_excluded: bool,
    /// True when live authority or privilege handles are excluded.
    pub live_authority_excluded: bool,
    /// Fidelity labels required by the corpus.
    pub required_fidelity_labels: Vec<LocalHistoryTimelineFidelityLabel>,
    /// Action classes required on every row.
    pub required_action_classes: Vec<LocalHistoryTimelineActionClass>,
    /// One matrix row per fixture.
    pub matrix_rows: Vec<LocalHistoryTimelineReportRow>,
    /// Per-fidelity rollups.
    pub fidelity_summaries: Vec<LocalHistoryTimelineFidelitySummaryRow>,
}

impl LocalHistoryTimelineReport {
    /// Returns true when the report is safe to include in support export.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.raw_private_material_excluded
            && self.live_authority_excluded
            && !self.matrix_rows.is_empty()
            && !self.fidelity_summaries.is_empty()
    }
}

/// One validation violation emitted by the timeline evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalHistoryTimelineViolation {
    /// Stable check id.
    pub check_id: String,
    /// Row, case, packet, or fixture ref that failed.
    pub subject_ref: String,
    /// Human-readable validation message.
    pub message: String,
}

/// Validation report returned when timeline checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalHistoryTimelineValidationReport {
    /// Violations emitted by the evaluator.
    pub violations: Vec<LocalHistoryTimelineViolation>,
}

impl fmt::Display for LocalHistoryTimelineValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} local-history timeline violation(s)",
            self.violations.len()
        )
    }
}

impl Error for LocalHistoryTimelineValidationReport {}

/// Evaluates local-history timeline alpha rows and fixture corpora.
#[derive(Debug, Default, Clone, Copy)]
pub struct LocalHistoryTimelineEvaluator;

impl LocalHistoryTimelineEvaluator {
    /// Creates a new evaluator.
    pub const fn new() -> Self {
        Self
    }

    /// Validates one timeline case.
    pub fn validate_case(
        &self,
        case: &LocalHistoryTimelineCase,
    ) -> Result<(), LocalHistoryTimelineValidationReport> {
        let violations = validate_case(case);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(LocalHistoryTimelineValidationReport { violations })
        }
    }

    /// Validates a checked-in corpus.
    pub fn validate_corpus(
        &self,
        corpus: &LocalHistoryTimelineCorpus,
    ) -> Result<(), LocalHistoryTimelineValidationReport> {
        let violations = validate_corpus(corpus);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(LocalHistoryTimelineValidationReport { violations })
        }
    }

    /// Builds a packet from the checked-in corpus after validation.
    pub fn packet(
        &self,
        packet_id: impl Into<String>,
        produced_at: impl Into<String>,
        consumer_surface: LocalHistoryTimelineConsumerSurface,
        corpus: &LocalHistoryTimelineCorpus,
    ) -> Result<LocalHistoryTimelineAlphaPacket, LocalHistoryTimelineValidationReport> {
        self.validate_corpus(corpus)?;
        let mut rows: Vec<LocalHistoryTimelineRow> = corpus
            .entries
            .iter()
            .map(|entry| entry.case.timeline_row.clone())
            .collect();
        rows.sort_by(|a, b| a.row_id.cmp(&b.row_id));
        let packet =
            LocalHistoryTimelineAlphaPacket::new(packet_id, produced_at, consumer_surface, rows);
        packet.validate()?;
        Ok(packet)
    }

    /// Builds a metadata-safe report for the checked-in corpus.
    pub fn report(
        &self,
        report_id: impl Into<String>,
        captured_at: impl Into<String>,
        corpus: &LocalHistoryTimelineCorpus,
    ) -> Result<LocalHistoryTimelineReport, LocalHistoryTimelineValidationReport> {
        self.validate_corpus(corpus)?;

        let mut matrix_rows: Vec<LocalHistoryTimelineReportRow> = corpus
            .entries
            .iter()
            .map(|entry| LocalHistoryTimelineReportRow::from_case(&entry.case))
            .collect();
        matrix_rows.sort_by(|a, b| a.row_id.cmp(&b.row_id));

        let fidelity_summaries = REQUIRED_TIMELINE_FIDELITY_LABELS
            .iter()
            .map(|label| summarize_fidelity(corpus, *label))
            .collect();

        Ok(LocalHistoryTimelineReport {
            schema_version: LOCAL_HISTORY_TIMELINE_SCHEMA_VERSION,
            record_kind: LOCAL_HISTORY_TIMELINE_REPORT_RECORD_KIND.to_owned(),
            report_id: report_id.into(),
            captured_at: captured_at.into(),
            doc_ref: LOCAL_HISTORY_TIMELINE_DOC_REF.to_owned(),
            schema_ref: LOCAL_HISTORY_TIMELINE_SCHEMA_REF.to_owned(),
            corpus_manifest_ref: LOCAL_HISTORY_TIMELINE_CORPUS_MANIFEST_REF.to_owned(),
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            live_authority_excluded: true,
            required_fidelity_labels: REQUIRED_TIMELINE_FIDELITY_LABELS.to_vec(),
            required_action_classes: REQUIRED_TIMELINE_ACTION_CLASSES.to_vec(),
            matrix_rows,
            fidelity_summaries,
        })
    }
}

impl LocalHistoryTimelineReportRow {
    fn from_case(case: &LocalHistoryTimelineCase) -> Self {
        let row = &case.timeline_row;
        let restore_action = row
            .actions
            .iter()
            .find(|action| action.action_class == LocalHistoryTimelineActionClass::Restore);
        Self {
            case_id: case.case_id.clone(),
            row_id: row.row_id.clone(),
            fidelity_label: row.fidelity_label,
            restore_level: row.restore_level,
            target_posture: row.target_posture,
            compare_basis: row.compare_basis,
            restore_action_availability: restore_action
                .map(|action| action.availability_class)
                .unwrap_or(LocalHistoryTimelineActionAvailability::DisabledExportOnly),
            support_export_safe: row.support_export.is_export_safe(),
            live_session_resumed: row.no_rerun_guard.live_session_resumed,
            privileged_run_resumed: row.no_rerun_guard.privileged_run_resumed,
        }
    }
}

fn summarize_fidelity(
    corpus: &LocalHistoryTimelineCorpus,
    fidelity_label: LocalHistoryTimelineFidelityLabel,
) -> LocalHistoryTimelineFidelitySummaryRow {
    let mut row = LocalHistoryTimelineFidelitySummaryRow {
        fidelity_label,
        case_count: 0,
        restore_available_count: 0,
        export_only_count: 0,
        resumption_claim_count: 0,
    };
    for entry in &corpus.entries {
        let timeline_row = &entry.case.timeline_row;
        if timeline_row.fidelity_label != fidelity_label {
            continue;
        }
        row.case_count += 1;
        if timeline_row.no_rerun_guard.live_session_resumed
            || timeline_row.no_rerun_guard.privileged_run_resumed
        {
            row.resumption_claim_count += 1;
        }
        if let Some(restore_action) = timeline_row
            .actions
            .iter()
            .find(|action| action.action_class == LocalHistoryTimelineActionClass::Restore)
        {
            if restore_action.availability_class.can_restore() {
                row.restore_available_count += 1;
            }
            if restore_action.availability_class
                == LocalHistoryTimelineActionAvailability::DisabledExportOnly
            {
                row.export_only_count += 1;
            }
        }
    }
    row
}

fn validate_corpus(corpus: &LocalHistoryTimelineCorpus) -> Vec<LocalHistoryTimelineViolation> {
    let mut violations = Vec::new();
    if corpus.entries.is_empty() {
        push_violation(
            &mut violations,
            "corpus.empty",
            LOCAL_HISTORY_TIMELINE_CORPUS_DIR,
            "corpus must contain at least one local-history timeline case",
        );
        return violations;
    }

    let mut fixture_refs = BTreeSet::new();
    let mut case_ids = BTreeSet::new();
    let mut row_ids = BTreeSet::new();
    let mut seen_fidelity_labels = BTreeSet::new();

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
        if !case_ids.insert(case.case_id.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_case_id",
                &case.case_id,
                "case_id must be unique within the corpus",
            );
        }
        if !row_ids.insert(case.timeline_row.row_id.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_row_id",
                &case.timeline_row.row_id,
                "timeline row ids must be unique within the corpus",
            );
        }
        seen_fidelity_labels.insert(case.timeline_row.fidelity_label);
        violations.extend(validate_case(case));
    }

    for label in REQUIRED_TIMELINE_FIDELITY_LABELS {
        if !seen_fidelity_labels.contains(&label) {
            push_violation(
                &mut violations,
                "corpus.required_fidelity_label_missing",
                label.as_str(),
                format!(
                    "corpus must seed at least one row with fidelity_label = {}",
                    label.as_str()
                ),
            );
        }
    }

    violations
}

fn validate_packet(packet: &LocalHistoryTimelineAlphaPacket) -> Vec<LocalHistoryTimelineViolation> {
    let mut violations = Vec::new();
    if packet.record_kind != LOCAL_HISTORY_TIMELINE_PACKET_RECORD_KIND {
        push_violation(
            &mut violations,
            "packet.record_kind",
            &packet.packet_id,
            format!("record_kind must be {LOCAL_HISTORY_TIMELINE_PACKET_RECORD_KIND}"),
        );
    }
    if packet.schema_version != LOCAL_HISTORY_TIMELINE_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "packet.schema_version",
            &packet.packet_id,
            "schema_version must be 1",
        );
    }
    if packet.rows.is_empty() {
        push_violation(
            &mut violations,
            "packet.rows.empty",
            &packet.packet_id,
            "packet must include at least one timeline row",
        );
    }
    if !packet.raw_payload_excluded
        || !packet.raw_private_material_excluded
        || !packet.live_authority_excluded
    {
        push_violation(
            &mut violations,
            "packet.export_safety",
            &packet.packet_id,
            "packet must exclude raw payloads, private material, and live authority",
        );
    }
    for row in &packet.rows {
        violations.extend(validate_row(row, &packet.packet_id));
    }
    violations
}

fn validate_case(case: &LocalHistoryTimelineCase) -> Vec<LocalHistoryTimelineViolation> {
    let mut violations = Vec::new();
    let target = case.case_id.as_str();

    if case.schema_version != LOCAL_HISTORY_TIMELINE_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "case.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if case.record_kind != LOCAL_HISTORY_TIMELINE_CASE_RECORD_KIND {
        push_violation(
            &mut violations,
            "case.record_kind",
            target,
            format!("record_kind must be {LOCAL_HISTORY_TIMELINE_CASE_RECORD_KIND}"),
        );
    }
    if case.case_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "case.case_id",
            target,
            "case_id must be non-empty",
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
    validate_references(&mut violations, target, &case.references);
    violations.extend(validate_row(&case.timeline_row, target));
    violations
}

fn validate_row(
    row: &LocalHistoryTimelineRow,
    subject: &str,
) -> Vec<LocalHistoryTimelineViolation> {
    let mut violations = Vec::new();
    if row.row_id.trim().is_empty() {
        push_violation(
            &mut violations,
            "row.row_id",
            subject,
            "row_id must be non-empty",
        );
    }
    if row.display_label.trim().is_empty() {
        push_violation(
            &mut violations,
            "row.display_label",
            &row.row_id,
            "display_label must be non-empty",
        );
    }
    if is_forbidden_ref(&row.source_entry_ref) {
        push_violation(
            &mut violations,
            "row.source_entry_ref.forbidden",
            &row.row_id,
            "source_entry_ref must not expose raw body, secret, or token refs",
        );
    }
    for reference in row
        .source_group_ref
        .iter()
        .chain(row.mutation_journal_ref.iter())
        .chain(row.new_checkpoint_on_restore_ref.iter())
    {
        if is_forbidden_ref(reference) {
            push_violation(
                &mut violations,
                "row.ref.forbidden",
                &row.row_id,
                "timeline refs must not expose raw body, secret, or token refs",
            );
        }
    }
    if row.restore_level != restore_level_for_label(row.fidelity_label) {
        push_violation(
            &mut violations,
            "row.restore_level_mismatch",
            &row.row_id,
            "restore_level must match fidelity_label",
        );
    }
    if row.fidelity_label != fidelity_for_target_posture(row.target_posture) {
        push_violation(
            &mut violations,
            "row.target_posture_mismatch",
            &row.row_id,
            "target_posture must derive the declared fidelity_label",
        );
    }
    if row.fidelity_label != fidelity_for_compare_basis(row.compare_basis) {
        push_violation(
            &mut violations,
            "row.compare_basis_mismatch",
            &row.row_id,
            "compare_basis must derive the declared fidelity_label",
        );
    }
    if !row.support_export.is_export_safe() {
        push_violation(
            &mut violations,
            "row.support_export.unsafe",
            &row.row_id,
            "support_export must include ids, fidelity labels, action vocabulary, and exclude raw material",
        );
    }
    validate_actions(&mut violations, row);
    validate_no_rerun_guard(&mut violations, row);
    violations
}

fn validate_actions(
    violations: &mut Vec<LocalHistoryTimelineViolation>,
    row: &LocalHistoryTimelineRow,
) {
    let mut seen = BTreeSet::new();
    for action in &row.actions {
        if !seen.insert(action.action_class) {
            push_violation(
                violations,
                "row.actions.duplicate",
                &row.row_id,
                format!("duplicate action_class {}", action.action_class.as_str()),
            );
        }
        if action.command_id.trim().is_empty() {
            push_violation(
                violations,
                "row.actions.command_id",
                &row.row_id,
                "action command_id must be non-empty",
            );
        }
        if action.fidelity_label != row.fidelity_label {
            push_violation(
                violations,
                "row.actions.fidelity_mismatch",
                &row.row_id,
                "action fidelity_label must match row fidelity_label",
            );
        }
        if action.restore_level != row.restore_level {
            push_violation(
                violations,
                "row.actions.restore_level_mismatch",
                &row.row_id,
                "action restore_level must match row restore_level",
            );
        }
        if action.action_class != LocalHistoryTimelineActionClass::Restore
            && action.writes_new_checkpoint
        {
            push_violation(
                violations,
                "row.actions.non_restore_writes_checkpoint",
                &row.row_id,
                "only restore actions may write a new checkpoint",
            );
        }
        if action.action_class == LocalHistoryTimelineActionClass::Export
            && action
                .support_export_ref
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
        {
            push_violation(
                violations,
                "row.actions.export_missing_support_ref",
                &row.row_id,
                "export actions must carry a support_export_ref",
            );
        }
    }
    for required in REQUIRED_TIMELINE_ACTION_CLASSES {
        if !seen.contains(&required) {
            push_violation(
                violations,
                "row.actions.required_missing",
                &row.row_id,
                format!("row must expose action_class {}", required.as_str()),
            );
        }
    }

    let restore_action = row
        .actions
        .iter()
        .find(|action| action.action_class == LocalHistoryTimelineActionClass::Restore);
    if let Some(action) = restore_action {
        if row.fidelity_label == LocalHistoryTimelineFidelityLabel::EvidenceOnly {
            if action.availability_class
                != LocalHistoryTimelineActionAvailability::DisabledExportOnly
            {
                push_violation(
                    violations,
                    "row.restore.evidence_only_availability",
                    &row.row_id,
                    "evidence-only rows must disable restore and leave export available",
                );
            }
            if action.writes_new_checkpoint {
                push_violation(
                    violations,
                    "row.restore.evidence_only_writes_checkpoint",
                    &row.row_id,
                    "evidence-only rows must not claim a restore checkpoint write",
                );
            }
        } else if action.availability_class.can_restore() {
            if !action.writes_new_checkpoint {
                push_violation(
                    violations,
                    "row.restore.missing_checkpoint_write",
                    &row.row_id,
                    "available restore actions must declare a new checkpoint write",
                );
            }
            if row
                .new_checkpoint_on_restore_ref
                .as_deref()
                .unwrap_or("")
                .trim()
                .is_empty()
            {
                push_violation(
                    violations,
                    "row.restore.missing_checkpoint_ref",
                    &row.row_id,
                    "available restore actions must name new_checkpoint_on_restore_ref",
                );
            }
        }
    }
}

fn validate_no_rerun_guard(
    violations: &mut Vec<LocalHistoryTimelineViolation>,
    row: &LocalHistoryTimelineRow,
) {
    let guard = &row.no_rerun_guard;
    if guard.summary.trim().is_empty() {
        push_violation(
            violations,
            "row.no_rerun.summary",
            &row.row_id,
            "no-rerun guard summary must be non-empty",
        );
    }
    if row.fidelity_label == LocalHistoryTimelineFidelityLabel::EvidenceOnly {
        if guard.resumption_posture != LocalHistoryTimelineResumptionPosture::StaticEvidenceOnly {
            push_violation(
                violations,
                "row.no_rerun.evidence_posture",
                &row.row_id,
                "evidence-only rows must use static_evidence_only posture",
            );
        }
        if guard.live_session_resumed || guard.privileged_run_resumed {
            push_violation(
                violations,
                "row.no_rerun.evidence_resumed_live_state",
                &row.row_id,
                "evidence-only rows must not claim live sessions or privileged runs resumed",
            );
        }
        if !guard.evidence_only_label_visible {
            push_violation(
                violations,
                "row.no_rerun.evidence_label_missing",
                &row.row_id,
                "evidence-only rows must keep the evidence-only label visible",
            );
        }
    }
    if row.fidelity_label == LocalHistoryTimelineFidelityLabel::LayoutOnly
        && guard.resumption_posture
            == LocalHistoryTimelineResumptionPosture::AuthorityVerifiedNoRerun
    {
        push_violation(
            violations,
            "row.no_rerun.layout_claims_authority",
            &row.row_id,
            "layout-only rows must not claim verified live authority",
        );
    }
}

fn validate_references(
    violations: &mut Vec<LocalHistoryTimelineViolation>,
    target: &str,
    refs: &LocalHistoryTimelineReferences,
) {
    if refs.doc_ref != LOCAL_HISTORY_TIMELINE_DOC_REF {
        push_violation(
            violations,
            "case.refs.doc_ref",
            target,
            "doc_ref must point at the timeline alpha doc",
        );
    }
    if refs.schema_ref != LOCAL_HISTORY_TIMELINE_SCHEMA_REF {
        push_violation(
            violations,
            "case.refs.schema_ref",
            target,
            "schema_ref must point at the timeline alpha schema",
        );
    }
    if refs.report_ref != LOCAL_HISTORY_TIMELINE_REPORT_REF {
        push_violation(
            violations,
            "case.refs.report_ref",
            target,
            "report_ref must point at the timeline alpha report",
        );
    }
}

const fn restore_level_for_label(
    label: LocalHistoryTimelineFidelityLabel,
) -> LocalHistoryTimelineRestoreLevel {
    match label {
        LocalHistoryTimelineFidelityLabel::Exact => LocalHistoryTimelineRestoreLevel::ExactRestore,
        LocalHistoryTimelineFidelityLabel::Compatible => {
            LocalHistoryTimelineRestoreLevel::CompatibleRestore
        }
        LocalHistoryTimelineFidelityLabel::LayoutOnly => {
            LocalHistoryTimelineRestoreLevel::LayoutOnly
        }
        LocalHistoryTimelineFidelityLabel::EvidenceOnly => {
            LocalHistoryTimelineRestoreLevel::EvidenceOnly
        }
    }
}

const fn fidelity_for_target_posture(
    posture: LocalHistoryTimelineTargetPosture,
) -> LocalHistoryTimelineFidelityLabel {
    match posture {
        LocalHistoryTimelineTargetPosture::SameObjectCurrent => {
            LocalHistoryTimelineFidelityLabel::Exact
        }
        LocalHistoryTimelineTargetPosture::CompatibleLogicalDocument => {
            LocalHistoryTimelineFidelityLabel::Compatible
        }
        LocalHistoryTimelineTargetPosture::LayoutMetadataOnly => {
            LocalHistoryTimelineFidelityLabel::LayoutOnly
        }
        LocalHistoryTimelineTargetPosture::EvidenceMetadataOnly => {
            LocalHistoryTimelineFidelityLabel::EvidenceOnly
        }
    }
}

const fn fidelity_for_compare_basis(
    basis: LocalHistoryTimelineCompareBasis,
) -> LocalHistoryTimelineFidelityLabel {
    match basis {
        LocalHistoryTimelineCompareBasis::ByteSnapshot => LocalHistoryTimelineFidelityLabel::Exact,
        LocalHistoryTimelineCompareBasis::CompatibleSnapshot => {
            LocalHistoryTimelineFidelityLabel::Compatible
        }
        LocalHistoryTimelineCompareBasis::LayoutTopology => {
            LocalHistoryTimelineFidelityLabel::LayoutOnly
        }
        LocalHistoryTimelineCompareBasis::EvidenceManifest => {
            LocalHistoryTimelineFidelityLabel::EvidenceOnly
        }
    }
}

fn is_forbidden_ref(value: &str) -> bool {
    value.starts_with("obj:")
        || value.starts_with("raw:")
        || value.starts_with("secret:")
        || value.starts_with("token:")
}

fn push_violation(
    violations: &mut Vec<LocalHistoryTimelineViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(LocalHistoryTimelineViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

/// Loads a YAML-encoded [`LocalHistoryTimelineCase`].
pub fn load_local_history_timeline_case(
    yaml: &str,
) -> Result<LocalHistoryTimelineCase, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Returns the checked-in local-history timeline alpha corpus.
pub fn current_local_history_timeline_corpus(
) -> Result<LocalHistoryTimelineCorpus, serde_yaml::Error> {
    let entries = CASE_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<LocalHistoryTimelineCase>(yaml).map(|case| {
                LocalHistoryTimelineCorpusEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    case,
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(LocalHistoryTimelineCorpus { entries })
}

/// Returns the timeline fixture refs in declaration order.
pub fn current_local_history_timeline_fixture_refs() -> impl Iterator<Item = &'static str> {
    CASE_FIXTURES.iter().map(|(fixture_ref, _)| *fixture_ref)
}

const CASE_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/recovery/m3/local_history_timeline/exact_snapshot_restore.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/local_history_timeline/exact_snapshot_restore.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/local_history_timeline/compatible_schema_compare.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/local_history_timeline/compatible_schema_compare.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/local_history_timeline/layout_only_placeholder.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/local_history_timeline/layout_only_placeholder.yaml"
        )),
    ),
    (
        "fixtures/recovery/m3/local_history_timeline/evidence_only_export.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/recovery/m3/local_history_timeline/evidence_only_export.yaml"
        )),
    ),
];
