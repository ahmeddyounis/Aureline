//! Finalize backup, restore, failover, and local-core continuity packets for
//! claimed enterprise profiles.
//!
//! This module produces a beta proof packet that demonstrates, for each
//! required enterprise deployment profile:
//!
//! 1. Backup state is explicit: what is backed up, the retention window, the
//!    last-backup timestamp reference, and which entity owns the backup target
//!    region.
//! 2. Restore procedure has a declared test posture: whether it has been
//!    drilled, when the last drill occurred, and the expected recovery time and
//!    recovery point objective tokens.
//! 3. Failover behavior is declared explicitly: what happens to local
//!    capabilities during managed-connectivity outages. Local-core capabilities
//!    (file editing, save, search, Git) must remain operational or be explicitly
//!    degraded — they must not be blocked by default.
//! 4. Local-core continuity is stated explicitly on every row: the
//!    local-editing floor cannot be silently removed by a managed capability
//!    change or an enterprise profile switch.
//! 5. Tenant/region ownership, policy source, and dependency class are visible
//!    on every non-local enterprise profile row.
//!
//! One condition forces `Withdrawn` immediately and cannot be overridden:
//!
//! - A row where [`FailoverBehaviorClass::LocalCoreBlocked`] is declared and
//!   [`LocalCoreContinuityPostureClass::BlockedByDefault`] is the stated
//!   posture (narrow reason:
//!   [`BackupRestoreFailoverNarrowReasonClass::LocalCoreBlockedByFailover`]).
//!   Enterprise features must not block local-core work by default; any profile
//!   that does so cannot qualify claimable.
//!
//! Surfaces (admin console, support export, shell trust center, headless
//! inspector) read [`seeded_backup_restore_failover_page`] rather than minting
//! parallel backup/failover checks. The seed covers all five required
//! enterprise profiles ([`EnterpriseProfileClass::ALL`]) and proves that:
//!
//! - every failover scenario preserves the local-editing floor;
//! - backup state, restore test posture, and local-core continuity posture are
//!   explicit on every row;
//! - tenant/region ownership, policy source, and dependency class are declared
//!   on all non-local enterprise profile rows.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/finalize-backup-restore-failover-and-local-core-continuity.md`
//! - Artifact: `artifacts/enterprise/m4/finalize-backup-restore-failover-and-local-core-continuity.md`
//! - Contract ref: [`BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF: &str =
    "policy:backup_restore_failover_continuity:v1";

/// Record-kind tag for [`BackupRestoreFailoverPage`] payloads.
pub const BACKUP_RESTORE_FAILOVER_PAGE_RECORD_KIND: &str =
    "policy_backup_restore_failover_continuity_page_record";

/// Record-kind tag for [`BackupRestoreFailoverRow`] payloads.
pub const BACKUP_RESTORE_FAILOVER_ROW_RECORD_KIND: &str =
    "policy_backup_restore_failover_continuity_row_record";

/// Record-kind tag for [`BackupRestoreFailoverDefect`] payloads.
pub const BACKUP_RESTORE_FAILOVER_DEFECT_RECORD_KIND: &str =
    "policy_backup_restore_failover_continuity_defect_record";

/// Record-kind tag for [`BackupRestoreFailoverSummary`] payloads.
pub const BACKUP_RESTORE_FAILOVER_SUMMARY_RECORD_KIND: &str =
    "policy_backup_restore_failover_continuity_summary_record";

/// Record-kind tag for [`BackupRestoreFailoverSupportExport`] payloads.
pub const BACKUP_RESTORE_FAILOVER_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_backup_restore_failover_continuity_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const BACKUP_RESTORE_FAILOVER_DOC_REF: &str =
    "docs/enterprise/m4/finalize-backup-restore-failover-and-local-core-continuity.md";

/// Repo-relative path of the artifact summary for this lane.
pub const BACKUP_RESTORE_FAILOVER_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/finalize-backup-restore-failover-and-local-core-continuity.md";

// ---------------------------------------------------------------------------
// Enterprise profile vocabulary
// ---------------------------------------------------------------------------

/// Enterprise deployment profile covered by the backup/restore/failover row.
///
/// Uses the same token vocabulary as `deployment_profile` in the deployment
/// summary card so every row can be correlated with its residency proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnterpriseProfileClass {
    /// Desktop-local, single-user, no managed control plane.
    IndividualLocal,
    /// Customer-operated control plane with customer-managed keys and region.
    SelfHosted,
    /// Hybrid remote-attach with vendor-provided managed services.
    EnterpriseOnline,
    /// Offline-capable air-gapped mirror; no public egress.
    AirGapped,
    /// Vendor-operated SaaS with vendor-managed keys by default.
    ManagedCloud,
}

impl EnterpriseProfileClass {
    /// All required enterprise profiles in canonical order.
    pub const ALL: [Self; 5] = [
        Self::IndividualLocal,
        Self::SelfHosted,
        Self::EnterpriseOnline,
        Self::AirGapped,
        Self::ManagedCloud,
    ];

    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IndividualLocal => "individual_local",
            Self::SelfHosted => "self_hosted",
            Self::EnterpriseOnline => "enterprise_online",
            Self::AirGapped => "air_gapped",
            Self::ManagedCloud => "managed_cloud",
        }
    }

    /// True when this profile has no managed control plane and therefore no
    /// enterprise backup/restore scope beyond local file storage.
    pub const fn is_local_only(self) -> bool {
        matches!(self, Self::IndividualLocal)
    }

    /// True when tenant/region ownership and policy source must be declared.
    pub const fn requires_tenant_region_declaration(self) -> bool {
        !self.is_local_only()
    }

    /// True when this profile may declare connectivity-dependent failover paths.
    pub const fn has_managed_failover_path(self) -> bool {
        !self.is_local_only()
    }
}

// ---------------------------------------------------------------------------
// Backup state vocabulary
// ---------------------------------------------------------------------------

/// Current backup state for the enterprise profile's data scope.
///
/// Reported from the declared backup posture; the verifier does not inspect
/// raw backup artefacts, only the declared state tokens and opaque refs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupStateClass {
    /// Backup is within its declared retention window and has been verified.
    Current,
    /// Backup is scheduled but not yet complete.
    Pending,
    /// Backup is past its scheduled window without a completed run.
    Overdue,
    /// A backup artefact exists but has not been verified against the restore
    /// procedure.
    Unverified,
    /// No enterprise data scope exists for this profile; backup does not apply.
    NotApplicable,
}

impl BackupStateClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Pending => "pending",
            Self::Overdue => "overdue",
            Self::Unverified => "unverified",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the backup state indicates a gap that narrows the row.
    pub const fn is_deficient(self) -> bool {
        matches!(self, Self::Overdue | Self::Unverified)
    }
}

// ---------------------------------------------------------------------------
// Restore test posture vocabulary
// ---------------------------------------------------------------------------

/// Restore test posture for the enterprise profile's recovery procedure.
///
/// A restore procedure that has never been drilled cannot qualify stable, and
/// an overdue drill narrows to beta. The test validity window is declared on
/// the row rather than enforced here; the verifier checks only that the token
/// and timestamp reference are present and consistent.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreTestPostureClass {
    /// Restore procedure was drilled and the drill is within its declared
    /// validity window.
    TestedAndCurrent,
    /// Restore procedure was previously drilled but the drill is now past its
    /// declared validity window; renewal required.
    TestedOverdue,
    /// Restore procedure has never been drilled; no evidence that recovery is
    /// achievable.
    NeverTested,
    /// No restore path exists for this profile (local-only profiles with no
    /// enterprise data scope).
    NotApplicable,
}

impl RestoreTestPostureClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TestedAndCurrent => "tested_and_current",
            Self::TestedOverdue => "tested_overdue",
            Self::NeverTested => "never_tested",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when the restore posture narrows the row below stable.
    pub const fn is_deficient(self) -> bool {
        matches!(self, Self::TestedOverdue | Self::NeverTested)
    }
}

// ---------------------------------------------------------------------------
// Failover behavior vocabulary
// ---------------------------------------------------------------------------

/// Declared behavior during managed-connectivity outage or failover event.
///
/// This describes what happens to local IDE capabilities when the managed
/// control plane is unavailable. The hard invariant is that local-core
/// capabilities (file editing, save, search, Git) must not be blocked by
/// default.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FailoverBehaviorClass {
    /// Local editing and core capabilities remain fully operational during
    /// failover; managed features degrade independently.
    LocalCorePreserved,
    /// Managed capabilities are degraded or suspended during failover; the
    /// local editing floor is explicitly preserved and unaffected.
    DegradedManagedOnly,
    /// Failover may temporarily impair some local-core capabilities (e.g., a
    /// short sync delay) but does not block local editing.
    LocalCoreMayBeImpaired,
    /// Failover blocks local-core capabilities by default. This is a hard
    /// guardrail violation; rows with this token are withdrawn.
    LocalCoreBlocked,
    /// No managed failover path; all work is local and unaffected by
    /// connectivity state.
    NotApplicable,
}

impl FailoverBehaviorClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalCorePreserved => "local_core_preserved",
            Self::DegradedManagedOnly => "degraded_managed_only",
            Self::LocalCoreMayBeImpaired => "local_core_may_be_impaired",
            Self::LocalCoreBlocked => "local_core_blocked",
            Self::NotApplicable => "not_applicable",
        }
    }

    /// True when this failover behavior triggers immediate withdrawal.
    pub const fn is_withdrawal_trigger(self) -> bool {
        matches!(self, Self::LocalCoreBlocked)
    }
}

// ---------------------------------------------------------------------------
// Local-core continuity posture vocabulary
// ---------------------------------------------------------------------------

/// Explicit statement of local-core continuity for this enterprise profile.
///
/// Every row must carry an explicit posture token; a missing or ambiguous
/// token narrows the row. The local-editing floor is the floor below which no
/// enterprise feature or profile switch may reduce capability without an
/// explicit user decision and a visible downgrade label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCoreContinuityPostureClass {
    /// The local editing floor is fully preserved for this profile; no managed
    /// capability change or profile switch may remove it without explicit user
    /// consent.
    Preserved,
    /// A managed dependency may degrade some local-core capabilities under
    /// specific conditions, but the local editing floor is still intact; the
    /// dependency and conditions are named explicitly.
    ImpairedManagedDependency,
    /// The profile blocks local-core capabilities by default. This is a hard
    /// guardrail violation and withdraws the row.
    BlockedByDefault,
}

impl LocalCoreContinuityPostureClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::ImpairedManagedDependency => "impaired_managed_dependency",
            Self::BlockedByDefault => "blocked_by_default",
        }
    }

    /// True when this posture triggers immediate withdrawal.
    pub const fn is_withdrawal_trigger(self) -> bool {
        matches!(self, Self::BlockedByDefault)
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Qualification tier for the finalize page and its rows.
///
/// The tier is derived, not asserted: it is set by the audit against the
/// required conditions. A caller may never assert `stable` without a clean
/// audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupRestoreFailoverQualificationClass {
    /// All required conditions hold.
    Stable,
    /// One or more non-critical conditions are unmet.
    Beta,
    /// A required enterprise profile has no row; coverage gap prevents a beta
    /// claim.
    Preview,
    /// A hard guardrail was triggered; the page is withdrawn immediately.
    Withdrawn,
}

impl BackupRestoreFailoverQualificationClass {
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
/// [`BackupRestoreFailoverQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BackupRestoreFailoverNarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// The failover behavior is declared as blocking local-core work by default.
    /// This is a hard guardrail and withdraws the packet.
    LocalCoreBlockedByFailover,
    /// Local-core continuity posture is not explicitly stated on a row.
    LocalCoreContinuityNotExplicit,
    /// Backup state is unverified for a profile that claims enterprise backups.
    BackupStateUnverified,
    /// Backup is overdue for a profile that claims enterprise backups.
    BackupStateOverdue,
    /// Restore procedure has never been drilled for a profile with an
    /// enterprise data scope.
    RestoreNeverDrilled,
    /// Restore drill is overdue; the last drill is outside the declared
    /// validity window.
    RestoreDrillOverdue,
    /// Failover behavior declaration is absent for a profile that has a
    /// managed failover path.
    FailoverBehaviorNotDeclared,
    /// Tenant/region ownership is not declared for a non-local enterprise
    /// profile.
    TenantRegionOwnershipNotDeclared,
    /// Policy source is not declared for a non-local enterprise profile.
    PolicySourceNotDeclared,
    /// Dependency class is not declared for a non-local enterprise profile.
    DependencyClassNotDeclared,
    /// A required enterprise profile has no row; narrows to preview.
    ProfileCoverageGap,
}

impl BackupRestoreFailoverNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::LocalCoreBlockedByFailover => "local_core_blocked_by_failover",
            Self::LocalCoreContinuityNotExplicit => "local_core_continuity_not_explicit",
            Self::BackupStateUnverified => "backup_state_unverified",
            Self::BackupStateOverdue => "backup_state_overdue",
            Self::RestoreNeverDrilled => "restore_never_drilled",
            Self::RestoreDrillOverdue => "restore_drill_overdue",
            Self::FailoverBehaviorNotDeclared => "failover_behavior_not_declared",
            Self::TenantRegionOwnershipNotDeclared => "tenant_region_ownership_not_declared",
            Self::PolicySourceNotDeclared => "policy_source_not_declared",
            Self::DependencyClassNotDeclared => "dependency_class_not_declared",
            Self::ProfileCoverageGap => "profile_coverage_gap",
        }
    }

    /// True when this reason triggers immediate withdrawal and cannot be
    /// overridden.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::LocalCoreBlockedByFailover)
    }
}

// ---------------------------------------------------------------------------
// Row-level backup/restore/failover declaration
// ---------------------------------------------------------------------------

/// Backup declaration for one enterprise profile row.
///
/// All fields are opaque refs or closed-vocabulary tokens; raw backup
/// artefacts, raw hostnames, raw credentials, and raw region labels stay
/// outside the support boundary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupDeclaration {
    /// Current state of the backup for this profile's data scope.
    pub backup_state: BackupStateClass,
    /// Stable token for [`Self::backup_state`].
    pub backup_state_token: String,
    /// Opaque ref to the last completed and verified backup artefact.
    /// Empty when `backup_state` is `not_applicable`.
    pub last_backup_ref: String,
    /// ISO 8601 timestamp reference for the last completed backup run.
    /// Empty when `backup_state` is `not_applicable`.
    pub last_backup_time: String,
    /// Declared retention window token (e.g., `rolling_30d`, `point_in_time_7d`).
    pub retention_window_token: String,
    /// Opaque ref identifying the backup target region owner.
    /// Empty when `backup_state` is `not_applicable`.
    pub backup_target_region_owner_ref: String,
    /// Plain-language labels for data scopes included in the backup.
    pub included_data_scope_labels: Vec<String>,
    /// Plain-language labels for data scopes explicitly excluded from the
    /// backup with a brief reason.
    pub excluded_data_scope_labels: Vec<String>,
}

impl BackupDeclaration {
    /// True when backup fields are fully declared for profiles that require them.
    pub fn is_declared_for_profile(&self, profile: EnterpriseProfileClass) -> bool {
        if profile.is_local_only() {
            return true;
        }
        self.backup_state != BackupStateClass::Unverified
            && !self.retention_window_token.is_empty()
            && !self.backup_target_region_owner_ref.is_empty()
    }
}

/// Restore test declaration for one enterprise profile row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RestoreTestDeclaration {
    /// Current restore test posture for this profile.
    pub restore_test_posture: RestoreTestPostureClass,
    /// Stable token for [`Self::restore_test_posture`].
    pub restore_test_posture_token: String,
    /// ISO 8601 timestamp reference for the last completed restore drill.
    /// Empty when posture is `not_applicable` or `never_tested`.
    pub last_drill_time: String,
    /// ISO 8601 timestamp until which the last drill is considered valid.
    /// Empty when posture is `not_applicable` or `never_tested`.
    pub drill_valid_until: String,
    /// Declared recovery time objective token (e.g., `rto_4h`, `rto_24h`).
    pub rto_token: String,
    /// Declared recovery point objective token (e.g., `rpo_1h`, `rpo_24h`).
    pub rpo_token: String,
    /// Plain-language label for the restore scope (what can and cannot be
    /// recovered from this restore path).
    pub restore_scope_label: String,
}

/// Failover and local-core continuity declaration for one enterprise profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FailoverContinuityDeclaration {
    /// Declared failover behavior for this profile.
    pub failover_behavior: FailoverBehaviorClass,
    /// Stable token for [`Self::failover_behavior`].
    pub failover_behavior_token: String,
    /// Explicit local-core continuity posture for this profile.
    pub local_core_posture: LocalCoreContinuityPostureClass,
    /// Stable token for [`Self::local_core_posture`].
    pub local_core_posture_token: String,
    /// Plain-language description of what local capabilities remain available
    /// during failover.
    pub local_capabilities_during_failover: Vec<String>,
    /// Plain-language description of managed capabilities that are suspended
    /// or degraded during failover.
    pub degraded_managed_capabilities: Vec<String>,
    /// Opaque ref identifying the managed dependency that would trigger
    /// failover or degradation. Empty for local-only profiles.
    pub managed_dependency_ref: String,
    /// Explicit downgrade label shown in UI when the failover condition is
    /// active. Must be non-empty for all non-local profiles.
    pub downgrade_label: String,
}

impl FailoverContinuityDeclaration {
    /// True when the failover and continuity declaration is complete for
    /// profiles that require it.
    pub fn is_declared_for_profile(&self, profile: EnterpriseProfileClass) -> bool {
        if profile.is_local_only() {
            return true;
        }
        !self.failover_behavior_token.is_empty() && !self.downgrade_label.is_empty()
    }
}

// ---------------------------------------------------------------------------
// Row, summary, defect
// ---------------------------------------------------------------------------

/// Finalize row for one enterprise deployment profile.
///
/// The row is the unit of qualification. Each row must carry a fully declared
/// backup state, restore test posture, failover behavior, and explicit
/// local-core continuity posture. Failure on any required condition narrows
/// the row and the page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Enterprise profile this row covers.
    pub enterprise_profile: EnterpriseProfileClass,
    /// Stable token for [`Self::enterprise_profile`].
    pub enterprise_profile_token: String,
    /// Backup declaration for this profile.
    pub backup: BackupDeclaration,
    /// Restore test declaration for this profile.
    pub restore: RestoreTestDeclaration,
    /// Failover and local-core continuity declaration for this profile.
    pub failover_continuity: FailoverContinuityDeclaration,
    /// Opaque ref identifying the tenant/org scope owner for this profile.
    /// Empty for `individual_local`.
    pub tenant_region_owner_ref: String,
    /// Opaque ref identifying the policy source that governs backup retention
    /// and restore authorisation for this profile.
    /// Empty for `individual_local`.
    pub policy_source_ref: String,
    /// Declared dependency class token for this profile's backup/restore path.
    /// Empty for `individual_local`.
    pub dependency_class_token: String,
    /// Derived qualification tier for this row.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed` when stable).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

/// Aggregate summary for the finalize page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct BackupRestoreFailoverSummary {
    /// Total row count.
    pub row_count: usize,
    /// Rows that qualify stable.
    pub stable_row_count: usize,
    /// Rows narrowed to beta.
    pub beta_row_count: usize,
    /// Rows narrowed to preview.
    pub preview_row_count: usize,
    /// Rows withdrawn.
    pub withdrawn_row_count: usize,
    /// Enterprise profile tokens present on the page.
    pub profiles_covered: Vec<String>,
    /// Number of rows with local-core continuity explicitly preserved.
    pub local_core_preserved_row_count: usize,
    /// Number of rows with a current verified backup.
    pub backup_current_row_count: usize,
    /// Number of rows with a tested-and-current restore posture.
    pub restore_tested_row_count: usize,
    /// Number of rows with a non-blocking failover behavior.
    pub failover_non_blocking_row_count: usize,
    /// Overall qualification token derived from all rows.
    pub overall_qualification_token: String,
}

impl BackupRestoreFailoverSummary {
    fn from_rows(rows: &[BackupRestoreFailoverRow]) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        let mut profiles: BTreeSet<String> = BTreeSet::new();
        let mut local_core_preserved = 0usize;
        let mut backup_current = 0usize;
        let mut restore_tested = 0usize;
        let mut failover_non_blocking = 0usize;

        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
            profiles.insert(row.enterprise_profile_token.clone());
            if row.failover_continuity.local_core_posture
                == LocalCoreContinuityPostureClass::Preserved
            {
                local_core_preserved += 1;
            }
            if row.backup.backup_state == BackupStateClass::Current
                || row.backup.backup_state == BackupStateClass::NotApplicable
            {
                backup_current += 1;
            }
            if row.restore.restore_test_posture == RestoreTestPostureClass::TestedAndCurrent
                || row.restore.restore_test_posture == RestoreTestPostureClass::NotApplicable
            {
                restore_tested += 1;
            }
            if !row.failover_continuity.failover_behavior.is_withdrawal_trigger() {
                failover_non_blocking += 1;
            }
        }

        let overall = if withdrawn > 0 {
            BackupRestoreFailoverQualificationClass::Withdrawn
        } else if preview > 0 {
            BackupRestoreFailoverQualificationClass::Preview
        } else if beta > 0 {
            BackupRestoreFailoverQualificationClass::Beta
        } else {
            BackupRestoreFailoverQualificationClass::Stable
        };

        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            profiles_covered: profiles.into_iter().collect(),
            local_core_preserved_row_count: local_core_preserved,
            backup_current_row_count: backup_current,
            restore_tested_row_count: restore_tested,
            failover_non_blocking_row_count: failover_non_blocking,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

/// Typed defect emitted by the finalize-page audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: BackupRestoreFailoverNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (row id or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl BackupRestoreFailoverDefect {
    fn new(
        narrow_reason: BackupRestoreFailoverNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: BACKUP_RESTORE_FAILOVER_DEFECT_RECORD_KIND.to_owned(),
            schema_version: BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
            shared_contract_ref: BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:backup-restore-failover:{}:{}",
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

/// Beta proof packet for backup, restore, failover, and local-core continuity
/// across claimed enterprise profiles.
///
/// This is the single inspectable record that proves the claims for this lane.
/// Dashboards, docs, Help/About surfaces, and support exports should ingest it
/// rather than cloning status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverPage {
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
    pub summary: BackupRestoreFailoverSummary,
    /// Per-profile qualification rows (one per enterprise profile).
    pub rows: Vec<BackupRestoreFailoverRow>,
    /// Typed validation defects for this packet.
    pub defects: Vec<BackupRestoreFailoverDefect>,
}

impl BackupRestoreFailoverPage {
    /// Build the finalize page from a set of rows.
    ///
    /// Defects are derived automatically from the audit. Rows are
    /// re-qualified based on the combined audit result.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<BackupRestoreFailoverRow>,
    ) -> Self {
        let defects = audit_backup_restore_failover_rows(&rows);
        let qualified_rows = qualify_rows(rows, &defects);
        let summary = BackupRestoreFailoverSummary::from_rows(&qualified_rows);
        Self {
            record_kind: BACKUP_RESTORE_FAILOVER_PAGE_RECORD_KIND.to_owned(),
            schema_version: BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
            shared_contract_ref: BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows: qualified_rows,
            defects,
        }
    }

    /// True when the overall qualification is stable.
    pub fn qualifies_stable(&self) -> bool {
        self.summary.overall_qualification_token
            == BackupRestoreFailoverQualificationClass::Stable.as_str()
    }

    /// True when no withdrawn rows are present.
    pub fn no_withdrawn_rows(&self) -> bool {
        self.summary.withdrawn_row_count == 0
    }

    /// True when all required enterprise profiles are covered.
    pub fn covers_all_required_profiles(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .map(|r| r.enterprise_profile_token.as_str())
            .collect();
        EnterpriseProfileClass::ALL
            .iter()
            .all(|p| covered.contains(p.as_str()))
    }

    /// True when all rows carry an explicitly preserved local-core continuity
    /// posture.
    pub fn all_rows_preserve_local_core(&self) -> bool {
        self.rows.iter().all(|r| {
            r.failover_continuity.local_core_posture == LocalCoreContinuityPostureClass::Preserved
                || r.failover_continuity.local_core_posture
                    == LocalCoreContinuityPostureClass::ImpairedManagedDependency
        })
    }

    /// True when no row declares a blocking failover behavior.
    pub fn no_row_blocks_local_core_by_failover(&self) -> bool {
        self.rows
            .iter()
            .all(|r| !r.failover_continuity.failover_behavior.is_withdrawal_trigger())
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper that quotes the finalize page plus a metadata-safe
/// defect roll-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackupRestoreFailoverSupportExport {
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
    /// The finalize page embedded as evidence.
    pub page: BackupRestoreFailoverPage,
    /// Narrow-reason tokens present in the page's defect list.
    pub narrow_reasons_present: Vec<BackupRestoreFailoverNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when raw private material is excluded from the export.
    pub raw_private_material_excluded: bool,
}

impl BackupRestoreFailoverSupportExport {
    /// Wrap a finalize page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: BackupRestoreFailoverPage,
    ) -> Self {
        let mut reasons: Vec<BackupRestoreFailoverNarrowReasonClass> = Vec::new();
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
            record_kind: BACKUP_RESTORE_FAILOVER_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
            shared_contract_ref: BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF.to_owned(),
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
// Public audit and validate functions
// ---------------------------------------------------------------------------

/// Re-run the finalize audit over the rows in the page.
pub fn audit_backup_restore_failover_page(
    page: &BackupRestoreFailoverPage,
) -> Vec<BackupRestoreFailoverDefect> {
    audit_backup_restore_failover_rows(&page.rows)
}

/// Validate the finalize page; returns `Ok` when the audit is clean.
pub fn validate_backup_restore_failover_page(
    page: &BackupRestoreFailoverPage,
) -> Result<(), Vec<BackupRestoreFailoverDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

/// Build the seeded finalize page covering all five required enterprise
/// profiles with backup, restore, failover, and local-core continuity declared.
pub fn seeded_backup_restore_failover_page() -> BackupRestoreFailoverPage {
    BackupRestoreFailoverPage::new(
        "policy:backup-restore-failover-continuity:seeded:0001",
        "Backup, restore, failover, and local-core continuity finalize packet",
        "2026-06-01T00:00:00Z",
        seeded_rows(),
    )
}

// ---------------------------------------------------------------------------
// Internal audit helpers
// ---------------------------------------------------------------------------

fn audit_backup_restore_failover_rows(
    rows: &[BackupRestoreFailoverRow],
) -> Vec<BackupRestoreFailoverDefect> {
    let mut defects: Vec<BackupRestoreFailoverDefect> = Vec::new();

    for row in rows {
        // Hard guardrail: failover blocks local-core work.
        if row.failover_continuity.failover_behavior.is_withdrawal_trigger()
            || row.failover_continuity.local_core_posture.is_withdrawal_trigger()
        {
            defects.push(BackupRestoreFailoverDefect::new(
                BackupRestoreFailoverNarrowReasonClass::LocalCoreBlockedByFailover,
                row.row_id.clone(),
                "row declares a failover behavior or local-core posture that blocks local-core \
                 work by default; enterprise features must not block local-core capabilities",
            ));
            // Withdrawal defect: skip further checks for this row.
            continue;
        }

        // Local-core continuity must be explicitly stated.
        if row.failover_continuity.local_core_posture_token.is_empty() {
            defects.push(BackupRestoreFailoverDefect::new(
                BackupRestoreFailoverNarrowReasonClass::LocalCoreContinuityNotExplicit,
                row.row_id.clone(),
                "row does not carry an explicit local_core_posture_token",
            ));
        }

        // Backup state checks for enterprise profiles.
        if !row.enterprise_profile.is_local_only() {
            if row.backup.backup_state == BackupStateClass::Unverified {
                defects.push(BackupRestoreFailoverDefect::new(
                    BackupRestoreFailoverNarrowReasonClass::BackupStateUnverified,
                    row.row_id.clone(),
                    "backup state is unverified; backup must be verified against the restore \
                     procedure before claiming a stable qualification",
                ));
            }
            if row.backup.backup_state == BackupStateClass::Overdue {
                defects.push(BackupRestoreFailoverDefect::new(
                    BackupRestoreFailoverNarrowReasonClass::BackupStateOverdue,
                    row.row_id.clone(),
                    "backup is overdue; the last backup run has passed its scheduled window",
                ));
            }
            // Restore test posture checks.
            if row.restore.restore_test_posture == RestoreTestPostureClass::NeverTested {
                defects.push(BackupRestoreFailoverDefect::new(
                    BackupRestoreFailoverNarrowReasonClass::RestoreNeverDrilled,
                    row.row_id.clone(),
                    "restore procedure has never been drilled; no evidence that recovery is \
                     achievable for this enterprise profile",
                ));
            }
            if row.restore.restore_test_posture == RestoreTestPostureClass::TestedOverdue {
                defects.push(BackupRestoreFailoverDefect::new(
                    BackupRestoreFailoverNarrowReasonClass::RestoreDrillOverdue,
                    row.row_id.clone(),
                    "restore drill is overdue; the last drill is outside the declared validity \
                     window",
                ));
            }
            // Failover behavior must be declared.
            if row.failover_continuity.failover_behavior_token.is_empty() {
                defects.push(BackupRestoreFailoverDefect::new(
                    BackupRestoreFailoverNarrowReasonClass::FailoverBehaviorNotDeclared,
                    row.row_id.clone(),
                    "failover behavior token is missing for a profile with a managed failover path",
                ));
            }
            // Tenant/region ownership must be declared.
            if row.tenant_region_owner_ref.is_empty() {
                defects.push(BackupRestoreFailoverDefect::new(
                    BackupRestoreFailoverNarrowReasonClass::TenantRegionOwnershipNotDeclared,
                    row.row_id.clone(),
                    "tenant/region ownership ref is missing for a non-local enterprise profile",
                ));
            }
            // Policy source must be declared.
            if row.policy_source_ref.is_empty() {
                defects.push(BackupRestoreFailoverDefect::new(
                    BackupRestoreFailoverNarrowReasonClass::PolicySourceNotDeclared,
                    row.row_id.clone(),
                    "policy source ref is missing for a non-local enterprise profile",
                ));
            }
            // Dependency class must be declared.
            if row.dependency_class_token.is_empty() {
                defects.push(BackupRestoreFailoverDefect::new(
                    BackupRestoreFailoverNarrowReasonClass::DependencyClassNotDeclared,
                    row.row_id.clone(),
                    "dependency class token is missing for a non-local enterprise profile",
                ));
            }
        }
    }

    // Coverage check: all required enterprise profiles must appear.
    let required_profiles: BTreeSet<&str> = EnterpriseProfileClass::ALL
        .iter()
        .map(|p| p.as_str())
        .collect();
    let observed_profiles: BTreeSet<&str> = rows
        .iter()
        .map(|r| r.enterprise_profile_token.as_str())
        .collect();
    for missing in required_profiles.difference(&observed_profiles) {
        defects.push(BackupRestoreFailoverDefect::new(
            BackupRestoreFailoverNarrowReasonClass::ProfileCoverageGap,
            "page",
            format!(
                "missing row for required enterprise profile '{missing}'; packet is narrowed to \
                 preview"
            ),
        ));
    }

    defects
}

fn qualify_rows(
    mut rows: Vec<BackupRestoreFailoverRow>,
    page_defects: &[BackupRestoreFailoverDefect],
) -> Vec<BackupRestoreFailoverRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects.iter().any(|d| {
        d.narrow_reason == BackupRestoreFailoverNarrowReasonClass::ProfileCoverageGap
    });

    let (overall_qual, overall_reason) = if has_withdrawal {
        let r = page_defects
            .iter()
            .find(|d| d.narrow_reason.is_withdrawal_reason())
            .map(|d| d.narrow_reason)
            .unwrap_or(BackupRestoreFailoverNarrowReasonClass::LocalCoreBlockedByFailover);
        (BackupRestoreFailoverQualificationClass::Withdrawn, r)
    } else if has_preview {
        (
            BackupRestoreFailoverQualificationClass::Preview,
            BackupRestoreFailoverNarrowReasonClass::ProfileCoverageGap,
        )
    } else if !page_defects.is_empty() {
        let r = page_defects[0].narrow_reason;
        (BackupRestoreFailoverQualificationClass::Beta, r)
    } else {
        (
            BackupRestoreFailoverQualificationClass::Stable,
            BackupRestoreFailoverNarrowReasonClass::NotNarrowed,
        )
    };

    for row in &mut rows {
        let row_qual = if has_withdrawal {
            BackupRestoreFailoverQualificationClass::Withdrawn
        } else if has_preview {
            BackupRestoreFailoverQualificationClass::Preview
        } else {
            let row_has_defect = page_defects.iter().any(|d| d.source == row.row_id);
            if row_has_defect || !page_defects.is_empty() {
                BackupRestoreFailoverQualificationClass::Beta
            } else {
                BackupRestoreFailoverQualificationClass::Stable
            }
        };

        let row_reason = if row_qual == overall_qual {
            overall_reason
        } else {
            page_defects
                .iter()
                .find(|d| d.source == row.row_id)
                .map(|d| d.narrow_reason)
                .unwrap_or(BackupRestoreFailoverNarrowReasonClass::NotNarrowed)
        };

        row.qualification_token = row_qual.as_str().to_owned();
        row.narrow_reason_token = row_reason.as_str().to_owned();
        row.plain_language_summary = build_row_summary(
            &row.row_id,
            &row.enterprise_profile_token,
            row_qual,
            row_reason,
        );
    }

    rows
}

fn build_row_summary(
    row_id: &str,
    profile_token: &str,
    qual: BackupRestoreFailoverQualificationClass,
    narrow_reason: BackupRestoreFailoverNarrowReasonClass,
) -> String {
    match qual {
        BackupRestoreFailoverQualificationClass::Stable => format!(
            "Row '{row_id}' ({profile_token}) qualifies stable: backup current, restore drilled, \
             failover behavior declared, local-core continuity explicit."
        ),
        BackupRestoreFailoverQualificationClass::Beta => format!(
            "Row '{row_id}' ({profile_token}) narrowed to beta (reason: {}): one or more required \
             conditions are unmet.",
            narrow_reason.as_str()
        ),
        BackupRestoreFailoverQualificationClass::Preview => format!(
            "Row '{row_id}' ({profile_token}) narrowed to preview: a required enterprise profile \
             is missing from the page."
        ),
        BackupRestoreFailoverQualificationClass::Withdrawn => format!(
            "Row '{row_id}' ({profile_token}) is withdrawn (reason: {}): hard guardrail \
             triggered — enterprise feature blocks local-core work by default.",
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded rows
// ---------------------------------------------------------------------------

fn seeded_rows() -> Vec<BackupRestoreFailoverRow> {
    vec![
        row_individual_local(),
        row_self_hosted(),
        row_enterprise_online(),
        row_air_gapped(),
        row_managed_cloud(),
    ]
}

fn make_row(
    row_id: &str,
    profile: EnterpriseProfileClass,
    backup_state: BackupStateClass,
    last_backup_ref: &str,
    last_backup_time: &str,
    retention_window_token: &str,
    backup_target_region_owner_ref: &str,
    included_data_scopes: Vec<&str>,
    excluded_data_scopes: Vec<&str>,
    restore_test_posture: RestoreTestPostureClass,
    last_drill_time: &str,
    drill_valid_until: &str,
    rto_token: &str,
    rpo_token: &str,
    restore_scope_label: &str,
    failover_behavior: FailoverBehaviorClass,
    local_core_posture: LocalCoreContinuityPostureClass,
    local_capabilities_during_failover: Vec<&str>,
    degraded_managed_capabilities: Vec<&str>,
    managed_dependency_ref: &str,
    downgrade_label: &str,
    tenant_region_owner_ref: &str,
    policy_source_ref: &str,
    dependency_class_token: &str,
) -> BackupRestoreFailoverRow {
    BackupRestoreFailoverRow {
        record_kind: BACKUP_RESTORE_FAILOVER_ROW_RECORD_KIND.to_owned(),
        schema_version: BACKUP_RESTORE_FAILOVER_SCHEMA_VERSION,
        shared_contract_ref: BACKUP_RESTORE_FAILOVER_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        enterprise_profile: profile,
        enterprise_profile_token: profile.as_str().to_owned(),
        backup: BackupDeclaration {
            backup_state,
            backup_state_token: backup_state.as_str().to_owned(),
            last_backup_ref: last_backup_ref.to_owned(),
            last_backup_time: last_backup_time.to_owned(),
            retention_window_token: retention_window_token.to_owned(),
            backup_target_region_owner_ref: backup_target_region_owner_ref.to_owned(),
            included_data_scope_labels: included_data_scopes
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
            excluded_data_scope_labels: excluded_data_scopes
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
        },
        restore: RestoreTestDeclaration {
            restore_test_posture,
            restore_test_posture_token: restore_test_posture.as_str().to_owned(),
            last_drill_time: last_drill_time.to_owned(),
            drill_valid_until: drill_valid_until.to_owned(),
            rto_token: rto_token.to_owned(),
            rpo_token: rpo_token.to_owned(),
            restore_scope_label: restore_scope_label.to_owned(),
        },
        failover_continuity: FailoverContinuityDeclaration {
            failover_behavior,
            failover_behavior_token: failover_behavior.as_str().to_owned(),
            local_core_posture,
            local_core_posture_token: local_core_posture.as_str().to_owned(),
            local_capabilities_during_failover: local_capabilities_during_failover
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
            degraded_managed_capabilities: degraded_managed_capabilities
                .iter()
                .map(|s| (*s).to_owned())
                .collect(),
            managed_dependency_ref: managed_dependency_ref.to_owned(),
            downgrade_label: downgrade_label.to_owned(),
        },
        tenant_region_owner_ref: tenant_region_owner_ref.to_owned(),
        policy_source_ref: policy_source_ref.to_owned(),
        dependency_class_token: dependency_class_token.to_owned(),
        // Qualification fields are filled in by qualify_rows.
        qualification_token: BackupRestoreFailoverQualificationClass::Stable
            .as_str()
            .to_owned(),
        narrow_reason_token: BackupRestoreFailoverNarrowReasonClass::NotNarrowed
            .as_str()
            .to_owned(),
        plain_language_summary: String::new(),
    }
}

fn row_individual_local() -> BackupRestoreFailoverRow {
    make_row(
        "backup-restore-failover:individual_local",
        EnterpriseProfileClass::IndividualLocal,
        BackupStateClass::NotApplicable,
        "",
        "",
        "",
        "",
        vec![],
        vec![],
        RestoreTestPostureClass::NotApplicable,
        "",
        "",
        "",
        "",
        "No enterprise data scope; local files are managed by the user's own OS backup \
         strategy.",
        FailoverBehaviorClass::NotApplicable,
        LocalCoreContinuityPostureClass::Preserved,
        vec![
            "file editing",
            "save",
            "search",
            "git",
            "language features",
        ],
        vec![],
        "",
        "",
        "",
        "",
        "",
    )
}

fn row_self_hosted() -> BackupRestoreFailoverRow {
    make_row(
        "backup-restore-failover:self_hosted",
        EnterpriseProfileClass::SelfHosted,
        BackupStateClass::Current,
        "backup:self-hosted:config-and-policy:2026.05.0001",
        "2026-05-28T02:00:00Z",
        "rolling_30d",
        "tenant-region-owner:customer-operated:self-hosted",
        vec![
            "policy bundles",
            "entitlement snapshots",
            "admin configuration",
            "audit logs",
        ],
        vec![
            "local workspace files (user-managed, outside enterprise data scope)",
        ],
        RestoreTestPostureClass::TestedAndCurrent,
        "2026-04-15T09:00:00Z",
        "2026-07-15T09:00:00Z",
        "rto_4h",
        "rpo_1h",
        "Full restore of policy bundles, entitlement snapshots, admin configuration, and audit \
         logs from rolling-30d backup to a clean self-hosted instance.",
        FailoverBehaviorClass::DegradedManagedOnly,
        LocalCoreContinuityPostureClass::Preserved,
        vec![
            "file editing",
            "save",
            "search",
            "git",
            "language features",
        ],
        vec![
            "managed policy enforcement (last-known-good rules apply)",
            "entitlement refresh (cached seat state applies)",
            "admin console (unavailable until connectivity restored)",
        ],
        "managed-dep:self-hosted-control-plane",
        "Self-hosted control plane is offline. Local editing and core capabilities remain \
         fully operational. Managed policy enforcement uses last-known-good rules until \
         connectivity is restored.",
        "tenant-region-owner:customer-operated:self-hosted",
        "policy-source:self-hosted-admin-bundle",
        "customer_operated_control_plane",
    )
}

fn row_enterprise_online() -> BackupRestoreFailoverRow {
    make_row(
        "backup-restore-failover:enterprise_online",
        EnterpriseProfileClass::EnterpriseOnline,
        BackupStateClass::Current,
        "backup:enterprise-online:config-and-policy:2026.05.0002",
        "2026-05-29T03:00:00Z",
        "point_in_time_7d",
        "tenant-region-owner:vendor-assisted:enterprise-online",
        vec![
            "policy bundles",
            "entitlement snapshots",
            "admin configuration",
            "relay state",
            "audit logs",
        ],
        vec![
            "local workspace files (user-managed, outside enterprise data scope)",
        ],
        RestoreTestPostureClass::TestedAndCurrent,
        "2026-04-20T10:00:00Z",
        "2026-07-20T10:00:00Z",
        "rto_2h",
        "rpo_30m",
        "Full restore of policy bundles, entitlement snapshots, admin configuration, relay \
         state, and audit logs from point-in-time-7d backup.",
        FailoverBehaviorClass::DegradedManagedOnly,
        LocalCoreContinuityPostureClass::Preserved,
        vec![
            "file editing",
            "save",
            "search",
            "git",
            "language features",
        ],
        vec![
            "managed AI features (suspended during outage)",
            "policy enforcement (last-known-good rules apply)",
            "entitlement refresh (cached seat state applies)",
        ],
        "managed-dep:vendor-assisted-relay",
        "Enterprise relay is offline. Local editing and core capabilities remain fully \
         operational. Managed AI features and policy enforcement use cached state until \
         connectivity is restored.",
        "tenant-region-owner:vendor-assisted:enterprise-online",
        "policy-source:enterprise-online-managed-bundle",
        "vendor_assisted_managed_services",
    )
}

fn row_air_gapped() -> BackupRestoreFailoverRow {
    make_row(
        "backup-restore-failover:air_gapped",
        EnterpriseProfileClass::AirGapped,
        BackupStateClass::Current,
        "backup:air-gapped:config-and-mirror:2026.04.0001",
        "2026-04-01T06:00:00Z",
        "rolling_90d",
        "tenant-region-owner:customer-operated:air-gapped",
        vec![
            "policy bundles (signed mirror snapshots)",
            "entitlement snapshots",
            "admin configuration",
            "mirror artefact index",
            "audit logs",
        ],
        vec![
            "live origin content (unavailable by design in air-gapped deployments)",
        ],
        RestoreTestPostureClass::TestedAndCurrent,
        "2026-03-10T08:00:00Z",
        "2026-09-10T08:00:00Z",
        "rto_8h",
        "rpo_24h",
        "Full restore of signed mirror snapshots, entitlement snapshots, admin configuration, \
         and audit logs from the most recent verified offline backup. Recovery to a clean \
         air-gapped instance.",
        FailoverBehaviorClass::LocalCorePreserved,
        LocalCoreContinuityPostureClass::Preserved,
        vec![
            "file editing",
            "save",
            "search",
            "git",
            "language features",
            "local mirror content",
        ],
        vec![
            "live origin policy refresh (not applicable in air-gapped mode)",
        ],
        "managed-dep:air-gapped-mirror-sync",
        "Air-gapped mirror sync is paused. All local editing and mirror-served capabilities \
         remain fully operational. Live origin connectivity is unavailable by design.",
        "tenant-region-owner:customer-operated:air-gapped",
        "policy-source:air-gapped-mirror-bundle",
        "customer_operated_air_gapped",
    )
}

fn row_managed_cloud() -> BackupRestoreFailoverRow {
    make_row(
        "backup-restore-failover:managed_cloud",
        EnterpriseProfileClass::ManagedCloud,
        BackupStateClass::Current,
        "backup:managed-cloud:config-and-state:2026.05.0003",
        "2026-05-30T04:00:00Z",
        "point_in_time_7d",
        "tenant-region-owner:vendor-managed:managed-cloud",
        vec![
            "policy bundles",
            "entitlement snapshots",
            "admin configuration",
            "audit logs",
            "managed workspace state",
        ],
        vec![
            "local workspace files (user-managed, outside enterprise data scope)",
        ],
        RestoreTestPostureClass::TestedAndCurrent,
        "2026-05-01T11:00:00Z",
        "2026-08-01T11:00:00Z",
        "rto_1h",
        "rpo_15m",
        "Full restore of policy bundles, entitlement snapshots, admin configuration, audit \
         logs, and managed workspace state from point-in-time-7d backup.",
        FailoverBehaviorClass::DegradedManagedOnly,
        LocalCoreContinuityPostureClass::Preserved,
        vec![
            "file editing",
            "save",
            "search",
            "git",
            "language features",
        ],
        vec![
            "managed AI features (suspended during outage)",
            "cloud workspace sync (suspended during outage)",
            "entitlement refresh (cached seat state applies)",
            "policy enforcement (last-known-good rules apply)",
        ],
        "managed-dep:vendor-managed-cloud-control-plane",
        "Managed cloud control plane is offline. Local editing and core capabilities remain \
         fully operational. Managed AI features and cloud sync are suspended until the \
         control plane is restored.",
        "tenant-region-owner:vendor-managed:managed-cloud",
        "policy-source:managed-cloud-vendor-bundle",
        "vendor_managed_cloud",
    )
}
