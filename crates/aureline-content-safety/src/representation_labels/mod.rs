//! Beta representation labels for risky safe-preview transfers.
//!
//! This module validates that risky preview surfaces keep safe-preview trust
//! class and transferred representation state explicit across copy, export,
//! browser-open, deeper-render, and support-attachment actions.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::TrustClass;

/// Schema version for beta representation-label packets.
pub const REPRESENTATION_LABELS_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for [`RepresentationLabelsBetaPacket`].
pub const REPRESENTATION_LABELS_BETA_PACKET_RECORD_KIND: &str = "representation_labels_beta_packet";

/// Stable record-kind tag for [`RepresentationLabelsBetaValidationReport`].
pub const REPRESENTATION_LABELS_BETA_VALIDATION_REPORT_RECORD_KIND: &str =
    "representation_labels_beta_validation_report";

/// Stable record-kind tag for [`RepresentationExportRecord`].
pub const REPRESENTATION_EXPORT_RECORD_KIND: &str = "representation_export_record";

/// Repo-relative beta UX contract for safe-preview copy and export parity.
pub const REPRESENTATION_LABELS_BETA_DOC_REF: &str = "docs/ux/m3/safe_preview_copy_export_beta.md";

/// Repo-relative content schema for representation export records.
pub const REPRESENTATION_EXPORT_SCHEMA_REF: &str =
    "schemas/content/representation_export.schema.json";

/// Repo-relative protected fixture directory for beta representation labels.
pub const REPRESENTATION_LABELS_BETA_FIXTURE_DIR: &str =
    "fixtures/content/m3/representation_copy_export";

const SAFE_PREVIEW_TRUST_CLASS_DOC_REF: &str = "docs/security/safe_preview_trust_classes.md";
const COPY_EXPORT_PARITY_DOC_REF: &str = "docs/ux/copy_export_representation_parity.md";
const INTERACTION_SAFETY_SCHEMA_REF: &str = "schemas/ux/interaction_safety.schema.json";

/// Risky beta surface kinds covered by representation-label validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationSurfaceKind {
    /// Documentation or help content pane.
    DocsHelpPane,
    /// Suspicious-content inspection surface.
    SuspiciousContentView,
    /// Generated artifact preview or review surface.
    GeneratedArtifact,
    /// Package, extension, install, or update review surface.
    PackageInstallReview,
    /// Repair preview surface.
    RepairPreview,
    /// Support-bundle attachment selection or review surface.
    SupportBundleAttachment,
}

impl RepresentationSurfaceKind {
    /// Stable token used in fixtures and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsHelpPane => "docs_help_pane",
            Self::SuspiciousContentView => "suspicious_content_view",
            Self::GeneratedArtifact => "generated_artifact",
            Self::PackageInstallReview => "package_install_review",
            Self::RepairPreview => "repair_preview",
            Self::SupportBundleAttachment => "support_bundle_attachment",
        }
    }
}

/// Required risky beta surface coverage.
pub const REPRESENTATION_LABELS_REQUIRED_SURFACES: [RepresentationSurfaceKind; 6] = [
    RepresentationSurfaceKind::DocsHelpPane,
    RepresentationSurfaceKind::SuspiciousContentView,
    RepresentationSurfaceKind::GeneratedArtifact,
    RepresentationSurfaceKind::PackageInstallReview,
    RepresentationSurfaceKind::RepairPreview,
    RepresentationSurfaceKind::SupportBundleAttachment,
];

/// Risk class for the content that makes representation labels material.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskyContentClass {
    /// Raw source and rendered output can communicate different meaning.
    RawRenderedAmbiguous,
    /// Suspicious-content detector warning is attached.
    SuspiciousContent,
    /// Content is oversized, windowed, streamed, or otherwise partial.
    OversizedArtifact,
    /// Generated output or generated artifact.
    GeneratedOutput,
}

impl RiskyContentClass {
    /// Stable token used in fixtures and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawRenderedAmbiguous => "raw_rendered_ambiguous",
            Self::SuspiciousContent => "suspicious_content",
            Self::OversizedArtifact => "oversized_artifact",
            Self::GeneratedOutput => "generated_output",
        }
    }
}

/// Required risky content class coverage.
pub const REPRESENTATION_LABELS_REQUIRED_CONTENT_CLASSES: [RiskyContentClass; 4] = [
    RiskyContentClass::RawRenderedAmbiguous,
    RiskyContentClass::SuspiciousContent,
    RiskyContentClass::OversizedArtifact,
    RiskyContentClass::GeneratedOutput,
];

/// Representation class that leaves or is handed off from a risky surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LabeledRepresentationClass {
    /// Exact source bytes or exact source text.
    Raw,
    /// Current rendered view.
    Rendered,
    /// Static sanitized snapshot.
    Sanitized,
    /// Redacted support or evidence representation.
    Redacted,
}

impl LabeledRepresentationClass {
    /// Stable token used in packets and schema rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Raw => "raw",
            Self::Rendered => "rendered",
            Self::Sanitized => "sanitized",
            Self::Redacted => "redacted",
        }
    }
}

/// Required representation labels for risky beta surfaces.
pub const REPRESENTATION_LABELS_REQUIRED_REPRESENTATIONS: [LabeledRepresentationClass; 4] = [
    LabeledRepresentationClass::Raw,
    LabeledRepresentationClass::Rendered,
    LabeledRepresentationClass::Sanitized,
    LabeledRepresentationClass::Redacted,
];

/// Action family whose result must preserve representation truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationActionKind {
    /// Clipboard transfer.
    Copy,
    /// Durable export or evidence packet.
    Export,
    /// System-browser or external browser handoff.
    BrowserOpen,
    /// Product-native richer render override.
    DeeperRenderOverride,
    /// Support-bundle attachment.
    SupportAttachment,
}

impl RepresentationActionKind {
    /// Stable token used in packets and schema rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Export => "export",
            Self::BrowserOpen => "browser_open",
            Self::DeeperRenderOverride => "deeper_render_override",
            Self::SupportAttachment => "support_attachment",
        }
    }
}

/// Required action-family coverage per risky surface.
pub const REPRESENTATION_LABELS_REQUIRED_ACTIONS: [RepresentationActionKind; 5] = [
    RepresentationActionKind::Copy,
    RepresentationActionKind::Export,
    RepresentationActionKind::BrowserOpen,
    RepresentationActionKind::DeeperRenderOverride,
    RepresentationActionKind::SupportAttachment,
];

/// Availability posture for a representation-labeled action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationActionPosture {
    /// Action is available under the current policy and trust class.
    Available,
    /// Action is available only after a review or preview sheet.
    RequiresReview,
    /// Action needs trust elevation before it can run.
    RequiresTrustUpgrade,
    /// Action is blocked.
    Blocked,
}

impl RepresentationActionPosture {
    /// Stable token used in packets and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::RequiresReview => "requires_review",
            Self::RequiresTrustUpgrade => "requires_trust_upgrade",
            Self::Blocked => "blocked",
        }
    }
}

/// Scope of the copied, exported, opened, or attached representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationScopeClass {
    /// Exact anchored selection or single reviewed row.
    AnchoredSelection,
    /// Visible rows, events, cells, or current viewport only.
    VisibleRowsOrEvents,
    /// Named static snapshot.
    NamedSnapshotOnly,
    /// Metadata envelope only.
    MetadataOnly,
}

impl RepresentationScopeClass {
    /// Stable token used in packets and schema rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AnchoredSelection => "anchored_selection",
            Self::VisibleRowsOrEvents => "visible_rows_or_events",
            Self::NamedSnapshotOnly => "named_snapshot_only",
            Self::MetadataOnly => "metadata_only",
        }
    }
}

/// Transform applied before a representation leaves the surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationTransformKind {
    /// No material transform was applied.
    None,
    /// Active or scriptable content was removed.
    SanitizedActiveContentRemoved,
    /// Payload is truncated or windowed.
    TruncatedOrWindowed,
    /// High-risk segments were redacted.
    RedactedHighRiskSegments,
    /// Payload is summarized or sampled.
    SummarizedOrSampled,
}

impl RepresentationTransformKind {
    /// Stable token used in packets and schema rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SanitizedActiveContentRemoved => "sanitized_active_content_removed",
            Self::TruncatedOrWindowed => "truncated_or_windowed",
            Self::RedactedHighRiskSegments => "redacted_high_risk_segments",
            Self::SummarizedOrSampled => "summarized_or_sampled",
        }
    }
}

/// Reason content was omitted from a transferred representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationOmissionReason {
    /// No content was omitted.
    None,
    /// Size budget blocked full-body transfer.
    SizeBudget,
    /// Policy blocked body transfer.
    Policy,
    /// Redaction removed segments.
    Redaction,
    /// Source representation was unavailable.
    SourceMissing,
    /// Type is unsupported for this transfer target.
    UnsupportedType,
}

impl RepresentationOmissionReason {
    /// Stable token used in packets and schema rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::SizeBudget => "size_budget",
            Self::Policy => "policy",
            Self::Redaction => "redaction",
            Self::SourceMissing => "source_missing",
            Self::UnsupportedType => "unsupported_type",
        }
    }
}

/// Origin class for representation-label validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationOriginClass {
    /// User-authored or imported source.
    UserAuthoredOrImported,
    /// Generated content or generated artifact.
    Generated,
    /// Unknown or unresolved origin.
    Unknown,
}

impl RepresentationOriginClass {
    /// Stable token used in packets and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserAuthoredOrImported => "user_authored_or_imported",
            Self::Generated => "generated",
            Self::Unknown => "unknown",
        }
    }
}

/// Fixture/input action supplied by a risky beta surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationLabelsActionInput {
    /// Stable action id on the originating surface.
    pub action_id: String,
    /// Action kind.
    pub action_kind: RepresentationActionKind,
    /// Representation class carried by the action.
    pub representation_class: LabeledRepresentationClass,
    /// Action availability posture.
    pub posture: RepresentationActionPosture,
    /// Human-facing label shown by the surface.
    pub visible_label: String,
    /// Whether this is the default action for its action family.
    pub default_for_kind: bool,
    /// Whether the trust-class badge is visible when this action is offered.
    pub trust_class_badge_visible: bool,
    /// Whether the transferred representation label is visible.
    pub representation_label_visible: bool,
    /// Whether the source trust class is retained in the outgoing packet.
    pub source_trust_class_visible: bool,
    /// Whether current policy permits this action.
    pub policy_allows: bool,
    /// Whether the current trust class permits this action.
    pub trust_class_allows: bool,
    /// Whether redaction was applied before transfer.
    pub redaction_applied: bool,
    /// Whether this action is safe for support-bundle attachment.
    pub support_attachment_safe: bool,
    /// Transfer scope class.
    pub scope_class: RepresentationScopeClass,
    /// Transforms applied to the representation.
    pub transforms_applied: Vec<RepresentationTransformKind>,
    /// Omission reasons disclosed with the representation.
    pub omission_reasons: Vec<RepresentationOmissionReason>,
    /// Citation anchors backing generated or docs/help transfers.
    #[serde(default)]
    pub citation_anchor_refs: Vec<String>,
    /// Browser handoff packet ref for browser-open actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Review ref for deeper-render override actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_review_ref: Option<String>,
}

/// Fixture/input surface for beta representation-label validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationLabelsSurfaceInput {
    /// Risky beta surface kind.
    pub surface_kind: RepresentationSurfaceKind,
    /// Stable surface ref.
    pub surface_ref: String,
    /// Opaque subject ref for the content.
    pub subject_ref: String,
    /// Claimed beta surface row this projection protects.
    pub declared_beta_surface_ref: String,
    /// Risk class for this surface's content.
    pub content_class: RiskyContentClass,
    /// Safe-preview trust class visible on the source surface.
    pub source_trust_class: TrustClass,
    /// Currently visible representation on the source surface.
    pub visible_representation: LabeledRepresentationClass,
    /// Origin class for the subject.
    pub origin_class: RepresentationOriginClass,
    /// Whether raw/source representation remains available.
    pub source_representation_available: bool,
    /// Whether raw and rendered forms can materially diverge.
    pub raw_rendered_ambiguity: bool,
    /// Suspicious-content warning refs attached to this surface.
    #[serde(default)]
    pub suspicious_content_refs: Vec<String>,
    /// Optional support attachment point ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_attachment_point_ref: Option<String>,
    /// Representation-labeled actions offered by the surface.
    pub actions: Vec<RepresentationLabelsActionInput>,
}

/// Case input consumed by fixtures and CLI validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationLabelsBetaCase {
    /// Stable case id.
    pub case_id: String,
    /// Claimed-surface register linked to beta gating.
    pub claim_register_ref: String,
    /// Source contracts consumed by this proof.
    pub source_contract_refs: Vec<String>,
    /// Packet mint timestamp.
    pub minted_at: String,
    /// Risky beta surface rows.
    pub surfaces: Vec<RepresentationLabelsSurfaceInput>,
}

/// Exportable representation record emitted for every protected action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationExportRecord {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable record id.
    pub export_record_id: String,
    /// Case id this record belongs to.
    pub case_id_ref: String,
    /// Action kind token.
    pub action_kind: String,
    /// Source surface token.
    pub source_surface: String,
    /// Source surface ref.
    pub source_surface_ref: String,
    /// Opaque subject ref for the content.
    pub source_subject_ref: String,
    /// Safe-preview trust class visible on the source surface.
    pub source_trust_class: String,
    /// Representation class token.
    pub representation_class: String,
    /// Scope class token.
    pub scope_class: String,
    /// Transform tokens applied before transfer or handoff.
    pub transforms_applied: Vec<String>,
    /// Omission reason tokens.
    pub omission_reasons: Vec<String>,
    /// Trust-class badge visibility at action time.
    pub trust_class_badge_visible: bool,
    /// Representation label visibility at action time.
    pub representation_label_visible: bool,
    /// Source trust class retention in the outgoing packet.
    pub source_trust_class_visible: bool,
    /// Whether redaction was applied before transfer.
    pub redaction_applied: bool,
    /// Whether the representation is safe for support attachment.
    pub support_attachment_safe: bool,
    /// Citation anchors backing generated or docs/help transfers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub citation_anchor_refs: Vec<String>,
    /// Browser handoff packet ref for browser-open actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Review ref for deeper-render override actions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub override_review_ref: Option<String>,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Projected action with its representation export record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationLabelsActionProjection {
    /// Stable action id on the originating surface.
    pub action_id: String,
    /// Action kind token.
    pub action_kind: String,
    /// Representation class token.
    pub representation_class: String,
    /// Action posture token.
    pub posture: String,
    /// Human-facing label shown by the surface.
    pub visible_label: String,
    /// Whether this is the default action for its action family.
    pub default_for_kind: bool,
    /// Whether current policy permits this action.
    pub policy_allows: bool,
    /// Whether the current trust class permits this action.
    pub trust_class_allows: bool,
    /// Representation export record for the action.
    pub representation_export_record: RepresentationExportRecord,
}

/// Projected risky beta surface row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationLabelsSurfaceRow {
    /// Risky beta surface kind enum.
    pub surface_kind: RepresentationSurfaceKind,
    /// Risky beta surface token.
    pub surface_token: String,
    /// Stable surface ref.
    pub surface_ref: String,
    /// Opaque subject ref for the content.
    pub subject_ref: String,
    /// Claimed beta surface row this projection protects.
    pub declared_beta_surface_ref: String,
    /// Risk class token for this surface's content.
    pub content_class: String,
    /// Safe-preview trust class visible on the source surface.
    pub source_trust_class: String,
    /// Currently visible representation token.
    pub visible_representation: String,
    /// Origin class token for the subject.
    pub origin_class: String,
    /// Whether raw/source representation remains available.
    pub source_representation_available: bool,
    /// Whether raw and rendered forms can materially diverge.
    pub raw_rendered_ambiguity: bool,
    /// Suspicious-content warning refs attached to this surface.
    pub suspicious_content_refs: Vec<String>,
    /// Optional support attachment point ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub support_attachment_point_ref: Option<String>,
    /// Representation-labeled actions offered by the surface.
    pub actions: Vec<RepresentationLabelsActionProjection>,
}

impl RepresentationLabelsSurfaceRow {
    /// Returns representation tokens offered by this surface.
    pub fn representation_tokens(&self) -> BTreeSet<&str> {
        self.actions
            .iter()
            .map(|action| action.representation_class.as_str())
            .collect()
    }

    /// Returns true when this surface maps every required action kind.
    pub fn covers_required_actions(&self) -> bool {
        let actual = self
            .actions
            .iter()
            .map(|action| action.action_kind.as_str())
            .collect::<BTreeSet<_>>();
        REPRESENTATION_LABELS_REQUIRED_ACTIONS
            .iter()
            .all(|kind| actual.contains(kind.as_str()))
    }
}

/// Beta packet proving representation labels on risky safe-preview surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationLabelsBetaPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable case id.
    pub case_id: String,
    /// Claimed-surface register linked to beta gating.
    pub claim_register_ref: String,
    /// Source contracts consumed by this proof.
    pub source_contract_refs: Vec<String>,
    /// Risky beta surface projections.
    pub surfaces: Vec<RepresentationLabelsSurfaceRow>,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl RepresentationLabelsBetaPacket {
    /// Builds a beta representation-label packet from fixture or consumer input.
    pub fn from_case(case: RepresentationLabelsBetaCase) -> Self {
        let case_id = case.case_id.clone();
        let minted_at = case.minted_at.clone();
        let surfaces = case
            .surfaces
            .into_iter()
            .map(|surface| project_surface(&case_id, &minted_at, surface))
            .collect();

        Self {
            record_kind: REPRESENTATION_LABELS_BETA_PACKET_RECORD_KIND.to_owned(),
            schema_version: REPRESENTATION_LABELS_BETA_SCHEMA_VERSION,
            case_id: case.case_id,
            claim_register_ref: case.claim_register_ref,
            source_contract_refs: case.source_contract_refs,
            surfaces,
            minted_at: case.minted_at,
        }
    }

    /// Validates representation-label parity and returns a gate report.
    pub fn validate(&self) -> RepresentationLabelsBetaValidationReport {
        let mut violations = Vec::new();

        validate_identity(self, &mut violations);
        validate_contracts(self, &mut violations);
        validate_surface_coverage(self, &mut violations);
        validate_content_coverage(self, &mut violations);
        for surface in &self.surfaces {
            validate_surface(self, surface, &mut violations);
        }

        let status = if violations.is_empty() {
            RepresentationLabelsBetaGateStatus::Green
        } else {
            RepresentationLabelsBetaGateStatus::Blocked
        };
        let blocked_beta_surface_refs = if violations.is_empty() {
            Vec::new()
        } else {
            self.surfaces
                .iter()
                .map(|surface| surface.declared_beta_surface_ref.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect()
        };

        RepresentationLabelsBetaValidationReport {
            record_kind: REPRESENTATION_LABELS_BETA_VALIDATION_REPORT_RECORD_KIND.to_owned(),
            schema_version: REPRESENTATION_LABELS_BETA_SCHEMA_VERSION,
            case_id: self.case_id.clone(),
            status: status.as_str().to_owned(),
            violations,
            validated_surface_count: self.surfaces.len(),
            observed_surface_tokens: observed_surface_tokens(&self.surfaces),
            observed_content_classes: observed_content_classes(&self.surfaces),
            observed_representation_tokens: observed_representation_tokens(&self.surfaces),
            blocked_beta_surface_refs,
        }
    }

    /// Returns true when validation reports a green beta gate.
    pub fn beta_gate_is_green(&self) -> bool {
        self.validate().is_green()
    }
}

/// Gate status for beta representation-label validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RepresentationLabelsBetaGateStatus {
    /// Packet is promotable for declared beta surfaces.
    Green,
    /// Packet blocks beta promotion.
    Blocked,
}

impl RepresentationLabelsBetaGateStatus {
    /// Stable token used in reports and admission hooks.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Blocked => "blocked",
        }
    }
}

/// Validation report emitted by [`RepresentationLabelsBetaPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationLabelsBetaValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Case id validated by this report.
    pub case_id: String,
    /// `green` when no violations were found, otherwise `blocked`.
    pub status: String,
    /// Violations found during validation.
    pub violations: Vec<RepresentationLabelsBetaViolation>,
    /// Number of surface rows validated.
    pub validated_surface_count: usize,
    /// Surface tokens observed in the packet.
    pub observed_surface_tokens: Vec<String>,
    /// Risky content class tokens observed in the packet.
    pub observed_content_classes: Vec<String>,
    /// Representation tokens observed across actions.
    pub observed_representation_tokens: Vec<String>,
    /// Claimed beta surface refs blocked by a non-green packet.
    pub blocked_beta_surface_refs: Vec<String>,
}

impl RepresentationLabelsBetaValidationReport {
    /// Returns true when the report has green status and no violations.
    pub fn is_green(&self) -> bool {
        self.status == RepresentationLabelsBetaGateStatus::Green.as_str()
            && self.violations.is_empty()
    }

    /// Returns the violation ids present in this report.
    pub fn violation_ids(&self) -> BTreeSet<&str> {
        self.violations
            .iter()
            .map(|violation| violation.violation_id.as_str())
            .collect()
    }
}

/// One validation violation surfaced by the beta representation-label gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RepresentationLabelsBetaViolation {
    /// Stable violation id.
    pub violation_id: String,
    /// Surface token, if the violation is surface-specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
    /// Action id, if the violation is action-specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub action_id: Option<String>,
    /// Reviewable summary.
    pub summary: String,
}

impl RepresentationLabelsBetaViolation {
    fn new(violation_id: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            violation_id: violation_id.into(),
            surface: None,
            action_id: None,
            summary: summary.into(),
        }
    }

    fn for_surface(
        surface: &RepresentationLabelsSurfaceRow,
        violation_id: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            violation_id: violation_id.into(),
            surface: Some(surface.surface_token.clone()),
            action_id: None,
            summary: summary.into(),
        }
    }

    fn for_action(
        surface: &RepresentationLabelsSurfaceRow,
        action: &RepresentationLabelsActionProjection,
        violation_id: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            violation_id: violation_id.into(),
            surface: Some(surface.surface_token.clone()),
            action_id: Some(action.action_id.clone()),
            summary: summary.into(),
        }
    }
}

fn project_surface(
    case_id: &str,
    minted_at: &str,
    surface: RepresentationLabelsSurfaceInput,
) -> RepresentationLabelsSurfaceRow {
    let surface_token = surface.surface_kind.as_str().to_owned();
    let content_class = surface.content_class.as_str().to_owned();
    let source_trust_class = surface.source_trust_class.as_str().to_owned();
    let visible_representation = surface.visible_representation.as_str().to_owned();
    let origin_class = surface.origin_class.as_str().to_owned();
    let actions = surface
        .actions
        .into_iter()
        .map(|action| {
            let export_record_id = format!(
                "representation-export:{}:{}",
                sanitize_id(&surface.surface_ref),
                sanitize_id(&action.action_id)
            );
            let representation_export_record = RepresentationExportRecord {
                record_kind: REPRESENTATION_EXPORT_RECORD_KIND.to_owned(),
                schema_version: REPRESENTATION_LABELS_BETA_SCHEMA_VERSION,
                export_record_id,
                case_id_ref: case_id.to_owned(),
                action_kind: action.action_kind.as_str().to_owned(),
                source_surface: surface_token.clone(),
                source_surface_ref: surface.surface_ref.clone(),
                source_subject_ref: surface.subject_ref.clone(),
                source_trust_class: source_trust_class.clone(),
                representation_class: action.representation_class.as_str().to_owned(),
                scope_class: action.scope_class.as_str().to_owned(),
                transforms_applied: tokens(
                    action.transforms_applied.iter().map(|item| item.as_str()),
                ),
                omission_reasons: tokens(action.omission_reasons.iter().map(|item| item.as_str())),
                trust_class_badge_visible: action.trust_class_badge_visible,
                representation_label_visible: action.representation_label_visible,
                source_trust_class_visible: action.source_trust_class_visible,
                redaction_applied: action.redaction_applied,
                support_attachment_safe: action.support_attachment_safe,
                citation_anchor_refs: action.citation_anchor_refs.clone(),
                browser_handoff_packet_ref: action.browser_handoff_packet_ref.clone(),
                override_review_ref: action.override_review_ref.clone(),
                minted_at: minted_at.to_owned(),
            };

            RepresentationLabelsActionProjection {
                action_id: action.action_id,
                action_kind: action.action_kind.as_str().to_owned(),
                representation_class: action.representation_class.as_str().to_owned(),
                posture: action.posture.as_str().to_owned(),
                visible_label: action.visible_label,
                default_for_kind: action.default_for_kind,
                policy_allows: action.policy_allows,
                trust_class_allows: action.trust_class_allows,
                representation_export_record,
            }
        })
        .collect();

    RepresentationLabelsSurfaceRow {
        surface_kind: surface.surface_kind,
        surface_token,
        surface_ref: surface.surface_ref,
        subject_ref: surface.subject_ref,
        declared_beta_surface_ref: surface.declared_beta_surface_ref,
        content_class,
        source_trust_class,
        visible_representation,
        origin_class,
        source_representation_available: surface.source_representation_available,
        raw_rendered_ambiguity: surface.raw_rendered_ambiguity,
        suspicious_content_refs: surface.suspicious_content_refs,
        support_attachment_point_ref: surface.support_attachment_point_ref,
        actions,
    }
}

fn validate_identity(
    packet: &RepresentationLabelsBetaPacket,
    violations: &mut Vec<RepresentationLabelsBetaViolation>,
) {
    if packet.record_kind != REPRESENTATION_LABELS_BETA_PACKET_RECORD_KIND {
        violations.push(RepresentationLabelsBetaViolation::new(
            "wrong_record_kind",
            "packet record_kind is not representation_labels_beta_packet",
        ));
    }
    if packet.schema_version != REPRESENTATION_LABELS_BETA_SCHEMA_VERSION {
        violations.push(RepresentationLabelsBetaViolation::new(
            "wrong_schema_version",
            "packet schema_version is not supported",
        ));
    }
    if packet.case_id.trim().is_empty()
        || packet.claim_register_ref.trim().is_empty()
        || packet.minted_at.trim().is_empty()
    {
        violations.push(RepresentationLabelsBetaViolation::new(
            "missing_identity",
            "packet identity, claim register, or mint timestamp is missing",
        ));
    }
}

fn validate_contracts(
    packet: &RepresentationLabelsBetaPacket,
    violations: &mut Vec<RepresentationLabelsBetaViolation>,
) {
    for required in [
        REPRESENTATION_LABELS_BETA_DOC_REF,
        REPRESENTATION_EXPORT_SCHEMA_REF,
        SAFE_PREVIEW_TRUST_CLASS_DOC_REF,
        COPY_EXPORT_PARITY_DOC_REF,
        INTERACTION_SAFETY_SCHEMA_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(RepresentationLabelsBetaViolation::new(
                "missing_source_contract_ref",
                format!("packet is missing source contract ref {required}"),
            ));
        }
    }
}

fn validate_surface_coverage(
    packet: &RepresentationLabelsBetaPacket,
    violations: &mut Vec<RepresentationLabelsBetaViolation>,
) {
    let counts = count_tokens(
        packet
            .surfaces
            .iter()
            .map(|surface| surface.surface_token.as_str()),
    );
    for required in REPRESENTATION_LABELS_REQUIRED_SURFACES {
        if !counts.contains_key(required.as_str()) {
            violations.push(RepresentationLabelsBetaViolation::new(
                "missing_required_surface",
                format!("packet is missing required surface {}", required.as_str()),
            ));
        }
    }
    for (surface, count) in counts {
        if count > 1 {
            violations.push(RepresentationLabelsBetaViolation::new(
                "duplicate_surface",
                format!("packet contains duplicate surface {surface}"),
            ));
        }
    }
}

fn validate_content_coverage(
    packet: &RepresentationLabelsBetaPacket,
    violations: &mut Vec<RepresentationLabelsBetaViolation>,
) {
    let present = packet
        .surfaces
        .iter()
        .map(|surface| surface.content_class.as_str())
        .collect::<BTreeSet<_>>();
    for required in REPRESENTATION_LABELS_REQUIRED_CONTENT_CLASSES {
        if !present.contains(required.as_str()) {
            violations.push(RepresentationLabelsBetaViolation::new(
                "missing_required_content_class",
                format!(
                    "packet is missing required risky content class {}",
                    required.as_str()
                ),
            ));
        }
    }
}

fn validate_surface(
    packet: &RepresentationLabelsBetaPacket,
    surface: &RepresentationLabelsSurfaceRow,
    violations: &mut Vec<RepresentationLabelsBetaViolation>,
) {
    if surface.surface_ref.trim().is_empty()
        || surface.subject_ref.trim().is_empty()
        || surface.declared_beta_surface_ref.trim().is_empty()
        || surface.source_trust_class.trim().is_empty()
        || surface.visible_representation.trim().is_empty()
    {
        violations.push(RepresentationLabelsBetaViolation::for_surface(
            surface,
            "surface_identity_missing",
            "surface identity, subject, claim ref, trust class, or visible representation is missing",
        ));
    }

    if !surface.covers_required_actions() {
        violations.push(RepresentationLabelsBetaViolation::for_surface(
            surface,
            "missing_required_action_kind",
            "surface must map copy, export, browser-open, deeper-render, and support-attachment actions",
        ));
    }

    validate_default_counts(surface, violations);
    validate_content_class_rules(surface, violations);
    for action in &surface.actions {
        validate_action(packet, surface, action, violations);
    }
}

fn validate_default_counts(
    surface: &RepresentationLabelsSurfaceRow,
    violations: &mut Vec<RepresentationLabelsBetaViolation>,
) {
    let mut counts = BTreeMap::<&str, usize>::new();
    for action in &surface.actions {
        if action.default_for_kind {
            *counts.entry(action.action_kind.as_str()).or_default() += 1;
        }
    }
    for required in REPRESENTATION_LABELS_REQUIRED_ACTIONS {
        if counts.get(required.as_str()).copied().unwrap_or_default() != 1 {
            violations.push(RepresentationLabelsBetaViolation::for_surface(
                surface,
                "default_action_count_invalid",
                format!(
                    "surface must declare exactly one default row for {}",
                    required.as_str()
                ),
            ));
        }
    }
}

fn validate_content_class_rules(
    surface: &RepresentationLabelsSurfaceRow,
    violations: &mut Vec<RepresentationLabelsBetaViolation>,
) {
    let representations = surface.representation_tokens();
    match surface.content_class.as_str() {
        "raw_rendered_ambiguous" => {
            if !surface.raw_rendered_ambiguity
                || !representations.contains(LabeledRepresentationClass::Raw.as_str())
                || !representations.contains(LabeledRepresentationClass::Rendered.as_str())
            {
                violations.push(RepresentationLabelsBetaViolation::for_surface(
                    surface,
                    "raw_rendered_ambiguity_not_labelled",
                    "raw/rendered ambiguous content must expose both raw and rendered labels",
                ));
            }
        }
        "suspicious_content" => {
            if surface.suspicious_content_refs.is_empty()
                || !representations.contains(LabeledRepresentationClass::Raw.as_str())
                || !representations.contains(LabeledRepresentationClass::Sanitized.as_str())
                || !representations.contains(LabeledRepresentationClass::Redacted.as_str())
            {
                violations.push(RepresentationLabelsBetaViolation::for_surface(
                    surface,
                    "suspicious_content_not_preserved",
                    "suspicious content must keep warning refs plus raw, sanitized, and redacted labels",
                ));
            }
        }
        "oversized_artifact" => {
            if !surface.actions.iter().any(action_discloses_size_limit) {
                violations.push(RepresentationLabelsBetaViolation::for_surface(
                    surface,
                    "oversized_scope_truth_missing",
                    "oversized content must disclose windowing, size-budget omission, or metadata-only scope",
                ));
            }
        }
        "generated_output" => {
            if surface.origin_class != RepresentationOriginClass::Generated.as_str()
                || surface.actions.iter().any(|action| {
                    action.action_kind != RepresentationActionKind::DeeperRenderOverride.as_str()
                        && action
                            .representation_export_record
                            .citation_anchor_refs
                            .is_empty()
                })
            {
                violations.push(RepresentationLabelsBetaViolation::for_surface(
                    surface,
                    "generated_output_citations_missing",
                    "generated output must keep generated origin and citation anchors on transferable actions",
                ));
            }
        }
        _ => violations.push(RepresentationLabelsBetaViolation::for_surface(
            surface,
            "unknown_content_class",
            "surface content class is not in the beta representation-label vocabulary",
        )),
    }
}

fn validate_action(
    packet: &RepresentationLabelsBetaPacket,
    surface: &RepresentationLabelsSurfaceRow,
    action: &RepresentationLabelsActionProjection,
    violations: &mut Vec<RepresentationLabelsBetaViolation>,
) {
    let record = &action.representation_export_record;
    if action.action_id.trim().is_empty() || action.visible_label.trim().is_empty() {
        violations.push(RepresentationLabelsBetaViolation::for_action(
            surface,
            action,
            "action_label_missing",
            "action id or visible label is missing",
        ));
    }
    if !record.trust_class_badge_visible
        || !record.representation_label_visible
        || !record.source_trust_class_visible
    {
        violations.push(RepresentationLabelsBetaViolation::for_action(
            surface,
            action,
            "representation_truth_not_visible",
            "action must keep trust class badge, representation label, and source trust class visible",
        ));
    }
    if !record_matches(packet, surface, action, record) {
        violations.push(RepresentationLabelsBetaViolation::for_action(
            surface,
            action,
            "representation_export_record_mismatch",
            "action does not reconcile with its representation export record",
        ));
    }
    if action_is_available(action) && (!action.policy_allows || !action.trust_class_allows) {
        violations.push(RepresentationLabelsBetaViolation::for_action(
            surface,
            action,
            "action_available_without_policy_or_trust",
            "available action is not permitted by both policy and the current trust class",
        ));
    }
    if record.representation_class == LabeledRepresentationClass::Raw.as_str()
        && action_is_available(action)
        && !surface.source_representation_available
    {
        violations.push(RepresentationLabelsBetaViolation::for_action(
            surface,
            action,
            "raw_representation_source_missing",
            "available raw representation requires source representation availability",
        ));
    }
    if action.action_kind == RepresentationActionKind::BrowserOpen.as_str()
        && action_is_available(action)
        && record.browser_handoff_packet_ref.is_none()
    {
        violations.push(RepresentationLabelsBetaViolation::for_action(
            surface,
            action,
            "browser_handoff_packet_missing",
            "available browser-open action must carry a handoff packet ref",
        ));
    }
    if action.action_kind == RepresentationActionKind::DeeperRenderOverride.as_str()
        && action_is_available(action)
        && record.override_review_ref.is_none()
    {
        violations.push(RepresentationLabelsBetaViolation::for_action(
            surface,
            action,
            "deeper_render_review_missing",
            "available deeper-render override must carry a review ref",
        ));
    }
    if action.action_kind == RepresentationActionKind::SupportAttachment.as_str() {
        validate_support_attachment(surface, action, violations);
    }
    if record.representation_class == LabeledRepresentationClass::Redacted.as_str()
        && !record.redaction_applied
    {
        violations.push(RepresentationLabelsBetaViolation::for_action(
            surface,
            action,
            "redacted_without_redaction",
            "redacted representation is declared without redaction applied",
        ));
    }
    if record.representation_class == LabeledRepresentationClass::Sanitized.as_str()
        && !record
            .transforms_applied
            .iter()
            .any(|item| item == RepresentationTransformKind::SanitizedActiveContentRemoved.as_str())
    {
        violations.push(RepresentationLabelsBetaViolation::for_action(
            surface,
            action,
            "sanitized_transform_missing",
            "sanitized representation must disclose active-content removal",
        ));
    }
}

fn validate_support_attachment(
    surface: &RepresentationLabelsSurfaceRow,
    action: &RepresentationLabelsActionProjection,
    violations: &mut Vec<RepresentationLabelsBetaViolation>,
) {
    let record = &action.representation_export_record;
    if surface.support_attachment_point_ref.is_none() {
        violations.push(RepresentationLabelsBetaViolation::for_action(
            surface,
            action,
            "support_attachment_point_missing",
            "support-attachment action must name the attachment point",
        ));
    }
    if !record.support_attachment_safe
        || !matches!(
            record.representation_class.as_str(),
            "sanitized" | "redacted"
        )
    {
        violations.push(RepresentationLabelsBetaViolation::for_action(
            surface,
            action,
            "support_attachment_unsafe_representation",
            "support attachments must use sanitized or redacted support-safe representations",
        ));
    }
}

fn action_discloses_size_limit(action: &RepresentationLabelsActionProjection) -> bool {
    let record = &action.representation_export_record;
    record.scope_class == RepresentationScopeClass::VisibleRowsOrEvents.as_str()
        || record.scope_class == RepresentationScopeClass::MetadataOnly.as_str()
        || record
            .transforms_applied
            .iter()
            .any(|item| item == RepresentationTransformKind::TruncatedOrWindowed.as_str())
        || record
            .omission_reasons
            .iter()
            .any(|item| item == RepresentationOmissionReason::SizeBudget.as_str())
}

fn action_is_available(action: &RepresentationLabelsActionProjection) -> bool {
    matches!(action.posture.as_str(), "available" | "requires_review")
}

fn record_matches(
    packet: &RepresentationLabelsBetaPacket,
    surface: &RepresentationLabelsSurfaceRow,
    action: &RepresentationLabelsActionProjection,
    record: &RepresentationExportRecord,
) -> bool {
    record.record_kind == REPRESENTATION_EXPORT_RECORD_KIND
        && record.schema_version == REPRESENTATION_LABELS_BETA_SCHEMA_VERSION
        && record.case_id_ref == packet.case_id
        && record.action_kind == action.action_kind
        && record.source_surface == surface.surface_token
        && record.source_surface_ref == surface.surface_ref
        && record.source_subject_ref == surface.subject_ref
        && record.source_trust_class == surface.source_trust_class
        && record.representation_class == action.representation_class
}

fn observed_surface_tokens(surfaces: &[RepresentationLabelsSurfaceRow]) -> Vec<String> {
    surfaces
        .iter()
        .map(|surface| surface.surface_token.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn observed_content_classes(surfaces: &[RepresentationLabelsSurfaceRow]) -> Vec<String> {
    surfaces
        .iter()
        .map(|surface| surface.content_class.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn observed_representation_tokens(surfaces: &[RepresentationLabelsSurfaceRow]) -> Vec<String> {
    surfaces
        .iter()
        .flat_map(|surface| {
            surface
                .actions
                .iter()
                .map(|action| action.representation_class.clone())
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn count_tokens<'a>(tokens: impl Iterator<Item = &'a str>) -> BTreeMap<&'a str, usize> {
    let mut counts = BTreeMap::new();
    for token in tokens {
        *counts.entry(token).or_default() += 1;
    }
    counts
}

fn tokens<'a>(values: impl Iterator<Item = &'a str>) -> Vec<String> {
    values.map(str::to_owned).collect()
}

fn sanitize_id(value: &str) -> String {
    let mut out = String::new();
    let mut last_sep = true;
    for ch in value.chars() {
        let ch = ch.to_ascii_lowercase();
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_sep = false;
            continue;
        }
        if last_sep {
            continue;
        }
        out.push('-');
        last_sep = true;
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.is_empty() {
        "root".to_owned()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn action(
        action_kind: RepresentationActionKind,
        representation_class: LabeledRepresentationClass,
    ) -> RepresentationLabelsActionInput {
        let action_id = format!("{}_{}", action_kind.as_str(), representation_class.as_str());
        let support = action_kind == RepresentationActionKind::SupportAttachment;
        let sanitized = representation_class == LabeledRepresentationClass::Sanitized;
        let redacted = representation_class == LabeledRepresentationClass::Redacted;
        RepresentationLabelsActionInput {
            action_id,
            action_kind,
            representation_class,
            posture: if action_kind == RepresentationActionKind::DeeperRenderOverride {
                RepresentationActionPosture::RequiresTrustUpgrade
            } else {
                RepresentationActionPosture::Available
            },
            visible_label: format!("{} {}", action_kind.as_str(), representation_class.as_str()),
            default_for_kind: true,
            trust_class_badge_visible: true,
            representation_label_visible: true,
            source_trust_class_visible: true,
            policy_allows: action_kind != RepresentationActionKind::DeeperRenderOverride,
            trust_class_allows: action_kind != RepresentationActionKind::DeeperRenderOverride,
            redaction_applied: redacted,
            support_attachment_safe: support,
            scope_class: RepresentationScopeClass::NamedSnapshotOnly,
            transforms_applied: if sanitized {
                vec![RepresentationTransformKind::SanitizedActiveContentRemoved]
            } else if redacted {
                vec![RepresentationTransformKind::RedactedHighRiskSegments]
            } else {
                vec![RepresentationTransformKind::None]
            },
            omission_reasons: if redacted {
                vec![RepresentationOmissionReason::Redaction]
            } else {
                vec![RepresentationOmissionReason::None]
            },
            citation_anchor_refs: Vec::new(),
            browser_handoff_packet_ref: (action_kind == RepresentationActionKind::BrowserOpen)
                .then(|| "browser-handoff:unit".to_owned()),
            override_review_ref: None,
        }
    }

    fn surface(surface_kind: RepresentationSurfaceKind) -> RepresentationLabelsSurfaceInput {
        RepresentationLabelsSurfaceInput {
            surface_kind,
            surface_ref: format!("surface:{}", surface_kind.as_str()),
            subject_ref: format!("subject:{}", surface_kind.as_str()),
            declared_beta_surface_ref: "beta_surface:representation_labels".to_owned(),
            content_class: RiskyContentClass::RawRenderedAmbiguous,
            source_trust_class: TrustClass::SanitizedRich,
            visible_representation: LabeledRepresentationClass::Rendered,
            origin_class: RepresentationOriginClass::UserAuthoredOrImported,
            source_representation_available: true,
            raw_rendered_ambiguity: true,
            suspicious_content_refs: Vec::new(),
            support_attachment_point_ref: Some("support-attachment:unit".to_owned()),
            actions: vec![
                action(
                    RepresentationActionKind::Copy,
                    LabeledRepresentationClass::Raw,
                ),
                action(
                    RepresentationActionKind::Export,
                    LabeledRepresentationClass::Sanitized,
                ),
                action(
                    RepresentationActionKind::BrowserOpen,
                    LabeledRepresentationClass::Rendered,
                ),
                action(
                    RepresentationActionKind::DeeperRenderOverride,
                    LabeledRepresentationClass::Rendered,
                ),
                action(
                    RepresentationActionKind::SupportAttachment,
                    LabeledRepresentationClass::Redacted,
                ),
            ],
        }
    }

    fn case() -> RepresentationLabelsBetaCase {
        let mut surfaces = REPRESENTATION_LABELS_REQUIRED_SURFACES
            .iter()
            .copied()
            .map(surface)
            .collect::<Vec<_>>();
        surfaces[1].content_class = RiskyContentClass::SuspiciousContent;
        surfaces[1].suspicious_content_refs = vec!["content-integrity:unit".to_owned()];
        surfaces[2].content_class = RiskyContentClass::GeneratedOutput;
        surfaces[2].origin_class = RepresentationOriginClass::Generated;
        surfaces[2].actions[0].representation_class = LabeledRepresentationClass::Rendered;
        for action in &mut surfaces[2].actions {
            if action.action_kind != RepresentationActionKind::DeeperRenderOverride {
                action.citation_anchor_refs = vec!["citation:unit".to_owned()];
            }
        }
        surfaces[4].content_class = RiskyContentClass::OversizedArtifact;
        surfaces[4].actions[1].scope_class = RepresentationScopeClass::VisibleRowsOrEvents;
        surfaces[4].actions[1].transforms_applied = vec![
            RepresentationTransformKind::SanitizedActiveContentRemoved,
            RepresentationTransformKind::TruncatedOrWindowed,
        ];
        surfaces[5].content_class = RiskyContentClass::OversizedArtifact;
        surfaces[5].actions[4].scope_class = RepresentationScopeClass::MetadataOnly;
        surfaces[5].actions[4].omission_reasons = vec![RepresentationOmissionReason::SizeBudget];

        RepresentationLabelsBetaCase {
            case_id: "case:representation-labels:unit".to_owned(),
            claim_register_ref: "artifacts/milestones/m3/claimed_surface_register.json".to_owned(),
            source_contract_refs: vec![
                REPRESENTATION_LABELS_BETA_DOC_REF.to_owned(),
                REPRESENTATION_EXPORT_SCHEMA_REF.to_owned(),
                SAFE_PREVIEW_TRUST_CLASS_DOC_REF.to_owned(),
                COPY_EXPORT_PARITY_DOC_REF.to_owned(),
                INTERACTION_SAFETY_SCHEMA_REF.to_owned(),
            ],
            minted_at: "2026-05-17T00:00:00Z".to_owned(),
            surfaces,
        }
    }

    #[test]
    fn validates_green_beta_packet() {
        let packet = RepresentationLabelsBetaPacket::from_case(case());
        let report = packet.validate();

        assert!(report.is_green(), "{report:#?}");
        assert_eq!(
            report.observed_representation_tokens,
            vec![
                "raw".to_owned(),
                "redacted".to_owned(),
                "rendered".to_owned(),
                "sanitized".to_owned(),
            ]
        );
    }

    #[test]
    fn blocks_browser_open_without_policy() {
        let mut case = case();
        let action = case.surfaces[0]
            .actions
            .iter_mut()
            .find(|action| action.action_kind == RepresentationActionKind::BrowserOpen)
            .expect("browser-open action exists");
        action.policy_allows = false;

        let report = RepresentationLabelsBetaPacket::from_case(case).validate();

        assert!(report
            .violation_ids()
            .contains("action_available_without_policy_or_trust"));
    }

    #[test]
    fn blocks_raw_support_attachment() {
        let mut case = case();
        let action = case.surfaces[0]
            .actions
            .iter_mut()
            .find(|action| action.action_kind == RepresentationActionKind::SupportAttachment)
            .expect("support attachment exists");
        action.representation_class = LabeledRepresentationClass::Raw;

        let report = RepresentationLabelsBetaPacket::from_case(case).validate();

        assert!(report
            .violation_ids()
            .contains("support_attachment_unsafe_representation"));
    }
}
