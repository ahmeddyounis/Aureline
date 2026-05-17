//! Release-truth wiring for docs/help, migration, Help/About, and service health.
//!
//! The docs browser is the narrowest user-facing proof surface for
//! documentation truth, so this module owns the cross-surface wiring check
//! that joins the current beta claim manifest to the current compatibility
//! report. The projection is intentionally read-only: it does not mint claim
//! rows, widen support copy, or repair stale evidence. It only answers which
//! surfaces resolve to the current manifest rows, which compatibility rows
//! back those claims, and which community or support route preserves the
//! current object and issue context before any browser or support handoff.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::service_health::{
    ClaimPostureClass, M3ClaimManifestSnapshot, ManifestChannelId, ManifestLoadError,
    ServiceHealthBetaRow, ServiceHealthBetaSurface,
};

/// Stable record-kind tag carried by the truth-wiring report.
pub const TRUTH_WIRING_REPORT_RECORD_KIND: &str = "docs_truth_wiring_report_record";

/// Schema version for [`TruthWiringReport`].
pub const TRUTH_WIRING_REPORT_SCHEMA_VERSION: u32 = 1;

/// Current claim manifest path used by the checked-in beta wiring report.
pub const CURRENT_CLAIM_MANIFEST_REF: &str = "artifacts/release/m3/claim_manifest.json";

/// Current compatibility report path used by the checked-in beta wiring report.
pub const CURRENT_COMPATIBILITY_REPORT_REF: &str = "artifacts/compat/m3/compatibility_report.json";

/// Current checked-in truth-wiring report path.
pub const CURRENT_TRUTH_WIRING_REPORT_REF: &str = "artifacts/docs/m3/truth_wiring_report.md";

const CURRENT_CLAIM_MANIFEST_BYTES: &[u8] =
    include_bytes!("../../../../artifacts/release/m3/claim_manifest.json");
const CURRENT_COMPATIBILITY_REPORT_BYTES: &[u8] =
    include_bytes!("../../../../artifacts/compat/m3/compatibility_report.json");

/// Surface family covered by the beta release-truth wiring check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthSurfaceClass {
    /// In-product docs/help browser.
    DocsBrowser,
    /// Migration center and its support-export mirror.
    MigrationCenter,
    /// Help/About release-truth card.
    HelpAbout,
    /// Service-health beta surface.
    ServiceHealth,
}

impl TruthSurfaceClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsBrowser => "docs_browser",
            Self::MigrationCenter => "migration_center",
            Self::HelpAbout => "help_about",
            Self::ServiceHealth => "service_health",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::DocsBrowser => "Docs browser",
            Self::MigrationCenter => "Migration center",
            Self::HelpAbout => "Help/About",
            Self::ServiceHealth => "Service health",
        }
    }

    /// Manifest channel that owns this surface's release-truth binding.
    pub const fn required_channel(self) -> ManifestChannelId {
        match self {
            Self::DocsBrowser => ManifestChannelId::DocsSite,
            Self::MigrationCenter => ManifestChannelId::MigrationNotes,
            Self::HelpAbout => ManifestChannelId::HelpAbout,
            Self::ServiceHealth => ManifestChannelId::ServiceHealth,
        }
    }

    /// Surface classes in deterministic report order.
    pub const fn all() -> [Self; 4] {
        [
            Self::DocsBrowser,
            Self::MigrationCenter,
            Self::HelpAbout,
            Self::ServiceHealth,
        ]
    }
}

/// Stable contract-state vocabulary for release-truth consumers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceContractState {
    /// Current evidence resolves without downgrade for this surface binding.
    Ready,
    /// Evidence resolves, but support, posture, or freshness is visibly narrowed.
    Degraded,
    /// The surface is usable only through local cached/export state.
    LocalOnly,
    /// The surface resolves, but evidence freshness is stale or expired.
    Stale,
    /// Claim rows or compatibility rows do not resolve consistently.
    ContractMismatch,
    /// A policy-disabled claim blocks the normal surface path.
    PolicyBlocked,
    /// The surface has no usable release-truth path.
    Unavailable,
}

impl ServiceContractState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::Degraded => "degraded",
            Self::LocalOnly => "local_only",
            Self::Stale => "stale",
            Self::ContractMismatch => "contract_mismatch",
            Self::PolicyBlocked => "policy_blocked",
            Self::Unavailable => "unavailable",
        }
    }

    /// Reviewer-facing label.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ready => "Ready",
            Self::Degraded => "Degraded",
            Self::LocalOnly => "Local only",
            Self::Stale => "Stale",
            Self::ContractMismatch => "Contract mismatch",
            Self::PolicyBlocked => "Policy blocked",
            Self::Unavailable => "Unavailable",
        }
    }
}

/// Destination trust class disclosed before a community/support handoff.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DestinationTrustClass {
    /// Public first-party destination.
    OfficialPublic,
    /// Authenticated first-party destination.
    OfficialAuthenticated,
    /// Community-run or community-moderated destination.
    Community,
    /// No destination outside the local product boundary.
    LocalOnly,
}

impl DestinationTrustClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OfficialPublic => "official_public",
            Self::OfficialAuthenticated => "official_authenticated",
            Self::Community => "community",
            Self::LocalOnly => "local_only",
        }
    }
}

/// Community/support route class used by Help/About handoff rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommunityHandoffRouteClass {
    /// Public issue tracker.
    PublicIssueTracker,
    /// Public RFC or design forum.
    PublicRfcForum,
    /// Private security intake.
    PrivateSecurityChannel,
    /// Private support intake.
    PrivateSupportChannel,
}

impl CommunityHandoffRouteClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PublicIssueTracker => "public_issue_tracker",
            Self::PublicRfcForum => "public_rfc_forum",
            Self::PrivateSecurityChannel => "private_security_channel",
            Self::PrivateSupportChannel => "private_support_channel",
        }
    }
}

/// Issue class used to choose the public or private handoff lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommunityIssueClass {
    /// Documentation source, freshness, or version-match issue.
    DocsTruthMismatch,
    /// Migration/import compatibility regression.
    MigrationCompatibilityRegression,
    /// Design proposal or governance discussion.
    DesignProposal,
    /// Security-sensitive report.
    SecuritySensitive,
    /// Private workspace, account, tenant, or live-device support issue.
    PrivateWorkspaceSupport,
}

impl CommunityIssueClass {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocsTruthMismatch => "docs_truth_mismatch",
            Self::MigrationCompatibilityRegression => "migration_compatibility_regression",
            Self::DesignProposal => "design_proposal",
            Self::SecuritySensitive => "security_sensitive",
            Self::PrivateWorkspaceSupport => "private_workspace_support",
        }
    }
}

/// Minimal parsed view of the current compatibility report.
#[derive(Debug, Clone, Deserialize)]
pub struct CompatibilityReportSnapshot {
    /// Upstream record-kind tag.
    pub record_kind: String,
    /// Compatibility report id.
    pub report_id: String,
    /// Compatibility report revision.
    pub report_revision: u32,
    /// Report state such as `draft`.
    pub report_state: String,
    /// Date the report describes.
    pub as_of: String,
    /// Generation timestamp.
    pub generated_at: String,
    /// Report owner.
    pub owner: String,
    /// Compatibility rows.
    pub rows: Vec<CompatibilityReportRowSnapshot>,
}

/// One row from the compatibility report.
#[derive(Debug, Clone, Deserialize)]
pub struct CompatibilityReportRowSnapshot {
    /// Stable compatibility row id.
    pub row_id: String,
    /// Report-local row id.
    pub report_row_id: String,
    /// Claimed product surface.
    pub claimed_surface: String,
    /// Boundary label the row governs.
    pub artifact_or_protocol_boundary_label: String,
    /// Row scope such as `desktop`, `schema`, or `deployment_profile`.
    pub row_scope: String,
    /// Optional support-class summary.
    #[serde(default)]
    pub support_class: Option<CompatibilitySupportSnapshot>,
}

/// Support-class summary on one compatibility row.
#[derive(Debug, Clone, Deserialize)]
pub struct CompatibilitySupportSnapshot {
    /// Declared support class.
    pub declared: String,
    /// Effective support class.
    pub effective: String,
    /// Downgrade triggers observed by the report.
    #[serde(default)]
    pub downgrade_triggers_fired: Vec<String>,
}

/// Errors raised while loading the compatibility report.
#[derive(Debug)]
pub enum CompatibilityReportLoadError {
    /// Filesystem read failed.
    Io(std::io::Error),
    /// JSON parsing failed.
    Parse(serde_json::Error),
    /// The record-kind tag did not match the expected compatibility report.
    SchemaMismatch {
        /// Expected record-kind tag.
        expected_record_kind: &'static str,
        /// Actual record-kind tag.
        actual_record_kind: String,
    },
}

impl std::fmt::Display for CompatibilityReportLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "io error reading compatibility report: {err}"),
            Self::Parse(err) => write!(f, "parse error in compatibility report: {err}"),
            Self::SchemaMismatch {
                expected_record_kind,
                actual_record_kind,
            } => write!(
                f,
                "compatibility report record_kind mismatch: expected {expected_record_kind}, got {actual_record_kind}"
            ),
        }
    }
}

impl std::error::Error for CompatibilityReportLoadError {}

impl From<std::io::Error> for CompatibilityReportLoadError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for CompatibilityReportLoadError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
}

impl CompatibilityReportSnapshot {
    /// Stable expected record-kind tag on the upstream artifact.
    pub const EXPECTED_RECORD_KIND: &'static str = "compatibility_report";

    /// Load and parse the report from disk.
    pub fn load_from_path(path: impl AsRef<Path>) -> Result<Self, CompatibilityReportLoadError> {
        let bytes = std::fs::read(path)?;
        Self::from_bytes(&bytes)
    }

    /// Parse the report from raw JSON bytes.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, CompatibilityReportLoadError> {
        let snapshot: Self = serde_json::from_slice(bytes)?;
        if snapshot.record_kind != Self::EXPECTED_RECORD_KIND {
            return Err(CompatibilityReportLoadError::SchemaMismatch {
                expected_record_kind: Self::EXPECTED_RECORD_KIND,
                actual_record_kind: snapshot.record_kind,
            });
        }
        Ok(snapshot)
    }

    /// Returns true when a compatibility row id exists in the report.
    pub fn has_row(&self, row_id: &str) -> bool {
        self.rows.iter().any(|row| row.row_id == row_id)
    }
}

/// Errors raised while loading both release-truth inputs.
#[derive(Debug)]
pub enum TruthWiringLoadError {
    /// Claim manifest load failed.
    ClaimManifest(ManifestLoadError),
    /// Compatibility report load failed.
    CompatibilityReport(CompatibilityReportLoadError),
}

impl std::fmt::Display for TruthWiringLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ClaimManifest(err) => write!(f, "{err}"),
            Self::CompatibilityReport(err) => write!(f, "{err}"),
        }
    }
}

impl std::error::Error for TruthWiringLoadError {}

/// One surface binding in the consolidated truth-wiring report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SurfaceTruthBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Surface family.
    pub surface_class: TruthSurfaceClass,
    /// Stable surface token.
    pub surface_class_token: String,
    /// Reviewer-facing surface label.
    pub surface_label: String,
    /// Surface implementation or artifact ref.
    pub surface_ref: String,
    /// Claim manifest path.
    pub claim_manifest_ref: String,
    /// Compatibility report path.
    pub compatibility_report_ref: String,
    /// Manifest channel required by this surface.
    pub required_channel_id: String,
    /// Claim row ids selected for this surface.
    pub claim_row_ids: Vec<String>,
    /// Claim families selected for this surface.
    pub claim_families: Vec<String>,
    /// Compatibility row refs collected from the selected claim rows.
    pub compatibility_row_refs: Vec<String>,
    /// Compatibility row refs missing from the current compatibility report.
    pub missing_compatibility_row_refs: Vec<String>,
    /// Freshness-state tokens observed on selected claim rows.
    pub freshness_state_tokens: Vec<String>,
    /// Contract state for this surface.
    pub service_contract_state: ServiceContractState,
    /// Stable contract-state token.
    pub service_contract_state_token: String,
    /// True when one selected row has stale or expired evidence.
    pub evidence_stale: bool,
    /// True when one selected row downgraded claim posture.
    pub claim_downgraded: bool,
    /// True when one selected row downgraded support class.
    pub support_downgraded: bool,
    /// True when the selected rows carry an honesty marker.
    pub honesty_marker_present: bool,
}

impl SurfaceTruthBinding {
    /// Build the binding for one surface from a service-health surface and
    /// compatibility report snapshot.
    pub fn project(
        surface_class: TruthSurfaceClass,
        beta_surface: &ServiceHealthBetaSurface,
        compatibility: &CompatibilityReportSnapshot,
    ) -> Self {
        let selected_rows = rows_for_surface(surface_class, beta_surface);
        let mut claim_row_ids = Vec::new();
        let mut claim_families = BTreeSet::new();
        let mut compatibility_refs = BTreeSet::new();
        let mut freshness_states = BTreeSet::new();
        let mut evidence_stale = false;
        let mut claim_downgraded = false;
        let mut support_downgraded = false;
        let mut honesty_marker_present = false;
        let mut policy_blocked = false;
        let mut withdrawn = false;

        for row in selected_rows {
            claim_row_ids.push(row.row_id.clone());
            claim_families.insert(row.claim_family.clone());
            freshness_states.insert(row.freshness.state_token.clone());
            evidence_stale |= row.evidence_stale() || row.evidence_expired();
            claim_downgraded |= row.claim_posture.downgraded;
            support_downgraded |= row.support.downgraded;
            honesty_marker_present |= row.honesty_marker_present;
            policy_blocked |= matches!(
                row.claim_posture.effective,
                ClaimPostureClass::PolicyDisabled
            );
            withdrawn |= matches!(row.claim_posture.effective, ClaimPostureClass::Withdrawn);
            for compat_ref in &row.compatibility_row_refs {
                compatibility_refs.insert(compat_ref.clone());
            }
        }

        let compatibility_row_refs: Vec<String> = compatibility_refs.into_iter().collect();
        let missing_compatibility_row_refs: Vec<String> = compatibility_row_refs
            .iter()
            .filter(|row_id| !compatibility.has_row(row_id))
            .cloned()
            .collect();

        let service_contract_state =
            if claim_row_ids.is_empty() || !missing_compatibility_row_refs.is_empty() {
                ServiceContractState::ContractMismatch
            } else if policy_blocked {
                ServiceContractState::PolicyBlocked
            } else if withdrawn {
                ServiceContractState::Unavailable
            } else if evidence_stale {
                ServiceContractState::Stale
            } else if claim_downgraded || support_downgraded || honesty_marker_present {
                ServiceContractState::Degraded
            } else {
                ServiceContractState::Ready
            };

        Self {
            binding_id: format!("truth-wiring:{}:current", surface_class.as_str()),
            surface_class,
            surface_class_token: surface_class.as_str().to_owned(),
            surface_label: surface_class.label().to_owned(),
            surface_ref: surface_ref(surface_class).to_owned(),
            claim_manifest_ref: CURRENT_CLAIM_MANIFEST_REF.to_owned(),
            compatibility_report_ref: CURRENT_COMPATIBILITY_REPORT_REF.to_owned(),
            required_channel_id: surface_class.required_channel().as_str().to_owned(),
            claim_row_ids,
            claim_families: claim_families.into_iter().collect(),
            compatibility_row_refs,
            missing_compatibility_row_refs,
            freshness_state_tokens: freshness_states.into_iter().collect(),
            service_contract_state,
            service_contract_state_token: service_contract_state.as_str().to_owned(),
            evidence_stale,
            claim_downgraded,
            support_downgraded,
            honesty_marker_present,
        }
    }
}

/// Handoff request that must preserve the current object and issue context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommunityHandoffRequest {
    /// Issue class being routed.
    pub issue_class: CommunityIssueClass,
    /// Surface the issue came from.
    pub source_surface: TruthSurfaceClass,
    /// Current object ref the destination must preserve.
    pub current_object_ref: String,
    /// Issue context ref the destination must preserve.
    pub issue_context_ref: String,
    /// Claim manifest ref attached to the route.
    pub claim_manifest_ref: String,
    /// Compatibility report ref attached to the route.
    pub compatibility_report_ref: String,
}

/// One resolved community/support handoff decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommunityHandoffDecision {
    /// Stable decision id.
    pub decision_id: String,
    /// Issue class.
    pub issue_class: CommunityIssueClass,
    /// Stable issue-class token.
    pub issue_class_token: String,
    /// Source surface.
    pub source_surface: TruthSurfaceClass,
    /// Stable source-surface token.
    pub source_surface_token: String,
    /// Route class.
    pub route_class: CommunityHandoffRouteClass,
    /// Stable route-class token.
    pub route_class_token: String,
    /// Destination trust class.
    pub destination_trust_class: DestinationTrustClass,
    /// Stable destination-trust token.
    pub destination_trust_class_token: String,
    /// Auth expectation disclosed before navigation.
    pub auth_expectation: String,
    /// Data-exit boundary disclosed before navigation.
    pub data_exit_boundary: String,
    /// Issue template or local-export template ref.
    pub issue_template_ref: String,
    /// Current object ref passed through to the destination.
    pub current_object_ref: String,
    /// Issue context ref passed through to the destination.
    pub issue_context_ref: String,
    /// Claim manifest ref attached to the handoff.
    pub claim_manifest_ref: String,
    /// Compatibility report ref attached to the handoff.
    pub compatibility_report_ref: String,
    /// Redaction profile applied to the handoff.
    pub redaction_profile: String,
    /// True when both current object and issue context refs survive routing.
    pub preserves_current_object_and_issue_context: bool,
    /// True when a local support/export packet must be saved before navigation.
    pub local_export_required_before_navigation: bool,
}

/// Route a community/support issue without dropping object or issue context.
pub fn route_community_handoff(request: CommunityHandoffRequest) -> CommunityHandoffDecision {
    let (
        route_class,
        destination_trust_class,
        auth_expectation,
        data_exit_boundary,
        template_ref,
        redaction_profile,
        local_export_required,
    ) = match request.issue_class {
        CommunityIssueClass::DocsTruthMismatch => (
            CommunityHandoffRouteClass::PublicIssueTracker,
            DestinationTrustClass::OfficialPublic,
            "no sign-in required to view; sign-in may be required to comment",
            "metadata-safe object refs may leave the product after review",
            "issue-template:docs-truth-mismatch",
            "metadata_safe_default",
            false,
        ),
        CommunityIssueClass::MigrationCompatibilityRegression => (
            CommunityHandoffRouteClass::PublicIssueTracker,
            DestinationTrustClass::OfficialPublic,
            "no sign-in required to view; sign-in may be required to comment",
            "migration session refs and compatibility row refs only",
            "issue-template:migration-compatibility-regression",
            "metadata_safe_default",
            false,
        ),
        CommunityIssueClass::DesignProposal => (
            CommunityHandoffRouteClass::PublicRfcForum,
            DestinationTrustClass::Community,
            "public forum account may be required",
            "proposal refs only; no local diagnostics attached automatically",
            "issue-template:public-rfc-proposal",
            "public_summary_only",
            false,
        ),
        CommunityIssueClass::SecuritySensitive => (
            CommunityHandoffRouteClass::PrivateSecurityChannel,
            DestinationTrustClass::OfficialAuthenticated,
            "security intake identity required",
            "security payloads leave only through the private security lane",
            "issue-template:private-security-intake",
            "security_private_default",
            true,
        ),
        CommunityIssueClass::PrivateWorkspaceSupport => (
            CommunityHandoffRouteClass::PrivateSupportChannel,
            DestinationTrustClass::OfficialAuthenticated,
            "support identity required",
            "redacted support packet leaves only after local preview",
            "issue-template:private-support-intake",
            "support_redacted_default",
            true,
        ),
    };

    let preserves_current_object_and_issue_context = !request.current_object_ref.trim().is_empty()
        && !request.issue_context_ref.trim().is_empty();

    CommunityHandoffDecision {
        decision_id: format!(
            "community-handoff:{}:{}",
            request.source_surface.as_str(),
            request.issue_class.as_str()
        ),
        issue_class: request.issue_class,
        issue_class_token: request.issue_class.as_str().to_owned(),
        source_surface: request.source_surface,
        source_surface_token: request.source_surface.as_str().to_owned(),
        route_class,
        route_class_token: route_class.as_str().to_owned(),
        destination_trust_class,
        destination_trust_class_token: destination_trust_class.as_str().to_owned(),
        auth_expectation: auth_expectation.to_owned(),
        data_exit_boundary: data_exit_boundary.to_owned(),
        issue_template_ref: template_ref.to_owned(),
        current_object_ref: request.current_object_ref,
        issue_context_ref: request.issue_context_ref,
        claim_manifest_ref: request.claim_manifest_ref,
        compatibility_report_ref: request.compatibility_report_ref,
        redaction_profile: redaction_profile.to_owned(),
        preserves_current_object_and_issue_context,
        local_export_required_before_navigation: local_export_required,
    }
}

/// Defect class emitted by the consolidated truth-wiring report.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TruthWiringDefectKind {
    /// A surface selected no claim rows.
    SurfaceMissingClaimRows,
    /// A selected compatibility row ref did not resolve in the report.
    MissingCompatibilityRow,
    /// A handoff route dropped current object or issue context.
    HandoffContextMissing,
}

impl TruthWiringDefectKind {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SurfaceMissingClaimRows => "surface_missing_claim_rows",
            Self::MissingCompatibilityRow => "missing_compatibility_row",
            Self::HandoffContextMissing => "handoff_context_missing",
        }
    }
}

/// One defect in the consolidated truth-wiring report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TruthWiringDefect {
    /// Stable defect id.
    pub defect_id: String,
    /// Defect kind.
    pub defect_kind: TruthWiringDefectKind,
    /// Stable defect-kind token.
    pub defect_kind_token: String,
    /// Affected surface token.
    pub surface_class_token: String,
    /// Field that failed.
    pub field: String,
    /// Reviewer-facing note.
    pub note: String,
}

impl TruthWiringDefect {
    fn new(
        kind: TruthWiringDefectKind,
        surface_class: TruthSurfaceClass,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            defect_id: format!(
                "truth-wiring:defect:{}:{}",
                surface_class.as_str(),
                kind.as_str()
            ),
            defect_kind: kind,
            defect_kind_token: kind.as_str().to_owned(),
            surface_class_token: surface_class.as_str().to_owned(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Summary counters for the consolidated report.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct TruthWiringSummary {
    /// Number of surface bindings checked.
    pub surface_count: u32,
    /// Bindings in `ready`.
    pub ready_count: u32,
    /// Bindings in `degraded`.
    pub degraded_count: u32,
    /// Bindings in `stale`.
    pub stale_count: u32,
    /// Bindings in `contract_mismatch`.
    pub contract_mismatch_count: u32,
    /// Handoff route decisions checked.
    pub handoff_route_count: u32,
    /// Handoff routes preserving object and issue context.
    pub handoff_context_preserved_count: u32,
    /// Defects emitted.
    pub defect_count: u32,
}

/// Consolidated docs/help truth-wiring report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TruthWiringReport {
    /// Stable record-kind tag.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Claim manifest id.
    pub manifest_id: String,
    /// Claim manifest revision.
    pub manifest_revision: u32,
    /// Claim manifest state.
    pub manifest_state_token: String,
    /// Claim manifest source path.
    pub claim_manifest_ref: String,
    /// Compatibility report id.
    pub compatibility_report_id: String,
    /// Compatibility report revision.
    pub compatibility_report_revision: u32,
    /// Compatibility report state.
    pub compatibility_report_state: String,
    /// Compatibility report source path.
    pub compatibility_report_ref: String,
    /// Evaluation date copied from the claim manifest.
    pub as_of: String,
    /// Generation timestamp copied from the claim manifest.
    pub generated_at: String,
    /// Surface bindings.
    pub surface_bindings: Vec<SurfaceTruthBinding>,
    /// Handoff routing decisions.
    pub handoff_routes: Vec<CommunityHandoffDecision>,
    /// Defects observed by the report.
    pub defects: Vec<TruthWiringDefect>,
    /// Summary counters.
    pub summary: TruthWiringSummary,
}

impl TruthWiringReport {
    /// Project the consolidated report from parsed claim and compatibility inputs.
    pub fn project(
        beta_surface: &ServiceHealthBetaSurface,
        compatibility: &CompatibilityReportSnapshot,
    ) -> Self {
        let surface_bindings: Vec<SurfaceTruthBinding> = TruthSurfaceClass::all()
            .into_iter()
            .map(|surface_class| {
                SurfaceTruthBinding::project(surface_class, beta_surface, compatibility)
            })
            .collect();
        let handoff_routes = seeded_handoff_routes(beta_surface, compatibility);
        let defects = compute_defects(&surface_bindings, &handoff_routes);
        let summary = compute_truth_summary(&surface_bindings, &handoff_routes, &defects);

        Self {
            record_kind: TRUTH_WIRING_REPORT_RECORD_KIND.to_owned(),
            schema_version: TRUTH_WIRING_REPORT_SCHEMA_VERSION,
            manifest_id: beta_surface.manifest_id.clone(),
            manifest_revision: beta_surface.manifest_revision,
            manifest_state_token: beta_surface.manifest_state_token.clone(),
            claim_manifest_ref: CURRENT_CLAIM_MANIFEST_REF.to_owned(),
            compatibility_report_id: compatibility.report_id.clone(),
            compatibility_report_revision: compatibility.report_revision,
            compatibility_report_state: compatibility.report_state.clone(),
            compatibility_report_ref: CURRENT_COMPATIBILITY_REPORT_REF.to_owned(),
            as_of: beta_surface.as_of.clone(),
            generated_at: beta_surface.generated_at.clone(),
            surface_bindings,
            handoff_routes,
            defects,
            summary,
        }
    }

    /// Returns the binding for a surface class.
    pub fn binding_for(&self, surface_class: TruthSurfaceClass) -> Option<&SurfaceTruthBinding> {
        self.surface_bindings
            .iter()
            .find(|binding| binding.surface_class == surface_class)
    }

    /// Render a deterministic Markdown report.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# Beta truth wiring report\n\n");
        out.push_str("This report joins the current claim manifest to the current compatibility report for docs/help truth surfaces. It is read-only: degraded or stale proof narrows the surface state instead of widening copy.\n\n");
        out.push_str("## Inputs\n\n");
        out.push_str(&format!(
            "- Claim manifest: `{}` (`{}`, rev {}, state `{}`)\n",
            self.claim_manifest_ref,
            self.manifest_id,
            self.manifest_revision,
            self.manifest_state_token
        ));
        out.push_str(&format!(
            "- Compatibility report: `{}` (`{}`, rev {}, state `{}`)\n",
            self.compatibility_report_ref,
            self.compatibility_report_id,
            self.compatibility_report_revision,
            self.compatibility_report_state
        ));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Surface Bindings\n\n");
        out.push_str("| Surface | State | Claim rows | Compatibility rows | Missing compatibility rows | Freshness | Honesty |\n");
        out.push_str("|---|---|---:|---:|---|---|---|\n");
        for binding in &self.surface_bindings {
            out.push_str(&format!(
                "| {} | `{}` | {} | {} | {} | {} | {} |\n",
                binding.surface_label,
                binding.service_contract_state_token,
                binding.claim_row_ids.len(),
                binding.compatibility_row_refs.len(),
                render_list_or_none(&binding.missing_compatibility_row_refs),
                render_list_or_none(&binding.freshness_state_tokens),
                if binding.honesty_marker_present {
                    "present"
                } else {
                    "none"
                },
            ));
        }

        out.push_str("\n## Surface Details\n\n");
        for binding in &self.surface_bindings {
            out.push_str(&format!("### {}\n\n", binding.surface_label));
            out.push_str(&format!("- Surface ref: `{}`\n", binding.surface_ref));
            out.push_str(&format!(
                "- Required channel: `{}`\n",
                binding.required_channel_id
            ));
            out.push_str(&format!(
                "- Contract state: `{}`\n",
                binding.service_contract_state_token
            ));
            out.push_str(&format!(
                "- Claim families: {}\n",
                render_list_or_none(&binding.claim_families)
            ));
            out.push_str(&format!(
                "- Claim rows: {}\n",
                render_list_or_none(&binding.claim_row_ids)
            ));
            out.push_str(&format!(
                "- Compatibility rows: {}\n\n",
                render_list_or_none(&binding.compatibility_row_refs)
            ));
        }

        out.push_str("## Community Handoff\n\n");
        out.push_str("| Issue | Route | Trust | Source | Context preserved | Template |\n");
        out.push_str("|---|---|---|---|---|---|\n");
        for route in &self.handoff_routes {
            out.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | {} | `{}` |\n",
                route.issue_class_token,
                route.route_class_token,
                route.destination_trust_class_token,
                route.source_surface_token,
                if route.preserves_current_object_and_issue_context {
                    "yes"
                } else {
                    "no"
                },
                route.issue_template_ref,
            ));
        }

        out.push_str("\n## Findings\n\n");
        if self.defects.is_empty() {
            out.push_str("_All checked surface bindings resolve their claim rows and compatibility rows, and all seeded handoff routes preserve object and issue context._\n");
        } else {
            for defect in &self.defects {
                out.push_str(&format!(
                    "- `{}` on `{}` field `{}`: {}\n",
                    defect.defect_kind_token, defect.surface_class_token, defect.field, defect.note,
                ));
            }
        }
        out.push('\n');
        out.push_str("## Refresh\n\n");
        out.push_str("Run `cargo test -p aureline-shell --lib docs_browser::truth_wiring` after refreshing the claim manifest or compatibility report. The checked-in report should be regenerated from `TruthWiringReport::render_markdown()` when either input changes.\n");
        out
    }
}

/// Load the checked-in claim and compatibility artifacts and project the report.
pub fn current_truth_wiring_report() -> Result<TruthWiringReport, TruthWiringLoadError> {
    let manifest = M3ClaimManifestSnapshot::from_bytes(CURRENT_CLAIM_MANIFEST_BYTES)
        .map_err(TruthWiringLoadError::ClaimManifest)?;
    let beta_surface = ServiceHealthBetaSurface::project_at_manifest_as_of(&manifest);
    let compatibility = CompatibilityReportSnapshot::from_bytes(CURRENT_COMPATIBILITY_REPORT_BYTES)
        .map_err(TruthWiringLoadError::CompatibilityReport)?;
    Ok(TruthWiringReport::project(&beta_surface, &compatibility))
}

/// Project the checked-in report, panicking if the checked-in artifacts are invalid.
pub fn seeded_truth_wiring_report() -> TruthWiringReport {
    current_truth_wiring_report().expect("checked-in release truth artifacts must parse")
}

fn rows_for_surface<'a>(
    surface_class: TruthSurfaceClass,
    beta_surface: &'a ServiceHealthBetaSurface,
) -> Vec<&'a ServiceHealthBetaRow> {
    match surface_class {
        TruthSurfaceClass::DocsBrowser => beta_surface
            .rows
            .iter()
            .filter(|row| {
                row.claim_family == "docs_freshness"
                    && projection_required(row, ManifestChannelId::DocsSite)
            })
            .collect(),
        TruthSurfaceClass::MigrationCenter => beta_surface
            .rows
            .iter()
            .filter(|row| {
                row.row_id == "m3_claim_row:beta_surface.importer_and_migration"
                    && projection_required(row, ManifestChannelId::MigrationNotes)
            })
            .collect(),
        TruthSurfaceClass::HelpAbout => beta_surface.rows_for_help_about(),
        TruthSurfaceClass::ServiceHealth => beta_surface.rows_for_service_health(),
    }
}

fn projection_required(row: &ServiceHealthBetaRow, channel: ManifestChannelId) -> bool {
    row.channel_projections.iter().any(|projection| {
        projection.channel_class == Some(channel) && projection.binding_status == "required"
    })
}

fn surface_ref(surface_class: TruthSurfaceClass) -> &'static str {
    match surface_class {
        TruthSurfaceClass::DocsBrowser => "crates/aureline-shell/src/docs_browser/",
        TruthSurfaceClass::MigrationCenter => "crates/aureline-shell/src/migration_center/mod.rs",
        TruthSurfaceClass::HelpAbout => "crates/aureline-shell/src/about/mod.rs",
        TruthSurfaceClass::ServiceHealth => "crates/aureline-shell/src/service_health/mod.rs",
    }
}

fn seeded_handoff_routes(
    beta_surface: &ServiceHealthBetaSurface,
    compatibility: &CompatibilityReportSnapshot,
) -> Vec<CommunityHandoffDecision> {
    let claim_manifest_ref = CURRENT_CLAIM_MANIFEST_REF.to_owned();
    let compatibility_report_ref = CURRENT_COMPATIBILITY_REPORT_REF.to_owned();
    let mut requests = Vec::new();
    requests.push(CommunityHandoffRequest {
        issue_class: CommunityIssueClass::DocsTruthMismatch,
        source_surface: TruthSurfaceClass::DocsBrowser,
        current_object_ref: "docs:help:m3:docs_browser_contract".to_owned(),
        issue_context_ref: "m3_claim_row:canonical.docs.freshness_truth".to_owned(),
        claim_manifest_ref: claim_manifest_ref.clone(),
        compatibility_report_ref: compatibility_report_ref.clone(),
    });
    requests.push(CommunityHandoffRequest {
        issue_class: CommunityIssueClass::MigrationCompatibilityRegression,
        source_surface: TruthSurfaceClass::MigrationCenter,
        current_object_ref: "shell:migration_center_beta:page:v1".to_owned(),
        issue_context_ref: "m3_claim_row:beta_surface.importer_and_migration".to_owned(),
        claim_manifest_ref: claim_manifest_ref.clone(),
        compatibility_report_ref: compatibility_report_ref.clone(),
    });
    requests.push(CommunityHandoffRequest {
        issue_class: CommunityIssueClass::DesignProposal,
        source_surface: TruthSurfaceClass::HelpAbout,
        current_object_ref: "help-about:community-handoff".to_owned(),
        issue_context_ref: beta_surface.manifest_id.clone(),
        claim_manifest_ref: claim_manifest_ref.clone(),
        compatibility_report_ref: compatibility_report_ref.clone(),
    });
    requests.push(CommunityHandoffRequest {
        issue_class: CommunityIssueClass::SecuritySensitive,
        source_surface: TruthSurfaceClass::HelpAbout,
        current_object_ref: "help-about:private-security-channel".to_owned(),
        issue_context_ref: "issue-context:security-sensitive".to_owned(),
        claim_manifest_ref: claim_manifest_ref.clone(),
        compatibility_report_ref: compatibility_report_ref.clone(),
    });
    requests.push(CommunityHandoffRequest {
        issue_class: CommunityIssueClass::PrivateWorkspaceSupport,
        source_surface: TruthSurfaceClass::ServiceHealth,
        current_object_ref: format!("service-health:{}", beta_surface.manifest_id),
        issue_context_ref: compatibility.report_id.clone(),
        claim_manifest_ref,
        compatibility_report_ref,
    });
    requests.into_iter().map(route_community_handoff).collect()
}

fn compute_defects(
    bindings: &[SurfaceTruthBinding],
    handoff_routes: &[CommunityHandoffDecision],
) -> Vec<TruthWiringDefect> {
    let mut defects = Vec::new();
    for binding in bindings {
        if binding.claim_row_ids.is_empty() {
            defects.push(TruthWiringDefect::new(
                TruthWiringDefectKind::SurfaceMissingClaimRows,
                binding.surface_class,
                "claim_row_ids",
                "surface selected no claim rows from the current claim manifest",
            ));
        }
        for missing in &binding.missing_compatibility_row_refs {
            defects.push(TruthWiringDefect::new(
                TruthWiringDefectKind::MissingCompatibilityRow,
                binding.surface_class,
                "compatibility_row_refs",
                format!("compatibility row `{missing}` is not present in the current report"),
            ));
        }
    }
    for route in handoff_routes {
        if !route.preserves_current_object_and_issue_context {
            defects.push(TruthWiringDefect::new(
                TruthWiringDefectKind::HandoffContextMissing,
                route.source_surface,
                "community_handoff",
                "handoff route dropped the current object or issue context",
            ));
        }
    }
    defects
}

fn compute_truth_summary(
    bindings: &[SurfaceTruthBinding],
    handoff_routes: &[CommunityHandoffDecision],
    defects: &[TruthWiringDefect],
) -> TruthWiringSummary {
    let mut counts: BTreeMap<ServiceContractState, u32> = BTreeMap::new();
    for binding in bindings {
        *counts.entry(binding.service_contract_state).or_default() += 1;
    }
    TruthWiringSummary {
        surface_count: bindings.len() as u32,
        ready_count: *counts.get(&ServiceContractState::Ready).unwrap_or(&0),
        degraded_count: *counts.get(&ServiceContractState::Degraded).unwrap_or(&0),
        stale_count: *counts.get(&ServiceContractState::Stale).unwrap_or(&0),
        contract_mismatch_count: *counts
            .get(&ServiceContractState::ContractMismatch)
            .unwrap_or(&0),
        handoff_route_count: handoff_routes.len() as u32,
        handoff_context_preserved_count: handoff_routes
            .iter()
            .filter(|route| route.preserves_current_object_and_issue_context)
            .count() as u32,
        defect_count: defects.len() as u32,
    }
}

fn render_list_or_none(values: &[String]) -> String {
    if values.is_empty() {
        "_(none)_".to_owned()
    } else {
        values
            .iter()
            .map(|value| format!("`{value}`"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_report_resolves_claim_and_compatibility_rows() {
        let report = seeded_truth_wiring_report();
        assert_eq!(report.record_kind, TRUTH_WIRING_REPORT_RECORD_KIND);
        assert_eq!(report.summary.surface_count, 4);
        assert_eq!(report.summary.contract_mismatch_count, 0);
        assert!(report.defects.is_empty());

        for binding in &report.surface_bindings {
            assert!(
                !binding.claim_row_ids.is_empty(),
                "{} must select claim rows",
                binding.surface_class_token
            );
            assert!(
                !binding.compatibility_row_refs.is_empty(),
                "{} must select compatibility rows",
                binding.surface_class_token
            );
            assert!(
                binding.missing_compatibility_row_refs.is_empty(),
                "{} has missing compatibility refs: {:?}",
                binding.surface_class_token,
                binding.missing_compatibility_row_refs,
            );
        }
    }

    #[test]
    fn handoff_routes_preserve_object_and_context() {
        let report = seeded_truth_wiring_report();
        assert_eq!(
            report.summary.handoff_context_preserved_count,
            report.summary.handoff_route_count
        );
        let security = report
            .handoff_routes
            .iter()
            .find(|route| route.issue_class == CommunityIssueClass::SecuritySensitive)
            .expect("security handoff seeded");
        assert_eq!(
            security.route_class,
            CommunityHandoffRouteClass::PrivateSecurityChannel
        );
        assert_eq!(
            security.destination_trust_class,
            DestinationTrustClass::OfficialAuthenticated
        );
        assert!(security.local_export_required_before_navigation);
        assert!(security.preserves_current_object_and_issue_context);
    }

    #[test]
    fn markdown_mentions_all_surface_bindings() {
        let report = seeded_truth_wiring_report();
        let markdown = report.render_markdown();
        for surface_class in TruthSurfaceClass::all() {
            assert!(
                markdown.contains(surface_class.label()),
                "markdown must mention {}",
                surface_class.label()
            );
        }
        assert!(markdown.contains("## Community Handoff"));
        assert!(markdown.contains("_All checked surface bindings resolve"));
    }

    #[test]
    fn compatibility_report_rejects_wrong_record_kind() {
        let payload = br#"{
            "record_kind": "not_compat",
            "report_id": "compat_report:x",
            "report_revision": 1,
            "report_state": "draft",
            "as_of": "2026-05-15",
            "generated_at": "2026-05-15T00:00:00Z",
            "owner": "@me",
            "rows": []
        }"#;
        let err = CompatibilityReportSnapshot::from_bytes(payload).unwrap_err();
        assert!(matches!(
            err,
            CompatibilityReportLoadError::SchemaMismatch { .. }
        ));
    }
}
