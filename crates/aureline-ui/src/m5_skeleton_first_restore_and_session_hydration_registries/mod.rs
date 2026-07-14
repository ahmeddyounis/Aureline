//! Implemented M5 skeleton-first-restore and no-rerun-session-hydration registries.
//!
//! The frozen [window-restore matrix][matrix] names Aureline's five workspace-restore families and locks
//! their controlled vocabulary. This module is the implement lane for the concrete restore-orchestration
//! flows: it turns the *skeleton-first restore* grammar and the *no-rerun session hydration* grammar into
//! registry resolvers that produce export-safe, honest projections. Every claimed M5 restore then rebuilds one
//! stable restore-skeleton object first — the restore-fidelity class, the window shell it rebuilds, the stable
//! versioned pane-tree structure, the preserved pane roles and placeholder set, the layout-skeleton root, and
//! the deferred-hydration plan kept distinct from it — before any heavy dependency hydrates, so restore is
//! progressively truthful (never all-or-nothing), a missing dependency produces a pane-role-preserving
//! placeholder instead of a silent layout collapse, session-scoped tools never silently rerun or reacquire
//! broader authority, and a restore that cannot explain which panes came back live, as placeholders,
//! context-only, or evidence-only degrades honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Rebuild one stable restore-skeleton object per restore before hydrating.**
//!   [`resolve_skeleton_restore_entry`] refuses to read as a clean, registry-bound skeleton entry unless it
//!   names a canonical registry token, a classified [restore-fidelity class][M5RestoreFidelityClass], a
//!   window-restore role, covers every [resolution form][M5RestoreOrchestrationResolutionForm] (the canonical
//!   object, the accessible summary, and the audit record), publishes every skeleton field (window shell,
//!   stable pane-tree structure, preserved pane roles, placeholder set, layout-skeleton root, and the distinct
//!   deferred-hydration plan), rebuilds the skeleton before heavy hydration, and preserves pane roles when it
//!   defers heavy hydration; otherwise it degrades.
//! * **Keep heavy hydration from preceding the layout skeleton.** [`skeleton_precedes_hydration`] rejects an
//!   entry whose heavy hydration ran before the layout skeleton was rebuilt so it degrades to
//!   [`M5SkeletonRestoreEntryDegradeReason::HydrationPrecededSkeleton`], and the
//!   `pane_roles_preserved_when_deferred` invariant degrades a deferred-hydration skeleton that dropped its
//!   pane roles.
//! * **Keep session hydration from rerunning session-scoped work or collapsing layout.**
//!   [`resolve_session_hydration_entry`] names a classified [hydration surface][M5SessionHydrationSurface],
//!   requires the preserved-pane-role / missing-dependency-class / restore-fidelity-hint disclosure triple,
//!   covers every resolution form, and degrades to
//!   [`M5SessionHydrationEntryDegradeReason::SessionHydrationRerunsOrCollapsesLayout`] when hydration reruns
//!   session-scoped work or reacquires broader authority, deletes layout structure silently on a missing
//!   dependency instead of substituting a pane-role-preserving placeholder, or overclaims restore fidelity on a
//!   deferred dependency, so a pane can never read as live when its heavy dependency never actually hydrated.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5WindowRestoreRole`] role vocabulary and
//! the [`M5WindowRestoreConsumerSurface`] consumer-surface taxonomy — so the shell, recovery, diagnostics,
//! admin, workspace, session, docs, CLI, and support surfaces can never fork their own restore-orchestration
//! meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_window_restore_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_skeleton_first_restore_and_session_hydration_registries,
    seeded_m5_skeleton_first_restore_and_session_hydration_registries_context_only_hydration_preview_narrowed,
    seeded_m5_skeleton_first_restore_and_session_hydration_registries_placeholder_pane_continuity_beta_narrowed,
    M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_PACKET_ID,
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

/// Stable record-kind tag carried by [`M5SkeletonFirstRestoreSessionHydrationRegistriesPacket`].
pub const M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_skeleton_first_restore_and_session_hydration_registries";

/// Schema version for M5 skeleton-first-restore / session-hydration registry records.
pub const M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_SCHEMA_REF: &str =
    "schemas/shell/m5-skeleton-first-restore-and-session-hydration-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_DOC_REF: &str =
    "docs/recovery/m5_skeleton_first_restore_and_session_hydration_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-skeleton-first-restore-and-session-hydration-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-skeleton-first-restore-and-session-hydration-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-skeleton-first-restore-and-session-hydration-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-skeleton-first-restore-and-session-hydration-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5SkeletonFirstRestoreSessionHydrationRegistriesConsumerSurface =
    M5WindowRestoreConsumerSurface;

/// One of the three resolution forms every skeleton-restore or session-hydration entry must hold across so its
/// truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary, or
/// written to the audit / support record. Minted by this lane because the frozen matrix names the
/// skeleton-first-restore and no-rerun-session-hydration *families* but not the concrete form set an entry must
/// cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreOrchestrationResolutionForm {
    /// The canonical resolved restore-skeleton / session-hydration object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved restore discoverable without visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved restore inspectable off-renderer.
    AuditRecord,
}

impl M5RestoreOrchestrationResolutionForm {
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

/// Controlled restore-fidelity class a skeleton-restore entry resolves, so the canonical restore-fidelity model
/// shares one registry rather than a hand-copied per-pane restore assumption. Minted by this lane because the
/// frozen matrix carries the workspace-restore families but not the concrete live-versus-placeholder-versus-
/// context-only-versus-evidence-only fidelity model a skeleton entry resolves against. Every classified class
/// carries its canonical restore-fidelity mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreFidelityClass {
    /// A pane restored fully live (its heavy dependency finished hydrating).
    LiveHydratedPane,
    /// A pane restored as a pane-role-preserving placeholder while its heavy dependency is still hydrating or
    /// missing.
    PaneRolePlaceholder,
    /// A pane restored context-only (its surrounding context reopened without live services).
    ContextOnlyPane,
    /// A pane restored evidence-only (only serialized evidence reopened, no live or context restore).
    EvidenceOnlyPane,
    /// The restore-fidelity class is unclassified, which is disallowed.
    FidelityUnclassified,
}

impl M5RestoreFidelityClass {
    /// Every restore-fidelity class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LiveHydratedPane,
        Self::PaneRolePlaceholder,
        Self::ContextOnlyPane,
        Self::EvidenceOnlyPane,
        Self::FidelityUnclassified,
    ];

    /// The four canonical restore-fidelity classes every claimed M5 restore must explain.
    pub const CANONICAL_CLASSES: [Self; 4] = [
        Self::LiveHydratedPane,
        Self::PaneRolePlaceholder,
        Self::ContextOnlyPane,
        Self::EvidenceOnlyPane,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveHydratedPane => "live_hydrated_pane",
            Self::PaneRolePlaceholder => "pane_role_placeholder",
            Self::ContextOnlyPane => "context_only_pane",
            Self::EvidenceOnlyPane => "evidence_only_pane",
            Self::FidelityUnclassified => "fidelity_unclassified",
        }
    }

    /// Whether the class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::FidelityUnclassified)
    }

    /// The canonical restore-fidelity mode for this class.
    pub const fn canonical_restore_fidelity_mode(self) -> &'static str {
        match self {
            Self::LiveHydratedPane => "live_hydrated",
            Self::PaneRolePlaceholder => "pane_role_placeholder",
            Self::ContextOnlyPane => "context_only",
            Self::EvidenceOnlyPane => "evidence_only",
            Self::FidelityUnclassified => "",
        }
    }

    /// Whether this class defers heavy hydration and so must preserve its pane roles explicitly. Only a fully
    /// live pane never defers.
    pub const fn defers_hydration(self) -> bool {
        matches!(
            self,
            Self::PaneRolePlaceholder | Self::ContextOnlyPane | Self::EvidenceOnlyPane
        )
    }
}

/// Controlled session-hydration surface a session-hydration entry must resolve its hydration from, so a
/// session-scoped dependency shares one registry rather than a hand-copied per-pane rehydration path. Minted by
/// this lane, tracking the session-scoped surfaces the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionHydrationSurface {
    /// The terminal / remote-shell hydration surface.
    TerminalOrRemoteShellHydration,
    /// The debugger / notebook hydration surface.
    DebuggerOrNotebookHydration,
    /// The preview / collaboration hydration surface.
    PreviewOrCollaborationHydration,
    /// The session-hydration surface is unclassified, which is disallowed.
    HydrationSurfaceUnclassified,
}

impl M5SessionHydrationSurface {
    /// Every session-hydration surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::TerminalOrRemoteShellHydration,
        Self::DebuggerOrNotebookHydration,
        Self::PreviewOrCollaborationHydration,
        Self::HydrationSurfaceUnclassified,
    ];

    /// The three canonical surfaces every session hydration must stay no-rerun across.
    pub const CANONICAL_SURFACES: [Self; 3] = [
        Self::TerminalOrRemoteShellHydration,
        Self::DebuggerOrNotebookHydration,
        Self::PreviewOrCollaborationHydration,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TerminalOrRemoteShellHydration => "terminal_or_remote_shell_hydration",
            Self::DebuggerOrNotebookHydration => "debugger_or_notebook_hydration",
            Self::PreviewOrCollaborationHydration => "preview_or_collaboration_hydration",
            Self::HydrationSurfaceUnclassified => "hydration_surface_unclassified",
        }
    }

    /// Whether the session-hydration surface is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::HydrationSurfaceUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a restore-fidelity or
/// session-hydration token's meaning stays stable whether it appears in the shell, recovery, diagnostics,
/// admin, or a support / export form. Minted by this lane, tracking the first-consumer surfaces the
/// implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreOrchestrationSurfaceContext {
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

impl M5RestoreOrchestrationSurfaceContext {
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

/// One mandatory rendered part a skeleton-restore or session-hydration entry must be able to show, so no
/// restore-fidelity class, window shell, pane-tree structure, pane role, placeholder, missing-dependency class,
/// hydration-plan hint, or registry fact is left implicit behind a hand-copied per-pane restore assumption.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreOrchestrationAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The restore-fidelity class the entry resolves (skeleton-restore entry).
    RestoreFidelityClass,
    /// The window shell and stable pane-tree structure the entry rebuilds (skeleton-restore entry).
    WindowShellAndPaneTree,
    /// The resolution-form coverage (canonical / accessible / audit).
    ResolutionFormCoverage,
    /// The preserved pane roles and placeholder set the entry publishes (skeleton-restore entry).
    PaneRoleAndPlaceholderState,
    /// The missing-dependency class the entry publishes (session-hydration entry).
    MissingDependencyClass,
    /// The deferred-hydration plan kept distinct from the layout skeleton (both entries).
    HydrationPlanHint,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved restore or hydration (both entries).
    PlainLanguageMeaning,
}

impl M5RestoreOrchestrationAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::RestoreFidelityClass,
        Self::WindowShellAndPaneTree,
        Self::ResolutionFormCoverage,
        Self::PaneRoleAndPlaceholderState,
        Self::MissingDependencyClass,
        Self::HydrationPlanHint,
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
            Self::RestoreFidelityClass => "restore_fidelity_class",
            Self::WindowShellAndPaneTree => "window_shell_and_pane_tree",
            Self::ResolutionFormCoverage => "resolution_form_coverage",
            Self::PaneRoleAndPlaceholderState => "pane_role_and_placeholder_state",
            Self::MissingDependencyClass => "missing_dependency_class",
            Self::HydrationPlanHint => "hydration_plan_hint",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// skeleton, a session hydration, or a degraded skeleton-restore / session-hydration entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RestoreOrchestrationNextAction {
    /// Expand the resolved skeleton's or hydration's plain-language meaning.
    ExpandRestoreMeaning,
    /// Inspect the restore-fidelity class or session-hydration surface the entry resolves.
    InspectFidelityOrSurface,
    /// Complete the canonical / accessible / audit resolution-form coverage.
    CompleteResolutionFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5RestoreOrchestrationNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandRestoreMeaning,
        Self::InspectFidelityOrSurface,
        Self::CompleteResolutionFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandRestoreMeaning => "expand_restore_meaning",
            Self::InspectFidelityOrSurface => "inspect_fidelity_or_surface",
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
pub enum M5RestoreOrchestrationExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The window-restore families covered.
    WindowRestoreFamilies,
    /// The restore-fidelity classes carried.
    RestoreFidelityClasses,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The resolution forms covered.
    ResolutionForms,
    /// The session-hydration surfaces carried.
    SessionHydrationSurfaces,
    /// The render / surface context.
    SurfaceContext,
    /// The restore-fidelity modes carried.
    RestoreFidelityModes,
    /// The accountable owner role.
    OwnerRole,
}

impl M5RestoreOrchestrationExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::WindowRestoreFamilies,
        Self::RestoreFidelityClasses,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ResolutionForms,
        Self::SessionHydrationSurfaces,
        Self::SurfaceContext,
        Self::RestoreFidelityModes,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::WindowRestoreFamilies,
        Self::RestoreFidelityClasses,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::WindowRestoreFamilies => "window_restore_families",
            Self::RestoreFidelityClasses => "restore_fidelity_classes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ResolutionForms => "resolution_forms",
            Self::SessionHydrationSurfaces => "session_hydration_surfaces",
            Self::SurfaceContext => "surface_context",
            Self::RestoreFidelityModes => "restore_fidelity_modes",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a skeleton-restore entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, hydration-first, field-incomplete, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SkeletonRestoreEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the skeleton means.
    SkeletonTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The restore-fidelity class is unclassified (not in the resolved taxonomy).
    RestoreFidelityClassUnclassified,
    /// The behavior is a hand-copied per-pane restore assumption instead of tracing to the canonical registry.
    SkeletonNotBoundToRegistry,
    /// The resolved restore-skeleton object is incomplete: window shell, stable pane-tree structure, preserved
    /// pane roles, placeholder set, layout-skeleton root, or the distinct deferred-hydration plan is unstated.
    RestoreSkeletonObjectIncomplete,
    /// Heavy hydration ran before the layout skeleton was rebuilt (an all-or-nothing restore instead of a
    /// skeleton-first one).
    HydrationPrecededSkeleton,
    /// The canonical / accessible / audit resolution-form coverage is incomplete.
    ResolutionFormCoverageIncomplete,
    /// The skeleton defers heavy hydration but the preserved pane roles are not kept.
    DeferredPaneRolesNotPreserved,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5SkeletonRestoreEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::SkeletonTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::RestoreFidelityClassUnclassified,
        Self::SkeletonNotBoundToRegistry,
        Self::RestoreSkeletonObjectIncomplete,
        Self::HydrationPrecededSkeleton,
        Self::ResolutionFormCoverageIncomplete,
        Self::DeferredPaneRolesNotPreserved,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SkeletonTokenUnstated => "skeleton_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::RestoreFidelityClassUnclassified => "restore_fidelity_class_unclassified",
            Self::SkeletonNotBoundToRegistry => "skeleton_not_bound_to_registry",
            Self::RestoreSkeletonObjectIncomplete => "restore_skeleton_object_incomplete",
            Self::HydrationPrecededSkeleton => "hydration_preceded_skeleton",
            Self::ResolutionFormCoverageIncomplete => "resolution_form_coverage_incomplete",
            Self::DeferredPaneRolesNotPreserved => "deferred_pane_roles_not_preserved",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5RestoreOrchestrationNextAction {
        match self {
            Self::SkeletonTokenUnstated | Self::SkeletonNotBoundToRegistry => {
                M5RestoreOrchestrationNextAction::TraceCanonicalRegistry
            }
            Self::RestoreFidelityClassUnclassified
            | Self::RestoreSkeletonObjectIncomplete
            | Self::HydrationPrecededSkeleton => {
                M5RestoreOrchestrationNextAction::InspectFidelityOrSurface
            }
            Self::ResolutionFormCoverageIncomplete => {
                M5RestoreOrchestrationNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::DeferredPaneRolesNotPreserved
            | Self::ProofStale => M5RestoreOrchestrationNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WindowRestoreDowngradeTrigger {
        match self {
            Self::SkeletonTokenUnstated | Self::ResolutionFormCoverageIncomplete => {
                M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::RestoreFidelityClassUnclassified => {
                M5WindowRestoreDowngradeTrigger::RestoreFidelityClassUnstated
            }
            Self::SkeletonNotBoundToRegistry => {
                M5WindowRestoreDowngradeTrigger::WindowTopologyBoundaryDriftedBySurface
            }
            Self::RestoreSkeletonObjectIncomplete => {
                M5WindowRestoreDowngradeTrigger::DeletedLayoutStructureSilentlyOnMissingExtensionOrRemoteTarget
            }
            Self::HydrationPrecededSkeleton | Self::DeferredPaneRolesNotPreserved => {
                M5WindowRestoreDowngradeTrigger::OverclaimedRestoreFidelityWhenOnlyContextOrEvidenceReopened
            }
            Self::ProofStale => M5WindowRestoreDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a session-hydration entry degraded below a clean, no-rerun state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SessionHydrationEntryDegradeReason {
    /// The canonical registry token name is unstated.
    HydrationTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The session-hydration surface is unclassified (not in the resolved taxonomy).
    SessionHydrationSurfaceUnclassified,
    /// The hydration reruns or collapses — hydration reran session-scoped work or reacquired broader authority,
    /// deleted layout structure silently on a missing dependency instead of substituting a pane-role-preserving
    /// placeholder, or overclaimed restore fidelity on a deferred dependency, or it dropped the preserved-pane-
    /// role / missing-dependency-class / restore-fidelity-hint disclosure triple.
    SessionHydrationRerunsOrCollapsesLayout,
    /// The canonical / accessible / audit resolution-form coverage of the hydration is incomplete.
    HydrationFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5SessionHydrationEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::HydrationTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::SessionHydrationSurfaceUnclassified,
        Self::SessionHydrationRerunsOrCollapsesLayout,
        Self::HydrationFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HydrationTokenUnstated => "hydration_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::SessionHydrationSurfaceUnclassified => "session_hydration_surface_unclassified",
            Self::SessionHydrationRerunsOrCollapsesLayout => {
                "session_hydration_reruns_or_collapses_layout"
            }
            Self::HydrationFormCoverageIncomplete => "hydration_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5RestoreOrchestrationNextAction {
        match self {
            Self::HydrationTokenUnstated => {
                M5RestoreOrchestrationNextAction::TraceCanonicalRegistry
            }
            Self::SessionHydrationSurfaceUnclassified
            | Self::SessionHydrationRerunsOrCollapsesLayout => {
                M5RestoreOrchestrationNextAction::InspectFidelityOrSurface
            }
            Self::HydrationFormCoverageIncomplete => {
                M5RestoreOrchestrationNextAction::CompleteResolutionFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5RestoreOrchestrationNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5WindowRestoreDowngradeTrigger {
        match self {
            Self::HydrationTokenUnstated => {
                M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SurfaceContextUnresolved | Self::SessionHydrationSurfaceUnclassified => {
                M5WindowRestoreDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::SessionHydrationRerunsOrCollapsesLayout => {
                M5WindowRestoreDowngradeTrigger::ReranCommandsOrReattachedPrivilegedSessionsImplicitlyDuringRestore
            }
            Self::HydrationFormCoverageIncomplete => {
                M5WindowRestoreDowngradeTrigger::SessionHydrationRuleUnstated
            }
            Self::ProofStale => M5WindowRestoreDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_skeleton_restore_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SkeletonRestoreEntryResolutionInput {
    /// Stable identity of the skeleton-restore-registry entry.
    pub entry_id: String,
    /// The stable restore-target ID this skeleton binds to (e.g. `restore.acme.cold-start`); empty means
    /// unstated.
    pub restore_target_id: String,
    /// The canonical registry token name (e.g. `restore.skeleton.first`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5WindowRestoreRole,
    /// The restore-fidelity class this entry resolves.
    pub restore_fidelity_class: M5RestoreFidelityClass,
    /// The render / surface context.
    pub surface_context: M5RestoreOrchestrationSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5RestoreOrchestrationResolutionForm>,
    /// The published rebuilt window-shell ID; empty means unstated.
    pub window_shell_id: String,
    /// The published stable, versioned pane-tree structure; empty means unstated.
    pub pane_tree_structure: String,
    /// The published preserved pane-role set; empty means unstated.
    pub pane_role_set: String,
    /// The published placeholder set for panes awaiting hydration; empty means unstated.
    pub placeholder_set: String,
    /// The published layout-skeleton root; empty means unstated.
    pub layout_skeleton_root: String,
    /// The published deferred-hydration plan reference kept distinct from the layout skeleton; empty means
    /// unstated.
    pub hydration_plan_ref: String,
    /// True when the behavior traces to the skeleton-restore registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the layout skeleton is rebuilt before any heavy dependency hydrates (a hard invariant when
    /// `false`).
    pub skeleton_rebuilt_before_hydration: bool,
    /// True when this skeleton defers heavy hydration (placeholders / context-only / evidence-only panes).
    pub defers_heavy_hydration: bool,
    /// True when preserved pane roles are kept when heavy hydration is deferred.
    pub pane_roles_preserved_when_deferred: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe skeleton-restore-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSkeletonRestoreEntry {
    /// Stable identity of the skeleton-restore-registry entry.
    pub entry_id: String,
    /// The stable restore-target ID this skeleton binds to.
    pub restore_target_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve window-local selection and no-rerun under shared authority.
    pub semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: bool,
    /// The restore-fidelity-class token named by the entry.
    pub restore_fidelity_class: String,
    /// Whether the restore-fidelity class is classified into the resolved taxonomy.
    pub restore_fidelity_class_is_classified: bool,
    /// The canonical restore-fidelity mode for the entry's class.
    pub canonical_restore_fidelity_mode: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published rebuilt window-shell ID.
    pub window_shell_id: String,
    /// The published stable pane-tree structure.
    pub pane_tree_structure: String,
    /// The published preserved pane-role set.
    pub pane_role_set: String,
    /// The published placeholder set.
    pub placeholder_set: String,
    /// The published layout-skeleton root.
    pub layout_skeleton_root: String,
    /// The published deferred-hydration plan reference.
    pub hydration_plan_ref: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// Whether the resolved restore-skeleton object publishes every required field.
    pub restore_skeleton_object_complete: bool,
    /// Whether the entry traces to the skeleton-restore registry.
    pub bound_to_registry: bool,
    /// Whether the layout skeleton is rebuilt before heavy hydration.
    pub skeleton_rebuilt_before_hydration: bool,
    /// Whether this skeleton defers heavy hydration.
    pub defers_heavy_hydration: bool,
    /// Whether preserved pane roles are kept when heavy hydration is deferred.
    pub pane_roles_preserved_when_deferred: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5SkeletonRestoreEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5RestoreOrchestrationNextAction,
    /// Whether the skeleton resolves to one stable object across every claimed restore (clean entry naming
    /// every fact).
    pub skeleton_resolves_across_restores: bool,
}

impl M5ResolvedSkeletonRestoreEntry {
    /// Whether this skeleton-restore entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_session_hydration_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SessionHydrationEntryResolutionInput {
    /// Stable identity of the session-hydration entry.
    pub entry_id: String,
    /// The stable pane ID this hydration binds to; empty means unstated.
    pub pane_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5WindowRestoreRole,
    /// The session-hydration surface this entry must resolve its hydration from.
    pub hydration_surface: M5SessionHydrationSurface,
    /// The render / surface context.
    pub surface_context: M5RestoreOrchestrationSurfaceContext,
    /// The resolution forms this entry holds across (must cover canonical / accessible / audit).
    pub resolution_form_coverage: Vec<M5RestoreOrchestrationResolutionForm>,
    /// The published preserved pane role kept during hydration or when a dependency is missing; empty means
    /// missing.
    pub preserved_pane_role: String,
    /// The published missing-dependency class (present / missing / expired / quarantined / unsupported); empty
    /// means missing.
    pub missing_dependency_class: String,
    /// The published restore-fidelity hint (live / placeholder / context-only / evidence-only) kept distinct
    /// from a live claim; empty means missing.
    pub restore_fidelity_hint: String,
    /// True when the hydration preserves the pane role and surrounding topology (never a silent collapse).
    pub preserves_pane_role_and_topology: bool,
    /// True when the hydration is truthful (never reruns session-scoped work, reacquires broader authority, or
    /// overclaims restore fidelity).
    pub hydration_is_truthful: bool,
    /// True when a dependency was missing / expired / quarantined / unsupported.
    pub dependency_missing_used: bool,
    /// True when a pane-role-preserving placeholder was substituted for a missing dependency rather than
    /// collapsing the layout.
    pub placeholder_substituted_on_missing: bool,
    /// True when a heavy dependency was deferred rather than hydrated inline.
    pub heavy_dependency_deferred: bool,
    /// True when a deferred heavy dependency's restore fidelity is disclosed honestly rather than overclaimed.
    pub deferred_fidelity_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe session-hydration projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSessionHydrationEntry {
    /// Stable identity of the session-hydration entry.
    pub entry_id: String,
    /// The stable pane ID this hydration binds to.
    pub pane_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve window-local selection and no-rerun under shared authority.
    pub semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: bool,
    /// The session-hydration-surface token named by the entry.
    pub hydration_surface: String,
    /// Whether the session-hydration surface is classified into the resolved taxonomy.
    pub hydration_surface_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The resolution-form tokens covered by the entry.
    pub resolution_form_coverage: Vec<String>,
    /// Whether the entry covers all three resolution forms.
    pub covers_all_resolution_forms: bool,
    /// The published preserved pane role.
    pub preserved_pane_role: String,
    /// The published missing-dependency class.
    pub missing_dependency_class: String,
    /// The published restore-fidelity hint.
    pub restore_fidelity_hint: String,
    /// Whether the hydration preserves the pane role and surrounding topology.
    pub preserves_pane_role_and_topology: bool,
    /// Whether the hydration is truthful.
    pub hydration_is_truthful: bool,
    /// Whether a dependency was missing / expired / quarantined / unsupported.
    pub dependency_missing_used: bool,
    /// Whether a pane-role-preserving placeholder was substituted for a missing dependency.
    pub placeholder_substituted_on_missing: bool,
    /// Whether a heavy dependency was deferred rather than hydrated inline.
    pub heavy_dependency_deferred: bool,
    /// Whether a deferred heavy dependency's restore fidelity is disclosed honestly.
    pub deferred_fidelity_disclosed: bool,
    /// Whether the hydration stays no-rerun and continuity-preserving (no silent rerun, pane role preserved,
    /// missing dependency placeholder-substituted, deferred fidelity disclosed).
    pub hydration_stays_no_rerun: bool,
    /// Whether the entry provides the complete preserved-pane-role / missing-dependency-class /
    /// restore-fidelity-hint disclosure triple.
    pub provides_complete_disclosure_triple: bool,
    /// Degrade reason, if the entry could not read as a clean, no-rerun state.
    pub degrade_reason: Option<M5SessionHydrationEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5RestoreOrchestrationNextAction,
    /// Whether the hydration is no-rerun on every claimed pane (clean entry naming every fact).
    pub hydration_no_rerun_on_every_pane: bool,
}

impl M5ResolvedSessionHydrationEntry {
    /// Whether this session-hydration entry reads as a clean, no-rerun state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5RestoreOrchestrationResolutionError {
    /// The skeleton-restore-entry id was empty.
    EmptySkeletonEntryId,
    /// The session-hydration-entry id was empty.
    EmptySessionHydrationEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5RestoreOrchestrationResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptySkeletonEntryId => "empty_skeleton_entry_id",
            Self::EmptySessionHydrationEntryId => "empty_session_hydration_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5RestoreOrchestrationResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 skeleton-restore / session-hydration registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5RestoreOrchestrationResolutionError {}

fn form_tokens(forms: &[M5RestoreOrchestrationResolutionForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_resolution_forms(forms: &[M5RestoreOrchestrationResolutionForm]) -> bool {
    let present: BTreeSet<M5RestoreOrchestrationResolutionForm> = forms.iter().copied().collect();
    M5RestoreOrchestrationResolutionForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the resolved restore-skeleton object publishes every required field: restore-fidelity mode (via a
/// classified class), window shell, stable pane-tree structure, preserved pane roles, placeholder set,
/// layout-skeleton root, and the distinct deferred-hydration plan. An unclassified class or any empty field
/// never resolves to a complete object.
#[allow(clippy::too_many_arguments)]
pub fn restore_skeleton_object_is_complete(
    class: M5RestoreFidelityClass,
    window_shell_id: &str,
    pane_tree_structure: &str,
    pane_role_set: &str,
    placeholder_set: &str,
    layout_skeleton_root: &str,
    hydration_plan_ref: &str,
) -> bool {
    class.is_classified()
        && !window_shell_id.trim().is_empty()
        && !pane_tree_structure.trim().is_empty()
        && !pane_role_set.trim().is_empty()
        && !placeholder_set.trim().is_empty()
        && !layout_skeleton_root.trim().is_empty()
        && !hydration_plan_ref.trim().is_empty()
}

/// Whether the layout skeleton is rebuilt before heavy hydration: the class must be classified, the skeleton
/// must be rebuilt before hydration, and a skeleton that defers heavy hydration must preserve its pane roles.
/// An unclassified class, hydration that preceded the skeleton, or dropped pane roles never matches.
pub fn skeleton_precedes_hydration(
    class: M5RestoreFidelityClass,
    skeleton_rebuilt_before_hydration: bool,
    defers_heavy_hydration: bool,
    pane_roles_preserved_when_deferred: bool,
) -> bool {
    class.is_classified()
        && skeleton_rebuilt_before_hydration
        && (!defers_heavy_hydration || pane_roles_preserved_when_deferred)
}

/// Whether a session hydration stays no-rerun and continuity-preserving: the surface must be classified, the
/// hydration must be truthful, it must preserve the pane role and surrounding topology, any missing dependency
/// must be placeholder-substituted rather than collapsed, and any deferred heavy dependency's restore fidelity
/// must be disclosed rather than overclaimed.
pub fn session_hydration_stays_no_rerun(
    surface: M5SessionHydrationSurface,
    hydration_is_truthful: bool,
    preserves_pane_role_and_topology: bool,
    dependency_missing_used: bool,
    placeholder_substituted_on_missing: bool,
    heavy_dependency_deferred: bool,
    deferred_fidelity_disclosed: bool,
) -> bool {
    surface.is_classified()
        && hydration_is_truthful
        && preserves_pane_role_and_topology
        && (!dependency_missing_used || placeholder_substituted_on_missing)
        && (!heavy_dependency_deferred || deferred_fidelity_disclosed)
}

/// Resolves a skeleton-restore-registry entry so it stays bound to the skeleton-restore registry: the entry
/// names its canonical token, semantic role, and restore-fidelity class, covers all three resolution forms,
/// publishes a complete restore-skeleton object (window shell, stable pane-tree structure, preserved pane roles,
/// placeholder set, layout-skeleton root, distinct deferred-hydration plan), rebuilds the skeleton before heavy
/// hydration, and preserves pane roles when it defers heavy hydration.
pub fn resolve_skeleton_restore_entry(
    input: M5SkeletonRestoreEntryResolutionInput,
) -> Result<M5ResolvedSkeletonRestoreEntry, M5RestoreOrchestrationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5RestoreOrchestrationResolutionError::EmptySkeletonEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.restore_target_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.window_shell_id)
        || string_is_forbidden(&input.pane_tree_structure)
        || string_is_forbidden(&input.pane_role_set)
        || string_is_forbidden(&input.placeholder_set)
        || string_is_forbidden(&input.layout_skeleton_root)
        || string_is_forbidden(&input.hydration_plan_ref)
    {
        return Err(M5RestoreOrchestrationResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let object_complete = restore_skeleton_object_is_complete(
        input.restore_fidelity_class,
        &input.window_shell_id,
        &input.pane_tree_structure,
        &input.pane_role_set,
        &input.placeholder_set,
        &input.layout_skeleton_root,
        &input.hydration_plan_ref,
    );
    let skeleton_ok = skeleton_precedes_hydration(
        input.restore_fidelity_class,
        input.skeleton_rebuilt_before_hydration,
        input.defers_heavy_hydration,
        input.pane_roles_preserved_when_deferred,
    );
    let roles_unpreserved =
        input.defers_heavy_hydration && !input.pane_roles_preserved_when_deferred;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5SkeletonRestoreEntryDegradeReason::SkeletonTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5SkeletonRestoreEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.restore_fidelity_class.is_classified() {
        Some(M5SkeletonRestoreEntryDegradeReason::RestoreFidelityClassUnclassified)
    } else if !input.bound_to_registry {
        Some(M5SkeletonRestoreEntryDegradeReason::SkeletonNotBoundToRegistry)
    } else if !object_complete {
        Some(M5SkeletonRestoreEntryDegradeReason::RestoreSkeletonObjectIncomplete)
    } else if !skeleton_ok {
        Some(M5SkeletonRestoreEntryDegradeReason::HydrationPrecededSkeleton)
    } else if !all_forms {
        Some(M5SkeletonRestoreEntryDegradeReason::ResolutionFormCoverageIncomplete)
    } else if roles_unpreserved {
        Some(M5SkeletonRestoreEntryDegradeReason::DeferredPaneRolesNotPreserved)
    } else if !input.proof_fresh {
        Some(M5SkeletonRestoreEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RestoreOrchestrationNextAction::ExpandRestoreMeaning,
    };

    Ok(M5ResolvedSkeletonRestoreEntry {
        entry_id: input.entry_id,
        restore_target_id: input.restore_target_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: input
            .semantic_role
            .must_preserve_window_local_selection_and_no_rerun_under_shared_authority(),
        restore_fidelity_class: input.restore_fidelity_class.as_str().to_owned(),
        restore_fidelity_class_is_classified: input.restore_fidelity_class.is_classified(),
        canonical_restore_fidelity_mode: input
            .restore_fidelity_class
            .canonical_restore_fidelity_mode()
            .to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        window_shell_id: input.window_shell_id,
        pane_tree_structure: input.pane_tree_structure,
        pane_role_set: input.pane_role_set,
        placeholder_set: input.placeholder_set,
        layout_skeleton_root: input.layout_skeleton_root,
        hydration_plan_ref: input.hydration_plan_ref,
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        restore_skeleton_object_complete: object_complete,
        bound_to_registry: input.bound_to_registry,
        skeleton_rebuilt_before_hydration: input.skeleton_rebuilt_before_hydration,
        defers_heavy_hydration: input.defers_heavy_hydration,
        pane_roles_preserved_when_deferred: input.pane_roles_preserved_when_deferred,
        degrade_reason,
        next_action,
        skeleton_resolves_across_restores: degrade_reason.is_none(),
    })
}

/// Resolves a session-hydration entry so its hydration stays no-rerun: the entry names its canonical token,
/// semantic role, and session-hydration surface, covers all three resolution forms, provides the
/// preserved-pane-role / missing-dependency-class / restore-fidelity-hint disclosure triple, and degrades
/// honestly when hydration reruns session-scoped work, reacquires broader authority, collapses layout on a
/// missing dependency, or overclaims restore fidelity on a deferred dependency.
pub fn resolve_session_hydration_entry(
    input: M5SessionHydrationEntryResolutionInput,
) -> Result<M5ResolvedSessionHydrationEntry, M5RestoreOrchestrationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5RestoreOrchestrationResolutionError::EmptySessionHydrationEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.pane_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.preserved_pane_role)
        || string_is_forbidden(&input.missing_dependency_class)
        || string_is_forbidden(&input.restore_fidelity_hint)
    {
        return Err(M5RestoreOrchestrationResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_resolution_forms(&input.resolution_form_coverage);
    let hydration_stays_no_rerun = session_hydration_stays_no_rerun(
        input.hydration_surface,
        input.hydration_is_truthful,
        input.preserves_pane_role_and_topology,
        input.dependency_missing_used,
        input.placeholder_substituted_on_missing,
        input.heavy_dependency_deferred,
        input.deferred_fidelity_disclosed,
    );
    let provides_triple = input.hydration_surface.is_classified()
        && !input.preserved_pane_role.trim().is_empty()
        && !input.missing_dependency_class.trim().is_empty()
        && !input.restore_fidelity_hint.trim().is_empty()
        && hydration_stays_no_rerun;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5SessionHydrationEntryDegradeReason::HydrationTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5SessionHydrationEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.hydration_surface.is_classified() {
        Some(M5SessionHydrationEntryDegradeReason::SessionHydrationSurfaceUnclassified)
    } else if !provides_triple {
        Some(M5SessionHydrationEntryDegradeReason::SessionHydrationRerunsOrCollapsesLayout)
    } else if !all_forms {
        Some(M5SessionHydrationEntryDegradeReason::HydrationFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5SessionHydrationEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5RestoreOrchestrationNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedSessionHydrationEntry {
        entry_id: input.entry_id,
        pane_id: input.pane_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_window_local_selection_and_no_rerun_under_shared_authority: input
            .semantic_role
            .must_preserve_window_local_selection_and_no_rerun_under_shared_authority(),
        hydration_surface: input.hydration_surface.as_str().to_owned(),
        hydration_surface_is_classified: input.hydration_surface.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        resolution_form_coverage: form_tokens(&input.resolution_form_coverage),
        covers_all_resolution_forms: all_forms,
        preserved_pane_role: input.preserved_pane_role,
        missing_dependency_class: input.missing_dependency_class,
        restore_fidelity_hint: input.restore_fidelity_hint,
        preserves_pane_role_and_topology: input.preserves_pane_role_and_topology,
        hydration_is_truthful: input.hydration_is_truthful,
        dependency_missing_used: input.dependency_missing_used,
        placeholder_substituted_on_missing: input.placeholder_substituted_on_missing,
        heavy_dependency_deferred: input.heavy_dependency_deferred,
        deferred_fidelity_disclosed: input.deferred_fidelity_disclosed,
        hydration_stays_no_rerun,
        provides_complete_disclosure_triple: provides_triple,
        degrade_reason,
        next_action,
        hydration_no_rerun_on_every_pane: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved skeleton-restore and session-hydration entries
/// it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SkeletonFirstRestoreSessionHydrationRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5SkeletonFirstRestoreSessionHydrationRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5RestoreOrchestrationAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5RestoreOrchestrationExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5WindowRestoreDowngradeTrigger>,
    /// Resolved skeleton-restore-registry examples.
    pub skeleton_restore_entries: Vec<M5ResolvedSkeletonRestoreEntry>,
    /// Resolved session-hydration examples.
    pub session_hydration_entries: Vec<M5ResolvedSessionHydrationEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include both the restore-fidelity and window-topology
    /// domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: session-scoped work never reruns or reattaches privileged sessions during restore. MUST
    /// be `false`.
    pub reruns_session_scoped_work_or_reattaches_privileged_sessions_during_restore: bool,
    /// Hard invariant: layout structure is never deleted silently on a missing dependency. MUST be `false`.
    pub deletes_layout_structure_silently_on_missing_dependency: bool,
    /// Hard invariant: skeleton and hydration state are never merged into one opaque blob. MUST be `false`.
    pub merges_skeleton_and_hydration_into_one_opaque_blob: bool,
    /// Hard invariant: restore fidelity is never overclaimed when only context or evidence reopened. MUST be
    /// `false`.
    pub overclaims_restore_fidelity_when_only_context_or_evidence_reopened: bool,
}

impl M5SkeletonFirstRestoreSessionHydrationRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5RestoreOrchestrationAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5RestoreOrchestrationAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5RestoreOrchestrationExportField> =
            self.export_fields.iter().copied().collect();
        M5RestoreOrchestrationExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.reruns_session_scoped_work_or_reattaches_privileged_sessions_during_restore
            && !self.deletes_layout_structure_silently_on_missing_dependency
            && !self.merges_skeleton_and_hydration_into_one_opaque_blob
            && !self.overclaims_restore_fidelity_when_only_context_or_evidence_reopened
    }

    /// True when a clean skeleton-restore entry preserves registry-bound truth: it traces to the registry,
    /// keeps a classified restore-fidelity class, publishes a complete skeleton object, rebuilds the skeleton
    /// before hydration, covers all three resolution forms, and preserves pane roles when it defers hydration.
    fn skeleton_is_honest(ex: &M5ResolvedSkeletonRestoreEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.restore_fidelity_class_is_classified
                && ex.restore_skeleton_object_complete
                && ex.skeleton_rebuilt_before_hydration
                && ex.covers_all_resolution_forms
                && (!ex.defers_heavy_hydration || ex.pane_roles_preserved_when_deferred))
    }

    /// True when a clean session-hydration entry preserves no-rerun continuity: it keeps a classified surface,
    /// provides the disclosure triple, stays no-rerun, and covers all three resolution forms.
    fn hydration_is_honest(ex: &M5ResolvedSessionHydrationEntry) -> bool {
        !ex.is_clean()
            || (ex.hydration_surface_is_classified
                && ex.provides_complete_disclosure_triple
                && ex.hydration_stays_no_rerun
                && ex.covers_all_resolution_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.skeleton_restore_entries
            .iter()
            .all(Self::skeleton_is_honest)
            && self
                .session_hydration_entries
                .iter()
                .all(Self::hydration_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SkeletonFirstRestoreSessionHydrationRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Resolution-form tokens (minted by this lane).
    pub resolution_forms: Vec<String>,
    /// Restore-fidelity-class tokens (minted by this lane).
    pub restore_fidelity_classes: Vec<String>,
    /// Session-hydration-surface tokens (minted by this lane).
    pub session_hydration_surfaces: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Skeleton-restore-entry degrade-reason tokens.
    pub skeleton_restore_degrade_reasons: Vec<String>,
    /// Session-hydration-entry degrade-reason tokens.
    pub session_hydration_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5SkeletonFirstRestoreSessionHydrationRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5WindowRestoreRole::ALL, |v| v.as_str()),
            resolution_forms: tokens(&M5RestoreOrchestrationResolutionForm::ALL, |v| v.as_str()),
            restore_fidelity_classes: tokens(&M5RestoreFidelityClass::ALL, |v| v.as_str()),
            session_hydration_surfaces: tokens(&M5SessionHydrationSurface::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5RestoreOrchestrationSurfaceContext::ALL, |v| v.as_str()),
            skeleton_restore_degrade_reasons: tokens(
                &M5SkeletonRestoreEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            session_hydration_degrade_reasons: tokens(
                &M5SessionHydrationEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5RestoreOrchestrationAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5RestoreOrchestrationNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5RestoreOrchestrationExportField::ALL, |v| v.as_str()),
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
pub struct M5SkeletonFirstRestoreSessionHydrationRegistriesGovernanceReview {
    /// The skeleton registry names a canonical token, semantic role, and restore-fidelity class for every
    /// entry.
    pub skeleton_registry_names_token_role_and_fidelity_class: bool,
    /// Every claimed restore rebuilds one stable restore-skeleton object from the shared registry, not
    /// per-pane reconstruction.
    pub restore_resolves_to_stable_skeleton_object_from_shared_registry: bool,
    /// Window shell, stable pane-tree structure, preserved pane roles, placeholder set, and the distinct
    /// deferred-hydration plan are published for every resolved restore.
    pub window_shell_pane_tree_roles_and_placeholders_published: bool,
    /// The layout skeleton is rebuilt before any heavy dependency hydrates.
    pub skeleton_rebuilt_before_heavy_hydration: bool,
    /// Session hydration preserves pane roles and never silently reruns session-scoped work or reacquires
    /// broader authority.
    pub session_hydration_keeps_pane_roles_and_never_reruns: bool,
    /// A missing dependency never collapses layout silently; it produces a pane-role-preserving placeholder.
    pub missing_dependency_never_collapses_layout_silently: bool,
    /// Every skeleton-restore and session-hydration entry covers the canonical / accessible / audit resolution
    /// forms.
    pub every_entry_covers_all_resolution_forms: bool,
    /// Skeleton-restore and session-hydration behavior stay bound to the shared registries rather than
    /// hand-copied per pane.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// Shell, recovery, diagnostics, and admin read a single restore-orchestration source.
    pub shell_recovery_diagnostics_admin_read_single_source: bool,
    /// A hydration-first restore, an incomplete object, or a collapsed layout is caught by fixtures before
    /// release evidence turns green.
    pub skeleton_or_hydration_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SkeletonFirstRestoreSessionHydrationRegistriesConsumerProjection {
    /// Shell and recovery consume the shared skeleton-restore registry.
    pub shell_and_recovery_consume_shared_registries: bool,
    /// Diagnostics and admin consume the shared session-hydration registry.
    pub diagnostics_and_admin_consume_shared_registries: bool,
    /// Session and workspace services consume the shared registries.
    pub session_and_workspace_services_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical restore-fidelity and window-topology domain contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical skeleton-restore / session-hydration registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SkeletonFirstRestoreSessionHydrationRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SkeletonFirstRestoreSessionHydrationRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting window-restore audit for the lane.
    pub window_restore_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SkeletonFirstRestoreSessionHydrationRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SkeletonFirstRestoreSessionHydrationRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SkeletonFirstRestoreSessionHydrationRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SkeletonFirstRestoreSessionHydrationRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SkeletonFirstRestoreSessionHydrationRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SkeletonFirstRestoreSessionHydrationRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SkeletonFirstRestoreSessionHydrationRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SkeletonFirstRestoreSessionHydrationRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 skeleton-first-restore and session-hydration registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SkeletonFirstRestoreSessionHydrationRegistriesPacket {
    /// Record kind; must equal [`M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5SkeletonFirstRestoreSessionHydrationRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SkeletonFirstRestoreSessionHydrationRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SkeletonFirstRestoreSessionHydrationRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SkeletonFirstRestoreSessionHydrationRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SkeletonFirstRestoreSessionHydrationRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SkeletonFirstRestoreSessionHydrationRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SkeletonFirstRestoreSessionHydrationRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5SkeletonFirstRestoreSessionHydrationRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_RECORD_KIND
                .to_owned(),
            schema_version: M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5SkeletonFirstRestoreSessionHydrationRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_RECORD_KIND {
            violations
                .push(M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version
            != M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_SCHEMA_VERSION
        {
            violations.push(
                M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::WrongSchemaVersion,
            );
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations
                .push(M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(
                M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::VocabularySetDrift,
            );
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 skeleton-restore / session-hydration registries packet serializes"),
        ) {
            violations.push(
                M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::RawMaterialInExport,
            );
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
            .expect("m5 skeleton-restore / session-hydration registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,skeleton_restore_entries,session_hydration_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .skeleton_restore_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.session_hydration_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.skeleton_restore_entries.len(),
                row.session_hydration_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Skeleton-First-Restore and Session-Hydration Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Restore-fidelity classes: {}\n",
            self.vocabulary_set.restore_fidelity_classes.join(", ")
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
                "  - Skeleton-restore entries: {} / session-hydration entries: {}\n",
                row.skeleton_restore_entries.len(),
                row.session_hydration_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-restore fidelity reference table generated from the registry, so docs and admin
    /// runbooks render the same restore-fidelity-mode / window-shell / pane-tree / pane-role / placeholder /
    /// layout-skeleton-root truth the resolvers produced rather than a hand-copied restore table. Only clean,
    /// registry-bound skeleton-restore entries are listed.
    pub fn render_restore_fidelity_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| restore_target_id | restore_fidelity_mode | window_shell_id | pane_tree_structure | pane_role_set | placeholder_set | layout_skeleton_root |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.skeleton_restore_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                    ex.restore_target_id,
                    ex.canonical_restore_fidelity_mode,
                    ex.window_shell_id,
                    ex.pane_tree_structure,
                    ex.pane_role_set,
                    ex.placeholder_set,
                    ex.layout_skeleton_root
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5SkeletonFirstRestoreSessionHydrationRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SkeletonFirstRestoreSessionHydrationRegistriesViolation>),
}

impl fmt::Display for M5SkeletonFirstRestoreSessionHydrationRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 skeleton-restore / session-hydration registries export parse failed: {error}"
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
                    "m5 skeleton-restore / session-hydration registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SkeletonFirstRestoreSessionHydrationRegistriesArtifactError {}

/// Validation failures emitted by [`M5SkeletonFirstRestoreSessionHydrationRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SkeletonFirstRestoreSessionHydrationRegistriesViolation {
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
    /// A registry row does not point at both the restore-fidelity and window-topology domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, hydration-first, field-incomplete,
    /// form-incomplete, or a session-hydration entry missing the disclosure triple).
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
    /// Skeleton-object-resolution is not proven: clean skeleton entries do not cover the canonical
    /// restore-fidelity classes or the first shell / recovery / diagnostics / admin / support surfaces, no
    /// object-incomplete example degrades, or a clean skeleton entry published an incomplete object.
    SkeletonObjectResolutionNotProven,
    /// Skeleton-before-hydration is not proven: no hydration-first example and no unbound example degrade, no
    /// clean skeleton-first entry is present, or a clean skeleton entry ran hydration first or is unbound.
    SkeletonBeforeHydrationNotProven,
    /// Pane-role-placeholder continuity is not proven: clean session-hydration entries do not cover the
    /// canonical terminal / debugger / preview surfaces with full resolution-form coverage while providing the
    /// disclosure triple, no reruns-or-collapses or form-incomplete example degrades, or a clean
    /// session-hydration entry is missing the triple.
    PaneRolePlaceholderContinuityNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SkeletonFirstRestoreSessionHydrationRegistriesViolation {
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
            Self::SkeletonObjectResolutionNotProven => "skeleton_object_resolution_not_proven",
            Self::SkeletonBeforeHydrationNotProven => "skeleton_before_hydration_not_proven",
            Self::PaneRolePlaceholderContinuityNotProven => {
                "pane_role_placeholder_continuity_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_skeleton_first_restore_and_session_hydration_registries_export() -> Result<
    M5SkeletonFirstRestoreSessionHydrationRegistriesPacket,
    M5SkeletonFirstRestoreSessionHydrationRegistriesArtifactError,
> {
    let packet: M5SkeletonFirstRestoreSessionHydrationRegistriesPacket = serde_json::from_str(include_str!(
        concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-skeleton-first-restore-and-session-hydration-registries-proof/support_export.json"
        )
    ))
    .map_err(M5SkeletonFirstRestoreSessionHydrationRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SkeletonFirstRestoreSessionHydrationRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5SkeletonFirstRestoreSessionHydrationRegistriesPacket,
    violations: &mut Vec<M5SkeletonFirstRestoreSessionHydrationRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_SCHEMA_REF,
        M5_SKELETON_FIRST_RESTORE_SESSION_HYDRATION_REGISTRIES_DOC_REF,
        M5_WINDOW_RESTORE_MATRIX_SCHEMA_REF,
        M5_WINDOW_RESTORE_MATRIX_DOC_REF,
        M5_RESTORE_FIDELITY_SCHEMA_REF,
        M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5SkeletonFirstRestoreSessionHydrationRegistriesPacket,
    violations: &mut Vec<M5SkeletonFirstRestoreSessionHydrationRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::NoRegistryRows);
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
            violations.push(
                M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::RegistryRowIncomplete,
            );
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_RESTORE_FIDELITY_SCHEMA_REF)
            || !refs.contains(M5_WINDOW_TOPOLOGY_DOMAIN_SCHEMA_REF)
        {
            violations.push(
                M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::DomainSchemaRefMissing,
            );
        }
        if row.skeleton_restore_entries.is_empty() || row.session_hydration_entries.is_empty() {
            violations
                .push(M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations
                .push(M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(
                M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::RowInvariantViolated,
            );
        }
    }
}

fn validate_governance_review(
    packet: &M5SkeletonFirstRestoreSessionHydrationRegistriesPacket,
    violations: &mut Vec<M5SkeletonFirstRestoreSessionHydrationRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.skeleton_registry_names_token_role_and_fidelity_class,
        review.restore_resolves_to_stable_skeleton_object_from_shared_registry,
        review.window_shell_pane_tree_roles_and_placeholders_published,
        review.skeleton_rebuilt_before_heavy_hydration,
        review.session_hydration_keeps_pane_roles_and_never_reruns,
        review.missing_dependency_never_collapses_layout_silently,
        review.every_entry_covers_all_resolution_forms,
        review.behavior_bound_to_registry_not_hand_copied,
        review.shell_recovery_diagnostics_admin_read_single_source,
        review.skeleton_or_hydration_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SkeletonFirstRestoreSessionHydrationRegistriesPacket,
    violations: &mut Vec<M5SkeletonFirstRestoreSessionHydrationRegistriesViolation>,
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
                M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SkeletonFirstRestoreSessionHydrationRegistriesPacket,
    violations: &mut Vec<M5SkeletonFirstRestoreSessionHydrationRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(
            M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::ProofFreshnessIncomplete,
        );
    }
}

fn validate_release_posture(
    packet: &M5SkeletonFirstRestoreSessionHydrationRegistriesPacket,
    violations: &mut Vec<M5SkeletonFirstRestoreSessionHydrationRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.window_restore_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(
            M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::ReleasePostureIncomplete,
        );
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted by
/// governance bools.
fn validate_acceptance_criteria(
    packet: &M5SkeletonFirstRestoreSessionHydrationRegistriesPacket,
    violations: &mut Vec<M5SkeletonFirstRestoreSessionHydrationRegistriesViolation>,
) {
    let skeletons = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.skeleton_restore_entries.iter())
    };
    let hydrations = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.session_hydration_entries.iter())
    };

    // AC (support/export explains live vs placeholder vs context-only vs evidence-only): every claimed restore
    // rebuilds one stable restore-skeleton object with window-shell / pane-tree / pane-role / placeholder /
    // distinct-plan fields. Clean skeleton entries cover the canonical restore-fidelity classes and the first
    // shell / recovery / diagnostics / admin / support surfaces, an object-incomplete example degrades, and no
    // clean skeleton entry published an incomplete object.
    let clean_classes: BTreeSet<String> = skeletons()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.restore_fidelity_class.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = skeletons()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let classes_covered = M5RestoreFidelityClass::CANONICAL_CLASSES
        .iter()
        .all(|c| clean_classes.contains(c.as_str()));
    let first_surfaces_covered = M5RestoreOrchestrationSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let object_incomplete_degrades = skeletons().any(|ex| {
        ex.degrade_reason
            == Some(M5SkeletonRestoreEntryDegradeReason::RestoreSkeletonObjectIncomplete)
    });
    let no_clean_incomplete =
        !skeletons().any(|ex| ex.is_clean() && !ex.restore_skeleton_object_complete);
    if !(classes_covered
        && first_surfaces_covered
        && object_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::SkeletonObjectResolutionNotProven,
        );
    }

    // AC (restored layouts appear as truthful skeletons before heavy services hydrate): the layout skeleton is
    // rebuilt before heavy hydration. A hydration-first example degrades, an unbound example degrades, at least
    // one clean skeleton-first entry is present, and no clean skeleton entry ran hydration first or is unbound.
    let preceded_degrades = skeletons().any(|ex| {
        ex.degrade_reason == Some(M5SkeletonRestoreEntryDegradeReason::HydrationPrecededSkeleton)
    });
    let unbound_degrades = skeletons().any(|ex| {
        ex.degrade_reason == Some(M5SkeletonRestoreEntryDegradeReason::SkeletonNotBoundToRegistry)
    });
    let rebuilt_clean_skeleton =
        skeletons().any(|ex| ex.is_clean() && ex.skeleton_rebuilt_before_hydration);
    let no_clean_unbound = !skeletons().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    let no_clean_hydration_first =
        !skeletons().any(|ex| ex.is_clean() && !ex.skeleton_rebuilt_before_hydration);
    if !(preceded_degrades
        && unbound_degrades
        && rebuilt_clean_skeleton
        && no_clean_unbound
        && no_clean_hydration_first)
    {
        violations.push(
            M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::SkeletonBeforeHydrationNotProven,
        );
    }

    // AC (missing dependencies produce pane-role-preserving placeholders instead of silent layout collapse):
    // clean session-hydration entries cover every canonical terminal / debugger / preview surface with full
    // resolution-form coverage while providing the disclosure triple, a reruns-or-collapses example degrades, a
    // form-incomplete example degrades, and no clean session-hydration entry is missing the triple.
    let clean_hydration_surfaces: BTreeSet<String> = hydrations()
        .filter(|ex| {
            ex.is_clean()
                && ex.hydration_surface_is_classified
                && ex.provides_complete_disclosure_triple
                && ex.covers_all_resolution_forms
        })
        .map(|ex| ex.hydration_surface.clone())
        .collect();
    let hydration_surfaces_covered = M5SessionHydrationSurface::CANONICAL_SURFACES
        .iter()
        .all(|s| clean_hydration_surfaces.contains(s.as_str()));
    let collapses_degrades = hydrations().any(|ex| {
        ex.degrade_reason
            == Some(M5SessionHydrationEntryDegradeReason::SessionHydrationRerunsOrCollapsesLayout)
    });
    let form_incomplete_degrades = hydrations().any(|ex| {
        ex.degrade_reason
            == Some(M5SessionHydrationEntryDegradeReason::HydrationFormCoverageIncomplete)
    });
    let no_clean_missing_triple =
        !hydrations().any(|ex| ex.is_clean() && !ex.provides_complete_disclosure_triple);
    if !(hydration_surfaces_covered
        && collapses_degrades
        && form_incomplete_degrades
        && no_clean_missing_triple)
    {
        violations.push(
            M5SkeletonFirstRestoreSessionHydrationRegistriesViolation::PaneRolePlaceholderContinuityNotProven,
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
    M5WindowRestoreFamily::SkeletonFirstRestore,
    M5WindowRestoreFamily::NoRerunSessionHydration,
];
