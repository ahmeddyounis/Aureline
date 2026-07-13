//! Implemented M5 install-topology and state-root-boundary registries.
//!
//! The frozen [install-topology matrix][matrix] names Aureline's five delivery-topology families and locks
//! their controlled vocabulary. This module is the first implement lane for the concrete delivery-topology
//! resolution flows: it turns the *per-user managed / per-machine managed / side-by-side stable-plus-preview*
//! install-topology grammar and the *portable-mode / offline-air-gap* state-root-boundary grammar into
//! registry resolvers that produce export-safe, honest projections. Every claimed M5 delivery profile then
//! resolves to one stable install-topology object — install mode, channel, updater owner, binary / artifact
//! root, primary writable state roots, policy roots, and rollback target — that About, update, diagnostics,
//! admin, and support / export surfaces can inspect without manual reconstruction, so shared-versus-isolated
//! state namespaces are explicit (never ad hoc path derivation), managed-versus-user scopes resolve to
//! distinct roots, side-by-side preview and stable channels never reuse a state namespace without an explicit
//! handoff, and a profile that cannot explain its shared-versus-isolated state degrades honestly instead of
//! reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one stable install-topology object per delivery profile.** [`resolve_install_topology_entry`]
//!   refuses to read as a clean, registry-bound install-topology entry unless it names a canonical registry
//!   token, a classified [delivery scope][M5DeliveryScope], an install-topology role, covers every
//!   [resolution form][M5InstallStateResolutionForm] (the canonical object, the accessible summary, and the
//!   audit record), publishes every install-topology field (install mode, channel, updater owner, binary
//!   root, writable state roots, policy roots, and rollback target), keeps managed-versus-user scopes on
//!   distinct roots, and explains any coexistence handoff; otherwise it degrades.
//! * **Enforce explicit shared-versus-isolated state namespaces.** [`state_namespace_is_isolated`] rejects a
//!   side-by-side entry whose preview and stable channels resolve to the same state namespace without an
//!   explicit handoff so a reused namespace degrades to
//!   [`M5InstallTopologyEntryDegradeReason::StateNamespaceReusedWithoutHandoff`], and the
//!   `managed_and_user_scopes_isolated` invariant degrades a managed-versus-user scope that collapses onto a
//!   shared root.
//! * **Bound writable state and rollback targets truthfully in state-root registries.**
//!   [`resolve_state_root_boundary_entry`] names a classified [state-root surface][M5StateRootSurface],
//!   requires the boundary to provide the writable-state-root / policy-root / rollback-target disclosure
//!   triple, covers every resolution form, and degrades to
//!   [`M5StateRootBoundaryEntryDegradeReason::StateBoundaryUntruthfulOrIncomplete`] when portable mode spills
//!   hidden machine-global durable state, a rollback narrows below the full artifact graph, or the boundary
//!   asserts an unexplained scope, so a delivery profile can never read as isolated when it is not.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5InstallTopologyRole`] role vocabulary,
//! the family-specific per-user / per-machine / side-by-side / portable / offline role vocabularies, and the
//! [`M5InstallTopologyConsumerSurface`] consumer-surface taxonomy — so About, update, diagnostics, admin,
//! installer, docs, CLI, and support surfaces can never fork their own delivery-topology meaning. Raw secret
//! values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_install_topology_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_install_topology_and_state_root_registries,
    seeded_m5_install_topology_and_state_root_registries_offline_airgap_bundle_preview_narrowed,
    seeded_m5_install_topology_and_state_root_registries_side_by_side_channel_beta_narrowed,
    M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_install_topology_matrix::{
    M5InstallTopologyAccessibilityRoute, M5InstallTopologyConsumerSurface,
    M5InstallTopologyDeploymentLine, M5InstallTopologyDowngradeTrigger, M5InstallTopologyFamily,
    M5InstallTopologyQualificationClass, M5InstallTopologyRequiredLabel, M5InstallTopologyRole,
    M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF, M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
    M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF, M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5InstallTopologyStateRootRegistriesPacket`].
pub const M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_install_topology_and_state_root_registries";

/// Schema version for M5 install-topology / state-root registry records.
pub const M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF: &str =
    "schemas/install/m5-install-topology-and-state-root-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_DOC_REF: &str =
    "docs/install/m5_install_topology_and_state_root_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-install-topology-and-state-root-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-install-topology-and-state-root-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-install-topology-and-state-root-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/install/m5-install-topology-and-state-root-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5InstallTopologyStateRootRegistriesConsumerSurface = M5InstallTopologyConsumerSurface;

/// One of the three resolution forms every install-topology or state-root-boundary entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary,
/// or written to the audit / support record. Minted by this lane because the frozen matrix names the
/// install-topology and state-root *families* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallStateResolutionForm {
    /// The canonical resolved install-topology / state-root object (install mode, roots, rollback target).
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved topology discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved topology inspectable off-renderer.
    AuditRecord,
}

impl M5InstallStateResolutionForm {
    /// Every resolution form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::CanonicalObject,
        Self::AccessibleSummary,
        Self::AuditRecord,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalObject => "canonical_object",
            Self::AccessibleSummary => "accessible_summary",
            Self::AuditRecord => "audit_record",
        }
    }
}

/// Controlled delivery scope an install-topology entry resolves, so the canonical scope model shares one
/// registry rather than a hand-copied per-profile path assumption. Minted by this lane because the frozen
/// matrix carries the delivery-topology families but not the concrete managed-versus-user scope model an
/// install-topology entry resolves against. Every classified scope carries its canonical install mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeliveryScope {
    /// The per-user managed scope (user-scoped binary root, per-user updater ownership, user state root).
    PerUserManagedScope,
    /// The per-machine managed scope (machine-scoped binary root, admin-owned updater, machine state root).
    PerMachineManagedScope,
    /// The side-by-side stable-plus-preview scope (isolated channel roots and isolated state namespaces).
    SideBySideChannelScope,
    /// The delivery scope is unclassified, which is disallowed.
    ScopeUnclassified,
}

impl M5DeliveryScope {
    /// Every delivery scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PerUserManagedScope,
        Self::PerMachineManagedScope,
        Self::SideBySideChannelScope,
        Self::ScopeUnclassified,
    ];

    /// The three canonical delivery scopes every claimed M5 install-topology profile resolves against.
    pub const CANONICAL_SCOPES: [Self; 3] = [
        Self::PerUserManagedScope,
        Self::PerMachineManagedScope,
        Self::SideBySideChannelScope,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PerUserManagedScope => "per_user_managed_scope",
            Self::PerMachineManagedScope => "per_machine_managed_scope",
            Self::SideBySideChannelScope => "side_by_side_channel_scope",
            Self::ScopeUnclassified => "scope_unclassified",
        }
    }

    /// Whether the scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ScopeUnclassified)
    }

    /// The canonical install mode for this delivery scope.
    pub const fn canonical_install_mode(self) -> &'static str {
        match self {
            Self::PerUserManagedScope => "per_user_managed_install",
            Self::PerMachineManagedScope => "per_machine_managed_install",
            Self::SideBySideChannelScope => "side_by_side_channels",
            Self::ScopeUnclassified => "",
        }
    }

    /// Whether this scope coexists with a sibling channel and so must isolate its state namespace explicitly.
    pub const fn requires_channel_isolation(self) -> bool {
        matches!(self, Self::SideBySideChannelScope)
    }
}

/// Controlled state-root surface a state-root-boundary entry must resolve its boundary from, so a state-root
/// boundary shares one registry rather than a hand-copied per-surface path. Minted by this lane, tracking the
/// delivery surfaces the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateRootSurface {
    /// The portable-mode colocated state-root boundary.
    PortableModeBoundary,
    /// The offline / air-gap bundled state-root boundary.
    OfflineAirgapBoundary,
    /// The diagnostics / admin state-root inspection boundary.
    DiagnosticsInspectionBoundary,
    /// The state-root surface is unclassified, which is disallowed.
    SurfaceUnclassified,
}

impl M5StateRootSurface {
    /// Every state-root surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PortableModeBoundary,
        Self::OfflineAirgapBoundary,
        Self::DiagnosticsInspectionBoundary,
        Self::SurfaceUnclassified,
    ];

    /// The three canonical surfaces every state-root boundary must stay truthful across.
    pub const CANONICAL_SURFACES: [Self; 3] = [
        Self::PortableModeBoundary,
        Self::OfflineAirgapBoundary,
        Self::DiagnosticsInspectionBoundary,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PortableModeBoundary => "portable_mode_boundary",
            Self::OfflineAirgapBoundary => "offline_airgap_boundary",
            Self::DiagnosticsInspectionBoundary => "diagnostics_inspection_boundary",
            Self::SurfaceUnclassified => "surface_unclassified",
        }
    }

    /// Whether the state-root surface is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::SurfaceUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so an install-topology or
/// state-root token's meaning stays stable whether it appears in About, the update flow, diagnostics, admin,
/// or a support / export form. Minted by this lane, tracking the first-consumer surfaces the implementation
/// requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallSurfaceContext {
    /// The About surface.
    AboutSurface,
    /// The update flow surface.
    UpdateFlow,
    /// The diagnostics surface.
    DiagnosticsSurface,
    /// The admin surface.
    AdminSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5InstallSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AboutSurface,
        Self::UpdateFlow,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::AboutSurface,
        Self::UpdateFlow,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AboutSurface => "about_surface",
            Self::UpdateFlow => "update_flow",
            Self::DiagnosticsSurface => "diagnostics_surface",
            Self::AdminSurface => "admin_surface",
            Self::SupportOrExportForm => "support_or_export_form",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part an install-topology or state-root-boundary entry must be able to show, so no
/// install mode, updater owner, state root, policy root, rollback target, or registry fact is left implicit
/// behind a hand-copied per-profile assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallStateAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The delivery scope the entry resolves (install-topology entry).
    DeliveryScope,
    /// The install mode and channel the entry publishes (install-topology entry).
    InstallModeAndChannel,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The updater owner the entry publishes (install-topology entry).
    UpdaterOwner,
    /// The writable state roots and policy roots the entry publishes (both entries).
    StateAndPolicyRoots,
    /// The rollback target the entry publishes (both entries).
    RollbackTarget,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved topology or boundary (both entries).
    PlainLanguageMeaning,
}

impl M5InstallStateAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::DeliveryScope,
        Self::InstallModeAndChannel,
        Self::ResolutionFormCoverage,
        Self::UpdaterOwner,
        Self::StateAndPolicyRoots,
        Self::RollbackTarget,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::RegistryReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::RegistryReference => "registry_reference",
            Self::DeliveryScope => "delivery_scope",
            Self::InstallModeAndChannel => "install_mode_and_channel",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::UpdaterOwner => "updater_owner",
            Self::StateAndPolicyRoots => "state_and_policy_roots",
            Self::RollbackTarget => "rollback_target",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// topology, a state-root boundary, or a degraded install-topology / state-root entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallStateNextAction {
    /// Expand the resolved topology's or boundary's plain-language meaning.
    ExpandInstallMeaning,
    /// Inspect the delivery scope or state-root surface the entry resolves.
    InspectScopeOrSurface,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5InstallStateNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandInstallMeaning,
        Self::InspectScopeOrSurface,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandInstallMeaning => "expand_install_meaning",
            Self::InspectScopeOrSurface => "inspect_scope_or_surface",
            Self::CompleteResolutionFormCoverage => "complete_resolution_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallStateExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The install-topology families covered.
    InstallTopologyFamilies,
    /// The delivery scopes carried.
    DeliveryScopes,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The state-root surfaces carried.
    StateRootSurfaces,
    /// The render / surface context.
    SurfaceContext,
    /// The install modes carried.
    InstallModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5InstallStateExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::InstallTopologyFamilies,
        Self::DeliveryScopes,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::StateRootSurfaces,
        Self::SurfaceContext,
        Self::InstallModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::InstallTopologyFamilies,
        Self::DeliveryScopes,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::InstallTopologyFamilies => "install_topology_families",
            Self::DeliveryScopes => "delivery_scopes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::StateRootSurfaces => "state_root_surfaces",
            Self::SurfaceContext => "surface_context",
            Self::InstallModes => "install_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason an install-topology entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, isolation-losing, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InstallTopologyEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the topology means.
    InstallTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The delivery scope is unclassified (not in the resolved taxonomy).
    DeliveryScopeUnclassified,
    /// The behavior is a hand-copied per-profile assumption instead of tracing to the canonical registry, or
    /// hides updater ownership / admin control in a managed flow.
    TopologyNotBoundToRegistry,
    /// The resolved install-topology object is incomplete: install mode, channel, updater owner, binary root,
    /// writable state roots, policy roots, or rollback target is unstated.
    InstallTopologyObjectIncomplete,
    /// A side-by-side preview channel reused the stable state namespace without an explicit handoff, or a
    /// managed-versus-user scope collapsed onto a shared root.
    StateNamespaceReusedWithoutHandoff,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// The scope coexists with a sibling channel but no explicit handoff path is explained.
    CoexistenceHandoffUnexplained,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5InstallTopologyEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::InstallTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::DeliveryScopeUnclassified,
        Self::TopologyNotBoundToRegistry,
        Self::InstallTopologyObjectIncomplete,
        Self::StateNamespaceReusedWithoutHandoff,
        Self::ResolutionFormCoverageIncomplete,
        Self::CoexistenceHandoffUnexplained,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstallTokenUnstated => "install_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::DeliveryScopeUnclassified => "delivery_scope_unclassified",
            Self::TopologyNotBoundToRegistry => "topology_not_bound_to_registry",
            Self::InstallTopologyObjectIncomplete => "install_topology_object_incomplete",
            Self::StateNamespaceReusedWithoutHandoff => "state_namespace_reused_without_handoff",
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::CoexistenceHandoffUnexplained => "coexistence_handoff_unexplained",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5InstallStateNextAction {
        match self {
            Self::InstallTokenUnstated | Self::TopologyNotBoundToRegistry => {
                M5InstallStateNextAction::TraceCanonicalRegistry
            }
            Self::DeliveryScopeUnclassified
            | Self::InstallTopologyObjectIncomplete
            | Self::StateNamespaceReusedWithoutHandoff => {
                M5InstallStateNextAction::InspectScopeOrSurface
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5InstallStateNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::CoexistenceHandoffUnexplained
            | Self::ProofStale => M5InstallStateNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5InstallTopologyDowngradeTrigger {
        match self {
            Self::InstallTokenUnstated | Self::ResolutionFormCoverageIncomplete => {
                M5InstallTopologyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5InstallTopologyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::DeliveryScopeUnclassified => M5InstallTopologyDowngradeTrigger::InstallModeUnstated,
            Self::TopologyNotBoundToRegistry => {
                M5InstallTopologyDowngradeTrigger::UpdaterOwnershipOrAdminControlHiddenInManagedFlow
            }
            Self::InstallTopologyObjectIncomplete => {
                M5InstallTopologyDowngradeTrigger::UpdaterOwnerUnstated
            }
            Self::StateNamespaceReusedWithoutHandoff => {
                M5InstallTopologyDowngradeTrigger::PreviewChannelReusedStableStateNamespaceWithoutHandoff
            }
            Self::CoexistenceHandoffUnexplained => {
                M5InstallTopologyDowngradeTrigger::StateRootBoundaryDriftedByTopology
            }
            Self::ProofStale => M5InstallTopologyDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a state-root-boundary entry degraded below a clean, truthful state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateRootBoundaryEntryDegradeReason {
    /// The canonical registry token name is unstated.
    BoundaryTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The state-root surface is unclassified (not in the resolved taxonomy).
    StateRootSurfaceUnclassified,
    /// The boundary is untruthful or incomplete — portable mode spilled hidden machine-global durable state, a
    /// rollback narrowed below the full artifact graph, an unexplained scope was asserted, or the boundary
    /// dropped the writable-state-root / policy-root / rollback-target disclosure triple.
    StateBoundaryUntruthfulOrIncomplete,
    /// The canonical / accessible / audit resolution-form coverage of the boundary is incomplete.
    BoundaryFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5StateRootBoundaryEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BoundaryTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::StateRootSurfaceUnclassified,
        Self::StateBoundaryUntruthfulOrIncomplete,
        Self::BoundaryFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundaryTokenUnstated => "boundary_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::StateRootSurfaceUnclassified => "state_root_surface_unclassified",
            Self::StateBoundaryUntruthfulOrIncomplete => "state_boundary_untruthful_or_incomplete",
            Self::BoundaryFormCoverageIncomplete => "boundary_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5InstallStateNextAction {
        match self {
            Self::BoundaryTokenUnstated => M5InstallStateNextAction::TraceCanonicalRegistry,
            Self::StateRootSurfaceUnclassified | Self::StateBoundaryUntruthfulOrIncomplete => {
                M5InstallStateNextAction::InspectScopeOrSurface
            }
            Self::BoundaryFormCoverageIncomplete => {
                M5InstallStateNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5InstallStateNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5InstallTopologyDowngradeTrigger {
        match self {
            Self::BoundaryTokenUnstated => {
                M5InstallTopologyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::StateRootSurfaceUnclassified => {
                M5InstallTopologyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::StateBoundaryUntruthfulOrIncomplete => {
                M5InstallTopologyDowngradeTrigger::PortableModeWroteHiddenMachineGlobalDurableState
            }
            Self::BoundaryFormCoverageIncomplete => {
                M5InstallTopologyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::ProofStale => M5InstallTopologyDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_install_topology_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5InstallTopologyEntryResolutionInput {
    /// Stable identity of the install-topology-registry entry.
    pub entry_id: String,
    /// The stable install-profile ID this topology binds to (e.g. `profile.per_user`); empty means unstated.
    pub profile_id: String,
    /// The canonical registry token name (e.g. `install.topology.per_user`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5InstallTopologyRole,
    /// The delivery scope this entry resolves.
    pub delivery_scope: M5DeliveryScope,
    /// The render / surface context.
    pub surface_context: M5InstallSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5InstallStateResolutionForm>,
    /// The published channel (e.g. `stable`, `preview`); empty means unstated.
    pub channel: String,
    /// The published updater owner (e.g. `per_user_updater`, `admin_owned_updater`); empty means unstated.
    pub updater_owner: String,
    /// The published binary / artifact root (a filesystem path, never a URL); empty means unstated.
    pub binary_root: String,
    /// The published primary writable state roots (filesystem paths); empty means unstated.
    pub writable_state_roots: String,
    /// The published policy roots (filesystem paths); empty means unstated.
    pub policy_roots: String,
    /// The published rollback target (the full artifact graph); empty means unstated.
    pub rollback_target: String,
    /// True when the behavior traces to the shared topology registry (never a hand-copied constant) and never
    /// hides updater ownership or admin control in a managed flow.
    pub bound_to_registry: bool,
    /// True when managed-versus-user scopes and side-by-side channels resolve to distinct state namespaces (a
    /// hard invariant when `false`).
    pub state_namespaces_isolated: bool,
    /// True when this scope coexists with a sibling channel (side-by-side).
    pub coexists_with_sibling_channel: bool,
    /// True when an explicit cross-channel handoff path is explained for a coexisting scope.
    pub coexistence_handoff_explained: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe install-topology-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedInstallTopologyEntry {
    /// Stable identity of the install-topology-registry entry.
    pub entry_id: String,
    /// The stable install-profile ID this topology binds to.
    pub profile_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve state isolation and ownership under coexistence.
    pub semantic_role_preserves_state_isolation_and_ownership_under_coexistence: bool,
    /// The delivery-scope token named by the entry.
    pub delivery_scope: String,
    /// Whether the delivery scope is classified into the resolved taxonomy.
    pub delivery_scope_is_classified: bool,
    /// The canonical install mode for the entry's delivery scope.
    pub canonical_install_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published channel.
    pub channel: String,
    /// The published updater owner.
    pub updater_owner: String,
    /// The published binary / artifact root.
    pub binary_root: String,
    /// The published primary writable state roots.
    pub writable_state_roots: String,
    /// The published policy roots.
    pub policy_roots: String,
    /// The published rollback target.
    pub rollback_target: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved install-topology object publishes every required field.
    pub install_topology_object_complete: bool,
    /// Whether the entry traces to the shared topology registry.
    pub bound_to_registry: bool,
    /// Whether managed-versus-user scopes and side-by-side channels resolve to distinct state namespaces.
    pub state_namespaces_isolated: bool,
    /// Whether this scope coexists with a sibling channel.
    pub coexists_with_sibling_channel: bool,
    /// Whether an explicit cross-channel handoff path is explained for a coexisting scope.
    pub coexistence_handoff_explained: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5InstallTopologyEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5InstallStateNextAction,
    /// Whether the topology resolves to one stable object across every claimed profile (clean entry naming
    /// every fact).
    pub topology_resolves_across_profiles: bool,
}

impl M5ResolvedInstallTopologyEntry {
    /// Whether this install-topology entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_state_root_boundary_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5StateRootBoundaryEntryResolutionInput {
    /// Stable identity of the state-root-boundary entry.
    pub entry_id: String,
    /// The stable install-profile ID this boundary binds to; empty means unstated.
    pub profile_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5InstallTopologyRole,
    /// The state-root surface this entry must resolve its boundary from.
    pub state_root_surface: M5StateRootSurface,
    /// The render / surface context.
    pub surface_context: M5InstallSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5InstallStateResolutionForm>,
    /// The published primary writable state roots (filesystem paths); empty means missing.
    pub writable_state_roots: String,
    /// The published policy roots (filesystem paths); empty means missing.
    pub policy_roots: String,
    /// The published rollback target (the full artifact graph); empty means missing.
    pub rollback_target: String,
    /// True when the rollback target restores the full artifact graph (never only the primary executable).
    pub rollback_targets_full_graph: bool,
    /// True when the boundary is truthful (never claims isolation over a hidden machine-global spill).
    pub boundary_is_truthful: bool,
    /// True when portable mode wrote hidden machine-global durable state.
    pub machine_global_spill_used: bool,
    /// True when any machine-global write is disclosed rather than hidden.
    pub machine_global_spill_disclosed: bool,
    /// True when a narrower delivery scope is asserted.
    pub narrower_scope_asserted: bool,
    /// True when an asserted narrower scope is explained rather than left implicit.
    pub narrower_scope_explained: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe state-root-boundary projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedStateRootBoundaryEntry {
    /// Stable identity of the state-root-boundary entry.
    pub entry_id: String,
    /// The stable install-profile ID this boundary binds to.
    pub profile_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve state isolation and ownership under coexistence.
    pub semantic_role_preserves_state_isolation_and_ownership_under_coexistence: bool,
    /// The state-root-surface token named by the entry.
    pub state_root_surface: String,
    /// Whether the state-root surface is classified into the resolved taxonomy.
    pub state_root_surface_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published primary writable state roots.
    pub writable_state_roots: String,
    /// The published policy roots.
    pub policy_roots: String,
    /// The published rollback target.
    pub rollback_target: String,
    /// Whether the rollback target restores the full artifact graph.
    pub rollback_targets_full_graph: bool,
    /// Whether the boundary is truthful.
    pub boundary_is_truthful: bool,
    /// Whether portable mode wrote hidden machine-global durable state.
    pub machine_global_spill_used: bool,
    /// Whether any machine-global write is disclosed.
    pub machine_global_spill_disclosed: bool,
    /// Whether a narrower delivery scope is asserted.
    pub narrower_scope_asserted: bool,
    /// Whether an asserted narrower scope is explained.
    pub narrower_scope_explained: bool,
    /// Whether the boundary stays truthful and complete (no hidden spill, full-graph rollback, explained
    /// scope).
    pub boundary_stays_truthful: bool,
    /// Whether the entry provides the complete writable-state-root / policy-root / rollback-target disclosure
    /// triple.
    pub provides_complete_disclosure_triple: bool,
    /// Degrade reason, if the entry could not read as a clean, truthful state.
    pub degrade_reason: Option<M5StateRootBoundaryEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5InstallStateNextAction,
    /// Whether the boundary is truthful on every claimed delivery profile (clean entry naming every fact).
    pub boundary_truthful_on_every_profile: bool,
}

impl M5ResolvedStateRootBoundaryEntry {
    /// Whether this state-root-boundary entry reads as a clean, truthful state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5InstallStateResolutionError {
    /// The install-topology-entry id was empty.
    EmptyInstallTopologyEntryId,
    /// The state-root-boundary-entry id was empty.
    EmptyStateRootBoundaryEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5InstallStateResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyInstallTopologyEntryId => "empty_install_topology_entry_id",
            Self::EmptyStateRootBoundaryEntryId => "empty_state_root_boundary_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5InstallStateResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 install-topology / state-root registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5InstallStateResolutionError {}

fn form_tokens(forms: &[M5InstallStateResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5InstallStateResolutionForm]) -> bool {
    let present: BTreeSet<M5InstallStateResolutionForm> = forms.iter().copied().collect();
    M5InstallStateResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved install-topology object publishes every required field: install mode (via a
/// classified scope), channel, updater owner, binary root, writable state roots, policy roots, and rollback
/// target. An unclassified scope or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn install_topology_object_is_complete(
    scope: M5DeliveryScope,
    channel: &str,
    updater_owner: &str,
    binary_root: &str,
    writable_state_roots: &str,
    policy_roots: &str,
    rollback_target: &str,
) -> bool {
    scope.is_classified()
        && !channel.trim().is_empty()
        && !updater_owner.trim().is_empty()
        && !binary_root.trim().is_empty()
        && !writable_state_roots.trim().is_empty()
        && !policy_roots.trim().is_empty()
        && !rollback_target.trim().is_empty()
}

/// Whether managed-versus-user scopes and side-by-side channels resolve to explicitly isolated state
/// namespaces: the scope must be classified, the namespaces must be marked isolated, and a scope that
/// coexists with a sibling channel must explain its cross-channel handoff. An unclassified scope, a
/// non-isolated namespace, or an unexplained coexistence handoff never matches.
pub fn state_namespace_is_isolated(
    scope: M5DeliveryScope,
    state_namespaces_isolated: bool,
    coexists_with_sibling_channel: bool,
    coexistence_handoff_explained: bool,
) -> bool {
    scope.is_classified()
        && state_namespaces_isolated
        && (!coexists_with_sibling_channel || coexistence_handoff_explained)
}

/// Whether a state-root boundary stays truthful and complete: the surface must be classified, the boundary
/// must be truthful, the rollback target must restore the full artifact graph, any machine-global spill must
/// be disclosed rather than hidden, and any asserted narrower scope must be explained.
pub fn state_root_boundary_stays_truthful(
    surface: M5StateRootSurface,
    boundary_is_truthful: bool,
    rollback_targets_full_graph: bool,
    machine_global_spill_used: bool,
    machine_global_spill_disclosed: bool,
    narrower_scope_asserted: bool,
    narrower_scope_explained: bool,
) -> bool {
    surface.is_classified()
        && boundary_is_truthful
        && rollback_targets_full_graph
        && (!machine_global_spill_used || machine_global_spill_disclosed)
        && (!narrower_scope_asserted || narrower_scope_explained)
}

/// Resolves an install-topology-registry entry so it stays bound to the shared topology registry: the entry
/// names its canonical token, semantic role, and delivery scope, covers all three resolution forms, publishes
/// a complete install-topology object (install mode, channel, updater owner, binary root, writable state
/// roots, policy roots, rollback target), keeps managed-versus-user scopes and side-by-side channels
/// isolated, and explains any coexistence handoff.
pub fn resolve_install_topology_entry(
    input: M5InstallTopologyEntryResolutionInput,
) -> Result<M5ResolvedInstallTopologyEntry, M5InstallStateResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5InstallStateResolutionError::EmptyInstallTopologyEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.profile_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.channel)
        || string_is_forbidden(&input.updater_owner)
        || string_is_forbidden(&input.binary_root)
        || string_is_forbidden(&input.writable_state_roots)
        || string_is_forbidden(&input.policy_roots)
        || string_is_forbidden(&input.rollback_target)
    {
        return Err(M5InstallStateResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = install_topology_object_is_complete(
        input.delivery_scope,
        &input.channel,
        &input.updater_owner,
        &input.binary_root,
        &input.writable_state_roots,
        &input.policy_roots,
        &input.rollback_target,
    );
    let namespace_isolated = state_namespace_is_isolated(
        input.delivery_scope,
        input.state_namespaces_isolated,
        input.coexists_with_sibling_channel,
        input.coexistence_handoff_explained,
    );
    let coexistence_unhandled =
        input.coexists_with_sibling_channel && !input.coexistence_handoff_explained;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5InstallTopologyEntryDegradeReason::InstallTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5InstallTopologyEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.delivery_scope.is_classified() {
        Some(M5InstallTopologyEntryDegradeReason::DeliveryScopeUnclassified)
    } else if !input.bound_to_registry {
        Some(M5InstallTopologyEntryDegradeReason::TopologyNotBoundToRegistry)
    } else if !object_complete {
        Some(M5InstallTopologyEntryDegradeReason::InstallTopologyObjectIncomplete)
    } else if !namespace_isolated {
        Some(M5InstallTopologyEntryDegradeReason::StateNamespaceReusedWithoutHandoff)
    } else if !all_forms {
        Some(M5InstallTopologyEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if coexistence_unhandled {
        Some(M5InstallTopologyEntryDegradeReason::CoexistenceHandoffUnexplained)
    } else if !input.proof_fresh {
        Some(M5InstallTopologyEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5InstallStateNextAction::ExpandInstallMeaning,
    };

    Ok(M5ResolvedInstallTopologyEntry {
        entry_id: input.entry_id,
        profile_id: input.profile_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_state_isolation_and_ownership_under_coexistence: input
            .semantic_role
            .must_preserve_state_isolation_and_ownership_under_coexistence(),
        delivery_scope: input.delivery_scope.as_str().to_owned(),
        delivery_scope_is_classified: input.delivery_scope.is_classified(),
        canonical_install_mode: input.delivery_scope.canonical_install_mode().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        channel: input.channel,
        updater_owner: input.updater_owner,
        binary_root: input.binary_root,
        writable_state_roots: input.writable_state_roots,
        policy_roots: input.policy_roots,
        rollback_target: input.rollback_target,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        install_topology_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        state_namespaces_isolated: input.state_namespaces_isolated,
        coexists_with_sibling_channel: input.coexists_with_sibling_channel,
        coexistence_handoff_explained: input.coexistence_handoff_explained,
        degrade_reason,
        next_action,
        topology_resolves_across_profiles: degrade_reason.is_none(),
    })
}

/// Resolves a state-root-boundary entry so its boundary stays truthful and complete: the entry names its
/// canonical token, semantic role, and state-root surface, covers all three resolution forms, provides the
/// writable-state-root / policy-root / rollback-target disclosure triple, and degrades honestly when portable
/// mode spills hidden machine-global durable state, a rollback narrows below the full artifact graph, or an
/// unexplained scope is asserted.
pub fn resolve_state_root_boundary_entry(
    input: M5StateRootBoundaryEntryResolutionInput,
) -> Result<M5ResolvedStateRootBoundaryEntry, M5InstallStateResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5InstallStateResolutionError::EmptyStateRootBoundaryEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.profile_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.writable_state_roots)
        || string_is_forbidden(&input.policy_roots)
        || string_is_forbidden(&input.rollback_target)
    {
        return Err(M5InstallStateResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let boundary_stays_truthful = state_root_boundary_stays_truthful(
        input.state_root_surface,
        input.boundary_is_truthful,
        input.rollback_targets_full_graph,
        input.machine_global_spill_used,
        input.machine_global_spill_disclosed,
        input.narrower_scope_asserted,
        input.narrower_scope_explained,
    );
    let provides_triple = input.state_root_surface.is_classified()
        && !input.writable_state_roots.trim().is_empty()
        && !input.policy_roots.trim().is_empty()
        && !input.rollback_target.trim().is_empty()
        && boundary_stays_truthful;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5StateRootBoundaryEntryDegradeReason::BoundaryTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5StateRootBoundaryEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.state_root_surface.is_classified() {
        Some(M5StateRootBoundaryEntryDegradeReason::StateRootSurfaceUnclassified)
    } else if !provides_triple {
        Some(M5StateRootBoundaryEntryDegradeReason::StateBoundaryUntruthfulOrIncomplete)
    } else if !all_forms {
        Some(M5StateRootBoundaryEntryDegradeReason::BoundaryFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5StateRootBoundaryEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5InstallStateNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedStateRootBoundaryEntry {
        entry_id: input.entry_id,
        profile_id: input.profile_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_state_isolation_and_ownership_under_coexistence: input
            .semantic_role
            .must_preserve_state_isolation_and_ownership_under_coexistence(),
        state_root_surface: input.state_root_surface.as_str().to_owned(),
        state_root_surface_is_classified: input.state_root_surface.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        writable_state_roots: input.writable_state_roots,
        policy_roots: input.policy_roots,
        rollback_target: input.rollback_target,
        rollback_targets_full_graph: input.rollback_targets_full_graph,
        boundary_is_truthful: input.boundary_is_truthful,
        machine_global_spill_used: input.machine_global_spill_used,
        machine_global_spill_disclosed: input.machine_global_spill_disclosed,
        narrower_scope_asserted: input.narrower_scope_asserted,
        narrower_scope_explained: input.narrower_scope_explained,
        boundary_stays_truthful,
        provides_complete_disclosure_triple: provides_triple,
        degrade_reason,
        next_action,
        boundary_truthful_on_every_profile: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved install-topology and state-root-boundary
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologyStateRootRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5InstallTopologyStateRootRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5InstallTopologyQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5InstallTopologyDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5InstallTopologyRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5InstallTopologyAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5InstallStateAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5InstallStateExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5InstallTopologyDowngradeTrigger>,
    /// Resolved install-topology-registry examples.
    pub install_topology_entries: Vec<M5ResolvedInstallTopologyEntry>,
    /// Resolved state-root-boundary examples.
    pub state_root_boundary_entries: Vec<M5ResolvedStateRootBoundaryEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the install-topology and
    /// state-root-boundaries domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: portable mode never writes hidden machine-global durable state. MUST be `false`.
    pub portable_mode_writes_hidden_machine_global_durable_state: bool,
    /// Hard invariant: a preview channel never reuses a stable state namespace without an explicit handoff.
    /// MUST be `false`.
    pub preview_channel_reuses_stable_state_namespace_without_handoff: bool,
    /// Hard invariant: rollback never targets only the primary executable while sidecars drift. MUST be
    /// `false`.
    pub rollback_targets_primary_executable_while_sidecars_drift: bool,
    /// Hard invariant: updater ownership or admin control is never hidden in a managed flow. MUST be `false`.
    pub hides_updater_ownership_or_admin_control_in_managed_flow: bool,
}

impl M5InstallTopologyStateRootRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5InstallStateAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5InstallStateAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5InstallStateExportField> =
            self.export_fields.iter().copied().collect();
        M5InstallStateExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.portable_mode_writes_hidden_machine_global_durable_state
            && !self.preview_channel_reuses_stable_state_namespace_without_handoff
            && !self.rollback_targets_primary_executable_while_sidecars_drift
            && !self.hides_updater_ownership_or_admin_control_in_managed_flow
    }

    /// True when a clean install-topology entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified delivery scope, publishes a complete install-topology object, keeps its state
    /// namespaces isolated, covers all three resolution forms, and explains any coexistence handoff.
    fn install_is_honest(ex: &M5ResolvedInstallTopologyEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.delivery_scope_is_classified
                && ex.install_topology_object_complete
                && ex.state_namespaces_isolated
                && ex.covers_all_resolution_forms
                && (!ex.coexists_with_sibling_channel || ex.coexistence_handoff_explained))
    }

    /// True when a clean state-root-boundary entry preserves truthful boundaries: it keeps a classified
    /// surface, provides the disclosure triple, stays truthful, and covers all three resolution forms.
    fn boundary_is_honest(ex: &M5ResolvedStateRootBoundaryEntry) -> bool {
        !ex.is_clean()
            || (ex.state_root_surface_is_classified
                && ex.provides_complete_disclosure_triple
                && ex.boundary_stays_truthful
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.install_topology_entries
            .iter()
            .all(Self::install_is_honest)
            && self
                .state_root_boundary_entries
                .iter()
                .all(Self::boundary_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologyStateRootRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Delivery-scope tokens (minted by this lane).
    pub delivery_scopes: Vec<String>,
    /// State-root-surface tokens (minted by this lane).
    pub state_root_surfaces: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Install-topology-entry degrade-reason tokens.
    pub install_topology_degrade_reasons: Vec<String>,
    /// State-root-boundary-entry degrade-reason tokens.
    pub state_root_boundary_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5InstallTopologyStateRootRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5InstallTopologyRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5InstallStateResolutionForm::ALL, |v| v.as_str()),
            delivery_scopes: tokens(&M5DeliveryScope::ALL, |v| v.as_str()),
            state_root_surfaces: tokens(&M5StateRootSurface::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5InstallSurfaceContext::ALL, |v| v.as_str()),
            install_topology_degrade_reasons: tokens(
                &M5InstallTopologyEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            state_root_boundary_degrade_reasons: tokens(
                &M5StateRootBoundaryEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5InstallStateAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5InstallStateNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5InstallStateExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5InstallTopologyConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5InstallTopologyStateRootRegistriesGovernanceReview {
    /// The topology registry names a canonical token, semantic role, and delivery scope for every entry.
    pub topology_registry_names_token_role_and_scope: bool,
    /// Every claimed delivery profile resolves to one stable install-topology object from the shared registry,
    /// not per-surface path derivation.
    pub profile_resolves_to_stable_object_from_shared_registry: bool,
    /// Install mode, channel, updater owner, binary root, writable state roots, policy roots, and rollback
    /// target are published for every resolved profile.
    pub install_mode_owner_roots_and_rollback_published: bool,
    /// Managed-versus-user scopes and side-by-side channels resolve to explicitly isolated state namespaces.
    pub managed_and_user_scopes_and_channels_isolated: bool,
    /// State-root boundaries stay truthful and complete, with full-graph rollback and disclosed spill.
    pub state_root_boundaries_truthful_and_complete: bool,
    /// Portable mode never spills hidden machine-global durable state and rollback never narrows below the full
    /// artifact graph.
    pub portable_mode_never_spills_and_rollback_full_graph: bool,
    /// Every install-topology and state-root entry covers the canonical / accessible / audit resolution forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Install-topology and state-root behavior stay bound to the shared registries rather than hand-copied per
    /// profile.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// About, update, diagnostics, admin, docs, and support read a single install-topology source.
    pub about_update_diagnostics_admin_read_single_source: bool,
    /// A reused state namespace, an incomplete object, or a hidden spill is caught by fixtures before release
    /// evidence turns green.
    pub topology_or_boundary_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologyStateRootRegistriesConsumerProjection {
    /// About and update consume the shared install-topology registry.
    pub about_and_update_consume_shared_registries: bool,
    /// Diagnostics and admin consume the shared state-root boundaries.
    pub diagnostics_and_admin_consume_shared_registries: bool,
    /// Installers consume the shared binary and state roots.
    pub installers_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical install-topology and state-root-boundaries domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical install-topology / state-root registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologyStateRootRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologyStateRootRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting install-topology audit for the lane.
    pub install_topology_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5InstallTopologyStateRootRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5InstallTopologyStateRootRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5InstallTopologyStateRootRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InstallTopologyStateRootRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InstallTopologyStateRootRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InstallTopologyStateRootRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5InstallTopologyStateRootRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5InstallTopologyStateRootRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 install-topology and state-root registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5InstallTopologyStateRootRegistriesPacket {
    /// Record kind; must equal [`M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5InstallTopologyStateRootRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5InstallTopologyStateRootRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5InstallTopologyStateRootRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5InstallTopologyStateRootRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5InstallTopologyStateRootRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5InstallTopologyStateRootRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5InstallTopologyStateRootRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5InstallTopologyStateRootRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_VERSION,
            packet_id: input.packet_id,
            registries_label: input.registries_label,
            registry_rows: input.registry_rows,
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

    /// Validates the registries-packet invariants.
    pub fn validate(&self) -> Vec<M5InstallTopologyStateRootRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_RECORD_KIND {
            violations.push(M5InstallTopologyStateRootRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5InstallTopologyStateRootRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5InstallTopologyStateRootRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5InstallTopologyStateRootRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 install-topology / state-root registries packet serializes"),
        ) {
            violations.push(M5InstallTopologyStateRootRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 install-topology / state-root registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,install_topology_entries,state_root_boundary_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .install_topology_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.state_root_boundary_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.install_topology_entries.len(),
                row.state_root_boundary_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Install-Topology and State-Root Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Delivery scopes: {}\n",
            self.vocabulary_set.delivery_scopes.join(", ")
        ));
        out.push_str(&format!(
            "- Resolution forms: {}\n",
            self.vocabulary_set.resolution_forms.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Consumer surfaces\n\n");
        for row in &self.registry_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Install-topology entries: {} / state-root entries: {}\n",
                row.install_topology_entries.len(),
                row.state_root_boundary_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-family state-root boundary reference table generated from the registry, so docs and
    /// admin runbooks render the same install-mode / updater-owner / binary-root / writable-state-roots /
    /// policy-roots / rollback-target truth the resolvers produced rather than a hand-copied path table. Only
    /// clean, registry-bound install-topology entries are listed.
    pub fn render_state_root_boundary_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| profile_id | install_mode | updater_owner | binary_root | writable_state_roots | policy_roots | rollback_target |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.install_topology_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | {} | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.profile_id,
                    ex.canonical_install_mode,
                    ex.updater_owner,
                    ex.binary_root,
                    ex.writable_state_roots,
                    ex.policy_roots,
                    ex.rollback_target
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5InstallTopologyStateRootRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5InstallTopologyStateRootRegistriesViolation>),
}

impl fmt::Display for M5InstallTopologyStateRootRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 install-topology / state-root registries export parse failed: {error}"
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
                    "m5 install-topology / state-root registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5InstallTopologyStateRootRegistriesArtifactError {}

/// Validation failures emitted by [`M5InstallTopologyStateRootRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5InstallTopologyStateRootRegistriesViolation {
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
    /// The registries packet declares no rows.
    NoRegistryRows,
    /// A registry row is incomplete.
    RegistryRowIncomplete,
    /// A registry row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A registry row omits one of the mandatory export fields.
    MandatoryExportFieldMissing,
    /// A registry row does not point at both the install-topology and state-root-boundaries domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, isolation-losing, field-incomplete,
    /// form-incomplete, or a state-root entry missing the disclosure triple).
    DishonestExample,
    /// A registry row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Stable-object-resolution is not proven: clean install-topology entries do not cover the canonical
    /// delivery scopes or the first About / update / diagnostics / admin / support surfaces, no
    /// object-incomplete example degrades, or a clean install entry published an incomplete object.
    StableObjectResolutionNotProven,
    /// State-namespace-isolation is not proven: no reused-namespace example and no unbound example degrade, no
    /// clean isolated install entry is present, or a clean install entry lost namespace isolation or is
    /// unbound.
    StateNamespaceIsolationNotProven,
    /// State-boundary-truth is not proven: clean state-root entries do not cover the canonical portable /
    /// offline / diagnostics surfaces with full resolution-form coverage while providing the disclosure triple,
    /// no untruthful-or-incomplete or form-incomplete example degrades, or a clean state-root entry is missing
    /// the triple.
    StateBoundaryTruthNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5InstallTopologyStateRootRegistriesViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::NoRegistryRows => "no_registry_rows",
            Self::RegistryRowIncomplete => "registry_row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportFieldMissing => "mandatory_export_field_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::ExamplesMissing => "examples_missing",
            Self::DishonestExample => "dishonest_example",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::StableObjectResolutionNotProven => "stable_object_resolution_not_proven",
            Self::StateNamespaceIsolationNotProven => "state_namespace_isolation_not_proven",
            Self::StateBoundaryTruthNotProven => "state_boundary_truth_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_install_topology_and_state_root_registries_export() -> Result<
    M5InstallTopologyStateRootRegistriesPacket,
    M5InstallTopologyStateRootRegistriesArtifactError,
> {
    let packet: M5InstallTopologyStateRootRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-install-topology-and-state-root-registries-proof/support_export.json"
        )
    ))
    .map_err(M5InstallTopologyStateRootRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5InstallTopologyStateRootRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5InstallTopologyStateRootRegistriesPacket,
    violations: &mut Vec<M5InstallTopologyStateRootRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_DOC_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
        M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF,
        M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5InstallTopologyStateRootRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5InstallTopologyStateRootRegistriesPacket,
    violations: &mut Vec<M5InstallTopologyStateRootRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5InstallTopologyStateRootRegistriesViolation::NoRegistryRows);
        return;
    }
    for row in &packet.registry_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.deployment_lines.is_empty()
            || row.required_labels.is_empty()
            || row.accessibility_routes.is_empty()
            || row.downgrade_triggers.is_empty()
            || row.required_proof_packet_refs.is_empty()
        {
            violations.push(M5InstallTopologyStateRootRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5InstallTopologyStateRootRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations
                .push(M5InstallTopologyStateRootRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_INSTALL_TOPOLOGY_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF)
        {
            violations.push(M5InstallTopologyStateRootRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.install_topology_entries.is_empty() || row.state_root_boundary_entries.is_empty() {
            violations.push(M5InstallTopologyStateRootRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5InstallTopologyStateRootRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5InstallTopologyStateRootRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5InstallTopologyStateRootRegistriesPacket,
    violations: &mut Vec<M5InstallTopologyStateRootRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.topology_registry_names_token_role_and_scope,
        review.profile_resolves_to_stable_object_from_shared_registry,
        review.install_mode_owner_roots_and_rollback_published,
        review.managed_and_user_scopes_and_channels_isolated,
        review.state_root_boundaries_truthful_and_complete,
        review.portable_mode_never_spills_and_rollback_full_graph,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.about_update_diagnostics_admin_read_single_source,
        review.topology_or_boundary_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5InstallTopologyStateRootRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5InstallTopologyStateRootRegistriesPacket,
    violations: &mut Vec<M5InstallTopologyStateRootRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.about_and_update_consume_shared_registries,
        projection.diagnostics_and_admin_consume_shared_registries,
        projection.installers_consume_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations
                .push(M5InstallTopologyStateRootRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5InstallTopologyStateRootRegistriesPacket,
    violations: &mut Vec<M5InstallTopologyStateRootRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5InstallTopologyStateRootRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5InstallTopologyStateRootRegistriesPacket,
    violations: &mut Vec<M5InstallTopologyStateRootRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.install_topology_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5InstallTopologyStateRootRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted
/// by governance bools.
fn validate_acceptance_criteria(
    packet: &M5InstallTopologyStateRootRegistriesPacket,
    violations: &mut Vec<M5InstallTopologyStateRootRegistriesViolation>,
) {
    let installs = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.install_topology_entries.iter())
    };
    let boundaries = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.state_root_boundary_entries.iter())
    };

    // AC1: every claimed delivery profile resolves to one stable install-topology object with channel / owner
    // / root / rollback fields. Clean install entries cover the canonical delivery scopes and the first About /
    // update / diagnostics / admin / support surfaces, an object-incomplete example degrades, and no clean
    // install entry published an incomplete object.
    let clean_scopes: BTreeSet<String> = installs()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.delivery_scope.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = installs()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let scopes_covered = M5DeliveryScope::CANONICAL_SCOPES
        .iter()
        .all(|s| clean_scopes.contains(s.as_str()));
    let first_surfaces_covered = M5InstallSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = installs().any(|ex| {
        ex.degrade_reason
            == Some(M5InstallTopologyEntryDegradeReason::InstallTopologyObjectIncomplete)
    });
    let no_clean_incomplete =
        !installs().any(|ex| ex.is_clean() && !ex.install_topology_object_complete);
    if !(scopes_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations
            .push(M5InstallTopologyStateRootRegistriesViolation::StableObjectResolutionNotProven);
    }

    // AC2: diagnostics and support packets report install topology and state-root boundaries without manual
    // reconstruction — shared-versus-isolated state is explicit. A reused-namespace example degrades, an
    // unbound example degrades, at least one clean isolated install entry is present, and no clean install
    // entry lost namespace isolation or is unbound.
    let namespace_reused_degrades = installs().any(|ex| {
        ex.degrade_reason
            == Some(M5InstallTopologyEntryDegradeReason::StateNamespaceReusedWithoutHandoff)
    });
    let unbound_degrades = installs().any(|ex| {
        ex.degrade_reason == Some(M5InstallTopologyEntryDegradeReason::TopologyNotBoundToRegistry)
    });
    let isolated_clean_install = installs().any(|ex| ex.is_clean() && ex.state_namespaces_isolated);
    let no_clean_unbound = !installs().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unisolated = !installs().any(|ex| ex.is_clean() && !ex.state_namespaces_isolated);
    if !(namespace_reused_degrades
        && unbound_degrades
        && isolated_clean_install
        && no_clean_unbound
        && no_clean_unisolated)
    {
        violations
            .push(M5InstallTopologyStateRootRegistriesViolation::StateNamespaceIsolationNotProven);
    }

    // AC3: the suite fails when a platform cannot explain shared-versus-isolated state for the active profile.
    // Clean state-root entries cover every canonical portable / offline / diagnostics surface with full
    // resolution-form coverage while providing the disclosure triple, an untruthful-or-incomplete example
    // degrades, a form-incomplete example degrades, and no clean state-root entry is missing the triple.
    let clean_boundary_surfaces: BTreeSet<String> = boundaries()
        .filter(|ex| {
            ex.is_clean()
                && ex.state_root_surface_is_classified
                && ex.provides_complete_disclosure_triple
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.state_root_surface.clone())
        .collect();
    let boundary_surfaces_covered = M5StateRootSurface::CANONICAL_SURFACES
        .iter()
        .all(|s| clean_boundary_surfaces.contains(s.as_str()));
    let untruthful_degrades = boundaries().any(|ex| {
        ex.degrade_reason
            == Some(M5StateRootBoundaryEntryDegradeReason::StateBoundaryUntruthfulOrIncomplete)
    });
    let form_incomplete_degrades = boundaries().any(|ex| {
        ex.degrade_reason
            == Some(M5StateRootBoundaryEntryDegradeReason::BoundaryFormCoverageIncomplete)
    });
    let no_clean_missing_triple =
        !boundaries().any(|ex| ex.is_clean() && !ex.provides_complete_disclosure_triple);
    if !(boundary_surfaces_covered
        && untruthful_degrades
        && form_incomplete_degrades
        && no_clean_missing_triple)
    {
        violations.push(M5InstallTopologyStateRootRegistriesViolation::StateBoundaryTruthNotProven);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray comma.
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

/// The install-topology families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5InstallTopologyFamily; 5] = [
    M5InstallTopologyFamily::PerUserManaged,
    M5InstallTopologyFamily::PerMachineManaged,
    M5InstallTopologyFamily::SideBySideStablePreview,
    M5InstallTopologyFamily::PortableMode,
    M5InstallTopologyFamily::OfflineAirgapBundle,
];
