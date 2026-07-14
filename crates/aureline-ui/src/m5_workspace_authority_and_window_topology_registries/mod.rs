//! Implemented M5 workspace-authority and window-topology registries.
//!
//! The frozen [window-restore matrix][matrix] names Aureline's five workspace-restore families and locks
//! their controlled vocabulary. This module is the first implement lane for the concrete workspace-ownership
//! resolution flows: it turns the *shared workspace authority* grammar and the *window-local topology* grammar
//! into registry resolvers that produce export-safe, honest projections. Every claimed M5 workspace then
//! resolves to one stable workspace-authority object — the authority scope, the windows it backs, the stable
//! versioned pane-tree IDs, the shared dirty-buffer / save / checkpoint state, the authoritative workspace
//! state root, and the profile-defaults reference kept distinct from it — that the shell, recovery,
//! diagnostics, admin, and support / export surfaces can inspect without manual reconstruction, so
//! shared-versus-window-local state is explicit (never ad hoc reconstruction), multiple windows share one
//! authority while keeping selection and focus window-local, window topology never absorbs shared authority
//! into private window state, and a workspace that cannot explain which state is shared and which is
//! window-local degrades honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Publish one stable workspace-authority object per workspace.** [`resolve_workspace_authority_entry`]
//!   refuses to read as a clean, registry-bound authority entry unless it names a canonical registry token, a
//!   classified [authority scope][M5WorkspaceAuthorityScope], a window-restore role, covers every
//!   [resolution form][M5WindowStateResolutionForm] (the canonical object, the accessible summary, and the
//!   audit record), publishes every authority field (backing windows, stable pane-tree IDs, shared
//!   dirty-buffer state, shared save / checkpoint state, authoritative workspace state root, and the distinct
//!   profile-defaults reference), keeps window-local selection and focus window-local, and preserves
//!   window-local history when one authority backs multiple windows; otherwise it degrades.
//! * **Keep window-local state from overwriting shared workspace authority.**
//!   [`window_local_state_stays_window_local`] rejects an entry whose window-local selection or focus overwrites
//!   the shared authority so it degrades to
//!   [`M5WorkspaceAuthorityEntryDegradeReason::WindowLocalStateOverwritesSharedAuthority`], and the
//!   `history_preserved_under_shared_authority` invariant degrades a multi-window authority that loses its
//!   window-local focus / selection history.
//! * **Keep window topology from absorbing shared authority into private window state.**
//!   [`resolve_window_topology_entry`] names a classified [topology surface][M5WindowTopologySurface],
//!   requires the window-local pane-tree / focus-history / display-affinity disclosure triple, covers every
//!   resolution form, and degrades to
//!   [`M5WindowTopologyEntryDegradeReason::WindowTopologyMergesOrLeaksSharedAuthority`] when window topology
//!   privately copies shared authority state without disclosure, merges authority and topology into one opaque
//!   blob, or lets profile defaults silently override authoritative topology, so a window can never read as
//!   independent when it has quietly become the workspace authority.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5WindowRestoreRole`] role vocabulary and
//! the [`M5WindowRestoreConsumerSurface`] consumer-surface taxonomy — so the shell, recovery, diagnostics,
//! admin, workspace, session, docs, CLI, and support surfaces can never fork their own workspace-ownership
//! meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_window_restore_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_workspace_authority_and_window_topology_registries,
    seeded_m5_workspace_authority_and_window_topology_registries_auxiliary_window_topology_preview_narrowed,
    seeded_m5_workspace_authority_and_window_topology_registries_multi_window_shared_authority_beta_narrowed,
    M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_window_restore_matrix::{
    M5WindowRestoreAccessibilityRoute, M5WindowRestoreConsumerSurface,
    M5WindowRestoreDeploymentLine, M5WindowRestoreDowngradeTrigger, M5WindowRestoreFamily,
    M5WindowRestoreQualificationClass, M5WindowRestoreRequiredLabel, M5WindowRestoreRole,
    M5_RESTORE_FIDELITY_SCHEMA_REF, M5_WINDOW_RESTORE_MATRIX_DOC_REF,
    M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF, M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5WorkspaceAuthorityWindowTopologyRegistriesPacket`].
pub const M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_workspace_authority_and_window_topology_registries";

/// Schema version for M5 workspace-authority / window-topology registry records.
pub const M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_SCHEMA_REF: &str =
    "schemas/shell/m5-workspace-authority-and-window-topology-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_DOC_REF: &str =
    "docs/recovery/m5_workspace_authority_and_window_topology_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-workspace-authority-and-window-topology-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-workspace-authority-and-window-topology-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-workspace-authority-and-window-topology-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-workspace-authority-and-window-topology-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5WorkspaceAuthorityWindowTopologyRegistriesConsumerSurface =
    M5WindowRestoreConsumerSurface;

/// One of the three resolution forms every workspace-authority or window-topology entry must hold across so its
/// truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// workspace-authority and window-topology *families* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowStateResolutionForm {
    /// The canonical resolved workspace-authority / window-topology object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved ownership discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved ownership inspectable off-renderer.
    AuditRecord,
}

impl M5WindowStateResolutionForm {
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

/// Controlled workspace-authority scope a workspace-authority entry resolves, so the canonical authority model
/// shares one registry rather than a hand-copied per-window ownership assumption. Minted by this lane because
/// the frozen matrix carries the workspace-restore families but not the concrete single-window-versus-shared
/// authority model an authority entry resolves against. Every classified scope carries its canonical authority
/// mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceAuthorityScope {
    /// A single-window authority (one window backed by one authority; no cross-window sharing).
    SingleWindowAuthorityScope,
    /// A multi-window shared authority (one authority backs multiple windows; selection and focus stay
    /// window-local).
    MultiWindowSharedAuthorityScope,
    /// A detached / auxiliary window sharing the workspace authority (a pulled-out pane or auxiliary surface).
    DetachedAuxiliaryWindowScope,
    /// The authority scope is unclassified, which is disallowed.
    ScopeUnclassified,
}

impl M5WorkspaceAuthorityScope {
    /// Every authority scope, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::SingleWindowAuthorityScope,
        Self::MultiWindowSharedAuthorityScope,
        Self::DetachedAuxiliaryWindowScope,
        Self::ScopeUnclassified,
    ];

    /// The three canonical authority scopes every claimed M5 workspace resolves against.
    pub const CANONICAL_SCOPES: [Self; 3] = [
        Self::SingleWindowAuthorityScope,
        Self::MultiWindowSharedAuthorityScope,
        Self::DetachedAuxiliaryWindowScope,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleWindowAuthorityScope => "single_window_authority_scope",
            Self::MultiWindowSharedAuthorityScope => "multi_window_shared_authority_scope",
            Self::DetachedAuxiliaryWindowScope => "detached_auxiliary_window_scope",
            Self::ScopeUnclassified => "scope_unclassified",
        }
    }

    /// Whether the scope is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::ScopeUnclassified)
    }

    /// The canonical authority mode for this scope.
    pub const fn canonical_authority_mode(self) -> &'static str {
        match self {
            Self::SingleWindowAuthorityScope => "single_window_authority",
            Self::MultiWindowSharedAuthorityScope => "multi_window_shared_authority",
            Self::DetachedAuxiliaryWindowScope => "detached_auxiliary_window_authority",
            Self::ScopeUnclassified => "",
        }
    }

    /// Whether this scope backs more than one window and so must keep window-local history explicitly.
    pub const fn backs_multiple_windows(self) -> bool {
        matches!(
            self,
            Self::MultiWindowSharedAuthorityScope | Self::DetachedAuxiliaryWindowScope
        )
    }
}

/// Controlled window-topology surface a window-topology entry must resolve its topology from, so a window
/// topology shares one registry rather than a hand-copied per-window layout. Minted by this lane, tracking the
/// window surfaces the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowTopologySurface {
    /// The primary window-topology surface.
    PrimaryWindowTopology,
    /// The auxiliary / detached window-topology surface.
    AuxiliaryWindowTopology,
    /// The diagnostics / admin window-topology inspection surface.
    DiagnosticsInspectionTopology,
    /// The window-topology surface is unclassified, which is disallowed.
    SurfaceUnclassified,
}

impl M5WindowTopologySurface {
    /// Every window-topology surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PrimaryWindowTopology,
        Self::AuxiliaryWindowTopology,
        Self::DiagnosticsInspectionTopology,
        Self::SurfaceUnclassified,
    ];

    /// The three canonical surfaces every window topology must stay distinct across.
    pub const CANONICAL_SURFACES: [Self; 3] = [
        Self::PrimaryWindowTopology,
        Self::AuxiliaryWindowTopology,
        Self::DiagnosticsInspectionTopology,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimaryWindowTopology => "primary_window_topology",
            Self::AuxiliaryWindowTopology => "auxiliary_window_topology",
            Self::DiagnosticsInspectionTopology => "diagnostics_inspection_topology",
            Self::SurfaceUnclassified => "surface_unclassified",
        }
    }

    /// Whether the window-topology surface is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::SurfaceUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a workspace-authority or
/// window-topology token's meaning stays stable whether it appears in the shell, recovery, diagnostics, admin,
/// or a support / export form. Minted by this lane, tracking the first-consumer surfaces the implementation
/// requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowRestoreSurfaceContext {
    /// The shell surface.
    ShellSurface,
    /// The recovery surface.
    RecoverySurface,
    /// The diagnostics surface.
    DiagnosticsSurface,
    /// The admin surface.
    AdminSurface,
    /// The support / export form surface.
    SupportOrExportForm,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5WindowRestoreSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ShellSurface,
        Self::RecoverySurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::ShellSurface,
        Self::RecoverySurface,
        Self::DiagnosticsSurface,
        Self::AdminSurface,
        Self::SupportOrExportForm,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellSurface => "shell_surface",
            Self::RecoverySurface => "recovery_surface",
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

/// One mandatory rendered part a workspace-authority or window-topology entry must be able to show, so no
/// authority scope, backing window, pane-tree ID, shared dirty-buffer / checkpoint state, window-local focus
/// history, display-affinity hint, or registry fact is left implicit behind a hand-copied per-window
/// assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowStateAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The authority scope the entry resolves (workspace-authority entry).
    WorkspaceAuthorityScope,
    /// The backing windows and stable pane-tree IDs the entry publishes (workspace-authority entry).
    BackingWindowsAndPaneTree,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The shared dirty-buffer / save / checkpoint state the entry publishes (workspace-authority entry).
    SharedDirtyBufferAndCheckpointState,
    /// The window-local focus / selection history the entry publishes (both entries).
    WindowLocalFocusHistory,
    /// The display-affinity / machine hint kept distinct from authoritative state (both entries).
    DisplayAffinityHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved ownership or topology (both entries).
    PlainLanguageMeaning,
}

impl M5WindowStateAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::WorkspaceAuthorityScope,
        Self::BackingWindowsAndPaneTree,
        Self::ResolutionFormCoverage,
        Self::SharedDirtyBufferAndCheckpointState,
        Self::WindowLocalFocusHistory,
        Self::DisplayAffinityHint,
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
            Self::WorkspaceAuthorityScope => "workspace_authority_scope",
            Self::BackingWindowsAndPaneTree => "backing_windows_and_pane_tree",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::SharedDirtyBufferAndCheckpointState => "shared_dirty_buffer_and_checkpoint_state",
            Self::WindowLocalFocusHistory => "window_local_focus_history",
            Self::DisplayAffinityHint => "display_affinity_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// authority, a window topology, or a degraded workspace-authority / window-topology entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowStateNextAction {
    /// Expand the resolved authority's or topology's plain-language meaning.
    ExpandOwnershipMeaning,
    /// Inspect the authority scope or window-topology surface the entry resolves.
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

impl M5WindowStateNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandOwnershipMeaning,
        Self::InspectScopeOrSurface,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandOwnershipMeaning => "expand_ownership_meaning",
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
pub enum M5WindowStateExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The window-restore families covered.
    WindowRestoreFamilies,
    /// The authority scopes carried.
    WorkspaceAuthorityScopes,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The window-topology surfaces carried.
    WindowTopologySurfaces,
    /// The render / surface context.
    SurfaceContext,
    /// The authority modes carried.
    AuthorityModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5WindowStateExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::WindowRestoreFamilies,
        Self::WorkspaceAuthorityScopes,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::WindowTopologySurfaces,
        Self::SurfaceContext,
        Self::AuthorityModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::WindowRestoreFamilies,
        Self::WorkspaceAuthorityScopes,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::WindowRestoreFamilies => "window_restore_families",
            Self::WorkspaceAuthorityScopes => "workspace_authority_scopes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::WindowTopologySurfaces => "window_topology_surfaces",
            Self::SurfaceContext => "surface_context",
            Self::AuthorityModes => "authority_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a workspace-authority entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, isolation-losing, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WorkspaceAuthorityEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the authority means.
    AuthorityTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The authority scope is unclassified (not in the resolved taxonomy).
    AuthorityScopeUnclassified,
    /// The behavior is a hand-copied per-window assumption instead of tracing to the canonical registry.
    AuthorityNotBoundToRegistry,
    /// The resolved workspace-authority object is incomplete: backing windows, stable pane-tree IDs, shared
    /// dirty-buffer state, shared save / checkpoint state, authoritative state root, or the distinct
    /// profile-defaults reference is unstated.
    WorkspaceAuthorityObjectIncomplete,
    /// A window-local selection or focus overwrote the shared workspace authority.
    WindowLocalStateOverwritesSharedAuthority,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// The authority backs multiple windows but window-local focus / selection history is not preserved.
    SharedAuthorityHistoryNotPreserved,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5WorkspaceAuthorityEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::AuthorityTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::AuthorityScopeUnclassified,
        Self::AuthorityNotBoundToRegistry,
        Self::WorkspaceAuthorityObjectIncomplete,
        Self::WindowLocalStateOverwritesSharedAuthority,
        Self::ResolutionFormCoverageIncomplete,
        Self::SharedAuthorityHistoryNotPreserved,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthorityTokenUnstated => "authority_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::AuthorityScopeUnclassified => "authority_scope_unclassified",
            Self::AuthorityNotBoundToRegistry => "authority_not_bound_to_registry",
            Self::WorkspaceAuthorityObjectIncomplete => "workspace_authority_object_incomplete",
            Self::WindowLocalStateOverwritesSharedAuthority => {
                "window_local_state_overwrites_shared_authority"
            }
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::SharedAuthorityHistoryNotPreserved => "shared_authority_history_not_preserved",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5WindowStateNextAction {
        match self {
            Self::AuthorityTokenUnstated | Self::AuthorityNotBoundToRegistry => {
                M5WindowStateNextAction::TraceCanonicalRegistry
            }
            Self::AuthorityScopeUnclassified
            | Self::WorkspaceAuthorityObjectIncomplete
            | Self::WindowLocalStateOverwritesSharedAuthority => {
                M5WindowStateNextAction::InspectScopeOrSurface
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5WindowStateNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::SharedAuthorityHistoryNotPreserved
            | Self::ProofStale => M5WindowStateNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WindowRestoreDowngradeTrigger {
        match self {
            Self::AuthorityTokenUnstated | Self::ResolutionFormCoverageIncomplete => {
                M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::AuthorityScopeUnclassified => {
                M5WindowRestoreDowngradeTrigger::WorkspaceAuthorityUnstated
            }
            Self::AuthorityNotBoundToRegistry => {
                M5WindowRestoreDowngradeTrigger::WindowTopologyBoundaryDriftedBySurface
            }
            Self::WorkspaceAuthorityObjectIncomplete => {
                M5WindowRestoreDowngradeTrigger::WorkspaceAuthorityUnstated
            }
            Self::WindowLocalStateOverwritesSharedAuthority
            | Self::SharedAuthorityHistoryNotPreserved => {
                M5WindowRestoreDowngradeTrigger::MergedWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob
            }
            Self::ProofStale => M5WindowRestoreDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a window-topology entry degraded below a clean, distinct state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowTopologyEntryDegradeReason {
    /// The canonical registry token name is unstated.
    TopologyTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The window-topology surface is unclassified (not in the resolved taxonomy).
    WindowTopologySurfaceUnclassified,
    /// The topology merges or leaks shared authority — window topology privately copied shared authority state
    /// without disclosure, merged authority and topology into one opaque blob, or let profile defaults override
    /// authoritative topology without explanation, or it dropped the window-local pane-tree / focus-history /
    /// display-affinity disclosure triple.
    WindowTopologyMergesOrLeaksSharedAuthority,
    /// The canonical / accessible / audit resolution-form coverage of the topology is incomplete.
    TopologyFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5WindowTopologyEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TopologyTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::WindowTopologySurfaceUnclassified,
        Self::WindowTopologyMergesOrLeaksSharedAuthority,
        Self::TopologyFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyTokenUnstated => "topology_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::WindowTopologySurfaceUnclassified => "window_topology_surface_unclassified",
            Self::WindowTopologyMergesOrLeaksSharedAuthority => {
                "window_topology_merges_or_leaks_shared_authority"
            }
            Self::TopologyFormCoverageIncomplete => "topology_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5WindowStateNextAction {
        match self {
            Self::TopologyTokenUnstated => M5WindowStateNextAction::TraceCanonicalRegistry,
            Self::WindowTopologySurfaceUnclassified
            | Self::WindowTopologyMergesOrLeaksSharedAuthority => {
                M5WindowStateNextAction::InspectScopeOrSurface
            }
            Self::TopologyFormCoverageIncomplete => {
                M5WindowStateNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5WindowStateNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WindowRestoreDowngradeTrigger {
        match self {
            Self::TopologyTokenUnstated => M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated,
            Self::SurfaceContextUnresolved | Self::WindowTopologySurfaceUnclassified => {
                M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::WindowTopologyMergesOrLeaksSharedAuthority => {
                M5WindowRestoreDowngradeTrigger::MergedWorkspaceAuthorityAndWindowTopologyIntoOneOpaqueBlob
            }
            Self::TopologyFormCoverageIncomplete => {
                M5WindowRestoreDowngradeTrigger::WindowTopologyBoundaryDriftedBySurface
            }
            Self::ProofStale => M5WindowRestoreDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_workspace_authority_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WorkspaceAuthorityEntryResolutionInput {
    /// Stable identity of the workspace-authority-registry entry.
    pub entry_id: String,
    /// The stable workspace ID this authority binds to (e.g. `workspace.acme`); empty means unstated.
    pub workspace_id: String,
    /// The canonical registry token name (e.g. `workspace.authority.shared`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5WindowRestoreRole,
    /// The authority scope this entry resolves.
    pub authority_scope: M5WorkspaceAuthorityScope,
    /// The render / surface context.
    pub surface_context: M5WindowRestoreSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5WindowStateResolutionForm>,
    /// The published backing window IDs this authority backs; empty means unstated.
    pub backing_window_ids: String,
    /// The published stable, versioned, attributable pane-tree IDs; empty means unstated.
    pub stable_pane_tree_ids: String,
    /// The published shared dirty-buffer state reference; empty means unstated.
    pub shared_dirty_buffer_state: String,
    /// The published shared save / checkpoint state reference; empty means unstated.
    pub shared_save_checkpoint_state: String,
    /// The published authoritative workspace state root; empty means unstated.
    pub authority_state_root: String,
    /// The published profile-defaults / machine-display-hints reference kept distinct from authoritative state;
    /// empty means unstated.
    pub profile_defaults_ref: String,
    /// True when the behavior traces to the shared authority registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when window-local selection and focus stay window-local and never overwrite shared authority (a hard
    /// invariant when `false`).
    pub window_local_state_isolated: bool,
    /// True when this authority backs multiple windows.
    pub shares_authority_across_windows: bool,
    /// True when window-local focus / selection history is preserved distinctly under shared authority.
    pub window_local_history_preserved: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe workspace-authority-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedWorkspaceAuthorityEntry {
    /// Stable identity of the workspace-authority-registry entry.
    pub entry_id: String,
    /// The stable workspace ID this authority binds to.
    pub workspace_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve window-local selection and no-rerun under shared authority.
    pub semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: bool,
    /// The authority-scope token named by the entry.
    pub authority_scope: String,
    /// Whether the authority scope is classified into the resolved taxonomy.
    pub authority_scope_is_classified: bool,
    /// The canonical authority mode for the entry's scope.
    pub canonical_authority_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published backing window IDs.
    pub backing_window_ids: String,
    /// The published stable pane-tree IDs.
    pub stable_pane_tree_ids: String,
    /// The published shared dirty-buffer state.
    pub shared_dirty_buffer_state: String,
    /// The published shared save / checkpoint state.
    pub shared_save_checkpoint_state: String,
    /// The published authoritative workspace state root.
    pub authority_state_root: String,
    /// The published profile-defaults / machine-display-hints reference.
    pub profile_defaults_ref: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved workspace-authority object publishes every required field.
    pub workspace_authority_object_complete: bool,
    /// Whether the entry traces to the shared authority registry.
    pub bound_to_registry: bool,
    /// Whether window-local selection and focus stay window-local.
    pub window_local_state_isolated: bool,
    /// Whether this authority backs multiple windows.
    pub shares_authority_across_windows: bool,
    /// Whether window-local focus / selection history is preserved distinctly.
    pub window_local_history_preserved: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5WorkspaceAuthorityEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5WindowStateNextAction,
    /// Whether the authority resolves to one stable object across every claimed workspace (clean entry naming
    /// every fact).
    pub authority_resolves_across_workspaces: bool,
}

impl M5ResolvedWorkspaceAuthorityEntry {
    /// Whether this workspace-authority entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_window_topology_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WindowTopologyEntryResolutionInput {
    /// Stable identity of the window-topology entry.
    pub entry_id: String,
    /// The stable window ID this topology binds to; empty means unstated.
    pub window_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5WindowRestoreRole,
    /// The window-topology surface this entry must resolve its topology from.
    pub topology_surface: M5WindowTopologySurface,
    /// The render / surface context.
    pub surface_context: M5WindowRestoreSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5WindowStateResolutionForm>,
    /// The published window-scoped pane tree; empty means missing.
    pub window_local_pane_tree: String,
    /// The published window-local focus / selection history; empty means missing.
    pub window_local_focus_history: String,
    /// The published display-affinity / machine hint kept distinct from authoritative state; empty means
    /// missing.
    pub display_affinity_hint: String,
    /// True when the topology keeps shared workspace authority state distinct (never absorbed into the window).
    pub keeps_authority_distinct: bool,
    /// True when the topology is truthful (never claims independence over a privately-copied authority).
    pub topology_is_truthful: bool,
    /// True when the topology copied shared authority state into private window state.
    pub authority_copied_into_window_used: bool,
    /// True when any copy of shared authority state into the window is disclosed rather than hidden.
    pub authority_copy_disclosed: bool,
    /// True when profile defaults or machine / display hints override authoritative topology.
    pub profile_default_override_asserted: bool,
    /// True when an asserted profile-default override is explained rather than left implicit.
    pub profile_default_override_explained: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe window-topology projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedWindowTopologyEntry {
    /// Stable identity of the window-topology entry.
    pub entry_id: String,
    /// The stable window ID this topology binds to.
    pub window_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve window-local selection and no-rerun under shared authority.
    pub semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: bool,
    /// The window-topology-surface token named by the entry.
    pub topology_surface: String,
    /// Whether the window-topology surface is classified into the resolved taxonomy.
    pub topology_surface_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published window-scoped pane tree.
    pub window_local_pane_tree: String,
    /// The published window-local focus / selection history.
    pub window_local_focus_history: String,
    /// The published display-affinity hint.
    pub display_affinity_hint: String,
    /// Whether the topology keeps shared authority distinct.
    pub keeps_authority_distinct: bool,
    /// Whether the topology is truthful.
    pub topology_is_truthful: bool,
    /// Whether the topology copied shared authority state into private window state.
    pub authority_copied_into_window_used: bool,
    /// Whether any copy of shared authority state into the window is disclosed.
    pub authority_copy_disclosed: bool,
    /// Whether profile defaults or machine / display hints override authoritative topology.
    pub profile_default_override_asserted: bool,
    /// Whether an asserted profile-default override is explained.
    pub profile_default_override_explained: bool,
    /// Whether the topology stays distinct and truthful (no undisclosed authority copy, authority kept distinct,
    /// explained overrides).
    pub topology_stays_distinct: bool,
    /// Whether the entry provides the complete window-local pane-tree / focus-history / display-affinity
    /// disclosure triple.
    pub provides_complete_disclosure_triple: bool,
    /// Degrade reason, if the entry could not read as a clean, distinct state.
    pub degrade_reason: Option<M5WindowTopologyEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5WindowStateNextAction,
    /// Whether the topology is distinct on every claimed window (clean entry naming every fact).
    pub topology_distinct_on_every_window: bool,
}

impl M5ResolvedWindowTopologyEntry {
    /// Whether this window-topology entry reads as a clean, distinct state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5WindowStateResolutionError {
    /// The workspace-authority-entry id was empty.
    EmptyWorkspaceAuthorityEntryId,
    /// The window-topology-entry id was empty.
    EmptyWindowTopologyEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5WindowStateResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyWorkspaceAuthorityEntryId => "empty_workspace_authority_entry_id",
            Self::EmptyWindowTopologyEntryId => "empty_window_topology_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5WindowStateResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 workspace-authority / window-topology registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5WindowStateResolutionError {}

fn form_tokens(forms: &[M5WindowStateResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5WindowStateResolutionForm]) -> bool {
    let present: BTreeSet<M5WindowStateResolutionForm> = forms.iter().copied().collect();
    M5WindowStateResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved workspace-authority object publishes every required field: authority mode (via a
/// classified scope), backing window IDs, stable pane-tree IDs, shared dirty-buffer state, shared save /
/// checkpoint state, authoritative state root, and the distinct profile-defaults reference. An unclassified
/// scope or any empty field never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn workspace_authority_object_is_complete(
    scope: M5WorkspaceAuthorityScope,
    backing_window_ids: &str,
    stable_pane_tree_ids: &str,
    shared_dirty_buffer_state: &str,
    shared_save_checkpoint_state: &str,
    authority_state_root: &str,
    profile_defaults_ref: &str,
) -> bool {
    scope.is_classified()
        && !backing_window_ids.trim().is_empty()
        && !stable_pane_tree_ids.trim().is_empty()
        && !shared_dirty_buffer_state.trim().is_empty()
        && !shared_save_checkpoint_state.trim().is_empty()
        && !authority_state_root.trim().is_empty()
        && !profile_defaults_ref.trim().is_empty()
}

/// Whether window-local selection and focus stay window-local under one shared authority: the scope must be
/// classified, the window-local state must be marked isolated, and an authority that backs multiple windows
/// must preserve its window-local focus / selection history. An unclassified scope, window-local state that
/// overwrites the shared authority, or lost window-local history never matches.
pub fn window_local_state_stays_window_local(
    scope: M5WorkspaceAuthorityScope,
    window_local_state_isolated: bool,
    shares_authority_across_windows: bool,
    window_local_history_preserved: bool,
) -> bool {
    scope.is_classified()
        && window_local_state_isolated
        && (!shares_authority_across_windows || window_local_history_preserved)
}

/// Whether a window topology stays distinct from the shared authority: the surface must be classified, the
/// topology must be truthful, it must keep shared authority state distinct, any copy of shared authority state
/// into the window must be disclosed rather than hidden, and any profile-default override must be explained.
pub fn window_topology_stays_distinct(
    surface: M5WindowTopologySurface,
    topology_is_truthful: bool,
    keeps_authority_distinct: bool,
    authority_copied_into_window_used: bool,
    authority_copy_disclosed: bool,
    profile_default_override_asserted: bool,
    profile_default_override_explained: bool,
) -> bool {
    surface.is_classified()
        && topology_is_truthful
        && keeps_authority_distinct
        && (!authority_copied_into_window_used || authority_copy_disclosed)
        && (!profile_default_override_asserted || profile_default_override_explained)
}

/// Resolves a workspace-authority-registry entry so it stays bound to the shared authority registry: the entry
/// names its canonical token, semantic role, and authority scope, covers all three resolution forms, publishes
/// a complete workspace-authority object (backing windows, stable pane-tree IDs, shared dirty-buffer / save /
/// checkpoint state, authoritative state root, distinct profile-defaults reference), keeps window-local
/// selection and focus window-local, and preserves window-local history under shared authority.
pub fn resolve_workspace_authority_entry(
    input: M5WorkspaceAuthorityEntryResolutionInput,
) -> Result<M5ResolvedWorkspaceAuthorityEntry, M5WindowStateResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5WindowStateResolutionError::EmptyWorkspaceAuthorityEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.workspace_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.backing_window_ids)
        || string_is_forbidden(&input.stable_pane_tree_ids)
        || string_is_forbidden(&input.shared_dirty_buffer_state)
        || string_is_forbidden(&input.shared_save_checkpoint_state)
        || string_is_forbidden(&input.authority_state_root)
        || string_is_forbidden(&input.profile_defaults_ref)
    {
        return Err(M5WindowStateResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = workspace_authority_object_is_complete(
        input.authority_scope,
        &input.backing_window_ids,
        &input.stable_pane_tree_ids,
        &input.shared_dirty_buffer_state,
        &input.shared_save_checkpoint_state,
        &input.authority_state_root,
        &input.profile_defaults_ref,
    );
    let window_local_ok = window_local_state_stays_window_local(
        input.authority_scope,
        input.window_local_state_isolated,
        input.shares_authority_across_windows,
        input.window_local_history_preserved,
    );
    let history_unpreserved =
        input.shares_authority_across_windows && !input.window_local_history_preserved;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5WorkspaceAuthorityEntryDegradeReason::AuthorityTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5WorkspaceAuthorityEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.authority_scope.is_classified() {
        Some(M5WorkspaceAuthorityEntryDegradeReason::AuthorityScopeUnclassified)
    } else if !input.bound_to_registry {
        Some(M5WorkspaceAuthorityEntryDegradeReason::AuthorityNotBoundToRegistry)
    } else if !object_complete {
        Some(M5WorkspaceAuthorityEntryDegradeReason::WorkspaceAuthorityObjectIncomplete)
    } else if !window_local_ok {
        Some(M5WorkspaceAuthorityEntryDegradeReason::WindowLocalStateOverwritesSharedAuthority)
    } else if !all_forms {
        Some(M5WorkspaceAuthorityEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if history_unpreserved {
        Some(M5WorkspaceAuthorityEntryDegradeReason::SharedAuthorityHistoryNotPreserved)
    } else if !input.proof_fresh {
        Some(M5WorkspaceAuthorityEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5WindowStateNextAction::ExpandOwnershipMeaning,
    };

    Ok(M5ResolvedWorkspaceAuthorityEntry {
        entry_id: input.entry_id,
        workspace_id: input.workspace_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: input
            .semantic_role
            .must_preserve_window_local_selection_and_no_rerun_under_shared_authority(),
        authority_scope: input.authority_scope.as_str().to_owned(),
        authority_scope_is_classified: input.authority_scope.is_classified(),
        canonical_authority_mode: input.authority_scope.canonical_authority_mode().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        backing_window_ids: input.backing_window_ids,
        stable_pane_tree_ids: input.stable_pane_tree_ids,
        shared_dirty_buffer_state: input.shared_dirty_buffer_state,
        shared_save_checkpoint_state: input.shared_save_checkpoint_state,
        authority_state_root: input.authority_state_root,
        profile_defaults_ref: input.profile_defaults_ref,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        workspace_authority_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        window_local_state_isolated: input.window_local_state_isolated,
        shares_authority_across_windows: input.shares_authority_across_windows,
        window_local_history_preserved: input.window_local_history_preserved,
        degrade_reason,
        next_action,
        authority_resolves_across_workspaces: degrade_reason.is_none(),
    })
}

/// Resolves a window-topology entry so its topology stays distinct from the shared authority: the entry names
/// its canonical token, semantic role, and window-topology surface, covers all three resolution forms, provides
/// the window-local pane-tree / focus-history / display-affinity disclosure triple, and degrades honestly when
/// window topology privately copies shared authority state, merges authority and topology into one opaque blob,
/// or lets profile defaults override authoritative topology.
pub fn resolve_window_topology_entry(
    input: M5WindowTopologyEntryResolutionInput,
) -> Result<M5ResolvedWindowTopologyEntry, M5WindowStateResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5WindowStateResolutionError::EmptyWindowTopologyEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.window_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.window_local_pane_tree)
        || string_is_forbidden(&input.window_local_focus_history)
        || string_is_forbidden(&input.display_affinity_hint)
    {
        return Err(M5WindowStateResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let topology_stays_distinct = window_topology_stays_distinct(
        input.topology_surface,
        input.topology_is_truthful,
        input.keeps_authority_distinct,
        input.authority_copied_into_window_used,
        input.authority_copy_disclosed,
        input.profile_default_override_asserted,
        input.profile_default_override_explained,
    );
    let provides_triple = input.topology_surface.is_classified()
        && !input.window_local_pane_tree.trim().is_empty()
        && !input.window_local_focus_history.trim().is_empty()
        && !input.display_affinity_hint.trim().is_empty()
        && topology_stays_distinct;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5WindowTopologyEntryDegradeReason::TopologyTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5WindowTopologyEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.topology_surface.is_classified() {
        Some(M5WindowTopologyEntryDegradeReason::WindowTopologySurfaceUnclassified)
    } else if !provides_triple {
        Some(M5WindowTopologyEntryDegradeReason::WindowTopologyMergesOrLeaksSharedAuthority)
    } else if !all_forms {
        Some(M5WindowTopologyEntryDegradeReason::TopologyFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5WindowTopologyEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5WindowStateNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedWindowTopologyEntry {
        entry_id: input.entry_id,
        window_id: input.window_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: input
            .semantic_role
            .must_preserve_window_local_selection_and_no_rerun_under_shared_authority(),
        topology_surface: input.topology_surface.as_str().to_owned(),
        topology_surface_is_classified: input.topology_surface.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        window_local_pane_tree: input.window_local_pane_tree,
        window_local_focus_history: input.window_local_focus_history,
        display_affinity_hint: input.display_affinity_hint,
        keeps_authority_distinct: input.keeps_authority_distinct,
        topology_is_truthful: input.topology_is_truthful,
        authority_copied_into_window_used: input.authority_copied_into_window_used,
        authority_copy_disclosed: input.authority_copy_disclosed,
        profile_default_override_asserted: input.profile_default_override_asserted,
        profile_default_override_explained: input.profile_default_override_explained,
        topology_stays_distinct,
        provides_complete_disclosure_triple: provides_triple,
        degrade_reason,
        next_action,
        topology_distinct_on_every_window: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved workspace-authority and window-topology entries
/// it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceAuthorityWindowTopologyRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5WorkspaceAuthorityWindowTopologyRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5WindowRestoreQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Restore contexts this row keeps the same truth across.
    pub deployment_lines: Vec<M5WindowRestoreDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5WindowRestoreRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5WindowRestoreAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5WindowStateAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5WindowStateExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5WindowRestoreDowngradeTrigger>,
    /// Resolved workspace-authority-registry examples.
    pub workspace_authority_entries: Vec<M5ResolvedWorkspaceAuthorityEntry>,
    /// Resolved window-topology examples.
    pub window_topology_entries: Vec<M5ResolvedWindowTopologyEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the window-topology and restore-fidelity
    /// domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: window-local state never overwrites shared workspace authority. MUST be `false`.
    pub window_local_state_overwrites_shared_workspace_authority: bool,
    /// Hard invariant: shared workspace authority never becomes private window state. MUST be `false`.
    pub shared_workspace_authority_becomes_private_window_state: bool,
    /// Hard invariant: workspace-authority and window-topology state are never merged into one opaque blob. MUST
    /// be `false`.
    pub merges_workspace_authority_and_window_topology_into_one_opaque_blob: bool,
    /// Hard invariant: shared dirty-buffer / save / checkpoint state never drifts across windows sharing one
    /// authority. MUST be `false`.
    pub dirty_buffer_state_drifts_across_windows_sharing_one_authority: bool,
}

impl M5WorkspaceAuthorityWindowTopologyRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5WindowStateAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5WindowStateAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5WindowStateExportField> =
            self.export_fields.iter().copied().collect();
        M5WindowStateExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.window_local_state_overwrites_shared_workspace_authority
            && !self.shared_workspace_authority_becomes_private_window_state
            && !self.merges_workspace_authority_and_window_topology_into_one_opaque_blob
            && !self.dirty_buffer_state_drifts_across_windows_sharing_one_authority
    }

    /// True when a clean workspace-authority entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified authority scope, publishes a complete authority object, keeps window-local state
    /// window-local, covers all three resolution forms, and preserves window-local history under shared
    /// authority.
    fn authority_is_honest(ex: &M5ResolvedWorkspaceAuthorityEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.authority_scope_is_classified
                && ex.workspace_authority_object_complete
                && ex.window_local_state_isolated
                && ex.covers_all_resolution_forms
                && (!ex.shares_authority_across_windows || ex.window_local_history_preserved))
    }

    /// True when a clean window-topology entry preserves distinct topology: it keeps a classified surface,
    /// provides the disclosure triple, stays distinct, and covers all three resolution forms.
    fn topology_is_honest(ex: &M5ResolvedWindowTopologyEntry) -> bool {
        !ex.is_clean()
            || (ex.topology_surface_is_classified
                && ex.provides_complete_disclosure_triple
                && ex.topology_stays_distinct
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.workspace_authority_entries
            .iter()
            .all(Self::authority_is_honest)
            && self
                .window_topology_entries
                .iter()
                .all(Self::topology_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceAuthorityWindowTopologyRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Authority-scope tokens (minted by this lane).
    pub authority_scopes: Vec<String>,
    /// Window-topology-surface tokens (minted by this lane).
    pub window_topology_surfaces: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Workspace-authority-entry degrade-reason tokens.
    pub workspace_authority_degrade_reasons: Vec<String>,
    /// Window-topology-entry degrade-reason tokens.
    pub window_topology_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5WorkspaceAuthorityWindowTopologyRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5WindowRestoreRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5WindowStateResolutionForm::ALL, |v| v.as_str()),
            authority_scopes: tokens(&M5WorkspaceAuthorityScope::ALL, |v| v.as_str()),
            window_topology_surfaces: tokens(&M5WindowTopologySurface::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5WindowRestoreSurfaceContext::ALL, |v| v.as_str()),
            workspace_authority_degrade_reasons: tokens(
                &M5WorkspaceAuthorityEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            window_topology_degrade_reasons: tokens(
                &M5WindowTopologyEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5WindowStateAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5WindowStateNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5WindowStateExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5WindowRestoreConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5WorkspaceAuthorityWindowTopologyRegistriesGovernanceReview {
    /// The authority registry names a canonical token, semantic role, and authority scope for every entry.
    pub authority_registry_names_token_role_and_scope: bool,
    /// Every claimed workspace resolves to one stable workspace-authority object from the shared registry, not
    /// per-window reconstruction.
    pub workspace_resolves_to_stable_object_from_shared_registry: bool,
    /// Backing windows, stable pane-tree IDs, shared dirty-buffer / save / checkpoint state, and the distinct
    /// profile-defaults reference are published for every resolved workspace.
    pub backing_windows_pane_ids_and_shared_state_published: bool,
    /// Window-local selection and focus stay window-local while one authority backs multiple windows.
    pub window_local_selection_and_focus_stay_window_local: bool,
    /// Window topology keeps shared workspace authority state distinct and never absorbs it privately.
    pub window_topology_keeps_shared_authority_distinct: bool,
    /// Shared workspace authority never becomes private window state and never merges into one opaque blob.
    pub shared_authority_never_becomes_private_window_state: bool,
    /// Every workspace-authority and window-topology entry covers the canonical / accessible / audit resolution
    /// forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Workspace-authority and window-topology behavior stay bound to the shared registries rather than
    /// hand-copied per window.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shell, recovery, diagnostics, and admin read a single workspace-ownership source.
    pub shell_recovery_diagnostics_admin_read_single_source: bool,
    /// A window-local overwrite, an incomplete object, or a leaked authority is caught by fixtures before
    /// release evidence turns green.
    pub authority_or_topology_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceAuthorityWindowTopologyRegistriesConsumerProjection {
    /// Shell and recovery consume the shared workspace-authority registry.
    pub shell_and_recovery_consume_shared_registries: bool,
    /// Diagnostics and admin consume the shared window-topology registry.
    pub diagnostics_and_admin_consume_shared_registries: bool,
    /// Session and workspace services consume the shared registries.
    pub session_and_workspace_services_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical window-topology and restore-fidelity domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical workspace-authority / window-topology registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceAuthorityWindowTopologyRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceAuthorityWindowTopologyRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting window-restore audit for the lane.
    pub window_restore_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5WorkspaceAuthorityWindowTopologyRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WorkspaceAuthorityWindowTopologyRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5WorkspaceAuthorityWindowTopologyRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WorkspaceAuthorityWindowTopologyRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WorkspaceAuthorityWindowTopologyRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WorkspaceAuthorityWindowTopologyRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WorkspaceAuthorityWindowTopologyRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WorkspaceAuthorityWindowTopologyRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 workspace-authority and window-topology registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5WorkspaceAuthorityWindowTopologyRegistriesPacket {
    /// Record kind; must equal [`M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5WorkspaceAuthorityWindowTopologyRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5WorkspaceAuthorityWindowTopologyRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5WorkspaceAuthorityWindowTopologyRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5WorkspaceAuthorityWindowTopologyRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5WorkspaceAuthorityWindowTopologyRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5WorkspaceAuthorityWindowTopologyRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5WorkspaceAuthorityWindowTopologyRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5WorkspaceAuthorityWindowTopologyRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5WorkspaceAuthorityWindowTopologyRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_RECORD_KIND {
            violations.push(M5WorkspaceAuthorityWindowTopologyRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_SCHEMA_VERSION {
            violations
                .push(M5WorkspaceAuthorityWindowTopologyRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5WorkspaceAuthorityWindowTopologyRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations
                .push(M5WorkspaceAuthorityWindowTopologyRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 workspace-authority / window-topology registries packet serializes"),
        ) {
            violations
                .push(M5WorkspaceAuthorityWindowTopologyRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 workspace-authority / window-topology registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,workspace_authority_entries,window_topology_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .workspace_authority_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.window_topology_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.workspace_authority_entries.len(),
                row.window_topology_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Workspace-Authority and Window-Topology Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Authority scopes: {}\n",
            self.vocabulary_set.authority_scopes.join(", ")
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
                "  - Workspace-authority entries: {} / window-topology entries: {}\n",
                row.workspace_authority_entries.len(),
                row.window_topology_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-workspace ownership reference table generated from the registry, so docs and admin
    /// runbooks render the same authority-mode / backing-windows / pane-tree-IDs / dirty-buffer / checkpoint /
    /// authoritative-state-root truth the resolvers produced rather than a hand-copied ownership table. Only
    /// clean, registry-bound workspace-authority entries are listed.
    pub fn render_workspace_ownership_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| workspace_id | authority_mode | backing_window_ids | stable_pane_tree_ids | shared_dirty_buffer_state | shared_save_checkpoint_state | authority_state_root |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.workspace_authority_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.workspace_id,
                    ex.canonical_authority_mode,
                    ex.backing_window_ids,
                    ex.stable_pane_tree_ids,
                    ex.shared_dirty_buffer_state,
                    ex.shared_save_checkpoint_state,
                    ex.authority_state_root
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5WorkspaceAuthorityWindowTopologyRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5WorkspaceAuthorityWindowTopologyRegistriesViolation>),
}

impl fmt::Display for M5WorkspaceAuthorityWindowTopologyRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 workspace-authority / window-topology registries export parse failed: {error}"
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
                    "m5 workspace-authority / window-topology registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5WorkspaceAuthorityWindowTopologyRegistriesArtifactError {}

/// Validation failures emitted by [`M5WorkspaceAuthorityWindowTopologyRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5WorkspaceAuthorityWindowTopologyRegistriesViolation {
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
    /// A registry row does not point at both the window-topology and restore-fidelity domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, isolation-losing, field-incomplete,
    /// form-incomplete, or a window-topology entry missing the disclosure triple).
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
    /// Stable-object-resolution is not proven: clean workspace-authority entries do not cover the canonical
    /// authority scopes or the first shell / recovery / diagnostics / admin / support surfaces, no
    /// object-incomplete example degrades, or a clean authority entry published an incomplete object.
    StableObjectResolutionNotProven,
    /// Window-local-isolation is not proven: no window-local-overwrite example and no unbound example degrade,
    /// no clean isolated authority entry is present, or a clean authority entry lost window-local isolation or
    /// is unbound.
    WindowLocalIsolationNotProven,
    /// Topology-distinctness is not proven: clean window-topology entries do not cover the canonical primary /
    /// auxiliary / diagnostics surfaces with full resolution-form coverage while providing the disclosure
    /// triple, no merges-or-leaks or form-incomplete example degrades, or a clean window-topology entry is
    /// missing the triple.
    TopologyDistinctnessNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5WorkspaceAuthorityWindowTopologyRegistriesViolation {
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
            Self::WindowLocalIsolationNotProven => "window_local_isolation_not_proven",
            Self::TopologyDistinctnessNotProven => "topology_distinctness_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_workspace_authority_and_window_topology_registries_export() -> Result<
    M5WorkspaceAuthorityWindowTopologyRegistriesPacket,
    M5WorkspaceAuthorityWindowTopologyRegistriesArtifactError,
> {
    let packet: M5WorkspaceAuthorityWindowTopologyRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-workspace-authority-and-window-topology-registries-proof/support_export.json"
        )
    ))
    .map_err(M5WorkspaceAuthorityWindowTopologyRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5WorkspaceAuthorityWindowTopologyRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5WorkspaceAuthorityWindowTopologyRegistriesPacket,
    violations: &mut Vec<M5WorkspaceAuthorityWindowTopologyRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_SCHEMA_REF,
        M5_WORKSPACE_AUTHORITY_WINDOW_TOPOLOGY_REGISTRIES_DOC_REF,
        M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
        M5_WINDOW_RESTORE_MATRIX_DOC_REF,
        M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
        M5_RESTORE_FIDELITY_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5WorkspaceAuthorityWindowTopologyRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5WorkspaceAuthorityWindowTopologyRegistriesPacket,
    violations: &mut Vec<M5WorkspaceAuthorityWindowTopologyRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5WorkspaceAuthorityWindowTopologyRegistriesViolation::NoRegistryRows);
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
            violations
                .push(M5WorkspaceAuthorityWindowTopologyRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5WorkspaceAuthorityWindowTopologyRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5WorkspaceAuthorityWindowTopologyRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF)
            || !refs.contains(M5_RESTORE_FIDELITY_SCHEMA_REF)
        {
            violations.push(
                M5WorkspaceAuthorityWindowTopologyRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.workspace_authority_entries.is_empty() || row.window_topology_entries.is_empty() {
            violations.push(M5WorkspaceAuthorityWindowTopologyRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations
                .push(M5WorkspaceAuthorityWindowTopologyRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations
                .push(M5WorkspaceAuthorityWindowTopologyRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5WorkspaceAuthorityWindowTopologyRegistriesPacket,
    violations: &mut Vec<M5WorkspaceAuthorityWindowTopologyRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.authority_registry_names_token_role_and_scope,
        review.workspace_resolves_to_stable_object_from_shared_registry,
        review.backing_windows_pane_ids_and_shared_state_published,
        review.window_local_selection_and_focus_stay_window_local,
        review.window_topology_keeps_shared_authority_distinct,
        review.shared_authority_never_becomes_private_window_state,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shell_recovery_diagnostics_admin_read_single_source,
        review.authority_or_topology_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5WorkspaceAuthorityWindowTopologyRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5WorkspaceAuthorityWindowTopologyRegistriesPacket,
    violations: &mut Vec<M5WorkspaceAuthorityWindowTopologyRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_recovery_consume_shared_registries,
        projection.diagnostics_and_admin_consume_shared_registries,
        projection.session_and_workspace_services_consume_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5WorkspaceAuthorityWindowTopologyRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5WorkspaceAuthorityWindowTopologyRegistriesPacket,
    violations: &mut Vec<M5WorkspaceAuthorityWindowTopologyRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations
            .push(M5WorkspaceAuthorityWindowTopologyRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5WorkspaceAuthorityWindowTopologyRegistriesPacket,
    violations: &mut Vec<M5WorkspaceAuthorityWindowTopologyRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.window_restore_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations
            .push(M5WorkspaceAuthorityWindowTopologyRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5WorkspaceAuthorityWindowTopologyRegistriesPacket,
    violations: &mut Vec<M5WorkspaceAuthorityWindowTopologyRegistriesViolation>,
) {
    let authorities = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.workspace_authority_entries.iter())
    };
    let topologies = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.window_topology_entries.iter())
    };

    // AC1: every claimed workspace resolves to one stable workspace-authority object with backing-windows /
    // pane-IDs / shared-state / distinct-defaults fields. Clean authority entries cover the canonical authority
    // scopes and the first shell / recovery / diagnostics / admin / support surfaces, an object-incomplete
    // example degrades, and no clean authority entry published an incomplete object.
    let clean_scopes: BTreeSet<String> = authorities()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.authority_scope.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = authorities()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let scopes_covered = M5WorkspaceAuthorityScope::CANONICAL_SCOPES
        .iter()
        .all(|s| clean_scopes.contains(s.as_str()));
    let first_surfaces_covered = M5WindowRestoreSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = authorities().any(|ex| {
        ex.degrade_reason
            == Some(M5WorkspaceAuthorityEntryDegradeReason::WorkspaceAuthorityObjectIncomplete)
    });
    let no_clean_incomplete =
        !authorities().any(|ex| ex.is_clean() && !ex.workspace_authority_object_complete);
    if !(scopes_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5WorkspaceAuthorityWindowTopologyRegistriesViolation::StableObjectResolutionNotProven,
        );
    }

    // AC2: multiple windows share one workspace authority while preserving independent layout and focus without
    // dirty-state drift — window-local state stays window-local. A window-local-overwrite example degrades, an
    // unbound example degrades, at least one clean isolated authority entry is present, and no clean authority
    // entry lost window-local isolation or is unbound.
    let overwrite_degrades = authorities().any(|ex| {
        ex.degrade_reason
            == Some(
                M5WorkspaceAuthorityEntryDegradeReason::WindowLocalStateOverwritesSharedAuthority,
            )
    });
    let unbound_degrades = authorities().any(|ex| {
        ex.degrade_reason
            == Some(M5WorkspaceAuthorityEntryDegradeReason::AuthorityNotBoundToRegistry)
    });
    let isolated_clean_authority =
        authorities().any(|ex| ex.is_clean() && ex.window_local_state_isolated);
    let no_clean_unbound = !authorities().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_unisolated =
        !authorities().any(|ex| ex.is_clean() && !ex.window_local_state_isolated);
    if !(overwrite_degrades
        && unbound_degrades
        && isolated_clean_authority
        && no_clean_unbound
        && no_clean_unisolated)
    {
        violations.push(
            M5WorkspaceAuthorityWindowTopologyRegistriesViolation::WindowLocalIsolationNotProven,
        );
    }

    // AC3: the suite fails when shared authority becomes private window state (or window-local state overwrites
    // shared authority). Clean window-topology entries cover every canonical primary / auxiliary / diagnostics
    // surface with full resolution-form coverage while providing the disclosure triple, a merges-or-leaks
    // example degrades, a form-incomplete example degrades, and no clean window-topology entry is missing the
    // triple.
    let clean_topology_surfaces: BTreeSet<String> = topologies()
        .filter(|ex| {
            ex.is_clean()
                && ex.topology_surface_is_classified
                && ex.provides_complete_disclosure_triple
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.topology_surface.clone())
        .collect();
    let topology_surfaces_covered = M5WindowTopologySurface::CANONICAL_SURFACES
        .iter()
        .all(|s| clean_topology_surfaces.contains(s.as_str()));
    let leaks_degrades = topologies().any(|ex| {
        ex.degrade_reason
            == Some(M5WindowTopologyEntryDegradeReason::WindowTopologyMergesOrLeaksSharedAuthority)
    });
    let form_incomplete_degrades = topologies().any(|ex| {
        ex.degrade_reason
            == Some(M5WindowTopologyEntryDegradeReason::TopologyFormCoverageIncomplete)
    });
    let no_clean_missing_triple =
        !topologies().any(|ex| ex.is_clean() && !ex.provides_complete_disclosure_triple);
    if !(topology_surfaces_covered
        && leaks_degrades
        && form_incomplete_degrades
        && no_clean_missing_triple)
    {
        violations.push(
            M5WorkspaceAuthorityWindowTopologyRegistriesViolation::TopologyDistinctnessNotProven,
        );
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

/// The window-restore families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5WindowRestoreFamily; 2] = [
    M5WindowRestoreFamily::SharedWorkspaceAuthority,
    M5WindowRestoreFamily::WindowLocalTopology,
];
