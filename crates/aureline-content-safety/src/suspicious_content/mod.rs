//! Beta content-integrity packet for shared suspicious-content detection.
//!
//! The lower-level detector and warning projection modules own byte-exact
//! findings. This module packages one detector run into a governed beta packet
//! that product, CLI, docs, review, search, AI, install, and support surfaces
//! can validate before claiming green content-integrity posture.

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    detect_suspicious_content, project_content_integrity_warnings_from_detection,
    ContentIntegritySurfaceKind, ContentIntegrityWarningRecord, SuspiciousContentDetection,
    TrustClass, CONTENT_INTEGRITY_WARNING_RECORD_KIND,
};

/// Stable record-kind tag for [`ContentIntegrityBetaPacket`].
pub const CONTENT_INTEGRITY_BETA_PACKET_RECORD_KIND: &str = "content_integrity_beta_packet";

/// Stable record-kind tag for [`ContentIntegrityBetaValidationReport`].
pub const CONTENT_INTEGRITY_BETA_VALIDATION_REPORT_RECORD_KIND: &str =
    "content_integrity_beta_validation_report";

/// Schema version for beta content-integrity packets and reports.
pub const CONTENT_INTEGRITY_BETA_SCHEMA_VERSION: u32 = 1;

/// Repo-relative docs contract consumed by this packet.
pub const CONTENT_INTEGRITY_BETA_DOC_REF: &str = "docs/security/m3/content_integrity_beta.md";

/// Repo-relative protected fixture directory for the shared detector packet.
pub const CONTENT_INTEGRITY_BETA_FIXTURE_DIR: &str = "fixtures/content_safety/m3/shared_detector";

/// Repo-relative checked-in packet emitted by the beta content-integrity lane.
pub const CONTENT_INTEGRITY_BETA_PACKET_REF: &str =
    "artifacts/security/m3/content_integrity_beta_packet.json";

const SUSPICIOUS_CONTENT_PACKET_DOC_REF: &str = "docs/security/suspicious_content_packet.md";
const SUSPICIOUS_TEXT_ALPHA_DOC_REF: &str = "docs/security/suspicious_text_alpha.md";
const REPRESENTATION_COPY_EXPORT_DOC_REF: &str =
    "docs/security/representation_copy_export_alpha.md";

/// Declared beta surface set covered by the shared detector packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentIntegrityBetaSurfaceKind {
    /// Editor buffer or raw source editor.
    Editor,
    /// Diff hunk or merge hunk surface.
    Diff,
    /// Search result row, query result, or quick-open row.
    Search,
    /// Review workspace, hunk thread, or review packet anchor.
    Review,
    /// Documentation/help preview or docs search result.
    Docs,
    /// Safe-preview card for rich or risky content.
    SafePreview,
    /// Package, extension, install, or update review surface.
    InstallReview,
    /// AI context, evidence, tainted-context, or review-assist surface.
    AiContext,
    /// Support export, diagnostic bundle, or operator evidence review surface.
    SupportExport,
}

impl ContentIntegrityBetaSurfaceKind {
    /// Stable token used in fixtures, packets, and validation reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Editor => "editor",
            Self::Diff => "diff",
            Self::Search => "search",
            Self::Review => "review",
            Self::Docs => "docs",
            Self::SafePreview => "safe_preview",
            Self::InstallReview => "install_review",
            Self::AiContext => "ai_context",
            Self::SupportExport => "support_export",
        }
    }

    const fn warning_surface_kind(self) -> ContentIntegritySurfaceKind {
        match self {
            Self::Editor => ContentIntegritySurfaceKind::Editor,
            Self::Diff => ContentIntegritySurfaceKind::Diff,
            Self::Search => ContentIntegritySurfaceKind::Search,
            Self::Review => ContentIntegritySurfaceKind::Review,
            Self::Docs => ContentIntegritySurfaceKind::Docs,
            Self::SafePreview => ContentIntegritySurfaceKind::Preview,
            Self::InstallReview => ContentIntegritySurfaceKind::Package,
            Self::AiContext => ContentIntegritySurfaceKind::AiContext,
            Self::SupportExport => ContentIntegritySurfaceKind::SupportExport,
        }
    }
}

/// Required surface coverage for the beta content-integrity packet.
pub const CONTENT_INTEGRITY_BETA_REQUIRED_SURFACES: [ContentIntegrityBetaSurfaceKind; 9] = [
    ContentIntegrityBetaSurfaceKind::Editor,
    ContentIntegrityBetaSurfaceKind::Diff,
    ContentIntegrityBetaSurfaceKind::Search,
    ContentIntegrityBetaSurfaceKind::Review,
    ContentIntegrityBetaSurfaceKind::Docs,
    ContentIntegrityBetaSurfaceKind::SafePreview,
    ContentIntegrityBetaSurfaceKind::InstallReview,
    ContentIntegrityBetaSurfaceKind::AiContext,
    ContentIntegrityBetaSurfaceKind::SupportExport,
];

/// Representation classes that must stay explicit in copy, export, and review flows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentIntegrityRepresentationClass {
    /// Exact source bytes or exact source text.
    Raw,
    /// Current rendered representation.
    Rendered,
    /// Static sanitized representation.
    Sanitized,
    /// Redacted support or evidence representation.
    Redacted,
}

impl ContentIntegrityRepresentationClass {
    /// Stable token used in packet rows and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Raw => "raw",
            Self::Rendered => "rendered",
            Self::Sanitized => "sanitized",
            Self::Redacted => "redacted",
        }
    }
}

/// Required representation vocabulary for beta copy/export paths.
pub const CONTENT_INTEGRITY_REQUIRED_REPRESENTATIONS: [ContentIntegrityRepresentationClass; 4] = [
    ContentIntegrityRepresentationClass::Raw,
    ContentIntegrityRepresentationClass::Rendered,
    ContentIntegrityRepresentationClass::Sanitized,
    ContentIntegrityRepresentationClass::Redacted,
];

/// Copy/export action kind for a representation choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContentIntegrityTransferKind {
    /// Clipboard transfer.
    Copy,
    /// Durable packet, support bundle, or evidence export.
    Export,
}

impl ContentIntegrityTransferKind {
    /// Stable token used in packet rows and reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Export => "export",
        }
    }
}

/// Operator-truth controls that must be visible on a beta surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentIntegrityOperatorTruthControls {
    /// Trust-class or safe-preview badge is visible where the surface renders.
    pub trust_class_badge_visible: bool,
    /// Raw and rendered states are explicitly named when ambiguity can matter.
    pub raw_rendered_state_visible: bool,
    /// Copy and export actions name their representation class.
    pub copy_export_representation_labels_visible: bool,
    /// Review surfaces keep warning refs attached to the reviewed hunk or row.
    pub review_flow_preserves_warning_refs: bool,
    /// Support exports preserve warning refs without silently exporting raw bodies.
    pub support_export_preserves_warning_refs: bool,
}

impl ContentIntegrityOperatorTruthControls {
    /// Returns true when all beta operator-truth controls are present.
    pub const fn is_green(&self) -> bool {
        self.trust_class_badge_visible
            && self.raw_rendered_state_visible
            && self.copy_export_representation_labels_visible
            && self.review_flow_preserves_warning_refs
            && self.support_export_preserves_warning_refs
    }
}

/// Fixture/input representation choice for a beta surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentIntegrityRepresentationChoiceInput {
    /// Stable action id on the originating surface.
    pub action_id: String,
    /// Copy or export action kind.
    pub transfer_kind: ContentIntegrityTransferKind,
    /// Representation class carried by the action.
    pub representation_class: ContentIntegrityRepresentationClass,
    /// Visible label shown by the surface.
    pub visible_label: String,
    /// Whether this is the default action for the surface.
    pub default_for_surface: bool,
    /// Whether the action remains visible in review/handoff flows.
    pub review_flow_visible: bool,
    /// Whether this action is valid for support/export boundaries.
    pub support_export_safe: bool,
    /// Whether redaction is applied before transfer.
    pub redaction_applied: bool,
}

/// Projected representation choice with attached warning refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentIntegrityRepresentationChoice {
    /// Stable action id on the originating surface.
    pub action_id: String,
    /// Copy or export action kind token.
    pub transfer_kind: String,
    /// Representation class token.
    pub representation_class: String,
    /// Visible label shown by the surface.
    pub visible_label: String,
    /// Whether this is the default action for the surface.
    pub default_for_surface: bool,
    /// Whether the action remains visible in review/handoff flows.
    pub review_flow_visible: bool,
    /// Whether this action is valid for support/export boundaries.
    pub support_export_safe: bool,
    /// Whether redaction is applied before transfer.
    pub redaction_applied: bool,
    /// Warning continuity refs preserved by this transfer.
    pub attached_warning_refs: Vec<String>,
}

/// Fixture/input surface row for [`ContentIntegrityBetaCase`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentIntegrityBetaSurfaceInput {
    /// Declared beta surface kind.
    pub surface_kind: ContentIntegrityBetaSurfaceKind,
    /// Stable projection ref for the surface.
    pub surface_ref: String,
    /// Opaque subject, row, hunk, packet, or support ref.
    pub subject_ref: String,
    /// Claimed beta surface row this projection protects.
    pub declared_beta_surface_ref: String,
    /// Trust class visible on the surface.
    pub trust_class: TrustClass,
    /// Operator-truth controls visible on this surface.
    pub operator_truth: ContentIntegrityOperatorTruthControls,
    /// Representation choices offered by copy/export/review flows.
    pub representation_choices: Vec<ContentIntegrityRepresentationChoiceInput>,
}

/// Case input consumed by fixtures and the CLI validation hook.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentIntegrityBetaCase {
    /// Stable case id.
    pub case_id: String,
    /// UTF-8 content inspected once by the shared detector.
    pub content: String,
    /// Claimed-surface register linked to beta gating.
    pub claim_register_ref: String,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Packet mint timestamp.
    pub minted_at: String,
    /// Declared beta surface rows.
    pub surfaces: Vec<ContentIntegrityBetaSurfaceInput>,
}

/// Projected beta surface row with shared detector warnings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentIntegrityBetaSurfaceRow {
    /// Declared beta surface kind.
    pub surface_kind: ContentIntegrityBetaSurfaceKind,
    /// Stable surface token.
    pub surface_token: String,
    /// Stable projection ref for the surface.
    pub surface_ref: String,
    /// Opaque subject, row, hunk, packet, or support ref.
    pub subject_ref: String,
    /// Claimed beta surface row this projection protects.
    pub declared_beta_surface_ref: String,
    /// Trust class visible on the surface.
    pub trust_class: String,
    /// Shared detector outcome token.
    pub detector_outcome_token: String,
    /// Shared suspicious-content class tokens found by the detector.
    pub warning_class_tokens: Vec<String>,
    /// Shared warning records attached to this surface.
    pub content_integrity_warnings: Vec<ContentIntegrityWarningRecord>,
    /// Representation choices with warning refs attached.
    pub representation_choices: Vec<ContentIntegrityRepresentationChoice>,
    /// Operator-truth controls visible on this surface.
    pub operator_truth: ContentIntegrityOperatorTruthControls,
}

impl ContentIntegrityBetaSurfaceRow {
    /// Returns the representation class tokens offered by this surface.
    pub fn representation_tokens(&self) -> BTreeSet<&str> {
        self.representation_choices
            .iter()
            .map(|choice| choice.representation_class.as_str())
            .collect()
    }

    /// Returns true when this surface offers the full beta representation vocabulary.
    pub fn offers_required_representations(&self) -> bool {
        let actual = self.representation_tokens();
        CONTENT_INTEGRITY_REQUIRED_REPRESENTATIONS
            .iter()
            .all(|representation| actual.contains(representation.as_str()))
    }
}

/// Beta packet proving shared content-integrity posture across declared surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentIntegrityBetaPacket {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Stable case id.
    pub case_id: String,
    /// Claimed-surface register linked to beta gating.
    pub claim_register_ref: String,
    /// Source contracts consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Shared detector outcome token.
    pub detector_outcome_token: String,
    /// Shared suspicious-content class tokens found by the detector.
    pub finding_class_tokens: Vec<String>,
    /// Number of bytes inspected by the detector.
    pub source_len_bytes: usize,
    /// Whether projection normalized or stripped bytes.
    pub normalization_applied: bool,
    /// Declared beta surface projections.
    pub surfaces: Vec<ContentIntegrityBetaSurfaceRow>,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ContentIntegrityBetaPacket {
    /// Builds a beta content-integrity packet from a fixture or first consumer.
    pub fn from_case(case: ContentIntegrityBetaCase) -> Self {
        let detection = detect_suspicious_content(&case.content);
        let finding_class_tokens = distinct_finding_class_tokens(&detection);
        let case_id = case.case_id.clone();
        let surfaces = case
            .surfaces
            .into_iter()
            .map(|surface| {
                project_surface(
                    &case_id,
                    surface,
                    &case.content,
                    &detection,
                    &finding_class_tokens,
                )
            })
            .collect();

        Self {
            record_kind: CONTENT_INTEGRITY_BETA_PACKET_RECORD_KIND.to_owned(),
            schema_version: CONTENT_INTEGRITY_BETA_SCHEMA_VERSION,
            case_id: case.case_id,
            claim_register_ref: case.claim_register_ref,
            source_contract_refs: case.source_contract_refs,
            detector_outcome_token: detection.outcome.as_str().to_owned(),
            finding_class_tokens,
            source_len_bytes: case.content.len(),
            normalization_applied: false,
            surfaces,
            minted_at: case.minted_at,
        }
    }

    /// Validates the packet and returns a green-or-blocked report.
    pub fn validate(&self) -> ContentIntegrityBetaValidationReport {
        let mut violations = Vec::new();

        validate_identity(self, &mut violations);
        validate_contracts(self, &mut violations);
        validate_surface_coverage(self, &mut violations);
        for surface in &self.surfaces {
            validate_surface(self, surface, &mut violations);
        }

        let status = if violations.is_empty() {
            ContentIntegrityBetaGateStatus::Green
        } else {
            ContentIntegrityBetaGateStatus::Blocked
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

        ContentIntegrityBetaValidationReport {
            record_kind: CONTENT_INTEGRITY_BETA_VALIDATION_REPORT_RECORD_KIND.to_owned(),
            schema_version: CONTENT_INTEGRITY_BETA_SCHEMA_VERSION,
            case_id: self.case_id.clone(),
            status: status.as_str().to_owned(),
            violations,
            validated_surface_count: self.surfaces.len(),
            observed_surface_tokens: observed_surface_tokens(&self.surfaces),
            observed_warning_class_tokens: self.finding_class_tokens.clone(),
            observed_representation_tokens: observed_representation_tokens(&self.surfaces),
            blocked_beta_surface_refs,
        }
    }

    /// Returns true when validation reports a green beta gate.
    pub fn beta_gate_is_green(&self) -> bool {
        self.validate().status == ContentIntegrityBetaGateStatus::Green.as_str()
    }

    /// Deterministic export-safe JSON for the packet.
    ///
    /// # Panics
    ///
    /// Panics only if serializing the packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("content-integrity beta packet serializes")
    }
}

/// Gate status for beta content-integrity validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ContentIntegrityBetaGateStatus {
    /// Packet is promotable for declared beta surfaces.
    Green,
    /// Packet blocks beta promotion.
    Blocked,
}

impl ContentIntegrityBetaGateStatus {
    /// Stable token used in reports and admission hooks.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Blocked => "blocked",
        }
    }
}

/// Validation report emitted by [`ContentIntegrityBetaPacket::validate`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentIntegrityBetaValidationReport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Integer schema version.
    pub schema_version: u32,
    /// Case id validated by this report.
    pub case_id: String,
    /// `green` when no violations were found, otherwise `blocked`.
    pub status: String,
    /// Violations found during validation.
    pub violations: Vec<ContentIntegrityBetaViolation>,
    /// Number of surface rows validated.
    pub validated_surface_count: usize,
    /// Surface tokens observed in the packet.
    pub observed_surface_tokens: Vec<String>,
    /// Shared suspicious-content class tokens observed in the packet.
    pub observed_warning_class_tokens: Vec<String>,
    /// Representation tokens observed across copy/export choices.
    pub observed_representation_tokens: Vec<String>,
    /// Claimed beta surface refs blocked by a non-green packet.
    pub blocked_beta_surface_refs: Vec<String>,
}

impl ContentIntegrityBetaValidationReport {
    /// Returns true when the report has green status and no violations.
    pub fn is_green(&self) -> bool {
        self.status == ContentIntegrityBetaGateStatus::Green.as_str() && self.violations.is_empty()
    }
}

/// One validation violation surfaced by the beta content-integrity gate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ContentIntegrityBetaViolation {
    /// Stable violation id.
    pub violation_id: String,
    /// Surface token, if the violation is surface-specific.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub surface: Option<String>,
    /// Reviewable summary.
    pub summary: String,
}

impl ContentIntegrityBetaViolation {
    fn new(violation_id: impl Into<String>, summary: impl Into<String>) -> Self {
        Self {
            violation_id: violation_id.into(),
            surface: None,
            summary: summary.into(),
        }
    }

    fn for_surface(
        surface: &ContentIntegrityBetaSurfaceRow,
        violation_id: impl Into<String>,
        summary: impl Into<String>,
    ) -> Self {
        Self {
            violation_id: violation_id.into(),
            surface: Some(surface.surface_token.clone()),
            summary: summary.into(),
        }
    }
}

/// Errors emitted when reading the checked-in beta content-integrity packet.
#[derive(Debug)]
pub enum ContentIntegrityBetaArtifactError {
    /// Packet failed to parse.
    Packet(serde_json::Error),
    /// Packet failed validation.
    Validation(ContentIntegrityBetaValidationReport),
}

impl fmt::Display for ContentIntegrityBetaArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Packet(error) => {
                write!(formatter, "content-integrity packet parse failed: {error}")
            }
            Self::Validation(report) => write!(
                formatter,
                "content-integrity packet validation failed with status {}",
                report.status
            ),
        }
    }
}

impl Error for ContentIntegrityBetaArtifactError {}

/// Returns the checked-in beta content-integrity packet.
///
/// # Errors
///
/// Returns an artifact error if the checked-in packet does not parse or is not green.
pub fn current_content_integrity_beta_packet(
) -> Result<ContentIntegrityBetaPacket, ContentIntegrityBetaArtifactError> {
    let packet: ContentIntegrityBetaPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/security/m3/content_integrity_beta_packet.json"
    )))
    .map_err(ContentIntegrityBetaArtifactError::Packet)?;
    let report = packet.validate();
    if report.is_green() {
        Ok(packet)
    } else {
        Err(ContentIntegrityBetaArtifactError::Validation(report))
    }
}

fn project_surface(
    case_id: &str,
    surface: ContentIntegrityBetaSurfaceInput,
    content: &str,
    detection: &SuspiciousContentDetection,
    finding_class_tokens: &[String],
) -> ContentIntegrityBetaSurfaceRow {
    let warning_surface_kind = surface.surface_kind.warning_surface_kind();
    let warnings = project_content_integrity_warnings_from_detection(
        case_id,
        warning_surface_kind,
        &surface.subject_ref,
        detection,
        Some(content),
    );
    let warning_refs = warnings
        .iter()
        .map(|warning| warning.continuity_ref.clone())
        .collect::<Vec<_>>();
    let representation_choices = surface
        .representation_choices
        .into_iter()
        .map(|choice| ContentIntegrityRepresentationChoice {
            action_id: choice.action_id,
            transfer_kind: choice.transfer_kind.as_str().to_owned(),
            representation_class: choice.representation_class.as_str().to_owned(),
            visible_label: choice.visible_label,
            default_for_surface: choice.default_for_surface,
            review_flow_visible: choice.review_flow_visible,
            support_export_safe: choice.support_export_safe,
            redaction_applied: choice.redaction_applied,
            attached_warning_refs: warning_refs.clone(),
        })
        .collect();

    ContentIntegrityBetaSurfaceRow {
        surface_kind: surface.surface_kind,
        surface_token: surface.surface_kind.as_str().to_owned(),
        surface_ref: surface.surface_ref,
        subject_ref: surface.subject_ref,
        declared_beta_surface_ref: surface.declared_beta_surface_ref,
        trust_class: surface.trust_class.as_str().to_owned(),
        detector_outcome_token: detection.outcome.as_str().to_owned(),
        warning_class_tokens: finding_class_tokens.to_vec(),
        content_integrity_warnings: warnings,
        representation_choices,
        operator_truth: surface.operator_truth,
    }
}

fn distinct_finding_class_tokens(detection: &SuspiciousContentDetection) -> Vec<String> {
    detection
        .findings
        .iter()
        .map(|finding| finding.class.as_str().to_owned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn validate_identity(
    packet: &ContentIntegrityBetaPacket,
    violations: &mut Vec<ContentIntegrityBetaViolation>,
) {
    if packet.record_kind != CONTENT_INTEGRITY_BETA_PACKET_RECORD_KIND {
        violations.push(ContentIntegrityBetaViolation::new(
            "wrong_record_kind",
            "packet record_kind is not content_integrity_beta_packet",
        ));
    }
    if packet.schema_version != CONTENT_INTEGRITY_BETA_SCHEMA_VERSION {
        violations.push(ContentIntegrityBetaViolation::new(
            "wrong_schema_version",
            "packet schema_version is not supported",
        ));
    }
    if packet.case_id.trim().is_empty()
        || packet.claim_register_ref.trim().is_empty()
        || packet.minted_at.trim().is_empty()
    {
        violations.push(ContentIntegrityBetaViolation::new(
            "missing_identity",
            "packet identity, claim register, or mint timestamp is missing",
        ));
    }
    if packet.normalization_applied {
        violations.push(ContentIntegrityBetaViolation::new(
            "normalization_applied",
            "content-integrity projection normalized or stripped source bytes",
        ));
    }
    if packet.finding_class_tokens.is_empty() {
        violations.push(ContentIntegrityBetaViolation::new(
            "missing_detector_findings",
            "beta packet must prove at least one suspicious detector finding",
        ));
    }
}

fn validate_contracts(
    packet: &ContentIntegrityBetaPacket,
    violations: &mut Vec<ContentIntegrityBetaViolation>,
) {
    for required in [
        CONTENT_INTEGRITY_BETA_DOC_REF,
        SUSPICIOUS_CONTENT_PACKET_DOC_REF,
        SUSPICIOUS_TEXT_ALPHA_DOC_REF,
        REPRESENTATION_COPY_EXPORT_DOC_REF,
    ] {
        if !packet
            .source_contract_refs
            .iter()
            .any(|reference| reference == required)
        {
            violations.push(ContentIntegrityBetaViolation::new(
                "missing_source_contract_ref",
                format!("packet is missing source contract ref {required}"),
            ));
        }
    }
}

fn validate_surface_coverage(
    packet: &ContentIntegrityBetaPacket,
    violations: &mut Vec<ContentIntegrityBetaViolation>,
) {
    let mut counts = BTreeMap::<&str, usize>::new();
    for surface in &packet.surfaces {
        *counts.entry(surface.surface_token.as_str()).or_default() += 1;
    }
    for required in CONTENT_INTEGRITY_BETA_REQUIRED_SURFACES {
        if !counts.contains_key(required.as_str()) {
            violations.push(ContentIntegrityBetaViolation::new(
                "missing_required_surface",
                format!("packet is missing required surface {}", required.as_str()),
            ));
        }
    }
    for (surface, count) in counts {
        if count > 1 {
            violations.push(ContentIntegrityBetaViolation::new(
                "duplicate_surface",
                format!("packet contains duplicate surface {surface}"),
            ));
        }
    }
}

fn validate_surface(
    packet: &ContentIntegrityBetaPacket,
    surface: &ContentIntegrityBetaSurfaceRow,
    violations: &mut Vec<ContentIntegrityBetaViolation>,
) {
    if surface.surface_ref.trim().is_empty()
        || surface.subject_ref.trim().is_empty()
        || surface.declared_beta_surface_ref.trim().is_empty()
        || surface.trust_class.trim().is_empty()
    {
        violations.push(ContentIntegrityBetaViolation::for_surface(
            surface,
            "surface_identity_missing",
            "surface identity, subject, claim ref, or trust class is missing",
        ));
    }

    if surface.detector_outcome_token != packet.detector_outcome_token {
        violations.push(ContentIntegrityBetaViolation::for_surface(
            surface,
            "detector_outcome_drift",
            "surface detector outcome differs from the packet outcome",
        ));
    }
    if surface.warning_class_tokens != packet.finding_class_tokens {
        violations.push(ContentIntegrityBetaViolation::for_surface(
            surface,
            "warning_class_drift",
            "surface warning classes differ from the shared detector classes",
        ));
    }
    if surface.content_integrity_warnings.is_empty() {
        violations.push(ContentIntegrityBetaViolation::for_surface(
            surface,
            "missing_content_integrity_warnings",
            "surface emitted no shared content-integrity warnings",
        ));
    }
    for warning in &surface.content_integrity_warnings {
        if warning.record_kind != CONTENT_INTEGRITY_WARNING_RECORD_KIND
            || warning.surface_token != surface.surface_kind.warning_surface_kind().as_str()
        {
            violations.push(ContentIntegrityBetaViolation::for_surface(
                surface,
                "warning_record_shape_drift",
                "surface warning record kind or surface token drifted",
            ));
            break;
        }
    }

    if !surface.operator_truth.is_green() {
        violations.push(ContentIntegrityBetaViolation::for_surface(
            surface,
            "operator_truth_not_green",
            "surface does not preserve trust class, raw/rendered state, labels, review refs, and support refs",
        ));
    }
    validate_representation_choices(surface, violations);
}

fn validate_representation_choices(
    surface: &ContentIntegrityBetaSurfaceRow,
    violations: &mut Vec<ContentIntegrityBetaViolation>,
) {
    if !surface.offers_required_representations() {
        violations.push(ContentIntegrityBetaViolation::for_surface(
            surface,
            "missing_required_representation",
            "surface must expose raw, rendered, sanitized, and redacted representation labels",
        ));
    }

    let default_count = surface
        .representation_choices
        .iter()
        .filter(|choice| choice.default_for_surface)
        .count();
    if default_count != 1 {
        violations.push(ContentIntegrityBetaViolation::for_surface(
            surface,
            "default_representation_count_invalid",
            "surface must declare exactly one default transfer action",
        ));
    }

    let warning_refs = surface
        .content_integrity_warnings
        .iter()
        .map(|warning| warning.continuity_ref.as_str())
        .collect::<BTreeSet<_>>();
    for choice in &surface.representation_choices {
        if choice.action_id.trim().is_empty() || choice.visible_label.trim().is_empty() {
            violations.push(ContentIntegrityBetaViolation::for_surface(
                surface,
                "representation_choice_missing_label",
                "representation choice action id or visible label is missing",
            ));
            break;
        }
        if !choice.review_flow_visible {
            violations.push(ContentIntegrityBetaViolation::for_surface(
                surface,
                "representation_not_review_visible",
                "representation choice is not visible in review or handoff flows",
            ));
            break;
        }
        if warning_refs.iter().any(|reference| {
            !choice
                .attached_warning_refs
                .iter()
                .any(|attached| attached == reference)
        }) {
            violations.push(ContentIntegrityBetaViolation::for_surface(
                surface,
                "representation_drops_warning_ref",
                "copy/export representation does not preserve every warning ref",
            ));
            break;
        }
        if choice.representation_class == ContentIntegrityRepresentationClass::Redacted.as_str()
            && !choice.redaction_applied
        {
            violations.push(ContentIntegrityBetaViolation::for_surface(
                surface,
                "redacted_representation_without_redaction",
                "redacted representation is declared without an applied redaction flag",
            ));
            break;
        }
        if matches!(
            choice.representation_class.as_str(),
            "sanitized" | "redacted"
        ) && !choice.support_export_safe
        {
            violations.push(ContentIntegrityBetaViolation::for_surface(
                surface,
                "support_export_representation_not_safe",
                "sanitized and redacted representations must be support-export safe",
            ));
            break;
        }
    }
}

fn observed_surface_tokens(surfaces: &[ContentIntegrityBetaSurfaceRow]) -> Vec<String> {
    surfaces
        .iter()
        .map(|surface| surface.surface_token.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn observed_representation_tokens(surfaces: &[ContentIntegrityBetaSurfaceRow]) -> Vec<String> {
    surfaces
        .iter()
        .flat_map(|surface| {
            surface
                .representation_choices
                .iter()
                .map(|choice| choice.representation_class.clone())
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn operator_truth() -> ContentIntegrityOperatorTruthControls {
        ContentIntegrityOperatorTruthControls {
            trust_class_badge_visible: true,
            raw_rendered_state_visible: true,
            copy_export_representation_labels_visible: true,
            review_flow_preserves_warning_refs: true,
            support_export_preserves_warning_refs: true,
        }
    }

    fn choices() -> Vec<ContentIntegrityRepresentationChoiceInput> {
        vec![
            ContentIntegrityRepresentationChoiceInput {
                action_id: "copy_raw".to_owned(),
                transfer_kind: ContentIntegrityTransferKind::Copy,
                representation_class: ContentIntegrityRepresentationClass::Raw,
                visible_label: "Copy raw source".to_owned(),
                default_for_surface: true,
                review_flow_visible: true,
                support_export_safe: false,
                redaction_applied: false,
            },
            ContentIntegrityRepresentationChoiceInput {
                action_id: "copy_rendered".to_owned(),
                transfer_kind: ContentIntegrityTransferKind::Copy,
                representation_class: ContentIntegrityRepresentationClass::Rendered,
                visible_label: "Copy rendered view".to_owned(),
                default_for_surface: false,
                review_flow_visible: true,
                support_export_safe: false,
                redaction_applied: false,
            },
            ContentIntegrityRepresentationChoiceInput {
                action_id: "export_sanitized_snapshot".to_owned(),
                transfer_kind: ContentIntegrityTransferKind::Export,
                representation_class: ContentIntegrityRepresentationClass::Sanitized,
                visible_label: "Export sanitized snapshot".to_owned(),
                default_for_surface: false,
                review_flow_visible: true,
                support_export_safe: true,
                redaction_applied: false,
            },
            ContentIntegrityRepresentationChoiceInput {
                action_id: "export_redacted_evidence".to_owned(),
                transfer_kind: ContentIntegrityTransferKind::Export,
                representation_class: ContentIntegrityRepresentationClass::Redacted,
                visible_label: "Export redacted evidence".to_owned(),
                default_for_surface: false,
                review_flow_visible: true,
                support_export_safe: true,
                redaction_applied: true,
            },
        ]
    }

    fn case() -> ContentIntegrityBetaCase {
        ContentIntegrityBetaCase {
            case_id: "case:content-integrity:beta:test".to_owned(),
            content: "const p\u{0430}yload = user\u{202E}name\u{200D};".to_owned(),
            claim_register_ref: "artifacts/milestones/m3/claimed_surface_register.json".to_owned(),
            source_contract_refs: vec![
                CONTENT_INTEGRITY_BETA_DOC_REF.to_owned(),
                SUSPICIOUS_CONTENT_PACKET_DOC_REF.to_owned(),
                SUSPICIOUS_TEXT_ALPHA_DOC_REF.to_owned(),
                REPRESENTATION_COPY_EXPORT_DOC_REF.to_owned(),
            ],
            minted_at: "2026-05-17T00:00:00Z".to_owned(),
            surfaces: CONTENT_INTEGRITY_BETA_REQUIRED_SURFACES
                .iter()
                .map(|surface| ContentIntegrityBetaSurfaceInput {
                    surface_kind: *surface,
                    surface_ref: format!("surface:{}", surface.as_str()),
                    subject_ref: format!("subject:{}", surface.as_str()),
                    declared_beta_surface_ref: "beta_surface:support_export_diagnostics".to_owned(),
                    trust_class: TrustClass::RawText,
                    operator_truth: operator_truth(),
                    representation_choices: choices(),
                })
                .collect(),
        }
    }

    #[test]
    fn validates_green_beta_packet() {
        let packet = ContentIntegrityBetaPacket::from_case(case());
        let report = packet.validate();

        assert!(report.is_green(), "{report:#?}");
        assert_eq!(
            report.observed_warning_class_tokens,
            vec![
                "bidi_control".to_owned(),
                "invisible_formatting".to_owned(),
                "mixed_script_confusable".to_owned(),
            ]
        );
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
    fn blocks_packet_when_redacted_representation_is_missing() {
        let mut case = case();
        case.surfaces[0].representation_choices.retain(|choice| {
            choice.representation_class != ContentIntegrityRepresentationClass::Redacted
        });
        let report = ContentIntegrityBetaPacket::from_case(case).validate();

        assert_eq!(report.status, "blocked");
        assert!(report
            .violations
            .iter()
            .any(|violation| { violation.violation_id == "missing_required_representation" }));
    }
}
