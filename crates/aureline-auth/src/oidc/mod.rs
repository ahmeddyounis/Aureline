//! Beta OIDC system-browser sign-in, recovery, and session-continuity audit.
//!
//! This module promotes the alpha [`crate::system_browser::ClaimedIdentityRow`]
//! and the existing system-browser return-paths beta into an enterprise-OIDC
//! page-level projection. Every audited row proves, on claimed enterprise rows,
//! that:
//!
//! 1. **Enterprise issuer source is disclosed.** Every claimed-enterprise row
//!    quotes a closed [`OidcIssuerSourceClass`] token plus the issuer label,
//!    issuer domain label, and JWKS source label. Rows MUST NOT silently fall
//!    back to a public issuer endpoint; the `public_issuer_fallback_used`
//!    flag flips a typed defect.
//! 2. **Tenant and workspace binding are visible.** Every claimed-enterprise
//!    row quotes a closed [`OidcTenantBindingClass`] token and exposes the
//!    bound tenant ref, workspace ref, and actor subject ref so the row is
//!    inspectable from the support export.
//! 3. **Return-path semantics are explicit.** Every claimed-enterprise row
//!    quotes a workspace label, target label, requested-action label,
//!    [`crate::browser_callback::ReturnModeClass`] token, origin-validation
//!    token, tenant/workspace match-rule token, and a stable return-anchor
//!    ref. Local rows are exempt from the workspace / anchor requirement.
//! 4. **Session continuity and sign-out preserve local editing.** Rows whose
//!    [`OidcSessionStateClass`] is `signed_out_local_intact`,
//!    `refresh_pending_managed_narrowed`, `refresh_expired_managed_blocked`,
//!    or `identity_outage_managed_blocked` MUST also quote a
//!    [`OidcSignOutContinuityClass`] that preserves local editing while
//!    narrowing managed actions. Rows that claim signed-in active scope
//!    MUST NOT narrow local editing.
//! 5. **Identity outages and denial degrade truthfully.** Rows in an outage
//!    or denial state MUST narrow managed actions, MUST NOT widen authority
//!    beyond the requested scope, and MUST quote a closed
//!    [`OidcRecoveryActionClass`] token that names the user-visible recovery
//!    path.
//! 6. **Support-export vocabulary parity.** The support row reuses the same
//!    closed-vocabulary tokens the live row paints (issuer source, tenant
//!    binding, return mode, session state, outage class, sign-out continuity,
//!    recovery action). Drift is a contract bug.
//!
//! The seeded page seeds zero defects. The validator and the headless
//! inspector are what surface a regression when a row drops a required field,
//! silently falls back to a public endpoint, widens authority on outage,
//! loses local editing on sign-out, or drifts vocabulary across the live row
//! and the support row.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::browser_callback::{
    ReturnModeClass, ReturnOriginValidationClass, ReturnTenantOrWorkspaceMatchRule,
};

/// Beta schema version exported with every record.
pub const OIDC_SYSTEM_BROWSER_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every record on the page.
pub const OIDC_SYSTEM_BROWSER_BETA_SHARED_CONTRACT_REF: &str = "auth:oidc_system_browser_beta:v1";

/// Record kind for [`OidcSystemBrowserBetaPage`] payloads.
pub const OIDC_SYSTEM_BROWSER_BETA_PAGE_RECORD_KIND: &str =
    "auth_oidc_system_browser_beta_page_record";

/// Record kind for [`OidcSystemBrowserBetaRow`] payloads.
pub const OIDC_SYSTEM_BROWSER_BETA_ROW_RECORD_KIND: &str =
    "auth_oidc_system_browser_beta_row_record";

/// Record kind for [`OidcSystemBrowserBetaSupportRow`] payloads.
pub const OIDC_SYSTEM_BROWSER_BETA_SUPPORT_ROW_RECORD_KIND: &str =
    "auth_oidc_system_browser_beta_support_row_record";

/// Record kind for [`OidcSystemBrowserBetaDefect`] payloads.
pub const OIDC_SYSTEM_BROWSER_BETA_DEFECT_RECORD_KIND: &str =
    "auth_oidc_system_browser_beta_defect_record";

/// Record kind for [`OidcSystemBrowserBetaSupportExport`] payloads.
pub const OIDC_SYSTEM_BROWSER_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "auth_oidc_system_browser_beta_support_export_record";

/// Record kind for [`OidcSystemBrowserBetaSummary`] payloads.
pub const OIDC_SYSTEM_BROWSER_BETA_SUMMARY_RECORD_KIND: &str =
    "auth_oidc_system_browser_beta_summary_record";

/// Profile under which a row is inspected. Mirrors the policy-pack beta
/// profile vocabulary so admin and support surfaces use one token set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OidcSystemBrowserBetaProfileClass {
    /// Connected beta profile with live OIDC issuer.
    Connected,
    /// Mirror-only profile served from a declared signed mirror.
    MirrorOnly,
    /// Offline profile served from a last-known-good or air-gapped snapshot.
    Offline,
    /// Enterprise-managed profile applying signed managed policy narrowing.
    EnterpriseManaged,
}

impl OidcSystemBrowserBetaProfileClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Connected => "connected",
            Self::MirrorOnly => "mirror_only",
            Self::Offline => "offline",
            Self::EnterpriseManaged => "enterprise_managed",
        }
    }
}

/// Closed vocabulary naming the OIDC issuer source of record. Account-free
/// local rows quote [`Self::NotApplicableAccountFreeLocal`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OidcIssuerSourceClass {
    /// Vendor-managed enterprise OIDC issuer (single-tenant or multi-tenant
    /// managed). Requires a verified discovery + JWKS source.
    ManagedEnterpriseIssuer,
    /// Customer-operated self-hosted enterprise OIDC issuer.
    CustomerSelfHostedIssuer,
    /// Signed mirror of an authoritative enterprise OIDC issuer.
    SignedMirrorIssuer,
    /// Manual signed-file import of the issuer metadata bundle.
    ManualSignedFileImport,
    /// Account-free local row; OIDC issuer is not applicable.
    NotApplicableAccountFreeLocal,
}

impl OidcIssuerSourceClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManagedEnterpriseIssuer => "managed_enterprise_issuer",
            Self::CustomerSelfHostedIssuer => "customer_self_hosted_issuer",
            Self::SignedMirrorIssuer => "signed_mirror_issuer",
            Self::ManualSignedFileImport => "manual_signed_file_import",
            Self::NotApplicableAccountFreeLocal => "not_applicable_account_free_local",
        }
    }

    /// True when the source represents a claimed enterprise row that MUST
    /// expose an issuer label, domain label, and JWKS source.
    pub const fn requires_issuer_disclosure(self) -> bool {
        matches!(
            self,
            Self::ManagedEnterpriseIssuer
                | Self::CustomerSelfHostedIssuer
                | Self::SignedMirrorIssuer
                | Self::ManualSignedFileImport
        )
    }
}

/// Closed vocabulary naming how the row binds to tenant + workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OidcTenantBindingClass {
    /// Row binds to both a tenant and a workspace.
    WorkspaceAndTenantBound,
    /// Row binds to a tenant only (workspace-agnostic).
    TenantOnlyBound,
    /// Row binds to a workspace inside an implicit tenant.
    WorkspaceOnlyBound,
    /// Account-free local row; no tenant or workspace binding.
    NoTenantBindingAccountFreeLocal,
}

impl OidcTenantBindingClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WorkspaceAndTenantBound => "workspace_and_tenant_bound",
            Self::TenantOnlyBound => "tenant_only_bound",
            Self::WorkspaceOnlyBound => "workspace_only_bound",
            Self::NoTenantBindingAccountFreeLocal => "no_tenant_binding_account_free_local",
        }
    }

    /// True when the binding requires both tenant and workspace refs.
    pub const fn requires_workspace_ref(self) -> bool {
        matches!(self, Self::WorkspaceAndTenantBound | Self::WorkspaceOnlyBound)
    }

    /// True when the binding requires a tenant ref.
    pub const fn requires_tenant_ref(self) -> bool {
        matches!(self, Self::WorkspaceAndTenantBound | Self::TenantOnlyBound)
    }
}

/// Closed vocabulary naming the OIDC session lifecycle state at the moment
/// the row is rendered.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OidcSessionStateClass {
    /// Active session with a current ID token / refresh token.
    SignedInActive,
    /// Refresh in progress; managed actions are narrowed while the user
    /// retains local editing.
    RefreshPendingManagedNarrowed,
    /// Refresh expired or the issuer denied refresh; managed actions are
    /// blocked while local editing remains intact.
    RefreshExpiredManagedBlocked,
    /// User signed out from the row; local editing is preserved, managed
    /// actions are blocked.
    SignedOutLocalIntact,
    /// The issuer is unreachable; managed actions are blocked, local
    /// editing remains intact.
    IdentityOutageManagedBlocked,
    /// The user or provider denied auth; managed actions are blocked,
    /// local editing remains intact.
    AuthDenialManagedBlocked,
    /// Account-free local row; OIDC session is not applicable.
    AccountFreeLocalNoAuthRequired,
}

impl OidcSessionStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedInActive => "signed_in_active",
            Self::RefreshPendingManagedNarrowed => "refresh_pending_managed_narrowed",
            Self::RefreshExpiredManagedBlocked => "refresh_expired_managed_blocked",
            Self::SignedOutLocalIntact => "signed_out_local_intact",
            Self::IdentityOutageManagedBlocked => "identity_outage_managed_blocked",
            Self::AuthDenialManagedBlocked => "auth_denial_managed_blocked",
            Self::AccountFreeLocalNoAuthRequired => "account_free_local_no_auth_required",
        }
    }

    /// True when the state narrows managed actions (i.e. the row is not in
    /// an active signed-in posture).
    pub const fn narrows_managed_actions(self) -> bool {
        matches!(
            self,
            Self::RefreshPendingManagedNarrowed
                | Self::RefreshExpiredManagedBlocked
                | Self::SignedOutLocalIntact
                | Self::IdentityOutageManagedBlocked
                | Self::AuthDenialManagedBlocked
        )
    }

    /// True when the state is an outage / denial / signed-out posture and the
    /// row MUST preserve local editing while narrowing managed authority.
    pub const fn requires_local_editing_preserved(self) -> bool {
        matches!(
            self,
            Self::RefreshPendingManagedNarrowed
                | Self::RefreshExpiredManagedBlocked
                | Self::SignedOutLocalIntact
                | Self::IdentityOutageManagedBlocked
                | Self::AuthDenialManagedBlocked
        )
    }

    /// True when the state is an outage / denial / refresh-expired posture
    /// that MUST also publish a typed [`OidcIdentityOutageClass`] beyond
    /// `no_outage`.
    pub const fn requires_outage_disclosure(self) -> bool {
        matches!(
            self,
            Self::RefreshExpiredManagedBlocked
                | Self::IdentityOutageManagedBlocked
                | Self::AuthDenialManagedBlocked
        )
    }

    /// True when the state requires no remote auth and grants no remote
    /// scope.
    pub const fn is_account_free_local(self) -> bool {
        matches!(self, Self::AccountFreeLocalNoAuthRequired)
    }
}

/// Closed vocabulary naming the identity-outage posture for the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OidcIdentityOutageClass {
    /// No identity outage. The session is healthy or already managed-narrowed
    /// for a non-outage reason (refresh pending, signed out).
    NoOutage,
    /// The issuer is unreachable. The row narrows to local-only while
    /// managed actions are blocked.
    IssuerUnreachableLocalOnly,
    /// The issuer is degraded; refresh is deferred while local editing
    /// remains intact.
    IssuerDegradedRefreshDeferred,
    /// The row is served from a last-known-good or air-gapped snapshot.
    IssuerOfflineSnapshotOnly,
    /// Account-free local row; outage classification is not applicable.
    NotApplicableAccountFreeLocal,
}

impl OidcIdentityOutageClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoOutage => "no_outage",
            Self::IssuerUnreachableLocalOnly => "issuer_unreachable_local_only",
            Self::IssuerDegradedRefreshDeferred => "issuer_degraded_refresh_deferred",
            Self::IssuerOfflineSnapshotOnly => "issuer_offline_snapshot_only",
            Self::NotApplicableAccountFreeLocal => "not_applicable_account_free_local",
        }
    }

    /// True when the outage class implies managed authority is unavailable.
    pub const fn narrows_managed_actions(self) -> bool {
        matches!(
            self,
            Self::IssuerUnreachableLocalOnly
                | Self::IssuerDegradedRefreshDeferred
                | Self::IssuerOfflineSnapshotOnly
        )
    }
}

/// Closed vocabulary naming the sign-out / continuity posture for the row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OidcSignOutContinuityClass {
    /// Active session; sign-out continuity is not applicable yet.
    SignedInActiveNoSignOutYet,
    /// Local editing is preserved while managed actions are narrowed.
    LocalEditingPreservedManagedNarrowed,
    /// Local editing is preserved while managed actions are fully blocked
    /// (e.g. sign-out, refresh expired, outage).
    LocalEditingPreservedManagedBlocked,
    /// Account-free local row; no sign-out continuity needed.
    AccountFreeLocalPassthrough,
}

impl OidcSignOutContinuityClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SignedInActiveNoSignOutYet => "signed_in_active_no_sign_out_yet",
            Self::LocalEditingPreservedManagedNarrowed => {
                "local_editing_preserved_managed_narrowed"
            }
            Self::LocalEditingPreservedManagedBlocked => "local_editing_preserved_managed_blocked",
            Self::AccountFreeLocalPassthrough => "account_free_local_passthrough",
        }
    }

    /// True when the posture preserves local editing.
    pub const fn preserves_local_editing(self) -> bool {
        matches!(
            self,
            Self::SignedInActiveNoSignOutYet
                | Self::LocalEditingPreservedManagedNarrowed
                | Self::LocalEditingPreservedManagedBlocked
                | Self::AccountFreeLocalPassthrough
        )
    }
}

/// Closed vocabulary naming the user-visible recovery action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OidcRecoveryActionClass {
    /// No recovery is required; the row is in a healthy signed-in posture.
    NoRecoveryRequired,
    /// Resume sign-in by reopening the system browser.
    ResumeInSystemBrowser,
    /// Retry refresh by reopening the system browser.
    RetryRefreshInSystemBrowser,
    /// Fall back to device-code auth.
    SwitchToDeviceCode,
    /// Continue working locally without sign-in.
    ContinueLocalWithoutSignIn,
    /// Inspect the admin policy that blocked auth.
    InspectAdminPolicy,
    /// Contact support with a redaction-safe export.
    ContactSupportWithExport,
}

impl OidcRecoveryActionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoRecoveryRequired => "no_recovery_required",
            Self::ResumeInSystemBrowser => "resume_in_system_browser",
            Self::RetryRefreshInSystemBrowser => "retry_refresh_in_system_browser",
            Self::SwitchToDeviceCode => "switch_to_device_code",
            Self::ContinueLocalWithoutSignIn => "continue_local_without_sign_in",
            Self::InspectAdminPolicy => "inspect_admin_policy",
            Self::ContactSupportWithExport => "contact_support_with_export",
        }
    }
}

/// Closed authority-scope vocabulary surfaced on the row to disclose the
/// requested vs granted scope of the row's OIDC session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OidcAuthorityScopeClass {
    /// No scope is granted (signed out, outage, denial, refresh expired).
    NoScopeGranted,
    /// Read-only access.
    ReadOnlyScope,
    /// Step-up authority for one risky action.
    StepUpScope,
    /// Read+write access on the bound workspace / tenant.
    ReadWriteScope,
    /// Workspace-admin authority.
    WorkspaceAdminScope,
    /// Tenant- or org-wide admin authority.
    TenantAdminScope,
}

impl OidcAuthorityScopeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoScopeGranted => "no_scope_granted",
            Self::ReadOnlyScope => "read_only_scope",
            Self::StepUpScope => "step_up_scope",
            Self::ReadWriteScope => "read_write_scope",
            Self::WorkspaceAdminScope => "workspace_admin_scope",
            Self::TenantAdminScope => "tenant_admin_scope",
        }
    }

    /// Comparable rank used by the no-widening check.
    pub const fn rank(self) -> u8 {
        match self {
            Self::NoScopeGranted => 0,
            Self::ReadOnlyScope => 1,
            Self::StepUpScope => 2,
            Self::ReadWriteScope => 3,
            Self::WorkspaceAdminScope => 4,
            Self::TenantAdminScope => 5,
        }
    }

    /// True when `granted` would widen authority beyond `requested`.
    pub fn widens(requested: Self, granted: Self) -> bool {
        granted.rank() > requested.rank()
    }
}

/// Closed semantic-axis vocabulary the audit verifies per row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OidcSystemBrowserBetaAxis {
    /// Claimed-enterprise row quotes a closed issuer source plus issuer
    /// label, domain label, and JWKS source label.
    EnterpriseIssuerSourceDisclosed,
    /// Claimed-enterprise row quotes tenant and workspace binding refs.
    TenantAndWorkspaceBindingDisclosed,
    /// Row quotes the return path labels and tokens needed to validate the
    /// auth-callback packet on return.
    ReturnPathLabelsPreserved,
    /// Session continuity / sign-out posture preserves local editing.
    SessionContinuityPreservesLocalEditing,
    /// Identity outage / denial / refresh-expired states narrow managed
    /// actions and never silently fall back to public endpoints.
    IdentityOutageDegradesTruthfully,
    /// Granted authority scope does not widen the requested scope.
    NoAuthorityWideningOnReturn,
    /// Support row reuses the same closed-vocabulary tokens as the live row.
    SupportExportVocabularyParity,
}

impl OidcSystemBrowserBetaAxis {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnterpriseIssuerSourceDisclosed => "enterprise_issuer_source_disclosed",
            Self::TenantAndWorkspaceBindingDisclosed => "tenant_and_workspace_binding_disclosed",
            Self::ReturnPathLabelsPreserved => "return_path_labels_preserved",
            Self::SessionContinuityPreservesLocalEditing => {
                "session_continuity_preserves_local_editing"
            }
            Self::IdentityOutageDegradesTruthfully => "identity_outage_degrades_truthfully",
            Self::NoAuthorityWideningOnReturn => "no_authority_widening_on_return",
            Self::SupportExportVocabularyParity => "support_export_vocabulary_parity",
        }
    }
}

/// Closed defect vocabulary the audit emits.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OidcSystemBrowserBetaDefectKind {
    /// Claimed-enterprise row missing an issuer label.
    IssuerLabelMissing,
    /// Claimed-enterprise row missing the issuer domain label.
    IssuerDomainLabelMissing,
    /// Claimed-enterprise row missing the JWKS source label.
    JwksSourceLabelMissing,
    /// Row silently fell back to a public issuer endpoint.
    PublicIssuerFallbackUsed,
    /// Row's tenant binding requires a tenant ref but did not provide one.
    TenantBindingMissing,
    /// Row's tenant binding requires a workspace ref but did not provide one.
    WorkspaceBindingMissing,
    /// Row missing a return-anchor ref while claiming a non-local return.
    ReturnAnchorRefMissing,
    /// Row missing a return-mode token.
    ReturnModeMissing,
    /// Row's outage or denial posture narrowed managed actions in name but
    /// did not preserve local editing.
    SignOutOrOutageLosesLocalEditing,
    /// Row in an outage / denial / refresh-expired posture but quoted
    /// `no_outage` instead of a typed outage class.
    IdentityOutageMissingClass,
    /// Outage row did not name a recovery action other than
    /// `no_recovery_required`.
    OutageRecoveryActionMissing,
    /// Row claimed a signed-in active posture but the granted scope is
    /// `no_scope_granted`.
    SignedInActiveWithoutGrantedScope,
    /// Row in a managed-narrowed posture still claimed a granted scope that
    /// widens beyond the requested scope.
    ReturnWidensAuthorityScope,
    /// Row in a non-account-free posture had no managed session state ref.
    ManagedSessionStateRefMissing,
    /// Support row drifted from the live row on a closed-vocabulary token.
    SupportRowVocabularyDrift,
    /// Row claimed account-free local but still quoted a non-local issuer
    /// source or tenant binding.
    AccountFreeLocalMislabeled,
}

impl OidcSystemBrowserBetaDefectKind {
    /// Stable token recorded on the defect.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IssuerLabelMissing => "issuer_label_missing",
            Self::IssuerDomainLabelMissing => "issuer_domain_label_missing",
            Self::JwksSourceLabelMissing => "jwks_source_label_missing",
            Self::PublicIssuerFallbackUsed => "public_issuer_fallback_used",
            Self::TenantBindingMissing => "tenant_binding_missing",
            Self::WorkspaceBindingMissing => "workspace_binding_missing",
            Self::ReturnAnchorRefMissing => "return_anchor_ref_missing",
            Self::ReturnModeMissing => "return_mode_missing",
            Self::SignOutOrOutageLosesLocalEditing => "sign_out_or_outage_loses_local_editing",
            Self::IdentityOutageMissingClass => "identity_outage_missing_class",
            Self::OutageRecoveryActionMissing => "outage_recovery_action_missing",
            Self::SignedInActiveWithoutGrantedScope => "signed_in_active_without_granted_scope",
            Self::ReturnWidensAuthorityScope => "return_widens_authority_scope",
            Self::ManagedSessionStateRefMissing => "managed_session_state_ref_missing",
            Self::SupportRowVocabularyDrift => "support_row_vocabulary_drift",
            Self::AccountFreeLocalMislabeled => "account_free_local_mislabeled",
        }
    }
}

/// OIDC issuer disclosure block. Account-free local rows leave the disclosure
/// fields empty and quote `not_applicable_account_free_local` as the source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcIssuerDisclosure {
    /// Closed source token from [`OidcIssuerSourceClass`].
    pub source_token: String,
    /// Plain-language issuer label.
    pub issuer_label: String,
    /// Issuer domain label used as the anti-phishing cue.
    pub issuer_domain_label: String,
    /// Plain-language JWKS source label (e.g. "mirror.acme.example/jwks.json").
    pub jwks_source_label: String,
    /// Whether the row silently fell back to a public issuer endpoint. The
    /// validator emits a typed defect when this flag is true.
    pub public_issuer_fallback_used: bool,
    /// Optional opaque issuer discovery ref (e.g. `discovery:acme:payments-prod`).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discovery_ref: Option<String>,
}

/// Tenant + workspace binding disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcTenantBinding {
    /// Closed binding token from [`OidcTenantBindingClass`].
    pub binding_token: String,
    /// Opaque tenant or org ref bound to this row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_tenant_or_org_ref: Option<String>,
    /// Opaque workspace ref bound to this row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_workspace_ref: Option<String>,
    /// Opaque actor subject ref bound to this row.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_actor_subject_ref: Option<String>,
}

/// Return-path labels carried on every audited row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcReturnPathLabel {
    /// Workspace label rendered on the return interstitial.
    pub workspace_label: String,
    /// Target label (e.g. "Settings → Provider sync").
    pub target_label: String,
    /// Plain-language label for the action the user requested before
    /// sign-in.
    pub requested_action_label: String,
    /// Return-mode token from [`ReturnModeClass`].
    pub return_mode_token: String,
    /// Origin-validation token from [`ReturnOriginValidationClass`].
    pub return_origin_validation_token: String,
    /// Tenant/workspace match-rule token from
    /// [`ReturnTenantOrWorkspaceMatchRule`].
    pub return_tenant_or_workspace_match_rule_token: String,
    /// Stable opaque return-anchor ref.
    pub return_anchor_ref: String,
}

/// Session continuity / sign-out block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcSessionContinuityBlock {
    /// Closed session-state token from [`OidcSessionStateClass`].
    pub session_state_token: String,
    /// Closed sign-out continuity token from [`OidcSignOutContinuityClass`].
    pub sign_out_continuity_token: String,
    /// Plain-language description of what remains usable locally.
    pub local_editing_summary_label: String,
    /// Plain-language description of which managed actions are narrowed or
    /// blocked.
    pub managed_action_narrowing_label: String,
    /// True when the row preserves local editing.
    pub local_editing_preserved: bool,
    /// True when the row narrows managed actions.
    pub managed_actions_narrowed: bool,
    /// Optional opaque managed-session state ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_session_state_ref: Option<String>,
}

/// Identity-outage block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcIdentityOutageBlock {
    /// Closed outage token from [`OidcIdentityOutageClass`].
    pub outage_token: String,
    /// Plain-language reason rendered next to the outage class.
    pub reason_label: String,
    /// Closed recovery-action token from [`OidcRecoveryActionClass`].
    pub recovery_action_token: String,
    /// Plain-language recovery action label.
    pub recovery_action_label: String,
}

/// Audited row for one claimed OIDC scenario in the beta projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcSystemBrowserBetaRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub row_id: String,
    pub source_claim_row_ref: String,
    pub profile_token: String,

    pub issuer: OidcIssuerDisclosure,
    pub tenant_binding: OidcTenantBinding,
    pub return_path_label: OidcReturnPathLabel,
    pub session_continuity: OidcSessionContinuityBlock,
    pub identity_outage: OidcIdentityOutageBlock,

    pub requested_authority_scope_token: String,
    pub granted_authority_scope_token: String,
    pub authority_scope_summary_label: String,

    pub promised_audit_axes: Vec<OidcSystemBrowserBetaAxis>,
    pub plain_language_summary: String,
    pub redaction_class_token: String,
}

/// Export-safe support row aligned 1:1 with [`OidcSystemBrowserBetaRow`] by
/// `row_id`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcSystemBrowserBetaSupportRow {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub case_id: String,
    pub row_id: String,
    pub profile_token: String,
    pub issuer_source_token: String,
    pub issuer_label: String,
    pub issuer_domain_label: String,
    pub jwks_source_label: String,
    pub public_issuer_fallback_used: bool,
    pub tenant_binding_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_tenant_or_org_ref: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub bound_workspace_ref: Option<String>,
    pub return_mode_token: String,
    pub return_origin_validation_token: String,
    pub return_tenant_or_workspace_match_rule_token: String,
    pub return_anchor_ref: String,
    pub workspace_label: String,
    pub target_label: String,
    pub requested_action_label: String,
    pub session_state_token: String,
    pub sign_out_continuity_token: String,
    pub outage_token: String,
    pub recovery_action_token: String,
    pub requested_authority_scope_token: String,
    pub granted_authority_scope_token: String,
    pub redaction_class_token: String,
}

impl OidcSystemBrowserBetaSupportRow {
    /// Project an export-safe support row from a live audited row.
    pub fn from_row(row: &OidcSystemBrowserBetaRow) -> Self {
        Self {
            record_kind: OIDC_SYSTEM_BROWSER_BETA_SUPPORT_ROW_RECORD_KIND.to_owned(),
            schema_version: OIDC_SYSTEM_BROWSER_BETA_SCHEMA_VERSION,
            shared_contract_ref: OIDC_SYSTEM_BROWSER_BETA_SHARED_CONTRACT_REF.to_owned(),
            case_id: row.case_id.clone(),
            row_id: row.row_id.clone(),
            profile_token: row.profile_token.clone(),
            issuer_source_token: row.issuer.source_token.clone(),
            issuer_label: row.issuer.issuer_label.clone(),
            issuer_domain_label: row.issuer.issuer_domain_label.clone(),
            jwks_source_label: row.issuer.jwks_source_label.clone(),
            public_issuer_fallback_used: row.issuer.public_issuer_fallback_used,
            tenant_binding_token: row.tenant_binding.binding_token.clone(),
            bound_tenant_or_org_ref: row.tenant_binding.bound_tenant_or_org_ref.clone(),
            bound_workspace_ref: row.tenant_binding.bound_workspace_ref.clone(),
            return_mode_token: row.return_path_label.return_mode_token.clone(),
            return_origin_validation_token: row
                .return_path_label
                .return_origin_validation_token
                .clone(),
            return_tenant_or_workspace_match_rule_token: row
                .return_path_label
                .return_tenant_or_workspace_match_rule_token
                .clone(),
            return_anchor_ref: row.return_path_label.return_anchor_ref.clone(),
            workspace_label: row.return_path_label.workspace_label.clone(),
            target_label: row.return_path_label.target_label.clone(),
            requested_action_label: row.return_path_label.requested_action_label.clone(),
            session_state_token: row.session_continuity.session_state_token.clone(),
            sign_out_continuity_token: row.session_continuity.sign_out_continuity_token.clone(),
            outage_token: row.identity_outage.outage_token.clone(),
            recovery_action_token: row.identity_outage.recovery_action_token.clone(),
            requested_authority_scope_token: row.requested_authority_scope_token.clone(),
            granted_authority_scope_token: row.granted_authority_scope_token.clone(),
            redaction_class_token: row.redaction_class_token.clone(),
        }
    }
}

/// Typed defect emitted by the audit validator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcSystemBrowserBetaDefect {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub defect_id: String,
    pub defect_kind: OidcSystemBrowserBetaDefectKind,
    pub defect_kind_token: String,
    pub row_id: String,
    pub field: String,
    pub note: String,
}

impl OidcSystemBrowserBetaDefect {
    fn new(
        defect_kind: OidcSystemBrowserBetaDefectKind,
        row_id: &str,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: OIDC_SYSTEM_BROWSER_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: OIDC_SYSTEM_BROWSER_BETA_SCHEMA_VERSION,
            shared_contract_ref: OIDC_SYSTEM_BROWSER_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_id: format!(
                "auth:defect:oidc-system-browser:{}:{}",
                defect_kind.as_str(),
                row_id
            ),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            row_id: row_id.to_owned(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate banner emitted with the page.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct OidcSystemBrowserBetaSummary {
    pub row_count: usize,
    pub support_row_count: usize,
    pub defect_count: usize,
    pub claimed_enterprise_row_count: usize,
    pub outage_or_denial_row_count: usize,
    pub signed_out_row_count: usize,
    pub profiles_present: Vec<String>,
    pub issuer_sources_present: Vec<String>,
    pub session_states_present: Vec<String>,
    pub recovery_actions_present: Vec<String>,
}

impl OidcSystemBrowserBetaSummary {
    fn from_rows(
        rows: &[OidcSystemBrowserBetaRow],
        support_rows: &[OidcSystemBrowserBetaSupportRow],
        defects: &[OidcSystemBrowserBetaDefect],
    ) -> Self {
        let mut profiles: Vec<String> = Vec::new();
        let mut sources: Vec<String> = Vec::new();
        let mut states: Vec<String> = Vec::new();
        let mut recoveries: Vec<String> = Vec::new();
        let mut claimed_enterprise = 0usize;
        let mut outage_or_denial = 0usize;
        let mut signed_out = 0usize;
        for row in rows {
            if !profiles.contains(&row.profile_token) {
                profiles.push(row.profile_token.clone());
            }
            if !sources.contains(&row.issuer.source_token) {
                sources.push(row.issuer.source_token.clone());
            }
            if !states.contains(&row.session_continuity.session_state_token) {
                states.push(row.session_continuity.session_state_token.clone());
            }
            if !recoveries.contains(&row.identity_outage.recovery_action_token) {
                recoveries.push(row.identity_outage.recovery_action_token.clone());
            }
            if row.issuer.source_token
                != OidcIssuerSourceClass::NotApplicableAccountFreeLocal.as_str()
            {
                claimed_enterprise += 1;
            }
            if matches!(
                row.session_continuity.session_state_token.as_str(),
                "identity_outage_managed_blocked"
                    | "auth_denial_managed_blocked"
                    | "refresh_expired_managed_blocked"
            ) {
                outage_or_denial += 1;
            }
            if row.session_continuity.session_state_token
                == OidcSessionStateClass::SignedOutLocalIntact.as_str()
            {
                signed_out += 1;
            }
        }
        profiles.sort();
        sources.sort();
        states.sort();
        recoveries.sort();
        Self {
            row_count: rows.len(),
            support_row_count: support_rows.len(),
            defect_count: defects.len(),
            claimed_enterprise_row_count: claimed_enterprise,
            outage_or_denial_row_count: outage_or_denial,
            signed_out_row_count: signed_out,
            profiles_present: profiles,
            issuer_sources_present: sources,
            session_states_present: states,
            recovery_actions_present: recoveries,
        }
    }
}

/// Top-level beta audit page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcSystemBrowserBetaPage {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub page_id: String,
    pub page_label: String,
    pub generated_at: String,
    pub summary: OidcSystemBrowserBetaSummary,
    pub rows: Vec<OidcSystemBrowserBetaRow>,
    pub support_rows: Vec<OidcSystemBrowserBetaSupportRow>,
    pub defects: Vec<OidcSystemBrowserBetaDefect>,
}

impl OidcSystemBrowserBetaPage {
    /// Build a page from rows. Support rows are projected automatically and
    /// the defect list is computed by [`audit_oidc_system_browser_beta_rows`].
    pub fn new(
        page_id: impl Into<String>,
        page_label: impl Into<String>,
        generated_at: impl Into<String>,
        rows: Vec<OidcSystemBrowserBetaRow>,
    ) -> Self {
        let support_rows: Vec<OidcSystemBrowserBetaSupportRow> = rows
            .iter()
            .map(OidcSystemBrowserBetaSupportRow::from_row)
            .collect();
        let defects = audit_oidc_system_browser_beta_rows(&rows, &support_rows);
        let summary = OidcSystemBrowserBetaSummary::from_rows(&rows, &support_rows, &defects);
        Self {
            record_kind: OIDC_SYSTEM_BROWSER_BETA_PAGE_RECORD_KIND.to_owned(),
            schema_version: OIDC_SYSTEM_BROWSER_BETA_SCHEMA_VERSION,
            shared_contract_ref: OIDC_SYSTEM_BROWSER_BETA_SHARED_CONTRACT_REF.to_owned(),
            page_id: page_id.into(),
            page_label: page_label.into(),
            generated_at: generated_at.into(),
            summary,
            rows,
            support_rows,
            defects,
        }
    }

    /// True when every claimed-enterprise row preserves local editing in
    /// its current session-continuity posture.
    pub fn session_continuity_preserves_local_editing(&self) -> bool {
        self.rows
            .iter()
            .all(|row| row.session_continuity.local_editing_preserved)
    }

    /// True when every outage / denial / refresh-expired row narrows managed
    /// actions and never silently falls back to a public endpoint.
    pub fn outages_degrade_truthfully(&self) -> bool {
        self.rows.iter().all(|row| {
            if matches!(
                row.session_continuity.session_state_token.as_str(),
                "identity_outage_managed_blocked"
                    | "auth_denial_managed_blocked"
                    | "refresh_expired_managed_blocked"
            ) {
                row.session_continuity.managed_actions_narrowed
                    && !row.issuer.public_issuer_fallback_used
            } else {
                true
            }
        })
    }
}

/// Support-export wrapper that quotes the audited page plus a metadata-safe
/// defect roll-up.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OidcSystemBrowserBetaSupportExport {
    pub record_kind: String,
    pub schema_version: u32,
    pub shared_contract_ref: String,
    pub export_id: String,
    pub generated_at: String,
    pub page: OidcSystemBrowserBetaPage,
    pub defect_kinds_present: Vec<OidcSystemBrowserBetaDefectKind>,
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    pub raw_private_material_excluded: bool,
}

impl OidcSystemBrowserBetaSupportExport {
    /// Wrap a page into a support-export envelope.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: OidcSystemBrowserBetaPage,
    ) -> Self {
        let mut kinds: Vec<OidcSystemBrowserBetaDefectKind> = Vec::new();
        let mut counts: BTreeMap<String, usize> = BTreeMap::new();
        for defect in &page.defects {
            if !kinds.contains(&defect.defect_kind) {
                kinds.push(defect.defect_kind);
            }
            *counts.entry(defect.defect_kind_token.clone()).or_insert(0) += 1;
        }
        kinds.sort();
        Self {
            record_kind: OIDC_SYSTEM_BROWSER_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: OIDC_SYSTEM_BROWSER_BETA_SCHEMA_VERSION,
            shared_contract_ref: OIDC_SYSTEM_BROWSER_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            generated_at: generated_at.into(),
            page,
            defect_kinds_present: kinds,
            defect_counts_by_kind: counts,
            raw_private_material_excluded: true,
        }
    }
}

/// Validate a beta page; returns `Ok` on a clean audit and the typed defect
/// list otherwise.
pub fn validate_oidc_system_browser_beta_page(
    page: &OidcSystemBrowserBetaPage,
) -> Result<(), Vec<OidcSystemBrowserBetaDefect>> {
    if page.defects.is_empty() {
        Ok(())
    } else {
        Err(page.defects.clone())
    }
}

fn parse_authority_scope(token: &str) -> Option<OidcAuthorityScopeClass> {
    match token {
        "no_scope_granted" => Some(OidcAuthorityScopeClass::NoScopeGranted),
        "read_only_scope" => Some(OidcAuthorityScopeClass::ReadOnlyScope),
        "step_up_scope" => Some(OidcAuthorityScopeClass::StepUpScope),
        "read_write_scope" => Some(OidcAuthorityScopeClass::ReadWriteScope),
        "workspace_admin_scope" => Some(OidcAuthorityScopeClass::WorkspaceAdminScope),
        "tenant_admin_scope" => Some(OidcAuthorityScopeClass::TenantAdminScope),
        _ => None,
    }
}

/// Re-run the audit over the row + support-row pair without rebuilding the
/// page. Tests and the headless inspector use this to surface a defect that
/// would otherwise hide behind a stale `page.defects` array.
pub fn audit_oidc_system_browser_beta_rows(
    rows: &[OidcSystemBrowserBetaRow],
    support_rows: &[OidcSystemBrowserBetaSupportRow],
) -> Vec<OidcSystemBrowserBetaDefect> {
    let mut defects: Vec<OidcSystemBrowserBetaDefect> = Vec::new();
    let support_by_id: BTreeMap<&str, &OidcSystemBrowserBetaSupportRow> = support_rows
        .iter()
        .map(|row| (row.row_id.as_str(), row))
        .collect();

    let account_free_local_source =
        OidcIssuerSourceClass::NotApplicableAccountFreeLocal.as_str();
    let account_free_local_binding =
        OidcTenantBindingClass::NoTenantBindingAccountFreeLocal.as_str();
    let account_free_local_state = OidcSessionStateClass::AccountFreeLocalNoAuthRequired.as_str();
    let no_outage_token = OidcIdentityOutageClass::NoOutage.as_str();
    let signed_in_active_token = OidcSessionStateClass::SignedInActive.as_str();
    let no_recovery_token = OidcRecoveryActionClass::NoRecoveryRequired.as_str();

    for row in rows {
        let is_local =
            row.issuer.source_token == account_free_local_source
                || row.session_continuity.session_state_token == account_free_local_state;

        // Axis 1: enterprise issuer source disclosed (for non-local rows).
        if !is_local {
            if row.issuer.issuer_label.trim().is_empty() {
                defects.push(OidcSystemBrowserBetaDefect::new(
                    OidcSystemBrowserBetaDefectKind::IssuerLabelMissing,
                    &row.row_id,
                    "issuer.issuer_label",
                    "claimed-enterprise row must name an issuer label",
                ));
            }
            if row.issuer.issuer_domain_label.trim().is_empty() {
                defects.push(OidcSystemBrowserBetaDefect::new(
                    OidcSystemBrowserBetaDefectKind::IssuerDomainLabelMissing,
                    &row.row_id,
                    "issuer.issuer_domain_label",
                    "claimed-enterprise row must quote the issuer domain label",
                ));
            }
            if row.issuer.jwks_source_label.trim().is_empty() {
                defects.push(OidcSystemBrowserBetaDefect::new(
                    OidcSystemBrowserBetaDefectKind::JwksSourceLabelMissing,
                    &row.row_id,
                    "issuer.jwks_source_label",
                    "claimed-enterprise row must quote the JWKS source label",
                ));
            }
        }
        if row.issuer.public_issuer_fallback_used {
            defects.push(OidcSystemBrowserBetaDefect::new(
                OidcSystemBrowserBetaDefectKind::PublicIssuerFallbackUsed,
                &row.row_id,
                "issuer.public_issuer_fallback_used",
                "row silently fell back to a public issuer endpoint",
            ));
        }
        if is_local
            && (row.issuer.source_token != account_free_local_source
                || row.tenant_binding.binding_token != account_free_local_binding)
        {
            defects.push(OidcSystemBrowserBetaDefect::new(
                OidcSystemBrowserBetaDefectKind::AccountFreeLocalMislabeled,
                &row.row_id,
                "issuer.source_token",
                "account-free local row must quote account-free issuer source and tenant binding",
            ));
        }

        // Axis 2: tenant + workspace binding disclosed (for non-local rows).
        if !is_local {
            let binding_token = row.tenant_binding.binding_token.as_str();
            let needs_tenant = matches!(
                binding_token,
                "workspace_and_tenant_bound" | "tenant_only_bound"
            );
            let needs_workspace = matches!(
                binding_token,
                "workspace_and_tenant_bound" | "workspace_only_bound"
            );
            if needs_tenant && row.tenant_binding.bound_tenant_or_org_ref.is_none() {
                defects.push(OidcSystemBrowserBetaDefect::new(
                    OidcSystemBrowserBetaDefectKind::TenantBindingMissing,
                    &row.row_id,
                    "tenant_binding.bound_tenant_or_org_ref",
                    "tenant-bound row must provide a tenant ref",
                ));
            }
            if needs_workspace && row.tenant_binding.bound_workspace_ref.is_none() {
                defects.push(OidcSystemBrowserBetaDefect::new(
                    OidcSystemBrowserBetaDefectKind::WorkspaceBindingMissing,
                    &row.row_id,
                    "tenant_binding.bound_workspace_ref",
                    "workspace-bound row must provide a workspace ref",
                ));
            }
            if row.session_continuity.managed_session_state_ref.is_none() {
                defects.push(OidcSystemBrowserBetaDefect::new(
                    OidcSystemBrowserBetaDefectKind::ManagedSessionStateRefMissing,
                    &row.row_id,
                    "session_continuity.managed_session_state_ref",
                    "claimed-enterprise row must provide a managed session state ref",
                ));
            }
        }

        // Axis 3: return path labels preserved.
        if row.return_path_label.return_mode_token.is_empty() {
            defects.push(OidcSystemBrowserBetaDefect::new(
                OidcSystemBrowserBetaDefectKind::ReturnModeMissing,
                &row.row_id,
                "return_path_label.return_mode_token",
                "row must quote a return-mode token",
            ));
        }
        if row.return_path_label.return_anchor_ref.is_empty() && !is_local {
            defects.push(OidcSystemBrowserBetaDefect::new(
                OidcSystemBrowserBetaDefectKind::ReturnAnchorRefMissing,
                &row.row_id,
                "return_path_label.return_anchor_ref",
                "non-local row must quote a return-anchor ref",
            ));
        }

        // Axis 4: session continuity preserves local editing.
        if row.session_continuity.session_state_token == signed_in_active_token {
            if row.granted_authority_scope_token
                == OidcAuthorityScopeClass::NoScopeGranted.as_str()
            {
                defects.push(OidcSystemBrowserBetaDefect::new(
                    OidcSystemBrowserBetaDefectKind::SignedInActiveWithoutGrantedScope,
                    &row.row_id,
                    "granted_authority_scope_token",
                    "signed-in active row must grant a non-empty scope",
                ));
            }
        } else if !is_local && !row.session_continuity.local_editing_preserved {
            defects.push(OidcSystemBrowserBetaDefect::new(
                OidcSystemBrowserBetaDefectKind::SignOutOrOutageLosesLocalEditing,
                &row.row_id,
                "session_continuity.local_editing_preserved",
                "outage / sign-out / refresh-expired row must preserve local editing",
            ));
        }

        // Axis 5: identity outage / denial / refresh-expired states degrade
        // truthfully.
        let requires_outage_class = matches!(
            row.session_continuity.session_state_token.as_str(),
            "identity_outage_managed_blocked"
                | "auth_denial_managed_blocked"
                | "refresh_expired_managed_blocked"
        );
        if requires_outage_class {
            if row.identity_outage.outage_token == no_outage_token {
                defects.push(OidcSystemBrowserBetaDefect::new(
                    OidcSystemBrowserBetaDefectKind::IdentityOutageMissingClass,
                    &row.row_id,
                    "identity_outage.outage_token",
                    "outage / denial / refresh-expired row must quote a typed outage class",
                ));
            }
            if row.identity_outage.recovery_action_token == no_recovery_token {
                defects.push(OidcSystemBrowserBetaDefect::new(
                    OidcSystemBrowserBetaDefectKind::OutageRecoveryActionMissing,
                    &row.row_id,
                    "identity_outage.recovery_action_token",
                    "outage / denial / refresh-expired row must name a recovery action",
                ));
            }
        }

        // Axis 6: no authority widening on return.
        if let (Some(req), Some(grant)) = (
            parse_authority_scope(&row.requested_authority_scope_token),
            parse_authority_scope(&row.granted_authority_scope_token),
        ) {
            if OidcAuthorityScopeClass::widens(req, grant) {
                defects.push(OidcSystemBrowserBetaDefect::new(
                    OidcSystemBrowserBetaDefectKind::ReturnWidensAuthorityScope,
                    &row.row_id,
                    "granted_authority_scope_token",
                    "granted scope widens beyond the requested scope",
                ));
            }
        }

        // Axis 7: support-row vocabulary parity.
        if let Some(support) = support_by_id.get(row.row_id.as_str()) {
            if support.profile_token != row.profile_token
                || support.issuer_source_token != row.issuer.source_token
                || support.issuer_label != row.issuer.issuer_label
                || support.issuer_domain_label != row.issuer.issuer_domain_label
                || support.jwks_source_label != row.issuer.jwks_source_label
                || support.public_issuer_fallback_used != row.issuer.public_issuer_fallback_used
                || support.tenant_binding_token != row.tenant_binding.binding_token
                || support.bound_tenant_or_org_ref != row.tenant_binding.bound_tenant_or_org_ref
                || support.bound_workspace_ref != row.tenant_binding.bound_workspace_ref
                || support.return_mode_token != row.return_path_label.return_mode_token
                || support.return_origin_validation_token
                    != row.return_path_label.return_origin_validation_token
                || support.return_tenant_or_workspace_match_rule_token
                    != row
                        .return_path_label
                        .return_tenant_or_workspace_match_rule_token
                || support.return_anchor_ref != row.return_path_label.return_anchor_ref
                || support.workspace_label != row.return_path_label.workspace_label
                || support.target_label != row.return_path_label.target_label
                || support.requested_action_label != row.return_path_label.requested_action_label
                || support.session_state_token != row.session_continuity.session_state_token
                || support.sign_out_continuity_token
                    != row.session_continuity.sign_out_continuity_token
                || support.outage_token != row.identity_outage.outage_token
                || support.recovery_action_token != row.identity_outage.recovery_action_token
                || support.requested_authority_scope_token != row.requested_authority_scope_token
                || support.granted_authority_scope_token != row.granted_authority_scope_token
            {
                defects.push(OidcSystemBrowserBetaDefect::new(
                    OidcSystemBrowserBetaDefectKind::SupportRowVocabularyDrift,
                    &row.row_id,
                    "support_row",
                    "support row drifted from live row on a closed-vocabulary token",
                ));
            }
        } else {
            defects.push(OidcSystemBrowserBetaDefect::new(
                OidcSystemBrowserBetaDefectKind::SupportRowVocabularyDrift,
                &row.row_id,
                "support_row",
                "live row has no aligned support row",
            ));
        }
    }

    defects
}

/// Inputs used to seed a beta row.
#[derive(Debug, Clone)]
pub struct StageOidcSystemBrowserBetaRowRequest<'a> {
    pub case_id: &'a str,
    pub row_id: &'a str,
    pub source_claim_row_ref: &'a str,
    pub profile: OidcSystemBrowserBetaProfileClass,

    pub issuer_source: OidcIssuerSourceClass,
    pub issuer_label: &'a str,
    pub issuer_domain_label: &'a str,
    pub jwks_source_label: &'a str,
    pub public_issuer_fallback_used: bool,
    pub discovery_ref: Option<&'a str>,

    pub tenant_binding: OidcTenantBindingClass,
    pub bound_tenant_or_org_ref: Option<&'a str>,
    pub bound_workspace_ref: Option<&'a str>,
    pub bound_actor_subject_ref: Option<&'a str>,

    pub workspace_label: &'a str,
    pub target_label: &'a str,
    pub requested_action_label: &'a str,
    pub return_mode_class: ReturnModeClass,
    pub return_origin_validation_class: ReturnOriginValidationClass,
    pub return_tenant_or_workspace_match_rule: ReturnTenantOrWorkspaceMatchRule,
    pub return_anchor_ref: &'a str,

    pub session_state: OidcSessionStateClass,
    pub sign_out_continuity: OidcSignOutContinuityClass,
    pub local_editing_summary_label: &'a str,
    pub managed_action_narrowing_label: &'a str,
    pub managed_session_state_ref: Option<&'a str>,

    pub identity_outage: OidcIdentityOutageClass,
    pub outage_reason_label: &'a str,
    pub recovery_action: OidcRecoveryActionClass,
    pub recovery_action_label: &'a str,

    pub requested_authority_scope: OidcAuthorityScopeClass,
    pub granted_authority_scope: OidcAuthorityScopeClass,
    pub authority_scope_summary_label: &'a str,

    pub plain_language_summary: &'a str,
}

impl<'a> StageOidcSystemBrowserBetaRowRequest<'a> {
    /// Mint a beta row with all closed-vocabulary tokens stamped from the
    /// passed inputs.
    pub fn stage(self) -> OidcSystemBrowserBetaRow {
        let local_editing_preserved = self.sign_out_continuity.preserves_local_editing();
        let managed_actions_narrowed = self.session_state.narrows_managed_actions();
        let promised_axes = vec![
            OidcSystemBrowserBetaAxis::EnterpriseIssuerSourceDisclosed,
            OidcSystemBrowserBetaAxis::TenantAndWorkspaceBindingDisclosed,
            OidcSystemBrowserBetaAxis::ReturnPathLabelsPreserved,
            OidcSystemBrowserBetaAxis::SessionContinuityPreservesLocalEditing,
            OidcSystemBrowserBetaAxis::IdentityOutageDegradesTruthfully,
            OidcSystemBrowserBetaAxis::NoAuthorityWideningOnReturn,
            OidcSystemBrowserBetaAxis::SupportExportVocabularyParity,
        ];
        OidcSystemBrowserBetaRow {
            record_kind: OIDC_SYSTEM_BROWSER_BETA_ROW_RECORD_KIND.to_owned(),
            schema_version: OIDC_SYSTEM_BROWSER_BETA_SCHEMA_VERSION,
            shared_contract_ref: OIDC_SYSTEM_BROWSER_BETA_SHARED_CONTRACT_REF.to_owned(),
            case_id: self.case_id.to_owned(),
            row_id: self.row_id.to_owned(),
            source_claim_row_ref: self.source_claim_row_ref.to_owned(),
            profile_token: self.profile.as_str().to_owned(),
            issuer: OidcIssuerDisclosure {
                source_token: self.issuer_source.as_str().to_owned(),
                issuer_label: self.issuer_label.to_owned(),
                issuer_domain_label: self.issuer_domain_label.to_owned(),
                jwks_source_label: self.jwks_source_label.to_owned(),
                public_issuer_fallback_used: self.public_issuer_fallback_used,
                discovery_ref: self.discovery_ref.map(str::to_owned),
            },
            tenant_binding: OidcTenantBinding {
                binding_token: self.tenant_binding.as_str().to_owned(),
                bound_tenant_or_org_ref: self.bound_tenant_or_org_ref.map(str::to_owned),
                bound_workspace_ref: self.bound_workspace_ref.map(str::to_owned),
                bound_actor_subject_ref: self.bound_actor_subject_ref.map(str::to_owned),
            },
            return_path_label: OidcReturnPathLabel {
                workspace_label: self.workspace_label.to_owned(),
                target_label: self.target_label.to_owned(),
                requested_action_label: self.requested_action_label.to_owned(),
                return_mode_token: self.return_mode_class.as_str().to_owned(),
                return_origin_validation_token: self
                    .return_origin_validation_class
                    .as_str()
                    .to_owned(),
                return_tenant_or_workspace_match_rule_token: self
                    .return_tenant_or_workspace_match_rule
                    .as_str()
                    .to_owned(),
                return_anchor_ref: self.return_anchor_ref.to_owned(),
            },
            session_continuity: OidcSessionContinuityBlock {
                session_state_token: self.session_state.as_str().to_owned(),
                sign_out_continuity_token: self.sign_out_continuity.as_str().to_owned(),
                local_editing_summary_label: self.local_editing_summary_label.to_owned(),
                managed_action_narrowing_label: self.managed_action_narrowing_label.to_owned(),
                local_editing_preserved,
                managed_actions_narrowed,
                managed_session_state_ref: self.managed_session_state_ref.map(str::to_owned),
            },
            identity_outage: OidcIdentityOutageBlock {
                outage_token: self.identity_outage.as_str().to_owned(),
                reason_label: self.outage_reason_label.to_owned(),
                recovery_action_token: self.recovery_action.as_str().to_owned(),
                recovery_action_label: self.recovery_action_label.to_owned(),
            },
            requested_authority_scope_token: self.requested_authority_scope.as_str().to_owned(),
            granted_authority_scope_token: self.granted_authority_scope.as_str().to_owned(),
            authority_scope_summary_label: self.authority_scope_summary_label.to_owned(),
            promised_audit_axes: promised_axes,
            plain_language_summary: self.plain_language_summary.to_owned(),
            redaction_class_token: "metadata_only_export_safe".to_owned(),
        }
    }
}

fn signed_in_active_row() -> OidcSystemBrowserBetaRow {
    StageOidcSystemBrowserBetaRowRequest {
        case_id: "signed_in_active_enterprise_session",
        row_id: "oidc:claimed:payments-prod:signed-in",
        source_claim_row_ref: "claim:system-browser-alpha:claimed-identity:managed:payments-prod",
        profile: OidcSystemBrowserBetaProfileClass::Connected,
        issuer_source: OidcIssuerSourceClass::ManagedEnterpriseIssuer,
        issuer_label: "Acme identity",
        issuer_domain_label: "login.acme.example",
        jwks_source_label: "login.acme.example/jwks.json",
        public_issuer_fallback_used: false,
        discovery_ref: Some("oidc-discovery:acme:payments-prod"),
        tenant_binding: OidcTenantBindingClass::WorkspaceAndTenantBound,
        bound_tenant_or_org_ref: Some("tenant:acme-prod"),
        bound_workspace_ref: Some("workspace:payments-prod"),
        bound_actor_subject_ref: Some("actor-subject:sam.acme"),
        workspace_label: "Workspace · payments-prod",
        target_label: "Settings → Provider sync",
        requested_action_label: "Resume Provider sync (read+write) after sign-in.",
        return_mode_class: ReturnModeClass::LoopbackHttpReturn,
        return_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
        return_tenant_or_workspace_match_rule:
            ReturnTenantOrWorkspaceMatchRule::MustMatchBoundWorkspaceAndTenant,
        return_anchor_ref: "return-anchor:loopback:payments-prod:provider-sync",
        session_state: OidcSessionStateClass::SignedInActive,
        sign_out_continuity: OidcSignOutContinuityClass::SignedInActiveNoSignOutYet,
        local_editing_summary_label:
            "Local editing, save, undo, search, local Git, and BYOK AI remain available.",
        managed_action_narrowing_label:
            "Managed provider sync, settings sync, and remote tasks are active.",
        managed_session_state_ref: Some("managed-session:payments-prod"),
        identity_outage: OidcIdentityOutageClass::NoOutage,
        outage_reason_label: "No outage; OIDC issuer is healthy.",
        recovery_action: OidcRecoveryActionClass::NoRecoveryRequired,
        recovery_action_label: "No recovery required.",
        requested_authority_scope: OidcAuthorityScopeClass::ReadWriteScope,
        granted_authority_scope: OidcAuthorityScopeClass::ReadWriteScope,
        authority_scope_summary_label:
            "Requested read+write on payments-prod; granted scope equals requested.",
        plain_language_summary:
            "Active OIDC session on payments-prod: the managed enterprise issuer is healthy, the system-browser sign-in completed with read+write scope, and the workspace + tenant binding match the bound refs.",
    }
    .stage()
}

fn refresh_expired_row() -> OidcSystemBrowserBetaRow {
    StageOidcSystemBrowserBetaRowRequest {
        case_id: "refresh_expired_managed_blocked_local_intact",
        row_id: "oidc:claimed:payments-prod:refresh-expired",
        source_claim_row_ref: "claim:system-browser-alpha:claimed-identity:managed:payments-prod",
        profile: OidcSystemBrowserBetaProfileClass::Connected,
        issuer_source: OidcIssuerSourceClass::ManagedEnterpriseIssuer,
        issuer_label: "Acme identity",
        issuer_domain_label: "login.acme.example",
        jwks_source_label: "login.acme.example/jwks.json",
        public_issuer_fallback_used: false,
        discovery_ref: Some("oidc-discovery:acme:payments-prod"),
        tenant_binding: OidcTenantBindingClass::WorkspaceAndTenantBound,
        bound_tenant_or_org_ref: Some("tenant:acme-prod"),
        bound_workspace_ref: Some("workspace:payments-prod"),
        bound_actor_subject_ref: Some("actor-subject:sam.acme"),
        workspace_label: "Workspace · payments-prod",
        target_label: "Activity center → Resume managed sign-in",
        requested_action_label: "Resume managed sign-in after refresh expired.",
        return_mode_class: ReturnModeClass::LoopbackHttpReturn,
        return_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
        return_tenant_or_workspace_match_rule:
            ReturnTenantOrWorkspaceMatchRule::MustMatchBoundWorkspaceAndTenant,
        return_anchor_ref: "return-anchor:loopback:payments-prod:resume",
        session_state: OidcSessionStateClass::RefreshExpiredManagedBlocked,
        sign_out_continuity: OidcSignOutContinuityClass::LocalEditingPreservedManagedBlocked,
        local_editing_summary_label:
            "Local files, unsaved edits, local Git, and local search remain available.",
        managed_action_narrowing_label:
            "Provider sync, settings sync, and remote tasks wait for sign-in.",
        managed_session_state_ref: Some("managed-session:payments-prod"),
        identity_outage: OidcIdentityOutageClass::IssuerDegradedRefreshDeferred,
        outage_reason_label: "Refresh token expired; sign-in required to resume managed actions.",
        recovery_action: OidcRecoveryActionClass::RetryRefreshInSystemBrowser,
        recovery_action_label: "Resume sign-in in your browser.",
        requested_authority_scope: OidcAuthorityScopeClass::ReadWriteScope,
        granted_authority_scope: OidcAuthorityScopeClass::NoScopeGranted,
        authority_scope_summary_label:
            "Refresh expired; managed scope is fully blocked until sign-in completes.",
        plain_language_summary:
            "Refresh expired on payments-prod: managed actions are blocked while local editing remains intact; the row names retry-refresh-in-system-browser as the recovery action.",
    }
    .stage()
}

fn signed_out_row() -> OidcSystemBrowserBetaRow {
    StageOidcSystemBrowserBetaRowRequest {
        case_id: "signed_out_local_intact_managed_narrowed",
        row_id: "oidc:claimed:payments-prod:signed-out",
        source_claim_row_ref: "claim:system-browser-alpha:claimed-identity:managed:payments-prod",
        profile: OidcSystemBrowserBetaProfileClass::EnterpriseManaged,
        issuer_source: OidcIssuerSourceClass::ManagedEnterpriseIssuer,
        issuer_label: "Acme identity",
        issuer_domain_label: "login.acme.example",
        jwks_source_label: "login.acme.example/jwks.json",
        public_issuer_fallback_used: false,
        discovery_ref: Some("oidc-discovery:acme:payments-prod"),
        tenant_binding: OidcTenantBindingClass::WorkspaceAndTenantBound,
        bound_tenant_or_org_ref: Some("tenant:acme-prod"),
        bound_workspace_ref: Some("workspace:payments-prod"),
        bound_actor_subject_ref: Some("actor-subject:sam.acme"),
        workspace_label: "Workspace · payments-prod",
        target_label: "Account menu → Sign back in",
        requested_action_label: "Sign back in to resume managed actions.",
        return_mode_class: ReturnModeClass::LoopbackHttpReturn,
        return_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
        return_tenant_or_workspace_match_rule:
            ReturnTenantOrWorkspaceMatchRule::MustMatchBoundWorkspaceAndTenant,
        return_anchor_ref: "return-anchor:loopback:payments-prod:sign-back-in",
        session_state: OidcSessionStateClass::SignedOutLocalIntact,
        sign_out_continuity: OidcSignOutContinuityClass::LocalEditingPreservedManagedBlocked,
        local_editing_summary_label:
            "Local files, unsaved edits, local Git, local tasks, and BYOK AI remain available.",
        managed_action_narrowing_label:
            "Managed provider sync, settings sync, and remote tasks pause until sign-in.",
        managed_session_state_ref: Some("managed-session:payments-prod"),
        identity_outage: OidcIdentityOutageClass::NoOutage,
        outage_reason_label: "User signed out; no identity outage.",
        recovery_action: OidcRecoveryActionClass::ResumeInSystemBrowser,
        recovery_action_label: "Sign back in to resume managed actions.",
        requested_authority_scope: OidcAuthorityScopeClass::ReadWriteScope,
        granted_authority_scope: OidcAuthorityScopeClass::NoScopeGranted,
        authority_scope_summary_label:
            "Signed out: managed scope is unavailable until the user signs back in.",
        plain_language_summary:
            "Sign-out on payments-prod: managed actions pause while local editing and BYOK AI remain available; sign-back-in resumes managed scope.",
    }
    .stage()
}

fn issuer_outage_row() -> OidcSystemBrowserBetaRow {
    StageOidcSystemBrowserBetaRowRequest {
        case_id: "issuer_unreachable_managed_blocked_local_intact",
        row_id: "oidc:claimed:payments-prod:issuer-unreachable",
        source_claim_row_ref: "claim:system-browser-alpha:claimed-identity:managed:payments-prod",
        profile: OidcSystemBrowserBetaProfileClass::Offline,
        issuer_source: OidcIssuerSourceClass::ManagedEnterpriseIssuer,
        issuer_label: "Acme identity",
        issuer_domain_label: "login.acme.example",
        jwks_source_label: "login.acme.example/jwks.json (last-known-good snapshot)",
        public_issuer_fallback_used: false,
        discovery_ref: Some("oidc-discovery:acme:payments-prod"),
        tenant_binding: OidcTenantBindingClass::WorkspaceAndTenantBound,
        bound_tenant_or_org_ref: Some("tenant:acme-prod"),
        bound_workspace_ref: Some("workspace:payments-prod"),
        bound_actor_subject_ref: Some("actor-subject:sam.acme"),
        workspace_label: "Workspace · payments-prod",
        target_label: "Activity center → Retry managed sign-in",
        requested_action_label: "Retry managed sign-in when the issuer is reachable.",
        return_mode_class: ReturnModeClass::LoopbackHttpReturn,
        return_origin_validation_class: ReturnOriginValidationClass::LoopbackPortPinned,
        return_tenant_or_workspace_match_rule:
            ReturnTenantOrWorkspaceMatchRule::MustMatchBoundWorkspaceAndTenant,
        return_anchor_ref: "return-anchor:loopback:payments-prod:retry",
        session_state: OidcSessionStateClass::IdentityOutageManagedBlocked,
        sign_out_continuity: OidcSignOutContinuityClass::LocalEditingPreservedManagedBlocked,
        local_editing_summary_label:
            "Local files, unsaved edits, local Git, and local search remain available offline.",
        managed_action_narrowing_label:
            "Managed actions are blocked while the OIDC issuer is unreachable.",
        managed_session_state_ref: Some("managed-session:payments-prod"),
        identity_outage: OidcIdentityOutageClass::IssuerUnreachableLocalOnly,
        outage_reason_label:
            "OIDC issuer login.acme.example is unreachable; the row degrades to local-only.",
        recovery_action: OidcRecoveryActionClass::ContinueLocalWithoutSignIn,
        recovery_action_label:
            "Continue locally; the row retries managed sign-in when the issuer is reachable again.",
        requested_authority_scope: OidcAuthorityScopeClass::ReadWriteScope,
        granted_authority_scope: OidcAuthorityScopeClass::NoScopeGranted,
        authority_scope_summary_label:
            "Issuer outage: no scope granted; managed actions are blocked.",
        plain_language_summary:
            "Issuer outage on payments-prod: managed actions are blocked while local editing remains intact; the row degrades to local-only without falling back to a public issuer.",
    }
    .stage()
}

fn account_free_local_row() -> OidcSystemBrowserBetaRow {
    StageOidcSystemBrowserBetaRowRequest {
        case_id: "account_free_local_no_oidc_required",
        row_id: "oidc:account-free-local",
        source_claim_row_ref: "claim:system-browser-alpha:claimed-identity:local:account-free",
        profile: OidcSystemBrowserBetaProfileClass::Connected,
        issuer_source: OidcIssuerSourceClass::NotApplicableAccountFreeLocal,
        issuer_label: "",
        issuer_domain_label: "",
        jwks_source_label: "",
        public_issuer_fallback_used: false,
        discovery_ref: None,
        tenant_binding: OidcTenantBindingClass::NoTenantBindingAccountFreeLocal,
        bound_tenant_or_org_ref: None,
        bound_workspace_ref: None,
        bound_actor_subject_ref: None,
        workspace_label: "",
        target_label: "Welcome → Continue without an account",
        requested_action_label: "Continue without signing in.",
        return_mode_class: ReturnModeClass::NotApplicable,
        return_origin_validation_class: ReturnOriginValidationClass::ManualResumeOnly,
        return_tenant_or_workspace_match_rule:
            ReturnTenantOrWorkspaceMatchRule::NoTenantOrWorkspaceBinding,
        return_anchor_ref: "",
        session_state: OidcSessionStateClass::AccountFreeLocalNoAuthRequired,
        sign_out_continuity: OidcSignOutContinuityClass::AccountFreeLocalPassthrough,
        local_editing_summary_label:
            "Account-free local mode keeps editing, save, undo, search, local Git, local tasks, and BYOK AI usable.",
        managed_action_narrowing_label: "No managed actions are claimed on this row.",
        managed_session_state_ref: None,
        identity_outage: OidcIdentityOutageClass::NotApplicableAccountFreeLocal,
        outage_reason_label: "Account-free local mode has no remote identity to fail.",
        recovery_action: OidcRecoveryActionClass::NoRecoveryRequired,
        recovery_action_label: "No recovery required.",
        requested_authority_scope: OidcAuthorityScopeClass::NoScopeGranted,
        granted_authority_scope: OidcAuthorityScopeClass::NoScopeGranted,
        authority_scope_summary_label:
            "Account-free local mode grants no remote scope.",
        plain_language_summary:
            "Account-free local row: no OIDC issuer, no tenant binding, no remote scope; local editing remains the source of truth.",
    }
    .stage()
}

/// Build the seeded beta page that the live shell, the headless inspector, and
/// the integration test all consume.
pub fn seeded_oidc_system_browser_beta_page() -> OidcSystemBrowserBetaPage {
    OidcSystemBrowserBetaPage::new(
        "auth:oidc_system_browser_beta:default",
        "OIDC system-browser sign-in, recovery, and session-continuity (beta)",
        "2026-05-16T00:00:00Z",
        vec![
            signed_in_active_row(),
            refresh_expired_row(),
            signed_out_row(),
            issuer_outage_row(),
            account_free_local_row(),
        ],
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page() -> OidcSystemBrowserBetaPage {
        seeded_oidc_system_browser_beta_page()
    }

    #[test]
    fn seeded_page_seeds_zero_defects_and_audits_clean() {
        let page = page();
        assert_eq!(page.summary.defect_count, 0);
        assert!(page.defects.is_empty());
        assert!(validate_oidc_system_browser_beta_page(&page).is_ok());
        assert!(page.session_continuity_preserves_local_editing());
        assert!(page.outages_degrade_truthfully());
    }

    #[test]
    fn seeded_page_includes_signed_in_active_and_recovery_rows() {
        let page = page();
        assert!(page
            .rows
            .iter()
            .any(|row| row.session_continuity.session_state_token == "signed_in_active"));
        assert!(page
            .rows
            .iter()
            .any(|row| row.session_continuity.session_state_token == "signed_out_local_intact"));
        assert!(page.rows.iter().any(|row| {
            row.session_continuity.session_state_token == "identity_outage_managed_blocked"
        }));
        assert!(page.summary.signed_out_row_count >= 1);
        assert!(page.summary.outage_or_denial_row_count >= 1);
        assert!(page.summary.claimed_enterprise_row_count >= 1);
    }

    #[test]
    fn defect_drill_public_issuer_fallback_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.session_continuity.session_state_token == "identity_outage_managed_blocked")
            .unwrap();
        row.issuer.public_issuer_fallback_used = true;
        let support_rows: Vec<OidcSystemBrowserBetaSupportRow> = page
            .rows
            .iter()
            .map(OidcSystemBrowserBetaSupportRow::from_row)
            .collect();
        let defects = audit_oidc_system_browser_beta_rows(&page.rows, &support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == OidcSystemBrowserBetaDefectKind::PublicIssuerFallbackUsed));
    }

    #[test]
    fn defect_drill_outage_widens_authority_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.session_continuity.session_state_token == "identity_outage_managed_blocked")
            .unwrap();
        row.granted_authority_scope_token =
            OidcAuthorityScopeClass::TenantAdminScope.as_str().to_owned();
        let support_rows: Vec<OidcSystemBrowserBetaSupportRow> = page
            .rows
            .iter()
            .map(OidcSystemBrowserBetaSupportRow::from_row)
            .collect();
        let defects = audit_oidc_system_browser_beta_rows(&page.rows, &support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == OidcSystemBrowserBetaDefectKind::ReturnWidensAuthorityScope));
    }

    #[test]
    fn defect_drill_sign_out_loses_local_editing_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.session_continuity.session_state_token == "signed_out_local_intact")
            .unwrap();
        row.session_continuity.local_editing_preserved = false;
        let support_rows: Vec<OidcSystemBrowserBetaSupportRow> = page
            .rows
            .iter()
            .map(OidcSystemBrowserBetaSupportRow::from_row)
            .collect();
        let defects = audit_oidc_system_browser_beta_rows(&page.rows, &support_rows);
        assert!(defects.iter().any(|d| d.defect_kind
            == OidcSystemBrowserBetaDefectKind::SignOutOrOutageLosesLocalEditing));
    }

    #[test]
    fn defect_drill_outage_missing_outage_class_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.session_continuity.session_state_token == "identity_outage_managed_blocked")
            .unwrap();
        row.identity_outage.outage_token = OidcIdentityOutageClass::NoOutage.as_str().to_owned();
        let support_rows: Vec<OidcSystemBrowserBetaSupportRow> = page
            .rows
            .iter()
            .map(OidcSystemBrowserBetaSupportRow::from_row)
            .collect();
        let defects = audit_oidc_system_browser_beta_rows(&page.rows, &support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == OidcSystemBrowserBetaDefectKind::IdentityOutageMissingClass));
    }

    #[test]
    fn defect_drill_support_row_drift_is_caught() {
        let page = page();
        let mut support_rows = page.support_rows.clone();
        support_rows[0].granted_authority_scope_token =
            OidcAuthorityScopeClass::WorkspaceAdminScope.as_str().to_owned();
        let defects = audit_oidc_system_browser_beta_rows(&page.rows, &support_rows);
        assert!(defects
            .iter()
            .any(|d| d.defect_kind == OidcSystemBrowserBetaDefectKind::SupportRowVocabularyDrift));
    }

    #[test]
    fn defect_drill_managed_session_state_ref_missing_is_caught() {
        let mut page = page();
        let row = page
            .rows
            .iter_mut()
            .find(|r| r.session_continuity.session_state_token == "signed_in_active")
            .unwrap();
        row.session_continuity.managed_session_state_ref = None;
        let support_rows: Vec<OidcSystemBrowserBetaSupportRow> = page
            .rows
            .iter()
            .map(OidcSystemBrowserBetaSupportRow::from_row)
            .collect();
        let defects = audit_oidc_system_browser_beta_rows(&page.rows, &support_rows);
        assert!(defects.iter().any(|d| d.defect_kind
            == OidcSystemBrowserBetaDefectKind::ManagedSessionStateRefMissing));
    }

    #[test]
    fn support_export_round_trips_with_zero_defects() {
        let page = page();
        let export = OidcSystemBrowserBetaSupportExport::from_page(
            "support-export:oidc-system-browser:001",
            "2026-05-16T00:00:00Z",
            page,
        );
        assert!(export.raw_private_material_excluded);
        assert!(export.defect_kinds_present.is_empty());
        assert!(export.defect_counts_by_kind.is_empty());
    }
}
