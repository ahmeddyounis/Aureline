//! Frozen M5 suspicious-content, safe-preview, and representation-labeled
//! copy/export matrix for the new M5 artifact and viewer families.
//!
//! This module locks the canonical M5 content-integrity qualification for every
//! claimed M5 artifact/viewer family — notebooks, docs/browser panels, AI
//! evidence viewers, pipeline and artifact browsers, provider overlays,
//! marketplace install/update surfaces, remote preview targets, incident/export
//! packets, generated artifacts, and structured compare views — into one
//! export-safe packet. Each [`M5ContentIntegrityMatrixFamilyRow`] binds a family
//! to its qualification class, trust-class ladder, raw-versus-rendered posture,
//! active-content policy, copy/export representation semantics, safe-preview
//! limited-mode posture, decision-strictness display mode, downgrade triggers,
//! source contracts, and consumer-surface parity.
//!
//! The matrix is the single source of truth for whether these families may ship
//! as Stable, Beta, Preview, or must narrow further before those lanes harden
//! their own incompatible trust or representation semantics. It references the
//! shared suspicious-content, trust-class, text-representation, and
//! representation-export contracts by id rather than embedding their content.
//! Raw suspicious bytes, raw rendered trees, raw provider payloads, credentials,
//! and live preview-origin responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/security/freeze-the-m5-suspicious-content-safe-preview-and-representation-copy-export-matrix.schema.json`](../../../../schemas/security/freeze-the-m5-suspicious-content-safe-preview-and-representation-copy-export-matrix.schema.json).
//! The contract doc is
//! [`docs/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix.md`](../../../../docs/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix.md).
//! The protected fixture directory is
//! [`fixtures/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix/`](../../../../fixtures/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ContentIntegrityMatrixPacket`].
pub const M5_CONTENT_INTEGRITY_MATRIX_RECORD_KIND: &str =
    "freeze_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix";

/// Schema version for the M5 content-integrity maturity-matrix records.
pub const M5_CONTENT_INTEGRITY_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_CONTENT_INTEGRITY_MATRIX_SCHEMA_REF: &str =
    "schemas/security/freeze-the-m5-suspicious-content-safe-preview-and-representation-copy-export-matrix.schema.json";

/// Repo-relative path of the M5 content-integrity maturity-matrix contract doc.
pub const M5_CONTENT_INTEGRITY_MATRIX_DOC_REF: &str =
    "docs/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix.md";

/// Repo-relative path of the frozen trust-class vocabulary contract.
pub const M5_CONTENT_INTEGRITY_MATRIX_TRUST_CLASS_CONTRACT_REF: &str =
    "schemas/security/trust_class.schema.json";

/// Repo-relative path of the frozen text-representation policy contract.
pub const M5_CONTENT_INTEGRITY_MATRIX_TEXT_REPRESENTATION_CONTRACT_REF: &str =
    "schemas/security/text_representation_policy.schema.json";

/// Repo-relative path of the frozen safe-preview trust-class contract.
pub const M5_CONTENT_INTEGRITY_MATRIX_SAFE_PREVIEW_TRUST_CONTRACT_REF: &str =
    "schemas/trust/safe-preview-trust-class.schema.json";

/// Repo-relative path of the frozen representation-export contract.
pub const M5_CONTENT_INTEGRITY_MATRIX_REPRESENTATION_EXPORT_CONTRACT_REF: &str =
    "schemas/content/representation_export.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_CONTENT_INTEGRITY_MATRIX_FIXTURE_DIR: &str =
    "fixtures/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CONTENT_INTEGRITY_MATRIX_ARTIFACT_REF: &str =
    "artifacts/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_CONTENT_INTEGRITY_MATRIX_SUMMARY_REF: &str =
    "artifacts/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix.md";

/// One M5 artifact/viewer family governed by this content-integrity matrix.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentIntegrityArtifactFamily {
    /// Notebook rich-output blocks.
    NotebookRichOutput,
    /// Docs and in-product browser panels.
    DocsBrowserPanel,
    /// AI evidence and finding-card viewers.
    AiEvidenceViewer,
    /// Pipeline run and artifact browsers.
    PipelineArtifactBrowser,
    /// Provider account/policy overlays.
    ProviderOverlay,
    /// Marketplace install and update surfaces.
    MarketplaceInstallUpdate,
    /// Remote preview targets.
    RemotePreviewTarget,
    /// Incident and support/export packets.
    IncidentExportPacket,
    /// Generated artifacts.
    GeneratedArtifact,
    /// Structured compare and diff views.
    StructuredCompareView,
}

impl M5ContentIntegrityArtifactFamily {
    /// Every family, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::NotebookRichOutput,
        Self::DocsBrowserPanel,
        Self::AiEvidenceViewer,
        Self::PipelineArtifactBrowser,
        Self::ProviderOverlay,
        Self::MarketplaceInstallUpdate,
        Self::RemotePreviewTarget,
        Self::IncidentExportPacket,
        Self::GeneratedArtifact,
        Self::StructuredCompareView,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookRichOutput => "notebook_rich_output",
            Self::DocsBrowserPanel => "docs_browser_panel",
            Self::AiEvidenceViewer => "ai_evidence_viewer",
            Self::PipelineArtifactBrowser => "pipeline_artifact_browser",
            Self::ProviderOverlay => "provider_overlay",
            Self::MarketplaceInstallUpdate => "marketplace_install_update",
            Self::RemotePreviewTarget => "remote_preview_target",
            Self::IncidentExportPacket => "incident_export_packet",
            Self::GeneratedArtifact => "generated_artifact",
            Self::StructuredCompareView => "structured_compare_view",
        }
    }

    /// Whether this family is a strong-decision surface (install/update,
    /// attach/share, collaboration, or policy review) that must render owner
    /// and origin identity more strictly than ordinary browsing panes.
    pub const fn is_strong_decision_surface(self) -> bool {
        matches!(
            self,
            Self::ProviderOverlay | Self::MarketplaceInstallUpdate | Self::RemotePreviewTarget
        )
    }

    /// Whether this family is an embedded or review surface that must never
    /// auto-execute active rich content.
    pub const fn is_review_or_export_surface(self) -> bool {
        matches!(
            self,
            Self::AiEvidenceViewer
                | Self::IncidentExportPacket
                | Self::GeneratedArtifact
                | Self::StructuredCompareView
        )
    }
}

/// Qualification class for an M5 artifact/viewer family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentIntegrityQualificationClass {
    /// Family qualifies for the Stable claim.
    Stable,
    /// Family is narrowed to Beta.
    Beta,
    /// Family is narrowed to Preview.
    Preview,
    /// Family is experimental and not claimed.
    Experimental,
    /// Family is unavailable on this build.
    Unavailable,
    /// Family is held pending upstream resolution.
    Held,
}

impl M5ContentIntegrityQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the family may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// A single rung in a family's safe-preview trust-class ladder.
///
/// Tokens mirror the closed trust-class vocabulary in
/// [`schemas/security/trust_class.schema.json`](../../../../schemas/security/trust_class.schema.json).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentIntegrityTrustClass {
    /// Plain, raw text — no rendering, no active behavior.
    RawText,
    /// Sanitized rich rendering with active content neutralized.
    SanitizedRich,
    /// Active content that may run only inside the declared trusted-local class.
    TrustedLocalActive,
    /// Active remote content confined to an isolated runtime class.
    IsolatedRemoteActive,
    /// Content blocked from rendering pending review.
    Blocked,
}

impl M5ContentIntegrityTrustClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawText => "raw_text",
            Self::SanitizedRich => "sanitized_rich",
            Self::TrustedLocalActive => "trusted_local_active",
            Self::IsolatedRemoteActive => "isolated_remote_active",
            Self::Blocked => "blocked",
        }
    }
}

/// Raw-versus-rendered posture for a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentIntegrityRawRenderedPosture {
    /// Raw and rendered forms can differ materially; raw inspection and raw copy
    /// stay reachable whenever they do.
    RawAndRenderedDistinctBothReachable,
    /// Rendered form is the default; raw inspection stays reachable on demand.
    RenderedDefaultRawOnDemand,
    /// Only raw form is presented; no rendering step can diverge from bytes.
    RawOnlyNoRendering,
    /// No raw/rendered distinction applies to this family.
    NotApplicable,
}

impl M5ContentIntegrityRawRenderedPosture {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawAndRenderedDistinctBothReachable => "raw_and_rendered_distinct_both_reachable",
            Self::RenderedDefaultRawOnDemand => "rendered_default_raw_on_demand",
            Self::RawOnlyNoRendering => "raw_only_no_rendering",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether raw inspection and raw copy must stay reachable for this posture.
    pub const fn requires_raw_reachable(self) -> bool {
        matches!(
            self,
            Self::RawAndRenderedDistinctBothReachable | Self::RenderedDefaultRawOnDemand
        )
    }
}

/// Active-content policy for a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentIntegrityActiveContentPolicy {
    /// Active content is inert and never executes in this surface.
    InertNeverExecutes,
    /// Active content may run only inside an isolated remote runtime class.
    IsolatedRemoteSandboxOnly,
    /// Active content may run only inside the declared trusted-local class.
    TrustedLocalOnly,
    /// Active content is blocked pending review.
    BlockedPendingReview,
}

impl M5ContentIntegrityActiveContentPolicy {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InertNeverExecutes => "inert_never_executes",
            Self::IsolatedRemoteSandboxOnly => "isolated_remote_sandbox_only",
            Self::TrustedLocalOnly => "trusted_local_only",
            Self::BlockedPendingReview => "blocked_pending_review",
        }
    }

    /// Whether this policy keeps active content from executing in the surface.
    pub const fn is_non_executing(self) -> bool {
        matches!(self, Self::InertNeverExecutes | Self::BlockedPendingReview)
    }
}

/// Copy/export representation semantics for a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentIntegrityCopyExportRepresentation {
    /// Raw and escaped/rendered copy are offered as distinctly labeled actions;
    /// rendered copy never masquerades as raw bytes.
    RawAndEscapedLabeledDistinct,
    /// Escaped/safe form is the default copy; raw copy stays reachable.
    EscapedDefaultRawReachable,
    /// Only redaction-safe metadata is exported; no raw content body crosses.
    MetadataOnlyNoRawBody,
    /// No copy/export representation choice applies to this family.
    NotApplicable,
}

impl M5ContentIntegrityCopyExportRepresentation {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawAndEscapedLabeledDistinct => "raw_and_escaped_labeled_distinct",
            Self::EscapedDefaultRawReachable => "escaped_default_raw_reachable",
            Self::MetadataOnlyNoRawBody => "metadata_only_no_raw_body",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// Whether this representation keeps a raw copy/export path reachable.
    pub const fn keeps_raw_reachable(self) -> bool {
        matches!(
            self,
            Self::RawAndEscapedLabeledDistinct | Self::EscapedDefaultRawReachable
        )
    }
}

/// Safe-preview limited-mode posture for a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentIntegritySafePreviewMode {
    /// Full safe preview is available.
    FullPreview,
    /// A limited-mode fallback is available when full preview cannot be trusted.
    LimitedModeAvailable,
    /// Limited mode is the default posture for this family.
    LimitedModeDefault,
    /// Preview is blocked; only metadata is shown.
    Blocked,
}

impl M5ContentIntegritySafePreviewMode {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullPreview => "full_preview",
            Self::LimitedModeAvailable => "limited_mode_available",
            Self::LimitedModeDefault => "limited_mode_default",
            Self::Blocked => "blocked",
        }
    }
}

/// Decision-strictness display mode for a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentIntegrityDisplayMode {
    /// Ordinary browsing-pane identity rendering.
    OrdinaryBrowsing,
    /// Stricter owner/origin identity rendering for strong-decision surfaces.
    StrongDecisionStrictIdentity,
}

impl M5ContentIntegrityDisplayMode {
    /// Stable token recorded in the matrix.
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

/// Evidence requirement level for a family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentIntegrityEvidenceRequirement {
    /// At least one evidence packet is required.
    Required,
    /// Evidence is recommended but not blocking.
    Recommended,
    /// Evidence is optional.
    Optional,
    /// Not applicable for this family's current qualification.
    NotApplicable,
}

impl M5ContentIntegrityEvidenceRequirement {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::Recommended => "recommended",
            Self::Optional => "optional",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Downgrade trigger that can narrow a family below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentIntegrityDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Suspicious content (bidi/invisible/confusable) was detected.
    SuspiciousContentDetected,
    /// Raw and rendered forms diverge and the divergence is unresolved.
    RawRenderedDivergenceUnresolved,
    /// Active content cannot be confined to its declared trust class.
    ActiveContentUntrusted,
    /// Safe-preview rendering is unavailable for this family.
    SafePreviewUnavailable,
    /// Surface trust narrowed.
    TrustNarrowing,
    /// Scope expanded beyond the qualified content-integrity boundary.
    ScopeExpansionUnqualified,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl M5ContentIntegrityDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::SuspiciousContentDetected,
        Self::RawRenderedDivergenceUnresolved,
        Self::ActiveContentUntrusted,
        Self::SafePreviewUnavailable,
        Self::TrustNarrowing,
        Self::ScopeExpansionUnqualified,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::SuspiciousContentDetected => "suspicious_content_detected",
            Self::RawRenderedDivergenceUnresolved => "raw_rendered_divergence_unresolved",
            Self::ActiveContentUntrusted => "active_content_untrusted",
            Self::SafePreviewUnavailable => "safe_preview_unavailable",
            Self::TrustNarrowing => "trust_narrowing",
            Self::ScopeExpansionUnqualified => "scope_expansion_unqualified",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project a family's content-integrity truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContentIntegrityConsumerSurface {
    /// Notebook viewer.
    NotebookViewer,
    /// Docs / in-product browser panel.
    DocsBrowserPanel,
    /// AI evidence viewer.
    AiEvidenceViewer,
    /// Pipeline / artifact browser.
    PipelineArtifactBrowser,
    /// Provider overlay.
    ProviderOverlay,
    /// Marketplace install/update surface.
    MarketplaceSurface,
    /// Remote preview panel.
    RemotePreviewPanel,
    /// Incident / support export packet.
    IncidentExport,
    /// Structured compare view.
    StructuredCompareView,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl M5ContentIntegrityConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::NotebookViewer,
        Self::DocsBrowserPanel,
        Self::AiEvidenceViewer,
        Self::PipelineArtifactBrowser,
        Self::ProviderOverlay,
        Self::MarketplaceSurface,
        Self::RemotePreviewPanel,
        Self::IncidentExport,
        Self::StructuredCompareView,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookViewer => "notebook_viewer",
            Self::DocsBrowserPanel => "docs_browser_panel",
            Self::AiEvidenceViewer => "ai_evidence_viewer",
            Self::PipelineArtifactBrowser => "pipeline_artifact_browser",
            Self::ProviderOverlay => "provider_overlay",
            Self::MarketplaceSurface => "marketplace_surface",
            Self::RemotePreviewPanel => "remote_preview_panel",
            Self::IncidentExport => "incident_export",
            Self::StructuredCompareView => "structured_compare_view",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One row in the M5 content-integrity maturity matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContentIntegrityMatrixFamilyRow {
    /// Artifact/viewer family.
    pub family: M5ContentIntegrityArtifactFamily,
    /// Qualification class earned by this family.
    pub qualification: M5ContentIntegrityQualificationClass,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Trust-class ladder this family climbs through.
    pub trust_class_ladder: Vec<M5ContentIntegrityTrustClass>,
    /// Raw-versus-rendered posture.
    pub raw_rendered_posture: M5ContentIntegrityRawRenderedPosture,
    /// Active-content policy.
    pub active_content_policy: M5ContentIntegrityActiveContentPolicy,
    /// Copy/export representation semantics.
    pub copy_export_representation: M5ContentIntegrityCopyExportRepresentation,
    /// Safe-preview limited-mode posture.
    pub safe_preview_mode: M5ContentIntegritySafePreviewMode,
    /// Decision-strictness display mode.
    pub display_mode: M5ContentIntegrityDisplayMode,
    /// Evidence requirement level.
    pub evidence_requirement: M5ContentIntegrityEvidenceRequirement,
    /// Required evidence packet refs for this qualification.
    pub required_evidence_packet_refs: Vec<String>,
    /// Downgrade triggers that apply to this family.
    pub downgrade_triggers: Vec<M5ContentIntegrityDowngradeTrigger>,
    /// Source contract refs consumed by this family.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that must project this family.
    pub consumer_surfaces: Vec<M5ContentIntegrityConsumerSurface>,
}

/// Trust and content-integrity review block.
///
/// Every field encodes a hard invariant; all must hold for the matrix to
/// validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContentIntegrityMatrixTrustReview {
    /// One shared suspicious-text and content-integrity policy library governs all families.
    pub one_shared_policy_library_governs_all_families: bool,
    /// Trust-decision surfaces use the stricter display mode.
    pub trust_decision_surfaces_use_stricter_display_mode: bool,
    /// Raw inspection and raw copy stay reachable whenever rendered form differs materially.
    pub raw_inspection_and_copy_reachable_on_divergence: bool,
    /// Active content never executes outside its declared trust class.
    pub active_content_never_executes_outside_trust_class: bool,
    /// Bidi/invisible/confusable fixes never rewrite bytes silently on save, format, organize-imports, or AI apply.
    pub bidi_invisible_confusable_never_silently_rewritten: bool,
    /// Suspicious bytes are never normalized away.
    pub suspicious_bytes_not_normalized_away: bool,
    /// Rendered copy never masquerades as raw bytes.
    pub rendered_copy_never_masquerades_as_raw: bool,
    /// Active rich content never auto-executes in embedded or review surfaces.
    pub no_auto_execute_in_embedded_or_review_surfaces: bool,
    /// Downgrade narrows the claim rather than hiding the family.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContentIntegrityMatrixConsumerProjection {
    /// Notebook viewer shows trust class and active-content state.
    pub notebook_shows_trust_class_and_active_content_state: bool,
    /// Docs/browser panel shows raw/rendered and safe-preview state.
    pub docs_browser_shows_raw_rendered_and_safe_preview_state: bool,
    /// AI evidence viewer shows representation labels.
    pub ai_evidence_viewer_shows_representation_labels: bool,
    /// Marketplace install/update uses strict identity rendering.
    pub marketplace_install_uses_strict_identity_rendering: bool,
    /// Copy/export affordances label raw versus rendered.
    pub copy_export_labels_raw_versus_rendered: bool,
    /// CLI / headless shows qualification truth.
    pub cli_headless_shows_qualification: bool,
    /// Support export shows qualification truth.
    pub support_export_shows_qualification: bool,
    /// Diagnostics shows qualification truth.
    pub diagnostics_shows_qualification: bool,
    /// Help / About shows qualification truth.
    pub help_about_shows_qualification: bool,
    /// Preview / Labs families are visibly labeled when not covered by this packet.
    pub preview_labs_label_for_unqualified_families: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContentIntegrityMatrixProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the family.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ContentIntegrityMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ContentIntegrityMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Family rows.
    pub family_rows: Vec<M5ContentIntegrityMatrixFamilyRow>,
    /// Trust review block.
    pub trust_review: M5ContentIntegrityMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ContentIntegrityMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ContentIntegrityMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 content-integrity maturity-matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContentIntegrityMatrixPacket {
    /// Record kind; must equal [`M5_CONTENT_INTEGRITY_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CONTENT_INTEGRITY_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Family rows.
    pub family_rows: Vec<M5ContentIntegrityMatrixFamilyRow>,
    /// Trust review block.
    pub trust_review: M5ContentIntegrityMatrixTrustReview,
    /// Consumer projection block.
    pub consumer_projection: M5ContentIntegrityMatrixConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ContentIntegrityMatrixProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ContentIntegrityMatrixPacket {
    /// Builds an M5 content-integrity maturity-matrix packet from frozen input.
    pub fn new(input: M5ContentIntegrityMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_CONTENT_INTEGRITY_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_CONTENT_INTEGRITY_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            family_rows: input.family_rows,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 content-integrity maturity-matrix invariants.
    pub fn validate(&self) -> Vec<M5ContentIntegrityMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CONTENT_INTEGRITY_MATRIX_RECORD_KIND {
            violations.push(M5ContentIntegrityMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CONTENT_INTEGRITY_MATRIX_SCHEMA_VERSION {
            violations.push(M5ContentIntegrityMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ContentIntegrityMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_family_rows(self, &mut violations);
        validate_trust_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 content-integrity matrix packet serializes"),
        ) {
            violations.push(M5ContentIntegrityMatrixViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 content-integrity matrix packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_families = self
            .family_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Suspicious-Content, Safe-Preview, and Copy/Export Matrix\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Families: {} ({} stable)\n",
            self.family_rows.len(),
            stable_families
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Families\n\n");
        for row in &self.family_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Raw/rendered: {} · Active content: {}\n",
                row.raw_rendered_posture.as_str(),
                row.active_content_policy.as_str()
            ));
            out.push_str(&format!(
                "  - Copy/export: {} · Safe preview: {} · Display: {}\n",
                row.copy_export_representation.as_str(),
                row.safe_preview_mode.as_str(),
                row.display_mode.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 content-integrity matrix export.
#[derive(Debug)]
pub enum M5ContentIntegrityMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ContentIntegrityMatrixViolation>),
}

impl fmt::Display for M5ContentIntegrityMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 content-integrity matrix export parse failed: {error}"
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
                    "m5 content-integrity matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ContentIntegrityMatrixArtifactError {}

/// Validation failures emitted by [`M5ContentIntegrityMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ContentIntegrityMatrixViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A required family is missing from the matrix.
    RequiredFamilyMissing,
    /// A family row is incomplete.
    FamilyRowIncomplete,
    /// A family claiming Stable is missing required evidence packet refs.
    StableFamilyMissingEvidence,
    /// A family has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A family has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A family has an empty trust-class ladder.
    TrustClassLadderMissing,
    /// Raw copy/inspection is unreachable while raw and rendered forms can diverge.
    RawCopyUnreachableOnDivergence,
    /// A strong-decision family does not use the stricter display mode.
    StrongDecisionDisplayModeTooWeak,
    /// An embedded or review surface allows active content to execute.
    ActiveContentInReviewSurface,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5ContentIntegrityMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::FamilyRowIncomplete => "family_row_incomplete",
            Self::StableFamilyMissingEvidence => "stable_family_missing_evidence",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustClassLadderMissing => "trust_class_ladder_missing",
            Self::RawCopyUnreachableOnDivergence => "raw_copy_unreachable_on_divergence",
            Self::StrongDecisionDisplayModeTooWeak => "strong_decision_display_mode_too_weak",
            Self::ActiveContentInReviewSurface => "active_content_in_review_surface",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Stable packet id minted by [`frozen_stable_m5_content_integrity_matrix_packet`].
pub const M5_CONTENT_INTEGRITY_MATRIX_PACKET_ID: &str = "m5-content-integrity-matrix:stable:0001";

/// Builds the canonical frozen stable M5 content-integrity matrix packet.
///
/// This is the single in-code source of truth for the checked-in support
/// export at [`M5_CONTENT_INTEGRITY_MATRIX_ARTIFACT_REF`]; the matrix bin emits
/// this packet and a test asserts the checked-in artifact deserializes back to
/// it unchanged.
pub fn frozen_stable_m5_content_integrity_matrix_packet() -> M5ContentIntegrityMatrixPacket {
    use M5ContentIntegrityActiveContentPolicy as Active;
    use M5ContentIntegrityArtifactFamily as Family;
    use M5ContentIntegrityConsumerSurface as Surface;
    use M5ContentIntegrityCopyExportRepresentation as Copy;
    use M5ContentIntegrityDisplayMode as Display;
    use M5ContentIntegrityDowngradeTrigger as Trigger;
    use M5ContentIntegrityEvidenceRequirement as Evidence;
    use M5ContentIntegrityQualificationClass as Qual;
    use M5ContentIntegrityRawRenderedPosture as RawRendered;
    use M5ContentIntegritySafePreviewMode as Preview;
    use M5ContentIntegrityTrustClass as Trust;

    let trust_class_contract = M5_CONTENT_INTEGRITY_MATRIX_TRUST_CLASS_CONTRACT_REF.to_owned();
    let representation_contract =
        M5_CONTENT_INTEGRITY_MATRIX_TEXT_REPRESENTATION_CONTRACT_REF.to_owned();

    let family_rows = vec![
        M5ContentIntegrityMatrixFamilyRow {
            family: Family::NotebookRichOutput,
            qualification: Qual::Beta,
            scope_summary: "Notebook rich-output blocks climb a trust-class ladder from raw text to sanitized rich and isolated remote active; suspicious text stays annotated, raw cells stay inspectable, and active output never executes outside its declared isolated runtime".to_owned(),
            trust_class_ladder: vec![Trust::RawText, Trust::SanitizedRich, Trust::TrustedLocalActive, Trust::IsolatedRemoteActive],
            raw_rendered_posture: RawRendered::RawAndRenderedDistinctBothReachable,
            active_content_policy: Active::IsolatedRemoteSandboxOnly,
            copy_export_representation: Copy::RawAndEscapedLabeledDistinct,
            safe_preview_mode: Preview::LimitedModeAvailable,
            display_mode: Display::OrdinaryBrowsing,
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec!["evidence:notebook-rich-output-safe-preview:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ProofStale, Trigger::SuspiciousContentDetected, Trigger::SafePreviewUnavailable, Trigger::ActiveContentUntrusted],
            source_contract_refs: vec![trust_class_contract.clone()],
            consumer_surfaces: vec![Surface::NotebookViewer, Surface::CliHeadless, Surface::SupportExport, Surface::Diagnostics],
        },
        M5ContentIntegrityMatrixFamilyRow {
            family: Family::DocsBrowserPanel,
            qualification: Qual::Stable,
            scope_summary: "Docs and in-product browser panels render sanitized rich content by default with raw source reachable on demand; remote active content is confined to an isolated runtime and never crosses into the trusted-local class".to_owned(),
            trust_class_ladder: vec![Trust::RawText, Trust::SanitizedRich, Trust::IsolatedRemoteActive],
            raw_rendered_posture: RawRendered::RenderedDefaultRawOnDemand,
            active_content_policy: Active::IsolatedRemoteSandboxOnly,
            copy_export_representation: Copy::EscapedDefaultRawReachable,
            safe_preview_mode: Preview::FullPreview,
            display_mode: Display::OrdinaryBrowsing,
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec!["evidence:docs-browser-raw-rendered-parity:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ProofStale, Trigger::SuspiciousContentDetected, Trigger::RawRenderedDivergenceUnresolved, Trigger::SafePreviewUnavailable],
            source_contract_refs: vec![representation_contract.clone()],
            consumer_surfaces: vec![Surface::DocsBrowserPanel, Surface::CliHeadless, Surface::SupportExport, Surface::Diagnostics],
        },
        M5ContentIntegrityMatrixFamilyRow {
            family: Family::AiEvidenceViewer,
            qualification: Qual::Stable,
            scope_summary: "AI evidence and finding-card viewers present raw and sanitized representations as distinctly labeled forms; quoted model and tool output is inert, never executes, and raw inspection of the underlying evidence stays reachable".to_owned(),
            trust_class_ladder: vec![Trust::RawText, Trust::SanitizedRich],
            raw_rendered_posture: RawRendered::RawAndRenderedDistinctBothReachable,
            active_content_policy: Active::InertNeverExecutes,
            copy_export_representation: Copy::RawAndEscapedLabeledDistinct,
            safe_preview_mode: Preview::FullPreview,
            display_mode: Display::OrdinaryBrowsing,
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec!["evidence:ai-evidence-representation-labels:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ProofStale, Trigger::SuspiciousContentDetected, Trigger::RawRenderedDivergenceUnresolved],
            source_contract_refs: vec![representation_contract.clone()],
            consumer_surfaces: vec![Surface::AiEvidenceViewer, Surface::CliHeadless, Surface::SupportExport, Surface::Diagnostics],
        },
        M5ContentIntegrityMatrixFamilyRow {
            family: Family::PipelineArtifactBrowser,
            qualification: Qual::Stable,
            scope_summary: "Pipeline run and artifact browsers render logs and artifact previews through the safe-preview boundary with raw download reachable; untrusted artifact bodies stay inert and are blocked rather than executed".to_owned(),
            trust_class_ladder: vec![Trust::RawText, Trust::SanitizedRich, Trust::Blocked],
            raw_rendered_posture: RawRendered::RenderedDefaultRawOnDemand,
            active_content_policy: Active::InertNeverExecutes,
            copy_export_representation: Copy::EscapedDefaultRawReachable,
            safe_preview_mode: Preview::LimitedModeAvailable,
            display_mode: Display::OrdinaryBrowsing,
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec!["evidence:pipeline-artifact-safe-preview:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ProofStale, Trigger::SuspiciousContentDetected, Trigger::SafePreviewUnavailable],
            source_contract_refs: vec![trust_class_contract.clone()],
            consumer_surfaces: vec![Surface::PipelineArtifactBrowser, Surface::CliHeadless, Surface::SupportExport, Surface::Diagnostics],
        },
        M5ContentIntegrityMatrixFamilyRow {
            family: Family::ProviderOverlay,
            qualification: Qual::Stable,
            scope_summary: "Provider account and policy overlays render owner and origin identity in strong-decision strict mode; embedded provider content is inert and suspicious identifiers are annotated rather than silently normalized".to_owned(),
            trust_class_ladder: vec![Trust::RawText, Trust::SanitizedRich],
            raw_rendered_posture: RawRendered::RenderedDefaultRawOnDemand,
            active_content_policy: Active::InertNeverExecutes,
            copy_export_representation: Copy::EscapedDefaultRawReachable,
            safe_preview_mode: Preview::FullPreview,
            display_mode: Display::StrongDecisionStrictIdentity,
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec!["evidence:provider-overlay-strict-identity:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ProofStale, Trigger::SuspiciousContentDetected, Trigger::TrustNarrowing, Trigger::PolicyBlocked],
            source_contract_refs: vec![trust_class_contract.clone()],
            consumer_surfaces: vec![Surface::ProviderOverlay, Surface::CliHeadless, Surface::SupportExport, Surface::Diagnostics],
        },
        M5ContentIntegrityMatrixFamilyRow {
            family: Family::MarketplaceInstallUpdate,
            qualification: Qual::Stable,
            scope_summary: "Marketplace install and update surfaces render publisher identity in strong-decision strict mode; active payloads are blocked pending review, raw and rendered manifests are labeled distinctly, and confusable publisher names are surfaced".to_owned(),
            trust_class_ladder: vec![Trust::RawText, Trust::SanitizedRich, Trust::Blocked],
            raw_rendered_posture: RawRendered::RawAndRenderedDistinctBothReachable,
            active_content_policy: Active::BlockedPendingReview,
            copy_export_representation: Copy::RawAndEscapedLabeledDistinct,
            safe_preview_mode: Preview::LimitedModeDefault,
            display_mode: Display::StrongDecisionStrictIdentity,
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec!["evidence:marketplace-install-strict-identity:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ProofStale, Trigger::SuspiciousContentDetected, Trigger::PolicyBlocked, Trigger::TrustNarrowing],
            source_contract_refs: vec![trust_class_contract.clone()],
            consumer_surfaces: vec![Surface::MarketplaceSurface, Surface::CliHeadless, Surface::SupportExport, Surface::Diagnostics],
        },
        M5ContentIntegrityMatrixFamilyRow {
            family: Family::RemotePreviewTarget,
            qualification: Qual::Beta,
            scope_summary: "Remote preview targets render in strong-decision strict mode with limited preview by default; remote active content is confined to an isolated runtime and the raw target identity stays reachable for inspection".to_owned(),
            trust_class_ladder: vec![Trust::RawText, Trust::SanitizedRich, Trust::IsolatedRemoteActive],
            raw_rendered_posture: RawRendered::RenderedDefaultRawOnDemand,
            active_content_policy: Active::IsolatedRemoteSandboxOnly,
            copy_export_representation: Copy::EscapedDefaultRawReachable,
            safe_preview_mode: Preview::LimitedModeDefault,
            display_mode: Display::StrongDecisionStrictIdentity,
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec!["evidence:remote-preview-target-isolation:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ProofStale, Trigger::ScopeExpansionUnqualified, Trigger::SafePreviewUnavailable, Trigger::PolicyBlocked],
            source_contract_refs: vec![M5_CONTENT_INTEGRITY_MATRIX_SAFE_PREVIEW_TRUST_CONTRACT_REF.to_owned()],
            consumer_surfaces: vec![Surface::RemotePreviewPanel, Surface::CliHeadless, Surface::SupportExport, Surface::HelpAbout],
        },
        M5ContentIntegrityMatrixFamilyRow {
            family: Family::IncidentExportPacket,
            qualification: Qual::Stable,
            scope_summary: "Incident and support/export packets carry redaction-safe metadata only; no raw suspicious body crosses the export boundary and quoted content is inert with no executable form".to_owned(),
            trust_class_ladder: vec![Trust::RawText, Trust::SanitizedRich],
            raw_rendered_posture: RawRendered::RawOnlyNoRendering,
            active_content_policy: Active::InertNeverExecutes,
            copy_export_representation: Copy::MetadataOnlyNoRawBody,
            safe_preview_mode: Preview::LimitedModeDefault,
            display_mode: Display::OrdinaryBrowsing,
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec!["evidence:incident-export-redaction-safe:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ProofStale, Trigger::SuspiciousContentDetected, Trigger::PolicyBlocked],
            source_contract_refs: vec![M5_CONTENT_INTEGRITY_MATRIX_REPRESENTATION_EXPORT_CONTRACT_REF.to_owned()],
            consumer_surfaces: vec![Surface::IncidentExport, Surface::CliHeadless, Surface::SupportExport, Surface::Diagnostics],
        },
        M5ContentIntegrityMatrixFamilyRow {
            family: Family::GeneratedArtifact,
            qualification: Qual::Beta,
            scope_summary: "Generated artifacts preserve raw and rendered forms as distinctly labeled representations; generated content is inert, never executes on view, and raw copy stays reachable whenever the rendered form differs".to_owned(),
            trust_class_ladder: vec![Trust::RawText, Trust::SanitizedRich],
            raw_rendered_posture: RawRendered::RawAndRenderedDistinctBothReachable,
            active_content_policy: Active::InertNeverExecutes,
            copy_export_representation: Copy::RawAndEscapedLabeledDistinct,
            safe_preview_mode: Preview::LimitedModeAvailable,
            display_mode: Display::OrdinaryBrowsing,
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec!["evidence:generated-artifact-raw-rendered-parity:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ProofStale, Trigger::SuspiciousContentDetected, Trigger::RawRenderedDivergenceUnresolved],
            source_contract_refs: vec![M5_CONTENT_INTEGRITY_MATRIX_REPRESENTATION_EXPORT_CONTRACT_REF.to_owned()],
            consumer_surfaces: vec![Surface::StructuredCompareView, Surface::CliHeadless, Surface::SupportExport, Surface::Diagnostics],
        },
        M5ContentIntegrityMatrixFamilyRow {
            family: Family::StructuredCompareView,
            qualification: Qual::Stable,
            scope_summary: "Structured compare and diff views render raw and rendered forms side by side with both reachable; compared content is inert, never executes, and rendered copy never masquerades as raw bytes".to_owned(),
            trust_class_ladder: vec![Trust::RawText, Trust::SanitizedRich],
            raw_rendered_posture: RawRendered::RawAndRenderedDistinctBothReachable,
            active_content_policy: Active::InertNeverExecutes,
            copy_export_representation: Copy::RawAndEscapedLabeledDistinct,
            safe_preview_mode: Preview::FullPreview,
            display_mode: Display::OrdinaryBrowsing,
            evidence_requirement: Evidence::Required,
            required_evidence_packet_refs: vec!["evidence:structured-compare-raw-rendered-parity:m5".to_owned()],
            downgrade_triggers: vec![Trigger::ProofStale, Trigger::SuspiciousContentDetected, Trigger::RawRenderedDivergenceUnresolved],
            source_contract_refs: vec![representation_contract.clone()],
            consumer_surfaces: vec![Surface::StructuredCompareView, Surface::CliHeadless, Surface::SupportExport, Surface::Diagnostics],
        },
    ];

    M5ContentIntegrityMatrixPacket::new(M5ContentIntegrityMatrixPacketInput {
        packet_id: M5_CONTENT_INTEGRITY_MATRIX_PACKET_ID.to_owned(),
        matrix_label:
            "M5 Suspicious-Content, Safe-Preview, and Representation-Labeled Copy/Export Matrix"
                .to_owned(),
        family_rows,
        trust_review: M5ContentIntegrityMatrixTrustReview {
            one_shared_policy_library_governs_all_families: true,
            trust_decision_surfaces_use_stricter_display_mode: true,
            raw_inspection_and_copy_reachable_on_divergence: true,
            active_content_never_executes_outside_trust_class: true,
            bidi_invisible_confusable_never_silently_rewritten: true,
            suspicious_bytes_not_normalized_away: true,
            rendered_copy_never_masquerades_as_raw: true,
            no_auto_execute_in_embedded_or_review_surfaces: true,
            downgrade_narrows_instead_of_hides: true,
            stale_or_underqualified_blocks_promotion: true,
        },
        consumer_projection: M5ContentIntegrityMatrixConsumerProjection {
            notebook_shows_trust_class_and_active_content_state: true,
            docs_browser_shows_raw_rendered_and_safe_preview_state: true,
            ai_evidence_viewer_shows_representation_labels: true,
            marketplace_install_uses_strict_identity_rendering: true,
            copy_export_labels_raw_versus_rendered: true,
            cli_headless_shows_qualification: true,
            support_export_shows_qualification: true,
            diagnostics_shows_qualification: true,
            help_about_shows_qualification: true,
            preview_labs_label_for_unqualified_families: true,
        },
        proof_freshness: M5ContentIntegrityMatrixProofFreshness {
            proof_freshness_slo_hours: 168,
            last_proof_refresh: "2026-06-10T00:00:00Z".to_owned(),
            auto_narrow_on_stale: true,
        },
        source_contract_refs: vec![
            M5_CONTENT_INTEGRITY_MATRIX_SCHEMA_REF.to_owned(),
            M5_CONTENT_INTEGRITY_MATRIX_DOC_REF.to_owned(),
            M5_CONTENT_INTEGRITY_MATRIX_TRUST_CLASS_CONTRACT_REF.to_owned(),
            M5_CONTENT_INTEGRITY_MATRIX_TEXT_REPRESENTATION_CONTRACT_REF.to_owned(),
            M5_CONTENT_INTEGRITY_MATRIX_SAFE_PREVIEW_TRUST_CONTRACT_REF.to_owned(),
            M5_CONTENT_INTEGRITY_MATRIX_REPRESENTATION_EXPORT_CONTRACT_REF.to_owned(),
        ],
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at: "2026-06-10T00:00:00Z".to_owned(),
    })
}

/// Reads and validates the checked-in stable M5 content-integrity matrix export.
pub fn current_stable_m5_content_integrity_matrix_export(
) -> Result<M5ContentIntegrityMatrixPacket, M5ContentIntegrityMatrixArtifactError> {
    let packet: M5ContentIntegrityMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/security/m5/freeze_the_m5_suspicious_content_safe_preview_and_representation_copy_export_matrix/support_export.json"
    )))
    .map_err(M5ContentIntegrityMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ContentIntegrityMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ContentIntegrityMatrixPacket,
    violations: &mut Vec<M5ContentIntegrityMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CONTENT_INTEGRITY_MATRIX_SCHEMA_REF,
        M5_CONTENT_INTEGRITY_MATRIX_DOC_REF,
        M5_CONTENT_INTEGRITY_MATRIX_TRUST_CLASS_CONTRACT_REF,
        M5_CONTENT_INTEGRITY_MATRIX_TEXT_REPRESENTATION_CONTRACT_REF,
        M5_CONTENT_INTEGRITY_MATRIX_SAFE_PREVIEW_TRUST_CONTRACT_REF,
        M5_CONTENT_INTEGRITY_MATRIX_REPRESENTATION_EXPORT_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ContentIntegrityMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_family_rows(
    packet: &M5ContentIntegrityMatrixPacket,
    violations: &mut Vec<M5ContentIntegrityMatrixViolation>,
) {
    let present: BTreeSet<M5ContentIntegrityArtifactFamily> =
        packet.family_rows.iter().map(|row| row.family).collect();
    for required in M5ContentIntegrityArtifactFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5ContentIntegrityMatrixViolation::RequiredFamilyMissing);
            return;
        }
    }

    for row in &packet.family_rows {
        if row.scope_summary.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(M5ContentIntegrityMatrixViolation::FamilyRowIncomplete);
        }
        if row.trust_class_ladder.is_empty() {
            violations.push(M5ContentIntegrityMatrixViolation::TrustClassLadderMissing);
        }
        if row.qualification.is_stable() && row.required_evidence_packet_refs.is_empty() {
            violations.push(M5ContentIntegrityMatrixViolation::StableFamilyMissingEvidence);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ContentIntegrityMatrixViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ContentIntegrityMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.raw_rendered_posture.requires_raw_reachable()
            && !row.copy_export_representation.keeps_raw_reachable()
        {
            violations.push(M5ContentIntegrityMatrixViolation::RawCopyUnreachableOnDivergence);
        }
        if row.family.is_strong_decision_surface() && !row.display_mode.is_strict() {
            violations.push(M5ContentIntegrityMatrixViolation::StrongDecisionDisplayModeTooWeak);
        }
        if row.family.is_review_or_export_surface() && !row.active_content_policy.is_non_executing()
        {
            violations.push(M5ContentIntegrityMatrixViolation::ActiveContentInReviewSurface);
        }
    }
}

fn validate_trust_review(
    packet: &M5ContentIntegrityMatrixPacket,
    violations: &mut Vec<M5ContentIntegrityMatrixViolation>,
) {
    let review = &packet.trust_review;
    for ok in [
        review.one_shared_policy_library_governs_all_families,
        review.trust_decision_surfaces_use_stricter_display_mode,
        review.raw_inspection_and_copy_reachable_on_divergence,
        review.active_content_never_executes_outside_trust_class,
        review.bidi_invisible_confusable_never_silently_rewritten,
        review.suspicious_bytes_not_normalized_away,
        review.rendered_copy_never_masquerades_as_raw,
        review.no_auto_execute_in_embedded_or_review_surfaces,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(M5ContentIntegrityMatrixViolation::TrustReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ContentIntegrityMatrixPacket,
    violations: &mut Vec<M5ContentIntegrityMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.notebook_shows_trust_class_and_active_content_state,
        projection.docs_browser_shows_raw_rendered_and_safe_preview_state,
        projection.ai_evidence_viewer_shows_representation_labels,
        projection.marketplace_install_uses_strict_identity_rendering,
        projection.copy_export_labels_raw_versus_rendered,
        projection.cli_headless_shows_qualification,
        projection.support_export_shows_qualification,
        projection.diagnostics_shows_qualification,
        projection.help_about_shows_qualification,
        projection.preview_labs_label_for_unqualified_families,
    ] {
        if !ok {
            violations.push(M5ContentIntegrityMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ContentIntegrityMatrixPacket,
    violations: &mut Vec<M5ContentIntegrityMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ContentIntegrityMatrixViolation::ProofFreshnessIncomplete);
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
