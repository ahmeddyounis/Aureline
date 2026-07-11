//! Implemented M5 workspace-trust-banner and root-trust-strip primitives.
//!
//! The frozen [workspace-trust / guided-repair component matrix][matrix] names the reusable trust
//! and repair UI components and locks their controlled vocabulary. This module is the first
//! implement lane over that matrix: it turns the two workspace-trust-facing components — the
//! **workspace-trust banner** and the **root-trust strip** — into resolvers that produce
//! export-safe, honest projections instead of prose buried in prompts or settings detail.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render the banner and strip with object identity, trust class, grant actor/source, policy
//!   epoch, and narrowed-capability summary.** [`resolve_workspace_trust_banner`] refuses to read
//!   as a clean, legible-at-a-glance banner when the object identity is unstated, the trust scope
//!   cannot be resolved, the grant source is undisclosed, a policy-managed grant hides its policy
//!   epoch, or a narrowed capability is left unnamed; it degrades instead.
//! * **Support mixed-root states without collapsing partially trusted workspaces into misleading
//!   fully trusted or fully restricted copy.** Both resolvers degrade to a collapse reason —
//!   [`M5WorkspaceTrustBannerDegradeReason::MixedRootCollapsedIntoUniform`] and
//!   [`M5RootTrustStripDegradeReason::PerRootTrustCollapsedIntoUniform`] — the moment a mixed-root
//!   banner or a per-root strip reads as uniform trust, and no clean example may collapse it.
//! * **Expose a command-backed path to inspect or reopen trust detail from every claimed consumer.**
//!   A clean banner or strip always offers [`M5WorkspaceTrustRootNextAction::OpenTrustDetail`] and
//!   degrades to a `TrustDetailPathMissing` reason whenever no command-backed detail entrypoint is
//!   reachable.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5WorkspaceTrustRepairDisposition`] trust / repair-disposition vocabulary, the
//! [`M5TrustScopeState`] trust-scope vocabulary, the [`M5TrustGrantSourceClass`] grant-source
//! vocabulary, the [`M5CapabilityNarrowState`] narrowed-capability vocabulary, and the
//! [`M5RootTrustState`] per-root trust vocabulary — so shell and workspace surfaces can never fork
//! their own trust, root, grant, or capability wording or invent feature-local badges.
//!
//! [matrix]: crate::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_workspace_trust_root_controls,
    seeded_m5_workspace_trust_root_controls_safe_mode_ui_preview_narrowed,
    seeded_m5_workspace_trust_root_controls_workspace_trust_ui_beta_narrowed,
    M5_WORKSPACE_TRUST_ROOT_CONTROLS_PACKET_ID,
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
    M5_ROOT_TRUST_STRIP_SCHEMA_REF, M5_WORKSPACE_TRUST_BANNER_SCHEMA_REF,
    M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF, M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5WorkspaceTrustRootControlsPacket`].
pub const M5_WORKSPACE_TRUST_ROOT_CONTROLS_RECORD_KIND: &str =
    "implement_m5_workspace_trust_banner_and_root_trust_strip_controls";

/// Schema version for M5 workspace-trust-banner / root-trust-strip controls records.
pub const M5_WORKSPACE_TRUST_ROOT_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_WORKSPACE_TRUST_ROOT_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-workspace-trust-banner-root-trust-strip-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_WORKSPACE_TRUST_ROOT_CONTROLS_DOC_REF: &str =
    "docs/trust/m5_workspace_trust_banner_and_root_trust_strip_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_WORKSPACE_TRUST_ROOT_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-workspace-trust-banner-root-trust-strip-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_WORKSPACE_TRUST_ROOT_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-workspace-trust-banner-root-trust-strip-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_WORKSPACE_TRUST_ROOT_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-workspace-trust-banner-root-trust-strip-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_WORKSPACE_TRUST_ROOT_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-workspace-trust-banner-root-trust-strip-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface
/// taxonomy so no lane invents a parallel surface set.
pub type M5WorkspaceTrustRootConsumerSurface = M5WorkspaceTrustRepairConsumerSurface;

/// One mandatory rendered part a workspace-trust banner or root-trust strip must be able to show, so
/// no trust truth is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustRootAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed trust disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The trusted object identity behind the banner (workspace-trust banner).
    ObjectIdentity,
    /// The trust class / scope named by the banner (workspace-trust banner).
    TrustClass,
    /// The grant actor / source behind the trust (both components).
    GrantSource,
    /// The policy epoch behind a managed grant (both components).
    PolicyEpoch,
    /// The narrowed-capability summary (workspace-trust banner).
    NarrowedCapability,
    /// The per-root trust state (root-trust strip).
    RootTrustState,
    /// The root identity a strip describes (root-trust strip).
    RootIdentity,
    /// The explicit mixed-root disclosure that keeps partial trust from reading uniform (both).
    MixedRootDisclosure,
    /// The command-backed path to inspect / reopen trust detail (both components).
    TrustDetailCommand,
}

impl M5WorkspaceTrustRootAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ObjectIdentity,
        Self::TrustClass,
        Self::GrantSource,
        Self::PolicyEpoch,
        Self::NarrowedCapability,
        Self::RootTrustState,
        Self::RootIdentity,
        Self::MixedRootDisclosure,
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
            Self::TrustClass => "trust_class",
            Self::GrantSource => "grant_source",
            Self::PolicyEpoch => "policy_epoch",
            Self::NarrowedCapability => "narrowed_capability",
            Self::RootTrustState => "root_trust_state",
            Self::RootIdentity => "root_identity",
            Self::MixedRootDisclosure => "mixed_root_disclosure",
            Self::TrustDetailCommand => "trust_detail_command",
        }
    }
}

/// Next safe action a component surfaces so a user is never left without a route into trust detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustRootNextAction {
    /// Open the command-backed trust-detail entrypoint.
    OpenTrustDetail,
    /// Review who granted the trust and under which policy epoch.
    ReviewGrantSource,
    /// Inspect the per-root trust breakdown.
    InspectRootTrust,
    /// Review the narrowed capability the current mode removes.
    ReviewNarrowedCapability,
    /// Review diagnostics for a stale or unresolved signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5WorkspaceTrustRootNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenTrustDetail,
        Self::ReviewGrantSource,
        Self::InspectRootTrust,
        Self::ReviewNarrowedCapability,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenTrustDetail => "open_trust_detail",
            Self::ReviewGrantSource => "review_grant_source",
            Self::InspectRootTrust => "inspect_root_trust",
            Self::ReviewNarrowedCapability => "review_narrowed_capability",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustRootExportField {
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
    /// The grant source named by the components.
    GrantSource,
    /// The policy epoch named by the components.
    PolicyEpoch,
    /// The trust scope named by the banner.
    TrustScope,
    /// The narrowed-capability state named by the banner.
    NarrowedCapability,
    /// The per-root trust state named by the strip.
    RootTrust,
    /// The accountable owner role.
    OwnerRole,
}

impl M5WorkspaceTrustRootExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::ComponentFamilies,
        Self::TrustDispositions,
        Self::DegradeReasons,
        Self::Qualification,
        Self::GrantSource,
        Self::PolicyEpoch,
        Self::TrustScope,
        Self::NarrowedCapability,
        Self::RootTrust,
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
            Self::GrantSource => "grant_source",
            Self::PolicyEpoch => "policy_epoch",
            Self::TrustScope => "trust_scope",
            Self::NarrowedCapability => "narrowed_capability",
            Self::RootTrust => "root_trust",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a workspace-trust banner degraded below a clean, legible-at-a-glance state. The
/// degrade-first ladder returns one of these instead of ever letting an ambiguous banner read as a
/// clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceTrustBannerDegradeReason {
    /// The trusted object identity is unstated; a user cannot tell what the trust applies to.
    ObjectIdentityUnstated,
    /// The trust scope cannot currently be resolved.
    TrustScopeUnresolved,
    /// The grant actor / source behind the trust is undisclosed.
    GrantSourceUnstated,
    /// A policy-managed grant hides its policy epoch.
    PolicyEpochUnstated,
    /// A narrowed capability is not named.
    NarrowedCapabilityUnstated,
    /// A mixed-root workspace reads as uniform (blanket) trust.
    MixedRootCollapsedIntoUniform,
    /// No command-backed trust-detail entrypoint is reachable.
    TrustDetailPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5WorkspaceTrustBannerDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ObjectIdentityUnstated,
        Self::TrustScopeUnresolved,
        Self::GrantSourceUnstated,
        Self::PolicyEpochUnstated,
        Self::NarrowedCapabilityUnstated,
        Self::MixedRootCollapsedIntoUniform,
        Self::TrustDetailPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObjectIdentityUnstated => "object_identity_unstated",
            Self::TrustScopeUnresolved => "trust_scope_unresolved",
            Self::GrantSourceUnstated => "grant_source_unstated",
            Self::PolicyEpochUnstated => "policy_epoch_unstated",
            Self::NarrowedCapabilityUnstated => "narrowed_capability_unstated",
            Self::MixedRootCollapsedIntoUniform => "mixed_root_collapsed_into_uniform",
            Self::TrustDetailPathMissing => "trust_detail_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5WorkspaceTrustRootNextAction {
        match self {
            Self::ObjectIdentityUnstated
            | Self::TrustScopeUnresolved
            | Self::TrustDetailPathMissing => M5WorkspaceTrustRootNextAction::OpenTrustDetail,
            Self::GrantSourceUnstated | Self::PolicyEpochUnstated => {
                M5WorkspaceTrustRootNextAction::ReviewGrantSource
            }
            Self::NarrowedCapabilityUnstated => {
                M5WorkspaceTrustRootNextAction::ReviewNarrowedCapability
            }
            Self::MixedRootCollapsedIntoUniform => M5WorkspaceTrustRootNextAction::InspectRootTrust,
            Self::ProofStale => M5WorkspaceTrustRootNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WorkspaceTrustRepairDowngradeTrigger {
        match self {
            Self::ObjectIdentityUnstated | Self::TrustScopeUnresolved => {
                M5WorkspaceTrustRepairDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::GrantSourceUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::GrantSourceUnstated
            }
            Self::PolicyEpochUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::PolicyEpochUnstated
            }
            Self::NarrowedCapabilityUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::NarrowedCapabilityUnstated
            }
            Self::MixedRootCollapsedIntoUniform => {
                M5WorkspaceTrustRepairDowngradeTrigger::MixedRootShownAsUniformTrust
            }
            Self::TrustDetailPathMissing => {
                M5WorkspaceTrustRepairDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5WorkspaceTrustRepairDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a root-trust strip degraded below a clean, fully-legible state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RootTrustStripDegradeReason {
    /// The root identity is unstated; a user cannot tell which root the strip describes.
    RootIdentityUnstated,
    /// The per-root trust state cannot currently be resolved.
    RootTrustUnresolved,
    /// The grant actor / source behind the root trust is undisclosed.
    GrantSourceUnstated,
    /// A policy-managed grant hides its policy epoch.
    PolicyEpochUnstated,
    /// A per-root trust reads as uniform with its siblings, hiding mixed-root trust.
    PerRootTrustCollapsedIntoUniform,
    /// No command-backed trust-detail entrypoint is reachable.
    TrustDetailPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5RootTrustStripDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::RootIdentityUnstated,
        Self::RootTrustUnresolved,
        Self::GrantSourceUnstated,
        Self::PolicyEpochUnstated,
        Self::PerRootTrustCollapsedIntoUniform,
        Self::TrustDetailPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RootIdentityUnstated => "root_identity_unstated",
            Self::RootTrustUnresolved => "root_trust_unresolved",
            Self::GrantSourceUnstated => "grant_source_unstated",
            Self::PolicyEpochUnstated => "policy_epoch_unstated",
            Self::PerRootTrustCollapsedIntoUniform => "per_root_trust_collapsed_into_uniform",
            Self::TrustDetailPathMissing => "trust_detail_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5WorkspaceTrustRootNextAction {
        match self {
            Self::RootIdentityUnstated
            | Self::RootTrustUnresolved
            | Self::PerRootTrustCollapsedIntoUniform => {
                M5WorkspaceTrustRootNextAction::InspectRootTrust
            }
            Self::GrantSourceUnstated | Self::PolicyEpochUnstated => {
                M5WorkspaceTrustRootNextAction::ReviewGrantSource
            }
            Self::TrustDetailPathMissing => M5WorkspaceTrustRootNextAction::OpenTrustDetail,
            Self::ProofStale => M5WorkspaceTrustRootNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WorkspaceTrustRepairDowngradeTrigger {
        match self {
            Self::RootIdentityUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::RootTrustUnresolved => {
                M5WorkspaceTrustRepairDowngradeTrigger::RootScopeCollapsedIntoBlanketTrust
            }
            Self::GrantSourceUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::GrantSourceUnstated
            }
            Self::PolicyEpochUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::PolicyEpochUnstated
            }
            Self::PerRootTrustCollapsedIntoUniform => {
                M5WorkspaceTrustRepairDowngradeTrigger::MixedRootShownAsUniformTrust
            }
            Self::TrustDetailPathMissing => {
                M5WorkspaceTrustRepairDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::ProofStale => M5WorkspaceTrustRepairDowngradeTrigger::ProofStale,
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

/// Maps a per-root trust state to the single controlled trust disposition, or `None` when the root
/// trust cannot be resolved.
fn disposition_for_root(state: M5RootTrustState) -> Option<M5WorkspaceTrustRepairDisposition> {
    use M5WorkspaceTrustRepairDisposition as D;
    match state {
        M5RootTrustState::RootTrusted | M5RootTrustState::RootInherited => Some(D::Trusted),
        M5RootTrustState::RootRestricted => Some(D::Restricted),
        M5RootTrustState::RootPolicyBlocked => Some(D::PolicyBlocked),
        M5RootTrustState::RootMixedChildren => Some(D::MixedRoot),
        M5RootTrustState::RootUnknown => None,
    }
}

/// Input to [`resolve_workspace_trust_banner`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WorkspaceTrustBannerResolutionInput {
    /// Stable identity of the banner instance.
    pub banner_id: String,
    /// The trusted object identity (workspace / root name); empty means unstated.
    pub object_identity: String,
    /// The trust scope of the workspace.
    pub trust_scope: M5TrustScopeState,
    /// Who granted the trust.
    pub grant_source: M5TrustGrantSourceClass,
    /// True when the grant actor / source is disclosed on the banner, never menu-only.
    pub grant_actor_stated: bool,
    /// The policy epoch behind a managed grant; empty means unstated.
    pub policy_epoch: String,
    /// The narrowed-capability state.
    pub capability_narrow: M5CapabilityNarrowState,
    /// True when the narrowed capability is named on the banner.
    pub capability_narrow_stated: bool,
    /// True when the banner reads as uniform (blanket) trust across roots.
    pub reads_as_uniform_trust: bool,
    /// True when a command-backed trust-detail entrypoint is reachable, never menu-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe workspace-trust banner projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedWorkspaceTrustBanner {
    /// Stable identity of the banner instance.
    pub banner_id: String,
    /// The trusted object identity named by the banner.
    pub object_identity: String,
    /// The trust-scope token named by the banner.
    pub trust_scope: String,
    /// Single controlled trust disposition, or `null` when the scope is unresolved.
    pub trust_disposition: Option<M5WorkspaceTrustRepairDisposition>,
    /// The grant-source token named by the banner.
    pub grant_source: String,
    /// The policy epoch named by the banner.
    pub policy_epoch: String,
    /// The narrowed-capability token named by the banner.
    pub capability_narrow: String,
    /// Whether any capability is narrowed relative to full trust.
    pub capability_narrowed: bool,
    /// Whether this banner describes a mixed-root workspace.
    pub is_mixed_root: bool,
    /// Guardrail (MUST be `false` on a clean banner): a mixed-root workspace reads as uniform trust.
    pub collapses_mixed_root_into_uniform: bool,
    /// Whether a command-backed trust-detail entrypoint is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the banner could not read as a clean, legible-at-a-glance state.
    pub degrade_reason: Option<M5WorkspaceTrustBannerDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5WorkspaceTrustRootNextAction,
    /// Whether the trust state is legible at a glance (clean banner naming every fact).
    pub legible_at_a_glance: bool,
}

impl M5ResolvedWorkspaceTrustBanner {
    /// Whether this banner reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_root_trust_strip`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5RootTrustStripResolutionInput {
    /// Stable identity of the strip instance.
    pub strip_id: String,
    /// The root identity the strip describes; empty means unstated.
    pub root_identity: String,
    /// The per-root trust state.
    pub root_trust: M5RootTrustState,
    /// Who granted the root trust.
    pub grant_source: M5TrustGrantSourceClass,
    /// True when the grant actor / source is disclosed on the strip.
    pub grant_actor_stated: bool,
    /// The policy epoch behind a managed grant; empty means unstated.
    pub policy_epoch: String,
    /// True when this root is one of several with differing trust in a mixed-root workspace.
    pub part_of_mixed_root: bool,
    /// True when the strip reads this root as uniform with its siblings.
    pub reads_as_uniform_with_siblings: bool,
    /// True when a command-backed trust-detail entrypoint is reachable.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe root-trust strip projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedRootTrustStrip {
    /// Stable identity of the strip instance.
    pub strip_id: String,
    /// The root identity named by the strip.
    pub root_identity: String,
    /// The per-root trust token named by the strip.
    pub root_trust: String,
    /// Single controlled trust disposition, or `null` when the root trust is unresolved.
    pub trust_disposition: Option<M5WorkspaceTrustRepairDisposition>,
    /// The grant-source token named by the strip.
    pub grant_source: String,
    /// The policy epoch named by the strip.
    pub policy_epoch: String,
    /// Whether this root is one of several with differing trust.
    pub part_of_mixed_root: bool,
    /// Guardrail (MUST be `false` on a clean strip): a per-root trust reads as uniform with siblings.
    pub collapses_per_root_into_uniform: bool,
    /// Whether a command-backed trust-detail entrypoint is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the strip could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5RootTrustStripDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5WorkspaceTrustRootNextAction,
    /// Whether the per-root trust stays explicit (never collapsed into uniform trust).
    pub per_root_trust_explicit: bool,
}

impl M5ResolvedRootTrustStrip {
    /// Whether this strip reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5WorkspaceTrustRootResolutionError {
    /// The banner id was empty.
    EmptyBannerId,
    /// The strip id was empty.
    EmptyStripId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5WorkspaceTrustRootResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyBannerId => "empty_banner_id",
            Self::EmptyStripId => "empty_strip_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5WorkspaceTrustRootResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 workspace-trust-root resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5WorkspaceTrustRootResolutionError {}

/// True when the grant source is resolved: a concrete grant class that is disclosed on the surface.
fn grant_is_resolved(source: M5TrustGrantSourceClass, actor_stated: bool) -> bool {
    !matches!(source, M5TrustGrantSourceClass::GrantSourceUnknown) && actor_stated
}

/// Resolves a workspace-trust banner, making trust state legible at a glance: the banner names its
/// object identity, trust class, grant source, policy epoch, and narrowed-capability summary, never
/// reads a mixed-root workspace as uniform trust, and always exposes a command-backed detail path.
pub fn resolve_workspace_trust_banner(
    input: M5WorkspaceTrustBannerResolutionInput,
) -> Result<M5ResolvedWorkspaceTrustBanner, M5WorkspaceTrustRootResolutionError> {
    if input.banner_id.trim().is_empty() {
        return Err(M5WorkspaceTrustRootResolutionError::EmptyBannerId);
    }
    if string_is_forbidden(&input.banner_id)
        || string_is_forbidden(&input.object_identity)
        || string_is_forbidden(&input.policy_epoch)
    {
        return Err(M5WorkspaceTrustRootResolutionError::ForbiddenMaterial);
    }

    let is_mixed_root = matches!(input.trust_scope, M5TrustScopeState::MixedRoot);
    let capability_narrowed = !matches!(
        input.capability_narrow,
        M5CapabilityNarrowState::FullCapability
    );
    let grant_resolved = grant_is_resolved(input.grant_source, input.grant_actor_stated);
    let policy_epoch_required =
        matches!(input.grant_source, M5TrustGrantSourceClass::PolicyManaged);
    let collapses_mixed_root_into_uniform = is_mixed_root && input.reads_as_uniform_trust;

    let degrade_reason = if input.object_identity.trim().is_empty() {
        Some(M5WorkspaceTrustBannerDegradeReason::ObjectIdentityUnstated)
    } else if matches!(input.trust_scope, M5TrustScopeState::ScopeUnknown) {
        Some(M5WorkspaceTrustBannerDegradeReason::TrustScopeUnresolved)
    } else if !grant_resolved {
        Some(M5WorkspaceTrustBannerDegradeReason::GrantSourceUnstated)
    } else if policy_epoch_required && input.policy_epoch.trim().is_empty() {
        Some(M5WorkspaceTrustBannerDegradeReason::PolicyEpochUnstated)
    } else if capability_narrowed && !input.capability_narrow_stated {
        Some(M5WorkspaceTrustBannerDegradeReason::NarrowedCapabilityUnstated)
    } else if collapses_mixed_root_into_uniform {
        Some(M5WorkspaceTrustBannerDegradeReason::MixedRootCollapsedIntoUniform)
    } else if !input.detail_command_available {
        Some(M5WorkspaceTrustBannerDegradeReason::TrustDetailPathMissing)
    } else if !input.proof_fresh {
        Some(M5WorkspaceTrustBannerDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5WorkspaceTrustRootNextAction::OpenTrustDetail,
    };

    Ok(M5ResolvedWorkspaceTrustBanner {
        banner_id: input.banner_id,
        object_identity: input.object_identity,
        trust_scope: input.trust_scope.as_str().to_owned(),
        trust_disposition: disposition_for_scope(input.trust_scope),
        grant_source: input.grant_source.as_str().to_owned(),
        policy_epoch: input.policy_epoch,
        capability_narrow: input.capability_narrow.as_str().to_owned(),
        capability_narrowed,
        is_mixed_root,
        collapses_mixed_root_into_uniform,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        legible_at_a_glance: degrade_reason.is_none(),
    })
}

/// Resolves a root-trust strip, proving per-root trust stays explicit: the strip names its root
/// identity, per-root trust, grant source, and policy epoch, never reads a root as uniform with its
/// siblings in a mixed-root workspace, and always exposes a command-backed detail path.
pub fn resolve_root_trust_strip(
    input: M5RootTrustStripResolutionInput,
) -> Result<M5ResolvedRootTrustStrip, M5WorkspaceTrustRootResolutionError> {
    if input.strip_id.trim().is_empty() {
        return Err(M5WorkspaceTrustRootResolutionError::EmptyStripId);
    }
    if string_is_forbidden(&input.strip_id)
        || string_is_forbidden(&input.root_identity)
        || string_is_forbidden(&input.policy_epoch)
    {
        return Err(M5WorkspaceTrustRootResolutionError::ForbiddenMaterial);
    }

    let grant_resolved = grant_is_resolved(input.grant_source, input.grant_actor_stated);
    let policy_epoch_required =
        matches!(input.grant_source, M5TrustGrantSourceClass::PolicyManaged);
    let collapses_per_root_into_uniform =
        input.part_of_mixed_root && input.reads_as_uniform_with_siblings;

    let degrade_reason = if input.root_identity.trim().is_empty() {
        Some(M5RootTrustStripDegradeReason::RootIdentityUnstated)
    } else if matches!(input.root_trust, M5RootTrustState::RootUnknown) {
        Some(M5RootTrustStripDegradeReason::RootTrustUnresolved)
    } else if !grant_resolved {
        Some(M5RootTrustStripDegradeReason::GrantSourceUnstated)
    } else if policy_epoch_required && input.policy_epoch.trim().is_empty() {
        Some(M5RootTrustStripDegradeReason::PolicyEpochUnstated)
    } else if collapses_per_root_into_uniform {
        Some(M5RootTrustStripDegradeReason::PerRootTrustCollapsedIntoUniform)
    } else if !input.detail_command_available {
        Some(M5RootTrustStripDegradeReason::TrustDetailPathMissing)
    } else if !input.proof_fresh {
        Some(M5RootTrustStripDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5WorkspaceTrustRootNextAction::OpenTrustDetail,
    };

    Ok(M5ResolvedRootTrustStrip {
        strip_id: input.strip_id,
        root_identity: input.root_identity,
        root_trust: input.root_trust.as_str().to_owned(),
        trust_disposition: disposition_for_root(input.root_trust),
        grant_source: input.grant_source.as_str().to_owned(),
        policy_epoch: input.policy_epoch,
        part_of_mixed_root: input.part_of_mixed_root,
        collapses_per_root_into_uniform,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        per_root_trust_explicit: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved workspace-trust banner and
/// root-trust strip examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceTrustRootControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5WorkspaceTrustRootConsumerSurface,
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
    pub anatomy_parts: Vec<M5WorkspaceTrustRootAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5WorkspaceTrustRootExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5WorkspaceTrustRepairDowngradeTrigger>,
    /// Resolved workspace-trust banner examples.
    pub workspace_trust_banner_examples: Vec<M5ResolvedWorkspaceTrustBanner>,
    /// Resolved root-trust strip examples.
    pub root_trust_strip_examples: Vec<M5ResolvedRootTrustStrip>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never imply blanket trust across roots, profiles, or routes.
    pub implies_blanket_trust_across_roots_or_routes: bool,
    /// Hard invariant: never hide the grant source or policy epoch behind menus only.
    pub hides_grant_source_or_policy_epoch_in_menus_only: bool,
    /// Hard invariant: never collapse mixed-root trust into misleading uniform trust.
    pub collapses_mixed_root_into_uniform_trust: bool,
    /// Hard invariant: never hide a narrowed capability behind generic "reduced mode" chrome.
    pub hides_narrowed_capability_behind_generic_chrome: bool,
}

impl M5WorkspaceTrustRootControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5WorkspaceTrustRootAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5WorkspaceTrustRootAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5WorkspaceTrustRootExportField> =
            self.export_fields.iter().copied().collect();
        M5WorkspaceTrustRootExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.implies_blanket_trust_across_roots_or_routes
            && !self.hides_grant_source_or_policy_epoch_in_menus_only
            && !self.collapses_mixed_root_into_uniform_trust
            && !self.hides_narrowed_capability_behind_generic_chrome
    }

    /// True when every resolved example on this row is honest: no clean banner or strip collapses
    /// mixed-root trust into uniform trust and no clean example hides the trust-detail path.
    fn examples_are_honest(&self) -> bool {
        self.workspace_trust_banner_examples.iter().all(|ex| {
            !(ex.is_clean()
                && (ex.collapses_mixed_root_into_uniform || !ex.detail_command_available))
        }) && self.root_trust_strip_examples.iter().all(|ex| {
            !(ex.is_clean() && (ex.collapses_per_root_into_uniform || !ex.detail_command_available))
        })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceTrustRootVocabularySet {
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
    /// Banner degrade-reason tokens.
    pub banner_degrade_reasons: Vec<String>,
    /// Strip degrade-reason tokens.
    pub strip_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5WorkspaceTrustRootVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            trust_dispositions: tokens(&M5WorkspaceTrustRepairDisposition::ALL, |v| v.as_str()),
            trust_scopes: tokens(&M5TrustScopeState::ALL, |v| v.as_str()),
            grant_sources: tokens(&M5TrustGrantSourceClass::ALL, |v| v.as_str()),
            capability_narrow_states: tokens(&M5CapabilityNarrowState::ALL, |v| v.as_str()),
            root_trust_states: tokens(&M5RootTrustState::ALL, |v| v.as_str()),
            banner_degrade_reasons: tokens(&M5WorkspaceTrustBannerDegradeReason::ALL, |v| {
                v.as_str()
            }),
            strip_degrade_reasons: tokens(&M5RootTrustStripDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5WorkspaceTrustRootAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5WorkspaceTrustRootNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5WorkspaceTrustRootExportField::ALL, |v| v.as_str()),
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
pub struct M5WorkspaceTrustRootGovernanceReview {
    /// The banner always names its object identity and trust class.
    pub banner_names_object_identity_and_trust_class: bool,
    /// The banner always names its grant source and policy epoch.
    pub banner_names_grant_source_and_policy_epoch: bool,
    /// The root-trust strip always names its per-root trust.
    pub root_strip_names_per_root_trust: bool,
    /// Mixed-root trust is always explicit and never reads as uniform.
    pub mixed_root_always_explicit_never_uniform: bool,
    /// A narrowed capability is always named, never left as vague reduced-mode chrome.
    pub narrowed_capability_always_named: bool,
    /// A command-backed trust-detail entrypoint is always reachable.
    pub trust_detail_command_always_reachable: bool,
    /// Shell and workspace surfaces share one trust / root vocabulary.
    pub trust_vocabulary_shared_across_shell_and_workspace: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceTrustRootConsumerProjection {
    /// Shell surfaces consume the shared trust-scope vocabulary.
    pub shell_surfaces_consume_trust_scope_vocabulary: bool,
    /// Workspace surfaces consume the shared per-root trust vocabulary.
    pub workspace_surfaces_consume_root_trust_vocabulary: bool,
    /// Trust state traces back to one canonical component contract.
    pub trust_detail_traces_to_single_component_contract: bool,
    /// Support / export reads a single canonical trust source.
    pub support_export_reads_single_trust_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceTrustRootProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceTrustRootReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5WorkspaceTrustRootControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WorkspaceTrustRootControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5WorkspaceTrustRootControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WorkspaceTrustRootVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WorkspaceTrustRootGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WorkspaceTrustRootConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WorkspaceTrustRootProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WorkspaceTrustRootReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 workspace-trust-banner / root-trust-strip controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceTrustRootControlsPacket {
    /// Record kind; must equal [`M5_WORKSPACE_TRUST_ROOT_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_WORKSPACE_TRUST_ROOT_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5WorkspaceTrustRootControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WorkspaceTrustRootVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WorkspaceTrustRootGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WorkspaceTrustRootConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WorkspaceTrustRootProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WorkspaceTrustRootReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5WorkspaceTrustRootControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5WorkspaceTrustRootControlsPacketInput) -> Self {
        Self {
            record_kind: M5_WORKSPACE_TRUST_ROOT_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_WORKSPACE_TRUST_ROOT_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5WorkspaceTrustRootControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_WORKSPACE_TRUST_ROOT_CONTROLS_RECORD_KIND {
            violations.push(M5WorkspaceTrustRootControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_WORKSPACE_TRUST_ROOT_CONTROLS_SCHEMA_VERSION {
            violations.push(M5WorkspaceTrustRootControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5WorkspaceTrustRootControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5WorkspaceTrustRootControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 workspace-trust-root controls packet serializes"),
        ) {
            violations.push(M5WorkspaceTrustRootControlsViolation::RawMaterialInExport);
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
            .expect("m5 workspace-trust-root controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,banner_examples,strip_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .workspace_trust_banner_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.root_trust_strip_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.workspace_trust_banner_examples.len(),
                row.root_trust_strip_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Workspace-Trust-Banner and Root-Trust-Strip Controls\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.controls_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.controls_rows.len()
        ));
        out.push_str(&format!(
            "- Trust scopes: {}\n",
            self.vocabulary_set.trust_scopes.join(", ")
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
                "  - Banner examples: {} / strip examples: {}\n",
                row.workspace_trust_banner_examples.len(),
                row.root_trust_strip_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5WorkspaceTrustRootControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5WorkspaceTrustRootControlsViolation>),
}

impl fmt::Display for M5WorkspaceTrustRootControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 workspace-trust-root controls export parse failed: {error}"
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
                    "m5 workspace-trust-root controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5WorkspaceTrustRootControlsArtifactError {}

/// Validation failures emitted by [`M5WorkspaceTrustRootControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5WorkspaceTrustRootControlsViolation {
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
    /// A controls row does not point at both component schemas.
    ComponentSchemaRefMissing,
    /// A controls row carries no resolved examples.
    ExamplesMissing,
    /// A controls row carries a dishonest clean example (collapse or hidden detail path).
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
    /// Mixed-root honesty is not proven: clean examples do not cover mixed-root trust, or no
    /// collapse-into-uniform example degrades.
    MixedRootHonestyNotProven,
    /// Trust traceability is not proven: clean examples do not expose the command-backed detail
    /// entrypoint, or no missing-detail example degrades.
    TrustTraceabilityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5WorkspaceTrustRootControlsViolation {
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
            Self::MixedRootHonestyNotProven => "mixed_root_honesty_not_proven",
            Self::TrustTraceabilityNotProven => "trust_traceability_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_workspace_trust_root_controls_export(
) -> Result<M5WorkspaceTrustRootControlsPacket, M5WorkspaceTrustRootControlsArtifactError> {
    let packet: M5WorkspaceTrustRootControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-workspace-trust-banner-root-trust-strip-controls-proof/support_export.json"
    )))
    .map_err(M5WorkspaceTrustRootControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5WorkspaceTrustRootControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5WorkspaceTrustRootControlsPacket,
    violations: &mut Vec<M5WorkspaceTrustRootControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_WORKSPACE_TRUST_ROOT_CONTROLS_SCHEMA_REF,
        M5_WORKSPACE_TRUST_ROOT_CONTROLS_DOC_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
        M5_WORKSPACE_TRUST_BANNER_SCHEMA_REF,
        M5_ROOT_TRUST_STRIP_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5WorkspaceTrustRootControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5WorkspaceTrustRootControlsPacket,
    violations: &mut Vec<M5WorkspaceTrustRootControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5WorkspaceTrustRootControlsViolation::NoControlsRows);
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
            violations.push(M5WorkspaceTrustRootControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5WorkspaceTrustRootControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5WorkspaceTrustRootControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_WORKSPACE_TRUST_BANNER_SCHEMA_REF)
            || !refs.contains(M5_ROOT_TRUST_STRIP_SCHEMA_REF)
        {
            violations.push(M5WorkspaceTrustRootControlsViolation::ComponentSchemaRefMissing);
        }
        if row.workspace_trust_banner_examples.is_empty()
            || row.root_trust_strip_examples.is_empty()
        {
            violations.push(M5WorkspaceTrustRootControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5WorkspaceTrustRootControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5WorkspaceTrustRootControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5WorkspaceTrustRootControlsPacket,
    violations: &mut Vec<M5WorkspaceTrustRootControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.banner_names_object_identity_and_trust_class,
        review.banner_names_grant_source_and_policy_epoch,
        review.root_strip_names_per_root_trust,
        review.mixed_root_always_explicit_never_uniform,
        review.narrowed_capability_always_named,
        review.trust_detail_command_always_reachable,
        review.trust_vocabulary_shared_across_shell_and_workspace,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5WorkspaceTrustRootControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5WorkspaceTrustRootControlsPacket,
    violations: &mut Vec<M5WorkspaceTrustRootControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_surfaces_consume_trust_scope_vocabulary,
        projection.workspace_surfaces_consume_root_trust_vocabulary,
        projection.trust_detail_traces_to_single_component_contract,
        projection.support_export_reads_single_trust_source,
    ] {
        if !ok {
            violations.push(M5WorkspaceTrustRootControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5WorkspaceTrustRootControlsPacket,
    violations: &mut Vec<M5WorkspaceTrustRootControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5WorkspaceTrustRootControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5WorkspaceTrustRootControlsPacket,
    violations: &mut Vec<M5WorkspaceTrustRootControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5WorkspaceTrustRootControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5WorkspaceTrustRootControlsPacket,
    violations: &mut Vec<M5WorkspaceTrustRootControlsViolation>,
) {
    let banners = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.workspace_trust_banner_examples.iter())
    };
    let strips = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.root_trust_strip_examples.iter())
    };

    // AC: mixed-root trust stays explicit. A clean banner covers a mixed-root workspace, a clean
    // strip covers a root with mixed children, and both a banner and a strip degrade when their
    // mixed-root trust is collapsed into uniform trust — with no clean example collapsing it.
    let clean_scopes: BTreeSet<&str> = banners()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.trust_scope.as_str())
        .collect();
    let covers_required_scopes = [
        M5TrustScopeState::TrustedWorkspace,
        M5TrustScopeState::RestrictedWorkspace,
        M5TrustScopeState::MixedRoot,
        M5TrustScopeState::PolicyBlocked,
    ]
    .iter()
    .all(|scope| clean_scopes.contains(scope.as_str()));
    let clean_mixed_root_strip = strips()
        .any(|ex| ex.is_clean() && ex.root_trust == M5RootTrustState::RootMixedChildren.as_str());
    let banner_collapse_degrades = banners().any(|ex| {
        ex.degrade_reason
            == Some(M5WorkspaceTrustBannerDegradeReason::MixedRootCollapsedIntoUniform)
            && ex.collapses_mixed_root_into_uniform
    });
    let strip_collapse_degrades = strips().any(|ex| {
        ex.degrade_reason == Some(M5RootTrustStripDegradeReason::PerRootTrustCollapsedIntoUniform)
            && ex.collapses_per_root_into_uniform
    });
    let no_clean_collapse = banners()
        .all(|ex| !(ex.is_clean() && ex.collapses_mixed_root_into_uniform))
        && strips().all(|ex| !(ex.is_clean() && ex.collapses_per_root_into_uniform));
    if !(covers_required_scopes
        && clean_mixed_root_strip
        && banner_collapse_degrades
        && strip_collapse_degrades
        && no_clean_collapse)
    {
        violations.push(M5WorkspaceTrustRootControlsViolation::MixedRootHonestyNotProven);
    }

    // AC: trust state traces to one canonical component contract and one command-backed detail
    // entrypoint. Every clean example exposes the detail command, at least one missing-detail
    // example degrades, and no clean example hides the detail path.
    let missing_detail_degrades = banners().any(|ex| {
        ex.degrade_reason == Some(M5WorkspaceTrustBannerDegradeReason::TrustDetailPathMissing)
    }) || strips()
        .any(|ex| ex.degrade_reason == Some(M5RootTrustStripDegradeReason::TrustDetailPathMissing));
    let clean_exposes_detail = banners().all(|ex| !ex.is_clean() || ex.detail_command_available)
        && strips().all(|ex| !ex.is_clean() || ex.detail_command_available);
    if !(missing_detail_degrades && clean_exposes_detail) {
        violations.push(M5WorkspaceTrustRootControlsViolation::TrustTraceabilityNotProven);
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

/// The two component families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5WorkspaceTrustRepairComponentFamily; 2] = [
    M5WorkspaceTrustRepairComponentFamily::WorkspaceTrustBanner,
    M5WorkspaceTrustRepairComponentFamily::RootTrustStrip,
];
