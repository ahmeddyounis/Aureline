//! Frozen M5 contextual-tip-card, migration-bridge-card, sequence-help-strip,
//! why-unavailable-explanation-row, and source-language-fallback component matrix.
//!
//! This module locks Aureline's reusable contextual-teaching, migration, and
//! blocked-action explanation components into one export-safe packet. Every teaching-
//! and help-facing subcomponent M5 claims that still drifts too easily by first-run,
//! guided-tour, command-palette, migration-report, inline-help, or CLI surface — the
//! contextual tip card, the migration bridge card, the sequence-help strip, the
//! why-unavailable explanation row, and the source-language fallback surface — is named
//! once here and constrained by the same command binding, migration mapping class,
//! imported source tool, sequence-help state, blocked-action owner and reason, next safe
//! action, and citation-preserving source-language reference regardless of the surface
//! family that renders it.
//!
//! What this matrix freezes is the stable vocabulary for the *components* themselves: the
//! component families, the tip trigger classes and dismissal states the tip card binds,
//! the migration mapping classes (`exact`, `native`, `bridge`, `shimmed`, `partial`,
//! `unsupported`) and imported source tools the migration bridge card binds, the
//! sequence-help states and step kinds the sequence-help strip binds, the shared
//! command-backing states the tip card and sequence-help strip bind, the blocked-action
//! owners, unavailable reasons, and next safe actions the why-unavailable row binds, the
//! source-language classes and fallback states the source-language fallback surface binds,
//! the deployment lines every component must survive, the non-visual accessibility routes,
//! and the mandatory labels every component must be able to show. It does not re-architect
//! the command descriptor, keybinding resolver, importer outcome, feature availability, or
//! locale fallback contracts that already own those records — it is the shared teaching /
//! migration / explanation contract layered on top of them.
//!
//! The matrix is the single source of truth for whether a claimed M5 onboarding, tour,
//! command-help, migration, or localized-help surface may publish a tip, a migrated-
//! behavior claim, a sequence-help state, a blocked-action explanation, or a source-
//! language fallback claim. Onboarding, migration, command-help, blocked-action, and
//! localized-help consumers all read this packet so one tip card names its command binding
//! and dismissal state, one migration bridge card names how an imported behavior maps
//! (native, bridge, shimmed, partial, or unsupported) and where it came from, one
//! sequence-help strip names its current sequence state, one why-unavailable row names the
//! owner, reason, and next safe action, and one source-language fallback surface names what
//! locale is shown and that its canonical citation is preserved. No M5 lane invents a
//! second teaching grammar or an alternate label for imported behavior, a blocked action,
//! or a source-language fallback state.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5ContextualTeachingComponentVocabularySet`] rather than minted per surface. Raw
//! docs bodies, pasted paths, credentials, and private endpoints stay outside the export
//! boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_contextual_teaching_component_matrix,
    seeded_m5_contextual_teaching_component_matrix_migration_bridge_card_beta_narrowed,
    seeded_m5_contextual_teaching_component_matrix_source_language_fallback_preview_narrowed,
    M5_CONTEXTUAL_TEACHING_COMPONENT_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5ContextualTeachingComponentMatrixPacket`].
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_MATRIX_RECORD_KIND: &str =
    "freeze_the_m5_contextual_tip_card_migration_bridge_card_sequence_help_strip_why_unavailable_explanation_row_and_source_language_fallback_component_matrix";

/// Schema version for M5 contextual-teaching / migration-bridge component-matrix records.
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the contextual-teaching / migration-bridge component boundary
/// schema.
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_SCHEMA_REF: &str =
    "schemas/ui/m5-contextual-teaching-migration-bridge-component-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_DOC_REF: &str =
    "docs/help/m5_contextual_teaching_migration_bridge_component_matrix.md";

/// Repo-relative path of the command-descriptor contract the tip and sequence-help
/// components bind against for stable command IDs.
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_COMMAND_DESCRIPTOR_REF: &str =
    "schemas/commands/command_descriptor.schema.json";

/// Repo-relative path of the importer-outcome contract the migration bridge card binds
/// against.
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_IMPORTER_OUTCOME_REF: &str =
    "schemas/migration/importer_outcome.schema.json";

/// Repo-relative path of the keybinding-resolver contract the sequence-help strip binds
/// against.
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_KEYBINDING_RESOLVER_REF: &str =
    "schemas/commands/keybinding_resolver.schema.json";

/// Repo-relative path of the feature-availability contract the why-unavailable row binds
/// against.
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_FEATURE_AVAILABILITY_REF: &str =
    "schemas/ux/feature_availability_row.schema.json";

/// Repo-relative path of the locale-fallback contract the source-language fallback surface
/// binds against.
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_LOCALE_FALLBACK_REF: &str =
    "schemas/ux/locale_fallback_state.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-contextual-teaching-components";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_ARTIFACT_REF: &str =
    "artifacts/release/m5-contextual-teaching-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_CSV_REF: &str =
    "artifacts/release/m5-contextual-teaching-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_CONTEXTUAL_TEACHING_COMPONENT_REPORT_REF: &str =
    "artifacts/design/m5-contextual-teaching-migration-bridge-component-matrix.md";

/// One of the five governed contextual-teaching / migration-bridge component families this
/// matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContextualTeachingComponentFamily {
    /// A contextual tip card carrying its trigger, command binding, and dismissal state.
    ContextualTipCard,
    /// A migration bridge card carrying its migration mapping class and imported source
    /// tool.
    MigrationBridgeCard,
    /// A sequence-help strip carrying its sequence-help state and command backing.
    SequenceHelpStrip,
    /// A why-unavailable explanation row carrying its blocked-action owner, reason, and
    /// next safe action.
    WhyUnavailableExplanationRow,
    /// A source-language fallback surface carrying its source-language class and fallback
    /// state.
    SourceLanguageFallback,
}

impl M5ContextualTeachingComponentFamily {
    /// Every governed component family, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ContextualTipCard,
        Self::MigrationBridgeCard,
        Self::SequenceHelpStrip,
        Self::WhyUnavailableExplanationRow,
        Self::SourceLanguageFallback,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ContextualTipCard => "contextual_tip_card",
            Self::MigrationBridgeCard => "migration_bridge_card",
            Self::SequenceHelpStrip => "sequence_help_strip",
            Self::WhyUnavailableExplanationRow => "why_unavailable_explanation_row",
            Self::SourceLanguageFallback => "source_language_fallback",
        }
    }

    /// `true` when this family is a contextual tip card and must therefore declare its tip
    /// trigger classes and dismissal states.
    pub const fn is_contextual_tip_card(self) -> bool {
        matches!(self, Self::ContextualTipCard)
    }

    /// `true` when this family is a migration bridge card and must therefore declare its
    /// migration mapping classes and source tools.
    pub const fn is_migration_bridge_card(self) -> bool {
        matches!(self, Self::MigrationBridgeCard)
    }

    /// `true` when this family is a sequence-help strip and must therefore declare its
    /// sequence-help states and step kinds.
    pub const fn is_sequence_help_strip(self) -> bool {
        matches!(self, Self::SequenceHelpStrip)
    }

    /// `true` when this family is a why-unavailable explanation row and must therefore
    /// declare its blocked-action owners, unavailable reasons, and next safe actions.
    pub const fn is_why_unavailable_explanation_row(self) -> bool {
        matches!(self, Self::WhyUnavailableExplanationRow)
    }

    /// `true` when this family is a source-language fallback surface and must therefore
    /// declare its source-language classes and fallback states.
    pub const fn is_source_language_fallback(self) -> bool {
        matches!(self, Self::SourceLanguageFallback)
    }
}

/// Controlled tip trigger class — why a contextual tip card appears, so a tip never leaves
/// its trigger implicit or invents a parallel trigger taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TipTriggerClass {
    /// First encounter with a surface.
    FirstEncounter,
    /// Discovery of a related feature.
    FeatureDiscovery,
    /// Recovery from an error.
    ErrorRecovery,
    /// A mode or profile change.
    ModeChange,
    /// An idle-time hint.
    IdleHint,
    /// A contextual follow-up to a prior action.
    ContextualFollowup,
}

impl M5TipTriggerClass {
    /// Every tip trigger class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FirstEncounter,
        Self::FeatureDiscovery,
        Self::ErrorRecovery,
        Self::ModeChange,
        Self::IdleHint,
        Self::ContextualFollowup,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstEncounter => "first_encounter",
            Self::FeatureDiscovery => "feature_discovery",
            Self::ErrorRecovery => "error_recovery",
            Self::ModeChange => "mode_change",
            Self::IdleHint => "idle_hint",
            Self::ContextualFollowup => "contextual_followup",
        }
    }
}

/// Controlled tip dismissal state — how a contextual tip card can be dismissed, so teaching
/// stays dismissible and never blocks the user or hides its dismissal affordance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TipDismissalState {
    /// Dismissible right now.
    Dismissible,
    /// Already dismissed.
    Dismissed,
    /// Snoozed for later.
    Snoozed,
    /// Persistent until the user acts.
    PersistentUntilActed,
    /// Auto-expired after its window.
    AutoExpired,
    /// Suppressed by a user preference.
    SuppressedByPreference,
}

impl M5TipDismissalState {
    /// Every tip dismissal state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Dismissible,
        Self::Dismissed,
        Self::Snoozed,
        Self::PersistentUntilActed,
        Self::AutoExpired,
        Self::SuppressedByPreference,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dismissible => "dismissible",
            Self::Dismissed => "dismissed",
            Self::Snoozed => "snoozed",
            Self::PersistentUntilActed => "persistent_until_acted",
            Self::AutoExpired => "auto_expired",
            Self::SuppressedByPreference => "suppressed_by_preference",
        }
    }
}

/// Controlled migration mapping class — how a migration bridge card maps an imported
/// behavior onto Aureline. This is the one governed vocabulary every migration surface
/// binds so imported behavior is never overstated. These are the exact acceptance-criteria
/// labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MigrationMappingClass {
    /// An exact one-to-one mapping.
    Exact,
    /// A native Aureline equivalent.
    Native,
    /// A bridge that approximates the imported behavior.
    Bridge,
    /// A shimmed compatibility behavior.
    Shimmed,
    /// A partial mapping missing some behavior.
    Partial,
    /// An unsupported behavior with no mapping.
    Unsupported,
}

impl M5MigrationMappingClass {
    /// Every migration mapping class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Exact,
        Self::Native,
        Self::Bridge,
        Self::Shimmed,
        Self::Partial,
        Self::Unsupported,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Exact => "exact",
            Self::Native => "native",
            Self::Bridge => "bridge",
            Self::Shimmed => "shimmed",
            Self::Partial => "partial",
            Self::Unsupported => "unsupported",
        }
    }
}

/// Controlled imported source tool — where a migration bridge card's imported behavior came
/// from, so the origin of an imported behavior is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SourceToolClass {
    /// A legacy code editor.
    LegacyEditor,
    /// A rival IDE.
    RivalIde,
    /// A modal (vim/emacs-style) editor.
    ModalEditor,
    /// An imported keymap.
    ImportedKeymap,
    /// A migrated workflow configuration.
    MigratedWorkflowConfig,
    /// An unknown source.
    UnknownSource,
}

impl M5SourceToolClass {
    /// Every imported source tool, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LegacyEditor,
        Self::RivalIde,
        Self::ModalEditor,
        Self::ImportedKeymap,
        Self::MigratedWorkflowConfig,
        Self::UnknownSource,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LegacyEditor => "legacy_editor",
            Self::RivalIde => "rival_ide",
            Self::ModalEditor => "modal_editor",
            Self::ImportedKeymap => "imported_keymap",
            Self::MigratedWorkflowConfig => "migrated_workflow_config",
            Self::UnknownSource => "unknown_source",
        }
    }
}

/// Controlled sequence-help state — the state of a keyboard command sequence a sequence-help
/// strip renders, so command-language help stays keyboard-first and never invents an
/// alternate label for a partial or blocked sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SequenceHelpState {
    /// Ready to accept the first key of a sequence.
    Ready,
    /// Awaiting the next key of a multi-key sequence.
    AwaitingNextKey,
    /// A partial match so far.
    PartialMatch,
    /// No binding for the entered keys.
    NoBinding,
    /// A conflicting binding needs resolution.
    ConflictingBinding,
    /// The sequence is disabled in the current context.
    DisabledInContext,
}

impl M5SequenceHelpState {
    /// Every sequence-help state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Ready,
        Self::AwaitingNextKey,
        Self::PartialMatch,
        Self::NoBinding,
        Self::ConflictingBinding,
        Self::DisabledInContext,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::AwaitingNextKey => "awaiting_next_key",
            Self::PartialMatch => "partial_match",
            Self::NoBinding => "no_binding",
            Self::ConflictingBinding => "conflicting_binding",
            Self::DisabledInContext => "disabled_in_context",
        }
    }
}

/// Controlled sequence step kind — the kind of step a sequence-help strip names, so no
/// surface invents an alternate label for a leader key, chord, or terminal action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SequenceStepKind {
    /// A leader key.
    LeaderKey,
    /// A chord.
    Chord,
    /// A prefix argument.
    PrefixArgument,
    /// A motion.
    Motion,
    /// An operator.
    Operator,
    /// A terminal action that completes the sequence.
    TerminalAction,
}

impl M5SequenceStepKind {
    /// Every sequence step kind, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LeaderKey,
        Self::Chord,
        Self::PrefixArgument,
        Self::Motion,
        Self::Operator,
        Self::TerminalAction,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LeaderKey => "leader_key",
            Self::Chord => "chord",
            Self::PrefixArgument => "prefix_argument",
            Self::Motion => "motion",
            Self::Operator => "operator",
            Self::TerminalAction => "terminal_action",
        }
    }
}

/// Controlled command-backing state — how a contextual tip card or sequence-help strip ties
/// to a stable command, declared by both so teaching stays command-backed and never
/// suggests an action it cannot invoke.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CommandBackingState {
    /// Backed by a bound command.
    BoundCommand,
    /// An unbound hint (no keybinding yet).
    UnboundHint,
    /// A deep-link command.
    DeepLinkCommand,
    /// A command-palette entry.
    PaletteEntry,
    /// A keybinding route.
    KeybindingRoute,
    /// No command backing at all.
    NoCommandBacking,
}

impl M5CommandBackingState {
    /// Every command-backing state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BoundCommand,
        Self::UnboundHint,
        Self::DeepLinkCommand,
        Self::PaletteEntry,
        Self::KeybindingRoute,
        Self::NoCommandBacking,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BoundCommand => "bound_command",
            Self::UnboundHint => "unbound_hint",
            Self::DeepLinkCommand => "deep_link_command",
            Self::PaletteEntry => "palette_entry",
            Self::KeybindingRoute => "keybinding_route",
            Self::NoCommandBacking => "no_command_backing",
        }
    }
}

/// Controlled blocked-action owner — who owns or gates a blocked action a why-unavailable
/// row explains, so a blocked action always names its owner. This is one of the governed
/// acceptance-criteria vocabularies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BlockedActionOwner {
    /// A policy owner.
    PolicyOwner,
    /// A workspace administrator.
    WorkspaceAdmin,
    /// A provider service.
    ProviderService,
    /// An upstream dependency.
    UpstreamDependency,
    /// The current user's own scope.
    CurrentUserScope,
    /// An unknown owner.
    UnknownOwner,
}

impl M5BlockedActionOwner {
    /// Every blocked-action owner, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PolicyOwner,
        Self::WorkspaceAdmin,
        Self::ProviderService,
        Self::UpstreamDependency,
        Self::CurrentUserScope,
        Self::UnknownOwner,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyOwner => "policy_owner",
            Self::WorkspaceAdmin => "workspace_admin",
            Self::ProviderService => "provider_service",
            Self::UpstreamDependency => "upstream_dependency",
            Self::CurrentUserScope => "current_user_scope",
            Self::UnknownOwner => "unknown_owner",
        }
    }
}

/// Controlled unavailable reason class — why an action is blocked, so a why-unavailable row
/// always names a reason and never leaves it implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5UnavailableReasonClass {
    /// Blocked by policy.
    PolicyBlocked,
    /// A missing permission.
    MissingPermission,
    /// An unmet precondition.
    UnmetPrecondition,
    /// A feature flag is off.
    FeatureFlagOff,
    /// Unavailable while offline.
    OfflineUnavailable,
    /// An unsupported target.
    UnsupportedTarget,
}

impl M5UnavailableReasonClass {
    /// Every unavailable reason class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PolicyBlocked,
        Self::MissingPermission,
        Self::UnmetPrecondition,
        Self::FeatureFlagOff,
        Self::OfflineUnavailable,
        Self::UnsupportedTarget,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PolicyBlocked => "policy_blocked",
            Self::MissingPermission => "missing_permission",
            Self::UnmetPrecondition => "unmet_precondition",
            Self::FeatureFlagOff => "feature_flag_off",
            Self::OfflineUnavailable => "offline_unavailable",
            Self::UnsupportedTarget => "unsupported_target",
        }
    }
}

/// Controlled next-safe-action class — the safe next step a why-unavailable row offers, so a
/// blocked action always names what the user can safely do next.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NextSafeActionClass {
    /// Request access.
    RequestAccess,
    /// Satisfy the precondition.
    SatisfyPrecondition,
    /// Switch context.
    SwitchContext,
    /// Open settings.
    OpenSettings,
    /// Read the docs.
    ReadDocs,
    /// No safe action is available.
    NoSafeAction,
}

impl M5NextSafeActionClass {
    /// Every next-safe-action class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RequestAccess,
        Self::SatisfyPrecondition,
        Self::SwitchContext,
        Self::OpenSettings,
        Self::ReadDocs,
        Self::NoSafeAction,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RequestAccess => "request_access",
            Self::SatisfyPrecondition => "satisfy_precondition",
            Self::SwitchContext => "switch_context",
            Self::OpenSettings => "open_settings",
            Self::ReadDocs => "read_docs",
            Self::NoSafeAction => "no_safe_action",
        }
    }
}

/// Controlled source-language class — the localization state of the help a source-language
/// fallback surface renders, so localized help never masquerades as authoritative or hides
/// that it is falling back to the source language.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SourceLanguageClass {
    /// Authored directly in the display locale.
    AuthoredLocale,
    /// Human-translated into the display locale.
    TranslatedLocale,
    /// Machine-translated.
    MachineTranslated,
    /// Falling back to the source language.
    FallbackToSource,
    /// A mixed-locale surface.
    MixedLocale,
    /// Untranslated source text.
    UntranslatedSource,
}

impl M5SourceLanguageClass {
    /// Every source-language class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::AuthoredLocale,
        Self::TranslatedLocale,
        Self::MachineTranslated,
        Self::FallbackToSource,
        Self::MixedLocale,
        Self::UntranslatedSource,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoredLocale => "authored_locale",
            Self::TranslatedLocale => "translated_locale",
            Self::MachineTranslated => "machine_translated",
            Self::FallbackToSource => "fallback_to_source",
            Self::MixedLocale => "mixed_locale",
            Self::UntranslatedSource => "untranslated_source",
        }
    }
}

/// Controlled fallback state class — how a source-language fallback surface preserves
/// canonical IDs and citations while showing fallback content, so a fallback never severs
/// the canonical citation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FallbackStateClass {
    /// Localized and current.
    LocalizedCurrent,
    /// Source language shown as fallback.
    SourceLanguageShown,
    /// A partial translation.
    PartialTranslation,
    /// A stale translation.
    StaleTranslation,
    /// A fallback with its canonical citation preserved.
    CitationPreservedFallback,
    /// No localization available.
    NoLocalization,
}

impl M5FallbackStateClass {
    /// Every fallback state class, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::LocalizedCurrent,
        Self::SourceLanguageShown,
        Self::PartialTranslation,
        Self::StaleTranslation,
        Self::CitationPreservedFallback,
        Self::NoLocalization,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalizedCurrent => "localized_current",
            Self::SourceLanguageShown => "source_language_shown",
            Self::PartialTranslation => "partial_translation",
            Self::StaleTranslation => "stale_translation",
            Self::CitationPreservedFallback => "citation_preserved_fallback",
            Self::NoLocalization => "no_localization",
        }
    }
}

/// Claimed M5 onboarding / help surface family that renders / consumes a contextual-teaching
/// component. No component may invent a parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingSurfaceFamily {
    /// The first-run onboarding surface.
    FirstRunOnboarding,
    /// The guided-tour surface.
    GuidedTour,
    /// The command-palette surface.
    CommandPalette,
    /// The migration-report surface.
    MigrationReport,
    /// The inline-help surface.
    InlineHelp,
    /// The CLI help surface.
    CliHelp,
}

impl M5TeachingSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::FirstRunOnboarding,
        Self::GuidedTour,
        Self::CommandPalette,
        Self::MigrationReport,
        Self::InlineHelp,
        Self::CliHelp,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FirstRunOnboarding => "first_run_onboarding",
            Self::GuidedTour => "guided_tour",
            Self::CommandPalette => "command_palette",
            Self::MigrationReport => "migration_report",
            Self::InlineHelp => "inline_help",
            Self::CliHelp => "cli_help",
        }
    }
}

/// Deployment line a component must survive with the same truth, so a component's tip,
/// migration mapping, sequence, blocked-action, or fallback truth never silently narrows or
/// widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingDeploymentLine {
    /// The local open-source line.
    LocalOss,
    /// The self-hosted line.
    SelfHosted,
    /// The managed line.
    Managed,
    /// The air-gapped line.
    AirGapped,
    /// The mirror / offline line.
    MirrorOffline,
}

impl M5TeachingDeploymentLine {
    /// Every deployment line, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::LocalOss,
        Self::SelfHosted,
        Self::Managed,
        Self::AirGapped,
        Self::MirrorOffline,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalOss => "local_oss",
            Self::SelfHosted => "self_hosted",
            Self::Managed => "managed",
            Self::AirGapped => "air_gapped",
            Self::MirrorOffline => "mirror_offline",
        }
    }
}

/// Subsystem that consumes a component's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingConsumerSurface {
    /// The onboarding UI.
    OnboardingUi,
    /// The tour-overlay UI.
    TourOverlayUi,
    /// The command-palette UI.
    CommandPaletteUi,
    /// The migration-report UI.
    MigrationReportUi,
    /// The inline-tip UI.
    InlineTipUi,
    /// The help-panel UI.
    HelpPanelUi,
    /// The CLI help surface.
    CliHelp,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5TeachingConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::OnboardingUi,
        Self::TourOverlayUi,
        Self::CommandPaletteUi,
        Self::MigrationReportUi,
        Self::InlineTipUi,
        Self::HelpPanelUi,
        Self::CliHelp,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OnboardingUi => "onboarding_ui",
            Self::TourOverlayUi => "tour_overlay_ui",
            Self::CommandPaletteUi => "command_palette_ui",
            Self::MigrationReportUi => "migration_report_ui",
            Self::InlineTipUi => "inline_tip_ui",
            Self::HelpPanelUi => "help_panel_ui",
            Self::CliHelp => "cli_help",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every component must offer so no teaching truth is
/// hover-only, pointer-only, or visually encoded alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader.
    ScreenReaderAnnounced,
    /// Reachable without pointer hover.
    NonHoverReachable,
    /// Pointer interaction is optional, never required.
    PointerOptional,
    /// Legible in high-contrast / reduced-motion modes.
    HighContrastSafe,
    /// Present in the support / export packet.
    SupportExportable,
}

impl M5TeachingAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::NonHoverReachable,
        Self::PointerOptional,
        Self::HighContrastSafe,
        Self::SupportExportable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::NonHoverReachable => "non_hover_reachable",
            Self::PointerOptional => "pointer_optional",
            Self::HighContrastSafe => "high_contrast_safe",
            Self::SupportExportable => "support_exportable",
        }
    }
}

/// Mandatory label a claimed contextual-teaching component must be able to show. The first
/// three are hard requirements on every component; the remaining three close the
/// acceptance-criteria ambiguity about command binding, migration mapping / source language,
/// and blocked-action owner / reason / next action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingRequiredLabel {
    /// The component's stable identity / what it represents.
    Identity,
    /// The component's current typed state.
    State,
    /// The non-visual keyboard route to the component.
    KeyboardRoute,
    /// The stable command binding behind the component.
    CommandBinding,
    /// The migration mapping class and source language behind the component.
    MigrationAndSourceLanguage,
    /// The blocked-action owner, reason, and next safe action behind the component.
    OwnerReasonAndNextAction,
}

impl M5TeachingRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::State,
        Self::KeyboardRoute,
        Self::CommandBinding,
        Self::MigrationAndSourceLanguage,
        Self::OwnerReasonAndNextAction,
    ];

    /// The three labels every claimed component must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::State, Self::KeyboardRoute];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::State => "state",
            Self::KeyboardRoute => "keyboard_route",
            Self::CommandBinding => "command_binding",
            Self::MigrationAndSourceLanguage => "migration_and_source_language",
            Self::OwnerReasonAndNextAction => "owner_reason_and_next_action",
        }
    }
}

/// Qualification class for an M5 contextual-teaching component row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingQualificationClass {
    /// Component qualifies for the Stable claim.
    Stable,
    /// Component is narrowed to Beta.
    Beta,
    /// Component is narrowed to Preview.
    Preview,
    /// Component is experimental and not claimed.
    Experimental,
    /// Component is unavailable on this build.
    Unavailable,
    /// Component is held pending upstream resolution.
    Held,
}

impl M5TeachingQualificationClass {
    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Preview => "preview",
            Self::Experimental => "experimental",
            Self::Unavailable => "unavailable",
            Self::Held => "held",
        }
    }

    /// Whether the component may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a contextual-teaching component below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TeachingDowngradeTrigger {
    /// A tip card left its command binding unstated.
    TipCommandBindingUnstated,
    /// A migration card left its mapping class unstated.
    MigrationMappingUnstated,
    /// A migration card left its imported source tool unstated.
    SourceToolUnstated,
    /// A sequence-help strip left its sequence state unstated.
    SequenceHelpStateUnstated,
    /// A tip card or sequence-help strip hid its command backing.
    CommandBackingHidden,
    /// A why-unavailable row left its blocked-action owner unstated.
    BlockedActionOwnerUnstated,
    /// A why-unavailable row left its reason unstated.
    UnavailableReasonUnstated,
    /// A why-unavailable row omitted its next safe action.
    NextSafeActionMissing,
    /// A source-language fallback surface left its fallback state unstated.
    SourceLanguageFallbackUnstated,
    /// A source-language fallback surface severed its canonical citation.
    CitationSevered,
    /// A surface invented an alternate label for a governed state.
    AlternateStateLabelInvented,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5TeachingDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::TipCommandBindingUnstated,
        Self::MigrationMappingUnstated,
        Self::SourceToolUnstated,
        Self::SequenceHelpStateUnstated,
        Self::CommandBackingHidden,
        Self::BlockedActionOwnerUnstated,
        Self::UnavailableReasonUnstated,
        Self::NextSafeActionMissing,
        Self::SourceLanguageFallbackUnstated,
        Self::CitationSevered,
        Self::AlternateStateLabelInvented,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TipCommandBindingUnstated => "tip_command_binding_unstated",
            Self::MigrationMappingUnstated => "migration_mapping_unstated",
            Self::SourceToolUnstated => "source_tool_unstated",
            Self::SequenceHelpStateUnstated => "sequence_help_state_unstated",
            Self::CommandBackingHidden => "command_backing_hidden",
            Self::BlockedActionOwnerUnstated => "blocked_action_owner_unstated",
            Self::UnavailableReasonUnstated => "unavailable_reason_unstated",
            Self::NextSafeActionMissing => "next_safe_action_missing",
            Self::SourceLanguageFallbackUnstated => "source_language_fallback_unstated",
            Self::CitationSevered => "citation_severed",
            Self::AlternateStateLabelInvented => "alternate_state_label_invented",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed contextual-teaching component family bound to the
/// surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTeachingComponentRow {
    /// Governed component family.
    pub component_family: M5ContextualTeachingComponentFamily,
    /// Qualification class earned by this component.
    pub qualification: M5TeachingQualificationClass,
    /// Owner role accountable for keeping this component governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 onboarding / help surface families that render / consume this component.
    pub surface_families: Vec<M5TeachingSurfaceFamily>,
    /// Deployment lines this component keeps the same truth across.
    pub deployment_lines: Vec<M5TeachingDeploymentLine>,
    /// Mandatory labels this component must be able to show (must include the three
    /// [`M5TeachingRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5TeachingRequiredLabel>,
    /// Tip trigger classes this component names (contextual-tip-card only).
    pub tip_trigger_classes: Vec<M5TipTriggerClass>,
    /// Tip dismissal states this component names (contextual-tip-card only).
    pub tip_dismissal_states: Vec<M5TipDismissalState>,
    /// Migration mapping classes this component names (migration-bridge-card only).
    pub migration_mapping_classes: Vec<M5MigrationMappingClass>,
    /// Imported source tools this component names (migration-bridge-card only).
    pub source_tool_classes: Vec<M5SourceToolClass>,
    /// Sequence-help states this component names (sequence-help-strip only).
    pub sequence_help_states: Vec<M5SequenceHelpState>,
    /// Sequence step kinds this component names (sequence-help-strip only).
    pub sequence_step_kinds: Vec<M5SequenceStepKind>,
    /// Command-backing states this component names (contextual-tip-card and
    /// sequence-help-strip).
    pub command_backing_states: Vec<M5CommandBackingState>,
    /// Blocked-action owners this component names (why-unavailable-explanation-row only).
    pub blocked_action_owners: Vec<M5BlockedActionOwner>,
    /// Unavailable reason classes this component names (why-unavailable-explanation-row
    /// only).
    pub unavailable_reason_classes: Vec<M5UnavailableReasonClass>,
    /// Next-safe-action classes this component names (why-unavailable-explanation-row
    /// only).
    pub next_safe_action_classes: Vec<M5NextSafeActionClass>,
    /// Source-language classes this component names (source-language-fallback only).
    pub source_language_classes: Vec<M5SourceLanguageClass>,
    /// Fallback state classes this component names (source-language-fallback only).
    pub fallback_state_classes: Vec<M5FallbackStateClass>,
    /// Non-visual accessibility routes this component offers.
    pub accessibility_routes: Vec<M5TeachingAccessibilityRoute>,
    /// Subsystems that consume this component's projection.
    pub consumer_surfaces: Vec<M5TeachingConsumerSurface>,
    /// Downgrade triggers that apply to this component.
    pub downgrade_triggers: Vec<M5TeachingDowngradeTrigger>,
    /// Proof packet refs that keep this component current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this component.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this component never masks its command binding or migration mapping.
    /// MUST be `false`.
    pub masks_command_binding_or_migration_mapping: bool,
    /// Hard invariant: this component never hides a blocked-action owner or reason. MUST be
    /// `false`.
    pub hides_blocked_action_owner_or_reason: bool,
    /// Hard invariant: this component never invents an alternate label for a governed
    /// state. MUST be `false`.
    pub invents_alternate_state_label: bool,
    /// Hard invariant: this component never severs a source-language citation. MUST be
    /// `false`.
    pub severs_source_language_citation: bool,
}

impl M5ContextualTeachingComponentRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5TeachingRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5TeachingRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.masks_command_binding_or_migration_mapping
            && !self.hides_blocked_action_owner_or_reason
            && !self.invents_alternate_state_label
            && !self.severs_source_language_citation
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTeachingComponentVocabularySet {
    /// Component-family tokens.
    pub component_families: Vec<String>,
    /// Tip-trigger-class tokens.
    pub tip_trigger_classes: Vec<String>,
    /// Tip-dismissal-state tokens.
    pub tip_dismissal_states: Vec<String>,
    /// Migration-mapping-class tokens.
    pub migration_mapping_classes: Vec<String>,
    /// Source-tool-class tokens.
    pub source_tool_classes: Vec<String>,
    /// Sequence-help-state tokens.
    pub sequence_help_states: Vec<String>,
    /// Sequence-step-kind tokens.
    pub sequence_step_kinds: Vec<String>,
    /// Command-backing-state tokens.
    pub command_backing_states: Vec<String>,
    /// Blocked-action-owner tokens.
    pub blocked_action_owners: Vec<String>,
    /// Unavailable-reason-class tokens.
    pub unavailable_reason_classes: Vec<String>,
    /// Next-safe-action-class tokens.
    pub next_safe_action_classes: Vec<String>,
    /// Source-language-class tokens.
    pub source_language_classes: Vec<String>,
    /// Fallback-state-class tokens.
    pub fallback_state_classes: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
}

impl M5ContextualTeachingComponentVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            component_families: tokens(&M5ContextualTeachingComponentFamily::ALL, |v| v.as_str()),
            tip_trigger_classes: tokens(&M5TipTriggerClass::ALL, |v| v.as_str()),
            tip_dismissal_states: tokens(&M5TipDismissalState::ALL, |v| v.as_str()),
            migration_mapping_classes: tokens(&M5MigrationMappingClass::ALL, |v| v.as_str()),
            source_tool_classes: tokens(&M5SourceToolClass::ALL, |v| v.as_str()),
            sequence_help_states: tokens(&M5SequenceHelpState::ALL, |v| v.as_str()),
            sequence_step_kinds: tokens(&M5SequenceStepKind::ALL, |v| v.as_str()),
            command_backing_states: tokens(&M5CommandBackingState::ALL, |v| v.as_str()),
            blocked_action_owners: tokens(&M5BlockedActionOwner::ALL, |v| v.as_str()),
            unavailable_reason_classes: tokens(&M5UnavailableReasonClass::ALL, |v| v.as_str()),
            next_safe_action_classes: tokens(&M5NextSafeActionClass::ALL, |v| v.as_str()),
            source_language_classes: tokens(&M5SourceLanguageClass::ALL, |v| v.as_str()),
            fallback_state_classes: tokens(&M5FallbackStateClass::ALL, |v| v.as_str()),
            surface_families: tokens(&M5TeachingSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5TeachingDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5TeachingConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5TeachingAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5TeachingRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5ContextualTeachingComponentGovernanceReview {
    /// The contextual tip card shows its command binding and dismissal state.
    pub tip_card_shows_command_binding_and_dismissal: bool,
    /// The migration bridge card shows its mapping class and imported source tool.
    pub migration_card_shows_mapping_class_and_source_tool: bool,
    /// The sequence-help strip shows its help state and command backing.
    pub sequence_strip_shows_help_state_and_command_backing: bool,
    /// The why-unavailable row shows its owner, reason, and next safe action.
    pub unavailable_row_shows_owner_reason_and_next_action: bool,
    /// The source-language fallback surface shows its source language and preserves its
    /// citation.
    pub fallback_shows_source_language_and_citation_preserved: bool,
    /// No surface invents an alternate label for a governed state.
    pub no_surface_invents_alternate_state_label: bool,
    /// The `exact` / `native` / `bridge` / `shimmed` / `partial` / `unsupported` mapping
    /// classes are named once.
    pub migration_mapping_vocabulary_named_once: bool,
    /// Blocked-action owner and sequence-help states are each named once.
    pub blocked_action_owner_and_sequence_help_named_once: bool,
    /// The next safe action is always explicit for a blocked action.
    pub next_safe_action_always_explicit: bool,
    /// The command binding is always explicit for a tip or sequence.
    pub command_binding_always_explicit: bool,
    /// The dismissal state is always explicit for a tip.
    pub dismissal_state_always_explicit: bool,
    /// A source-language citation is never severed.
    pub source_language_citation_never_severed: bool,
    /// Every component keeps the same truth across every deployment line.
    pub every_component_declares_deployment_lines: bool,
    /// Every component declares a non-visual accessibility route.
    pub every_component_declares_accessibility_route: bool,
    /// Later M5 rows cannot invent parallel teaching vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTeachingComponentConsumerProjection {
    /// Onboarding surfaces consume the shared tip-card vocabulary.
    pub onboarding_surfaces_consume_tip_vocabulary: bool,
    /// Migration surfaces consume the migration-mapping vocabulary.
    pub migration_surfaces_consume_mapping_vocabulary: bool,
    /// Command-help surfaces consume the sequence-help vocabulary.
    pub command_help_surfaces_consume_sequence_vocabulary: bool,
    /// Blocked-action surfaces consume the owner / reason / next-action vocabulary.
    pub blocked_action_surfaces_consume_owner_reason_vocabulary: bool,
    /// Localized-help surfaces consume the source-language fallback vocabulary.
    pub localized_help_surfaces_consume_fallback_vocabulary: bool,
    /// Support / export reads a single canonical teaching source.
    pub support_export_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTeachingComponentProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the component.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the contextual-teaching component lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTeachingComponentReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting teaching-component audit for the lane.
    pub teaching_component_audit_ref: String,
    /// True when support/export parity is required for every component.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every component.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5ContextualTeachingComponentMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ContextualTeachingComponentMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ContextualTeachingComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ContextualTeachingComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ContextualTeachingComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ContextualTeachingComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ContextualTeachingComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ContextualTeachingComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 contextual-teaching component matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ContextualTeachingComponentMatrixPacket {
    /// Record kind; must equal [`M5_CONTEXTUAL_TEACHING_COMPONENT_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal
    /// [`M5_CONTEXTUAL_TEACHING_COMPONENT_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Component rows.
    pub component_rows: Vec<M5ContextualTeachingComponentRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5ContextualTeachingComponentVocabularySet,
    /// Governance-review block.
    pub governance_review: M5ContextualTeachingComponentGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5ContextualTeachingComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5ContextualTeachingComponentProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5ContextualTeachingComponentReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ContextualTeachingComponentMatrixPacket {
    /// Builds an M5 contextual-teaching component matrix packet from stable-lane input.
    pub fn new(input: M5ContextualTeachingComponentMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_CONTEXTUAL_TEACHING_COMPONENT_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_CONTEXTUAL_TEACHING_COMPONENT_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            component_rows: input.component_rows,
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

    /// Validates the M5 contextual-teaching component matrix invariants.
    pub fn validate(&self) -> Vec<M5ContextualTeachingComponentMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CONTEXTUAL_TEACHING_COMPONENT_MATRIX_RECORD_KIND {
            violations.push(M5ContextualTeachingComponentMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CONTEXTUAL_TEACHING_COMPONENT_MATRIX_SCHEMA_VERSION {
            violations.push(M5ContextualTeachingComponentMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ContextualTeachingComponentMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_component_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 contextual-teaching component matrix packet serializes"),
        ) {
            violations.push(M5ContextualTeachingComponentMatrixViolation::RawMaterialInExport);
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
            .expect("m5 contextual-teaching component matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed component.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "component_family,qualification,owner,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.component_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                row.component_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.surface_families, |v| v.as_str()),
                join_tokens(&row.deployment_lines, |v| v.as_str()),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_components = self
            .component_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Contextual-Tip-Card, Migration-Bridge-Card, Sequence-Help-Strip, Why-Unavailable-Explanation-Row, and Source-Language-Fallback Component Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Component families: {} ({} stable)\n",
            self.component_rows.len(),
            stable_components
        ));
        out.push_str(&format!(
            "- Migration mapping classes: {}\n",
            self.vocabulary_set.migration_mapping_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Sequence-help states: {}\n",
            self.vocabulary_set.sequence_help_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Component families\n\n");
        for row in &self.component_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.component_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Accessibility routes: {}\n",
                row.accessibility_routes
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 contextual-teaching matrix export.
#[derive(Debug)]
pub enum M5ContextualTeachingComponentMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ContextualTeachingComponentMatrixViolation>),
}

impl fmt::Display for M5ContextualTeachingComponentMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 contextual-teaching component matrix export parse failed: {error}"
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
                    "m5 contextual-teaching component matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5ContextualTeachingComponentMatrixArtifactError {}

/// Validation failures emitted by [`M5ContextualTeachingComponentMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ContextualTeachingComponentMatrixViolation {
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
    /// A required governed component family is missing from the matrix.
    RequiredComponentMissing,
    /// A component row is incomplete.
    ComponentRowIncomplete,
    /// A component row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A contextual-tip-card component declares no tip trigger classes.
    TipTriggerClassMissing,
    /// A contextual-tip-card component declares no tip dismissal states.
    TipDismissalStateMissing,
    /// A migration-bridge-card component declares no migration mapping classes.
    MigrationMappingClassMissing,
    /// A migration-bridge-card component declares no imported source tools.
    SourceToolClassMissing,
    /// A sequence-help-strip component declares no sequence-help states.
    SequenceHelpStateMissing,
    /// A sequence-help-strip component declares no sequence step kinds.
    SequenceStepKindMissing,
    /// A contextual-tip-card or sequence-help-strip component declares no command-backing
    /// states.
    CommandBackingStateMissing,
    /// A why-unavailable-explanation-row component declares no blocked-action owners.
    BlockedActionOwnerMissing,
    /// A why-unavailable-explanation-row component declares no unavailable reasons.
    UnavailableReasonClassMissing,
    /// A why-unavailable-explanation-row component declares no next safe actions.
    NextSafeActionClassMissing,
    /// A source-language-fallback component declares no source-language classes.
    SourceLanguageClassMissing,
    /// A source-language-fallback component declares no fallback states.
    FallbackStateClassMissing,
    /// A component declares no surface families.
    SurfaceFamilyMissing,
    /// A component declares no deployment lines.
    DeploymentLineMissing,
    /// A component declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A component declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A component declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A component claiming Stable is missing required proof packet refs.
    StableComponentMissingProof,
    /// A component violates a hard invariant (masked command binding / migration mapping,
    /// hidden blocked-action owner or reason, invented alternate state label, or severed
    /// source-language citation).
    ComponentInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5ContextualTeachingComponentMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredComponentMissing => "required_component_missing",
            Self::ComponentRowIncomplete => "component_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::TipTriggerClassMissing => "tip_trigger_class_missing",
            Self::TipDismissalStateMissing => "tip_dismissal_state_missing",
            Self::MigrationMappingClassMissing => "migration_mapping_class_missing",
            Self::SourceToolClassMissing => "source_tool_class_missing",
            Self::SequenceHelpStateMissing => "sequence_help_state_missing",
            Self::SequenceStepKindMissing => "sequence_step_kind_missing",
            Self::CommandBackingStateMissing => "command_backing_state_missing",
            Self::BlockedActionOwnerMissing => "blocked_action_owner_missing",
            Self::UnavailableReasonClassMissing => "unavailable_reason_class_missing",
            Self::NextSafeActionClassMissing => "next_safe_action_class_missing",
            Self::SourceLanguageClassMissing => "source_language_class_missing",
            Self::FallbackStateClassMissing => "fallback_state_class_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableComponentMissingProof => "stable_component_missing_proof",
            Self::ComponentInvariantViolated => "component_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 contextual-teaching matrix export.
pub fn current_stable_m5_contextual_teaching_component_matrix_export() -> Result<
    M5ContextualTeachingComponentMatrixPacket,
    M5ContextualTeachingComponentMatrixArtifactError,
> {
    let packet: M5ContextualTeachingComponentMatrixPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/release/m5-contextual-teaching-proof/support_export.json"
        )))
        .map_err(M5ContextualTeachingComponentMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ContextualTeachingComponentMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ContextualTeachingComponentMatrixPacket,
    violations: &mut Vec<M5ContextualTeachingComponentMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_CONTEXTUAL_TEACHING_COMPONENT_SCHEMA_REF,
        M5_CONTEXTUAL_TEACHING_COMPONENT_DOC_REF,
        M5_CONTEXTUAL_TEACHING_COMPONENT_COMMAND_DESCRIPTOR_REF,
        M5_CONTEXTUAL_TEACHING_COMPONENT_IMPORTER_OUTCOME_REF,
        M5_CONTEXTUAL_TEACHING_COMPONENT_KEYBINDING_RESOLVER_REF,
        M5_CONTEXTUAL_TEACHING_COMPONENT_FEATURE_AVAILABILITY_REF,
        M5_CONTEXTUAL_TEACHING_COMPONENT_LOCALE_FALLBACK_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5ContextualTeachingComponentMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5ContextualTeachingComponentMatrixPacket,
    violations: &mut Vec<M5ContextualTeachingComponentMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5ContextualTeachingComponentMatrixViolation::VocabularySetDrift);
    }
}

fn validate_component_rows(
    packet: &M5ContextualTeachingComponentMatrixPacket,
    violations: &mut Vec<M5ContextualTeachingComponentMatrixViolation>,
) {
    let present: BTreeSet<M5ContextualTeachingComponentFamily> = packet
        .component_rows
        .iter()
        .map(|row| row.component_family)
        .collect();
    for required in M5ContextualTeachingComponentFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5ContextualTeachingComponentMatrixViolation::RequiredComponentMissing);
            return;
        }
    }

    for row in &packet.component_rows {
        let family = row.component_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5ContextualTeachingComponentMatrixViolation::ComponentRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5ContextualTeachingComponentMatrixViolation::MandatoryLabelMissing);
        }
        if family.is_contextual_tip_card() && row.tip_trigger_classes.is_empty() {
            violations.push(M5ContextualTeachingComponentMatrixViolation::TipTriggerClassMissing);
        }
        if family.is_contextual_tip_card() && row.tip_dismissal_states.is_empty() {
            violations.push(M5ContextualTeachingComponentMatrixViolation::TipDismissalStateMissing);
        }
        if family.is_migration_bridge_card() && row.migration_mapping_classes.is_empty() {
            violations
                .push(M5ContextualTeachingComponentMatrixViolation::MigrationMappingClassMissing);
        }
        if family.is_migration_bridge_card() && row.source_tool_classes.is_empty() {
            violations.push(M5ContextualTeachingComponentMatrixViolation::SourceToolClassMissing);
        }
        if family.is_sequence_help_strip() && row.sequence_help_states.is_empty() {
            violations.push(M5ContextualTeachingComponentMatrixViolation::SequenceHelpStateMissing);
        }
        if family.is_sequence_help_strip() && row.sequence_step_kinds.is_empty() {
            violations.push(M5ContextualTeachingComponentMatrixViolation::SequenceStepKindMissing);
        }
        // Command-backing state is shared by the contextual-tip-card and the
        // sequence-help-strip.
        if (family.is_contextual_tip_card() || family.is_sequence_help_strip())
            && row.command_backing_states.is_empty()
        {
            violations
                .push(M5ContextualTeachingComponentMatrixViolation::CommandBackingStateMissing);
        }
        if family.is_why_unavailable_explanation_row() && row.blocked_action_owners.is_empty() {
            violations
                .push(M5ContextualTeachingComponentMatrixViolation::BlockedActionOwnerMissing);
        }
        if family.is_why_unavailable_explanation_row() && row.unavailable_reason_classes.is_empty()
        {
            violations
                .push(M5ContextualTeachingComponentMatrixViolation::UnavailableReasonClassMissing);
        }
        if family.is_why_unavailable_explanation_row() && row.next_safe_action_classes.is_empty() {
            violations
                .push(M5ContextualTeachingComponentMatrixViolation::NextSafeActionClassMissing);
        }
        if family.is_source_language_fallback() && row.source_language_classes.is_empty() {
            violations
                .push(M5ContextualTeachingComponentMatrixViolation::SourceLanguageClassMissing);
        }
        if family.is_source_language_fallback() && row.fallback_state_classes.is_empty() {
            violations
                .push(M5ContextualTeachingComponentMatrixViolation::FallbackStateClassMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5ContextualTeachingComponentMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5ContextualTeachingComponentMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations
                .push(M5ContextualTeachingComponentMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5ContextualTeachingComponentMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5ContextualTeachingComponentMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations
                .push(M5ContextualTeachingComponentMatrixViolation::StableComponentMissingProof);
        }
        if !row.honours_invariants() {
            violations
                .push(M5ContextualTeachingComponentMatrixViolation::ComponentInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5ContextualTeachingComponentMatrixPacket,
    violations: &mut Vec<M5ContextualTeachingComponentMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.tip_card_shows_command_binding_and_dismissal,
        review.migration_card_shows_mapping_class_and_source_tool,
        review.sequence_strip_shows_help_state_and_command_backing,
        review.unavailable_row_shows_owner_reason_and_next_action,
        review.fallback_shows_source_language_and_citation_preserved,
        review.no_surface_invents_alternate_state_label,
        review.migration_mapping_vocabulary_named_once,
        review.blocked_action_owner_and_sequence_help_named_once,
        review.next_safe_action_always_explicit,
        review.command_binding_always_explicit,
        review.dismissal_state_always_explicit,
        review.source_language_citation_never_severed,
        review.every_component_declares_deployment_lines,
        review.every_component_declares_accessibility_route,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations
                .push(M5ContextualTeachingComponentMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5ContextualTeachingComponentMatrixPacket,
    violations: &mut Vec<M5ContextualTeachingComponentMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.onboarding_surfaces_consume_tip_vocabulary,
        projection.migration_surfaces_consume_mapping_vocabulary,
        projection.command_help_surfaces_consume_sequence_vocabulary,
        projection.blocked_action_surfaces_consume_owner_reason_vocabulary,
        projection.localized_help_surfaces_consume_fallback_vocabulary,
        projection.support_export_reads_single_source,
    ] {
        if !ok {
            violations
                .push(M5ContextualTeachingComponentMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5ContextualTeachingComponentMatrixPacket,
    violations: &mut Vec<M5ContextualTeachingComponentMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5ContextualTeachingComponentMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5ContextualTeachingComponentMatrixPacket,
    violations: &mut Vec<M5ContextualTeachingComponentMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.teaching_component_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5ContextualTeachingComponentMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
