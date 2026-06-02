//! Hardened identity and admin support-export parity, audit-safe redaction, and
//! no-vendor-control-plane local governance paths.
//!
//! This module produces a stable proof packet that demonstrates enterprise
//! identity and admin surfaces follow a typed contract for directory/provider
//! cards, user/seat lifecycle rows, provisioning failure distinction, policy-
//! target dry runs, and local governance path documentation. Every admin,
//! support, shell, and review surface can explain:
//!
//! 1. **Which provisioning class is active** — via a closed
//!    [`IdentityAdminProvisioningClass`] vocabulary (`oidc`, `scim`,
//!    `signed_file_bundle`, `manual`).
//! 2. **What sync freshness applies** — via a typed
//!    [`IdentityAdminSyncFreshnessClass`] (`live`, `cached`, `stale`,
//!    `expired`, `missing`).
//! 3. **What fallback or manual path exists** — every applicable row carries a
//!    non-empty `fallback_manual_path_label`.
//! 4. **What remains local versus tenant-scoped** — every row declares an exact
//!    [`LocalTenantScopeClass`] (`local_state_only`, `tenant_scoped`,
//!    `hybrid_local_tenant`) and a plain-language `local_artifact_safety_note`.
//! 5. **Admin action/result lineage** — every row carries an
//!    [`AdminActionLineage`] block that names the action, result, actor ref,
//!    and timestamp without exposing raw secrets or raw tenant data.
//! 6. **Distinguished provisioning failures** — policy-target dry runs and
//!    provisioning failure rows name a specific [`ProvisioningFailureKind`
//!    ] (`provider_outage`, `auth_drift`, `scope_mismatch`, `seat_loss`,
//!    `deprovisioning_impact`) rather than collapsing into generic admin error
//!    copy.
//!
//! The five required row classes are: `directory_provider_card`,
//! `user_seat_lifecycle`, `provisioning_failure_log`, `policy_target_dry_run`,
//! and `local_governance_path`. Each row carries explicit provisioning class,
//! sync freshness, scope, and redaction state.
//!
//! Surfaces (admin/settings center, support export, shell identity summary,
//! headless inspector) read [`seeded_harden_identity_admin_page`] rather than
//! minting parallel identity-admin checks.
//!
//! **Canonical truth paths:**
//! - Doc: `docs/enterprise/m4/harden-identity-and-admin-support-export-parity-audit.md`
//! - Artifact: `artifacts/enterprise/m4/harden-identity-and-admin-support-export-parity-audit.md`
//! - Contract ref: [`HARDEN_IDENTITY_ADMIN_SHARED_CONTRACT_REF`]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

/// Schema version carried on every record in this module.
pub const HARDEN_IDENTITY_ADMIN_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every record in this module.
pub const HARDEN_IDENTITY_ADMIN_SHARED_CONTRACT_REF: &str =
    "policy:harden_identity_admin_support_export_parity:v1";

/// Record-kind tag for [`HardenIdentityAdminPage`] payloads.
pub const HARDEN_IDENTITY_ADMIN_PAGE_RECORD_KIND: &str = "policy_harden_identity_admin_page_record";

/// Record-kind tag for [`IdentityAdminRow`] payloads.
pub const HARDEN_IDENTITY_ADMIN_ROW_RECORD_KIND: &str = "policy_harden_identity_admin_row_record";

/// Record-kind tag for [`HardenIdentityAdminDefect`] payloads.
pub const HARDEN_IDENTITY_ADMIN_DEFECT_RECORD_KIND: &str =
    "policy_harden_identity_admin_defect_record";

/// Record-kind tag for [`HardenIdentityAdminSummary`] payloads.
pub const HARDEN_IDENTITY_ADMIN_SUMMARY_RECORD_KIND: &str =
    "policy_harden_identity_admin_summary_record";

/// Record-kind tag for [`HardenIdentityAdminSupportExport`] payloads.
pub const HARDEN_IDENTITY_ADMIN_SUPPORT_EXPORT_RECORD_KIND: &str =
    "policy_harden_identity_admin_support_export_record";

/// Repo-relative path of the stable doc for this lane.
pub const HARDEN_IDENTITY_ADMIN_DOC_REF: &str =
    "docs/enterprise/m4/harden-identity-and-admin-support-export-parity-audit.md";

/// Repo-relative path of the artifact summary for this lane.
pub const HARDEN_IDENTITY_ADMIN_ARTIFACT_REF: &str =
    "artifacts/enterprise/m4/harden-identity-and-admin-support-export-parity-audit.md";

// ---------------------------------------------------------------------------
// Row class vocabulary
// ---------------------------------------------------------------------------

/// Identity-admin row class under which a row is inspected.
///
/// These five row classes form the required coverage set for the hardened
/// identity and admin support-export parity proof packet. Each row explains
/// which aspect of identity/admin state is covered, what provisioning class
/// applies, what the sync freshness is, and what remains local versus
/// tenant-scoped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityAdminRowClass {
    /// Directory and provider card naming the identity provider, directory
    /// source, and provisioning class.
    DirectoryProviderCard,
    /// User and seat lifecycle row covering creation, transfer, suspension,
    /// reactivation, and deprovision flows.
    UserSeatLifecycle,
    /// Provisioning failure log that distinguishes failure kinds rather than
    /// collapsing into generic admin error copy.
    ProvisioningFailureLog,
    /// Policy-target dry run result showing what would change before commit.
    PolicyTargetDryRun,
    /// Local governance path row documenting no-vendor-control-plane fallback
    /// and local-artifact safety.
    LocalGovernancePath,
}

impl IdentityAdminRowClass {
    /// All required identity-admin row classes in canonical order.
    pub const ALL: [Self; 5] = [
        Self::DirectoryProviderCard,
        Self::UserSeatLifecycle,
        Self::ProvisioningFailureLog,
        Self::PolicyTargetDryRun,
        Self::LocalGovernancePath,
    ];

    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DirectoryProviderCard => "directory_provider_card",
            Self::UserSeatLifecycle => "user_seat_lifecycle",
            Self::ProvisioningFailureLog => "provisioning_failure_log",
            Self::PolicyTargetDryRun => "policy_target_dry_run",
            Self::LocalGovernancePath => "local_governance_path",
        }
    }

    /// True when this row class requires a provisioning class declaration.
    pub const fn requires_provisioning_class(self) -> bool {
        matches!(
            self,
            Self::DirectoryProviderCard | Self::UserSeatLifecycle | Self::ProvisioningFailureLog
        )
    }

    /// True when this row class requires a sync freshness declaration.
    pub const fn requires_sync_freshness(self) -> bool {
        matches!(
            self,
            Self::DirectoryProviderCard
                | Self::UserSeatLifecycle
                | Self::ProvisioningFailureLog
                | Self::PolicyTargetDryRun
        )
    }

    /// True when this row class requires a local vs tenant scope declaration.
    pub const fn requires_local_tenant_scope(self) -> bool {
        true
    }

    /// True when this row class requires an admin action/result lineage block.
    pub const fn requires_action_lineage(self) -> bool {
        matches!(
            self,
            Self::UserSeatLifecycle
                | Self::ProvisioningFailureLog
                | Self::PolicyTargetDryRun
                | Self::LocalGovernancePath
        )
    }

    /// True when this row class may carry a provisioning failure kind.
    pub const fn may_carry_failure_kind(self) -> bool {
        matches!(
            self,
            Self::ProvisioningFailureLog | Self::PolicyTargetDryRun
        )
    }
}

// ---------------------------------------------------------------------------
// Provisioning class vocabulary
// ---------------------------------------------------------------------------

/// Provisioning class disclosed by an identity-admin row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityAdminProvisioningClass {
    /// OIDC-based identity provisioning and authentication.
    Oidc,
    /// SCIM-based lifecycle provisioning.
    Scim,
    /// Signed file bundle import providing org lifecycle state.
    SignedFileBundle,
    /// Manual admin action or file-based setup.
    Manual,
}

impl IdentityAdminProvisioningClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Oidc => "oidc",
            Self::Scim => "scim",
            Self::SignedFileBundle => "signed_file_bundle",
            Self::Manual => "manual",
        }
    }
}

// ---------------------------------------------------------------------------
// Sync freshness vocabulary
// ---------------------------------------------------------------------------

/// Sync freshness posture for a provisioning or identity source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityAdminSyncFreshnessClass {
    /// Live sync within the accepted freshness window.
    Live,
    /// Cached sync that may be slightly behind live.
    Cached,
    /// Stale sync past the acceptable window.
    Stale,
    /// Expired sync; the last known good is past its validity.
    Expired,
    /// Missing sync; no source has been successfully contacted.
    Missing,
}

impl IdentityAdminSyncFreshnessClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Live => "live",
            Self::Cached => "cached",
            Self::Stale => "stale",
            Self::Expired => "expired",
            Self::Missing => "missing",
        }
    }

    /// True when this freshness posture narrows or blocks managed actions.
    pub const fn fails_closed(self) -> bool {
        matches!(self, Self::Stale | Self::Expired | Self::Missing)
    }
}

// ---------------------------------------------------------------------------
// Provisioning failure kind vocabulary
// ---------------------------------------------------------------------------

/// Distinguished provisioning failure kind.
///
/// Every failure row must name a specific kind rather than collapsing into
/// generic admin error copy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProvisioningFailureKind {
    /// The identity provider or provisioning endpoint is unreachable or
    /// returning errors.
    ProviderOutage,
    /// The local auth state has drifted from the provider's expected state
    /// (e.g., token expiry, session mismatch).
    AuthDrift,
    /// The requested action exceeds the granted scope or entitlement.
    ScopeMismatch,
    /// A seat was removed, transferred, or otherwise lost.
    SeatLoss,
    /// The failure occurred during a deprovisioning or offboarding flow.
    DeprovisioningImpact,
}

impl ProvisioningFailureKind {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProviderOutage => "provider_outage",
            Self::AuthDrift => "auth_drift",
            Self::ScopeMismatch => "scope_mismatch",
            Self::SeatLoss => "seat_loss",
            Self::DeprovisioningImpact => "deprovisioning_impact",
        }
    }
}

// ---------------------------------------------------------------------------
// Local vs tenant scope vocabulary
// ---------------------------------------------------------------------------

/// Scope classification for what remains local versus tenant-scoped on a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalTenantScopeClass {
    /// The row covers only local device state; no tenant-scoped data is
    /// involved.
    LocalStateOnly,
    /// The row covers only tenant-scoped state; local artifacts are not
    /// affected.
    TenantScoped,
    /// The row covers both local and tenant state; the exact boundary is
    /// explained in the row's `local_artifact_safety_note`.
    HybridLocalTenant,
}

impl LocalTenantScopeClass {
    /// Stable token recorded on rows.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalStateOnly => "local_state_only",
            Self::TenantScoped => "tenant_scoped",
            Self::HybridLocalTenant => "hybrid_local_tenant",
        }
    }
}

// ---------------------------------------------------------------------------
// Qualification and narrow-reason vocabulary
// ---------------------------------------------------------------------------

/// Qualification tier for the hardened identity-admin page.
///
/// The tier is derived, not asserted: it is set by the audit. A caller may
/// never assert `stable` without a clean audit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenIdentityAdminQualificationClass {
    /// All required conditions hold.
    Stable,
    /// One or more non-critical conditions are unmet.
    Beta,
    /// A required row class has no row; coverage gap prevents any claim.
    Preview,
    /// A hard guardrail was triggered; the page is withdrawn immediately.
    Withdrawn,
}

impl HardenIdentityAdminQualificationClass {
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
}

/// Typed reason a packet or row was narrowed below
/// [`HardenIdentityAdminQualificationClass::Stable`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HardenIdentityAdminNarrowReasonClass {
    /// No narrowing — the packet qualifies stable.
    NotNarrowed,
    /// A required row class has no row; narrows to preview.
    MissingRowClassCoverage,
    /// A row does not declare a provisioning class where one is required.
    MissingProvisioningClass,
    /// A row does not declare a sync freshness token.
    MissingSyncFreshness,
    /// A row does not declare a local vs tenant scope.
    MissingLocalTenantScope,
    /// A provisioning failure or dry-run row uses a generic failure kind
    /// rather than a specific [`ProvisioningFailureKind`].
    GenericFailureKindUsed,
    /// A local governance path row does not carry explicit local-core
    /// continuity.
    LocalCoreContinuityNotExplicit,
    /// A directory/provider card row does not name a fallback or manual path.
    MissingFallbackManualPath,
    /// Raw secret or private material was exposed in a row; withdraws the
    /// packet immediately.
    RawSecretOrPrivateMaterialExposed,
    /// A row does not carry an admin action/result lineage block where one
    /// is required.
    MissingActionLineage,
}

impl HardenIdentityAdminNarrowReasonClass {
    /// Stable token recorded on every serialized record.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotNarrowed => "not_narrowed",
            Self::MissingRowClassCoverage => "missing_row_class_coverage",
            Self::MissingProvisioningClass => "missing_provisioning_class",
            Self::MissingSyncFreshness => "missing_sync_freshness",
            Self::MissingLocalTenantScope => "missing_local_tenant_scope",
            Self::GenericFailureKindUsed => "generic_failure_kind_used",
            Self::LocalCoreContinuityNotExplicit => "local_core_continuity_not_explicit",
            Self::MissingFallbackManualPath => "missing_fallback_manual_path",
            Self::RawSecretOrPrivateMaterialExposed => "raw_secret_or_private_material_exposed",
            Self::MissingActionLineage => "missing_action_lineage",
        }
    }

    /// True when this reason triggers immediate withdrawal and cannot be
    /// overridden.
    pub const fn is_withdrawal_reason(self) -> bool {
        matches!(self, Self::RawSecretOrPrivateMaterialExposed)
    }
}

// ---------------------------------------------------------------------------
// Admin action/result lineage
// ---------------------------------------------------------------------------

/// Admin action and result lineage block attached to rows that require it.
///
/// This block names the action, result, actor, and timestamp without exposing
/// raw secrets, raw tenant data, raw user emails, or raw credential material.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AdminActionLineage {
    /// Action token (e.g., `provision`, `suspend`, `transfer_seat`,
    /// `deprovision`, `dry_run`, `apply_policy`).
    pub action_token: String,
    /// Human-readable action label.
    pub action_label: String,
    /// Result token (e.g., `succeeded`, `failed`, `narrowed`, `blocked`).
    pub result_token: String,
    /// Human-readable result label.
    pub result_label: String,
    /// Opaque actor ref (admin id, service principal, or policy actor).
    pub actor_ref: String,
    /// Timestamp when the action was applied.
    pub applied_at: String,
    /// Opaque transaction id tying the action to audit logs.
    pub transaction_id: String,
}

// ---------------------------------------------------------------------------
// Identity-admin row
// ---------------------------------------------------------------------------

/// One identity-admin row covering a directory/provider card, user/seat
/// lifecycle event, provisioning failure, policy-target dry run, or local
/// governance path.
///
/// Each row proves:
/// - which aspect of identity/admin state is covered (row class);
/// - which provisioning class applies (OIDC, SCIM, signed file bundle, manual);
/// - what the sync freshness is;
/// - what fallback or manual path exists;
/// - what remains local versus tenant-scoped;
/// - whether raw secret material is excluded.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentityAdminRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable row id.
    pub row_id: String,
    /// Row class.
    pub row_class: IdentityAdminRowClass,
    /// Stable token for [`Self::row_class`].
    pub row_class_token: String,
    /// Provisioning class.
    pub provisioning_class: IdentityAdminProvisioningClass,
    /// Stable token for [`Self::provisioning_class`].
    pub provisioning_class_token: String,
    /// Sync freshness posture.
    pub sync_freshness: IdentityAdminSyncFreshnessClass,
    /// Stable token for [`Self::sync_freshness`].
    pub sync_freshness_token: String,
    /// Fallback or manual path label.
    pub fallback_manual_path_label: String,
    /// Local-artifact safety note explaining what remains local.
    pub local_artifact_safety_note: String,
    /// Local vs tenant scope classification.
    pub local_tenant_scope: LocalTenantScopeClass,
    /// Stable token for [`Self::local_tenant_scope`].
    pub local_tenant_scope_token: String,
    /// Admin action/result lineage block.
    pub action_lineage: Option<AdminActionLineage>,
    /// Specific provisioning failure kind, when this row is a failure log or
    /// dry run.
    pub failure_kind: Option<ProvisioningFailureKind>,
    /// Stable token for [`Self::failure_kind`], or empty when none.
    pub failure_kind_token: String,
    /// True when local-core continuity is stated explicitly on this row.
    pub local_core_continuity_explicit: bool,
    /// True when raw secret or private-key material is excluded from the record.
    pub raw_secret_or_private_material_excluded: bool,
    /// Derived qualification tier for this row.
    pub qualification_token: String,
    /// Why the row was narrowed (or `not_narrowed`).
    pub narrow_reason_token: String,
    /// Plain-language summary of the qualification for this row.
    pub plain_language_summary: String,
}

// ---------------------------------------------------------------------------
// Summary
// ---------------------------------------------------------------------------

/// Aggregate summary for the hardened identity-admin page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct HardenIdentityAdminSummary {
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
    /// Row class tokens present on the page.
    pub row_classes_covered: Vec<String>,
    /// Provisioning class tokens present across rows.
    pub provisioning_classes_present: Vec<String>,
    /// Sync freshness tokens present across rows.
    pub sync_freshness_present: Vec<String>,
    /// Local-tenant scope tokens present across rows.
    pub local_tenant_scopes_present: Vec<String>,
    /// Failure kind tokens present across rows.
    pub failure_kinds_present: Vec<String>,
    /// Number of rows with `local_core_continuity_explicit: true`.
    pub local_core_continuity_explicit_row_count: usize,
    /// Number of rows with `raw_secret_or_private_material_excluded: true`.
    pub raw_secret_excluded_row_count: usize,
    /// Number of rows carrying an admin action/result lineage block.
    pub action_lineage_row_count: usize,
    /// Overall qualification token.
    pub overall_qualification_token: String,
}

impl HardenIdentityAdminSummary {
    fn from_rows(rows: &[IdentityAdminRow]) -> Self {
        let mut stable = 0usize;
        let mut beta = 0usize;
        let mut preview = 0usize;
        let mut withdrawn = 0usize;
        let mut row_classes: BTreeSet<String> = BTreeSet::new();
        let mut provisioning_classes: BTreeSet<String> = BTreeSet::new();
        let mut freshness_states: BTreeSet<String> = BTreeSet::new();
        let mut scopes: BTreeSet<String> = BTreeSet::new();
        let mut failure_kinds: BTreeSet<String> = BTreeSet::new();
        let mut local_core_ok = 0usize;
        let mut raw_secret_ok = 0usize;
        let mut action_lineage_ok = 0usize;

        for row in rows {
            match row.qualification_token.as_str() {
                "stable" => stable += 1,
                "beta" => beta += 1,
                "preview" => preview += 1,
                "withdrawn" => withdrawn += 1,
                _ => {}
            }
            row_classes.insert(row.row_class_token.clone());
            provisioning_classes.insert(row.provisioning_class_token.clone());
            freshness_states.insert(row.sync_freshness_token.clone());
            scopes.insert(row.local_tenant_scope_token.clone());
            if !row.failure_kind_token.is_empty() {
                failure_kinds.insert(row.failure_kind_token.clone());
            }
            if row.local_core_continuity_explicit {
                local_core_ok += 1;
            }
            if row.raw_secret_or_private_material_excluded {
                raw_secret_ok += 1;
            }
            if row.action_lineage.is_some() {
                action_lineage_ok += 1;
            }
        }

        let overall = if withdrawn > 0 {
            HardenIdentityAdminQualificationClass::Withdrawn
        } else if preview > 0 {
            HardenIdentityAdminQualificationClass::Preview
        } else if beta > 0 {
            HardenIdentityAdminQualificationClass::Beta
        } else {
            HardenIdentityAdminQualificationClass::Stable
        };

        Self {
            row_count: rows.len(),
            stable_row_count: stable,
            beta_row_count: beta,
            preview_row_count: preview,
            withdrawn_row_count: withdrawn,
            row_classes_covered: row_classes.into_iter().collect(),
            provisioning_classes_present: provisioning_classes.into_iter().collect(),
            sync_freshness_present: freshness_states.into_iter().collect(),
            local_tenant_scopes_present: scopes.into_iter().collect(),
            failure_kinds_present: failure_kinds.into_iter().collect(),
            local_core_continuity_explicit_row_count: local_core_ok,
            raw_secret_excluded_row_count: raw_secret_ok,
            action_lineage_row_count: action_lineage_ok,
            overall_qualification_token: overall.as_str().to_owned(),
        }
    }
}

// ---------------------------------------------------------------------------
// Defect
// ---------------------------------------------------------------------------

/// Typed defect emitted by the hardened identity-admin audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenIdentityAdminDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable defect id.
    pub defect_id: String,
    /// Narrow reason for this defect.
    pub narrow_reason: HardenIdentityAdminNarrowReasonClass,
    /// Stable token for [`Self::narrow_reason`].
    pub narrow_reason_token: String,
    /// Subject id (row id, row class token, or `page`).
    pub source: String,
    /// Export-safe explanation.
    pub note: String,
}

impl HardenIdentityAdminDefect {
    fn new(
        narrow_reason: HardenIdentityAdminNarrowReasonClass,
        source: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        let source_str = source.into();
        Self {
            record_kind: HARDEN_IDENTITY_ADMIN_DEFECT_RECORD_KIND.to_owned(),
            schema_version: HARDEN_IDENTITY_ADMIN_SCHEMA_VERSION,
            shared_contract_ref: HARDEN_IDENTITY_ADMIN_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "policy:defect:harden-identity-admin:{}:{}",
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

/// Stable proof packet for hardened identity and admin support-export parity,
/// audit-safe redaction, and no-vendor-control-plane local governance paths.
///
/// This is the single inspectable record that proves all required identity-
/// admin row classes are covered with explicit provisioning class, sync
/// freshness, scope, fallback path, and local-artifact safety declarations.
/// Dashboards, docs, Help/About surfaces, and support exports should ingest it
/// rather than cloning status text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenIdentityAdminPage {
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
    pub summary: HardenIdentityAdminSummary,
    /// Per-class qualification rows (one per required row class).
    pub rows: Vec<IdentityAdminRow>,
    /// Typed validation defects.
    pub defects: Vec<HardenIdentityAdminDefect>,
}

impl HardenIdentityAdminPage {
    /// Build the page from a set of rows.
    ///
    /// Defects are derived automatically from the audit.
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<IdentityAdminRow>,
    ) -> Self {
        let defects = audit_identity_admin_rows(&rows);
        let qualified_rows = qualify_rows(rows, &defects);
        let summary = HardenIdentityAdminSummary::from_rows(&qualified_rows);
        Self {
            record_kind: HARDEN_IDENTITY_ADMIN_PAGE_RECORD_KIND.to_owned(),
            schema_version: HARDEN_IDENTITY_ADMIN_SCHEMA_VERSION,
            shared_contract_ref: HARDEN_IDENTITY_ADMIN_SHARED_CONTRACT_REF.to_owned(),
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
            == HardenIdentityAdminQualificationClass::Stable.as_str()
    }

    /// True when all five required row classes are covered.
    pub fn covers_all_required_row_classes(&self) -> bool {
        let covered: BTreeSet<&str> = self
            .rows
            .iter()
            .map(|r| r.row_class_token.as_str())
            .collect();
        IdentityAdminRowClass::ALL
            .iter()
            .all(|r| covered.contains(r.as_str()))
    }

    /// True when every row excludes raw secret or private-key material.
    pub fn all_rows_exclude_raw_secret_material(&self) -> bool {
        self.rows
            .iter()
            .all(|r| r.raw_secret_or_private_material_excluded)
    }

    /// True when all rows that require a provisioning class carry one.
    pub fn all_required_provisioning_classes_declared(&self) -> bool {
        self.rows.iter().all(|r| {
            if r.row_class.requires_provisioning_class() {
                !r.provisioning_class_token.is_empty()
            } else {
                true
            }
        })
    }

    /// True when all rows carry explicit sync freshness tokens.
    pub fn all_rows_have_sync_freshness(&self) -> bool {
        self.rows.iter().all(|r| !r.sync_freshness_token.is_empty())
    }

    /// True when all rows carry explicit local vs tenant scope.
    pub fn all_rows_have_local_tenant_scope(&self) -> bool {
        self.rows
            .iter()
            .all(|r| !r.local_tenant_scope_token.is_empty())
    }
}

// ---------------------------------------------------------------------------
// Support export
// ---------------------------------------------------------------------------

/// Support-export wrapper for the hardened identity-admin page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HardenIdentityAdminSupportExport {
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
    /// The hardened identity-admin page embedded as evidence.
    pub page: HardenIdentityAdminPage,
    /// Narrow-reason tokens present in the page's defect list.
    pub narrow_reasons_present: Vec<HardenIdentityAdminNarrowReasonClass>,
    /// Defect counts by narrow-reason token.
    pub defect_counts_by_narrow_reason: BTreeMap<String, usize>,
    /// True when raw secret or private-key material is excluded from the export.
    pub raw_secret_or_private_material_excluded: bool,
}

impl HardenIdentityAdminSupportExport {
    /// Wrap a page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: HardenIdentityAdminPage,
    ) -> Self {
        let mut reasons: Vec<HardenIdentityAdminNarrowReasonClass> = Vec::new();
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
            record_kind: HARDEN_IDENTITY_ADMIN_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: HARDEN_IDENTITY_ADMIN_SCHEMA_VERSION,
            shared_contract_ref: HARDEN_IDENTITY_ADMIN_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            narrow_reasons_present: reasons,
            defect_counts_by_narrow_reason: counts,
            raw_secret_or_private_material_excluded: true,
        }
    }
}

// ---------------------------------------------------------------------------
// Public audit and validate functions
// ---------------------------------------------------------------------------

/// Re-run the identity-admin audit over the rows.
pub fn audit_harden_identity_admin_page(
    page: &HardenIdentityAdminPage,
) -> Vec<HardenIdentityAdminDefect> {
    audit_identity_admin_rows(&page.rows)
}

/// Validate the page; returns `Ok` when the audit is clean.
pub fn validate_harden_identity_admin_page(
    page: &HardenIdentityAdminPage,
) -> Result<(), Vec<HardenIdentityAdminDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

/// Build the seeded hardened identity-admin page covering all five required
/// row classes with explicit provisioning class, sync freshness, scope,
/// fallback path, and local-artifact safety declarations.
pub fn seeded_harden_identity_admin_page() -> HardenIdentityAdminPage {
    let rows = seeded_rows();
    HardenIdentityAdminPage::new(
        "policy:harden-identity-admin:seeded:0001",
        "Hardened identity and admin support-export parity, audit-safe redaction, and no-vendor-control-plane local governance paths",
        "2026-06-01T00:00:00Z",
        rows,
    )
}

// ---------------------------------------------------------------------------
// Internal audit helpers
// ---------------------------------------------------------------------------

fn audit_identity_admin_rows(rows: &[IdentityAdminRow]) -> Vec<HardenIdentityAdminDefect> {
    let mut defects: Vec<HardenIdentityAdminDefect> = Vec::new();

    // Hard guardrail: raw secret or private key material in any row.
    for row in rows {
        if !row.raw_secret_or_private_material_excluded {
            defects.push(HardenIdentityAdminDefect::new(
                HardenIdentityAdminNarrowReasonClass::RawSecretOrPrivateMaterialExposed,
                row.row_id.clone(),
                "row does not exclude raw secret or private-key material",
            ));
        }
    }

    // If withdrawal triggered, return immediately with only the raw-material
    // defects (other checks are irrelevant after withdrawal).
    let has_withdrawal = defects.iter().any(|d| {
        d.narrow_reason == HardenIdentityAdminNarrowReasonClass::RawSecretOrPrivateMaterialExposed
    });
    if has_withdrawal {
        return defects;
    }

    for row in rows {
        // Rows requiring a provisioning class must carry one.
        if row.row_class.requires_provisioning_class() && row.provisioning_class_token.is_empty() {
            defects.push(HardenIdentityAdminDefect::new(
                HardenIdentityAdminNarrowReasonClass::MissingProvisioningClass,
                row.row_id.clone(),
                "row does not declare a provisioning class where one is required",
            ));
        }

        // Rows requiring sync freshness must carry one.
        if row.row_class.requires_sync_freshness() && row.sync_freshness_token.is_empty() {
            defects.push(HardenIdentityAdminDefect::new(
                HardenIdentityAdminNarrowReasonClass::MissingSyncFreshness,
                row.row_id.clone(),
                "row does not declare a sync freshness token",
            ));
        }

        // All rows must declare local vs tenant scope.
        if row.row_class.requires_local_tenant_scope() && row.local_tenant_scope_token.is_empty() {
            defects.push(HardenIdentityAdminDefect::new(
                HardenIdentityAdminNarrowReasonClass::MissingLocalTenantScope,
                row.row_id.clone(),
                "row does not declare a local vs tenant scope",
            ));
        }

        // Rows that may carry a failure kind must name a specific one when
        // `failure_kind_token` is non-empty, and must not be generic.
        if row.row_class.may_carry_failure_kind() {
            if row.failure_kind_token.is_empty() {
                // It's acceptable for dry-run rows to have no failure kind when
                // the dry run succeeded. We only flag when the row looks like a
                // failure but uses a generic token.
            } else {
                // Verify the token matches a known ProvisioningFailureKind.
                let known = [
                    ProvisioningFailureKind::ProviderOutage,
                    ProvisioningFailureKind::AuthDrift,
                    ProvisioningFailureKind::ScopeMismatch,
                    ProvisioningFailureKind::SeatLoss,
                    ProvisioningFailureKind::DeprovisioningImpact,
                ];
                let is_known = known.iter().any(|k| k.as_str() == row.failure_kind_token);
                if !is_known {
                    defects.push(HardenIdentityAdminDefect::new(
                        HardenIdentityAdminNarrowReasonClass::GenericFailureKindUsed,
                        row.row_id.clone(),
                        "row uses a generic or unknown failure kind token instead of a specific ProvisioningFailureKind",
                    ));
                }
            }
        }

        // Local governance path rows must carry explicit local-core continuity.
        if row.row_class == IdentityAdminRowClass::LocalGovernancePath
            && !row.local_core_continuity_explicit
        {
            defects.push(HardenIdentityAdminDefect::new(
                HardenIdentityAdminNarrowReasonClass::LocalCoreContinuityNotExplicit,
                row.row_id.clone(),
                "local_governance_path row must carry local_core_continuity_explicit: true",
            ));
        }

        // Directory/provider card rows must name a fallback/manual path.
        if row.row_class == IdentityAdminRowClass::DirectoryProviderCard
            && row.fallback_manual_path_label.is_empty()
        {
            defects.push(HardenIdentityAdminDefect::new(
                HardenIdentityAdminNarrowReasonClass::MissingFallbackManualPath,
                row.row_id.clone(),
                "directory_provider_card row must name a fallback or manual path",
            ));
        }

        // Rows requiring action lineage must carry one.
        if row.row_class.requires_action_lineage() && row.action_lineage.is_none() {
            defects.push(HardenIdentityAdminDefect::new(
                HardenIdentityAdminNarrowReasonClass::MissingActionLineage,
                row.row_id.clone(),
                "row does not carry an admin action/result lineage block where one is required",
            ));
        }
    }

    // Coverage check: all five required row classes must appear at least once.
    let required_classes: BTreeSet<&str> = IdentityAdminRowClass::ALL
        .iter()
        .map(|r| r.as_str())
        .collect();
    let observed_classes: BTreeSet<&str> =
        rows.iter().map(|r| r.row_class_token.as_str()).collect();
    for missing in required_classes.difference(&observed_classes) {
        defects.push(HardenIdentityAdminDefect::new(
            HardenIdentityAdminNarrowReasonClass::MissingRowClassCoverage,
            "page",
            format!(
                "missing row for required identity-admin row class '{missing}'; packet narrowed to preview"
            ),
        ));
    }

    defects
}

fn qualify_rows(
    mut rows: Vec<IdentityAdminRow>,
    page_defects: &[HardenIdentityAdminDefect],
) -> Vec<IdentityAdminRow> {
    let has_withdrawal = page_defects
        .iter()
        .any(|d| d.narrow_reason.is_withdrawal_reason());
    let has_preview = page_defects
        .iter()
        .any(|d| d.narrow_reason == HardenIdentityAdminNarrowReasonClass::MissingRowClassCoverage);

    let (overall_qual, overall_reason) = if has_withdrawal {
        let r = page_defects
            .iter()
            .find(|d| d.narrow_reason.is_withdrawal_reason())
            .map(|d| d.narrow_reason)
            .unwrap_or(HardenIdentityAdminNarrowReasonClass::RawSecretOrPrivateMaterialExposed);
        (HardenIdentityAdminQualificationClass::Withdrawn, r)
    } else if has_preview {
        (
            HardenIdentityAdminQualificationClass::Preview,
            HardenIdentityAdminNarrowReasonClass::MissingRowClassCoverage,
        )
    } else if !page_defects.is_empty() {
        let r = page_defects[0].narrow_reason;
        (HardenIdentityAdminQualificationClass::Beta, r)
    } else {
        (
            HardenIdentityAdminQualificationClass::Stable,
            HardenIdentityAdminNarrowReasonClass::NotNarrowed,
        )
    };

    for row in &mut rows {
        let row_qual = if has_withdrawal {
            HardenIdentityAdminQualificationClass::Withdrawn
        } else if has_preview {
            HardenIdentityAdminQualificationClass::Preview
        } else {
            let row_has_defect = page_defects.iter().any(|d| d.source == row.row_id);
            if row_has_defect {
                HardenIdentityAdminQualificationClass::Beta
            } else {
                HardenIdentityAdminQualificationClass::Stable
            }
        };

        let row_reason = if row_qual == overall_qual {
            overall_reason
        } else {
            page_defects
                .iter()
                .find(|d| d.source == row.row_id)
                .map(|d| d.narrow_reason)
                .unwrap_or(HardenIdentityAdminNarrowReasonClass::NotNarrowed)
        };

        row.qualification_token = row_qual.as_str().to_owned();
        row.narrow_reason_token = row_reason.as_str().to_owned();
        row.plain_language_summary =
            build_row_summary(&row.row_id, &row.row_class_token, row_qual, row_reason);
    }

    rows
}

fn build_row_summary(
    row_id: &str,
    row_class_token: &str,
    qual: HardenIdentityAdminQualificationClass,
    narrow_reason: HardenIdentityAdminNarrowReasonClass,
) -> String {
    match qual {
        HardenIdentityAdminQualificationClass::Stable => format!(
            "Row '{row_id}' (class: {row_class_token}) qualifies stable: \
             provisioning class declared, sync freshness explicit, local vs tenant scope named, \
             action lineage present, raw secret material excluded."
        ),
        HardenIdentityAdminQualificationClass::Beta => format!(
            "Row '{row_id}' (class: {row_class_token}) narrowed to beta \
             (reason: {}): one or more required conditions are unmet.",
            narrow_reason.as_str()
        ),
        HardenIdentityAdminQualificationClass::Preview => format!(
            "Row '{row_id}' (class: {row_class_token}) narrowed to preview: \
             a required identity-admin row class is missing from the page."
        ),
        HardenIdentityAdminQualificationClass::Withdrawn => format!(
            "Row '{row_id}' (class: {row_class_token}) is withdrawn \
             (reason: {}): hard guardrail triggered.",
            narrow_reason.as_str()
        ),
    }
}

// ---------------------------------------------------------------------------
// Seeded rows
// ---------------------------------------------------------------------------

fn seeded_rows() -> Vec<IdentityAdminRow> {
    vec![
        row_directory_provider_card(),
        row_user_seat_lifecycle(),
        row_provisioning_failure_log(),
        row_policy_target_dry_run(),
        row_local_governance_path(),
    ]
}

fn make_row(
    row_id: &str,
    row_class: IdentityAdminRowClass,
    provisioning_class: IdentityAdminProvisioningClass,
    sync_freshness: IdentityAdminSyncFreshnessClass,
    fallback_manual_path_label: &str,
    local_artifact_safety_note: &str,
    local_tenant_scope: LocalTenantScopeClass,
    action_lineage: Option<AdminActionLineage>,
    failure_kind: Option<ProvisioningFailureKind>,
    local_core_continuity_explicit: bool,
) -> IdentityAdminRow {
    let failure_kind_token = failure_kind
        .map(|k| k.as_str().to_owned())
        .unwrap_or_default();
    IdentityAdminRow {
        record_kind: HARDEN_IDENTITY_ADMIN_ROW_RECORD_KIND.to_owned(),
        schema_version: HARDEN_IDENTITY_ADMIN_SCHEMA_VERSION,
        shared_contract_ref: HARDEN_IDENTITY_ADMIN_SHARED_CONTRACT_REF.to_owned(),
        row_id: row_id.to_owned(),
        row_class,
        row_class_token: row_class.as_str().to_owned(),
        provisioning_class,
        provisioning_class_token: provisioning_class.as_str().to_owned(),
        sync_freshness,
        sync_freshness_token: sync_freshness.as_str().to_owned(),
        fallback_manual_path_label: fallback_manual_path_label.to_owned(),
        local_artifact_safety_note: local_artifact_safety_note.to_owned(),
        local_tenant_scope,
        local_tenant_scope_token: local_tenant_scope.as_str().to_owned(),
        action_lineage,
        failure_kind,
        failure_kind_token,
        local_core_continuity_explicit,
        raw_secret_or_private_material_excluded: true,
        // Filled in by qualify_rows.
        qualification_token: HardenIdentityAdminQualificationClass::Stable
            .as_str()
            .to_owned(),
        narrow_reason_token: HardenIdentityAdminNarrowReasonClass::NotNarrowed
            .as_str()
            .to_owned(),
        plain_language_summary: String::new(),
    }
}

fn lineage(
    action_token: &str,
    action_label: &str,
    result_token: &str,
    result_label: &str,
    actor_ref: &str,
    applied_at: &str,
    transaction_id: &str,
) -> AdminActionLineage {
    AdminActionLineage {
        action_token: action_token.to_owned(),
        action_label: action_label.to_owned(),
        result_token: result_token.to_owned(),
        result_label: result_label.to_owned(),
        actor_ref: actor_ref.to_owned(),
        applied_at: applied_at.to_owned(),
        transaction_id: transaction_id.to_owned(),
    }
}

fn row_directory_provider_card() -> IdentityAdminRow {
    make_row(
        "harden-identity-admin:directory_provider_card",
        IdentityAdminRowClass::DirectoryProviderCard,
        IdentityAdminProvisioningClass::Oidc,
        IdentityAdminSyncFreshnessClass::Live,
        "Manual local account creation via Settings > Identity; SCIM fallback via signed file import when OIDC issuer is unreachable.",
        "Provider metadata (issuer URL, tenant id) is tenant-scoped and cached locally as opaque refs. Local editing history and settings remain on the device regardless of provider state.",
        LocalTenantScopeClass::HybridLocalTenant,
        Some(lineage(
            "configure_identity_provider",
            "Configure identity provider",
            "succeeded",
            "OIDC issuer registered and tenant binding established",
            "admin:local:0001",
            "2026-05-15T08:00:00Z",
            "txn:identity:configure:001",
        )),
        None,
        true,
    )
}

fn row_user_seat_lifecycle() -> IdentityAdminRow {
    make_row(
        "harden-identity-admin:user_seat_lifecycle",
        IdentityAdminRowClass::UserSeatLifecycle,
        IdentityAdminProvisioningClass::Scim,
        IdentityAdminSyncFreshnessClass::Cached,
        "Manual user creation via admin console; seat transfer requires admin-signed request file when SCIM endpoint is stale.",
        "User directory state is tenant-scoped and synced from the SCIM source. Local project files, history, and personal settings remain local-only and are never modified by seat changes.",
        LocalTenantScopeClass::HybridLocalTenant,
        Some(lineage(
            "seat_transfer",
            "Transfer seat from user:0099 to user:0100",
            "succeeded",
            "Seat transferred; local artifacts of user:0099 preserved on device",
            "admin:scim:managed",
            "2026-05-15T09:00:00Z",
            "txn:seat:transfer:042",
        )),
        None,
        true,
    )
}

fn row_provisioning_failure_log() -> IdentityAdminRow {
    make_row(
        "harden-identity-admin:provisioning_failure_log",
        IdentityAdminRowClass::ProvisioningFailureLog,
        IdentityAdminProvisioningClass::Scim,
        IdentityAdminSyncFreshnessClass::Stale,
        "Fallback to last-known-good signed provisioning snapshot; manual admin review required before re-enabling live SCIM sync.",
        "Failure is tenant-scoped (SCIM endpoint unreachable). Local editing remains fully available because local capabilities do not depend on the provisioning source.",
        LocalTenantScopeClass::TenantScoped,
        Some(lineage(
            "sync_provisioning",
            "Attempt live SCIM provisioning sync",
            "failed",
            "Provider outage detected; fallback to cached snapshot activated",
            "service:scim:sync",
            "2026-05-15T10:00:00Z",
            "txn:provision:sync:099",
        )),
        Some(ProvisioningFailureKind::ProviderOutage),
        true,
    )
}

fn row_policy_target_dry_run() -> IdentityAdminRow {
    make_row(
        "harden-identity-admin:policy_target_dry_run",
        IdentityAdminRowClass::PolicyTargetDryRun,
        IdentityAdminProvisioningClass::SignedFileBundle,
        IdentityAdminSyncFreshnessClass::Live,
        "Manual policy bundle import via admin console when automatic distribution is unavailable.",
        "Dry-run result is computed locally from the imported bundle. The policy bundle itself is tenant-scoped, but the dry-run engine runs locally and does not send local file contents to the tenant.",
        LocalTenantScopeClass::HybridLocalTenant,
        Some(lineage(
            "policy_dry_run",
            "Evaluate policy bundle against current workspace",
            "narrowed",
            "Two targets would be blocked; no local-only targets affected",
            "admin:local:0001",
            "2026-05-15T11:00:00Z",
            "txn:policy:dry_run:015",
        )),
        None,
        true,
    )
}

fn row_local_governance_path() -> IdentityAdminRow {
    make_row(
        "harden-identity-admin:local_governance_path",
        IdentityAdminRowClass::LocalGovernancePath,
        IdentityAdminProvisioningClass::Manual,
        IdentityAdminSyncFreshnessClass::Live,
        "No external dependency required; local governance is enforced by the local policy engine using signed bundles or built-in defaults.",
        "All governance state for local-only mode is local_state_only. Tenant-scoped policy may augment but cannot override local-core continuity. Local artifacts are never subject to remote deletion or deprovisioning.",
        LocalTenantScopeClass::LocalStateOnly,
        Some(lineage(
            "apply_local_governance",
            "Apply local governance defaults",
            "succeeded",
            "Local-core continuity enforced; no tenant connection required",
            "system:local_governance",
            "2026-05-15T12:00:00Z",
            "txn:governance:local:001",
        )),
        None,
        true,
    )
}
