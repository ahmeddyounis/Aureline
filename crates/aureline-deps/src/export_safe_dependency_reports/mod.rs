//! Export-safe advisory, vulnerability, license, notice, and SBOM dependency
//! reports for stable dependency-health surfaces.
//!
//! This module publishes the canonical vocabulary and typed packet that lets
//! dependency-health surfaces emit reports without overstating their truth.
//! Every report row is labeled with a [`ClaimClass`] (verified, asserted,
//! mirrored, or incomplete) tied to a [`SourceClass`] (local analysis,
//! imported feed, enterprise mirror, or stale snapshot) and a
//! [`FreshnessClass`]. A [`ConnectivityDisclosure`] keeps mirror, auth, and
//! offline reality explicit so an empty report never silently reads as a clean
//! "no findings" claim. Each export format is declared as a documented, open,
//! redaction-safe [`ExportFormatDescriptor`] so SBOM/license/advisory exports
//! stay attributable and machine-readable for release and support workflows
//! without leaking private registry URLs or secrets by default.
//!
//! The packet is checked in at
//! `artifacts/deps/m5/export-safe-dependency-reports.json` and embedded here,
//! so this typed consumer and the CI gate agree on every row without a cargo
//! build in CI.
//!
//! The model is metadata-only: every field is a typed state or an opaque ref.
//! Date arithmetic lives in the CI gate; this model enforces the structural
//! and logical invariants that hold regardless of the clock.

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Supported export-safe dependency reports packet schema version.
pub const EXPORT_SAFE_DEPENDENCY_REPORTS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag for the packet.
pub const EXPORT_SAFE_DEPENDENCY_REPORTS_RECORD_KIND: &str = "export_safe_dependency_reports";

/// Repo-relative path to the checked-in packet.
pub const EXPORT_SAFE_DEPENDENCY_REPORTS_PATH: &str =
    "artifacts/deps/m5/export-safe-dependency-reports.json";

/// Embedded checked-in packet JSON.
pub const EXPORT_SAFE_DEPENDENCY_REPORTS_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../artifacts/deps/m5/export-safe-dependency-reports.json"
));

/// Report kind that names which dependency-health surface a row speaks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportKind {
    /// Security advisory finding.
    Advisory,
    /// Vulnerability match against a resolved dependency.
    Vulnerability,
    /// License-review row for a package.
    License,
    /// Notice or attribution row for a package.
    Notice,
    /// Software bill-of-materials component row.
    Sbom,
}

impl ReportKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Advisory,
        Self::Vulnerability,
        Self::License,
        Self::Notice,
        Self::Sbom,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Advisory => "advisory",
            Self::Vulnerability => "vulnerability",
            Self::License => "license",
            Self::Notice => "notice",
            Self::Sbom => "sbom",
        }
    }
}

/// Claim class that separates verified findings from asserted, mirrored, or
/// incomplete ones so badges never overstate dependency truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimClass {
    /// Independently confirmed by current local analysis of the exact build.
    Verified,
    /// Asserted by an external feed or import, not independently confirmed.
    Asserted,
    /// Served from an enterprise mirror of an origin feed.
    Mirrored,
    /// Data is incomplete, stale, or policy-blocked; no full claim is made.
    Incomplete,
}

impl ClaimClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Verified,
        Self::Asserted,
        Self::Mirrored,
        Self::Incomplete,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Asserted => "asserted",
            Self::Mirrored => "mirrored",
            Self::Incomplete => "incomplete",
        }
    }

    /// Whether the class represents an independently verified claim.
    pub const fn is_verified(self) -> bool {
        matches!(self, Self::Verified)
    }

    /// Whether the given `source` and `freshness` may back this claim class.
    ///
    /// Verified claims may only come from current local analysis, and mirrored
    /// claims may only come from an enterprise mirror. Asserted and incomplete
    /// claims accept any source.
    pub const fn permitted_for(self, source: SourceClass, freshness: FreshnessClass) -> bool {
        match self {
            Self::Verified => {
                matches!(source, SourceClass::LocalAnalysis)
                    && matches!(freshness, FreshnessClass::Current)
            }
            Self::Mirrored => matches!(source, SourceClass::EnterpriseMirror),
            Self::Asserted | Self::Incomplete => true,
        }
    }
}

/// Source class that names where a report row's data originated.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceClass {
    /// Computed by local analysis of the resolved dependency graph.
    LocalAnalysis,
    /// Imported from an external scanner or audit feed.
    ImportedFeed,
    /// Served from an enterprise mirror of an origin feed.
    EnterpriseMirror,
    /// Read from a stale snapshot or offline bundle.
    StaleSnapshot,
}

impl SourceClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LocalAnalysis,
        Self::ImportedFeed,
        Self::EnterpriseMirror,
        Self::StaleSnapshot,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalAnalysis => "local_analysis",
            Self::ImportedFeed => "imported_feed",
            Self::EnterpriseMirror => "enterprise_mirror",
            Self::StaleSnapshot => "stale_snapshot",
        }
    }
}

/// Freshness class that names whether a report row's data is current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FreshnessClass {
    /// Data is current and within freshness SLO.
    Current,
    /// Data is present but stale.
    Stale,
    /// Only a point-in-time snapshot is available.
    SnapshotOnly,
    /// Freshness cannot be established.
    Unknown,
}

impl FreshnessClass {
    /// Every class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Current,
        Self::Stale,
        Self::SnapshotOnly,
        Self::Unknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Stale => "stale",
            Self::SnapshotOnly => "snapshot_only",
            Self::Unknown => "unknown",
        }
    }
}

/// Connectivity state of the dependency-feed plane while a report was built.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConnectivityState {
    /// Origin feeds are reachable and current.
    Online,
    /// Only an enterprise mirror is reachable; origin feeds are not.
    MirrorOnly,
    /// A credential is required before the feed can be queried.
    AuthRequired,
    /// The environment is air-gapped; no network feed is reachable.
    AirGapped,
    /// Only a previously captured offline snapshot is available.
    OfflineSnapshot,
}

impl ConnectivityState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Online,
        Self::MirrorOnly,
        Self::AuthRequired,
        Self::AirGapped,
        Self::OfflineSnapshot,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Online => "online",
            Self::MirrorOnly => "mirror_only",
            Self::AuthRequired => "auth_required",
            Self::AirGapped => "air_gapped",
            Self::OfflineSnapshot => "offline_snapshot",
        }
    }

    /// Whether the state can back a genuine "no findings" claim.
    pub const fn allows_genuine_empty(self) -> bool {
        matches!(self, Self::Online)
    }

    /// Whether the state must preserve a last-known-good timestamp because live
    /// feeds are unreachable.
    pub const fn requires_last_known_good(self) -> bool {
        matches!(
            self,
            Self::MirrorOnly | Self::AirGapped | Self::OfflineSnapshot
        )
    }
}

/// Reason that explains why a report section returned no rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EmptyResultReason {
    /// The feed was queried online and genuinely returned nothing.
    GenuinelyEmpty,
    /// The mirror data is stale; absence cannot be claimed.
    MirrorStale,
    /// A credential is required before results can be fetched.
    AuthRequired,
    /// Only a snapshot is available; absence cannot be claimed.
    SnapshotOnly,
    /// The feed is unreachable; absence cannot be claimed.
    FeedUnreachable,
}

impl EmptyResultReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::GenuinelyEmpty,
        Self::MirrorStale,
        Self::AuthRequired,
        Self::SnapshotOnly,
        Self::FeedUnreachable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GenuinelyEmpty => "genuinely_empty",
            Self::MirrorStale => "mirror_stale",
            Self::AuthRequired => "auth_required",
            Self::SnapshotOnly => "snapshot_only",
            Self::FeedUnreachable => "feed_unreachable",
        }
    }

    /// Whether the reason permits claiming a clean "no findings" posture.
    pub const fn can_claim_clean(self) -> bool {
        matches!(self, Self::GenuinelyEmpty)
    }
}

/// Report scope that names how much of the workspace a report covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReportScopeKind {
    /// The whole repository / workspace.
    FullRepo,
    /// A reviewer-selected set of manifests.
    SelectedManifests,
    /// A narrower slice (e.g., a single crate or path subtree).
    Slice,
}

impl ReportScopeKind {
    /// Every kind, in declaration order.
    pub const ALL: [Self; 3] = [Self::FullRepo, Self::SelectedManifests, Self::Slice];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullRepo => "full_repo",
            Self::SelectedManifests => "selected_manifests",
            Self::Slice => "slice",
        }
    }

    /// Whether the scope is bounded to an explicit manifest set.
    pub const fn is_bounded(self) -> bool {
        matches!(self, Self::SelectedManifests | Self::Slice)
    }
}

/// Documented, open export format a report can be emitted in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportFormat {
    /// SPDX SBOM in JSON.
    SpdxJson,
    /// CycloneDX SBOM in JSON.
    CyclonedxJson,
    /// SARIF report for advisory/vulnerability findings.
    Sarif,
    /// Generic JSON projection.
    Json,
    /// Comma-separated values.
    Csv,
    /// Human-readable Markdown summary.
    Markdown,
}

impl ExportFormat {
    /// Every format, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SpdxJson,
        Self::CyclonedxJson,
        Self::Sarif,
        Self::Json,
        Self::Csv,
        Self::Markdown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SpdxJson => "spdx_json",
            Self::CyclonedxJson => "cyclonedx_json",
            Self::Sarif => "sarif",
            Self::Json => "json",
            Self::Csv => "csv",
            Self::Markdown => "markdown",
        }
    }

    /// Whether the format is machine-readable for release/support tooling.
    pub const fn is_machine_readable(self) -> bool {
        !matches!(self, Self::Markdown)
    }

    /// Whether the format is an SBOM-capable interchange format.
    pub const fn is_sbom_capable(self) -> bool {
        matches!(self, Self::SpdxJson | Self::CyclonedxJson)
    }
}

/// Redaction posture for an export format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RedactionPosture {
    /// Private registry URLs and secrets are redacted by default.
    RedactedByDefault,
    /// Internal-scope export where private registry URLs are disclosed by an
    /// explicit operator opt-in. Secrets are still always redacted.
    OptInDisclosure,
}

impl RedactionPosture {
    /// Every posture, in declaration order.
    pub const ALL: [Self; 2] = [Self::RedactedByDefault, Self::OptInDisclosure];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RedactedByDefault => "redacted_by_default",
            Self::OptInDisclosure => "opt_in_disclosure",
        }
    }
}

/// Exact report context that every export row ties back to.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportContext {
    /// Build identifier or exact-build ref.
    pub build_id: String,
    /// Workspace scope ref (e.g., the workspace manifest path or identity).
    pub workspace_scope_ref: String,
    /// Lockfile fingerprint or equivalent digest.
    pub lockfile_fingerprint: String,
    /// Scope kind that names how much of the workspace this report covers.
    pub scope_kind: ReportScopeKind,
    /// Manifests in scope when the scope is bounded; empty for full-repo scope.
    #[serde(default)]
    pub manifests_in_scope: Vec<String>,
    /// Source refs that ground the report context.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

/// Explicit disclosure of feed connectivity and what an empty result means.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectivityDisclosure {
    /// Connectivity state of the feed plane while this report was built.
    pub connectivity_state: ConnectivityState,
    /// Why the report would be empty if no rows were present.
    pub empty_result_reason: EmptyResultReason,
    /// Last-known-good UTC timestamp, preserved when live feeds are
    /// unreachable so mirror/offline reports keep last-known truth.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_known_good_at: Option<String>,
    /// Source refs that ground the disclosure.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

impl ConnectivityDisclosure {
    /// Whether the disclosure permits claiming a clean "no findings" posture.
    ///
    /// A clean claim requires both an online feed and a genuinely-empty reason;
    /// any mirror-stale, auth-required, snapshot-only, or unreachable state
    /// keeps the absence of findings unprovable.
    pub const fn can_claim_clean(&self) -> bool {
        self.connectivity_state.allows_genuine_empty() && self.empty_result_reason.can_claim_clean()
    }
}

/// One export-format descriptor declared by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExportFormatDescriptor {
    /// The export format.
    pub format: ExportFormat,
    /// Whether the format follows a documented, open standard.
    pub open_standard: bool,
    /// Whether the format is machine-readable.
    pub machine_readable: bool,
    /// Redaction posture for this format.
    pub redaction_posture: RedactionPosture,
    /// Whether private registry URLs are redacted in this format.
    pub redacts_registry_urls: bool,
    /// Whether secret material is redacted in this format.
    pub redacts_secrets: bool,
    /// Media type emitted for this format.
    pub media_type: String,
    /// Reviewer-facing note.
    pub note: String,
}

/// One report row in the export packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportRow {
    /// Stable report row id.
    pub row_id: String,
    /// Report kind this row speaks for.
    pub report_kind: ReportKind,
    /// Human-readable title.
    pub title: String,
    /// Package coordinate or dependency ref this row speaks for.
    pub subject_ref: String,
    /// Claim class that separates verified from asserted/mirrored/incomplete.
    pub claim_class: ClaimClass,
    /// Source class that names where the row's data originated.
    pub source_class: SourceClass,
    /// Freshness class that names whether the row's data is current.
    pub freshness_class: FreshnessClass,
    /// Opaque ref to the source feed, import, or analysis that produced the row.
    pub source_ref: String,
    /// Ref backing an open-details action, when available.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_details_ref: Option<String>,
    /// Export formats this row can be emitted in; each must be declared by the
    /// packet's export-format descriptors.
    #[serde(default)]
    pub export_formats: Vec<ExportFormat>,
    /// Source refs backing the row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    /// Reviewer-facing note.
    pub note: String,
}

/// Summary counts carried by the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExportSafeDependencyReportsSummary {
    /// Total report rows.
    pub total_rows: usize,
    /// Advisory rows.
    pub advisory_rows: usize,
    /// Vulnerability rows.
    pub vulnerability_rows: usize,
    /// License rows.
    pub license_rows: usize,
    /// Notice rows.
    pub notice_rows: usize,
    /// SBOM rows.
    pub sbom_rows: usize,
    /// Rows with a verified claim.
    pub verified_rows: usize,
    /// Rows with an asserted claim.
    pub asserted_rows: usize,
    /// Rows with a mirrored claim.
    pub mirrored_rows: usize,
    /// Rows with an incomplete claim.
    pub incomplete_rows: usize,
    /// Declared export formats.
    pub total_export_formats: usize,
    /// Declared export formats that are machine-readable.
    pub machine_readable_formats: usize,
    /// Declared export formats that redact registry URLs by default.
    pub redacted_by_default_formats: usize,
}

/// A redaction-safe export row projected from the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportSafeDependencyReportsExportRow {
    /// Row id.
    pub row_id: String,
    /// Report kind token.
    pub report_kind: String,
    /// Claim class token.
    pub claim_class: String,
    /// Source class token.
    pub source_class: String,
    /// Freshness class token.
    pub freshness_class: String,
    /// Whether the row carries an independently verified claim.
    pub verified: bool,
    /// Human-readable summary.
    pub summary: String,
}

/// A redaction-safe export projection of the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExportSafeDependencyReportsExportProjection {
    /// Packet id this projection was produced from.
    pub packet_id: String,
    /// Packet as-of date.
    pub as_of: String,
    /// Report context.
    pub report_context: ReportContext,
    /// Connectivity disclosure.
    pub connectivity: ConnectivityDisclosure,
    /// Projected rows.
    pub rows: Vec<ExportSafeDependencyReportsExportRow>,
    /// Whether a clean "no findings" posture can be claimed from this report.
    pub can_claim_clean: bool,
}

/// The typed export-safe dependency reports packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExportSafeDependencyReports {
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
    /// Exact report context.
    pub report_context: ReportContext,
    /// Connectivity disclosure for the feed plane.
    pub connectivity: ConnectivityDisclosure,
    /// Closed report-kind vocabulary.
    pub report_kinds: Vec<ReportKind>,
    /// Closed claim-class vocabulary.
    pub claim_classes: Vec<ClaimClass>,
    /// Closed source-class vocabulary.
    pub source_classes: Vec<SourceClass>,
    /// Closed freshness-class vocabulary.
    pub freshness_classes: Vec<FreshnessClass>,
    /// Closed connectivity-state vocabulary.
    pub connectivity_states: Vec<ConnectivityState>,
    /// Closed empty-result-reason vocabulary.
    pub empty_result_reasons: Vec<EmptyResultReason>,
    /// Closed report-scope-kind vocabulary.
    pub report_scope_kinds: Vec<ReportScopeKind>,
    /// Closed export-format vocabulary.
    pub export_format_kinds: Vec<ExportFormat>,
    /// Closed redaction-posture vocabulary.
    pub redaction_postures: Vec<RedactionPosture>,
    /// Declared export-format descriptors.
    pub export_formats: Vec<ExportFormatDescriptor>,
    /// Report rows.
    #[serde(default)]
    pub rows: Vec<ReportRow>,
    /// Summary counts.
    pub summary: ExportSafeDependencyReportsSummary,
}

impl ExportSafeDependencyReports {
    /// Returns the report row for `row_id`.
    pub fn row(&self, row_id: &str) -> Option<&ReportRow> {
        self.rows.iter().find(|r| r.row_id == row_id)
    }

    /// Returns the descriptor for `format`, when declared.
    pub fn export_format_descriptor(
        &self,
        format: ExportFormat,
    ) -> Option<&ExportFormatDescriptor> {
        self.export_formats.iter().find(|d| d.format == format)
    }

    /// Whether the packet declares an SBOM-capable export format.
    pub fn has_sbom_export(&self) -> bool {
        self.export_formats
            .iter()
            .any(|d| d.format.is_sbom_capable())
    }

    /// Whether a clean "no findings" posture can be claimed from this report.
    ///
    /// A clean claim requires no rows, an online feed, and a genuinely-empty
    /// reason. Any mirror-stale, auth-required, snapshot-only, or unreachable
    /// state keeps absence unprovable even when no rows are present.
    pub fn can_claim_clean(&self) -> bool {
        self.rows.is_empty() && self.connectivity.can_claim_clean()
    }

    /// Recomputes the summary block from the rows and declared formats.
    pub fn computed_summary(&self) -> ExportSafeDependencyReportsSummary {
        let count_kind =
            |kind: ReportKind| self.rows.iter().filter(|r| r.report_kind == kind).count();
        let count_claim =
            |claim: ClaimClass| self.rows.iter().filter(|r| r.claim_class == claim).count();
        ExportSafeDependencyReportsSummary {
            total_rows: self.rows.len(),
            advisory_rows: count_kind(ReportKind::Advisory),
            vulnerability_rows: count_kind(ReportKind::Vulnerability),
            license_rows: count_kind(ReportKind::License),
            notice_rows: count_kind(ReportKind::Notice),
            sbom_rows: count_kind(ReportKind::Sbom),
            verified_rows: count_claim(ClaimClass::Verified),
            asserted_rows: count_claim(ClaimClass::Asserted),
            mirrored_rows: count_claim(ClaimClass::Mirrored),
            incomplete_rows: count_claim(ClaimClass::Incomplete),
            total_export_formats: self.export_formats.len(),
            machine_readable_formats: self
                .export_formats
                .iter()
                .filter(|d| d.machine_readable)
                .count(),
            redacted_by_default_formats: self
                .export_formats
                .iter()
                .filter(|d| d.redaction_posture == RedactionPosture::RedactedByDefault)
                .count(),
        }
    }

    /// Produces an export projection that downstream surfaces render instead of
    /// cloning status text.
    pub fn export_projection(&self) -> ExportSafeDependencyReportsExportProjection {
        let rows = self
            .rows
            .iter()
            .map(|row| ExportSafeDependencyReportsExportRow {
                row_id: row.row_id.clone(),
                report_kind: row.report_kind.as_str().to_owned(),
                claim_class: row.claim_class.as_str().to_owned(),
                source_class: row.source_class.as_str().to_owned(),
                freshness_class: row.freshness_class.as_str().to_owned(),
                verified: row.claim_class.is_verified(),
                summary: format!(
                    "{}: {} via {} ({}, {})",
                    row.title,
                    row.report_kind.as_str(),
                    row.source_class.as_str(),
                    row.claim_class.as_str(),
                    row.freshness_class.as_str()
                ),
            })
            .collect();
        ExportSafeDependencyReportsExportProjection {
            packet_id: self.packet_id.clone(),
            as_of: self.as_of.clone(),
            report_context: self.report_context.clone(),
            connectivity: self.connectivity.clone(),
            rows,
            can_claim_clean: self.can_claim_clean(),
        }
    }

    /// Validates the packet, returning every violation found.
    pub fn validate(&self) -> Vec<ExportSafeDependencyReportsViolation> {
        let mut violations = Vec::new();
        self.validate_envelope(&mut violations);
        self.validate_context(&mut violations);
        self.validate_connectivity(&mut violations);
        self.validate_export_formats(&mut violations);

        let mut seen = BTreeSet::new();
        for row in &self.rows {
            if !seen.insert(row.row_id.clone()) {
                violations.push(ExportSafeDependencyReportsViolation::DuplicateRowId {
                    row_id: row.row_id.clone(),
                });
            }
            self.validate_row(row, &mut violations);
        }

        if self.summary != self.computed_summary() {
            violations.push(ExportSafeDependencyReportsViolation::SummaryMismatch);
        }

        violations
    }

    fn validate_envelope(&self, violations: &mut Vec<ExportSafeDependencyReportsViolation>) {
        if self.schema_version != EXPORT_SAFE_DEPENDENCY_REPORTS_SCHEMA_VERSION {
            violations.push(
                ExportSafeDependencyReportsViolation::UnsupportedSchemaVersion {
                    actual: self.schema_version,
                },
            );
        }
        if self.record_kind != EXPORT_SAFE_DEPENDENCY_REPORTS_RECORD_KIND {
            violations.push(
                ExportSafeDependencyReportsViolation::UnsupportedRecordKind {
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
                violations.push(ExportSafeDependencyReportsViolation::EmptyField {
                    id: "<packet>".to_owned(),
                    field_name: field,
                });
            }
        }
        for (field, ok) in [
            (
                "report_kinds",
                self.report_kinds == ReportKind::ALL.to_vec(),
            ),
            (
                "claim_classes",
                self.claim_classes == ClaimClass::ALL.to_vec(),
            ),
            (
                "source_classes",
                self.source_classes == SourceClass::ALL.to_vec(),
            ),
            (
                "freshness_classes",
                self.freshness_classes == FreshnessClass::ALL.to_vec(),
            ),
            (
                "connectivity_states",
                self.connectivity_states == ConnectivityState::ALL.to_vec(),
            ),
            (
                "empty_result_reasons",
                self.empty_result_reasons == EmptyResultReason::ALL.to_vec(),
            ),
            (
                "report_scope_kinds",
                self.report_scope_kinds == ReportScopeKind::ALL.to_vec(),
            ),
            (
                "export_format_kinds",
                self.export_format_kinds == ExportFormat::ALL.to_vec(),
            ),
            (
                "redaction_postures",
                self.redaction_postures == RedactionPosture::ALL.to_vec(),
            ),
        ] {
            if !ok {
                violations
                    .push(ExportSafeDependencyReportsViolation::ClosedVocabularyMismatch { field });
            }
        }
    }

    fn validate_context(&self, violations: &mut Vec<ExportSafeDependencyReportsViolation>) {
        for (field, value) in [
            ("build_id", &self.report_context.build_id),
            (
                "workspace_scope_ref",
                &self.report_context.workspace_scope_ref,
            ),
            (
                "lockfile_fingerprint",
                &self.report_context.lockfile_fingerprint,
            ),
        ] {
            if value.trim().is_empty() {
                violations.push(ExportSafeDependencyReportsViolation::EmptyField {
                    id: "<report_context>".to_owned(),
                    field_name: field,
                });
            }
        }
        let scope = self.report_context.scope_kind;
        let manifests = &self.report_context.manifests_in_scope;
        if scope.is_bounded() && manifests.is_empty() {
            violations.push(
                ExportSafeDependencyReportsViolation::ScopeManifestMismatch {
                    scope: scope.as_str(),
                },
            );
        }
        if !scope.is_bounded() && !manifests.is_empty() {
            violations.push(
                ExportSafeDependencyReportsViolation::ScopeManifestMismatch {
                    scope: scope.as_str(),
                },
            );
        }
    }

    fn validate_connectivity(&self, violations: &mut Vec<ExportSafeDependencyReportsViolation>) {
        if self.connectivity.note.trim().is_empty() {
            violations.push(ExportSafeDependencyReportsViolation::EmptyField {
                id: "<connectivity>".to_owned(),
                field_name: "note",
            });
        }
        // An empty report must never silently read as a clean claim when the
        // feed plane is degraded.
        if self.rows.is_empty()
            && !self.connectivity.connectivity_state.allows_genuine_empty()
            && self.connectivity.empty_result_reason.can_claim_clean()
        {
            violations.push(ExportSafeDependencyReportsViolation::MisleadingEmptyClaim {
                connectivity: self.connectivity.connectivity_state.as_str(),
            });
        }
        // Mirror/offline reports must preserve last-known truth.
        if self
            .connectivity
            .connectivity_state
            .requires_last_known_good()
            && self.connectivity.last_known_good_at.is_none()
        {
            violations.push(ExportSafeDependencyReportsViolation::MissingLastKnownGood {
                connectivity: self.connectivity.connectivity_state.as_str(),
            });
        }
    }

    fn validate_export_formats(&self, violations: &mut Vec<ExportSafeDependencyReportsViolation>) {
        let mut seen = BTreeSet::new();
        for descriptor in &self.export_formats {
            if !seen.insert(descriptor.format) {
                violations.push(
                    ExportSafeDependencyReportsViolation::DuplicateExportFormat {
                        format: descriptor.format.as_str(),
                    },
                );
            }
            if descriptor.media_type.trim().is_empty() {
                violations.push(ExportSafeDependencyReportsViolation::EmptyField {
                    id: descriptor.format.as_str().to_owned(),
                    field_name: "media_type",
                });
            }
            if !descriptor.open_standard {
                violations.push(ExportSafeDependencyReportsViolation::NonOpenExportFormat {
                    format: descriptor.format.as_str(),
                });
            }
            if descriptor.machine_readable != descriptor.format.is_machine_readable() {
                violations.push(
                    ExportSafeDependencyReportsViolation::MachineReadableMismatch {
                        format: descriptor.format.as_str(),
                    },
                );
            }
            // Secrets are never disclosed in any posture.
            if !descriptor.redacts_secrets {
                violations.push(ExportSafeDependencyReportsViolation::SecretLeakRisk {
                    format: descriptor.format.as_str(),
                });
            }
            // The default-safe posture must redact private registry URLs.
            if descriptor.redaction_posture == RedactionPosture::RedactedByDefault
                && !descriptor.redacts_registry_urls
            {
                violations.push(ExportSafeDependencyReportsViolation::RegistryUrlLeakRisk {
                    format: descriptor.format.as_str(),
                });
            }
        }
        // SBOM rows require an SBOM-capable export format so they stay
        // attributable and machine-readable.
        if self.rows.iter().any(|r| r.report_kind == ReportKind::Sbom) && !self.has_sbom_export() {
            violations.push(ExportSafeDependencyReportsViolation::MissingSbomExportFormat);
        }
    }

    fn validate_row(
        &self,
        row: &ReportRow,
        violations: &mut Vec<ExportSafeDependencyReportsViolation>,
    ) {
        for (field, value) in [
            ("row_id", &row.row_id),
            ("title", &row.title),
            ("subject_ref", &row.subject_ref),
            ("source_ref", &row.source_ref),
            ("note", &row.note),
        ] {
            if value.trim().is_empty() {
                violations.push(ExportSafeDependencyReportsViolation::EmptyField {
                    id: row.row_id.clone(),
                    field_name: field,
                });
            }
        }
        if !row
            .claim_class
            .permitted_for(row.source_class, row.freshness_class)
        {
            violations.push(ExportSafeDependencyReportsViolation::OverstatedClaim {
                row_id: row.row_id.clone(),
                claim: row.claim_class.as_str(),
                source: row.source_class.as_str(),
                freshness: row.freshness_class.as_str(),
            });
        }
        for format in &row.export_formats {
            if self.export_format_descriptor(*format).is_none() {
                violations.push(
                    ExportSafeDependencyReportsViolation::UndeclaredExportFormat {
                        row_id: row.row_id.clone(),
                        format: format.as_str(),
                    },
                );
            }
        }
    }
}

/// A validation violation for the export-safe dependency reports packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportSafeDependencyReportsViolation {
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
    /// A report row id appears more than once.
    DuplicateRowId {
        /// Duplicate row id.
        row_id: String,
    },
    /// An export-format descriptor appears more than once.
    DuplicateExportFormat {
        /// Duplicate format token.
        format: &'static str,
    },
    /// The scope kind and the manifests-in-scope set disagree.
    ScopeManifestMismatch {
        /// Scope token.
        scope: &'static str,
    },
    /// An empty report claims a clean posture while the feed plane is degraded.
    MisleadingEmptyClaim {
        /// Connectivity token.
        connectivity: &'static str,
    },
    /// A mirror/offline report omits the last-known-good timestamp.
    MissingLastKnownGood {
        /// Connectivity token.
        connectivity: &'static str,
    },
    /// An export format is not a documented, open standard.
    NonOpenExportFormat {
        /// Format token.
        format: &'static str,
    },
    /// A descriptor's machine-readable flag disagrees with its format.
    MachineReadableMismatch {
        /// Format token.
        format: &'static str,
    },
    /// An export format would leak secret material.
    SecretLeakRisk {
        /// Format token.
        format: &'static str,
    },
    /// A default-safe export format would leak private registry URLs.
    RegistryUrlLeakRisk {
        /// Format token.
        format: &'static str,
    },
    /// SBOM rows are present but no SBOM-capable export format is declared.
    MissingSbomExportFormat,
    /// A claim class overstates the row's source/freshness.
    OverstatedClaim {
        /// Row id.
        row_id: String,
        /// Claim token.
        claim: &'static str,
        /// Source token.
        source: &'static str,
        /// Freshness token.
        freshness: &'static str,
    },
    /// A row references an export format the packet does not declare.
    UndeclaredExportFormat {
        /// Row id carrying the ref.
        row_id: String,
        /// Undeclared format token.
        format: &'static str,
    },
    /// The summary counts disagree with the rows.
    SummaryMismatch,
}

impl fmt::Display for ExportSafeDependencyReportsViolation {
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
            Self::DuplicateRowId { row_id } => {
                write!(f, "duplicate report row id {row_id}")
            }
            Self::DuplicateExportFormat { format } => {
                write!(f, "duplicate export-format descriptor {format}")
            }
            Self::ScopeManifestMismatch { scope } => {
                write!(f, "scope {scope} disagrees with the manifests_in_scope set")
            }
            Self::MisleadingEmptyClaim { connectivity } => {
                write!(
                    f,
                    "empty report claims a clean posture while connectivity is {connectivity}"
                )
            }
            Self::MissingLastKnownGood { connectivity } => {
                write!(
                    f,
                    "connectivity {connectivity} requires a last_known_good_at timestamp"
                )
            }
            Self::NonOpenExportFormat { format } => {
                write!(
                    f,
                    "export format {format} is not a documented, open standard"
                )
            }
            Self::MachineReadableMismatch { format } => {
                write!(
                    f,
                    "export format {format} machine_readable flag disagrees with its format"
                )
            }
            Self::SecretLeakRisk { format } => {
                write!(f, "export format {format} does not redact secrets")
            }
            Self::RegistryUrlLeakRisk { format } => {
                write!(
                    f,
                    "default-safe export format {format} does not redact private registry URLs"
                )
            }
            Self::MissingSbomExportFormat => {
                write!(
                    f,
                    "SBOM rows are present but no SBOM-capable export format is declared"
                )
            }
            Self::OverstatedClaim {
                row_id,
                claim,
                source,
                freshness,
            } => {
                write!(
                    f,
                    "row {row_id} claims {claim} but its source is {source} and freshness is {freshness}"
                )
            }
            Self::UndeclaredExportFormat { row_id, format } => {
                write!(
                    f,
                    "row {row_id} references undeclared export format {format}"
                )
            }
            Self::SummaryMismatch => {
                write!(f, "packet summary counts disagree with the rows")
            }
        }
    }
}

impl Error for ExportSafeDependencyReportsViolation {}

/// Loads the embedded export-safe dependency reports packet.
///
/// # Errors
///
/// Returns a JSON parse error when the checked-in packet no longer matches
/// [`ExportSafeDependencyReports`].
pub fn current_export_safe_dependency_reports(
) -> Result<ExportSafeDependencyReports, serde_json::Error> {
    serde_json::from_str(EXPORT_SAFE_DEPENDENCY_REPORTS_JSON)
}

#[cfg(test)]
mod tests;
