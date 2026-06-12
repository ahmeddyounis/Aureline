//! Connected-account, installation-grant, delegated-credential, and
//! effective-scope resolution on provider-linked rows.
//!
//! This module owns the typed beta contract that makes provider authority
//! honest. The connected-provider registry alpha
//! ([`crate::registry`]) froze the provider-linked surface vocabulary and the
//! approval-ticket alpha ([`crate::approval_tickets`]) froze the
//! mutation-authority lineage. The account-scope beta builds on both by
//! separating the four authority shapes Aureline can act under on a
//! provider-linked row and resolving the effective scope explicitly:
//!
//! - One [`ConnectedAccountRow`] per claimed human-account identity. The row
//!   names the workspace-bound subject, the provider host scope, the
//!   account-linked authentication source, and the lifecycle posture of the
//!   signed-in session.
//! - One [`InstallationGrantRow`] per claimed installation, app, or
//!   project-scoped grant. The row names the issuer, the bounded target
//!   scope, the grant lifecycle posture, and the optional managed-policy
//!   binding.
//! - One [`DelegatedCredentialRow`] per claimed delegated credential. The
//!   row names the delegating actor, the on-behalf-of actor, the delegated
//!   scope refs, the credential lifecycle posture, and the typed expiry
//!   horizon.
//! - One [`EffectiveScopeResolutionRow`] per provider-linked row. The
//!   resolution names the acting-identity class, the bound identity row,
//!   the requested-action class, the resolved-scope refs, the typed
//!   [`AuthorityDecisionClass`], and the typed
//!   [`GrantResolutionReasonClass`]. Resolutions that cannot proceed name
//!   the typed [`ReapprovalRouteClass`] surfaces MUST route through.
//! - One [`ScopeDriftEvent`] per observed scope drift or grant loss. The
//!   event names the originating identity row, the affected resolution row,
//!   the typed [`ScopeDriftTriggerClass`], and the typed
//!   [`AuthorityDowngradeClass`] surfaces MUST render so the resolution
//!   never silently keeps mutation authority.
//!
//! The page-level [`AccountScopeBetaPage`] folds all four record kinds into
//! one validator-checked projection over connected, mirror, offline, and
//! enterprise-managed beta profiles.
//! [`AccountScopeBetaSupportExport`] wraps the page in a redaction-safe
//! envelope: raw access tokens, raw delegated-token bodies, raw provider
//! payloads, and raw policy-injector material are excluded; identity,
//! resolution, and drift lineage are preserved verbatim so support and
//! reviewer surfaces can name which identity actually acted.
//!
//! Reviewer-facing landing page:
//! [`/docs/security/m3/provider_scope_beta.md`](../../../../docs/security/m3/provider_scope_beta.md).
//! The cross-tool boundary vocabulary lives at
//! [`/schemas/providers/effective_scope.schema.json`](../../../../schemas/providers/effective_scope.schema.json).

use std::collections::{BTreeMap, BTreeSet};

use aureline_auth::{
    secret_boundary_use_audit_result_for_health, seeded_secret_boundary_active_repair_state,
    seeded_secret_boundary_profile_parity_rows, seeded_secret_boundary_repairable_states,
    SecretBoundaryActingIdentityClass, SecretBoundaryConsumerIdentityClass,
    SecretBoundaryConsumerIdentityReceipt, SecretBoundaryCredentialMode,
    SecretBoundaryCredentialStateRow, SecretBoundaryDeclinePath,
    SecretBoundaryDelegatedCredentialRow, SecretBoundaryDelegatedUseClass,
    SecretBoundaryExportSafetyBanner, SecretBoundaryHealthStateClass,
    SecretBoundaryProjectionControl, SecretBoundaryProjectionControlClass,
    SecretBoundaryProjectionMode, SecretBoundaryProjectionModeAudit,
    SecretBoundaryRepairOwnerClass, SecretBoundarySecretAccessPrompt, SecretBoundarySecretClass,
    SecretBoundaryStorageClass, SecretBoundarySurfaceState, SecretBoundaryVaultPickerOption,
    SecretBoundaryVaultPickerState, SecretBoundaryWorkflowDependency,
    M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF,
};
use serde::{Deserialize, Serialize};

/// Beta schema version exported with every account-scope beta record.
pub const ACCOUNT_SCOPE_BETA_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every account-scope beta record.
pub const ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF: &str = "providers:account_scope_beta:v1";

const PROVIDER_SCOPE_MATRIX_ROW_ID: &str = "m5.secret.provider_model.scope_registry";

/// Source matrix ref consumed by this beta projection.
pub const ACCOUNT_SCOPE_BETA_SOURCE_MATRIX_REF: &str =
    "artifacts/security/m3/provider_scope/account_scope_matrix.yaml";

/// Stable record kind for [`AccountScopeBetaPage`] payloads.
pub const ACCOUNT_SCOPE_BETA_PAGE_RECORD_KIND: &str = "providers_account_scope_beta_page_record";

/// Stable record kind for [`ConnectedAccountRow`] payloads.
pub const ACCOUNT_SCOPE_BETA_CONNECTED_ACCOUNT_ROW_RECORD_KIND: &str =
    "providers_account_scope_beta_connected_account_row_record";

/// Stable record kind for [`InstallationGrantRow`] payloads.
pub const ACCOUNT_SCOPE_BETA_INSTALLATION_GRANT_ROW_RECORD_KIND: &str =
    "providers_account_scope_beta_installation_grant_row_record";

/// Stable record kind for [`DelegatedCredentialRow`] payloads.
pub const ACCOUNT_SCOPE_BETA_DELEGATED_CREDENTIAL_ROW_RECORD_KIND: &str =
    "providers_account_scope_beta_delegated_credential_row_record";

/// Stable record kind for [`EffectiveScopeResolutionRow`] payloads.
pub const ACCOUNT_SCOPE_BETA_EFFECTIVE_SCOPE_ROW_RECORD_KIND: &str =
    "providers_account_scope_beta_effective_scope_row_record";

/// Stable record kind for [`ScopeDriftEvent`] payloads.
pub const ACCOUNT_SCOPE_BETA_SCOPE_DRIFT_EVENT_RECORD_KIND: &str =
    "providers_account_scope_beta_scope_drift_event_record";

/// Stable record kind for [`AccountScopeBetaSummary`] payloads.
pub const ACCOUNT_SCOPE_BETA_SUMMARY_RECORD_KIND: &str =
    "providers_account_scope_beta_summary_record";

/// Stable record kind for [`AccountScopeBetaDefect`] payloads.
pub const ACCOUNT_SCOPE_BETA_DEFECT_RECORD_KIND: &str =
    "providers_account_scope_beta_defect_record";

/// Stable record kind for [`AccountScopeBetaSupportExport`] payloads.
pub const ACCOUNT_SCOPE_BETA_SUPPORT_EXPORT_RECORD_KIND: &str =
    "providers_account_scope_beta_support_export_record";

/// Beta profile under which the account-scope page is inspected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountScopeBetaProfileClass {
    /// Connected workstation reaching live providers.
    Connected,
    /// Mirror-only workstation reaching providers through a signed mirror.
    MirrorOnly,
    /// Offline workstation working from imported snapshots.
    Offline,
    /// Enterprise-managed workstation under managed-policy authority.
    EnterpriseManaged,
}

impl AccountScopeBetaProfileClass {
    /// All four beta profiles in their canonical order.
    pub const ALL: [Self; 4] = [
        Self::Connected,
        Self::MirrorOnly,
        Self::Offline,
        Self::EnterpriseManaged,
    ];

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

/// Acting-identity class bound to a provider-linked row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActingIdentityClass {
    /// Aureline acts as a signed-in human user account.
    ConnectedAccount,
    /// Aureline acts under an installation, app, or project-scoped grant.
    InstallationGrant,
    /// Aureline acts under a delegated credential held on behalf of a user.
    DelegatedCredential,
}

impl ActingIdentityClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConnectedAccount => "connected_account",
            Self::InstallationGrant => "installation_grant",
            Self::DelegatedCredential => "delegated_credential",
        }
    }
}

/// Auth source backing an acting identity. Mirrors the connected-provider
/// auth-source vocabulary, refined to the identity shapes admitted by this
/// beta page.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountAuthSourceClass {
    /// Signed-in human session (system browser).
    HumanSession,
    /// Provider installation grant.
    InstallationGrant,
    /// Delegated user credential.
    DelegatedCredential,
    /// Project-scoped provider grant.
    ProjectScopedGrant,
    /// Policy-injected service identity issued by enterprise managed authority.
    PolicyInjectedService,
}

impl AccountAuthSourceClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HumanSession => "human_session",
            Self::InstallationGrant => "installation_grant",
            Self::DelegatedCredential => "delegated_credential",
            Self::ProjectScopedGrant => "project_scoped_grant",
            Self::PolicyInjectedService => "policy_injected_service",
        }
    }
}

/// Lifecycle posture observed on a connected-account row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountLifecycleStateClass {
    /// Account session is active and within freshness floor.
    Active,
    /// Account session is reachable but requires re-auth before mutation.
    ReauthRequired,
    /// Account session was revoked or signed out at the provider.
    Revoked,
    /// Account is suspended by the provider.
    Suspended,
    /// Account is unreachable (network outage, host mismatch).
    Unreachable,
}

impl AccountLifecycleStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::ReauthRequired => "reauth_required",
            Self::Revoked => "revoked",
            Self::Suspended => "suspended",
            Self::Unreachable => "unreachable",
        }
    }

    /// True when this lifecycle state holds mutation authority closed.
    pub const fn holds_mutation_closed(self) -> bool {
        !matches!(self, Self::Active)
    }
}

/// Lifecycle posture observed on an installation grant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InstallationGrantLifecycleStateClass {
    /// Grant is installed and currently within freshness floor.
    Installed,
    /// Grant was uninstalled or removed at the provider.
    Uninstalled,
    /// Grant is suspended by the provider or by enterprise managed authority.
    Suspended,
    /// Grant scope narrowed at the provider and requires re-consent.
    ScopeNarrowed,
    /// Grant secret expired and must be rotated.
    SecretExpired,
}

impl InstallationGrantLifecycleStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Installed => "installed",
            Self::Uninstalled => "uninstalled",
            Self::Suspended => "suspended",
            Self::ScopeNarrowed => "scope_narrowed",
            Self::SecretExpired => "secret_expired",
        }
    }

    /// True when this lifecycle state holds mutation authority closed.
    pub const fn holds_mutation_closed(self) -> bool {
        !matches!(self, Self::Installed)
    }
}

/// Lifecycle posture observed on a delegated credential.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DelegatedCredentialLifecycleStateClass {
    /// Delegation is active and the credential is within freshness floor.
    Active,
    /// Delegation was revoked by the delegating actor or the provider.
    Revoked,
    /// Delegated credential expired and must be re-issued.
    Expired,
    /// Delegation scope narrowed and requires re-consent.
    ScopeNarrowed,
    /// Delegating actor lost the underlying grant the delegation chained from.
    DelegatorLostGrant,
}

impl DelegatedCredentialLifecycleStateClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Revoked => "revoked",
            Self::Expired => "expired",
            Self::ScopeNarrowed => "scope_narrowed",
            Self::DelegatorLostGrant => "delegator_lost_grant",
        }
    }

    /// True when this lifecycle state holds mutation authority closed.
    pub const fn holds_mutation_closed(self) -> bool {
        !matches!(self, Self::Active)
    }
}

/// Top-level decision class assigned to one effective-scope resolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityDecisionClass {
    /// Resolution admits the requested action with the resolved scope.
    Allowed,
    /// Resolution refuses the requested action; no mutation may proceed.
    Denied,
    /// Resolution routes through a browser handoff for completion.
    BrowserOnly,
    /// Resolution retains the action as a local draft until prerequisites
    /// align.
    LocalDraftOnly,
}

impl AuthorityDecisionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Denied => "denied",
            Self::BrowserOnly => "browser_only",
            Self::LocalDraftOnly => "local_draft_only",
        }
    }

    /// True when this decision admits a mutation authority widening.
    pub const fn admits_mutation(self) -> bool {
        matches!(self, Self::Allowed)
    }
}

/// Resolution reason carried on every effective-scope row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantResolutionReasonClass {
    /// Resolution allowed at requested mutation mode.
    Allowed,
    /// Resolution allowed with a downgrade (narrower scope, inspect-only, etc.).
    AllowedWithDowngrade,
    /// Resolution allowed only through a browser handoff.
    AllowedWithBrowserHandoff,
    /// Resolution allowed only as a deferred publish-later queue item.
    AllowedWithDeferredPublish,
    /// Required scope is missing from the actor's provider-declared grant.
    DeniedScopeMissing,
    /// Policy bundle forbids this action on this target.
    DeniedPolicyBundle,
    /// Workspace trust is restricted; trust elevation required.
    DeniedWorkspaceTrust,
    /// The acting-identity class is forbidden for this action.
    DeniedActorClassForbidden,
    /// Provider truth is past its freshness floor.
    DeniedFreshnessFloor,
    /// Underlying grant was revoked at the provider.
    DeniedRevoked,
    /// Underlying grant or credential is suspended.
    DeniedSuspended,
    /// Approval ticket is missing for a publish-now mutation.
    DeniedApprovalTicketMissing,
    /// A step-up authenticator is required before the mutation proceeds.
    DeniedStepUpRequired,
    /// Resolution failed because the actor class is unknown.
    DeniedUnknownActorClass,
}

impl GrantResolutionReasonClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::AllowedWithDowngrade => "allowed_with_downgrade",
            Self::AllowedWithBrowserHandoff => "allowed_with_browser_handoff",
            Self::AllowedWithDeferredPublish => "allowed_with_deferred_publish",
            Self::DeniedScopeMissing => "denied_scope_missing",
            Self::DeniedPolicyBundle => "denied_policy_bundle",
            Self::DeniedWorkspaceTrust => "denied_workspace_trust",
            Self::DeniedActorClassForbidden => "denied_actor_class_forbidden",
            Self::DeniedFreshnessFloor => "denied_freshness_floor",
            Self::DeniedRevoked => "denied_revoked",
            Self::DeniedSuspended => "denied_suspended",
            Self::DeniedApprovalTicketMissing => "denied_approval_ticket_missing",
            Self::DeniedStepUpRequired => "denied_step_up_required",
            Self::DeniedUnknownActorClass => "denied_unknown_actor_class",
        }
    }

    /// True when this reason can pair with [`AuthorityDecisionClass::Allowed`].
    pub const fn is_allowed_family(self) -> bool {
        matches!(
            self,
            Self::Allowed
                | Self::AllowedWithDowngrade
                | Self::AllowedWithBrowserHandoff
                | Self::AllowedWithDeferredPublish
        )
    }
}

/// Class of requested action a resolution adjudicates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequestedActionClass {
    /// Read-only inspection of provider state.
    ReadOnlyInspection,
    /// Human-authored comment on a provider object.
    HumanAuthoredComment,
    /// Review-decision publish (approve/merge/close).
    ReviewDecisionPublish,
    /// Mutation of an issue or work item.
    IssueOrWorkItemMutation,
    /// CI run or check mutation.
    CiRunOrCheckMutation,
    /// Release-publish or package-publish action.
    ReleasePublish,
    /// Credential projection through the broker.
    CredentialProjection,
}

impl RequestedActionClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyInspection => "read_only_inspection",
            Self::HumanAuthoredComment => "human_authored_comment",
            Self::ReviewDecisionPublish => "review_decision_publish",
            Self::IssueOrWorkItemMutation => "issue_or_work_item_mutation",
            Self::CiRunOrCheckMutation => "ci_run_or_check_mutation",
            Self::ReleasePublish => "release_publish",
            Self::CredentialProjection => "credential_projection",
        }
    }

    /// True when this action class proposes a mutation on provider state.
    pub const fn proposes_mutation(self) -> bool {
        !matches!(self, Self::ReadOnlyInspection)
    }
}

/// Typed reapproval route a non-allowed resolution names so surfaces never
/// silently keep mutation authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReapprovalRouteClass {
    /// No reapproval is needed (resolution allowed at the requested mode).
    NoneRequired,
    /// Route the user to system-browser reauth before retrying.
    SystemBrowserReauth,
    /// Route the user to step-up authenticator (passkey/MFA).
    StepUpAuthenticator,
    /// Route the user to account reselection before retrying.
    AccountReselection,
    /// Route the user to installation-grant reconsent.
    InstallationGrantReconsent,
    /// Route the user to delegated-credential re-issue.
    DelegatedCredentialReissue,
    /// Route the user to a browser handoff to complete the action.
    BrowserHandoff,
    /// Route the action into the publish-later queue until prerequisites
    /// align.
    PublishLaterDeferred,
    /// Route the user to admin review or a workspace-trust grant.
    AdminReviewOrTrustGrant,
}

impl ReapprovalRouteClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneRequired => "none_required",
            Self::SystemBrowserReauth => "system_browser_reauth",
            Self::StepUpAuthenticator => "step_up_authenticator",
            Self::AccountReselection => "account_reselection",
            Self::InstallationGrantReconsent => "installation_grant_reconsent",
            Self::DelegatedCredentialReissue => "delegated_credential_reissue",
            Self::BrowserHandoff => "browser_handoff",
            Self::PublishLaterDeferred => "publish_later_deferred",
            Self::AdminReviewOrTrustGrant => "admin_review_or_trust_grant",
        }
    }
}

/// Typed trigger observed for a scope-drift event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeDriftTriggerClass {
    /// Underlying grant was revoked at the provider.
    GrantRevoked,
    /// Underlying grant or credential was suspended.
    GrantSuspended,
    /// Provider-declared scopes narrowed.
    ScopeNarrowed,
    /// Delegated credential expired.
    DelegatedCredentialExpired,
    /// Installation grant secret expired or was rotated out.
    InstallationSecretExpired,
    /// Acting-identity class changed (e.g., human session swapped under us).
    ActorClassChanged,
    /// Host mismatch detected between cached identity and current target.
    HostMismatchDetected,
    /// Tenant or org membership changed under the actor.
    TenantOrOrgMembershipChanged,
    /// Policy epoch rolled and invalidated cached scope.
    PolicyEpochRolled,
    /// Workspace trust was downgraded under the actor.
    TrustStateDowngraded,
    /// Freshness floor drifted past the cached observation.
    FreshnessFloorDrifted,
}

impl ScopeDriftTriggerClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::GrantRevoked => "grant_revoked",
            Self::GrantSuspended => "grant_suspended",
            Self::ScopeNarrowed => "scope_narrowed",
            Self::DelegatedCredentialExpired => "delegated_credential_expired",
            Self::InstallationSecretExpired => "installation_secret_expired",
            Self::ActorClassChanged => "actor_class_changed",
            Self::HostMismatchDetected => "host_mismatch_detected",
            Self::TenantOrOrgMembershipChanged => "tenant_or_org_membership_changed",
            Self::PolicyEpochRolled => "policy_epoch_rolled",
            Self::TrustStateDowngraded => "trust_state_downgraded",
            Self::FreshnessFloorDrifted => "freshness_floor_drifted",
        }
    }
}

/// Typed downgrade action a scope-drift event forces.
///
/// `NoDowngradeRequired` is admitted only for benign refresh events; every
/// other trigger must force a visible downgrade so the resolution never
/// silently keeps mutation authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityDowngradeClass {
    /// No downgrade required; the underlying resolution remains valid.
    NoDowngradeRequired,
    /// Force the resolution to inspect-only until repair completes.
    ForceInspectOnly,
    /// Force the resolution to local-draft only until repair completes.
    ForceLocalDraftOnly,
    /// Force the resolution to browser-handoff only until repair completes.
    ForceBrowserHandoffOnly,
    /// Force the user through a step-up authenticator before the next action.
    ForceStepUpAuthenticator,
    /// Force account reselection before the next action.
    ForceAccountReselection,
    /// Force admin review or a workspace-trust grant before the next action.
    ForceAdminReview,
    /// Force disconnect of the underlying identity until repair completes.
    ForceDisconnectUntilRepair,
}

impl AuthorityDowngradeClass {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDowngradeRequired => "no_downgrade_required",
            Self::ForceInspectOnly => "force_inspect_only",
            Self::ForceLocalDraftOnly => "force_local_draft_only",
            Self::ForceBrowserHandoffOnly => "force_browser_handoff_only",
            Self::ForceStepUpAuthenticator => "force_step_up_authenticator",
            Self::ForceAccountReselection => "force_account_reselection",
            Self::ForceAdminReview => "force_admin_review",
            Self::ForceDisconnectUntilRepair => "force_disconnect_until_repair",
        }
    }
}

/// Provider-host binding shared by every identity row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderHostBinding {
    /// Opaque canonical-host ref (e.g. `provider-host:github:enterprise`).
    pub canonical_host_ref: String,
    /// Opaque tenant, org, or project scope ref.
    pub tenant_or_org_scope_ref: String,
    /// Reviewable host label safe for support export.
    pub host_label: String,
}

/// Subject identity carried on a connected-account row. The subject is
/// always opaque and never carries raw provider tokens or raw subject ids.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectedAccountSubject {
    /// Opaque workspace-bound subject ref.
    pub subject_ref: String,
    /// Reviewable subject label safe for support export.
    pub subject_label: String,
    /// Opaque capability-hash ref bound to the subject.
    pub capability_hash_ref: String,
}

/// One claimed connected-account identity row.
///
/// A connected-account row represents Aureline acting as a signed-in human
/// user. The row carries the workspace-bound subject, the provider host
/// binding, the auth source, and the lifecycle posture, but never carries
/// raw access tokens, raw refresh tokens, or raw provider-side identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectedAccountRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque identity row id.
    pub identity_row_id: String,
    /// Reviewable row label safe for UI and support export.
    pub display_label: String,
    /// Profile under which the row is inspected.
    pub profile: AccountScopeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Acting identity class. Always [`ActingIdentityClass::ConnectedAccount`]
    /// for this row kind.
    pub acting_identity_class: ActingIdentityClass,
    /// Stable token for [`Self::acting_identity_class`].
    pub acting_identity_class_token: String,
    /// Subject identity bound to the row.
    pub subject: ConnectedAccountSubject,
    /// Provider-host binding the account belongs to.
    pub provider_host: ProviderHostBinding,
    /// Auth source backing the connected account.
    pub auth_source: AccountAuthSourceClass,
    /// Stable token for [`Self::auth_source`].
    pub auth_source_token: String,
    /// Lifecycle posture observed for the account session.
    pub lifecycle_state: AccountLifecycleStateClass,
    /// Stable token for [`Self::lifecycle_state`].
    pub lifecycle_state_token: String,
    /// Export-safe explanation of the lifecycle posture.
    pub lifecycle_note: String,
    /// Timestamp at which the lifecycle posture was observed.
    pub observed_at: String,
    /// Optional timestamp at which the session expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Beta guardrail: raw access-token material is not present on the row.
    pub raw_token_material_present: bool,
    /// Beta guardrail: row never offers a silent public-endpoint fallback.
    pub public_endpoint_fallback_offered: bool,
    /// Beta guardrail: local editing is preserved through this lifecycle state.
    pub local_editing_preserved: bool,
}

/// One claimed installation-grant identity row.
///
/// An installation-grant row represents Aureline acting under an installation,
/// app, or project-scoped grant minted by the provider. The row carries the
/// issuer, the bounded target scope refs, the lifecycle posture, and the
/// optional managed-policy binding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InstallationGrantRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque identity row id.
    pub identity_row_id: String,
    /// Reviewable row label safe for UI and support export.
    pub display_label: String,
    /// Profile under which the row is inspected.
    pub profile: AccountScopeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Acting identity class. Always
    /// [`ActingIdentityClass::InstallationGrant`] for this row kind.
    pub acting_identity_class: ActingIdentityClass,
    /// Stable token for [`Self::acting_identity_class`].
    pub acting_identity_class_token: String,
    /// Opaque grant id ref.
    pub installation_grant_ref: String,
    /// Reviewable grant label safe for UI and support export.
    pub grant_label: String,
    /// Opaque issuer ref (managed authority, provider, project).
    pub issuer_ref: String,
    /// Reviewable issuer label.
    pub issuer_label: String,
    /// Provider-host binding the grant belongs to.
    pub provider_host: ProviderHostBinding,
    /// Bounded target scope refs the grant covers.
    pub bounded_target_scope_refs: Vec<String>,
    /// Auth source backing the installation grant.
    pub auth_source: AccountAuthSourceClass,
    /// Stable token for [`Self::auth_source`].
    pub auth_source_token: String,
    /// Lifecycle posture observed for the grant.
    pub lifecycle_state: InstallationGrantLifecycleStateClass,
    /// Stable token for [`Self::lifecycle_state`].
    pub lifecycle_state_token: String,
    /// Export-safe explanation of the lifecycle posture.
    pub lifecycle_note: String,
    /// Optional opaque managed-policy bundle ref bound to the grant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub managed_policy_bundle_ref: Option<String>,
    /// Timestamp at which the lifecycle posture was observed.
    pub observed_at: String,
    /// Optional timestamp at which the grant secret expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret_expires_at: Option<String>,
    /// Beta guardrail: raw installation-secret material is not present.
    pub raw_token_material_present: bool,
    /// Beta guardrail: row never offers a silent public-endpoint fallback.
    pub public_endpoint_fallback_offered: bool,
    /// Beta guardrail: local editing is preserved through this lifecycle state.
    pub local_editing_preserved: bool,
}

/// One claimed delegated-credential identity row.
///
/// A delegated-credential row represents Aureline acting through a delegated
/// credential held on behalf of a user. The row carries the delegating actor
/// (who chained the delegation), the on-behalf-of actor (who the credential
/// is acting for), the delegated scope refs, the lifecycle posture, and the
/// expiry horizon.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DelegatedCredentialRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque identity row id.
    pub identity_row_id: String,
    /// Reviewable row label safe for UI and support export.
    pub display_label: String,
    /// Profile under which the row is inspected.
    pub profile: AccountScopeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Acting identity class. Always
    /// [`ActingIdentityClass::DelegatedCredential`] for this row kind.
    pub acting_identity_class: ActingIdentityClass,
    /// Stable token for [`Self::acting_identity_class`].
    pub acting_identity_class_token: String,
    /// Opaque delegation ref.
    pub delegation_ref: String,
    /// Opaque delegating-actor ref (the actor that chained the delegation).
    pub delegator_actor_ref: String,
    /// Reviewable delegating-actor label.
    pub delegator_label: String,
    /// Opaque on-behalf-of-actor ref (the actor the credential acts for).
    pub on_behalf_of_actor_ref: String,
    /// Reviewable on-behalf-of-actor label.
    pub on_behalf_of_label: String,
    /// Provider-host binding the delegated credential belongs to.
    pub provider_host: ProviderHostBinding,
    /// Delegated scope refs. The credential is bounded to this set.
    pub delegated_scope_refs: Vec<String>,
    /// Auth source backing the delegated credential.
    pub auth_source: AccountAuthSourceClass,
    /// Stable token for [`Self::auth_source`].
    pub auth_source_token: String,
    /// Lifecycle posture observed for the credential.
    pub lifecycle_state: DelegatedCredentialLifecycleStateClass,
    /// Stable token for [`Self::lifecycle_state`].
    pub lifecycle_state_token: String,
    /// Export-safe explanation of the lifecycle posture.
    pub lifecycle_note: String,
    /// Timestamp at which the lifecycle posture was observed.
    pub observed_at: String,
    /// Optional timestamp at which the credential expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Beta guardrail: raw delegated-token material is not present.
    pub raw_token_material_present: bool,
    /// Beta guardrail: row never offers a silent public-endpoint fallback.
    pub public_endpoint_fallback_offered: bool,
    /// Beta guardrail: local editing is preserved through this lifecycle state.
    pub local_editing_preserved: bool,
}

/// Target identity for a provider-linked row that an effective-scope
/// resolution adjudicates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderTargetIdentity {
    /// Opaque target ref.
    pub target_ref: String,
    /// Reviewable target label.
    pub target_label: String,
    /// Opaque provider-linked row ref the resolution binds to.
    pub provider_linked_row_ref: String,
}

/// One effective-scope resolution on a provider-linked row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveScopeResolutionRow {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque resolution id.
    pub resolution_id: String,
    /// Reviewable resolution label safe for UI and support export.
    pub display_label: String,
    /// Profile under which the resolution is inspected.
    pub profile: AccountScopeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Acting identity class assigned to this resolution.
    pub acting_identity_class: ActingIdentityClass,
    /// Stable token for [`Self::acting_identity_class`].
    pub acting_identity_class_token: String,
    /// Identity row id this resolution binds to. Must reference one of the
    /// page's identity rows of the same class.
    pub bound_identity_row_ref: String,
    /// Requested action class the resolution adjudicates.
    pub requested_action: RequestedActionClass,
    /// Stable token for [`Self::requested_action`].
    pub requested_action_token: String,
    /// Target the resolution binds to.
    pub target: ProviderTargetIdentity,
    /// Provider-declared scope refs currently held by the bound identity.
    pub provider_declared_scope_refs: Vec<String>,
    /// Resolved scope refs (intersection of provider-declared scope and
    /// policy locks) the resolution will act under.
    pub resolved_scope_refs: Vec<String>,
    /// Typed decision class.
    pub decision: AuthorityDecisionClass,
    /// Stable token for [`Self::decision`].
    pub decision_token: String,
    /// Typed resolution reason.
    pub resolution_reason: GrantResolutionReasonClass,
    /// Stable token for [`Self::resolution_reason`].
    pub resolution_reason_token: String,
    /// Reviewable decision summary safe for UI and support export.
    pub decision_summary: String,
    /// Typed reapproval route for non-allowed decisions.
    pub reapproval_route: ReapprovalRouteClass,
    /// Stable token for [`Self::reapproval_route`].
    pub reapproval_route_token: String,
    /// Reviewable reapproval-route label safe for UI.
    pub reapproval_route_label: String,
    /// Optional opaque ref to a remediation path that completes the
    /// reapproval (system browser handoff, admin review, etc.).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remediation_path_ref: Option<String>,
    /// Optional opaque ref to the approval-ticket required for publish-now
    /// resolutions. Required when `decision` is `Allowed` and the requested
    /// action proposes mutation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// Optional opaque ref to the browser-handoff packet required for
    /// browser-only resolutions.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Optional opaque ref to the publish-later queue item the local-draft
    /// resolution is parked under.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_later_queue_item_ref: Option<String>,
    /// Timestamp at which the resolution was computed.
    pub computed_at: String,
    /// Optional timestamp at which the resolution expires.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Beta guardrail: raw access-token material is not present.
    pub raw_token_material_present: bool,
    /// Beta guardrail: resolution did not silently fall back to a public
    /// endpoint.
    pub public_endpoint_fallback_taken: bool,
    /// Beta guardrail: resolution did not silently widen mutation authority
    /// beyond the resolved scope refs.
    pub silent_mutation_authority_widened: bool,
    /// Beta guardrail: local editing is preserved through this resolution.
    pub local_editing_preserved: bool,
}

/// One scope-drift event on a previously resolved provider-linked row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeDriftEvent {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable opaque event id.
    pub event_id: String,
    /// Profile under which the event is inspected.
    pub profile: AccountScopeBetaProfileClass,
    /// Stable token for [`Self::profile`].
    pub profile_token: String,
    /// Identity row id the drift originates from.
    pub originating_identity_row_ref: String,
    /// Resolution row id affected by the drift.
    pub affected_resolution_row_ref: String,
    /// Typed drift trigger.
    pub trigger: ScopeDriftTriggerClass,
    /// Stable token for [`Self::trigger`].
    pub trigger_token: String,
    /// Typed forced downgrade.
    pub forced_downgrade: AuthorityDowngradeClass,
    /// Stable token for [`Self::forced_downgrade`].
    pub forced_downgrade_token: String,
    /// Typed reapproval route surfaces MUST render after the downgrade.
    pub reapproval_route: ReapprovalRouteClass,
    /// Stable token for [`Self::reapproval_route`].
    pub reapproval_route_token: String,
    /// Export-safe rationale for the downgrade.
    pub rationale_summary: String,
    /// Timestamp at which the drift was observed.
    pub observed_at: String,
    /// Beta guardrail: drift event did not silently keep mutation authority.
    pub silent_mutation_authority_retained: bool,
    /// Beta guardrail: local editing is preserved through the drift event.
    pub local_editing_preserved: bool,
}

/// Defect-kind vocabulary surfaced by the beta validator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccountScopeBetaDefectKind {
    /// Identity row claims raw token material is present.
    RawTokenMaterialPresent,
    /// Row offered an undeclared public-endpoint fallback.
    PublicEndpointFallbackOffered,
    /// Resolution silently took a public-endpoint fallback.
    PublicEndpointFallbackTaken,
    /// Resolution silently widened mutation authority beyond resolved scope.
    SilentMutationAuthorityWidened,
    /// Drift event silently kept mutation authority after a drift.
    SilentMutationAuthorityRetainedAfterDrift,
    /// Local editing was not preserved through the row.
    LocalEditingNotPreserved,
    /// Identity row carries a class token that does not match its row kind.
    IdentityClassMismatch,
    /// Resolution row binds an identity ref that is not present on the page.
    BoundIdentityRefUnknown,
    /// Resolution row binds an identity whose class does not match
    /// `acting_identity_class`.
    BoundIdentityClassMismatch,
    /// Resolution binds a bound identity whose lifecycle holds mutation
    /// closed but the decision is `Allowed`.
    AllowedDecisionOnClosedLifecycle,
    /// Resolution decision is `Allowed` paired with a non-allowed-family
    /// resolution reason.
    AllowedDecisionWithNonAllowedReason,
    /// Resolution decision is `Denied`/`BrowserOnly`/`LocalDraftOnly` paired
    /// with an allowed-family resolution reason.
    NonAllowedDecisionWithAllowedReason,
    /// Resolution decision is `BrowserOnly` without a browser-handoff packet
    /// ref.
    BrowserOnlyWithoutHandoffRef,
    /// Resolution decision is `LocalDraftOnly` without a publish-later queue
    /// item ref.
    LocalDraftOnlyWithoutQueueRef,
    /// Resolution that proposes a mutation is `Allowed` without an approval
    /// ticket ref.
    AllowedMutationWithoutApprovalTicket,
    /// Resolution decision is not allowed but `reapproval_route` is
    /// `NoneRequired`.
    NonAllowedDecisionMissingReapprovalRoute,
    /// Resolved scope refs are not a subset of provider-declared scope refs.
    ResolvedScopeNotSubsetOfDeclared,
    /// Allowed resolution did not name any resolved scope refs.
    AllowedResolutionWithoutResolvedScope,
    /// Drift event references an identity row that is not present on the
    /// page.
    DriftEventIdentityRefUnknown,
    /// Drift event references a resolution row that is not present on the
    /// page.
    DriftEventResolutionRefUnknown,
    /// Drift event has a non-benign trigger paired with
    /// `NoDowngradeRequired`.
    DriftEventDowngradeMissing,
    /// Drift event names a downgrade but `reapproval_route` is
    /// `NoneRequired`.
    DriftEventReapprovalRouteMissing,
    /// One of the four required beta profiles has no claimed row.
    ProfileCoverageMissing,
    /// `profile_token` did not match `profile`.
    ProfileTokenDrift,
    /// `acting_identity_class_token` did not match `acting_identity_class`.
    ActingIdentityClassTokenDrift,
    /// `auth_source_token` did not match `auth_source`.
    AuthSourceTokenDrift,
    /// `lifecycle_state_token` did not match `lifecycle_state`.
    LifecycleStateTokenDrift,
    /// `decision_token` did not match `decision`.
    DecisionTokenDrift,
    /// `resolution_reason_token` did not match `resolution_reason`.
    ResolutionReasonTokenDrift,
    /// `requested_action_token` did not match `requested_action`.
    RequestedActionTokenDrift,
    /// `reapproval_route_token` did not match `reapproval_route`.
    ReapprovalRouteTokenDrift,
    /// `trigger_token` did not match `trigger`.
    DriftTriggerTokenDrift,
    /// `forced_downgrade_token` did not match `forced_downgrade`.
    DriftDowngradeTokenDrift,
}

impl AccountScopeBetaDefectKind {
    /// Stable token recorded on records.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RawTokenMaterialPresent => "raw_token_material_present",
            Self::PublicEndpointFallbackOffered => "public_endpoint_fallback_offered",
            Self::PublicEndpointFallbackTaken => "public_endpoint_fallback_taken",
            Self::SilentMutationAuthorityWidened => "silent_mutation_authority_widened",
            Self::SilentMutationAuthorityRetainedAfterDrift => {
                "silent_mutation_authority_retained_after_drift"
            }
            Self::LocalEditingNotPreserved => "local_editing_not_preserved",
            Self::IdentityClassMismatch => "identity_class_mismatch",
            Self::BoundIdentityRefUnknown => "bound_identity_ref_unknown",
            Self::BoundIdentityClassMismatch => "bound_identity_class_mismatch",
            Self::AllowedDecisionOnClosedLifecycle => "allowed_decision_on_closed_lifecycle",
            Self::AllowedDecisionWithNonAllowedReason => "allowed_decision_with_non_allowed_reason",
            Self::NonAllowedDecisionWithAllowedReason => "non_allowed_decision_with_allowed_reason",
            Self::BrowserOnlyWithoutHandoffRef => "browser_only_without_handoff_ref",
            Self::LocalDraftOnlyWithoutQueueRef => "local_draft_only_without_queue_ref",
            Self::AllowedMutationWithoutApprovalTicket => {
                "allowed_mutation_without_approval_ticket"
            }
            Self::NonAllowedDecisionMissingReapprovalRoute => {
                "non_allowed_decision_missing_reapproval_route"
            }
            Self::ResolvedScopeNotSubsetOfDeclared => "resolved_scope_not_subset_of_declared",
            Self::AllowedResolutionWithoutResolvedScope => {
                "allowed_resolution_without_resolved_scope"
            }
            Self::DriftEventIdentityRefUnknown => "drift_event_identity_ref_unknown",
            Self::DriftEventResolutionRefUnknown => "drift_event_resolution_ref_unknown",
            Self::DriftEventDowngradeMissing => "drift_event_downgrade_missing",
            Self::DriftEventReapprovalRouteMissing => "drift_event_reapproval_route_missing",
            Self::ProfileCoverageMissing => "profile_coverage_missing",
            Self::ProfileTokenDrift => "profile_token_drift",
            Self::ActingIdentityClassTokenDrift => "acting_identity_class_token_drift",
            Self::AuthSourceTokenDrift => "auth_source_token_drift",
            Self::LifecycleStateTokenDrift => "lifecycle_state_token_drift",
            Self::DecisionTokenDrift => "decision_token_drift",
            Self::ResolutionReasonTokenDrift => "resolution_reason_token_drift",
            Self::RequestedActionTokenDrift => "requested_action_token_drift",
            Self::ReapprovalRouteTokenDrift => "reapproval_route_token_drift",
            Self::DriftTriggerTokenDrift => "drift_trigger_token_drift",
            Self::DriftDowngradeTokenDrift => "drift_downgrade_token_drift",
        }
    }
}

/// Typed validation defect for the account-scope beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountScopeBetaDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: AccountScopeBetaDefectKind,
    /// Stable token for [`Self::defect_kind`].
    pub defect_kind_token: String,
    /// Subject id (row id, resolution id, event id, or `"page"`).
    pub subject_id: String,
    /// Field that failed validation.
    pub field: String,
    /// Export-safe explanation.
    pub note: String,
}

impl AccountScopeBetaDefect {
    fn new(
        defect_kind: AccountScopeBetaDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: ACCOUNT_SCOPE_BETA_DEFECT_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Aggregate summary for the account-scope beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountScopeBetaSummary {
    /// Stable record kind of the parent page.
    pub page_record_kind: String,
    /// Stable record kind of the summary.
    pub record_kind: String,
    /// Number of connected-account rows.
    pub connected_account_row_count: usize,
    /// Number of installation-grant rows.
    pub installation_grant_row_count: usize,
    /// Number of delegated-credential rows.
    pub delegated_credential_row_count: usize,
    /// Number of effective-scope resolution rows.
    pub resolution_row_count: usize,
    /// Number of scope-drift events.
    pub scope_drift_event_count: usize,
    /// Profile tokens present across the page.
    pub profiles_present: Vec<String>,
    /// Acting-identity class tokens present across resolution rows.
    pub identity_classes_present: Vec<String>,
    /// Decision tokens present across resolution rows.
    pub decisions_present: Vec<String>,
    /// Resolution-reason tokens present across resolution rows.
    pub resolution_reasons_present: Vec<String>,
    /// Drift-trigger tokens present across drift events.
    pub drift_triggers_present: Vec<String>,
    /// Forced-downgrade tokens present across drift events.
    pub forced_downgrades_present: Vec<String>,
    /// Counts of resolutions by decision token.
    pub resolutions_by_decision: BTreeMap<String, usize>,
    /// Counts of drift events by trigger token.
    pub drift_events_by_trigger: BTreeMap<String, usize>,
    /// Number of defects.
    pub defect_count: usize,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl AccountScopeBetaSummary {
    /// Builds the summary from rows, resolutions, drift events, and defects.
    pub fn from_records(
        connected_accounts: &[ConnectedAccountRow],
        installation_grants: &[InstallationGrantRow],
        delegated_credentials: &[DelegatedCredentialRow],
        resolutions: &[EffectiveScopeResolutionRow],
        drift_events: &[ScopeDriftEvent],
        defects: &[AccountScopeBetaDefect],
    ) -> Self {
        let mut profiles_present: BTreeSet<String> = BTreeSet::new();
        let mut identity_classes_present: BTreeSet<String> = BTreeSet::new();
        let mut decisions_present: BTreeSet<String> = BTreeSet::new();
        let mut resolution_reasons_present: BTreeSet<String> = BTreeSet::new();
        let mut drift_triggers_present: BTreeSet<String> = BTreeSet::new();
        let mut forced_downgrades_present: BTreeSet<String> = BTreeSet::new();

        for row in connected_accounts {
            profiles_present.insert(row.profile_token.clone());
        }
        for row in installation_grants {
            profiles_present.insert(row.profile_token.clone());
        }
        for row in delegated_credentials {
            profiles_present.insert(row.profile_token.clone());
        }
        for row in resolutions {
            profiles_present.insert(row.profile_token.clone());
            identity_classes_present.insert(row.acting_identity_class_token.clone());
            decisions_present.insert(row.decision_token.clone());
            resolution_reasons_present.insert(row.resolution_reason_token.clone());
        }
        for event in drift_events {
            profiles_present.insert(event.profile_token.clone());
            drift_triggers_present.insert(event.trigger_token.clone());
            forced_downgrades_present.insert(event.forced_downgrade_token.clone());
        }

        let mut resolutions_by_decision: BTreeMap<String, usize> = BTreeMap::new();
        for row in resolutions {
            *resolutions_by_decision
                .entry(row.decision_token.clone())
                .or_insert(0) += 1;
        }
        let mut drift_events_by_trigger: BTreeMap<String, usize> = BTreeMap::new();
        for event in drift_events {
            *drift_events_by_trigger
                .entry(event.trigger_token.clone())
                .or_insert(0) += 1;
        }
        let mut defect_counts_by_kind: BTreeMap<String, usize> = BTreeMap::new();
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind_token.clone())
                .or_insert(0) += 1;
        }

        Self {
            page_record_kind: ACCOUNT_SCOPE_BETA_PAGE_RECORD_KIND.to_owned(),
            record_kind: ACCOUNT_SCOPE_BETA_SUMMARY_RECORD_KIND.to_owned(),
            connected_account_row_count: connected_accounts.len(),
            installation_grant_row_count: installation_grants.len(),
            delegated_credential_row_count: delegated_credentials.len(),
            resolution_row_count: resolutions.len(),
            scope_drift_event_count: drift_events.len(),
            profiles_present: profiles_present.into_iter().collect(),
            identity_classes_present: identity_classes_present.into_iter().collect(),
            decisions_present: decisions_present.into_iter().collect(),
            resolution_reasons_present: resolution_reasons_present.into_iter().collect(),
            drift_triggers_present: drift_triggers_present.into_iter().collect(),
            forced_downgrades_present: forced_downgrades_present.into_iter().collect(),
            resolutions_by_decision,
            drift_events_by_trigger,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Top-level account-scope beta page consumed by admin, support, shell, and
/// reviewer fixtures.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountScopeBetaPage {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Source matrix ref.
    pub source_matrix_ref: String,
    /// Claimed connected-account identity rows.
    pub connected_account_rows: Vec<ConnectedAccountRow>,
    /// Claimed installation-grant identity rows.
    pub installation_grant_rows: Vec<InstallationGrantRow>,
    /// Claimed delegated-credential identity rows.
    pub delegated_credential_rows: Vec<DelegatedCredentialRow>,
    /// Claimed effective-scope resolution rows.
    pub resolution_rows: Vec<EffectiveScopeResolutionRow>,
    /// Claimed scope-drift events.
    pub scope_drift_events: Vec<ScopeDriftEvent>,
    /// Typed validation defects.
    pub defects: Vec<AccountScopeBetaDefect>,
    /// Aggregate summary.
    pub summary: AccountScopeBetaSummary,
}

impl AccountScopeBetaPage {
    /// Projects the shared M5 secret-boundary state for the provider scope
    /// registry and delegated-identity surface.
    pub fn secret_boundary_states(&self) -> Vec<SecretBoundarySurfaceState> {
        let delegated = self.delegated_credential_rows.first();
        let installation = self.installation_grant_rows.first();
        let connected = self.connected_account_rows.first();
        let resolution = self.resolution_rows.first();

        let (
            display_label,
            auth_source,
            target_label,
            expires_at,
            policy_owner_label,
            health_state,
        ) = if let Some(row) = delegated {
            (
                row.display_label.clone(),
                row.auth_source,
                row.provider_host.host_label.clone(),
                row.expires_at.clone(),
                row.delegator_label.clone(),
                delegated_health_state(row.lifecycle_state),
            )
        } else if let Some(row) = installation {
            (
                row.display_label.clone(),
                row.auth_source,
                row.provider_host.host_label.clone(),
                row.secret_expires_at.clone(),
                row.issuer_label.clone(),
                installation_health_state(row.lifecycle_state),
            )
        } else if let Some(row) = connected {
            (
                row.display_label.clone(),
                row.auth_source,
                row.provider_host.host_label.clone(),
                row.expires_at.clone(),
                row.subject.subject_label.clone(),
                connected_health_state(row.lifecycle_state),
            )
        } else {
            return Vec::new();
        };

        let credential_mode = account_scope_credential_mode(auth_source);
        let storage_class = account_scope_storage_class(auth_source);
        let projection_mode = account_scope_projection_mode(auth_source);
        let secret_class = account_scope_secret_class(auth_source);
        let actor_identity =
            account_scope_actor_identity(delegated.is_some(), installation.is_some());
        let consumer_identity = SecretBoundaryConsumerIdentityClass::ServiceIssuedDelegate;
        let delegated_use_class = if delegated.is_some() {
            SecretBoundaryDelegatedUseClass::ServiceIssuedDelegatedIdentity
        } else if installation.is_some() {
            SecretBoundaryDelegatedUseClass::RemoteVaultFetch
        } else {
            SecretBoundaryDelegatedUseClass::LocalSecretHandle
        };
        let decline_summary = match resolution.map(|row| row.decision) {
            Some(AuthorityDecisionClass::Allowed) => {
                "Declining keeps local draft and scope inspection available while live mutation authority stays closed."
            }
            _ => {
                "Declining keeps scope inspection, drift review, and local draft fallback available."
            }
        };
        let decline_path = SecretBoundaryDeclinePath {
            decline_label: "Continue with local draft".to_owned(),
            still_works_summary: decline_summary.to_owned(),
        };
        let workflows = vec![
            account_scope_workflow("workflow:provider.scope.inspect", "Inspect provider scope"),
            account_scope_workflow(
                "workflow:provider.scope.repair",
                "Repair scope or delegated identity",
            ),
        ];
        let projection_controls = account_scope_projection_controls(
            PROVIDER_SCOPE_MATRIX_ROW_ID,
            delegated.is_some(),
            installation.is_some(),
        );
        let audit_result = secret_boundary_use_audit_result_for_health(health_state);

        vec![SecretBoundarySurfaceState {
            matrix_row_id: PROVIDER_SCOPE_MATRIX_ROW_ID.to_owned(),
            vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
            secret_access_prompt: SecretBoundarySecretAccessPrompt {
                matrix_row_id: PROVIDER_SCOPE_MATRIX_ROW_ID.to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                requester_label: "Provider scope registry".to_owned(),
                secret_class,
                target_workflow_label: display_label.clone(),
                storage_class,
                credential_mode,
                projection_mode,
                lifetime_label: "Delegated or provider-scoped authority".to_owned(),
                expires_at: expires_at.clone(),
                dependent_workflows: workflows.clone(),
                decline_path: decline_path.clone(),
            },
            credential_state_row: SecretBoundaryCredentialStateRow {
                matrix_row_id: PROVIDER_SCOPE_MATRIX_ROW_ID.to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                display_label: "Provider scope credential state".to_owned(),
                secret_class,
                source_class: credential_mode,
                target_boundary_label: target_label.clone(),
                storage_class,
                projection_mode,
                health_state,
                expires_at: expires_at.clone(),
                rotate_action_label: "Rotate or reissue scoped auth".to_owned(),
                revoke_action_label: "Revoke provider scope auth".to_owned(),
                test_action_label: "Test provider scope".to_owned(),
                dependent_workflows: workflows,
                decline_path,
            },
            vault_picker: Some(account_scope_picker_state()),
            delegated_credential_row: Some(SecretBoundaryDelegatedCredentialRow {
                matrix_row_id: PROVIDER_SCOPE_MATRIX_ROW_ID.to_owned(),
                vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
                delegated_use_class,
                target_host_or_workspace_label: display_label,
                expires_at,
                policy_owner_label,
                projection_controls: projection_controls.clone(),
            }),
            consumer_identity_receipt: SecretBoundaryConsumerIdentityReceipt::new(
                format!("{PROVIDER_SCOPE_MATRIX_ROW_ID}:consumer-receipt"),
                PROVIDER_SCOPE_MATRIX_ROW_ID,
                actor_identity,
                consumer_identity,
                "Provider scope registry",
                target_label.clone(),
                credential_mode,
                projection_mode,
                storage_class,
                audit_result,
            ),
            projection_mode_audit: SecretBoundaryProjectionModeAudit::new(
                format!("{PROVIDER_SCOPE_MATRIX_ROW_ID}:projection-audit"),
                PROVIDER_SCOPE_MATRIX_ROW_ID,
                actor_identity,
                consumer_identity,
                "Provider scope registry",
                target_label,
                projection_mode,
                audit_result,
                SecretBoundaryRepairOwnerClass::ProviderOperator,
                projection_controls
                    .iter()
                    .map(|control| control.control_class)
                    .collect(),
            ),
            repairable_states: seeded_secret_boundary_repairable_states(
                PROVIDER_SCOPE_MATRIX_ROW_ID,
            ),
            active_repair_state: seeded_secret_boundary_active_repair_state(
                PROVIDER_SCOPE_MATRIX_ROW_ID,
                health_state,
            ),
            profile_parity_rows: seeded_secret_boundary_profile_parity_rows(
                PROVIDER_SCOPE_MATRIX_ROW_ID,
            ),
            export_safety_banner: SecretBoundaryExportSafetyBanner::standard(
                PROVIDER_SCOPE_MATRIX_ROW_ID,
                "Raw provider grants, delegated credentials, and vault-backed values remain excluded from support bundles, scope exports, and portable registry packets.",
            ),
        }]
    }
}

fn account_scope_workflow(
    workflow_ref: impl Into<String>,
    workflow_label: impl Into<String>,
) -> SecretBoundaryWorkflowDependency {
    SecretBoundaryWorkflowDependency {
        workflow_ref: workflow_ref.into(),
        workflow_label: workflow_label.into(),
    }
}

fn account_scope_actor_identity(
    delegated_present: bool,
    installation_present: bool,
) -> SecretBoundaryActingIdentityClass {
    if delegated_present {
        SecretBoundaryActingIdentityClass::DelegatedCredential
    } else if installation_present {
        SecretBoundaryActingIdentityClass::ServiceIssuedAuthority
    } else {
        SecretBoundaryActingIdentityClass::HumanAccount
    }
}

fn account_scope_projection_controls(
    matrix_row_id: &str,
    delegated_present: bool,
    installation_present: bool,
) -> Vec<SecretBoundaryProjectionControl> {
    let local_safe_note =
        "Scope inspection, drift review, and local draft fallback remain available.";
    let mut controls = vec![SecretBoundaryProjectionControl::new(
        matrix_row_id,
        SecretBoundaryProjectionControlClass::StopUsingSecret,
        "Stop provider scope auth",
        local_safe_note,
    )];
    if delegated_present || installation_present {
        controls.push(SecretBoundaryProjectionControl::new(
            matrix_row_id,
            SecretBoundaryProjectionControlClass::DropDelegatedIdentity,
            "Drop delegated provider scope",
            local_safe_note,
        ));
    }
    if installation_present {
        controls.push(SecretBoundaryProjectionControl::new(
            matrix_row_id,
            SecretBoundaryProjectionControlClass::PauseForwarding,
            "Pause forwarded provider scope",
            local_safe_note,
        ));
    }
    controls
}

fn account_scope_picker_state() -> SecretBoundaryVaultPickerState {
    SecretBoundaryVaultPickerState {
        matrix_row_id: PROVIDER_SCOPE_MATRIX_ROW_ID.to_owned(),
        vocabulary_ref: M5_SECRET_BOUNDARY_DEPTH_VOCABULARY_REF.to_owned(),
        picker_label: "Provider scope source picker".to_owned(),
        options: vec![
            SecretBoundaryVaultPickerOption {
                option_id: "provider-scope:delegated".to_owned(),
                option_label: "Delegated credential".to_owned(),
                source_class: SecretBoundaryCredentialMode::Delegated,
                storage_class: SecretBoundaryStorageClass::SessionOnly,
                access_scope_label: "Provider scope or installation grant".to_owned(),
                reveal_policy_label: "No raw delegated token reveal".to_owned(),
                portability_note: "Exports preserve scope and lifecycle only.".to_owned(),
                open_source_of_truth_action_label: "Open scope lineage".to_owned(),
                selectable: true,
            },
            SecretBoundaryVaultPickerOption {
                option_id: "provider-scope:vault".to_owned(),
                option_label: "Enterprise vault or policy service".to_owned(),
                source_class: SecretBoundaryCredentialMode::EnterpriseVault,
                storage_class: SecretBoundaryStorageClass::EnterpriseVault,
                access_scope_label: "Managed provider scope".to_owned(),
                reveal_policy_label: "Vault ref or policy injection only".to_owned(),
                portability_note: "Portable exports omit raw values.".to_owned(),
                open_source_of_truth_action_label: "Open policy owner detail".to_owned(),
                selectable: true,
            },
        ],
    }
}

fn account_scope_secret_class(auth_source: AccountAuthSourceClass) -> SecretBoundarySecretClass {
    match auth_source {
        AccountAuthSourceClass::DelegatedCredential
        | AccountAuthSourceClass::PolicyInjectedService => {
            SecretBoundarySecretClass::CloudDelegatedIdentity
        }
        AccountAuthSourceClass::InstallationGrant | AccountAuthSourceClass::ProjectScopedGrant => {
            SecretBoundarySecretClass::CodeHostOrRegistryToken
        }
        AccountAuthSourceClass::HumanSession => SecretBoundarySecretClass::AiProviderToken,
    }
}

fn account_scope_credential_mode(
    auth_source: AccountAuthSourceClass,
) -> SecretBoundaryCredentialMode {
    match auth_source {
        AccountAuthSourceClass::HumanSession => SecretBoundaryCredentialMode::OsStore,
        AccountAuthSourceClass::InstallationGrant | AccountAuthSourceClass::ProjectScopedGrant => {
            SecretBoundaryCredentialMode::HandleOnly
        }
        AccountAuthSourceClass::DelegatedCredential => SecretBoundaryCredentialMode::Delegated,
        AccountAuthSourceClass::PolicyInjectedService => {
            SecretBoundaryCredentialMode::EnterpriseVault
        }
    }
}

fn account_scope_storage_class(auth_source: AccountAuthSourceClass) -> SecretBoundaryStorageClass {
    match auth_source {
        AccountAuthSourceClass::HumanSession => SecretBoundaryStorageClass::OsStore,
        AccountAuthSourceClass::PolicyInjectedService => {
            SecretBoundaryStorageClass::EnterpriseVault
        }
        AccountAuthSourceClass::DelegatedCredential
        | AccountAuthSourceClass::InstallationGrant
        | AccountAuthSourceClass::ProjectScopedGrant => SecretBoundaryStorageClass::SessionOnly,
    }
}

fn account_scope_projection_mode(
    auth_source: AccountAuthSourceClass,
) -> SecretBoundaryProjectionMode {
    match auth_source {
        AccountAuthSourceClass::DelegatedCredential => SecretBoundaryProjectionMode::Delegated,
        AccountAuthSourceClass::PolicyInjectedService => {
            SecretBoundaryProjectionMode::RemoteVaultFetch
        }
        _ => SecretBoundaryProjectionMode::RequestHeader,
    }
}

fn connected_health_state(state: AccountLifecycleStateClass) -> SecretBoundaryHealthStateClass {
    match state {
        AccountLifecycleStateClass::Active => SecretBoundaryHealthStateClass::Healthy,
        AccountLifecycleStateClass::ReauthRequired => SecretBoundaryHealthStateClass::ExpiringSoon,
        AccountLifecycleStateClass::Revoked => SecretBoundaryHealthStateClass::Revoked,
        AccountLifecycleStateClass::Suspended => SecretBoundaryHealthStateClass::PolicyBlocked,
        AccountLifecycleStateClass::Unreachable => SecretBoundaryHealthStateClass::Unavailable,
    }
}

fn installation_health_state(
    state: InstallationGrantLifecycleStateClass,
) -> SecretBoundaryHealthStateClass {
    match state {
        InstallationGrantLifecycleStateClass::Installed => SecretBoundaryHealthStateClass::Healthy,
        InstallationGrantLifecycleStateClass::SecretExpired => {
            SecretBoundaryHealthStateClass::Expired
        }
        InstallationGrantLifecycleStateClass::ScopeNarrowed
        | InstallationGrantLifecycleStateClass::Suspended => {
            SecretBoundaryHealthStateClass::PolicyBlocked
        }
        InstallationGrantLifecycleStateClass::Uninstalled => {
            SecretBoundaryHealthStateClass::Revoked
        }
    }
}

fn delegated_health_state(
    state: DelegatedCredentialLifecycleStateClass,
) -> SecretBoundaryHealthStateClass {
    match state {
        DelegatedCredentialLifecycleStateClass::Active => SecretBoundaryHealthStateClass::Healthy,
        DelegatedCredentialLifecycleStateClass::Expired => SecretBoundaryHealthStateClass::Expired,
        DelegatedCredentialLifecycleStateClass::Revoked => SecretBoundaryHealthStateClass::Revoked,
        DelegatedCredentialLifecycleStateClass::ScopeNarrowed
        | DelegatedCredentialLifecycleStateClass::DelegatorLostGrant => {
            SecretBoundaryHealthStateClass::PolicyBlocked
        }
    }
}

/// Support-export wrapper for the account-scope beta page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccountScopeBetaSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable export id.
    pub export_id: String,
    /// Export timestamp.
    pub exported_at: String,
    /// Exported page.
    pub page: AccountScopeBetaPage,
    /// Defect-kind tokens present in the page.
    pub defect_kinds_present: Vec<String>,
    /// Defect counts by defect-kind token.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
    /// True when raw access tokens, raw refresh tokens, raw delegated-token
    /// bodies, raw provider payloads, and raw policy-injector material are
    /// excluded from the export.
    pub raw_tokens_excluded: bool,
    /// True when identity lineage (connected account, installation grant,
    /// delegated credential) is preserved verbatim so support can name
    /// which identity actually acted.
    pub identity_lineage_preserved: bool,
    /// True when resolution lineage (acting identity class, resolved scope,
    /// decision, resolution reason) is preserved verbatim.
    pub resolution_lineage_preserved: bool,
    /// True when scope-drift lineage (trigger, forced downgrade, reapproval
    /// route) is preserved verbatim so support can replay the drift.
    pub drift_lineage_preserved: bool,
    /// True when the export proves the no-silent-public-endpoint-fallback
    /// and no-silent-mutation-widening invariants.
    pub fail_closed_invariant: bool,
    /// Reviewable summary of the redaction posture.
    pub redaction_summary: String,
}

impl AccountScopeBetaSupportExport {
    /// Builds a support-export wrapper from a beta page. The beta page never
    /// carries raw token material, raw provider payloads, or plaintext
    /// fallback material, so identity, resolution, and drift lineage are
    /// preserved verbatim without further redaction.
    pub fn from_page(
        export_id: impl Into<String>,
        exported_at: impl Into<String>,
        page: AccountScopeBetaPage,
    ) -> Self {
        let defect_counts_by_kind = page.summary.defect_counts_by_kind.clone();
        let defect_kinds_present = defect_counts_by_kind.keys().cloned().collect();
        Self {
            record_kind: ACCOUNT_SCOPE_BETA_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            export_id: export_id.into(),
            exported_at: exported_at.into(),
            page,
            defect_kinds_present,
            defect_counts_by_kind,
            raw_tokens_excluded: true,
            identity_lineage_preserved: true,
            resolution_lineage_preserved: true,
            drift_lineage_preserved: true,
            fail_closed_invariant: true,
            redaction_summary:
                "Metadata-only account-scope beta export: connected-account, installation-grant, \
                 and delegated-credential identity lineage; resolved-scope and decision lineage; \
                 and scope-drift trigger and forced-downgrade lineage are preserved verbatim. Raw \
                 access tokens, raw delegated-token bodies, raw provider payloads, and raw \
                 policy-injector material are excluded because the beta projection never carries \
                 them."
                    .to_owned(),
        }
    }
}

/// Validates the account-scope beta page and returns typed defects on
/// failure.
pub fn validate_account_scope_beta_page(
    page: &AccountScopeBetaPage,
) -> Result<(), Vec<AccountScopeBetaDefect>> {
    let defects = audit_account_scope_beta_page(
        &page.connected_account_rows,
        &page.installation_grant_rows,
        &page.delegated_credential_rows,
        &page.resolution_rows,
        &page.scope_drift_events,
    );
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for an account-scope beta page.
pub fn audit_account_scope_beta_page(
    connected_accounts: &[ConnectedAccountRow],
    installation_grants: &[InstallationGrantRow],
    delegated_credentials: &[DelegatedCredentialRow],
    resolutions: &[EffectiveScopeResolutionRow],
    drift_events: &[ScopeDriftEvent],
) -> Vec<AccountScopeBetaDefect> {
    let mut defects = Vec::new();

    let identity_classes: BTreeMap<String, ActingIdentityClass> = connected_accounts
        .iter()
        .map(|row| {
            (
                row.identity_row_id.clone(),
                ActingIdentityClass::ConnectedAccount,
            )
        })
        .chain(installation_grants.iter().map(|row| {
            (
                row.identity_row_id.clone(),
                ActingIdentityClass::InstallationGrant,
            )
        }))
        .chain(delegated_credentials.iter().map(|row| {
            (
                row.identity_row_id.clone(),
                ActingIdentityClass::DelegatedCredential,
            )
        }))
        .collect();

    let connected_closed: BTreeMap<&str, bool> = connected_accounts
        .iter()
        .map(|row| {
            (
                row.identity_row_id.as_str(),
                row.lifecycle_state.holds_mutation_closed(),
            )
        })
        .collect();
    let installation_closed: BTreeMap<&str, bool> = installation_grants
        .iter()
        .map(|row| {
            (
                row.identity_row_id.as_str(),
                row.lifecycle_state.holds_mutation_closed(),
            )
        })
        .collect();
    let delegated_closed: BTreeMap<&str, bool> = delegated_credentials
        .iter()
        .map(|row| {
            (
                row.identity_row_id.as_str(),
                row.lifecycle_state.holds_mutation_closed(),
            )
        })
        .collect();

    let resolution_ids: BTreeSet<&str> = resolutions
        .iter()
        .map(|row| row.resolution_id.as_str())
        .collect();

    for row in connected_accounts {
        audit_identity_common(
            &mut defects,
            &row.identity_row_id,
            row.profile,
            &row.profile_token,
            row.acting_identity_class,
            ActingIdentityClass::ConnectedAccount,
            &row.acting_identity_class_token,
            row.auth_source,
            &row.auth_source_token,
            row.lifecycle_state.as_str(),
            &row.lifecycle_state_token,
            row.raw_token_material_present,
            row.public_endpoint_fallback_offered,
            row.local_editing_preserved,
        );
    }
    for row in installation_grants {
        audit_identity_common(
            &mut defects,
            &row.identity_row_id,
            row.profile,
            &row.profile_token,
            row.acting_identity_class,
            ActingIdentityClass::InstallationGrant,
            &row.acting_identity_class_token,
            row.auth_source,
            &row.auth_source_token,
            row.lifecycle_state.as_str(),
            &row.lifecycle_state_token,
            row.raw_token_material_present,
            row.public_endpoint_fallback_offered,
            row.local_editing_preserved,
        );
    }
    for row in delegated_credentials {
        audit_identity_common(
            &mut defects,
            &row.identity_row_id,
            row.profile,
            &row.profile_token,
            row.acting_identity_class,
            ActingIdentityClass::DelegatedCredential,
            &row.acting_identity_class_token,
            row.auth_source,
            &row.auth_source_token,
            row.lifecycle_state.as_str(),
            &row.lifecycle_state_token,
            row.raw_token_material_present,
            row.public_endpoint_fallback_offered,
            row.local_editing_preserved,
        );
    }

    for row in resolutions {
        if row.profile_token != row.profile.as_str() {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::ProfileTokenDrift,
                row.resolution_id.clone(),
                "profile_token",
                "profile_token must match profile",
            ));
        }
        if row.acting_identity_class_token != row.acting_identity_class.as_str() {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::ActingIdentityClassTokenDrift,
                row.resolution_id.clone(),
                "acting_identity_class_token",
                "acting_identity_class_token must match acting_identity_class",
            ));
        }
        if row.decision_token != row.decision.as_str() {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::DecisionTokenDrift,
                row.resolution_id.clone(),
                "decision_token",
                "decision_token must match decision",
            ));
        }
        if row.resolution_reason_token != row.resolution_reason.as_str() {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::ResolutionReasonTokenDrift,
                row.resolution_id.clone(),
                "resolution_reason_token",
                "resolution_reason_token must match resolution_reason",
            ));
        }
        if row.requested_action_token != row.requested_action.as_str() {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::RequestedActionTokenDrift,
                row.resolution_id.clone(),
                "requested_action_token",
                "requested_action_token must match requested_action",
            ));
        }
        if row.reapproval_route_token != row.reapproval_route.as_str() {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::ReapprovalRouteTokenDrift,
                row.resolution_id.clone(),
                "reapproval_route_token",
                "reapproval_route_token must match reapproval_route",
            ));
        }

        if row.raw_token_material_present {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::RawTokenMaterialPresent,
                row.resolution_id.clone(),
                "raw_token_material_present",
                "claimed beta resolution must not carry raw token material",
            ));
        }
        if row.public_endpoint_fallback_taken {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::PublicEndpointFallbackTaken,
                row.resolution_id.clone(),
                "public_endpoint_fallback_taken",
                "claimed beta resolution must not silently fall back to a public endpoint",
            ));
        }
        if row.silent_mutation_authority_widened {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::SilentMutationAuthorityWidened,
                row.resolution_id.clone(),
                "silent_mutation_authority_widened",
                "claimed beta resolution must not silently widen mutation authority",
            ));
        }
        if !row.local_editing_preserved {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::LocalEditingNotPreserved,
                row.resolution_id.clone(),
                "local_editing_preserved",
                "claimed beta resolution must preserve local editing",
            ));
        }

        match identity_classes.get(&row.bound_identity_row_ref) {
            None => {
                defects.push(AccountScopeBetaDefect::new(
                    AccountScopeBetaDefectKind::BoundIdentityRefUnknown,
                    row.resolution_id.clone(),
                    "bound_identity_row_ref",
                    "bound_identity_row_ref must reference an identity row on the page",
                ));
            }
            Some(class) if *class != row.acting_identity_class => {
                defects.push(AccountScopeBetaDefect::new(
                    AccountScopeBetaDefectKind::BoundIdentityClassMismatch,
                    row.resolution_id.clone(),
                    "acting_identity_class",
                    "acting_identity_class must match the bound identity row's class",
                ));
            }
            Some(_) => {
                let closed = match row.acting_identity_class {
                    ActingIdentityClass::ConnectedAccount => connected_closed
                        .get(row.bound_identity_row_ref.as_str())
                        .copied()
                        .unwrap_or(false),
                    ActingIdentityClass::InstallationGrant => installation_closed
                        .get(row.bound_identity_row_ref.as_str())
                        .copied()
                        .unwrap_or(false),
                    ActingIdentityClass::DelegatedCredential => delegated_closed
                        .get(row.bound_identity_row_ref.as_str())
                        .copied()
                        .unwrap_or(false),
                };
                if closed && row.decision.admits_mutation() {
                    defects.push(AccountScopeBetaDefect::new(
                        AccountScopeBetaDefectKind::AllowedDecisionOnClosedLifecycle,
                        row.resolution_id.clone(),
                        "decision",
                        "decision must not admit mutation when the bound identity lifecycle holds \
                         authority closed",
                    ));
                }
            }
        }

        if row.decision.admits_mutation() && !row.resolution_reason.is_allowed_family() {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::AllowedDecisionWithNonAllowedReason,
                row.resolution_id.clone(),
                "resolution_reason",
                "allowed decision must pair with an allowed-family resolution reason",
            ));
        }
        if !row.decision.admits_mutation()
            && row.resolution_reason.is_allowed_family()
            && row.resolution_reason != GrantResolutionReasonClass::AllowedWithBrowserHandoff
            && row.resolution_reason != GrantResolutionReasonClass::AllowedWithDeferredPublish
        {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::NonAllowedDecisionWithAllowedReason,
                row.resolution_id.clone(),
                "resolution_reason",
                "non-allowed decision must pair with a non-allowed resolution reason",
            ));
        }

        match row.decision {
            AuthorityDecisionClass::BrowserOnly => {
                if row
                    .browser_handoff_packet_ref
                    .as_deref()
                    .unwrap_or("")
                    .is_empty()
                {
                    defects.push(AccountScopeBetaDefect::new(
                        AccountScopeBetaDefectKind::BrowserOnlyWithoutHandoffRef,
                        row.resolution_id.clone(),
                        "browser_handoff_packet_ref",
                        "browser_only decision must cite a browser_handoff_packet_ref",
                    ));
                }
            }
            AuthorityDecisionClass::LocalDraftOnly => {
                if row
                    .publish_later_queue_item_ref
                    .as_deref()
                    .unwrap_or("")
                    .is_empty()
                {
                    defects.push(AccountScopeBetaDefect::new(
                        AccountScopeBetaDefectKind::LocalDraftOnlyWithoutQueueRef,
                        row.resolution_id.clone(),
                        "publish_later_queue_item_ref",
                        "local_draft_only decision must cite a publish_later_queue_item_ref",
                    ));
                }
            }
            AuthorityDecisionClass::Allowed => {
                if row.requested_action.proposes_mutation()
                    && row.approval_ticket_ref.as_deref().unwrap_or("").is_empty()
                {
                    defects.push(AccountScopeBetaDefect::new(
                        AccountScopeBetaDefectKind::AllowedMutationWithoutApprovalTicket,
                        row.resolution_id.clone(),
                        "approval_ticket_ref",
                        "allowed mutation must cite an approval_ticket_ref",
                    ));
                }
                if row.resolved_scope_refs.is_empty() {
                    defects.push(AccountScopeBetaDefect::new(
                        AccountScopeBetaDefectKind::AllowedResolutionWithoutResolvedScope,
                        row.resolution_id.clone(),
                        "resolved_scope_refs",
                        "allowed resolution must name resolved scope refs",
                    ));
                }
            }
            AuthorityDecisionClass::Denied => {}
        }

        if !row.decision.admits_mutation()
            && row.reapproval_route == ReapprovalRouteClass::NoneRequired
        {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::NonAllowedDecisionMissingReapprovalRoute,
                row.resolution_id.clone(),
                "reapproval_route",
                "non-allowed decision must name a typed reapproval route",
            ));
        }

        let declared: BTreeSet<&str> = row
            .provider_declared_scope_refs
            .iter()
            .map(String::as_str)
            .collect();
        for resolved in &row.resolved_scope_refs {
            if !declared.contains(resolved.as_str()) {
                defects.push(AccountScopeBetaDefect::new(
                    AccountScopeBetaDefectKind::ResolvedScopeNotSubsetOfDeclared,
                    row.resolution_id.clone(),
                    "resolved_scope_refs",
                    "resolved_scope_refs must be a subset of provider_declared_scope_refs",
                ));
                break;
            }
        }
    }

    for event in drift_events {
        if event.profile_token != event.profile.as_str() {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::ProfileTokenDrift,
                event.event_id.clone(),
                "profile_token",
                "profile_token must match profile",
            ));
        }
        if event.trigger_token != event.trigger.as_str() {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::DriftTriggerTokenDrift,
                event.event_id.clone(),
                "trigger_token",
                "trigger_token must match trigger",
            ));
        }
        if event.forced_downgrade_token != event.forced_downgrade.as_str() {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::DriftDowngradeTokenDrift,
                event.event_id.clone(),
                "forced_downgrade_token",
                "forced_downgrade_token must match forced_downgrade",
            ));
        }
        if event.reapproval_route_token != event.reapproval_route.as_str() {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::ReapprovalRouteTokenDrift,
                event.event_id.clone(),
                "reapproval_route_token",
                "reapproval_route_token must match reapproval_route",
            ));
        }

        if !identity_classes.contains_key(&event.originating_identity_row_ref) {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::DriftEventIdentityRefUnknown,
                event.event_id.clone(),
                "originating_identity_row_ref",
                "originating_identity_row_ref must reference an identity row on the page",
            ));
        }
        if !resolution_ids.contains(event.affected_resolution_row_ref.as_str()) {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::DriftEventResolutionRefUnknown,
                event.event_id.clone(),
                "affected_resolution_row_ref",
                "affected_resolution_row_ref must reference a resolution on the page",
            ));
        }
        if event.silent_mutation_authority_retained {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::SilentMutationAuthorityRetainedAfterDrift,
                event.event_id.clone(),
                "silent_mutation_authority_retained",
                "drift event must not silently keep mutation authority",
            ));
        }
        if !event.local_editing_preserved {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::LocalEditingNotPreserved,
                event.event_id.clone(),
                "local_editing_preserved",
                "drift event must preserve local editing",
            ));
        }
        if event.forced_downgrade == AuthorityDowngradeClass::NoDowngradeRequired {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::DriftEventDowngradeMissing,
                event.event_id.clone(),
                "forced_downgrade",
                "drift event must force a visible downgrade so mutation authority does not \
                 silently continue",
            ));
        }
        if event.reapproval_route == ReapprovalRouteClass::NoneRequired {
            defects.push(AccountScopeBetaDefect::new(
                AccountScopeBetaDefectKind::DriftEventReapprovalRouteMissing,
                event.event_id.clone(),
                "reapproval_route",
                "drift event must name a typed reapproval route",
            ));
        }
    }

    let mut observed_profiles: BTreeSet<&str> = BTreeSet::new();
    for row in connected_accounts {
        observed_profiles.insert(row.profile_token.as_str());
    }
    for row in installation_grants {
        observed_profiles.insert(row.profile_token.as_str());
    }
    for row in delegated_credentials {
        observed_profiles.insert(row.profile_token.as_str());
    }
    for row in resolutions {
        observed_profiles.insert(row.profile_token.as_str());
    }
    let required_profiles: BTreeSet<&str> = AccountScopeBetaProfileClass::ALL
        .iter()
        .map(|profile| profile.as_str())
        .collect();
    for missing in required_profiles.difference(&observed_profiles) {
        defects.push(AccountScopeBetaDefect::new(
            AccountScopeBetaDefectKind::ProfileCoverageMissing,
            "page",
            "profiles",
            format!("missing {} profile coverage", missing),
        ));
    }

    defects
}

#[allow(clippy::too_many_arguments)]
fn audit_identity_common(
    defects: &mut Vec<AccountScopeBetaDefect>,
    identity_row_id: &str,
    profile: AccountScopeBetaProfileClass,
    profile_token: &str,
    acting_identity_class: ActingIdentityClass,
    expected_class: ActingIdentityClass,
    acting_identity_class_token: &str,
    auth_source: AccountAuthSourceClass,
    auth_source_token: &str,
    lifecycle_state_str: &str,
    lifecycle_state_token: &str,
    raw_token_material_present: bool,
    public_endpoint_fallback_offered: bool,
    local_editing_preserved: bool,
) {
    if profile_token != profile.as_str() {
        defects.push(AccountScopeBetaDefect::new(
            AccountScopeBetaDefectKind::ProfileTokenDrift,
            identity_row_id.to_owned(),
            "profile_token",
            "profile_token must match profile",
        ));
    }
    if acting_identity_class != expected_class {
        defects.push(AccountScopeBetaDefect::new(
            AccountScopeBetaDefectKind::IdentityClassMismatch,
            identity_row_id.to_owned(),
            "acting_identity_class",
            "identity row's acting_identity_class must match its row kind",
        ));
    }
    if acting_identity_class_token != acting_identity_class.as_str() {
        defects.push(AccountScopeBetaDefect::new(
            AccountScopeBetaDefectKind::ActingIdentityClassTokenDrift,
            identity_row_id.to_owned(),
            "acting_identity_class_token",
            "acting_identity_class_token must match acting_identity_class",
        ));
    }
    if auth_source_token != auth_source.as_str() {
        defects.push(AccountScopeBetaDefect::new(
            AccountScopeBetaDefectKind::AuthSourceTokenDrift,
            identity_row_id.to_owned(),
            "auth_source_token",
            "auth_source_token must match auth_source",
        ));
    }
    if lifecycle_state_token != lifecycle_state_str {
        defects.push(AccountScopeBetaDefect::new(
            AccountScopeBetaDefectKind::LifecycleStateTokenDrift,
            identity_row_id.to_owned(),
            "lifecycle_state_token",
            "lifecycle_state_token must match lifecycle_state",
        ));
    }
    if raw_token_material_present {
        defects.push(AccountScopeBetaDefect::new(
            AccountScopeBetaDefectKind::RawTokenMaterialPresent,
            identity_row_id.to_owned(),
            "raw_token_material_present",
            "claimed beta identity row must not carry raw token material",
        ));
    }
    if public_endpoint_fallback_offered {
        defects.push(AccountScopeBetaDefect::new(
            AccountScopeBetaDefectKind::PublicEndpointFallbackOffered,
            identity_row_id.to_owned(),
            "public_endpoint_fallback_offered",
            "claimed beta identity row must not offer a silent public-endpoint fallback",
        ));
    }
    if !local_editing_preserved {
        defects.push(AccountScopeBetaDefect::new(
            AccountScopeBetaDefectKind::LocalEditingNotPreserved,
            identity_row_id.to_owned(),
            "local_editing_preserved",
            "claimed beta identity row must preserve local editing",
        ));
    }
}

/// Builds the seeded account-scope beta page covering connected, mirror,
/// offline, and enterprise-managed profiles.
pub fn seeded_account_scope_beta_page() -> AccountScopeBetaPage {
    let connected_account_rows = seed_connected_account_rows();
    let installation_grant_rows = seed_installation_grant_rows();
    let delegated_credential_rows = seed_delegated_credential_rows();
    let resolution_rows = seed_resolution_rows();
    let scope_drift_events = seed_scope_drift_events();

    let defects = audit_account_scope_beta_page(
        &connected_account_rows,
        &installation_grant_rows,
        &delegated_credential_rows,
        &resolution_rows,
        &scope_drift_events,
    );
    let summary = AccountScopeBetaSummary::from_records(
        &connected_account_rows,
        &installation_grant_rows,
        &delegated_credential_rows,
        &resolution_rows,
        &scope_drift_events,
        &defects,
    );

    AccountScopeBetaPage {
        record_kind: ACCOUNT_SCOPE_BETA_PAGE_RECORD_KIND.to_owned(),
        schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
        shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
        source_matrix_ref: ACCOUNT_SCOPE_BETA_SOURCE_MATRIX_REF.to_owned(),
        connected_account_rows,
        installation_grant_rows,
        delegated_credential_rows,
        resolution_rows,
        scope_drift_events,
        defects,
        summary,
    }
}

fn provider_host(
    canonical_host_ref: &str,
    tenant_or_org_scope_ref: &str,
    host_label: &str,
) -> ProviderHostBinding {
    ProviderHostBinding {
        canonical_host_ref: canonical_host_ref.to_owned(),
        tenant_or_org_scope_ref: tenant_or_org_scope_ref.to_owned(),
        host_label: host_label.to_owned(),
    }
}

fn seed_connected_account_rows() -> Vec<ConnectedAccountRow> {
    vec![
        ConnectedAccountRow {
            record_kind: ACCOUNT_SCOPE_BETA_CONNECTED_ACCOUNT_ROW_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            identity_row_id: "account-scope-beta:connected-account:connected:human-dev".to_owned(),
            display_label: "Signed-in human developer account".to_owned(),
            profile: AccountScopeBetaProfileClass::Connected,
            profile_token: AccountScopeBetaProfileClass::Connected.as_str().to_owned(),
            acting_identity_class: ActingIdentityClass::ConnectedAccount,
            acting_identity_class_token: ActingIdentityClass::ConnectedAccount.as_str().to_owned(),
            subject: ConnectedAccountSubject {
                subject_ref: "subject:human-account:workspace:payments:dev-001".to_owned(),
                subject_label: "Workspace developer".to_owned(),
                capability_hash_ref: "capability-hash:human-account:payments:dev-001:beta"
                    .to_owned(),
            },
            provider_host: provider_host(
                "provider-host:code-host:public",
                "tenant:org:payments",
                "Public code host (payments org)",
            ),
            auth_source: AccountAuthSourceClass::HumanSession,
            auth_source_token: AccountAuthSourceClass::HumanSession.as_str().to_owned(),
            lifecycle_state: AccountLifecycleStateClass::Active,
            lifecycle_state_token: AccountLifecycleStateClass::Active.as_str().to_owned(),
            lifecycle_note: "Active system-browser session within freshness floor.".to_owned(),
            observed_at: "2026-05-16T10:00:00Z".to_owned(),
            expires_at: Some("2026-05-16T18:00:00Z".to_owned()),
            raw_token_material_present: false,
            public_endpoint_fallback_offered: false,
            local_editing_preserved: true,
        },
        ConnectedAccountRow {
            record_kind: ACCOUNT_SCOPE_BETA_CONNECTED_ACCOUNT_ROW_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            identity_row_id: "account-scope-beta:connected-account:mirror_only:human-reviewer"
                .to_owned(),
            display_label: "Signed-in reviewer on mirror-only profile".to_owned(),
            profile: AccountScopeBetaProfileClass::MirrorOnly,
            profile_token: AccountScopeBetaProfileClass::MirrorOnly.as_str().to_owned(),
            acting_identity_class: ActingIdentityClass::ConnectedAccount,
            acting_identity_class_token: ActingIdentityClass::ConnectedAccount.as_str().to_owned(),
            subject: ConnectedAccountSubject {
                subject_ref: "subject:human-account:workspace:payments:reviewer-001".to_owned(),
                subject_label: "Workspace reviewer".to_owned(),
                capability_hash_ref: "capability-hash:human-account:payments:reviewer-001:beta"
                    .to_owned(),
            },
            provider_host: provider_host(
                "provider-host:code-host:enterprise-mirror",
                "tenant:org:payments",
                "Enterprise mirror (payments org)",
            ),
            auth_source: AccountAuthSourceClass::HumanSession,
            auth_source_token: AccountAuthSourceClass::HumanSession.as_str().to_owned(),
            lifecycle_state: AccountLifecycleStateClass::ReauthRequired,
            lifecycle_state_token: AccountLifecycleStateClass::ReauthRequired
                .as_str()
                .to_owned(),
            lifecycle_note: "Mirror requires step-up reauth before review-decision publish."
                .to_owned(),
            observed_at: "2026-05-16T10:05:00Z".to_owned(),
            expires_at: Some("2026-05-16T10:35:00Z".to_owned()),
            raw_token_material_present: false,
            public_endpoint_fallback_offered: false,
            local_editing_preserved: true,
        },
    ]
}

fn seed_installation_grant_rows() -> Vec<InstallationGrantRow> {
    vec![
        InstallationGrantRow {
            record_kind: ACCOUNT_SCOPE_BETA_INSTALLATION_GRANT_ROW_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            identity_row_id: "account-scope-beta:installation-grant:connected:ci-bot".to_owned(),
            display_label: "CI bot installation grant".to_owned(),
            profile: AccountScopeBetaProfileClass::Connected,
            profile_token: AccountScopeBetaProfileClass::Connected.as_str().to_owned(),
            acting_identity_class: ActingIdentityClass::InstallationGrant,
            acting_identity_class_token: ActingIdentityClass::InstallationGrant.as_str().to_owned(),
            installation_grant_ref: "grant:installation:ci-bot:payments-org".to_owned(),
            grant_label: "Payments CI bot".to_owned(),
            issuer_ref: "issuer:code-host:public".to_owned(),
            issuer_label: "Public code host".to_owned(),
            provider_host: provider_host(
                "provider-host:code-host:public",
                "tenant:org:payments",
                "Public code host (payments org)",
            ),
            bounded_target_scope_refs: vec![
                "scope:repo:payments/backend:ci_run_or_check_mutation".to_owned(),
                "scope:repo:payments/frontend:ci_run_or_check_mutation".to_owned(),
            ],
            auth_source: AccountAuthSourceClass::InstallationGrant,
            auth_source_token: AccountAuthSourceClass::InstallationGrant
                .as_str()
                .to_owned(),
            lifecycle_state: InstallationGrantLifecycleStateClass::Installed,
            lifecycle_state_token: InstallationGrantLifecycleStateClass::Installed
                .as_str()
                .to_owned(),
            lifecycle_note: "Installation grant active; secret within rotation window.".to_owned(),
            managed_policy_bundle_ref: None,
            observed_at: "2026-05-16T10:10:00Z".to_owned(),
            secret_expires_at: Some("2026-06-16T10:10:00Z".to_owned()),
            raw_token_material_present: false,
            public_endpoint_fallback_offered: false,
            local_editing_preserved: true,
        },
        InstallationGrantRow {
            record_kind: ACCOUNT_SCOPE_BETA_INSTALLATION_GRANT_ROW_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            identity_row_id: "account-scope-beta:installation-grant:enterprise_managed:managed-bot"
                .to_owned(),
            display_label: "Enterprise-managed deploy bot grant".to_owned(),
            profile: AccountScopeBetaProfileClass::EnterpriseManaged,
            profile_token: AccountScopeBetaProfileClass::EnterpriseManaged
                .as_str()
                .to_owned(),
            acting_identity_class: ActingIdentityClass::InstallationGrant,
            acting_identity_class_token: ActingIdentityClass::InstallationGrant.as_str().to_owned(),
            installation_grant_ref: "grant:installation:managed-deploy:tenant-001".to_owned(),
            grant_label: "Managed deploy bot".to_owned(),
            issuer_ref: "issuer:managed-policy:tenant-001".to_owned(),
            issuer_label: "Managed policy authority (tenant 001)".to_owned(),
            provider_host: provider_host(
                "provider-host:code-host:enterprise",
                "tenant:enterprise:tenant-001",
                "Enterprise code host (tenant 001)",
            ),
            bounded_target_scope_refs: vec![
                "scope:repo:tenant-001/fleet:ci_run_or_check_mutation".to_owned()
            ],
            auth_source: AccountAuthSourceClass::PolicyInjectedService,
            auth_source_token: AccountAuthSourceClass::PolicyInjectedService
                .as_str()
                .to_owned(),
            lifecycle_state: InstallationGrantLifecycleStateClass::Suspended,
            lifecycle_state_token: InstallationGrantLifecycleStateClass::Suspended
                .as_str()
                .to_owned(),
            lifecycle_note: "Managed policy authority suspended the grant pending admin review."
                .to_owned(),
            managed_policy_bundle_ref: Some(
                "managed-policy-bundle:tenant-001:deploy-bot:v3".to_owned(),
            ),
            observed_at: "2026-05-16T10:20:00Z".to_owned(),
            secret_expires_at: None,
            raw_token_material_present: false,
            public_endpoint_fallback_offered: false,
            local_editing_preserved: true,
        },
    ]
}

fn seed_delegated_credential_rows() -> Vec<DelegatedCredentialRow> {
    vec![DelegatedCredentialRow {
        record_kind: ACCOUNT_SCOPE_BETA_DELEGATED_CREDENTIAL_ROW_RECORD_KIND.to_owned(),
        schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
        shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
        identity_row_id: "account-scope-beta:delegated-credential:offline:release-signer"
            .to_owned(),
        display_label: "Delegated release-signer credential on offline profile".to_owned(),
        profile: AccountScopeBetaProfileClass::Offline,
        profile_token: AccountScopeBetaProfileClass::Offline.as_str().to_owned(),
        acting_identity_class: ActingIdentityClass::DelegatedCredential,
        acting_identity_class_token: ActingIdentityClass::DelegatedCredential.as_str().to_owned(),
        delegation_ref: "delegation:release-signer:fleet-0001".to_owned(),
        delegator_actor_ref: "actor:human-account:release-admin".to_owned(),
        delegator_label: "Release administrator".to_owned(),
        on_behalf_of_actor_ref: "actor:human-account:release-engineer".to_owned(),
        on_behalf_of_label: "Release engineer".to_owned(),
        provider_host: provider_host(
            "provider-host:release-registry:enterprise",
            "tenant:enterprise:tenant-001",
            "Release registry (tenant 001)",
        ),
        delegated_scope_refs: vec![
            "scope:release:fleet-0001:release_publish".to_owned(),
            "scope:release:fleet-0001:read_only_inspection".to_owned(),
        ],
        auth_source: AccountAuthSourceClass::DelegatedCredential,
        auth_source_token: AccountAuthSourceClass::DelegatedCredential
            .as_str()
            .to_owned(),
        lifecycle_state: DelegatedCredentialLifecycleStateClass::Expired,
        lifecycle_state_token: DelegatedCredentialLifecycleStateClass::Expired
            .as_str()
            .to_owned(),
        lifecycle_note:
            "Delegated credential expired on the offline profile; reissue required from the \
                 air-gapped channel."
                .to_owned(),
        observed_at: "2026-05-16T10:25:00Z".to_owned(),
        expires_at: Some("2026-05-16T09:00:00Z".to_owned()),
        raw_token_material_present: false,
        public_endpoint_fallback_offered: false,
        local_editing_preserved: true,
    }]
}

fn seed_resolution_rows() -> Vec<EffectiveScopeResolutionRow> {
    vec![
        EffectiveScopeResolutionRow {
            record_kind: ACCOUNT_SCOPE_BETA_EFFECTIVE_SCOPE_ROW_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            resolution_id: "account-scope-beta:resolution:connected:human-dev:comment-allowed"
                .to_owned(),
            display_label: "Comment on PR allowed under human account".to_owned(),
            profile: AccountScopeBetaProfileClass::Connected,
            profile_token: AccountScopeBetaProfileClass::Connected.as_str().to_owned(),
            acting_identity_class: ActingIdentityClass::ConnectedAccount,
            acting_identity_class_token: ActingIdentityClass::ConnectedAccount.as_str().to_owned(),
            bound_identity_row_ref: "account-scope-beta:connected-account:connected:human-dev"
                .to_owned(),
            requested_action: RequestedActionClass::HumanAuthoredComment,
            requested_action_token: RequestedActionClass::HumanAuthoredComment
                .as_str()
                .to_owned(),
            target: ProviderTargetIdentity {
                target_ref: "target:pull_request:payments/backend:1234".to_owned(),
                target_label: "PR #1234 on payments/backend".to_owned(),
                provider_linked_row_ref: "provider-linked-row:pr:payments/backend:1234".to_owned(),
            },
            provider_declared_scope_refs: vec![
                "scope:repo:payments/backend:human_authored_comment".to_owned(),
                "scope:repo:payments/backend:review_decision_publish".to_owned(),
                "scope:repo:payments/backend:read_only_inspection".to_owned(),
            ],
            resolved_scope_refs: vec![
                "scope:repo:payments/backend:human_authored_comment".to_owned()
            ],
            decision: AuthorityDecisionClass::Allowed,
            decision_token: AuthorityDecisionClass::Allowed.as_str().to_owned(),
            resolution_reason: GrantResolutionReasonClass::Allowed,
            resolution_reason_token: GrantResolutionReasonClass::Allowed.as_str().to_owned(),
            decision_summary:
                "Allowed: human-authored comment on PR #1234 under the signed-in human account."
                    .to_owned(),
            reapproval_route: ReapprovalRouteClass::NoneRequired,
            reapproval_route_token: ReapprovalRouteClass::NoneRequired.as_str().to_owned(),
            reapproval_route_label: "No reapproval required".to_owned(),
            remediation_path_ref: None,
            approval_ticket_ref: Some(
                "approval-ticket:human-authored-comment:pr:1234:beta".to_owned(),
            ),
            browser_handoff_packet_ref: None,
            publish_later_queue_item_ref: None,
            computed_at: "2026-05-16T10:00:30Z".to_owned(),
            expires_at: Some("2026-05-16T10:30:30Z".to_owned()),
            raw_token_material_present: false,
            public_endpoint_fallback_taken: false,
            silent_mutation_authority_widened: false,
            local_editing_preserved: true,
        },
        EffectiveScopeResolutionRow {
            record_kind: ACCOUNT_SCOPE_BETA_EFFECTIVE_SCOPE_ROW_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            resolution_id:
                "account-scope-beta:resolution:mirror_only:human-reviewer:browser-only-merge"
                    .to_owned(),
            display_label: "Review-decision merge routed through browser handoff".to_owned(),
            profile: AccountScopeBetaProfileClass::MirrorOnly,
            profile_token: AccountScopeBetaProfileClass::MirrorOnly.as_str().to_owned(),
            acting_identity_class: ActingIdentityClass::ConnectedAccount,
            acting_identity_class_token: ActingIdentityClass::ConnectedAccount.as_str().to_owned(),
            bound_identity_row_ref:
                "account-scope-beta:connected-account:mirror_only:human-reviewer".to_owned(),
            requested_action: RequestedActionClass::ReviewDecisionPublish,
            requested_action_token: RequestedActionClass::ReviewDecisionPublish
                .as_str()
                .to_owned(),
            target: ProviderTargetIdentity {
                target_ref: "target:pull_request:payments/backend:1234".to_owned(),
                target_label: "PR #1234 on payments/backend".to_owned(),
                provider_linked_row_ref: "provider-linked-row:pr:payments/backend:1234".to_owned(),
            },
            provider_declared_scope_refs: vec![
                "scope:repo:payments/backend:review_decision_publish".to_owned(),
            ],
            resolved_scope_refs: vec![],
            decision: AuthorityDecisionClass::BrowserOnly,
            decision_token: AuthorityDecisionClass::BrowserOnly.as_str().to_owned(),
            resolution_reason: GrantResolutionReasonClass::AllowedWithBrowserHandoff,
            resolution_reason_token: GrantResolutionReasonClass::AllowedWithBrowserHandoff
                .as_str()
                .to_owned(),
            decision_summary:
                "Browser-only: mirror requires step-up reauth, so merge is routed through a \
                 browser handoff."
                    .to_owned(),
            reapproval_route: ReapprovalRouteClass::BrowserHandoff,
            reapproval_route_token: ReapprovalRouteClass::BrowserHandoff.as_str().to_owned(),
            reapproval_route_label: "Complete in browser".to_owned(),
            remediation_path_ref: Some("remediation-path:browser-handoff:pr:1234:merge".to_owned()),
            approval_ticket_ref: None,
            browser_handoff_packet_ref: Some(
                "browser-handoff-packet:pr:1234:merge:mirror_only".to_owned(),
            ),
            publish_later_queue_item_ref: None,
            computed_at: "2026-05-16T10:05:30Z".to_owned(),
            expires_at: Some("2026-05-16T10:35:30Z".to_owned()),
            raw_token_material_present: false,
            public_endpoint_fallback_taken: false,
            silent_mutation_authority_widened: false,
            local_editing_preserved: true,
        },
        EffectiveScopeResolutionRow {
            record_kind: ACCOUNT_SCOPE_BETA_EFFECTIVE_SCOPE_ROW_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            resolution_id: "account-scope-beta:resolution:connected:ci-bot:check-run-allowed"
                .to_owned(),
            display_label: "CI check mutation allowed under installation grant".to_owned(),
            profile: AccountScopeBetaProfileClass::Connected,
            profile_token: AccountScopeBetaProfileClass::Connected.as_str().to_owned(),
            acting_identity_class: ActingIdentityClass::InstallationGrant,
            acting_identity_class_token: ActingIdentityClass::InstallationGrant.as_str().to_owned(),
            bound_identity_row_ref: "account-scope-beta:installation-grant:connected:ci-bot"
                .to_owned(),
            requested_action: RequestedActionClass::CiRunOrCheckMutation,
            requested_action_token: RequestedActionClass::CiRunOrCheckMutation
                .as_str()
                .to_owned(),
            target: ProviderTargetIdentity {
                target_ref: "target:check_run:payments/backend:9876".to_owned(),
                target_label: "Check run 9876 on payments/backend".to_owned(),
                provider_linked_row_ref: "provider-linked-row:check-run:payments/backend:9876"
                    .to_owned(),
            },
            provider_declared_scope_refs: vec![
                "scope:repo:payments/backend:ci_run_or_check_mutation".to_owned(),
                "scope:repo:payments/frontend:ci_run_or_check_mutation".to_owned(),
            ],
            resolved_scope_refs: vec![
                "scope:repo:payments/backend:ci_run_or_check_mutation".to_owned()
            ],
            decision: AuthorityDecisionClass::Allowed,
            decision_token: AuthorityDecisionClass::Allowed.as_str().to_owned(),
            resolution_reason: GrantResolutionReasonClass::Allowed,
            resolution_reason_token: GrantResolutionReasonClass::Allowed.as_str().to_owned(),
            decision_summary:
                "Allowed: check-run mutation on payments/backend under the CI bot installation \
                 grant."
                    .to_owned(),
            reapproval_route: ReapprovalRouteClass::NoneRequired,
            reapproval_route_token: ReapprovalRouteClass::NoneRequired.as_str().to_owned(),
            reapproval_route_label: "No reapproval required".to_owned(),
            remediation_path_ref: None,
            approval_ticket_ref: Some("approval-ticket:ci-bot:check-run:9876:beta".to_owned()),
            browser_handoff_packet_ref: None,
            publish_later_queue_item_ref: None,
            computed_at: "2026-05-16T10:10:30Z".to_owned(),
            expires_at: Some("2026-05-16T11:10:30Z".to_owned()),
            raw_token_material_present: false,
            public_endpoint_fallback_taken: false,
            silent_mutation_authority_widened: false,
            local_editing_preserved: true,
        },
        EffectiveScopeResolutionRow {
            record_kind: ACCOUNT_SCOPE_BETA_EFFECTIVE_SCOPE_ROW_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            resolution_id:
                "account-scope-beta:resolution:enterprise_managed:managed-bot:denied-suspended"
                    .to_owned(),
            display_label: "Managed deploy bot mutation denied (grant suspended)".to_owned(),
            profile: AccountScopeBetaProfileClass::EnterpriseManaged,
            profile_token: AccountScopeBetaProfileClass::EnterpriseManaged
                .as_str()
                .to_owned(),
            acting_identity_class: ActingIdentityClass::InstallationGrant,
            acting_identity_class_token: ActingIdentityClass::InstallationGrant.as_str().to_owned(),
            bound_identity_row_ref:
                "account-scope-beta:installation-grant:enterprise_managed:managed-bot".to_owned(),
            requested_action: RequestedActionClass::CiRunOrCheckMutation,
            requested_action_token: RequestedActionClass::CiRunOrCheckMutation
                .as_str()
                .to_owned(),
            target: ProviderTargetIdentity {
                target_ref: "target:check_run:tenant-001/fleet:42".to_owned(),
                target_label: "Check run 42 on tenant-001/fleet".to_owned(),
                provider_linked_row_ref: "provider-linked-row:check-run:tenant-001/fleet:42"
                    .to_owned(),
            },
            provider_declared_scope_refs: vec![
                "scope:repo:tenant-001/fleet:ci_run_or_check_mutation".to_owned(),
            ],
            resolved_scope_refs: vec![],
            decision: AuthorityDecisionClass::Denied,
            decision_token: AuthorityDecisionClass::Denied.as_str().to_owned(),
            resolution_reason: GrantResolutionReasonClass::DeniedSuspended,
            resolution_reason_token: GrantResolutionReasonClass::DeniedSuspended
                .as_str()
                .to_owned(),
            decision_summary:
                "Denied: managed deploy bot grant is suspended by the managed policy authority."
                    .to_owned(),
            reapproval_route: ReapprovalRouteClass::AdminReviewOrTrustGrant,
            reapproval_route_token: ReapprovalRouteClass::AdminReviewOrTrustGrant
                .as_str()
                .to_owned(),
            reapproval_route_label: "Contact workspace admin".to_owned(),
            remediation_path_ref: Some(
                "remediation-path:admin-review:managed-deploy:tenant-001".to_owned(),
            ),
            approval_ticket_ref: None,
            browser_handoff_packet_ref: None,
            publish_later_queue_item_ref: None,
            computed_at: "2026-05-16T10:20:30Z".to_owned(),
            expires_at: None,
            raw_token_material_present: false,
            public_endpoint_fallback_taken: false,
            silent_mutation_authority_widened: false,
            local_editing_preserved: true,
        },
        EffectiveScopeResolutionRow {
            record_kind: ACCOUNT_SCOPE_BETA_EFFECTIVE_SCOPE_ROW_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            resolution_id: "account-scope-beta:resolution:offline:release-signer:local-draft"
                .to_owned(),
            display_label: "Release publish parked as local draft on expired delegated credential"
                .to_owned(),
            profile: AccountScopeBetaProfileClass::Offline,
            profile_token: AccountScopeBetaProfileClass::Offline.as_str().to_owned(),
            acting_identity_class: ActingIdentityClass::DelegatedCredential,
            acting_identity_class_token: ActingIdentityClass::DelegatedCredential
                .as_str()
                .to_owned(),
            bound_identity_row_ref:
                "account-scope-beta:delegated-credential:offline:release-signer".to_owned(),
            requested_action: RequestedActionClass::ReleasePublish,
            requested_action_token: RequestedActionClass::ReleasePublish.as_str().to_owned(),
            target: ProviderTargetIdentity {
                target_ref: "target:release:fleet-0001:v1.2.3".to_owned(),
                target_label: "Release v1.2.3 for fleet-0001".to_owned(),
                provider_linked_row_ref: "provider-linked-row:release:fleet-0001:v1.2.3".to_owned(),
            },
            provider_declared_scope_refs: vec![
                "scope:release:fleet-0001:release_publish".to_owned()
            ],
            resolved_scope_refs: vec![],
            decision: AuthorityDecisionClass::LocalDraftOnly,
            decision_token: AuthorityDecisionClass::LocalDraftOnly.as_str().to_owned(),
            resolution_reason: GrantResolutionReasonClass::AllowedWithDeferredPublish,
            resolution_reason_token: GrantResolutionReasonClass::AllowedWithDeferredPublish
                .as_str()
                .to_owned(),
            decision_summary:
                "Local-draft only: delegated credential expired on the offline profile; release \
                 publish is parked as a publish-later queue item until reissue."
                    .to_owned(),
            reapproval_route: ReapprovalRouteClass::DelegatedCredentialReissue,
            reapproval_route_token: ReapprovalRouteClass::DelegatedCredentialReissue
                .as_str()
                .to_owned(),
            reapproval_route_label: "Re-issue delegated credential".to_owned(),
            remediation_path_ref: Some(
                "remediation-path:delegated-credential-reissue:release-signer:fleet-0001"
                    .to_owned(),
            ),
            approval_ticket_ref: None,
            browser_handoff_packet_ref: None,
            publish_later_queue_item_ref: Some(
                "publish-later-queue-item:release:fleet-0001:v1.2.3".to_owned(),
            ),
            computed_at: "2026-05-16T10:25:30Z".to_owned(),
            expires_at: None,
            raw_token_material_present: false,
            public_endpoint_fallback_taken: false,
            silent_mutation_authority_widened: false,
            local_editing_preserved: true,
        },
    ]
}

fn seed_scope_drift_events() -> Vec<ScopeDriftEvent> {
    vec![
        ScopeDriftEvent {
            record_kind: ACCOUNT_SCOPE_BETA_SCOPE_DRIFT_EVENT_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            event_id: "account-scope-beta:drift:enterprise_managed:managed-bot:suspended"
                .to_owned(),
            profile: AccountScopeBetaProfileClass::EnterpriseManaged,
            profile_token: AccountScopeBetaProfileClass::EnterpriseManaged
                .as_str()
                .to_owned(),
            originating_identity_row_ref:
                "account-scope-beta:installation-grant:enterprise_managed:managed-bot".to_owned(),
            affected_resolution_row_ref:
                "account-scope-beta:resolution:enterprise_managed:managed-bot:denied-suspended"
                    .to_owned(),
            trigger: ScopeDriftTriggerClass::GrantSuspended,
            trigger_token: ScopeDriftTriggerClass::GrantSuspended.as_str().to_owned(),
            forced_downgrade: AuthorityDowngradeClass::ForceAdminReview,
            forced_downgrade_token: AuthorityDowngradeClass::ForceAdminReview
                .as_str()
                .to_owned(),
            reapproval_route: ReapprovalRouteClass::AdminReviewOrTrustGrant,
            reapproval_route_token: ReapprovalRouteClass::AdminReviewOrTrustGrant
                .as_str()
                .to_owned(),
            rationale_summary:
                "Managed policy authority suspended the deploy bot grant; downstream resolutions \
                 are downgraded to admin review and no longer admit mutation authority."
                    .to_owned(),
            observed_at: "2026-05-16T10:20:45Z".to_owned(),
            silent_mutation_authority_retained: false,
            local_editing_preserved: true,
        },
        ScopeDriftEvent {
            record_kind: ACCOUNT_SCOPE_BETA_SCOPE_DRIFT_EVENT_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            event_id: "account-scope-beta:drift:offline:release-signer:credential-expired"
                .to_owned(),
            profile: AccountScopeBetaProfileClass::Offline,
            profile_token: AccountScopeBetaProfileClass::Offline.as_str().to_owned(),
            originating_identity_row_ref:
                "account-scope-beta:delegated-credential:offline:release-signer".to_owned(),
            affected_resolution_row_ref:
                "account-scope-beta:resolution:offline:release-signer:local-draft".to_owned(),
            trigger: ScopeDriftTriggerClass::DelegatedCredentialExpired,
            trigger_token: ScopeDriftTriggerClass::DelegatedCredentialExpired
                .as_str()
                .to_owned(),
            forced_downgrade: AuthorityDowngradeClass::ForceLocalDraftOnly,
            forced_downgrade_token: AuthorityDowngradeClass::ForceLocalDraftOnly
                .as_str()
                .to_owned(),
            reapproval_route: ReapprovalRouteClass::DelegatedCredentialReissue,
            reapproval_route_token: ReapprovalRouteClass::DelegatedCredentialReissue
                .as_str()
                .to_owned(),
            rationale_summary:
                "Delegated credential expired on the offline profile; release publish is forced \
                 to local-draft only until the delegated credential is reissued."
                    .to_owned(),
            observed_at: "2026-05-16T10:25:45Z".to_owned(),
            silent_mutation_authority_retained: false,
            local_editing_preserved: true,
        },
        ScopeDriftEvent {
            record_kind: ACCOUNT_SCOPE_BETA_SCOPE_DRIFT_EVENT_RECORD_KIND.to_owned(),
            schema_version: ACCOUNT_SCOPE_BETA_SCHEMA_VERSION,
            shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
            event_id: "account-scope-beta:drift:mirror_only:human-reviewer:trust-downgraded"
                .to_owned(),
            profile: AccountScopeBetaProfileClass::MirrorOnly,
            profile_token: AccountScopeBetaProfileClass::MirrorOnly.as_str().to_owned(),
            originating_identity_row_ref:
                "account-scope-beta:connected-account:mirror_only:human-reviewer".to_owned(),
            affected_resolution_row_ref:
                "account-scope-beta:resolution:mirror_only:human-reviewer:browser-only-merge"
                    .to_owned(),
            trigger: ScopeDriftTriggerClass::TrustStateDowngraded,
            trigger_token: ScopeDriftTriggerClass::TrustStateDowngraded
                .as_str()
                .to_owned(),
            forced_downgrade: AuthorityDowngradeClass::ForceBrowserHandoffOnly,
            forced_downgrade_token: AuthorityDowngradeClass::ForceBrowserHandoffOnly
                .as_str()
                .to_owned(),
            reapproval_route: ReapprovalRouteClass::BrowserHandoff,
            reapproval_route_token: ReapprovalRouteClass::BrowserHandoff.as_str().to_owned(),
            rationale_summary:
                "Workspace trust downgraded under the reviewer; mirror-only review-decision \
                 publish is forced through a browser handoff."
                    .to_owned(),
            observed_at: "2026-05-16T10:05:45Z".to_owned(),
            silent_mutation_authority_retained: false,
            local_editing_preserved: true,
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seeded_page_validates_with_zero_defects() {
        let page = seeded_account_scope_beta_page();
        validate_account_scope_beta_page(&page).expect("seeded page validates");
        assert!(page.defects.is_empty());
        for profile in AccountScopeBetaProfileClass::ALL {
            assert!(page
                .summary
                .profiles_present
                .iter()
                .any(|token| token == profile.as_str()));
        }
    }

    #[test]
    fn seeded_page_separates_three_identity_classes() {
        let page = seeded_account_scope_beta_page();
        assert!(!page.connected_account_rows.is_empty());
        assert!(!page.installation_grant_rows.is_empty());
        assert!(!page.delegated_credential_rows.is_empty());
        let classes: BTreeSet<&str> = page
            .summary
            .identity_classes_present
            .iter()
            .map(String::as_str)
            .collect();
        assert!(classes.contains("connected_account"));
        assert!(classes.contains("installation_grant"));
        assert!(classes.contains("delegated_credential"));
    }

    #[test]
    fn seeded_page_includes_allowed_and_drift_forced_downgrade() {
        let page = seeded_account_scope_beta_page();
        let decisions: BTreeSet<&str> = page
            .summary
            .decisions_present
            .iter()
            .map(String::as_str)
            .collect();
        assert!(decisions.contains("allowed"));
        assert!(decisions.contains("denied"));
        assert!(decisions.contains("browser_only"));
        assert!(decisions.contains("local_draft_only"));

        let downgrades: BTreeSet<&str> = page
            .summary
            .forced_downgrades_present
            .iter()
            .map(String::as_str)
            .collect();
        assert!(downgrades.contains("force_admin_review"));
        assert!(downgrades.contains("force_local_draft_only"));
    }

    #[test]
    fn seeded_page_projects_m5_secret_boundary_state() {
        let page = seeded_account_scope_beta_page();
        let states = page.secret_boundary_states();
        assert_eq!(states.len(), 1);
        assert_eq!(
            states[0].matrix_row_id,
            "m5.secret.provider_model.scope_registry"
        );
        assert!(states[0].delegated_credential_row.is_some());
        assert!(!states[0].export_safety_banner.raw_secret_values_included);
    }

    #[test]
    fn validator_flags_silent_mutation_widening() {
        let mut page = seeded_account_scope_beta_page();
        page.resolution_rows[0].silent_mutation_authority_widened = true;
        let defects = audit_account_scope_beta_page(
            &page.connected_account_rows,
            &page.installation_grant_rows,
            &page.delegated_credential_rows,
            &page.resolution_rows,
            &page.scope_drift_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == AccountScopeBetaDefectKind::SilentMutationAuthorityWidened));
    }

    #[test]
    fn validator_flags_raw_token_material_on_identity_row() {
        let mut page = seeded_account_scope_beta_page();
        page.installation_grant_rows[0].raw_token_material_present = true;
        let defects = audit_account_scope_beta_page(
            &page.connected_account_rows,
            &page.installation_grant_rows,
            &page.delegated_credential_rows,
            &page.resolution_rows,
            &page.scope_drift_events,
        );
        assert!(defects.iter().any(
            |defect| defect.defect_kind == AccountScopeBetaDefectKind::RawTokenMaterialPresent
        ));
    }

    #[test]
    fn validator_flags_allowed_decision_on_closed_lifecycle() {
        let mut page = seeded_account_scope_beta_page();
        let row = page
            .installation_grant_rows
            .iter_mut()
            .find(|row| {
                row.identity_row_id
                    == "account-scope-beta:installation-grant:enterprise_managed:managed-bot"
            })
            .expect("seeded managed-bot row");
        row.lifecycle_state = InstallationGrantLifecycleStateClass::Suspended;
        row.lifecycle_state_token = InstallationGrantLifecycleStateClass::Suspended
            .as_str()
            .to_owned();
        let resolution = page
            .resolution_rows
            .iter_mut()
            .find(|row| {
                row.resolution_id
                    == "account-scope-beta:resolution:enterprise_managed:managed-bot:denied-suspended"
            })
            .expect("seeded denied resolution");
        resolution.decision = AuthorityDecisionClass::Allowed;
        resolution.decision_token = AuthorityDecisionClass::Allowed.as_str().to_owned();
        resolution.resolution_reason = GrantResolutionReasonClass::Allowed;
        resolution.resolution_reason_token =
            GrantResolutionReasonClass::Allowed.as_str().to_owned();
        resolution.approval_ticket_ref =
            Some("approval-ticket:managed-bot:check-run:42:beta".to_owned());
        resolution.resolved_scope_refs =
            vec!["scope:repo:tenant-001/fleet:ci_run_or_check_mutation".to_owned()];
        resolution.reapproval_route = ReapprovalRouteClass::NoneRequired;
        resolution.reapproval_route_token = ReapprovalRouteClass::NoneRequired.as_str().to_owned();
        resolution.remediation_path_ref = None;

        let defects = audit_account_scope_beta_page(
            &page.connected_account_rows,
            &page.installation_grant_rows,
            &page.delegated_credential_rows,
            &page.resolution_rows,
            &page.scope_drift_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == AccountScopeBetaDefectKind::AllowedDecisionOnClosedLifecycle));
    }

    #[test]
    fn validator_flags_resolved_scope_not_subset_of_declared() {
        let mut page = seeded_account_scope_beta_page();
        let resolution = page
            .resolution_rows
            .iter_mut()
            .find(|row| {
                row.resolution_id
                    == "account-scope-beta:resolution:connected:human-dev:comment-allowed"
            })
            .expect("seeded allowed resolution");
        resolution.resolved_scope_refs =
            vec!["scope:repo:payments/backend:release_publish".to_owned()];

        let defects = audit_account_scope_beta_page(
            &page.connected_account_rows,
            &page.installation_grant_rows,
            &page.delegated_credential_rows,
            &page.resolution_rows,
            &page.scope_drift_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == AccountScopeBetaDefectKind::ResolvedScopeNotSubsetOfDeclared));
    }

    #[test]
    fn validator_flags_drift_event_without_downgrade() {
        let mut page = seeded_account_scope_beta_page();
        page.scope_drift_events[0].forced_downgrade = AuthorityDowngradeClass::NoDowngradeRequired;
        page.scope_drift_events[0].forced_downgrade_token =
            AuthorityDowngradeClass::NoDowngradeRequired
                .as_str()
                .to_owned();
        let defects = audit_account_scope_beta_page(
            &page.connected_account_rows,
            &page.installation_grant_rows,
            &page.delegated_credential_rows,
            &page.resolution_rows,
            &page.scope_drift_events,
        );
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == AccountScopeBetaDefectKind::DriftEventDowngradeMissing));
    }

    #[test]
    fn validator_flags_drift_event_silently_retained_authority() {
        let mut page = seeded_account_scope_beta_page();
        page.scope_drift_events[0].silent_mutation_authority_retained = true;
        let defects = audit_account_scope_beta_page(
            &page.connected_account_rows,
            &page.installation_grant_rows,
            &page.delegated_credential_rows,
            &page.resolution_rows,
            &page.scope_drift_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == AccountScopeBetaDefectKind::SilentMutationAuthorityRetainedAfterDrift));
    }

    #[test]
    fn validator_flags_bound_identity_class_mismatch() {
        let mut page = seeded_account_scope_beta_page();
        let resolution = page
            .resolution_rows
            .iter_mut()
            .find(|row| {
                row.resolution_id
                    == "account-scope-beta:resolution:connected:human-dev:comment-allowed"
            })
            .expect("seeded comment resolution");
        resolution.acting_identity_class = ActingIdentityClass::InstallationGrant;
        resolution.acting_identity_class_token =
            ActingIdentityClass::InstallationGrant.as_str().to_owned();
        let defects = audit_account_scope_beta_page(
            &page.connected_account_rows,
            &page.installation_grant_rows,
            &page.delegated_credential_rows,
            &page.resolution_rows,
            &page.scope_drift_events,
        );
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == AccountScopeBetaDefectKind::BoundIdentityClassMismatch));
    }

    #[test]
    fn validator_flags_browser_only_without_handoff_ref() {
        let mut page = seeded_account_scope_beta_page();
        let resolution = page
            .resolution_rows
            .iter_mut()
            .find(|row| row.decision == AuthorityDecisionClass::BrowserOnly)
            .expect("seeded browser_only resolution");
        resolution.browser_handoff_packet_ref = None;
        let defects = audit_account_scope_beta_page(
            &page.connected_account_rows,
            &page.installation_grant_rows,
            &page.delegated_credential_rows,
            &page.resolution_rows,
            &page.scope_drift_events,
        );
        assert!(defects
            .iter()
            .any(|defect| defect.defect_kind
                == AccountScopeBetaDefectKind::BrowserOnlyWithoutHandoffRef));
    }

    #[test]
    fn validator_flags_profile_coverage_missing() {
        let mut page = seeded_account_scope_beta_page();
        page.connected_account_rows
            .retain(|row| row.profile != AccountScopeBetaProfileClass::MirrorOnly);
        page.installation_grant_rows
            .retain(|row| row.profile != AccountScopeBetaProfileClass::MirrorOnly);
        page.delegated_credential_rows
            .retain(|row| row.profile != AccountScopeBetaProfileClass::MirrorOnly);
        page.resolution_rows
            .retain(|row| row.profile != AccountScopeBetaProfileClass::MirrorOnly);
        let defects = audit_account_scope_beta_page(
            &page.connected_account_rows,
            &page.installation_grant_rows,
            &page.delegated_credential_rows,
            &page.resolution_rows,
            &page.scope_drift_events,
        );
        assert!(defects.iter().any(|defect| defect.defect_kind
            == AccountScopeBetaDefectKind::ProfileCoverageMissing
            && defect.note.contains("mirror_only")));
    }

    #[test]
    fn support_export_round_trip_preserves_lineage() {
        let page = seeded_account_scope_beta_page();
        let export = AccountScopeBetaSupportExport::from_page(
            "account-scope-beta:support-export:001",
            "2026-05-16T11:00:00Z",
            page,
        );
        assert!(export.raw_tokens_excluded);
        assert!(export.identity_lineage_preserved);
        assert!(export.resolution_lineage_preserved);
        assert!(export.drift_lineage_preserved);
        assert!(export.fail_closed_invariant);
        assert!(export.defect_kinds_present.is_empty());
    }

    #[test]
    fn summary_counts_match_records() {
        let page = seeded_account_scope_beta_page();
        assert_eq!(
            page.summary.connected_account_row_count,
            page.connected_account_rows.len()
        );
        assert_eq!(
            page.summary.installation_grant_row_count,
            page.installation_grant_rows.len()
        );
        assert_eq!(
            page.summary.delegated_credential_row_count,
            page.delegated_credential_rows.len()
        );
        assert_eq!(
            page.summary.resolution_row_count,
            page.resolution_rows.len()
        );
        assert_eq!(
            page.summary.scope_drift_event_count,
            page.scope_drift_events.len()
        );
        let decision_total: usize = page.summary.resolutions_by_decision.values().sum();
        assert_eq!(decision_total, page.resolution_rows.len());
        let drift_total: usize = page.summary.drift_events_by_trigger.values().sum();
        assert_eq!(drift_total, page.scope_drift_events.len());
    }
}
