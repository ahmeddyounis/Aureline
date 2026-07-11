//! Implemented M5 trust-fact-grid and trust-elevation-sheet primitives.
//!
//! The frozen [workspace-trust / guided-repair component matrix][matrix] names the reusable trust
//! and repair UI components and locks their controlled vocabulary. This module is the second
//! implement lane over that matrix: it turns the two trust-review components — the **trust-fact
//! grid** and the **trust-elevation sheet** — into resolvers that produce export-safe, honest
//! projections, so a trust elevation is a reviewed fact sheet a user can inspect before approving
//! rather than a one-off confirmation prompt.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render trust-fact grids and elevation sheets with actor / object / scope, policy source,
//!   capability deltas, a reduced-mode alternative, and lasting-versus-one-time effect.**
//!   [`resolve_trust_fact_grid`] refuses to read as a clean grid when the actor or object identity
//!   is unstated, the trust scope cannot be resolved, the policy source (grant source / policy
//!   epoch) is undisclosed, or a narrowed capability is unnamed; it degrades instead.
//!   [`resolve_trust_elevation_sheet`] additionally refuses to read as reviewed-before-approval
//!   unless it names the reduced-mode alternative (what still works without trust) and the effect
//!   duration (lasting versus one-time).
//! * **Show what still works without trust and what changes if trust is granted.** An elevation
//!   sheet degrades to [`M5TrustElevationSheetDegradeReason::ReducedModeAlternativeUnstated`] the
//!   moment it hides the reduced-mode path and to
//!   [`M5TrustElevationSheetDegradeReason::CapabilityDeltaUnstated`] when it hides the capability
//!   delta a grant would change.
//! * **Prevent approval copy from implying broader scope than the actual object being trusted.** An
//!   elevation sheet degrades to [`M5TrustElevationSheetDegradeReason::AmbientScopeImplied`] the
//!   moment its copy implies an ambient or inherited grant beyond the reviewed object and scope, and
//!   no clean sheet may imply it.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the single controlled
//! [`M5WorkspaceTrustRepairDisposition`] trust / repair-disposition vocabulary, the
//! [`M5TrustScopeState`] trust-scope vocabulary, the [`M5TrustGrantSourceClass`] grant-source
//! vocabulary, the [`M5CapabilityNarrowState`] narrowed-capability vocabulary, and the
//! [`M5RootTrustState`] per-root trust vocabulary — so every claimed M5 trust prompt exposes the
//! same fields, delta grammar, and reduced-mode path instead of forking its own wording.
//!
//! [matrix]: crate::freeze_the_m5_workspace_trust_banner_trust_fact_grid_trust_elevation_sheet_restricted_capability_row_root_trust_strip_repair_transaction_preview_card_rollback_class_strip_and_repair_result_receipt_row_component_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_trust_fact_grid_elevation_controls,
    seeded_m5_trust_fact_grid_elevation_controls_safe_mode_ui_preview_narrowed,
    seeded_m5_trust_fact_grid_elevation_controls_workspace_trust_ui_beta_narrowed,
    M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_PACKET_ID,
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
    M5_TRUST_ELEVATION_SHEET_SCHEMA_REF, M5_TRUST_FACT_GRID_SCHEMA_REF,
    M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF, M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5TrustFactGridElevationControlsPacket`].
pub const M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_RECORD_KIND: &str =
    "implement_m5_trust_fact_grid_and_trust_elevation_sheet_controls";

/// Schema version for M5 trust-fact-grid / trust-elevation-sheet controls records.
pub const M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined controls schema.
pub const M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-trust-fact-grid-trust-elevation-sheet-controls.schema.json";

/// Repo-relative path of the controls doc.
pub const M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_DOC_REF: &str =
    "docs/trust/m5_trust_fact_grid_and_trust_elevation_sheet_controls.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-trust-fact-grid-trust-elevation-sheet-controls-proof/support_export.json";

/// Repo-relative path of the checked machine-readable controls CSV.
pub const M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_CSV_REF: &str =
    "artifacts/release/m5-trust-fact-grid-trust-elevation-sheet-controls-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_REPORT_REF: &str =
    "artifacts/release/m5-trust-fact-grid-trust-elevation-sheet-controls-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-trust-fact-grid-trust-elevation-sheet-controls";

/// Consumer surface a controls row projects onto. Reuses the frozen matrix consumer-surface
/// taxonomy so no lane invents a parallel surface set.
pub type M5TrustFactGridElevationConsumerSurface = M5WorkspaceTrustRepairConsumerSurface;

/// Controlled effect-duration class for a trust elevation — whether granting the trust lasts until
/// revoked, applies once, or is scoped to a single action, so a one-time grant never reads as a
/// lasting one and the "lasting versus one-time" fact is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustElevationEffectClass {
    /// The grant persists across sessions until it is explicitly revoked.
    LastingUntilRevoked,
    /// The grant applies to this session only.
    OneTimeThisSession,
    /// The grant applies to a single action and is dropped afterwards.
    SingleActionOnly,
    /// The effect duration cannot currently be resolved.
    EffectUnknown,
}

impl M5TrustElevationEffectClass {
    /// Every effect class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LastingUntilRevoked,
        Self::OneTimeThisSession,
        Self::SingleActionOnly,
        Self::EffectUnknown,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LastingUntilRevoked => "lasting_until_revoked",
            Self::OneTimeThisSession => "one_time_this_session",
            Self::SingleActionOnly => "single_action_only",
            Self::EffectUnknown => "effect_unknown",
        }
    }

    /// Whether the grant is lasting rather than one-time / single-action.
    pub const fn is_lasting(self) -> bool {
        matches!(self, Self::LastingUntilRevoked)
    }

    /// Whether the effect duration is resolved (named on the sheet).
    pub const fn is_stated(self) -> bool {
        !matches!(self, Self::EffectUnknown)
    }
}

/// One mandatory rendered part a trust-fact grid or trust-elevation sheet must be able to show, so
/// no trust fact is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustFactGridElevationAnatomyPart {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed trust disposition.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The actor requesting or granting the trust (both components).
    ActorIdentity,
    /// The trusted object identity behind the grid or sheet (both components).
    ObjectIdentity,
    /// The trust class / scope named by the component (both components).
    TrustScope,
    /// The policy source — grant source and policy epoch (both components).
    PolicySource,
    /// The capability delta — what a grant changes / what is narrowed (both components).
    CapabilityDelta,
    /// The reduced-mode alternative — what still works without trust (elevation sheet).
    ReducedModeAlternative,
    /// The lasting-versus-one-time effect of a grant (elevation sheet).
    EffectDuration,
    /// The per-root trust breakdown so mixed-root trust stays explicit (trust-fact grid).
    PerRootTrust,
    /// The command-backed path to inspect scope and source before approval (both components).
    TrustDetailCommand,
}

impl M5TrustFactGridElevationAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::ActorIdentity,
        Self::ObjectIdentity,
        Self::TrustScope,
        Self::PolicySource,
        Self::CapabilityDelta,
        Self::ReducedModeAlternative,
        Self::EffectDuration,
        Self::PerRootTrust,
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
            Self::ActorIdentity => "actor_identity",
            Self::ObjectIdentity => "object_identity",
            Self::TrustScope => "trust_scope",
            Self::PolicySource => "policy_source",
            Self::CapabilityDelta => "capability_delta",
            Self::ReducedModeAlternative => "reduced_mode_alternative",
            Self::EffectDuration => "effect_duration",
            Self::PerRootTrust => "per_root_trust",
            Self::TrustDetailCommand => "trust_detail_command",
        }
    }
}

/// Next safe action a component surfaces so a user can inspect scope and source before approval and
/// is never left without a route into trust detail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustFactGridElevationNextAction {
    /// Open the command-backed trust-detail entrypoint to inspect scope and source in place.
    OpenTrustDetail,
    /// Review the policy source — who granted the trust and under which policy epoch.
    ReviewPolicySource,
    /// Review the capability delta a grant would change.
    ReviewCapabilityDelta,
    /// Continue in reduced mode — the alternative to granting trust.
    ContinueInReducedMode,
    /// Review diagnostics for a stale or unresolved signal.
    ReviewDiagnostics,
    /// No action is needed; the component is clean.
    NoActionNeeded,
}

impl M5TrustFactGridElevationNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OpenTrustDetail,
        Self::ReviewPolicySource,
        Self::ReviewCapabilityDelta,
        Self::ContinueInReducedMode,
        Self::ReviewDiagnostics,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenTrustDetail => "open_trust_detail",
            Self::ReviewPolicySource => "review_policy_source",
            Self::ReviewCapabilityDelta => "review_capability_delta",
            Self::ContinueInReducedMode => "continue_in_reduced_mode",
            Self::ReviewDiagnostics => "review_diagnostics",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a controls row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustFactGridElevationExportField {
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
    /// The trust scope named by the components.
    TrustScope,
    /// The capability delta named by the components.
    CapabilityDelta,
    /// The lasting-versus-one-time effect duration named by the sheet.
    EffectDuration,
    /// The accountable owner role.
    OwnerRole,
}

impl M5TrustFactGridElevationExportField {
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
        Self::CapabilityDelta,
        Self::EffectDuration,
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
            Self::CapabilityDelta => "capability_delta",
            Self::EffectDuration => "effect_duration",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a trust-fact grid degraded below a clean, fully-legible state. The degrade-first ladder
/// returns one of these instead of ever letting an ambiguous grid read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustFactGridDegradeReason {
    /// The trusted object identity is unstated; a user cannot tell what the trust applies to.
    ObjectIdentityUnstated,
    /// The actor requesting or granting the trust is unstated.
    ActorIdentityUnstated,
    /// The trust scope cannot currently be resolved.
    TrustScopeUnresolved,
    /// The grant actor / source (policy source) is undisclosed.
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

impl M5TrustFactGridDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ObjectIdentityUnstated,
        Self::ActorIdentityUnstated,
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
            Self::ActorIdentityUnstated => "actor_identity_unstated",
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
    pub const fn next_action(self) -> M5TrustFactGridElevationNextAction {
        match self {
            Self::ObjectIdentityUnstated
            | Self::ActorIdentityUnstated
            | Self::TrustScopeUnresolved
            | Self::MixedRootCollapsedIntoUniform
            | Self::TrustDetailPathMissing => M5TrustFactGridElevationNextAction::OpenTrustDetail,
            Self::GrantSourceUnstated | Self::PolicyEpochUnstated => {
                M5TrustFactGridElevationNextAction::ReviewPolicySource
            }
            Self::NarrowedCapabilityUnstated => {
                M5TrustFactGridElevationNextAction::ReviewCapabilityDelta
            }
            Self::ProofStale => M5TrustFactGridElevationNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WorkspaceTrustRepairDowngradeTrigger {
        match self {
            Self::ObjectIdentityUnstated
            | Self::ActorIdentityUnstated
            | Self::TrustScopeUnresolved
            | Self::TrustDetailPathMissing => {
                M5WorkspaceTrustRepairDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::GrantSourceUnstated => M5WorkspaceTrustRepairDowngradeTrigger::GrantSourceUnstated,
            Self::PolicyEpochUnstated => M5WorkspaceTrustRepairDowngradeTrigger::PolicyEpochUnstated,
            Self::NarrowedCapabilityUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::NarrowedCapabilityUnstated
            }
            Self::MixedRootCollapsedIntoUniform => {
                M5WorkspaceTrustRepairDowngradeTrigger::MixedRootShownAsUniformTrust
            }
            Self::ProofStale => M5WorkspaceTrustRepairDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a trust-elevation sheet degraded below a clean, reviewed-before-approval state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustElevationSheetDegradeReason {
    /// The trusted object identity is unstated; a user cannot tell what the elevation applies to.
    ObjectIdentityUnstated,
    /// The actor requesting or granting the trust is unstated.
    ActorIdentityUnstated,
    /// The trust scope cannot currently be resolved.
    TrustScopeUnresolved,
    /// The grant actor / source (policy source) is undisclosed.
    GrantSourceUnstated,
    /// A policy-managed grant hides its policy epoch.
    PolicyEpochUnstated,
    /// The capability delta a grant would change is not named.
    CapabilityDeltaUnstated,
    /// The reduced-mode alternative — what still works without trust — is not named.
    ReducedModeAlternativeUnstated,
    /// The lasting-versus-one-time effect duration is not named.
    EffectDurationUnstated,
    /// The approval copy implies an ambient or inherited grant beyond the reviewed object and scope.
    AmbientScopeImplied,
    /// No command-backed trust-detail entrypoint is reachable.
    TrustDetailPathMissing,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5TrustElevationSheetDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ObjectIdentityUnstated,
        Self::ActorIdentityUnstated,
        Self::TrustScopeUnresolved,
        Self::GrantSourceUnstated,
        Self::PolicyEpochUnstated,
        Self::CapabilityDeltaUnstated,
        Self::ReducedModeAlternativeUnstated,
        Self::EffectDurationUnstated,
        Self::AmbientScopeImplied,
        Self::TrustDetailPathMissing,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ObjectIdentityUnstated => "object_identity_unstated",
            Self::ActorIdentityUnstated => "actor_identity_unstated",
            Self::TrustScopeUnresolved => "trust_scope_unresolved",
            Self::GrantSourceUnstated => "grant_source_unstated",
            Self::PolicyEpochUnstated => "policy_epoch_unstated",
            Self::CapabilityDeltaUnstated => "capability_delta_unstated",
            Self::ReducedModeAlternativeUnstated => "reduced_mode_alternative_unstated",
            Self::EffectDurationUnstated => "effect_duration_unstated",
            Self::AmbientScopeImplied => "ambient_scope_implied",
            Self::TrustDetailPathMissing => "trust_detail_path_missing",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5TrustFactGridElevationNextAction {
        match self {
            Self::ObjectIdentityUnstated
            | Self::ActorIdentityUnstated
            | Self::TrustScopeUnresolved
            | Self::EffectDurationUnstated
            | Self::AmbientScopeImplied
            | Self::TrustDetailPathMissing => M5TrustFactGridElevationNextAction::OpenTrustDetail,
            Self::GrantSourceUnstated | Self::PolicyEpochUnstated => {
                M5TrustFactGridElevationNextAction::ReviewPolicySource
            }
            Self::CapabilityDeltaUnstated => {
                M5TrustFactGridElevationNextAction::ReviewCapabilityDelta
            }
            Self::ReducedModeAlternativeUnstated => {
                M5TrustFactGridElevationNextAction::ContinueInReducedMode
            }
            Self::ProofStale => M5TrustFactGridElevationNextAction::ReviewDiagnostics,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WorkspaceTrustRepairDowngradeTrigger {
        match self {
            Self::ObjectIdentityUnstated
            | Self::ActorIdentityUnstated
            | Self::TrustScopeUnresolved
            | Self::ReducedModeAlternativeUnstated
            | Self::EffectDurationUnstated
            | Self::TrustDetailPathMissing => {
                M5WorkspaceTrustRepairDowngradeTrigger::GenericChromeWordingUsed
            }
            Self::GrantSourceUnstated => M5WorkspaceTrustRepairDowngradeTrigger::GrantSourceUnstated,
            Self::PolicyEpochUnstated => M5WorkspaceTrustRepairDowngradeTrigger::PolicyEpochUnstated,
            Self::CapabilityDeltaUnstated => {
                M5WorkspaceTrustRepairDowngradeTrigger::NarrowedCapabilityUnstated
            }
            Self::AmbientScopeImplied => {
                M5WorkspaceTrustRepairDowngradeTrigger::RootScopeCollapsedIntoBlanketTrust
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

/// True when the grant source is resolved: a concrete grant class that is disclosed on the surface.
fn grant_is_resolved(source: M5TrustGrantSourceClass, actor_stated: bool) -> bool {
    !matches!(source, M5TrustGrantSourceClass::GrantSourceUnknown) && actor_stated
}

/// Input to [`resolve_trust_fact_grid`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TrustFactGridResolutionInput {
    /// Stable identity of the grid instance.
    pub grid_id: String,
    /// The actor requesting or granting the trust; empty means unstated.
    pub actor_identity: String,
    /// The trusted object identity (workspace / root name); empty means unstated.
    pub object_identity: String,
    /// The trust scope of the workspace.
    pub trust_scope: M5TrustScopeState,
    /// The per-root trust state named alongside the scope.
    pub root_trust: M5RootTrustState,
    /// Who granted the trust.
    pub grant_source: M5TrustGrantSourceClass,
    /// True when the grant actor / source is disclosed on the grid, never menu-only.
    pub grant_actor_stated: bool,
    /// The policy epoch behind a managed grant; empty means unstated.
    pub policy_epoch: String,
    /// The narrowed-capability state.
    pub capability_narrow: M5CapabilityNarrowState,
    /// True when the narrowed capability is named on the grid.
    pub capability_narrow_stated: bool,
    /// True when the grid reads a mixed-root workspace as uniform (blanket) trust.
    pub reads_as_uniform_trust: bool,
    /// True when a command-backed trust-detail entrypoint is reachable, never menu-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe trust-fact grid projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTrustFactGrid {
    /// Stable identity of the grid instance.
    pub grid_id: String,
    /// The actor named by the grid.
    pub actor_identity: String,
    /// The trusted object identity named by the grid.
    pub object_identity: String,
    /// The trust-scope token named by the grid.
    pub trust_scope: String,
    /// Single controlled trust disposition, or `null` when the scope is unresolved.
    pub trust_disposition: Option<M5WorkspaceTrustRepairDisposition>,
    /// The per-root trust token named by the grid.
    pub root_trust: String,
    /// The grant-source token named by the grid.
    pub grant_source: String,
    /// The policy epoch named by the grid.
    pub policy_epoch: String,
    /// The narrowed-capability token named by the grid.
    pub capability_narrow: String,
    /// Whether any capability is narrowed relative to full trust.
    pub capability_narrowed: bool,
    /// Whether this grid describes a mixed-root workspace.
    pub is_mixed_root: bool,
    /// Guardrail (MUST be `false` on a clean grid): a mixed-root workspace reads as uniform trust.
    pub collapses_per_root_into_uniform: bool,
    /// Whether a command-backed trust-detail entrypoint is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the grid could not read as a clean, fully-legible state.
    pub degrade_reason: Option<M5TrustFactGridDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5TrustFactGridElevationNextAction,
    /// Whether every trust fact is named (clean grid).
    pub all_facts_named: bool,
}

impl M5ResolvedTrustFactGrid {
    /// Whether this grid reads as a clean, fully-legible state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_trust_elevation_sheet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TrustElevationSheetResolutionInput {
    /// Stable identity of the sheet instance.
    pub sheet_id: String,
    /// The actor requesting or granting the trust; empty means unstated.
    pub actor_identity: String,
    /// The trusted object identity (workspace / root name); empty means unstated.
    pub object_identity: String,
    /// The trust scope the elevation would grant.
    pub trust_scope: M5TrustScopeState,
    /// Who is granting the trust.
    pub grant_source: M5TrustGrantSourceClass,
    /// True when the grant actor / source is disclosed on the sheet, never menu-only.
    pub grant_actor_stated: bool,
    /// The policy epoch behind a managed grant; empty means unstated.
    pub policy_epoch: String,
    /// The narrowed capability a grant would change.
    pub capability_narrow: M5CapabilityNarrowState,
    /// True when the capability delta (what changes if trust is granted) is named on the sheet.
    pub capability_delta_stated: bool,
    /// True when the reduced-mode alternative (what still works without trust) is named.
    pub reduced_mode_alternative_stated: bool,
    /// The lasting-versus-one-time effect of the grant.
    pub effect_class: M5TrustElevationEffectClass,
    /// True when the approval copy implies an ambient or inherited grant beyond the reviewed object.
    pub implies_ambient_grant: bool,
    /// True when a command-backed trust-detail entrypoint is reachable, never menu-only.
    pub detail_command_available: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe trust-elevation sheet projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedTrustElevationSheet {
    /// Stable identity of the sheet instance.
    pub sheet_id: String,
    /// The actor named by the sheet.
    pub actor_identity: String,
    /// The trusted object identity named by the sheet.
    pub object_identity: String,
    /// The trust-scope token named by the sheet.
    pub trust_scope: String,
    /// Single controlled trust disposition, or `null` when the scope is unresolved.
    pub trust_disposition: Option<M5WorkspaceTrustRepairDisposition>,
    /// The grant-source token named by the sheet.
    pub grant_source: String,
    /// The policy epoch named by the sheet.
    pub policy_epoch: String,
    /// The narrowed-capability token named by the sheet.
    pub capability_narrow: String,
    /// Whether any capability is narrowed relative to full trust.
    pub capability_narrowed: bool,
    /// Whether the reduced-mode alternative (what still works without trust) is named.
    pub reduced_mode_alternative_stated: bool,
    /// The effect-duration token named by the sheet.
    pub effect_class: String,
    /// Whether the grant is lasting rather than one-time / single-action.
    pub effect_lasting: bool,
    /// Guardrail (MUST be `false` on a clean sheet): the copy implies an ambient / inherited grant
    /// beyond the reviewed object and scope.
    pub implies_ambient_scope: bool,
    /// Whether a command-backed trust-detail entrypoint is reachable.
    pub detail_command_available: bool,
    /// Degrade reason, if the sheet could not read as reviewed-before-approval.
    pub degrade_reason: Option<M5TrustElevationSheetDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5TrustFactGridElevationNextAction,
    /// Whether the sheet is fully reviewable before approval (clean sheet naming every fact).
    pub reviewed_before_approval: bool,
}

impl M5ResolvedTrustElevationSheet {
    /// Whether this sheet reads as a clean, reviewed-before-approval state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5TrustFactGridElevationResolutionError {
    /// The grid id was empty.
    EmptyGridId,
    /// The sheet id was empty.
    EmptySheetId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5TrustFactGridElevationResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyGridId => "empty_grid_id",
            Self::EmptySheetId => "empty_sheet_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5TrustFactGridElevationResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 trust-fact-grid / trust-elevation-sheet resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5TrustFactGridElevationResolutionError {}

/// Resolves a trust-fact grid, making the trust facts legible in one place: the grid names its
/// actor, object identity, trust class, policy source, narrowed capability, and per-root trust, and
/// never reads a mixed-root workspace as uniform trust.
pub fn resolve_trust_fact_grid(
    input: M5TrustFactGridResolutionInput,
) -> Result<M5ResolvedTrustFactGrid, M5TrustFactGridElevationResolutionError> {
    if input.grid_id.trim().is_empty() {
        return Err(M5TrustFactGridElevationResolutionError::EmptyGridId);
    }
    if string_is_forbidden(&input.grid_id)
        || string_is_forbidden(&input.actor_identity)
        || string_is_forbidden(&input.object_identity)
        || string_is_forbidden(&input.policy_epoch)
    {
        return Err(M5TrustFactGridElevationResolutionError::ForbiddenMaterial);
    }

    let is_mixed_root = matches!(input.trust_scope, M5TrustScopeState::MixedRoot);
    let capability_narrowed = !matches!(
        input.capability_narrow,
        M5CapabilityNarrowState::FullCapability
    );
    let grant_resolved = grant_is_resolved(input.grant_source, input.grant_actor_stated);
    let policy_epoch_required = matches!(input.grant_source, M5TrustGrantSourceClass::PolicyManaged);
    let collapses_per_root_into_uniform = is_mixed_root && input.reads_as_uniform_trust;

    let degrade_reason = if input.object_identity.trim().is_empty() {
        Some(M5TrustFactGridDegradeReason::ObjectIdentityUnstated)
    } else if input.actor_identity.trim().is_empty() {
        Some(M5TrustFactGridDegradeReason::ActorIdentityUnstated)
    } else if matches!(input.trust_scope, M5TrustScopeState::ScopeUnknown) {
        Some(M5TrustFactGridDegradeReason::TrustScopeUnresolved)
    } else if !grant_resolved {
        Some(M5TrustFactGridDegradeReason::GrantSourceUnstated)
    } else if policy_epoch_required && input.policy_epoch.trim().is_empty() {
        Some(M5TrustFactGridDegradeReason::PolicyEpochUnstated)
    } else if capability_narrowed && !input.capability_narrow_stated {
        Some(M5TrustFactGridDegradeReason::NarrowedCapabilityUnstated)
    } else if collapses_per_root_into_uniform {
        Some(M5TrustFactGridDegradeReason::MixedRootCollapsedIntoUniform)
    } else if !input.detail_command_available {
        Some(M5TrustFactGridDegradeReason::TrustDetailPathMissing)
    } else if !input.proof_fresh {
        Some(M5TrustFactGridDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5TrustFactGridElevationNextAction::OpenTrustDetail,
    };

    Ok(M5ResolvedTrustFactGrid {
        grid_id: input.grid_id,
        actor_identity: input.actor_identity,
        object_identity: input.object_identity,
        trust_scope: input.trust_scope.as_str().to_owned(),
        trust_disposition: disposition_for_scope(input.trust_scope),
        root_trust: input.root_trust.as_str().to_owned(),
        grant_source: input.grant_source.as_str().to_owned(),
        policy_epoch: input.policy_epoch,
        capability_narrow: input.capability_narrow.as_str().to_owned(),
        capability_narrowed,
        is_mixed_root,
        collapses_per_root_into_uniform,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        all_facts_named: degrade_reason.is_none(),
    })
}

/// Resolves a trust-elevation sheet, proving trust elevation is a reviewed fact sheet: the sheet
/// names its actor, object, scope, policy source, capability delta, reduced-mode alternative, and
/// lasting-versus-one-time effect, never implies an ambient or inherited grant beyond the reviewed
/// object and scope, and always exposes a command-backed detail path for inspecting before approval.
pub fn resolve_trust_elevation_sheet(
    input: M5TrustElevationSheetResolutionInput,
) -> Result<M5ResolvedTrustElevationSheet, M5TrustFactGridElevationResolutionError> {
    if input.sheet_id.trim().is_empty() {
        return Err(M5TrustFactGridElevationResolutionError::EmptySheetId);
    }
    if string_is_forbidden(&input.sheet_id)
        || string_is_forbidden(&input.actor_identity)
        || string_is_forbidden(&input.object_identity)
        || string_is_forbidden(&input.policy_epoch)
    {
        return Err(M5TrustFactGridElevationResolutionError::ForbiddenMaterial);
    }

    let capability_narrowed = !matches!(
        input.capability_narrow,
        M5CapabilityNarrowState::FullCapability
    );
    let grant_resolved = grant_is_resolved(input.grant_source, input.grant_actor_stated);
    let policy_epoch_required = matches!(input.grant_source, M5TrustGrantSourceClass::PolicyManaged);

    let degrade_reason = if input.object_identity.trim().is_empty() {
        Some(M5TrustElevationSheetDegradeReason::ObjectIdentityUnstated)
    } else if input.actor_identity.trim().is_empty() {
        Some(M5TrustElevationSheetDegradeReason::ActorIdentityUnstated)
    } else if matches!(input.trust_scope, M5TrustScopeState::ScopeUnknown) {
        Some(M5TrustElevationSheetDegradeReason::TrustScopeUnresolved)
    } else if !grant_resolved {
        Some(M5TrustElevationSheetDegradeReason::GrantSourceUnstated)
    } else if policy_epoch_required && input.policy_epoch.trim().is_empty() {
        Some(M5TrustElevationSheetDegradeReason::PolicyEpochUnstated)
    } else if capability_narrowed && !input.capability_delta_stated {
        Some(M5TrustElevationSheetDegradeReason::CapabilityDeltaUnstated)
    } else if !input.reduced_mode_alternative_stated {
        Some(M5TrustElevationSheetDegradeReason::ReducedModeAlternativeUnstated)
    } else if !input.effect_class.is_stated() {
        Some(M5TrustElevationSheetDegradeReason::EffectDurationUnstated)
    } else if input.implies_ambient_grant {
        Some(M5TrustElevationSheetDegradeReason::AmbientScopeImplied)
    } else if !input.detail_command_available {
        Some(M5TrustElevationSheetDegradeReason::TrustDetailPathMissing)
    } else if !input.proof_fresh {
        Some(M5TrustElevationSheetDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5TrustFactGridElevationNextAction::OpenTrustDetail,
    };

    Ok(M5ResolvedTrustElevationSheet {
        sheet_id: input.sheet_id,
        actor_identity: input.actor_identity,
        object_identity: input.object_identity,
        trust_scope: input.trust_scope.as_str().to_owned(),
        trust_disposition: disposition_for_scope(input.trust_scope),
        grant_source: input.grant_source.as_str().to_owned(),
        policy_epoch: input.policy_epoch,
        capability_narrow: input.capability_narrow.as_str().to_owned(),
        capability_narrowed,
        reduced_mode_alternative_stated: input.reduced_mode_alternative_stated,
        effect_class: input.effect_class.as_str().to_owned(),
        effect_lasting: input.effect_class.is_lasting(),
        implies_ambient_scope: input.implies_ambient_grant,
        detail_command_available: input.detail_command_available,
        degrade_reason,
        next_action,
        reviewed_before_approval: degrade_reason.is_none(),
    })
}

/// One controls row: one consumer surface bound to the resolved trust-fact grid and trust-elevation
/// sheet examples it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustFactGridElevationControlsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5TrustFactGridElevationConsumerSurface,
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
    pub anatomy_parts: Vec<M5TrustFactGridElevationAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5TrustFactGridElevationExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5WorkspaceTrustRepairDowngradeTrigger>,
    /// Resolved trust-fact grid examples.
    pub trust_fact_grid_examples: Vec<M5ResolvedTrustFactGrid>,
    /// Resolved trust-elevation sheet examples.
    pub trust_elevation_sheet_examples: Vec<M5ResolvedTrustElevationSheet>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both component schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: never imply an ambient or inherited grant beyond the reviewed object.
    pub implies_ambient_or_inherited_grant_beyond_reviewed_object: bool,
    /// Hard invariant: never hide the policy source or capability delta behind menus only.
    pub hides_policy_source_or_capability_delta_in_menus_only: bool,
    /// Hard invariant: never collapse the reduced-mode alternative into generic chrome.
    pub collapses_reduced_mode_alternative_into_generic_chrome: bool,
    /// Hard invariant: never collapse a one-time effect into a lasting / generic grant.
    pub collapses_effect_duration_into_generic_grant: bool,
}

impl M5TrustFactGridElevationControlsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5TrustFactGridElevationAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5TrustFactGridElevationAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5TrustFactGridElevationExportField> =
            self.export_fields.iter().copied().collect();
        M5TrustFactGridElevationExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.implies_ambient_or_inherited_grant_beyond_reviewed_object
            && !self.hides_policy_source_or_capability_delta_in_menus_only
            && !self.collapses_reduced_mode_alternative_into_generic_chrome
            && !self.collapses_effect_duration_into_generic_grant
    }

    /// True when every resolved example on this row is honest: no clean grid collapses mixed-root
    /// trust, no clean sheet implies ambient scope, and no clean example hides the detail path.
    fn examples_are_honest(&self) -> bool {
        self.trust_fact_grid_examples.iter().all(|ex| {
            !(ex.is_clean() && (ex.collapses_per_root_into_uniform || !ex.detail_command_available))
        }) && self.trust_elevation_sheet_examples.iter().all(|ex| {
            !(ex.is_clean() && (ex.implies_ambient_scope || !ex.detail_command_available))
        })
    }
}

/// Self-describing controlled-vocabulary set frozen by the controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustFactGridElevationVocabularySet {
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
    /// Effect-duration tokens (minted by this lane).
    pub effect_classes: Vec<String>,
    /// Grid degrade-reason tokens.
    pub grid_degrade_reasons: Vec<String>,
    /// Sheet degrade-reason tokens.
    pub sheet_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5TrustFactGridElevationVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            trust_dispositions: tokens(&M5WorkspaceTrustRepairDisposition::ALL, |v| v.as_str()),
            trust_scopes: tokens(&M5TrustScopeState::ALL, |v| v.as_str()),
            grant_sources: tokens(&M5TrustGrantSourceClass::ALL, |v| v.as_str()),
            capability_narrow_states: tokens(&M5CapabilityNarrowState::ALL, |v| v.as_str()),
            root_trust_states: tokens(&M5RootTrustState::ALL, |v| v.as_str()),
            effect_classes: tokens(&M5TrustElevationEffectClass::ALL, |v| v.as_str()),
            grid_degrade_reasons: tokens(&M5TrustFactGridDegradeReason::ALL, |v| v.as_str()),
            sheet_degrade_reasons: tokens(&M5TrustElevationSheetDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5TrustFactGridElevationAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5TrustFactGridElevationNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5TrustFactGridElevationExportField::ALL, |v| v.as_str()),
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
pub struct M5TrustFactGridElevationGovernanceReview {
    /// The grid always names its actor, object identity, and trust scope.
    pub grid_names_actor_object_and_scope: bool,
    /// The grid always names its policy source and narrowed capability.
    pub grid_names_policy_source_and_capability: bool,
    /// The elevation sheet always names the reduced-mode alternative.
    pub elevation_sheet_names_reduced_mode_alternative: bool,
    /// The elevation sheet always names the lasting-versus-one-time effect.
    pub elevation_sheet_names_effect_duration: bool,
    /// No trust prompt implies an ambient or inherited grant beyond the reviewed object.
    pub no_prompt_implies_ambient_grant_beyond_object: bool,
    /// A command-backed trust-detail entrypoint is always reachable before approval.
    pub trust_detail_command_always_reachable: bool,
    /// Trust prompts share one field and delta vocabulary across surfaces.
    pub trust_vocabulary_shared_across_surfaces: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustFactGridElevationConsumerProjection {
    /// Trust prompts across surfaces expose the same fields and delta grammar.
    pub trust_prompts_expose_same_fields_and_delta_grammar: bool,
    /// Scope and source are inspectable before approval without leaving the workflow.
    pub scope_and_source_inspectable_before_approval: bool,
    /// Elevation state traces back to one canonical component contract.
    pub elevation_traces_to_single_component_contract: bool,
    /// Support / export reads a single canonical trust source.
    pub support_export_reads_single_trust_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustFactGridElevationProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the controls lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustFactGridElevationReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting component audit for the lane.
    pub component_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5TrustFactGridElevationControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5TrustFactGridElevationControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5TrustFactGridElevationControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TrustFactGridElevationVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TrustFactGridElevationGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TrustFactGridElevationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TrustFactGridElevationProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TrustFactGridElevationReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 trust-fact-grid / trust-elevation-sheet controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5TrustFactGridElevationControlsPacket {
    /// Record kind; must equal [`M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable controls label.
    pub controls_label: String,
    /// Controls rows.
    pub controls_rows: Vec<M5TrustFactGridElevationControlsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5TrustFactGridElevationVocabularySet,
    /// Governance-review block.
    pub governance_review: M5TrustFactGridElevationGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5TrustFactGridElevationConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5TrustFactGridElevationProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5TrustFactGridElevationReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5TrustFactGridElevationControlsPacket {
    /// Builds a controls packet from stable-lane input.
    pub fn new(input: M5TrustFactGridElevationControlsPacketInput) -> Self {
        Self {
            record_kind: M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5TrustFactGridElevationControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_RECORD_KIND {
            violations.push(M5TrustFactGridElevationControlsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_SCHEMA_VERSION {
            violations.push(M5TrustFactGridElevationControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.controls_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5TrustFactGridElevationControlsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5TrustFactGridElevationControlsViolation::VocabularySetDrift);
        }
        validate_controls_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 trust-fact-grid / trust-elevation-sheet controls packet serializes"),
        ) {
            violations.push(M5TrustFactGridElevationControlsViolation::RawMaterialInExport);
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
            .expect("m5 trust-fact-grid / trust-elevation-sheet controls packet serializes")
    }

    /// Deterministic, machine-readable controls CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,grid_examples,sheet_examples,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.controls_rows {
            let degrades: Vec<&str> = row
                .trust_fact_grid_examples
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.trust_elevation_sheet_examples
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.trust_fact_grid_examples.len(),
                row.trust_elevation_sheet_examples.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Trust-Fact-Grid and Trust-Elevation-Sheet Controls\n\n");
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
            "- Effect classes: {}\n",
            self.vocabulary_set.effect_classes.join(", ")
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
                "  - Grid examples: {} / sheet examples: {}\n",
                row.trust_fact_grid_examples.len(),
                row.trust_elevation_sheet_examples.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable controls export.
#[derive(Debug)]
pub enum M5TrustFactGridElevationControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5TrustFactGridElevationControlsViolation>),
}

impl fmt::Display for M5TrustFactGridElevationControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 trust-fact-grid / trust-elevation-sheet controls export parse failed: {error}"
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
                    "m5 trust-fact-grid / trust-elevation-sheet controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5TrustFactGridElevationControlsArtifactError {}

/// Validation failures emitted by [`M5TrustFactGridElevationControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5TrustFactGridElevationControlsViolation {
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
    /// A controls row carries a dishonest clean example (ambient scope, collapse, or hidden detail).
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
    /// No-ambient-grant honesty is not proven: clean sheets do not cover the required scopes, or no
    /// ambient-scope example degrades, or a clean sheet implies ambient scope.
    NoAmbientGrantNotProven,
    /// Field / delta / reduced-mode parity is not proven: clean sheets do not name the reduced-mode
    /// alternative and effect duration, or no missing-field example degrades.
    FieldAndReducedModeParityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5TrustFactGridElevationControlsViolation {
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
            Self::NoAmbientGrantNotProven => "no_ambient_grant_not_proven",
            Self::FieldAndReducedModeParityNotProven => "field_and_reduced_mode_parity_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable controls export.
pub fn current_stable_m5_trust_fact_grid_elevation_controls_export(
) -> Result<M5TrustFactGridElevationControlsPacket, M5TrustFactGridElevationControlsArtifactError> {
    let packet: M5TrustFactGridElevationControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-trust-fact-grid-trust-elevation-sheet-controls-proof/support_export.json"
    )))
    .map_err(M5TrustFactGridElevationControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5TrustFactGridElevationControlsArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5TrustFactGridElevationControlsPacket,
    violations: &mut Vec<M5TrustFactGridElevationControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_SCHEMA_REF,
        M5_TRUST_FACT_GRID_ELEVATION_CONTROLS_DOC_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_SCHEMA_REF,
        M5_WORKSPACE_TRUST_REPAIR_COMPONENT_DOC_REF,
        M5_TRUST_FACT_GRID_SCHEMA_REF,
        M5_TRUST_ELEVATION_SHEET_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5TrustFactGridElevationControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_controls_rows(
    packet: &M5TrustFactGridElevationControlsPacket,
    violations: &mut Vec<M5TrustFactGridElevationControlsViolation>,
) {
    if packet.controls_rows.is_empty() {
        violations.push(M5TrustFactGridElevationControlsViolation::NoControlsRows);
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
            violations.push(M5TrustFactGridElevationControlsViolation::ControlsRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5TrustFactGridElevationControlsViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5TrustFactGridElevationControlsViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_TRUST_FACT_GRID_SCHEMA_REF)
            || !refs.contains(M5_TRUST_ELEVATION_SHEET_SCHEMA_REF)
        {
            violations.push(M5TrustFactGridElevationControlsViolation::ComponentSchemaRefMissing);
        }
        if row.trust_fact_grid_examples.is_empty() || row.trust_elevation_sheet_examples.is_empty() {
            violations.push(M5TrustFactGridElevationControlsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5TrustFactGridElevationControlsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5TrustFactGridElevationControlsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5TrustFactGridElevationControlsPacket,
    violations: &mut Vec<M5TrustFactGridElevationControlsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.grid_names_actor_object_and_scope,
        review.grid_names_policy_source_and_capability,
        review.elevation_sheet_names_reduced_mode_alternative,
        review.elevation_sheet_names_effect_duration,
        review.no_prompt_implies_ambient_grant_beyond_object,
        review.trust_detail_command_always_reachable,
        review.trust_vocabulary_shared_across_surfaces,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5TrustFactGridElevationControlsViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5TrustFactGridElevationControlsPacket,
    violations: &mut Vec<M5TrustFactGridElevationControlsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.trust_prompts_expose_same_fields_and_delta_grammar,
        projection.scope_and_source_inspectable_before_approval,
        projection.elevation_traces_to_single_component_contract,
        projection.support_export_reads_single_trust_source,
    ] {
        if !ok {
            violations.push(M5TrustFactGridElevationControlsViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5TrustFactGridElevationControlsPacket,
    violations: &mut Vec<M5TrustFactGridElevationControlsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5TrustFactGridElevationControlsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5TrustFactGridElevationControlsPacket,
    violations: &mut Vec<M5TrustFactGridElevationControlsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5TrustFactGridElevationControlsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the two acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5TrustFactGridElevationControlsPacket,
    violations: &mut Vec<M5TrustFactGridElevationControlsViolation>,
) {
    let grids = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.trust_fact_grid_examples.iter())
    };
    let sheets = || {
        packet
            .controls_rows
            .iter()
            .flat_map(|row| row.trust_elevation_sheet_examples.iter())
    };

    // AC: no claimed trust prompt implies an ambient or inherited grant beyond the reviewed object
    // and scope. Clean sheets cover the trusted-workspace, trusted-root, restricted, and mixed-root
    // scopes so scope is always explicit (a trusted root is never presented as a trusted
    // workspace), at least one sheet degrades to `ambient_scope_implied`, and no clean sheet implies
    // ambient scope.
    let clean_sheet_scopes: BTreeSet<&str> = sheets()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.trust_scope.as_str())
        .collect();
    let covers_required_scopes = [
        M5TrustScopeState::TrustedWorkspace,
        M5TrustScopeState::TrustedRoot,
        M5TrustScopeState::RestrictedWorkspace,
        M5TrustScopeState::MixedRoot,
    ]
    .iter()
    .all(|scope| clean_sheet_scopes.contains(scope.as_str()));
    let ambient_degrades = sheets().any(|ex| {
        ex.degrade_reason == Some(M5TrustElevationSheetDegradeReason::AmbientScopeImplied)
            && ex.implies_ambient_scope
    });
    let no_clean_ambient = sheets().all(|ex| !(ex.is_clean() && ex.implies_ambient_scope));
    if !(covers_required_scopes && ambient_degrades && no_clean_ambient) {
        violations.push(M5TrustFactGridElevationControlsViolation::NoAmbientGrantNotProven);
    }

    // AC: trust prompts expose the same fields, delta grammar, and reduced-mode path, and scope /
    // source is inspectable before approval. Every clean grid and sheet exposes the command-backed
    // detail path; clean sheets always name the reduced-mode alternative and effect duration and
    // cover both a lasting and a one-time grant; and missing-field examples degrade for the
    // capability delta, the reduced-mode alternative, the effect duration, and the detail path.
    let clean_exposes_detail = grids().all(|ex| !ex.is_clean() || ex.detail_command_available)
        && sheets().all(|ex| !ex.is_clean() || ex.detail_command_available);
    let clean_sheets_name_reduced_mode_and_effect = sheets()
        .filter(|ex| ex.is_clean())
        .all(|ex| ex.reduced_mode_alternative_stated && ex.effect_class != "effect_unknown");
    let covers_lasting = sheets().any(|ex| ex.is_clean() && ex.effect_lasting);
    let covers_one_time = sheets().any(|ex| {
        ex.is_clean() && !ex.effect_lasting && ex.effect_class != "effect_unknown"
    });
    let capability_delta_degrades = sheets()
        .any(|ex| ex.degrade_reason == Some(M5TrustElevationSheetDegradeReason::CapabilityDeltaUnstated));
    let reduced_mode_degrades = sheets().any(|ex| {
        ex.degrade_reason == Some(M5TrustElevationSheetDegradeReason::ReducedModeAlternativeUnstated)
    });
    let effect_degrades = sheets()
        .any(|ex| ex.degrade_reason == Some(M5TrustElevationSheetDegradeReason::EffectDurationUnstated));
    let detail_degrades = grids()
        .any(|ex| ex.degrade_reason == Some(M5TrustFactGridDegradeReason::TrustDetailPathMissing))
        || sheets().any(|ex| {
            ex.degrade_reason == Some(M5TrustElevationSheetDegradeReason::TrustDetailPathMissing)
        });
    if !(clean_exposes_detail
        && clean_sheets_name_reduced_mode_and_effect
        && covers_lasting
        && covers_one_time
        && capability_delta_degrades
        && reduced_mode_degrades
        && effect_degrades
        && detail_degrades)
    {
        violations
            .push(M5TrustFactGridElevationControlsViolation::FieldAndReducedModeParityNotProven);
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
    M5WorkspaceTrustRepairComponentFamily::TrustFactGrid,
    M5WorkspaceTrustRepairComponentFamily::TrustElevationSheet,
];
