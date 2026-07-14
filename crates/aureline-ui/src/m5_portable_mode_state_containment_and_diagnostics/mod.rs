//! Implemented M5 portable-mode state-containment and diagnostics registries.
//!
//! The frozen [install-topology matrix][matrix] names Aureline's portable-mode delivery-topology family and
//! the first implement lane [over the whole matrix][registries] resolves the portable-mode / offline-air-gap
//! state-root boundary. This module is the portable-mode runtime-enforcement lane: it makes *portable* a
//! contract instead of a marketing shortcut. It turns the *colocated-or-named-sibling state layout* grammar
//! and the *discoverable portable-mode diagnostics* grammar into registry resolvers that produce export-safe,
//! honest projections. A claimed portable profile then resolves to one stable portable-state layout — the
//! executable root, the colocated / named-sibling state roots, and the full durable-root inventory (durable
//! settings, stored secrets, background services, and shell hooks) — that proves hidden machine-global
//! mutation is absent or explicitly blocked, and to one discoverable portable-diagnostics record — executable
//! root, state roots, log / crash locations, update posture, and any explicitly unsupported shell-integration
//! paths — that support and export surfaces can distinguish from ordinary installed state without guessing.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Enforce colocated or explicitly named sibling-state layouts and block hidden machine-global writes.**
//!   [`resolve_portable_state_layout_entry`] refuses to read as a clean, registry-bound layout unless it names
//!   a canonical token, a classified [containment][M5PortableStateContainment], a portable role, covers every
//!   [presentation form][M5PortablePresentationForm], inventories every mandatory durable state class, keeps a
//!   distinguishable [state origin][M5PortableStateOrigin], and proves no durable settings, secrets, services,
//!   or shell hooks spilled into a hidden machine-global path; a spill degrades to
//!   [`M5PortableStateLayoutEntryDegradeReason::HiddenMachineGlobalDurableSpill`].
//! * **Publish discoverable portable-mode diagnostics.** [`resolve_portable_diagnostics_entry`] names a
//!   classified [diagnostics surface][M5PortableDiagnosticsSurface], must disclose every mandatory
//!   [diagnostics field][M5PortableDiagnosticsField] (executable root, state roots, log / crash locations,
//!   update posture, and unsupported shell-integration paths), and degrades to
//!   [`M5PortableDiagnosticsEntryDegradeReason::DiagnosticsDisclosureIncomplete`] when a field or an
//!   unsupported shell-integration path is left implicit.
//! * **Preserve manual or tightly-controlled portable-update continuity.** A diagnostics entry that does not
//!   classify its [update posture][M5PortableUpdatePosture] or document retained-versus-replaced state
//!   degrades to [`M5PortableDiagnosticsEntryDegradeReason::UpdateContinuityUndocumented`].
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5InstallTopologyRole`] role vocabulary,
//! the [`M5InstallTopologyConsumerSurface`] consumer-surface taxonomy, and the matrix downgrade triggers — so
//! About, update, diagnostics, admin, installer, docs, CLI, and support surfaces can never fork their own
//! portable-mode meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_install_topology_matrix
//! [registries]: crate::m5_install_topology_and_state_root_registries

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_portable_mode_state_containment_and_diagnostics,
    seeded_m5_portable_mode_state_containment_and_diagnostics_offline_airgap_bundle_preview_narrowed,
    seeded_m5_portable_mode_state_containment_and_diagnostics_side_by_side_channel_beta_narrowed,
    M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_install_topology_and_state_root_registries::M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF;
use crate::m5_install_topology_matrix::{
    M5InstallTopologyAccessibilityRoute, M5InstallTopologyConsumerSurface,
    M5InstallTopologyDeploymentLine, M5InstallTopologyDowngradeTrigger, M5InstallTopologyFamily,
    M5InstallTopologyQualificationClass, M5InstallTopologyRequiredLabel, M5InstallTopologyRole,
    M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF, M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
    M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5PortableModeStateContainmentAndDiagnosticsPacket`].
pub const M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_RECORD_KIND: &str =
    "implement_m5_portable_mode_state_containment_and_diagnostics";

/// Schema version for M5 portable-mode state-containment / diagnostics registry records.
pub const M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_SCHEMA_REF: &str =
    "schemas/install/m5-portable-mode-state-containment-and-diagnostics.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_DOC_REF: &str =
    "docs/install/m5_portable_mode_state_containment_and_diagnostics.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_ARTIFACT_REF: &str =
    "artifacts/release/m5-portable-mode-state-containment-and-diagnostics-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_CSV_REF: &str =
    "artifacts/release/m5-portable-mode-state-containment-and-diagnostics-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_REPORT_REF: &str =
    "artifacts/release/m5-portable-mode-state-containment-and-diagnostics-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_FIXTURE_DIR: &str =
    "fixtures/install/m5-portable-mode-state-containment-and-diagnostics";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5PortableModeConsumerSurface = M5InstallTopologyConsumerSurface;

/// One of the three presentation forms every portable-state layout or diagnostics entry must hold across so
/// its truth keeps whether it is shown as the canonical resolved object, announced as an accessible summary,
/// or written to the audit / support record. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortablePresentationForm {
    /// The canonical resolved portable-state layout / diagnostics object.
    CanonicalObject,
    /// The accessible plain-language summary that keeps the resolved portable truth discoverable without
    /// visuals.
    AccessibleSummary,
    /// The audit / support-export record that keeps the resolved portable truth inspectable off-renderer.
    AuditRecord,
}

impl M5PortablePresentationForm {
    /// Every presentation form, in declaration order. A clean entry must cover all three.
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

/// Controlled portable-mode state-containment layout a layout entry resolves, so a portable profile keeps its
/// durable state colocated with the executable or in an explicitly named sibling directory rather than
/// deriving a hidden per-profile path. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortableStateContainment {
    /// State is colocated under the executable's own portable root.
    ColocatedUnderExecutable,
    /// State lives in an explicitly named sibling directory next to the executable root.
    NamedSiblingDirectory,
    /// State spilled into a hidden machine-global path, which is disallowed.
    HiddenMachineGlobal,
    /// The containment is unclassified, which is disallowed.
    ContainmentUnclassified,
}

impl M5PortableStateContainment {
    /// Every containment, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ColocatedUnderExecutable,
        Self::NamedSiblingDirectory,
        Self::HiddenMachineGlobal,
        Self::ContainmentUnclassified,
    ];

    /// The two canonical portable containments every claimed portable profile resolves against.
    pub const CANONICAL_CONTAINMENTS: [Self; 2] =
        [Self::ColocatedUnderExecutable, Self::NamedSiblingDirectory];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ColocatedUnderExecutable => "colocated_under_executable",
            Self::NamedSiblingDirectory => "named_sibling_directory",
            Self::HiddenMachineGlobal => "hidden_machine_global",
            Self::ContainmentUnclassified => "containment_unclassified",
        }
    }

    /// Whether the containment keeps state colocated with, or in a named sibling of, the executable (never a
    /// hidden machine-global path and never unclassified).
    pub const fn is_colocated_or_sibling(self) -> bool {
        matches!(
            self,
            Self::ColocatedUnderExecutable | Self::NamedSiblingDirectory
        )
    }
}

/// Controlled durable state class portable mode must contain inside a documented portable root, so no durable
/// settings, secrets, services, or shell hooks silently leak into a hidden machine-global path. Minted by this
/// lane, tracking the durable classes the spec names by hand.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortableDurableStateClass {
    /// Durable settings and preferences.
    DurableSettings,
    /// Stored secrets and credential material (kept in the portable secure store).
    StoredSecrets,
    /// Background services and daemons registered by the app.
    BackgroundServices,
    /// Shell hooks and OS integration points.
    ShellHooks,
    /// Caches, logs, and crash artifacts.
    CacheAndLogs,
}

impl M5PortableDurableStateClass {
    /// Every durable state class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::DurableSettings,
        Self::StoredSecrets,
        Self::BackgroundServices,
        Self::ShellHooks,
        Self::CacheAndLogs,
    ];

    /// The four durable classes a portable layout must inventory before it can read as complete — the exact
    /// classes the guardrail forbids spilling into a hidden machine-global path.
    pub const MANDATORY: [Self; 4] = [
        Self::DurableSettings,
        Self::StoredSecrets,
        Self::BackgroundServices,
        Self::ShellHooks,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableSettings => "durable_settings",
            Self::StoredSecrets => "stored_secrets",
            Self::BackgroundServices => "background_services",
            Self::ShellHooks => "shell_hooks",
            Self::CacheAndLogs => "cache_and_logs",
        }
    }
}

/// Controlled state origin a layout entry resolves, so support and export can tell portable state from
/// ordinary installed state without guessing. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortableStateOrigin {
    /// State colocated under the portable executable root.
    PortableColocated,
    /// State in an explicitly named portable sibling directory.
    PortableNamedSibling,
    /// Ordinary installed state (per-user or per-machine roots), explicitly not portable.
    OrdinaryInstalled,
    /// The origin cannot be distinguished, which is disallowed.
    OriginAmbiguous,
}

impl M5PortableStateOrigin {
    /// Every state origin, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PortableColocated,
        Self::PortableNamedSibling,
        Self::OrdinaryInstalled,
        Self::OriginAmbiguous,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PortableColocated => "portable_colocated",
            Self::PortableNamedSibling => "portable_named_sibling",
            Self::OrdinaryInstalled => "ordinary_installed",
            Self::OriginAmbiguous => "origin_ambiguous",
        }
    }

    /// Whether the origin is distinguishable (never the ambiguous sentinel).
    pub const fn is_distinguishable(self) -> bool {
        !matches!(self, Self::OriginAmbiguous)
    }
}

/// Controlled diagnostics surface a diagnostics entry resolves, so the discoverable portable-mode diagnostics
/// share one registry rather than a per-surface reconstruction. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortableDiagnosticsSurface {
    /// The in-product portable-mode diagnostics card.
    PortableDiagnosticsCard,
    /// The support-export diagnostics record.
    SupportExportDiagnostics,
    /// The docs / help diagnostics reference.
    DocsHelpDiagnostics,
    /// The diagnostics surface is unclassified, which is disallowed.
    SurfaceUnclassified,
}

impl M5PortableDiagnosticsSurface {
    /// Every diagnostics surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PortableDiagnosticsCard,
        Self::SupportExportDiagnostics,
        Self::DocsHelpDiagnostics,
        Self::SurfaceUnclassified,
    ];

    /// The three canonical diagnostics surfaces the discoverable diagnostics must stay complete across.
    pub const CANONICAL_SURFACES: [Self; 3] = [
        Self::PortableDiagnosticsCard,
        Self::SupportExportDiagnostics,
        Self::DocsHelpDiagnostics,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PortableDiagnosticsCard => "portable_diagnostics_card",
            Self::SupportExportDiagnostics => "support_export_diagnostics",
            Self::DocsHelpDiagnostics => "docs_help_diagnostics",
            Self::SurfaceUnclassified => "surface_unclassified",
        }
    }

    /// Whether the diagnostics surface is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::SurfaceUnclassified)
    }
}

/// One diagnostics field the discoverable portable-mode diagnostics must publish so nothing about the portable
/// runtime's roots, update posture, or unsupported shell integration is left implicit. Minted by this lane,
/// tracking the fields the implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortableDiagnosticsField {
    /// The executable root the portable app runs from.
    ExecutableRoot,
    /// The writable state roots the portable app owns.
    StateRoots,
    /// The log and crash-artifact locations.
    LogAndCrashLocations,
    /// The update posture (manual replace / tightly-controlled in-place / unsupported).
    UpdatePosture,
    /// The explicitly unsupported shell-integration paths.
    UnsupportedShellIntegrationPaths,
}

impl M5PortableDiagnosticsField {
    /// Every diagnostics field, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ExecutableRoot,
        Self::StateRoots,
        Self::LogAndCrashLocations,
        Self::UpdatePosture,
        Self::UnsupportedShellIntegrationPaths,
    ];

    /// Every field is mandatory: a discoverable diagnostics record must publish all five.
    pub const MANDATORY: [Self; 5] = Self::ALL;

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExecutableRoot => "executable_root",
            Self::StateRoots => "state_roots",
            Self::LogAndCrashLocations => "log_and_crash_locations",
            Self::UpdatePosture => "update_posture",
            Self::UnsupportedShellIntegrationPaths => "unsupported_shell_integration_paths",
        }
    }
}

/// Controlled portable-update posture a diagnostics entry resolves, so retained-versus-replaced state stays
/// documented under a manual or tightly-controlled update flow. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortableUpdatePosture {
    /// Manual replace: the user swaps the portable bundle and state is retained alongside.
    ManualReplace,
    /// Tightly-controlled in-place update with explicit retained-versus-replaced continuity notes.
    TightlyControlledInPlace,
    /// Automatic updates are explicitly unsupported in portable mode.
    UpdatesUnsupported,
    /// The update posture is unclassified, which is disallowed.
    PostureUnclassified,
}

impl M5PortableUpdatePosture {
    /// Every update posture, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ManualReplace,
        Self::TightlyControlledInPlace,
        Self::UpdatesUnsupported,
        Self::PostureUnclassified,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManualReplace => "manual_replace",
            Self::TightlyControlledInPlace => "tightly_controlled_in_place",
            Self::UpdatesUnsupported => "updates_unsupported",
            Self::PostureUnclassified => "posture_unclassified",
        }
    }

    /// Whether the update posture is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::PostureUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a portable token's
/// meaning stays stable whether it appears in About, the update flow, diagnostics, admin, or a support / export
/// form. Minted by this lane.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortableSurfaceContext {
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

impl M5PortableSurfaceContext {
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

/// One mandatory rendered part a portable-state layout or diagnostics entry must be able to show, so no
/// executable root, state root, durable class, update posture, or registry fact is left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortableAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The containment the layout resolves (layout entry).
    Containment,
    /// The executable root and colocated state roots (both entries).
    ExecutableAndStateRoots,
    /// The presentation-form coverage (canonical / accessible / audit).
    PresentationFormCoverage,
    /// The durable-root inventory (layout entry).
    DurableRootInventory,
    /// The state origin distinguishing portable from installed state (layout entry).
    StateOrigin,
    /// The update posture and continuity note (diagnostics entry).
    UpdatePostureAndContinuity,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the resolved layout or diagnostics record (both entries).
    PlainLanguageMeaning,
}

impl M5PortableAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::Containment,
        Self::ExecutableAndStateRoots,
        Self::PresentationFormCoverage,
        Self::DurableRootInventory,
        Self::StateOrigin,
        Self::UpdatePostureAndContinuity,
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
            Self::Containment => "containment",
            Self::ExecutableAndStateRoots => "executable_and_state_roots",
            Self::PresentationFormCoverage => "presentation_form_coverage",
            Self::DurableRootInventory => "durable_root_inventory",
            Self::StateOrigin => "state_origin",
            Self::UpdatePostureAndContinuity => "update_posture_and_continuity",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a resolved
/// layout, a diagnostics record, or a degraded entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortableNextAction {
    /// Expand the resolved layout's or diagnostics record's plain-language meaning.
    ExpandPortableMeaning,
    /// Inspect the containment or diagnostics surface the entry resolves.
    InspectContainmentOrSurface,
    /// Complete the canonical / accessible / audit presentation-form coverage.
    CompletePresentationFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5PortableNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandPortableMeaning,
        Self::InspectContainmentOrSurface,
        Self::CompletePresentationFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandPortableMeaning => "expand_portable_meaning",
            Self::InspectContainmentOrSurface => "inspect_containment_or_surface",
            Self::CompletePresentationFormCoverage => "complete_presentation_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortableExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The portable-mode families covered.
    PortableFamilies,
    /// The containments carried.
    Containments,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The presentation forms covered.
    PresentationForms,
    /// The diagnostics surfaces carried.
    DiagnosticsSurfaces,
    /// The render / surface context.
    SurfaceContext,
    /// The state origins carried.
    StateOrigins,
    /// The accountable owner role.
    OwnerRole,
}

impl M5PortableExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::PortableFamilies,
        Self::Containments,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::PresentationForms,
        Self::DiagnosticsSurfaces,
        Self::SurfaceContext,
        Self::StateOrigins,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::PortableFamilies,
        Self::Containments,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::PortableFamilies => "portable_families",
            Self::Containments => "containments",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::PresentationForms => "presentation_forms",
            Self::DiagnosticsSurfaces => "diagnostics_surfaces",
            Self::SurfaceContext => "surface_context",
            Self::StateOrigins => "state_origins",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a portable-state layout entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, spilling, inventory-incomplete, or
/// form-incomplete layout read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortableStateLayoutEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the layout means.
    LayoutTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The containment is unclassified (not in the resolved taxonomy).
    ContainmentUnclassified,
    /// The behavior is a hand-copied per-profile assumption instead of tracing to the canonical registry.
    LayoutNotBoundToRegistry,
    /// The durable-root inventory is incomplete: a mandatory durable class (settings, secrets, services, or
    /// shell hooks) is not inventoried inside a documented portable root, or the executable / state roots are
    /// unstated.
    DurableRootInventoryIncomplete,
    /// Portable mode wrote durable settings, secrets, services, or shell hooks into a hidden machine-global
    /// path, and that write is not explicitly blocked.
    HiddenMachineGlobalDurableSpill,
    /// The state origin is ambiguous, so support / export cannot tell portable state from installed state.
    StateOriginAmbiguous,
    /// The canonical / accessible / audit presentation-form coverage is incomplete.
    PresentationFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PortableStateLayoutEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::LayoutTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::ContainmentUnclassified,
        Self::LayoutNotBoundToRegistry,
        Self::DurableRootInventoryIncomplete,
        Self::HiddenMachineGlobalDurableSpill,
        Self::StateOriginAmbiguous,
        Self::PresentationFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LayoutTokenUnstated => "layout_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ContainmentUnclassified => "containment_unclassified",
            Self::LayoutNotBoundToRegistry => "layout_not_bound_to_registry",
            Self::DurableRootInventoryIncomplete => "durable_root_inventory_incomplete",
            Self::HiddenMachineGlobalDurableSpill => "hidden_machine_global_durable_spill",
            Self::StateOriginAmbiguous => "state_origin_ambiguous",
            Self::PresentationFormCoverageIncomplete => "presentation_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5PortableNextAction {
        match self {
            Self::LayoutTokenUnstated | Self::LayoutNotBoundToRegistry => {
                M5PortableNextAction::TraceCanonicalRegistry
            }
            Self::ContainmentUnclassified
            | Self::DurableRootInventoryIncomplete
            | Self::HiddenMachineGlobalDurableSpill
            | Self::StateOriginAmbiguous => M5PortableNextAction::InspectContainmentOrSurface,
            Self::PresentationFormCoverageIncomplete => {
                M5PortableNextAction::CompletePresentationFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5PortableNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5InstallTopologyDowngradeTrigger {
        match self {
            Self::LayoutTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::PresentationFormCoverageIncomplete => {
                M5InstallTopologyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::ContainmentUnclassified | Self::DurableRootInventoryIncomplete => {
                M5InstallTopologyDowngradeTrigger::StateRootUnstated
            }
            Self::LayoutNotBoundToRegistry | Self::StateOriginAmbiguous => {
                M5InstallTopologyDowngradeTrigger::StateRootBoundaryDriftedByTopology
            }
            Self::HiddenMachineGlobalDurableSpill => {
                M5InstallTopologyDowngradeTrigger::PortableModeWroteHiddenMachineGlobalDurableState
            }
            Self::ProofStale => M5InstallTopologyDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a portable-diagnostics entry degraded below a clean, discoverable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortableDiagnosticsEntryDegradeReason {
    /// The canonical registry token name is unstated.
    DiagnosticsTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The diagnostics surface is unclassified (not in the resolved taxonomy).
    DiagnosticsSurfaceUnclassified,
    /// The diagnostics disclosure is incomplete — a mandatory field (executable root, state roots, log / crash
    /// locations, update posture, or unsupported shell-integration paths) is missing.
    DiagnosticsDisclosureIncomplete,
    /// The update posture is unclassified or the retained-versus-replaced continuity note is absent.
    UpdateContinuityUndocumented,
    /// The canonical / accessible / audit presentation-form coverage of the diagnostics record is incomplete.
    DiagnosticsFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PortableDiagnosticsEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DiagnosticsTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::DiagnosticsSurfaceUnclassified,
        Self::DiagnosticsDisclosureIncomplete,
        Self::UpdateContinuityUndocumented,
        Self::DiagnosticsFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiagnosticsTokenUnstated => "diagnostics_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::DiagnosticsSurfaceUnclassified => "diagnostics_surface_unclassified",
            Self::DiagnosticsDisclosureIncomplete => "diagnostics_disclosure_incomplete",
            Self::UpdateContinuityUndocumented => "update_continuity_undocumented",
            Self::DiagnosticsFormCoverageIncomplete => "diagnostics_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5PortableNextAction {
        match self {
            Self::DiagnosticsTokenUnstated => M5PortableNextAction::TraceCanonicalRegistry,
            Self::DiagnosticsSurfaceUnclassified
            | Self::DiagnosticsDisclosureIncomplete
            | Self::UpdateContinuityUndocumented => {
                M5PortableNextAction::InspectContainmentOrSurface
            }
            Self::DiagnosticsFormCoverageIncomplete => {
                M5PortableNextAction::CompletePresentationFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5PortableNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5InstallTopologyDowngradeTrigger {
        match self {
            Self::DiagnosticsTokenUnstated
            | Self::SurfaceContextUnresolved
            | Self::DiagnosticsSurfaceUnclassified
            | Self::DiagnosticsFormCoverageIncomplete => {
                M5InstallTopologyDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::DiagnosticsDisclosureIncomplete => {
                M5InstallTopologyDowngradeTrigger::StateRootUnstated
            }
            Self::UpdateContinuityUndocumented => {
                M5InstallTopologyDowngradeTrigger::RollbackTargetedPrimaryExecutableWhileSidecarsDrifted
            }
            Self::ProofStale => M5InstallTopologyDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_portable_state_layout_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PortableStateLayoutEntryResolutionInput {
    /// Stable identity of the layout entry.
    pub entry_id: String,
    /// The stable portable-profile ID this layout binds to; empty means unstated.
    pub profile_id: String,
    /// The canonical registry token name (e.g. `portable.layout.colocated`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5InstallTopologyRole,
    /// The containment this entry resolves.
    pub containment: M5PortableStateContainment,
    /// The render / surface context.
    pub surface_context: M5PortableSurfaceContext,
    /// The presentation forms this entry holds across (must cover canonical / accessible / audit).
    pub presentation_form_coverage: Vec<M5PortablePresentationForm>,
    /// The published executable root (a filesystem path, never a URL); empty means unstated.
    pub executable_root: String,
    /// The published colocated / named-sibling writable state roots (filesystem paths); empty means unstated.
    pub colocated_state_root: String,
    /// The published log / crash-artifact root (a filesystem path); empty means unstated.
    pub log_and_crash_root: String,
    /// The durable state classes inventoried inside a documented portable root (must cover every mandatory
    /// class).
    pub durable_classes_covered: Vec<M5PortableDurableStateClass>,
    /// The state origin distinguishing portable state from ordinary installed state.
    pub state_origin: M5PortableStateOrigin,
    /// True when the behavior traces to the shared portable registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when a durable class actually wrote to a hidden machine-global path (a spill).
    pub hidden_machine_global_write_used: bool,
    /// True when hidden machine-global durable writes are explicitly blocked, proving absence.
    pub hidden_machine_global_write_blocked: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe portable-state-layout projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPortableStateLayoutEntry {
    /// Stable identity of the layout entry.
    pub entry_id: String,
    /// The stable portable-profile ID this layout binds to.
    pub profile_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve state isolation and ownership under coexistence.
    pub semantic_role_preserves_state_isolation_and_ownership_under_coexistence: bool,
    /// The containment token named by the entry.
    pub containment: String,
    /// Whether the containment keeps state colocated with, or in a named sibling of, the executable.
    pub containment_is_colocated_or_sibling: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The published executable root.
    pub executable_root: String,
    /// The published colocated / named-sibling writable state roots.
    pub colocated_state_root: String,
    /// The published log / crash-artifact root.
    pub log_and_crash_root: String,
    /// The durable-state-class tokens inventoried by the entry.
    pub durable_classes_covered: Vec<String>,
    /// The state-origin token named by the entry.
    pub state_origin: String,
    /// Whether the state origin is distinguishable from ordinary installed state.
    pub state_origin_is_distinguishable: bool,
    /// The presentation-form tokens covered by the entry.
    pub presentation_form_coverage: Vec<String>,
    /// Whether the entry covers all three presentation forms.
    pub covers_all_presentation_forms: bool,
    /// Whether the durable-root inventory publishes every required root and mandatory durable class.
    pub durable_root_inventory_complete: bool,
    /// Whether the layout keeps every durable class contained with no hidden machine-global spill.
    pub layout_is_contained: bool,
    /// Whether the behavior traces to the shared portable registry.
    pub bound_to_registry: bool,
    /// Whether a durable class actually wrote to a hidden machine-global path.
    pub hidden_machine_global_write_used: bool,
    /// Whether hidden machine-global durable writes are explicitly blocked.
    pub hidden_machine_global_write_blocked: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5PortableStateLayoutEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5PortableNextAction,
    /// Whether the layout resolves cleanly across every claimed profile (clean entry naming every fact).
    pub layout_resolves_across_profiles: bool,
}

impl M5ResolvedPortableStateLayoutEntry {
    /// Whether this layout entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_portable_diagnostics_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PortableDiagnosticsEntryResolutionInput {
    /// Stable identity of the diagnostics entry.
    pub entry_id: String,
    /// The stable portable-profile ID this diagnostics record binds to; empty means unstated.
    pub profile_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5InstallTopologyRole,
    /// The diagnostics surface this entry resolves.
    pub diagnostics_surface: M5PortableDiagnosticsSurface,
    /// The render / surface context.
    pub surface_context: M5PortableSurfaceContext,
    /// The presentation forms this entry holds across (must cover canonical / accessible / audit).
    pub presentation_form_coverage: Vec<M5PortablePresentationForm>,
    /// The published executable root (a filesystem path); empty means unstated.
    pub executable_root: String,
    /// The published writable state roots (filesystem paths); empty means unstated.
    pub state_roots: String,
    /// The diagnostics fields disclosed by this record (must cover every mandatory field).
    pub disclosed_fields: Vec<M5PortableDiagnosticsField>,
    /// The update posture this record resolves.
    pub update_posture: M5PortableUpdatePosture,
    /// True when the retained-versus-replaced continuity note is documented.
    pub update_continuity_documented: bool,
    /// True when the explicitly unsupported shell-integration paths are disclosed.
    pub unsupported_shell_paths_disclosed: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe portable-diagnostics projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPortableDiagnosticsEntry {
    /// Stable identity of the diagnostics entry.
    pub entry_id: String,
    /// The stable portable-profile ID this diagnostics record binds to.
    pub profile_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve state isolation and ownership under coexistence.
    pub semantic_role_preserves_state_isolation_and_ownership_under_coexistence: bool,
    /// The diagnostics-surface token named by the entry.
    pub diagnostics_surface: String,
    /// Whether the diagnostics surface is classified into the resolved taxonomy.
    pub diagnostics_surface_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The presentation-form tokens covered by the entry.
    pub presentation_form_coverage: Vec<String>,
    /// Whether the entry covers all three presentation forms.
    pub covers_all_presentation_forms: bool,
    /// The published executable root.
    pub executable_root: String,
    /// The published writable state roots.
    pub state_roots: String,
    /// The diagnostics-field tokens disclosed by the entry.
    pub disclosed_fields: Vec<String>,
    /// The update-posture token named by the entry.
    pub update_posture: String,
    /// Whether the update posture is classified.
    pub update_posture_is_classified: bool,
    /// Whether the retained-versus-replaced continuity note is documented.
    pub update_continuity_documented: bool,
    /// Whether the explicitly unsupported shell-integration paths are disclosed.
    pub unsupported_shell_paths_disclosed: bool,
    /// Whether the diagnostics record discloses every mandatory field and the unsupported shell-integration
    /// paths.
    pub diagnostics_is_discoverable: bool,
    /// Whether the update posture is classified and continuity is documented.
    pub update_is_continuous: bool,
    /// Degrade reason, if the entry could not read as a clean, discoverable state.
    pub degrade_reason: Option<M5PortableDiagnosticsEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5PortableNextAction,
    /// Whether the diagnostics record is discoverable on every claimed profile (clean entry naming every fact).
    pub diagnostics_discoverable_on_every_profile: bool,
}

impl M5ResolvedPortableDiagnosticsEntry {
    /// Whether this diagnostics entry reads as a clean, discoverable state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5PortableResolutionError {
    /// The layout-entry id was empty.
    EmptyLayoutEntryId,
    /// The diagnostics-entry id was empty.
    EmptyDiagnosticsEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5PortableResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyLayoutEntryId => "empty_layout_entry_id",
            Self::EmptyDiagnosticsEntryId => "empty_diagnostics_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5PortableResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 portable-mode state-containment / diagnostics registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5PortableResolutionError {}

fn presentation_form_tokens(forms: &[M5PortablePresentationForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_presentation_forms(forms: &[M5PortablePresentationForm]) -> bool {
    let present: BTreeSet<M5PortablePresentationForm> = forms.iter().copied().collect();
    M5PortablePresentationForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

fn durable_classes_cover_mandatory(classes: &[M5PortableDurableStateClass]) -> bool {
    let present: BTreeSet<M5PortableDurableStateClass> = classes.iter().copied().collect();
    M5PortableDurableStateClass::MANDATORY
        .iter()
        .all(|class| present.contains(class))
}

fn diagnostics_fields_cover_mandatory(fields: &[M5PortableDiagnosticsField]) -> bool {
    let present: BTreeSet<M5PortableDiagnosticsField> = fields.iter().copied().collect();
    M5PortableDiagnosticsField::MANDATORY
        .iter()
        .all(|field| present.contains(field))
}

/// Whether the portable durable-root inventory publishes every required root and inventories every mandatory
/// durable class: the containment must be colocated / named-sibling, the executable / colocated-state /
/// log-and-crash roots must all be stated, and durable settings, secrets, services, and shell hooks must all
/// be inventoried inside a documented portable root.
pub fn portable_durable_root_inventory_is_complete(
    containment: M5PortableStateContainment,
    executable_root: &str,
    colocated_state_root: &str,
    log_and_crash_root: &str,
    durable_classes_covered: &[M5PortableDurableStateClass],
) -> bool {
    containment.is_colocated_or_sibling()
        && !executable_root.trim().is_empty()
        && !colocated_state_root.trim().is_empty()
        && !log_and_crash_root.trim().is_empty()
        && durable_classes_cover_mandatory(durable_classes_covered)
}

/// Whether the portable layout keeps every durable class contained: the containment must be colocated /
/// named-sibling, the durable-root inventory must be complete, no durable class may have written to a hidden
/// machine-global path, and hidden machine-global durable writes must be explicitly blocked (proving absence).
#[allow(clippy::too_many_arguments)]
pub fn portable_layout_is_contained(
    containment: M5PortableStateContainment,
    executable_root: &str,
    colocated_state_root: &str,
    log_and_crash_root: &str,
    durable_classes_covered: &[M5PortableDurableStateClass],
    hidden_machine_global_write_used: bool,
    hidden_machine_global_write_blocked: bool,
) -> bool {
    portable_durable_root_inventory_is_complete(
        containment,
        executable_root,
        colocated_state_root,
        log_and_crash_root,
        durable_classes_covered,
    ) && !hidden_machine_global_write_used
        && hidden_machine_global_write_blocked
}

/// Whether the discoverable portable-mode diagnostics disclose everything: the surface must be classified,
/// every mandatory diagnostics field must be present, and the explicitly unsupported shell-integration paths
/// must be disclosed.
pub fn portable_diagnostics_is_discoverable(
    surface: M5PortableDiagnosticsSurface,
    disclosed_fields: &[M5PortableDiagnosticsField],
    unsupported_shell_paths_disclosed: bool,
) -> bool {
    surface.is_classified()
        && diagnostics_fields_cover_mandatory(disclosed_fields)
        && unsupported_shell_paths_disclosed
}

/// Whether portable-update continuity stays documented: the update posture must be classified and the
/// retained-versus-replaced continuity note must be documented.
pub fn portable_update_is_continuous(
    posture: M5PortableUpdatePosture,
    update_continuity_documented: bool,
) -> bool {
    posture.is_classified() && update_continuity_documented
}

/// Resolves a portable-state-layout entry so it stays bound to the shared portable registry: the entry names
/// its canonical token, semantic role, and containment, covers all three presentation forms, inventories every
/// durable root, keeps a distinguishable state origin, and proves no durable state spilled into a hidden
/// machine-global path.
pub fn resolve_portable_state_layout_entry(
    input: M5PortableStateLayoutEntryResolutionInput,
) -> Result<M5ResolvedPortableStateLayoutEntry, M5PortableResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5PortableResolutionError::EmptyLayoutEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.profile_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.executable_root)
        || string_is_forbidden(&input.colocated_state_root)
        || string_is_forbidden(&input.log_and_crash_root)
    {
        return Err(M5PortableResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_presentation_forms(&input.presentation_form_coverage);
    let inventory_complete = portable_durable_root_inventory_is_complete(
        input.containment,
        &input.executable_root,
        &input.colocated_state_root,
        &input.log_and_crash_root,
        &input.durable_classes_covered,
    );
    let is_contained = portable_layout_is_contained(
        input.containment,
        &input.executable_root,
        &input.colocated_state_root,
        &input.log_and_crash_root,
        &input.durable_classes_covered,
        input.hidden_machine_global_write_used,
        input.hidden_machine_global_write_blocked,
    );
    let spill_detected =
        input.hidden_machine_global_write_used || !input.hidden_machine_global_write_blocked;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5PortableStateLayoutEntryDegradeReason::LayoutTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5PortableStateLayoutEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.containment.is_colocated_or_sibling() {
        Some(M5PortableStateLayoutEntryDegradeReason::ContainmentUnclassified)
    } else if !input.bound_to_registry {
        Some(M5PortableStateLayoutEntryDegradeReason::LayoutNotBoundToRegistry)
    } else if !inventory_complete {
        Some(M5PortableStateLayoutEntryDegradeReason::DurableRootInventoryIncomplete)
    } else if spill_detected {
        Some(M5PortableStateLayoutEntryDegradeReason::HiddenMachineGlobalDurableSpill)
    } else if !input.state_origin.is_distinguishable() {
        Some(M5PortableStateLayoutEntryDegradeReason::StateOriginAmbiguous)
    } else if !all_forms {
        Some(M5PortableStateLayoutEntryDegradeReason::PresentationFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5PortableStateLayoutEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5PortableNextAction::ExpandPortableMeaning,
    };

    Ok(M5ResolvedPortableStateLayoutEntry {
        entry_id: input.entry_id,
        profile_id: input.profile_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_state_isolation_and_ownership_under_coexistence: input
            .semantic_role
            .must_preserve_state_isolation_and_ownership_under_coexistence(),
        containment: input.containment.as_str().to_owned(),
        containment_is_colocated_or_sibling: input.containment.is_colocated_or_sibling(),
        surface_context: input.surface_context.as_str().to_owned(),
        executable_root: input.executable_root,
        colocated_state_root: input.colocated_state_root,
        log_and_crash_root: input.log_and_crash_root,
        durable_classes_covered: input
            .durable_classes_covered
            .iter()
            .map(|c| c.as_str().to_owned())
            .collect(),
        state_origin: input.state_origin.as_str().to_owned(),
        state_origin_is_distinguishable: input.state_origin.is_distinguishable(),
        presentation_form_coverage: presentation_form_tokens(&input.presentation_form_coverage),
        covers_all_presentation_forms: all_forms,
        durable_root_inventory_complete: inventory_complete,
        layout_is_contained: is_contained,
        bound_to_registry: input.bound_to_registry,
        hidden_machine_global_write_used: input.hidden_machine_global_write_used,
        hidden_machine_global_write_blocked: input.hidden_machine_global_write_blocked,
        degrade_reason,
        next_action,
        layout_resolves_across_profiles: degrade_reason.is_none(),
    })
}

/// Resolves a portable-diagnostics entry so its diagnostics stay discoverable and its update continuity stays
/// documented: the entry names its canonical token, semantic role, and diagnostics surface, covers all three
/// presentation forms, discloses every mandatory diagnostics field, and documents its retained-versus-replaced
/// update continuity.
pub fn resolve_portable_diagnostics_entry(
    input: M5PortableDiagnosticsEntryResolutionInput,
) -> Result<M5ResolvedPortableDiagnosticsEntry, M5PortableResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5PortableResolutionError::EmptyDiagnosticsEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.profile_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.executable_root)
        || string_is_forbidden(&input.state_roots)
    {
        return Err(M5PortableResolutionError::ForbiddenMaterial);
    }

    let all_forms = covers_all_presentation_forms(&input.presentation_form_coverage);
    let is_discoverable = portable_diagnostics_is_discoverable(
        input.diagnostics_surface,
        &input.disclosed_fields,
        input.unsupported_shell_paths_disclosed,
    ) && !input.executable_root.trim().is_empty()
        && !input.state_roots.trim().is_empty();
    let is_continuous =
        portable_update_is_continuous(input.update_posture, input.update_continuity_documented);

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5PortableDiagnosticsEntryDegradeReason::DiagnosticsTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5PortableDiagnosticsEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.diagnostics_surface.is_classified() {
        Some(M5PortableDiagnosticsEntryDegradeReason::DiagnosticsSurfaceUnclassified)
    } else if !is_discoverable {
        Some(M5PortableDiagnosticsEntryDegradeReason::DiagnosticsDisclosureIncomplete)
    } else if !is_continuous {
        Some(M5PortableDiagnosticsEntryDegradeReason::UpdateContinuityUndocumented)
    } else if !all_forms {
        Some(M5PortableDiagnosticsEntryDegradeReason::DiagnosticsFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5PortableDiagnosticsEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5PortableNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedPortableDiagnosticsEntry {
        entry_id: input.entry_id,
        profile_id: input.profile_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_state_isolation_and_ownership_under_coexistence: input
            .semantic_role
            .must_preserve_state_isolation_and_ownership_under_coexistence(),
        diagnostics_surface: input.diagnostics_surface.as_str().to_owned(),
        diagnostics_surface_is_classified: input.diagnostics_surface.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        presentation_form_coverage: presentation_form_tokens(&input.presentation_form_coverage),
        covers_all_presentation_forms: all_forms,
        executable_root: input.executable_root,
        state_roots: input.state_roots,
        disclosed_fields: input
            .disclosed_fields
            .iter()
            .map(|f| f.as_str().to_owned())
            .collect(),
        update_posture: input.update_posture.as_str().to_owned(),
        update_posture_is_classified: input.update_posture.is_classified(),
        update_continuity_documented: input.update_continuity_documented,
        unsupported_shell_paths_disclosed: input.unsupported_shell_paths_disclosed,
        diagnostics_is_discoverable: is_discoverable,
        update_is_continuous: is_continuous,
        degrade_reason,
        next_action,
        diagnostics_discoverable_on_every_profile: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved portable-state-layout and portable-diagnostics
/// entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PortableModeStateContainmentAndDiagnosticsRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5PortableModeConsumerSurface,
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
    pub anatomy_parts: Vec<M5PortableAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5PortableExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5InstallTopologyDowngradeTrigger>,
    /// Resolved portable-state-layout examples.
    pub portable_state_layout_entries: Vec<M5ResolvedPortableStateLayoutEntry>,
    /// Resolved portable-diagnostics examples.
    pub portable_diagnostics_entries: Vec<M5ResolvedPortableDiagnosticsEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the state-root-boundaries domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: portable mode never writes hidden machine-global durable state. MUST be `false`.
    pub portable_mode_writes_hidden_machine_global_durable_state: bool,
    /// Hard invariant: portable state is never indistinguishable from installed state. MUST be `false`.
    pub portable_state_indistinguishable_from_installed_state: bool,
    /// Hard invariant: a portable update never drops retained state without notice. MUST be `false`.
    pub portable_update_drops_retained_state_without_notice: bool,
    /// Hard invariant: an unsupported shell-integration path is never left undisclosed. MUST be `false`.
    pub unsupported_shell_integration_path_left_undisclosed: bool,
}

impl M5PortableModeStateContainmentAndDiagnosticsRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5PortableAnatomyPart> = self.anatomy_parts.iter().copied().collect();
        M5PortableAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5PortableExportField> = self.export_fields.iter().copied().collect();
        M5PortableExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.portable_mode_writes_hidden_machine_global_durable_state
            && !self.portable_state_indistinguishable_from_installed_state
            && !self.portable_update_drops_retained_state_without_notice
            && !self.unsupported_shell_integration_path_left_undisclosed
    }

    /// True when a clean layout entry preserves registry-bound truth: it traces to the registry, keeps a
    /// colocated / sibling containment, inventories every durable root, stays contained (no hidden spill),
    /// keeps a distinguishable origin, and covers all three presentation forms.
    fn layout_is_honest(ex: &M5ResolvedPortableStateLayoutEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && ex.containment_is_colocated_or_sibling
                && ex.durable_root_inventory_complete
                && ex.layout_is_contained
                && ex.state_origin_is_distinguishable
                && ex.covers_all_presentation_forms)
    }

    /// True when a clean diagnostics entry preserves discoverable truth: it keeps a classified surface,
    /// discloses everything, documents update continuity, and covers all three presentation forms.
    fn diagnostics_is_honest(ex: &M5ResolvedPortableDiagnosticsEntry) -> bool {
        !ex.is_clean()
            || (ex.diagnostics_surface_is_classified
                && ex.diagnostics_is_discoverable
                && ex.update_is_continuous
                && ex.covers_all_presentation_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.portable_state_layout_entries
            .iter()
            .all(Self::layout_is_honest)
            && self
                .portable_diagnostics_entries
                .iter()
                .all(Self::diagnostics_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PortableModeStateContainmentAndDiagnosticsVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Presentation-form tokens (minted by this lane).
    pub presentation_forms: Vec<String>,
    /// Containment tokens (minted by this lane).
    pub containments: Vec<String>,
    /// Durable-state-class tokens (minted by this lane).
    pub durable_state_classes: Vec<String>,
    /// State-origin tokens (minted by this lane).
    pub state_origins: Vec<String>,
    /// Diagnostics-surface tokens (minted by this lane).
    pub diagnostics_surfaces: Vec<String>,
    /// Diagnostics-field tokens (minted by this lane).
    pub diagnostics_fields: Vec<String>,
    /// Update-posture tokens (minted by this lane).
    pub update_postures: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Layout-entry degrade-reason tokens.
    pub layout_degrade_reasons: Vec<String>,
    /// Diagnostics-entry degrade-reason tokens.
    pub diagnostics_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5PortableModeStateContainmentAndDiagnosticsVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5InstallTopologyRole::ALL, |v| v.as_str()),
            presentation_forms: tokens(&M5PortablePresentationForm::ALL, |v| v.as_str()),
            containments: tokens(&M5PortableStateContainment::ALL, |v| v.as_str()),
            durable_state_classes: tokens(&M5PortableDurableStateClass::ALL, |v| v.as_str()),
            state_origins: tokens(&M5PortableStateOrigin::ALL, |v| v.as_str()),
            diagnostics_surfaces: tokens(&M5PortableDiagnosticsSurface::ALL, |v| v.as_str()),
            diagnostics_fields: tokens(&M5PortableDiagnosticsField::ALL, |v| v.as_str()),
            update_postures: tokens(&M5PortableUpdatePosture::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5PortableSurfaceContext::ALL, |v| v.as_str()),
            layout_degrade_reasons: tokens(&M5PortableStateLayoutEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            diagnostics_degrade_reasons: tokens(
                &M5PortableDiagnosticsEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5PortableAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5PortableNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5PortableExportField::ALL, |v| v.as_str()),
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
pub struct M5PortableModeStateContainmentAndDiagnosticsGovernanceReview {
    /// The portable registry names a canonical token, semantic role, and containment for every entry.
    pub portable_registry_names_token_role_and_containment: bool,
    /// Every claimed portable profile resolves to a colocated or explicitly named sibling-state layout.
    pub profile_resolves_to_colocated_or_named_sibling_layout: bool,
    /// Every durable root (settings, secrets, services, shell hooks) is identified and inventoried.
    pub all_durable_roots_identified_and_inventoried: bool,
    /// Hidden machine-global durable mutation is absent or explicitly blocked on every profile.
    pub hidden_machine_global_mutation_absent_or_blocked: bool,
    /// Portable state is distinguishable from ordinary installed state without guessing.
    pub portable_state_distinguishable_from_installed_state: bool,
    /// Portable-mode diagnostics are discoverable across the diagnostics, support, and docs surfaces.
    pub portable_diagnostics_discoverable_across_surfaces: bool,
    /// Every layout and diagnostics entry covers the canonical / accessible / audit presentation forms.
    pub every_entry_covers_all_presentation_forms: bool,
    /// Retained-versus-replaced update continuity is documented under manual or tightly-controlled updates.
    pub update_continuity_documented_for_retained_versus_replaced_state: bool,
    /// Portable behavior stays bound to the shared registries rather than hand-copied per profile.
    pub behavior_bound_to_registry_not_hand_copied: bool,
    /// About, update, diagnostics, admin, docs, and support read a single portable-mode source.
    pub about_update_diagnostics_admin_read_single_source: bool,
    /// A hidden machine-global spill, an ambiguous origin, or an undisclosed diagnostics field is caught by
    /// fixtures before release evidence turns green.
    pub portable_spill_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PortableModeStateContainmentAndDiagnosticsConsumerProjection {
    /// About and update consume the shared portable registry.
    pub about_and_update_consume_shared_registries: bool,
    /// Diagnostics and admin consume the shared portable registry.
    pub diagnostics_and_admin_consume_shared_registries: bool,
    /// Installers and the portable launcher consume the shared roots.
    pub installers_and_portable_launcher_consume_shared_registries: bool,
    /// Docs, help, and CLI export consume the shared registries.
    pub docs_help_and_cli_consume_shared_registries: bool,
    /// Behavior traces back to the canonical state-root-boundaries and install-topology contracts.
    pub behavior_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical portable-mode registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PortableModeStateContainmentAndDiagnosticsProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PortableModeStateContainmentAndDiagnosticsReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting portable-diagnostics audit for the lane.
    pub portable_diagnostics_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5PortableModeStateContainmentAndDiagnosticsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PortableModeStateContainmentAndDiagnosticsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5PortableModeStateContainmentAndDiagnosticsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PortableModeStateContainmentAndDiagnosticsVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PortableModeStateContainmentAndDiagnosticsGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PortableModeStateContainmentAndDiagnosticsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PortableModeStateContainmentAndDiagnosticsProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PortableModeStateContainmentAndDiagnosticsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 portable-mode state-containment and diagnostics registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5PortableModeStateContainmentAndDiagnosticsPacket {
    /// Record kind; must equal [`M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5PortableModeStateContainmentAndDiagnosticsRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5PortableModeStateContainmentAndDiagnosticsVocabularySet,
    /// Governance-review block.
    pub governance_review: M5PortableModeStateContainmentAndDiagnosticsGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5PortableModeStateContainmentAndDiagnosticsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5PortableModeStateContainmentAndDiagnosticsProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5PortableModeStateContainmentAndDiagnosticsReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5PortableModeStateContainmentAndDiagnosticsPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5PortableModeStateContainmentAndDiagnosticsPacketInput) -> Self {
        Self {
            record_kind: M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_RECORD_KIND.to_owned(),
            schema_version: M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5PortableModeStateContainmentAndDiagnosticsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_RECORD_KIND {
            violations.push(M5PortableModeStateContainmentAndDiagnosticsViolation::WrongRecordKind);
        }
        if self.schema_version != M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_SCHEMA_VERSION
        {
            violations
                .push(M5PortableModeStateContainmentAndDiagnosticsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5PortableModeStateContainmentAndDiagnosticsViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations
                .push(M5PortableModeStateContainmentAndDiagnosticsViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 portable-mode state-containment / diagnostics packet serializes"),
        ) {
            violations
                .push(M5PortableModeStateContainmentAndDiagnosticsViolation::RawMaterialInExport);
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
            .expect("m5 portable-mode state-containment / diagnostics packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,portable_state_layout_entries,portable_diagnostics_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .portable_state_layout_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.portable_diagnostics_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.portable_state_layout_entries.len(),
                row.portable_diagnostics_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Portable-Mode State-Containment and Diagnostics Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Containments: {}\n",
            self.vocabulary_set.containments.join(", ")
        ));
        out.push_str(&format!(
            "- Presentation forms: {}\n",
            self.vocabulary_set.presentation_forms.join(", ")
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
                "  - Layout entries: {} / diagnostics entries: {}\n",
                row.portable_state_layout_entries.len(),
                row.portable_diagnostics_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-profile portable root-inventory reference table generated from the registry, so docs
    /// and support runbooks render the same containment / executable-root / colocated-state-root /
    /// log-and-crash-root / state-origin truth the resolvers produced rather than a hand-copied path table.
    /// Only clean, registry-bound layout entries are listed.
    pub fn render_portable_root_inventory_table(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "| profile_id | containment | executable_root | colocated_state_root | log_and_crash_root | state_origin |\n",
        );
        out.push_str("| --- | --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.portable_state_layout_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | `{}` | `{}` | {} |\n",
                    ex.profile_id,
                    ex.containment,
                    ex.executable_root,
                    ex.colocated_state_root,
                    ex.log_and_crash_root,
                    ex.state_origin
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5PortableModeStateContainmentAndDiagnosticsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5PortableModeStateContainmentAndDiagnosticsViolation>),
}

impl fmt::Display for M5PortableModeStateContainmentAndDiagnosticsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 portable-mode state-containment / diagnostics export parse failed: {error}"
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
                    "m5 portable-mode state-containment / diagnostics export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5PortableModeStateContainmentAndDiagnosticsArtifactError {}

/// Validation failures emitted by [`M5PortableModeStateContainmentAndDiagnosticsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5PortableModeStateContainmentAndDiagnosticsViolation {
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
    /// A registry row does not point at the state-root-boundaries domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, spilling, inventory-incomplete,
    /// origin-ambiguous, or a diagnostics entry missing a disclosure).
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
    /// Portable-root-inventory is not proven: clean layout entries do not cover the canonical containments or
    /// the first About / update / diagnostics / admin / support surfaces, no inventory-incomplete example
    /// degrades, or a clean layout entry published an incomplete inventory.
    PortableRootInventoryNotProven,
    /// Portable-state-distinguishability is not proven: no origin-ambiguous example degrades, no clean
    /// distinguishable layout entry is present, a clean layout entry is ambiguous, or clean diagnostics entries
    /// do not cover the canonical diagnostics surfaces with full presentation-form coverage while discoverable.
    PortableStateDistinguishabilityNotProven,
    /// Portable-spill-detection is not proven: no hidden-machine-global-spill example degrades, a clean layout
    /// entry spilled, no diagnostics-disclosure-incomplete example degrades, or no update-continuity-
    /// undocumented example degrades.
    PortableSpillDetectionNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5PortableModeStateContainmentAndDiagnosticsViolation {
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
            Self::PortableRootInventoryNotProven => "portable_root_inventory_not_proven",
            Self::PortableStateDistinguishabilityNotProven => {
                "portable_state_distinguishability_not_proven"
            }
            Self::PortableSpillDetectionNotProven => "portable_spill_detection_not_proven",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_portable_mode_state_containment_and_diagnostics_export() -> Result<
    M5PortableModeStateContainmentAndDiagnosticsPacket,
    M5PortableModeStateContainmentAndDiagnosticsArtifactError,
> {
    let packet: M5PortableModeStateContainmentAndDiagnosticsPacket = serde_json::from_str(
        include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-portable-mode-state-containment-and-diagnostics-proof/support_export.json"
        )),
    )
    .map_err(M5PortableModeStateContainmentAndDiagnosticsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5PortableModeStateContainmentAndDiagnosticsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5PortableModeStateContainmentAndDiagnosticsPacket,
    violations: &mut Vec<M5PortableModeStateContainmentAndDiagnosticsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_SCHEMA_REF,
        M5_PORTABLE_MODE_STATE_CONTAINMENT_AND_DIAGNOSTICS_DOC_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_MATRIX_DOC_REF,
        M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF,
        M5_INSTALL_TOPOLOGY_STATE_ROOT_REGISTRIES_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(
                M5PortableModeStateContainmentAndDiagnosticsViolation::MissingSourceContracts,
            );
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5PortableModeStateContainmentAndDiagnosticsPacket,
    violations: &mut Vec<M5PortableModeStateContainmentAndDiagnosticsViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5PortableModeStateContainmentAndDiagnosticsViolation::NoRegistryRows);
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
                .push(M5PortableModeStateContainmentAndDiagnosticsViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(
                M5PortableModeStateContainmentAndDiagnosticsViolation::MandatoryAnatomyMissing,
            );
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(
                M5PortableModeStateContainmentAndDiagnosticsViolation::MandatoryExportFieldMissing,
            );
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_STATE_ROOT_BOUNDARIES_SCHEMA_REF) {
            violations.push(
                M5PortableModeStateContainmentAndDiagnosticsViolation::DomainSchemaRefMissing,
            );
        }
        if row.portable_state_layout_entries.is_empty()
            || row.portable_diagnostics_entries.is_empty()
        {
            violations.push(M5PortableModeStateContainmentAndDiagnosticsViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations
                .push(M5PortableModeStateContainmentAndDiagnosticsViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations
                .push(M5PortableModeStateContainmentAndDiagnosticsViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5PortableModeStateContainmentAndDiagnosticsPacket,
    violations: &mut Vec<M5PortableModeStateContainmentAndDiagnosticsViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.portable_registry_names_token_role_and_containment,
        review.profile_resolves_to_colocated_or_named_sibling_layout,
        review.all_durable_roots_identified_and_inventoried,
        review.hidden_machine_global_mutation_absent_or_blocked,
        review.portable_state_distinguishable_from_installed_state,
        review.portable_diagnostics_discoverable_across_surfaces,
        review.every_entry_covers_all_presentation_forms,
        review.update_continuity_documented_for_retained_versus_replaced_state,
        review.behavior_bound_to_registry_not_hand_copied,
        review.about_update_diagnostics_admin_read_single_source,
        review.portable_spill_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(
                M5PortableModeStateContainmentAndDiagnosticsViolation::GovernanceReviewIncomplete,
            );
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5PortableModeStateContainmentAndDiagnosticsPacket,
    violations: &mut Vec<M5PortableModeStateContainmentAndDiagnosticsViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.about_and_update_consume_shared_registries,
        projection.diagnostics_and_admin_consume_shared_registries,
        projection.installers_and_portable_launcher_consume_shared_registries,
        projection.docs_help_and_cli_consume_shared_registries,
        projection.behavior_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(
                M5PortableModeStateContainmentAndDiagnosticsViolation::ConsumerProjectionIncomplete,
            );
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5PortableModeStateContainmentAndDiagnosticsPacket,
    violations: &mut Vec<M5PortableModeStateContainmentAndDiagnosticsViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations
            .push(M5PortableModeStateContainmentAndDiagnosticsViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5PortableModeStateContainmentAndDiagnosticsPacket,
    violations: &mut Vec<M5PortableModeStateContainmentAndDiagnosticsViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.portable_diagnostics_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations
            .push(M5PortableModeStateContainmentAndDiagnosticsViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted
/// by governance bools.
fn validate_acceptance_criteria(
    packet: &M5PortableModeStateContainmentAndDiagnosticsPacket,
    violations: &mut Vec<M5PortableModeStateContainmentAndDiagnosticsViolation>,
) {
    let layouts = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.portable_state_layout_entries.iter())
    };
    let diagnostics = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.portable_diagnostics_entries.iter())
    };

    // AC1: portable mode can identify all durable roots. Clean layout entries cover the canonical containments
    // and the first About / update / diagnostics / admin / support surfaces, an inventory-incomplete example
    // degrades, and no clean layout entry published an incomplete inventory.
    let clean_containments: BTreeSet<String> = layouts()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.containment.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = layouts()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let containments_covered = M5PortableStateContainment::CANONICAL_CONTAINMENTS
        .iter()
        .all(|c| clean_containments.contains(c.as_str()));
    let first_surfaces_covered = M5PortableSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let inventory_incomplete_degrades = layouts().any(|ex| {
        ex.degrade_reason
            == Some(M5PortableStateLayoutEntryDegradeReason::DurableRootInventoryIncomplete)
    });
    let no_clean_incomplete =
        !layouts().any(|ex| ex.is_clean() && !ex.durable_root_inventory_complete);
    if !(containments_covered
        && first_surfaces_covered
        && inventory_incomplete_degrades
        && no_clean_incomplete)
    {
        violations.push(
            M5PortableModeStateContainmentAndDiagnosticsViolation::PortableRootInventoryNotProven,
        );
    }

    // AC2: support / export can distinguish portable state from ordinary installed state without guessing. An
    // origin-ambiguous example degrades, at least one clean distinguishable layout entry is present, no clean
    // layout entry is ambiguous, and clean diagnostics entries cover the canonical diagnostics surfaces with
    // full presentation-form coverage while discoverable.
    let origin_ambiguous_degrades = layouts().any(|ex| {
        ex.degrade_reason == Some(M5PortableStateLayoutEntryDegradeReason::StateOriginAmbiguous)
    });
    let distinguishable_clean_layout =
        layouts().any(|ex| ex.is_clean() && ex.state_origin_is_distinguishable);
    let no_clean_ambiguous =
        !layouts().any(|ex| ex.is_clean() && !ex.state_origin_is_distinguishable);
    let clean_diagnostics_surfaces: BTreeSet<String> = diagnostics()
        .filter(|ex| {
            ex.is_clean()
                && ex.diagnostics_surface_is_classified
                && ex.diagnostics_is_discoverable
                && ex.covers_all_presentation_forms
        })
        .map(|ex| ex.diagnostics_surface.clone())
        .collect();
    let diagnostics_surfaces_covered = M5PortableDiagnosticsSurface::CANONICAL_SURFACES
        .iter()
        .all(|s| clean_diagnostics_surfaces.contains(s.as_str()));
    if !(origin_ambiguous_degrades
        && distinguishable_clean_layout
        && no_clean_ambiguous
        && diagnostics_surfaces_covered)
    {
        violations.push(
            M5PortableModeStateContainmentAndDiagnosticsViolation::PortableStateDistinguishabilityNotProven,
        );
    }

    // AC3: the suite fails when durable settings, secrets, or services spill outside documented portable roots
    // or discoverable diagnostics / update continuity is dropped. A hidden-machine-global-spill example
    // degrades, no clean layout entry spilled, a diagnostics-disclosure-incomplete example degrades, and an
    // update-continuity-undocumented example degrades.
    let spill_degrades = layouts().any(|ex| {
        ex.degrade_reason
            == Some(M5PortableStateLayoutEntryDegradeReason::HiddenMachineGlobalDurableSpill)
    });
    let no_clean_spill = !layouts().any(|ex| {
        ex.is_clean()
            && (ex.hidden_machine_global_write_used || !ex.hidden_machine_global_write_blocked)
    });
    let disclosure_incomplete_degrades = diagnostics().any(|ex| {
        ex.degrade_reason
            == Some(M5PortableDiagnosticsEntryDegradeReason::DiagnosticsDisclosureIncomplete)
    });
    let continuity_undocumented_degrades = diagnostics().any(|ex| {
        ex.degrade_reason
            == Some(M5PortableDiagnosticsEntryDegradeReason::UpdateContinuityUndocumented)
    });
    if !(spill_degrades
        && no_clean_spill
        && disclosure_incomplete_degrades
        && continuity_undocumented_degrades)
    {
        violations.push(
            M5PortableModeStateContainmentAndDiagnosticsViolation::PortableSpillDetectionNotProven,
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

/// The install-topology families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5InstallTopologyFamily; 1] =
    [M5InstallTopologyFamily::PortableMode];
