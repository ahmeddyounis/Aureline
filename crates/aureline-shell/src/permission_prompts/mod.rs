//! Bounded typed-permission-prompt wedge on one certified
//! ecosystem-bearing install-review path.
//!
//! ## What the wedge is for
//!
//! Without a typed prompt contract, every authority-bearing surface is
//! free to render a generic "Install?" / "Allow?" button. The user
//! cannot tell who is asking, what boundary is being crossed, why it is
//! needed, what still works if they decline, or how long the grant will
//! last. The bounded prototype in this module is the first M1 surface
//! that pins those facts in one inspectable record, projected directly
//! from the upstream [`crate::install_review_fact_grid`] truth, so the
//! chrome cannot collapse the prompt into a one-word approval.
//!
//! ## Reused vocabularies
//!
//! - [`crate::install_review_fact_grid::InstallReviewFactGridRecord`]
//!   carries the publisher identity, origin, declared permissions, the
//!   declared-vs-effective diff, the activation budget, the rollback
//!   posture, and the typed install decision. The wedge does not fork
//!   any of those vocabularies; it projects them into the prompt
//!   record verbatim.
//! - [`aureline_extensions::manifest_baseline::PermissionScopeClass`] is
//!   the closed permission-scope vocabulary the requested-capability
//!   list quotes.
//! - [`crate::state_cards::DegradedStateToken`] supplies the chrome chip
//!   when the prompt is rendered for review-only / policy-blocked
//!   paths.
//!
//! ## What the wedge owns (and only what it owns)
//!
//! The wedge owns five new closed vocabularies the upstream
//! install-review fact grid does not own:
//!
//! - [`RequesterClass`] — who is asking (`extension`,
//!   `extension_publisher_flow`, `user_initiated_install`).
//! - [`AuthorityIssuerClass`] — who owns the decision (`shell`,
//!   `policy_service`).
//! - [`ScopeFilterClass`] — what scope filter the grant applies to
//!   (`current_root`, `named_workset`, `full_workspace`,
//!   `policy_limited_view`).
//! - [`GrantScopeClass`] — how long the grant lasts (`once`, `session`,
//!   `workspace`, `profile`, `policy_managed`).
//! - [`DegradedCapabilityClass`] — what still works if the prompt is
//!   declined (`no_degrade_available`, `local_only_continues`,
//!   `read_only_inspection_continues`, `preview_only_continues`,
//!   `metadata_only_export`, `install_disabled_capability_removed`).
//!
//! Plus three frozen support vocabularies the chrome quotes verbatim:
//!
//! - [`DecisionActionRole`] — the closed action-role set for each
//!   offered button (`primary_approve`, `primary_deny`,
//!   `safer_alternative`, `details`, `request_admin_review`).
//! - [`TypedPermissionPromptClaimLimit`] — the canonical claim-limit
//!   set under every card so the chrome cannot imply org-policy-pack
//!   or admin-console breadth.
//! - [`TypedPermissionPromptInvariantViolation`] — the typed failures
//!   the chrome MUST surface verbatim before any approve action is
//!   offered.
//!
//! ## Bounded scope (deliberately)
//!
//! - One ecosystem-bearing prototype path only — the certified
//!   extension-install-review path described in
//!   [`crate::install_review_fact_grid`]. Org policy packs, admin
//!   approval consoles, and multi-lane permission-system productization
//!   stay out of scope.
//! - The wedge is read-only: it does not own the install pipeline, the
//!   policy-pack narrowing engine, or the persistent grant store. It
//!   projects the fact grid's truth into a prompt record the chrome
//!   quotes verbatim.

use serde::{Deserialize, Serialize};

use aureline_extensions::manifest_baseline::{InstallDecisionClass, PermissionScopeClass};

use crate::install_review_fact_grid::{
    InstallReviewFactGridRecord, InstallReviewFactGridWedge,
};
use crate::state_cards::DegradedStateToken;

/// Stable record-kind tag carried on serialized
/// [`TypedPermissionPromptRecord`] payloads.
pub const TYPED_PERMISSION_PROMPT_RECORD_KIND: &str = "typed_permission_prompt_record";

/// Schema version for the [`TypedPermissionPromptRecord`] payload shape.
pub const TYPED_PERMISSION_PROMPT_SCHEMA_VERSION: u32 = 1;

/// Prototype label carried on every card. Chrome quotes the token
/// verbatim; the chip MUST NOT be dropped.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrototypeLabel {
    /// Bounded M1 prototype: typed permission prompt on one certified
    /// install-review wedge.
    M1PrototypeTypedPermissionPrompt,
}

impl PrototypeLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::M1PrototypeTypedPermissionPrompt => "m1_prototype_typed_permission_prompt",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::M1PrototypeTypedPermissionPrompt => {
                "Prototype — typed permission prompt (one certified install-review wedge)"
            }
        }
    }
}

/// Closed requester-class vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequesterClass {
    /// An extension manifest asked the user to grant authority before
    /// the install / review wedge commits.
    Extension,
    /// A publisher-side flow (the extension's vendor) asked for
    /// authority widening on behalf of the published extension.
    ExtensionPublisherFlow,
    /// The user explicitly initiated an install / review on a row they
    /// already opened.
    UserInitiatedInstall,
}

impl RequesterClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Extension => "extension",
            Self::ExtensionPublisherFlow => "extension_publisher_flow",
            Self::UserInitiatedInstall => "user_initiated_install",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Extension => "Extension manifest",
            Self::ExtensionPublisherFlow => "Extension publisher flow",
            Self::UserInitiatedInstall => "User-initiated install",
        }
    }
}

/// Closed authority-issuer vocabulary mirrored from the upstream
/// runtime-authority contract (issuer is one of `shell`,
/// `policy_service`, `supervisor`; only `shell` and `policy_service`
/// can issue this M1 prompt).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthorityIssuerClass {
    /// The user-facing shell owns the decision and minted the prompt.
    Shell,
    /// A policy service narrows or forbids the grant; the shell renders
    /// the prompt but cannot widen beyond what the policy permits.
    PolicyService,
}

impl AuthorityIssuerClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::PolicyService => "policy_service",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Shell => "Shell (user-facing approval)",
            Self::PolicyService => "Policy service (admin-managed)",
        }
    }
}

/// Closed scope-filter vocabulary mirrored from the trust-prompt
/// contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScopeFilterClass {
    /// Grant applies to the current workspace root only.
    CurrentRoot,
    /// Grant applies to a named, declared workset under the current
    /// root.
    NamedWorkset,
    /// Grant applies to the whole workspace (every root, every member).
    FullWorkspace,
    /// Policy already narrows the visible scope; the grant cannot widen
    /// beyond it.
    PolicyLimitedView,
}

impl ScopeFilterClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CurrentRoot => "current_root",
            Self::NamedWorkset => "named_workset",
            Self::FullWorkspace => "full_workspace",
            Self::PolicyLimitedView => "policy_limited_view",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::CurrentRoot => "Current workspace root",
            Self::NamedWorkset => "Named workset under the current root",
            Self::FullWorkspace => "Entire workspace",
            Self::PolicyLimitedView => "Policy-limited view (cannot widen)",
        }
    }
}

/// Closed grant-scope (persistence) vocabulary mirrored from the
/// trust-prompt contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GrantScopeClass {
    /// One-shot grant for this single action only.
    Once,
    /// Session grant — expires when this shell session ends.
    Session,
    /// Workspace grant — persists for the current workspace until
    /// revoked.
    Workspace,
    /// Profile grant — persists across every workspace under this user
    /// profile until revoked.
    Profile,
    /// Policy-managed grant — the user cannot remember the decision;
    /// the policy service is the only authority.
    PolicyManaged,
}

impl GrantScopeClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Once => "once",
            Self::Session => "session",
            Self::Workspace => "workspace",
            Self::Profile => "profile",
            Self::PolicyManaged => "policy_managed",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Once => "Once — this action only",
            Self::Session => "Session — until the shell exits",
            Self::Workspace => "Workspace — until revoked",
            Self::Profile => "Profile — across workspaces, until revoked",
            Self::PolicyManaged => "Policy-managed — admin owns the decision",
        }
    }
}

/// Closed degraded-capability vocabulary — what still works if the
/// user declines the prompt.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DegradedCapabilityClass {
    /// Nothing else can continue; denying blocks the row entirely.
    NoDegradeAvailable,
    /// Local edits and search continue; the requested external
    /// boundary is not crossed.
    LocalOnlyContinues,
    /// Read-only inspection of the manifest, metadata, and declared
    /// permissions continues; activation is suppressed.
    ReadOnlyInspectionContinues,
    /// Preview-only continues; the install is not committed but the
    /// user can review.
    PreviewOnlyContinues,
    /// Metadata-only export remains available for support.
    MetadataOnlyExport,
    /// The capability is removed; the extension installs without it.
    InstallDisabledCapabilityRemoved,
}

impl DegradedCapabilityClass {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoDegradeAvailable => "no_degrade_available",
            Self::LocalOnlyContinues => "local_only_continues",
            Self::ReadOnlyInspectionContinues => "read_only_inspection_continues",
            Self::PreviewOnlyContinues => "preview_only_continues",
            Self::MetadataOnlyExport => "metadata_only_export",
            Self::InstallDisabledCapabilityRemoved => "install_disabled_capability_removed",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::NoDegradeAvailable => "No safe continuation — denying blocks the row",
            Self::LocalOnlyContinues => "Local work continues; external boundary stays closed",
            Self::ReadOnlyInspectionContinues => {
                "Read-only inspection of the manifest continues"
            }
            Self::PreviewOnlyContinues => "Preview-only continues; install is not committed",
            Self::MetadataOnlyExport => "Metadata-only support export remains available",
            Self::InstallDisabledCapabilityRemoved => {
                "Capability is removed; the extension installs without it"
            }
        }
    }
}

/// Closed decision-action-role vocabulary.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DecisionActionRole {
    /// Approve the requested grant for the named persistence scope.
    PrimaryApprove,
    /// Deny the requested grant; the chrome MUST show the deny path
    /// and the degraded-capability class.
    PrimaryDeny,
    /// A narrower, less-privileged alternative the user may pick
    /// instead of approving.
    SaferAlternative,
    /// Open the inline details / inspector route; not a decision.
    Details,
    /// Forward the request to an admin / policy owner for review; the
    /// chrome offers this when a policy-locked path needs admin action.
    RequestAdminReview,
}

impl DecisionActionRole {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryApprove => "primary_approve",
            Self::PrimaryDeny => "primary_deny",
            Self::SaferAlternative => "safer_alternative",
            Self::Details => "details",
            Self::RequestAdminReview => "request_admin_review",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::PrimaryApprove => "Approve",
            Self::PrimaryDeny => "Deny",
            Self::SaferAlternative => "Safer alternative",
            Self::Details => "Details",
            Self::RequestAdminReview => "Request admin review",
        }
    }
}

/// Closed decision-state vocabulary. Tracks whether the user has acted
/// on the prompt yet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PermissionPromptDecisionState {
    /// Prompt is rendered; the user has not approved or denied yet.
    Pending,
    /// User approved the grant. The persistence scope on the record is
    /// authoritative.
    Approved,
    /// User denied the grant. The denial branch / degraded-capability
    /// class on the record is authoritative.
    Denied,
    /// User dismissed the prompt without approving or denying.
    Cancelled,
    /// A policy lock forbids local approval; the chrome MUST NOT
    /// render the approve action.
    BlockedByPolicy,
}

impl PermissionPromptDecisionState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pending => "pending",
            Self::Approved => "approved",
            Self::Denied => "denied",
            Self::Cancelled => "cancelled",
            Self::BlockedByPolicy => "blocked_by_policy",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Pending => "Pending decision",
            Self::Approved => "Approved",
            Self::Denied => "Denied",
            Self::Cancelled => "Cancelled",
            Self::BlockedByPolicy => "Blocked by policy",
        }
    }
}

/// Frozen claim-limit vocabulary. The chrome quotes verbatim under
/// every card.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TypedPermissionPromptClaimLimit {
    /// One bounded prompt wedge only.
    SingleBoundedWedgeOnly,
    /// Not an org-policy-pack productization; only the live
    /// install-review path consumes it.
    NoOrgPolicyPackProductization,
    /// Not an admin-approval console; the wedge does not stand up an
    /// admin / managed approval surface.
    NoAdminApprovalConsole,
    /// Not a multi-lane permission system; the wedge does not own a
    /// publish, AI, or remote-attach prompt lane.
    NoMultiLanePermissionSystem,
}

impl TypedPermissionPromptClaimLimit {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleBoundedWedgeOnly => "single_bounded_wedge_only",
            Self::NoOrgPolicyPackProductization => "no_org_policy_pack_productization",
            Self::NoAdminApprovalConsole => "no_admin_approval_console",
            Self::NoMultiLanePermissionSystem => "no_multi_lane_permission_system",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::SingleBoundedWedgeOnly => {
                "One bounded permission-prompt wedge on the install-review path."
            }
            Self::NoOrgPolicyPackProductization => {
                "Does not productise org policy packs or remembered-decision stores."
            }
            Self::NoAdminApprovalConsole => {
                "Does not stand up an admin approval / managed-policy console."
            }
            Self::NoMultiLanePermissionSystem => {
                "Does not own a publish, AI, or remote-attach permission lane."
            }
        }
    }

    /// Canonical M1 claim-limit set. Order is stable; chrome MUST
    /// render in this order.
    pub const fn canonical_set() -> [TypedPermissionPromptClaimLimit; 4] {
        [
            Self::SingleBoundedWedgeOnly,
            Self::NoOrgPolicyPackProductization,
            Self::NoAdminApprovalConsole,
            Self::NoMultiLanePermissionSystem,
        ]
    }
}

/// One claim-limit row on the serialized card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedPermissionPromptClaimLimitRow {
    pub token: String,
    pub label: String,
}

impl TypedPermissionPromptClaimLimitRow {
    fn from_limit(limit: TypedPermissionPromptClaimLimit) -> Self {
        Self {
            token: limit.as_str().to_owned(),
            label: limit.label().to_owned(),
        }
    }
}

/// Closed invariant-violation vocabulary surfaced on the card.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", tag = "violation")]
pub enum TypedPermissionPromptInvariantViolation {
    /// The card is missing the prototype-label chip.
    MissingPrototypeLabel,
    /// The canonical claim-limit set is missing or out of order.
    ClaimLimitsMissingOrOutOfOrder,
    /// The requester ref / label is missing. The user cannot tell who
    /// is asking.
    RequesterIdentityMissing,
    /// The authority owner (issuer / source) is missing. The user
    /// cannot tell who owns the decision.
    AuthorityOwnerMissing,
    /// The scope filter or scope target is missing. The user cannot
    /// tell what boundary is being crossed.
    ScopeMissing,
    /// The grant-scope (persistence) field is missing. The user cannot
    /// tell how long the grant lasts.
    GrantPersistenceMissing,
    /// The denial branch description is empty. The user cannot tell
    /// what still works if they decline.
    DenyPathMissing,
    /// The consequence summary ("what changes if allowed") is missing.
    ConsequenceMissing,
    /// One of the five required prompt questions is unanswered.
    PromptQuestionUnanswered { question_token: String },
    /// The card offers an approve action while the underlying
    /// install-review fact grid carries invariant violations. Approving
    /// would commit on dishonest facts.
    ApproveOfferedWithBlockedInstallReview { upstream_violation_count: u32 },
    /// The fact-grid decision is `denied` but the prompt nonetheless
    /// offers an approve action.
    ApproveOfferedAgainstDeniedDecision { install_decision_class: String },
    /// The card has no deny action. A typed prompt MUST always offer a
    /// deny path.
    NoDenyActionPath,
    /// A non-policy-blocked, non-denied row does not offer any approve
    /// action even though the fact-grid decision permits one.
    NoApprovePathOnAdmittableRow { install_decision_class: String },
    /// The grant scope is `once` but the prompt's persistence label
    /// implies a longer-lasting grant (or vice versa). The chrome
    /// would mislead the user about how long the grant lasts.
    GrantPersistenceInconsistent {
        grant_scope_class: String,
        persistence_label: String,
    },
    /// Decision state is `approved` but the card carries one or more
    /// blocking invariants. Approval MUST NOT proceed on a blocked
    /// card.
    ApprovedWhileBlocked { blocking_count: u32 },
}

impl TypedPermissionPromptInvariantViolation {
    pub fn token(&self) -> &'static str {
        match self {
            Self::MissingPrototypeLabel => "missing_prototype_label",
            Self::ClaimLimitsMissingOrOutOfOrder => "claim_limits_missing_or_out_of_order",
            Self::RequesterIdentityMissing => "requester_identity_missing",
            Self::AuthorityOwnerMissing => "authority_owner_missing",
            Self::ScopeMissing => "scope_missing",
            Self::GrantPersistenceMissing => "grant_persistence_missing",
            Self::DenyPathMissing => "deny_path_missing",
            Self::ConsequenceMissing => "consequence_missing",
            Self::PromptQuestionUnanswered { .. } => "prompt_question_unanswered",
            Self::ApproveOfferedWithBlockedInstallReview { .. } => {
                "approve_offered_with_blocked_install_review"
            }
            Self::ApproveOfferedAgainstDeniedDecision { .. } => {
                "approve_offered_against_denied_decision"
            }
            Self::NoDenyActionPath => "no_deny_action_path",
            Self::NoApprovePathOnAdmittableRow { .. } => "no_approve_path_on_admittable_row",
            Self::GrantPersistenceInconsistent { .. } => "grant_persistence_inconsistent",
            Self::ApprovedWhileBlocked { .. } => "approved_while_blocked",
        }
    }

    pub fn label(&self) -> String {
        match self {
            Self::MissingPrototypeLabel => "Prototype-label chip is missing.".to_owned(),
            Self::ClaimLimitsMissingOrOutOfOrder => {
                "Canonical claim-limit set is missing or out of order.".to_owned()
            }
            Self::RequesterIdentityMissing => {
                "Requester identity is missing; the prompt cannot answer 'who is asking'."
                    .to_owned()
            }
            Self::AuthorityOwnerMissing => {
                "Authority owner is missing; the prompt cannot answer who owns the decision."
                    .to_owned()
            }
            Self::ScopeMissing => {
                "Scope is missing; the prompt cannot answer 'what boundary is being crossed'."
                    .to_owned()
            }
            Self::GrantPersistenceMissing => {
                "Grant persistence is missing; the prompt cannot answer how long the grant lasts."
                    .to_owned()
            }
            Self::DenyPathMissing => {
                "Deny path is missing; the prompt cannot answer what still works if denied."
                    .to_owned()
            }
            Self::ConsequenceMissing => {
                "Consequence summary is missing; the prompt cannot answer what changes if allowed."
                    .to_owned()
            }
            Self::PromptQuestionUnanswered { question_token } => format!(
                "Required prompt question {question_token} is unanswered; the prompt MUST refuse to render a generic approval."
            ),
            Self::ApproveOfferedWithBlockedInstallReview { upstream_violation_count } => format!(
                "Approve action offered while the upstream install-review fact grid carries {upstream_violation_count} blocking invariant(s); approval MUST be withheld."
            ),
            Self::ApproveOfferedAgainstDeniedDecision { install_decision_class } => format!(
                "Approve action offered while install_decision_class is {install_decision_class}; the chrome MUST NOT offer approval against a denied decision."
            ),
            Self::NoDenyActionPath => {
                "No deny action is offered; a typed prompt MUST always offer a deny path."
                    .to_owned()
            }
            Self::NoApprovePathOnAdmittableRow { install_decision_class } => format!(
                "No approve action is offered even though install_decision_class is {install_decision_class}; the prompt is unactionable."
            ),
            Self::GrantPersistenceInconsistent { grant_scope_class, persistence_label } => format!(
                "Grant-scope token {grant_scope_class} disagrees with the rendered persistence label {persistence_label}; the chrome would mislead the user."
            ),
            Self::ApprovedWhileBlocked { blocking_count } => format!(
                "Decision state is approved but the card carries {blocking_count} blocking invariant(s); approval MUST NOT proceed."
            ),
        }
    }
}

/// One invariant row on the serialized card.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedPermissionPromptInvariantRow {
    pub violation_token: String,
    pub violation_label: String,
    pub violation: TypedPermissionPromptInvariantViolation,
}

impl TypedPermissionPromptInvariantRow {
    fn from_violation(violation: TypedPermissionPromptInvariantViolation) -> Self {
        Self {
            violation_token: violation.token().to_owned(),
            violation_label: violation.label(),
            violation,
        }
    }
}

/// Requester (actor) block — who is asking?
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionPromptRequester {
    pub requester_class: RequesterClass,
    pub requester_class_token: String,
    pub requester_ref: String,
    pub requester_display_label: String,
    pub request_origin_label: String,
}

/// Authority owner block — who owns the decision?
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionPromptAuthorityOwner {
    pub issuer_class: AuthorityIssuerClass,
    pub issuer_class_token: String,
    pub issuer_source_ref: String,
    pub issuer_source_label: String,
}

/// Scope block — what boundary is being crossed?
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionPromptScope {
    pub scope_filter_class: ScopeFilterClass,
    pub scope_filter_token: String,
    pub scope_target_label: String,
    pub grant_scope_class: GrantScopeClass,
    pub grant_scope_token: String,
    pub grant_persistence_label: String,
    pub requested_permissions: Vec<PermissionPromptCapabilityRow>,
}

/// One requested-capability row, projected from a manifest declared
/// permission.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionPromptCapabilityRow {
    pub scope_class: PermissionScopeClass,
    pub scope_class_token: String,
    pub scope_target: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_constraint: Option<String>,
    pub rationale_label: String,
}

/// Consequence block — what changes if allowed?
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionPromptConsequence {
    pub consequence_summary: String,
    pub install_decision_class_token: String,
    pub install_decision_reason_class_token: String,
}

/// Denial-branch block — what still works if denied?
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionPromptDenialBranch {
    pub degraded_capability_class: DegradedCapabilityClass,
    pub degraded_capability_token: String,
    pub deny_path_label: String,
    pub preserved_work_refs: Vec<String>,
}

/// The five required prompt questions every typed prompt must answer.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionPromptQuestions {
    /// Who is asking?
    pub who_is_asking: String,
    /// What boundary is being crossed?
    pub what_boundary: String,
    /// Why is it needed?
    pub why_needed: String,
    /// What changes if allowed?
    pub what_changes_if_allowed: String,
    /// What still works if denied?
    pub what_works_if_denied: String,
    /// How long does the grant last?
    pub grant_persistence_statement: String,
}

impl PermissionPromptQuestions {
    fn answered(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if self.who_is_asking.trim().is_empty() {
            missing.push("who_is_asking");
        }
        if self.what_boundary.trim().is_empty() {
            missing.push("what_boundary");
        }
        if self.why_needed.trim().is_empty() {
            missing.push("why_needed");
        }
        if self.what_changes_if_allowed.trim().is_empty() {
            missing.push("what_changes_if_allowed");
        }
        if self.what_works_if_denied.trim().is_empty() {
            missing.push("what_works_if_denied");
        }
        if self.grant_persistence_statement.trim().is_empty() {
            missing.push("grant_persistence_statement");
        }
        missing
    }
}

/// One offered decision action button.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionPromptDecisionAction {
    pub action_role: DecisionActionRole,
    pub action_role_token: String,
    pub action_id: String,
    pub label: String,
    pub resulting_state_class: PermissionPromptDecisionState,
    pub resulting_state_token: String,
}

/// Serialized typed-permission-prompt record. Chrome and exports quote
/// verbatim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedPermissionPromptRecord {
    pub record_kind: String,
    pub schema_version: u32,
    pub prototype_label_token: String,
    pub prototype_label_display: String,
    pub prompt_id: String,
    pub install_review_card_ref: String,
    pub install_review_card_clean: bool,
    pub upstream_install_decision_class_token: String,
    pub upstream_invariant_violation_count: u32,
    pub requester: PermissionPromptRequester,
    pub authority_owner: PermissionPromptAuthorityOwner,
    pub scope: PermissionPromptScope,
    pub consequence: PermissionPromptConsequence,
    pub denial_branch: PermissionPromptDenialBranch,
    pub prompt_questions: PermissionPromptQuestions,
    pub decision_actions: Vec<PermissionPromptDecisionAction>,
    pub decision_state: PermissionPromptDecisionState,
    pub decision_state_token: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub degraded_token: Option<String>,
    pub claim_limits: Vec<TypedPermissionPromptClaimLimitRow>,
    pub invariants: Vec<TypedPermissionPromptInvariantRow>,
    pub has_invariant_violations: bool,
    pub approve_action_offered: bool,
    pub deny_action_offered: bool,
    pub summary_line: String,
}

impl TypedPermissionPromptRecord {
    /// Deterministic plaintext block. Stable across hosts; never bakes
    /// in wall-clock time.
    pub fn render_plaintext(&self) -> String {
        let mut out = String::new();
        out.push_str(&format!(
            "[{}] {}\n",
            self.prototype_label_token, self.prototype_label_display
        ));
        out.push_str(&format!(
            "prompt_id={} install_review_card_ref={} install_review_clean={}\n",
            self.prompt_id, self.install_review_card_ref, self.install_review_card_clean,
        ));
        out.push_str(&format!(
            "upstream_install_decision={} upstream_invariant_count={}\n",
            self.upstream_install_decision_class_token, self.upstream_invariant_violation_count,
        ));
        out.push_str("requester:\n");
        out.push_str(&format!(
            "  class={} ref={} display={} origin={}\n",
            self.requester.requester_class_token,
            self.requester.requester_ref,
            self.requester.requester_display_label,
            self.requester.request_origin_label,
        ));
        out.push_str("authority_owner:\n");
        out.push_str(&format!(
            "  issuer={} source_ref={} source_label={}\n",
            self.authority_owner.issuer_class_token,
            self.authority_owner.issuer_source_ref,
            self.authority_owner.issuer_source_label,
        ));
        out.push_str("scope:\n");
        out.push_str(&format!(
            "  filter={} target={} grant_scope={} persistence={}\n",
            self.scope.scope_filter_token,
            self.scope.scope_target_label,
            self.scope.grant_scope_token,
            self.scope.grant_persistence_label,
        ));
        out.push_str("requested_permissions:\n");
        if self.scope.requested_permissions.is_empty() {
            out.push_str("  - (none)\n");
        } else {
            for row in &self.scope.requested_permissions {
                let constraint = row
                    .scope_constraint
                    .as_deref()
                    .map(|c| format!(" constraint={c}"))
                    .unwrap_or_default();
                out.push_str(&format!(
                    "  - {}={}{} rationale={}\n",
                    row.scope_class_token, row.scope_target, constraint, row.rationale_label,
                ));
            }
        }
        out.push_str("consequence:\n");
        out.push_str(&format!(
            "  install_decision={} reason={} summary={}\n",
            self.consequence.install_decision_class_token,
            self.consequence.install_decision_reason_class_token,
            self.consequence.consequence_summary,
        ));
        out.push_str("denial_branch:\n");
        out.push_str(&format!(
            "  degraded_capability={} deny_path={}\n",
            self.denial_branch.degraded_capability_token, self.denial_branch.deny_path_label,
        ));
        if self.denial_branch.preserved_work_refs.is_empty() {
            out.push_str("  preserved_work_refs: (none)\n");
        } else {
            out.push_str("  preserved_work_refs:\n");
            for r in &self.denial_branch.preserved_work_refs {
                out.push_str(&format!("    - {r}\n"));
            }
        }
        out.push_str("prompt_questions:\n");
        out.push_str(&format!(
            "  who_is_asking={}\n",
            self.prompt_questions.who_is_asking
        ));
        out.push_str(&format!(
            "  what_boundary={}\n",
            self.prompt_questions.what_boundary
        ));
        out.push_str(&format!(
            "  why_needed={}\n",
            self.prompt_questions.why_needed
        ));
        out.push_str(&format!(
            "  what_changes_if_allowed={}\n",
            self.prompt_questions.what_changes_if_allowed
        ));
        out.push_str(&format!(
            "  what_works_if_denied={}\n",
            self.prompt_questions.what_works_if_denied
        ));
        out.push_str(&format!(
            "  grant_persistence_statement={}\n",
            self.prompt_questions.grant_persistence_statement
        ));
        out.push_str("decision_actions:\n");
        if self.decision_actions.is_empty() {
            out.push_str("  - (none)\n");
        } else {
            for a in &self.decision_actions {
                out.push_str(&format!(
                    "  - role={} id={} label={} resulting_state={}\n",
                    a.action_role_token, a.action_id, a.label, a.resulting_state_token,
                ));
            }
        }
        out.push_str(&format!(
            "decision_state={}\napprove_offered={} deny_offered={}\n",
            self.decision_state_token, self.approve_action_offered, self.deny_action_offered,
        ));
        if let Some(token) = &self.degraded_token {
            out.push_str(&format!("degraded={token}\n"));
        }
        out.push_str("claim_limits:\n");
        for row in &self.claim_limits {
            out.push_str(&format!("  - {}: {}\n", row.token, row.label));
        }
        out.push_str("invariants:\n");
        if self.invariants.is_empty() {
            out.push_str("  - clean\n");
        } else {
            for row in &self.invariants {
                out.push_str(&format!(
                    "  - {}: {}\n",
                    row.violation_token, row.violation_label
                ));
            }
        }
        out.push_str(&format!("summary: {}\n", self.summary_line));
        out
    }

    /// True when the prompt is clean (no invariant violations) and
    /// the decision state is `approved`.
    pub fn is_clean_approve(&self) -> bool {
        !self.has_invariant_violations
            && matches!(self.decision_state, PermissionPromptDecisionState::Approved)
    }

    /// True when the prompt is clean and the decision state is
    /// `denied` (an honest decline).
    pub fn is_clean_deny(&self) -> bool {
        !self.has_invariant_violations
            && matches!(self.decision_state, PermissionPromptDecisionState::Denied)
    }
}

/// Bounded typed-permission-prompt wedge.
#[derive(Debug, Clone)]
pub struct PermissionPromptWedge {
    install_review: InstallReviewFactGridRecord,
    prompt_id: String,
    requester: PermissionPromptRequester,
    authority_owner: PermissionPromptAuthorityOwner,
    scope_filter: ScopeFilterClass,
    scope_target_label: String,
    grant_scope: GrantScopeClass,
    grant_persistence_label: String,
    denial_branch: PermissionPromptDenialBranch,
    questions: PermissionPromptQuestions,
    decision_state: PermissionPromptDecisionState,
    degraded_token: Option<DegradedStateToken>,
    /// When true, the wedge is asked to render a no-deny path so the
    /// failure drill can assert the typed `no_deny_action_path`
    /// invariant fires.
    suppress_deny_action: bool,
    /// Force-offer an approve action even when the fact-grid forbids
    /// it (used by failure drills to prove the invariant fires).
    force_offer_approve_on_blocked: bool,
}

impl PermissionPromptWedge {
    /// Build a new wedge.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        install_review: InstallReviewFactGridRecord,
        prompt_id: impl Into<String>,
        requester: PermissionPromptRequester,
        authority_owner: PermissionPromptAuthorityOwner,
        scope_filter: ScopeFilterClass,
        scope_target_label: impl Into<String>,
        grant_scope: GrantScopeClass,
        grant_persistence_label: impl Into<String>,
        denial_branch: PermissionPromptDenialBranch,
        questions: PermissionPromptQuestions,
    ) -> Self {
        Self {
            install_review,
            prompt_id: prompt_id.into(),
            requester,
            authority_owner,
            scope_filter,
            scope_target_label: scope_target_label.into(),
            grant_scope,
            grant_persistence_label: grant_persistence_label.into(),
            denial_branch,
            questions,
            decision_state: PermissionPromptDecisionState::Pending,
            degraded_token: None,
            suppress_deny_action: false,
            force_offer_approve_on_blocked: false,
        }
    }

    /// Convenience: build the wedge from an already-constructed fact
    /// grid wedge by materialising its card.
    #[allow(clippy::too_many_arguments)]
    pub fn from_install_review_wedge(
        fact_grid_wedge: &InstallReviewFactGridWedge,
        prompt_id: impl Into<String>,
        requester: PermissionPromptRequester,
        authority_owner: PermissionPromptAuthorityOwner,
        scope_filter: ScopeFilterClass,
        scope_target_label: impl Into<String>,
        grant_scope: GrantScopeClass,
        grant_persistence_label: impl Into<String>,
        denial_branch: PermissionPromptDenialBranch,
        questions: PermissionPromptQuestions,
    ) -> Self {
        Self::new(
            fact_grid_wedge.card(),
            prompt_id,
            requester,
            authority_owner,
            scope_filter,
            scope_target_label,
            grant_scope,
            grant_persistence_label,
            denial_branch,
            questions,
        )
    }

    pub fn with_degraded(mut self, token: DegradedStateToken) -> Self {
        self.degraded_token = Some(token);
        self
    }

    /// Mark the decision state. The wedge does not own a persistent
    /// grant store; this only records what the user did on this
    /// prompt.
    pub fn with_decision_state(mut self, state: PermissionPromptDecisionState) -> Self {
        self.decision_state = state;
        self
    }

    /// Failure-drill toggle: suppress the deny action so the typed
    /// `no_deny_action_path` invariant can be asserted.
    pub fn with_suppress_deny_action(mut self, value: bool) -> Self {
        self.suppress_deny_action = value;
        self
    }

    /// Failure-drill toggle: force-offer the approve action even when
    /// the upstream fact grid is blocked.
    pub fn with_force_offer_approve_on_blocked(mut self, value: bool) -> Self {
        self.force_offer_approve_on_blocked = value;
        self
    }

    /// Materialise the current prompt card.
    pub fn card(&self) -> TypedPermissionPromptRecord {
        let label = PrototypeLabel::M1PrototypeTypedPermissionPrompt;
        let install_review_clean = !self.install_review.has_invariant_violations;
        let upstream_invariant_count = self.install_review.invariants.len() as u32;
        let upstream_install_decision_class_token = self
            .install_review
            .decision
            .install_decision_class_token
            .clone();
        let upstream_install_decision_class = self.install_review.decision.install_decision_class;

        let scope = PermissionPromptScope {
            scope_filter_class: self.scope_filter,
            scope_filter_token: self.scope_filter.as_str().to_owned(),
            scope_target_label: self.scope_target_label.clone(),
            grant_scope_class: self.grant_scope,
            grant_scope_token: self.grant_scope.as_str().to_owned(),
            grant_persistence_label: self.grant_persistence_label.clone(),
            requested_permissions: self
                .install_review
                .declared_permissions
                .iter()
                .map(|row| PermissionPromptCapabilityRow {
                    scope_class: row.scope_class,
                    scope_class_token: row.scope_class_token.clone(),
                    scope_target: row.scope_target.clone(),
                    scope_constraint: row.scope_constraint.clone(),
                    rationale_label: row.rationale_label.clone(),
                })
                .collect(),
        };

        let consequence = PermissionPromptConsequence {
            consequence_summary: self.install_review.decision.decision_summary.clone(),
            install_decision_class_token: self
                .install_review
                .decision
                .install_decision_class_token
                .clone(),
            install_decision_reason_class_token: self
                .install_review
                .decision
                .install_decision_reason_class_token
                .clone(),
        };

        let approve_offered = self.compute_approve_offered(
            install_review_clean,
            upstream_install_decision_class,
        );
        let deny_offered = !self.suppress_deny_action;

        let mut decision_actions: Vec<PermissionPromptDecisionAction> = Vec::new();
        if approve_offered {
            decision_actions.push(PermissionPromptDecisionAction {
                action_role: DecisionActionRole::PrimaryApprove,
                action_role_token: DecisionActionRole::PrimaryApprove.as_str().to_owned(),
                action_id: format!("{}.approve", self.prompt_id),
                label: format!(
                    "Approve — grant {} for {}",
                    scope.requested_permission_label(),
                    self.grant_scope.label(),
                ),
                resulting_state_class: PermissionPromptDecisionState::Approved,
                resulting_state_token: PermissionPromptDecisionState::Approved.as_str().to_owned(),
            });
        }
        if deny_offered {
            decision_actions.push(PermissionPromptDecisionAction {
                action_role: DecisionActionRole::PrimaryDeny,
                action_role_token: DecisionActionRole::PrimaryDeny.as_str().to_owned(),
                action_id: format!("{}.deny", self.prompt_id),
                label: format!(
                    "Deny — {}",
                    self.denial_branch.degraded_capability_class.label(),
                ),
                resulting_state_class: PermissionPromptDecisionState::Denied,
                resulting_state_token: PermissionPromptDecisionState::Denied.as_str().to_owned(),
            });
        }
        // Always offer details (read-only) and admin-review (for
        // policy-locked rows) so the chrome cannot turn the prompt
        // into a primary-only sheet.
        decision_actions.push(PermissionPromptDecisionAction {
            action_role: DecisionActionRole::Details,
            action_role_token: DecisionActionRole::Details.as_str().to_owned(),
            action_id: format!("{}.details", self.prompt_id),
            label: "Inspect details".to_owned(),
            resulting_state_class: PermissionPromptDecisionState::Pending,
            resulting_state_token: PermissionPromptDecisionState::Pending.as_str().to_owned(),
        });
        if matches!(self.authority_owner.issuer_class, AuthorityIssuerClass::PolicyService) {
            decision_actions.push(PermissionPromptDecisionAction {
                action_role: DecisionActionRole::RequestAdminReview,
                action_role_token: DecisionActionRole::RequestAdminReview.as_str().to_owned(),
                action_id: format!("{}.request_admin_review", self.prompt_id),
                label: "Request admin review".to_owned(),
                resulting_state_class: PermissionPromptDecisionState::BlockedByPolicy,
                resulting_state_token: PermissionPromptDecisionState::BlockedByPolicy
                    .as_str()
                    .to_owned(),
            });
        }

        let claim_limits: Vec<TypedPermissionPromptClaimLimitRow> =
            TypedPermissionPromptClaimLimit::canonical_set()
                .into_iter()
                .map(TypedPermissionPromptClaimLimitRow::from_limit)
                .collect();

        let invariants_raw = self.validate_invariants(
            install_review_clean,
            upstream_invariant_count,
            upstream_install_decision_class,
            approve_offered,
            deny_offered,
        );
        let has_invariant_violations = !invariants_raw.is_empty();
        let invariants: Vec<TypedPermissionPromptInvariantRow> = invariants_raw
            .into_iter()
            .map(TypedPermissionPromptInvariantRow::from_violation)
            .collect();
        let summary_line = self.summary_line(has_invariant_violations);

        TypedPermissionPromptRecord {
            record_kind: TYPED_PERMISSION_PROMPT_RECORD_KIND.to_owned(),
            schema_version: TYPED_PERMISSION_PROMPT_SCHEMA_VERSION,
            prototype_label_token: label.as_str().to_owned(),
            prototype_label_display: label.label().to_owned(),
            prompt_id: self.prompt_id.clone(),
            install_review_card_ref: self.install_review.wedge_id.clone(),
            install_review_card_clean: install_review_clean,
            upstream_install_decision_class_token,
            upstream_invariant_violation_count: upstream_invariant_count,
            requester: self.requester.clone(),
            authority_owner: self.authority_owner.clone(),
            scope,
            consequence,
            denial_branch: self.denial_branch.clone(),
            prompt_questions: self.questions.clone(),
            decision_actions,
            decision_state: self.decision_state,
            decision_state_token: self.decision_state.as_str().to_owned(),
            degraded_token: self.degraded_token.map(|t| t.token().to_owned()),
            claim_limits,
            invariants,
            has_invariant_violations,
            approve_action_offered: approve_offered,
            deny_action_offered: deny_offered,
            summary_line,
        }
    }

    fn compute_approve_offered(
        &self,
        install_review_clean: bool,
        upstream_install_decision_class: InstallDecisionClass,
    ) -> bool {
        if self.force_offer_approve_on_blocked {
            return true;
        }
        if !install_review_clean {
            return false;
        }
        // The chrome only offers approve when the fact grid's decision
        // permits it. `Denied` never offers approve; `ReviewOnly`
        // never auto-admits (so approve at the prompt level is also
        // suppressed).
        match upstream_install_decision_class {
            InstallDecisionClass::Admit | InstallDecisionClass::AdmitWithStepUp => true,
            InstallDecisionClass::ReviewOnly | InstallDecisionClass::Denied => false,
        }
    }

    fn summary_line(&self, has_invariant_violations: bool) -> String {
        let suffix = if has_invariant_violations {
            "INVARIANTS BLOCKED"
        } else {
            self.decision_state.as_str()
        };
        format!(
            "prompt={} requester={} issuer={} grant={} decision={} — {}",
            self.prompt_id,
            self.requester.requester_class_token,
            self.authority_owner.issuer_class_token,
            self.grant_scope.as_str(),
            self.install_review.decision.install_decision_class_token,
            suffix,
        )
    }

    fn validate_invariants(
        &self,
        install_review_clean: bool,
        upstream_invariant_count: u32,
        upstream_install_decision_class: InstallDecisionClass,
        approve_offered: bool,
        deny_offered: bool,
    ) -> Vec<TypedPermissionPromptInvariantViolation> {
        let mut out = Vec::new();

        // Requester identity.
        if self.requester.requester_ref.trim().is_empty()
            || self.requester.requester_display_label.trim().is_empty()
        {
            out.push(TypedPermissionPromptInvariantViolation::RequesterIdentityMissing);
        }
        // Authority owner identity.
        if self.authority_owner.issuer_source_ref.trim().is_empty()
            || self.authority_owner.issuer_source_label.trim().is_empty()
        {
            out.push(TypedPermissionPromptInvariantViolation::AuthorityOwnerMissing);
        }
        // Scope target.
        if self.scope_target_label.trim().is_empty() {
            out.push(TypedPermissionPromptInvariantViolation::ScopeMissing);
        }
        // Grant persistence label.
        if self.grant_persistence_label.trim().is_empty() {
            out.push(TypedPermissionPromptInvariantViolation::GrantPersistenceMissing);
        }
        // Deny path label.
        if self.denial_branch.deny_path_label.trim().is_empty() {
            out.push(TypedPermissionPromptInvariantViolation::DenyPathMissing);
        }
        // Consequence summary.
        if self.install_review.decision.decision_summary.trim().is_empty() {
            out.push(TypedPermissionPromptInvariantViolation::ConsequenceMissing);
        }
        // Prompt questions.
        for missing in self.questions.answered() {
            out.push(
                TypedPermissionPromptInvariantViolation::PromptQuestionUnanswered {
                    question_token: missing.to_owned(),
                },
            );
        }
        // No deny action.
        if !deny_offered {
            out.push(TypedPermissionPromptInvariantViolation::NoDenyActionPath);
        }
        // Approve offered against blocked upstream fact grid.
        if approve_offered && !install_review_clean {
            out.push(
                TypedPermissionPromptInvariantViolation::ApproveOfferedWithBlockedInstallReview {
                    upstream_violation_count: upstream_invariant_count,
                },
            );
        }
        // Approve offered against a denied install decision.
        if approve_offered && matches!(upstream_install_decision_class, InstallDecisionClass::Denied) {
            out.push(
                TypedPermissionPromptInvariantViolation::ApproveOfferedAgainstDeniedDecision {
                    install_decision_class: self
                        .install_review
                        .decision
                        .install_decision_class_token
                        .clone(),
                },
            );
        }
        // Admittable row but no approve path offered.
        if !approve_offered
            && install_review_clean
            && matches!(
                upstream_install_decision_class,
                InstallDecisionClass::Admit | InstallDecisionClass::AdmitWithStepUp
            )
            && !matches!(self.decision_state, PermissionPromptDecisionState::BlockedByPolicy)
        {
            out.push(
                TypedPermissionPromptInvariantViolation::NoApprovePathOnAdmittableRow {
                    install_decision_class: self
                        .install_review
                        .decision
                        .install_decision_class_token
                        .clone(),
                },
            );
        }
        // Grant persistence label must agree with the grant scope
        // token (cheap consistency check on the rendered label).
        if !grant_persistence_label_agrees(self.grant_scope, &self.grant_persistence_label) {
            out.push(
                TypedPermissionPromptInvariantViolation::GrantPersistenceInconsistent {
                    grant_scope_class: self.grant_scope.as_str().to_owned(),
                    persistence_label: self.grant_persistence_label.clone(),
                },
            );
        }
        // Decision-state vs invariant consistency. If the caller marks
        // the decision approved while we already have blocking
        // invariants, surface that as a typed failure so the chrome
        // cannot record an approval against a blocked prompt.
        if matches!(self.decision_state, PermissionPromptDecisionState::Approved) && !out.is_empty()
        {
            let blocking_count = out.len() as u32;
            out.push(
                TypedPermissionPromptInvariantViolation::ApprovedWhileBlocked { blocking_count },
            );
        }

        out
    }
}

/// Cheap consistency rule: the rendered persistence label must mention
/// the same token the grant-scope class represents. The check is
/// lenient (case-insensitive substring match against the canonical
/// token) so phrasing variations are still allowed.
fn grant_persistence_label_agrees(scope: GrantScopeClass, label: &str) -> bool {
    let needle = match scope {
        GrantScopeClass::Once => "once",
        GrantScopeClass::Session => "session",
        GrantScopeClass::Workspace => "workspace",
        GrantScopeClass::Profile => "profile",
        GrantScopeClass::PolicyManaged => "policy",
    };
    label.to_ascii_lowercase().contains(needle)
}

impl PermissionPromptScope {
    fn requested_permission_label(&self) -> String {
        if self.requested_permissions.is_empty() {
            return "no additional capabilities".to_owned();
        }
        let mut tokens: Vec<&str> = self
            .requested_permissions
            .iter()
            .map(|row| row.scope_class_token.as_str())
            .collect();
        tokens.sort();
        tokens.dedup();
        tokens.join(", ")
    }
}

#[cfg(test)]
mod tests;
