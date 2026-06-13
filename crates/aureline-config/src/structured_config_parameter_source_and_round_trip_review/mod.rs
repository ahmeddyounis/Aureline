//! Canonical M5 parameter-source, secret/reference, round-trip-review, and
//! export-review truth for structured config families.
//!
//! This packet is the next layer above the M4 structured-editor qualification
//! record and the M5 family/mode freezes:
//!
//! - per-parameter rows keep key/path identity, masked display, source class,
//!   resolution time, winner, override action, and copy/export posture in one
//!   shared shape;
//! - value chips distinguish literal values, env refs, secret handles,
//!   policy-injected values, and runtime-discovered values without implying
//!   that raw secret material is portable;
//! - round-trip banners and compare-before-save sheets keep comment, unknown
//!   key, ordering, and extension-namespace loss visible before any structured
//!   rewrite lands; and
//! - effective-value review sheets and export summaries disclose whether a
//!   surface contains literal values, references/handles, redacted
//!   placeholders, or only key-path metadata.
//!
//! The packet is metadata-only. It reuses the family and surface vocabulary
//! already frozen by [`crate::structured_config_artifact_modes_and_layers`] and
//! [`crate::structured_config_policy_bundle_and_entitlement_matrix`].

use std::collections::BTreeSet;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::structured_config_artifact_modes_and_layers::{
    ConsumerSurfaceClass, STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_SHARED_CONTRACT_REF,
};
use crate::structured_config_policy_bundle_and_entitlement_matrix::{
    ArtifactFamilyKind, QualificationLabel,
    STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_SHARED_CONTRACT_REF,
};

#[cfg(test)]
mod tests;

/// Stable record-kind tag for the canonical review packet.
pub const STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_RECORD_KIND: &str =
    "structured_config_parameter_source_and_round_trip_review";

/// Schema version for [`StructuredConfigParameterSourceRoundTripReviewPacket`].
pub const STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref quoted by every consumer.
pub const STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_SHARED_CONTRACT_REF: &str =
    "config:structured_config_parameter_source_and_round_trip_review:v1";

/// Repo-relative path to the checked-in canonical packet.
pub const STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_PATH: &str =
    "artifacts/config/structured_config_parameter_source_and_round_trip_review.json";

/// Reviewer-facing notice repeated on UI, CLI inspect, docs/help, and support.
pub const STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_NOTICE: &str =
    "Structured-config parameter-source and save-review truth stays explicit: every per-value row \
     shows key/path identity, masked display, source class, resolution time, winner, override \
     action, and copy/export posture; value chips distinguish literal values, env refs, secret \
     handles, policy-injected values, and runtime-discovered values without widening access to raw \
     secrets; round-trip banners and compare-before-save sheets keep comment, unknown-key, \
     ordering, and extension-namespace rewrite risk visible before mutation; and effective-value \
     review plus export summaries reuse the same disclosure classes so support/export never imply \
     a safer or richer payload than the editor actually has.";

const REQUIRED_EDITOR_FAMILIES: [ArtifactFamilyKind; 9] = [
    ArtifactFamilyKind::RequestWorkspaceEnvironment,
    ArtifactFamilyKind::DatabaseProfile,
    ArtifactFamilyKind::ApiProfile,
    ArtifactFamilyKind::NotebookRuntimeManifest,
    ArtifactFamilyKind::PreviewRuntimeConfig,
    ArtifactFamilyKind::WorkflowBundleManifest,
    ArtifactFamilyKind::CiEnvironmentDescriptor,
    ArtifactFamilyKind::InfraEnvironmentDescriptor,
    ArtifactFamilyKind::ManagedPolicyOverlay,
];

/// Field every parameter-source row must expose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ParameterRowField {
    /// Stable key/path identity.
    KeyPath,
    /// Masked or placeholder display value.
    DisplayValue,
    /// Literal vs reference/handle vs discovered class.
    SourceClass,
    /// When the value becomes concrete.
    ResolutionTime,
    /// Which layer won or why the row is unresolved.
    Winner,
    /// Layer-bounded override/remove/reset action.
    OverrideAction,
    /// Copy/export posture shown on the row.
    CopyExportPosture,
}

impl ParameterRowField {
    /// All required row fields.
    pub const ALL: [Self; 7] = [
        Self::KeyPath,
        Self::DisplayValue,
        Self::SourceClass,
        Self::ResolutionTime,
        Self::Winner,
        Self::OverrideAction,
        Self::CopyExportPosture,
    ];
}

/// Shared value chip class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ValueChipClass {
    /// Raw literal value stored in the authored source.
    LiteralValue,
    /// Environment or variable reference.
    EnvReference,
    /// Secret handle or vault/keychain alias.
    SecretHandle,
    /// Policy-injected or policy-owned value.
    PolicyInjected,
    /// Runtime-discovered or observed value.
    RuntimeDiscovered,
}

impl ValueChipClass {
    /// All required chip classes.
    pub const ALL: [Self; 5] = [
        Self::LiteralValue,
        Self::EnvReference,
        Self::SecretHandle,
        Self::PolicyInjected,
        Self::RuntimeDiscovered,
    ];
}

/// Output disclosure class reused by effective-value reviews and exports.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputDisclosureClass {
    /// Export or review contains a literal value.
    LiteralValue,
    /// Export or review contains a reference or handle rather than raw bytes.
    ReferenceHandle,
    /// Export or review contains a redacted placeholder.
    RedactedPlaceholder,
    /// Export or review contains only key-path or metadata identity.
    KeyPathMetadataOnly,
}

impl OutputDisclosureClass {
    /// All required disclosure classes.
    pub const ALL: [Self; 4] = [
        Self::LiteralValue,
        Self::ReferenceHandle,
        Self::RedactedPlaceholder,
        Self::KeyPathMetadataOnly,
    ];
}

/// Copy/export posture shown on a row, chip, or review summary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CopyExportPosture {
    /// Literal value may be copied only after local-only reveal/review.
    LocalLiteralAfterReview,
    /// Only the reference or handle may be copied/exported.
    ReferenceHandleOnly,
    /// Only a redacted placeholder may be copied/exported.
    RedactedPlaceholderOnly,
    /// Only the key/path metadata may be copied/exported.
    KeyPathMetadataOnly,
    /// Only a metadata-and-sources summary may be copied/exported.
    MetadataSummaryOnly,
}

/// Action rendered on a parameter row or review sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewActionClass {
    /// Reset only the current layer.
    ResetCurrentLayer,
    /// Clear the current override.
    ClearOverride,
    /// Reveal or inspect locally with friction.
    RevealLocally,
    /// Open the canonical source.
    OpenSource,
    /// Open the policy source or signed bundle.
    ViewPolicySource,
    /// Explicit compare-before-save checkpoint.
    CompareBeforeSave,
    /// Compare against live or observed state.
    CompareLive,
}

/// Round-trip-loss class shown on risk banners and compare sheets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundTripRiskFlag {
    /// Comments may not survive the structured rewrite.
    CommentsMayRewrite,
    /// Unknown keys may be reordered or dropped.
    UnknownKeysMayRewrite,
    /// Authored key ordering may be normalized.
    OrderingMayRewrite,
    /// Extension namespaces may be rewritten or narrowed.
    ExtensionNamespacesMayRewrite,
}

impl RoundTripRiskFlag {
    /// All required risk flags covered by the packet.
    pub const ALL: [Self; 4] = [
        Self::CommentsMayRewrite,
        Self::UnknownKeysMayRewrite,
        Self::OrderingMayRewrite,
        Self::ExtensionNamespacesMayRewrite,
    ];
}

/// Shared field definition for parameter-source rows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterRowFieldDefinition {
    /// Stable field token.
    pub field: ParameterRowField,
    /// Reviewer-facing label.
    pub label: String,
    /// Explanation reused across surfaces.
    pub description: String,
}

/// Shared chip vocabulary row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueChipVocabularyRow {
    /// Stable chip class.
    pub chip_class: ValueChipClass,
    /// Reviewer-facing label.
    pub label: String,
    /// Explanation reused across surfaces.
    pub description: String,
    /// True when the default export blocks raw secret material for this class.
    pub raw_secret_export_blocked_by_default: bool,
}

/// Shared disclosure vocabulary row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OutputDisclosureVocabularyRow {
    /// Stable disclosure class.
    pub output_class: OutputDisclosureClass,
    /// Reviewer-facing label.
    pub label: String,
    /// Explanation reused across surfaces.
    pub description: String,
}

/// Shared surface-parity binding for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceVocabularyBinding {
    /// Surface consuming the packet.
    pub surface: ConsumerSurfaceClass,
    /// Shared contract ref rendered by the surface.
    pub shared_contract_ref: String,
    /// Whether parameter rows are rendered.
    pub renders_parameter_rows: bool,
    /// Whether chips are rendered.
    pub renders_value_chips: bool,
    /// Whether risk banners are rendered.
    pub renders_round_trip_banner: bool,
    /// Whether compare-before-save sheets are rendered.
    pub renders_compare_before_save_sheet: bool,
    /// Whether effective-value review sheets are rendered.
    pub renders_effective_value_review_sheet: bool,
    /// Whether export summaries are rendered.
    pub renders_export_summary: bool,
}

/// Action row reused across parameter and review surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewActionRow {
    /// Stable action class.
    pub action_class: ReviewActionClass,
    /// Exact action label.
    pub action_label: String,
}

/// One per-parameter source row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ParameterSourceRow {
    /// Stable key or path identity.
    pub key_path: String,
    /// Masked, redacted, or placeholder display.
    pub display_value: String,
    /// Literal/reference/discovered class.
    pub source_class: ValueChipClass,
    /// When the value becomes concrete.
    pub resolution_time_label: String,
    /// Winner or provenance outcome.
    pub winner_label: String,
    /// Layer-bounded action.
    pub override_action: ReviewActionRow,
    /// Copy/export posture for this row.
    pub copy_export_posture: CopyExportPosture,
    /// Whether this row is the effective winner.
    pub wins_effective_value: bool,
}

/// One visible value chip.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ValueChipRow {
    /// Stable key or path identity.
    pub key_path: String,
    /// Chip class.
    pub chip_class: ValueChipClass,
    /// Visible chip label.
    pub label: String,
    /// Copy/export posture for the chip.
    pub copy_export_posture: CopyExportPosture,
    /// Review/reveal posture shown beside the chip.
    pub reveal_posture_label: String,
    /// Whether raw secret material is blocked by default.
    pub raw_secret_export_blocked_by_default: bool,
}

/// Banner shown when structured save may rewrite semantics or structure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RoundTripRiskBanner {
    /// Exact reviewer-facing risk summary.
    pub risk_summary: String,
    /// Affected artifact or scope label.
    pub affected_scope_label: String,
    /// Recommended safer path.
    pub safe_path_label: String,
    /// Explicit review action before mutation.
    pub review_action: ReviewActionRow,
    /// Structural-loss classes the banner covers.
    pub risk_flags: Vec<RoundTripRiskFlag>,
}

/// Compare-before-save review sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareBeforeSaveSheet {
    /// Stable review-sheet id.
    pub sheet_id: String,
    /// Artifact scope or fragment label.
    pub artifact_scope_label: String,
    /// Key set shown on the sheet.
    pub selected_key_set: Vec<String>,
    /// Structural-loss classes under review.
    pub risk_flags: Vec<RoundTripRiskFlag>,
    /// Exact limitation summary.
    pub limitation_summary: String,
    /// Recommended fallback path.
    pub fallback_path_label: String,
    /// Explicit review action label.
    pub review_action: ReviewActionRow,
    /// True when the user must acknowledge the review before save.
    pub requires_explicit_acknowledgement: bool,
    /// Opaque compare artifact ref.
    pub compare_ref: String,
}

/// Effective-value review sheet reused before export or support capture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveValueReviewSheet {
    /// Selected keys shown on the sheet.
    pub key_set: Vec<String>,
    /// Winning layers or sources shown on the sheet.
    pub winning_layers: Vec<String>,
    /// Unresolved or deferred rows shown on the sheet.
    pub unresolved: Vec<String>,
    /// Human-readable export posture label.
    pub export_posture_label: String,
    /// Disclosure classes the sheet explicitly names.
    pub output_disclosure_classes: Vec<OutputDisclosureClass>,
    /// Bounded actions available from the sheet.
    pub actions: Vec<ReviewActionRow>,
}

/// Export/support summary reused across flows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportSummary {
    /// Stable summary id.
    pub summary_id: String,
    /// Human-readable posture summary.
    pub posture_label: String,
    /// Disclosure classes the summary explicitly names.
    pub output_disclosure_classes: Vec<OutputDisclosureClass>,
    /// Support/export-safe lines reused across surfaces.
    pub summary_lines: Vec<String>,
}

/// One artifact-family review row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewRow {
    /// Structured-config family under review.
    pub family: ArtifactFamilyKind,
    /// Public qualification label inherited from the family matrix.
    pub qualification_label: QualificationLabel,
    /// Opaque ref into the artifact-mode packet or family matrix.
    pub artifact_surface_ref: String,
    /// Per-parameter source rows.
    pub parameter_source_rows: Vec<ParameterSourceRow>,
    /// Value chips rendered for the artifact.
    pub value_chips: Vec<ValueChipRow>,
    /// Round-trip-risk banner when structured save is risky.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub round_trip_risk_banner: Option<RoundTripRiskBanner>,
    /// Compare-before-save sheet when structured save is risky.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub compare_before_save_sheet: Option<CompareBeforeSaveSheet>,
    /// Effective-value review sheet reused before export/save.
    pub effective_value_review_sheet: EffectiveValueReviewSheet,
    /// Export summary reused by export and support flows.
    pub export_summary: ExportSummary,
    /// True when support/export reuses the same summary metadata.
    pub support_export_reuses_export_summary: bool,
}

/// Derived packet summary for release review and invariant checks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PacketSummary {
    /// Total artifact-review rows.
    pub artifact_review_count: usize,
    /// Total parameter-source rows.
    pub parameter_source_row_count: usize,
    /// Families with a round-trip-risk banner.
    pub family_count_with_round_trip_banner: usize,
    /// Families with a compare-before-save sheet.
    pub family_count_with_compare_before_save: usize,
    /// Families with an effective-value review sheet.
    pub family_count_with_effective_value_review: usize,
    /// Families whose export summary is explicitly reused by support export.
    pub family_count_reusing_export_summary: usize,
    /// Coverage count of the five chip classes.
    pub chip_class_coverage_count: usize,
    /// Coverage count of the four output disclosure classes.
    pub output_disclosure_coverage_count: usize,
    /// Required shared surfaces covered by the packet.
    pub shared_surface_count: usize,
    /// True when raw secret export stays blocked everywhere it should.
    pub raw_secret_export_blocked_everywhere: bool,
    /// True when support/export always reuses the same summary metadata.
    pub support_export_metadata_reused_everywhere: bool,
}

/// Canonical packet consumed by editor, CLI inspect, help/docs, and support.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StructuredConfigParameterSourceRoundTripReviewPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Reviewer-facing notice.
    pub notice: String,
    /// Upstream family matrix contract ref.
    pub upstream_family_matrix_ref: String,
    /// Upstream artifact-mode contract ref.
    pub upstream_artifact_modes_ref: String,
    /// Parameter-row vocabulary.
    pub parameter_row_vocabulary: Vec<ParameterRowFieldDefinition>,
    /// Value-chip vocabulary.
    pub value_chip_vocabulary: Vec<ValueChipVocabularyRow>,
    /// Output-disclosure vocabulary.
    pub output_disclosure_vocabulary: Vec<OutputDisclosureVocabularyRow>,
    /// Surface-parity bindings.
    pub surface_vocabulary: Vec<SurfaceVocabularyBinding>,
    /// One review row per structured-config family.
    pub artifact_reviews: Vec<ArtifactReviewRow>,
    /// Derived summary.
    pub summary: PacketSummary,
    /// Narrative doc ref.
    pub docs_ref: String,
    /// Boundary schema ref.
    pub schema_ref: String,
}

impl StructuredConfigParameterSourceRoundTripReviewPacket {
    /// Returns support/export-safe summary lines.
    pub fn support_export_lines(&self) -> Vec<String> {
        vec![
            format!("packet_id: {}", self.packet_id),
            format!(
                "artifact_review_count: {}",
                self.summary.artifact_review_count
            ),
            format!(
                "parameter_source_row_count: {}",
                self.summary.parameter_source_row_count
            ),
            format!(
                "compare_before_save_families: {}",
                self.summary.family_count_with_compare_before_save
            ),
            format!(
                "chip_class_coverage_count: {}",
                self.summary.chip_class_coverage_count
            ),
            format!(
                "output_disclosure_coverage_count: {}",
                self.summary.output_disclosure_coverage_count
            ),
            format!(
                "raw_secret_export_blocked_everywhere: {}",
                self.summary.raw_secret_export_blocked_everywhere
            ),
            format!(
                "support_export_metadata_reused_everywhere: {}",
                self.summary.support_export_metadata_reused_everywhere
            ),
        ]
    }
}

/// Validation defect returned by [`audit_structured_config_parameter_source_and_round_trip_review`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PacketValidationError {
    /// A required family is missing.
    MissingFamily(ArtifactFamilyKind),
    /// A family appears more than once.
    DuplicateFamily(ArtifactFamilyKind),
    /// A required shared surface is missing.
    MissingSurface(ConsumerSurfaceClass),
    /// A vocabulary row set is incomplete.
    IncompleteVocabulary(&'static str),
    /// A parameter row is malformed.
    InvalidParameterRow {
        /// Family owning the row.
        family: ArtifactFamilyKind,
        /// Key/path identity.
        key_path: String,
    },
    /// A family omits an effective winner.
    MissingEffectiveWinner(ArtifactFamilyKind),
    /// A chip row is malformed.
    InvalidValueChip {
        /// Family owning the chip.
        family: ArtifactFamilyKind,
        /// Key/path identity.
        key_path: String,
    },
    /// A family leaked raw secret export posture.
    RawSecretExportNotBlocked(ArtifactFamilyKind),
    /// Compare-before-save sheet and banner are inconsistent.
    InconsistentCompareReview(ArtifactFamilyKind),
    /// Effective review and export summary do not align.
    ExportSummaryMismatch(ArtifactFamilyKind),
    /// Summary count drifted from derived truth.
    SummaryCountMismatch {
        /// Field name.
        field: &'static str,
        /// Derived value.
        expected: usize,
        /// Stored value.
        actual: usize,
    },
    /// Summary flag drifted from derived truth.
    SummaryFlagMismatch {
        /// Field name.
        field: &'static str,
        /// Derived value.
        expected: bool,
        /// Stored value.
        actual: bool,
    },
}

impl fmt::Display for PacketValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingFamily(family) => {
                write!(f, "missing artifact review row for {family:?}")
            }
            Self::DuplicateFamily(family) => {
                write!(f, "duplicate artifact review row for {family:?}")
            }
            Self::MissingSurface(surface) => write!(f, "missing surface binding for {surface:?}"),
            Self::IncompleteVocabulary(section) => {
                write!(f, "incomplete vocabulary section: {section}")
            }
            Self::InvalidParameterRow { family, key_path } => {
                write!(f, "invalid parameter row for {family:?} / {key_path}")
            }
            Self::MissingEffectiveWinner(family) => {
                write!(f, "{family:?} does not expose an effective winner")
            }
            Self::InvalidValueChip { family, key_path } => {
                write!(f, "invalid value chip for {family:?} / {key_path}")
            }
            Self::RawSecretExportNotBlocked(family) => {
                write!(f, "{family:?} exposes raw secret export")
            }
            Self::InconsistentCompareReview(family) => {
                write!(f, "{family:?} compare-before-save review is inconsistent")
            }
            Self::ExportSummaryMismatch(family) => {
                write!(
                    f,
                    "{family:?} export summary does not align with the review sheet"
                )
            }
            Self::SummaryCountMismatch {
                field,
                expected,
                actual,
            } => write!(
                f,
                "summary count {field} expected {expected} but found {actual}"
            ),
            Self::SummaryFlagMismatch {
                field,
                expected,
                actual,
            } => write!(
                f,
                "summary flag {field} expected {expected} but found {actual}"
            ),
        }
    }
}

impl std::error::Error for PacketValidationError {}

/// Returns the deterministic canonical packet.
pub fn seeded_structured_config_parameter_source_and_round_trip_review(
) -> StructuredConfigParameterSourceRoundTripReviewPacket {
    let parameter_row_vocabulary = seeded_parameter_row_vocabulary();
    let value_chip_vocabulary = seeded_value_chip_vocabulary();
    let output_disclosure_vocabulary = seeded_output_disclosure_vocabulary();
    let surface_vocabulary = seeded_surface_vocabulary();
    let artifact_reviews = seeded_artifact_reviews();
    let summary = derive_summary(&surface_vocabulary, &artifact_reviews);

    StructuredConfigParameterSourceRoundTripReviewPacket {
        record_kind: STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_RECORD_KIND.to_owned(),
        schema_version: STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_SCHEMA_VERSION,
        packet_id: "config:structured-config-parameter-source-round-trip-review".to_owned(),
        shared_contract_ref:
            STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_SHARED_CONTRACT_REF.to_owned(),
        notice: STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_NOTICE.to_owned(),
        upstream_family_matrix_ref: STRUCTURED_CONFIG_POLICY_ENTITLEMENT_MATRIX_SHARED_CONTRACT_REF
            .to_owned(),
        upstream_artifact_modes_ref:
            STRUCTURED_CONFIG_ARTIFACT_MODES_AND_LAYERS_SHARED_CONTRACT_REF.to_owned(),
        parameter_row_vocabulary,
        value_chip_vocabulary,
        output_disclosure_vocabulary,
        surface_vocabulary,
        artifact_reviews,
        summary,
        docs_ref: "docs/config/structured_config_parameter_source_and_round_trip_review.md"
            .to_owned(),
        schema_ref:
            "schemas/config/structured_config_parameter_source_and_round_trip_review.schema.json"
                .to_owned(),
    }
}

/// Parses a packet from JSON text.
pub fn parse_structured_config_parameter_source_and_round_trip_review(
    json: &str,
) -> Result<StructuredConfigParameterSourceRoundTripReviewPacket, serde_json::Error> {
    serde_json::from_str(json)
}

/// Audits the packet and returns every defect found.
pub fn audit_structured_config_parameter_source_and_round_trip_review(
    packet: &StructuredConfigParameterSourceRoundTripReviewPacket,
) -> Vec<PacketValidationError> {
    let mut defects = Vec::new();

    append_presence_defects(
        &mut defects,
        &packet.artifact_reviews,
        REQUIRED_EDITOR_FAMILIES.as_slice(),
        |row| row.family,
        PacketValidationError::MissingFamily,
        PacketValidationError::DuplicateFamily,
    );
    append_surface_presence_defects(&mut defects, &packet.surface_vocabulary);

    if !covers_all(
        packet.parameter_row_vocabulary.iter().map(|row| row.field),
        ParameterRowField::ALL,
    ) {
        defects.push(PacketValidationError::IncompleteVocabulary(
            "parameter_row_vocabulary",
        ));
    }
    if !covers_all(
        packet
            .value_chip_vocabulary
            .iter()
            .map(|row| row.chip_class),
        ValueChipClass::ALL,
    ) {
        defects.push(PacketValidationError::IncompleteVocabulary(
            "value_chip_vocabulary",
        ));
    }
    if !covers_all(
        packet
            .output_disclosure_vocabulary
            .iter()
            .map(|row| row.output_class),
        OutputDisclosureClass::ALL,
    ) {
        defects.push(PacketValidationError::IncompleteVocabulary(
            "output_disclosure_vocabulary",
        ));
    }

    for binding in &packet.surface_vocabulary {
        if binding.shared_contract_ref != packet.shared_contract_ref
            || !binding.renders_parameter_rows
            || !binding.renders_value_chips
            || !binding.renders_round_trip_banner
            || !binding.renders_compare_before_save_sheet
            || !binding.renders_effective_value_review_sheet
            || !binding.renders_export_summary
        {
            defects.push(PacketValidationError::IncompleteVocabulary(
                "surface_vocabulary",
            ));
        }
    }

    for review in &packet.artifact_reviews {
        if review.artifact_surface_ref.trim().is_empty() || review.parameter_source_rows.is_empty()
        {
            defects.push(PacketValidationError::InvalidParameterRow {
                family: review.family,
                key_path: "<missing>".to_owned(),
            });
        }

        if !review
            .parameter_source_rows
            .iter()
            .any(|row| row.wins_effective_value)
        {
            defects.push(PacketValidationError::MissingEffectiveWinner(review.family));
        }

        for row in &review.parameter_source_rows {
            if row.key_path.trim().is_empty()
                || row.display_value.trim().is_empty()
                || row.resolution_time_label.trim().is_empty()
                || row.winner_label.trim().is_empty()
                || row.override_action.action_label.trim().is_empty()
            {
                defects.push(PacketValidationError::InvalidParameterRow {
                    family: review.family,
                    key_path: row.key_path.clone(),
                });
            }
        }

        if review.value_chips.is_empty() {
            defects.push(PacketValidationError::InvalidValueChip {
                family: review.family,
                key_path: "<missing>".to_owned(),
            });
        }

        for chip in &review.value_chips {
            if chip.key_path.trim().is_empty()
                || chip.label.trim().is_empty()
                || chip.reveal_posture_label.trim().is_empty()
            {
                defects.push(PacketValidationError::InvalidValueChip {
                    family: review.family,
                    key_path: chip.key_path.clone(),
                });
            }
            if matches!(
                chip.chip_class,
                ValueChipClass::SecretHandle | ValueChipClass::PolicyInjected
            ) && !chip.raw_secret_export_blocked_by_default
            {
                defects.push(PacketValidationError::RawSecretExportNotBlocked(
                    review.family,
                ));
            }
        }

        match (
            review.round_trip_risk_banner.as_ref(),
            review.compare_before_save_sheet.as_ref(),
        ) {
            (Some(banner), Some(sheet)) => {
                if banner.risk_summary.trim().is_empty()
                    || banner.affected_scope_label.trim().is_empty()
                    || banner.safe_path_label.trim().is_empty()
                    || banner.review_action.action_label.trim().is_empty()
                    || banner.risk_flags.is_empty()
                    || sheet.sheet_id.trim().is_empty()
                    || sheet.artifact_scope_label.trim().is_empty()
                    || sheet.selected_key_set.is_empty()
                    || sheet.risk_flags.is_empty()
                    || sheet.limitation_summary.trim().is_empty()
                    || sheet.fallback_path_label.trim().is_empty()
                    || sheet.review_action.action_label.trim().is_empty()
                    || !sheet.requires_explicit_acknowledgement
                    || sheet.compare_ref.trim().is_empty()
                {
                    defects.push(PacketValidationError::InconsistentCompareReview(
                        review.family,
                    ));
                }
            }
            (None, None) => {}
            _ => defects.push(PacketValidationError::InconsistentCompareReview(
                review.family,
            )),
        }

        if review.effective_value_review_sheet.key_set.is_empty()
            || review
                .effective_value_review_sheet
                .winning_layers
                .is_empty()
            || review
                .effective_value_review_sheet
                .export_posture_label
                .trim()
                .is_empty()
            || review
                .effective_value_review_sheet
                .output_disclosure_classes
                .is_empty()
            || review.effective_value_review_sheet.actions.is_empty()
            || review.export_summary.summary_id.trim().is_empty()
            || review.export_summary.posture_label.trim().is_empty()
            || review.export_summary.output_disclosure_classes.is_empty()
            || review.export_summary.summary_lines.is_empty()
            || !review.support_export_reuses_export_summary
        {
            defects.push(PacketValidationError::ExportSummaryMismatch(review.family));
        }

        let review_disclosures: BTreeSet<_> = review
            .effective_value_review_sheet
            .output_disclosure_classes
            .iter()
            .copied()
            .collect();
        let export_disclosures: BTreeSet<_> = review
            .export_summary
            .output_disclosure_classes
            .iter()
            .copied()
            .collect();
        if review_disclosures != export_disclosures {
            defects.push(PacketValidationError::ExportSummaryMismatch(review.family));
        }
    }

    let expected_summary = derive_summary(&packet.surface_vocabulary, &packet.artifact_reviews);
    compare_summary_count(
        &mut defects,
        "artifact_review_count",
        expected_summary.artifact_review_count,
        packet.summary.artifact_review_count,
    );
    compare_summary_count(
        &mut defects,
        "parameter_source_row_count",
        expected_summary.parameter_source_row_count,
        packet.summary.parameter_source_row_count,
    );
    compare_summary_count(
        &mut defects,
        "family_count_with_round_trip_banner",
        expected_summary.family_count_with_round_trip_banner,
        packet.summary.family_count_with_round_trip_banner,
    );
    compare_summary_count(
        &mut defects,
        "family_count_with_compare_before_save",
        expected_summary.family_count_with_compare_before_save,
        packet.summary.family_count_with_compare_before_save,
    );
    compare_summary_count(
        &mut defects,
        "family_count_with_effective_value_review",
        expected_summary.family_count_with_effective_value_review,
        packet.summary.family_count_with_effective_value_review,
    );
    compare_summary_count(
        &mut defects,
        "family_count_reusing_export_summary",
        expected_summary.family_count_reusing_export_summary,
        packet.summary.family_count_reusing_export_summary,
    );
    compare_summary_count(
        &mut defects,
        "chip_class_coverage_count",
        expected_summary.chip_class_coverage_count,
        packet.summary.chip_class_coverage_count,
    );
    compare_summary_count(
        &mut defects,
        "output_disclosure_coverage_count",
        expected_summary.output_disclosure_coverage_count,
        packet.summary.output_disclosure_coverage_count,
    );
    compare_summary_count(
        &mut defects,
        "shared_surface_count",
        expected_summary.shared_surface_count,
        packet.summary.shared_surface_count,
    );
    compare_summary_flag(
        &mut defects,
        "raw_secret_export_blocked_everywhere",
        expected_summary.raw_secret_export_blocked_everywhere,
        packet.summary.raw_secret_export_blocked_everywhere,
    );
    compare_summary_flag(
        &mut defects,
        "support_export_metadata_reused_everywhere",
        expected_summary.support_export_metadata_reused_everywhere,
        packet.summary.support_export_metadata_reused_everywhere,
    );

    defects
}

/// Validates a packet, returning every defect when validation fails.
pub fn validate_structured_config_parameter_source_and_round_trip_review(
    packet: &StructuredConfigParameterSourceRoundTripReviewPacket,
) -> Result<(), Vec<PacketValidationError>> {
    let defects = audit_structured_config_parameter_source_and_round_trip_review(packet);
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

fn compare_summary_count(
    defects: &mut Vec<PacketValidationError>,
    field: &'static str,
    expected: usize,
    actual: usize,
) {
    if expected != actual {
        defects.push(PacketValidationError::SummaryCountMismatch {
            field,
            expected,
            actual,
        });
    }
}

fn compare_summary_flag(
    defects: &mut Vec<PacketValidationError>,
    field: &'static str,
    expected: bool,
    actual: bool,
) {
    if expected != actual {
        defects.push(PacketValidationError::SummaryFlagMismatch {
            field,
            expected,
            actual,
        });
    }
}

fn append_presence_defects<T, K>(
    defects: &mut Vec<PacketValidationError>,
    rows: &[T],
    required: &[K],
    key: impl Fn(&T) -> K,
    missing: impl Fn(K) -> PacketValidationError,
    duplicate: impl Fn(K) -> PacketValidationError,
) where
    K: Ord + Copy,
{
    let mut seen = BTreeSet::new();
    for row in rows {
        let token = key(row);
        if !seen.insert(token) {
            defects.push(duplicate(token));
        }
    }
    for required_token in required {
        if !seen.contains(required_token) {
            defects.push(missing(*required_token));
        }
    }
}

fn append_surface_presence_defects(
    defects: &mut Vec<PacketValidationError>,
    rows: &[SurfaceVocabularyBinding],
) {
    let present: BTreeSet<_> = rows.iter().map(|row| row.surface).collect();
    for required in ConsumerSurfaceClass::ALL {
        if !present.contains(&required) {
            defects.push(PacketValidationError::MissingSurface(required));
        }
    }
}

fn covers_all<K, const N: usize>(present: impl Iterator<Item = K>, required: [K; N]) -> bool
where
    K: Ord + Copy,
{
    let set: BTreeSet<_> = present.collect();
    required.iter().all(|item| set.contains(item))
}

fn derive_summary(
    surface_vocabulary: &[SurfaceVocabularyBinding],
    artifact_reviews: &[ArtifactReviewRow],
) -> PacketSummary {
    let chip_classes: BTreeSet<_> = artifact_reviews
        .iter()
        .flat_map(|review| review.value_chips.iter().map(|chip| chip.chip_class))
        .collect();
    let output_classes: BTreeSet<_> = artifact_reviews
        .iter()
        .flat_map(|review| {
            review
                .export_summary
                .output_disclosure_classes
                .iter()
                .copied()
        })
        .collect();

    PacketSummary {
        artifact_review_count: artifact_reviews.len(),
        parameter_source_row_count: artifact_reviews
            .iter()
            .map(|review| review.parameter_source_rows.len())
            .sum(),
        family_count_with_round_trip_banner: artifact_reviews
            .iter()
            .filter(|review| review.round_trip_risk_banner.is_some())
            .count(),
        family_count_with_compare_before_save: artifact_reviews
            .iter()
            .filter(|review| review.compare_before_save_sheet.is_some())
            .count(),
        family_count_with_effective_value_review: artifact_reviews.len(),
        family_count_reusing_export_summary: artifact_reviews
            .iter()
            .filter(|review| review.support_export_reuses_export_summary)
            .count(),
        chip_class_coverage_count: chip_classes.len(),
        output_disclosure_coverage_count: output_classes.len(),
        shared_surface_count: surface_vocabulary.len(),
        raw_secret_export_blocked_everywhere: artifact_reviews.iter().all(|review| {
            review.value_chips.iter().all(|chip| {
                !matches!(
                    chip.chip_class,
                    ValueChipClass::SecretHandle | ValueChipClass::PolicyInjected
                ) || chip.raw_secret_export_blocked_by_default
            })
        }),
        support_export_metadata_reused_everywhere: artifact_reviews
            .iter()
            .all(|review| review.support_export_reuses_export_summary),
    }
}

fn seeded_parameter_row_vocabulary() -> Vec<ParameterRowFieldDefinition> {
    vec![
        ParameterRowFieldDefinition {
            field: ParameterRowField::KeyPath,
            label: "Key".to_owned(),
            description: "Stable key or path identity used across editor, review, export, and support.".to_owned(),
        },
        ParameterRowFieldDefinition {
            field: ParameterRowField::DisplayValue,
            label: "Display".to_owned(),
            description: "Masked, redacted, or placeholder display rather than raw secret bytes.".to_owned(),
        },
        ParameterRowFieldDefinition {
            field: ParameterRowField::SourceClass,
            label: "Source class".to_owned(),
            description: "Literal, env ref, secret handle, policy-injected, or runtime-discovered classification.".to_owned(),
        },
        ParameterRowFieldDefinition {
            field: ParameterRowField::ResolutionTime,
            label: "Resolution time".to_owned(),
            description: "When the value becomes concrete for the active target or runtime.".to_owned(),
        },
        ParameterRowFieldDefinition {
            field: ParameterRowField::Winner,
            label: "Winner".to_owned(),
            description: "Which layer won precedence or why the row remains unresolved/deferred.".to_owned(),
        },
        ParameterRowFieldDefinition {
            field: ParameterRowField::OverrideAction,
            label: "Override action".to_owned(),
            description: "Layer-bounded reset, clear, source-open, or policy-open action.".to_owned(),
        },
        ParameterRowFieldDefinition {
            field: ParameterRowField::CopyExportPosture,
            label: "Copy/export posture".to_owned(),
            description: "Whether the row exports a local literal, reference/handle, redacted placeholder, key path, or metadata summary only.".to_owned(),
        },
    ]
}

fn seeded_value_chip_vocabulary() -> Vec<ValueChipVocabularyRow> {
    vec![
        chip_vocab(
            ValueChipClass::LiteralValue,
            "Literal",
            "Stored as an authored literal value; copy/export stays local-review first when sensitive.",
            false,
        ),
        chip_vocab(
            ValueChipClass::EnvReference,
            "Env ref",
            "Stored as an environment-variable reference rather than a concrete secret.",
            true,
        ),
        chip_vocab(
            ValueChipClass::SecretHandle,
            "Secret handle",
            "Stored as a vault, keychain, or delegated handle; raw secret export stays blocked by default.",
            true,
        ),
        chip_vocab(
            ValueChipClass::PolicyInjected,
            "Policy injected",
            "Resolved from managed policy or signed bundle rather than authored inline.",
            true,
        ),
        chip_vocab(
            ValueChipClass::RuntimeDiscovered,
            "Runtime discovered",
            "Observed or derived at runtime and never masquerades as canonical authored text.",
            true,
        ),
    ]
}

fn chip_vocab(
    chip_class: ValueChipClass,
    label: &str,
    description: &str,
    raw_secret_export_blocked_by_default: bool,
) -> ValueChipVocabularyRow {
    ValueChipVocabularyRow {
        chip_class,
        label: label.to_owned(),
        description: description.to_owned(),
        raw_secret_export_blocked_by_default,
    }
}

fn seeded_output_disclosure_vocabulary() -> Vec<OutputDisclosureVocabularyRow> {
    vec![
        disclosure_vocab(
            OutputDisclosureClass::LiteralValue,
            "Literal value",
            "The review/export contains at least one literal value.",
        ),
        disclosure_vocab(
            OutputDisclosureClass::ReferenceHandle,
            "Reference/handle",
            "The review/export contains only a handle, alias, or env reference for the secret-bearing row.",
        ),
        disclosure_vocab(
            OutputDisclosureClass::RedactedPlaceholder,
            "Redacted placeholder",
            "The review/export contains a placeholder instead of the value body.",
        ),
        disclosure_vocab(
            OutputDisclosureClass::KeyPathMetadataOnly,
            "Key-path metadata only",
            "The review/export contains only key/path identity and source metadata.",
        ),
    ]
}

fn disclosure_vocab(
    output_class: OutputDisclosureClass,
    label: &str,
    description: &str,
) -> OutputDisclosureVocabularyRow {
    OutputDisclosureVocabularyRow {
        output_class,
        label: label.to_owned(),
        description: description.to_owned(),
    }
}

fn seeded_surface_vocabulary() -> Vec<SurfaceVocabularyBinding> {
    ConsumerSurfaceClass::ALL
        .into_iter()
        .map(|surface| SurfaceVocabularyBinding {
            surface,
            shared_contract_ref:
                STRUCTURED_CONFIG_PARAMETER_SOURCE_ROUND_TRIP_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            renders_parameter_rows: true,
            renders_value_chips: true,
            renders_round_trip_banner: true,
            renders_compare_before_save_sheet: true,
            renders_effective_value_review_sheet: true,
            renders_export_summary: true,
        })
        .collect()
}

fn seeded_artifact_reviews() -> Vec<ArtifactReviewRow> {
    vec![
        request_workspace_environment_review(),
        database_profile_review(),
        api_profile_review(),
        notebook_runtime_manifest_review(),
        preview_runtime_config_review(),
        workflow_bundle_manifest_review(),
        ci_environment_descriptor_review(),
        infra_environment_descriptor_review(),
        managed_policy_overlay_review(),
    ]
}

fn request_workspace_environment_review() -> ArtifactReviewRow {
    ArtifactReviewRow {
        family: ArtifactFamilyKind::RequestWorkspaceEnvironment,
        qualification_label: QualificationLabel::Stable,
        artifact_surface_ref: family_ref(ArtifactFamilyKind::RequestWorkspaceEnvironment),
        parameter_source_rows: vec![
            row(
                "env.DB_HOST",
                "db.internal",
                ValueChipClass::LiteralValue,
                "authored",
                "workspace source",
                action(ReviewActionClass::ResetCurrentLayer, "Reset current layer"),
                CopyExportPosture::LocalLiteralAfterReview,
                true,
            ),
            row(
                "env.DB_PASSWORD",
                "${env:DB_PASSWORD}",
                ValueChipClass::EnvReference,
                "at run",
                "profile override",
                action(ReviewActionClass::ClearOverride, "Clear override"),
                CopyExportPosture::ReferenceHandleOnly,
                true,
            ),
            row(
                "env.REQUEST_ID",
                "runtime value",
                ValueChipClass::RuntimeDiscovered,
                "at run",
                "runtime discovery",
                action(ReviewActionClass::OpenSource, "Open source"),
                CopyExportPosture::KeyPathMetadataOnly,
                false,
            ),
        ],
        value_chips: vec![
            chip(
                "env.DB_HOST",
                ValueChipClass::LiteralValue,
                "Literal",
                CopyExportPosture::LocalLiteralAfterReview,
                "Reveal locally before copying the literal.",
                false,
            ),
            chip(
                "env.DB_PASSWORD",
                ValueChipClass::EnvReference,
                "Env ref",
                CopyExportPosture::ReferenceHandleOnly,
                "Copy the env reference, not a concrete password.",
                true,
            ),
            chip(
                "env.REQUEST_ID",
                ValueChipClass::RuntimeDiscovered,
                "Runtime discovered",
                CopyExportPosture::KeyPathMetadataOnly,
                "Observed at runtime only; source/export stays metadata-only.",
                true,
            ),
        ],
        round_trip_risk_banner: None,
        compare_before_save_sheet: None,
        effective_value_review_sheet: review_sheet(
            &["env.DB_HOST", "env.DB_PASSWORD", "env.REQUEST_ID"],
            &["workspace source", "profile override", "runtime discovery"],
            &[],
            "Literal values stay local-only; refs/metadata travel by default.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::KeyPathMetadataOnly,
            ],
            vec![
                action(ReviewActionClass::OpenSource, "Open source"),
                action(ReviewActionClass::CompareLive, "Compare live"),
            ],
        ),
        export_summary: export_summary(
            "export-summary:request-workspace-environment",
            "Mixed literal + reference + metadata summary.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::KeyPathMetadataOnly,
            ],
            &[
                "Contains literal values only after local review.",
                "Contains env references instead of concrete secrets.",
                "Runtime-only values export as key-path metadata.",
            ],
        ),
        support_export_reuses_export_summary: true,
    }
}

fn database_profile_review() -> ArtifactReviewRow {
    ArtifactReviewRow {
        family: ArtifactFamilyKind::DatabaseProfile,
        qualification_label: QualificationLabel::Stable,
        artifact_surface_ref: family_ref(ArtifactFamilyKind::DatabaseProfile),
        parameter_source_rows: vec![
            row(
                "connection.host",
                "db.prod.internal",
                ValueChipClass::LiteralValue,
                "authored",
                "workspace source",
                action(ReviewActionClass::ResetCurrentLayer, "Reset current layer"),
                CopyExportPosture::LocalLiteralAfterReview,
                true,
            ),
            row(
                "credentials.password",
                "vault://prod/db/password",
                ValueChipClass::SecretHandle,
                "at connect",
                "vault handle",
                action(ReviewActionClass::ViewPolicySource, "View secret source"),
                CopyExportPosture::ReferenceHandleOnly,
                true,
            ),
            row(
                "ssl.mode",
                "verify-full",
                ValueChipClass::PolicyInjected,
                "computed locally now",
                "managed policy",
                action(ReviewActionClass::ViewPolicySource, "View policy"),
                CopyExportPosture::MetadataSummaryOnly,
                true,
            ),
        ],
        value_chips: vec![
            chip(
                "connection.host",
                ValueChipClass::LiteralValue,
                "Literal",
                CopyExportPosture::LocalLiteralAfterReview,
                "Hostname copies only after local review.",
                false,
            ),
            chip(
                "credentials.password",
                ValueChipClass::SecretHandle,
                "Secret handle",
                CopyExportPosture::ReferenceHandleOnly,
                "Only the vault handle is portable.",
                true,
            ),
            chip(
                "ssl.mode",
                ValueChipClass::PolicyInjected,
                "Policy injected",
                CopyExportPosture::MetadataSummaryOnly,
                "Managed policy owns the effective value.",
                true,
            ),
        ],
        round_trip_risk_banner: None,
        compare_before_save_sheet: None,
        effective_value_review_sheet: review_sheet(
            &["connection.host", "credentials.password", "ssl.mode"],
            &["workspace source", "vault handle", "managed policy"],
            &[],
            "Support/export keeps handles and policy metadata distinct from literals.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::RedactedPlaceholder,
            ],
            vec![
                action(ReviewActionClass::OpenSource, "Open source"),
                action(ReviewActionClass::ViewPolicySource, "Open policy"),
            ],
        ),
        export_summary: export_summary(
            "export-summary:database-profile",
            "Literal host + handle-only secret + redacted policy summary.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::RedactedPlaceholder,
            ],
            &[
                "Contains the literal host value after local review.",
                "Contains a secret handle instead of the password body.",
                "Policy-owned rows downgrade to redacted placeholders in support/export.",
            ],
        ),
        support_export_reuses_export_summary: true,
    }
}

fn api_profile_review() -> ArtifactReviewRow {
    ArtifactReviewRow {
        family: ArtifactFamilyKind::ApiProfile,
        qualification_label: QualificationLabel::Stable,
        artifact_surface_ref: family_ref(ArtifactFamilyKind::ApiProfile),
        parameter_source_rows: vec![
            row(
                "base_url",
                "https://api.internal",
                ValueChipClass::LiteralValue,
                "authored",
                "workspace source",
                action(ReviewActionClass::ResetCurrentLayer, "Reset current layer"),
                CopyExportPosture::LocalLiteralAfterReview,
                true,
            ),
            row(
                "auth.token",
                "keychain://api-prod",
                ValueChipClass::SecretHandle,
                "at request time",
                "brokered handle",
                action(ReviewActionClass::RevealLocally, "Reveal locally"),
                CopyExportPosture::ReferenceHandleOnly,
                true,
            ),
            row(
                "headers.X-Tenant",
                "managed tenant id",
                ValueChipClass::PolicyInjected,
                "computed locally now",
                "managed policy",
                action(ReviewActionClass::ViewPolicySource, "View policy"),
                CopyExportPosture::MetadataSummaryOnly,
                true,
            ),
            row(
                "oauth.audience",
                "provider discovered",
                ValueChipClass::RuntimeDiscovered,
                "after auth handshake",
                "runtime discovery",
                action(ReviewActionClass::CompareLive, "Compare live"),
                CopyExportPosture::KeyPathMetadataOnly,
                false,
            ),
        ],
        value_chips: vec![
            chip(
                "base_url",
                ValueChipClass::LiteralValue,
                "Literal",
                CopyExportPosture::LocalLiteralAfterReview,
                "Literal URL copies only after local review.",
                false,
            ),
            chip(
                "auth.token",
                ValueChipClass::SecretHandle,
                "Secret handle",
                CopyExportPosture::ReferenceHandleOnly,
                "Handle-only auth; raw token stays blocked.",
                true,
            ),
            chip(
                "headers.X-Tenant",
                ValueChipClass::PolicyInjected,
                "Policy injected",
                CopyExportPosture::MetadataSummaryOnly,
                "Managed policy owns the final header.",
                true,
            ),
            chip(
                "oauth.audience",
                ValueChipClass::RuntimeDiscovered,
                "Runtime discovered",
                CopyExportPosture::KeyPathMetadataOnly,
                "Observed after auth; never authored as stable text.",
                true,
            ),
        ],
        round_trip_risk_banner: None,
        compare_before_save_sheet: None,
        effective_value_review_sheet: review_sheet(
            &["base_url", "auth.token", "headers.X-Tenant", "oauth.audience"],
            &[
                "workspace source",
                "brokered handle",
                "managed policy",
                "runtime discovery",
            ],
            &["oauth.audience in browser companion"],
            "Literal base URL stays local-review first; auth/policy/runtime rows narrow to handles or metadata.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::RedactedPlaceholder,
                OutputDisclosureClass::KeyPathMetadataOnly,
            ],
            vec![
                action(ReviewActionClass::OpenSource, "Open source"),
                action(ReviewActionClass::CompareLive, "Compare live"),
            ],
        ),
        export_summary: export_summary(
            "export-summary:api-profile",
            "Mixed literal, handle, redacted policy, and key-path metadata summary.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::RedactedPlaceholder,
                OutputDisclosureClass::KeyPathMetadataOnly,
            ],
            &[
                "Contains the literal base URL after local review.",
                "Contains a handle instead of the auth token body.",
                "Policy-owned rows export as redacted placeholders.",
                "Runtime-only rows export as key-path metadata.",
            ],
        ),
        support_export_reuses_export_summary: true,
    }
}

fn notebook_runtime_manifest_review() -> ArtifactReviewRow {
    let banner = banner(
        "Unknown extension namespaces and inline comments may rewrite during structured save.",
        "notebook runtime manifest fragment",
        "Edit raw source instead when extension metadata is authoritative.",
        action(ReviewActionClass::CompareBeforeSave, "Compare before save"),
        &[
            RoundTripRiskFlag::CommentsMayRewrite,
            RoundTripRiskFlag::UnknownKeysMayRewrite,
            RoundTripRiskFlag::ExtensionNamespacesMayRewrite,
        ],
    );
    ArtifactReviewRow {
        family: ArtifactFamilyKind::NotebookRuntimeManifest,
        qualification_label: QualificationLabel::Beta,
        artifact_surface_ref: family_ref(ArtifactFamilyKind::NotebookRuntimeManifest),
        parameter_source_rows: vec![
            row(
                "kernel.env.PYTHONPATH",
                "${env:PYTHONPATH}",
                ValueChipClass::EnvReference,
                "at kernel launch",
                "workspace source",
                action(ReviewActionClass::ClearOverride, "Clear override"),
                CopyExportPosture::ReferenceHandleOnly,
                true,
            ),
            row(
                "kernel.auth.api_key",
                "vault://research/notebook-api",
                ValueChipClass::SecretHandle,
                "at run",
                "vault handle",
                action(ReviewActionClass::ViewPolicySource, "Open secret source"),
                CopyExportPosture::ReferenceHandleOnly,
                true,
            ),
            row(
                "kernel.metadata.custom.runtimeClass",
                "gpu-preview",
                ValueChipClass::LiteralValue,
                "authored",
                "extension namespace",
                action(ReviewActionClass::OpenSource, "Open source"),
                CopyExportPosture::LocalLiteralAfterReview,
                true,
            ),
        ],
        value_chips: vec![
            chip(
                "kernel.env.PYTHONPATH",
                ValueChipClass::EnvReference,
                "Env ref",
                CopyExportPosture::ReferenceHandleOnly,
                "Reference survives export; resolved value does not.",
                true,
            ),
            chip(
                "kernel.auth.api_key",
                ValueChipClass::SecretHandle,
                "Secret handle",
                CopyExportPosture::ReferenceHandleOnly,
                "Handle-only notebook auth.",
                true,
            ),
            chip(
                "kernel.metadata.custom.runtimeClass",
                ValueChipClass::LiteralValue,
                "Literal",
                CopyExportPosture::LocalLiteralAfterReview,
                "Literal extension metadata copies only after review.",
                false,
            ),
        ],
        round_trip_risk_banner: Some(banner.clone()),
        compare_before_save_sheet: Some(compare_sheet(
            "compare-sheet:notebook-runtime-manifest",
            "notebook runtime manifest",
            &[
                "kernel.env.PYTHONPATH",
                "kernel.auth.api_key",
                "kernel.metadata.custom.runtimeClass",
            ],
            &banner.risk_flags,
            "Structured save cannot prove that comments and extension namespaces survive unchanged.",
            "Open raw manifest source for authoritative edits.",
            "compare:notebook-runtime-manifest",
        )),
        effective_value_review_sheet: review_sheet(
            &[
                "kernel.env.PYTHONPATH",
                "kernel.auth.api_key",
                "kernel.metadata.custom.runtimeClass",
            ],
            &["workspace source", "vault handle", "extension namespace"],
            &[],
            "Manifest exports keep handles and may redact namespace-carrying rows in support packets.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::RedactedPlaceholder,
            ],
            vec![
                action(ReviewActionClass::CompareBeforeSave, "Compare before save"),
                action(ReviewActionClass::OpenSource, "Open source"),
            ],
        ),
        export_summary: export_summary(
            "export-summary:notebook-runtime-manifest",
            "Literal namespace rows + handle-only secrets with redacted support posture.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::RedactedPlaceholder,
            ],
            &[
                "Literal extension metadata is disclosed explicitly.",
                "Secret-bearing auth rows export as handles only.",
                "Support packets may further redact namespace-heavy rows instead of rewriting them.",
            ],
        ),
        support_export_reuses_export_summary: true,
    }
}

fn preview_runtime_config_review() -> ArtifactReviewRow {
    let banner = banner(
        "Preview runtime config may normalize ordering and strip comments on structured save.",
        "preview runtime config",
        "Use raw source when comment layout or authored ordering carries meaning.",
        action(ReviewActionClass::CompareBeforeSave, "Compare before save"),
        &[
            RoundTripRiskFlag::CommentsMayRewrite,
            RoundTripRiskFlag::OrderingMayRewrite,
        ],
    );
    ArtifactReviewRow {
        family: ArtifactFamilyKind::PreviewRuntimeConfig,
        qualification_label: QualificationLabel::Preview,
        artifact_surface_ref: family_ref(ArtifactFamilyKind::PreviewRuntimeConfig),
        parameter_source_rows: vec![
            row(
                "preview.proxy.origin",
                "https://preview.internal",
                ValueChipClass::LiteralValue,
                "authored",
                "workspace source",
                action(ReviewActionClass::ResetCurrentLayer, "Reset current layer"),
                CopyExportPosture::LocalLiteralAfterReview,
                true,
            ),
            row(
                "preview.auth.cookie",
                "policy-backed handle",
                ValueChipClass::PolicyInjected,
                "at preview start",
                "managed policy",
                action(ReviewActionClass::ViewPolicySource, "View policy"),
                CopyExportPosture::MetadataSummaryOnly,
                true,
            ),
            row(
                "preview.device_token",
                "runtime token",
                ValueChipClass::RuntimeDiscovered,
                "observed live",
                "runtime discovery",
                action(ReviewActionClass::CompareLive, "Compare live"),
                CopyExportPosture::KeyPathMetadataOnly,
                false,
            ),
        ],
        value_chips: vec![
            chip(
                "preview.proxy.origin",
                ValueChipClass::LiteralValue,
                "Literal",
                CopyExportPosture::LocalLiteralAfterReview,
                "Literal origin copies only after review.",
                false,
            ),
            chip(
                "preview.auth.cookie",
                ValueChipClass::PolicyInjected,
                "Policy injected",
                CopyExportPosture::MetadataSummaryOnly,
                "Managed preview auth never exports raw cookie material.",
                true,
            ),
            chip(
                "preview.device_token",
                ValueChipClass::RuntimeDiscovered,
                "Runtime discovered",
                CopyExportPosture::KeyPathMetadataOnly,
                "Preview token is machine-local runtime state.",
                true,
            ),
        ],
        round_trip_risk_banner: Some(banner.clone()),
        compare_before_save_sheet: Some(compare_sheet(
            "compare-sheet:preview-runtime-config",
            "preview runtime config",
            &["preview.proxy.origin", "preview.auth.cookie", "preview.device_token"],
            &banner.risk_flags,
            "Structured editing is not allowed to hide comment loss or ordering changes behind preview-looking defaults.",
            "Edit the canonical source or review the compare sheet before save.",
            "compare:preview-runtime-config",
        )),
        effective_value_review_sheet: review_sheet(
            &["preview.proxy.origin", "preview.auth.cookie", "preview.device_token"],
            &["workspace source", "managed policy", "runtime discovery"],
            &["preview.device_token on offline snapshot"],
            "Preview exports keep policy/runtime state narrow and machine-local.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::RedactedPlaceholder,
                OutputDisclosureClass::KeyPathMetadataOnly,
            ],
            vec![
                action(ReviewActionClass::CompareBeforeSave, "Compare before save"),
                action(ReviewActionClass::CompareLive, "Compare live"),
            ],
        ),
        export_summary: export_summary(
            "export-summary:preview-runtime-config",
            "Literal authored rows + redacted policy state + key-path runtime metadata.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::RedactedPlaceholder,
                OutputDisclosureClass::KeyPathMetadataOnly,
            ],
            &[
                "Literal preview origin is disclosed explicitly.",
                "Policy-backed preview auth narrows to redacted placeholders.",
                "Runtime device tokens export as key-path metadata only.",
            ],
        ),
        support_export_reuses_export_summary: true,
    }
}

fn workflow_bundle_manifest_review() -> ArtifactReviewRow {
    let banner = banner(
        "Workflow bundle manifests may reorder unknown step keys and imported extension fields.",
        "workflow bundle manifest",
        "Prefer raw source or the compare sheet when imported bundle metadata is authoritative.",
        action(ReviewActionClass::CompareBeforeSave, "Compare before save"),
        &[
            RoundTripRiskFlag::UnknownKeysMayRewrite,
            RoundTripRiskFlag::OrderingMayRewrite,
        ],
    );
    ArtifactReviewRow {
        family: ArtifactFamilyKind::WorkflowBundleManifest,
        qualification_label: QualificationLabel::Beta,
        artifact_surface_ref: family_ref(ArtifactFamilyKind::WorkflowBundleManifest),
        parameter_source_rows: vec![
            row(
                "bundle.inputs.REGISTRY_TOKEN",
                "vault://release/registry-token",
                ValueChipClass::SecretHandle,
                "at workflow start",
                "vault handle",
                action(ReviewActionClass::RevealLocally, "Reveal locally"),
                CopyExportPosture::ReferenceHandleOnly,
                true,
            ),
            row(
                "bundle.steps[0].env.NODE_OPTIONS",
                "--max-old-space-size=4096",
                ValueChipClass::LiteralValue,
                "authored",
                "bundle source",
                action(ReviewActionClass::ResetCurrentLayer, "Reset current layer"),
                CopyExportPosture::LocalLiteralAfterReview,
                true,
            ),
            row(
                "bundle.policy.target",
                "release-approved only",
                ValueChipClass::PolicyInjected,
                "computed locally now",
                "managed policy",
                action(ReviewActionClass::ViewPolicySource, "View policy"),
                CopyExportPosture::MetadataSummaryOnly,
                true,
            ),
        ],
        value_chips: vec![
            chip(
                "bundle.inputs.REGISTRY_TOKEN",
                ValueChipClass::SecretHandle,
                "Secret handle",
                CopyExportPosture::ReferenceHandleOnly,
                "Handle-only workflow input.",
                true,
            ),
            chip(
                "bundle.steps[0].env.NODE_OPTIONS",
                ValueChipClass::LiteralValue,
                "Literal",
                CopyExportPosture::LocalLiteralAfterReview,
                "Literal workflow env is reviewable and local-first.",
                false,
            ),
            chip(
                "bundle.policy.target",
                ValueChipClass::PolicyInjected,
                "Policy injected",
                CopyExportPosture::MetadataSummaryOnly,
                "Policy target is review-only locally.",
                true,
            ),
        ],
        round_trip_risk_banner: Some(banner.clone()),
        compare_before_save_sheet: Some(compare_sheet(
            "compare-sheet:workflow-bundle-manifest",
            "workflow bundle manifest",
            &[
                "bundle.inputs.REGISTRY_TOKEN",
                "bundle.steps[0].env.NODE_OPTIONS",
                "bundle.policy.target",
            ],
            &banner.risk_flags,
            "Unknown step keys and imported extension metadata may reorder under structured save.",
            "Open the canonical bundle source when imported metadata must stay byte-stable.",
            "compare:workflow-bundle-manifest",
        )),
        effective_value_review_sheet: review_sheet(
            &[
                "bundle.inputs.REGISTRY_TOKEN",
                "bundle.steps[0].env.NODE_OPTIONS",
                "bundle.policy.target",
            ],
            &["vault handle", "bundle source", "managed policy"],
            &[],
            "Bundle review/export keeps handles and policy ceilings explicit before publish.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::RedactedPlaceholder,
            ],
            vec![
                action(ReviewActionClass::CompareBeforeSave, "Compare before save"),
                action(ReviewActionClass::OpenSource, "Open source"),
            ],
        ),
        export_summary: export_summary(
            "export-summary:workflow-bundle-manifest",
            "Literal bundle env + handle-only secrets + redacted policy ceilings.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::RedactedPlaceholder,
            ],
            &[
                "Literal workflow env rows are named explicitly.",
                "Registry auth remains a handle, never a raw token.",
                "Policy targets narrow to redacted placeholders in export/support.",
            ],
        ),
        support_export_reuses_export_summary: true,
    }
}

fn ci_environment_descriptor_review() -> ArtifactReviewRow {
    let banner = banner(
        "CI env descriptors may rewrite comments and normalize authored order during structured save.",
        "CI environment descriptor",
        "Compare before save when inline comments or matrix order are part of the review surface.",
        action(ReviewActionClass::CompareBeforeSave, "Compare before save"),
        &[
            RoundTripRiskFlag::CommentsMayRewrite,
            RoundTripRiskFlag::OrderingMayRewrite,
        ],
    );
    ArtifactReviewRow {
        family: ArtifactFamilyKind::CiEnvironmentDescriptor,
        qualification_label: QualificationLabel::Stable,
        artifact_surface_ref: family_ref(ArtifactFamilyKind::CiEnvironmentDescriptor),
        parameter_source_rows: vec![
            row(
                "jobs.build.env.NPM_TOKEN",
                "${env:NPM_TOKEN}",
                ValueChipClass::EnvReference,
                "at job start",
                "workspace source",
                action(ReviewActionClass::ClearOverride, "Clear override"),
                CopyExportPosture::ReferenceHandleOnly,
                true,
            ),
            row(
                "jobs.deploy.env.AWS_REGION",
                "eu-west-1",
                ValueChipClass::PolicyInjected,
                "computed locally now",
                "managed policy",
                action(ReviewActionClass::ViewPolicySource, "View policy"),
                CopyExportPosture::MetadataSummaryOnly,
                true,
            ),
            row(
                "jobs.test.matrix.OS",
                "ubuntu-24.04",
                ValueChipClass::LiteralValue,
                "authored",
                "workspace source",
                action(ReviewActionClass::ResetCurrentLayer, "Reset current layer"),
                CopyExportPosture::LocalLiteralAfterReview,
                true,
            ),
        ],
        value_chips: vec![
            chip(
                "jobs.build.env.NPM_TOKEN",
                ValueChipClass::EnvReference,
                "Env ref",
                CopyExportPosture::ReferenceHandleOnly,
                "Reference survives; concrete token does not.",
                true,
            ),
            chip(
                "jobs.deploy.env.AWS_REGION",
                ValueChipClass::PolicyInjected,
                "Policy injected",
                CopyExportPosture::MetadataSummaryOnly,
                "Managed routing remains policy-owned.",
                true,
            ),
            chip(
                "jobs.test.matrix.OS",
                ValueChipClass::LiteralValue,
                "Literal",
                CopyExportPosture::LocalLiteralAfterReview,
                "Literal matrix rows are reviewable and explicit.",
                false,
            ),
        ],
        round_trip_risk_banner: Some(banner.clone()),
        compare_before_save_sheet: Some(compare_sheet(
            "compare-sheet:ci-environment-descriptor",
            "CI environment descriptor",
            &[
                "jobs.build.env.NPM_TOKEN",
                "jobs.deploy.env.AWS_REGION",
                "jobs.test.matrix.OS",
            ],
            &banner.risk_flags,
            "Structured edits may drop authored comments or reorder matrix rows during save.",
            "Use the compare sheet or edit raw source when order/comment fidelity matters.",
            "compare:ci-environment-descriptor",
        )),
        effective_value_review_sheet: review_sheet(
            &[
                "jobs.build.env.NPM_TOKEN",
                "jobs.deploy.env.AWS_REGION",
                "jobs.test.matrix.OS",
            ],
            &["workspace source", "managed policy", "workspace source"],
            &[],
            "CI exports keep token refs separate from policy-owned region state.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::RedactedPlaceholder,
            ],
            vec![
                action(ReviewActionClass::CompareBeforeSave, "Compare before save"),
                action(ReviewActionClass::OpenSource, "Open source"),
            ],
        ),
        export_summary: export_summary(
            "export-summary:ci-environment-descriptor",
            "Literal matrix rows + env refs + redacted policy state.",
            &[
                OutputDisclosureClass::LiteralValue,
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::RedactedPlaceholder,
            ],
            &[
                "Literal matrix values remain explicit.",
                "NPM token stays an env reference.",
                "Policy-owned region state narrows to redacted placeholders in support/export.",
            ],
        ),
        support_export_reuses_export_summary: true,
    }
}

fn infra_environment_descriptor_review() -> ArtifactReviewRow {
    let banner = banner(
        "Infra descriptors may rewrite unknown provider keys and extension namespaces on structured save.",
        "infra environment descriptor",
        "Prefer compare-before-save or raw source when provider-specific namespaces are authoritative.",
        action(ReviewActionClass::CompareBeforeSave, "Compare before save"),
        &[
            RoundTripRiskFlag::UnknownKeysMayRewrite,
            RoundTripRiskFlag::ExtensionNamespacesMayRewrite,
        ],
    );
    ArtifactReviewRow {
        family: ArtifactFamilyKind::InfraEnvironmentDescriptor,
        qualification_label: QualificationLabel::Beta,
        artifact_surface_ref: family_ref(ArtifactFamilyKind::InfraEnvironmentDescriptor),
        parameter_source_rows: vec![
            row(
                "providers.aws.profile",
                "${env:AWS_PROFILE}",
                ValueChipClass::EnvReference,
                "at plan time",
                "workspace source",
                action(ReviewActionClass::ClearOverride, "Clear override"),
                CopyExportPosture::ReferenceHandleOnly,
                true,
            ),
            row(
                "providers.aws.assume_role",
                "vault://infra/role-alias",
                ValueChipClass::SecretHandle,
                "at apply time",
                "vault handle",
                action(ReviewActionClass::RevealLocally, "Reveal locally"),
                CopyExportPosture::ReferenceHandleOnly,
                true,
            ),
            row(
                "stacks.prod.endpoint",
                "observed from runtime",
                ValueChipClass::RuntimeDiscovered,
                "observed live",
                "runtime discovery",
                action(ReviewActionClass::CompareLive, "Compare live"),
                CopyExportPosture::KeyPathMetadataOnly,
                false,
            ),
        ],
        value_chips: vec![
            chip(
                "providers.aws.profile",
                ValueChipClass::EnvReference,
                "Env ref",
                CopyExportPosture::ReferenceHandleOnly,
                "Profile ref is portable; resolved host state is not.",
                true,
            ),
            chip(
                "providers.aws.assume_role",
                ValueChipClass::SecretHandle,
                "Secret handle",
                CopyExportPosture::ReferenceHandleOnly,
                "Assume-role alias never exports raw credentials.",
                true,
            ),
            chip(
                "stacks.prod.endpoint",
                ValueChipClass::RuntimeDiscovered,
                "Runtime discovered",
                CopyExportPosture::KeyPathMetadataOnly,
                "Observed endpoint remains metadata-only.",
                true,
            ),
        ],
        round_trip_risk_banner: Some(banner.clone()),
        compare_before_save_sheet: Some(compare_sheet(
            "compare-sheet:infra-environment-descriptor",
            "infra environment descriptor",
            &[
                "providers.aws.profile",
                "providers.aws.assume_role",
                "stacks.prod.endpoint",
            ],
            &banner.risk_flags,
            "Provider-owned keys and extension namespaces may not round-trip safely through structured edits.",
            "Open the raw descriptor when provider-specific namespaces are contract-critical.",
            "compare:infra-environment-descriptor",
        )),
        effective_value_review_sheet: review_sheet(
            &[
                "providers.aws.profile",
                "providers.aws.assume_role",
                "stacks.prod.endpoint",
            ],
            &["workspace source", "vault handle", "runtime discovery"],
            &["stacks.prod.endpoint in offline mirror"],
            "Infra review keeps provider refs/handles explicit and runtime observations metadata-only.",
            &[
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::KeyPathMetadataOnly,
            ],
            vec![
                action(ReviewActionClass::CompareBeforeSave, "Compare before save"),
                action(ReviewActionClass::CompareLive, "Compare live"),
            ],
        ),
        export_summary: export_summary(
            "export-summary:infra-environment-descriptor",
            "Reference/handle rows plus key-path-only runtime observations.",
            &[
                OutputDisclosureClass::ReferenceHandle,
                OutputDisclosureClass::KeyPathMetadataOnly,
            ],
            &[
                "Provider refs and role aliases remain handles/references.",
                "Observed runtime endpoints export as key-path metadata only.",
            ],
        ),
        support_export_reuses_export_summary: true,
    }
}

fn managed_policy_overlay_review() -> ArtifactReviewRow {
    ArtifactReviewRow {
        family: ArtifactFamilyKind::ManagedPolicyOverlay,
        qualification_label: QualificationLabel::Beta,
        artifact_surface_ref: family_ref(ArtifactFamilyKind::ManagedPolicyOverlay),
        parameter_source_rows: vec![
            row(
                "network.egress.allowlist",
                "key-path only",
                ValueChipClass::PolicyInjected,
                "from signed bundle",
                "signed policy bundle",
                action(ReviewActionClass::ViewPolicySource, "Open signed bundle"),
                CopyExportPosture::KeyPathMetadataOnly,
                true,
            ),
            row(
                "credentials.registry_token",
                "redacted by policy",
                ValueChipClass::PolicyInjected,
                "from signed bundle",
                "signed policy bundle",
                action(ReviewActionClass::ViewPolicySource, "Open signed bundle"),
                CopyExportPosture::RedactedPlaceholderOnly,
                true,
            ),
            row(
                "entitlement.offline_grace",
                "cache observation",
                ValueChipClass::RuntimeDiscovered,
                "from cache refresh",
                "runtime observation",
                action(ReviewActionClass::CompareLive, "Compare live"),
                CopyExportPosture::KeyPathMetadataOnly,
                false,
            ),
        ],
        value_chips: vec![
            chip(
                "network.egress.allowlist",
                ValueChipClass::PolicyInjected,
                "Policy injected",
                CopyExportPosture::KeyPathMetadataOnly,
                "Policy-owned row exports only the key/path metadata.",
                true,
            ),
            chip(
                "credentials.registry_token",
                ValueChipClass::PolicyInjected,
                "Policy injected",
                CopyExportPosture::RedactedPlaceholderOnly,
                "Credential rows stay redacted unless reviewed locally from the signed bundle.",
                true,
            ),
            chip(
                "entitlement.offline_grace",
                ValueChipClass::RuntimeDiscovered,
                "Runtime discovered",
                CopyExportPosture::KeyPathMetadataOnly,
                "Observed cache state is metadata-only.",
                true,
            ),
        ],
        round_trip_risk_banner: None,
        compare_before_save_sheet: None,
        effective_value_review_sheet: review_sheet(
            &[
                "network.egress.allowlist",
                "credentials.registry_token",
                "entitlement.offline_grace",
            ],
            &["signed policy bundle", "signed policy bundle", "runtime observation"],
            &[],
            "Managed policy overlays export as key-path metadata and redacted placeholders only.",
            &[
                OutputDisclosureClass::RedactedPlaceholder,
                OutputDisclosureClass::KeyPathMetadataOnly,
            ],
            vec![
                action(ReviewActionClass::ViewPolicySource, "Open signed bundle"),
                action(ReviewActionClass::CompareLive, "Compare live"),
            ],
        ),
        export_summary: export_summary(
            "export-summary:managed-policy-overlay",
            "Key-path metadata plus redacted placeholders; no raw literals or secrets.",
            &[
                OutputDisclosureClass::RedactedPlaceholder,
                OutputDisclosureClass::KeyPathMetadataOnly,
            ],
            &[
                "Support/export keeps policy-owned rows at key-path or redacted-placeholder depth only.",
                "Observed entitlement cache state remains metadata-only.",
            ],
        ),
        support_export_reuses_export_summary: true,
    }
}

fn row(
    key_path: &str,
    display_value: &str,
    source_class: ValueChipClass,
    resolution_time_label: &str,
    winner_label: &str,
    override_action: ReviewActionRow,
    copy_export_posture: CopyExportPosture,
    wins_effective_value: bool,
) -> ParameterSourceRow {
    ParameterSourceRow {
        key_path: key_path.to_owned(),
        display_value: display_value.to_owned(),
        source_class,
        resolution_time_label: resolution_time_label.to_owned(),
        winner_label: winner_label.to_owned(),
        override_action,
        copy_export_posture,
        wins_effective_value,
    }
}

fn chip(
    key_path: &str,
    chip_class: ValueChipClass,
    label: &str,
    copy_export_posture: CopyExportPosture,
    reveal_posture_label: &str,
    raw_secret_export_blocked_by_default: bool,
) -> ValueChipRow {
    ValueChipRow {
        key_path: key_path.to_owned(),
        chip_class,
        label: label.to_owned(),
        copy_export_posture,
        reveal_posture_label: reveal_posture_label.to_owned(),
        raw_secret_export_blocked_by_default,
    }
}

fn banner(
    risk_summary: &str,
    affected_scope_label: &str,
    safe_path_label: &str,
    review_action: ReviewActionRow,
    risk_flags: &[RoundTripRiskFlag],
) -> RoundTripRiskBanner {
    RoundTripRiskBanner {
        risk_summary: risk_summary.to_owned(),
        affected_scope_label: affected_scope_label.to_owned(),
        safe_path_label: safe_path_label.to_owned(),
        review_action,
        risk_flags: risk_flags.to_vec(),
    }
}

fn compare_sheet(
    sheet_id: &str,
    artifact_scope_label: &str,
    selected_key_set: &[&str],
    risk_flags: &[RoundTripRiskFlag],
    limitation_summary: &str,
    fallback_path_label: &str,
    compare_ref: &str,
) -> CompareBeforeSaveSheet {
    CompareBeforeSaveSheet {
        sheet_id: sheet_id.to_owned(),
        artifact_scope_label: artifact_scope_label.to_owned(),
        selected_key_set: selected_key_set.iter().map(ToString::to_string).collect(),
        risk_flags: risk_flags.to_vec(),
        limitation_summary: limitation_summary.to_owned(),
        fallback_path_label: fallback_path_label.to_owned(),
        review_action: action(ReviewActionClass::CompareBeforeSave, "Compare before save"),
        requires_explicit_acknowledgement: true,
        compare_ref: compare_ref.to_owned(),
    }
}

fn review_sheet(
    key_set: &[&str],
    winning_layers: &[&str],
    unresolved: &[&str],
    export_posture_label: &str,
    output_disclosure_classes: &[OutputDisclosureClass],
    actions: Vec<ReviewActionRow>,
) -> EffectiveValueReviewSheet {
    EffectiveValueReviewSheet {
        key_set: key_set.iter().map(ToString::to_string).collect(),
        winning_layers: winning_layers.iter().map(ToString::to_string).collect(),
        unresolved: unresolved.iter().map(ToString::to_string).collect(),
        export_posture_label: export_posture_label.to_owned(),
        output_disclosure_classes: output_disclosure_classes.to_vec(),
        actions,
    }
}

fn export_summary(
    summary_id: &str,
    posture_label: &str,
    output_disclosure_classes: &[OutputDisclosureClass],
    summary_lines: &[&str],
) -> ExportSummary {
    ExportSummary {
        summary_id: summary_id.to_owned(),
        posture_label: posture_label.to_owned(),
        output_disclosure_classes: output_disclosure_classes.to_vec(),
        summary_lines: summary_lines.iter().map(ToString::to_string).collect(),
    }
}

fn action(action_class: ReviewActionClass, action_label: &str) -> ReviewActionRow {
    ReviewActionRow {
        action_class,
        action_label: action_label.to_owned(),
    }
}

fn family_ref(family: ArtifactFamilyKind) -> String {
    format!("aureline://config-structured-review/{family:?}").to_lowercase()
}
