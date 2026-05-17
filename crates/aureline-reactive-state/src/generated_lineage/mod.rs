//! Generated-artifact lineage and drift-state beta projection.
//!
//! This module is the canonical loader, validator, projector, and
//! reporter for the generated-artifact lineage beta contract. A
//! lineage packet binds one consumer surface (`search`, `review`,
//! `ai_context`, `support_export`) to one artifact under a closed
//! artifact-family vocabulary (`build_output`, `lockfile`,
//! `preview_render`, `notebook_output`, `run_result_artifact`) and
//! captures generator identity, canonical source refs, lineage class,
//! drift state, and the default edit posture in a single
//! metadata-safe exportable object. The `default_edit_posture` is
//! re-derived from `lineage_class` and the `downgrade_label` is
//! re-derived from `drift_state` so prose cannot lie about generator
//! identity, and a closed downgrade label downgrades a failing row
//! without inventing new vocabulary.
//!
//! Bound to the boundary schema at
//! [`/schemas/state/generated_artifact.schema.json`](../../../../schemas/state/generated_artifact.schema.json),
//! the reviewer doc at
//! [`/docs/state/m3/generated_artifact_lineage_beta.md`](../../../../docs/state/m3/generated_artifact_lineage_beta.md),
//! and the baseline report at
//! [`/artifacts/support/m3/generated_artifact_lineage_report.md`](../../../../artifacts/support/m3/generated_artifact_lineage_report.md).
//!
//! Search, review, AI-context, and support-export pipelines read this
//! module's report projection so the in-product chrome and the
//! support packet quote the same lineage, drift, generator, and
//! edit-posture fields without re-running generators and without
//! forcing raw payload capture.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag for a generated-artifact lineage record.
pub const GENERATED_ARTIFACT_LINEAGE_RECORD_KIND: &str = "generated_artifact_lineage_record";

/// Stable record-kind tag for the lineage report record.
pub const GENERATED_ARTIFACT_LINEAGE_REPORT_RECORD_KIND: &str =
    "generated_artifact_lineage_report_record";

/// Frozen schema version for generated-artifact lineage records.
pub const GENERATED_ARTIFACT_LINEAGE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const GENERATED_ARTIFACT_LINEAGE_SCHEMA_REF: &str = "schemas/state/generated_artifact.schema.json";

/// Repo-relative path of the reviewer doc.
pub const GENERATED_ARTIFACT_LINEAGE_DOC_REF: &str =
    "docs/state/m3/generated_artifact_lineage_beta.md";

/// Repo-relative path of the baseline report.
pub const GENERATED_ARTIFACT_LINEAGE_REPORT_REF: &str =
    "artifacts/support/m3/generated_artifact_lineage_report.md";

/// Repo-relative path of the protected fixture corpus directory.
pub const GENERATED_ARTIFACT_LINEAGE_CORPUS_DIR: &str = "fixtures/state/generated_artifacts_beta";

/// Repo-relative path of the protected corpus manifest.
pub const GENERATED_ARTIFACT_LINEAGE_CORPUS_MANIFEST_REF: &str =
    "fixtures/state/generated_artifacts_beta/manifest.yaml";

/// Closed consumer-surface vocabulary for lineage packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageConsumerSurface {
    Search,
    Review,
    AiContext,
    SupportExport,
}

impl LineageConsumerSurface {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Search => "search",
            Self::Review => "review",
            Self::AiContext => "ai_context",
            Self::SupportExport => "support_export",
        }
    }
}

/// Closed list of consumer surfaces the corpus must cover.
pub const REQUIRED_LINEAGE_CONSUMER_SURFACES: [LineageConsumerSurface; 4] = [
    LineageConsumerSurface::Search,
    LineageConsumerSurface::Review,
    LineageConsumerSurface::AiContext,
    LineageConsumerSurface::SupportExport,
];

/// Closed artifact-family vocabulary for lineage packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactFamily {
    BuildOutput,
    Lockfile,
    PreviewRender,
    NotebookOutput,
    RunResultArtifact,
}

impl ArtifactFamily {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildOutput => "build_output",
            Self::Lockfile => "lockfile",
            Self::PreviewRender => "preview_render",
            Self::NotebookOutput => "notebook_output",
            Self::RunResultArtifact => "run_result_artifact",
        }
    }
}

/// Closed list of artifact families the corpus must cover.
pub const REQUIRED_ARTIFACT_FAMILIES: [ArtifactFamily; 5] = [
    ArtifactFamily::BuildOutput,
    ArtifactFamily::Lockfile,
    ArtifactFamily::PreviewRender,
    ArtifactFamily::NotebookOutput,
    ArtifactFamily::RunResultArtifact,
];

/// Closed lineage-class vocabulary describing the source-of-truth
/// posture for the artifact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageClass {
    CanonicalSource,
    GeneratedFromLocalSource,
    RegenerableLockfileArtifact,
    MirroredFromLocalSource,
    ImportedExternalArtifact,
    DerivedFromRunArtifact,
    PreviewedFromLocalSource,
    UnknownLineage,
}

impl LineageClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalSource => "canonical_source",
            Self::GeneratedFromLocalSource => "generated_from_local_source",
            Self::RegenerableLockfileArtifact => "regenerable_lockfile_artifact",
            Self::MirroredFromLocalSource => "mirrored_from_local_source",
            Self::ImportedExternalArtifact => "imported_external_artifact",
            Self::DerivedFromRunArtifact => "derived_from_run_artifact",
            Self::PreviewedFromLocalSource => "previewed_from_local_source",
            Self::UnknownLineage => "unknown_lineage",
        }
    }

    /// Returns true when the lineage class denotes an artifact that
    /// originated outside the workspace.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedExternalArtifact)
    }
}

/// Closed list of lineage classes the corpus must cover.
pub const REQUIRED_LINEAGE_CLASSES: [LineageClass; 5] = [
    LineageClass::GeneratedFromLocalSource,
    LineageClass::RegenerableLockfileArtifact,
    LineageClass::PreviewedFromLocalSource,
    LineageClass::DerivedFromRunArtifact,
    LineageClass::ImportedExternalArtifact,
];

/// Closed drift-state vocabulary describing the lineage row's
/// posture relative to its generator inputs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DriftState {
    Aligned,
    SourceDrifted,
    RegenPending,
    StaleGenerated,
    GeneratorMissing,
    ImportedNoLocalSource,
    OutOfScope,
}

impl DriftState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Aligned => "aligned",
            Self::SourceDrifted => "source_drifted",
            Self::RegenPending => "regen_pending",
            Self::StaleGenerated => "stale_generated",
            Self::GeneratorMissing => "generator_missing",
            Self::ImportedNoLocalSource => "imported_no_local_source",
            Self::OutOfScope => "out_of_scope",
        }
    }

    /// Returns true when the drift state is considered healthy for
    /// the lineage class — `aligned` is always healthy, and an
    /// imported artifact whose drift state is `imported_no_local_source`
    /// is the natural state for that class rather than an anomaly.
    pub const fn is_healthy(self) -> bool {
        matches!(self, Self::Aligned | Self::ImportedNoLocalSource)
    }
}

/// Closed default edit posture vocabulary for the artifact row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DefaultEditPosture {
    EditableCanonical,
    ReadOnlyGenerated,
    RegenerateOnly,
    ReviewRequiredBeforeEdit,
    ImportedReadOnly,
    TransientRunArtifact,
}

impl DefaultEditPosture {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditableCanonical => "editable_canonical",
            Self::ReadOnlyGenerated => "read_only_generated",
            Self::RegenerateOnly => "regenerate_only",
            Self::ReviewRequiredBeforeEdit => "review_required_before_edit",
            Self::ImportedReadOnly => "imported_read_only",
            Self::TransientRunArtifact => "transient_run_artifact",
        }
    }
}

/// Closed downgrade-label vocabulary; one of these labels applies
/// when a lineage packet downgrades a beta row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LineageDowngradeLabel {
    None,
    RedBlocksBetaRow,
    YellowDriftPending,
    YellowGeneratorUnknown,
    YellowPartialCoverage,
    DegradedToMetadataOnly,
    StaleCorpusBlocksReleaseCandidate,
}

impl LineageDowngradeLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RedBlocksBetaRow => "red_blocks_beta_row",
            Self::YellowDriftPending => "yellow_drift_pending",
            Self::YellowGeneratorUnknown => "yellow_generator_unknown",
            Self::YellowPartialCoverage => "yellow_partial_coverage",
            Self::DegradedToMetadataOnly => "degraded_to_metadata_only",
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
pub enum LineageOpenGapClass {
    None,
    RegenPending,
    GeneratorIdentityPending,
    SourceRefPending,
    SurfaceCoveragePending,
    LineagePending,
    EvidenceExportPending,
}

impl LineageOpenGapClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::RegenPending => "regen_pending",
            Self::GeneratorIdentityPending => "generator_identity_pending",
            Self::SourceRefPending => "source_ref_pending",
            Self::SurfaceCoveragePending => "surface_coverage_pending",
            Self::LineagePending => "lineage_pending",
            Self::EvidenceExportPending => "evidence_export_pending",
        }
    }
}

/// One open-gap row attached to a lineage packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageOpenGapEntry {
    pub gap_class: LineageOpenGapClass,
    pub summary: String,
}

/// Closed generator-kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratorKind {
    BuildSystem,
    PackageManager,
    PreviewRenderer,
    NotebookKernel,
    TaskRunner,
    ExternalImport,
    UnknownGenerator,
}

impl GeneratorKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BuildSystem => "build_system",
            Self::PackageManager => "package_manager",
            Self::PreviewRenderer => "preview_renderer",
            Self::NotebookKernel => "notebook_kernel",
            Self::TaskRunner => "task_runner",
            Self::ExternalImport => "external_import",
            Self::UnknownGenerator => "unknown_generator",
        }
    }
}

/// Pinned generator identity carried by every lineage packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratorIdentity {
    pub generator_kind: GeneratorKind,
    pub generator_ref: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub generator_version_label: Option<String>,
}

/// Closed source-kind vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    LocalSourceFile,
    LocalSourceManifest,
    LocalNotebookCell,
    ExternalImportDescriptor,
    RunInvocationDescriptor,
    NoLocalSource,
}

impl SourceKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalSourceFile => "local_source_file",
            Self::LocalSourceManifest => "local_source_manifest",
            Self::LocalNotebookCell => "local_notebook_cell",
            Self::ExternalImportDescriptor => "external_import_descriptor",
            Self::RunInvocationDescriptor => "run_invocation_descriptor",
            Self::NoLocalSource => "no_local_source",
        }
    }
}

/// One canonical source reference for the artifact.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageSourceRef {
    pub source_kind: SourceKind,
    pub source_path: String,
}

/// Metadata-safe evidence-export projection pinned on each packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageEvidenceExportProjection {
    pub preserves_artifact_family_label: bool,
    pub preserves_lineage_label: bool,
    pub preserves_drift_state_label: bool,
    pub preserves_edit_posture_label: bool,
    pub preserves_consumer_surface_label: bool,
    pub preserves_generator_identity: bool,
    pub preserves_source_refs: bool,
    pub raw_payload_excluded: bool,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub preserves_user_authored_files: bool,
}

impl LineageEvidenceExportProjection {
    pub const fn metadata_safe_baseline() -> Self {
        Self {
            preserves_artifact_family_label: true,
            preserves_lineage_label: true,
            preserves_drift_state_label: true,
            preserves_edit_posture_label: true,
            preserves_consumer_surface_label: true,
            preserves_generator_identity: true,
            preserves_source_refs: true,
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            preserves_user_authored_files: true,
        }
    }
}

/// Safety baseline pinned on every packet and on the report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineagePacketSafety {
    pub raw_payload_excluded: bool,
    pub raw_private_material_excluded: bool,
    pub ambient_authority_excluded: bool,
    pub destructive_resets_present: bool,
    pub preserves_user_authored_files: bool,
}

impl LineagePacketSafety {
    pub const fn metadata_safe_baseline() -> Self {
        Self {
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            destructive_resets_present: false,
            preserves_user_authored_files: true,
        }
    }
}

/// Companion refs quoted on each packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineagePacketReferences {
    pub doc_ref: String,
    pub schema_ref: String,
    pub report_ref: String,
}

impl LineagePacketReferences {
    pub fn pinned() -> Self {
        Self {
            doc_ref: GENERATED_ARTIFACT_LINEAGE_DOC_REF.to_owned(),
            schema_ref: GENERATED_ARTIFACT_LINEAGE_SCHEMA_REF.to_owned(),
            report_ref: GENERATED_ARTIFACT_LINEAGE_REPORT_REF.to_owned(),
        }
    }
}

/// One generated-artifact lineage record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedArtifactLineagePacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub title: String,
    pub consumer_surface: LineageConsumerSurface,
    pub artifact_family: ArtifactFamily,
    pub artifact_ref: String,
    pub workspace_id: String,
    pub lineage_class: LineageClass,
    pub drift_state: DriftState,
    pub default_edit_posture: DefaultEditPosture,
    pub downgrade_label: LineageDowngradeLabel,
    pub generator_identity: GeneratorIdentity,
    pub source_refs: Vec<LineageSourceRef>,
    pub evidence_export: LineageEvidenceExportProjection,
    #[serde(default)]
    pub open_gaps: Vec<LineageOpenGapEntry>,
    pub safety: LineagePacketSafety,
    pub references: LineagePacketReferences,
    pub captured_at: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub reviewer_summary: Option<String>,
}

/// One corpus entry pairing a fixture ref to its packet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedArtifactLineageCorpusEntry {
    pub fixture_ref: String,
    pub packet: GeneratedArtifactLineagePacket,
}

/// Generated-artifact lineage corpus loaded from checked-in fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedArtifactLineageCorpus {
    pub entries: Vec<GeneratedArtifactLineageCorpusEntry>,
}

/// One row in the report matrix projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageReportMatrixRow {
    pub packet_id: String,
    pub consumer_surface: LineageConsumerSurface,
    pub artifact_family: ArtifactFamily,
    pub artifact_ref: String,
    pub lineage_class: LineageClass,
    pub drift_state: DriftState,
    pub default_edit_posture: DefaultEditPosture,
    pub downgrade_label: LineageDowngradeLabel,
    pub open_gap_classes: Vec<LineageOpenGapClass>,
}

impl LineageReportMatrixRow {
    fn from_packet(packet: &GeneratedArtifactLineagePacket) -> Self {
        let mut open_gap_classes: Vec<LineageOpenGapClass> =
            packet.open_gaps.iter().map(|gap| gap.gap_class).collect();
        if open_gap_classes.is_empty() {
            open_gap_classes.push(LineageOpenGapClass::None);
        }
        Self {
            packet_id: packet.packet_id.clone(),
            consumer_surface: packet.consumer_surface,
            artifact_family: packet.artifact_family,
            artifact_ref: packet.artifact_ref.clone(),
            lineage_class: packet.lineage_class,
            drift_state: packet.drift_state,
            default_edit_posture: packet.default_edit_posture,
            downgrade_label: packet.downgrade_label,
            open_gap_classes,
        }
    }
}

/// Per-consumer-surface summary row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageConsumerSurfaceSummaryRow {
    pub consumer_surface: LineageConsumerSurface,
    pub packet_count: u32,
    pub aligned_count: u32,
    pub drift_count: u32,
    pub imported_count: u32,
    pub downgrade_required_count: u32,
}

/// Per-artifact-family summary row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactFamilySummaryRow {
    pub artifact_family: ArtifactFamily,
    pub packet_count: u32,
    pub aligned_count: u32,
    pub drift_count: u32,
    pub downgrade_required_count: u32,
}

/// Per-lineage-class summary row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageClassSummaryRow {
    pub lineage_class: LineageClass,
    pub packet_count: u32,
    pub aligned_count: u32,
    pub drift_count: u32,
    pub downgrade_required_count: u32,
}

/// Metadata-safe generated-artifact lineage report record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedArtifactLineageReport {
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
    pub required_consumer_surfaces: Vec<LineageConsumerSurface>,
    pub required_artifact_families: Vec<ArtifactFamily>,
    pub required_lineages: Vec<LineageClass>,
    pub matrix_rows: Vec<LineageReportMatrixRow>,
    pub consumer_surface_summaries: Vec<LineageConsumerSurfaceSummaryRow>,
    pub artifact_family_summaries: Vec<ArtifactFamilySummaryRow>,
    pub lineage_summaries: Vec<LineageClassSummaryRow>,
}

impl GeneratedArtifactLineageReport {
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
        if self.consumer_surface_summaries.is_empty()
            || self.artifact_family_summaries.is_empty()
            || self.lineage_summaries.is_empty()
        {
            return false;
        }
        true
    }

    /// Returns a deterministic plaintext rendering of the report.
    /// Suitable for the baseline artifact and snapshot tests; the
    /// rendering preserves every closed-vocabulary token surfaces
    /// expose in-product.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "{} {} captured_at={}\n",
            self.record_kind, self.report_id, self.captured_at
        ));
        for row in &self.matrix_rows {
            out.push_str(&format!(
                "row {} surface={} family={} lineage={} drift={} posture={} downgrade={}\n",
                row.packet_id,
                row.consumer_surface.as_str(),
                row.artifact_family.as_str(),
                row.lineage_class.as_str(),
                row.drift_state.as_str(),
                row.default_edit_posture.as_str(),
                row.downgrade_label.as_str(),
            ));
        }
        out
    }
}

/// One validation violation emitted by the evaluator.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedArtifactLineageViolation {
    pub check_id: String,
    pub subject_ref: String,
    pub message: String,
}

/// Validation report returned when one or more checks fail.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratedArtifactLineageValidationReport {
    pub violations: Vec<GeneratedArtifactLineageViolation>,
}

impl fmt::Display for GeneratedArtifactLineageValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} generated-artifact lineage violation(s)",
            self.violations.len()
        )
    }
}

impl Error for GeneratedArtifactLineageValidationReport {}

/// Generated-artifact lineage packet evaluator.
#[derive(Debug, Default, Clone, Copy)]
pub struct GeneratedArtifactLineageEvaluator;

impl GeneratedArtifactLineageEvaluator {
    pub const fn new() -> Self {
        Self
    }

    pub fn validate_packet(
        &self,
        packet: &GeneratedArtifactLineagePacket,
    ) -> Result<(), GeneratedArtifactLineageValidationReport> {
        let violations = validate_packet(packet);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(GeneratedArtifactLineageValidationReport { violations })
        }
    }

    pub fn validate_corpus(
        &self,
        corpus: &GeneratedArtifactLineageCorpus,
    ) -> Result<(), GeneratedArtifactLineageValidationReport> {
        let violations = validate_corpus(corpus);
        if violations.is_empty() {
            Ok(())
        } else {
            Err(GeneratedArtifactLineageValidationReport { violations })
        }
    }

    pub fn report(
        &self,
        report_id: impl Into<String>,
        captured_at: impl Into<String>,
        corpus: &GeneratedArtifactLineageCorpus,
    ) -> Result<GeneratedArtifactLineageReport, GeneratedArtifactLineageValidationReport> {
        self.validate_corpus(corpus)?;
        let mut matrix_rows: Vec<LineageReportMatrixRow> = corpus
            .entries
            .iter()
            .map(|entry| LineageReportMatrixRow::from_packet(&entry.packet))
            .collect();
        matrix_rows.sort_by(|a, b| a.packet_id.cmp(&b.packet_id));

        let consumer_surface_summaries = REQUIRED_LINEAGE_CONSUMER_SURFACES
            .iter()
            .map(|surface| summarize_consumer_surface(corpus, *surface))
            .collect();
        let artifact_family_summaries = REQUIRED_ARTIFACT_FAMILIES
            .iter()
            .map(|family| summarize_artifact_family(corpus, *family))
            .collect();
        let lineage_summaries = REQUIRED_LINEAGE_CLASSES
            .iter()
            .map(|lineage| summarize_lineage(corpus, *lineage))
            .collect();

        Ok(GeneratedArtifactLineageReport {
            schema_version: GENERATED_ARTIFACT_LINEAGE_SCHEMA_VERSION,
            record_kind: GENERATED_ARTIFACT_LINEAGE_REPORT_RECORD_KIND.to_owned(),
            report_id: report_id.into(),
            captured_at: captured_at.into(),
            doc_ref: GENERATED_ARTIFACT_LINEAGE_DOC_REF.to_owned(),
            schema_ref: GENERATED_ARTIFACT_LINEAGE_SCHEMA_REF.to_owned(),
            corpus_manifest_ref: GENERATED_ARTIFACT_LINEAGE_CORPUS_MANIFEST_REF.to_owned(),
            raw_payload_excluded: true,
            raw_private_material_excluded: true,
            ambient_authority_excluded: true,
            required_consumer_surfaces: REQUIRED_LINEAGE_CONSUMER_SURFACES.to_vec(),
            required_artifact_families: REQUIRED_ARTIFACT_FAMILIES.to_vec(),
            required_lineages: REQUIRED_LINEAGE_CLASSES.to_vec(),
            matrix_rows,
            consumer_surface_summaries,
            artifact_family_summaries,
            lineage_summaries,
        })
    }
}

fn summarize_consumer_surface(
    corpus: &GeneratedArtifactLineageCorpus,
    surface: LineageConsumerSurface,
) -> LineageConsumerSurfaceSummaryRow {
    let mut row = LineageConsumerSurfaceSummaryRow {
        consumer_surface: surface,
        packet_count: 0,
        aligned_count: 0,
        drift_count: 0,
        imported_count: 0,
        downgrade_required_count: 0,
    };
    for entry in &corpus.entries {
        if entry.packet.consumer_surface != surface {
            continue;
        }
        row.packet_count += 1;
        match entry.packet.drift_state {
            DriftState::Aligned => row.aligned_count += 1,
            DriftState::ImportedNoLocalSource => row.imported_count += 1,
            _ => row.drift_count += 1,
        }
        if !entry.packet.downgrade_label.is_healthy() {
            row.downgrade_required_count += 1;
        }
    }
    row
}

fn summarize_artifact_family(
    corpus: &GeneratedArtifactLineageCorpus,
    family: ArtifactFamily,
) -> ArtifactFamilySummaryRow {
    let mut row = ArtifactFamilySummaryRow {
        artifact_family: family,
        packet_count: 0,
        aligned_count: 0,
        drift_count: 0,
        downgrade_required_count: 0,
    };
    for entry in &corpus.entries {
        if entry.packet.artifact_family != family {
            continue;
        }
        row.packet_count += 1;
        if entry.packet.drift_state.is_healthy() {
            row.aligned_count += 1;
        } else {
            row.drift_count += 1;
        }
        if !entry.packet.downgrade_label.is_healthy() {
            row.downgrade_required_count += 1;
        }
    }
    row
}

fn summarize_lineage(
    corpus: &GeneratedArtifactLineageCorpus,
    lineage: LineageClass,
) -> LineageClassSummaryRow {
    let mut row = LineageClassSummaryRow {
        lineage_class: lineage,
        packet_count: 0,
        aligned_count: 0,
        drift_count: 0,
        downgrade_required_count: 0,
    };
    for entry in &corpus.entries {
        if entry.packet.lineage_class != lineage {
            continue;
        }
        row.packet_count += 1;
        if entry.packet.drift_state.is_healthy() {
            row.aligned_count += 1;
        } else {
            row.drift_count += 1;
        }
        if !entry.packet.downgrade_label.is_healthy() {
            row.downgrade_required_count += 1;
        }
    }
    row
}

fn validate_corpus(
    corpus: &GeneratedArtifactLineageCorpus,
) -> Vec<GeneratedArtifactLineageViolation> {
    let mut violations = Vec::new();

    if corpus.entries.is_empty() {
        push_violation(
            &mut violations,
            "corpus.empty",
            GENERATED_ARTIFACT_LINEAGE_CORPUS_DIR,
            "corpus must contain at least one generated-artifact lineage packet",
        );
        return violations;
    }

    let mut packet_ids: BTreeSet<String> = BTreeSet::new();
    let mut fixture_refs: BTreeSet<String> = BTreeSet::new();
    let mut seen_surfaces: BTreeSet<LineageConsumerSurface> = BTreeSet::new();
    let mut seen_families: BTreeSet<ArtifactFamily> = BTreeSet::new();
    let mut seen_lineages: BTreeSet<LineageClass> = BTreeSet::new();
    let mut seen_drift_row = false;

    for entry in &corpus.entries {
        if !fixture_refs.insert(entry.fixture_ref.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_fixture_ref",
                &entry.fixture_ref,
                "fixture_ref must be unique within the corpus",
            );
        }
        let packet = &entry.packet;
        if !packet_ids.insert(packet.packet_id.clone()) {
            push_violation(
                &mut violations,
                "corpus.duplicate_packet_id",
                &packet.packet_id,
                "packet_id must be unique within the corpus",
            );
        }
        seen_surfaces.insert(packet.consumer_surface);
        seen_families.insert(packet.artifact_family);
        seen_lineages.insert(packet.lineage_class);
        if !matches!(packet.drift_state, DriftState::Aligned) {
            seen_drift_row = true;
        }
        violations.extend(validate_packet(packet));
    }

    for surface in REQUIRED_LINEAGE_CONSUMER_SURFACES {
        if !seen_surfaces.contains(&surface) {
            push_violation(
                &mut violations,
                "corpus.required_consumer_surface_missing",
                surface.as_str(),
                format!(
                    "corpus must seed at least one packet for consumer_surface = {}",
                    surface.as_str()
                ),
            );
        }
    }
    for family in REQUIRED_ARTIFACT_FAMILIES {
        if !seen_families.contains(&family) {
            push_violation(
                &mut violations,
                "corpus.required_artifact_family_missing",
                family.as_str(),
                format!(
                    "corpus must seed at least one packet for artifact_family = {}",
                    family.as_str()
                ),
            );
        }
    }
    for lineage in REQUIRED_LINEAGE_CLASSES {
        if !seen_lineages.contains(&lineage) {
            push_violation(
                &mut violations,
                "corpus.required_lineage_missing",
                lineage.as_str(),
                format!(
                    "corpus must seed at least one packet with lineage_class = {}",
                    lineage.as_str()
                ),
            );
        }
    }
    if !seen_drift_row {
        push_violation(
            &mut violations,
            "corpus.no_drift_row",
            GENERATED_ARTIFACT_LINEAGE_CORPUS_DIR,
            "corpus must seed at least one packet with a non-aligned drift_state so the drift contract is exercised by a fixture",
        );
    }

    violations
}

fn validate_packet(
    packet: &GeneratedArtifactLineagePacket,
) -> Vec<GeneratedArtifactLineageViolation> {
    let mut violations = Vec::new();
    let target = packet.packet_id.as_str();

    if packet.schema_version != GENERATED_ARTIFACT_LINEAGE_SCHEMA_VERSION {
        push_violation(
            &mut violations,
            "packet.schema_version",
            target,
            "schema_version must be 1",
        );
    }
    if packet.record_kind != GENERATED_ARTIFACT_LINEAGE_RECORD_KIND {
        push_violation(
            &mut violations,
            "packet.record_kind",
            target,
            format!("record_kind must be {GENERATED_ARTIFACT_LINEAGE_RECORD_KIND}"),
        );
    }
    for (field, value) in [
        ("packet_id", packet.packet_id.as_str()),
        ("title", packet.title.as_str()),
        ("artifact_ref", packet.artifact_ref.as_str()),
        ("workspace_id", packet.workspace_id.as_str()),
        ("captured_at", packet.captured_at.as_str()),
    ] {
        if value.trim().is_empty() {
            push_violation(
                &mut violations,
                format!("packet.{field}"),
                target,
                format!("{field} must be non-empty"),
            );
        }
    }

    validate_drift_lineage_pair(&mut violations, target, packet);
    validate_edit_posture(&mut violations, target, packet);
    validate_outcome_and_downgrade(&mut violations, target, packet);
    validate_generator_identity(&mut violations, target, &packet.generator_identity);
    validate_source_refs(&mut violations, target, &packet.source_refs);
    validate_evidence_export(&mut violations, target, &packet.evidence_export);
    validate_open_gaps(&mut violations, target, &packet.open_gaps);
    validate_safety(&mut violations, target, &packet.safety);
    validate_references(&mut violations, target, &packet.references);

    violations
}

fn validate_drift_lineage_pair(
    violations: &mut Vec<GeneratedArtifactLineageViolation>,
    target: &str,
    packet: &GeneratedArtifactLineagePacket,
) {
    if packet.lineage_class.is_imported()
        && !matches!(packet.drift_state, DriftState::ImportedNoLocalSource)
    {
        push_violation(
            violations,
            "packet.drift.imported_must_pin_imported_no_local_source",
            target,
            "imported_external_artifact packets must pin drift_state = imported_no_local_source",
        );
    }
    if !packet.lineage_class.is_imported()
        && matches!(packet.drift_state, DriftState::ImportedNoLocalSource)
    {
        push_violation(
            violations,
            "packet.drift.imported_only_for_imported_lineage",
            target,
            "drift_state = imported_no_local_source is only valid for imported_external_artifact lineage",
        );
    }
    if matches!(packet.lineage_class, LineageClass::CanonicalSource)
        && !matches!(packet.drift_state, DriftState::Aligned)
    {
        push_violation(
            violations,
            "packet.drift.canonical_must_be_aligned",
            target,
            "canonical_source packets must pin drift_state = aligned",
        );
    }
}

fn validate_edit_posture(
    violations: &mut Vec<GeneratedArtifactLineageViolation>,
    target: &str,
    packet: &GeneratedArtifactLineagePacket,
) {
    let expected = derive_default_edit_posture(packet.lineage_class);
    if expected != packet.default_edit_posture {
        push_violation(
            violations,
            "packet.default_edit_posture.derived_mismatch",
            target,
            format!(
                "default_edit_posture must be {} for lineage_class = {}; got {}",
                expected.as_str(),
                packet.lineage_class.as_str(),
                packet.default_edit_posture.as_str(),
            ),
        );
    }
}

fn validate_outcome_and_downgrade(
    violations: &mut Vec<GeneratedArtifactLineageViolation>,
    target: &str,
    packet: &GeneratedArtifactLineagePacket,
) {
    let healthy = packet.drift_state.is_healthy();
    let downgrade_healthy = packet.downgrade_label.is_healthy();
    match (healthy, downgrade_healthy) {
        (true, false) => {
            push_violation(
                violations,
                "packet.outcome.healthy_drift_must_not_carry_downgrade",
                target,
                "healthy drift_state must declare downgrade_label = none",
            );
        }
        (false, true) => {
            push_violation(
                violations,
                "packet.outcome.drift_must_declare_downgrade",
                target,
                "non-healthy drift_state must declare a non-none downgrade_label",
            );
        }
        _ => {}
    }
    if !downgrade_healthy {
        let has_open_gap = packet
            .open_gaps
            .iter()
            .any(|gap| gap.gap_class != LineageOpenGapClass::None);
        if !has_open_gap {
            push_violation(
                violations,
                "packet.outcome.drift_must_record_open_gap",
                target,
                "downgraded packets must record at least one open_gap with a non-none gap_class",
            );
        }
    } else if packet
        .open_gaps
        .iter()
        .any(|gap| gap.gap_class != LineageOpenGapClass::None)
    {
        push_violation(
            violations,
            "packet.outcome.healthy_must_not_record_open_gap",
            target,
            "healthy packets must not declare any open_gap with a non-none gap_class",
        );
    }
    let expected = derive_downgrade_label(packet.drift_state);
    if expected != packet.downgrade_label {
        push_violation(
            violations,
            "packet.outcome.downgrade_label_mismatch",
            target,
            format!(
                "downgrade_label must be {} for drift_state = {}; got {}",
                expected.as_str(),
                packet.drift_state.as_str(),
                packet.downgrade_label.as_str(),
            ),
        );
    }
}

fn validate_generator_identity(
    violations: &mut Vec<GeneratedArtifactLineageViolation>,
    target: &str,
    identity: &GeneratorIdentity,
) {
    if identity.generator_ref.trim().is_empty() {
        push_violation(
            violations,
            "packet.generator_identity.generator_ref",
            target,
            "generator_identity.generator_ref must be non-empty",
        );
    }
    if let Some(label) = &identity.generator_version_label {
        if label.trim().is_empty() {
            push_violation(
                violations,
                "packet.generator_identity.generator_version_label",
                target,
                "generator_identity.generator_version_label must be non-empty when present",
            );
        }
    }
}

fn validate_source_refs(
    violations: &mut Vec<GeneratedArtifactLineageViolation>,
    target: &str,
    refs: &[LineageSourceRef],
) {
    if refs.is_empty() {
        push_violation(
            violations,
            "packet.source_refs.empty",
            target,
            "source_refs must contain at least one entry",
        );
    }
    for source_ref in refs {
        if source_ref.source_path.trim().is_empty() {
            push_violation(
                violations,
                "packet.source_refs.source_path",
                target,
                "source_refs.source_path must be non-empty",
            );
        }
    }
}

fn validate_evidence_export(
    violations: &mut Vec<GeneratedArtifactLineageViolation>,
    target: &str,
    export: &LineageEvidenceExportProjection,
) {
    if !export.preserves_artifact_family_label {
        push_violation(
            violations,
            "packet.evidence_export.preserves_artifact_family_label",
            target,
            "evidence_export.preserves_artifact_family_label must be true",
        );
    }
    if !export.preserves_lineage_label {
        push_violation(
            violations,
            "packet.evidence_export.preserves_lineage_label",
            target,
            "evidence_export.preserves_lineage_label must be true",
        );
    }
    if !export.preserves_drift_state_label {
        push_violation(
            violations,
            "packet.evidence_export.preserves_drift_state_label",
            target,
            "evidence_export.preserves_drift_state_label must be true",
        );
    }
    if !export.preserves_edit_posture_label {
        push_violation(
            violations,
            "packet.evidence_export.preserves_edit_posture_label",
            target,
            "evidence_export.preserves_edit_posture_label must be true",
        );
    }
    if !export.preserves_consumer_surface_label {
        push_violation(
            violations,
            "packet.evidence_export.preserves_consumer_surface_label",
            target,
            "evidence_export.preserves_consumer_surface_label must be true",
        );
    }
    if !export.preserves_generator_identity {
        push_violation(
            violations,
            "packet.evidence_export.preserves_generator_identity",
            target,
            "evidence_export.preserves_generator_identity must be true",
        );
    }
    if !export.preserves_source_refs {
        push_violation(
            violations,
            "packet.evidence_export.preserves_source_refs",
            target,
            "evidence_export.preserves_source_refs must be true",
        );
    }
    if !export.raw_payload_excluded {
        push_violation(
            violations,
            "packet.evidence_export.raw_payload_excluded",
            target,
            "evidence_export.raw_payload_excluded must be true",
        );
    }
    if !export.raw_private_material_excluded {
        push_violation(
            violations,
            "packet.evidence_export.raw_private_material_excluded",
            target,
            "evidence_export.raw_private_material_excluded must be true",
        );
    }
    if !export.ambient_authority_excluded {
        push_violation(
            violations,
            "packet.evidence_export.ambient_authority_excluded",
            target,
            "evidence_export.ambient_authority_excluded must be true",
        );
    }
    if !export.preserves_user_authored_files {
        push_violation(
            violations,
            "packet.evidence_export.preserves_user_authored_files",
            target,
            "evidence_export.preserves_user_authored_files must be true",
        );
    }
}

fn validate_open_gaps(
    violations: &mut Vec<GeneratedArtifactLineageViolation>,
    target: &str,
    gaps: &[LineageOpenGapEntry],
) {
    let mut seen: BTreeSet<LineageOpenGapClass> = BTreeSet::new();
    for gap in gaps {
        if gap.summary.trim().is_empty() {
            push_violation(
                violations,
                "packet.open_gaps.summary",
                target,
                "open_gaps.summary must be non-empty",
            );
        }
        if !seen.insert(gap.gap_class) {
            push_violation(
                violations,
                "packet.open_gaps.duplicate_gap_class",
                target,
                format!("duplicate open_gap_class {}", gap.gap_class.as_str()),
            );
        }
    }
}

fn validate_safety(
    violations: &mut Vec<GeneratedArtifactLineageViolation>,
    target: &str,
    safety: &LineagePacketSafety,
) {
    if !safety.raw_payload_excluded {
        push_violation(
            violations,
            "packet.safety.raw_payload_excluded",
            target,
            "raw_payload_excluded must be true",
        );
    }
    if !safety.raw_private_material_excluded {
        push_violation(
            violations,
            "packet.safety.raw_private_material_excluded",
            target,
            "raw_private_material_excluded must be true",
        );
    }
    if !safety.ambient_authority_excluded {
        push_violation(
            violations,
            "packet.safety.ambient_authority_excluded",
            target,
            "ambient_authority_excluded must be true",
        );
    }
    if safety.destructive_resets_present {
        push_violation(
            violations,
            "packet.safety.destructive_resets_present",
            target,
            "destructive_resets_present must be false",
        );
    }
    if !safety.preserves_user_authored_files {
        push_violation(
            violations,
            "packet.safety.preserves_user_authored_files",
            target,
            "preserves_user_authored_files must be true",
        );
    }
}

fn validate_references(
    violations: &mut Vec<GeneratedArtifactLineageViolation>,
    target: &str,
    refs: &LineagePacketReferences,
) {
    if refs.doc_ref != GENERATED_ARTIFACT_LINEAGE_DOC_REF {
        push_violation(
            violations,
            "packet.references.doc_ref",
            target,
            format!("references.doc_ref must pin {GENERATED_ARTIFACT_LINEAGE_DOC_REF}"),
        );
    }
    if refs.schema_ref != GENERATED_ARTIFACT_LINEAGE_SCHEMA_REF {
        push_violation(
            violations,
            "packet.references.schema_ref",
            target,
            format!("references.schema_ref must pin {GENERATED_ARTIFACT_LINEAGE_SCHEMA_REF}"),
        );
    }
    if refs.report_ref != GENERATED_ARTIFACT_LINEAGE_REPORT_REF {
        push_violation(
            violations,
            "packet.references.report_ref",
            target,
            format!("references.report_ref must pin {GENERATED_ARTIFACT_LINEAGE_REPORT_REF}"),
        );
    }
}

fn push_violation(
    violations: &mut Vec<GeneratedArtifactLineageViolation>,
    check_id: impl Into<String>,
    subject_ref: impl Into<String>,
    message: impl Into<String>,
) {
    violations.push(GeneratedArtifactLineageViolation {
        check_id: check_id.into(),
        subject_ref: subject_ref.into(),
        message: message.into(),
    });
}

/// Re-derives the default edit posture from the lineage class.
pub const fn derive_default_edit_posture(lineage: LineageClass) -> DefaultEditPosture {
    match lineage {
        LineageClass::CanonicalSource => DefaultEditPosture::EditableCanonical,
        LineageClass::GeneratedFromLocalSource => DefaultEditPosture::ReadOnlyGenerated,
        LineageClass::RegenerableLockfileArtifact => DefaultEditPosture::RegenerateOnly,
        LineageClass::MirroredFromLocalSource => DefaultEditPosture::ReadOnlyGenerated,
        LineageClass::ImportedExternalArtifact => DefaultEditPosture::ImportedReadOnly,
        LineageClass::DerivedFromRunArtifact => DefaultEditPosture::TransientRunArtifact,
        LineageClass::PreviewedFromLocalSource => DefaultEditPosture::RegenerateOnly,
        LineageClass::UnknownLineage => DefaultEditPosture::ReviewRequiredBeforeEdit,
    }
}

/// Re-derives the downgrade label from the drift state.
pub const fn derive_downgrade_label(drift: DriftState) -> LineageDowngradeLabel {
    match drift {
        DriftState::Aligned => LineageDowngradeLabel::None,
        DriftState::ImportedNoLocalSource => LineageDowngradeLabel::None,
        DriftState::SourceDrifted => LineageDowngradeLabel::YellowDriftPending,
        DriftState::RegenPending => LineageDowngradeLabel::YellowDriftPending,
        DriftState::StaleGenerated => LineageDowngradeLabel::YellowDriftPending,
        DriftState::GeneratorMissing => LineageDowngradeLabel::YellowGeneratorUnknown,
        DriftState::OutOfScope => LineageDowngradeLabel::DegradedToMetadataOnly,
    }
}

/// Loads a YAML-encoded [`GeneratedArtifactLineagePacket`].
pub fn load_generated_artifact_lineage_packet(
    yaml: &str,
) -> Result<GeneratedArtifactLineagePacket, serde_yaml::Error> {
    serde_yaml::from_str(yaml)
}

/// Returns the checked-in generated-artifact lineage corpus loaded
/// from the embedded fixtures.
pub fn current_generated_artifact_lineage_corpus(
) -> Result<GeneratedArtifactLineageCorpus, serde_yaml::Error> {
    let entries = PACKET_FIXTURES
        .iter()
        .map(|(fixture_ref, yaml)| {
            serde_yaml::from_str::<GeneratedArtifactLineagePacket>(yaml).map(|packet| {
                GeneratedArtifactLineageCorpusEntry {
                    fixture_ref: (*fixture_ref).to_owned(),
                    packet,
                }
            })
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(GeneratedArtifactLineageCorpus { entries })
}

/// Returns the set of fixture refs the corpus loads, in declaration
/// order.
pub fn current_generated_artifact_lineage_fixture_refs() -> impl Iterator<Item = &'static str> {
    PACKET_FIXTURES.iter().map(|(fixture_ref, _)| *fixture_ref)
}

const PACKET_FIXTURES: &[(&str, &str)] = &[
    (
        "fixtures/state/generated_artifacts_beta/search_build_output_aligned_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/generated_artifacts_beta/search_build_output_aligned_packet.yaml"
        )),
    ),
    (
        "fixtures/state/generated_artifacts_beta/review_lockfile_aligned_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/generated_artifacts_beta/review_lockfile_aligned_packet.yaml"
        )),
    ),
    (
        "fixtures/state/generated_artifacts_beta/ai_context_preview_aligned_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/generated_artifacts_beta/ai_context_preview_aligned_packet.yaml"
        )),
    ),
    (
        "fixtures/state/generated_artifacts_beta/support_export_notebook_aligned_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/generated_artifacts_beta/support_export_notebook_aligned_packet.yaml"
        )),
    ),
    (
        "fixtures/state/generated_artifacts_beta/support_export_run_result_aligned_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/generated_artifacts_beta/support_export_run_result_aligned_packet.yaml"
        )),
    ),
    (
        "fixtures/state/generated_artifacts_beta/review_build_output_source_drifted_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/generated_artifacts_beta/review_build_output_source_drifted_packet.yaml"
        )),
    ),
    (
        "fixtures/state/generated_artifacts_beta/ai_context_build_output_stale_generated_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/generated_artifacts_beta/ai_context_build_output_stale_generated_packet.yaml"
        )),
    ),
    (
        "fixtures/state/generated_artifacts_beta/search_build_output_imported_external_packet.yaml",
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../fixtures/state/generated_artifacts_beta/search_build_output_imported_external_packet.yaml"
        )),
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn aligned_packet() -> GeneratedArtifactLineagePacket {
        GeneratedArtifactLineagePacket {
            schema_version: GENERATED_ARTIFACT_LINEAGE_SCHEMA_VERSION,
            record_kind: GENERATED_ARTIFACT_LINEAGE_RECORD_KIND.to_owned(),
            packet_id: "lineage:test:aligned".to_owned(),
            title: "Aligned test packet".to_owned(),
            consumer_surface: LineageConsumerSurface::Search,
            artifact_family: ArtifactFamily::BuildOutput,
            artifact_ref: "workspace:test/target/debug/app".to_owned(),
            workspace_id: "workspace:test".to_owned(),
            lineage_class: LineageClass::GeneratedFromLocalSource,
            drift_state: DriftState::Aligned,
            default_edit_posture: DefaultEditPosture::ReadOnlyGenerated,
            downgrade_label: LineageDowngradeLabel::None,
            generator_identity: GeneratorIdentity {
                generator_kind: GeneratorKind::BuildSystem,
                generator_ref: "build:cargo".to_owned(),
                generator_version_label: Some("cargo-1.79.0".to_owned()),
            },
            source_refs: vec![LineageSourceRef {
                source_kind: SourceKind::LocalSourceFile,
                source_path: "workspace:test/src/main.rs".to_owned(),
            }],
            evidence_export: LineageEvidenceExportProjection::metadata_safe_baseline(),
            open_gaps: Vec::new(),
            safety: LineagePacketSafety::metadata_safe_baseline(),
            references: LineagePacketReferences::pinned(),
            captured_at: "2026-05-16T10:00:00Z".to_owned(),
            reviewer_summary: None,
        }
    }

    #[test]
    fn aligned_packet_validates() {
        GeneratedArtifactLineageEvaluator::new()
            .validate_packet(&aligned_packet())
            .expect("aligned packet must validate");
    }

    #[test]
    fn refuses_aligned_with_downgrade() {
        let mut packet = aligned_packet();
        packet.downgrade_label = LineageDowngradeLabel::YellowDriftPending;
        let err = GeneratedArtifactLineageEvaluator::new()
            .validate_packet(&packet)
            .expect_err("aligned with downgrade must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "packet.outcome.healthy_drift_must_not_carry_downgrade"));
    }

    #[test]
    fn refuses_edit_posture_mismatch() {
        let mut packet = aligned_packet();
        packet.default_edit_posture = DefaultEditPosture::EditableCanonical;
        let err = GeneratedArtifactLineageEvaluator::new()
            .validate_packet(&packet)
            .expect_err("mismatched edit posture must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "packet.default_edit_posture.derived_mismatch"));
    }

    #[test]
    fn refuses_downgrade_mismatch() {
        let mut packet = aligned_packet();
        packet.drift_state = DriftState::SourceDrifted;
        packet.downgrade_label = LineageDowngradeLabel::RedBlocksBetaRow;
        packet.open_gaps.push(LineageOpenGapEntry {
            gap_class: LineageOpenGapClass::RegenPending,
            summary: "regen pending".to_owned(),
        });
        let err = GeneratedArtifactLineageEvaluator::new()
            .validate_packet(&packet)
            .expect_err("mismatched downgrade must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "packet.outcome.downgrade_label_mismatch"));
    }

    #[test]
    fn refuses_destructive_reset() {
        let mut packet = aligned_packet();
        packet.safety.destructive_resets_present = true;
        let err = GeneratedArtifactLineageEvaluator::new()
            .validate_packet(&packet)
            .expect_err("destructive reset must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "packet.safety.destructive_resets_present"));
    }

    #[test]
    fn refuses_imported_lineage_without_imported_drift() {
        let mut packet = aligned_packet();
        packet.lineage_class = LineageClass::ImportedExternalArtifact;
        packet.default_edit_posture = DefaultEditPosture::ImportedReadOnly;
        // drift_state still Aligned — invalid for imported lineage.
        let err = GeneratedArtifactLineageEvaluator::new()
            .validate_packet(&packet)
            .expect_err("imported lineage must require imported_no_local_source");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "packet.drift.imported_must_pin_imported_no_local_source"));
    }

    #[test]
    fn refuses_corpus_without_drift_row() {
        let corpus = GeneratedArtifactLineageCorpus {
            entries: vec![GeneratedArtifactLineageCorpusEntry {
                fixture_ref: "fixtures/test/only_aligned.yaml".to_owned(),
                packet: aligned_packet(),
            }],
        };
        let err = GeneratedArtifactLineageEvaluator::new()
            .validate_corpus(&corpus)
            .expect_err("corpus without drift row must fail");
        assert!(err
            .violations
            .iter()
            .any(|v| v.check_id == "corpus.no_drift_row"));
    }

    #[test]
    fn checked_in_corpus_loads_and_validates() {
        let corpus =
            current_generated_artifact_lineage_corpus().expect("checked-in corpus must parse");
        GeneratedArtifactLineageEvaluator::new()
            .validate_corpus(&corpus)
            .expect("checked-in corpus must validate");
        for surface in REQUIRED_LINEAGE_CONSUMER_SURFACES {
            assert!(
                corpus
                    .entries
                    .iter()
                    .any(|entry| entry.packet.consumer_surface == surface),
                "checked-in corpus must seed a packet for consumer_surface = {}",
                surface.as_str()
            );
        }
        for family in REQUIRED_ARTIFACT_FAMILIES {
            assert!(
                corpus
                    .entries
                    .iter()
                    .any(|entry| entry.packet.artifact_family == family),
                "checked-in corpus must seed a packet for artifact_family = {}",
                family.as_str()
            );
        }
        for lineage in REQUIRED_LINEAGE_CLASSES {
            assert!(
                corpus
                    .entries
                    .iter()
                    .any(|entry| entry.packet.lineage_class == lineage),
                "checked-in corpus must seed a packet for lineage_class = {}",
                lineage.as_str()
            );
        }
    }

    #[test]
    fn report_is_export_safe() {
        let corpus = current_generated_artifact_lineage_corpus().unwrap();
        let report = GeneratedArtifactLineageEvaluator::new()
            .report("report:test", "2026-05-16T00:00:00Z", &corpus)
            .expect("report builds");
        assert!(report.is_export_safe());
        assert_eq!(report.matrix_rows.len(), corpus.entries.len());
        assert_eq!(
            report.artifact_family_summaries.len(),
            REQUIRED_ARTIFACT_FAMILIES.len()
        );
        assert_eq!(
            report.lineage_summaries.len(),
            REQUIRED_LINEAGE_CLASSES.len()
        );
        assert_eq!(
            report.consumer_surface_summaries.len(),
            REQUIRED_LINEAGE_CONSUMER_SURFACES.len()
        );
    }
}
