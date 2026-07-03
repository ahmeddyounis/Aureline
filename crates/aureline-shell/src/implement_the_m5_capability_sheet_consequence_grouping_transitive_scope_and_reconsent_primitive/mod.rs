//! One reusable M5 capability-sheet primitive: consequence-grouped requests,
//! transitive-scope disclosure, reduced-mode choices, and revoke / re-consent
//! parity across every M5 trust lane that asks for meaningful access.
//!
//! Aureline's frozen component matrix
//! ([`crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix`])
//! names the permission / capability sheet as one governed component family and
//! freezes its controlled consequence classes and scope states. This module
//! *implements* that capability-sheet contract as one reusable primitive so actor
//! identity, consequence grouping, transitive scope, reduced-mode choices, and the
//! stable revoke / re-consent paths stay consistent instead of drifting into vague
//! per-feature "grant access?" prompts.
//!
//! The primitive has two halves:
//!
//! 1. A resolver — [`resolve_capability_sheet`] — that takes one actor's requested
//!    capabilities (each carrying a consequence class, purpose, decision posture,
//!    policy pre-decision, transitive origin, and reduced-mode availability) and
//!    produces one [`M5ResolvedCapabilitySheet`] carrying the per-request scope
//!    state, the consequence groups in canonical order, whether the effective
//!    scope is widened by a transitive dependency, whether reduced mode is offered,
//!    whether re-consent is required, and whether remembered grants are revocable
//!    from a stable surface. The resolver never groups by internal API name, never
//!    hides transitive scope, and never lets a policy pre-denied capability be
//!    approved.
//! 2. A parity matrix — [`M5CapabilitySheetPrimitivePacket`] — that binds one row
//!    per claimed M5 trust lane (extension install, AI tool, provider route, remote
//!    connector, automation flow, and privileged helper) to the shared sheet
//!    anatomy, the same consequence classes and scope states, the same consent
//!    disclosures and focus behaviors, and the same export fields, so the support /
//!    export packet reconstructs capability truth from one shared model on every
//!    lane.
//!
//! The consequence classes ([`M5CapabilityConsequenceClass`]), the scope states
//! ([`M5CapabilityScopeState`]), the non-visual accessibility routes
//! ([`M5TrustAccessibilityRoute`]), the qualification classes
//! ([`M5TrustQualificationClass`]), and the downgrade triggers
//! ([`M5TrustComponentDowngradeTrigger`]) are reused verbatim from the frozen
//! component matrix; the shell topology — zones, responsive classes, window
//! classes, and consumer surfaces — is reused from the frozen shell-zone matrix.
//! This module mints new vocabulary only for what the frozen matrix left implicit
//! about the capability sheet itself: its trust-lane families, its anatomy parts,
//! its consent disclosures, its focus behaviors, and its export fields. No M5
//! surface invents a second capability-prompt grammar.
//!
//! Raw URLs, raw local paths, raw usernames, raw hostnames, tokens, credentials,
//! and user text bodies stay outside the support boundary; opaque, export-safe
//! reprs are the only material carried.
//!
//! The boundary schema is
//! [`schemas/ui/m5-capability-sheet.schema.json`](../../../../schemas/ui/m5-capability-sheet.schema.json)
//! and the contract doc is
//! [`docs/components/m5_capability_sheet_primitive_contract.md`](../../../../docs/components/m5_capability_sheet_primitive_contract.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-capability-sheet-primitive/`](../../../../fixtures/ui/m5-capability-sheet-primitive/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_capability_sheet_primitive_automation_flow_beta_narrowed,
    seeded_m5_capability_sheet_primitive_packet,
    seeded_m5_capability_sheet_primitive_privileged_helper_preview_narrowed,
    M5_CAPABILITY_SHEET_PRIMITIVE_PACKET_ID,
};

// The capability consequence classes, scope states, accessibility routes,
// qualification classes, and downgrade triggers are frozen once, in the
// trust-chronology component matrix. This primitive reuses them verbatim so it
// never invents a parallel capability vocabulary.
pub use crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix::{
    M5CapabilityConsequenceClass, M5CapabilityScopeState, M5TrustAccessibilityRoute,
    M5TrustComponentDowngradeTrigger, M5TrustQualificationClass,
};

// The canonical shell topology — zones, responsive classes, window classes, and
// consumer surfaces — is frozen once, in the shell-zone matrix.
pub use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix::{
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellZoneSlot, M5WindowClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5CapabilitySheetPrimitivePacket`].
pub const M5_CAPABILITY_SHEET_PRIMITIVE_RECORD_KIND: &str =
    "implement_m5_capability_sheet_consequence_grouping_transitive_scope_and_reconsent_primitive";

/// Schema version for M5 capability-sheet-primitive records.
pub const M5_CAPABILITY_SHEET_PRIMITIVE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the capability-sheet-primitive boundary schema.
pub const M5_CAPABILITY_SHEET_SCHEMA_REF: &str = "schemas/ui/m5-capability-sheet.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CAPABILITY_SHEET_DOC_REF: &str =
    "docs/components/m5_capability_sheet_primitive_contract.md";

/// Repo-relative path of the frozen shell-zone schema this primitive binds
/// against.
pub const M5_CAPABILITY_SHEET_SHELL_ZONE_REF: &str = "schemas/shell/m5-shell-zone.schema.json";

/// Repo-relative path of the frozen component matrix this primitive narrows from.
pub const M5_CAPABILITY_SHEET_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-trust-chronology-components.schema.json";

/// Repo-relative path of the trust capability-sheet contract this primitive
/// projects from.
pub const M5_CAPABILITY_SHEET_CONTRACT_REF: &str = "schemas/trust/capability_sheet.schema.json";

/// Repo-relative path of the effective-permission record contract this primitive
/// consumes.
pub const M5_CAPABILITY_SHEET_EFFECTIVE_PERMISSION_REF: &str =
    "schemas/extensions/effective_permission.schema.json";

/// Repo-relative path of the permission-prompt event contract this primitive
/// consumes.
pub const M5_CAPABILITY_SHEET_PERMISSION_PROMPT_REF: &str =
    "schemas/policy/permission_prompt_event.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_CAPABILITY_SHEET_FIXTURE_DIR: &str = "fixtures/ui/m5-capability-sheet-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CAPABILITY_SHEET_ARTIFACT_REF: &str =
    "artifacts/release/m5-capability-sheet-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_CAPABILITY_SHEET_CSV_REF: &str =
    "artifacts/release/m5-capability-sheet-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_CAPABILITY_SHEET_REPORT_REF: &str =
    "artifacts/components/m5-capability-sheet-primitive.md";

/// One claimed M5 trust lane that renders the shared capability sheet. These are
/// the actors the goal names — an extension, AI tool, provider route, remote
/// connector, automation flow, or privileged helper — asking for meaningful
/// access.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilitySurfaceFamily {
    /// An extension requesting access at install / enable time.
    ExtensionInstall,
    /// An AI tool requesting access to run.
    AiToolRequest,
    /// A connected provider route requesting access.
    ProviderRoute,
    /// A remote connector requesting access.
    RemoteConnector,
    /// An automation flow requesting access.
    AutomationFlow,
    /// A privileged helper requesting elevated access.
    PrivilegedHelper,
}

impl M5CapabilitySurfaceFamily {
    /// Every claimed trust lane, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExtensionInstall,
        Self::AiToolRequest,
        Self::ProviderRoute,
        Self::RemoteConnector,
        Self::AutomationFlow,
        Self::PrivilegedHelper,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExtensionInstall => "extension_install",
            Self::AiToolRequest => "ai_tool_request",
            Self::ProviderRoute => "provider_route",
            Self::RemoteConnector => "remote_connector",
            Self::AutomationFlow => "automation_flow",
            Self::PrivilegedHelper => "privileged_helper",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::ExtensionInstall => "Extension Install",
            Self::AiToolRequest => "AI Tool Request",
            Self::ProviderRoute => "Provider Route",
            Self::RemoteConnector => "Remote Connector",
            Self::AutomationFlow => "Automation Flow",
            Self::PrivilegedHelper => "Privileged Helper",
        }
    }
}

/// One anatomy part the shared capability sheet surfaces. The first eight in
/// [`M5CapabilitySheetAnatomyPart::MANDATORY`] are required on every sheet; the
/// last is the conditional transitive-scope disclosure that appears whenever a
/// dependency widens effective scope.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilitySheetAnatomyPart {
    /// The requesting actor's identity.
    ActorIdentity,
    /// Plain-language purpose text for the request.
    PurposeText,
    /// The requested capabilities grouped by consequence / risk class.
    ConsequenceGroupedRequests,
    /// The scope choice (full versus a narrower grant).
    ScopeChoice,
    /// The reduced-mode option to grant less than requested.
    ReducedModeOption,
    /// The approve action.
    ApproveAction,
    /// The deny action.
    DenyAction,
    /// The detail action escalating to the full disclosure.
    DetailAction,
    /// The transitive-scope disclosure shown when a dependency widens scope.
    TransitiveScopeDisclosure,
}

impl M5CapabilitySheetAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ActorIdentity,
        Self::PurposeText,
        Self::ConsequenceGroupedRequests,
        Self::ScopeChoice,
        Self::ReducedModeOption,
        Self::ApproveAction,
        Self::DenyAction,
        Self::DetailAction,
        Self::TransitiveScopeDisclosure,
    ];

    /// The anatomy parts every capability sheet must render.
    pub const MANDATORY: [Self; 8] = [
        Self::ActorIdentity,
        Self::PurposeText,
        Self::ConsequenceGroupedRequests,
        Self::ScopeChoice,
        Self::ReducedModeOption,
        Self::ApproveAction,
        Self::DenyAction,
        Self::DetailAction,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActorIdentity => "actor_identity",
            Self::PurposeText => "purpose_text",
            Self::ConsequenceGroupedRequests => "consequence_grouped_requests",
            Self::ScopeChoice => "scope_choice",
            Self::ReducedModeOption => "reduced_mode_option",
            Self::ApproveAction => "approve_action",
            Self::DenyAction => "deny_action",
            Self::DetailAction => "detail_action",
            Self::TransitiveScopeDisclosure => "transitive_scope_disclosure",
        }
    }
}

/// How a remembered grant, a reduced scope, a transitive origin, and a re-consent
/// are disclosed. The first three in [`M5CapabilityConsentDisclosure::MANDATORY`]
/// are required so a remembered approval always shows a stable revoke path and
/// scope never widens silently.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityConsentDisclosure {
    /// A previously remembered grant is shown.
    RememberedGrantShown,
    /// The stable revoke path in the trust / settings surface is shown.
    RevokePathShown,
    /// Scope never widens without an explicit new consent.
    NoSilentScopeWidening,
    /// The reason re-consent is required is explained.
    ReConsentReasonExplained,
    /// The narrowing of a reduced-scope grant is disclosed.
    ReducedScopeDisclosed,
    /// The dependency that widened scope transitively is named.
    TransitiveOriginShown,
}

impl M5CapabilityConsentDisclosure {
    /// Every consent disclosure, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RememberedGrantShown,
        Self::RevokePathShown,
        Self::NoSilentScopeWidening,
        Self::ReConsentReasonExplained,
        Self::ReducedScopeDisclosed,
        Self::TransitiveOriginShown,
    ];

    /// The consent disclosures every capability sheet must offer.
    pub const MANDATORY: [Self; 3] = [
        Self::RememberedGrantShown,
        Self::RevokePathShown,
        Self::NoSilentScopeWidening,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RememberedGrantShown => "remembered_grant_shown",
            Self::RevokePathShown => "revoke_path_shown",
            Self::NoSilentScopeWidening => "no_silent_scope_widening",
            Self::ReConsentReasonExplained => "re_consent_reason_explained",
            Self::ReducedScopeDisclosed => "reduced_scope_disclosed",
            Self::TransitiveOriginShown => "transitive_origin_shown",
        }
    }
}

/// A focus / navigation behavior the capability sheet supports so approval stays
/// deliberate, per-group navigation is reachable, and the revoke path is
/// deep-linkable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilitySheetFocusBehavior {
    /// Approve requires an explicit deliberate focus; it is never default-focused.
    ApproveRequiresExplicitFocus,
    /// Detail escalates to a side sheet when the inline disclosure is insufficient.
    DetailSideSheetEscalation,
    /// Focus returns to the sheet after a side sheet closes.
    ReturnFocusOnClose,
    /// Keyboard navigation moves per consequence group.
    PerConsequenceGroupNavigation,
    /// The reduced-mode toggle is keyboard-reachable.
    ReducedModeToggleReachable,
    /// A stable deep-link anchor jumps to the revoke surface.
    DeepLinkToRevoke,
}

impl M5CapabilitySheetFocusBehavior {
    /// Every focus behavior, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ApproveRequiresExplicitFocus,
        Self::DetailSideSheetEscalation,
        Self::ReturnFocusOnClose,
        Self::PerConsequenceGroupNavigation,
        Self::ReducedModeToggleReachable,
        Self::DeepLinkToRevoke,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ApproveRequiresExplicitFocus => "approve_requires_explicit_focus",
            Self::DetailSideSheetEscalation => "detail_side_sheet_escalation",
            Self::ReturnFocusOnClose => "return_focus_on_close",
            Self::PerConsequenceGroupNavigation => "per_consequence_group_navigation",
            Self::ReducedModeToggleReachable => "reduced_mode_toggle_reachable",
            Self::DeepLinkToRevoke => "deep_link_to_revoke",
        }
    }
}

/// A field the support / export packet carries so capability truth is
/// reconstructable from the shared sheet model with the same vocabulary. The first
/// four in [`M5CapabilitySheetExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilitySheetExportField {
    /// The opaque actor identity representation.
    ActorIdentityRepr,
    /// The consequence class of each request.
    ConsequenceClass,
    /// The opaque capability token of each request.
    CapabilityToken,
    /// The resolved scope state of each request.
    ScopeState,
    /// The opaque transitive origin representation, when scope widened.
    TransitiveOriginRepr,
    /// Whether reduced mode was offered for each request.
    ReducedModeOffered,
    /// Whether the grant is revocable from a stable surface.
    RevocableFromSettings,
}

impl M5CapabilitySheetExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::ActorIdentityRepr,
        Self::ConsequenceClass,
        Self::CapabilityToken,
        Self::ScopeState,
        Self::TransitiveOriginRepr,
        Self::ReducedModeOffered,
        Self::RevocableFromSettings,
    ];

    /// The export fields every capability-sheet export must carry.
    pub const MANDATORY: [Self; 4] = [
        Self::ActorIdentityRepr,
        Self::ConsequenceClass,
        Self::CapabilityToken,
        Self::ScopeState,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActorIdentityRepr => "actor_identity_repr",
            Self::ConsequenceClass => "consequence_class",
            Self::CapabilityToken => "capability_token",
            Self::ScopeState => "scope_state",
            Self::TransitiveOriginRepr => "transitive_origin_repr",
            Self::ReducedModeOffered => "reduced_mode_offered",
            Self::RevocableFromSettings => "revocable_from_settings",
        }
    }
}

/// The decision posture a user (or a remembered choice) applies to one requested
/// capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityDecision {
    /// Requested but not yet granted.
    RequestedNotGranted,
    /// Approved at the full requested scope.
    ApproveFull,
    /// Approved at a narrower, reduced scope.
    ApproveReduced,
    /// Revoked a previously granted scope.
    Revoke,
}

impl M5CapabilityDecision {
    /// Every decision, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::RequestedNotGranted,
        Self::ApproveFull,
        Self::ApproveReduced,
        Self::Revoke,
    ];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestedNotGranted => "requested_not_granted",
            Self::ApproveFull => "approve_full",
            Self::ApproveReduced => "approve_reduced",
            Self::Revoke => "revoke",
        }
    }

    /// True when the decision grants access at some scope.
    const fn grants(self) -> bool {
        matches!(self, Self::ApproveFull | Self::ApproveReduced)
    }
}

/// A policy pre-decision that must be preserved: an administrator may pre-approve
/// or pre-deny a capability before the user ever sees the sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CapabilityPolicyPredecision {
    /// No policy pre-decision applies.
    NoPolicy,
    /// Policy pre-approves this capability.
    PreApproved,
    /// Policy pre-denies this capability; it can never be approved locally.
    PreDenied,
}

impl M5CapabilityPolicyPredecision {
    /// Every pre-decision, in declaration order.
    pub const ALL: [Self; 3] = [Self::NoPolicy, Self::PreApproved, Self::PreDenied];

    /// Stable token recorded in worked cases.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoPolicy => "no_policy",
            Self::PreApproved => "pre_approved",
            Self::PreDenied => "pre_denied",
        }
    }
}

/// One requested capability, before resolution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityRequestItem {
    /// Opaque, export-safe capability token (never a raw endpoint or path).
    pub capability_token: String,
    /// The real-world consequence class this capability carries.
    pub consequence_class: M5CapabilityConsequenceClass,
    /// Plain-language, export-safe purpose text.
    pub purpose_repr: String,
    /// The decision posture applied to this capability.
    pub decision: M5CapabilityDecision,
    /// The policy pre-decision that applies to this capability.
    pub policy_predecision: M5CapabilityPolicyPredecision,
    /// True when this capability entered scope transitively via a dependency.
    pub is_transitive: bool,
    /// Opaque, export-safe origin of a transitive widening. Required when
    /// `is_transitive` is true.
    pub transitive_origin_repr: Option<String>,
    /// True when a reduced-scope grant is available for this capability.
    pub reduced_mode_available: bool,
    /// True when a previously granted scope now requires re-consent (e.g. the
    /// actor updated or the requested scope widened).
    pub re_consent_triggered: bool,
    /// True when a remembered grant already exists for this capability.
    pub has_prior_grant: bool,
}

impl M5CapabilityRequestItem {
    /// True when the purpose, token, or transitive origin carries forbidden
    /// material.
    fn carries_forbidden_material(&self) -> bool {
        repr_is_forbidden(&self.capability_token)
            || repr_is_forbidden(&self.purpose_repr)
            || self
                .transitive_origin_repr
                .as_deref()
                .is_some_and(repr_is_forbidden)
    }
}

/// The full input to the capability-sheet resolver for one actor's request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilitySheetResolutionInput {
    /// The trust lane this sheet renders on.
    pub surface_family: M5CapabilitySurfaceFamily,
    /// Opaque, export-safe actor identity (never a raw username or path).
    pub actor_identity_repr: String,
    /// The requested capabilities. Must be non-empty with unique tokens.
    pub requests: Vec<M5CapabilityRequestItem>,
}

/// The resolved posture of one requested capability.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCapabilityRequest {
    /// The opaque capability token.
    pub capability_token: String,
    /// The consequence class this capability carries.
    pub consequence_class: M5CapabilityConsequenceClass,
    /// The resolved scope state.
    pub scope_state: M5CapabilityScopeState,
    /// The policy pre-decision that applies.
    pub policy_predecision: M5CapabilityPolicyPredecision,
    /// True when this capability entered scope transitively.
    pub is_transitive: bool,
    /// The opaque origin of the transitive widening, when transitive.
    pub transitive_origin_repr: Option<String>,
    /// True when a reduced-scope grant is offered.
    pub reduced_mode_offered: bool,
    /// True when the resulting grant is revocable from a stable surface.
    pub revocable: bool,
}

/// One consequence group: every requested capability that carries the same
/// real-world consequence, so the sheet groups by consequence rather than by an
/// arbitrary permission list.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilityConsequenceGroup {
    /// The consequence class this group carries.
    pub consequence_class: M5CapabilityConsequenceClass,
    /// The opaque capability tokens in this group, in request order.
    pub capability_tokens: Vec<String>,
}

/// The resolved capability truth for one sheet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCapabilitySheet {
    /// The trust lane this sheet renders on.
    pub surface_family: M5CapabilitySurfaceFamily,
    /// The opaque actor identity representation.
    pub actor_identity_repr: String,
    /// The resolved requests, in request order.
    pub resolved_requests: Vec<M5ResolvedCapabilityRequest>,
    /// The requested capabilities grouped by consequence, in canonical
    /// consequence-class order.
    pub consequence_groups: Vec<M5CapabilityConsequenceGroup>,
    /// True when a transitive dependency widens the effective scope.
    pub widens_effective_scope: bool,
    /// True when at least one request offers a reduced-mode grant.
    pub reduced_mode_offered: bool,
    /// True when at least one request requires re-consent.
    pub requires_re_consent: bool,
    /// True when at least one remembered grant is revocable from a stable surface.
    pub revocable_from_settings: bool,
}

/// Errors returned by [`resolve_capability_sheet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5CapabilityResolutionError {
    /// The actor identity was empty.
    EmptyActorIdentity,
    /// The input carried no requested capabilities.
    NoRequests,
    /// A requested capability had an empty token.
    EmptyCapabilityToken,
    /// A requested capability had an empty purpose.
    EmptyPurpose,
    /// The same capability token appeared more than once.
    DuplicateCapability(String),
    /// A transitive request did not name its origin.
    MissingTransitiveOrigin(String),
    /// A reduced-scope grant was chosen where reduced mode is unavailable.
    ReducedSelectedButUnavailable(String),
    /// A policy pre-denied capability was approved locally.
    PolicyDeniedCapabilityApproved(String),
    /// A representation carried forbidden material.
    ForbiddenMaterial,
}

impl M5CapabilityResolutionError {
    /// Stable token for tests and diagnostics.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyActorIdentity => "empty_actor_identity",
            Self::NoRequests => "no_requests",
            Self::EmptyCapabilityToken => "empty_capability_token",
            Self::EmptyPurpose => "empty_purpose",
            Self::DuplicateCapability(_) => "duplicate_capability",
            Self::MissingTransitiveOrigin(_) => "missing_transitive_origin",
            Self::ReducedSelectedButUnavailable(_) => "reduced_selected_but_unavailable",
            Self::PolicyDeniedCapabilityApproved(_) => "policy_denied_capability_approved",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5CapabilityResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "capability-sheet resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5CapabilityResolutionError {}

/// Resolves one capability sheet from an actor's requested capabilities.
///
/// Each request resolves to exactly one [`M5CapabilityScopeState`]. A revoke wins
/// over everything (the grant is gone but kept in history); a triggered re-consent
/// wins over a standing grant; a reduced approval reads as a reduced-scope grant; a
/// full approval reads as a full-scope grant; a transitive-but-ungranted request
/// reads as transitive-scope-disclosed; otherwise the request is
/// requested-but-not-granted. Requests are then grouped by consequence class in
/// canonical order so the sheet never groups by an internal API name.
pub fn resolve_capability_sheet(
    input: &M5CapabilitySheetResolutionInput,
) -> Result<M5ResolvedCapabilitySheet, M5CapabilityResolutionError> {
    if input.actor_identity_repr.trim().is_empty() {
        return Err(M5CapabilityResolutionError::EmptyActorIdentity);
    }
    if repr_is_forbidden(&input.actor_identity_repr) {
        return Err(M5CapabilityResolutionError::ForbiddenMaterial);
    }
    if input.requests.is_empty() {
        return Err(M5CapabilityResolutionError::NoRequests);
    }

    let mut seen: BTreeSet<&str> = BTreeSet::new();
    for request in &input.requests {
        if request.capability_token.trim().is_empty() {
            return Err(M5CapabilityResolutionError::EmptyCapabilityToken);
        }
        if request.purpose_repr.trim().is_empty() {
            return Err(M5CapabilityResolutionError::EmptyPurpose);
        }
        if !seen.insert(request.capability_token.as_str()) {
            return Err(M5CapabilityResolutionError::DuplicateCapability(
                request.capability_token.clone(),
            ));
        }
        if request.carries_forbidden_material() {
            return Err(M5CapabilityResolutionError::ForbiddenMaterial);
        }
        if request.is_transitive
            && request
                .transitive_origin_repr
                .as_deref()
                .map(str::trim)
                .map(str::is_empty)
                .unwrap_or(true)
        {
            return Err(M5CapabilityResolutionError::MissingTransitiveOrigin(
                request.capability_token.clone(),
            ));
        }
        if request.decision == M5CapabilityDecision::ApproveReduced
            && !request.reduced_mode_available
        {
            return Err(M5CapabilityResolutionError::ReducedSelectedButUnavailable(
                request.capability_token.clone(),
            ));
        }
        if request.policy_predecision == M5CapabilityPolicyPredecision::PreDenied
            && request.decision.grants()
        {
            return Err(M5CapabilityResolutionError::PolicyDeniedCapabilityApproved(
                request.capability_token.clone(),
            ));
        }
    }

    let resolved_requests: Vec<M5ResolvedCapabilityRequest> =
        input.requests.iter().map(resolve_request).collect();

    let consequence_groups = group_by_consequence(&input.requests);

    let widens_effective_scope = resolved_requests
        .iter()
        .any(|request| request.is_transitive);
    let reduced_mode_offered = resolved_requests
        .iter()
        .any(|request| request.reduced_mode_offered);
    let requires_re_consent = resolved_requests
        .iter()
        .any(|request| request.scope_state == M5CapabilityScopeState::ReConsentRequired);
    let revocable_from_settings = resolved_requests.iter().any(|request| request.revocable);

    Ok(M5ResolvedCapabilitySheet {
        surface_family: input.surface_family,
        actor_identity_repr: input.actor_identity_repr.clone(),
        resolved_requests,
        consequence_groups,
        widens_effective_scope,
        reduced_mode_offered,
        requires_re_consent,
        revocable_from_settings,
    })
}

/// Resolves one request's scope state and revocability.
fn resolve_request(request: &M5CapabilityRequestItem) -> M5ResolvedCapabilityRequest {
    let scope_state = if request.decision == M5CapabilityDecision::Revoke {
        M5CapabilityScopeState::RevokedWithHistory
    } else if request.re_consent_triggered && request.has_prior_grant {
        M5CapabilityScopeState::ReConsentRequired
    } else if request.decision == M5CapabilityDecision::ApproveReduced {
        M5CapabilityScopeState::GrantedReducedScope
    } else if request.decision == M5CapabilityDecision::ApproveFull {
        M5CapabilityScopeState::GrantedFullScope
    } else if request.is_transitive {
        M5CapabilityScopeState::TransitiveScopeDisclosed
    } else {
        M5CapabilityScopeState::RequestedNotGranted
    };

    // A remembered grant is revocable from the stable trust surface whenever a
    // scope is currently held; a not-yet-granted, transitive-disclosed, or
    // already-revoked request has nothing to revoke.
    let revocable = matches!(
        scope_state,
        M5CapabilityScopeState::GrantedFullScope
            | M5CapabilityScopeState::GrantedReducedScope
            | M5CapabilityScopeState::ReConsentRequired
    );

    M5ResolvedCapabilityRequest {
        capability_token: request.capability_token.clone(),
        consequence_class: request.consequence_class,
        scope_state,
        policy_predecision: request.policy_predecision,
        is_transitive: request.is_transitive,
        transitive_origin_repr: request.transitive_origin_repr.clone(),
        reduced_mode_offered: request.reduced_mode_available,
        revocable,
    }
}

/// Groups requests by consequence class in canonical order; a group is emitted
/// only when at least one request carries that consequence class.
fn group_by_consequence(requests: &[M5CapabilityRequestItem]) -> Vec<M5CapabilityConsequenceGroup> {
    let mut groups = Vec::new();
    for class in M5CapabilityConsequenceClass::ALL {
        let tokens: Vec<String> = requests
            .iter()
            .filter(|request| request.consequence_class == class)
            .map(|request| request.capability_token.clone())
            .collect();
        if !tokens.is_empty() {
            groups.push(M5CapabilityConsequenceGroup {
                consequence_class: class,
                capability_tokens: tokens,
            });
        }
    }
    groups
}

/// One worked resolution case carried in the packet so the support / export packet
/// reconstructs capability truth from the shared model.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilitySheetResolutionCase {
    /// The resolver input.
    pub input: M5CapabilitySheetResolutionInput,
    /// The resolved capability truth. Must equal
    /// `resolve_capability_sheet(&input)`.
    pub resolved: M5ResolvedCapabilitySheet,
}

impl M5CapabilitySheetResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5CapabilitySheetResolutionInput) -> Self {
        let resolved = resolve_capability_sheet(&input).expect("seed resolution case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_capability_sheet(&self.input).as_ref() == Ok(&self.resolved)
    }
}

/// One row in the primitive matrix: one trust lane bound to the shared
/// capability-sheet anatomy, consequence classes, scope states, consent
/// disclosures, focus behaviors, and export fields.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilitySheetSurfaceRow {
    /// Trust-lane family.
    pub surface_family: M5CapabilitySurfaceFamily,
    /// Qualification class earned by this lane.
    pub qualification: M5TrustQualificationClass,
    /// Owner role accountable for keeping this lane governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical shell zone this capability sheet attaches to.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this sheet must survive.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this sheet keeps continuity across.
    pub window_classes: Vec<M5WindowClass>,
    /// Anatomy parts this sheet renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5CapabilitySheetAnatomyPart>,
    /// Consequence classes this sheet groups by.
    pub consequence_classes: Vec<M5CapabilityConsequenceClass>,
    /// Scope states this sheet projects.
    pub scope_states: Vec<M5CapabilityScopeState>,
    /// Consent disclosures this sheet offers (must include the mandatory ones).
    pub consent_disclosures: Vec<M5CapabilityConsentDisclosure>,
    /// Focus behaviors this sheet supports.
    pub focus_behaviors: Vec<M5CapabilitySheetFocusBehavior>,
    /// Export fields this sheet carries (must include the mandatory fields).
    pub export_fields: Vec<M5CapabilitySheetExportField>,
    /// Non-visual accessibility routes this sheet offers.
    pub accessibility_routes: Vec<M5TrustAccessibilityRoute>,
    /// Shell subsystems that consume this sheet's projection.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this sheet.
    pub downgrade_triggers: Vec<M5TrustComponentDowngradeTrigger>,
    /// Proof packet refs that keep this lane current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this lane.
    pub source_contract_refs: Vec<String>,
    /// Worked resolution cases proving the resolver on this lane.
    pub example_sheets: Vec<M5CapabilitySheetResolutionCase>,
    /// Hard invariant: this sheet never drops consequence grouping (never uses a
    /// vague generic access prompt). MUST be `false`.
    pub drops_consequence_grouping: bool,
    /// Hard invariant: this sheet never hides transitive scope. MUST be `false`.
    pub hides_transitive_scope: bool,
    /// Hard invariant: this sheet never skips required re-consent. MUST be `false`.
    pub skips_required_re_consent: bool,
    /// Hard invariant: this sheet never drops export / audit truth. MUST be
    /// `false`.
    pub drops_export_or_audit_truth: bool,
}

impl M5CapabilitySheetSurfaceRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5CapabilitySheetAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5CapabilitySheetAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory consent disclosure.
    fn declares_mandatory_consent_disclosures(&self) -> bool {
        let present: BTreeSet<M5CapabilityConsentDisclosure> =
            self.consent_disclosures.iter().copied().collect();
        M5CapabilityConsentDisclosure::MANDATORY
            .iter()
            .all(|disclosure| present.contains(disclosure))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5CapabilitySheetExportField> =
            self.export_fields.iter().copied().collect();
        M5CapabilitySheetExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.drops_consequence_grouping
            && !self.hides_transitive_scope
            && !self.skips_required_re_consent
            && !self.drops_export_or_audit_truth
    }
}

/// Self-describing controlled-vocabulary set minted by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilitySheetVocabularySet {
    /// Trust-lane-family tokens.
    pub surface_families: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Consequence-class tokens (reused from the frozen matrix).
    pub consequence_classes: Vec<String>,
    /// Scope-state tokens (reused from the frozen matrix).
    pub scope_states: Vec<String>,
    /// Consent-disclosure tokens.
    pub consent_disclosures: Vec<String>,
    /// Focus-behavior tokens.
    pub focus_behaviors: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
}

impl M5CapabilitySheetVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5CapabilitySurfaceFamily::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5CapabilitySheetAnatomyPart::ALL, |v| v.as_str()),
            consequence_classes: tokens(&M5CapabilityConsequenceClass::ALL, |v| v.as_str()),
            scope_states: tokens(&M5CapabilityScopeState::ALL, |v| v.as_str()),
            consent_disclosures: tokens(&M5CapabilityConsentDisclosure::ALL, |v| v.as_str()),
            focus_behaviors: tokens(&M5CapabilitySheetFocusBehavior::ALL, |v| v.as_str()),
            export_fields: tokens(&M5CapabilitySheetExportField::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TrustAccessibilityRoute::ALL, |v| v.as_str()),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

fn tokens<T: Copy>(items: &[T], to_token: impl Fn(T) -> &'static str) -> Vec<String> {
    items.iter().map(|v| to_token(*v).to_owned()).collect()
}

/// Governance-review block; every flag is a hard invariant.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilitySheetGovernanceReview {
    /// One capability sheet groups requests by consequence on every lane.
    pub one_sheet_groups_by_consequence: bool,
    /// Transitive / downstream scope is always disclosed before approval.
    pub transitive_scope_always_disclosed: bool,
    /// Reduced-mode behavior is visible before approval.
    pub reduced_mode_visible_before_approval: bool,
    /// Policy pre-approve / pre-deny and re-consent triggers are preserved.
    pub policy_and_re_consent_preserved: bool,
    /// Remembered approvals are revocable from a stable trust / settings surface.
    pub remembered_approvals_revocable_from_stable_surface: bool,
    /// The support / export packet keeps the same capability vocabulary.
    pub support_export_keeps_capability_vocabulary: bool,
    /// No lane uses a vague generic access prompt.
    pub no_surface_uses_generic_access_prompt: bool,
    /// Every sheet is bound to a canonical shell zone.
    pub every_sheet_bound_to_shell_zone: bool,
    /// Every sheet declares a non-visual accessibility route.
    pub every_sheet_declares_accessibility_route: bool,
    /// Later M5 sheets cannot invent parallel capability vocabulary.
    pub later_sheets_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilitySheetConsumerProjection {
    /// Extension / AI / provider / remote / automation / helper lanes all consume
    /// the shared sheet.
    pub trust_lanes_consume_shared_sheet: bool,
    /// The scope resolver reads a single canonical scope ladder.
    pub resolver_reads_single_scope_ladder: bool,
    /// The revoke path reads a single canonical source.
    pub revoke_path_reads_single_source: bool,
    /// Transitive-scope disclosure reads a single canonical source.
    pub transitive_disclosure_reads_single_source: bool,
    /// Support / export reads a single canonical capability-sheet source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilitySheetProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the capability-sheet primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilitySheetReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting capability-sheet audit.
    pub capability_sheet_audit_ref: String,
    /// True when support / export parity is required for every lane.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every lane.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5CapabilitySheetPrimitivePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CapabilitySheetPrimitivePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5CapabilitySheetSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CapabilitySheetVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CapabilitySheetGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CapabilitySheetConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CapabilitySheetProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CapabilitySheetReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 capability-sheet-primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CapabilitySheetPrimitivePacket {
    /// Record kind; must equal [`M5_CAPABILITY_SHEET_PRIMITIVE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CAPABILITY_SHEET_PRIMITIVE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5CapabilitySheetSurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5CapabilitySheetVocabularySet,
    /// Governance-review block.
    pub governance_review: M5CapabilitySheetGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5CapabilitySheetConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5CapabilitySheetProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5CapabilitySheetReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5CapabilitySheetPrimitivePacket {
    /// Builds an M5 capability-sheet-primitive packet from stable-lane input.
    pub fn new(input: M5CapabilitySheetPrimitivePacketInput) -> Self {
        Self {
            record_kind: M5_CAPABILITY_SHEET_PRIMITIVE_RECORD_KIND.to_owned(),
            schema_version: M5_CAPABILITY_SHEET_PRIMITIVE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
            vocabulary_set: input.vocabulary_set,
            governance_review: input.governance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the M5 capability-sheet-primitive invariants.
    pub fn validate(&self) -> Vec<M5CapabilitySheetPrimitiveViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CAPABILITY_SHEET_PRIMITIVE_RECORD_KIND {
            violations.push(M5CapabilitySheetPrimitiveViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CAPABILITY_SHEET_PRIMITIVE_SCHEMA_VERSION {
            violations.push(M5CapabilitySheetPrimitiveViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5CapabilitySheetPrimitiveViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_transitive_disclosure_covered(self, &mut violations);
        validate_reduced_mode_covered(self, &mut violations);
        validate_revocable_grant_covered(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 capability-sheet primitive packet serializes"),
        ) {
            violations.push(M5CapabilitySheetPrimitiveViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 capability-sheet primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per trust lane.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,qualification,owner,shell_zone_slot,anatomy_parts,consequence_classes,scope_states,consent_disclosures,export_fields,example_count\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shell_zone_slot.as_str(),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_tokens(&row.consequence_classes, |v| v.as_str()),
                join_tokens(&row.scope_states, |v| v.as_str()),
                join_tokens(&row.consent_disclosures, |v| v.as_str()),
                join_tokens(&row.export_fields, |v| v.as_str()),
                row.example_sheets.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .surface_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Capability-Sheet Primitive: Consequence Grouping, Transitive Scope, and Re-consent\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Trust lanes: {} ({} stable)\n",
            self.surface_rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Anatomy parts: {}\n",
            self.vocabulary_set.anatomy_parts.join(", ")
        ));
        out.push_str(&format!(
            "- Consequence classes: {}\n",
            self.vocabulary_set.consequence_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Scope states: {}\n",
            self.vocabulary_set.scope_states.join(", ")
        ));
        out.push_str(&format!(
            "- Export fields: {}\n",
            self.vocabulary_set.export_fields.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Trust lanes\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.surface_family.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Shell zone: `{}`\n",
                row.shell_zone_slot.as_str()
            ));
            out.push_str(&format!(
                "  - Worked sheets: {}\n",
                row.example_sheets.len()
            ));
            for case in &row.example_sheets {
                out.push_str(&format!(
                    "    - `{}` — {} request(s), {} consequence group(s){}\n",
                    case.resolved.actor_identity_repr,
                    case.resolved.resolved_requests.len(),
                    case.resolved.consequence_groups.len(),
                    if case.resolved.widens_effective_scope {
                        ", widens scope transitively"
                    } else {
                        ""
                    }
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 capability-sheet-primitive
/// export.
#[derive(Debug)]
pub enum M5CapabilitySheetPrimitiveArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5CapabilitySheetPrimitiveViolation>),
}

impl fmt::Display for M5CapabilitySheetPrimitiveArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 capability-sheet primitive export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 capability-sheet primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5CapabilitySheetPrimitiveArtifactError {}

/// Validation failures emitted by [`M5CapabilitySheetPrimitivePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5CapabilitySheetPrimitiveViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required trust-lane family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A surface row declares no consequence classes.
    ConsequenceClassMissing,
    /// A surface row declares no scope states.
    ScopeStateMissing,
    /// A surface row omits one of the mandatory consent disclosures.
    MandatoryConsentDisclosureMissing,
    /// A surface row declares no focus behaviors.
    FocusBehaviorMissing,
    /// A surface row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A surface row declares no accessibility routes (or misses keyboard focus).
    AccessibilityRouteMissing,
    /// A surface row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface row declares no worked resolution cases.
    ExampleSheetMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleSheetDrift,
    /// A lane claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// No worked resolution across the matrix proves transitive scope disclosed
    /// before approval.
    TransitiveDisclosureUnproven,
    /// No worked resolution across the matrix proves a reduced-scope grant.
    ReducedModeUnproven,
    /// No worked resolution across the matrix proves a remembered grant revocable
    /// from a stable surface.
    RevocableGrantUnproven,
    /// A surface row violates a hard invariant.
    SurfaceInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5CapabilitySheetPrimitiveViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::ConsequenceClassMissing => "consequence_class_missing",
            Self::ScopeStateMissing => "scope_state_missing",
            Self::MandatoryConsentDisclosureMissing => "mandatory_consent_disclosure_missing",
            Self::FocusBehaviorMissing => "focus_behavior_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ExampleSheetMissing => "example_sheet_missing",
            Self::ExampleSheetDrift => "example_sheet_drift",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::TransitiveDisclosureUnproven => "transitive_disclosure_unproven",
            Self::ReducedModeUnproven => "reduced_mode_unproven",
            Self::RevocableGrantUnproven => "revocable_grant_unproven",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 capability-sheet-primitive export.
pub fn current_stable_m5_capability_sheet_primitive_export(
) -> Result<M5CapabilitySheetPrimitivePacket, M5CapabilitySheetPrimitiveArtifactError> {
    let packet: M5CapabilitySheetPrimitivePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-capability-sheet-proof/support_export.json"
    )))
    .map_err(M5CapabilitySheetPrimitiveArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5CapabilitySheetPrimitiveArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5CapabilitySheetPrimitivePacket,
    violations: &mut Vec<M5CapabilitySheetPrimitiveViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CAPABILITY_SHEET_SCHEMA_REF,
        M5_CAPABILITY_SHEET_DOC_REF,
        M5_CAPABILITY_SHEET_SHELL_ZONE_REF,
        M5_CAPABILITY_SHEET_COMPONENT_MATRIX_REF,
        M5_CAPABILITY_SHEET_CONTRACT_REF,
        M5_CAPABILITY_SHEET_EFFECTIVE_PERMISSION_REF,
        M5_CAPABILITY_SHEET_PERMISSION_PROMPT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5CapabilitySheetPrimitiveViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5CapabilitySheetPrimitivePacket,
    violations: &mut Vec<M5CapabilitySheetPrimitiveViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5CapabilitySheetPrimitiveViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5CapabilitySheetPrimitivePacket,
    violations: &mut Vec<M5CapabilitySheetPrimitiveViolation>,
) {
    let present: BTreeSet<M5CapabilitySurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5CapabilitySurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5CapabilitySheetPrimitiveViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
        {
            violations.push(M5CapabilitySheetPrimitiveViolation::SurfaceRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5CapabilitySheetPrimitiveViolation::MandatoryAnatomyMissing);
        }
        if row.consequence_classes.is_empty() {
            violations.push(M5CapabilitySheetPrimitiveViolation::ConsequenceClassMissing);
        }
        if row.scope_states.is_empty() {
            violations.push(M5CapabilitySheetPrimitiveViolation::ScopeStateMissing);
        }
        if !row.declares_mandatory_consent_disclosures() {
            violations.push(M5CapabilitySheetPrimitiveViolation::MandatoryConsentDisclosureMissing);
        }
        if row.focus_behaviors.is_empty() {
            violations.push(M5CapabilitySheetPrimitiveViolation::FocusBehaviorMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5CapabilitySheetPrimitiveViolation::MandatoryExportFieldMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5TrustAccessibilityRoute::KeyboardFocusable)
        {
            violations.push(M5CapabilitySheetPrimitiveViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5CapabilitySheetPrimitiveViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5CapabilitySheetPrimitiveViolation::DowngradeTriggersMissing);
        }
        if row.example_sheets.is_empty() {
            violations.push(M5CapabilitySheetPrimitiveViolation::ExampleSheetMissing);
        }
        if row
            .example_sheets
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5CapabilitySheetPrimitiveViolation::ExampleSheetDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5CapabilitySheetPrimitiveViolation::StableSurfaceMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5CapabilitySheetPrimitiveViolation::SurfaceInvariantViolated);
        }
    }
}

/// At least one worked resolution must prove a `TransitiveScopeDisclosed` request
/// that names its transitive origin — the acceptance-criterion example that
/// transitive scope is visible before approval.
fn validate_transitive_disclosure_covered(
    packet: &M5CapabilitySheetPrimitivePacket,
    violations: &mut Vec<M5CapabilitySheetPrimitiveViolation>,
) {
    let proven = packet.surface_rows.iter().any(|row| {
        row.example_sheets.iter().any(|case| {
            case.resolved.widens_effective_scope
                && case.resolved.resolved_requests.iter().any(|request| {
                    request.scope_state == M5CapabilityScopeState::TransitiveScopeDisclosed
                        && request.transitive_origin_repr.is_some()
                })
        })
    });
    if !proven {
        violations.push(M5CapabilitySheetPrimitiveViolation::TransitiveDisclosureUnproven);
    }
}

/// At least one worked resolution must prove a `GrantedReducedScope` request — the
/// acceptance-criterion example that reduced-mode behavior is visible before
/// approval.
fn validate_reduced_mode_covered(
    packet: &M5CapabilitySheetPrimitivePacket,
    violations: &mut Vec<M5CapabilitySheetPrimitiveViolation>,
) {
    let proven = packet.surface_rows.iter().any(|row| {
        row.example_sheets.iter().any(|case| {
            case.resolved.resolved_requests.iter().any(|request| {
                request.scope_state == M5CapabilityScopeState::GrantedReducedScope
                    && request.reduced_mode_offered
            })
        })
    });
    if !proven {
        violations.push(M5CapabilitySheetPrimitiveViolation::ReducedModeUnproven);
    }
}

/// At least one worked resolution must prove a remembered grant that is revocable
/// from a stable surface — the acceptance-criterion example that remembered
/// approvals stay revocable.
fn validate_revocable_grant_covered(
    packet: &M5CapabilitySheetPrimitivePacket,
    violations: &mut Vec<M5CapabilitySheetPrimitiveViolation>,
) {
    let proven = packet.surface_rows.iter().any(|row| {
        row.example_sheets.iter().any(|case| {
            case.resolved.revocable_from_settings
                && case
                    .resolved
                    .resolved_requests
                    .iter()
                    .any(|request| request.revocable)
        })
    });
    if !proven {
        violations.push(M5CapabilitySheetPrimitiveViolation::RevocableGrantUnproven);
    }
}

fn validate_governance_review(
    packet: &M5CapabilitySheetPrimitivePacket,
    violations: &mut Vec<M5CapabilitySheetPrimitiveViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.one_sheet_groups_by_consequence,
        review.transitive_scope_always_disclosed,
        review.reduced_mode_visible_before_approval,
        review.policy_and_re_consent_preserved,
        review.remembered_approvals_revocable_from_stable_surface,
        review.support_export_keeps_capability_vocabulary,
        review.no_surface_uses_generic_access_prompt,
        review.every_sheet_bound_to_shell_zone,
        review.every_sheet_declares_accessibility_route,
        review.later_sheets_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5CapabilitySheetPrimitiveViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5CapabilitySheetPrimitivePacket,
    violations: &mut Vec<M5CapabilitySheetPrimitiveViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.trust_lanes_consume_shared_sheet,
        projection.resolver_reads_single_scope_ladder,
        projection.revoke_path_reads_single_source,
        projection.transitive_disclosure_reads_single_source,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations.push(M5CapabilitySheetPrimitiveViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5CapabilitySheetPrimitivePacket,
    violations: &mut Vec<M5CapabilitySheetPrimitiveViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5CapabilitySheetPrimitiveViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5CapabilitySheetPrimitivePacket,
    violations: &mut Vec<M5CapabilitySheetPrimitiveViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.capability_sheet_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5CapabilitySheetPrimitiveViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
