//! Convention diagnostics with confidence labels, suppressibility, and
//! proving-file disclosure.
//!
//! This module locks the canonical, export-safe packet for the framework-pack
//! convention-diagnostic lane. Each [`ConventionDiagnosticRow`] binds one
//! convention diagnostic — a check a framework pack or template raises about a
//! naming, file-location, registration, configuration, or API-usage convention —
//! to its confidence label, how analysis-fresh it is, whether and how it may be
//! suppressed, which proving file or manifest grounds it, the support class on
//! which it may be presented, and its downgrade banner. The editor diagnostics,
//! problems panel, diff-review, run, diagnostics, and support surfaces project the
//! same truth about how confident a diagnostic is, whether it is suppressed, and
//! what file proves it, instead of presenting a heuristic, bridged, or
//! ungrounded convention as exact first-party truth.
//!
//! The packet is metadata only. Raw source bodies, raw manifests, repository URLs,
//! hostnames, secrets, and user-authored content never cross this boundary; rows
//! carry opaque refs, closed-vocabulary class tokens, short reviewable summaries,
//! structural locators, and export-safe chip labels. It references the upstream
//! template-manifest and framework-pack contracts by ref rather than embedding
//! them.
//!
//! [`ConventionDiagnosticPacket::apply_downgrade_automation`] narrows diagnostics
//! whose proving file went unavailable, whose confidence could not be verified,
//! whose analysis went stale, that were suppressed, or whose proof or upstream
//! dependency narrowed — withholding confident display and surfacing a downgrade
//! banner or suppression label rather than hiding the diagnostic, so CI or release
//! tooling narrows a stale or underqualified diagnostic before it is presented.
//!
//! The boundary schema is
//! [`schemas/templates/add-convention-diagnostics-confidence-labels-suppressibility-and-proving-file-disclosure.schema.json`](../../../../schemas/templates/add-convention-diagnostics-confidence-labels-suppressibility-and-proving-file-disclosure.schema.json).
//! The contract doc is
//! [`docs/frameworks/m5/add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure.md`](../../../../docs/frameworks/m5/add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure.md).
//! The protected fixture directory is
//! [`fixtures/templates/m5/add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure/`](../../../../fixtures/templates/m5/add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`ConventionDiagnosticPacket`].
pub const CONVENTION_DIAGNOSTIC_RECORD_KIND: &str =
    "convention_diagnostic_confidence_and_suppressibility_rows";

/// Schema version for convention-diagnostic packets.
pub const CONVENTION_DIAGNOSTIC_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const CONVENTION_DIAGNOSTIC_SCHEMA_REF: &str =
    "schemas/templates/add-convention-diagnostics-confidence-labels-suppressibility-and-proving-file-disclosure.schema.json";

/// Repo-relative path of the contract doc.
pub const CONVENTION_DIAGNOSTIC_DOC_REF: &str =
    "docs/frameworks/m5/add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure.md";

/// Repo-relative path of the upstream template-manifest contract this packet references.
pub const TEMPLATE_MANIFEST_CONTRACT_REF: &str =
    "schemas/templates/template_manifest_alpha.schema.json";

/// Repo-relative path of the upstream framework-pack contract this packet references.
pub const FRAMEWORK_PACK_CONTRACT_REF: &str =
    "schemas/templates/implement-framework-pack-headers-pack-version-or-freshness-chips-and-capability-or-downgrade-banners.schema.json";

/// Repo-relative path of the template-registry and scaffold contract doc.
pub const TEMPLATE_REGISTRY_CONTRACT_DOC_REF: &str =
    "docs/templates/template_registry_and_scaffold_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const CONVENTION_DIAGNOSTIC_FIXTURE_DIR: &str =
    "fixtures/templates/m5/add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure";

/// Repo-relative path of the checked support-export artifact.
pub const CONVENTION_DIAGNOSTIC_ARTIFACT_REF: &str =
    "artifacts/templates/m5/add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure/support_export.json";

/// Which kind of convention a diagnostic checks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConventionDiagnosticKind {
    /// A naming convention (file, type, or symbol name).
    NamingConvention,
    /// A file-location convention (where a file is expected to live).
    FileLocation,
    /// A required-registration convention (e.g. a route or module must be registered).
    RequiredRegistration,
    /// A configuration convention (a config key or value the pack expects).
    ConfigConvention,
    /// An API-usage convention (how a framework API is expected to be called).
    ApiUsageConvention,
}

impl ConventionDiagnosticKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NamingConvention => "naming_convention",
            Self::FileLocation => "file_location",
            Self::RequiredRegistration => "required_registration",
            Self::ConfigConvention => "config_convention",
            Self::ApiUsageConvention => "api_usage_convention",
        }
    }
}

/// Severity a convention diagnostic is presented at.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSeverity {
    /// Blocking error.
    Error,
    /// Non-blocking warning.
    Warning,
    /// Informational note.
    Info,
    /// Low-key hint.
    Hint,
}

impl DiagnosticSeverity {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
            Self::Info => "info",
            Self::Hint => "hint",
        }
    }
}

/// Confidence label a diagnostic is presented with — the central truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfidenceLabel {
    /// Proven exactly from a manifest or spec; first-party exact truth.
    Exact,
    /// Strong static evidence; high but not exact.
    High,
    /// Inferred from naming or layout conventions only.
    Heuristic,
    /// Weak signal; low confidence.
    Low,
    /// Confidence could not be determined.
    ConfidenceUnknown,
}

impl ConfidenceLabel {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::High => "high",
            Self::Heuristic => "heuristic",
            Self::Low => "low",
            Self::ConfidenceUnknown => "confidence_unknown",
        }
    }

    /// Whether this label is a confident claim (exact or high) that must be grounded.
    pub const fn is_confident(self) -> bool {
        matches!(self, Self::Exact | Self::High)
    }

    /// Whether this label must show a confidence or downgrade banner.
    pub const fn requires_banner(self) -> bool {
        matches!(self, Self::Heuristic | Self::Low | Self::ConfidenceUnknown)
    }

    /// Whether confidence is unresolved and must block confident display.
    pub const fn is_unknown(self) -> bool {
        matches!(self, Self::ConfidenceUnknown)
    }
}

/// Analysis freshness state for a diagnostic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticFreshnessClass {
    /// Verified fresh against the last analysis.
    Fresh,
    /// A newer analysis is available but the current result is still serviceable.
    RescanAvailable,
    /// Aging; a re-analysis is recommended.
    Aging,
    /// Stale; the analysis is past its freshness window.
    Stale,
    /// Freshness could not be determined.
    FreshnessUnknown,
}

impl DiagnosticFreshnessClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::RescanAvailable => "rescan_available",
            Self::Aging => "aging",
            Self::Stale => "stale",
            Self::FreshnessUnknown => "freshness_unknown",
        }
    }

    /// Whether this freshness state blocks presenting the diagnostic as current.
    pub const fn is_blocking(self) -> bool {
        matches!(self, Self::Stale | Self::FreshnessUnknown)
    }
}

/// Whether and how a diagnostic may be suppressed — keeps suppression honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuppressionClass {
    /// May be suppressed by the user; currently active (shown).
    Suppressible,
    /// Cannot be suppressed (hard requirement); currently active.
    NotSuppressible,
    /// Currently suppressed by an explicit user action.
    SuppressedByUser,
    /// Currently suppressed by project configuration or policy scope.
    SuppressedByScope,
    /// Suppressibility could not be determined.
    SuppressionUnknown,
}

impl SuppressionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Suppressible => "suppressible",
            Self::NotSuppressible => "not_suppressible",
            Self::SuppressedByUser => "suppressed_by_user",
            Self::SuppressedByScope => "suppressed_by_scope",
            Self::SuppressionUnknown => "suppression_unknown",
        }
    }

    /// Whether the diagnostic is currently suppressed and must be labeled, not hidden.
    pub const fn is_suppressed(self) -> bool {
        matches!(self, Self::SuppressedByUser | Self::SuppressedByScope)
    }
}

/// How the diagnostic's proving file or manifest is disclosed — the evidence cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvingDisclosureClass {
    /// A concrete proving file (and range) is disclosed.
    ProvingFileDisclosed,
    /// A generator or framework manifest grounds the convention exactly.
    ProvingManifestDisclosed,
    /// The convention is proven without a single file; none is needed.
    NoProvingFileNeeded,
    /// No proving file could be disclosed; the diagnostic must downgrade.
    ProvingFileUnavailable,
}

impl ProvingDisclosureClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProvingFileDisclosed => "proving_file_disclosed",
            Self::ProvingManifestDisclosed => "proving_manifest_disclosed",
            Self::NoProvingFileNeeded => "no_proving_file_needed",
            Self::ProvingFileUnavailable => "proving_file_unavailable",
        }
    }

    /// Whether a confident claim is grounded by a disclosed file or manifest.
    pub const fn is_grounded(self) -> bool {
        matches!(
            self,
            Self::ProvingFileDisclosed | Self::ProvingManifestDisclosed
        )
    }

    /// Whether no proving file could be disclosed.
    pub const fn is_unavailable(self) -> bool {
        matches!(self, Self::ProvingFileUnavailable)
    }
}

/// Support class on which a diagnostic may be presented — keeps bridge/heuristic honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticSupportClass {
    /// Exactly modeled first-party convention.
    ExactlyModeled,
    /// Experimental; may change without notice.
    Experimental,
    /// Bridge behavior: bridged from another tool rather than modeled natively.
    BridgeBehavior,
    /// Heuristic mapping; inferred rather than exactly modeled.
    HeuristicMapping,
    /// Explicitly unsupported.
    Unsupported,
    /// Support class unknown.
    SupportUnknown,
}

impl DiagnosticSupportClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactlyModeled => "exactly_modeled",
            Self::Experimental => "experimental",
            Self::BridgeBehavior => "bridge_behavior",
            Self::HeuristicMapping => "heuristic_mapping",
            Self::Unsupported => "unsupported",
            Self::SupportUnknown => "support_unknown",
        }
    }

    /// Whether this class is bridge or heuristic behavior that must be disclosed.
    ///
    /// Bridge and heuristic diagnostics must never be presented as exact first-party
    /// truth without a known issue, a support-class banner, and the matching
    /// disclosure trigger.
    pub const fn requires_disclosure(self) -> bool {
        matches!(self, Self::BridgeBehavior | Self::HeuristicMapping)
    }
}

/// Downgrade banner shown for a diagnostic — the explicit narrowing cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticDowngradeBannerClass {
    /// No downgrade banner is shown.
    NoBanner,
    /// Freshness banner: the analysis is aging, stale, or unverifiable.
    FreshnessBanner,
    /// Confidence banner: the diagnostic is heuristic, low, or unknown confidence.
    ConfidenceBanner,
    /// Support-class banner: bridge or heuristic behavior is disclosed.
    SupportClassBanner,
    /// Proving-file banner: no proving file could be disclosed.
    ProvingFileUnavailableBanner,
    /// Policy-block banner: the diagnostic is blocked by policy or trust.
    PolicyBlockBanner,
}

impl DiagnosticDowngradeBannerClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoBanner => "no_banner",
            Self::FreshnessBanner => "freshness_banner",
            Self::ConfidenceBanner => "confidence_banner",
            Self::SupportClassBanner => "support_class_banner",
            Self::ProvingFileUnavailableBanner => "proving_file_unavailable_banner",
            Self::PolicyBlockBanner => "policy_block_banner",
        }
    }

    /// Whether a banner is shown at all.
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::NoBanner)
    }

    /// Whether this banner hard-blocks confident display (not merely a soft cue).
    pub const fn is_hard_block(self) -> bool {
        matches!(
            self,
            Self::ProvingFileUnavailableBanner | Self::PolicyBlockBanner
        )
    }
}

/// Downgrade trigger that can narrow a convention diagnostic below its claimed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// No proving file could be disclosed.
    ProvingFileUnavailable,
    /// The diagnostic's confidence degraded below its declared label.
    ConfidenceDegraded,
    /// The analysis that produced the diagnostic went stale.
    AnalysisStale,
    /// Heuristic confidence or mapping is disclosed and held from exact-truth claims.
    HeuristicConfidenceDisclosed,
    /// Bridge behavior is disclosed and held from exact-truth claims.
    BridgeBehaviorDisclosed,
    /// The diagnostic was suppressed and is labeled rather than silently hidden.
    SuppressionApplied,
    /// A blocking known issue applies.
    KnownIssueBlocking,
    /// A validation bundle failed.
    ValidationFailed,
    /// An upstream dependency lane narrowed.
    UpstreamDependencyNarrowed,
}

impl DiagnosticDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ProvingFileUnavailable,
        Self::ConfidenceDegraded,
        Self::AnalysisStale,
        Self::HeuristicConfidenceDisclosed,
        Self::BridgeBehaviorDisclosed,
        Self::SuppressionApplied,
        Self::KnownIssueBlocking,
        Self::ValidationFailed,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ProvingFileUnavailable => "proving_file_unavailable",
            Self::ConfidenceDegraded => "confidence_degraded",
            Self::AnalysisStale => "analysis_stale",
            Self::HeuristicConfidenceDisclosed => "heuristic_confidence_disclosed",
            Self::BridgeBehaviorDisclosed => "bridge_behavior_disclosed",
            Self::SuppressionApplied => "suppression_applied",
            Self::KnownIssueBlocking => "known_issue_blocking",
            Self::ValidationFailed => "validation_failed",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must project a convention diagnostic's truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticConsumerSurface {
    /// Inline editor diagnostics.
    EditorDiagnostics,
    /// Problems / diagnostics panel.
    ProblemsPanel,
    /// Generation diff-review surface.
    DiffReview,
    /// Scaffold or app run surface.
    RunSurface,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Diagnostics or telemetry surface.
    Diagnostics,
    /// Help / About surface.
    HelpAbout,
}

impl DiagnosticConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::EditorDiagnostics,
        Self::ProblemsPanel,
        Self::DiffReview,
        Self::RunSurface,
        Self::CliHeadless,
        Self::SupportExport,
        Self::Diagnostics,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorDiagnostics => "editor_diagnostics",
            Self::ProblemsPanel => "problems_panel",
            Self::DiffReview => "diff_review",
            Self::RunSurface => "run_surface",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::Diagnostics => "diagnostics",
            Self::HelpAbout => "help_about",
        }
    }
}

/// One convention-diagnostic row: a diagnostic and its confidence, suppression,
/// proving-file, and banner truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConventionDiagnosticRow {
    /// Opaque stable row id.
    pub row_id: String,
    /// Which kind of convention this diagnostic checks.
    pub diagnostic_kind: ConventionDiagnosticKind,
    /// Opaque stable diagnostic id.
    pub diagnostic_id: String,
    /// Display label for the diagnostic.
    pub diagnostic_label: String,
    /// Structural locator for the convention rule.
    pub convention_locator: String,
    /// Opaque stable app / project id.
    pub app_id: String,
    /// Opaque framework-pack ref this diagnostic belongs to; a sentinel otherwise.
    pub framework_pack_ref: String,
    /// Short reviewable diagnostic message.
    pub message_summary: String,
    /// Severity the diagnostic is presented at.
    pub severity: DiagnosticSeverity,
    /// Confidence label the diagnostic is presented with.
    pub confidence_label: ConfidenceLabel,
    /// Short reviewable confidence summary.
    pub confidence_summary: String,
    /// Analysis freshness state.
    pub freshness_class: DiagnosticFreshnessClass,
    /// Export-safe freshness/analysis chip label.
    pub freshness_chip_label: String,
    /// RFC 3339 timestamp the analysis last ran.
    pub last_analyzed: String,
    /// Whether and how the diagnostic may be suppressed.
    pub suppression_class: SuppressionClass,
    /// Short reviewable suppression summary.
    pub suppression_summary: String,
    /// How the proving file or manifest is disclosed.
    pub proving_disclosure_class: ProvingDisclosureClass,
    /// Opaque proving-file or manifest locators that ground the convention.
    pub proving_file_refs: Vec<String>,
    /// Short reviewable proving-file summary.
    pub proving_summary: String,
    /// Support class on which the diagnostic may be presented.
    pub support_class: DiagnosticSupportClass,
    /// Downgrade banner shown for this diagnostic.
    pub downgrade_banner_class: DiagnosticDowngradeBannerClass,
    /// Opaque known-issue refs disclosed before the diagnostic is presented.
    pub known_issue_refs: Vec<String>,
    /// Whether this diagnostic is admitted to be presented as confident active truth.
    pub admitted_for_display: bool,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<DiagnosticDowngradeTrigger>,
    /// Consumer surfaces that must project this row.
    pub consumer_surfaces: Vec<DiagnosticConsumerSurface>,
}

impl ConventionDiagnosticRow {
    /// Whether this row is structurally blocked from confident display.
    pub const fn is_blocked(&self) -> bool {
        self.freshness_class.is_blocking()
            || self.confidence_label.is_unknown()
            || self.proving_disclosure_class.is_unavailable()
            || self.downgrade_banner_class.is_hard_block()
    }
}

/// Review block asserting the lane's honesty invariants.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConventionDiagnosticReview {
    /// A confidence label is shown for every diagnostic.
    pub confidence_label_shown_for_every_diagnostic: bool,
    /// A proving file or manifest is disclosed, or the diagnostic marks none needed.
    pub proving_file_disclosed_or_marked_not_needed: bool,
    /// A confident (exact or high) claim always discloses a proving file or manifest.
    pub confident_claim_discloses_proving_file: bool,
    /// A confidence banner is shown whenever confidence is not exact or high.
    pub confidence_banner_shown_when_not_confident: bool,
    /// Suppressibility is disclosed for every diagnostic.
    pub suppressibility_disclosed_for_every_diagnostic: bool,
    /// A suppressed diagnostic is labeled rather than silently hidden.
    pub suppressed_diagnostic_labeled_not_hidden: bool,
    /// A heuristic or bridged diagnostic is never presented as exact truth.
    pub heuristic_or_bridge_never_presented_as_exact_truth: bool,
    /// An unavailable proving file blocks any confident claim.
    pub proving_file_unavailable_blocks_confident_claim: bool,
    /// A stale analysis is never presented as current.
    pub stale_analysis_not_presented_as_current: bool,
    /// The support class is visible before a diagnostic is presented.
    pub support_class_visible_before_display: bool,
    /// Known issues are disclosed before a diagnostic is presented.
    pub known_issues_disclosed_before_display: bool,
    /// No raw source bodies or URLs cross the export boundary.
    pub no_raw_source_bodies_or_urls_in_export: bool,
    /// Downgrade narrows the diagnostic's claim rather than hiding the diagnostic.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified rows automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConventionDiagnosticConsumerProjection {
    /// Editor diagnostics show the confidence label.
    pub editor_shows_confidence_label: bool,
    /// Problems panel shows the suppressibility state.
    pub problems_panel_shows_suppressibility: bool,
    /// Diff-review shows the proving file.
    pub diff_review_shows_proving_file: bool,
    /// Run surface shows the confidence banner.
    pub run_surface_shows_confidence_banner: bool,
    /// CLI / headless shows diagnostic rows.
    pub cli_headless_shows_diagnostic_rows: bool,
    /// Support export shows diagnostic rows.
    pub support_export_shows_diagnostic_rows: bool,
    /// Diagnostics shows confidence and proving state.
    pub diagnostics_shows_confidence_and_proving_state: bool,
    /// Suppressed diagnostics are visibly labeled rather than hidden.
    pub suppressed_diagnostics_labeled_not_hidden: bool,
    /// Blocked diagnostics are visibly labeled rather than hidden.
    pub blocked_diagnostics_labeled_not_hidden: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConventionDiagnosticProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the affected rows.
    pub auto_narrow_on_stale: bool,
}

/// Per-row observation fed to [`ConventionDiagnosticPacket::apply_downgrade_automation`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConventionDiagnosticRowObservation {
    /// Row id the observation applies to.
    pub row_id: String,
    /// True when the diagnostic's proving file currently resolves.
    pub proving_file_available: bool,
    /// True when the diagnostic's confidence currently verifies.
    pub confidence_verified: bool,
    /// True when the analysis is currently fresh.
    pub analysis_fresh: bool,
    /// True when the diagnostic is currently suppressed.
    pub suppression_active: bool,
    /// True when the row's proof is within its freshness SLO.
    pub proof_fresh: bool,
    /// True when an upstream dependency of the row narrowed.
    pub upstream_narrowed: bool,
}

/// Constructor input for [`ConventionDiagnosticPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConventionDiagnosticPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Diagnostic rows.
    pub rows: Vec<ConventionDiagnosticRow>,
    /// Review block.
    pub review: ConventionDiagnosticReview,
    /// Consumer projection block.
    pub consumer_projection: ConventionDiagnosticConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ConventionDiagnosticProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe convention-diagnostic packet with confidence, suppressibility, and
/// proving-file disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConventionDiagnosticPacket {
    /// Record kind; must equal [`CONVENTION_DIAGNOSTIC_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`CONVENTION_DIAGNOSTIC_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Diagnostic rows.
    pub rows: Vec<ConventionDiagnosticRow>,
    /// Review block.
    pub review: ConventionDiagnosticReview,
    /// Consumer projection block.
    pub consumer_projection: ConventionDiagnosticConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ConventionDiagnosticProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ConventionDiagnosticPacket {
    /// Builds a convention-diagnostic packet from stable-row input.
    pub fn new(input: ConventionDiagnosticPacketInput) -> Self {
        Self {
            record_kind: CONVENTION_DIAGNOSTIC_RECORD_KIND.to_owned(),
            schema_version: CONVENTION_DIAGNOSTIC_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            rows: input.rows,
            review: input.review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Narrows diagnostics whose proving file went unavailable, whose confidence
    /// could not be verified, whose analysis went stale, that were suppressed, or
    /// whose proof or upstream narrowed.
    ///
    /// An unavailable proving file is the hardest block: the proving disclosure is
    /// marked unavailable, the confidence is narrowed to unknown, its file refs are
    /// cleared, the proving-file banner is raised, and the diagnostic loses confident
    /// display. An unverified confidence narrows confidence to unknown and raises a
    /// confidence banner. A stale analysis narrows freshness to stale and raises a
    /// freshness banner. A newly active suppression labels the row suppressed and
    /// withdraws active display. Stale proof or a narrowed upstream withholds display
    /// until evidence refreshes. A raised banner is never lowered. Rows without a
    /// matching observation are left unchanged.
    pub fn apply_downgrade_automation(
        &mut self,
        observations: &[ConventionDiagnosticRowObservation],
    ) {
        for row in &mut self.rows {
            let Some(observation) = observations.iter().find(|obs| obs.row_id == row.row_id) else {
                continue;
            };

            if !observation.proving_file_available {
                row.proving_disclosure_class = ProvingDisclosureClass::ProvingFileUnavailable;
                row.confidence_label = ConfidenceLabel::ConfidenceUnknown;
                row.proving_file_refs.clear();
                row.downgrade_banner_class =
                    DiagnosticDowngradeBannerClass::ProvingFileUnavailableBanner;
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    DiagnosticDowngradeTrigger::ProvingFileUnavailable,
                );
                continue;
            }

            if !observation.confidence_verified {
                row.confidence_label = ConfidenceLabel::ConfidenceUnknown;
                raise_banner(row, DiagnosticDowngradeBannerClass::ConfidenceBanner);
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    DiagnosticDowngradeTrigger::ConfidenceDegraded,
                );
            }

            if !observation.analysis_fresh {
                if !row.freshness_class.is_blocking() {
                    row.freshness_class = DiagnosticFreshnessClass::Stale;
                }
                raise_banner(row, DiagnosticDowngradeBannerClass::FreshnessBanner);
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    DiagnosticDowngradeTrigger::AnalysisStale,
                );
            }

            if observation.suppression_active && !row.suppression_class.is_suppressed() {
                row.suppression_class = SuppressionClass::SuppressedByScope;
                row.admitted_for_display = false;
                push_unique_trigger(
                    &mut row.downgrade_triggers,
                    DiagnosticDowngradeTrigger::SuppressionApplied,
                );
            }

            if (!observation.proof_fresh || observation.upstream_narrowed)
                && row.admitted_for_display
            {
                row.admitted_for_display = false;
                let trigger = if observation.proof_fresh {
                    DiagnosticDowngradeTrigger::UpstreamDependencyNarrowed
                } else {
                    DiagnosticDowngradeTrigger::ProofStale
                };
                push_unique_trigger(&mut row.downgrade_triggers, trigger);
            }
        }
    }

    /// Validates the convention-diagnostic invariants.
    pub fn validate(&self) -> Vec<ConventionDiagnosticViolation> {
        let mut violations = Vec::new();

        if self.record_kind != CONVENTION_DIAGNOSTIC_RECORD_KIND {
            violations.push(ConventionDiagnosticViolation::WrongRecordKind);
        }
        if self.schema_version != CONVENTION_DIAGNOSTIC_SCHEMA_VERSION {
            violations.push(ConventionDiagnosticViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ConventionDiagnosticViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("convention-diagnostic packet serializes"),
        ) {
            violations.push(ConventionDiagnosticViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("convention-diagnostic packet serializes")
    }

    /// Rows currently admitted to be presented as confident active truth.
    pub fn admitted_rows(&self) -> impl Iterator<Item = &ConventionDiagnosticRow> {
        self.rows.iter().filter(|row| row.admitted_for_display)
    }

    /// Deterministic Markdown summary for diagnostics, support, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let admitted = self.admitted_rows().count();
        let mut out = String::new();
        out.push_str(
            "# Convention Diagnostics with Confidence Labels, Suppressibility, and Proving-File Disclosure\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Rows: {} ({} admitted for display)\n",
            self.rows.len(),
            admitted
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** `{}` ({}): {} / {}\n",
                row.diagnostic_label,
                row.convention_locator,
                row.diagnostic_kind.as_str(),
                row.confidence_label.as_str(),
                row.support_class.as_str()
            ));
            out.push_str(&format!("  - Message: {}\n", row.message_summary));
            out.push_str(&format!(
                "  - Confidence: {} ({})\n",
                row.confidence_summary,
                row.confidence_label.as_str()
            ));
            out.push_str(&format!(
                "  - Freshness chip: {} ({})\n",
                row.freshness_chip_label,
                row.freshness_class.as_str()
            ));
            out.push_str(&format!(
                "  - Suppression: {} ({})\n",
                row.suppression_summary,
                row.suppression_class.as_str()
            ));
            out.push_str(&format!(
                "  - Proving file: {} ({})\n",
                row.proving_summary,
                row.proving_disclosure_class.as_str()
            ));
            out.push_str(&format!(
                "  - Banner: {}\n",
                row.downgrade_banner_class.as_str()
            ));
            out.push_str(&format!("  - Displayed: {}\n", row.admitted_for_display));
        }
        out
    }
}

/// Errors emitted when reading the checked-in convention-diagnostic export.
#[derive(Debug)]
pub enum ConventionDiagnosticArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ConventionDiagnosticViolation>),
}

impl fmt::Display for ConventionDiagnosticArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "convention-diagnostic export parse failed: {error}"
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
                    "convention-diagnostic export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ConventionDiagnosticArtifactError {}

/// Validation failures emitted by [`ConventionDiagnosticPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConventionDiagnosticViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The packet carries no rows.
    RowsEmpty,
    /// A row is incomplete.
    RowIncomplete,
    /// A non-confident row is missing its confidence or downgrade banner.
    ConfidenceBannerMissing,
    /// A confident (exact or high) row does not disclose a proving file or manifest.
    ProvingFileUndisclosedForConfidentClaim,
    /// A grounded proving disclosure carries no proving-file refs.
    ProvingFileRefsMissing,
    /// A proving-file-unavailable row is missing its proving-file banner.
    ProvingFileUnavailableBannerMissing,
    /// A proving-file-unavailable row did not narrow confidence to unknown.
    ProvingFileUnavailableConfidenceNotNarrowed,
    /// A bridge/heuristic row is missing a known issue, banner, or disclosure trigger.
    SupportClassUndisclosed,
    /// A stale or unknown-freshness row is missing a downgrade banner.
    FreshnessBannerMissing,
    /// A suppressed row is still admitted or missing its suppression trigger.
    SuppressionUndisclosed,
    /// A blocked row is still admitted for confident display.
    BlockedDisplayAdmitted,
    /// A row has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Review block does not satisfy required invariants.
    ReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ConventionDiagnosticViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RowsEmpty => "rows_empty",
            Self::RowIncomplete => "row_incomplete",
            Self::ConfidenceBannerMissing => "confidence_banner_missing",
            Self::ProvingFileUndisclosedForConfidentClaim => {
                "proving_file_undisclosed_for_confident_claim"
            }
            Self::ProvingFileRefsMissing => "proving_file_refs_missing",
            Self::ProvingFileUnavailableBannerMissing => "proving_file_unavailable_banner_missing",
            Self::ProvingFileUnavailableConfidenceNotNarrowed => {
                "proving_file_unavailable_confidence_not_narrowed"
            }
            Self::SupportClassUndisclosed => "support_class_undisclosed",
            Self::FreshnessBannerMissing => "freshness_banner_missing",
            Self::SuppressionUndisclosed => "suppression_undisclosed",
            Self::BlockedDisplayAdmitted => "blocked_display_admitted",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ReviewIncomplete => "review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in convention-diagnostic export.
///
/// This is the first real consumer of the convention-diagnostic lane: an editor
/// diagnostics, problems-panel, run, diagnostics, or support-export surface calls
/// it to ingest the canonical packet rather than cloning status text.
///
/// # Errors
///
/// Returns [`ConventionDiagnosticArtifactError`] when the checked-in support export
/// fails to parse or fails validation.
pub fn current_convention_diagnostic_export(
) -> Result<ConventionDiagnosticPacket, ConventionDiagnosticArtifactError> {
    let packet: ConventionDiagnosticPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/templates/m5/add_convention_diagnostics_confidence_labels_suppressibility_and_proving_file_disclosure/support_export.json"
    )))
    .map_err(ConventionDiagnosticArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ConventionDiagnosticArtifactError::Validation(violations))
    }
}

/// Canonical review block with every invariant satisfied.
pub fn canonical_review() -> ConventionDiagnosticReview {
    ConventionDiagnosticReview {
        confidence_label_shown_for_every_diagnostic: true,
        proving_file_disclosed_or_marked_not_needed: true,
        confident_claim_discloses_proving_file: true,
        confidence_banner_shown_when_not_confident: true,
        suppressibility_disclosed_for_every_diagnostic: true,
        suppressed_diagnostic_labeled_not_hidden: true,
        heuristic_or_bridge_never_presented_as_exact_truth: true,
        proving_file_unavailable_blocks_confident_claim: true,
        stale_analysis_not_presented_as_current: true,
        support_class_visible_before_display: true,
        known_issues_disclosed_before_display: true,
        no_raw_source_bodies_or_urls_in_export: true,
        downgrade_narrows_instead_of_hides: true,
        stale_or_underqualified_blocks_promotion: true,
    }
}

/// Canonical consumer projection block with every surface projecting diagnostic truth.
pub fn canonical_consumer_projection() -> ConventionDiagnosticConsumerProjection {
    ConventionDiagnosticConsumerProjection {
        editor_shows_confidence_label: true,
        problems_panel_shows_suppressibility: true,
        diff_review_shows_proving_file: true,
        run_surface_shows_confidence_banner: true,
        cli_headless_shows_diagnostic_rows: true,
        support_export_shows_diagnostic_rows: true,
        diagnostics_shows_confidence_and_proving_state: true,
        suppressed_diagnostics_labeled_not_hidden: true,
        blocked_diagnostics_labeled_not_hidden: true,
    }
}

/// Canonical source contract refs that every convention-diagnostic export must carry.
pub fn canonical_source_contract_refs() -> Vec<String> {
    vec![
        CONVENTION_DIAGNOSTIC_SCHEMA_REF.to_owned(),
        CONVENTION_DIAGNOSTIC_DOC_REF.to_owned(),
        TEMPLATE_MANIFEST_CONTRACT_REF.to_owned(),
        FRAMEWORK_PACK_CONTRACT_REF.to_owned(),
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF.to_owned(),
    ]
}

/// Builds the canonical convention-diagnostic packet from stable-row truth.
///
/// The rows mirror the checked-in support export and cover the confidence,
/// suppression, and proving-file spectrum: an exact convention proven from a
/// manifest and shown active with no banner, a high-confidence naming convention
/// grounded by a proving file, a heuristic convention disclosed and held behind its
/// confidence and support-class banner, a high-confidence diagnostic the user
/// suppressed and labeled rather than hidden, a diagnostic whose proving file went
/// unavailable and is blocked rather than presented, and a low-confidence
/// diagnostic bridged from an external linter and held from exact-truth claims.
pub fn canonical_convention_diagnostics(
    packet_id: String,
    packet_label: String,
    minted_at: String,
    proof_freshness: ConventionDiagnosticProofFreshness,
) -> ConventionDiagnosticPacket {
    ConventionDiagnosticPacket::new(ConventionDiagnosticPacketInput {
        packet_id,
        packet_label,
        rows: canonical_rows(),
        review: canonical_review(),
        consumer_projection: canonical_consumer_projection(),
        proof_freshness,
        source_contract_refs: canonical_source_contract_refs(),
        redaction_class_token: "metadata_safe_default".to_owned(),
        minted_at,
    })
}

/// Canonical rows that match the checked-in support export.
pub fn canonical_rows() -> Vec<ConventionDiagnosticRow> {
    use DiagnosticConsumerSurface as Surface;
    use DiagnosticDowngradeTrigger as Trigger;

    vec![
        ConventionDiagnosticRow {
            row_id: "convention-diagnostic-row:file_location.controllers.exact:2026.06".to_owned(),
            diagnostic_kind: ConventionDiagnosticKind::FileLocation,
            diagnostic_id: "diagnostic:file_location.controllers".to_owned(),
            diagnostic_label: "Controller file location".to_owned(),
            convention_locator: "rule:framework_pack/controllers/file_location".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:rust.axum@1.8.0".to_owned(),
            message_summary: "Controllers are expected under the managed controllers directory; this file matches the convention exactly".to_owned(),
            severity: DiagnosticSeverity::Warning,
            confidence_label: ConfidenceLabel::Exact,
            confidence_summary: "Proven exactly from the framework-pack convention manifest; the expected location is declared, not inferred".to_owned(),
            freshness_class: DiagnosticFreshnessClass::Fresh,
            freshness_chip_label: "analyzed · fresh".to_owned(),
            last_analyzed: "2026-06-08T00:00:00Z".to_owned(),
            suppression_class: SuppressionClass::NotSuppressible,
            suppression_summary: "Hard framework requirement; cannot be suppressed because the pack relies on the managed location".to_owned(),
            proving_disclosure_class: ProvingDisclosureClass::ProvingManifestDisclosed,
            proving_file_refs: vec![
                "manifest:framework_pack/conventions/controllers.toml#L12-L18".to_owned(),
            ],
            proving_summary: "Grounded by the convention manifest entry that declares the expected controllers directory".to_owned(),
            support_class: DiagnosticSupportClass::ExactlyModeled,
            downgrade_banner_class: DiagnosticDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_display: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProvingFileUnavailable,
                Trigger::ConfidenceDegraded,
                Trigger::AnalysisStale,
            ],
            consumer_surfaces: vec![
                Surface::EditorDiagnostics,
                Surface::ProblemsPanel,
                Surface::SupportExport,
            ],
        },
        ConventionDiagnosticRow {
            row_id: "convention-diagnostic-row:naming.model.high:2026.06".to_owned(),
            diagnostic_kind: ConventionDiagnosticKind::NamingConvention,
            diagnostic_id: "diagnostic:naming.model_struct".to_owned(),
            diagnostic_label: "Model type naming".to_owned(),
            convention_locator: "rule:framework_pack/models/type_naming".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:rust.axum@1.8.0".to_owned(),
            message_summary: "Model types are expected in PascalCase singular; this type matches with high confidence".to_owned(),
            severity: DiagnosticSeverity::Info,
            confidence_label: ConfidenceLabel::High,
            confidence_summary: "Strong static evidence from the analyzed type declaration; high but not manifest-exact".to_owned(),
            freshness_class: DiagnosticFreshnessClass::Fresh,
            freshness_chip_label: "analyzed · fresh".to_owned(),
            last_analyzed: "2026-06-08T00:00:00Z".to_owned(),
            suppression_class: SuppressionClass::Suppressible,
            suppression_summary: "May be suppressed per-file or per-project; currently active and shown".to_owned(),
            proving_disclosure_class: ProvingDisclosureClass::ProvingFileDisclosed,
            proving_file_refs: vec!["file:src/models/user.rs#L1-L3".to_owned()],
            proving_summary: "Grounded by the analyzed model type declaration that the convention matches".to_owned(),
            support_class: DiagnosticSupportClass::ExactlyModeled,
            downgrade_banner_class: DiagnosticDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_display: true,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProvingFileUnavailable,
                Trigger::ConfidenceDegraded,
                Trigger::AnalysisStale,
            ],
            consumer_surfaces: vec![
                Surface::EditorDiagnostics,
                Surface::ProblemsPanel,
                Surface::DiffReview,
                Surface::SupportExport,
            ],
        },
        ConventionDiagnosticRow {
            row_id: "convention-diagnostic-row:api_usage.legacy.heuristic:2026.05".to_owned(),
            diagnostic_kind: ConventionDiagnosticKind::ApiUsageConvention,
            diagnostic_id: "diagnostic:api_usage.legacy_payment".to_owned(),
            diagnostic_label: "Legacy payment API usage".to_owned(),
            convention_locator: "rule:framework_pack/api/payment_usage".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:rust.axum@1.8.0".to_owned(),
            message_summary: "This call may use a deprecated payment API; the match is inferred from naming and call shape, not modeled exactly".to_owned(),
            severity: DiagnosticSeverity::Warning,
            confidence_label: ConfidenceLabel::Heuristic,
            confidence_summary: "Inferred from naming and call-shape conventions only; this is a heuristic match, not exact modeling, and is disclosed by the confidence banner".to_owned(),
            freshness_class: DiagnosticFreshnessClass::Aging,
            freshness_chip_label: "analyzed · aging".to_owned(),
            last_analyzed: "2026-05-20T00:00:00Z".to_owned(),
            suppression_class: SuppressionClass::Suppressible,
            suppression_summary: "May be suppressed per-file or per-project; held from confident display while heuristic".to_owned(),
            proving_disclosure_class: ProvingDisclosureClass::ProvingFileDisclosed,
            proving_file_refs: vec!["file:src/services/legacy_payment.rs#L40-L55".to_owned()],
            proving_summary: "Grounded by the analyzed call site, but the deprecation match itself is heuristic".to_owned(),
            support_class: DiagnosticSupportClass::HeuristicMapping,
            downgrade_banner_class: DiagnosticDowngradeBannerClass::SupportClassBanner,
            known_issue_refs: vec![
                "known-issue:convention_diagnostics:heuristic_api_usage_inference".to_owned(),
            ],
            admitted_for_display: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::HeuristicConfidenceDisclosed,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::EditorDiagnostics,
                Surface::ProblemsPanel,
                Surface::DiffReview,
                Surface::SupportExport,
                Surface::HelpAbout,
            ],
        },
        ConventionDiagnosticRow {
            row_id: "convention-diagnostic-row:required_registration.route.suppressed:2026.06".to_owned(),
            diagnostic_kind: ConventionDiagnosticKind::RequiredRegistration,
            diagnostic_id: "diagnostic:required_registration.route".to_owned(),
            diagnostic_label: "Route registration".to_owned(),
            convention_locator: "rule:framework_pack/routes/registration".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:rust.axum@1.8.0".to_owned(),
            message_summary: "A handler is expected to be registered in the router; the user has suppressed this diagnostic for this route".to_owned(),
            severity: DiagnosticSeverity::Warning,
            confidence_label: ConfidenceLabel::High,
            confidence_summary: "Strong static evidence the handler is unregistered; high confidence, but the user has chosen to suppress the diagnostic".to_owned(),
            freshness_class: DiagnosticFreshnessClass::Fresh,
            freshness_chip_label: "analyzed · fresh".to_owned(),
            last_analyzed: "2026-06-08T00:00:00Z".to_owned(),
            suppression_class: SuppressionClass::SuppressedByUser,
            suppression_summary: "Suppressed by an explicit user action for this route; labeled as suppressed rather than silently hidden".to_owned(),
            proving_disclosure_class: ProvingDisclosureClass::ProvingFileDisclosed,
            proving_file_refs: vec!["file:src/routes/mod.rs#L20-L22".to_owned()],
            proving_summary: "Grounded by the router module that lacks the expected registration".to_owned(),
            support_class: DiagnosticSupportClass::ExactlyModeled,
            downgrade_banner_class: DiagnosticDowngradeBannerClass::NoBanner,
            known_issue_refs: vec![],
            admitted_for_display: false,
            downgrade_triggers: vec![Trigger::ProofStale, Trigger::SuppressionApplied],
            consumer_surfaces: vec![
                Surface::ProblemsPanel,
                Surface::SupportExport,
                Surface::Diagnostics,
            ],
        },
        ConventionDiagnosticRow {
            row_id: "convention-diagnostic-row:config_convention.proving_unavailable:2026.04".to_owned(),
            diagnostic_kind: ConventionDiagnosticKind::ConfigConvention,
            diagnostic_id: "diagnostic:config_convention.database_url".to_owned(),
            diagnostic_label: "Database config convention".to_owned(),
            convention_locator: "rule:framework_pack/config/database".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:rust.axum@1.8.0".to_owned(),
            message_summary: "A config convention was asserted, but the proving file could not be disclosed, so the diagnostic is blocked rather than presented".to_owned(),
            severity: DiagnosticSeverity::Warning,
            confidence_label: ConfidenceLabel::ConfidenceUnknown,
            confidence_summary: "Confidence could not be determined because no proving file is available; the diagnostic is labeled unknown and blocked".to_owned(),
            freshness_class: DiagnosticFreshnessClass::Fresh,
            freshness_chip_label: "analyzed · fresh".to_owned(),
            last_analyzed: "2026-04-10T00:00:00Z".to_owned(),
            suppression_class: SuppressionClass::Suppressible,
            suppression_summary: "May be suppressed per-project, but is blocked from confident display while no proving file is available".to_owned(),
            proving_disclosure_class: ProvingDisclosureClass::ProvingFileUnavailable,
            proving_file_refs: vec![],
            proving_summary: "No proving file or manifest could be disclosed for this convention; the diagnostic is held from any confident claim".to_owned(),
            support_class: DiagnosticSupportClass::SupportUnknown,
            downgrade_banner_class: DiagnosticDowngradeBannerClass::ProvingFileUnavailableBanner,
            known_issue_refs: vec![
                "known-issue:convention_diagnostics:proving_file_unavailable".to_owned(),
            ],
            admitted_for_display: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::ProvingFileUnavailable,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::ProblemsPanel,
                Surface::SupportExport,
                Surface::Diagnostics,
            ],
        },
        ConventionDiagnosticRow {
            row_id: "convention-diagnostic-row:naming.bridge.external_linter.low:2026.06".to_owned(),
            diagnostic_kind: ConventionDiagnosticKind::NamingConvention,
            diagnostic_id: "diagnostic:naming.helper_bridge".to_owned(),
            diagnostic_label: "Helper naming (bridged)".to_owned(),
            convention_locator: "rule:framework_pack/helpers/naming_bridge".to_owned(),
            app_id: "app:rust.axum.sample_service".to_owned(),
            framework_pack_ref: "framework-pack:rust.axum@1.8.0".to_owned(),
            message_summary: "A naming concern bridged from an external linter; bridge behavior is disclosed and held from exact first-party truth".to_owned(),
            severity: DiagnosticSeverity::Hint,
            confidence_label: ConfidenceLabel::Low,
            confidence_summary: "Low confidence: the finding is bridged from an external linter rather than modeled natively, and is disclosed by the support-class banner".to_owned(),
            freshness_class: DiagnosticFreshnessClass::RescanAvailable,
            freshness_chip_label: "analyzed · rescan available".to_owned(),
            last_analyzed: "2026-06-06T00:00:00Z".to_owned(),
            suppression_class: SuppressionClass::Suppressible,
            suppression_summary: "May be suppressed per-file or per-project; held from confident display while bridged".to_owned(),
            proving_disclosure_class: ProvingDisclosureClass::ProvingFileDisclosed,
            proving_file_refs: vec!["file:src/legacy/helper.rs#L5".to_owned()],
            proving_summary: "Grounded by the bridged finding's reported location, but the finding is bridged, not modeled".to_owned(),
            support_class: DiagnosticSupportClass::BridgeBehavior,
            downgrade_banner_class: DiagnosticDowngradeBannerClass::SupportClassBanner,
            known_issue_refs: vec![
                "known-issue:convention_diagnostics:external_linter_bridge".to_owned(),
            ],
            admitted_for_display: false,
            downgrade_triggers: vec![
                Trigger::ProofStale,
                Trigger::BridgeBehaviorDisclosed,
                Trigger::KnownIssueBlocking,
            ],
            consumer_surfaces: vec![
                Surface::EditorDiagnostics,
                Surface::ProblemsPanel,
                Surface::SupportExport,
                Surface::HelpAbout,
            ],
        },
    ]
}

fn validate_source_contracts(
    packet: &ConventionDiagnosticPacket,
    violations: &mut Vec<ConventionDiagnosticViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        CONVENTION_DIAGNOSTIC_SCHEMA_REF,
        CONVENTION_DIAGNOSTIC_DOC_REF,
        TEMPLATE_MANIFEST_CONTRACT_REF,
        FRAMEWORK_PACK_CONTRACT_REF,
        TEMPLATE_REGISTRY_CONTRACT_DOC_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ConventionDiagnosticViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &ConventionDiagnosticPacket,
    violations: &mut Vec<ConventionDiagnosticViolation>,
) {
    if packet.rows.is_empty() {
        violations.push(ConventionDiagnosticViolation::RowsEmpty);
        return;
    }

    for row in &packet.rows {
        if row.row_id.trim().is_empty()
            || row.diagnostic_id.trim().is_empty()
            || row.diagnostic_label.trim().is_empty()
            || row.convention_locator.trim().is_empty()
            || row.app_id.trim().is_empty()
            || row.framework_pack_ref.trim().is_empty()
            || row.message_summary.trim().is_empty()
            || row.confidence_summary.trim().is_empty()
            || row.freshness_chip_label.trim().is_empty()
            || row.last_analyzed.trim().is_empty()
            || row.suppression_summary.trim().is_empty()
            || row.proving_summary.trim().is_empty()
        {
            violations.push(ConventionDiagnosticViolation::RowIncomplete);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(ConventionDiagnosticViolation::DowngradeTriggersMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(ConventionDiagnosticViolation::ConsumerSurfacesMissing);
        }

        validate_row_banners(row, violations);
    }
}

fn validate_row_banners(
    row: &ConventionDiagnosticRow,
    violations: &mut Vec<ConventionDiagnosticViolation>,
) {
    // A non-confident confidence label must show a confidence or downgrade banner.
    if row.confidence_label.requires_banner() && !row.downgrade_banner_class.is_present() {
        violations.push(ConventionDiagnosticViolation::ConfidenceBannerMissing);
    }

    // A confident (exact or high) claim must disclose a proving file or manifest.
    if row.confidence_label.is_confident() && !row.proving_disclosure_class.is_grounded() {
        violations.push(ConventionDiagnosticViolation::ProvingFileUndisclosedForConfidentClaim);
    }

    // A grounded proving disclosure must carry at least one proving-file ref.
    if row.proving_disclosure_class.is_grounded() && row.proving_file_refs.is_empty() {
        violations.push(ConventionDiagnosticViolation::ProvingFileRefsMissing);
    }

    // An unavailable proving file must raise the proving-file banner and narrow
    // confidence to unknown, so an ungrounded diagnostic is never presented.
    if row.proving_disclosure_class.is_unavailable() {
        if row.downgrade_banner_class
            != DiagnosticDowngradeBannerClass::ProvingFileUnavailableBanner
        {
            violations.push(ConventionDiagnosticViolation::ProvingFileUnavailableBannerMissing);
        }
        if !row.confidence_label.is_unknown() {
            violations
                .push(ConventionDiagnosticViolation::ProvingFileUnavailableConfidenceNotNarrowed);
        }
    }

    // Bridge/heuristic diagnostics must disclose a known issue, a banner, and the matching trigger.
    if row.support_class.requires_disclosure() {
        let matching_trigger = match row.support_class {
            DiagnosticSupportClass::BridgeBehavior => {
                DiagnosticDowngradeTrigger::BridgeBehaviorDisclosed
            }
            _ => DiagnosticDowngradeTrigger::HeuristicConfidenceDisclosed,
        };
        if row.known_issue_refs.is_empty()
            || !row.downgrade_banner_class.is_present()
            || !row.downgrade_triggers.contains(&matching_trigger)
        {
            violations.push(ConventionDiagnosticViolation::SupportClassUndisclosed);
        }
    }

    // A stale or unknown-freshness diagnostic must show a downgrade banner.
    if row.freshness_class.is_blocking() && !row.downgrade_banner_class.is_present() {
        violations.push(ConventionDiagnosticViolation::FreshnessBannerMissing);
    }

    // A suppressed diagnostic must be withdrawn from active display and labeled with
    // the suppression-applied trigger rather than silently hidden.
    if row.suppression_class.is_suppressed()
        && (row.admitted_for_display
            || !row
                .downgrade_triggers
                .contains(&DiagnosticDowngradeTrigger::SuppressionApplied))
    {
        violations.push(ConventionDiagnosticViolation::SuppressionUndisclosed);
    }

    // A blocked diagnostic cannot be admitted for confident display.
    if row.is_blocked() && row.admitted_for_display {
        violations.push(ConventionDiagnosticViolation::BlockedDisplayAdmitted);
    }
}

fn validate_review(
    packet: &ConventionDiagnosticPacket,
    violations: &mut Vec<ConventionDiagnosticViolation>,
) {
    let review = &packet.review;
    for ok in [
        review.confidence_label_shown_for_every_diagnostic,
        review.proving_file_disclosed_or_marked_not_needed,
        review.confident_claim_discloses_proving_file,
        review.confidence_banner_shown_when_not_confident,
        review.suppressibility_disclosed_for_every_diagnostic,
        review.suppressed_diagnostic_labeled_not_hidden,
        review.heuristic_or_bridge_never_presented_as_exact_truth,
        review.proving_file_unavailable_blocks_confident_claim,
        review.stale_analysis_not_presented_as_current,
        review.support_class_visible_before_display,
        review.known_issues_disclosed_before_display,
        review.no_raw_source_bodies_or_urls_in_export,
        review.downgrade_narrows_instead_of_hides,
        review.stale_or_underqualified_blocks_promotion,
    ] {
        if !ok {
            violations.push(ConventionDiagnosticViolation::ReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &ConventionDiagnosticPacket,
    violations: &mut Vec<ConventionDiagnosticViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.editor_shows_confidence_label,
        projection.problems_panel_shows_suppressibility,
        projection.diff_review_shows_proving_file,
        projection.run_surface_shows_confidence_banner,
        projection.cli_headless_shows_diagnostic_rows,
        projection.support_export_shows_diagnostic_rows,
        projection.diagnostics_shows_confidence_and_proving_state,
        projection.suppressed_diagnostics_labeled_not_hidden,
        projection.blocked_diagnostics_labeled_not_hidden,
    ] {
        if !ok {
            violations.push(ConventionDiagnosticViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &ConventionDiagnosticPacket,
    violations: &mut Vec<ConventionDiagnosticViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(ConventionDiagnosticViolation::ProofFreshnessIncomplete);
    }
}

/// Raises the row's downgrade banner only when none is currently shown, so an
/// already-raised banner is never lowered to a softer cue.
fn raise_banner(row: &mut ConventionDiagnosticRow, banner: DiagnosticDowngradeBannerClass) {
    if !row.downgrade_banner_class.is_present() {
        row.downgrade_banner_class = banner;
    }
}

fn push_unique_trigger(
    triggers: &mut Vec<DiagnosticDowngradeTrigger>,
    trigger: DiagnosticDowngradeTrigger,
) {
    if !triggers.contains(&trigger) {
        triggers.push(trigger);
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
