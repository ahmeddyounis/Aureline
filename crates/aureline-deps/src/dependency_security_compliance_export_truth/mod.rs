//! Dependency, security, compliance, and export-truth packet for stable
//! dependency and compliance surfaces.
//!
//! This module publishes the canonical vocabulary and typed packet that
//! ties SBOM references, notices, advisories, suppressions, and review
//! decisions back to exact build, workspace, profile, and evidence-source
//! context. It distinguishes `No active findings` from `No current feed data`,
//! preserves suppression actor/reason/scope/expiry/reopen behavior, and
//! produces redaction-safe projections for UI, CLI, support bundles, release
//! packets, and public proof.
//!
//! The packet is checked in at
//! `artifacts/deps/m4/dependency-security-compliance-export-truth.json`
//! and embedded here, so this typed consumer and the CI gate agree on every
//! row without a cargo build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref.
//! Date arithmetic lives in the CI gate; this model enforces the structural
//! and logical invariants that hold regardless of the clock.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported export-truth packet schema version.
pub const DEPENDENCY_SECURITY_COMPLIANCE_EXPORT_TRUTH_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const DEPENDENCY_SECURITY_COMPLIANCE_EXPORT_TRUTH_RECORD_KIND: &str =
    "dependency_security_compliance_export_truth";

/// Repo-relative path to the checked-in packet.
pub const DEPENDENCY_SECURITY_COMPLIANCE_EXPORT_TRUTH_PATH: &str =
    "artifacts/deps/m4/dependency-security-compliance-export-truth.json";

/// Embedded checked-in packet JSON.
pub const DEPENDENCY_SECURITY_COMPLIANCE_EXPORT_TRUTH_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m4/dependency-security-compliance-export-truth.json"
));

/// Advisory source or feed class that names where a finding originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisorySourceClass {
    /// Live public feed (e.g., OSV, GHSA, RustSec).
    LivePublicFeed,
    /// Enterprise mirror of a public feed.
    EnterpriseMirror,
    /// Imported scanner or audit report.
    ImportedReport,
    /// Stale local cache with known age.
    StaleLocalCache,
    /// Offline bundle used in air-gapped environments.
    OfflineBundle,
}

impl AdvisorySourceClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LivePublicFeed,
        Self::EnterpriseMirror,
        Self::ImportedReport,
        Self::StaleLocalCache,
        Self::OfflineBundle,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LivePublicFeed => "live_public_feed",
            Self::EnterpriseMirror => "enterprise_mirror",
            Self::ImportedReport => "imported_report",
            Self::StaleLocalCache => "stale_local_cache",
            Self::OfflineBundle => "offline_bundle",
        }
    }
}

/// Advisory freshness class that names whether the finding data is current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisoryFreshnessClass {
    /// Feed data is current and within SLO.
    Current,
    /// Feed data is stale but still present.
    Stale,
    /// No feed data is available.
    Missing,
    /// Only mirror data is available; origin feed unreachable.
    MirrorOnly,
    /// Origin feed is explicitly known to be offline or in outage.
    FeedOutage,
}

impl AdvisoryFreshnessClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Current,
        Self::Stale,
        Self::Missing,
        Self::MirrorOnly,
        Self::FeedOutage,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::Missing => "missing",
            Self::MirrorOnly => "mirror_only",
            Self::FeedOutage => "feed_outage",
        }
    }
}

/// Advisory severity class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisorySeverityClass {
    /// Low severity.
    Low,
    /// Moderate severity.
    Moderate,
    /// High severity.
    High,
    /// Critical severity.
    Critical,
    /// Severity is unknown or unassigned.
    Unknown,
}

impl AdvisorySeverityClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Low,
        Self::Moderate,
        Self::High,
        Self::Critical,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Low => "low",
            Self::Moderate => "moderate",
            Self::High => "high",
            Self::Critical => "critical",
            Self::Unknown => "unknown",
        }
    }
}

/// Suppression state for an advisory or finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuppressionState {
    /// The finding is not suppressed.
    NotSuppressed,
    /// The finding is suppressed with a time-bound expiry.
    ActiveTimeBound,
    /// The suppression has expired and the finding is reopened.
    ExpiredReopened,
    /// The finding is suppressed by policy and cannot expire.
    PolicyLocked,
}

impl SuppressionState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NotSuppressed,
        Self::ActiveTimeBound,
        Self::ExpiredReopened,
        Self::PolicyLocked,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotSuppressed => "not_suppressed",
            Self::ActiveTimeBound => "active_time_bound",
            Self::ExpiredReopened => "expired_reopened",
            Self::PolicyLocked => "policy_locked",
        }
    }

    /// Whether the finding is visually suppressed in product surfaces.
    pub const fn is_suppressed(self) -> bool {
        matches!(self, Self::ActiveTimeBound | Self::PolicyLocked)
    }

    /// Whether the finding has reopened because suppression expired.
    pub const fn is_reopened(self) -> bool {
        matches!(self, Self::ExpiredReopened)
    }
}

/// License-review posture for a dependency or package.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LicenseReviewPosture {
    /// License is approved for use.
    Approved,
    /// License is approved but requires a notice attribution.
    ApprovedWithNotice,
    /// License review is pending or incomplete.
    ReviewRequired,
    /// License is denied by policy.
    DeniedByPolicy,
    /// License state is unknown and requires review.
    UnknownRequiresReview,
}

impl LicenseReviewPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Approved,
        Self::ApprovedWithNotice,
        Self::ReviewRequired,
        Self::DeniedByPolicy,
        Self::UnknownRequiresReview,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::ApprovedWithNotice => "approved_with_notice",
            Self::ReviewRequired => "review_required",
            Self::DeniedByPolicy => "denied_by_policy",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Whether the posture blocks release publication.
    pub const fn blocks_release(self) -> bool {
        matches!(self, Self::DeniedByPolicy | Self::UnknownRequiresReview)
    }
}

/// Notice source that backs license evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NoticeSource {
    /// SPDX SBOM document.
    SpdxSbom,
    /// Human-readable notice file.
    HumanReadableNotice,
    /// REUSE-compliant repository.
    ReuseCompliant,
    /// Third-party import manifest.
    ImportManifest,
    /// Notice source is missing.
    Missing,
}

impl NoticeSource {
    /// Every source, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SpdxSbom,
        Self::HumanReadableNotice,
        Self::ReuseCompliant,
        Self::ImportManifest,
        Self::Missing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpdxSbom => "spdx_sbom",
            Self::HumanReadableNotice => "human_readable_notice",
            Self::ReuseCompliant => "reuse_compliant",
            Self::ImportManifest => "import_manifest",
            Self::Missing => "missing",
        }
    }
}

/// Export scope that names which downstream surface consumes the packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportScope {
    /// In-product UI inspection surface.
    UiInspection,
    /// CLI and headless output.
    CliHeadless,
    /// Support bundle export.
    SupportBundle,
    /// Release engineering packet.
    ReleasePacket,
    /// Public proof and certification.
    PublicProof,
}

impl ExportScope {
    /// Every scope, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::UiInspection,
        Self::CliHeadless,
        Self::SupportBundle,
        Self::ReleasePacket,
        Self::PublicProof,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UiInspection => "ui_inspection",
            Self::CliHeadless => "cli_headless",
            Self::SupportBundle => "support_bundle",
            Self::ReleasePacket => "release_packet",
            Self::PublicProof => "public_proof",
        }
    }
}

/// Lockfile-risk class that names how a dependency is resolved and whether it
/// carries known risk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockfileRiskClass {
    /// Resolved to an exact version in the lockfile.
    ResolvedExact,
    /// Pinned by policy and matches the lockfile.
    PolicyPinned,
    /// Out of date relative to the manifest requirement or policy.
    OutOfDate,
    /// Unresolved or missing from the lockfile.
    Unresolved,
    /// Known vulnerable according to current advisory data.
    Vulnerable,
}

impl LockfileRiskClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ResolvedExact,
        Self::PolicyPinned,
        Self::OutOfDate,
        Self::Unresolved,
        Self::Vulnerable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResolvedExact => "resolved_exact",
            Self::PolicyPinned => "policy_pinned",
            Self::OutOfDate => "out_of_date",
            Self::Unresolved => "unresolved",
            Self::Vulnerable => "vulnerable",
        }
    }

    /// Whether the class represents an actionable risk.
    pub const fn is_risk(self) -> bool {
        matches!(self, Self::OutOfDate | Self::Unresolved | Self::Vulnerable)
    }
}

/// Findings state that distinguishes whether advisories are present, absent,
/// or unverifiable due to feed state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingsState {
    /// Active advisories were searched and none were found.
    NoActiveFindings,
    /// Advisory data is not available; the absence of findings cannot be
    /// claimed.
    NoCurrentFeedData,
    /// One or more active findings are present.
    FindingsPresent,
    /// The feed is in an explicit outage; no claim can be made.
    FeedOutage,
}

impl FindingsState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NoActiveFindings,
        Self::NoCurrentFeedData,
        Self::FindingsPresent,
        Self::FeedOutage,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoActiveFindings => "no_active_findings",
            Self::NoCurrentFeedData => "no_current_feed_data",
            Self::FindingsPresent => "findings_present",
            Self::FeedOutage => "feed_outage",
        }
    }

    /// Whether the state allows claiming a clean advisory posture.
    pub const fn can_claim_clean(self) -> bool {
        matches!(self, Self::NoActiveFindings)
    }
}

/// Exact build context that every export row ties back to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BuildContext {
    /// Build identifier or exact-build ref.
    pub build_id: String,
    /// Workspace scope ref (e.g., the workspace manifest path or identity).
    pub workspace_scope_ref: String,
    /// Build profile (e.g., `dev`, `release`).
    pub profile: String,
    /// Cargo.lock fingerprint or equivalent lockfile digest.
    pub lockfile_fingerprint: String,
    /// Source refs that ground the build context.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

/// One advisory row in the export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdvisoryRow {
    /// Stable advisory row id.
    pub advisory_id: String,
    /// Human-readable title.
    pub title: String,
    /// Source class that names where this advisory originated.
    pub source_class: AdvisorySourceClass,
    /// Freshness class that names whether the data is current.
    pub freshness_class: AdvisoryFreshnessClass,
    /// Severity class.
    pub severity_class: AdvisorySeverityClass,
    /// Affected dependency refs (package coordinates or lockfile refs).
    #[serde(default)]
    pub affected_dependency_refs: Vec<String>,
    /// Affected version ranges.
    #[serde(default)]
    pub affected_ranges: Vec<String>,
    /// Suppression refs that apply to this advisory.
    #[serde(default)]
    pub suppression_refs: Vec<String>,
    /// State of findings for this advisory (present, absent, or unverifiable).
    pub findings_state: FindingsState,
    /// UTC timestamp when the advisory was matched against the lockfile.
    pub matched_at: String,
    /// Source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

/// One suppression row in the export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SuppressionRow {
    /// Stable suppression id.
    pub suppression_id: String,
    /// Human-readable title.
    pub title: String,
    /// Current suppression state.
    pub suppression_state: SuppressionState,
    /// Ref to the actor who created or authorized the suppression.
    pub actor_ref: String,
    /// Ref to the reason or policy that justifies the suppression.
    pub reason_ref: String,
    /// Scope ref that names what the suppression applies to.
    pub scoped_to_ref: String,
    /// Expiry timestamp, when the suppression is time-bound.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Whether the suppression will reopen the underlying finding when it
    /// expires rather than leaving it visually green.
    pub reopen_on_expiry: bool,
    /// Source refs that ground the suppression decision.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

/// One license/notice row in the export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LicenseNoticeRow {
    /// Stable row id.
    pub row_id: String,
    /// Package or dependency ref this row speaks for.
    pub package_ref: String,
    /// License-review posture.
    pub license_posture: LicenseReviewPosture,
    /// Notice source that backs the license evidence.
    pub notice_source: NoticeSource,
    /// SPDX license expression, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spdx_expression: Option<String>,
    /// SBOM refs that cover this package.
    #[serde(default)]
    pub sbom_refs: Vec<String>,
    /// Source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

/// One lockfile-risk row in the export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LockfileRiskRow {
    /// Stable row id.
    pub row_id: String,
    /// Package or dependency ref this row speaks for.
    pub package_ref: String,
    /// Lockfile-risk class.
    pub risk_class: LockfileRiskClass,
    /// Manifest requirement, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_requirement: Option<String>,
    /// Resolved version in the lockfile.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_version: Option<String>,
    /// Advisory refs that contribute to the risk class.
    #[serde(default)]
    pub advisory_refs: Vec<String>,
    /// Source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencySecurityComplianceExportTruthSummary {
    /// Total advisory rows.
    pub total_advisory_rows: usize,
    /// Advisory rows with active findings.
    pub active_advisory_rows: usize,
    /// Advisory rows with `No active findings`.
    pub no_active_findings_rows: usize,
    /// Advisory rows with `No current feed data`.
    pub no_current_feed_data_rows: usize,
    /// Advisory rows in feed outage.
    pub feed_outage_rows: usize,
    /// Total suppression rows.
    pub total_suppression_rows: usize,
    /// Active suppression rows.
    pub active_suppression_rows: usize,
    /// Expired-reopened suppression rows.
    pub expired_reopened_rows: usize,
    /// Total license/notice rows.
    pub total_license_notice_rows: usize,
    /// License/notice rows that block release.
    pub blocking_license_notice_rows: usize,
    /// Total lockfile-risk rows.
    pub total_lockfile_risk_rows: usize,
    /// Lockfile-risk rows that carry actionable risk.
    pub risky_lockfile_rows: usize,
}

/// A redaction-safe export row projected from the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencySecurityComplianceExportTruthExportRow {
    /// Row id.
    pub row_id: String,
    /// Row kind discriminator.
    pub row_kind: String,
    /// Effective state token for the row.
    pub effective_state: String,
    /// Whether the row blocks a stable claim.
    pub blocks_stable: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencySecurityComplianceExportTruthExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Build context.
    pub build_context: BuildContext,
    /// Projected rows.
    pub rows: Vec<DependencySecurityComplianceExportTruthExportRow>,
    /// Overall findings state.
    pub overall_findings_state: FindingsState,
    /// Whether any row blocks a stable claim.
    pub blocks_stable: bool,
}

/// The typed dependency-security-compliance export-truth packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DependencySecurityComplianceExportTruth {
    /// Packet schema version.
    pub schema_version: u32,
    /// Record-kind discriminator.
    pub record_kind: String,
    /// Stable packet identifier.
    pub packet_id: String,
    /// Lifecycle status of this packet.
    pub status: String,
    /// Human-readable companion document.
    pub overview_page: String,
    /// UTC date this snapshot is current as of.
    pub as_of: String,
    /// Exact build context.
    pub build_context: BuildContext,
    /// Closed advisory-source-class vocabulary.
    pub advisory_source_classes: Vec<AdvisorySourceClass>,
    /// Closed advisory-freshness-class vocabulary.
    pub advisory_freshness_classes: Vec<AdvisoryFreshnessClass>,
    /// Closed advisory-severity-class vocabulary.
    pub advisory_severity_classes: Vec<AdvisorySeverityClass>,
    /// Closed suppression-state vocabulary.
    pub suppression_states: Vec<SuppressionState>,
    /// Closed license-review-posture vocabulary.
    pub license_review_postures: Vec<LicenseReviewPosture>,
    /// Closed notice-source vocabulary.
    pub notice_sources: Vec<NoticeSource>,
    /// Closed export-scope vocabulary.
    pub export_scopes: Vec<ExportScope>,
    /// Closed lockfile-risk-class vocabulary.
    pub lockfile_risk_classes: Vec<LockfileRiskClass>,
    /// Closed findings-state vocabulary.
    pub findings_states: Vec<FindingsState>,
    /// Advisory rows.
    #[serde(default)]
    pub advisory_rows: Vec<AdvisoryRow>,
    /// Suppression rows.
    #[serde(default)]
    pub suppression_rows: Vec<SuppressionRow>,
    /// License/notice rows.
    #[serde(default)]
    pub license_notice_rows: Vec<LicenseNoticeRow>,
    /// Lockfile-risk rows.
    #[serde(default)]
    pub lockfile_risk_rows: Vec<LockfileRiskRow>,
    /// Export scopes this packet targets.
    pub export_scopes_targeted: Vec<ExportScope>,
    /// Summary counts.
    pub summary: DependencySecurityComplianceExportTruthSummary,
}

impl DependencySecurityComplianceExportTruth {
    /// Returns the advisory row for `advisory_id`.
    pub fn advisory_row(&self, advisory_id: &str) -> Option<&AdvisoryRow> {
        self.advisory_rows
            .iter()
            .find(|r| r.advisory_id == advisory_id)
    }

    /// Returns the suppression row for `suppression_id`.
    pub fn suppression_row(&self, suppression_id: &str) -> Option<&SuppressionRow> {
        self.suppression_rows
            .iter()
            .find(|r| r.suppression_id == suppression_id)
    }

    /// Returns the license/notice row for `row_id`.
    pub fn license_notice_row(&self, row_id: &str) -> Option<&LicenseNoticeRow> {
        self.license_notice_rows.iter().find(|r| r.row_id == row_id)
    }

    /// Returns the lockfile-risk row for `row_id`.
    pub fn lockfile_risk_row(&self, row_id: &str) -> Option<&LockfileRiskRow> {
        self.lockfile_risk_rows.iter().find(|r| r.row_id == row_id)
    }

    /// Recomputes the overall findings state from advisory rows.
    pub fn computed_overall_findings_state(&self) -> FindingsState {
        if self
            .advisory_rows
            .iter()
            .any(|r| r.findings_state == FindingsState::FeedOutage)
        {
            return FindingsState::FeedOutage;
        }
        if self
            .advisory_rows
            .iter()
            .any(|r| r.findings_state == FindingsState::NoCurrentFeedData)
        {
            return FindingsState::NoCurrentFeedData;
        }
        if self
            .advisory_rows
            .iter()
            .any(|r| r.findings_state == FindingsState::FindingsPresent)
        {
            return FindingsState::FindingsPresent;
        }
        FindingsState::NoActiveFindings
    }

    /// Whether any row blocks a stable claim.
    pub fn blocks_stable(&self) -> bool {
        self.advisory_rows.iter().any(|r| {
            r.findings_state == FindingsState::FindingsPresent
                && !r.suppression_refs.iter().any(|sup_ref| {
                    self.suppression_rows.iter().any(|sup| {
                        &sup.suppression_id == sup_ref && sup.suppression_state.is_suppressed()
                    })
                })
        }) || self
            .license_notice_rows
            .iter()
            .any(|r| r.license_posture.blocks_release())
            || self
                .lockfile_risk_rows
                .iter()
                .any(|r| r.risk_class.is_risk())
    }

    /// Recomputes the summary block from the rows.
    pub fn computed_summary(&self) -> DependencySecurityComplianceExportTruthSummary {
        DependencySecurityComplianceExportTruthSummary {
            total_advisory_rows: self.advisory_rows.len(),
            active_advisory_rows: self
                .advisory_rows
                .iter()
                .filter(|r| r.findings_state == FindingsState::FindingsPresent)
                .count(),
            no_active_findings_rows: self
                .advisory_rows
                .iter()
                .filter(|r| r.findings_state == FindingsState::NoActiveFindings)
                .count(),
            no_current_feed_data_rows: self
                .advisory_rows
                .iter()
                .filter(|r| r.findings_state == FindingsState::NoCurrentFeedData)
                .count(),
            feed_outage_rows: self
                .advisory_rows
                .iter()
                .filter(|r| r.findings_state == FindingsState::FeedOutage)
                .count(),
            total_suppression_rows: self.suppression_rows.len(),
            active_suppression_rows: self
                .suppression_rows
                .iter()
                .filter(|r| r.suppression_state.is_suppressed())
                .count(),
            expired_reopened_rows: self
                .suppression_rows
                .iter()
                .filter(|r| r.suppression_state.is_reopened())
                .count(),
            total_license_notice_rows: self.license_notice_rows.len(),
            blocking_license_notice_rows: self
                .license_notice_rows
                .iter()
                .filter(|r| r.license_posture.blocks_release())
                .count(),
            total_lockfile_risk_rows: self.lockfile_risk_rows.len(),
            risky_lockfile_rows: self
                .lockfile_risk_rows
                .iter()
                .filter(|r| r.risk_class.is_risk())
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces render instead
    /// of cloning status text.
    pub fn export_projection(&self) -> DependencySecurityComplianceExportTruthExportProjection {
        let mut rows = Vec::new();
        for advisory in &self.advisory_rows {
            let suppressed = advisory.suppression_refs.iter().any(|sup_ref| {
                self.suppression_rows.iter().any(|sup| {
                    &sup.suppression_id == sup_ref && sup.suppression_state.is_suppressed()
                })
            });
            let effective_state = if suppressed {
                "suppressed".to_owned()
            } else {
                advisory.findings_state.as_str().to_owned()
            };
            rows.push(DependencySecurityComplianceExportTruthExportRow {
                row_id: advisory.advisory_id.clone(),
                row_kind: "advisory".to_owned(),
                effective_state: effective_state.clone(),
                blocks_stable: advisory.findings_state == FindingsState::FindingsPresent
                    && !suppressed,
                summary: format!("{}: {}", advisory.title, effective_state),
            });
        }
        for suppression in &self.suppression_rows {
            rows.push(DependencySecurityComplianceExportTruthExportRow {
                row_id: suppression.suppression_id.clone(),
                row_kind: "suppression".to_owned(),
                effective_state: suppression.suppression_state.as_str().to_owned(),
                blocks_stable: suppression.suppression_state.is_reopened(),
                summary: format!(
                    "{}: {}",
                    suppression.title,
                    suppression.suppression_state.as_str()
                ),
            });
        }
        for license in &self.license_notice_rows {
            rows.push(DependencySecurityComplianceExportTruthExportRow {
                row_id: license.row_id.clone(),
                row_kind: "license_notice".to_owned(),
                effective_state: license.license_posture.as_str().to_owned(),
                blocks_stable: license.license_posture.blocks_release(),
                summary: format!(
                    "{}: {} (notice: {})",
                    license.package_ref,
                    license.license_posture.as_str(),
                    license.notice_source.as_str()
                ),
            });
        }
        for risk in &self.lockfile_risk_rows {
            rows.push(DependencySecurityComplianceExportTruthExportRow {
                row_id: risk.row_id.clone(),
                row_kind: "lockfile_risk".to_owned(),
                effective_state: risk.risk_class.as_str().to_owned(),
                blocks_stable: risk.risk_class.is_risk(),
                summary: format!(
                    "{}: {} (resolved: {})",
                    risk.package_ref,
                    risk.risk_class.as_str(),
                    risk.resolved_version.as_deref().unwrap_or("unknown")
                ),
            });
        }
        DependencySecurityComplianceExportTruthExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            build_context: self.build_context.clone(),
            rows,
            overall_findings_state: self.computed_overall_findings_state(),
            blocks_stable: self.blocks_stable(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<DependencySecurityComplianceExportTruthViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.advisory_rows {
            if !seen.insert(format!("advisory:{}", row.advisory_id)) {
                violations.push(
                    DependencySecurityComplianceExportTruthViolation::DuplicateRowId {
                        row_id: row.advisory_id.clone(),
                        row_kind: "advisory",
                    },
                );
            }
            self.validate_advisory_row(row, &mut violations);
        }

        seen.clear();
        for row in &self.suppression_rows {
            if !seen.insert(format!("suppression:{}", row.suppression_id)) {
                violations.push(
                    DependencySecurityComplianceExportTruthViolation::DuplicateRowId {
                        row_id: row.suppression_id.clone(),
                        row_kind: "suppression",
                    },
                );
            }
            self.validate_suppression_row(row, &mut violations);
        }

        seen.clear();
        for row in &self.license_notice_rows {
            if !seen.insert(format!("license_notice:{}", row.row_id)) {
                violations.push(
                    DependencySecurityComplianceExportTruthViolation::DuplicateRowId {
                        row_id: row.row_id.clone(),
                        row_kind: "license_notice",
                    },
                );
            }
            self.validate_license_notice_row(row, &mut violations);
        }

        seen.clear();
        for row in &self.lockfile_risk_rows {
            if !seen.insert(format!("lockfile_risk:{}", row.row_id)) {
                violations.push(
                    DependencySecurityComplianceExportTruthViolation::DuplicateRowId {
                        row_id: row.row_id.clone(),
                        row_kind: "lockfile_risk",
                    },
                );
            }
            self.validate_lockfile_risk_row(row, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(DependencySecurityComplianceExportTruthViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(
        &self,
        violations: &mut Vec<DependencySecurityComplianceExportTruthViolation>,
    ) {
        if self.schema_version != DEPENDENCY_SECURITY_COMPLIANCE_EXPORT_TRUTH_SCHEMA_VERSION {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != DEPENDENCY_SECURITY_COMPLIANCE_EXPORT_TRUTH_RECORD_KIND {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::UnsupportedRecordKind {
                    actual: self.record_kind.clone(),
                },
            );
        }
        for (field, value) in [
            ("packet_id", &self.packet_id),
            ("status", &self.status),
            ("overview_page", &self.overview_page),
            ("as_of", &self.as_of),
        ] {
            if value.trim().is_empty() {
                violations.push(
                    DependencySecurityComplianceExportTruthViolation::EmptyField {
                        id: "<packet>".to_owned(),
                        field_name: field,
                    },
                );
            }
        }
        if self.advisory_source_classes != AdvisorySourceClass::ALL.to_vec() {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::ClosedVocabularyMismatch {
                    field: "advisory_source_classes",
                },
            );
        }
        if self.advisory_freshness_classes != AdvisoryFreshnessClass::ALL.to_vec() {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::ClosedVocabularyMismatch {
                    field: "advisory_freshness_classes",
                },
            );
        }
        if self.advisory_severity_classes != AdvisorySeverityClass::ALL.to_vec() {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::ClosedVocabularyMismatch {
                    field: "advisory_severity_classes",
                },
            );
        }
        if self.suppression_states != SuppressionState::ALL.to_vec() {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::ClosedVocabularyMismatch {
                    field: "suppression_states",
                },
            );
        }
        if self.license_review_postures != LicenseReviewPosture::ALL.to_vec() {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::ClosedVocabularyMismatch {
                    field: "license_review_postures",
                },
            );
        }
        if self.notice_sources != NoticeSource::ALL.to_vec() {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::ClosedVocabularyMismatch {
                    field: "notice_sources",
                },
            );
        }
        if self.export_scopes != ExportScope::ALL.to_vec() {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::ClosedVocabularyMismatch {
                    field: "export_scopes",
                },
            );
        }
        if self.lockfile_risk_classes != LockfileRiskClass::ALL.to_vec() {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::ClosedVocabularyMismatch {
                    field: "lockfile_risk_classes",
                },
            );
        }
        if self.findings_states != FindingsState::ALL.to_vec() {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::ClosedVocabularyMismatch {
                    field: "findings_states",
                },
            );
        }
        if self.build_context.build_id.trim().is_empty() {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::EmptyField {
                    id: "<build_context>".to_owned(),
                    field_name: "build_id",
                },
            );
        }
        if self.build_context.workspace_scope_ref.trim().is_empty() {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::EmptyField {
                    id: "<build_context>".to_owned(),
                    field_name: "workspace_scope_ref",
                },
            );
        }
        if self.build_context.lockfile_fingerprint.trim().is_empty() {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::EmptyField {
                    id: "<build_context>".to_owned(),
                    field_name: "lockfile_fingerprint",
                },
            );
        }
    }

    fn validate_advisory_row(
        &self,
        row: &AdvisoryRow,
        violations: &mut Vec<DependencySecurityComplianceExportTruthViolation>,
    ) {
        for (field, value) in [
            ("advisory_id", &row.advisory_id),
            ("title", &row.title),
            ("matched_at", &row.matched_at),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(
                    DependencySecurityComplianceExportTruthViolation::EmptyField {
                        id: row.advisory_id.clone(),
                        field_name: field,
                    },
                );
            }
        }
        for sup_ref in &row.suppression_refs {
            if self.suppression_row(sup_ref).is_none() {
                violations.push(
                    DependencySecurityComplianceExportTruthViolation::DanglingSuppressionRef {
                        row_id: row.advisory_id.clone(),
                        suppression_ref: sup_ref.clone(),
                    },
                );
            }
        }
    }

    fn validate_suppression_row(
        &self,
        row: &SuppressionRow,
        violations: &mut Vec<DependencySecurityComplianceExportTruthViolation>,
    ) {
        for (field, value) in [
            ("suppression_id", &row.suppression_id),
            ("title", &row.title),
            ("actor_ref", &row.actor_ref),
            ("reason_ref", &row.reason_ref),
            ("scoped_to_ref", &row.scoped_to_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(
                    DependencySecurityComplianceExportTruthViolation::EmptyField {
                        id: row.suppression_id.clone(),
                        field_name: field,
                    },
                );
            }
        }
        if row.suppression_state == SuppressionState::ActiveTimeBound && row.expires_at.is_none() {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::TimeBoundSuppressionWithoutExpiry {
                    suppression_id: row.suppression_id.clone(),
                },
            );
        }
        if row.suppression_state == SuppressionState::ExpiredReopened && !row.reopen_on_expiry {
            violations.push(
                DependencySecurityComplianceExportTruthViolation::ExpiredReopenedWithoutReopenFlag {
                    suppression_id: row.suppression_id.clone(),
                },
            );
        }
    }

    fn validate_license_notice_row(
        &self,
        row: &LicenseNoticeRow,
        violations: &mut Vec<DependencySecurityComplianceExportTruthViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("package_ref", &row.package_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(
                    DependencySecurityComplianceExportTruthViolation::EmptyField {
                        id: row.row_id.clone(),
                        field_name: field,
                    },
                );
            }
        }
    }

    fn validate_lockfile_risk_row(
        &self,
        row: &LockfileRiskRow,
        violations: &mut Vec<DependencySecurityComplianceExportTruthViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("package_ref", &row.package_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(
                    DependencySecurityComplianceExportTruthViolation::EmptyField {
                        id: row.row_id.clone(),
                        field_name: field,
                    },
                );
            }
        }
        for adv_ref in &row.advisory_refs {
            if self.advisory_row(adv_ref).is_none() {
                violations.push(
                    DependencySecurityComplianceExportTruthViolation::DanglingAdvisoryRef {
                        row_id: row.row_id.clone(),
                        advisory_ref: adv_ref.clone(),
                    },
                );
            }
        }
    }
}

/// A validation violation for the export-truth packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DependencySecurityComplianceExportTruthViolation {
    /// The packet carries an unsupported schema version.
    UnsupportedSchemaVersion {
        /// Version found in the packet.
        actual: u32,
    },
    /// The packet carries an unsupported record kind.
    UnsupportedRecordKind {
        /// Record kind found in the packet.
        actual: String,
    },
    /// A closed vocabulary or pinned value is not canonical.
    ClosedVocabularyMismatch {
        /// Offending field.
        field: &'static str,
    },
    /// A required field is empty.
    EmptyField {
        /// Row, section, or packet id.
        id: String,
        /// Field name.
        field_name: &'static str,
    },
    /// A row id appears more than once within its kind.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
        /// Row kind discriminator.
        row_kind: &'static str,
    },
    /// A suppression ref on an advisory row does not resolve.
    DanglingSuppressionRef {
        /// Row id carrying the ref.
        row_id: String,
        /// Unresolvable suppression ref.
        suppression_ref: String,
    },
    /// An advisory ref on a lockfile-risk row does not resolve.
    DanglingAdvisoryRef {
        /// Row id carrying the ref.
        row_id: String,
        /// Unresolvable advisory ref.
        advisory_ref: String,
    },
    /// An active-time-bound suppression lacks an expiry.
    TimeBoundSuppressionWithoutExpiry {
        /// Suppression id.
        suppression_id: String,
    },
    /// An expired-reopened suppression lacks the reopen flag.
    ExpiredReopenedWithoutReopenFlag {
        /// Suppression id.
        suppression_id: String,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for DependencySecurityComplianceExportTruthViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedSchemaVersion { actual } => {
                write!(f, "unsupported packet schema_version {actual}")
            }
            Self::UnsupportedRecordKind { actual } => {
                write!(f, "unsupported packet record_kind {actual}")
            }
            Self::ClosedVocabularyMismatch { field } => {
                write!(f, "packet {field} is not the canonical value")
            }
            Self::EmptyField { id, field_name } => {
                write!(f, "{id} has empty field {field_name}")
            }
            Self::DuplicateRowId { row_id, row_kind } => {
                write!(f, "duplicate {row_kind} row id {row_id}")
            }
            Self::DanglingSuppressionRef {
                row_id,
                suppression_ref,
            } => {
                write!(
                    f,
                    "advisory row {row_id} references missing suppression {suppression_ref}"
                )
            }
            Self::DanglingAdvisoryRef {
                row_id,
                advisory_ref,
            } => {
                write!(
                    f,
                    "lockfile-risk row {row_id} references missing advisory {advisory_ref}"
                )
            }
            Self::TimeBoundSuppressionWithoutExpiry { suppression_id } => {
                write!(
                    f,
                    "suppression {suppression_id} is active_time_bound but has no expires_at"
                )
            }
            Self::ExpiredReopenedWithoutReopenFlag { suppression_id } => {
                write!(
                    f,
                    "suppression {suppression_id} is expired_reopened but reopen_on_expiry is false"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for DependencySecurityComplianceExportTruthViolation {}

/// Loads the embedded dependency-security-compliance export-truth packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`DependencySecurityComplianceExportTruth`].
pub fn current_dependency_security_compliance_export_truth(
) -> Result<DependencySecurityComplianceExportTruth, serde_json::Error> {
    serde_json::from_str(DEPENDENCY_SECURITY_COMPLIANCE_EXPORT_TRUTH_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn packet() -> DependencySecurityComplianceExportTruth {
        current_dependency_security_compliance_export_truth().expect("packet parses")
    }

    #[test]
    fn embedded_packet_parses_and_validates() {
        let packet = packet();
        assert_eq!(
            packet.schema_version,
            DEPENDENCY_SECURITY_COMPLIANCE_EXPORT_TRUTH_SCHEMA_VERSION
        );
        assert_eq!(
            packet.record_kind,
            DEPENDENCY_SECURITY_COMPLIANCE_EXPORT_TRUTH_RECORD_KIND
        );
        assert_eq!(packet.validate(), Vec::new());
    }

    #[test]
    fn packet_has_build_context() {
        let packet = packet();
        assert!(!packet.build_context.build_id.is_empty());
        assert!(!packet.build_context.workspace_scope_ref.is_empty());
        assert!(!packet.build_context.lockfile_fingerprint.is_empty());
    }

    #[test]
    fn summary_counts_match_rows() {
        let packet = packet();
        assert_eq!(packet.summary, packet.computed_summary());
    }

    #[test]
    fn overall_findings_state_is_coherent() {
        let packet = packet();
        let computed = packet.computed_overall_findings_state();
        // If there are feed-outage rows, overall must be feed_outage.
        if packet
            .advisory_rows
            .iter()
            .any(|r| r.findings_state == FindingsState::FeedOutage)
        {
            assert_eq!(computed, FindingsState::FeedOutage);
        }
    }

    #[test]
    fn export_projection_includes_all_rows() {
        let packet = packet();
        let projection = packet.export_projection();
        let expected_rows = packet.advisory_rows.len()
            + packet.suppression_rows.len()
            + packet.license_notice_rows.len()
            + packet.lockfile_risk_rows.len();
        assert_eq!(projection.rows.len(), expected_rows);
        assert_eq!(projection.packet_id, packet.packet_id);
    }

    #[test]
    fn suppression_state_tokens_are_stable() {
        assert_eq!(SuppressionState::NotSuppressed.as_str(), "not_suppressed");
        assert_eq!(
            SuppressionState::ActiveTimeBound.as_str(),
            "active_time_bound"
        );
        assert_eq!(
            SuppressionState::ExpiredReopened.as_str(),
            "expired_reopened"
        );
        assert_eq!(SuppressionState::PolicyLocked.as_str(), "policy_locked");
    }

    #[test]
    fn findings_state_distinguishes_clean_from_unknown() {
        assert!(FindingsState::NoActiveFindings.can_claim_clean());
        assert!(!FindingsState::NoCurrentFeedData.can_claim_clean());
        assert!(!FindingsState::FindingsPresent.can_claim_clean());
        assert!(!FindingsState::FeedOutage.can_claim_clean());
    }

    #[test]
    fn validate_flags_dangling_suppression_ref() {
        let mut packet = packet();
        if let Some(row) = packet.advisory_rows.first_mut() {
            row.suppression_refs
                .push("suppression:does_not_exist".to_owned());
            let violations = packet.validate();
            assert!(violations.iter().any(|v| matches!(
                v,
                DependencySecurityComplianceExportTruthViolation::DanglingSuppressionRef { .. }
            )));
        }
    }

    #[test]
    fn validate_flags_time_bound_without_expiry() {
        let mut packet = packet();
        if let Some(row) = packet.suppression_rows.first_mut() {
            row.suppression_state = SuppressionState::ActiveTimeBound;
            row.expires_at = None;
            let violations = packet.validate();
            assert!(violations.iter().any(|v| matches!(
                v,
                DependencySecurityComplianceExportTruthViolation::TimeBoundSuppressionWithoutExpiry { .. }
            )));
        }
    }

    #[test]
    fn validate_flags_summary_mismatch() {
        let mut packet = packet();
        packet.summary.total_advisory_rows = packet.summary.total_advisory_rows.wrapping_add(1);
        let violations = packet.validate();
        assert!(
            violations.contains(&DependencySecurityComplianceExportTruthViolation::SummaryMismatch)
        );
    }

    #[test]
    fn advisory_source_classes_are_exhaustive() {
        let packet = packet();
        let present: BTreeSet<AdvisorySourceClass> = packet
            .advisory_rows
            .iter()
            .map(|r| r.source_class)
            .collect();
        for class in AdvisorySourceClass::ALL {
            assert!(
                present.contains(&class),
                "missing advisory source class {}",
                class.as_str()
            );
        }
    }

    #[test]
    fn advisory_freshness_classes_are_exhaustive() {
        let packet = packet();
        let present: BTreeSet<AdvisoryFreshnessClass> = packet
            .advisory_rows
            .iter()
            .map(|r| r.freshness_class)
            .collect();
        for class in AdvisoryFreshnessClass::ALL {
            assert!(
                present.contains(&class),
                "missing advisory freshness class {}",
                class.as_str()
            );
        }
    }

    #[test]
    fn suppression_states_are_exhaustive() {
        let packet = packet();
        let present: BTreeSet<SuppressionState> = packet
            .suppression_rows
            .iter()
            .map(|r| r.suppression_state)
            .collect();
        for state in SuppressionState::ALL {
            assert!(
                present.contains(&state),
                "missing suppression state {}",
                state.as_str()
            );
        }
    }

    #[test]
    fn license_review_postures_are_exhaustive() {
        let packet = packet();
        let present: BTreeSet<LicenseReviewPosture> = packet
            .license_notice_rows
            .iter()
            .map(|r| r.license_posture)
            .collect();
        for posture in LicenseReviewPosture::ALL {
            assert!(
                present.contains(&posture),
                "missing license review posture {}",
                posture.as_str()
            );
        }
    }

    #[test]
    fn lockfile_risk_classes_are_exhaustive() {
        let packet = packet();
        let present: BTreeSet<LockfileRiskClass> = packet
            .lockfile_risk_rows
            .iter()
            .map(|r| r.risk_class)
            .collect();
        for class in LockfileRiskClass::ALL {
            assert!(
                present.contains(&class),
                "missing lockfile risk class {}",
                class.as_str()
            );
        }
    }
}
