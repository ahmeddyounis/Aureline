//! Implemented M5 file-path-presentation / reveal and native-window / menu registries.
//!
//! The frozen [platform-fit matrix][matrix] names Aureline's six platform-fit families and locks their
//! controlled vocabulary. This module is the implement lane for the file-and-window everyday flows: it turns
//! the concrete *file / path / reveal / open-save terminology* grammar of the file-path-reveal family and the
//! *native window-chrome and menu availability* grammar of the platform-convention family into registry
//! resolvers that produce export-safe, honest projections. A user can then trust that file, folder,
//! workspace, and reveal flows use host-correct terms and separators on every claimed macOS, Windows, and
//! Linux desktop profile, that literal-versus-canonical path truth stays explicit, that no critical action is
//! reachable only through OS chrome or the menu bar, and that a surface showing the wrong path separator,
//! reveal verb, or a menu-only action degrades honestly instead of reading as a clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Adopt host-appropriate file / path separators, drive or mount vocabulary, reveal verbs, and open /
//!   save terminology while keeping literal-versus-canonical path truth explicit.**
//!   [`resolve_file_path_presentation_entry`] refuses to read as a clean, registry-bound path entry unless it
//!   names a canonical registry token, a classified [host platform][M5HostPlatform], a file-path-reveal role,
//!   covers every [presentation form][M5PathPresentationForm] (the host-styled display, the canonical path
//!   truth, and the accessible announcement), renders a path separator and reveal verb that match the host
//!   platform's convention, preserves the canonical path truth, and explains any unavailable-reveal fallback;
//!   otherwise it degrades.
//! * **Ensure high-frequency actions are available from product surfaces and commands rather than hidden only
//!   in OS menus or title-bar affordances.** [`resolve_window_menu_action_entry`] names a classified
//!   [product action surface][M5ProductActionSurface], requires the action to be reachable by stable command
//!   ID, an in-product surface, and a command, covers every presentation form, and degrades to
//!   [`M5WindowMenuActionEntryDegradeReason::ReachableOnlyInOsChrome`] when an action drops any leg of the
//!   reachability triple, so a primary action can never be reachable only through OS chrome.
//! * **Preserve native window-chrome expectations and menu phrasing without letting platform differences
//!   create divergent product meaning or hidden workflows, and generate platform docs / help / screenshots
//!   from the same terminology registry.** [`path_presentation_matches_host`] rejects a Windows entry rendered
//!   with a forward-slash path or a `Reveal in Finder` verb and a macOS entry rendered with a backslash path
//!   or a `Show in Explorer` verb so a mislabeled path or reveal verb degrades to
//!   [`M5FilePathPresentationEntryDegradeReason::PathOrRevealMislabeledForHost`], and
//!   [`M5FilePathRevealRegistriesPacket::render_platform_path_reveal_table`] emits the same path / reveal
//!   truth the resolvers produced.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5PlatformFitRole`] role vocabulary, the
//! [`M5FilePathRevealRole`] file-path-reveal-role vocabulary, and the [`M5PlatformConventionRole`]
//! platform-convention-role vocabulary — so shell, settings, docs, onboarding, CLI, and support surfaces can
//! never fork their own path or window / menu meaning. Raw secret values and private endpoints stay outside
//! the export boundary.
//!
//! [matrix]: crate::m5_platform_fit_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_file_path_reveal_and_native_window_menu_registries,
    seeded_m5_file_path_reveal_and_native_window_menu_registries_docs_help_beta_narrowed,
    seeded_m5_file_path_reveal_and_native_window_menu_registries_reveal_preview_narrowed,
    M5_FILE_PATH_REVEAL_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_platform_fit_matrix::{
    M5FilePathRevealRole, M5PlatformConventionRole, M5PlatformFitAccessibilityRoute,
    M5PlatformFitConsumerSurface, M5PlatformFitDeploymentLine, M5PlatformFitDowngradeTrigger,
    M5PlatformFitFamily, M5PlatformFitQualificationClass, M5PlatformFitRequiredLabel,
    M5PlatformFitRole, M5_FILE_PATH_AND_REVEAL_SCHEMA_REF, M5_PLATFORM_FIT_MATRIX_DOC_REF,
    M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5FilePathRevealRegistriesPacket`].
pub const M5_FILE_PATH_REVEAL_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_file_path_reveal_and_native_window_menu_registries";

/// Schema version for M5 file-path-reveal / window-menu registry records.
pub const M5_FILE_PATH_REVEAL_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_FILE_PATH_REVEAL_REGISTRIES_SCHEMA_REF: &str =
    "schemas/platform/m5-file-path-reveal-and-native-window-menu-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_FILE_PATH_REVEAL_REGISTRIES_DOC_REF: &str =
    "docs/platform/m5_file_path_reveal_and_native_window_menu_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_FILE_PATH_REVEAL_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-file-path-reveal-and-native-window-menu-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_FILE_PATH_REVEAL_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-file-path-reveal-and-native-window-menu-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_FILE_PATH_REVEAL_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-file-path-reveal-and-native-window-menu-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_FILE_PATH_REVEAL_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/platform/m5-file-path-reveal-and-native-window-menu-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5FilePathRevealRegistriesConsumerSurface = M5PlatformFitConsumerSurface;

/// One of the three presentation forms every file-path or window-menu entry must hold across so a path's or
/// action's meaning keeps its truth whether it is shown in the host style, resolved to its canonical form, or
/// announced to a screen reader. Minted by this lane because the frozen matrix names the file-path-reveal
/// *family* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PathPresentationForm {
    /// The host-styled display form (platform-native separators, drive / mount vocabulary, reveal verb).
    HostStyledDisplay,
    /// The literal / canonical path truth kept explicit alongside the host-styled display.
    CanonicalTruth,
    /// The spoken / searchable accessible announcement that keeps the path or action discoverable.
    AccessibleAnnouncement,
}

impl M5PathPresentationForm {
    /// Every presentation form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::HostStyledDisplay,
        Self::CanonicalTruth,
        Self::AccessibleAnnouncement,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostStyledDisplay => "host_styled_display",
            Self::CanonicalTruth => "canonical_truth",
            Self::AccessibleAnnouncement => "accessible_announcement",
        }
    }
}

/// Controlled host platform a path entry adapts to, so the canonical separator and reveal verb share one
/// registry rather than a hand-copied per-platform string. Minted by this lane because the frozen matrix
/// carries the macOS / Windows / Linux surface families but not the concrete separator and reveal convention
/// an entry must match. Every classified platform carries its canonical path separator and reveal verb.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HostPlatform {
    /// The macOS platform (`/` separator, `Reveal in Finder`).
    Macos,
    /// The Windows platform (`\` separator, `Show in Explorer`).
    Windows,
    /// The Linux platform (`/` separator, `Open Containing Folder`).
    Linux,
    /// The host platform is unclassified, which is disallowed.
    PlatformUnknown,
}

impl M5HostPlatform {
    /// Every host platform, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::Macos,
        Self::Windows,
        Self::Linux,
        Self::PlatformUnknown,
    ];

    /// The three canonical desktop platforms every claimed M5 profile resolves path truth from.
    pub const CANONICAL_PLATFORMS: [Self; 3] = [Self::Macos, Self::Windows, Self::Linux];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
            Self::PlatformUnknown => "platform_unknown",
        }
    }

    /// Whether the platform is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::PlatformUnknown)
    }

    /// Whether this platform presents paths with a backslash separator (Windows) rather than a forward slash.
    pub const fn uses_backslash_separator(self) -> bool {
        matches!(self, Self::Windows)
    }

    /// The canonical path separator for this platform.
    pub const fn canonical_path_separator(self) -> &'static str {
        match self {
            Self::Macos | Self::Linux => "/",
            Self::Windows => "\\",
            Self::PlatformUnknown => "",
        }
    }

    /// The canonical reveal-in-shell verb for this platform.
    pub const fn canonical_reveal_verb(self) -> &'static str {
        match self {
            Self::Macos => "Reveal in Finder",
            Self::Windows => "Show in Explorer",
            Self::Linux => "Open Containing Folder",
            Self::PlatformUnknown => "",
        }
    }
}

/// Controlled in-product action surface a window / menu action must also be reachable from, so a
/// high-frequency action is never reachable only through OS chrome. Minted by this lane, tracking the
/// product surfaces the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ProductActionSurface {
    /// The command palette.
    CommandPalette,
    /// The in-product toolbar.
    ProductToolbar,
    /// The in-product command list.
    CommandList,
    /// The product action surface is unclassified, which is disallowed.
    SurfaceUnclassified,
}

impl M5ProductActionSurface {
    /// Every product action surface, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CommandPalette,
        Self::ProductToolbar,
        Self::CommandList,
        Self::SurfaceUnclassified,
    ];

    /// The three canonical product surfaces every high-frequency action must also be reachable from.
    pub const CANONICAL_SURFACES: [Self; 3] = [
        Self::CommandPalette,
        Self::ProductToolbar,
        Self::CommandList,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandPalette => "command_palette",
            Self::ProductToolbar => "product_toolbar",
            Self::CommandList => "command_list",
            Self::SurfaceUnclassified => "surface_unclassified",
        }
    }

    /// Whether the product surface is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::SurfaceUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a path or action
/// token's meaning stays stable whether it appears in a file-open dialog, save dialog, reveal menu, path
/// breadcrumb, or help. Minted by this lane, tracking the first-consumer surfaces the implementation
/// requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FilePathSurfaceContext {
    /// The file-open dialog surface.
    FileOpenDialog,
    /// The save dialog surface.
    SaveDialog,
    /// The reveal-in-shell menu surface.
    RevealMenu,
    /// The path-breadcrumb surface.
    PathBreadcrumb,
    /// The help / docs surface.
    DocsHelp,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5FilePathSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FileOpenDialog,
        Self::SaveDialog,
        Self::RevealMenu,
        Self::PathBreadcrumb,
        Self::DocsHelp,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::FileOpenDialog,
        Self::SaveDialog,
        Self::RevealMenu,
        Self::PathBreadcrumb,
        Self::DocsHelp,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FileOpenDialog => "file_open_dialog",
            Self::SaveDialog => "save_dialog",
            Self::RevealMenu => "reveal_menu",
            Self::PathBreadcrumb => "path_breadcrumb",
            Self::DocsHelp => "docs_help",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a file-path or window-menu entry must be able to show, so no path, reveal
/// verb, action label, or registry fact is left implicit behind a hand-copied per-platform string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FilePathRegistryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The host platform the entry adapts to (path entry).
    HostPlatform,
    /// The rendered host-styled path text (path entry).
    RenderedPath,
    /// The presentation-form coverage (host-styled / canonical / accessible).
    PresentationFormCoverage,
    /// The rendered reveal verb (path entry).
    RevealVerb,
    /// The in-product action label the entry maps (window-menu entry).
    ActionLabel,
    /// The render / surface context (both entries).
    SurfaceContext,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the path or action (both entries).
    PlainLanguageMeaning,
}

impl M5FilePathRegistryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::HostPlatform,
        Self::RenderedPath,
        Self::PresentationFormCoverage,
        Self::RevealVerb,
        Self::ActionLabel,
        Self::SurfaceContext,
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
            Self::HostPlatform => "host_platform",
            Self::RenderedPath => "rendered_path",
            Self::PresentationFormCoverage => "presentation_form_coverage",
            Self::RevealVerb => "reveal_verb",
            Self::ActionLabel => "action_label",
            Self::SurfaceContext => "surface_context",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a path,
/// reveal verb, action mapping, or a degraded file-path / window-menu entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FilePathRegistryNextAction {
    /// Expand the path's or action's plain-language meaning.
    ExpandPathMeaning,
    /// Inspect the host platform or product surface the entry maps.
    InspectPlatformOrSurface,
    /// Complete the host-styled / canonical / accessible presentation-form coverage.
    CompletePresentationFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5FilePathRegistryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandPathMeaning,
        Self::InspectPlatformOrSurface,
        Self::CompletePresentationFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandPathMeaning => "expand_path_meaning",
            Self::InspectPlatformOrSurface => "inspect_platform_or_surface",
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
pub enum M5FilePathRegistryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The platform-fit families covered.
    PlatformFitFamilies,
    /// The host platforms carried.
    HostPlatforms,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The presentation forms covered.
    PresentationForms,
    /// The product action surfaces carried.
    ProductActionSurfaces,
    /// The render / surface context.
    SurfaceContext,
    /// The rendered reveal verbs carried.
    RevealVerbs,
    /// The accountable owner role.
    OwnerRole,
}

impl M5FilePathRegistryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::PlatformFitFamilies,
        Self::HostPlatforms,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::PresentationForms,
        Self::ProductActionSurfaces,
        Self::SurfaceContext,
        Self::RevealVerbs,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::PlatformFitFamilies,
        Self::HostPlatforms,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::PlatformFitFamilies => "platform_fit_families",
            Self::HostPlatforms => "host_platforms",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::PresentationForms => "presentation_forms",
            Self::ProductActionSurfaces => "product_action_surfaces",
            Self::SurfaceContext => "surface_context",
            Self::RevealVerbs => "reveal_verbs",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a file-path-presentation entry degraded below a clean, registry-bound state. The degrade-first
/// ladder returns one of these instead of ever letting a hand-copied, mislabeled, canonical-truth-losing, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FilePathPresentationEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the path means.
    PathTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The host platform is unclassified (not in the preserved taxonomy).
    HostPlatformUnclassified,
    /// The terminology is a hand-copied per-platform string instead of tracing to the canonical registry.
    TerminologyNotBoundToRegistry,
    /// The rendered path separator or reveal verb does not match the host platform's convention.
    PathOrRevealMislabeledForHost,
    /// The entry does not keep the literal / canonical path truth explicit alongside the host-styled display.
    CanonicalPathTruthNotPreserved,
    /// The host-styled / canonical / accessible presentation-form coverage is incomplete.
    PresentationFormCoverageIncomplete,
    /// Reveal-in-shell is unavailable on this surface and no fallback vocabulary is explained.
    RevealUnavailableWithoutFallback,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5FilePathPresentationEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::PathTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::HostPlatformUnclassified,
        Self::TerminologyNotBoundToRegistry,
        Self::PathOrRevealMislabeledForHost,
        Self::CanonicalPathTruthNotPreserved,
        Self::PresentationFormCoverageIncomplete,
        Self::RevealUnavailableWithoutFallback,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PathTokenUnstated => "path_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::HostPlatformUnclassified => "host_platform_unclassified",
            Self::TerminologyNotBoundToRegistry => "terminology_not_bound_to_registry",
            Self::PathOrRevealMislabeledForHost => "path_or_reveal_mislabeled_for_host",
            Self::CanonicalPathTruthNotPreserved => "canonical_path_truth_not_preserved",
            Self::PresentationFormCoverageIncomplete => "presentation_form_coverage_incomplete",
            Self::RevealUnavailableWithoutFallback => "reveal_unavailable_without_fallback",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5FilePathRegistryNextAction {
        match self {
            Self::PathTokenUnstated | Self::TerminologyNotBoundToRegistry => {
                M5FilePathRegistryNextAction::TraceCanonicalRegistry
            }
            Self::HostPlatformUnclassified
            | Self::PathOrRevealMislabeledForHost
            | Self::CanonicalPathTruthNotPreserved => {
                M5FilePathRegistryNextAction::InspectPlatformOrSurface
            }
            Self::PresentationFormCoverageIncomplete => {
                M5FilePathRegistryNextAction::CompletePresentationFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::RevealUnavailableWithoutFallback
            | Self::ProofStale => M5FilePathRegistryNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5PlatformFitDowngradeTrigger {
        match self {
            Self::PathTokenUnstated | Self::PresentationFormCoverageIncomplete => {
                M5PlatformFitDowngradeTrigger::PathVerbUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5PlatformFitDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::HostPlatformUnclassified => M5PlatformFitDowngradeTrigger::HostPlatformUnstated,
            Self::TerminologyNotBoundToRegistry => {
                M5PlatformFitDowngradeTrigger::ShortcutNotationDriftedByPlatform
            }
            Self::PathOrRevealMislabeledForHost => {
                M5PlatformFitDowngradeTrigger::ScreenshotOrDocsMislabeledShortcutOrPathVerb
            }
            Self::CanonicalPathTruthNotPreserved => {
                M5PlatformFitDowngradeTrigger::PlatformWordingChangedCommandOrPermissionMeaning
            }
            Self::RevealUnavailableWithoutFallback => {
                M5PlatformFitDowngradeTrigger::PrimaryActionHiddenOnlyInOsChrome
            }
            Self::ProofStale => M5PlatformFitDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a window / menu action entry degraded below a clean, reachable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5WindowMenuActionEntryDegradeReason {
    /// The canonical registry token name is unstated.
    ActionTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The product action surface is unclassified (not in the preserved taxonomy).
    ActionSurfaceUnclassified,
    /// The action is reachable only through OS chrome — not by stable ID, an in-product surface, and command.
    ReachableOnlyInOsChrome,
    /// The host-styled / canonical / accessible presentation-form coverage of the window / menu phrasing is
    /// incomplete.
    WindowMenuPhrasingCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5WindowMenuActionEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ActionTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::ActionSurfaceUnclassified,
        Self::ReachableOnlyInOsChrome,
        Self::WindowMenuPhrasingCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ActionTokenUnstated => "action_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::ActionSurfaceUnclassified => "action_surface_unclassified",
            Self::ReachableOnlyInOsChrome => "reachable_only_in_os_chrome",
            Self::WindowMenuPhrasingCoverageIncomplete => {
                "window_menu_phrasing_coverage_incomplete"
            }
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5FilePathRegistryNextAction {
        match self {
            Self::ActionTokenUnstated => M5FilePathRegistryNextAction::TraceCanonicalRegistry,
            Self::ActionSurfaceUnclassified | Self::ReachableOnlyInOsChrome => {
                M5FilePathRegistryNextAction::InspectPlatformOrSurface
            }
            Self::WindowMenuPhrasingCoverageIncomplete => {
                M5FilePathRegistryNextAction::CompletePresentationFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5FilePathRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5PlatformFitDowngradeTrigger {
        match self {
            Self::ActionTokenUnstated => M5PlatformFitDowngradeTrigger::RegistryReferenceUnstated,
            Self::SurfaceContextUnresolved | Self::ActionSurfaceUnclassified => {
                M5PlatformFitDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::ReachableOnlyInOsChrome => {
                M5PlatformFitDowngradeTrigger::PrimaryActionHiddenOnlyInOsChrome
            }
            Self::WindowMenuPhrasingCoverageIncomplete => {
                M5PlatformFitDowngradeTrigger::PathVerbUnstated
            }
            Self::ProofStale => M5PlatformFitDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_file_path_presentation_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5FilePathPresentationEntryResolutionInput {
    /// Stable identity of the file-path-presentation-registry entry.
    pub entry_id: String,
    /// The stable command ID this path binds to (e.g. `command.file.reveal`); empty means unstated.
    pub command_id: String,
    /// The canonical registry token name (e.g. `path.reveal.macos`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5PlatformFitRole,
    /// The file-path-reveal role (from the frozen matrix vocabulary).
    pub path_role: M5FilePathRevealRole,
    /// The host platform this entry adapts to.
    pub host_platform: M5HostPlatform,
    /// The render / surface context.
    pub surface_context: M5FilePathSurfaceContext,
    /// The presentation forms this entry holds across (must cover host-styled / canonical / accessible).
    pub presentation_form_coverage: Vec<M5PathPresentationForm>,
    /// The rendered host-styled display path (e.g. `/Users/ana/Documents` or `C:\Users\ana\Documents`).
    pub rendered_path: String,
    /// The rendered reveal-in-shell verb (e.g. `Reveal in Finder` or `Show in Explorer`).
    pub reveal_verb: String,
    /// True when the terminology traces to the shared path registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the entry keeps the literal / canonical path truth explicit (a hard invariant when `false`).
    pub preserves_canonical_path_truth: bool,
    /// True when reveal-in-shell is unavailable on this surface (e.g. remote / web session).
    pub reveal_target_unavailable: bool,
    /// True when an explicit fallback vocabulary is explained for an unavailable reveal target.
    pub fallback_explained: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe file-path-presentation-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedFilePathPresentationEntry {
    /// Stable identity of the file-path-presentation-registry entry.
    pub entry_id: String,
    /// The stable command ID this path binds to.
    pub command_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve command identity as platform labels and notation adapt.
    pub semantic_role_preserves_command_identity_under_platform_adaptation: bool,
    /// The file-path-reveal-role token named by the entry.
    pub path_role: String,
    /// Whether the path role names the disallowed mislabeled-path-verb token.
    pub path_role_mislabeled: bool,
    /// The host-platform token named by the entry.
    pub host_platform: String,
    /// Whether the host platform is classified into the preserved taxonomy.
    pub host_platform_is_classified: bool,
    /// Whether the host platform presents paths with a backslash separator rather than a forward slash.
    pub host_uses_backslash_separator: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The rendered host-styled display path.
    pub rendered_path: String,
    /// The rendered reveal verb.
    pub reveal_verb: String,
    /// The canonical reveal verb for the entry's host platform.
    pub canonical_reveal_verb: String,
    /// The presentation-form tokens covered by the entry.
    pub presentation_form_coverage: Vec<String>,
    /// Whether the entry covers all three presentation forms.
    pub covers_all_presentation_forms: bool,
    /// Whether the rendered separator and reveal verb match the host platform's convention.
    pub path_matches_host: bool,
    /// Whether the entry traces to the shared path registry.
    pub bound_to_registry: bool,
    /// Whether the entry keeps the literal / canonical path truth explicit.
    pub preserves_canonical_path_truth: bool,
    /// Whether reveal-in-shell is unavailable on this surface.
    pub reveal_target_unavailable: bool,
    /// Whether an explicit fallback vocabulary is explained for an unavailable reveal target.
    pub fallback_explained: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5FilePathPresentationEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5FilePathRegistryNextAction,
    /// Whether the path truth holds across every presentation form and platform (clean entry naming every
    /// fact).
    pub path_truth_holds_across_surfaces_and_platforms: bool,
}

impl M5ResolvedFilePathPresentationEntry {
    /// Whether this file-path-presentation entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_window_menu_action_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5WindowMenuActionEntryResolutionInput {
    /// Stable identity of the window / menu action entry.
    pub entry_id: String,
    /// The stable command ID this action binds to; empty means unstated.
    pub command_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The platform-convention role this action carries (from the frozen matrix vocabulary).
    pub action_role: M5PlatformConventionRole,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5PlatformFitRole,
    /// The in-product surface this action must also be reachable from.
    pub action_surface: M5ProductActionSurface,
    /// The render / surface context.
    pub surface_context: M5FilePathSurfaceContext,
    /// The presentation forms this entry holds across (must cover host-styled / canonical / accessible).
    pub presentation_form_coverage: Vec<M5PathPresentationForm>,
    /// The human-readable action label (e.g. `Reveal in Finder`); empty means missing.
    pub human_label: String,
    /// The in-product command route the action is reachable through (e.g. `command.file.reveal`); empty means
    /// missing.
    pub in_product_route: String,
    /// True when the action is reachable by stable ID, an in-product surface, and a command (never only OS
    /// chrome).
    pub reachable_by_id_surface_and_command: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe window / menu action projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedWindowMenuActionEntry {
    /// Stable identity of the window / menu action entry.
    pub entry_id: String,
    /// The stable command ID this action binds to.
    pub command_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The platform-convention-role token named by the entry.
    pub action_role: String,
    /// Whether the action role names the disallowed invented-private-convention token.
    pub action_role_invented: bool,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// The product-action-surface token named by the entry.
    pub action_surface: String,
    /// Whether the product action surface is classified into the preserved taxonomy.
    pub action_surface_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The presentation-form tokens covered by the entry.
    pub presentation_form_coverage: Vec<String>,
    /// Whether the entry covers all three presentation forms.
    pub covers_all_presentation_forms: bool,
    /// The human-readable action label named by the entry.
    pub human_label: String,
    /// The in-product command route named by the entry.
    pub in_product_route: String,
    /// Whether the action is reachable by stable ID, an in-product surface, and a command.
    pub reachable_by_id_surface_and_command: bool,
    /// Whether the entry provides the complete stable-ID / in-product-surface / command reachability triple.
    pub provides_complete_reachability_triple: bool,
    /// Degrade reason, if the entry could not read as a clean, reachable state.
    pub degrade_reason: Option<M5WindowMenuActionEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5FilePathRegistryNextAction,
    /// Whether the action is reachable on every claimed desktop profile (clean entry naming every fact).
    pub action_reachable_on_every_profile: bool,
}

impl M5ResolvedWindowMenuActionEntry {
    /// Whether this window / menu action entry reads as a clean, reachable state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5FilePathRevealResolutionError {
    /// The file-path-presentation-entry id was empty.
    EmptyFilePathPresentationEntryId,
    /// The window-menu-action-entry id was empty.
    EmptyWindowMenuActionEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5FilePathRevealResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyFilePathPresentationEntryId => "empty_file_path_presentation_entry_id",
            Self::EmptyWindowMenuActionEntryId => "empty_window_menu_action_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5FilePathRevealResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 file-path-reveal / window-menu registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5FilePathRevealResolutionError {}

fn form_tokens(forms: &[M5PathPresentationForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_presentation_forms(forms: &[M5PathPresentationForm]) -> bool {
    let present: BTreeSet<M5PathPresentationForm> = forms.iter().copied().collect();
    M5PathPresentationForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether the rendered path separator and reveal verb match the host platform's convention: a Windows entry
/// must render a backslash path and the `Show in Explorer` verb and never a forward-slash path or a
/// `Reveal in Finder` verb, and a macOS or Linux entry must render a forward-slash path and its own reveal
/// verb and never a backslash path. An unclassified, empty, or verb-mismatched entry never matches.
pub fn path_presentation_matches_host(
    host: M5HostPlatform,
    rendered_path: &str,
    reveal_verb: &str,
) -> bool {
    if !host.is_classified() || rendered_path.trim().is_empty() || reveal_verb.trim().is_empty() {
        return false;
    }
    let verb_matches = reveal_verb
        .trim()
        .eq_ignore_ascii_case(host.canonical_reveal_verb());
    let separator_matches = match host {
        M5HostPlatform::Macos | M5HostPlatform::Linux => {
            rendered_path.contains('/') && !rendered_path.contains('\\')
        }
        M5HostPlatform::Windows => rendered_path.contains('\\') && !rendered_path.contains('/'),
        M5HostPlatform::PlatformUnknown => false,
    };
    verb_matches && separator_matches
}

/// Resolves a file-path-presentation-registry entry so it stays bound to the shared path registry: the entry
/// names its canonical token, semantic role, path role, and host platform, covers all three presentation
/// forms, renders a separator and reveal verb that match the host convention, keeps the canonical path truth
/// explicit, and explains any unavailable-reveal fallback.
pub fn resolve_file_path_presentation_entry(
    input: M5FilePathPresentationEntryResolutionInput,
) -> Result<M5ResolvedFilePathPresentationEntry, M5FilePathRevealResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5FilePathRevealResolutionError::EmptyFilePathPresentationEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.command_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.rendered_path)
        || string_is_forbidden(&input.reveal_verb)
    {
        return Err(M5FilePathRevealResolutionError::ForbiddenMaterial);
    }

    let path_role_mislabeled = matches!(
        input.path_role,
        M5FilePathRevealRole::MislabeledPathVerbDisallowed
    );
    let all_forms = covers_all_presentation_forms(&input.presentation_form_coverage);
    let matches_host = path_presentation_matches_host(
        input.host_platform,
        &input.rendered_path,
        &input.reveal_verb,
    );
    let reveal_unhandled = input.reveal_target_unavailable && !input.fallback_explained;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5FilePathPresentationEntryDegradeReason::PathTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5FilePathPresentationEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.host_platform.is_classified() {
        Some(M5FilePathPresentationEntryDegradeReason::HostPlatformUnclassified)
    } else if path_role_mislabeled || !input.bound_to_registry {
        Some(M5FilePathPresentationEntryDegradeReason::TerminologyNotBoundToRegistry)
    } else if !matches_host {
        Some(M5FilePathPresentationEntryDegradeReason::PathOrRevealMislabeledForHost)
    } else if !input.preserves_canonical_path_truth {
        Some(M5FilePathPresentationEntryDegradeReason::CanonicalPathTruthNotPreserved)
    } else if !all_forms {
        Some(M5FilePathPresentationEntryDegradeReason::PresentationFormCoverageIncomplete)
    } else if reveal_unhandled {
        Some(M5FilePathPresentationEntryDegradeReason::RevealUnavailableWithoutFallback)
    } else if !input.proof_fresh {
        Some(M5FilePathPresentationEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5FilePathRegistryNextAction::ExpandPathMeaning,
    };

    Ok(M5ResolvedFilePathPresentationEntry {
        entry_id: input.entry_id,
        command_id: input.command_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_command_identity_under_platform_adaptation: input
            .semantic_role
            .must_preserve_command_identity_under_platform_adaptation(),
        path_role: input.path_role.as_str().to_owned(),
        path_role_mislabeled,
        host_platform: input.host_platform.as_str().to_owned(),
        host_platform_is_classified: input.host_platform.is_classified(),
        host_uses_backslash_separator: input.host_platform.uses_backslash_separator(),
        surface_context: input.surface_context.as_str().to_owned(),
        rendered_path: input.rendered_path,
        reveal_verb: input.reveal_verb,
        canonical_reveal_verb: input.host_platform.canonical_reveal_verb().to_owned(),
        presentation_form_coverage: form_tokens(&input.presentation_form_coverage),
        covers_all_presentation_forms: all_forms,
        path_matches_host: matches_host,
        bound_to_registry: input.bound_to_registry,
        preserves_canonical_path_truth: input.preserves_canonical_path_truth,
        reveal_target_unavailable: input.reveal_target_unavailable,
        fallback_explained: input.fallback_explained,
        degrade_reason,
        next_action,
        path_truth_holds_across_surfaces_and_platforms: degrade_reason.is_none(),
    })
}

/// Resolves a window / menu action entry so a high-frequency action stays reachable: the entry names its
/// canonical token, action role, semantic role, and product surface, covers all three presentation forms,
/// provides the stable-ID / in-product-surface / command reachability triple, and degrades honestly when the
/// action would be reachable only through OS chrome.
pub fn resolve_window_menu_action_entry(
    input: M5WindowMenuActionEntryResolutionInput,
) -> Result<M5ResolvedWindowMenuActionEntry, M5FilePathRevealResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5FilePathRevealResolutionError::EmptyWindowMenuActionEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.command_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.human_label)
        || string_is_forbidden(&input.in_product_route)
    {
        return Err(M5FilePathRevealResolutionError::ForbiddenMaterial);
    }

    let action_role_invented = matches!(
        input.action_role,
        M5PlatformConventionRole::InventedPrivateConventionDisallowed
    );
    let all_forms = covers_all_presentation_forms(&input.presentation_form_coverage);
    let provides_triple = input.action_surface.is_classified()
        && !input.command_id.trim().is_empty()
        && !input.human_label.trim().is_empty()
        && !input.in_product_route.trim().is_empty()
        && input.reachable_by_id_surface_and_command;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5WindowMenuActionEntryDegradeReason::ActionTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5WindowMenuActionEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.action_surface.is_classified() {
        Some(M5WindowMenuActionEntryDegradeReason::ActionSurfaceUnclassified)
    } else if action_role_invented || !provides_triple {
        Some(M5WindowMenuActionEntryDegradeReason::ReachableOnlyInOsChrome)
    } else if !all_forms {
        Some(M5WindowMenuActionEntryDegradeReason::WindowMenuPhrasingCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5WindowMenuActionEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5FilePathRegistryNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedWindowMenuActionEntry {
        entry_id: input.entry_id,
        command_id: input.command_id,
        token_name: input.token_name,
        action_role: input.action_role.as_str().to_owned(),
        action_role_invented,
        semantic_role: input.semantic_role.as_str().to_owned(),
        action_surface: input.action_surface.as_str().to_owned(),
        action_surface_is_classified: input.action_surface.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        presentation_form_coverage: form_tokens(&input.presentation_form_coverage),
        covers_all_presentation_forms: all_forms,
        human_label: input.human_label,
        in_product_route: input.in_product_route,
        reachable_by_id_surface_and_command: input.reachable_by_id_surface_and_command,
        provides_complete_reachability_triple: provides_triple,
        degrade_reason,
        next_action,
        action_reachable_on_every_profile: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved file-path-presentation and window / menu
/// action entries it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FilePathRevealRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5FilePathRevealRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5PlatformFitQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5PlatformFitDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5PlatformFitRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5PlatformFitAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5FilePathRegistryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5FilePathRegistryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5PlatformFitDowngradeTrigger>,
    /// Resolved file-path-presentation-registry examples.
    pub file_path_presentation_entries: Vec<M5ResolvedFilePathPresentationEntry>,
    /// Resolved window / menu action examples.
    pub window_menu_action_entries: Vec<M5ResolvedWindowMenuActionEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical file-path-and-reveal domain
    /// schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: platform-specific path wording never changes command or permission meaning. MUST be
    /// `false`.
    pub path_terminology_changes_command_or_permission_meaning: bool,
    /// Hard invariant: a primary action is never reachable only in OS chrome (menus / title bars). MUST be
    /// `false`.
    pub primary_action_reachable_only_in_os_chrome: bool,
    /// Hard invariant: terminology is never hand-copied per platform instead of tracing to the registry. MUST
    /// be `false`.
    pub terminology_hardcoded_instead_of_registry: bool,
    /// Hard invariant: a screenshot or docs page never mislabels a path or reveal verb. MUST be `false`.
    pub screenshot_or_docs_mislabels_path_verb: bool,
}

impl M5FilePathRevealRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5FilePathRegistryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5FilePathRegistryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5FilePathRegistryExportField> =
            self.export_fields.iter().copied().collect();
        M5FilePathRegistryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.path_terminology_changes_command_or_permission_meaning
            && !self.primary_action_reachable_only_in_os_chrome
            && !self.terminology_hardcoded_instead_of_registry
            && !self.screenshot_or_docs_mislabels_path_verb
    }

    /// True when a clean file-path entry preserves registry-bound terminology: it traces to the registry,
    /// never names the disallowed mislabeled-verb role, keeps a classified host platform, matches the host
    /// convention, keeps the canonical path truth explicit, covers all three presentation forms, and explains
    /// any unavailable-reveal fallback.
    fn path_is_honest(ex: &M5ResolvedFilePathPresentationEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && !ex.path_role_mislabeled
                && ex.host_platform_is_classified
                && ex.path_matches_host
                && ex.preserves_canonical_path_truth
                && ex.covers_all_presentation_forms
                && (!ex.reveal_target_unavailable || ex.fallback_explained))
    }

    /// True when a clean window / menu action entry preserves reachability: it keeps a classified product
    /// surface, never names the disallowed invented-convention role, provides the reachability triple, and
    /// covers all three presentation forms.
    fn action_is_honest(ex: &M5ResolvedWindowMenuActionEntry) -> bool {
        !ex.is_clean()
            || (ex.action_surface_is_classified
                && !ex.action_role_invented
                && ex.provides_complete_reachability_triple
                && ex.covers_all_presentation_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.file_path_presentation_entries
            .iter()
            .all(Self::path_is_honest)
            && self
                .window_menu_action_entries
                .iter()
                .all(Self::action_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FilePathRevealRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// File-path-reveal-role tokens (bound from the frozen matrix).
    pub path_roles: Vec<String>,
    /// Platform-convention-role tokens (bound from the frozen matrix).
    pub convention_roles: Vec<String>,
    /// Presentation-form tokens (minted by this lane).
    pub presentation_forms: Vec<String>,
    /// Host-platform tokens (minted by this lane).
    pub host_platforms: Vec<String>,
    /// Product-action-surface tokens (minted by this lane).
    pub product_action_surfaces: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// File-path-presentation-entry degrade-reason tokens.
    pub file_path_presentation_degrade_reasons: Vec<String>,
    /// Window-menu-action-entry degrade-reason tokens.
    pub window_menu_action_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5FilePathRevealRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5PlatformFitRole::ALL, |v| v.as_str()),
            path_roles: tokens(&M5FilePathRevealRole::ALL, |v| v.as_str()),
            convention_roles: tokens(&M5PlatformConventionRole::ALL, |v| v.as_str()),
            presentation_forms: tokens(&M5PathPresentationForm::ALL, |v| v.as_str()),
            host_platforms: tokens(&M5HostPlatform::ALL, |v| v.as_str()),
            product_action_surfaces: tokens(&M5ProductActionSurface::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5FilePathSurfaceContext::ALL, |v| v.as_str()),
            file_path_presentation_degrade_reasons: tokens(
                &M5FilePathPresentationEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            window_menu_action_degrade_reasons: tokens(
                &M5WindowMenuActionEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            anatomy_parts: tokens(&M5FilePathRegistryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5FilePathRegistryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5FilePathRegistryExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5PlatformFitConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5FilePathRevealRegistriesGovernanceReview {
    /// The file-path registry names a canonical token, path role, and host platform for every entry.
    pub terminology_registry_names_token_role_and_platform: bool,
    /// Host-correct terms and separators render from the shared registry, not per-surface strings.
    pub host_correct_terms_and_separators_rendered_from_shared_registry: bool,
    /// Literal-versus-canonical path truth is kept explicit on every path entry.
    pub literal_versus_canonical_path_truth_kept_explicit: bool,
    /// Reveal and save terminology match the host platform on every claimed profile.
    pub reveal_and_save_terminology_match_host_platform: bool,
    /// High-frequency actions are reachable from product surfaces and commands, not only OS chrome.
    pub high_frequency_actions_reachable_from_product_surfaces: bool,
    /// Native window-chrome expectations and menu phrasing are preserved without divergent product meaning.
    pub native_window_chrome_and_menu_phrasing_preserved: bool,
    /// Every path and action entry covers the host-styled / canonical / accessible presentation forms.
    pub every_entry_covers_all_presentation_forms: bool,
    /// Terminology stays bound to one registry rather than hand-copied per platform.
    pub terminology_bound_to_single_registry_not_hand_copied: bool,
    /// Docs, help, and screenshots are generated from the same path / reveal registry.
    pub docs_help_and_screenshots_generated_from_registry: bool,
    /// Mislabeled path / reveal verbs or a menu-only action are caught by fixtures before release evidence
    /// turns green.
    pub path_or_chrome_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FilePathRevealRegistriesConsumerProjection {
    /// The shell (file dialogs / reveal menu) consumes the shared file-path / window-menu registries.
    pub shell_consumes_shared_registries: bool,
    /// The settings (path presentation) consumes the shared registries.
    pub settings_consumes_shared_registries: bool,
    /// Docs and help consume the shared registries.
    pub docs_help_consumes_shared_registries: bool,
    /// Onboarding and CLI export consume the shared registries.
    pub onboarding_and_cli_consume_shared_registries: bool,
    /// Terminology traces back to one canonical file-path-and-reveal domain contract.
    pub terminology_traces_to_single_domain_contract: bool,
    /// Support / export reads a single canonical file-path / window-menu registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FilePathRevealRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FilePathRevealRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting platform-fit audit for the lane.
    pub platform_fit_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5FilePathRevealRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5FilePathRevealRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5FilePathRevealRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5FilePathRevealRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5FilePathRevealRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FilePathRevealRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5FilePathRevealRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5FilePathRevealRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 file-path-presentation and native-window / menu registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FilePathRevealRegistriesPacket {
    /// Record kind; must equal [`M5_FILE_PATH_REVEAL_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_FILE_PATH_REVEAL_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5FilePathRevealRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5FilePathRevealRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5FilePathRevealRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FilePathRevealRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5FilePathRevealRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5FilePathRevealRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5FilePathRevealRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5FilePathRevealRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_FILE_PATH_REVEAL_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_FILE_PATH_REVEAL_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5FilePathRevealRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_FILE_PATH_REVEAL_REGISTRIES_RECORD_KIND {
            violations.push(M5FilePathRevealRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_FILE_PATH_REVEAL_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5FilePathRevealRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5FilePathRevealRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5FilePathRevealRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 file-path-reveal / window-menu registries packet serializes"),
        ) {
            violations.push(M5FilePathRevealRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 file-path-reveal / window-menu registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,file_path_presentation_entries,window_menu_action_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .file_path_presentation_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.window_menu_action_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.file_path_presentation_entries.len(),
                row.window_menu_action_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 File-Path-Presentation and Native-Window / Menu Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Host platforms: {}\n",
            self.vocabulary_set.host_platforms.join(", ")
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
                "  - File-path entries: {} / window-menu entries: {}\n",
                row.file_path_presentation_entries.len(),
                row.window_menu_action_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-platform help / screenshot path / reveal table generated from the registry, so docs
    /// and tutorials render the same command / platform / path / reveal-verb truth the resolvers produced
    /// rather than a hand-copied screenshot. Only clean, registry-bound path entries are listed.
    pub fn render_platform_path_reveal_table(&self) -> String {
        let mut out = String::new();
        out.push_str("| command_id | host_platform | path | reveal_verb | surface |\n");
        out.push_str("| --- | --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.file_path_presentation_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | {} | {} |\n",
                    ex.command_id,
                    ex.host_platform,
                    ex.rendered_path,
                    ex.reveal_verb,
                    ex.surface_context
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5FilePathRevealRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5FilePathRevealRegistriesViolation>),
}

impl fmt::Display for M5FilePathRevealRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 file-path-reveal / window-menu registries export parse failed: {error}"
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
                    "m5 file-path-reveal / window-menu registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5FilePathRevealRegistriesArtifactError {}

/// Validation failures emitted by [`M5FilePathRevealRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5FilePathRevealRegistriesViolation {
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
    /// A registry row does not point at the canonical file-path-and-reveal domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, mislabeled, canonical-truth-losing,
    /// form-incomplete, or a window / menu entry missing the reachability triple).
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
    /// Host-correct terminology is not proven across surfaces: clean file-path entries do not cover the
    /// path-terminology / command-stability semantic-role families or the first file-open / save / reveal /
    /// breadcrumb / help surfaces, no hand-copied example degrades, or a clean entry is not bound to the
    /// registry.
    HostCorrectTermsAcrossSurfacesNotProven,
    /// Host-correct action reachability is not proven across profiles: clean window / menu entries do not
    /// cover the command-palette / toolbar / command-list product surfaces with full presentation-form
    /// coverage while providing the reachability triple, no reachable-only-in-os-chrome or phrasing-incomplete
    /// example degrades, or a clean entry is missing the triple.
    HostCorrectActionReachableOnEveryProfileNotProven,
    /// Wrong path verb or window / menu behavior is not detectable: no mislabeled-path example and no
    /// reachable-only-in-os-chrome example degrade, clean entries do not trace to the registry, or a clean
    /// entry is mislabeled for its host.
    WrongPathVerbOrChromeDetectableNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5FilePathRevealRegistriesViolation {
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
            Self::HostCorrectTermsAcrossSurfacesNotProven => {
                "host_correct_terms_across_surfaces_not_proven"
            }
            Self::HostCorrectActionReachableOnEveryProfileNotProven => {
                "host_correct_action_reachable_on_every_profile_not_proven"
            }
            Self::WrongPathVerbOrChromeDetectableNotProven => {
                "wrong_path_verb_or_chrome_detectable_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_file_path_reveal_and_native_window_menu_registries_export(
) -> Result<M5FilePathRevealRegistriesPacket, M5FilePathRevealRegistriesArtifactError> {
    let packet: M5FilePathRevealRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-file-path-reveal-and-native-window-menu-registries-proof/support_export.json"
    )))
    .map_err(M5FilePathRevealRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5FilePathRevealRegistriesArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5FilePathRevealRegistriesPacket,
    violations: &mut Vec<M5FilePathRevealRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_FILE_PATH_REVEAL_REGISTRIES_SCHEMA_REF,
        M5_FILE_PATH_REVEAL_REGISTRIES_DOC_REF,
        M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
        M5_PLATFORM_FIT_MATRIX_DOC_REF,
        M5_FILE_PATH_AND_REVEAL_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5FilePathRevealRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5FilePathRevealRegistriesPacket,
    violations: &mut Vec<M5FilePathRevealRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5FilePathRevealRegistriesViolation::NoRegistryRows);
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
            violations.push(M5FilePathRevealRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5FilePathRevealRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5FilePathRevealRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_FILE_PATH_AND_REVEAL_SCHEMA_REF) {
            violations.push(M5FilePathRevealRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.file_path_presentation_entries.is_empty()
            || row.window_menu_action_entries.is_empty()
        {
            violations.push(M5FilePathRevealRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5FilePathRevealRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5FilePathRevealRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5FilePathRevealRegistriesPacket,
    violations: &mut Vec<M5FilePathRevealRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.terminology_registry_names_token_role_and_platform,
        review.host_correct_terms_and_separators_rendered_from_shared_registry,
        review.literal_versus_canonical_path_truth_kept_explicit,
        review.reveal_and_save_terminology_match_host_platform,
        review.high_frequency_actions_reachable_from_product_surfaces,
        review.native_window_chrome_and_menu_phrasing_preserved,
        review.every_entry_covers_all_presentation_forms,
        review.terminology_bound_to_single_registry_not_hand_copied,
        review.docs_help_and_screenshots_generated_from_registry,
        review.path_or_chrome_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5FilePathRevealRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5FilePathRevealRegistriesPacket,
    violations: &mut Vec<M5FilePathRevealRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_shared_registries,
        projection.settings_consumes_shared_registries,
        projection.docs_help_consumes_shared_registries,
        projection.onboarding_and_cli_consume_shared_registries,
        projection.terminology_traces_to_single_domain_contract,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(M5FilePathRevealRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5FilePathRevealRegistriesPacket,
    violations: &mut Vec<M5FilePathRevealRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5FilePathRevealRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5FilePathRevealRegistriesPacket,
    violations: &mut Vec<M5FilePathRevealRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.platform_fit_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5FilePathRevealRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted
/// by governance bools.
fn validate_acceptance_criteria(
    packet: &M5FilePathRevealRegistriesPacket,
    violations: &mut Vec<M5FilePathRevealRegistriesViolation>,
) {
    let paths = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.file_path_presentation_entries.iter())
    };
    let actions = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.window_menu_action_entries.iter())
    };

    // AC1: file, folder, workspace, and reveal flows use host-correct terms and separators across surfaces.
    // Clean file-path entries cover the path-terminology / command-stability semantic-role families and the
    // first file-open / save / reveal / breadcrumb / help surfaces, a hand-copied example degrades, and no
    // clean entry is unbound.
    let clean_semantic_roles: BTreeSet<String> = paths()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.semantic_role.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = paths()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let semantic_families_covered = [
        M5PlatformFitRole::PathTerminology.as_str(),
        M5PlatformFitRole::CommandStability.as_str(),
    ]
    .iter()
    .all(|r| clean_semantic_roles.contains(*r));
    let first_surfaces_covered = M5FilePathSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let hand_copied_degrades = paths().any(|ex| {
        ex.degrade_reason
            == Some(M5FilePathPresentationEntryDegradeReason::TerminologyNotBoundToRegistry)
    });
    let no_clean_unbound = !paths().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    if !(semantic_families_covered
        && first_surfaces_covered
        && hand_copied_degrades
        && no_clean_unbound)
    {
        violations
            .push(M5FilePathRevealRegistriesViolation::HostCorrectTermsAcrossSurfacesNotProven);
    }

    // AC2: no critical action is reachable only through OS chrome or menu-bar affordances. Clean window /
    // menu entries cover every canonical product surface with full presentation-form coverage while providing
    // the reachability triple, a reachable-only-in-os-chrome example degrades, a phrasing-incomplete example
    // degrades, and no clean entry is missing the triple.
    let clean_action_surfaces: BTreeSet<String> = actions()
        .filter(|ex| {
            ex.is_clean()
                && ex.action_surface_is_classified
                && ex.provides_complete_reachability_triple
                && ex.covers_all_presentation_forms
        })
        .map(|ex| ex.action_surface.clone())
        .collect();
    let action_surfaces_covered = M5ProductActionSurface::CANONICAL_SURFACES
        .iter()
        .all(|s| clean_action_surfaces.contains(s.as_str()));
    let os_chrome_only_degrades = actions().any(|ex| {
        ex.degrade_reason == Some(M5WindowMenuActionEntryDegradeReason::ReachableOnlyInOsChrome)
    });
    let phrasing_incomplete_degrades = actions().any(|ex| {
        ex.degrade_reason
            == Some(M5WindowMenuActionEntryDegradeReason::WindowMenuPhrasingCoverageIncomplete)
    });
    let no_clean_missing_triple =
        !actions().any(|ex| ex.is_clean() && !ex.provides_complete_reachability_triple);
    if !(action_surfaces_covered
        && os_chrome_only_degrades
        && phrasing_incomplete_degrades
        && no_clean_missing_triple)
    {
        violations.push(
            M5FilePathRevealRegistriesViolation::HostCorrectActionReachableOnEveryProfileNotProven,
        );
    }

    // AC3: cross-platform review fixtures fail when a surface shows the wrong path separator, reveal verb, or
    // a menu-only action. A mislabeled-path example and a reachable-only-in-os-chrome example both degrade, at
    // least one clean path and one clean window / menu entry trace to the registry, no clean path is unbound,
    // and no clean path is mislabeled for its host.
    let mislabeled_degrades = paths().any(|ex| {
        ex.degrade_reason
            == Some(M5FilePathPresentationEntryDegradeReason::PathOrRevealMislabeledForHost)
    });
    let bound_path = paths().any(|ex| ex.is_clean() && ex.bound_to_registry);
    let bound_action =
        actions().any(|ex| ex.is_clean() && ex.provides_complete_reachability_triple);
    let no_clean_mislabeled = !paths().any(|ex| ex.is_clean() && !ex.path_matches_host);
    if !(mislabeled_degrades
        && os_chrome_only_degrades
        && bound_path
        && bound_action
        && no_clean_unbound
        && no_clean_mislabeled)
    {
        violations
            .push(M5FilePathRevealRegistriesViolation::WrongPathVerbOrChromeDetectableNotProven);
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

/// The platform-fit families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5PlatformFitFamily; 2] = [
    M5PlatformFitFamily::FilePathReveal,
    M5PlatformFitFamily::PlatformConvention,
];
