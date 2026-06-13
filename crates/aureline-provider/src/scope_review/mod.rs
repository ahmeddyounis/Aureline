//! Canonical M5 provider scope-review sheets, least-privilege fallbacks, and
//! invalidation truth.
//!
//! This module closes the gap between account-scope identity rows and the
//! provider-backed work-item and review surfaces that consume them. It turns
//! effective provider authority into one reusable decision object that desktop,
//! CLI/headless, companion, and support/export surfaces can quote without
//! inventing local wording.
//!
//! Each review page keeps five facts explicit:
//!
//! - the acting identity and its health state;
//! - the requested action, target object, and effective scope refs;
//! - the policy locks and trust posture applied on top of provider-declared
//!   scopes;
//! - the least-privilege alternatives still available when the requested write
//!   path is blocked, narrowed, browser-only, or deferred; and
//! - the typed invalidation events that must degrade cached authority instead
//!   of silently reusing it after revocation, suspension, host mismatch,
//!   membership loss, or tenant switch.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::account_scope::ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF;
use crate::registry::{ProviderActorClass, ProviderFamily, RedactionClass};
use crate::route_resolution::ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF;
use crate::work_item_sync::WORK_ITEM_SYNC_BETA_SHARED_CONTRACT_REF;
use crate::work_items::{
    TrustPosture, WorkItemMutationMode, WORK_ITEM_TRANSITION_BETA_SHARED_CONTRACT_REF,
};

/// Schema version exported by scope-review records.
pub const PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every scope-review record.
pub const PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF: &str = "providers:scope_review:v1";

/// Stable record kind for [`ProviderScopeReviewPage`].
pub const PROVIDER_SCOPE_REVIEW_PAGE_RECORD_KIND: &str = "providers_scope_review_page_record";

/// Stable record kind for [`ProviderScopeResolutionRecord`].
pub const PROVIDER_SCOPE_REVIEW_RESOLUTION_RECORD_KIND: &str =
    "provider_scope_review_resolution_record";

/// Stable record kind for [`LeastPrivilegeAlternativeRecord`].
pub const PROVIDER_SCOPE_REVIEW_ALTERNATIVE_RECORD_KIND: &str =
    "provider_scope_review_alternative_record";

/// Stable record kind for [`EffectiveScopeInvalidationEventRecord`].
pub const PROVIDER_SCOPE_REVIEW_INVALIDATION_RECORD_KIND: &str =
    "provider_scope_review_invalidation_record";

/// Stable record kind for [`ScopeReviewConsumerProjectionRecord`].
pub const PROVIDER_SCOPE_REVIEW_CONSUMER_PROJECTION_RECORD_KIND: &str =
    "provider_scope_review_consumer_projection_record";

/// Stable record kind for [`ProviderScopeReviewSummary`].
pub const PROVIDER_SCOPE_REVIEW_SUMMARY_RECORD_KIND: &str = "providers_scope_review_summary_record";

/// Stable record kind for [`ProviderScopeReviewDefect`].
pub const PROVIDER_SCOPE_REVIEW_DEFECT_RECORD_KIND: &str = "providers_scope_review_defect_record";

/// Stable record kind for [`ProviderScopeReviewValidationReport`].
pub const PROVIDER_SCOPE_REVIEW_VALIDATION_REPORT_RECORD_KIND: &str =
    "providers_scope_review_validation_report";

/// Stable record kind for [`ProviderScopeReviewSupportExport`].
pub const PROVIDER_SCOPE_REVIEW_SUPPORT_EXPORT_RECORD_KIND: &str =
    "providers_scope_review_support_export_record";

/// Stable schema reference for the page-level review packet.
pub const PROVIDER_SCOPE_REVIEW_SCHEMA_REF: &str =
    "schemas/providers/provider_scope_review.schema.json";

/// Cross-tool boundary schema reference for the decision vocabulary.
pub const PROVIDER_SCOPE_REVIEW_EFFECTIVE_SCOPE_SCHEMA_REF: &str =
    "schemas/providers/effective_scope_resolution.schema.json";

/// Fixture directory for checked review packets.
pub const PROVIDER_SCOPE_REVIEW_FIXTURE_DIR: &str = "fixtures/providers/m5/provider_scope_review";

/// Markdown artifact summarizing the seeded page.
pub const PROVIDER_SCOPE_REVIEW_ARTIFACT_REF: &str =
    "artifacts/provider/m5/provider_scope_review.md";

/// Support-export artifact generated from the seeded page.
pub const PROVIDER_SCOPE_REVIEW_SUPPORT_EXPORT_ARTIFACT_REF: &str =
    "artifacts/provider/m5/provider_scope_review/support_export.json";

/// Provider class carried by one scope-review target.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReviewProviderClass {
    /// Code-host or pull-request provider.
    CodeHost,
    /// Issue or planning tracker.
    IssueTracker,
    /// CI or checks provider.
    CiChecks,
    /// Release publishing surface.
    ReleasePublisher,
    /// Documentation or portal surface.
    DocsOrPortal,
    /// Managed admin surface.
    ManagedAdmin,
    /// Package or artifact registry.
    ArtifactRegistry,
    /// Generic other provider class.
    Other,
}

impl ScopeReviewProviderClass {
    /// Returns the stable export token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeHost => "code_host",
            Self::IssueTracker => "issue_tracker",
            Self::CiChecks => "ci_checks",
            Self::ReleasePublisher => "release_publisher",
            Self::DocsOrPortal => "docs_or_portal",
            Self::ManagedAdmin => "managed_admin",
            Self::ArtifactRegistry => "artifact_registry",
            Self::Other => "other",
        }
    }
}

/// Target object class a resolution adjudicates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReviewTargetObjectClass {
    /// Pull request, merge request, or changeset.
    PullRequest,
    /// Issue, task, incident, or planning item.
    IssueOrWorkItem,
    /// Check run or CI run.
    CheckRun,
    /// Release or publication artifact.
    ReleaseArtifact,
    /// Docs page or portal entry.
    DocsPage,
    /// Admin or consent flow surface.
    AdminSurface,
    /// Package version or registry entry.
    PackageVersion,
    /// Principal or identity subject.
    PrincipalSubject,
    /// Install target such as repo or org install.
    InstallTarget,
    /// Tenant or organization boundary object.
    TenantOrOrg,
    /// Generic other object.
    Other,
}

impl ScopeReviewTargetObjectClass {
    /// Returns the stable export token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PullRequest => "pull_request",
            Self::IssueOrWorkItem => "issue_or_work_item",
            Self::CheckRun => "check_run",
            Self::ReleaseArtifact => "release_artifact",
            Self::DocsPage => "docs_page",
            Self::AdminSurface => "admin_surface",
            Self::PackageVersion => "package_version",
            Self::PrincipalSubject => "principal_subject",
            Self::InstallTarget => "install_target",
            Self::TenantOrOrg => "tenant_or_org",
            Self::Other => "other",
        }
    }
}

/// Health class for the acting identity or authority the review uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReviewAuthorityHealthClass {
    /// Authority is healthy and within freshness floor.
    Healthy,
    /// Authority is nearing expiry and may require step-up or reauth.
    Expiring,
    /// Authority was revoked.
    Revoked,
    /// Authority was suspended.
    Suspended,
    /// Authority exists but its scope narrowed.
    ScopeNarrowed,
    /// Authority cannot currently reach the provider.
    Unreachable,
    /// Authority no longer matches the reviewed host.
    HostMismatch,
    /// Authority lost required org membership.
    OrgMembershipLost,
    /// Authority switched tenants and must be reselected.
    TenantSwitchDetected,
}

impl ScopeReviewAuthorityHealthClass {
    /// Returns the stable export token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Healthy => "healthy",
            Self::Expiring => "expiring",
            Self::Revoked => "revoked",
            Self::Suspended => "suspended",
            Self::ScopeNarrowed => "scope_narrowed",
            Self::Unreachable => "unreachable",
            Self::HostMismatch => "host_mismatch",
            Self::OrgMembershipLost => "org_membership_lost",
            Self::TenantSwitchDetected => "tenant_switch_detected",
        }
    }

    /// True when this health class holds mutation authority closed.
    pub const fn holds_mutation_closed(self) -> bool {
        !matches!(self, Self::Healthy)
    }
}

/// Requested action class the scope review adjudicates.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReviewRequestedActionClass {
    /// Read-only inspection.
    ReadOnlyInspection,
    /// Human-authored comment or note.
    HumanAuthoredComment,
    /// Merge, approve, or close action.
    ReviewDecisionPublish,
    /// Issue or work-item mutation.
    IssueOrWorkItemMutation,
    /// CI or check mutation.
    CiRunOrCheckMutation,
    /// Docs or portal publish.
    DocsOrPortalPublish,
    /// Package publish.
    PackagePublish,
    /// Release publish.
    ReleasePublish,
    /// Consent or admin delegation.
    ConsentOrAdminDelegation,
    /// Credential projection or scope inspection.
    CredentialProjection,
}

impl ScopeReviewRequestedActionClass {
    /// Returns the stable export token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnlyInspection => "read_only_inspection",
            Self::HumanAuthoredComment => "human_authored_comment",
            Self::ReviewDecisionPublish => "review_decision_publish",
            Self::IssueOrWorkItemMutation => "issue_or_work_item_mutation",
            Self::CiRunOrCheckMutation => "ci_run_or_check_mutation",
            Self::DocsOrPortalPublish => "docs_or_portal_publish",
            Self::PackagePublish => "package_publish",
            Self::ReleasePublish => "release_publish",
            Self::ConsentOrAdminDelegation => "consent_or_admin_delegation",
            Self::CredentialProjection => "credential_projection",
        }
    }

    /// True when the action proposes a write to provider-owned state.
    pub const fn proposes_mutation(self) -> bool {
        !matches!(self, Self::ReadOnlyInspection | Self::CredentialProjection)
    }
}

/// Top-level effective-scope decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReviewDecisionClass {
    /// The reviewed provider write is admitted.
    Allowed,
    /// The reviewed provider write is denied.
    Denied,
    /// The action may proceed only through a browser-owned surface.
    BrowserOnly,
    /// The action may proceed only as a local draft or deferred queue item.
    LocalDraftOnly,
}

impl ScopeReviewDecisionClass {
    /// Returns the stable export token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Allowed => "allowed",
            Self::Denied => "denied",
            Self::BrowserOnly => "browser_only",
            Self::LocalDraftOnly => "local_draft_only",
        }
    }

    /// True when the decision admits a direct provider write.
    pub const fn admits_direct_provider_write(self) -> bool {
        matches!(self, Self::Allowed)
    }
}

/// Resolution reason attached to one decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReviewGrantResolutionReasonClass {
    /// Resolution allowed without narrowing.
    Allowed,
    /// Resolution allowed with narrower effective scope.
    AllowedWithDowngrade,
    /// Resolution allowed only through browser handoff.
    AllowedWithBrowserHandoff,
    /// Resolution allowed only through deferred publish or local draft.
    AllowedWithDeferredPublish,
    /// Required provider scope is missing.
    DeniedScopeMissing,
    /// Policy bundle forbids the action.
    DeniedPolicyBundle,
    /// Workspace trust blocks the action.
    DeniedWorkspaceTrust,
    /// The acting actor class is forbidden.
    DeniedActorClassForbidden,
    /// The reviewed target drifted and must be compared again.
    DeniedTargetConflict,
    /// Cached truth is past its freshness floor.
    DeniedFreshnessFloor,
    /// Underlying authority was revoked.
    DeniedRevoked,
    /// Underlying authority was suspended.
    DeniedSuspended,
    /// Host binding no longer matches the reviewed target.
    DeniedHostMismatch,
    /// Approval ticket is missing.
    DeniedApprovalTicketMissing,
    /// Approval ticket expired or was revoked.
    DeniedApprovalTicketExpired,
    /// Step-up authentication is required.
    DeniedStepUpRequired,
    /// Provider route is unavailable or unreachable.
    DeniedUnreachable,
    /// Actor class could not be resolved safely.
    DeniedUnknownActorClass,
}

impl ScopeReviewGrantResolutionReasonClass {
    /// Returns the stable export token.
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
            Self::DeniedTargetConflict => "denied_target_conflict",
            Self::DeniedFreshnessFloor => "denied_freshness_floor",
            Self::DeniedRevoked => "denied_revoked",
            Self::DeniedSuspended => "denied_suspended",
            Self::DeniedHostMismatch => "denied_host_mismatch",
            Self::DeniedApprovalTicketMissing => "denied_approval_ticket_missing",
            Self::DeniedApprovalTicketExpired => "denied_approval_ticket_expired",
            Self::DeniedStepUpRequired => "denied_step_up_required",
            Self::DeniedUnreachable => "denied_unreachable",
            Self::DeniedUnknownActorClass => "denied_unknown_actor_class",
        }
    }

    /// True when the reason belongs to the allowed family.
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

/// Policy lock applied over provider-declared scopes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReviewPolicyLockClass {
    /// No local policy lock was active.
    NoLocalPolicyLock,
    /// Policy bundle forbids this action.
    PolicyBundleForbidsAction,
    /// Policy bundle narrows the admitted actor class.
    PolicyBundleNarrowsActorClass,
    /// Policy bundle requires browser-only completion.
    PolicyBundleRequiresBrowserHandoff,
    /// Policy bundle requires deferred publish.
    PolicyBundleRequiresDeferredPublish,
    /// Policy bundle requires step-up auth.
    PolicyBundleRequiresStepUp,
    /// Workspace trust blocks the action.
    WorkspaceTrustRestrictedLock,
    /// Managed admin boundary blocks the action.
    ManagedProviderAdminLock,
    /// Release publisher boundary blocks the action.
    ReleasePublisherLock,
    /// Credential projection boundary blocks the action.
    CredentialProjectionLock,
}

impl ScopeReviewPolicyLockClass {
    /// Returns the stable export token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoLocalPolicyLock => "no_local_policy_lock",
            Self::PolicyBundleForbidsAction => "policy_bundle_forbids_action",
            Self::PolicyBundleNarrowsActorClass => "policy_bundle_narrows_actor_class",
            Self::PolicyBundleRequiresBrowserHandoff => "policy_bundle_requires_browser_handoff",
            Self::PolicyBundleRequiresDeferredPublish => "policy_bundle_requires_deferred_publish",
            Self::PolicyBundleRequiresStepUp => "policy_bundle_requires_step_up",
            Self::WorkspaceTrustRestrictedLock => "workspace_trust_restricted_lock",
            Self::ManagedProviderAdminLock => "managed_provider_admin_lock",
            Self::ReleasePublisherLock => "release_publisher_lock",
            Self::CredentialProjectionLock => "credential_projection_lock",
        }
    }
}

/// Least-privilege alternative offered beside a non-green decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReviewAlternativeClass {
    /// Ask for a narrower scope.
    NarrowerScopeRequest,
    /// Switch actor class or account.
    SwitchActorClass,
    /// Keep the action inspect-only.
    SwitchToInspectOnly,
    /// Keep the action as a local draft.
    SwitchToLocalDraft,
    /// Keep the action as deferred publish.
    SwitchToDeferredPublish,
    /// Complete the action through browser handoff.
    RouteThroughBrowserHandoff,
    /// Route through admin delegation.
    RouteThroughAdminDelegation,
    /// Request a step-up authenticator.
    RequestStepUpAuthenticator,
    /// Request a workspace-trust grant.
    RequestWorkspaceTrustGrant,
    /// Request admin review.
    RequestAdminReview,
    /// Copy or export evidence instead of mutating.
    CopyOrExportEvidence,
    /// No least-privilege alternative remains.
    NoAlternativeAvailable,
}

impl ScopeReviewAlternativeClass {
    /// Returns the stable export token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NarrowerScopeRequest => "narrower_scope_request",
            Self::SwitchActorClass => "switch_actor_class",
            Self::SwitchToInspectOnly => "switch_to_inspect_only",
            Self::SwitchToLocalDraft => "switch_to_local_draft",
            Self::SwitchToDeferredPublish => "switch_to_deferred_publish",
            Self::RouteThroughBrowserHandoff => "route_through_browser_handoff",
            Self::RouteThroughAdminDelegation => "route_through_admin_delegation",
            Self::RequestStepUpAuthenticator => "request_step_up_authenticator",
            Self::RequestWorkspaceTrustGrant => "request_workspace_trust_grant",
            Self::RequestAdminReview => "request_admin_review",
            Self::CopyOrExportEvidence => "copy_or_export_evidence",
            Self::NoAlternativeAvailable => "no_alternative_available",
        }
    }
}

/// Freshness state of cached provider truth a decision depends on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReviewStalenessClass {
    /// Fresh provider truth.
    Fresh,
    /// Stale but still bounded and explicitly reviewed.
    BoundedStale,
    /// Stale beyond the admitted review window.
    UnboundedStale,
}

impl ScopeReviewStalenessClass {
    /// Returns the stable export token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::BoundedStale => "bounded_stale",
            Self::UnboundedStale => "unbounded_stale",
        }
    }
}

/// Trigger that invalidates cached effective-scope truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReviewInvalidationTriggerClass {
    /// Underlying actor class or grant was revoked.
    ActorClassRevoked,
    /// Underlying actor class or grant was suspended.
    ActorClassSuspended,
    /// Acting actor class changed.
    ActorClassChanged,
    /// Delegated credential expired.
    DelegatedCredentialExpired,
    /// Host mismatch invalidated the decision.
    HostMismatchDetected,
    /// Tenant switch invalidated the decision.
    TenantSwitchDetected,
    /// Required org membership was lost.
    OrgMembershipLost,
    /// Policy epoch rolled.
    PolicyEpochRolled,
    /// Workspace trust degraded.
    TrustStateDowngraded,
    /// Freshness floor drifted.
    FreshnessFloorDrifted,
    /// Approval ticket was revoked or expired.
    ApprovalTicketRevoked,
    /// Provider health degraded.
    ProviderHealthDegraded,
}

impl ScopeReviewInvalidationTriggerClass {
    /// Returns the stable export token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActorClassRevoked => "actor_class_revoked",
            Self::ActorClassSuspended => "actor_class_suspended",
            Self::ActorClassChanged => "actor_class_changed",
            Self::DelegatedCredentialExpired => "delegated_credential_expired",
            Self::HostMismatchDetected => "host_mismatch_detected",
            Self::TenantSwitchDetected => "tenant_switch_detected",
            Self::OrgMembershipLost => "org_membership_lost",
            Self::PolicyEpochRolled => "policy_epoch_rolled",
            Self::TrustStateDowngraded => "trust_state_downgraded",
            Self::FreshnessFloorDrifted => "freshness_floor_drifted",
            Self::ApprovalTicketRevoked => "approval_ticket_revoked",
            Self::ProviderHealthDegraded => "provider_health_degraded",
        }
    }
}

/// Downgrade action a surface must render after invalidation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReviewDowngradeActionClass {
    /// No downgrade was required.
    NoDowngradeRequired,
    /// Force inspect-only until repair completes.
    ForceInspectOnlyUntilRepair,
    /// Force local-draft only until repair completes.
    ForceLocalDraftOnlyUntilRepair,
    /// Force browser-handoff only until repair completes.
    ForceBrowserHandoffOnlyUntilRepair,
    /// Force a step-up authenticator.
    ForceStepUpAuthenticator,
    /// Force account reselection.
    ForceAccountReselection,
    /// Force admin review.
    ForceAdminReview,
    /// Force disconnect until repair.
    ForceDisconnectUntilRepair,
}

impl ScopeReviewDowngradeActionClass {
    /// Returns the stable export token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDowngradeRequired => "no_downgrade_required",
            Self::ForceInspectOnlyUntilRepair => "force_inspect_only_until_repair",
            Self::ForceLocalDraftOnlyUntilRepair => "force_local_draft_only_until_repair",
            Self::ForceBrowserHandoffOnlyUntilRepair => "force_browser_handoff_only_until_repair",
            Self::ForceStepUpAuthenticator => "force_step_up_authenticator",
            Self::ForceAccountReselection => "force_account_reselection",
            Self::ForceAdminReview => "force_admin_review",
            Self::ForceDisconnectUntilRepair => "force_disconnect_until_repair",
        }
    }
}

/// Consumer surface that must mirror the same decision object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeReviewSurfaceClass {
    /// Desktop review or work-item surface.
    Desktop,
    /// CLI or headless explain surface.
    CliHeadless,
    /// Companion surface.
    Companion,
    /// Support export or support-center surface.
    SupportExport,
}

impl ScopeReviewSurfaceClass {
    /// Returns the stable export token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::CliHeadless => "cli_headless",
            Self::Companion => "companion",
            Self::SupportExport => "support_export",
        }
    }
}

/// Fixture metadata recorded on the seeded page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderScopeReviewFixtureMetadata {
    /// Fixture name.
    pub name: String,
    /// Redaction-safe scenario summary.
    pub scenario: String,
}

/// Upstream contracts consumed by the scope-review page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderScopeReviewContractRefs {
    /// Page-level schema reference.
    pub provider_scope_review_schema_ref: String,
    /// Boundary schema for effective-scope records.
    pub effective_scope_resolution_schema_ref: String,
    /// Account-scope contract reference.
    pub account_scope_beta_shared_contract_ref: String,
    /// Route-resolution contract reference.
    pub route_resolution_beta_shared_contract_ref: String,
    /// Work-item transition contract reference.
    pub work_item_transition_beta_shared_contract_ref: String,
    /// Work-item sync contract reference.
    pub work_item_sync_beta_shared_contract_ref: String,
}

impl ProviderScopeReviewContractRefs {
    fn all_refs(&self) -> [&str; 6] {
        [
            &self.provider_scope_review_schema_ref,
            &self.effective_scope_resolution_schema_ref,
            &self.account_scope_beta_shared_contract_ref,
            &self.route_resolution_beta_shared_contract_ref,
            &self.work_item_transition_beta_shared_contract_ref,
            &self.work_item_sync_beta_shared_contract_ref,
        ]
    }
}

/// Provider-side identity for the reviewed target object.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeReviewTargetObjectIdentity {
    /// Provider class the target belongs to.
    pub provider_class: ScopeReviewProviderClass,
    /// Target object class.
    pub target_object_class: ScopeReviewTargetObjectClass,
    /// Opaque provider-side object id.
    pub provider_side_id: String,
    /// Opaque provider host ref.
    pub provider_host_ref: String,
    /// Opaque tenant, org, or project scope ref.
    pub tenant_or_org_scope_ref: String,
    /// Reviewable target label safe for support export.
    pub target_label: String,
}

/// Policy context used when the decision was computed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeReviewPolicyContext {
    /// Policy epoch ref.
    pub policy_epoch: String,
    /// Workspace trust posture.
    pub trust_posture: TrustPosture,
    /// Execution context id.
    pub execution_context_id: String,
    /// Optional policy block ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub policy_block_ref: Option<String>,
}

/// Freshness block for provider truth the decision depends on.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeReviewFreshnessBlock {
    /// Observation time of the provider truth.
    pub observed_at: String,
    /// Optional freshness-floor ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness_floor_ref: Option<String>,
    /// Staleness class.
    pub staleness_class: ScopeReviewStalenessClass,
    /// Optional rationale when the block is stale.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub staleness_rationale: Option<String>,
}

/// One least-privilege fallback attached to a reviewed decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LeastPrivilegeAlternativeRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque alternative id.
    pub alternative_id: String,
    /// Resolution the alternative belongs to.
    pub originating_resolution_id: String,
    /// Alternative class.
    pub alternative_class: ScopeReviewAlternativeClass,
    /// Reviewable summary safe for UI and support export.
    pub alternative_summary: String,
    /// Suggested actor class when the alternative changes authority.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_actor_class: Option<ProviderActorClass>,
    /// Suggested mutation mode for the alternative.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub suggested_mutation_mode: Option<WorkItemMutationMode>,
    /// Narrowed scope refs the alternative would use.
    #[serde(default)]
    pub narrowed_scope_refs: Vec<String>,
    /// Optional repair hook ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_hook_ref: Option<String>,
    /// Redaction class.
    pub redaction_class: RedactionClass,
}

/// Canonical effective-scope decision object reused across consumer surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderScopeResolutionRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque resolution id.
    pub resolution_id: String,
    /// Connected-provider record ref.
    pub connected_provider_record_id: String,
    /// Provider family for the owning connection.
    pub provider_family: ProviderFamily,
    /// Acting actor class.
    pub actor_class: ProviderActorClass,
    /// Opaque actor subject or grant ref.
    pub actor_subject_ref: String,
    /// Reviewable actor label.
    pub actor_display_label: String,
    /// Authority health at review time.
    pub authority_health_class: ScopeReviewAuthorityHealthClass,
    /// Requested action.
    pub requested_action_class: ScopeReviewRequestedActionClass,
    /// Requested mutation mode.
    pub requested_mutation_mode: WorkItemMutationMode,
    /// Reviewed target object.
    pub target_object_identity: ScopeReviewTargetObjectIdentity,
    /// Provider-declared scope refs currently held by the actor.
    pub provider_declared_scope_refs: Vec<String>,
    /// Effective scope refs the action would actually use.
    pub effective_scope_refs: Vec<String>,
    /// Policy locks applied on top of provider scope.
    pub policy_lock_classes: Vec<ScopeReviewPolicyLockClass>,
    /// Trust posture quoted at the decision site.
    pub trust_posture: TrustPosture,
    /// Decision class.
    pub decision_class: ScopeReviewDecisionClass,
    /// Resolution reason.
    pub grant_resolution_reason: ScopeReviewGrantResolutionReasonClass,
    /// Reviewable decision summary.
    pub decision_summary: String,
    /// Optional freshness block.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub freshness: Option<ScopeReviewFreshnessBlock>,
    /// Policy context.
    pub policy_context: ScopeReviewPolicyContext,
    /// Optional approval-ticket ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub approval_ticket_ref: Option<String>,
    /// Optional browser-handoff ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub browser_handoff_packet_ref: Option<String>,
    /// Optional publish-later queue item ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub publish_later_queue_item_ref: Option<String>,
    /// Alternative refs surfaces may offer instead of widening scope.
    #[serde(default)]
    pub least_privilege_alternative_refs: Vec<String>,
    /// Decision computation time.
    pub computed_at: String,
    /// Optional expiry time.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    /// Downstream audit event refs.
    #[serde(default)]
    pub audit_event_refs: Vec<String>,
    /// Guardrail: no raw scope material crossed this boundary.
    pub raw_scope_material_present: bool,
    /// Redaction class.
    pub redaction_class: RedactionClass,
}

/// Invalidation of a cached effective-scope decision.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EffectiveScopeInvalidationEventRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque event id.
    pub event_id: String,
    /// Event time.
    pub event_time: String,
    /// Resolution invalidated by the event.
    pub originating_resolution_id: String,
    /// Connected-provider record ref.
    pub connected_provider_record_id: String,
    /// Actor class that was invalidated.
    pub actor_class: ProviderActorClass,
    /// Invalidation trigger.
    pub invalidation_trigger_class: ScopeReviewInvalidationTriggerClass,
    /// Optional upstream account invalidation event ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originating_account_invalidation_event_ref: Option<String>,
    /// Forced downgrade action.
    pub downgrade_action_class: ScopeReviewDowngradeActionClass,
    /// Reviewable rationale safe for support export.
    pub rationale_summary: String,
    /// Dependent refs invalidated alongside the resolution.
    #[serde(default)]
    pub invalidated_dependent_refs: Vec<String>,
    /// Policy epoch after invalidation.
    pub policy_epoch: String,
    /// Trust posture after invalidation.
    pub trust_posture: TrustPosture,
    /// Optional repair hook ref.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub repair_hook_ref: Option<String>,
    /// Redaction class.
    pub redaction_class: RedactionClass,
}

/// Projection of one resolution into a concrete consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScopeReviewConsumerProjectionRecord {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque projection id.
    pub projection_id: String,
    /// Consumer surface.
    pub surface_class: ScopeReviewSurfaceClass,
    /// Resolution this projection mirrors.
    pub originating_resolution_id: String,
    /// Projected decision class.
    pub projected_decision_class: ScopeReviewDecisionClass,
    /// Projected reason class.
    pub projected_grant_resolution_reason: ScopeReviewGrantResolutionReasonClass,
    /// Projected decision summary.
    pub projected_decision_summary: String,
    /// Alternative refs shown on the consumer.
    #[serde(default)]
    pub projected_alternative_refs: Vec<String>,
    /// Guardrail: no raw scope material crossed the projection.
    pub raw_scope_material_present: bool,
}

/// Aggregate coverage and counts for a page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderScopeReviewSummary {
    /// Parent page record kind.
    pub page_record_kind: String,
    /// Summary record kind.
    pub record_kind: String,
    /// Resolution count.
    pub resolution_count: usize,
    /// Alternative count.
    pub alternative_count: usize,
    /// Invalidation event count.
    pub invalidation_event_count: usize,
    /// Consumer projection count.
    pub consumer_projection_count: usize,
    /// Provider families covered.
    pub provider_families_present: Vec<String>,
    /// Provider classes covered.
    pub provider_classes_present: Vec<String>,
    /// Actor classes covered.
    pub actor_classes_present: Vec<String>,
    /// Decision classes covered.
    pub decision_classes_present: Vec<String>,
    /// Alternative classes covered.
    pub alternative_classes_present: Vec<String>,
    /// Invalidation triggers covered.
    pub invalidation_triggers_present: Vec<String>,
    /// Consumer surface classes covered.
    pub consumer_surfaces_present: Vec<String>,
    /// Resolution counts by decision class.
    pub resolutions_by_decision: BTreeMap<String, usize>,
    /// Defect count.
    pub defect_count: usize,
    /// Defect counts by defect kind.
    pub defect_counts_by_kind: BTreeMap<String, usize>,
}

impl ProviderScopeReviewSummary {
    /// Builds the summary from page records.
    pub fn from_records(
        resolutions: &[ProviderScopeResolutionRecord],
        alternatives: &[LeastPrivilegeAlternativeRecord],
        invalidations: &[EffectiveScopeInvalidationEventRecord],
        consumer_projections: &[ScopeReviewConsumerProjectionRecord],
        defects: &[ProviderScopeReviewDefect],
    ) -> Self {
        let mut provider_families_present = BTreeSet::new();
        let mut provider_classes_present = BTreeSet::new();
        let mut actor_classes_present = BTreeSet::new();
        let mut decision_classes_present = BTreeSet::new();
        let mut alternative_classes_present = BTreeSet::new();
        let mut invalidation_triggers_present = BTreeSet::new();
        let mut consumer_surfaces_present = BTreeSet::new();
        let mut resolutions_by_decision = BTreeMap::new();
        let mut defect_counts_by_kind = BTreeMap::new();

        for resolution in resolutions {
            provider_families_present
                .insert(provider_family_token(resolution.provider_family).to_owned());
            provider_classes_present.insert(
                resolution
                    .target_object_identity
                    .provider_class
                    .as_str()
                    .to_owned(),
            );
            actor_classes_present
                .insert(provider_actor_class_token(resolution.actor_class).to_owned());
            decision_classes_present.insert(resolution.decision_class.as_str().to_owned());
            *resolutions_by_decision
                .entry(resolution.decision_class.as_str().to_owned())
                .or_insert(0) += 1;
        }
        for alternative in alternatives {
            alternative_classes_present.insert(alternative.alternative_class.as_str().to_owned());
        }
        for invalidation in invalidations {
            invalidation_triggers_present
                .insert(invalidation.invalidation_trigger_class.as_str().to_owned());
        }
        for projection in consumer_projections {
            consumer_surfaces_present.insert(projection.surface_class.as_str().to_owned());
        }
        for defect in defects {
            *defect_counts_by_kind
                .entry(defect.defect_kind.as_str().to_owned())
                .or_insert(0) += 1;
        }

        Self {
            page_record_kind: PROVIDER_SCOPE_REVIEW_PAGE_RECORD_KIND.to_owned(),
            record_kind: PROVIDER_SCOPE_REVIEW_SUMMARY_RECORD_KIND.to_owned(),
            resolution_count: resolutions.len(),
            alternative_count: alternatives.len(),
            invalidation_event_count: invalidations.len(),
            consumer_projection_count: consumer_projections.len(),
            provider_families_present: provider_families_present.into_iter().collect(),
            provider_classes_present: provider_classes_present.into_iter().collect(),
            actor_classes_present: actor_classes_present.into_iter().collect(),
            decision_classes_present: decision_classes_present.into_iter().collect(),
            alternative_classes_present: alternative_classes_present.into_iter().collect(),
            invalidation_triggers_present: invalidation_triggers_present.into_iter().collect(),
            consumer_surfaces_present: consumer_surfaces_present.into_iter().collect(),
            resolutions_by_decision,
            defect_count: defects.len(),
            defect_counts_by_kind,
        }
    }
}

/// Defect kind surfaced by scope-review validation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderScopeReviewDefectKind {
    /// Duplicate ids were detected.
    DuplicateId,
    /// A contract ref was empty.
    ContractRefMissing,
    /// Raw scope material crossed the boundary.
    RawScopeMaterialPresent,
    /// Effective scope widened beyond declared scope.
    EffectiveScopeNotSubsetOfDeclared,
    /// Allowed mutation had no effective scopes.
    AllowedResolutionWithoutEffectiveScope,
    /// A closed health state admitted a direct write.
    AllowedDirectWriteOnClosedHealth,
    /// Decision and reason family disagreed.
    DecisionReasonMismatch,
    /// Browser-only resolution omitted a handoff ref.
    BrowserOnlyWithoutHandoffRef,
    /// Local-draft-only resolution omitted a queue ref.
    LocalDraftOnlyWithoutQueueRef,
    /// Allowed provider write omitted an approval ticket.
    AllowedWriteWithoutApprovalTicket,
    /// Policy lock array was incoherent.
    PolicyLockIncoherent,
    /// Least-privilege alternative coverage was missing or unknown.
    AlternativeCoverageBroken,
    /// Invalidation coverage was missing or unsafe.
    InvalidationBroken,
    /// Consumer projection drifted from the source resolution.
    ConsumerProjectionDrift,
    /// Consumer coverage was missing.
    ConsumerCoverageMissing,
}

impl ProviderScopeReviewDefectKind {
    /// Returns the stable export token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DuplicateId => "duplicate_id",
            Self::ContractRefMissing => "contract_ref_missing",
            Self::RawScopeMaterialPresent => "raw_scope_material_present",
            Self::EffectiveScopeNotSubsetOfDeclared => "effective_scope_not_subset_of_declared",
            Self::AllowedResolutionWithoutEffectiveScope => {
                "allowed_resolution_without_effective_scope"
            }
            Self::AllowedDirectWriteOnClosedHealth => "allowed_direct_write_on_closed_health",
            Self::DecisionReasonMismatch => "decision_reason_mismatch",
            Self::BrowserOnlyWithoutHandoffRef => "browser_only_without_handoff_ref",
            Self::LocalDraftOnlyWithoutQueueRef => "local_draft_only_without_queue_ref",
            Self::AllowedWriteWithoutApprovalTicket => "allowed_write_without_approval_ticket",
            Self::PolicyLockIncoherent => "policy_lock_incoherent",
            Self::AlternativeCoverageBroken => "alternative_coverage_broken",
            Self::InvalidationBroken => "invalidation_broken",
            Self::ConsumerProjectionDrift => "consumer_projection_drift",
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
        }
    }
}

/// Typed defect emitted by scope-review validation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderScopeReviewDefect {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Defect kind.
    pub defect_kind: ProviderScopeReviewDefectKind,
    /// Stable defect token.
    pub defect_kind_token: String,
    /// Offending subject id.
    pub subject_id: String,
    /// Failed field.
    pub field: String,
    /// Redaction-safe note.
    pub note: String,
}

impl ProviderScopeReviewDefect {
    fn new(
        defect_kind: ProviderScopeReviewDefectKind,
        subject_id: impl Into<String>,
        field: impl Into<String>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            record_kind: PROVIDER_SCOPE_REVIEW_DEFECT_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            defect_kind,
            defect_kind_token: defect_kind.as_str().to_owned(),
            subject_id: subject_id.into(),
            field: field.into(),
            note: note.into(),
        }
    }
}

/// Page-level scope-review packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderScopeReviewPage {
    /// Optional fixture metadata.
    #[serde(
        default,
        rename = "__fixture__",
        skip_serializing_if = "Option::is_none"
    )]
    pub fixture_metadata: Option<ProviderScopeReviewFixtureMetadata>,
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Opaque page id.
    pub page_id: String,
    /// Upstream contract refs.
    pub contract_refs: ProviderScopeReviewContractRefs,
    /// Canonical decision objects.
    pub resolutions: Vec<ProviderScopeResolutionRecord>,
    /// Least-privilege fallback rows.
    pub alternatives: Vec<LeastPrivilegeAlternativeRecord>,
    /// Invalidation events.
    pub invalidation_events: Vec<EffectiveScopeInvalidationEventRecord>,
    /// Consumer projections that must mirror the same resolution wording.
    pub consumer_projections: Vec<ScopeReviewConsumerProjectionRecord>,
    /// Typed validation defects.
    pub defects: Vec<ProviderScopeReviewDefect>,
    /// Aggregate summary.
    pub summary: ProviderScopeReviewSummary,
}

impl ProviderScopeReviewPage {
    /// Builds a redaction-safe support export projection.
    pub fn support_export_projection(&self) -> ProviderScopeReviewSupportExport {
        ProviderScopeReviewSupportExport::from_page(
            format!("{}:support_export", self.page_id),
            self.resolutions
                .first()
                .map(|resolution| resolution.computed_at.clone())
                .unwrap_or_else(|| "1970-01-01T00:00:00Z".to_owned()),
            self,
        )
    }

    /// Validates the page and returns a typed report.
    pub fn validate(&self) -> ProviderScopeReviewValidationReport {
        let defects = audit_provider_scope_review_page(
            &self.contract_refs,
            &self.resolutions,
            &self.alternatives,
            &self.invalidation_events,
            &self.consumer_projections,
        );
        ProviderScopeReviewValidationReport {
            record_kind: PROVIDER_SCOPE_REVIEW_VALIDATION_REPORT_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            page_id: self.page_id.clone(),
            passed: defects.is_empty(),
            defects,
        }
    }
}

/// Validation report for one scope-review page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderScopeReviewValidationReport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Validated page id.
    pub page_id: String,
    /// Whether validation passed.
    pub passed: bool,
    /// Typed defects emitted by validation.
    pub defects: Vec<ProviderScopeReviewDefect>,
}

/// Redaction-safe summary of one resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderScopeReviewSupportSummary {
    /// Resolution id.
    pub resolution_id: String,
    /// Provider family.
    pub provider_family: ProviderFamily,
    /// Provider class.
    pub provider_class: ScopeReviewProviderClass,
    /// Actor class.
    pub actor_class: ProviderActorClass,
    /// Actor health class.
    pub authority_health_class: ScopeReviewAuthorityHealthClass,
    /// Decision class.
    pub decision_class: ScopeReviewDecisionClass,
    /// Resolution reason.
    pub grant_resolution_reason: ScopeReviewGrantResolutionReasonClass,
    /// Effective scope refs.
    pub effective_scope_refs: Vec<String>,
    /// Policy lock classes.
    pub policy_lock_classes: Vec<ScopeReviewPolicyLockClass>,
    /// Alternative refs.
    pub least_privilege_alternative_refs: Vec<String>,
    /// Reviewable summary.
    pub summary: String,
}

impl From<&ProviderScopeResolutionRecord> for ProviderScopeReviewSupportSummary {
    fn from(record: &ProviderScopeResolutionRecord) -> Self {
        Self {
            resolution_id: record.resolution_id.clone(),
            provider_family: record.provider_family,
            provider_class: record.target_object_identity.provider_class,
            actor_class: record.actor_class,
            authority_health_class: record.authority_health_class,
            decision_class: record.decision_class,
            grant_resolution_reason: record.grant_resolution_reason,
            effective_scope_refs: record.effective_scope_refs.clone(),
            policy_lock_classes: record.policy_lock_classes.clone(),
            least_privilege_alternative_refs: record.least_privilege_alternative_refs.clone(),
            summary: record.decision_summary.clone(),
        }
    }
}

/// Redaction-safe summary of one invalidation event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderScopeInvalidationSupportSummary {
    /// Event id.
    pub event_id: String,
    /// Invalidated resolution id.
    pub originating_resolution_id: String,
    /// Trigger class.
    pub invalidation_trigger_class: ScopeReviewInvalidationTriggerClass,
    /// Downgrade action class.
    pub downgrade_action_class: ScopeReviewDowngradeActionClass,
    /// Reviewable rationale.
    pub rationale_summary: String,
}

impl From<&EffectiveScopeInvalidationEventRecord> for ProviderScopeInvalidationSupportSummary {
    fn from(record: &EffectiveScopeInvalidationEventRecord) -> Self {
        Self {
            event_id: record.event_id.clone(),
            originating_resolution_id: record.originating_resolution_id.clone(),
            invalidation_trigger_class: record.invalidation_trigger_class,
            downgrade_action_class: record.downgrade_action_class,
            rationale_summary: record.rationale_summary.clone(),
        }
    }
}

/// Redaction-safe summary of one consumer projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderScopeConsumerSupportSummary {
    /// Projection id.
    pub projection_id: String,
    /// Surface class.
    pub surface_class: ScopeReviewSurfaceClass,
    /// Resolution id quoted by the consumer.
    pub originating_resolution_id: String,
    /// Projected decision class.
    pub projected_decision_class: ScopeReviewDecisionClass,
    /// Reviewable summary quoted by the consumer.
    pub projected_decision_summary: String,
}

impl From<&ScopeReviewConsumerProjectionRecord> for ProviderScopeConsumerSupportSummary {
    fn from(record: &ScopeReviewConsumerProjectionRecord) -> Self {
        Self {
            projection_id: record.projection_id.clone(),
            surface_class: record.surface_class,
            originating_resolution_id: record.originating_resolution_id.clone(),
            projected_decision_class: record.projected_decision_class,
            projected_decision_summary: record.projected_decision_summary.clone(),
        }
    }
}

/// Redaction-safe support export for one page.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProviderScopeReviewSupportExport {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Export id.
    pub export_id: String,
    /// Page id.
    pub page_id: String,
    /// Generated-at timestamp.
    pub generated_at: String,
    /// Resolution summaries.
    pub resolution_summaries: Vec<ProviderScopeReviewSupportSummary>,
    /// Invalidation summaries.
    pub invalidation_summaries: Vec<ProviderScopeInvalidationSupportSummary>,
    /// Consumer summaries.
    pub consumer_surface_summaries: Vec<ProviderScopeConsumerSupportSummary>,
    /// Redaction class for the export.
    pub redaction_class: RedactionClass,
    /// Guardrail proving raw scope material stayed excluded.
    pub raw_scope_material_excluded: bool,
    /// Reviewable redaction summary.
    pub redaction_summary: String,
}

impl ProviderScopeReviewSupportExport {
    /// Builds a redaction-safe export from the page.
    pub fn from_page(
        export_id: impl Into<String>,
        generated_at: impl Into<String>,
        page: &ProviderScopeReviewPage,
    ) -> Self {
        Self {
            record_kind: PROVIDER_SCOPE_REVIEW_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            export_id: export_id.into(),
            page_id: page.page_id.clone(),
            generated_at: generated_at.into(),
            resolution_summaries: page
                .resolutions
                .iter()
                .map(ProviderScopeReviewSupportSummary::from)
                .collect(),
            invalidation_summaries: page
                .invalidation_events
                .iter()
                .map(ProviderScopeInvalidationSupportSummary::from)
                .collect(),
            consumer_surface_summaries: page
                .consumer_projections
                .iter()
                .map(ProviderScopeConsumerSupportSummary::from)
                .collect(),
            redaction_class: RedactionClass::MetadataSafeDefault,
            raw_scope_material_excluded: true,
            redaction_summary:
                "Metadata-only provider scope-review export: provider class, acting-identity \
                 class, authority health, effective-scope result, least-privilege alternatives, \
                 and invalidation lineage are preserved. Raw tokens, raw provider scope text, \
                 hidden delegation material, and raw provider payloads remain excluded."
                    .to_owned(),
        }
    }
}

/// Validates the scope-review page.
pub fn validate_provider_scope_review_page(
    page: &ProviderScopeReviewPage,
) -> Result<(), Vec<ProviderScopeReviewDefect>> {
    let defects = audit_provider_scope_review_page(
        &page.contract_refs,
        &page.resolutions,
        &page.alternatives,
        &page.invalidation_events,
        &page.consumer_projections,
    );
    if defects.is_empty() {
        Ok(())
    } else {
        Err(defects)
    }
}

/// Recomputes defects for a scope-review page.
pub fn audit_provider_scope_review_page(
    contract_refs: &ProviderScopeReviewContractRefs,
    resolutions: &[ProviderScopeResolutionRecord],
    alternatives: &[LeastPrivilegeAlternativeRecord],
    invalidation_events: &[EffectiveScopeInvalidationEventRecord],
    consumer_projections: &[ScopeReviewConsumerProjectionRecord],
) -> Vec<ProviderScopeReviewDefect> {
    let mut defects = Vec::new();

    for contract_ref in contract_refs.all_refs() {
        if contract_ref.trim().is_empty() {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::ContractRefMissing,
                "page",
                "contract_refs",
                "scope-review contract refs must all be non-empty",
            ));
            break;
        }
    }

    let mut seen_ids = BTreeSet::new();
    for id in resolutions
        .iter()
        .map(|row| row.resolution_id.as_str())
        .chain(alternatives.iter().map(|row| row.alternative_id.as_str()))
        .chain(invalidation_events.iter().map(|row| row.event_id.as_str()))
        .chain(
            consumer_projections
                .iter()
                .map(|row| row.projection_id.as_str()),
        )
    {
        if !seen_ids.insert(id) {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::DuplicateId,
                id,
                "id",
                "scope-review ids must be globally unique within the page",
            ));
        }
    }

    let resolution_map: BTreeMap<&str, &ProviderScopeResolutionRecord> = resolutions
        .iter()
        .map(|resolution| (resolution.resolution_id.as_str(), resolution))
        .collect();
    let alternative_map: BTreeMap<&str, &LeastPrivilegeAlternativeRecord> = alternatives
        .iter()
        .map(|alternative| (alternative.alternative_id.as_str(), alternative))
        .collect();
    let surface_coverage: BTreeSet<_> = consumer_projections
        .iter()
        .map(|projection| projection.surface_class)
        .collect();

    for resolution in resolutions {
        if resolution.raw_scope_material_present {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::RawScopeMaterialPresent,
                &resolution.resolution_id,
                "raw_scope_material_present",
                "scope-review resolutions must never carry raw scope or token material",
            ));
        }

        let declared: BTreeSet<_> = resolution
            .provider_declared_scope_refs
            .iter()
            .map(String::as_str)
            .collect();
        if resolution
            .effective_scope_refs
            .iter()
            .any(|scope| !declared.contains(scope.as_str()))
        {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::EffectiveScopeNotSubsetOfDeclared,
                &resolution.resolution_id,
                "effective_scope_refs",
                "effective_scope_refs must stay within provider_declared_scope_refs",
            ));
        }

        if resolution.decision_class.admits_direct_provider_write()
            && resolution.effective_scope_refs.is_empty()
            && resolution.requested_action_class.proposes_mutation()
        {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::AllowedResolutionWithoutEffectiveScope,
                &resolution.resolution_id,
                "effective_scope_refs",
                "allowed provider writes must name effective_scope_refs",
            ));
        }

        if resolution.decision_class.admits_direct_provider_write()
            && resolution.authority_health_class.holds_mutation_closed()
            && resolution.requested_action_class.proposes_mutation()
        {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::AllowedDirectWriteOnClosedHealth,
                &resolution.resolution_id,
                "authority_health_class",
                "closed authority health must not admit a direct provider write",
            ));
        }

        let reason_allowed = resolution.grant_resolution_reason.is_allowed_family();
        match resolution.decision_class {
            ScopeReviewDecisionClass::Allowed if !reason_allowed => {
                defects.push(ProviderScopeReviewDefect::new(
                    ProviderScopeReviewDefectKind::DecisionReasonMismatch,
                    &resolution.resolution_id,
                    "grant_resolution_reason",
                    "allowed decisions must use an allowed-family resolution reason",
                ))
            }
            ScopeReviewDecisionClass::Denied if reason_allowed => {
                defects.push(ProviderScopeReviewDefect::new(
                    ProviderScopeReviewDefectKind::DecisionReasonMismatch,
                    &resolution.resolution_id,
                    "grant_resolution_reason",
                    "denied decisions must use a denied-family resolution reason",
                ))
            }
            ScopeReviewDecisionClass::BrowserOnly
                if resolution.grant_resolution_reason
                    != ScopeReviewGrantResolutionReasonClass::AllowedWithBrowserHandoff =>
            {
                defects.push(ProviderScopeReviewDefect::new(
                    ProviderScopeReviewDefectKind::DecisionReasonMismatch,
                    &resolution.resolution_id,
                    "grant_resolution_reason",
                    "browser-only decisions must use allowed_with_browser_handoff",
                ));
            }
            ScopeReviewDecisionClass::LocalDraftOnly
                if resolution.grant_resolution_reason
                    != ScopeReviewGrantResolutionReasonClass::AllowedWithDeferredPublish =>
            {
                defects.push(ProviderScopeReviewDefect::new(
                    ProviderScopeReviewDefectKind::DecisionReasonMismatch,
                    &resolution.resolution_id,
                    "grant_resolution_reason",
                    "local-draft-only decisions must use allowed_with_deferred_publish",
                ));
            }
            _ => {}
        }

        if matches!(
            resolution.decision_class,
            ScopeReviewDecisionClass::BrowserOnly
        ) && resolution.browser_handoff_packet_ref.is_none()
        {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::BrowserOnlyWithoutHandoffRef,
                &resolution.resolution_id,
                "browser_handoff_packet_ref",
                "browser-only resolutions must cite a browser_handoff_packet_ref",
            ));
        }

        if matches!(
            resolution.decision_class,
            ScopeReviewDecisionClass::LocalDraftOnly
        ) && resolution.publish_later_queue_item_ref.is_none()
        {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::LocalDraftOnlyWithoutQueueRef,
                &resolution.resolution_id,
                "publish_later_queue_item_ref",
                "local-draft-only resolutions must cite a publish_later_queue_item_ref",
            ));
        }

        if matches!(resolution.decision_class, ScopeReviewDecisionClass::Allowed)
            && resolution.requested_action_class.proposes_mutation()
            && matches!(
                resolution.requested_mutation_mode,
                WorkItemMutationMode::PublishNow | WorkItemMutationMode::OpenInProvider
            )
            && resolution.approval_ticket_ref.is_none()
        {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::AllowedWriteWithoutApprovalTicket,
                &resolution.resolution_id,
                "approval_ticket_ref",
                "allowed publish-now or open-in-provider writes must cite an approval_ticket_ref",
            ));
        }

        let lock_set: BTreeSet<_> = resolution.policy_lock_classes.iter().copied().collect();
        if lock_set.is_empty()
            || (lock_set.contains(&ScopeReviewPolicyLockClass::NoLocalPolicyLock)
                && lock_set.len() > 1)
        {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::PolicyLockIncoherent,
                &resolution.resolution_id,
                "policy_lock_classes",
                "policy_lock_classes must be non-empty and no_local_policy_lock cannot mix with \
                 other locks",
            ));
        }

        if resolution.grant_resolution_reason
            == ScopeReviewGrantResolutionReasonClass::DeniedWorkspaceTrust
            && !lock_set.contains(&ScopeReviewPolicyLockClass::WorkspaceTrustRestrictedLock)
        {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::PolicyLockIncoherent,
                &resolution.resolution_id,
                "policy_lock_classes",
                "denied_workspace_trust must cite workspace_trust_restricted_lock",
            ));
        }

        if resolution.actor_class == ProviderActorClass::UnknownActorClass
            && (resolution.decision_class != ScopeReviewDecisionClass::Denied
                || resolution.grant_resolution_reason
                    != ScopeReviewGrantResolutionReasonClass::DeniedUnknownActorClass)
        {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::DecisionReasonMismatch,
                &resolution.resolution_id,
                "actor_class",
                "unknown actor class must degrade to denied_unknown_actor_class",
            ));
        }

        let must_offer_alternative =
            !matches!(resolution.decision_class, ScopeReviewDecisionClass::Allowed);
        if must_offer_alternative && resolution.least_privilege_alternative_refs.is_empty() {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::AlternativeCoverageBroken,
                &resolution.resolution_id,
                "least_privilege_alternative_refs",
                "non-allowed decisions must offer at least one least-privilege alternative",
            ));
        }
        for alternative_ref in &resolution.least_privilege_alternative_refs {
            match alternative_map.get(alternative_ref.as_str()) {
                None => defects.push(ProviderScopeReviewDefect::new(
                    ProviderScopeReviewDefectKind::AlternativeCoverageBroken,
                    &resolution.resolution_id,
                    "least_privilege_alternative_refs",
                    format!("unknown least-privilege alternative ref: {alternative_ref}"),
                )),
                Some(alternative)
                    if alternative.originating_resolution_id != resolution.resolution_id =>
                {
                    defects.push(ProviderScopeReviewDefect::new(
                        ProviderScopeReviewDefectKind::AlternativeCoverageBroken,
                        &resolution.resolution_id,
                        "least_privilege_alternative_refs",
                        "alternative ref must point back to the originating resolution",
                    ));
                }
                Some(_) => {}
            }
        }
    }

    for alternative in alternatives {
        if !resolution_map.contains_key(alternative.originating_resolution_id.as_str()) {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::AlternativeCoverageBroken,
                &alternative.alternative_id,
                "originating_resolution_id",
                "least-privilege alternatives must reference a known resolution",
            ));
        }
        if matches!(
            alternative.alternative_class,
            ScopeReviewAlternativeClass::NoAlternativeAvailable
        ) && alternative.repair_hook_ref.is_none()
        {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::AlternativeCoverageBroken,
                &alternative.alternative_id,
                "repair_hook_ref",
                "no_alternative_available must still cite a repair hook",
            ));
        }
    }

    for invalidation in invalidation_events {
        if !resolution_map.contains_key(invalidation.originating_resolution_id.as_str()) {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::InvalidationBroken,
                &invalidation.event_id,
                "originating_resolution_id",
                "invalidation events must reference a known resolution",
            ));
        }
        if matches!(
            invalidation.invalidation_trigger_class,
            ScopeReviewInvalidationTriggerClass::HostMismatchDetected
                | ScopeReviewInvalidationTriggerClass::TenantSwitchDetected
                | ScopeReviewInvalidationTriggerClass::OrgMembershipLost
        ) && invalidation.downgrade_action_class
            == ScopeReviewDowngradeActionClass::NoDowngradeRequired
        {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::InvalidationBroken,
                &invalidation.event_id,
                "downgrade_action_class",
                "host mismatch, tenant switch, and org-membership loss must visibly degrade the \
                 cached decision",
            ));
        }
        if invalidation.downgrade_action_class
            != ScopeReviewDowngradeActionClass::NoDowngradeRequired
            && invalidation.repair_hook_ref.is_none()
        {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::InvalidationBroken,
                &invalidation.event_id,
                "repair_hook_ref",
                "non-benign invalidations must cite a repair hook",
            ));
        }
    }

    for projection in consumer_projections {
        if projection.raw_scope_material_present {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::RawScopeMaterialPresent,
                &projection.projection_id,
                "raw_scope_material_present",
                "consumer projections must not carry raw scope material",
            ));
        }
        match resolution_map.get(projection.originating_resolution_id.as_str()) {
            None => defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::ConsumerProjectionDrift,
                &projection.projection_id,
                "originating_resolution_id",
                "consumer projections must point at a known resolution",
            )),
            Some(source) => {
                if projection.projected_decision_class != source.decision_class
                    || projection.projected_grant_resolution_reason
                        != source.grant_resolution_reason
                    || projection.projected_decision_summary != source.decision_summary
                    || projection.projected_alternative_refs
                        != source.least_privilege_alternative_refs
                {
                    defects.push(ProviderScopeReviewDefect::new(
                        ProviderScopeReviewDefectKind::ConsumerProjectionDrift,
                        &projection.projection_id,
                        "projected_decision_summary",
                        "consumer projections must mirror the canonical resolution object \
                         exactly",
                    ));
                }
            }
        }
    }

    let required_surfaces = [
        ScopeReviewSurfaceClass::Desktop,
        ScopeReviewSurfaceClass::CliHeadless,
        ScopeReviewSurfaceClass::Companion,
        ScopeReviewSurfaceClass::SupportExport,
    ];
    for surface in required_surfaces {
        if !surface_coverage.contains(&surface) {
            defects.push(ProviderScopeReviewDefect::new(
                ProviderScopeReviewDefectKind::ConsumerCoverageMissing,
                "page",
                "consumer_projections",
                format!("missing {} consumer projection coverage", surface.as_str()),
            ));
        }
    }

    defects
}

/// Builds the canonical seeded scope-review page.
pub fn seeded_provider_scope_review_page() -> ProviderScopeReviewPage {
    let contract_refs = ProviderScopeReviewContractRefs {
        provider_scope_review_schema_ref: PROVIDER_SCOPE_REVIEW_SCHEMA_REF.to_owned(),
        effective_scope_resolution_schema_ref: PROVIDER_SCOPE_REVIEW_EFFECTIVE_SCOPE_SCHEMA_REF
            .to_owned(),
        account_scope_beta_shared_contract_ref: ACCOUNT_SCOPE_BETA_SHARED_CONTRACT_REF.to_owned(),
        route_resolution_beta_shared_contract_ref: ROUTE_RESOLUTION_BETA_SHARED_CONTRACT_REF
            .to_owned(),
        work_item_transition_beta_shared_contract_ref:
            WORK_ITEM_TRANSITION_BETA_SHARED_CONTRACT_REF.to_owned(),
        work_item_sync_beta_shared_contract_ref: WORK_ITEM_SYNC_BETA_SHARED_CONTRACT_REF.to_owned(),
    };
    let resolutions = seed_resolutions();
    let alternatives = seed_alternatives();
    let invalidation_events = seed_invalidations();
    let consumer_projections = seed_consumer_projections(&resolutions);
    let defects = audit_provider_scope_review_page(
        &contract_refs,
        &resolutions,
        &alternatives,
        &invalidation_events,
        &consumer_projections,
    );
    let summary = ProviderScopeReviewSummary::from_records(
        &resolutions,
        &alternatives,
        &invalidation_events,
        &consumer_projections,
        &defects,
    );

    ProviderScopeReviewPage {
        fixture_metadata: Some(ProviderScopeReviewFixtureMetadata {
            name: "provider_scope_review".to_owned(),
            scenario: "Canonical M5 provider scope review with human, install, delegated, policy, \
                 host-mismatch, and invalidation coverage."
                .to_owned(),
        }),
        record_kind: PROVIDER_SCOPE_REVIEW_PAGE_RECORD_KIND.to_owned(),
        schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
        page_id: "providers:scope_review:m5:seed".to_owned(),
        contract_refs,
        resolutions,
        alternatives,
        invalidation_events,
        consumer_projections,
        defects,
        summary,
    }
}

fn seed_resolutions() -> Vec<ProviderScopeResolutionRecord> {
    vec![
        ProviderScopeResolutionRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_RESOLUTION_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            resolution_id: "scope_review:resolution:code_host:human:comment".to_owned(),
            connected_provider_record_id: "connected_provider:code_host:payments".to_owned(),
            provider_family: ProviderFamily::CodeHost,
            actor_class: ProviderActorClass::HumanAccount,
            actor_subject_ref: "account_scope.connected_account.primary".to_owned(),
            actor_display_label: "Signed-in reviewer account".to_owned(),
            authority_health_class: ScopeReviewAuthorityHealthClass::Healthy,
            requested_action_class: ScopeReviewRequestedActionClass::HumanAuthoredComment,
            requested_mutation_mode: WorkItemMutationMode::PublishNow,
            target_object_identity: ScopeReviewTargetObjectIdentity {
                provider_class: ScopeReviewProviderClass::CodeHost,
                target_object_class: ScopeReviewTargetObjectClass::PullRequest,
                provider_side_id: "pr:payments/backend:1234".to_owned(),
                provider_host_ref: "provider_host:github-enterprise".to_owned(),
                tenant_or_org_scope_ref: "tenant:payments".to_owned(),
                target_label: "PR #1234 on payments/backend".to_owned(),
            },
            provider_declared_scope_refs: vec![
                "scope:repo:payments/backend:comment".to_owned(),
                "scope:repo:payments/backend:read".to_owned(),
            ],
            effective_scope_refs: vec!["scope:repo:payments/backend:comment".to_owned()],
            policy_lock_classes: vec![ScopeReviewPolicyLockClass::NoLocalPolicyLock],
            trust_posture: TrustPosture::Trusted,
            decision_class: ScopeReviewDecisionClass::Allowed,
            grant_resolution_reason: ScopeReviewGrantResolutionReasonClass::Allowed,
            decision_summary:
                "Allowed: Aureline acts as the signed-in reviewer account and can publish the \
                 PR comment with repo comment scope."
                    .to_owned(),
            freshness: Some(ScopeReviewFreshnessBlock {
                observed_at: "2026-06-12T20:05:00Z".to_owned(),
                freshness_floor_ref: Some("freshness_floor:provider:code_host:comment".to_owned()),
                staleness_class: ScopeReviewStalenessClass::Fresh,
                staleness_rationale: None,
            }),
            policy_context: ScopeReviewPolicyContext {
                policy_epoch: "policy-epoch:2026-06-12T20:00:00Z".to_owned(),
                trust_posture: TrustPosture::Trusted,
                execution_context_id: "desktop:workspace:payments".to_owned(),
                policy_block_ref: None,
            },
            approval_ticket_ref: Some("approval_ticket:pr_comment:1234".to_owned()),
            browser_handoff_packet_ref: None,
            publish_later_queue_item_ref: None,
            least_privilege_alternative_refs: vec![],
            computed_at: "2026-06-12T20:05:30Z".to_owned(),
            expires_at: Some("2026-06-12T20:35:30Z".to_owned()),
            audit_event_refs: vec!["audit:provider_scope_resolved:comment:1234".to_owned()],
            raw_scope_material_present: false,
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        ProviderScopeResolutionRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_RESOLUTION_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            resolution_id: "scope_review:resolution:ci:install:rerun".to_owned(),
            connected_provider_record_id: "connected_provider:ci:payments".to_owned(),
            provider_family: ProviderFamily::CiChecks,
            actor_class: ProviderActorClass::InstallationOrAppGrant,
            actor_subject_ref: "installation_grant:ci-bot".to_owned(),
            actor_display_label: "CI bot installation grant".to_owned(),
            authority_health_class: ScopeReviewAuthorityHealthClass::Healthy,
            requested_action_class: ScopeReviewRequestedActionClass::CiRunOrCheckMutation,
            requested_mutation_mode: WorkItemMutationMode::PublishNow,
            target_object_identity: ScopeReviewTargetObjectIdentity {
                provider_class: ScopeReviewProviderClass::CiChecks,
                target_object_class: ScopeReviewTargetObjectClass::CheckRun,
                provider_side_id: "check_run:9876".to_owned(),
                provider_host_ref: "provider_host:github-enterprise".to_owned(),
                tenant_or_org_scope_ref: "tenant:payments".to_owned(),
                target_label: "Check run 9876 on payments/backend".to_owned(),
            },
            provider_declared_scope_refs: vec![
                "scope:repo:payments/backend:check_run_write".to_owned(),
                "scope:repo:payments/backend:read".to_owned(),
            ],
            effective_scope_refs: vec!["scope:repo:payments/backend:check_run_write".to_owned()],
            policy_lock_classes: vec![ScopeReviewPolicyLockClass::NoLocalPolicyLock],
            trust_posture: TrustPosture::Trusted,
            decision_class: ScopeReviewDecisionClass::Allowed,
            grant_resolution_reason: ScopeReviewGrantResolutionReasonClass::Allowed,
            decision_summary:
                "Allowed: Aureline acts as the CI bot installation grant and can rerun the check \
                 with install-scoped write authority."
                    .to_owned(),
            freshness: Some(ScopeReviewFreshnessBlock {
                observed_at: "2026-06-12T20:06:00Z".to_owned(),
                freshness_floor_ref: Some("freshness_floor:provider:ci:check_run".to_owned()),
                staleness_class: ScopeReviewStalenessClass::Fresh,
                staleness_rationale: None,
            }),
            policy_context: ScopeReviewPolicyContext {
                policy_epoch: "policy-epoch:2026-06-12T20:00:00Z".to_owned(),
                trust_posture: TrustPosture::Trusted,
                execution_context_id: "desktop:workspace:payments".to_owned(),
                policy_block_ref: None,
            },
            approval_ticket_ref: Some("approval_ticket:check_run:9876".to_owned()),
            browser_handoff_packet_ref: None,
            publish_later_queue_item_ref: None,
            least_privilege_alternative_refs: vec![],
            computed_at: "2026-06-12T20:06:15Z".to_owned(),
            expires_at: Some("2026-06-12T21:06:15Z".to_owned()),
            audit_event_refs: vec!["audit:provider_scope_resolved:check_run:9876".to_owned()],
            raw_scope_material_present: false,
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        ProviderScopeResolutionRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_RESOLUTION_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            resolution_id: "scope_review:resolution:code_host:browser_only:merge".to_owned(),
            connected_provider_record_id: "connected_provider:code_host:payments".to_owned(),
            provider_family: ProviderFamily::CodeHost,
            actor_class: ProviderActorClass::HumanAccount,
            actor_subject_ref: "account_scope.connected_account.mirror".to_owned(),
            actor_display_label: "Mirror-bound reviewer account".to_owned(),
            authority_health_class: ScopeReviewAuthorityHealthClass::Expiring,
            requested_action_class: ScopeReviewRequestedActionClass::ReviewDecisionPublish,
            requested_mutation_mode: WorkItemMutationMode::OpenInProvider,
            target_object_identity: ScopeReviewTargetObjectIdentity {
                provider_class: ScopeReviewProviderClass::CodeHost,
                target_object_class: ScopeReviewTargetObjectClass::PullRequest,
                provider_side_id: "pr:payments/backend:1234".to_owned(),
                provider_host_ref: "provider_host:github-enterprise".to_owned(),
                tenant_or_org_scope_ref: "tenant:payments".to_owned(),
                target_label: "PR #1234 on payments/backend".to_owned(),
            },
            provider_declared_scope_refs: vec![
                "scope:repo:payments/backend:merge".to_owned(),
                "scope:repo:payments/backend:read".to_owned(),
            ],
            effective_scope_refs: vec![],
            policy_lock_classes: vec![
                ScopeReviewPolicyLockClass::PolicyBundleRequiresBrowserHandoff,
            ],
            trust_posture: TrustPosture::Trusted,
            decision_class: ScopeReviewDecisionClass::BrowserOnly,
            grant_resolution_reason:
                ScopeReviewGrantResolutionReasonClass::AllowedWithBrowserHandoff,
            decision_summary:
                "Browser-only: Aureline can inspect the merge scope, but policy requires the \
                 final merge to complete through the provider browser handoff."
                    .to_owned(),
            freshness: Some(ScopeReviewFreshnessBlock {
                observed_at: "2026-06-12T20:07:00Z".to_owned(),
                freshness_floor_ref: Some("freshness_floor:provider:code_host:merge".to_owned()),
                staleness_class: ScopeReviewStalenessClass::BoundedStale,
                staleness_rationale: Some(
                    "Mirror freshness remains inside the bounded review window, but publish-now \
                     is held behind a browser-only policy lock."
                        .to_owned(),
                ),
            }),
            policy_context: ScopeReviewPolicyContext {
                policy_epoch: "policy-epoch:2026-06-12T20:00:00Z".to_owned(),
                trust_posture: TrustPosture::Trusted,
                execution_context_id: "desktop:workspace:payments".to_owned(),
                policy_block_ref: Some("policy_lock:browser_only:merge".to_owned()),
            },
            approval_ticket_ref: None,
            browser_handoff_packet_ref: Some("browser_handoff:pr_merge:1234".to_owned()),
            publish_later_queue_item_ref: None,
            least_privilege_alternative_refs: vec![
                "scope_review:alternative:merge:browser".to_owned(),
                "scope_review:alternative:merge:copy_export".to_owned(),
            ],
            computed_at: "2026-06-12T20:07:20Z".to_owned(),
            expires_at: Some("2026-06-12T20:20:00Z".to_owned()),
            audit_event_refs: vec!["audit:provider_scope_browser_only:merge:1234".to_owned()],
            raw_scope_material_present: false,
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        ProviderScopeResolutionRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_RESOLUTION_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            resolution_id: "scope_review:resolution:issue:delegated:local_draft".to_owned(),
            connected_provider_record_id: "connected_provider:issues:planning".to_owned(),
            provider_family: ProviderFamily::IssueTracker,
            actor_class: ProviderActorClass::DelegatedUserToken,
            actor_subject_ref: "delegated_credential:release_scribe".to_owned(),
            actor_display_label: "Release scribe delegated credential".to_owned(),
            authority_health_class: ScopeReviewAuthorityHealthClass::Unreachable,
            requested_action_class: ScopeReviewRequestedActionClass::IssueOrWorkItemMutation,
            requested_mutation_mode: WorkItemMutationMode::DeferredPublish,
            target_object_identity: ScopeReviewTargetObjectIdentity {
                provider_class: ScopeReviewProviderClass::IssueTracker,
                target_object_class: ScopeReviewTargetObjectClass::IssueOrWorkItem,
                provider_side_id: "issue:planning:84".to_owned(),
                provider_host_ref: "provider_host:jira-enterprise".to_owned(),
                tenant_or_org_scope_ref: "tenant:planning".to_owned(),
                target_label: "Planning item ENG-84".to_owned(),
            },
            provider_declared_scope_refs: vec![
                "scope:board:planning:item_write".to_owned(),
                "scope:board:planning:read".to_owned(),
            ],
            effective_scope_refs: vec![],
            policy_lock_classes: vec![
                ScopeReviewPolicyLockClass::PolicyBundleRequiresDeferredPublish,
            ],
            trust_posture: TrustPosture::Trusted,
            decision_class: ScopeReviewDecisionClass::LocalDraftOnly,
            grant_resolution_reason:
                ScopeReviewGrantResolutionReasonClass::AllowedWithDeferredPublish,
            decision_summary:
                "Local-draft only: Aureline keeps the work-item edit as a deferred publish item \
                 because the delegated credential cannot currently reach the provider."
                    .to_owned(),
            freshness: Some(ScopeReviewFreshnessBlock {
                observed_at: "2026-06-12T20:08:00Z".to_owned(),
                freshness_floor_ref: Some("freshness_floor:provider:issues:item".to_owned()),
                staleness_class: ScopeReviewStalenessClass::UnboundedStale,
                staleness_rationale: Some(
                    "Provider route is unreachable, so the cached item cannot justify a live \
                     publish-now mutation."
                        .to_owned(),
                ),
            }),
            policy_context: ScopeReviewPolicyContext {
                policy_epoch: "policy-epoch:2026-06-12T20:00:00Z".to_owned(),
                trust_posture: TrustPosture::Trusted,
                execution_context_id: "desktop:workspace:planning".to_owned(),
                policy_block_ref: Some("policy_lock:deferred_publish:item_mutation".to_owned()),
            },
            approval_ticket_ref: None,
            browser_handoff_packet_ref: None,
            publish_later_queue_item_ref: Some("publish_later:item:eng-84".to_owned()),
            least_privilege_alternative_refs: vec![
                "scope_review:alternative:item:local_draft".to_owned(),
                "scope_review:alternative:item:inspect_only".to_owned(),
            ],
            computed_at: "2026-06-12T20:08:30Z".to_owned(),
            expires_at: None,
            audit_event_refs: vec!["audit:provider_scope_deferred:item:eng-84".to_owned()],
            raw_scope_material_present: false,
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        ProviderScopeResolutionRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_RESOLUTION_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            resolution_id: "scope_review:resolution:release:host_mismatch:denied".to_owned(),
            connected_provider_record_id: "connected_provider:release:fleet".to_owned(),
            provider_family: ProviderFamily::IssueTracker,
            actor_class: ProviderActorClass::PolicyInjectedServiceIdentity,
            actor_subject_ref: "policy_service:release-publisher".to_owned(),
            actor_display_label: "Managed release publisher".to_owned(),
            authority_health_class: ScopeReviewAuthorityHealthClass::HostMismatch,
            requested_action_class: ScopeReviewRequestedActionClass::ReleasePublish,
            requested_mutation_mode: WorkItemMutationMode::PublishNow,
            target_object_identity: ScopeReviewTargetObjectIdentity {
                provider_class: ScopeReviewProviderClass::ReleasePublisher,
                target_object_class: ScopeReviewTargetObjectClass::ReleaseArtifact,
                provider_side_id: "release:fleet-0001:v1.2.3".to_owned(),
                provider_host_ref: "provider_host:release-primary".to_owned(),
                tenant_or_org_scope_ref: "tenant:fleet-0001".to_owned(),
                target_label: "Release v1.2.3 for fleet-0001".to_owned(),
            },
            provider_declared_scope_refs: vec![
                "scope:release:fleet-0001:publish".to_owned(),
                "scope:release:fleet-0001:read".to_owned(),
            ],
            effective_scope_refs: vec![],
            policy_lock_classes: vec![
                ScopeReviewPolicyLockClass::ReleasePublisherLock,
                ScopeReviewPolicyLockClass::ManagedProviderAdminLock,
            ],
            trust_posture: TrustPosture::Trusted,
            decision_class: ScopeReviewDecisionClass::Denied,
            grant_resolution_reason: ScopeReviewGrantResolutionReasonClass::DeniedHostMismatch,
            decision_summary:
                "Denied: the managed release publisher no longer matches the reviewed provider \
                 host and tenant, so Aureline falls back to inspect-only and export-safe \
                 evidence."
                    .to_owned(),
            freshness: Some(ScopeReviewFreshnessBlock {
                observed_at: "2026-06-12T20:09:00Z".to_owned(),
                freshness_floor_ref: None,
                staleness_class: ScopeReviewStalenessClass::UnboundedStale,
                staleness_rationale: Some(
                    "Host and tenant drift invalidated the cached effective scope before the \
                     publish decision could be reused."
                        .to_owned(),
                ),
            }),
            policy_context: ScopeReviewPolicyContext {
                policy_epoch: "policy-epoch:2026-06-12T20:00:00Z".to_owned(),
                trust_posture: TrustPosture::Trusted,
                execution_context_id: "headless:release:fleet".to_owned(),
                policy_block_ref: Some("policy_lock:release_host_mismatch".to_owned()),
            },
            approval_ticket_ref: None,
            browser_handoff_packet_ref: None,
            publish_later_queue_item_ref: None,
            least_privilege_alternative_refs: vec![
                "scope_review:alternative:release:inspect".to_owned(),
                "scope_review:alternative:release:admin_review".to_owned(),
                "scope_review:alternative:release:copy_export".to_owned(),
            ],
            computed_at: "2026-06-12T20:09:15Z".to_owned(),
            expires_at: None,
            audit_event_refs: vec!["audit:provider_scope_denied:release:fleet-0001".to_owned()],
            raw_scope_material_present: false,
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
    ]
}

fn seed_alternatives() -> Vec<LeastPrivilegeAlternativeRecord> {
    vec![
        LeastPrivilegeAlternativeRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_ALTERNATIVE_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            alternative_id: "scope_review:alternative:merge:browser".to_owned(),
            originating_resolution_id: "scope_review:resolution:code_host:browser_only:merge"
                .to_owned(),
            alternative_class: ScopeReviewAlternativeClass::RouteThroughBrowserHandoff,
            alternative_summary:
                "Complete the merge through the typed browser handoff while keeping the same \
                 reviewed target and return anchor."
                    .to_owned(),
            suggested_actor_class: Some(ProviderActorClass::HumanAccount),
            suggested_mutation_mode: Some(WorkItemMutationMode::OpenInProvider),
            narrowed_scope_refs: vec!["scope:repo:payments/backend:merge".to_owned()],
            repair_hook_ref: Some("repair_hook:browser_handoff:pr_merge:1234".to_owned()),
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        LeastPrivilegeAlternativeRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_ALTERNATIVE_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            alternative_id: "scope_review:alternative:merge:copy_export".to_owned(),
            originating_resolution_id: "scope_review:resolution:code_host:browser_only:merge"
                .to_owned(),
            alternative_class: ScopeReviewAlternativeClass::CopyOrExportEvidence,
            alternative_summary:
                "Copy the merge rationale or export the review packet without attempting the \
                 provider mutation in-product."
                    .to_owned(),
            suggested_actor_class: None,
            suggested_mutation_mode: Some(WorkItemMutationMode::InspectOnly),
            narrowed_scope_refs: vec![],
            repair_hook_ref: Some("repair_hook:export:review_packet:pr_merge:1234".to_owned()),
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        LeastPrivilegeAlternativeRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_ALTERNATIVE_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            alternative_id: "scope_review:alternative:item:local_draft".to_owned(),
            originating_resolution_id: "scope_review:resolution:issue:delegated:local_draft"
                .to_owned(),
            alternative_class: ScopeReviewAlternativeClass::SwitchToLocalDraft,
            alternative_summary:
                "Keep editing locally and preserve the change as a deferred publish item until \
                 the delegated credential can be reissued."
                    .to_owned(),
            suggested_actor_class: Some(ProviderActorClass::DelegatedUserToken),
            suggested_mutation_mode: Some(WorkItemMutationMode::DeferredPublish),
            narrowed_scope_refs: vec![],
            repair_hook_ref: Some("repair_hook:delegated_credential:reissue".to_owned()),
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        LeastPrivilegeAlternativeRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_ALTERNATIVE_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            alternative_id: "scope_review:alternative:item:inspect_only".to_owned(),
            originating_resolution_id: "scope_review:resolution:issue:delegated:local_draft"
                .to_owned(),
            alternative_class: ScopeReviewAlternativeClass::SwitchToInspectOnly,
            alternative_summary:
                "Inspect the provider-backed work item read-only until connectivity and delegated \
                 scope health return."
                    .to_owned(),
            suggested_actor_class: None,
            suggested_mutation_mode: Some(WorkItemMutationMode::InspectOnly),
            narrowed_scope_refs: vec!["scope:board:planning:read".to_owned()],
            repair_hook_ref: Some("repair_hook:provider_refresh:planning".to_owned()),
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        LeastPrivilegeAlternativeRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_ALTERNATIVE_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            alternative_id: "scope_review:alternative:release:inspect".to_owned(),
            originating_resolution_id: "scope_review:resolution:release:host_mismatch:denied"
                .to_owned(),
            alternative_class: ScopeReviewAlternativeClass::SwitchToInspectOnly,
            alternative_summary:
                "Review the release packet read-only while the managed publisher host and tenant \
                 are revalidated."
                    .to_owned(),
            suggested_actor_class: None,
            suggested_mutation_mode: Some(WorkItemMutationMode::InspectOnly),
            narrowed_scope_refs: vec!["scope:release:fleet-0001:read".to_owned()],
            repair_hook_ref: Some("repair_hook:release:refresh_binding".to_owned()),
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        LeastPrivilegeAlternativeRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_ALTERNATIVE_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            alternative_id: "scope_review:alternative:release:admin_review".to_owned(),
            originating_resolution_id: "scope_review:resolution:release:host_mismatch:denied"
                .to_owned(),
            alternative_class: ScopeReviewAlternativeClass::RequestAdminReview,
            alternative_summary:
                "Route the publish decision through managed admin review instead of guessing \
                 whether the installation grant still matches the release host."
                    .to_owned(),
            suggested_actor_class: Some(ProviderActorClass::PolicyInjectedServiceIdentity),
            suggested_mutation_mode: None,
            narrowed_scope_refs: vec![],
            repair_hook_ref: Some("repair_hook:managed_admin:release_host_review".to_owned()),
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        LeastPrivilegeAlternativeRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_ALTERNATIVE_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            alternative_id: "scope_review:alternative:release:copy_export".to_owned(),
            originating_resolution_id: "scope_review:resolution:release:host_mismatch:denied"
                .to_owned(),
            alternative_class: ScopeReviewAlternativeClass::CopyOrExportEvidence,
            alternative_summary:
                "Export the release packet and scope-review evidence without exposing raw \
                 delegated or service-issued material."
                    .to_owned(),
            suggested_actor_class: None,
            suggested_mutation_mode: Some(WorkItemMutationMode::InspectOnly),
            narrowed_scope_refs: vec![],
            repair_hook_ref: Some("repair_hook:support_export:release_scope_review".to_owned()),
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
    ]
}

fn seed_invalidations() -> Vec<EffectiveScopeInvalidationEventRecord> {
    vec![
        EffectiveScopeInvalidationEventRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_INVALIDATION_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            event_id: "scope_review:invalidation:ci:revoked".to_owned(),
            event_time: "2026-06-12T20:11:00Z".to_owned(),
            originating_resolution_id: "scope_review:resolution:ci:install:rerun".to_owned(),
            connected_provider_record_id: "connected_provider:ci:payments".to_owned(),
            actor_class: ProviderActorClass::InstallationOrAppGrant,
            invalidation_trigger_class: ScopeReviewInvalidationTriggerClass::ActorClassRevoked,
            originating_account_invalidation_event_ref: Some(
                "account_invalidation:ci-bot:grant_revoked".to_owned(),
            ),
            downgrade_action_class: ScopeReviewDowngradeActionClass::ForceInspectOnlyUntilRepair,
            rationale_summary:
                "The installation grant was revoked, so cached check-run write authority is \
                 invalidated and rerun controls degrade to inspect-only."
                    .to_owned(),
            invalidated_dependent_refs: vec![
                "approval_ticket:check_run:9876".to_owned(),
                "scope_review:resolution:ci:install:rerun".to_owned(),
            ],
            policy_epoch: "policy-epoch:2026-06-12T20:10:45Z".to_owned(),
            trust_posture: TrustPosture::Trusted,
            repair_hook_ref: Some("repair_hook:installation_grant:reconsent".to_owned()),
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        EffectiveScopeInvalidationEventRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_INVALIDATION_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            event_id: "scope_review:invalidation:release:host_mismatch".to_owned(),
            event_time: "2026-06-12T20:11:30Z".to_owned(),
            originating_resolution_id: "scope_review:resolution:release:host_mismatch:denied"
                .to_owned(),
            connected_provider_record_id: "connected_provider:release:fleet".to_owned(),
            actor_class: ProviderActorClass::PolicyInjectedServiceIdentity,
            invalidation_trigger_class: ScopeReviewInvalidationTriggerClass::HostMismatchDetected,
            originating_account_invalidation_event_ref: None,
            downgrade_action_class: ScopeReviewDowngradeActionClass::ForceDisconnectUntilRepair,
            rationale_summary:
                "The release publisher host binding drifted; cached provider scope is \
                 disconnected until the host and tenant are reviewed."
                    .to_owned(),
            invalidated_dependent_refs: vec![
                "scope_review:resolution:release:host_mismatch:denied".to_owned(),
            ],
            policy_epoch: "policy-epoch:2026-06-12T20:11:20Z".to_owned(),
            trust_posture: TrustPosture::Trusted,
            repair_hook_ref: Some("repair_hook:release:refresh_binding".to_owned()),
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        EffectiveScopeInvalidationEventRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_INVALIDATION_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            event_id: "scope_review:invalidation:release:tenant_switch".to_owned(),
            event_time: "2026-06-12T20:12:00Z".to_owned(),
            originating_resolution_id: "scope_review:resolution:release:host_mismatch:denied"
                .to_owned(),
            connected_provider_record_id: "connected_provider:release:fleet".to_owned(),
            actor_class: ProviderActorClass::PolicyInjectedServiceIdentity,
            invalidation_trigger_class: ScopeReviewInvalidationTriggerClass::TenantSwitchDetected,
            originating_account_invalidation_event_ref: None,
            downgrade_action_class: ScopeReviewDowngradeActionClass::ForceAccountReselection,
            rationale_summary:
                "The managed publisher tenant changed, so cached effective scope is invalidated \
                 and the release route requires an explicit tenant reselection."
                    .to_owned(),
            invalidated_dependent_refs: vec!["approval_ticket:release:fleet-0001".to_owned()],
            policy_epoch: "policy-epoch:2026-06-12T20:11:50Z".to_owned(),
            trust_posture: TrustPosture::Trusted,
            repair_hook_ref: Some("repair_hook:tenant_reselect:release".to_owned()),
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        EffectiveScopeInvalidationEventRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_INVALIDATION_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            event_id: "scope_review:invalidation:release:org_membership_lost".to_owned(),
            event_time: "2026-06-12T20:12:30Z".to_owned(),
            originating_resolution_id: "scope_review:resolution:release:host_mismatch:denied"
                .to_owned(),
            connected_provider_record_id: "connected_provider:release:fleet".to_owned(),
            actor_class: ProviderActorClass::PolicyInjectedServiceIdentity,
            invalidation_trigger_class: ScopeReviewInvalidationTriggerClass::OrgMembershipLost,
            originating_account_invalidation_event_ref: None,
            downgrade_action_class: ScopeReviewDowngradeActionClass::ForceAdminReview,
            rationale_summary:
                "Required org membership was lost, so release authority downgrades to admin \
                 review and support/export keeps the degraded state visible."
                    .to_owned(),
            invalidated_dependent_refs: vec![
                "scope_review:resolution:release:host_mismatch:denied".to_owned(),
            ],
            policy_epoch: "policy-epoch:2026-06-12T20:12:20Z".to_owned(),
            trust_posture: TrustPosture::Trusted,
            repair_hook_ref: Some("repair_hook:admin_review:org_membership".to_owned()),
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
        EffectiveScopeInvalidationEventRecord {
            record_kind: PROVIDER_SCOPE_REVIEW_INVALIDATION_RECORD_KIND.to_owned(),
            schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
            shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
            event_id: "scope_review:invalidation:item:provider_health".to_owned(),
            event_time: "2026-06-12T20:13:00Z".to_owned(),
            originating_resolution_id: "scope_review:resolution:issue:delegated:local_draft"
                .to_owned(),
            connected_provider_record_id: "connected_provider:issues:planning".to_owned(),
            actor_class: ProviderActorClass::DelegatedUserToken,
            invalidation_trigger_class: ScopeReviewInvalidationTriggerClass::ProviderHealthDegraded,
            originating_account_invalidation_event_ref: None,
            downgrade_action_class: ScopeReviewDowngradeActionClass::ForceLocalDraftOnlyUntilRepair,
            rationale_summary:
                "Provider health degraded after the deferred decision was computed, so queued \
                 publishes remain local and visible until the route recovers."
                    .to_owned(),
            invalidated_dependent_refs: vec!["publish_later:item:eng-84".to_owned()],
            policy_epoch: "policy-epoch:2026-06-12T20:12:50Z".to_owned(),
            trust_posture: TrustPosture::Trusted,
            repair_hook_ref: Some("repair_hook:provider_health:planning".to_owned()),
            redaction_class: RedactionClass::MetadataSafeDefault,
        },
    ]
}

fn seed_consumer_projections(
    resolutions: &[ProviderScopeResolutionRecord],
) -> Vec<ScopeReviewConsumerProjectionRecord> {
    let source = resolutions
        .iter()
        .find(|row| row.resolution_id == "scope_review:resolution:release:host_mismatch:denied")
        .expect("release scope review source");

    [
        ScopeReviewSurfaceClass::Desktop,
        ScopeReviewSurfaceClass::CliHeadless,
        ScopeReviewSurfaceClass::Companion,
        ScopeReviewSurfaceClass::SupportExport,
    ]
    .into_iter()
    .map(|surface_class| ScopeReviewConsumerProjectionRecord {
        record_kind: PROVIDER_SCOPE_REVIEW_CONSUMER_PROJECTION_RECORD_KIND.to_owned(),
        schema_version: PROVIDER_SCOPE_REVIEW_SCHEMA_VERSION,
        shared_contract_ref: PROVIDER_SCOPE_REVIEW_SHARED_CONTRACT_REF.to_owned(),
        projection_id: format!(
            "scope_review:projection:{}:release_host_mismatch",
            surface_class.as_str()
        ),
        surface_class,
        originating_resolution_id: source.resolution_id.clone(),
        projected_decision_class: source.decision_class,
        projected_grant_resolution_reason: source.grant_resolution_reason,
        projected_decision_summary: source.decision_summary.clone(),
        projected_alternative_refs: source.least_privilege_alternative_refs.clone(),
        raw_scope_material_present: false,
    })
    .collect()
}

fn provider_family_token(family: ProviderFamily) -> &'static str {
    match family {
        ProviderFamily::CodeHost => "code_host",
        ProviderFamily::IssueTracker => "issue_tracker",
        ProviderFamily::CiChecks => "ci_checks",
    }
}

fn provider_actor_class_token(actor_class: ProviderActorClass) -> &'static str {
    match actor_class {
        ProviderActorClass::HumanAccount => "human_account",
        ProviderActorClass::InstallationOrAppGrant => "installation_or_app_grant",
        ProviderActorClass::DelegatedUserToken => "delegated_user_token",
        ProviderActorClass::ProjectScopedGrant => "project_scoped_grant",
        ProviderActorClass::PolicyInjectedServiceIdentity => "policy_injected_service_identity",
        ProviderActorClass::UnknownActorClass => "unknown_actor_class",
    }
}
