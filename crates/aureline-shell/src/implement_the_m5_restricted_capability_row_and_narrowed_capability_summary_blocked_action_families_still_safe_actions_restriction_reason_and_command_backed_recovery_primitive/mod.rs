//! Implemented M5 restricted-capability-row and narrowed-capability-summary primitives.
//!
//! The frozen [workspace-trust / guided-repair component matrix][matrix] names the reusable trust
//! and repair UI components and locks their controlled vocabulary. This module is the
//! restricted-mode implement lane over that matrix: it turns the **restricted-capability row** and
//! its **narrowed-capability summary** into resolvers that produce export-safe, honest projections,
//! so restricted mode reads as a stable operating posture a user can act inside rather than a vague
//! blocked state.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render restricted-capability rows that enumerate blocked action families, still-safe
//!   actions, and why the restriction exists.** [`resolve_restricted_capability_row`] refuses to
//!   read as a clean, legible row unless it names the restricted object, resolves the restriction
//!   scope, names the restriction source and reason, names the narrowed capability, enumerates at
//!   least one blocked action family, and names at least one still-safe action; it degrades instead.
//!   It never lets a restricted surface collapse into generic
//!   [`M5RestrictedCapabilityRowDegradeReason::CollapsedIntoGenericUnavailable`] copy.
//! * **Preserve command-backed recovery actions such as inspect trust, reopen restricted, continue
//!   limited, or request approval where allowed.** Every resolved row carries a command-backed
//!   [`M5RestrictedRecoveryAction`] set — always inspect-trust and reopen-restricted, continue-limited
//!   when a capability is narrowed, and request-approval only where the restriction allows it — and a
//!   row degrades to [`M5RestrictedCapabilityRowDegradeReason::RecoveryPathMissing`] the moment the
//!   command-backed entrypoint is unreachable and recovery would be docs- or logs-only.
//! * **Keep restricted summaries aligned across shell, editor, remote, AI, extension, and support
//!   entrypoints.** [`resolve_narrowed_capability_summary`] projects the same fields and the same
//!   command-backed recovery grammar as the row, so a narrowed-capability rollup on any surface
//!   still names what is blocked, what stays safe, and how to recover instead of forking its own
//!   "some features are unavailable" wording.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5WorkspaceTrustRepairDisposition`] trust / repair-disposition vocabulary, the
//! [`M5TrustScopeState`] trust-scope vocabulary, the [`M5TrustGrantSourceClass`] grant-source
//! vocabulary, the [`M5CapabilityNarrowState`] narrowed-capability vocabulary, and the
//! [`M5RootTrustState`] per-root trust vocabulary — so every claimed M5 restricted surface exposes
//! the same restriction, still-safe, and recovery grammar instead of forking its own copy.
//!
//! [matrix]: crate::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_restricted_capability_controls,
    seeded_m5_restricted_capability_controls_safe_mode_ui_preview_narrowed,
    seeded_m5_restricted_capability_controls_workspace_trust_ui_beta_narrowed,
    M5_RESTRICTED_CAPABILITY_CONTROLS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix::{
    M5CapabilityNarrowState, M5RootTrustState, M5TrustGrantSourceClass, M5TrustScopeState,
    M5WorkspaceTrustRepairAccessibilityRoute, M5WorkspaceTrustRepairComponentFamily,
    M5WorkspaceTrustRepairConsumerSurface, M5WorkspaceTrustRepairDeploymentLine,
    M5WorkspaceTrustRepairDisposition, M5WorkspaceTrustRepairDowngradeTrigger,
    M5WorkspaceTrustRepairQualificationClass, M5WorkspaceTrustRepairRequiredLabel,
    M5_RESTRICTED_CAPABILITY_ROW_SCHEMA_REF, M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
    M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5RestrictedCapabilityControlsPacket`].
pub const M5_RESTRICTED_CAPABILITY_CONTROLS_RECORD_KIND: &str =
    "implement_m5_restricted_capability_row_and_narrowed_capability_summary_controls";

/// Schema version for M5 restricted-capability controls records.
pub const M5_RESTRICTED_CAPABILITY_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_RESTRICTED_CAPABILITY_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-restricted-capability-row-narrowed-capability-summary-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_RESTRICTED_CAPABILITY_CONTROLS_DOC_REF: &str =
    "docs/trust/m5_restricted_capability_row_and_narrowed_capability_summary_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_RESTRICTED_CAPABILITY_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-restricted-capability-row-narrowed-capability-summary-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_RESTRICTED_CAPABILITY_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-restricted-capability-row-narrowed-capability-summary-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_RESTRICTED_CAPABILITY_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-restricted-capability-row-narrowed-capability-summary-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_RESTRICTED_CAPABILITY_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-restricted-capability-row-narrowed-capability-summary-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface
/// taxonomy so no lane invents a parallel surface set.
pub type M5RestrictedCapabilityConsumerSurface = M5WorkspaceTrustRepairConsumerSurface;

/// A named family of actions a restriction can block or leave safe. Enumerating real action
/// families keeps a restricted surface from collapsing into a single vague "some features are
/// unavailable" — a user can see exactly which families are blocked and which remain safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestrictedActionFamily {
    /// Running code, cells, or scripts.
    CodeExecution,
    /// Running tasks, builds, or automation.
    TaskAutomation,
    /// Activating or running workspace extensions.
    ExtensionActivation,
    /// Attaching or launching a debugger.
    DebuggerAttach,
    /// Writing workspace settings or configuration.
    WorkspaceSettingsWrite,
    /// Making outbound network / external requests.
    OutboundRequests,
    /// Editing or saving files in the workspace.
    FileEditing,
    /// Read-only navigation — browsing, reading, and searching (typically safe).
    ReadOnlyNavigation,
}

impl M5RestrictedActionFamily {
    /// Every action family, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::CodeExecution,
        Self::TaskAutomation,
        Self::ExtensionActivation,
        Self::DebuggerAttach,
        Self::WorkspaceSettingsWrite,
        Self::OutboundRequests,
        Self::FileEditing,
        Self::ReadOnlyNavigation,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CodeExecution => "code_execution",
            Self::TaskAutomation => "task_automation",
            Self::ExtensionActivation => "extension_activation",
            Self::DebuggerAttach => "debugger_attach",
            Self::WorkspaceSettingsWrite => "workspace_settings_write",
            Self::OutboundRequests => "outbound_requests",
            Self::FileEditing => "file_editing",
            Self::ReadOnlyNavigation => "read_only_navigation",
        }
    }

    /// Whether this family is one that typically stays safe under restriction.
    pub const fn is_typically_safe(self) -> bool {
        matches!(self, Self::ReadOnlyNavigation)
    }
}

/// A command-backed recovery action a restricted surface always keeps reachable, so a user is never
/// stuck in restricted mode without a route out and recovery choices stay consistent across
/// consumers rather than being scattered through docs or logs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestrictedRecoveryAction {
    /// Inspect trust — the command-backed entrypoint into trust detail, always reachable.
    InspectTrust,
    /// Reopen the object in restricted mode.
    ReopenRestricted,
    /// Continue with limited (reduced-mode) capability.
    ContinueLimited,
    /// Request approval to lift the restriction, where the restriction allows it.
    RequestApproval,
    /// Review diagnostics for a stale or unresolved signal.
    ReviewDiagnostics,
    /// No recovery is needed.
    NoRecoveryNeeded,
}

impl M5RestrictedRecoveryAction {
    /// Every recovery action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InspectTrust,
        Self::ReopenRestricted,
        Self::ContinueLimited,
        Self::RequestApproval,
        Self::ReviewDiagnostics,
        Self::NoRecoveryNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectTrust => "inspect_trust",
            Self::ReopenRestricted => "reopen_restricted",
            Self::ContinueLimited => "continue_limited",
            Self::RequestApproval => "request_approval",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoRecoveryNeeded => "no_recovery_needed",
        }
    }
}

/// One mandatory rendered part a restricted-capability row or narrowed-capability summary must be
/// able to show, so no restriction fact is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestrictedCapabilityAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed trust disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The restricted object identity behind the row or summary.
    ObjectIdentity,
    /// The restriction scope / trust class the object sits in.
    RestrictionScope,
    /// Why the restriction exists — the human-readable reason.
    RestrictionReason,
    /// The grant source that imposed the restriction.
    GrantSource,
    /// The narrowed capability the restriction removes.
    NarrowedCapability,
    /// The enumerated blocked action families.
    BlockedActionFamilies,
    /// The still-safe actions that remain available.
    StillSafeActions,
    /// The command-backed recovery actions.
    RecoveryActions,
    /// The command-backed path to inspect trust before acting.
    TrustDetailCommand,
}

impl M5RestrictedCapabilityAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ObjectIdentity,
        Self::RestrictionScope,
        Self::RestrictionReason,
        Self::GrantSource,
        Self::NarrowedCapability,
        Self::BlockedActionFamilies,
        Self::StillSafeActions,
        Self::RecoveryActions,
        Self::TrustDetailCommand,
    ];

    /// The three parts every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::ObjectIdentity => "object_identity",
            Self::RestrictionScope => "restriction_scope",
            Self::RestrictionReason => "restriction_reason",
            Self::GrantSource => "grant_source",
            Self::NarrowedCapability => "narrowed_capability",
            Self::BlockedActionFamilies => "blocked_action_families",
            Self::StillSafeActions => "still_safe_actions",
            Self::RecoveryActions => "recovery_actions",
            Self::TrustDetailCommand => "trust_detail_command",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestrictedCapabilityExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The component families covered.
    ComponentFamilies,
    /// The trust dispositions carried.
    TrustDispositions,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The restriction scope named by the components.
    RestrictionScope,
    /// The grant source named by the components.
    GrantSource,
    /// The narrowed capability named by the components.
    NarrowedCapability,
    /// The blocked action families enumerated by the components.
    BlockedActionFamilies,
    /// The still-safe actions named by the components.
    StillSafeActions,
    /// The command-backed recovery actions offered by the components.
    RecoveryActions,
    /// The accountable owner role.
    OwnerRole,
}

impl M5RestrictedCapabilityExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::TrustDispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::RestrictionScope,
        Self::GrantSource,
        Self::NarrowedCapability,
        Self::BlockedActionFamilies,
        Self::StillSafeActions,
        Self::RecoveryActions,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::TrustDispositions,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::ComponentFamilies => "component_families",
            Self::TrustDispositions => "trust_dispositions",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::RestrictionScope => "restriction_scope",
            Self::GrantSource => "grant_source",
            Self::NarrowedCapability => "narrowed_capability",
            Self::BlockedActionFamilies => "blocked_action_families",
            Self::StillSafeActions => "still_safe_actions",
            Self::RecoveryActions => "recovery_actions",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a restricted-capability row degraded below a clean, fully-legible state. The degrade-first
/// ladder returns one of these instead of ever letting a restricted surface read as a generic,
/// unexplained blocked state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestrictedCapabilityRowDegradeReason {
    /// The restricted object identity is unstated; a user cannot tell what the restriction applies to.
    ObjectIdentityUnstated,
    /// The restriction scope cannot currently be resolved.
    RestrictionScopeUnresolved,
    /// The grant source that imposed the restriction is undisclosed.
    RestrictionSourceUnstated,
    /// Why the restriction exists is not stated.
    RestrictionReasonUnstated,
    /// The narrowed capability is not named.
    NarrowedCapabilityUnstated,
    /// No blocked action family is enumerated; the restriction is left vague.
    BlockedActionFamiliesUnstated,
    /// No still-safe action is named; a user cannot tell what remains safe.
    StillSafeActionsUnstated,
    /// The restriction collapsed into generic "unavailable" copy.
    CollapsedIntoGenericUnavailable,
    /// A mixed-root restriction reads as a uniform (blanket) restriction across roots.
    MixedRootCollapsedIntoUniform,
    /// No command-backed recovery path is reachable; recovery would be docs- or logs-only.
    RecoveryPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RestrictedCapabilityRowDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ObjectIdentityUnstated,
        Self::RestrictionScopeUnresolved,
        Self::RestrictionSourceUnstated,
        Self::RestrictionReasonUnstated,
        Self::NarrowedCapabilityUnstated,
        Self::BlockedActionFamiliesUnstated,
        Self::StillSafeActionsUnstated,
        Self::CollapsedIntoGenericUnavailable,
        Self::MixedRootCollapsedIntoUniform,
        Self::RecoveryPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObjectIdentityUnstated => "object_identity_unstated",
            Self::RestrictionScopeUnresolved => "restriction_scope_unresolved",
            Self::RestrictionSourceUnstated => "restriction_source_unstated",
            Self::RestrictionReasonUnstated => "restriction_reason_unstated",
            Self::NarrowedCapabilityUnstated => "narrowed_capability_unstated",
            Self::BlockedActionFamiliesUnstated => "blocked_action_families_unstated",
            Self::StillSafeActionsUnstated => "still_safe_actions_unstated",
            Self::CollapsedIntoGenericUnavailable => "collapsed_into_generic_unavailable",
            Self::MixedRootCollapsedIntoUniform => "mixed_root_collapsed_into_uniform",
            Self::RecoveryPathMissing => "recovery_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe recovery action for this reason.
    pub const fn next_action(self) -> M5RestrictedRecoveryAction {
        match self {
            Self::ProofStale => M5RestrictedRecoveryAction::ReviewDiagnostics,
            _ => M5RestrictedRecoveryAction::InspectTrust,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WorkspaceTrustRepairDowngradeTrigger {
        match self {
            Self::RestrictionSourceUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::GrantSourceUnstated
            }
            Self::NarrowedCapabilityUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::NarrowedCapabilityUnstated
            }
            Self::MixedRootCollapsedIntoUniform => {
                M5WorkspaceTrustRepairDowngradeTrigger::MixedRootShownAsUniformTrust
            }
            Self::ProofStale => M5WorkspaceTrustRepairDowngradeTrigger::ProofStale,
            _ => M5WorkspaceTrustRepairDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// Reason a narrowed-capability summary degraded below a clean, legible rollup.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NarrowedCapabilitySummaryDegradeReason {
    /// The posture object identity is unstated.
    PostureIdentityUnstated,
    /// The capability posture (scope) cannot currently be resolved.
    PostureUnresolved,
    /// The grant source that imposed the restriction is undisclosed.
    RestrictionSourceUnstated,
    /// Why the restriction exists is not stated.
    RestrictionReasonUnstated,
    /// The narrowed capability is not named.
    NarrowedCapabilityUnstated,
    /// No blocked action family is enumerated.
    BlockedActionFamiliesUnstated,
    /// Distinct blocked families collapsed into a single generic count.
    BlockedFamiliesCollapsedIntoGenericCount,
    /// No still-safe action is named.
    StillSafeActionsUnstated,
    /// The summary collapsed into generic "unavailable" copy.
    CollapsedIntoGenericUnavailable,
    /// No command-backed recovery path is reachable.
    RecoveryPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5NarrowedCapabilitySummaryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::PostureIdentityUnstated,
        Self::PostureUnresolved,
        Self::RestrictionSourceUnstated,
        Self::RestrictionReasonUnstated,
        Self::NarrowedCapabilityUnstated,
        Self::BlockedActionFamiliesUnstated,
        Self::BlockedFamiliesCollapsedIntoGenericCount,
        Self::StillSafeActionsUnstated,
        Self::CollapsedIntoGenericUnavailable,
        Self::RecoveryPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PostureIdentityUnstated => "posture_identity_unstated",
            Self::PostureUnresolved => "posture_unresolved",
            Self::RestrictionSourceUnstated => "restriction_source_unstated",
            Self::RestrictionReasonUnstated => "restriction_reason_unstated",
            Self::NarrowedCapabilityUnstated => "narrowed_capability_unstated",
            Self::BlockedActionFamiliesUnstated => "blocked_action_families_unstated",
            Self::BlockedFamiliesCollapsedIntoGenericCount => {
                "blocked_families_collapsed_into_generic_count"
            }
            Self::StillSafeActionsUnstated => "still_safe_actions_unstated",
            Self::CollapsedIntoGenericUnavailable => "collapsed_into_generic_unavailable",
            Self::RecoveryPathMissing => "recovery_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe recovery action for this reason.
    pub const fn next_action(self) -> M5RestrictedRecoveryAction {
        match self {
            Self::ProofStale => M5RestrictedRecoveryAction::ReviewDiagnostics,
            _ => M5RestrictedRecoveryAction::InspectTrust,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WorkspaceTrustRepairDowngradeTrigger {
        match self {
            Self::RestrictionSourceUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::GrantSourceUnstated
            }
            Self::NarrowedCapabilityUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::NarrowedCapabilityUnstated
            }
            Self::ProofStale => M5WorkspaceTrustRepairDowngradeTrigger::ProofStale,
            _ => M5WorkspaceTrustRepairDowngradeTrigger::GenericChromeWordingUsed,
        }
    }
}

/// Maps a trust scope to the single controlled trust disposition, or `None` when the scope cannot be
/// resolved — an unresolved scope never borrows a trusted or restricted word.
fn disposition_for_scope(scope: M5TrustScopeState) -> Option<M5WorkspaceTrustRepairDisposition> {
    use M5WorkspaceTrustRepairDisposition as D;
    match scope {
        M5TrustScopeState::TrustedWorkspace | M5TrustScopeState::TrustedRoot => Some(D::Trusted),
        M5TrustScopeState::RestrictedWorkspace => Some(D::Restricted),
        M5TrustScopeState::MixedRoot => Some(D::MixedRoot),
        M5TrustScopeState::PolicyBlocked => Some(D::PolicyBlocked),
        M5TrustScopeState::ScopeUnknown => None,
    }
}

/// True when the grant source that imposed the restriction is a concrete, disclosed class.
fn grant_is_resolved(source: M5TrustGrantSourceClass) -> bool {
    !matches!(source, M5TrustGrantSourceClass::GrantSourceUnknown)
}

/// Builds the command-backed recovery set every restricted surface keeps reachable. Inspect-trust
/// and reopen-restricted are always offered; continue-limited is offered when a capability is
/// narrowed; request-approval is offered only where the restriction allows it.
fn recovery_actions_for(
    capability_narrowed: bool,
    approval_allowed: bool,
) -> Vec<M5RestrictedRecoveryAction> {
    let mut actions = vec![
        M5RestrictedRecoveryAction::InspectTrust,
        M5RestrictedRecoveryAction::ReopenRestricted,
    ];
    if capability_narrowed {
        actions.push(M5RestrictedRecoveryAction::ContinueLimited);
    }
    if approval_allowed {
        actions.push(M5RestrictedRecoveryAction::RequestApproval);
    }
    actions
}

/// Input to [`resolve_restricted_capability_row`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RestrictedCapabilityRowResolutionInput {
    /// Stable identity of the row instance.
    pub row_id: String,
    /// The restricted object identity (workspace / root / feature); empty means unstated.
    pub object_identity: String,
    /// The restriction scope of the object.
    pub trust_scope: M5TrustScopeState,
    /// The per-root trust state named alongside the scope.
    pub root_trust: M5RootTrustState,
    /// The grant source that imposed the restriction.
    pub grant_source: M5TrustGrantSourceClass,
    /// True when the human-readable reason for the restriction is named on the row.
    pub restriction_reason_stated: bool,
    /// The narrowed-capability state.
    pub capability_narrow: M5CapabilityNarrowState,
    /// True when the narrowed capability is named on the row.
    pub capability_narrow_stated: bool,
    /// The blocked action families enumerated by the row.
    pub blocked_action_families: Vec<M5RestrictedActionFamily>,
    /// The still-safe actions the row names as remaining available.
    pub still_safe_actions: Vec<M5RestrictedActionFamily>,
    /// True when the restriction allows requesting approval to lift it.
    pub approval_allowed: bool,
    /// True when the row collapses the restriction into generic "unavailable" copy.
    pub reads_as_generic_unavailable: bool,
    /// True when the row reads a mixed-root restriction as uniform across roots.
    pub reads_as_uniform_trust: bool,
    /// True when a command-backed trust-detail / recovery entrypoint is reachable, never docs-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe restricted-capability row projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRestrictedCapabilityRow {
    /// Stable identity of the row instance.
    pub row_id: String,
    /// The restricted object identity named by the row.
    pub object_identity: String,
    /// The restriction-scope token named by the row.
    pub restriction_scope: String,
    /// Single controlled trust disposition, or `null` when the scope is unresolved.
    pub trust_disposition: Option<M5WorkspaceTrustRepairDisposition>,
    /// The per-root trust token named by the row.
    pub root_trust: String,
    /// The grant-source token that imposed the restriction.
    pub grant_source: String,
    /// Whether the human-readable restriction reason is named.
    pub restriction_reason_stated: bool,
    /// The narrowed-capability token named by the row.
    pub capability_narrow: String,
    /// Whether any capability is narrowed relative to full capability.
    pub capability_narrowed: bool,
    /// The blocked action families enumerated by the row.
    pub blocked_action_families: Vec<M5RestrictedActionFamily>,
    /// The still-safe actions the row names as remaining available.
    pub still_safe_actions: Vec<M5RestrictedActionFamily>,
    /// The command-backed recovery actions the row keeps reachable.
    pub recovery_actions: Vec<M5RestrictedRecoveryAction>,
    /// Whether request-approval is offered on this row.
    pub approval_available: bool,
    /// Whether this row describes a mixed-root restriction.
    pub is_mixed_root: bool,
    /// Guardrail (MUST be `false` on a clean row): the restriction collapses into generic unavailable.
    pub collapses_into_generic_unavailable: bool,
    /// Guardrail (MUST be `false` on a clean row): a mixed-root restriction reads as uniform.
    pub collapses_per_root_into_uniform: bool,
    /// Whether a command-backed trust-detail / recovery entrypoint is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the row could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5RestrictedCapabilityRowDegradeReason>,
    /// Next safe recovery action offered.
    pub next_action: M5RestrictedRecoveryAction,
    /// Whether restricted mode reads as a stable, fully-legible posture (clean row).
    pub restricted_posture_legible: bool,
}

impl M5ResolvedRestrictedCapabilityRow {
    /// Whether this row reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_narrowed_capability_summary`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5NarrowedCapabilitySummaryResolutionInput {
    /// Stable identity of the summary instance.
    pub summary_id: String,
    /// The posture object identity; empty means unstated.
    pub object_identity: String,
    /// The capability posture (restriction scope).
    pub trust_scope: M5TrustScopeState,
    /// The grant source that imposed the restriction.
    pub grant_source: M5TrustGrantSourceClass,
    /// True when the human-readable reason for the restriction is named on the summary.
    pub restriction_reason_stated: bool,
    /// The narrowed-capability state.
    pub capability_narrow: M5CapabilityNarrowState,
    /// True when the narrowed capability is named on the summary.
    pub capability_narrow_stated: bool,
    /// The blocked action families summarised.
    pub blocked_action_families: Vec<M5RestrictedActionFamily>,
    /// The still-safe actions the summary names as remaining available.
    pub still_safe_actions: Vec<M5RestrictedActionFamily>,
    /// True when the restriction allows requesting approval to lift it.
    pub approval_allowed: bool,
    /// True when the summary collapses the restriction into generic "unavailable" copy.
    pub reads_as_generic_unavailable: bool,
    /// True when the summary collapses distinct blocked families into a single generic count.
    pub collapses_blocked_families: bool,
    /// True when a command-backed trust-detail / recovery entrypoint is reachable, never docs-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe narrowed-capability summary projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedNarrowedCapabilitySummary {
    /// Stable identity of the summary instance.
    pub summary_id: String,
    /// The posture object identity named by the summary.
    pub object_identity: String,
    /// The restriction-scope token named by the summary.
    pub restriction_scope: String,
    /// Single controlled trust disposition, or `null` when the scope is unresolved.
    pub trust_disposition: Option<M5WorkspaceTrustRepairDisposition>,
    /// The grant-source token that imposed the restriction.
    pub grant_source: String,
    /// Whether the human-readable restriction reason is named.
    pub restriction_reason_stated: bool,
    /// The narrowed-capability token named by the summary.
    pub capability_narrow: String,
    /// Whether any capability is narrowed relative to full capability.
    pub capability_narrowed: bool,
    /// The count of blocked action families.
    pub blocked_family_count: usize,
    /// The blocked action families summarised.
    pub blocked_action_families: Vec<M5RestrictedActionFamily>,
    /// The count of still-safe actions.
    pub safe_action_count: usize,
    /// The still-safe actions the summary names as remaining available.
    pub still_safe_actions: Vec<M5RestrictedActionFamily>,
    /// The command-backed recovery actions the summary keeps reachable.
    pub recovery_actions: Vec<M5RestrictedRecoveryAction>,
    /// Guardrail (MUST be `false` on a clean summary): the restriction collapses into generic
    /// unavailable.
    pub collapses_into_generic_unavailable: bool,
    /// Guardrail (MUST be `false` on a clean summary): distinct blocked families collapse into a
    /// generic count.
    pub collapses_blocked_families: bool,
    /// Whether a command-backed trust-detail / recovery entrypoint is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the summary could not read as a clean, legible rollup.
    pub degrade_reason: Option<M5NarrowedCapabilitySummaryDegradeReason>,
    /// Next safe recovery action offered.
    pub next_action: M5RestrictedRecoveryAction,
    /// Whether the narrowed-capability posture reads as legible (clean summary).
    pub posture_legible: bool,
}

impl M5ResolvedNarrowedCapabilitySummary {
    /// Whether this summary reads as a clean, legible rollup.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5RestrictedCapabilityResolutionError {
    /// The row id was empty.
    EmptyRowId,
    /// The summary id was empty.
    EmptySummaryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5RestrictedCapabilityResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyRowId => "empty_row_id",
            Self::EmptySummaryId => "empty_summary_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5RestrictedCapabilityResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 restricted-capability resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RestrictedCapabilityResolutionError {}

/// Resolves a restricted-capability row, making restricted mode a legible operating posture: the row
/// names its object, restriction scope, restriction source and reason, narrowed capability,
/// enumerated blocked action families, and still-safe actions, keeps a command-backed recovery set
/// reachable, and never collapses into a generic unexplained blocked state.
pub fn resolve_restricted_capability_row(
    input: M5RestrictedCapabilityRowResolutionInput,
) -> Result<M5ResolvedRestrictedCapabilityRow, M5RestrictedCapabilityResolutionError> {
    if input.row_id.trim().is_empty() {
        return Err(M5RestrictedCapabilityResolutionError::EmptyRowId);
    }
    if string_is_forbidden(&input.row_id) || string_is_forbidden(&input.object_identity) {
        return Err(M5RestrictedCapabilityResolutionError::ForbiddenMaterial);
    }

    let is_mixed_root = matches!(input.trust_scope, M5TrustScopeState::MixedRoot);
    let capability_narrowed = !matches!(
        input.capability_narrow,
        M5CapabilityNarrowState::FullCapability
    );
    let collapses_per_root_into_uniform = is_mixed_root && input.reads_as_uniform_trust;
    let recovery_actions = recovery_actions_for(capability_narrowed, input.approval_allowed);

    let degrade_reason = if input.object_identity.trim().is_empty() {
        Some(M5RestrictedCapabilityRowDegradeReason::ObjectIdentityUnstated)
    } else if matches!(input.trust_scope, M5TrustScopeState::ScopeUnknown) {
        Some(M5RestrictedCapabilityRowDegradeReason::RestrictionScopeUnresolved)
    } else if !grant_is_resolved(input.grant_source) {
        Some(M5RestrictedCapabilityRowDegradeReason::RestrictionSourceUnstated)
    } else if !input.restriction_reason_stated {
        Some(M5RestrictedCapabilityRowDegradeReason::RestrictionReasonUnstated)
    } else if capability_narrowed && !input.capability_narrow_stated {
        Some(M5RestrictedCapabilityRowDegradeReason::NarrowedCapabilityUnstated)
    } else if input.blocked_action_families.is_empty() {
        Some(M5RestrictedCapabilityRowDegradeReason::BlockedActionFamiliesUnstated)
    } else if input.still_safe_actions.is_empty() {
        Some(M5RestrictedCapabilityRowDegradeReason::StillSafeActionsUnstated)
    } else if input.reads_as_generic_unavailable {
        Some(M5RestrictedCapabilityRowDegradeReason::CollapsedIntoGenericUnavailable)
    } else if collapses_per_root_into_uniform {
        Some(M5RestrictedCapabilityRowDegradeReason::MixedRootCollapsedIntoUniform)
    } else if !input.detail_command_available {
        Some(M5RestrictedCapabilityRowDegradeReason::RecoveryPathMissing)
    } else if !input.proof_fresh {
        Some(M5RestrictedCapabilityRowDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RestrictedRecoveryAction::InspectTrust,
    };

    Ok(M5ResolvedRestrictedCapabilityRow {
        row_id: input.row_id,
        object_identity: input.object_identity,
        restriction_scope: input.trust_scope.as_str().to_owned(),
        trust_disposition: disposition_for_scope(input.trust_scope),
        root_trust: input.root_trust.as_str().to_owned(),
        grant_source: input.grant_source.as_str().to_owned(),
        restriction_reason_stated: input.restriction_reason_stated,
        capability_narrow: input.capability_narrow.as_str().to_owned(),
        capability_narrowed,
        blocked_action_families: input.blocked_action_families,
        still_safe_actions: input.still_safe_actions,
        recovery_actions,
        approval_available: input.approval_allowed,
        is_mixed_root,
        collapses_into_generic_unavailable: input.reads_as_generic_unavailable,
        collapses_per_root_into_uniform,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        restricted_posture_legible: degrade_reason.is_none(),
    })
}

/// Resolves a narrowed-capability summary, keeping a restricted rollup aligned with the row: the
/// summary names its posture, restriction source and reason, narrowed capability, enumerated blocked
/// families, and still-safe actions, keeps the same command-backed recovery grammar, and never
/// collapses distinct blocked families or the whole posture into generic unavailable copy.
pub fn resolve_narrowed_capability_summary(
    input: M5NarrowedCapabilitySummaryResolutionInput,
) -> Result<M5ResolvedNarrowedCapabilitySummary, M5RestrictedCapabilityResolutionError> {
    if input.summary_id.trim().is_empty() {
        return Err(M5RestrictedCapabilityResolutionError::EmptySummaryId);
    }
    if string_is_forbidden(&input.summary_id) || string_is_forbidden(&input.object_identity) {
        return Err(M5RestrictedCapabilityResolutionError::ForbiddenMaterial);
    }

    let capability_narrowed = !matches!(
        input.capability_narrow,
        M5CapabilityNarrowState::FullCapability
    );
    let recovery_actions = recovery_actions_for(capability_narrowed, input.approval_allowed);

    let degrade_reason = if input.object_identity.trim().is_empty() {
        Some(M5NarrowedCapabilitySummaryDegradeReason::PostureIdentityUnstated)
    } else if matches!(input.trust_scope, M5TrustScopeState::ScopeUnknown) {
        Some(M5NarrowedCapabilitySummaryDegradeReason::PostureUnresolved)
    } else if !grant_is_resolved(input.grant_source) {
        Some(M5NarrowedCapabilitySummaryDegradeReason::RestrictionSourceUnstated)
    } else if !input.restriction_reason_stated {
        Some(M5NarrowedCapabilitySummaryDegradeReason::RestrictionReasonUnstated)
    } else if capability_narrowed && !input.capability_narrow_stated {
        Some(M5NarrowedCapabilitySummaryDegradeReason::NarrowedCapabilityUnstated)
    } else if input.blocked_action_families.is_empty() {
        Some(M5NarrowedCapabilitySummaryDegradeReason::BlockedActionFamiliesUnstated)
    } else if input.collapses_blocked_families {
        Some(M5NarrowedCapabilitySummaryDegradeReason::BlockedFamiliesCollapsedIntoGenericCount)
    } else if input.still_safe_actions.is_empty() {
        Some(M5NarrowedCapabilitySummaryDegradeReason::StillSafeActionsUnstated)
    } else if input.reads_as_generic_unavailable {
        Some(M5NarrowedCapabilitySummaryDegradeReason::CollapsedIntoGenericUnavailable)
    } else if !input.detail_command_available {
        Some(M5NarrowedCapabilitySummaryDegradeReason::RecoveryPathMissing)
    } else if !input.proof_fresh {
        Some(M5NarrowedCapabilitySummaryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RestrictedRecoveryAction::InspectTrust,
    };

    Ok(M5ResolvedNarrowedCapabilitySummary {
        summary_id: input.summary_id,
        object_identity: input.object_identity,
        restriction_scope: input.trust_scope.as_str().to_owned(),
        trust_disposition: disposition_for_scope(input.trust_scope),
        grant_source: input.grant_source.as_str().to_owned(),
        restriction_reason_stated: input.restriction_reason_stated,
        capability_narrow: input.capability_narrow.as_str().to_owned(),
        capability_narrowed,
        blocked_family_count: input.blocked_action_families.len(),
        blocked_action_families: input.blocked_action_families,
        safe_action_count: input.still_safe_actions.len(),
        still_safe_actions: input.still_safe_actions,
        recovery_actions,
        collapses_into_generic_unavailable: input.reads_as_generic_unavailable,
        collapses_blocked_families: input.collapses_blocked_families,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        posture_legible: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved restricted-capability row and
/// narrowed-capability summary examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestrictedCapabilityControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5RestrictedCapabilityConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5WorkspaceTrustRepairQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5WorkspaceTrustRepairDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5WorkspaceTrustRepairRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5WorkspaceTrustRepairAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5RestrictedCapabilityAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5RestrictedCapabilityExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5WorkspaceTrustRepairDowngradeTrigger>,
    /// Resolved restricted-capability row examples.
    pub restricted_capability_row_examples: Vec<M5ResolvedRestrictedCapabilityRow>,
    /// Resolved narrowed-capability summary examples.
    pub narrowed_capability_summary_examples: Vec<M5ResolvedNarrowedCapabilitySummary>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the component schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never collapse a restricted surface into generic unavailable copy.
    pub collapses_restricted_into_generic_unavailable: bool,
    /// Hard invariant: never hide the blocked action families or the still-safe actions.
    pub hides_blocked_families_or_still_safe_actions: bool,
    /// Hard invariant: never route recovery through docs or logs only, off the command path.
    pub routes_recovery_through_docs_or_logs_only: bool,
    /// Hard invariant: never imply a blanket restriction across roots or routes.
    pub implies_blanket_restriction_across_roots_or_routes: bool,
}

impl M5RestrictedCapabilityControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5RestrictedCapabilityAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5RestrictedCapabilityAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5RestrictedCapabilityExportField> =
            self.export_fields.iter().copied().collect();
        M5RestrictedCapabilityExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.collapses_restricted_into_generic_unavailable
            && !self.hides_blocked_families_or_still_safe_actions
            && !self.routes_recovery_through_docs_or_logs_only
            && !self.implies_blanket_restriction_across_roots_or_routes
    }

    /// True when every resolved example on this row is honest: no clean row/summary collapses into
    /// generic unavailable, no clean row collapses per-root restriction into uniform, no clean
    /// summary collapses distinct blocked families, and no clean example hides the recovery path.
    fn examples_are_honest(&self) -> bool {
        self.restricted_capability_row_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.collapses_into_generic_unavailable
                    || ex.collapses_per_root_into_uniform
                    || !ex.detail_command_available))
        }) && self.narrowed_capability_summary_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.collapses_into_generic_unavailable
                    || ex.collapses_blocked_families
                    || !ex.detail_command_available))
        })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestrictedCapabilityVocabularySet {
    /// Trust-disposition tokens (bound from the frozen matrix).
    pub trust_dispositions: Vec<String>,
    /// Trust-scope tokens (bound from the frozen matrix).
    pub trust_scopes: Vec<String>,
    /// Grant-source tokens (bound from the frozen matrix).
    pub grant_sources: Vec<String>,
    /// Narrowed-capability tokens (bound from the frozen matrix).
    pub capability_narrow_states: Vec<String>,
    /// Per-root trust tokens (bound from the frozen matrix).
    pub root_trust_states: Vec<String>,
    /// Action-family tokens (minted by this lane).
    pub action_families: Vec<String>,
    /// Recovery-action tokens (minted by this lane).
    pub recovery_actions: Vec<String>,
    /// Row degrade-reason tokens.
    pub row_degrade_reasons: Vec<String>,
    /// Summary degrade-reason tokens.
    pub summary_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5RestrictedCapabilityVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            trust_dispositions: tokens(&M5WorkspaceTrustRepairDisposition::ALL, |v| v.as_str()),
            trust_scopes: tokens(&M5TrustScopeState::ALL, |v| v.as_str()),
            grant_sources: tokens(&M5TrustGrantSourceClass::ALL, |v| v.as_str()),
            capability_narrow_states: tokens(&M5CapabilityNarrowState::ALL, |v| v.as_str()),
            root_trust_states: tokens(&M5RootTrustState::ALL, |v| v.as_str()),
            action_families: tokens(&M5RestrictedActionFamily::ALL, |v| v.as_str()),
            recovery_actions: tokens(&M5RestrictedRecoveryAction::ALL, |v| v.as_str()),
            row_degrade_reasons: tokens(&M5RestrictedCapabilityRowDegradeReason::ALL, |v| {
                v.as_str()
            }),
            summary_degrade_reasons: tokens(&M5NarrowedCapabilitySummaryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5RestrictedCapabilityAnatomyPart::ALL, |v| v.as_str()),
            export_fields: tokens(&M5RestrictedCapabilityExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5WorkspaceTrustRepairConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5RestrictedCapabilityGovernanceReview {
    /// Every restricted row names its object, restriction scope, and restriction source.
    pub row_names_object_scope_and_source: bool,
    /// Every restricted row enumerates blocked action families and still-safe actions.
    pub row_enumerates_blocked_and_still_safe: bool,
    /// No restricted surface collapses into generic unavailable copy.
    pub no_surface_collapses_into_generic_unavailable: bool,
    /// A command-backed recovery path is always reachable.
    pub command_backed_recovery_always_reachable: bool,
    /// Recovery choices are consistent across consumers.
    pub recovery_choices_consistent_across_consumers: bool,
    /// No restricted surface implies a blanket restriction across roots or routes.
    pub no_surface_implies_blanket_restriction: bool,
    /// Restricted surfaces share one field and recovery vocabulary across surfaces.
    pub restricted_vocabulary_shared_across_surfaces: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestrictedCapabilityConsumerProjection {
    /// Restricted surfaces across consumers expose the same fields and recovery grammar.
    pub restricted_surfaces_expose_same_fields_and_recovery: bool,
    /// Still-safe actions are legible without hunting through docs or logs.
    pub still_safe_actions_legible_without_docs: bool,
    /// Restricted state traces back to one canonical component contract.
    pub restricted_traces_to_single_component_contract: bool,
    /// Support / export reads a single canonical restricted source.
    pub support_export_reads_single_restricted_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestrictedCapabilityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestrictedCapabilityReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5RestrictedCapabilityControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RestrictedCapabilityControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5RestrictedCapabilityControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RestrictedCapabilityVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RestrictedCapabilityGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RestrictedCapabilityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RestrictedCapabilityProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RestrictedCapabilityReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 restricted-capability-row / narrowed-capability-summary controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RestrictedCapabilityControlsPacket {
    /// Record kind; must equal [`M5_RESTRICTED_CAPABILITY_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_RESTRICTED_CAPABILITY_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5RestrictedCapabilityControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5RestrictedCapabilityVocabularySet,
    /// Governance-review block.
    pub governance_review: M5RestrictedCapabilityGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5RestrictedCapabilityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5RestrictedCapabilityProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5RestrictedCapabilityReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5RestrictedCapabilityControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5RestrictedCapabilityControlsPacketInput) -> Self {
        Self {
            record_kind: M5_RESTRICTED_CAPABILITY_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_RESTRICTED_CAPABILITY_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            controls_label: input.controls_label,
            controls_rows: input.controls_rows,
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

    /// Validates the controls-packet invariants.
    pub fn validate(&self) -> Vec<M5RestrictedCapabilityControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_RESTRICTED_CAPABILITY_CONTROLS_RECORD_KIND {
            violations.push(M5RestrictedCapabilityControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_RESTRICTED_CAPABILITY_CONTROLS_SCHEMA_VERSION {
            violations.push(M5RestrictedCapabilityControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5RestrictedCapabilityControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5RestrictedCapabilityControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 restricted-capability controls packet serializes"),
        ) {
            violations.push(M5RestrictedCapabilityControlsViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 restricted-capability controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,row_examples,summary_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .restricted_capability_row_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.narrowed_capability_summary_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.restricted_capability_row_examples.len(),
                row.narrowed_capability_summary_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Restricted-Capability-Row and Narrowed-Capability-Summary Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Restriction scopes: {}\n",
            self.vocabulary_set.trust_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Action families: {}\n",
            self.vocabulary_set.action_families.join(", ")
        ));
        out.push_str(&format!(
            "- Recovery actions: {}\n",
            self.vocabulary_set.recovery_actions.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.controls_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Row examples: {} / summary examples: {}\n",
                row.restricted_capability_row_examples.len(),
                row.narrowed_capability_summary_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5RestrictedCapabilityControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5RestrictedCapabilityControlsViolation>),
}

impl fmt::Display for M5RestrictedCapabilityControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 restricted-capability controls export parse failed: {error}"
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
                    "m5 restricted-capability controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5RestrictedCapabilityControlsArtifactError {}

/// Validation failures emitted by [`M5RestrictedCapabilityControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5RestrictedCapabilityControlsViolation {
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
    /// The controls packet declares no rows.
    NoControlsRows,
    /// A controls row is incomplete.
    ControlsRowIncomplete,
    /// A controls row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A controls row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A controls row does not point at the restricted-capability-row component schema.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (generic-unavailable collapse or hidden path).
    DishonestExample,
    /// A controls row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// No-generic-unavailable honesty is not proven: clean rows/summaries do not cover the required
    /// restriction scopes, or no generic-unavailable example degrades, or a clean example collapses.
    NoGenericUnavailableNotProven,
    /// Still-safe / command-backed recovery parity is not proven: clean examples do not name a
    /// still-safe action and a command-backed recovery path, or no missing example degrades.
    StillSafeAndRecoveryNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5RestrictedCapabilityControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoControlsRows => "no_controls_rows",
            Self::ControlsRowIncomplete => "controls_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::ComponentSchemaRefMissing => "component_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::NoGenericUnavailableNotProven => "no_generic_unavailable_not_proven",
            Self::StillSafeAndRecoveryNotProven => "still_safe_and_recovery_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_restricted_capability_controls_export(
) -> Result<M5RestrictedCapabilityControlsPacket, M5RestrictedCapabilityControlsArtifactError> {
    let packet: M5RestrictedCapabilityControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-restricted-capability-row-narrowed-capability-summary-controls-proof/support_export.json"
    )))
    .map_err(M5RestrictedCapabilityControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5RestrictedCapabilityControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5RestrictedCapabilityControlsPacket,
    violations: &mut Vec<M5RestrictedCapabilityControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_RESTRICTED_CAPABILITY_CONTROLS_SCHEMA_REF,
        M5_RESTRICTED_CAPABILITY_CONTROLS_DOC_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
        M5_RESTRICTED_CAPABILITY_ROW_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5RestrictedCapabilityControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5RestrictedCapabilityControlsPacket,
    violations: &mut Vec<M5RestrictedCapabilityControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5RestrictedCapabilityControlsViolation::NoControlsRows);
        return;
    }
    for row in &packet.controls_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5RestrictedCapabilityControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5RestrictedCapabilityControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5RestrictedCapabilityControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_RESTRICTED_CAPABILITY_ROW_SCHEMA_REF) {
            violations.push(M5RestrictedCapabilityControlsViolation::ComponentSchemaRefMissing);
        }
        if row.restricted_capability_row_examples.is_empty()
            || row.narrowed_capability_summary_examples.is_empty()
        {
            violations.push(M5RestrictedCapabilityControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5RestrictedCapabilityControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5RestrictedCapabilityControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5RestrictedCapabilityControlsPacket,
    violations: &mut Vec<M5RestrictedCapabilityControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.row_names_object_scope_and_source,
        review.row_enumerates_blocked_and_still_safe,
        review.no_surface_collapses_into_generic_unavailable,
        review.command_backed_recovery_always_reachable,
        review.recovery_choices_consistent_across_consumers,
        review.no_surface_implies_blanket_restriction,
        review.restricted_vocabulary_shared_across_surfaces,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5RestrictedCapabilityControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5RestrictedCapabilityControlsPacket,
    violations: &mut Vec<M5RestrictedCapabilityControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.restricted_surfaces_expose_same_fields_and_recovery,
        projection.still_safe_actions_legible_without_docs,
        projection.restricted_traces_to_single_component_contract,
        projection.support_export_reads_single_restricted_source,
    ] {
        if !ok {
            violations.push(M5RestrictedCapabilityControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5RestrictedCapabilityControlsPacket,
    violations: &mut Vec<M5RestrictedCapabilityControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5RestrictedCapabilityControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5RestrictedCapabilityControlsPacket,
    violations: &mut Vec<M5RestrictedCapabilityControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5RestrictedCapabilityControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5RestrictedCapabilityControlsPacket,
    violations: &mut Vec<M5RestrictedCapabilityControlsViolation>,
) {
    let rows = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.restricted_capability_row_examples.iter())
    };
    let summaries = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.narrowed_capability_summary_examples.iter())
    };

    // AC: restricted surfaces no longer collapse into generic unavailable copy. Clean rows cover the
    // restricted, policy-blocked, and mixed-root restriction scopes so a restriction is always a
    // differentiated posture; at least one row and one summary degrade to
    // `collapsed_into_generic_unavailable`; no clean example collapses; and every clean example
    // enumerates at least one blocked action family.
    let clean_row_scopes: BTreeSet<&str> = rows()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.restriction_scope.as_str())
        .collect();
    let covers_required_scopes = [
        M5TrustScopeState::RestrictedWorkspace,
        M5TrustScopeState::PolicyBlocked,
        M5TrustScopeState::MixedRoot,
    ]
    .iter()
    .all(|scope| clean_row_scopes.contains(scope.as_str()));
    let generic_row_degrades = rows().any(|ex| {
        ex.degrade_reason
            == Some(M5RestrictedCapabilityRowDegradeReason::CollapsedIntoGenericUnavailable)
            && ex.collapses_into_generic_unavailable
    });
    let generic_summary_degrades = summaries().any(|ex| {
        ex.degrade_reason
            == Some(M5NarrowedCapabilitySummaryDegradeReason::CollapsedIntoGenericUnavailable)
            && ex.collapses_into_generic_unavailable
    });
    let no_clean_generic = rows()
        .all(|ex| !(ex.is_clean() && ex.collapses_into_generic_unavailable))
        && summaries().all(|ex| !(ex.is_clean() && ex.collapses_into_generic_unavailable));
    let clean_enumerate_blocked = rows()
        .filter(|ex| ex.is_clean())
        .all(|ex| !ex.blocked_action_families.is_empty())
        && summaries()
            .filter(|ex| ex.is_clean())
            .all(|ex| !ex.blocked_action_families.is_empty());
    if !(covers_required_scopes
        && generic_row_degrades
        && generic_summary_degrades
        && no_clean_generic
        && clean_enumerate_blocked)
    {
        violations.push(M5RestrictedCapabilityControlsViolation::NoGenericUnavailableNotProven);
    }

    // AC: users can tell which actions remain safe, and recovery is consistent and command-backed.
    // Every clean row/summary names at least one still-safe action and exposes a command-backed
    // recovery path anchored on inspect-trust; missing-still-safe and missing-recovery examples
    // degrade; and every clean row's recovery set is anchored on the same inspect-trust entrypoint so
    // recovery choices stay consistent across consumers.
    let clean_names_still_safe = rows()
        .filter(|ex| ex.is_clean())
        .all(|ex| !ex.still_safe_actions.is_empty())
        && summaries()
            .filter(|ex| ex.is_clean())
            .all(|ex| !ex.still_safe_actions.is_empty());
    let clean_exposes_recovery = rows().filter(|ex| ex.is_clean()).all(|ex| {
        ex.detail_command_available
            && ex
                .recovery_actions
                .contains(&M5RestrictedRecoveryAction::InspectTrust)
    }) && summaries().filter(|ex| ex.is_clean()).all(|ex| {
        ex.detail_command_available
            && ex
                .recovery_actions
                .contains(&M5RestrictedRecoveryAction::InspectTrust)
    });
    let recovery_consistent = rows()
        .filter(|ex| ex.is_clean())
        .all(|ex| ex.recovery_actions.first() == Some(&M5RestrictedRecoveryAction::InspectTrust))
        && summaries().filter(|ex| ex.is_clean()).all(|ex| {
            ex.recovery_actions.first() == Some(&M5RestrictedRecoveryAction::InspectTrust)
        });
    let still_safe_degrades = rows().any(|ex| {
        ex.degrade_reason == Some(M5RestrictedCapabilityRowDegradeReason::StillSafeActionsUnstated)
    }) || summaries().any(|ex| {
        ex.degrade_reason
            == Some(M5NarrowedCapabilitySummaryDegradeReason::StillSafeActionsUnstated)
    });
    let recovery_degrades = rows().any(|ex| {
        ex.degrade_reason == Some(M5RestrictedCapabilityRowDegradeReason::RecoveryPathMissing)
    }) || summaries().any(|ex| {
        ex.degrade_reason == Some(M5NarrowedCapabilitySummaryDegradeReason::RecoveryPathMissing)
    });
    if !(clean_names_still_safe
        && clean_exposes_recovery
        && recovery_consistent
        && still_safe_degrades
        && recovery_degrades)
    {
        violations.push(M5RestrictedCapabilityControlsViolation::StillSafeAndRecoveryNotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

fn string_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("password")
        || lower.contains("passphrase")
        || lower.contains("bearer ")
        || lower.contains("://")
        || lower.contains("-----begin")
}

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => string_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The component family this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5WorkspaceTrustRepairComponentFamily; 1] =
    [M5WorkspaceTrustRepairComponentFamily::RestrictedCapabilityRow];
