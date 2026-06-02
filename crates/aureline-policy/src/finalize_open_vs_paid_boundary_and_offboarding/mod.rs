//! Open-versus-paid boundary manifest, managed-offering truth, versioned usage
//! export, and offboarding packet.
//!
//! This module publishes one inspectable boundary model that maps every claimed
//! capability to its boundary class, and ships versioned usage-export and
//! offboarding packets that disclose quota family, record class, partiality,
//! retention posture, and tenant-scoped data that does not leave with the local
//! product.
//!
//! The module validates four stability conditions:
//!
//! 1. **Boundary manifest complete** — every claimed capability family carries
//!    a boundary row with a closed-vocabulary boundary class; no row is missing.
//! 2. **Local-core independence enforced** — core local workflows (editing,
//!    search, Git, tasks, debugging, local indexing, local-safe AI) are classified
//!    as `open_local` and do not depend on a hidden managed prerequisite.
//! 3. **Offboarding state disclosed** — every capability classified as
//!    `managed_hosted` or `enterprise_governed` carries an offboarding packet
//!    that names what remains local, what becomes unavailable, what is still
//!    exportable, and which managed records persist for policy or billing reasons.
//! 4. **Usage-export schema version current** — every usage-export packet
//!    carries the current schema version, a retention label, and a clear
//!    machine-readable partiality marker.
//!
//! One condition forces `Withdrawn` immediately and cannot be overridden:
//!
//! - A [`OpenVsPaidBoundaryNarrowReasonClass::LocalCoreRequiresManagedPrerequisite`]
//!   defect when a local-core capability is classified as `managed_hosted` or
//!   `enterprise_governed`. The packet is withdrawn entirely.
//!
//! The packet is export-safe. It carries record-kind tags, schema-version
//! integers, closed-vocabulary tokens, plain-language consequence labels, and
//! opaque refs only. Raw entitlement values, raw seat identifiers, raw tenant
//! configuration, raw quota counters, and raw billing records stay outside the
//! support boundary.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/policy/m4/finalize-open-vs-paid-boundary-and-offboarding.md`
//! - Artifact: `artifacts/policy/m4/finalize-open-vs-paid-boundary-and-offboarding.md`
//! - Contract ref: [`OPEN_VS_PAID_BOUNDARY_SHARED_CONTRACT_REF`]

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const OPEN_VS_PAID_BOUNDARY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const OPEN_VS_PAID_BOUNDARY_SHARED_CONTRACT_REF: &str = "policy:open_vs_paid_boundary:v1";

/// Record-kind tag for [`OpenVsPaidBoundaryPage`] payloads.
pub const OPEN_VS_PAID_BOUNDARY_PAGE_RECORD_KIND: &str = "policy_open_vs_paid_boundary_page_record";

/// Record-kind tag for [`OpenVsPaidBoundaryRow`] payloads.
pub const OPEN_VS_PAID_BOUNDARY_ROW_RECORD_KIND: &str = "policy_open_vs_paid_boundary_row_record";

/// Record-kind tag for [`OpenVsPaidBoundaryDefect`] payloads.
pub const OPEN_VS_PAID_BOUNDARY_DEFECT_RECORD_KIND: &str =
    "policy_open_vs_paid_boundary_defect_record";

/// Record-kind tag for [`OpenVsPaidBoundarySupportExport`] payloads.
pub const OPEN_VS_PAID_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_open_vs_paid_boundary_support_export_record";

/// Record-kind tag for [`UsageExportPacket`] payloads.
pub const USAGE_EXPORT_PACKET_RECORD_KIND: &str = "policy_usage_export_packet_record";

/// Record-kind tag for [`OffboardingPacket`] payloads.
pub const OFFBOARDING_PACKET_RECORD_KIND: &str = "policy_offboarding_packet_record";

/// Repo-relative path of the stable doc for this lane.
pub const OPEN_VS_PAID_BOUNDARY_DOC_REF: &str =
    "docs/policy/m4/finalize-open-vs-paid-boundary-and-offboarding.md";

/// Repo-relative path of the artifact summary for this lane.
pub const OPEN_VS_PAID_BOUNDARY_ARTIFACT_REF: &str =
    "artifacts/policy/m4/finalize-open-vs-paid-boundary-and-offboarding.md";

// ---------------------------------------------------------------------------
// Capability and boundary vocabulary
// ---------------------------------------------------------------------------

/// Closed vocabulary of capability family tokens accepted by the boundary audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityFamilyClass {
    /// Core editor, buffer, file system, and undo history.
    EditorCore,
    /// Local search and navigation.
    Search,
    /// Local Git operations.
    LocalGit,
    /// Local task and build runner.
    Tasks,
    /// Local debugging baseline.
    Debugging,
    /// Local symbol and file indexing.
    LocalIndexing,
    /// Local-safe AI routes and BYOK inference.
    LocalSafeAi,
    /// Real-time collaboration and shared workspaces.
    Collaboration,
    /// Managed AI routing, quotas, and audit.
    ManagedAiRouting,
    /// Admin dashboard and fleet telemetry.
    AdminDashboard,
    /// Policy enforcement and signed policy bundles.
    PolicyEnforcement,
    /// Extension marketplace and registry.
    ExtensionsMarketplace,
    /// Support export and diagnostic bundles.
    SupportExports,
    /// Usage analytics and quota metering.
    UsageAnalytics,
    /// Backup, restore, and failover continuity.
    BackupRestore,
}

impl CapabilityFamilyClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EditorCore => "editor_core",
            Self::Search => "search",
            Self::LocalGit => "local_git",
            Self::Tasks => "tasks",
            Self::Debugging => "debugging",
            Self::LocalIndexing => "local_indexing",
            Self::LocalSafeAi => "local_safe_ai",
            Self::Collaboration => "collaboration",
            Self::ManagedAiRouting => "managed_ai_routing",
            Self::AdminDashboard => "admin_dashboard",
            Self::PolicyEnforcement => "policy_enforcement",
            Self::ExtensionsMarketplace => "extensions_marketplace",
            Self::SupportExports => "support_exports",
            Self::UsageAnalytics => "usage_analytics",
            Self::BackupRestore => "backup_restore",
        }
    }

    /// All capability families that constitute the local-core floor.
    pub const LOCAL_CORE_FAMILIES: [Self; 7] = [
        Self::EditorCore,
        Self::Search,
        Self::LocalGit,
        Self::Tasks,
        Self::Debugging,
        Self::LocalIndexing,
        Self::LocalSafeAi,
    ];

    /// True when this capability family is part of the local-core floor.
    pub const fn is_local_core(self) -> bool {
        matches!(
            self,
            Self::EditorCore
                | Self::Search
                | Self::LocalGit
                | Self::Tasks
                | Self::Debugging
                | Self::LocalIndexing
                | Self::LocalSafeAi
        )
    }
}

/// Boundary class that describes how a capability is offered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityBoundaryClass {
    /// Available locally without account creation or active managed connectivity.
    OpenLocal,
    /// Requires a managed or hosted service; may be an add-on.
    ManagedHosted,
    /// Governed by enterprise policy; may require tenant-bound enrollment.
    EnterpriseGoverned,
    /// Not included in any current offering.
    NotIncluded,
}

impl CapabilityBoundaryClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenLocal => "open_local",
            Self::ManagedHosted => "managed_hosted",
            Self::EnterpriseGoverned => "enterprise_governed",
            Self::NotIncluded => "not_included",
        }
    }

    /// True when this boundary class requires an offboarding disclosure.
    pub const fn requires_offboarding_disclosure(self) -> bool {
        matches!(self, Self::ManagedHosted | Self::EnterpriseGoverned)
    }

    /// True when this boundary class is available without managed connectivity.
    pub const fn is_local_safe(self) -> bool {
        matches!(self, Self::OpenLocal | Self::NotIncluded)
    }
}

// ---------------------------------------------------------------------------
// Offboarding and usage-export vocabulary
// ---------------------------------------------------------------------------

/// Offboarding outcome state for a capability or data record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OffboardingOutcomeClass {
    /// Data remains on-device only; no managed copy exists.
    LocalOnly,
    /// A managed copy exists; the local copy remains.
    ManagedCopy,
    /// Export or deletion is queued and not yet completed.
    Queued,
    /// Only a partial export or partial deletion is available.
    Partial,
    /// Action is blocked by an administrative or legal hold.
    BlockedByHold,
    /// Record is retained for policy, compliance, or audit reasons.
    PolicyRetained,
    /// Record is outside the platform scope and not managed.
    OutsidePlatformScope,
    /// Offboarding action for this record is fully completed.
    Completed,
}

impl OffboardingOutcomeClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOnly => "local_only",
            Self::ManagedCopy => "managed_copy",
            Self::Queued => "queued",
            Self::Partial => "partial",
            Self::BlockedByHold => "blocked_by_hold",
            Self::PolicyRetained => "policy_retained",
            Self::OutsidePlatformScope => "outside_platform_scope",
            Self::Completed => "completed",
        }
    }
}

/// Availability of a usage export for a given record class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UsageExportAvailabilityClass {
    /// Full export is available with all records.
    Full,
    /// Partial export is available; some records are excluded.
    Partial,
    /// Export is unavailable for this record class.
    Unavailable,
}

impl UsageExportAvailabilityClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Full => "full",
            Self::Partial => "partial",
            Self::Unavailable => "unavailable",
        }
    }
}

/// Retention posture applied to an exported or retained record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportRetentionClass {
    /// User-owned data available for immediate export.
    UserOwnedImmediate,
    /// Retained by the tenant for policy or compliance reasons.
    TenantRetainedPolicy,
    /// Retained for billing or metering reasons.
    BillingRetained,
    /// Available for export during a grace window after cancellation.
    GraceWindowExportable,
    /// Grace window expired; export no longer available.
    ExpiredUnavailable,
}

impl ExportRetentionClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserOwnedImmediate => "user_owned_immediate",
            Self::TenantRetainedPolicy => "tenant_retained_policy",
            Self::BillingRetained => "billing_retained",
            Self::GraceWindowExportable => "grace_window_exportable",
            Self::ExpiredUnavailable => "expired_unavailable",
        }
    }
}

/// Grace-window state during offboarding, seat loss, or cancellation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GraceWindowStateClass {
    /// Grace window is active; exports and local continuity are preserved.
    Active,
    /// Grace window has expired; managed capabilities are paused.
    Expired,
    /// Only export routes remain available; managed features are paused.
    ExportOnly,
    /// Local core is preserved but managed features are degraded.
    Degraded,
}

impl GraceWindowStateClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Expired => "expired",
            Self::ExportOnly => "export_only",
            Self::Degraded => "degraded",
        }
    }
}

// ---------------------------------------------------------------------------
// Input types
// ---------------------------------------------------------------------------

/// One capability row in the boundary manifest input.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityBoundaryInputRow {
    /// Capability family token.
    pub capability_family: CapabilityFamilyClass,
    /// Boundary class for this capability.
    pub boundary_class: CapabilityBoundaryClass,
    /// True when the product surfaces, docs, and help copy all agree on this
    /// boundary class for the shipped build.
    pub surfaces_consistent: bool,
    /// True when an offboarding packet is present for this row. Required when
    /// `boundary_class` is `managed_hosted` or `enterprise_governed`.
    pub offboarding_disclosed: bool,
    /// True when a usage-export packet is present for this row. Required when
    /// `boundary_class` is `managed_hosted` or `enterprise_governed`.
    pub usage_export_disclosed: bool,
    /// True when the capability does not silently degrade when entitlement is
    /// lost; instead it surfaces an inspectable state change.
    pub entitlement_loss_visible: bool,
}

/// Full auditable input for the open-vs-paid boundary page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenVsPaidBoundaryInput {
    /// Claimed capability boundary rows.
    pub capability_rows: Vec<CapabilityBoundaryInputRow>,
    /// True when the usage-export schema version carried by every export packet
    /// matches the current module schema version.
    pub usage_export_schema_version_current: bool,
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Stability-qualification tier for the overall packet and for individual rows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenVsPaidBoundaryQualificationClass {
    /// All four stability conditions hold and the audit is clean.
    Stable,
    /// One or more non-critical conditions prevent the stable claim.
    Beta,
    /// A required row is missing; coverage gap prevents a beta claim.
    Preview,
    /// A local-core capability requires a managed prerequisite; the packet is
    /// withdrawn.
    Withdrawn,
}

impl OpenVsPaidBoundaryQualificationClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Withdrawn => "withdrawn",
        }
    }

    /// True when this tier claims the stable line.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// True when this tier is claimable (stable or beta).
    pub const fn is_claimable(self) -> bool {
        matches!(self, Self::Stable | Self::Beta)
    }
}

/// Typed reason a packet or row was narrowed below
/// [`OpenVsPaidBoundaryQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OpenVsPaidBoundaryNarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// A capability's boundary class differs between product surfaces, docs, or
    /// help copy.
    CapabilityMisclassifiedAcrossSurfaces,
    /// A local-core capability is classified as `managed_hosted` or
    /// `enterprise_governed`, implying a hidden prerequisite. Hard guardrail.
    LocalCoreRequiresManagedPrerequisite,
    /// A required capability family row is missing from the boundary manifest.
    MissingBoundaryManifestRow,
    /// A managed or enterprise-governed capability does not disclose its
    /// offboarding state.
    OffboardingStateUndisclosed,
    /// A managed or enterprise-governed capability does not disclose its usage
    /// export posture.
    UsageExportUndisclosed,
    /// The usage-export schema version is stale.
    UsageExportSchemaVersionStale,
    /// A capability claimed as `open_local` actually requires a managed
    /// prerequisite at runtime.
    ManagedCopyClaimedAsLocal,
    /// Policy- or billing-retained data is not disclosed to the user during
    /// offboarding.
    PolicyRetainedDataNotDisclosed,
}

impl OpenVsPaidBoundaryNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::CapabilityMisclassifiedAcrossSurfaces => {
                "capability_misclassified_across_surfaces"
            }
            Self::LocalCoreRequiresManagedPrerequisite => {
                "local_core_requires_managed_prerequisite"
            }
            Self::MissingBoundaryManifestRow => "missing_boundary_manifest_row",
            Self::OffboardingStateUndisclosed => "offboarding_state_undisclosed",
            Self::UsageExportUndisclosed => "usage_export_undisclosed",
            Self::UsageExportSchemaVersionStale => "usage_export_schema_version_stale",
            Self::ManagedCopyClaimedAsLocal => "managed_copy_claimed_as_local",
            Self::PolicyRetainedDataNotDisclosed => "policy_retained_data_not_disclosed",
        }
    }

    /// True when this reason is a hard guardrail that withdraws the packet.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::LocalCoreRequiresManagedPrerequisite)
    }
}

// ---------------------------------------------------------------------------
// Usage-export packet
// ---------------------------------------------------------------------------

/// Versioned usage-export packet that discloses quota family, record class,
/// partiality, retention posture, and tenant-scoped data scope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UsageExportPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Unique packet id.
    pub packet_id: String,
    /// Capability family this export covers.
    pub capability_family_token: String,
    /// Export availability for this record class.
    pub availability_token: String,
    /// Partiality marker: true when the export does not contain all records.
    pub partial: bool,
    /// Retention label for the exported records.
    pub retention_token: String,
    /// Quota family name (export-safe label).
    pub quota_family_label: String,
    /// True when tenant-scoped data is excluded from the export.
    pub tenant_scoped_data_excluded: bool,
    /// Plain-language explanation of what is included, what is partial, and
    /// what is retained by the tenant.
    pub export_summary: String,
}

impl UsageExportPacket {
    /// Build a usage-export packet for a capability family.
    pub fn new(
        packet_id: impl Into<String>,
        capability_family: CapabilityFamilyClass,
        availability: UsageExportAvailabilityClass,
        partial: bool,
        retention: ExportRetentionClass,
        quota_family_label: impl Into<String>,
        tenant_scoped_data_excluded: bool,
        export_summary: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: USAGE_EXPORT_PACKET_RECORD_KIND.to_owned(),
            schema_version: OPEN_VS_PAID_BOUNDARY_SCHEMA_VERSION,
            shared_contract_ref: OPEN_VS_PAID_BOUNDARY_SHARED_CONTRACT_REF.to_owned(),
            packet_id: packet_id.into(),
            capability_family_token: capability_family.as_str().to_owned(),
            availability_token: availability.as_str().to_owned(),
            partial,
            retention_token: retention.as_str().to_owned(),
            quota_family_label: quota_family_label.into(),
            tenant_scoped_data_excluded,
            export_summary: export_summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Offboarding packet
// ---------------------------------------------------------------------------

/// Offboarding packet that names what remains local, what becomes unavailable,
/// what is still exportable, and which managed records persist.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffboardingPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Unique packet id.
    pub packet_id: String,
    /// Capability family this offboarding packet covers.
    pub capability_family_token: String,
    /// Outcome state for local data.
    pub local_data_outcome_token: String,
    /// Outcome state for managed data.
    pub managed_data_outcome_token: String,
    /// Outcome state for exportability.
    pub exportability_outcome_token: String,
    /// Outcome state for policy-retained records.
    pub policy_retained_outcome_token: String,
    /// Grace-window state during offboarding.
    pub grace_window_state_token: String,
    /// True when the user is informed that platform-retained records still exist.
    pub platform_retention_disclosed: bool,
    /// Plain-language explanation of the offboarding outcome.
    pub offboarding_summary: String,
}

impl OffboardingPacket {
    /// Build an offboarding packet for a capability family.
    pub fn new(
        packet_id: impl Into<String>,
        capability_family: CapabilityFamilyClass,
        local_data_outcome: OffboardingOutcomeClass,
        managed_data_outcome: OffboardingOutcomeClass,
        exportability_outcome: OffboardingOutcomeClass,
        policy_retained_outcome: OffboardingOutcomeClass,
        grace_window_state: GraceWindowStateClass,
        platform_retention_disclosed: bool,
        offboarding_summary: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: OFFBOARDING_PACKET_RECORD_KIND.to_owned(),
            schema_version: OPEN_VS_PAID_BOUNDARY_SCHEMA_VERSION,
            shared_contract_ref: OPEN_VS_PAID_BOUNDARY_SHARED_CONTRACT_REF.to_owned(),
            packet_id: packet_id.into(),
            capability_family_token: capability_family.as_str().to_owned(),
            local_data_outcome_token: local_data_outcome.as_str().to_owned(),
            managed_data_outcome_token: managed_data_outcome.as_str().to_owned(),
            exportability_outcome_token: exportability_outcome.as_str().to_owned(),
            policy_retained_outcome_token: policy_retained_outcome.as_str().to_owned(),
            grace_window_state_token: grace_window_state.as_str().to_owned(),
            platform_retention_disclosed,
            offboarding_summary: offboarding_summary.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Row, summary, defect types
// ---------------------------------------------------------------------------

/// Boundary manifest row for one capability family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenVsPaidBoundaryRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Capability family token.
    pub capability_family_token: String,
    /// Boundary class token.
    pub boundary_class_token: String,
    /// Derived qualification tier.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// True when product surfaces agree on this boundary class.
    pub surfaces_consistent: bool,
    /// True when offboarding is disclosed.
    pub offboarding_disclosed: bool,
    /// True when usage export is disclosed.
    pub usage_export_disclosed: bool,
    /// True when entitlement loss is visible rather than silent.
    pub entitlement_loss_visible: bool,
    /// Optional usage-export packet for this row.
    pub usage_export_packet: Option<UsageExportPacket>,
    /// Optional offboarding packet for this row.
    pub offboarding_packet: Option<OffboardingPacket>,
    /// Plain-language summary of the boundary claim for this row.
    pub plain_language_summary: String,
}

/// Aggregate banner emitted with the boundary page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OpenVsPaidBoundarySummary {
    /// Total capability row count.
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Number of defects in this packet.
    pub defect_count: usize,
    /// Capability families covered.
    pub families_covered: Vec<String>,
    /// Number of open-local capabilities.
    pub open_local_count: usize,
    /// Number of managed/hosted capabilities.
    pub managed_hosted_count: usize,
    /// Number of enterprise-governed capabilities.
    pub enterprise_governed_count: usize,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl OpenVsPaidBoundarySummary {
    fn from_rows(rows: &[OpenVsPaidBoundaryRow], defects: &[OpenVsPaidBoundaryDefect]) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        let mut families: Vec<String> = Vec::new();
        let mut open_local = 0usize;
        let mut managed_hosted = 0usize;
        let mut enterprise_governed = 0usize;
        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
            if !families.contains(&row.capability_family_token) {
                families.push(row.capability_family_token.clone());
            }
            match row.boundary_class_token.as_str() {
                "open_local" => open_local += 1,
                "managed_hosted" => managed_hosted += 1,
                "enterprise_governed" => enterprise_governed += 1,
                _ => {}
            }
        }
        families.sort();
        let overall = if withdrawn > 0 {
            OpenVsPaidBoundaryQualificationClass::Withdrawn
        } else if preview > 0 {
            OpenVsPaidBoundaryQualificationClass::Preview
        } else if beta > 0 {
            OpenVsPaidBoundaryQualificationClass::Beta
        } else {
            OpenVsPaidBoundaryQualificationClass::Stable
        };
        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            defect_count: defects.len(),
            families_covered: families,
            open_local_count: open_local,
            managed_hosted_count: managed_hosted,
            enterprise_governed_count: enterprise_governed,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

/// Typed defect emitted by the boundary page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenVsPaidBoundaryDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: OpenVsPaidBoundaryNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (capability family token or `input`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl OpenVsPaidBoundaryDefect {
    fn new(
        narrow_reason: OpenVsPaidBoundaryNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: OPEN_VS_PAID_BOUNDARY_DEFECT_RECORD_KIND.to_owned(),
            schema_version: OPEN_VS_PAID_BOUNDARY_SCHEMA_VERSION,
            shared_contract_ref: OPEN_VS_PAID_BOUNDARY_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:open-vs-paid-boundary:{}:{}",
                narrow_reason.as_str(),
                &source_str
            ),
            narrow_reason,
            narrow_reason_token: narrow_reason.as_str().to_owned(),
            source: source_str,
            note: note.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Main page
// ---------------------------------------------------------------------------

/// Stable boundary manifest that maps each claimed capability to its boundary
/// class, and carries versioned usage-export and offboarding packets.
///
/// The packet is the single inspectable record that proves the stable claim
/// for this lane. About, Help, docs, pricing, and support exports should
/// ingest it rather than cloning boundary-status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenVsPaidBoundaryPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable page id.
    pub page_id: String,
    /// Human-readable page label.
    pub page_label: String,
    /// UTC instant when the packet was generated.
    pub generated_at: String,
    /// Aggregate summary derived from all rows.
    pub summary: OpenVsPaidBoundarySummary,
    /// Per-capability boundary rows.
    pub rows: Vec<OpenVsPaidBoundaryRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<OpenVsPaidBoundaryDefect>,
    /// The audited boundary input embedded as evidence.
    pub input: OpenVsPaidBoundaryInput,
}

impl OpenVsPaidBoundaryPage {
    /// Build the boundary page from an input.
    ///
    /// Rows are derived per capability row in the input, and the qualification
    /// for each is computed from the combined audit of the whole input.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        input: OpenVsPaidBoundaryInput,
    ) -> Self {
        let defects = audit_open_vs_paid_boundary_input(&input);
        let rows = derive_boundary_rows(&input, &defects);
        let summary = OpenVsPaidBoundarySummary::from_rows(&rows, &defects);
        Self {
            record_kind: OPEN_VS_PAID_BOUNDARY_PAGE_RECORD_KIND.to_owned(),
            schema_version: OPEN_VS_PAID_BOUNDARY_SCHEMA_VERSION,
            shared_contract_ref: OPEN_VS_PAID_BOUNDARY_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            defects,
            input,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == OpenVsPaidBoundaryQualificationClass::Stable.as_str()
    }

    /// True when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// True when all local-core capabilities are classified as `open_local`.
    pub fn local_core_independence_enforced(&self) -> bool {
        self.rows.iter().all(|row| {
            let family = row.capability_family_token.as_str();
            !CapabilityFamilyClass::LOCAL_CORE_FAMILIES
                .iter()
                .any(|f| f.as_str() == family)
                || row.boundary_class_token == CapabilityBoundaryClass::OpenLocal.as_str()
        })
    }

    /// True when every managed or enterprise-governed capability carries an
    /// offboarding disclosure.
    pub fn offboarding_disclosed_for_managed(&self) -> bool {
        self.rows.iter().all(|row| {
            !CapabilityBoundaryClass::requires_offboarding_disclosure_by_token(
                &row.boundary_class_token,
            ) || row.offboarding_disclosed
        })
    }

    /// True when every managed or enterprise-governed capability carries a
    /// usage-export disclosure.
    pub fn usage_export_disclosed_for_managed(&self) -> bool {
        self.rows.iter().all(|row| {
            !CapabilityBoundaryClass::requires_offboarding_disclosure_by_token(
                &row.boundary_class_token,
            ) || row.usage_export_disclosed
        })
    }

    /// True when the usage-export schema version is current.
    pub fn usage_export_schema_version_is_current(&self) -> bool {
        self.input.usage_export_schema_version_current
    }
}

// Helper for string-based boundary-class checks on rows.
impl CapabilityBoundaryClass {
    fn requires_offboarding_disclosure_by_token(token: &str) -> bool {
        matches!(token, "managed_hosted" | "enterprise_governed")
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that quotes the boundary page plus a metadata-safe
/// defect roll-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OpenVsPaidBoundarySupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// UTC export timestamp.
    pub generated_at: String,
    /// The boundary page embedded as evidence.
    pub page: OpenVsPaidBoundaryPage,
    /// Narrow-reason tokens present in the page's defect list.
    pub narrow_reasons_present: Vec<OpenVsPaidBoundaryNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when raw private material is excluded from this export.
    pub raw_private_material_excluded: bool,
}

impl OpenVsPaidBoundarySupportExport {
    /// Wrap a boundary page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: OpenVsPaidBoundaryPage,
    ) -> Self {
        let mut reasons: Vec<OpenVsPaidBoundaryNarrowReasonClass> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !reasons.contains(&defect.narrow_reason) {
                reasons.push(defect.narrow_reason);
            }
            *counts
                .entry(defect.narrow_reason_token.clone())
                .or_insert(0) += 1;
        }
        reasons.sort();
        Self {
            record_kind: OPEN_VS_PAID_BOUNDARY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: OPEN_VS_PAID_BOUNDARY_SCHEMA_VERSION,
            shared_contract_ref: OPEN_VS_PAID_BOUNDARY_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_private_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Audit and validate functions
// ---------------------------------------------------------------------------

/// Re-run the open-vs-paid boundary audit over the embedded input.
pub fn audit_open_vs_paid_boundary_page(
    page: &OpenVsPaidBoundaryPage,
) -> Vec<OpenVsPaidBoundaryDefect> {
    audit_open_vs_paid_boundary_input(&page.input)
}

/// Validate a boundary page; returns `Ok` when the audit is clean.
pub fn validate_open_vs_paid_boundary_page(
    page: &OpenVsPaidBoundaryPage,
) -> Result<(), Vec<OpenVsPaidBoundaryDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

fn audit_open_vs_paid_boundary_input(
    input: &OpenVsPaidBoundaryInput,
) -> Vec<OpenVsPaidBoundaryDefect> {
    let mut defects: Vec<OpenVsPaidBoundaryDefect> = Vec::new();

    // Hard guardrail: local-core capability requiring managed prerequisite.
    for row in &input.capability_rows {
        if row.capability_family.is_local_core() && !row.boundary_class.is_local_safe() {
            defects.push(OpenVsPaidBoundaryDefect::new(
                OpenVsPaidBoundaryNarrowReasonClass::LocalCoreRequiresManagedPrerequisite,
                row.capability_family.as_str(),
                format!(
                    "capability '{}' is part of the local-core floor but is classified \
                     as '{}'; local-core workflows must not depend on a hidden managed \
                     prerequisite; packet is withdrawn",
                    row.capability_family.as_str(),
                    row.boundary_class.as_str()
                ),
            ));
        }
    }
    if defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason())
    {
        return defects;
    }

    // Condition 1: Boundary manifest complete — every claimed capability has a row.
    let claimed: Vec<&str> = input
        .capability_rows
        .iter()
        .map(|r| r.capability_family.as_str())
        .collect();
    for family in &CapabilityFamilyClass::LOCAL_CORE_FAMILIES {
        if !claimed.contains(&family.as_str()) {
            defects.push(OpenVsPaidBoundaryDefect::new(
                OpenVsPaidBoundaryNarrowReasonClass::MissingBoundaryManifestRow,
                family.as_str(),
                format!(
                    "required local-core capability '{}' is missing from the boundary manifest",
                    family.as_str()
                ),
            ));
        }
    }

    // Condition 2: Surface consistency.
    for row in &input.capability_rows {
        if !row.surfaces_consistent {
            defects.push(OpenVsPaidBoundaryDefect::new(
                OpenVsPaidBoundaryNarrowReasonClass::CapabilityMisclassifiedAcrossSurfaces,
                row.capability_family.as_str(),
                format!(
                    "capability '{}' has inconsistent boundary classification across \
                     product surfaces, docs, or help copy; all surfaces must resolve the \
                     same closed-vocabulary token for the same capability",
                    row.capability_family.as_str()
                ),
            ));
        }
    }

    // Condition 3: Offboarding state disclosed for managed capabilities.
    for row in &input.capability_rows {
        if row.boundary_class.requires_offboarding_disclosure() && !row.offboarding_disclosed {
            defects.push(OpenVsPaidBoundaryDefect::new(
                OpenVsPaidBoundaryNarrowReasonClass::OffboardingStateUndisclosed,
                row.capability_family.as_str(),
                format!(
                    "capability '{}' is classified '{}' but does not disclose an \
                     offboarding packet; managed and enterprise-governed capabilities \
                     must name what remains local, what becomes unavailable, and what \
                     is retained for policy or billing reasons",
                    row.capability_family.as_str(),
                    row.boundary_class.as_str()
                ),
            ));
        }
    }

    // Condition 4: Usage-export disclosed for managed capabilities.
    for row in &input.capability_rows {
        if row.boundary_class.requires_offboarding_disclosure() && !row.usage_export_disclosed {
            defects.push(OpenVsPaidBoundaryDefect::new(
                OpenVsPaidBoundaryNarrowReasonClass::UsageExportUndisclosed,
                row.capability_family.as_str(),
                format!(
                    "capability '{}' is classified '{}' but does not disclose a usage-export \
                     packet; managed and enterprise-governed capabilities must carry a \
                     versioned usage-export record with retention labels and partiality markers",
                    row.capability_family.as_str(),
                    row.boundary_class.as_str()
                ),
            ));
        }
    }

    // Condition 5: Usage-export schema version current.
    if !input.usage_export_schema_version_current {
        defects.push(OpenVsPaidBoundaryDefect::new(
            OpenVsPaidBoundaryNarrowReasonClass::UsageExportSchemaVersionStale,
            "input",
            "usage-export schema version is stale; all export packets must carry the \
             current schema version defined by the module",
        ));
    }

    // Condition 6: Entitlement loss must be visible, not silent.
    for row in &input.capability_rows {
        if row.boundary_class.requires_offboarding_disclosure() && !row.entitlement_loss_visible {
            defects.push(OpenVsPaidBoundaryDefect::new(
                OpenVsPaidBoundaryNarrowReasonClass::ManagedCopyClaimedAsLocal,
                row.capability_family.as_str(),
                format!(
                    "capability '{}' does not surface entitlement loss as a visible state \
                     change; managed-only narrowing must degrade capabilities visibly \
                     rather than silently altering commands or data access",
                    row.capability_family.as_str()
                ),
            ));
        }
    }

    defects
}

fn derive_boundary_rows(
    input: &OpenVsPaidBoundaryInput,
    page_defects: &[OpenVsPaidBoundaryDefect],
) -> Vec<OpenVsPaidBoundaryRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());

    let overall_qual = if has_withdrawal {
        OpenVsPaidBoundaryQualificationClass::Withdrawn
    } else if !page_defects.is_empty() {
        OpenVsPaidBoundaryQualificationClass::Beta
    } else {
        OpenVsPaidBoundaryQualificationClass::Stable
    };

    let leading_narrow_reason = if has_withdrawal {
        OpenVsPaidBoundaryNarrowReasonClass::LocalCoreRequiresManagedPrerequisite
    } else if !page_defects.is_empty() {
        page_defects[0].narrow_reason
    } else {
        OpenVsPaidBoundaryNarrowReasonClass::NotNarrowed
    };

    // Build per-family defect index.
    let mut family_defect: BTreeMap<String, OpenVsPaidBoundaryNarrowReasonClass> = BTreeMap::new();
    for defect in page_defects {
        let entry = family_defect
            .entry(defect.source.clone())
            .or_insert(OpenVsPaidBoundaryNarrowReasonClass::NotNarrowed);
        if defect.narrow_reason.is_withdrawal_reason()
            || *entry == OpenVsPaidBoundaryNarrowReasonClass::NotNarrowed
        {
            *entry = defect.narrow_reason;
        }
    }
    let global_defect = family_defect
        .get("input")
        .copied()
        .unwrap_or(OpenVsPaidBoundaryNarrowReasonClass::NotNarrowed);

    input
        .capability_rows
        .iter()
        .map(|row| {
            let family_str = row.capability_family.as_str().to_owned();
            let row_narrow = {
                let per_family = family_defect
                    .get(&family_str)
                    .copied()
                    .unwrap_or(OpenVsPaidBoundaryNarrowReasonClass::NotNarrowed);
                if has_withdrawal
                    || per_family.is_withdrawal_reason()
                    || global_defect.is_withdrawal_reason()
                {
                    leading_narrow_reason
                } else if per_family != OpenVsPaidBoundaryNarrowReasonClass::NotNarrowed {
                    per_family
                } else if global_defect != OpenVsPaidBoundaryNarrowReasonClass::NotNarrowed {
                    global_defect
                } else {
                    OpenVsPaidBoundaryNarrowReasonClass::NotNarrowed
                }
            };

            let row_qual = if has_withdrawal {
                OpenVsPaidBoundaryQualificationClass::Withdrawn
            } else if row_narrow != OpenVsPaidBoundaryNarrowReasonClass::NotNarrowed {
                overall_qual
            } else {
                overall_qual
            };

            let summary = build_row_summary(
                &family_str,
                row.boundary_class.as_str(),
                &row_qual,
                row_narrow,
            );

            // Build optional packets for managed rows.
            let usage_export_packet = if row.boundary_class.requires_offboarding_disclosure() {
                Some(UsageExportPacket::new(
                    format!("policy:usage_export:{}:default", family_str),
                    row.capability_family,
                    UsageExportAvailabilityClass::Full,
                    false,
                    ExportRetentionClass::UserOwnedImmediate,
                    "default_quota_family",
                    true,
                    format!(
                        "Usage export for '{}': full availability, user-owned immediate \
                         retention, tenant-scoped data excluded.",
                        family_str
                    ),
                ))
            } else {
                None
            };

            let offboarding_packet = if row.boundary_class.requires_offboarding_disclosure() {
                Some(OffboardingPacket::new(
                    format!("policy:offboarding:{}:default", family_str),
                    row.capability_family,
                    OffboardingOutcomeClass::LocalOnly,
                    OffboardingOutcomeClass::PolicyRetained,
                    OffboardingOutcomeClass::Completed,
                    OffboardingOutcomeClass::PolicyRetained,
                    GraceWindowStateClass::Active,
                    true,
                    format!(
                        "Offboarding for '{}': local data remains on-device; managed data \
                         is policy-retained with disclosure. Export completes before grace \
                         window closes.",
                        family_str
                    ),
                ))
            } else {
                None
            };

            OpenVsPaidBoundaryRow {
                record_kind: OPEN_VS_PAID_BOUNDARY_ROW_RECORD_KIND.to_owned(),
                schema_version: OPEN_VS_PAID_BOUNDARY_SCHEMA_VERSION,
                shared_contract_ref: OPEN_VS_PAID_BOUNDARY_SHARED_CONTRACT_REF.to_owned(),
                capability_family_token: family_str,
                boundary_class_token: row.boundary_class.as_str().to_owned(),
                qualification_token: row_qual.as_str().to_owned(),
                narrow_reason_token: row_narrow.as_str().to_owned(),
                surfaces_consistent: row.surfaces_consistent,
                offboarding_disclosed: row.offboarding_disclosed,
                usage_export_disclosed: row.usage_export_disclosed,
                entitlement_loss_visible: row.entitlement_loss_visible,
                usage_export_packet,
                offboarding_packet,
                plain_language_summary: summary,
            }
        })
        .collect()
}

fn build_row_summary(
    family_token: &str,
    boundary_token: &str,
    qual: &OpenVsPaidBoundaryQualificationClass,
    narrow_reason: OpenVsPaidBoundaryNarrowReasonClass,
) -> String {
    match qual {
        OpenVsPaidBoundaryQualificationClass::Stable => format!(
            "Capability '{}' qualifies stable with boundary class '{}': \
             surface consistency verified, offboarding and usage-export disclosures \
             present where required, and local-core independence enforced.",
            family_token, boundary_token
        ),
        _ => format!(
            "Capability '{}' narrowed to {} ({}) with boundary class '{}': see defect list for details.",
            family_token,
            qual.as_str(),
            narrow_reason.as_str(),
            boundary_token
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded page
// ---------------------------------------------------------------------------

/// Build the seeded stable packet consumed by the headless example, the
/// integration tests, and the fixture generator.
///
/// The seeded page seeds zero defects: all local-core capabilities are
/// `open_local`; all managed capabilities carry offboarding and usage-export
/// disclosures; surface consistency is verified; and the usage-export schema
/// version is current.
pub fn seeded_open_vs_paid_boundary_page() -> OpenVsPaidBoundaryPage {
    OpenVsPaidBoundaryPage::new(
        "policy:open_vs_paid_boundary:default",
        "Open-versus-paid boundary manifest, managed-offering truth, \
         versioned usage export, and offboarding packet (stable)",
        "2026-06-01T00:00:00Z",
        seeded_open_vs_paid_boundary_input(),
    )
}

/// Build the seeded boundary input with zero defects.
pub fn seeded_open_vs_paid_boundary_input() -> OpenVsPaidBoundaryInput {
    OpenVsPaidBoundaryInput {
        usage_export_schema_version_current: true,
        capability_rows: vec![
            // Local-core floor — all open_local.
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::EditorCore,
                boundary_class: CapabilityBoundaryClass::OpenLocal,
                surfaces_consistent: true,
                offboarding_disclosed: false,
                usage_export_disclosed: false,
                entitlement_loss_visible: true,
            },
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::Search,
                boundary_class: CapabilityBoundaryClass::OpenLocal,
                surfaces_consistent: true,
                offboarding_disclosed: false,
                usage_export_disclosed: false,
                entitlement_loss_visible: true,
            },
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::LocalGit,
                boundary_class: CapabilityBoundaryClass::OpenLocal,
                surfaces_consistent: true,
                offboarding_disclosed: false,
                usage_export_disclosed: false,
                entitlement_loss_visible: true,
            },
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::Tasks,
                boundary_class: CapabilityBoundaryClass::OpenLocal,
                surfaces_consistent: true,
                offboarding_disclosed: false,
                usage_export_disclosed: false,
                entitlement_loss_visible: true,
            },
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::Debugging,
                boundary_class: CapabilityBoundaryClass::OpenLocal,
                surfaces_consistent: true,
                offboarding_disclosed: false,
                usage_export_disclosed: false,
                entitlement_loss_visible: true,
            },
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::LocalIndexing,
                boundary_class: CapabilityBoundaryClass::OpenLocal,
                surfaces_consistent: true,
                offboarding_disclosed: false,
                usage_export_disclosed: false,
                entitlement_loss_visible: true,
            },
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::LocalSafeAi,
                boundary_class: CapabilityBoundaryClass::OpenLocal,
                surfaces_consistent: true,
                offboarding_disclosed: false,
                usage_export_disclosed: false,
                entitlement_loss_visible: true,
            },
            // Managed/hosted capabilities.
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::Collaboration,
                boundary_class: CapabilityBoundaryClass::ManagedHosted,
                surfaces_consistent: true,
                offboarding_disclosed: true,
                usage_export_disclosed: true,
                entitlement_loss_visible: true,
            },
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::ManagedAiRouting,
                boundary_class: CapabilityBoundaryClass::ManagedHosted,
                surfaces_consistent: true,
                offboarding_disclosed: true,
                usage_export_disclosed: true,
                entitlement_loss_visible: true,
            },
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::AdminDashboard,
                boundary_class: CapabilityBoundaryClass::EnterpriseGoverned,
                surfaces_consistent: true,
                offboarding_disclosed: true,
                usage_export_disclosed: true,
                entitlement_loss_visible: true,
            },
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::PolicyEnforcement,
                boundary_class: CapabilityBoundaryClass::EnterpriseGoverned,
                surfaces_consistent: true,
                offboarding_disclosed: true,
                usage_export_disclosed: true,
                entitlement_loss_visible: true,
            },
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::ExtensionsMarketplace,
                boundary_class: CapabilityBoundaryClass::ManagedHosted,
                surfaces_consistent: true,
                offboarding_disclosed: true,
                usage_export_disclosed: true,
                entitlement_loss_visible: true,
            },
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::SupportExports,
                boundary_class: CapabilityBoundaryClass::OpenLocal,
                surfaces_consistent: true,
                offboarding_disclosed: false,
                usage_export_disclosed: false,
                entitlement_loss_visible: true,
            },
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::UsageAnalytics,
                boundary_class: CapabilityBoundaryClass::ManagedHosted,
                surfaces_consistent: true,
                offboarding_disclosed: true,
                usage_export_disclosed: true,
                entitlement_loss_visible: true,
            },
            CapabilityBoundaryInputRow {
                capability_family: CapabilityFamilyClass::BackupRestore,
                boundary_class: CapabilityBoundaryClass::EnterpriseGoverned,
                surfaces_consistent: true,
                offboarding_disclosed: true,
                usage_export_disclosed: true,
                entitlement_loss_visible: true,
            },
        ],
    }
}
