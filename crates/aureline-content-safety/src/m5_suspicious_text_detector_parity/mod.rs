//! Shared suspicious-text detector parity across the new M5 viewer surfaces.
//!
//! The detector in [`crate::detector`] owns the byte-level suspicious-content
//! findings, and [`crate::suspicious_text`] projects them across the core
//! source surfaces (editor, diff, search, review). This module extends that
//! same shared detector onto the *new* M5 artifact and viewer surfaces that
//! would otherwise be free to disagree about suspicious text: notebook output,
//! docs/browser panels, marketplace install/update, remote-host attach,
//! collaboration share, AI evidence viewers, and provider/policy overlays.
//!
//! It binds three things together so no surface is safer than another:
//!
//! - **One detector, one threat-class vocabulary.** Every surface projects the
//!   same [`detect_suspicious_content`] run through the same
//!   [`SuspiciousTextThreatClass`] cues, so a bidi control, an invisible
//!   codepoint, or a mixed-script identifier reads identically on a notebook
//!   cell, a marketplace manifest, and a provider overlay.
//! - **Raw inspection stays reachable.** Whenever a threat-class cue materially
//!   affects trust or action safety, the surface keeps a raw-inspection path
//!   and a distinctly labeled raw-versus-escaped copy/export affordance; bytes
//!   are never normalized away and rendered copy never masquerades as raw.
//! - **Strong-decision surfaces render stricter identity.** Install/update,
//!   attach/share, collaboration, and policy-review surfaces use the
//!   strong-decision strict display mode rather than ordinary browsing chrome.
//!
//! The packet also carries a [`M5SuspiciousTextSupportAdminExport`] block so a
//! support or admin reviewer can preserve the same threat-class cues without
//! reproducing the warning in a specific pane. That export carries escaped
//! exemplars and continuity refs only — never raw suspicious bytes.
//!
//! The boundary schema is
//! [`schemas/security/m5-suspicious-text-detector-parity.schema.json`](../../../../schemas/security/m5-suspicious-text-detector-parity.schema.json).
//! The contract doc is
//! [`docs/security/m5/m5_suspicious_text_detector_parity.md`](../../../../docs/security/m5/m5_suspicious_text_detector_parity.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::{
    detect_suspicious_content, has_suspicious_content, BodyPosture, DetectorOutcomeClass,
    RepresentationActionId, RepresentationClass, SuspiciousContentClass, SuspiciousFinding,
    SuspiciousTextWarningAction,
};

/// Stable record-kind tag carried by [`M5SuspiciousTextParityPacket`].
pub const M5_SUSPICIOUS_TEXT_PARITY_RECORD_KIND: &str = "m5_suspicious_text_detector_parity_packet";

/// Stable record-kind tag carried by [`M5SuspiciousTextSupportAdminExport`].
pub const M5_SUSPICIOUS_TEXT_SUPPORT_ADMIN_RECORD_KIND: &str =
    "m5_suspicious_text_support_admin_export";

/// Schema version for the M5 suspicious-text parity packet.
pub const M5_SUSPICIOUS_TEXT_PARITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_SUSPICIOUS_TEXT_PARITY_SCHEMA_REF: &str =
    "schemas/security/m5-suspicious-text-detector-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SUSPICIOUS_TEXT_PARITY_DOC_REF: &str =
    "docs/security/m5/m5_suspicious_text_detector_parity.md";

/// Repo-relative path of the checked-in support-export artifact.
pub const M5_SUSPICIOUS_TEXT_PARITY_ARTIFACT_REF: &str =
    "artifacts/security/m5/m5_suspicious_text_detector_parity/support_export.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SUSPICIOUS_TEXT_PARITY_FIXTURE_DIR: &str =
    "fixtures/security/m5/m5_suspicious_text_detector_parity";

/// Stable packet id minted by [`frozen_m5_suspicious_text_parity_packet`].
pub const M5_SUSPICIOUS_TEXT_PARITY_PACKET_ID: &str = "m5-suspicious-text-parity:stable:0001";

/// New M5 viewer/decision surface that must share the suspicious-text detector.
///
/// These are the surfaces named by this lane that did not previously share the
/// core source-surface projection (editor, diff, search, review).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuspiciousTextSurface {
    /// Notebook rich-output cell or block.
    NotebookOutput,
    /// Docs page or in-product browser panel.
    DocsBrowserPanel,
    /// Marketplace install / update surface (strong decision).
    MarketplaceInstallUpdate,
    /// Remote-host attach / connect surface (strong decision).
    RemoteHostAttach,
    /// Collaboration share / invite surface (strong decision).
    CollaborationShare,
    /// AI evidence or finding-card viewer.
    AiEvidenceViewer,
    /// Provider account / policy overlay (strong decision).
    ProviderPolicyOverlay,
}

impl M5SuspiciousTextSurface {
    /// Every surface this lane must cover, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::NotebookOutput,
        Self::DocsBrowserPanel,
        Self::MarketplaceInstallUpdate,
        Self::RemoteHostAttach,
        Self::CollaborationShare,
        Self::AiEvidenceViewer,
        Self::ProviderPolicyOverlay,
    ];

    /// Stable token recorded in packets, fixtures, and CLI output.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookOutput => "notebook_output",
            Self::DocsBrowserPanel => "docs_browser_panel",
            Self::MarketplaceInstallUpdate => "marketplace_install_update",
            Self::RemoteHostAttach => "remote_host_attach",
            Self::CollaborationShare => "collaboration_share",
            Self::AiEvidenceViewer => "ai_evidence_viewer",
            Self::ProviderPolicyOverlay => "provider_policy_overlay",
        }
    }

    /// Human-readable noun used in copy/export action labels.
    pub const fn label_noun(self) -> &'static str {
        match self {
            Self::NotebookOutput => "cell output",
            Self::DocsBrowserPanel => "page content",
            Self::MarketplaceInstallUpdate => "manifest",
            Self::RemoteHostAttach => "host identity",
            Self::CollaborationShare => "shared content",
            Self::AiEvidenceViewer => "evidence",
            Self::ProviderPolicyOverlay => "policy text",
        }
    }

    /// Whether this is a strong-decision surface (install/update, attach/share,
    /// collaboration, or policy review) that must render owner and origin
    /// identity more strictly than an ordinary browsing pane.
    pub const fn is_strong_decision_surface(self) -> bool {
        matches!(
            self,
            Self::MarketplaceInstallUpdate
                | Self::RemoteHostAttach
                | Self::CollaborationShare
                | Self::ProviderPolicyOverlay
        )
    }

    /// Display mode this surface must use.
    pub const fn display_mode(self) -> M5SuspiciousTextDisplayMode {
        if self.is_strong_decision_surface() {
            M5SuspiciousTextDisplayMode::StrongDecisionStrictIdentity
        } else {
            M5SuspiciousTextDisplayMode::OrdinaryBrowsing
        }
    }
}

/// Decision-strictness display mode for a surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuspiciousTextDisplayMode {
    /// Ordinary browsing-pane identity rendering.
    OrdinaryBrowsing,
    /// Stricter owner/origin identity rendering for strong-decision surfaces.
    StrongDecisionStrictIdentity,
}

impl M5SuspiciousTextDisplayMode {
    /// Stable token recorded in packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryBrowsing => "ordinary_browsing",
            Self::StrongDecisionStrictIdentity => "strong_decision_strict_identity",
        }
    }

    /// Whether this is the stricter strong-decision display mode.
    pub const fn is_strict(self) -> bool {
        matches!(self, Self::StrongDecisionStrictIdentity)
    }
}

/// Shared threat-class vocabulary that every M5 surface maps detector findings
/// to, so the cue reads identically regardless of pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuspiciousTextThreatClass {
    /// Glyphs can display out of source order (bidi controls).
    TextReorderingSpoof,
    /// Hidden codepoints smuggled between visible glyphs (invisible formatting).
    HiddenCodepointSmuggling,
    /// An identifier impersonates another via confusable scripts.
    IdentityConfusableSpoof,
    /// Rendered output diverges from the raw source bytes.
    RenderedSourceDivergence,
}

impl M5SuspiciousTextThreatClass {
    /// Stable token recorded in packets, fixtures, and CLI output.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TextReorderingSpoof => "text_reordering_spoof",
            Self::HiddenCodepointSmuggling => "hidden_codepoint_smuggling",
            Self::IdentityConfusableSpoof => "identity_confusable_spoof",
            Self::RenderedSourceDivergence => "rendered_source_divergence",
        }
    }

    /// Maps a shared suspicious-content class onto its threat class.
    pub const fn from_content_class(class: SuspiciousContentClass) -> Self {
        match class {
            SuspiciousContentClass::BidiControl => Self::TextReorderingSpoof,
            SuspiciousContentClass::InvisibleFormatting => Self::HiddenCodepointSmuggling,
            SuspiciousContentClass::MixedScriptConfusable
            | SuspiciousContentClass::WholeScriptConfusable => Self::IdentityConfusableSpoof,
            SuspiciousContentClass::RawRenderedDivergence => Self::RenderedSourceDivergence,
        }
    }

    /// Short user-facing cue label for chips or review rows.
    pub const fn cue_label(self) -> &'static str {
        match self {
            Self::TextReorderingSpoof => "Text can display out of source order",
            Self::HiddenCodepointSmuggling => "Hidden characters between glyphs",
            Self::IdentityConfusableSpoof => "Identifier can impersonate another",
            Self::RenderedSourceDivergence => "Rendered form differs from source",
        }
    }

    /// Longer cue detail describing why the cue affects trust.
    pub const fn cue_detail(self) -> &'static str {
        match self {
            Self::TextReorderingSpoof => {
                "Bidi control codepoints reorder displayed glyphs without changing the source bytes."
            }
            Self::HiddenCodepointSmuggling => {
                "Invisible or zero-width codepoints hide bytes between visible glyphs."
            }
            Self::IdentityConfusableSpoof => {
                "Letters from more than one script make an identifier look like a different name."
            }
            Self::RenderedSourceDivergence => {
                "The rendered representation does not match the raw source bytes."
            }
        }
    }

    /// Fixed severity for this threat class.
    pub const fn severity(self) -> M5SuspiciousTextThreatSeverity {
        match self {
            Self::IdentityConfusableSpoof => M5SuspiciousTextThreatSeverity::Critical,
            Self::TextReorderingSpoof | Self::HiddenCodepointSmuggling => {
                M5SuspiciousTextThreatSeverity::High
            }
            Self::RenderedSourceDivergence => M5SuspiciousTextThreatSeverity::Elevated,
        }
    }

    /// Whether this threat class materially affects trust or action safety.
    ///
    /// Every threat class in this vocabulary materially affects trust, which is
    /// why raw inspection must stay reachable wherever any cue is shown.
    pub const fn materially_affects_trust(self) -> bool {
        true
    }
}

/// Fixed severity ranking for a threat class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SuspiciousTextThreatSeverity {
    /// Highest concern: identity impersonation on a decision surface.
    Critical,
    /// High concern: reordering or hidden codepoints.
    High,
    /// Elevated concern: raw/rendered divergence.
    Elevated,
}

impl M5SuspiciousTextThreatSeverity {
    /// Stable token recorded in packets.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Critical => "critical",
            Self::High => "high",
            Self::Elevated => "elevated",
        }
    }
}

/// Threat-class cue attached to one warning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuspiciousTextThreatCue {
    /// Shared threat class.
    pub threat_class: M5SuspiciousTextThreatClass,
    /// Stable threat-class token for compact consumers.
    pub threat_class_token: String,
    /// Short user-facing cue label.
    pub cue_label: String,
    /// Longer cue detail.
    pub cue_detail: String,
    /// Fixed severity for the cue.
    pub severity: M5SuspiciousTextThreatSeverity,
    /// Whether the cue materially affects trust or action safety.
    pub materially_affects_trust: bool,
}

impl M5SuspiciousTextThreatCue {
    /// Builds the cue for a suspicious-content class.
    pub fn for_content_class(class: SuspiciousContentClass) -> Self {
        let threat_class = M5SuspiciousTextThreatClass::from_content_class(class);
        Self {
            threat_class,
            threat_class_token: threat_class.as_str().to_owned(),
            cue_label: threat_class.cue_label().to_owned(),
            cue_detail: threat_class.cue_detail().to_owned(),
            severity: threat_class.severity(),
            materially_affects_trust: threat_class.materially_affects_trust(),
        }
    }
}

/// One suspicious-text warning projected onto a concrete M5 surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuspiciousTextWarning {
    /// Surface-stable warning id.
    pub warning_id: String,
    /// Detector finding id this warning projects from.
    pub detector_finding_id: String,
    /// Shared suspicious-content class from the detector.
    pub content_class: SuspiciousContentClass,
    /// Shared threat-class cue for this warning.
    pub threat_cue: M5SuspiciousTextThreatCue,
    /// Shared continuity ref used to join the same finding across surfaces.
    pub continuity_ref: String,
    /// Byte offset of the first suspicious codepoint in the source text.
    pub byte_offset: usize,
    /// Character offset of the first suspicious codepoint in the source text.
    pub char_offset: usize,
    /// Number of Unicode scalar values covered by the warning.
    pub length_chars: usize,
    /// Codepoints observed by the detector for this warning.
    pub matched_codepoints: Vec<u32>,
    /// Exact source snippet for this finding.
    pub raw_snippet: String,
    /// Safe-inspection snippet with risky codepoints escaped.
    pub escaped_snippet: String,
    /// Reveal/copy/export actions that must remain reachable from this warning.
    pub available_actions: Vec<SuspiciousTextWarningAction>,
}

impl M5SuspiciousTextWarning {
    /// True when the warning exposes both raw and escaped reveal actions.
    pub fn offers_raw_and_escaped_reveal(&self) -> bool {
        self.available_actions
            .contains(&SuspiciousTextWarningAction::RevealRawSource)
            && self
                .available_actions
                .contains(&SuspiciousTextWarningAction::RevealEscapedSource)
    }

    /// True when raw inspection stays reachable (a codepoint inspector plus a
    /// raw reveal/copy path) for a cue that materially affects trust.
    pub fn raw_inspection_reachable(&self) -> bool {
        self.offers_raw_and_escaped_reveal()
            && self
                .available_actions
                .contains(&SuspiciousTextWarningAction::InspectCodepoints)
            && self
                .available_actions
                .contains(&SuspiciousTextWarningAction::CopyRaw)
    }
}

/// Representation transfer choice offered by one surface projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuspiciousTextTransferChoice {
    /// Transfer action id drawn from the representation policy vocabulary.
    pub action_id: String,
    /// Representation class drawn from the representation policy vocabulary.
    pub representation_class: String,
    /// Body posture drawn from the representation policy vocabulary.
    pub body_posture: String,
    /// Surface label that names the representation explicitly.
    pub label: String,
    /// True when this action is the safe path for sharing suspicious text.
    pub safe_representation_path: bool,
    /// Warning ids that must travel with the transferred representation.
    pub attached_warning_ids: Vec<String>,
}

/// Per-surface projection of the shared detector run.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuspiciousTextSurfaceProjection {
    /// Surface this projection describes.
    pub surface: M5SuspiciousTextSurface,
    /// Stable surface token for compact consumers.
    pub surface_token: String,
    /// Opaque surface subject ref used by reopen and inspection paths.
    pub subject_ref: String,
    /// Display mode this surface must render in.
    pub display_mode: M5SuspiciousTextDisplayMode,
    /// Warnings attached to exact source locations on this surface.
    pub warnings: Vec<M5SuspiciousTextWarning>,
    /// Copy choices surfaced with explicit raw/escaped representation labels.
    pub copy_choices: Vec<M5SuspiciousTextTransferChoice>,
    /// Whether a raw-inspection path stays reachable on this surface.
    pub raw_inspection_reachable: bool,
}

impl M5SuspiciousTextSurfaceProjection {
    /// Distinct suspicious-content class tokens visible on this surface.
    pub fn content_class_tokens(&self) -> BTreeSet<&'static str> {
        self.warnings
            .iter()
            .map(|warning| warning.content_class.as_str())
            .collect()
    }

    /// Distinct threat-class tokens visible on this surface.
    pub fn threat_class_tokens(&self) -> BTreeSet<&'static str> {
        self.warnings
            .iter()
            .map(|warning| warning.threat_cue.threat_class.as_str())
            .collect()
    }

    /// True when every warning whose cue materially affects trust keeps a raw
    /// inspection path reachable.
    pub fn raw_inspection_reachable_where_required(&self) -> bool {
        self.warnings.iter().all(|warning| {
            !warning.threat_cue.materially_affects_trust || warning.raw_inspection_reachable()
        })
    }

    /// True when copy choices expose a distinctly labeled escaped safe path and
    /// retain every warning id.
    pub fn offers_labeled_raw_and_escaped_copy(&self) -> bool {
        let has_raw = self
            .copy_choices
            .iter()
            .any(|choice| choice.action_id == RepresentationActionId::CopyRaw.as_str());
        let has_escaped = self.copy_choices.iter().any(|choice| {
            choice.action_id == RepresentationActionId::CopyEscaped.as_str()
                && choice.safe_representation_path
        });
        has_raw && has_escaped
    }
}

/// Per-threat-class summary carried in the support/admin export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuspiciousTextThreatCueSummary {
    /// Shared threat class.
    pub threat_class: M5SuspiciousTextThreatClass,
    /// Stable threat-class token.
    pub threat_class_token: String,
    /// Short cue label preserved for the reviewer.
    pub cue_label: String,
    /// Fixed severity for the cue.
    pub severity: M5SuspiciousTextThreatSeverity,
    /// Whether the cue materially affects trust or action safety.
    pub materially_affects_trust: bool,
    /// Number of warnings of this threat class across all surfaces.
    pub warning_count: usize,
    /// Surface tokens on which this threat class appears.
    pub surfaces: Vec<String>,
    /// Escaped exemplar snippet (never raw suspicious bytes).
    pub escaped_exemplar: String,
    /// Continuity refs joined to this threat class.
    pub continuity_refs: Vec<String>,
}

/// Support/admin export that preserves threat-class cues without a pane.
///
/// Carries escaped exemplars and continuity refs only; raw suspicious bytes
/// never cross this boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuspiciousTextSupportAdminExport {
    /// Stable record-kind discriminator.
    pub record_kind: String,
    /// Case id shared with the parity packet.
    pub case_id: String,
    /// Threat-class summaries the reviewer can read without reproducing a pane.
    pub threat_cue_summaries: Vec<M5SuspiciousTextThreatCueSummary>,
    /// Surface tokens covered by this export.
    pub surfaces_covered: Vec<String>,
    /// Whether the export preserves cues without requiring a specific pane.
    pub preserves_cues_without_pane: bool,
    /// Redaction class token.
    pub redaction_class_token: String,
}

/// Inputs needed to project one detector run across the M5 surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct M5SuspiciousTextParitySeed<'a> {
    /// Stable case id shared by all surface projections.
    pub case_id: &'a str,
    /// Source text inspected by the shared detector.
    pub content: &'a str,
    /// Opaque per-surface subject refs, in [`M5SuspiciousTextSurface::ALL`] order.
    pub subject_refs: [&'a str; 7],
    /// Packet mint timestamp (RFC 3339).
    pub minted_at: &'a str,
}

/// Cross-surface parity packet emitted by the M5 suspicious-text projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SuspiciousTextParityPacket {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Integer schema version for this packet.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Case id shared by all projections.
    pub case_id: String,
    /// Detector outcome before surface projection.
    pub detector_outcome: DetectorOutcomeClass,
    /// Distinct suspicious-content class tokens found in the source text.
    pub finding_classes: Vec<String>,
    /// Distinct threat-class tokens found in the source text.
    pub threat_classes: Vec<String>,
    /// Number of bytes inspected by the detector.
    pub source_len_bytes: usize,
    /// Whether projection normalized or stripped the source (always false).
    pub normalization_applied: bool,
    /// Surface projections for every M5 surface.
    pub surfaces: Vec<M5SuspiciousTextSurfaceProjection>,
    /// Support/admin export preserving the threat-class cues.
    pub support_admin_export: M5SuspiciousTextSupportAdminExport,
    /// Source contract refs consumed by this packet.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SuspiciousTextParityPacket {
    /// Returns true when every required M5 surface is present exactly once.
    pub fn covers_all_m5_surfaces(&self) -> bool {
        let present: BTreeSet<_> = self
            .surfaces
            .iter()
            .map(|surface| surface.surface)
            .collect();
        M5SuspiciousTextSurface::ALL
            .iter()
            .all(|surface| present.contains(surface))
            && present.len() == M5SuspiciousTextSurface::ALL.len()
    }

    /// Returns true when every surface exposes the same content-class set.
    pub fn all_surfaces_share_content_classes(&self) -> bool {
        let expected: BTreeSet<_> = self.finding_classes.iter().map(String::as_str).collect();
        self.surfaces
            .iter()
            .all(|surface| surface.content_class_tokens() == expected)
    }

    /// Returns true when every surface exposes the same threat-class set.
    pub fn all_surfaces_share_threat_classes(&self) -> bool {
        let expected: BTreeSet<_> = self.threat_classes.iter().map(String::as_str).collect();
        self.surfaces
            .iter()
            .all(|surface| surface.threat_class_tokens() == expected)
    }

    /// Returns true when raw inspection stays reachable wherever a cue
    /// materially affects trust.
    pub fn raw_inspection_reachable_where_required(&self) -> bool {
        self.surfaces
            .iter()
            .all(M5SuspiciousTextSurfaceProjection::raw_inspection_reachable_where_required)
    }

    /// Returns true when every strong-decision surface uses strict display.
    pub fn strong_decision_surfaces_use_strict_display(&self) -> bool {
        self.surfaces.iter().all(|surface| {
            !surface.surface.is_strong_decision_surface() || surface.display_mode.is_strict()
        })
    }

    /// Validates the parity invariants.
    pub fn validate(&self) -> Vec<M5SuspiciousTextParityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SUSPICIOUS_TEXT_PARITY_RECORD_KIND {
            violations.push(M5SuspiciousTextParityViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SUSPICIOUS_TEXT_PARITY_SCHEMA_VERSION {
            violations.push(M5SuspiciousTextParityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.case_id.trim().is_empty()
            || self.minted_at.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
        {
            violations.push(M5SuspiciousTextParityViolation::MissingIdentity);
        }
        if self.source_contract_refs.is_empty() {
            violations.push(M5SuspiciousTextParityViolation::MissingSourceContracts);
        }
        if self.normalization_applied {
            violations.push(M5SuspiciousTextParityViolation::NormalizationApplied);
        }
        if !self.covers_all_m5_surfaces() {
            violations.push(M5SuspiciousTextParityViolation::SurfaceMissing);
        }
        if !self.all_surfaces_share_content_classes() || !self.all_surfaces_share_threat_classes() {
            violations.push(M5SuspiciousTextParityViolation::SurfacesDisagreeOnClasses);
        }
        if !self.raw_inspection_reachable_where_required() {
            violations.push(M5SuspiciousTextParityViolation::RawInspectionUnreachable);
        }
        if !self.strong_decision_surfaces_use_strict_display() {
            violations.push(M5SuspiciousTextParityViolation::StrongDecisionDisplayTooWeak);
        }
        for surface in &self.surfaces {
            if !surface.warnings.is_empty() && !surface.offers_labeled_raw_and_escaped_copy() {
                violations.push(M5SuspiciousTextParityViolation::CopyChoicesNotLabeled);
                break;
            }
        }

        validate_support_admin_export(self, &mut violations);

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 suspicious-text parity packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Shared Suspicious-Text Detector Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Case: `{}`\n", self.case_id));
        out.push_str(&format!(
            "- Detector outcome: `{}`\n",
            self.detector_outcome_token()
        ));
        out.push_str(&format!(
            "- Finding classes: {}\n",
            self.finding_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Threat classes: {}\n",
            self.threat_classes.join(", ")
        ));
        out.push_str("\n## Surfaces\n\n");
        for surface in &self.surfaces {
            out.push_str(&format!(
                "- **{}** ({}): {} warning(s), raw inspection reachable: {}\n",
                surface.surface.as_str(),
                surface.display_mode.as_str(),
                surface.warnings.len(),
                surface.raw_inspection_reachable
            ));
        }
        out.push_str("\n## Support/admin threat-class cues\n\n");
        for summary in &self.support_admin_export.threat_cue_summaries {
            out.push_str(&format!(
                "- `{}` ({}): {} warning(s) across {} surface(s)\n",
                summary.threat_class_token,
                summary.severity.as_str(),
                summary.warning_count,
                summary.surfaces.len()
            ));
        }
        out
    }

    fn detector_outcome_token(&self) -> &'static str {
        self.detector_outcome.as_str()
    }
}

/// Validation failures emitted by [`M5SuspiciousTextParityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SuspiciousTextParityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are missing.
    MissingSourceContracts,
    /// Projection normalized or stripped the source bytes.
    NormalizationApplied,
    /// A required M5 surface is missing.
    SurfaceMissing,
    /// Surfaces disagree on the shared content/threat-class set.
    SurfacesDisagreeOnClasses,
    /// Raw inspection is unreachable where a cue materially affects trust.
    RawInspectionUnreachable,
    /// A strong-decision surface does not use strict display mode.
    StrongDecisionDisplayTooWeak,
    /// A surface with warnings does not expose labeled raw/escaped copy.
    CopyChoicesNotLabeled,
    /// The support/admin export does not preserve threat-class cues.
    SupportExportDropsCues,
    /// The support/admin export leaks raw suspicious bytes.
    SupportExportLeaksRawBytes,
}

impl M5SuspiciousTextParityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::NormalizationApplied => "normalization_applied",
            Self::SurfaceMissing => "surface_missing",
            Self::SurfacesDisagreeOnClasses => "surfaces_disagree_on_classes",
            Self::RawInspectionUnreachable => "raw_inspection_unreachable",
            Self::StrongDecisionDisplayTooWeak => "strong_decision_display_too_weak",
            Self::CopyChoicesNotLabeled => "copy_choices_not_labeled",
            Self::SupportExportDropsCues => "support_export_drops_cues",
            Self::SupportExportLeaksRawBytes => "support_export_leaks_raw_bytes",
        }
    }
}

/// Projects one shared detector run across every new M5 surface.
pub fn project_m5_suspicious_text_parity(
    seed: &M5SuspiciousTextParitySeed<'_>,
) -> M5SuspiciousTextParityPacket {
    let detection = detect_suspicious_content(seed.content);
    let finding_classes = distinct_tokens(detection.findings.iter().map(|f| f.class.as_str()));
    let threat_classes = distinct_tokens(
        detection
            .findings
            .iter()
            .map(|f| M5SuspiciousTextThreatClass::from_content_class(f.class).as_str()),
    );

    let surfaces: Vec<_> = M5SuspiciousTextSurface::ALL
        .iter()
        .zip(seed.subject_refs.iter())
        .map(|(&surface, &subject_ref)| {
            project_surface(
                seed.case_id,
                surface,
                subject_ref,
                &detection.findings,
                seed.content,
            )
        })
        .collect();

    let support_admin_export =
        build_support_admin_export(seed.case_id, &surfaces, &detection.findings);

    M5SuspiciousTextParityPacket {
        record_kind: M5_SUSPICIOUS_TEXT_PARITY_RECORD_KIND.to_owned(),
        schema_version: M5_SUSPICIOUS_TEXT_PARITY_SCHEMA_VERSION,
        packet_id: M5_SUSPICIOUS_TEXT_PARITY_PACKET_ID.to_owned(),
        case_id: seed.case_id.to_owned(),
        detector_outcome: detection.outcome,
        finding_classes,
        threat_classes,
        source_len_bytes: seed.content.len(),
        normalization_applied: false,
        surfaces,
        support_admin_export,
        source_contract_refs: vec![
            M5_SUSPICIOUS_TEXT_PARITY_SCHEMA_REF.to_owned(),
            M5_SUSPICIOUS_TEXT_PARITY_DOC_REF.to_owned(),
            "schemas/security/trust_class.schema.json".to_owned(),
            "schemas/security/text_representation_policy.schema.json".to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: seed.minted_at.to_owned(),
    }
}

/// Builds the canonical frozen stable M5 suspicious-text parity packet.
///
/// This is the single in-code source of truth for the checked-in support export
/// at [`M5_SUSPICIOUS_TEXT_PARITY_ARTIFACT_REF`]; the bin emits this packet and a
/// test asserts the checked-in artifact deserializes back to it unchanged.
pub fn frozen_m5_suspicious_text_parity_packet() -> M5SuspiciousTextParityPacket {
    project_m5_suspicious_text_parity(&M5SuspiciousTextParitySeed {
        case_id: "case:m5-suspicious-text-parity:stable",
        content: "let p\u{0430}yload = \"ok\";\nlet admin\u{202E} = user\u{200D}name;\n",
        subject_refs: [
            "notebook:cell:demo:out:1",
            "docs:page:demo#install",
            "marketplace:listing:demo@1.2.0",
            "remote:host:demo.example.dev",
            "collab:share:demo:thread:1",
            "ai:evidence:demo:finding:1",
            "provider:overlay:demo:policy",
        ],
        minted_at: "2026-06-10T00:00:00Z",
    })
}

/// Reads and validates the checked-in stable M5 suspicious-text parity export.
pub fn current_m5_suspicious_text_parity_export(
) -> Result<M5SuspiciousTextParityPacket, M5SuspiciousTextParityExportError> {
    let packet: M5SuspiciousTextParityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/security/m5/m5_suspicious_text_detector_parity/support_export.json"
    )))
    .map_err(M5SuspiciousTextParityExportError::Parse)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SuspiciousTextParityExportError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in M5 suspicious-text parity export.
#[derive(Debug)]
pub enum M5SuspiciousTextParityExportError {
    /// Support export failed to parse.
    Parse(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SuspiciousTextParityViolation>),
}

impl std::fmt::Display for M5SuspiciousTextParityExportError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Parse(error) => {
                write!(
                    formatter,
                    "m5 suspicious-text parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 suspicious-text parity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl std::error::Error for M5SuspiciousTextParityExportError {}

fn distinct_tokens<'a, I>(tokens: I) -> Vec<String>
where
    I: IntoIterator<Item = &'a str>,
{
    tokens
        .into_iter()
        .map(str::to_owned)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn project_surface(
    case_id: &str,
    surface: M5SuspiciousTextSurface,
    subject_ref: &str,
    findings: &[SuspiciousFinding],
    content: &str,
) -> M5SuspiciousTextSurfaceProjection {
    let warnings: Vec<_> = findings
        .iter()
        .map(|finding| project_warning(case_id, surface, finding, content))
        .collect();
    let warning_ids: Vec<_> = warnings
        .iter()
        .map(|warning| warning.warning_id.clone())
        .collect();
    let copy_choices = if warnings.is_empty() {
        Vec::new()
    } else {
        vec![
            raw_copy_choice(surface, warning_ids.clone()),
            escaped_copy_choice(surface, warning_ids),
        ]
    };
    let raw_inspection_reachable = warnings
        .iter()
        .all(M5SuspiciousTextWarning::raw_inspection_reachable);

    M5SuspiciousTextSurfaceProjection {
        surface,
        surface_token: surface.as_str().to_owned(),
        subject_ref: subject_ref.to_owned(),
        display_mode: surface.display_mode(),
        warnings,
        copy_choices,
        raw_inspection_reachable,
    }
}

fn project_warning(
    case_id: &str,
    surface: M5SuspiciousTextSurface,
    finding: &SuspiciousFinding,
    content: &str,
) -> M5SuspiciousTextWarning {
    let raw_snippet = snippet_for_finding(content, finding);
    let escaped_snippet = escaped_snippet_for_finding(&raw_snippet, finding.class);
    let detector_id = finding.finding_id.replace(':', "_");

    M5SuspiciousTextWarning {
        warning_id: format!("warning:{}:{detector_id}", surface.as_str()),
        detector_finding_id: finding.finding_id.clone(),
        content_class: finding.class,
        threat_cue: M5SuspiciousTextThreatCue::for_content_class(finding.class),
        continuity_ref: format!(
            "suspicious-text-parity:{case_id}:{}:{}",
            surface.as_str(),
            finding.finding_id
        ),
        byte_offset: finding.byte_offset,
        char_offset: finding.char_offset,
        length_chars: finding.length_chars,
        matched_codepoints: finding.matched_codepoints.clone(),
        raw_snippet,
        escaped_snippet,
        available_actions: vec![
            SuspiciousTextWarningAction::RevealRawSource,
            SuspiciousTextWarningAction::RevealEscapedSource,
            SuspiciousTextWarningAction::InspectCodepoints,
            SuspiciousTextWarningAction::CopyRaw,
            SuspiciousTextWarningAction::CopyEscaped,
            SuspiciousTextWarningAction::ExportSafeRepresentation,
        ],
    }
}

fn raw_copy_choice(
    surface: M5SuspiciousTextSurface,
    attached_warning_ids: Vec<String>,
) -> M5SuspiciousTextTransferChoice {
    M5SuspiciousTextTransferChoice {
        action_id: RepresentationActionId::CopyRaw.as_str().to_owned(),
        representation_class: RepresentationClass::Raw.as_str().to_owned(),
        body_posture: BodyPosture::ExactSourceBytes.as_str().to_owned(),
        label: format!("Copy raw {}", surface.label_noun()),
        safe_representation_path: false,
        attached_warning_ids,
    }
}

fn escaped_copy_choice(
    surface: M5SuspiciousTextSurface,
    attached_warning_ids: Vec<String>,
) -> M5SuspiciousTextTransferChoice {
    M5SuspiciousTextTransferChoice {
        action_id: RepresentationActionId::CopyEscaped.as_str().to_owned(),
        representation_class: RepresentationClass::Escaped.as_str().to_owned(),
        body_posture: BodyPosture::EscapedSourceText.as_str().to_owned(),
        label: format!("Copy escaped {}", surface.label_noun()),
        safe_representation_path: true,
        attached_warning_ids,
    }
}

fn build_support_admin_export(
    case_id: &str,
    surfaces: &[M5SuspiciousTextSurfaceProjection],
    findings: &[SuspiciousFinding],
) -> M5SuspiciousTextSupportAdminExport {
    let mut summaries: Vec<M5SuspiciousTextThreatCueSummary> = Vec::new();

    for threat_class in distinct_threat_classes(findings) {
        let mut warning_count = 0usize;
        let mut surface_tokens: BTreeSet<String> = BTreeSet::new();
        let mut continuity_refs: Vec<String> = Vec::new();
        let mut escaped_exemplar = String::new();

        for surface in surfaces {
            for warning in &surface.warnings {
                if warning.threat_cue.threat_class == threat_class {
                    warning_count += 1;
                    surface_tokens.insert(surface.surface_token.clone());
                    continuity_refs.push(warning.continuity_ref.clone());
                    if escaped_exemplar.is_empty() {
                        escaped_exemplar = warning.escaped_snippet.clone();
                    }
                }
            }
        }

        summaries.push(M5SuspiciousTextThreatCueSummary {
            threat_class,
            threat_class_token: threat_class.as_str().to_owned(),
            cue_label: threat_class.cue_label().to_owned(),
            severity: threat_class.severity(),
            materially_affects_trust: threat_class.materially_affects_trust(),
            warning_count,
            surfaces: surface_tokens.into_iter().collect(),
            escaped_exemplar,
            continuity_refs,
        });
    }

    M5SuspiciousTextSupportAdminExport {
        record_kind: M5_SUSPICIOUS_TEXT_SUPPORT_ADMIN_RECORD_KIND.to_owned(),
        case_id: case_id.to_owned(),
        threat_cue_summaries: summaries,
        surfaces_covered: surfaces
            .iter()
            .map(|surface| surface.surface_token.clone())
            .collect(),
        preserves_cues_without_pane: true,
        redaction_class_token: "metadata_safe_default".to_owned(),
    }
}

fn distinct_threat_classes(findings: &[SuspiciousFinding]) -> Vec<M5SuspiciousTextThreatClass> {
    findings
        .iter()
        .map(|finding| M5SuspiciousTextThreatClass::from_content_class(finding.class))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn validate_support_admin_export(
    packet: &M5SuspiciousTextParityPacket,
    violations: &mut Vec<M5SuspiciousTextParityViolation>,
) {
    let export = &packet.support_admin_export;

    let expected: BTreeSet<&str> = packet.threat_classes.iter().map(String::as_str).collect();
    let present: BTreeSet<&str> = export
        .threat_cue_summaries
        .iter()
        .map(|summary| summary.threat_class_token.as_str())
        .collect();
    if present != expected || !export.preserves_cues_without_pane {
        violations.push(M5SuspiciousTextParityViolation::SupportExportDropsCues);
    }

    // The export must never carry raw suspicious bytes: re-running the detector
    // over its serialized strings must come back clean.
    if support_export_leaks_raw_bytes(export) {
        violations.push(M5SuspiciousTextParityViolation::SupportExportLeaksRawBytes);
    }
}

fn support_export_leaks_raw_bytes(export: &M5SuspiciousTextSupportAdminExport) -> bool {
    export
        .threat_cue_summaries
        .iter()
        .any(|summary| has_suspicious_content(&summary.escaped_exemplar))
}

fn snippet_for_finding(text: &str, finding: &SuspiciousFinding) -> String {
    text.chars()
        .skip(finding.char_offset)
        .take(finding.length_chars)
        .collect()
}

fn escaped_snippet_for_finding(raw_snippet: &str, class: SuspiciousContentClass) -> String {
    match class {
        SuspiciousContentClass::BidiControl
        | SuspiciousContentClass::InvisibleFormatting
        | SuspiciousContentClass::RawRenderedDivergence => {
            raw_snippet.chars().map(escape_char).collect()
        }
        SuspiciousContentClass::MixedScriptConfusable
        | SuspiciousContentClass::WholeScriptConfusable => raw_snippet
            .chars()
            .map(|ch| {
                if ch.is_ascii() {
                    ch.to_string()
                } else {
                    escape_char(ch)
                }
            })
            .collect(),
    }
}

fn escape_char(ch: char) -> String {
    if ch.is_control() || !ch.is_ascii() {
        format!("\\u{{{:04X}}}", ch as u32)
    } else {
        ch.to_string()
    }
}
