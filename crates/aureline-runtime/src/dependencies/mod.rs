//! Dependency intelligence records for advisory, lockfile, and release-debt review.
//!
//! This module owns metadata-only records that normalize manifest, lockfile,
//! workspace-path, and registry dependency facts into one graph. The same
//! records carry advisory truth, suppression state, lockfile mutation previews,
//! and release-visible debt summaries across local, mirrored, and offline
//! deployments.

use serde::{Deserialize, Serialize};

use crate::packages::{
    LockfileMutationMode, ManifestDeltaClass, MirrorOrOfflineStateClass,
    PackageOperationAlphaPacket, PackageOperationClass, PackageRedactionClass, ValidationTaskClass,
};

/// Schema version for dependency-intelligence records.
pub const DEPENDENCY_INTELLIGENCE_SCHEMA_VERSION: u32 = 1;
/// Runtime implementation version quoted by dependency-intelligence records.
pub const DEPENDENCY_INTELLIGENCE_REVIEWER_VERSION: &str =
    "dependency_intelligence.reviewer.beta.v1";
/// Stable record-kind tag for [`DependencyRecord`].
pub const DEPENDENCY_RECORD_KIND: &str = "dependency_record";
/// Stable record-kind tag for [`DependencyGraphRecord`].
pub const DEPENDENCY_GRAPH_RECORD_KIND: &str = "dependency_graph_record";
/// Stable record-kind tag for [`DependencyAdvisoryRecord`].
pub const DEPENDENCY_ADVISORY_RECORD_KIND: &str = "dependency_advisory_record";
/// Stable record-kind tag for [`LockfileMutationPreview`].
pub const LOCKFILE_MUTATION_PREVIEW_RECORD_KIND: &str = "lockfile_mutation_preview";
/// Stable record-kind tag for [`DependencyDebtPacket`].
pub const DEPENDENCY_DEBT_PACKET_RECORD_KIND: &str = "dependency_debt_packet";

/// Relationship class for a dependency node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyRelationshipClass {
    /// Dependency is declared directly by the owning manifest.
    Direct,
    /// Dependency is introduced through another dependency.
    Transitive,
    /// Dependency resolves to another local workspace member.
    WorkspaceLocal,
    /// Dependency resolves through a path or VCS source.
    PathOrVcsSource,
}

impl DependencyRelationshipClass {
    /// Returns the stable schema token for this relationship class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Direct => "direct",
            Self::Transitive => "transitive",
            Self::WorkspaceLocal => "workspace_local",
            Self::PathOrVcsSource => "path_or_vcs_source",
        }
    }
}

/// Source class for a dependency node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencySourceClass {
    /// Dependency was read from a manifest.
    ManifestDependency,
    /// Dependency was read from a lockfile node.
    LockfileNode,
    /// Dependency resolves from a workspace path.
    WorkspacePathDependency,
    /// Dependency resolves from a registry.
    RegistryDependency,
    /// Dependency resolves from a registry mirror.
    MirrorRegistryDependency,
    /// Dependency resolves from a VCS source.
    VcsDependency,
    /// Dependency came from an imported inventory or report.
    ImportedInventory,
    /// Dependency came from an offline bundle.
    OfflineBundleDependency,
}

impl DependencySourceClass {
    /// Returns the stable schema token for this source class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManifestDependency => "manifest_dependency",
            Self::LockfileNode => "lockfile_node",
            Self::WorkspacePathDependency => "workspace_path_dependency",
            Self::RegistryDependency => "registry_dependency",
            Self::MirrorRegistryDependency => "mirror_registry_dependency",
            Self::VcsDependency => "vcs_dependency",
            Self::ImportedInventory => "imported_inventory",
            Self::OfflineBundleDependency => "offline_bundle_dependency",
        }
    }
}

/// Freshness class for dependency, license, and advisory evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyFreshnessClass {
    /// Current local analysis produced the record.
    FreshLocalAnalysis,
    /// Imported feed data produced the record.
    ImportedFeedData,
    /// Mirror data produced the record.
    MirroredData,
    /// Offline or stale data produced the record.
    StaleOfflineState,
    /// Freshness could not be proven and requires review.
    FreshnessUnknownRequiresReview,
}

impl DependencyFreshnessClass {
    /// Returns the stable schema token for this freshness class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshLocalAnalysis => "fresh_local_analysis",
            Self::ImportedFeedData => "imported_feed_data",
            Self::MirroredData => "mirrored_data",
            Self::StaleOfflineState => "stale_offline_state",
            Self::FreshnessUnknownRequiresReview => "freshness_unknown_requires_review",
        }
    }
}

/// Provenance class for a dependency fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyProvenanceClass {
    /// Fact was declared by a manifest.
    ManifestDeclared,
    /// Fact was resolved by a lockfile.
    LockfileResolved,
    /// Fact was resolved through a local workspace path.
    WorkspacePathResolved,
    /// Fact came from registry metadata.
    RegistryMetadata,
    /// Fact came from a mirror snapshot.
    MirrorSnapshot,
    /// Fact came from an offline bundle.
    OfflineBundle,
    /// Fact came from an imported report.
    ImportedReport,
    /// Fact was inferred from a transitive edge.
    InferredTransitive,
}

impl DependencyProvenanceClass {
    /// Returns the stable schema token for this provenance class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManifestDeclared => "manifest_declared",
            Self::LockfileResolved => "lockfile_resolved",
            Self::WorkspacePathResolved => "workspace_path_resolved",
            Self::RegistryMetadata => "registry_metadata",
            Self::MirrorSnapshot => "mirror_snapshot",
            Self::OfflineBundle => "offline_bundle",
            Self::ImportedReport => "imported_report",
            Self::InferredTransitive => "inferred_transitive",
        }
    }
}

/// Resolution class for requested and resolved dependency truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyResolutionClass {
    /// Only the requested range or source is known.
    RequestedOnly,
    /// Requested range and resolved exact version are both known.
    RequestedAndResolved,
    /// Lockfile gives an exact resolved version.
    ResolvedExactLockfile,
    /// Policy pins version or source choice.
    PolicyPinned,
    /// Resolution state is stale or unknown.
    UnknownStale,
}

impl DependencyResolutionClass {
    /// Returns the stable schema token for this resolution class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestedOnly => "requested_only",
            Self::RequestedAndResolved => "requested_and_resolved",
            Self::ResolvedExactLockfile => "resolved_exact_lockfile",
            Self::PolicyPinned => "policy_pinned",
            Self::UnknownStale => "unknown_stale",
        }
    }
}

/// License decision class for a dependency node.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LicenseDecisionClass {
    /// License is allowed by policy.
    Allowed,
    /// License is allowed and requires notice output.
    AllowedWithNotice,
    /// License needs human review.
    ReviewRequired,
    /// License is denied by policy.
    DeniedByPolicy,
    /// License state could not be proven.
    UnknownRequiresReview,
}

impl LicenseDecisionClass {
    /// Returns the stable schema token for this license decision.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::AllowedWithNotice => "allowed_with_notice",
            Self::ReviewRequired => "review_required",
            Self::DeniedByPolicy => "denied_by_policy",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }
}

/// Source class for advisory evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisorySourceClass {
    /// Local scanner or resolver produced advisory evidence.
    LocalScanner,
    /// Public advisory feed produced advisory evidence.
    PublicAdvisoryFeed,
    /// Enterprise mirror produced advisory evidence.
    EnterpriseMirror,
    /// Offline advisory bundle produced advisory evidence.
    OfflineBundle,
    /// Imported scanner report produced advisory evidence.
    ImportedScannerReport,
    /// Stale local cache produced advisory evidence.
    StaleCache,
}

impl AdvisorySourceClass {
    /// Returns the stable schema token for this source class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalScanner => "local_scanner",
            Self::PublicAdvisoryFeed => "public_advisory_feed",
            Self::EnterpriseMirror => "enterprise_mirror",
            Self::OfflineBundle => "offline_bundle",
            Self::ImportedScannerReport => "imported_scanner_report",
            Self::StaleCache => "stale_cache",
        }
    }
}

/// Truth class for advisory evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisoryTruthClass {
    /// Current local analysis confirmed the advisory state.
    FreshLocalAnalysis,
    /// Imported feed data supplies the advisory state.
    ImportedFeedData,
    /// Mirrored data supplies the advisory state.
    MirroredData,
    /// Stale or offline data supplies the advisory state.
    StaleOfflineState,
}

impl AdvisoryTruthClass {
    /// Returns the stable schema token for this advisory truth class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FreshLocalAnalysis => "fresh_local_analysis",
            Self::ImportedFeedData => "imported_feed_data",
            Self::MirroredData => "mirrored_data",
            Self::StaleOfflineState => "stale_offline_state",
        }
    }
}

/// Severity class for advisory records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisorySeverityClass {
    /// Low severity advisory.
    Low,
    /// Moderate severity advisory.
    Moderate,
    /// High severity advisory.
    High,
    /// Critical severity advisory.
    Critical,
    /// Severity could not be proven.
    Unknown,
}

impl AdvisorySeverityClass {
    /// Returns the stable schema token for this severity class.
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

/// Lifecycle class for advisory records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdvisoryLifecycleClass {
    /// Advisory is active.
    Active,
    /// Advisory is active but narrowed by a reviewed suppression.
    SuppressedUntilExpiry,
    /// Advisory has been remediated.
    Remediated,
    /// Advisory feed is stale and needs review.
    StaleFeedNeedsReview,
    /// Advisory was withdrawn.
    Withdrawn,
}

impl AdvisoryLifecycleClass {
    /// Returns the stable schema token for this lifecycle class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::SuppressedUntilExpiry => "suppressed_until_expiry",
            Self::Remediated => "remediated",
            Self::StaleFeedNeedsReview => "stale_feed_needs_review",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// Returns true when the advisory remains release-visible debt.
    pub const fn is_release_visible(self) -> bool {
        matches!(
            self,
            Self::Active | Self::SuppressedUntilExpiry | Self::StaleFeedNeedsReview
        )
    }
}

/// Suppression state for an advisory or dependency finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SuppressionStateClass {
    /// No suppression applies.
    NotSuppressed,
    /// Suppression is active and time-bounded.
    ActiveTimeBound,
    /// Suppression expired and the finding reopened.
    ExpiredReopened,
    /// Suppression is locked by policy.
    PolicyLocked,
}

impl SuppressionStateClass {
    /// Returns the stable schema token for this suppression state.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotSuppressed => "not_suppressed",
            Self::ActiveTimeBound => "active_time_bound",
            Self::ExpiredReopened => "expired_reopened",
            Self::PolicyLocked => "policy_locked",
        }
    }

    /// Returns true when the suppression narrows current finding visibility.
    pub const fn is_active(self) -> bool {
        matches!(self, Self::ActiveTimeBound | Self::PolicyLocked)
    }
}

/// Action class for an explicit lockfile mutation preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockfilePreviewActionClass {
    /// Preview for installing a dependency.
    PackageInstall,
    /// Preview for updating a dependency.
    PackageUpdate,
    /// Preview for removing a dependency.
    PackageRemove,
    /// Preview for advisory remediation.
    AdvisoryRemediation,
    /// Preview for automation or AI proposed package action.
    AutomationOrAiPackageAction,
    /// Preview for a lockfile refresh.
    LockfileRefresh,
    /// Preview for read-only lockfile inspection.
    AuditOnlyNoMutation,
}

impl LockfilePreviewActionClass {
    /// Returns the stable schema token for this action class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PackageInstall => "package_install",
            Self::PackageUpdate => "package_update",
            Self::PackageRemove => "package_remove",
            Self::AdvisoryRemediation => "advisory_remediation",
            Self::AutomationOrAiPackageAction => "automation_or_ai_package_action",
            Self::LockfileRefresh => "lockfile_refresh",
            Self::AuditOnlyNoMutation => "audit_only_no_mutation",
        }
    }

    /// Returns true when the action can write a lockfile.
    pub const fn is_mutating(self) -> bool {
        !matches!(self, Self::AuditOnlyNoMutation)
    }
}

impl From<PackageOperationClass> for LockfilePreviewActionClass {
    fn from(value: PackageOperationClass) -> Self {
        match value {
            PackageOperationClass::InstallNewDependency => Self::PackageInstall,
            PackageOperationClass::UpgradeExistingDependency => Self::PackageUpdate,
            PackageOperationClass::RemoveExistingDependency => Self::PackageRemove,
            PackageOperationClass::AuditOnlyNoStateChange => Self::AuditOnlyNoMutation,
            PackageOperationClass::RestoreLockfileToCheckpoint => Self::LockfileRefresh,
        }
    }
}

/// Outcome class for a lockfile mutation preview.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LockfilePreviewOutcomeClass {
    /// Preview is complete and apply remains pending.
    PreviewReadyApplyPending,
    /// Preview is blocked because no checkpoint is present.
    PreviewBlockedMissingCheckpoint,
    /// Preview is blocked because lockfile impact is unknown.
    PreviewBlockedUnknownImpact,
    /// Preview is read-only and reports inspection only.
    PreviewReportedReadOnly,
}

impl LockfilePreviewOutcomeClass {
    /// Returns the stable schema token for this outcome class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreviewReadyApplyPending => "preview_ready_apply_pending",
            Self::PreviewBlockedMissingCheckpoint => "preview_blocked_missing_checkpoint",
            Self::PreviewBlockedUnknownImpact => "preview_blocked_unknown_impact",
            Self::PreviewReportedReadOnly => "preview_reported_read_only",
        }
    }
}

/// Debt kind class for release-visible dependency packets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyDebtKindClass {
    /// Active advisory remains unresolved.
    ActiveAdvisory,
    /// Active suppression narrows finding visibility.
    ActiveSuppression,
    /// Expired suppression reopened a finding.
    ExpiredSuppression,
    /// License or notice implication remains unresolved.
    UnresolvedLicenseNotice,
    /// Advisory source is stale or offline.
    StaleAdvisorySource,
    /// Lockfile mutation preview is missing or blocked.
    LockfilePreviewMissing,
}

impl DependencyDebtKindClass {
    /// Returns the stable schema token for this debt kind.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActiveAdvisory => "active_advisory",
            Self::ActiveSuppression => "active_suppression",
            Self::ExpiredSuppression => "expired_suppression",
            Self::UnresolvedLicenseNotice => "unresolved_license_notice",
            Self::StaleAdvisorySource => "stale_advisory_source",
            Self::LockfilePreviewMissing => "lockfile_preview_missing",
        }
    }
}

/// Release visibility class for dependency debt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DebtReleaseVisibilityClass {
    /// Debt is visible in beta release packets.
    BetaReleaseVisible,
    /// Debt is visible in support packets.
    SupportPacketVisible,
    /// Debt is internal-review only.
    InternalReviewOnly,
}

impl DebtReleaseVisibilityClass {
    /// Returns the stable schema token for this release visibility class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BetaReleaseVisible => "beta_release_visible",
            Self::SupportPacketVisible => "support_packet_visible",
            Self::InternalReviewOnly => "internal_review_only",
        }
    }

    /// Returns true when the row must appear in release or support evidence.
    pub const fn is_release_or_support_visible(self) -> bool {
        matches!(self, Self::BetaReleaseVisible | Self::SupportPacketVisible)
    }
}

/// Validation issue emitted by dependency-intelligence records.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyIntelligenceViolation {
    /// Graph has no dependency nodes.
    GraphHasNoDependencies,
    /// Advisory has no affected dependency refs.
    AdvisoryMissingAffectedDependency,
    /// Mirrored advisory is missing a mirror snapshot ref.
    MirroredAdvisoryMissingSnapshot,
    /// Stale advisory is missing a stale reason.
    StaleAdvisoryMissingReason,
    /// Active suppression is missing an expiry.
    ActiveSuppressionMissingExpiry,
    /// Lockfile preview has no affected manifest or lockfile refs.
    LockfilePreviewMissingAffectedFiles,
    /// Lockfile preview is mutating without a rollback checkpoint.
    LockfilePreviewMissingCheckpoint,
    /// Lockfile preview is not attributable.
    LockfilePreviewNotAttributable,
    /// Debt packet has no release-visible rows.
    DebtPacketNotReleaseVisible,
    /// License debt row is missing a notice or license implication ref.
    DebtPacketMissingNoticeImplicationForLicenseDebt,
}

impl DependencyIntelligenceViolation {
    /// Returns the stable schema token for this violation.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GraphHasNoDependencies => "graph_has_no_dependencies",
            Self::AdvisoryMissingAffectedDependency => "advisory_missing_affected_dependency",
            Self::MirroredAdvisoryMissingSnapshot => "mirrored_advisory_missing_snapshot",
            Self::StaleAdvisoryMissingReason => "stale_advisory_missing_reason",
            Self::ActiveSuppressionMissingExpiry => "active_suppression_missing_expiry",
            Self::LockfilePreviewMissingAffectedFiles => "lockfile_preview_missing_affected_files",
            Self::LockfilePreviewMissingCheckpoint => "lockfile_preview_missing_checkpoint",
            Self::LockfilePreviewNotAttributable => "lockfile_preview_not_attributable",
            Self::DebtPacketNotReleaseVisible => "debt_packet_not_release_visible",
            Self::DebtPacketMissingNoticeImplicationForLicenseDebt => {
                "debt_packet_missing_notice_implication_for_license_debt"
            }
        }
    }
}

/// Seed fields for creating a dependency record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyRecordSeed {
    /// Stable dependency id.
    pub dependency_id: String,
    /// Package-manager family token.
    pub package_manager_family_token: String,
    /// Opaque package coordinate ref.
    pub package_coordinate_ref: String,
    /// Human-reviewable package name.
    pub package_name: String,
    /// Dependency relationship class.
    pub relationship_class: DependencyRelationshipClass,
    /// Dependency source class.
    pub source_class: DependencySourceClass,
    /// Dependency freshness class.
    pub freshness_class: DependencyFreshnessClass,
    /// Dependency provenance class.
    pub provenance_class: DependencyProvenanceClass,
    /// Dependency resolution class.
    pub resolution_class: DependencyResolutionClass,
    /// Requested requirement or source.
    pub requested_requirement: Option<String>,
    /// Resolved exact version or source.
    pub resolved_version: Option<String>,
    /// Owning manifest ref.
    pub manifest_ref: Option<String>,
    /// Owning manifest path.
    pub manifest_path: Option<String>,
    /// Owning lockfile ref.
    pub lockfile_ref: Option<String>,
    /// Owning lockfile path.
    pub lockfile_path: Option<String>,
    /// Workspace member ref.
    pub workspace_member_ref: Option<String>,
    /// Registry source ref.
    pub registry_source_ref: Option<String>,
    /// Advisory refs that currently affect this dependency.
    pub advisory_refs: Vec<String>,
    /// License decision class.
    pub license_decision_class: LicenseDecisionClass,
    /// Notice or license implication refs.
    pub notice_implication_refs: Vec<String>,
    /// Evidence refs that support the record.
    pub evidence_refs: Vec<String>,
}

/// Normalized dependency node shared by graph, advisory, review, and export lanes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable dependency id.
    pub dependency_id: String,
    /// Package-manager family token.
    pub package_manager_family_token: String,
    /// Opaque package coordinate ref.
    pub package_coordinate_ref: String,
    /// Human-reviewable package name.
    pub package_name: String,
    /// Dependency relationship class.
    pub relationship_class: DependencyRelationshipClass,
    /// Stable relationship token.
    pub relationship_token: String,
    /// Dependency source class.
    pub source_class: DependencySourceClass,
    /// Stable source token.
    pub source_token: String,
    /// Dependency freshness class.
    pub freshness_class: DependencyFreshnessClass,
    /// Stable freshness token.
    pub freshness_token: String,
    /// Dependency provenance class.
    pub provenance_class: DependencyProvenanceClass,
    /// Stable provenance token.
    pub provenance_token: String,
    /// Dependency resolution class.
    pub resolution_class: DependencyResolutionClass,
    /// Stable resolution token.
    pub resolution_token: String,
    /// Requested requirement or source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_requirement: Option<String>,
    /// Resolved exact version or source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub resolved_version: Option<String>,
    /// Owning manifest ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_ref: Option<String>,
    /// Owning manifest path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest_path: Option<String>,
    /// Owning lockfile ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lockfile_ref: Option<String>,
    /// Owning lockfile path.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lockfile_path: Option<String>,
    /// Workspace member ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_member_ref: Option<String>,
    /// Registry source ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub registry_source_ref: Option<String>,
    /// Advisory refs that currently affect this dependency.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub advisory_refs: Vec<String>,
    /// License decision class.
    pub license_decision_class: LicenseDecisionClass,
    /// Stable license decision token.
    pub license_decision_token: String,
    /// Notice or license implication refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notice_implication_refs: Vec<String>,
    /// Evidence refs that support the record.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
    /// True when the record omits raw manifests, lockfiles, registry URLs, and secrets.
    pub export_safe: bool,
}

impl DependencyRecord {
    /// Creates a normalized dependency record from reviewed seed fields.
    pub fn new(seed: DependencyRecordSeed) -> Self {
        Self {
            record_kind: DEPENDENCY_RECORD_KIND.to_owned(),
            schema_version: DEPENDENCY_INTELLIGENCE_SCHEMA_VERSION,
            dependency_id: seed.dependency_id,
            package_manager_family_token: seed.package_manager_family_token,
            package_coordinate_ref: seed.package_coordinate_ref,
            package_name: seed.package_name,
            relationship_class: seed.relationship_class,
            relationship_token: seed.relationship_class.as_str().to_owned(),
            source_class: seed.source_class,
            source_token: seed.source_class.as_str().to_owned(),
            freshness_class: seed.freshness_class,
            freshness_token: seed.freshness_class.as_str().to_owned(),
            provenance_class: seed.provenance_class,
            provenance_token: seed.provenance_class.as_str().to_owned(),
            resolution_class: seed.resolution_class,
            resolution_token: seed.resolution_class.as_str().to_owned(),
            requested_requirement: seed.requested_requirement,
            resolved_version: seed.resolved_version,
            manifest_ref: seed.manifest_ref,
            manifest_path: seed.manifest_path,
            lockfile_ref: seed.lockfile_ref,
            lockfile_path: seed.lockfile_path,
            workspace_member_ref: seed.workspace_member_ref,
            registry_source_ref: seed.registry_source_ref,
            advisory_refs: seed.advisory_refs,
            license_decision_class: seed.license_decision_class,
            license_decision_token: seed.license_decision_class.as_str().to_owned(),
            notice_implication_refs: seed.notice_implication_refs,
            evidence_refs: seed.evidence_refs,
            export_safe: true,
        }
    }
}

/// Directed edge between dependency nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyEdgeRecord {
    /// Stable edge id.
    pub edge_id: String,
    /// Source node ref.
    pub from_node_ref: String,
    /// Target node ref.
    pub to_node_ref: String,
    /// Relationship class for the edge.
    pub relationship_class: DependencyRelationshipClass,
    /// Stable relationship token.
    pub relationship_token: String,
    /// Evidence refs that support the edge.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
}

impl DependencyEdgeRecord {
    /// Creates a dependency edge record with stable token fields.
    pub fn new(
        edge_id: impl Into<String>,
        from_node_ref: impl Into<String>,
        to_node_ref: impl Into<String>,
        relationship_class: DependencyRelationshipClass,
        evidence_refs: Vec<String>,
    ) -> Self {
        Self {
            edge_id: edge_id.into(),
            from_node_ref: from_node_ref.into(),
            to_node_ref: to_node_ref.into(),
            relationship_class,
            relationship_token: relationship_class.as_str().to_owned(),
            evidence_refs,
        }
    }
}

/// Normalized dependency graph for one workspace scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyGraphRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Runtime implementation version.
    pub reviewer_version: String,
    /// Stable graph id.
    pub graph_id: String,
    /// Workspace scope ref.
    pub workspace_scope_ref: String,
    /// Timestamp supplied by the caller.
    pub generated_at: String,
    /// Graph-level freshness class.
    pub source_freshness_class: DependencyFreshnessClass,
    /// Stable graph-level freshness token.
    pub source_freshness_token: String,
    /// Mirror/offline posture.
    pub mirror_or_offline_state_class: MirrorOrOfflineStateClass,
    /// Stable mirror/offline token.
    pub mirror_or_offline_state_token: String,
    /// Dependency nodes.
    pub dependencies: Vec<DependencyRecord>,
    /// Dependency edges.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub edges: Vec<DependencyEdgeRecord>,
    /// Source refs consumed to build the graph.
    pub graph_source_refs: Vec<String>,
    /// True when the graph omits raw manifests, lockfiles, registry URLs, and secrets.
    pub export_safe: bool,
}

impl DependencyGraphRecord {
    /// Creates a normalized dependency graph record.
    pub fn new(
        graph_id: impl Into<String>,
        workspace_scope_ref: impl Into<String>,
        generated_at: impl Into<String>,
        source_freshness_class: DependencyFreshnessClass,
        mirror_or_offline_state_class: MirrorOrOfflineStateClass,
        dependencies: Vec<DependencyRecord>,
        edges: Vec<DependencyEdgeRecord>,
        graph_source_refs: Vec<String>,
    ) -> Self {
        Self {
            record_kind: DEPENDENCY_GRAPH_RECORD_KIND.to_owned(),
            schema_version: DEPENDENCY_INTELLIGENCE_SCHEMA_VERSION,
            reviewer_version: DEPENDENCY_INTELLIGENCE_REVIEWER_VERSION.to_owned(),
            graph_id: graph_id.into(),
            workspace_scope_ref: workspace_scope_ref.into(),
            generated_at: generated_at.into(),
            source_freshness_class,
            source_freshness_token: source_freshness_class.as_str().to_owned(),
            mirror_or_offline_state_class,
            mirror_or_offline_state_token: mirror_or_offline_state_class.as_str().to_owned(),
            dependencies,
            edges,
            graph_source_refs,
            export_safe: true,
        }
    }

    /// Returns typed validation issues for graph review and tests.
    pub fn validation_issues(&self) -> Vec<DependencyIntelligenceViolation> {
        if self.dependencies.is_empty() {
            vec![DependencyIntelligenceViolation::GraphHasNoDependencies]
        } else {
            Vec::new()
        }
    }
}

/// Affected range row for an advisory.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdvisoryAffectedRange {
    /// Opaque package coordinate ref.
    pub package_coordinate_ref: String,
    /// Affected version or source range.
    pub affected_range: String,
    /// Fixed version requirement, when known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fixed_version_requirement: Option<String>,
}

impl AdvisoryAffectedRange {
    /// Creates an affected range row.
    pub fn new(
        package_coordinate_ref: impl Into<String>,
        affected_range: impl Into<String>,
        fixed_version_requirement: Option<String>,
    ) -> Self {
        Self {
            package_coordinate_ref: package_coordinate_ref.into(),
            affected_range: affected_range.into(),
            fixed_version_requirement,
        }
    }
}

/// Reviewed suppression reference attached to an advisory or dependency finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SuppressionRef {
    /// Stable suppression ref.
    pub suppression_ref: String,
    /// Suppression state class.
    pub suppression_state_class: SuppressionStateClass,
    /// Stable suppression state token.
    pub suppression_state_token: String,
    /// Opaque reason ref.
    pub reason_ref: String,
    /// Actor ref that created or owns the suppression.
    pub actor_ref: String,
    /// Scope ref narrowed by the suppression.
    pub scoped_to_ref: String,
    /// Expiry timestamp, when time-bounded.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Evidence refs that support the suppression.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
}

impl SuppressionRef {
    /// Creates a suppression ref with stable token fields.
    pub fn new(
        suppression_ref: impl Into<String>,
        suppression_state_class: SuppressionStateClass,
        reason_ref: impl Into<String>,
        actor_ref: impl Into<String>,
        scoped_to_ref: impl Into<String>,
        expires_at: Option<String>,
        evidence_refs: Vec<String>,
    ) -> Self {
        Self {
            suppression_ref: suppression_ref.into(),
            suppression_state_class,
            suppression_state_token: suppression_state_class.as_str().to_owned(),
            reason_ref: reason_ref.into(),
            actor_ref: actor_ref.into(),
            scoped_to_ref: scoped_to_ref.into(),
            expires_at,
            evidence_refs,
        }
    }
}

/// Seed fields for creating an advisory record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyAdvisoryRecordSeed {
    /// Stable advisory id.
    pub advisory_id: String,
    /// Advisory source ref.
    pub advisory_source_ref: String,
    /// Advisory source class.
    pub source_class: AdvisorySourceClass,
    /// Advisory truth class.
    pub truth_class: AdvisoryTruthClass,
    /// Advisory severity class.
    pub severity_class: AdvisorySeverityClass,
    /// Advisory lifecycle class.
    pub lifecycle_class: AdvisoryLifecycleClass,
    /// Affected dependency refs.
    pub affected_dependency_refs: Vec<String>,
    /// Affected version ranges.
    pub affected_ranges: Vec<AdvisoryAffectedRange>,
    /// Feed epoch ref.
    pub feed_epoch_ref: Option<String>,
    /// Mirror snapshot ref.
    pub mirror_snapshot_ref: Option<String>,
    /// Offline bundle ref.
    pub offline_bundle_ref: Option<String>,
    /// Imported report ref.
    pub imported_report_ref: Option<String>,
    /// Reason stale or offline data is being used.
    pub stale_reason: Option<String>,
    /// Suppression refs.
    pub suppression_refs: Vec<SuppressionRef>,
    /// Evidence refs.
    pub evidence_refs: Vec<String>,
    /// Export object refs.
    pub export_object_refs: Vec<String>,
    /// Timestamp supplied by the caller.
    pub matched_at: String,
}

/// Advisory record with local, imported, mirrored, or stale truth labels.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyAdvisoryRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable advisory id.
    pub advisory_id: String,
    /// Advisory source ref.
    pub advisory_source_ref: String,
    /// Advisory source class.
    pub source_class: AdvisorySourceClass,
    /// Stable source token.
    pub source_token: String,
    /// Advisory truth class.
    pub truth_class: AdvisoryTruthClass,
    /// Stable truth token.
    pub truth_token: String,
    /// Advisory severity class.
    pub severity_class: AdvisorySeverityClass,
    /// Stable severity token.
    pub severity_token: String,
    /// Advisory lifecycle class.
    pub lifecycle_class: AdvisoryLifecycleClass,
    /// Stable lifecycle token.
    pub lifecycle_token: String,
    /// Affected dependency refs.
    pub affected_dependency_refs: Vec<String>,
    /// Affected version ranges.
    pub affected_ranges: Vec<AdvisoryAffectedRange>,
    /// Feed epoch ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub feed_epoch_ref: Option<String>,
    /// Mirror snapshot ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mirror_snapshot_ref: Option<String>,
    /// Offline bundle ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offline_bundle_ref: Option<String>,
    /// Imported report ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub imported_report_ref: Option<String>,
    /// Reason stale or offline data is being used.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stale_reason: Option<String>,
    /// Suppression refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suppression_refs: Vec<SuppressionRef>,
    /// Evidence refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
    /// Export object refs with the same vocabulary as connected rows.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub export_object_refs: Vec<String>,
    /// Timestamp supplied by the caller.
    pub matched_at: String,
    /// True when the record omits raw reports, package sources, registry URLs, and secrets.
    pub export_safe: bool,
}

impl DependencyAdvisoryRecord {
    /// Creates an advisory record from reviewed seed fields.
    pub fn new(seed: DependencyAdvisoryRecordSeed) -> Self {
        Self {
            record_kind: DEPENDENCY_ADVISORY_RECORD_KIND.to_owned(),
            schema_version: DEPENDENCY_INTELLIGENCE_SCHEMA_VERSION,
            advisory_id: seed.advisory_id,
            advisory_source_ref: seed.advisory_source_ref,
            source_class: seed.source_class,
            source_token: seed.source_class.as_str().to_owned(),
            truth_class: seed.truth_class,
            truth_token: seed.truth_class.as_str().to_owned(),
            severity_class: seed.severity_class,
            severity_token: seed.severity_class.as_str().to_owned(),
            lifecycle_class: seed.lifecycle_class,
            lifecycle_token: seed.lifecycle_class.as_str().to_owned(),
            affected_dependency_refs: seed.affected_dependency_refs,
            affected_ranges: seed.affected_ranges,
            feed_epoch_ref: seed.feed_epoch_ref,
            mirror_snapshot_ref: seed.mirror_snapshot_ref,
            offline_bundle_ref: seed.offline_bundle_ref,
            imported_report_ref: seed.imported_report_ref,
            stale_reason: seed.stale_reason,
            suppression_refs: seed.suppression_refs,
            evidence_refs: seed.evidence_refs,
            export_object_refs: seed.export_object_refs,
            matched_at: seed.matched_at,
            export_safe: true,
        }
    }

    /// Returns typed validation issues for advisory review and tests.
    pub fn validation_issues(&self) -> Vec<DependencyIntelligenceViolation> {
        let mut issues = Vec::new();
        if self.affected_dependency_refs.is_empty() {
            issues.push(DependencyIntelligenceViolation::AdvisoryMissingAffectedDependency);
        }
        if self.truth_class == AdvisoryTruthClass::MirroredData
            && self.mirror_snapshot_ref.is_none()
        {
            issues.push(DependencyIntelligenceViolation::MirroredAdvisoryMissingSnapshot);
        }
        if self.truth_class == AdvisoryTruthClass::StaleOfflineState && self.stale_reason.is_none()
        {
            issues.push(DependencyIntelligenceViolation::StaleAdvisoryMissingReason);
        }
        if self.suppression_refs.iter().any(|suppression| {
            suppression.suppression_state_class == SuppressionStateClass::ActiveTimeBound
                && suppression.expires_at.is_none()
        }) {
            issues.push(DependencyIntelligenceViolation::ActiveSuppressionMissingExpiry);
        }
        issues
    }

    /// Returns true when the advisory remains visible in release debt.
    pub fn is_release_visible_debt(&self) -> bool {
        self.lifecycle_class.is_release_visible()
            || self.truth_class == AdvisoryTruthClass::StaleOfflineState
    }
}

/// Explicit lockfile mutation preview object minted before any write.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LockfileMutationPreview {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable preview id.
    pub preview_id: String,
    /// Preview action class.
    pub action_class: LockfilePreviewActionClass,
    /// Stable action token.
    pub action_token: String,
    /// Package operation preview ref, when this preview is derived from one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub package_operation_ref: Option<String>,
    /// Actor ref.
    pub actor_ref: String,
    /// Command id ref.
    pub command_id_ref: String,
    /// Target scope ref.
    pub target_scope_ref: String,
    /// Affected manifest refs.
    pub affected_manifest_refs: Vec<String>,
    /// Affected manifest paths.
    pub affected_manifest_paths: Vec<String>,
    /// Affected lockfile refs.
    pub affected_lockfile_refs: Vec<String>,
    /// Affected lockfile paths.
    pub affected_lockfile_paths: Vec<String>,
    /// Manifest delta token.
    pub manifest_delta_token: String,
    /// Lockfile impact token.
    pub lockfile_impact_token: String,
    /// Lockfile mutation mode token.
    pub mutation_mode_token: String,
    /// Transitive impact token.
    pub transitive_impact_token: String,
    /// Rollback checkpoint refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub rollback_checkpoint_refs: Vec<String>,
    /// Validation task tokens.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_task_tokens: Vec<String>,
    /// True when the preview was created before writing.
    pub preview_created_before_write: bool,
    /// True when the preview is attributable to actor, command, target, and scope.
    pub attributable_preview_object: bool,
    /// True when a write may proceed after review of this preview.
    pub writes_allowed_after_preview: bool,
    /// True when the source package operation is export-safe.
    pub source_package_operation_export_safe: bool,
    /// Preview outcome class.
    pub outcome_class: LockfilePreviewOutcomeClass,
    /// Stable outcome token.
    pub outcome_token: String,
    /// Timestamp supplied by the caller.
    pub minted_at: String,
    /// Redaction class for this preview.
    pub redaction_class: PackageRedactionClass,
    /// Stable redaction token.
    pub redaction_token: String,
    /// True when the preview omits raw manifests, lockfiles, registry URLs, and secrets.
    pub export_safe: bool,
}

impl LockfileMutationPreview {
    /// Creates a lockfile mutation preview from an existing package operation packet.
    pub fn from_package_operation(
        preview_id: impl Into<String>,
        action_class: LockfilePreviewActionClass,
        packet: &PackageOperationAlphaPacket,
        minted_at: impl Into<String>,
    ) -> Self {
        let affected_lockfile_refs = packet
            .lockfile_impact
            .affected_lockfiles
            .iter()
            .map(|lockfile| lockfile.lockfile_ref.clone())
            .collect::<Vec<_>>();
        let affected_lockfile_paths = packet
            .lockfile_impact
            .affected_lockfiles
            .iter()
            .map(|lockfile| lockfile.lockfile_path.clone())
            .collect::<Vec<_>>();
        let rollback_checkpoint_refs = [
            packet.rollback.lockfile_checkpoint_ref.clone(),
            packet.rollback.workspace_snapshot_checkpoint_ref.clone(),
            packet.rollback.cache_checkpoint_ref.clone(),
        ]
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();
        let outcome_class = preview_outcome_for(
            action_class,
            packet.lockfile_impact.mutation_mode,
            &rollback_checkpoint_refs,
        );
        Self {
            record_kind: LOCKFILE_MUTATION_PREVIEW_RECORD_KIND.to_owned(),
            schema_version: DEPENDENCY_INTELLIGENCE_SCHEMA_VERSION,
            preview_id: preview_id.into(),
            action_class,
            action_token: action_class.as_str().to_owned(),
            package_operation_ref: Some(packet.package_operation_id.clone()),
            actor_ref: packet.audit_lineage.actor_ref.clone(),
            command_id_ref: packet.audit_lineage.command_id_ref.clone(),
            target_scope_ref: packet.audit_lineage.workspace_scope_ref.clone(),
            affected_manifest_refs: vec![packet.manifest_scope.active_manifest_ref.clone()],
            affected_manifest_paths: vec![packet.manifest_scope.active_manifest_path.clone()],
            affected_lockfile_refs,
            affected_lockfile_paths,
            manifest_delta_token: packet.manifest_diff.manifest_delta_token.clone(),
            lockfile_impact_token: packet.lockfile_impact.lockfile_impact_token.clone(),
            mutation_mode_token: packet.lockfile_impact.mutation_mode_token.clone(),
            transitive_impact_token: packet.lockfile_impact.transitive_impact_token.clone(),
            rollback_checkpoint_refs,
            validation_task_tokens: packet.validation_task_tokens.clone(),
            preview_created_before_write: true,
            attributable_preview_object: true,
            writes_allowed_after_preview: outcome_class
                == LockfilePreviewOutcomeClass::PreviewReadyApplyPending,
            source_package_operation_export_safe: packet.export_safe,
            outcome_class,
            outcome_token: outcome_class.as_str().to_owned(),
            minted_at: minted_at.into(),
            redaction_class: packet.redaction_class,
            redaction_token: packet.redaction_token.clone(),
            export_safe: packet.export_safe,
        }
    }

    /// Returns typed validation issues for lockfile preview review and tests.
    pub fn validation_issues(&self) -> Vec<DependencyIntelligenceViolation> {
        let mut issues = Vec::new();
        if self.affected_manifest_refs.is_empty() || self.affected_lockfile_refs.is_empty() {
            issues.push(DependencyIntelligenceViolation::LockfilePreviewMissingAffectedFiles);
        }
        if self.action_class.is_mutating() && self.rollback_checkpoint_refs.is_empty() {
            issues.push(DependencyIntelligenceViolation::LockfilePreviewMissingCheckpoint);
        }
        if !self.preview_created_before_write || !self.attributable_preview_object {
            issues.push(DependencyIntelligenceViolation::LockfilePreviewNotAttributable);
        }
        issues
    }

    /// Returns true when a write remains blocked by the preview contract.
    pub fn blocks_write(&self) -> bool {
        !self.validation_issues().is_empty()
            || !self.writes_allowed_after_preview && self.action_class.is_mutating()
    }
}

/// One release-visible dependency debt row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyDebtRow {
    /// Stable debt id.
    pub debt_id: String,
    /// Debt kind class.
    pub debt_kind_class: DependencyDebtKindClass,
    /// Stable debt kind token.
    pub debt_kind_token: String,
    /// Dependency ref.
    pub dependency_ref: String,
    /// Advisory ref, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advisory_ref: Option<String>,
    /// Suppression ref, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suppression_ref: Option<String>,
    /// License or notice implication ref, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub license_notice_ref: Option<String>,
    /// Lockfile preview ref, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lockfile_preview_ref: Option<String>,
    /// Advisory severity token, when applicable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub severity_token: Option<String>,
    /// Source truth token for the row.
    pub source_truth_token: String,
    /// Release visibility class.
    pub release_visibility_class: DebtReleaseVisibilityClass,
    /// Stable release visibility token.
    pub release_visibility_token: String,
    /// Owner ref for the debt row.
    pub owner_ref: String,
    /// Due or review timestamp.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub due_at: Option<String>,
    /// Evidence refs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence_refs: Vec<String>,
}

impl DependencyDebtRow {
    /// Creates a dependency debt row with stable token fields.
    pub fn new(
        debt_id: impl Into<String>,
        debt_kind_class: DependencyDebtKindClass,
        dependency_ref: impl Into<String>,
        source_truth_token: impl Into<String>,
        release_visibility_class: DebtReleaseVisibilityClass,
        owner_ref: impl Into<String>,
    ) -> Self {
        Self {
            debt_id: debt_id.into(),
            debt_kind_class,
            debt_kind_token: debt_kind_class.as_str().to_owned(),
            dependency_ref: dependency_ref.into(),
            advisory_ref: None,
            suppression_ref: None,
            license_notice_ref: None,
            lockfile_preview_ref: None,
            severity_token: None,
            source_truth_token: source_truth_token.into(),
            release_visibility_class,
            release_visibility_token: release_visibility_class.as_str().to_owned(),
            owner_ref: owner_ref.into(),
            due_at: None,
            evidence_refs: Vec::new(),
        }
    }
}

/// Seed fields for creating a dependency debt packet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DependencyDebtPacketSeed {
    /// Stable packet id.
    pub debt_packet_id: String,
    /// Timestamp supplied by the caller.
    pub generated_at: String,
    /// Release candidate ref.
    pub release_candidate_ref: String,
    /// Artifact family refs.
    pub artifact_family_refs: Vec<String>,
    /// Dependency graph ref.
    pub dependency_graph_ref: String,
    /// Advisory refs included in the packet.
    pub advisory_refs: Vec<String>,
    /// Suppression refs included in the packet.
    pub suppression_refs: Vec<String>,
    /// Lockfile preview refs included in the packet.
    pub lockfile_preview_refs: Vec<String>,
    /// Notice or license implication refs included in the packet.
    pub notice_license_implication_refs: Vec<String>,
    /// Debt rows.
    pub rows: Vec<DependencyDebtRow>,
    /// Mirror/offline posture for the packet.
    pub mirror_or_offline_state_class: MirrorOrOfflineStateClass,
}

/// Release-visible dependency debt packet for release, support, and audit surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DependencyDebtPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Payload schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub debt_packet_id: String,
    /// Timestamp supplied by the caller.
    pub generated_at: String,
    /// Release candidate ref.
    pub release_candidate_ref: String,
    /// Artifact family refs.
    pub artifact_family_refs: Vec<String>,
    /// Dependency graph ref.
    pub dependency_graph_ref: String,
    /// Advisory refs included in the packet.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub advisory_refs: Vec<String>,
    /// Suppression refs included in the packet.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub suppression_refs: Vec<String>,
    /// Lockfile preview refs included in the packet.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub lockfile_preview_refs: Vec<String>,
    /// Notice or license implication refs included in the packet.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub notice_license_implication_refs: Vec<String>,
    /// Debt rows.
    pub rows: Vec<DependencyDebtRow>,
    /// Count of active advisory rows.
    pub active_advisory_count: u32,
    /// Count of active suppression rows.
    pub active_suppression_count: u32,
    /// Count of stale or offline source rows.
    pub stale_or_offline_source_count: u32,
    /// Count of unresolved license or notice rows.
    pub unresolved_license_notice_count: u32,
    /// Count of release-visible rows.
    pub release_visible_debt_count: u32,
    /// Mirror/offline posture for the packet.
    pub mirror_or_offline_state_class: MirrorOrOfflineStateClass,
    /// Stable mirror/offline token.
    pub mirror_or_offline_state_token: String,
    /// True when the packet omits raw manifests, lockfiles, registry URLs, and secrets.
    pub export_safe: bool,
}

impl DependencyDebtPacket {
    /// Creates a release-visible dependency debt packet.
    pub fn new(seed: DependencyDebtPacketSeed) -> Self {
        let active_advisory_count = count_kind(&seed.rows, DependencyDebtKindClass::ActiveAdvisory);
        let active_suppression_count =
            count_kind(&seed.rows, DependencyDebtKindClass::ActiveSuppression);
        let stale_or_offline_source_count =
            count_kind(&seed.rows, DependencyDebtKindClass::StaleAdvisorySource);
        let unresolved_license_notice_count =
            count_kind(&seed.rows, DependencyDebtKindClass::UnresolvedLicenseNotice);
        let release_visible_debt_count = seed
            .rows
            .iter()
            .filter(|row| row.release_visibility_class.is_release_or_support_visible())
            .count() as u32;
        Self {
            record_kind: DEPENDENCY_DEBT_PACKET_RECORD_KIND.to_owned(),
            schema_version: DEPENDENCY_INTELLIGENCE_SCHEMA_VERSION,
            debt_packet_id: seed.debt_packet_id,
            generated_at: seed.generated_at,
            release_candidate_ref: seed.release_candidate_ref,
            artifact_family_refs: seed.artifact_family_refs,
            dependency_graph_ref: seed.dependency_graph_ref,
            advisory_refs: seed.advisory_refs,
            suppression_refs: seed.suppression_refs,
            lockfile_preview_refs: seed.lockfile_preview_refs,
            notice_license_implication_refs: seed.notice_license_implication_refs,
            rows: seed.rows,
            active_advisory_count,
            active_suppression_count,
            stale_or_offline_source_count,
            unresolved_license_notice_count,
            release_visible_debt_count,
            mirror_or_offline_state_class: seed.mirror_or_offline_state_class,
            mirror_or_offline_state_token: seed.mirror_or_offline_state_class.as_str().to_owned(),
            export_safe: true,
        }
    }

    /// Returns typed validation issues for dependency debt packet review and tests.
    pub fn validation_issues(&self) -> Vec<DependencyIntelligenceViolation> {
        let mut issues = Vec::new();
        if self.release_visible_debt_count == 0 {
            issues.push(DependencyIntelligenceViolation::DebtPacketNotReleaseVisible);
        }
        if self.rows.iter().any(|row| {
            row.debt_kind_class == DependencyDebtKindClass::UnresolvedLicenseNotice
                && row.license_notice_ref.is_none()
        }) {
            issues.push(
                DependencyIntelligenceViolation::DebtPacketMissingNoticeImplicationForLicenseDebt,
            );
        }
        issues
    }
}

fn preview_outcome_for(
    action_class: LockfilePreviewActionClass,
    mutation_mode: LockfileMutationMode,
    rollback_checkpoint_refs: &[String],
) -> LockfilePreviewOutcomeClass {
    if !action_class.is_mutating() {
        return LockfilePreviewOutcomeClass::PreviewReportedReadOnly;
    }
    if mutation_mode == LockfileMutationMode::BlockedUnknownRequiresReview {
        return LockfilePreviewOutcomeClass::PreviewBlockedUnknownImpact;
    }
    if rollback_checkpoint_refs.is_empty() {
        return LockfilePreviewOutcomeClass::PreviewBlockedMissingCheckpoint;
    }
    LockfilePreviewOutcomeClass::PreviewReadyApplyPending
}

fn count_kind(rows: &[DependencyDebtRow], kind: DependencyDebtKindClass) -> u32 {
    rows.iter()
        .filter(|row| row.debt_kind_class == kind)
        .count() as u32
}

/// Returns validation task tokens for external callers that only need strings.
pub fn validation_task_tokens(tasks: &[ValidationTaskClass]) -> Vec<String> {
    tasks.iter().map(|task| task.as_str().to_owned()).collect()
}

/// Returns a stable manifest delta token for external preview adapters.
pub fn manifest_delta_token(delta: ManifestDeltaClass) -> &'static str {
    delta.as_str()
}
