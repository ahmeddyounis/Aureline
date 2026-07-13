//! Implemented M5 shortcut-notation and command-label mapping registries.
//!
//! The frozen [platform-fit matrix][matrix] names Aureline's six platform-fit families and locks their
//! controlled vocabulary. This module is the first implement lane over that matrix: it turns the concrete
//! *keyboard-notation* grammar of the shortcut-notation family into registry resolvers that produce
//! export-safe, honest projections. A user can then trust that the same command is discoverable by its
//! stable command ID, its human label, and platform-appropriate shortcut text on every claimed macOS,
//! Windows, and Linux desktop profile, that keyboard notation and help / screenshot content stay consistent
//! with the active platform without changing command semantics, and that a surface showing the wrong
//! modifier notation, label mapping, or reserved-key explanation degrades honestly instead of reading as a
//! clean pass.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Render platform-native shortcut notation and modifier vocabulary while preserving stable command IDs
//!   and searchability.** [`resolve_shortcut_notation_entry`] refuses to read as a clean, registry-bound
//!   notation entry unless it names a canonical registry token, a classified [host platform][M5HostPlatform],
//!   a shortcut-notation role, covers every [notation form][M5ShortcutNotationForm] (the visual notation, the
//!   spoken accessible form, and the searchable command text), renders notation that matches the host
//!   platform's modifier convention, preserves the stable command ID, and explains any OS-reserved fallback;
//!   otherwise it degrades.
//! * **Support macOS glyph and naming conventions, Windows and Linux modifier names, and explicit fallback
//!   vocabulary for unsupported or reserved shortcuts.** Each host platform carries its canonical primary
//!   modifier representation, and [`notation_matches_host`] rejects a macOS entry rendered with `Ctrl` /
//!   `Alt` text or a Windows / Linux entry rendered with `⌘` / `⌥` glyphs so a mislabeled notation degrades to
//!   [`M5ShortcutNotationEntryDegradeReason::NotationMislabeledForHost`].
//! * **Generate platform-specific docs / help / screenshot outputs from the same keybinding and label
//!   registry.** [`resolve_command_label_mapping_entry`] names a classified [label kind][M5CommandLabelKind],
//!   requires the command to be discoverable by stable ID, human label, and platform-appropriate shortcut
//!   text, covers every notation form, and degrades to
//!   [`M5CommandLabelMappingDegradeReason::DiscoveryTripleIncomplete`] when a command drops any leg of the
//!   discovery triple, so a screenshot or tutorial cannot reintroduce incorrect notation.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5PlatformFitRole`] role vocabulary and
//! the [`M5ShortcutNotationRole`] shortcut-notation-role vocabulary — so shell, settings, docs, onboarding,
//! CLI, and support surfaces can never fork their own shortcut-notation or command-label meaning. Raw secret
//! values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_platform_fit_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_shortcut_notation_command_label_registries,
    seeded_m5_shortcut_notation_command_label_registries_docs_help_beta_narrowed,
    seeded_m5_shortcut_notation_command_label_registries_onboarding_preview_narrowed,
    M5_SHORTCUT_NOTATION_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_platform_fit_matrix::{
    M5PlatformFitAccessibilityRoute, M5PlatformFitConsumerSurface, M5PlatformFitDeploymentLine,
    M5PlatformFitDowngradeTrigger, M5PlatformFitFamily, M5PlatformFitQualificationClass,
    M5PlatformFitRequiredLabel, M5PlatformFitRole, M5ShortcutNotationRole,
    M5_PLATFORM_FIT_MATRIX_DOC_REF, M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
    M5_SHORTCUT_NOTATION_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5ShortcutNotationRegistriesPacket`].
pub const M5_SHORTCUT_NOTATION_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_shortcut_notation_and_command_label_registries";

/// Schema version for M5 shortcut-notation / command-label registry records.
pub const M5_SHORTCUT_NOTATION_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_SHORTCUT_NOTATION_REGISTRIES_SCHEMA_REF: &str =
    "schemas/platform/m5-shortcut-notation-and-command-label-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_SHORTCUT_NOTATION_REGISTRIES_DOC_REF: &str =
    "docs/platform/m5_shortcut_notation_and_command_label_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SHORTCUT_NOTATION_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-shortcut-notation-and-command-label-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_SHORTCUT_NOTATION_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-shortcut-notation-and-command-label-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SHORTCUT_NOTATION_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-shortcut-notation-and-command-label-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_SHORTCUT_NOTATION_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/platform/m5-shortcut-notation-and-command-label-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5ShortcutNotationRegistriesConsumerSurface = M5PlatformFitConsumerSurface;

/// One of the three notation forms every shortcut-notation / command-label entry must hold across so a
/// command's keyboard notation keeps its meaning whether it is shown visually, announced to a screen reader,
/// or matched by command search. Minted by this lane because the frozen matrix names the shortcut-notation
/// *family* but not the concrete form set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShortcutNotationForm {
    /// The visual notation (platform-native glyphs or modifier names).
    VisualNotation,
    /// The spoken, accessible form announced to a screen reader.
    SpokenAccessibleForm,
    /// The searchable command text that keeps the command discoverable by name.
    SearchableCommandText,
}

impl M5ShortcutNotationForm {
    /// Every notation form, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [
        Self::VisualNotation,
        Self::SpokenAccessibleForm,
        Self::SearchableCommandText,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VisualNotation => "visual_notation",
            Self::SpokenAccessibleForm => "spoken_accessible_form",
            Self::SearchableCommandText => "searchable_command_text",
        }
    }
}

/// Controlled host platform a notation entry adapts to, so the canonical modifier glyphs and names share one
/// registry rather than a hand-copied per-platform string. Minted by this lane because the frozen matrix
/// carries the macOS / Windows / Linux surface families but not the concrete modifier convention an entry
/// must match. Every classified platform carries its canonical primary-modifier representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HostPlatform {
    /// The macOS platform (glyph notation: ⌘ ⌥ ⌃ ⇧).
    Macos,
    /// The Windows platform (modifier-name notation: Ctrl / Alt / Shift).
    Windows,
    /// The Linux platform (modifier-name notation: Ctrl / Alt / Shift).
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

    /// The three canonical desktop platforms every claimed M5 profile resolves notation from.
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

    /// Whether this platform renders shortcuts with glyphs (macOS) rather than modifier names.
    pub const fn uses_glyph_notation(self) -> bool {
        matches!(self, Self::Macos)
    }

    /// The canonical primary-modifier representation for this platform, before command-specific chords.
    pub const fn canonical_primary_modifier(self) -> &'static str {
        match self {
            Self::Macos => "⌘",
            Self::Windows | Self::Linux => "Ctrl",
            Self::PlatformUnknown => "",
        }
    }
}

/// Controlled command-label kind a mapping entry maps, so the menu label, palette label, and help label
/// share one registry and stay discoverable by stable command ID, human label, and shortcut text. Minted by
/// this lane, tracking the label surfaces the acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CommandLabelKind {
    /// A menu-bar label.
    MenuLabel,
    /// A command-palette label.
    PaletteLabel,
    /// A help / docs label.
    HelpLabel,
    /// The label kind is unclassified, which is disallowed.
    LabelUnclassified,
}

impl M5CommandLabelKind {
    /// Every label kind, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::MenuLabel,
        Self::PaletteLabel,
        Self::HelpLabel,
        Self::LabelUnclassified,
    ];

    /// The three canonical label kinds whose mappings must keep the command discoverable.
    pub const CANONICAL_LABELS: [Self; 3] = [Self::MenuLabel, Self::PaletteLabel, Self::HelpLabel];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MenuLabel => "menu_label",
            Self::PaletteLabel => "palette_label",
            Self::HelpLabel => "help_label",
            Self::LabelUnclassified => "label_unclassified",
        }
    }

    /// Whether the label kind is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::LabelUnclassified)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a shortcut-notation or
/// command-label token's meaning stays stable whether it appears in the menu bar, command palette, keybinding
/// inspector, help, or onboarding. Minted by this lane, tracking the first-consumer surfaces the
/// implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShortcutSurfaceContext {
    /// The menu-bar surface.
    MenuBar,
    /// The command-palette surface.
    CommandPalette,
    /// The keybinding-inspector surface.
    KeybindingInspector,
    /// The help / docs surface.
    HelpDoc,
    /// The onboarding surface.
    Onboarding,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5ShortcutSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::MenuBar,
        Self::CommandPalette,
        Self::KeybindingInspector,
        Self::HelpDoc,
        Self::Onboarding,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::MenuBar,
        Self::CommandPalette,
        Self::KeybindingInspector,
        Self::HelpDoc,
        Self::Onboarding,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MenuBar => "menu_bar",
            Self::CommandPalette => "command_palette",
            Self::KeybindingInspector => "keybinding_inspector",
            Self::HelpDoc => "help_doc",
            Self::Onboarding => "onboarding",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a shortcut-notation or command-label entry must be able to show, so no
/// notation, label, or registry fact is left implicit behind a hand-copied per-platform string.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShortcutRegistryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical registry reference the entry points at.
    RegistryReference,
    /// The host platform the entry adapts to (notation entry).
    HostPlatform,
    /// The rendered platform-native notation text (notation entry).
    NotationText,
    /// The notation-form coverage (visual / spoken / searchable).
    NotationFormCoverage,
    /// The command label the entry maps (command-label entry).
    CommandLabel,
    /// The platform-appropriate shortcut text the entry shows (command-label entry).
    ShortcutText,
    /// The render / surface context (both entries).
    SurfaceContext,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the command (both entries).
    PlainLanguageMeaning,
}

impl M5ShortcutRegistryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::RegistryReference,
        Self::HostPlatform,
        Self::NotationText,
        Self::NotationFormCoverage,
        Self::CommandLabel,
        Self::ShortcutText,
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
            Self::NotationText => "notation_text",
            Self::NotationFormCoverage => "notation_form_coverage",
            Self::CommandLabel => "command_label",
            Self::ShortcutText => "shortcut_text",
            Self::SurfaceContext => "surface_context",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a notation,
/// label mapping, or a degraded shortcut-notation / command-label entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShortcutRegistryNextAction {
    /// Expand the notation's plain-language meaning.
    ExpandNotationMeaning,
    /// Inspect the host platform or label kind the entry maps.
    InspectPlatformOrLabel,
    /// Complete the visual / spoken / searchable notation-form coverage.
    CompleteNotationFormCoverage,
    /// Trace the entry back to its canonical registry token.
    TraceCanonicalRegistry,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5ShortcutRegistryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandNotationMeaning,
        Self::InspectPlatformOrLabel,
        Self::CompleteNotationFormCoverage,
        Self::TraceCanonicalRegistry,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandNotationMeaning => "expand_notation_meaning",
            Self::InspectPlatformOrLabel => "inspect_platform_or_label",
            Self::CompleteNotationFormCoverage => "complete_notation_form_coverage",
            Self::TraceCanonicalRegistry => "trace_canonical_registry",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShortcutRegistryExportField {
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
    /// The notation forms covered.
    NotationForms,
    /// The command-label kinds carried.
    CommandLabelKinds,
    /// The render / surface context.
    SurfaceContext,
    /// The rendered shortcut texts carried.
    ShortcutTexts,
    /// The accountable owner role.
    OwnerRole,
}

impl M5ShortcutRegistryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::PlatformFitFamilies,
        Self::HostPlatforms,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::NotationForms,
        Self::CommandLabelKinds,
        Self::SurfaceContext,
        Self::ShortcutTexts,
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
            Self::NotationForms => "notation_forms",
            Self::CommandLabelKinds => "command_label_kinds",
            Self::SurfaceContext => "surface_context",
            Self::ShortcutTexts => "shortcut_texts",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a shortcut-notation entry degraded below a clean, registry-bound state. The degrade-first ladder
/// returns one of these instead of ever letting a hand-copied, mislabeled, identity-unstable, or
/// form-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShortcutNotationEntryDegradeReason {
    /// The canonical registry token name is unstated; a user cannot trace what the command means.
    CommandTokenUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The host platform is unclassified (not in the preserved taxonomy).
    HostPlatformUnclassified,
    /// The notation is a hand-copied per-platform string instead of tracing to the canonical registry.
    NotationNotBoundToRegistry,
    /// The rendered notation does not match the host platform's modifier convention.
    NotationMislabeledForHost,
    /// The rendered notation does not preserve the stable command ID.
    CommandIdentityNotStable,
    /// The visual / spoken / searchable notation-form coverage is incomplete.
    NotationFormCoverageIncomplete,
    /// The shortcut is reserved by the OS and no fallback vocabulary is explained.
    ReservedKeyWithoutFallback,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ShortcutNotationEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::CommandTokenUnstated,
        Self::SurfaceContextUnresolved,
        Self::HostPlatformUnclassified,
        Self::NotationNotBoundToRegistry,
        Self::NotationMislabeledForHost,
        Self::CommandIdentityNotStable,
        Self::NotationFormCoverageIncomplete,
        Self::ReservedKeyWithoutFallback,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandTokenUnstated => "command_token_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::HostPlatformUnclassified => "host_platform_unclassified",
            Self::NotationNotBoundToRegistry => "notation_not_bound_to_registry",
            Self::NotationMislabeledForHost => "notation_mislabeled_for_host",
            Self::CommandIdentityNotStable => "command_identity_not_stable",
            Self::NotationFormCoverageIncomplete => "notation_form_coverage_incomplete",
            Self::ReservedKeyWithoutFallback => "reserved_key_without_fallback",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ShortcutRegistryNextAction {
        match self {
            Self::CommandTokenUnstated | Self::NotationNotBoundToRegistry => {
                M5ShortcutRegistryNextAction::TraceCanonicalRegistry
            }
            Self::HostPlatformUnclassified
            | Self::NotationMislabeledForHost
            | Self::CommandIdentityNotStable => {
                M5ShortcutRegistryNextAction::InspectPlatformOrLabel
            }
            Self::NotationFormCoverageIncomplete => {
                M5ShortcutRegistryNextAction::CompleteNotationFormCoverage
            }
            Self::SurfaceContextUnresolved
            | Self::ReservedKeyWithoutFallback
            | Self::ProofStale => M5ShortcutRegistryNextAction::ReviewBlockedOrDegraded,
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5PlatformFitDowngradeTrigger {
        match self {
            Self::CommandTokenUnstated | Self::NotationFormCoverageIncomplete => {
                M5PlatformFitDowngradeTrigger::ShortcutNotationUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5PlatformFitDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::HostPlatformUnclassified => M5PlatformFitDowngradeTrigger::HostPlatformUnstated,
            Self::NotationNotBoundToRegistry => {
                M5PlatformFitDowngradeTrigger::ShortcutNotationDriftedByPlatform
            }
            Self::NotationMislabeledForHost => {
                M5PlatformFitDowngradeTrigger::ScreenshotOrDocsMislabeledShortcutOrPathVerb
            }
            Self::CommandIdentityNotStable => {
                M5PlatformFitDowngradeTrigger::PlatformWordingChangedCommandOrPermissionMeaning
            }
            Self::ReservedKeyWithoutFallback => {
                M5PlatformFitDowngradeTrigger::PrimaryActionHiddenOnlyInOsChrome
            }
            Self::ProofStale => M5PlatformFitDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a command-label mapping entry degraded below a clean, discoverable state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CommandLabelMappingDegradeReason {
    /// The canonical registry token name is unstated.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The label kind is unclassified (not in the preserved taxonomy).
    LabelKindUnclassified,
    /// The command is not discoverable by stable ID, human label, and platform-appropriate shortcut text.
    DiscoveryTripleIncomplete,
    /// The visual / spoken / searchable notation-form coverage is incomplete.
    NotationFormCoverageIncomplete,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5CommandLabelMappingDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::LabelKindUnclassified,
        Self::DiscoveryTripleIncomplete,
        Self::NotationFormCoverageIncomplete,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::LabelKindUnclassified => "label_kind_unclassified",
            Self::DiscoveryTripleIncomplete => "discovery_triple_incomplete",
            Self::NotationFormCoverageIncomplete => "notation_form_coverage_incomplete",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5ShortcutRegistryNextAction {
        match self {
            Self::TokenNameUnstated => M5ShortcutRegistryNextAction::TraceCanonicalRegistry,
            Self::LabelKindUnclassified | Self::DiscoveryTripleIncomplete => {
                M5ShortcutRegistryNextAction::InspectPlatformOrLabel
            }
            Self::NotationFormCoverageIncomplete => {
                M5ShortcutRegistryNextAction::CompleteNotationFormCoverage
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5ShortcutRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5PlatformFitDowngradeTrigger {
        match self {
            Self::TokenNameUnstated | Self::NotationFormCoverageIncomplete => {
                M5PlatformFitDowngradeTrigger::ShortcutNotationUnstated
            }
            Self::SurfaceContextUnresolved | Self::LabelKindUnclassified => {
                M5PlatformFitDowngradeTrigger::RegistryReferenceUnstated
            }
            Self::DiscoveryTripleIncomplete => {
                M5PlatformFitDowngradeTrigger::ScreenshotOrDocsMislabeledShortcutOrPathVerb
            }
            Self::ProofStale => M5PlatformFitDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_shortcut_notation_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ShortcutNotationEntryResolutionInput {
    /// Stable identity of the shortcut-notation-registry entry.
    pub entry_id: String,
    /// The stable command ID this notation binds to (e.g. `command.file.save`); empty means unstated.
    pub command_id: String,
    /// The canonical registry token name (e.g. `shortcut.file.save.macos`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5PlatformFitRole,
    /// The shortcut-notation role (from the frozen matrix vocabulary).
    pub notation_role: M5ShortcutNotationRole,
    /// The host platform this entry adapts to.
    pub host_platform: M5HostPlatform,
    /// The render / surface context.
    pub surface_context: M5ShortcutSurfaceContext,
    /// The notation forms this entry holds across (must cover visual / spoken / searchable).
    pub notation_form_coverage: Vec<M5ShortcutNotationForm>,
    /// The rendered platform-native notation text (e.g. `⌘S` or `Ctrl+S`).
    pub rendered_notation: String,
    /// True when the notation traces to the shared keybinding registry (never a hand-copied constant).
    pub bound_to_registry: bool,
    /// True when the rendered notation preserves the stable command ID (a hard invariant when `false`).
    pub preserves_command_id: bool,
    /// True when the shortcut is reserved by the OS on this platform.
    pub reserved_by_os: bool,
    /// True when an explicit fallback vocabulary is explained for a reserved / unsupported shortcut.
    pub fallback_explained: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe shortcut-notation-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedShortcutNotationEntry {
    /// Stable identity of the shortcut-notation-registry entry.
    pub entry_id: String,
    /// The stable command ID this notation binds to.
    pub command_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role must preserve command identity as platform labels and notation adapt.
    pub semantic_role_preserves_command_identity_under_platform_adaptation: bool,
    /// The shortcut-notation-role token named by the entry.
    pub notation_role: String,
    /// Whether the notation role names the disallowed hard-coded-platform-notation token.
    pub notation_role_hardcoded: bool,
    /// The host-platform token named by the entry.
    pub host_platform: String,
    /// Whether the host platform is classified into the preserved taxonomy.
    pub host_platform_is_classified: bool,
    /// Whether the host platform renders shortcuts with glyphs rather than modifier names.
    pub host_uses_glyph_notation: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The rendered platform-native notation text.
    pub rendered_notation: String,
    /// The notation-form tokens covered by the entry.
    pub notation_form_coverage: Vec<String>,
    /// Whether the entry covers all three notation forms.
    pub covers_all_notation_forms: bool,
    /// Whether the rendered notation matches the host platform's modifier convention.
    pub notation_matches_host: bool,
    /// Whether the entry traces to the shared keybinding registry.
    pub bound_to_registry: bool,
    /// Whether the rendered notation preserves the stable command ID.
    pub preserves_command_id: bool,
    /// Whether the shortcut is reserved by the OS on this platform.
    pub reserved_by_os: bool,
    /// Whether an explicit fallback vocabulary is explained for a reserved / unsupported shortcut.
    pub fallback_explained: bool,
    /// Degrade reason, if the entry could not read as a clean, registry-bound state.
    pub degrade_reason: Option<M5ShortcutNotationEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ShortcutRegistryNextAction,
    /// Whether the notation holds across every notation form and platform (clean entry naming every fact).
    pub notation_holds_across_surfaces_and_platforms: bool,
}

impl M5ResolvedShortcutNotationEntry {
    /// Whether this shortcut-notation entry reads as a clean, registry-bound state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_command_label_mapping_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5CommandLabelMappingResolutionInput {
    /// Stable identity of the command-label mapping entry.
    pub entry_id: String,
    /// The stable command ID this label binds to; empty means unstated.
    pub command_id: String,
    /// The canonical registry token name; empty means unstated.
    pub token_name: String,
    /// The shortcut-notation role this mapping carries (from the frozen matrix vocabulary).
    pub mapping_role: M5ShortcutNotationRole,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5PlatformFitRole,
    /// The command-label kind this entry maps.
    pub label_kind: M5CommandLabelKind,
    /// The render / surface context.
    pub surface_context: M5ShortcutSurfaceContext,
    /// The notation forms this entry holds across (must cover visual / spoken / searchable).
    pub notation_form_coverage: Vec<M5ShortcutNotationForm>,
    /// The human-readable command label (e.g. `Save`); empty means missing.
    pub human_label: String,
    /// The platform-appropriate shortcut text (e.g. `Ctrl+S`); empty means missing.
    pub shortcut_text: String,
    /// True when the command is discoverable by stable ID, human label, and shortcut text.
    pub discoverable_by_id_label_and_shortcut: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe command-label mapping projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedCommandLabelMappingEntry {
    /// Stable identity of the command-label mapping entry.
    pub entry_id: String,
    /// The stable command ID this label binds to.
    pub command_id: String,
    /// The canonical registry token name named by the entry.
    pub token_name: String,
    /// The shortcut-notation-role token named by the entry.
    pub mapping_role: String,
    /// Whether the mapping role names the disallowed hard-coded-platform-notation token.
    pub mapping_role_hardcoded: bool,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// The command-label-kind token named by the entry.
    pub label_kind: String,
    /// Whether the label kind is classified into the preserved taxonomy.
    pub label_kind_is_classified: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The notation-form tokens covered by the entry.
    pub notation_form_coverage: Vec<String>,
    /// Whether the entry covers all three notation forms.
    pub covers_all_notation_forms: bool,
    /// The human-readable command label named by the entry.
    pub human_label: String,
    /// The platform-appropriate shortcut text named by the entry.
    pub shortcut_text: String,
    /// Whether the command is discoverable by stable ID, human label, and shortcut text.
    pub discoverable_by_id_label_and_shortcut: bool,
    /// Whether the entry provides the complete stable-ID / human-label / shortcut-text discovery triple.
    pub provides_complete_discovery_triple: bool,
    /// Degrade reason, if the entry could not read as a clean, discoverable state.
    pub degrade_reason: Option<M5CommandLabelMappingDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5ShortcutRegistryNextAction,
    /// Whether the command is discoverable on every claimed desktop profile (clean entry naming every fact).
    pub command_discoverable_on_every_profile: bool,
}

impl M5ResolvedCommandLabelMappingEntry {
    /// Whether this command-label mapping entry reads as a clean, discoverable state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5ShortcutNotationResolutionError {
    /// The shortcut-notation-entry id was empty.
    EmptyShortcutNotationEntryId,
    /// The command-label-entry id was empty.
    EmptyCommandLabelEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5ShortcutNotationResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyShortcutNotationEntryId => "empty_shortcut_notation_entry_id",
            Self::EmptyCommandLabelEntryId => "empty_command_label_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5ShortcutNotationResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 shortcut-notation / command-label registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5ShortcutNotationResolutionError {}

fn form_tokens(forms: &[M5ShortcutNotationForm]) -> Vec<String> {
    forms.iter().map(|f| f.as_str().to_owned()).collect()
}

fn covers_all_notation_forms(forms: &[M5ShortcutNotationForm]) -> bool {
    let present: BTreeSet<M5ShortcutNotationForm> = forms.iter().copied().collect();
    M5ShortcutNotationForm::ALL
        .iter()
        .all(|form| present.contains(form))
}

/// Whether a string carries any macOS modifier glyph.
fn contains_mac_glyph(value: &str) -> bool {
    value.chars().any(|c| matches!(c, '⌘' | '⌥' | '⌃' | '⇧'))
}

/// Whether a string carries a Windows / Linux modifier name.
fn contains_pc_modifier_word(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("ctrl")
        || lower.contains("alt")
        || lower.contains("shift")
        || lower.contains("win")
}

/// Whether the rendered notation matches the host platform's modifier convention: a macOS entry must render
/// with glyphs and never with `Ctrl` / `Alt` text, and a Windows / Linux entry must render with modifier
/// names and never with macOS glyphs. An unclassified or empty notation never matches.
pub fn notation_matches_host(host: M5HostPlatform, rendered_notation: &str) -> bool {
    if !host.is_classified() || rendered_notation.trim().is_empty() {
        return false;
    }
    match host {
        M5HostPlatform::Macos => {
            contains_mac_glyph(rendered_notation) && !contains_pc_modifier_word(rendered_notation)
        }
        M5HostPlatform::Windows | M5HostPlatform::Linux => {
            contains_pc_modifier_word(rendered_notation) && !contains_mac_glyph(rendered_notation)
        }
        M5HostPlatform::PlatformUnknown => false,
    }
}

/// Resolves a shortcut-notation-registry entry so it stays bound to the shared keybinding registry: the entry
/// names its canonical token, semantic role, notation role, and host platform, covers all three notation
/// forms, renders notation that matches the host convention, preserves the stable command ID, and explains
/// any reserved-key fallback.
pub fn resolve_shortcut_notation_entry(
    input: M5ShortcutNotationEntryResolutionInput,
) -> Result<M5ResolvedShortcutNotationEntry, M5ShortcutNotationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ShortcutNotationResolutionError::EmptyShortcutNotationEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.command_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.rendered_notation)
    {
        return Err(M5ShortcutNotationResolutionError::ForbiddenMaterial);
    }

    let notation_role_hardcoded = matches!(
        input.notation_role,
        M5ShortcutNotationRole::HardcodedPlatformNotationDisallowed
    );
    let all_forms = covers_all_notation_forms(&input.notation_form_coverage);
    let matches_host = notation_matches_host(input.host_platform, &input.rendered_notation);
    let reserved_without_fallback = input.reserved_by_os && !input.fallback_explained;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5ShortcutNotationEntryDegradeReason::CommandTokenUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ShortcutNotationEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.host_platform.is_classified() {
        Some(M5ShortcutNotationEntryDegradeReason::HostPlatformUnclassified)
    } else if notation_role_hardcoded || !input.bound_to_registry {
        Some(M5ShortcutNotationEntryDegradeReason::NotationNotBoundToRegistry)
    } else if !matches_host {
        Some(M5ShortcutNotationEntryDegradeReason::NotationMislabeledForHost)
    } else if !input.preserves_command_id {
        Some(M5ShortcutNotationEntryDegradeReason::CommandIdentityNotStable)
    } else if !all_forms {
        Some(M5ShortcutNotationEntryDegradeReason::NotationFormCoverageIncomplete)
    } else if reserved_without_fallback {
        Some(M5ShortcutNotationEntryDegradeReason::ReservedKeyWithoutFallback)
    } else if !input.proof_fresh {
        Some(M5ShortcutNotationEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ShortcutRegistryNextAction::ExpandNotationMeaning,
    };

    Ok(M5ResolvedShortcutNotationEntry {
        entry_id: input.entry_id,
        command_id: input.command_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_preserves_command_identity_under_platform_adaptation: input
            .semantic_role
            .must_preserve_command_identity_under_platform_adaptation(),
        notation_role: input.notation_role.as_str().to_owned(),
        notation_role_hardcoded,
        host_platform: input.host_platform.as_str().to_owned(),
        host_platform_is_classified: input.host_platform.is_classified(),
        host_uses_glyph_notation: input.host_platform.uses_glyph_notation(),
        surface_context: input.surface_context.as_str().to_owned(),
        rendered_notation: input.rendered_notation,
        notation_form_coverage: form_tokens(&input.notation_form_coverage),
        covers_all_notation_forms: all_forms,
        notation_matches_host: matches_host,
        bound_to_registry: input.bound_to_registry,
        preserves_command_id: input.preserves_command_id,
        reserved_by_os: input.reserved_by_os,
        fallback_explained: input.fallback_explained,
        degrade_reason,
        next_action,
        notation_holds_across_surfaces_and_platforms: degrade_reason.is_none(),
    })
}

/// Resolves a command-label mapping entry so the same command stays discoverable: the entry names its
/// canonical token, mapping role, semantic role, and label kind, covers all three notation forms, provides
/// the stable-ID / human-label / shortcut-text discovery triple, and degrades honestly when any leg is
/// missing.
pub fn resolve_command_label_mapping_entry(
    input: M5CommandLabelMappingResolutionInput,
) -> Result<M5ResolvedCommandLabelMappingEntry, M5ShortcutNotationResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5ShortcutNotationResolutionError::EmptyCommandLabelEntryId);
    }
    if string_is_forbidden(&input.entry_id)
        || string_is_forbidden(&input.command_id)
        || string_is_forbidden(&input.token_name)
        || string_is_forbidden(&input.human_label)
        || string_is_forbidden(&input.shortcut_text)
    {
        return Err(M5ShortcutNotationResolutionError::ForbiddenMaterial);
    }

    let mapping_role_hardcoded = matches!(
        input.mapping_role,
        M5ShortcutNotationRole::HardcodedPlatformNotationDisallowed
    );
    let all_forms = covers_all_notation_forms(&input.notation_form_coverage);
    let provides_triple = input.label_kind.is_classified()
        && !input.command_id.trim().is_empty()
        && !input.human_label.trim().is_empty()
        && !input.shortcut_text.trim().is_empty()
        && input.discoverable_by_id_label_and_shortcut;

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5CommandLabelMappingDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5CommandLabelMappingDegradeReason::SurfaceContextUnresolved)
    } else if !input.label_kind.is_classified() {
        Some(M5CommandLabelMappingDegradeReason::LabelKindUnclassified)
    } else if mapping_role_hardcoded || !provides_triple {
        Some(M5CommandLabelMappingDegradeReason::DiscoveryTripleIncomplete)
    } else if !all_forms {
        Some(M5CommandLabelMappingDegradeReason::NotationFormCoverageIncomplete)
    } else if !input.proof_fresh {
        Some(M5CommandLabelMappingDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5ShortcutRegistryNextAction::TraceCanonicalRegistry,
    };

    Ok(M5ResolvedCommandLabelMappingEntry {
        entry_id: input.entry_id,
        command_id: input.command_id,
        token_name: input.token_name,
        mapping_role: input.mapping_role.as_str().to_owned(),
        mapping_role_hardcoded,
        semantic_role: input.semantic_role.as_str().to_owned(),
        label_kind: input.label_kind.as_str().to_owned(),
        label_kind_is_classified: input.label_kind.is_classified(),
        surface_context: input.surface_context.as_str().to_owned(),
        notation_form_coverage: form_tokens(&input.notation_form_coverage),
        covers_all_notation_forms: all_forms,
        human_label: input.human_label,
        shortcut_text: input.shortcut_text,
        discoverable_by_id_label_and_shortcut: input.discoverable_by_id_label_and_shortcut,
        provides_complete_discovery_triple: provides_triple,
        degrade_reason,
        next_action,
        command_discoverable_on_every_profile: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved shortcut-notation and command-label entries
/// it must project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShortcutNotationRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5ShortcutNotationRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5ShortcutRegistryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5ShortcutRegistryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5PlatformFitDowngradeTrigger>,
    /// Resolved shortcut-notation-registry examples.
    pub shortcut_notation_entries: Vec<M5ResolvedShortcutNotationEntry>,
    /// Resolved command-label mapping examples.
    pub command_label_entries: Vec<M5ResolvedCommandLabelMappingEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical shortcut-notation domain
    /// schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: platform-specific notation never changes command or permission meaning. MUST be
    /// `false`.
    pub notation_changes_command_or_permission_meaning: bool,
    /// Hard invariant: a primary command is never hidden only in OS chrome (menus / title bars). MUST be
    /// `false`.
    pub primary_command_hidden_only_in_os_chrome: bool,
    /// Hard invariant: notation is never hand-copied per platform instead of tracing to the registry. MUST
    /// be `false`.
    pub notation_hardcoded_instead_of_registry: bool,
    /// Hard invariant: a screenshot or docs page never mislabels a shortcut. MUST be `false`.
    pub screenshot_or_docs_mislabels_shortcut: bool,
}

impl M5ShortcutNotationRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5ShortcutRegistryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5ShortcutRegistryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5ShortcutRegistryExportField> =
            self.export_fields.iter().copied().collect();
        M5ShortcutRegistryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.notation_changes_command_or_permission_meaning
            && !self.primary_command_hidden_only_in_os_chrome
            && !self.notation_hardcoded_instead_of_registry
            && !self.screenshot_or_docs_mislabels_shortcut
    }

    /// True when a clean shortcut-notation entry preserves registry-bound notation: it traces to the
    /// registry, never names the disallowed hard-coded role, keeps a classified host platform, matches the
    /// host convention, preserves the command ID, covers all three notation forms, and explains any reserved
    /// fallback.
    fn notation_is_honest(ex: &M5ResolvedShortcutNotationEntry) -> bool {
        !ex.is_clean()
            || (ex.bound_to_registry
                && !ex.notation_role_hardcoded
                && ex.host_platform_is_classified
                && ex.notation_matches_host
                && ex.preserves_command_id
                && ex.covers_all_notation_forms
                && (!ex.reserved_by_os || ex.fallback_explained))
    }

    /// True when a clean command-label entry preserves discoverability: it keeps a classified label kind,
    /// never names the disallowed hard-coded role, provides the discovery triple, and covers all three
    /// notation forms.
    fn label_is_honest(ex: &M5ResolvedCommandLabelMappingEntry) -> bool {
        !ex.is_clean()
            || (ex.label_kind_is_classified
                && !ex.mapping_role_hardcoded
                && ex.provides_complete_discovery_triple
                && ex.covers_all_notation_forms)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.shortcut_notation_entries
            .iter()
            .all(Self::notation_is_honest)
            && self.command_label_entries.iter().all(Self::label_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShortcutNotationRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Shortcut-notation-role tokens (bound from the frozen matrix).
    pub notation_roles: Vec<String>,
    /// Notation-form tokens (minted by this lane).
    pub notation_forms: Vec<String>,
    /// Host-platform tokens (minted by this lane).
    pub host_platforms: Vec<String>,
    /// Command-label-kind tokens (minted by this lane).
    pub command_label_kinds: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Shortcut-notation-entry degrade-reason tokens.
    pub shortcut_notation_degrade_reasons: Vec<String>,
    /// Command-label-entry degrade-reason tokens.
    pub command_label_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5ShortcutNotationRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5PlatformFitRole::ALL, |v| v.as_str()),
            notation_roles: tokens(&M5ShortcutNotationRole::ALL, |v| v.as_str()),
            notation_forms: tokens(&M5ShortcutNotationForm::ALL, |v| v.as_str()),
            host_platforms: tokens(&M5HostPlatform::ALL, |v| v.as_str()),
            command_label_kinds: tokens(&M5CommandLabelKind::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5ShortcutSurfaceContext::ALL, |v| v.as_str()),
            shortcut_notation_degrade_reasons: tokens(
                &M5ShortcutNotationEntryDegradeReason::ALL,
                |v| v.as_str(),
            ),
            command_label_degrade_reasons: tokens(&M5CommandLabelMappingDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5ShortcutRegistryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5ShortcutRegistryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5ShortcutRegistryExportField::ALL, |v| v.as_str()),
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
pub struct M5ShortcutNotationRegistriesGovernanceReview {
    /// The shortcut-notation registry names a canonical token, notation role, and host platform for every
    /// entry.
    pub notation_registry_names_token_role_and_platform: bool,
    /// Platform-native notation and modifier vocabulary render from the shared registry, not per-surface
    /// strings.
    pub platform_native_notation_rendered_from_shared_registry: bool,
    /// The same command is discoverable by stable command ID, human label, and platform-appropriate shortcut
    /// text on every claimed profile.
    pub command_discoverable_by_id_label_and_shortcut: bool,
    /// Command IDs stay stable while platform notation and labels adapt.
    pub command_ids_stable_while_notation_adapts: bool,
    /// macOS glyphs, Windows / Linux modifier names, and reserved-key fallbacks are all supported.
    pub macos_glyphs_windows_linux_names_and_fallbacks_supported: bool,
    /// Every notation and label entry covers the visual / spoken / searchable notation forms.
    pub every_entry_covers_all_notation_forms: bool,
    /// Notation stays bound to one registry rather than hand-copied per platform.
    pub notation_bound_to_single_registry_not_hand_copied: bool,
    /// Docs, help, and screenshots are generated from the same keybinding and label registry.
    pub docs_help_and_screenshots_generated_from_registry: bool,
    /// Mislabeled notation, wrong label mapping, or a missing reserved-key explanation is caught by fixtures
    /// before release evidence turns green.
    pub notation_drift_caught_before_release: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShortcutNotationRegistriesConsumerProjection {
    /// The shell (menus / palette) consumes the shared shortcut-notation / command-label registries.
    pub shell_consumes_shared_registries: bool,
    /// The settings (keybinding inspector) consumes the shared registries.
    pub settings_consumes_shared_registries: bool,
    /// Docs and help consume the shared registries.
    pub docs_help_consumes_shared_registries: bool,
    /// Onboarding and CLI export consume the shared registries.
    pub onboarding_and_cli_consume_shared_registries: bool,
    /// Notation traces back to one canonical shortcut-notation domain contract.
    pub notation_traces_to_single_domain_contract: bool,
    /// Support / export reads a single canonical shortcut-notation / command-label registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShortcutNotationRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShortcutNotationRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting platform-fit audit for the lane.
    pub platform_fit_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ShortcutNotationRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ShortcutNotationRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5ShortcutNotationRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ShortcutNotationRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ShortcutNotationRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ShortcutNotationRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ShortcutNotationRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ShortcutNotationRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 shortcut-notation and command-label mapping registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ShortcutNotationRegistriesPacket {
    /// Record kind; must equal [`M5_SHORTCUT_NOTATION_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SHORTCUT_NOTATION_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5ShortcutNotationRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ShortcutNotationRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ShortcutNotationRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ShortcutNotationRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ShortcutNotationRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ShortcutNotationRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ShortcutNotationRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5ShortcutNotationRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_SHORTCUT_NOTATION_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_SHORTCUT_NOTATION_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5ShortcutNotationRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SHORTCUT_NOTATION_REGISTRIES_RECORD_KIND {
            violations.push(M5ShortcutNotationRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SHORTCUT_NOTATION_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5ShortcutNotationRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ShortcutNotationRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5ShortcutNotationRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 shortcut-notation / command-label registries packet serializes"),
        ) {
            violations.push(M5ShortcutNotationRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 shortcut-notation / command-label registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,shortcut_notation_entries,command_label_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .shortcut_notation_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.command_label_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.shortcut_notation_entries.len(),
                row.command_label_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Shortcut-Notation and Command-Label Registries\n\n");
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
            "- Notation forms: {}\n",
            self.vocabulary_set.notation_forms.join(", ")
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
                "  - Shortcut-notation entries: {} / command-label entries: {}\n",
                row.shortcut_notation_entries.len(),
                row.command_label_entries.len()
            ));
        }
        out
    }

    /// Deterministic per-platform help / screenshot notation table generated from the registry, so docs and
    /// tutorials render the same command / platform / notation truth the resolvers produced rather than a
    /// hand-copied screenshot. Only clean, registry-bound notation entries are listed.
    pub fn render_platform_help_notation_table(&self) -> String {
        let mut out = String::new();
        out.push_str("| command_id | host_platform | notation | surface |\n");
        out.push_str("| --- | --- | --- | --- |\n");
        for row in &self.registry_rows {
            for ex in &row.shortcut_notation_entries {
                if !ex.is_clean() {
                    continue;
                }
                out.push_str(&format!(
                    "| `{}` | {} | `{}` | {} |\n",
                    ex.command_id, ex.host_platform, ex.rendered_notation, ex.surface_context
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5ShortcutNotationRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ShortcutNotationRegistriesViolation>),
}

impl fmt::Display for M5ShortcutNotationRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 shortcut-notation / command-label registries export parse failed: {error}"
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
                    "m5 shortcut-notation / command-label registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ShortcutNotationRegistriesArtifactError {}

/// Validation failures emitted by [`M5ShortcutNotationRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ShortcutNotationRegistriesViolation {
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
    /// A registry row does not point at the canonical shortcut-notation domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (hand-copied, mislabeled, identity-unstable,
    /// form-incomplete, or a command-label entry missing the discovery triple).
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
    /// Same-command discoverability is not proven: clean shortcut-notation entries do not cover the shortcut
    /// / command-stability semantic-role families or the first menu / palette / inspector / help / onboarding
    /// surfaces, no hand-copied example degrades, or a clean entry is not bound to the registry.
    SameCommandDiscoverableAcrossSurfacesNotProven,
    /// Command-label discoverability is not proven across profiles: clean command-label entries do not cover
    /// the menu / palette / help label kinds with full notation-form coverage while providing the discovery
    /// triple, no discovery-incomplete or form-incomplete example degrades, or a clean entry is missing the
    /// triple.
    CommandDiscoverableOnEveryProfileNotProven,
    /// Wrong notation or label mapping is not detectable: no mislabeled-notation example and no
    /// discovery-incomplete example degrade, clean entries do not trace to the registry, or a clean entry is
    /// mislabeled for its host.
    WrongNotationOrLabelDetectableNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ShortcutNotationRegistriesViolation {
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
            Self::SameCommandDiscoverableAcrossSurfacesNotProven => {
                "same_command_discoverable_across_surfaces_not_proven"
            }
            Self::CommandDiscoverableOnEveryProfileNotProven => {
                "command_discoverable_on_every_profile_not_proven"
            }
            Self::WrongNotationOrLabelDetectableNotProven => {
                "wrong_notation_or_label_detectable_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_shortcut_notation_command_label_registries_export(
) -> Result<M5ShortcutNotationRegistriesPacket, M5ShortcutNotationRegistriesArtifactError> {
    let packet: M5ShortcutNotationRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-shortcut-notation-and-command-label-registries-proof/support_export.json"
    )))
    .map_err(M5ShortcutNotationRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ShortcutNotationRegistriesArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5ShortcutNotationRegistriesPacket,
    violations: &mut Vec<M5ShortcutNotationRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SHORTCUT_NOTATION_REGISTRIES_SCHEMA_REF,
        M5_SHORTCUT_NOTATION_REGISTRIES_DOC_REF,
        M5_PLATFORM_FIT_MATRIX_SCHEMA_REF,
        M5_PLATFORM_FIT_MATRIX_DOC_REF,
        M5_SHORTCUT_NOTATION_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ShortcutNotationRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5ShortcutNotationRegistriesPacket,
    violations: &mut Vec<M5ShortcutNotationRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5ShortcutNotationRegistriesViolation::NoRegistryRows);
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
            violations.push(M5ShortcutNotationRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5ShortcutNotationRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5ShortcutNotationRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_SHORTCUT_NOTATION_SCHEMA_REF) {
            violations.push(M5ShortcutNotationRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.shortcut_notation_entries.is_empty() || row.command_label_entries.is_empty() {
            violations.push(M5ShortcutNotationRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5ShortcutNotationRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5ShortcutNotationRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ShortcutNotationRegistriesPacket,
    violations: &mut Vec<M5ShortcutNotationRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.notation_registry_names_token_role_and_platform,
        review.platform_native_notation_rendered_from_shared_registry,
        review.command_discoverable_by_id_label_and_shortcut,
        review.command_ids_stable_while_notation_adapts,
        review.macos_glyphs_windows_linux_names_and_fallbacks_supported,
        review.every_entry_covers_all_notation_forms,
        review.notation_bound_to_single_registry_not_hand_copied,
        review.docs_help_and_screenshots_generated_from_registry,
        review.notation_drift_caught_before_release,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5ShortcutNotationRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ShortcutNotationRegistriesPacket,
    violations: &mut Vec<M5ShortcutNotationRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_shared_registries,
        projection.settings_consumes_shared_registries,
        projection.docs_help_consumes_shared_registries,
        projection.onboarding_and_cli_consume_shared_registries,
        projection.notation_traces_to_single_domain_contract,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(M5ShortcutNotationRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ShortcutNotationRegistriesPacket,
    violations: &mut Vec<M5ShortcutNotationRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ShortcutNotationRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ShortcutNotationRegistriesPacket,
    violations: &mut Vec<M5ShortcutNotationRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.platform_fit_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ShortcutNotationRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted
/// by governance bools.
fn validate_acceptance_criteria(
    packet: &M5ShortcutNotationRegistriesPacket,
    violations: &mut Vec<M5ShortcutNotationRegistriesViolation>,
) {
    let notations = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.shortcut_notation_entries.iter())
    };
    let labels = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.command_label_entries.iter())
    };

    // AC1: the same command can be discovered by stable command ID, human label, and platform-appropriate
    // shortcut text on every claimed profile. Clean shortcut-notation entries cover the shortcut /
    // command-stability semantic-role families and the first menu / palette / inspector / help / onboarding
    // surfaces, a hand-copied example degrades, and no clean entry is unbound.
    let clean_semantic_roles: BTreeSet<String> = notations()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.semantic_role.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = notations()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let semantic_families_covered = [
        M5PlatformFitRole::Shortcut.as_str(),
        M5PlatformFitRole::CommandStability.as_str(),
    ]
    .iter()
    .all(|r| clean_semantic_roles.contains(*r));
    let first_surfaces_covered = M5ShortcutSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let hand_copied_degrades = notations().any(|ex| {
        ex.degrade_reason == Some(M5ShortcutNotationEntryDegradeReason::NotationNotBoundToRegistry)
    });
    let no_clean_unbound = !notations().any(|ex| ex.is_clean() && !ex.bound_to_registry);
    if !(semantic_families_covered
        && first_surfaces_covered
        && hand_copied_degrades
        && no_clean_unbound)
    {
        violations.push(
            M5ShortcutNotationRegistriesViolation::SameCommandDiscoverableAcrossSurfacesNotProven,
        );
    }

    // AC2: user-visible keyboard notation and help / screenshot content stay consistent with the active
    // platform. Clean command-label entries cover every canonical label kind with full notation-form
    // coverage while providing the discovery triple, a discovery-incomplete example degrades, a
    // form-incomplete example degrades, and no clean entry is missing the triple.
    let clean_label_kinds: BTreeSet<String> = labels()
        .filter(|ex| {
            ex.is_clean()
                && ex.label_kind_is_classified
                && ex.provides_complete_discovery_triple
                && ex.covers_all_notation_forms
        })
        .map(|ex| ex.label_kind.clone())
        .collect();
    let label_kinds_covered = M5CommandLabelKind::CANONICAL_LABELS
        .iter()
        .all(|k| clean_label_kinds.contains(k.as_str()));
    let discovery_incomplete_degrades = labels().any(|ex| {
        ex.degrade_reason == Some(M5CommandLabelMappingDegradeReason::DiscoveryTripleIncomplete)
    });
    let label_form_incomplete_degrades = labels().any(|ex| {
        ex.degrade_reason
            == Some(M5CommandLabelMappingDegradeReason::NotationFormCoverageIncomplete)
    });
    let no_clean_missing_triple =
        !labels().any(|ex| ex.is_clean() && !ex.provides_complete_discovery_triple);
    if !(label_kinds_covered
        && discovery_incomplete_degrades
        && label_form_incomplete_degrades
        && no_clean_missing_triple)
    {
        violations.push(
            M5ShortcutNotationRegistriesViolation::CommandDiscoverableOnEveryProfileNotProven,
        );
    }

    // AC3: regression suites fail when a platform surface shows the wrong modifier notation, label mapping, or
    // reserved-key explanation. A mislabeled-notation example and a discovery-incomplete example both
    // degrade, at least one clean notation and one clean label-mapping trace to the registry, no clean
    // notation is unbound, and no clean notation is mislabeled for its host.
    let mislabeled_degrades = notations().any(|ex| {
        ex.degrade_reason == Some(M5ShortcutNotationEntryDegradeReason::NotationMislabeledForHost)
    });
    let bound_notation = notations().any(|ex| ex.is_clean() && ex.bound_to_registry);
    let bound_label = labels().any(|ex| ex.is_clean() && ex.provides_complete_discovery_triple);
    let no_clean_mislabeled = !notations().any(|ex| ex.is_clean() && !ex.notation_matches_host);
    if !(mislabeled_degrades
        && discovery_incomplete_degrades
        && bound_notation
        && bound_label
        && no_clean_unbound
        && no_clean_mislabeled)
    {
        violations
            .push(M5ShortcutNotationRegistriesViolation::WrongNotationOrLabelDetectableNotProven);
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

/// The platform-fit family this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5PlatformFitFamily; 1] = [M5PlatformFitFamily::ShortcutNotation];
