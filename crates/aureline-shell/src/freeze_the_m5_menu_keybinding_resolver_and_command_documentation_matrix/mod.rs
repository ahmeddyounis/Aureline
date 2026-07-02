//! Frozen M5 menu-affordance, keybinding-resolver, and command-documentation
//! matrix.
//!
//! This module locks Aureline's canonical M5 last-mile command-discovery
//! surfaces into one export-safe packet. Every discoverability primitive M5
//! claims — menu items, menu groups, context menus, command bars, keybinding
//! resolver layers, conflict review sheets, import-bridge rows, disabled-command
//! explainers, leader/sequence help overlays, and command-documentation/detail
//! surfaces — projects from one canonical command record. No surface may invent
//! a second naming system, widen authority, hide a disabled-state reason, or drop
//! preview/approval/lifecycle truth.
//!
//! The canonical command record is the one already frozen by
//! [`crate::m5_command_registry`]; this matrix re-exports its feature-family,
//! lifecycle-label, preview-class, disabled-reason-mode, and discovery-channel
//! vocabulary rather than minting parallel terms. What this matrix adds is the
//! stable vocabulary for the *discoverability* surfaces themselves: the
//! command-surface families, the shortcut-source classes and their precedence,
//! the conflict reasons, the import-translation states, the stale-target
//! invalidation states, the why-unavailable explanation classes, the mandatory
//! per-surface labels, and the cross-modality parity surfaces.
//!
//! The matrix is the single source of truth for whether a claimed M5
//! discoverability surface may publish a menu, keybinding, or command-doc claim.
//! Menus, context menus, command bars, keybinding inspectors, leader/sequence
//! help, and command-documentation surfaces all consume this packet so the same
//! action keeps the same label, the same shortcut truth, the same disabled-state
//! explanation, and the same authority posture regardless of which surface
//! reaches it.
//!
//! The controlled vocabularies are frozen in one self-describing
//! [`M5DiscoverabilityVocabularySet`] rather than minted per surface. Raw URLs,
//! raw local paths, raw usernames, raw hostnames, tokens, raw diagnostics,
//! private endpoints, credentials, and user text bodies stay outside the support
//! boundary.
//!
//! The boundary schema is
//! [`schemas/commands/m5-discoverability-affordances.schema.json`](../../../../schemas/commands/m5-discoverability-affordances.schema.json)
//! and the contract doc is
//! [`docs/commands/m5_discoverability_affordances_contract.md`](../../../../docs/commands/m5_discoverability_affordances_contract.md).
//! The protected fixture directory is
//! [`fixtures/commands/m5-discoverability-affordances/`](../../../../fixtures/commands/m5-discoverability-affordances/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_discoverability_matrix,
    seeded_m5_discoverability_matrix_imported_keymap_approximated_narrowed,
    seeded_m5_discoverability_matrix_leader_sequence_help_preview_narrowed,
    M5_DISCOVERABILITY_MATRIX_PACKET_ID,
};

// The canonical command record and its vocabulary are frozen once, in the M5
// command registry. This matrix reuses them verbatim so no discoverability
// surface invents a parallel naming system.
pub use crate::m5_command_registry::{
    M5DisabledReasonMode, M5DiscoveryChannel, M5FeatureFamily, M5LifecycleLabel, M5PreviewClass,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5DiscoverabilityMatrixPacket`].
pub const M5_DISCOVERABILITY_MATRIX_RECORD_KIND: &str =
    "freeze_m5_menu_keybinding_resolver_and_command_documentation_matrix";

/// Schema version for M5 discoverability-matrix records.
pub const M5_DISCOVERABILITY_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the discoverability boundary schema.
pub const M5_DISCOVERABILITY_SCHEMA_REF: &str =
    "schemas/commands/m5-discoverability-affordances.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_DISCOVERABILITY_DOC_REF: &str =
    "docs/commands/m5_discoverability_affordances_contract.md";

/// Repo-relative path of the canonical command-descriptor schema every surface
/// projects from.
pub const M5_DISCOVERABILITY_COMMAND_DESCRIPTOR_REF: &str =
    "schemas/commands/command_descriptor.schema.json";

/// Repo-relative path of the keybinding-resolver schema this matrix governs.
pub const M5_DISCOVERABILITY_KEYBINDING_RESOLVER_REF: &str =
    "schemas/commands/keybinding_resolver.schema.json";

/// Repo-relative path of the menu-item schema this matrix governs.
pub const M5_DISCOVERABILITY_MENU_ITEM_REF: &str = "schemas/commands/menu_item.schema.json";

/// Repo-relative path of the leader-overlay schema this matrix governs.
pub const M5_DISCOVERABILITY_LEADER_OVERLAY_REF: &str =
    "schemas/commands/leader_overlay.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_DISCOVERABILITY_FIXTURE_DIR: &str = "fixtures/commands/m5-discoverability-affordances";

/// Repo-relative path of the checked support-export artifact.
pub const M5_DISCOVERABILITY_ARTIFACT_REF: &str =
    "artifacts/release/m5-discoverability-affordances-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_DISCOVERABILITY_CSV_REF: &str =
    "artifacts/release/m5-discoverability-affordances-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const M5_DISCOVERABILITY_REPORT_REF: &str =
    "artifacts/commands/m5-discoverability-affordances.md";

/// One of the ten governed command-surface families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CommandSurfaceFamily {
    /// A single application/menu-bar menu item.
    MenuItem,
    /// A named group of menu items (a section / submenu).
    MenuGroup,
    /// A context (right-click / long-press) menu.
    ContextMenu,
    /// A command bar / palette-adjacent action bar.
    CommandBar,
    /// A keybinding resolver layer (default/user/extension/imported/workspace).
    KeybindingResolverLayer,
    /// A keybinding conflict review sheet (winners/losers).
    ConflictReviewSheet,
    /// An import-bridge row translating a foreign keymap binding.
    ImportBridgeRow,
    /// A disabled-command explainer (why-unavailable packet).
    DisabledCommandExplainer,
    /// A leader / sequence help overlay.
    LeaderSequenceHelp,
    /// A command-documentation / command-detail surface.
    CommandDocumentationSurface,
}

impl M5CommandSurfaceFamily {
    /// Every governed surface family, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::MenuItem,
        Self::MenuGroup,
        Self::ContextMenu,
        Self::CommandBar,
        Self::KeybindingResolverLayer,
        Self::ConflictReviewSheet,
        Self::ImportBridgeRow,
        Self::DisabledCommandExplainer,
        Self::LeaderSequenceHelp,
        Self::CommandDocumentationSurface,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MenuItem => "menu_item",
            Self::MenuGroup => "menu_group",
            Self::ContextMenu => "context_menu",
            Self::CommandBar => "command_bar",
            Self::KeybindingResolverLayer => "keybinding_resolver_layer",
            Self::ConflictReviewSheet => "conflict_review_sheet",
            Self::ImportBridgeRow => "import_bridge_row",
            Self::DisabledCommandExplainer => "disabled_command_explainer",
            Self::LeaderSequenceHelp => "leader_sequence_help",
            Self::CommandDocumentationSurface => "command_documentation_surface",
        }
    }

    /// `true` when this family resolves or explains keyboard shortcuts and must
    /// therefore declare its shortcut-source classes.
    pub const fn resolves_shortcuts(self) -> bool {
        matches!(
            self,
            Self::KeybindingResolverLayer | Self::ConflictReviewSheet | Self::LeaderSequenceHelp
        )
    }

    /// `true` when this family reviews shortcut conflicts and must therefore
    /// declare its conflict reasons.
    pub const fn reviews_conflicts(self) -> bool {
        matches!(
            self,
            Self::KeybindingResolverLayer | Self::ConflictReviewSheet
        )
    }

    /// `true` when this family translates an imported keymap and must therefore
    /// declare its import-translation states.
    pub const fn translates_imports(self) -> bool {
        matches!(self, Self::ImportBridgeRow)
    }
}

/// Controlled shortcut-source class — the precedence layer a resolved shortcut
/// comes from. Keybinding help and conflict sheets name the winner and losers
/// from this one closed set.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShortcutSourceClass {
    /// The shipped default keymap.
    DefaultKeymap,
    /// A platform-reserved default binding.
    PlatformDefault,
    /// An imported foreign keymap (VS Code, JetBrains, ...).
    ImportedKeymap,
    /// An installed extension's binding.
    ExtensionKeybinding,
    /// A workspace-scoped binding.
    WorkspaceKeybinding,
    /// The user's personal binding.
    UserKeybinding,
    /// A leader / multi-key sequence binding.
    LeaderSequence,
}

impl M5ShortcutSourceClass {
    /// Every shortcut-source class, in precedence order (lowest first).
    pub const ALL: [Self; 7] = [
        Self::PlatformDefault,
        Self::DefaultKeymap,
        Self::ImportedKeymap,
        Self::ExtensionKeybinding,
        Self::WorkspaceKeybinding,
        Self::UserKeybinding,
        Self::LeaderSequence,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultKeymap => "default_keymap",
            Self::PlatformDefault => "platform_default",
            Self::ImportedKeymap => "imported_keymap",
            Self::ExtensionKeybinding => "extension_keybinding",
            Self::WorkspaceKeybinding => "workspace_keybinding",
            Self::UserKeybinding => "user_keybinding",
            Self::LeaderSequence => "leader_sequence",
        }
    }

    /// Precedence rank; a higher rank wins a chord conflict. `LeaderSequence`
    /// is scoped to its prefix and never shadows a single-chord winner, so it
    /// shares the user rank only inside its own sequence namespace.
    pub const fn precedence_rank(self) -> u8 {
        match self {
            Self::PlatformDefault => 0,
            Self::DefaultKeymap => 1,
            Self::ImportedKeymap => 2,
            Self::ExtensionKeybinding => 3,
            Self::WorkspaceKeybinding => 4,
            Self::UserKeybinding => 5,
            Self::LeaderSequence => 5,
        }
    }
}

/// Controlled reason a keybinding conflict-review sheet reports. A resolver may
/// not surface a bare "conflict" without one of these named reasons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ConflictReason {
    /// The same chord is bound to two different commands.
    SameChordDifferentCommand,
    /// A higher-precedence layer shadows a lower one.
    HigherLayerShadowed,
    /// A chord is a prefix of a leader sequence (or vice versa).
    SequencePrefixCollision,
    /// Two bindings overlap in the same when-context scope.
    ContextScopeOverlap,
    /// An imported binding collides with an existing native binding.
    ImportedBindingCollision,
    /// A chord is reserved by the OS / platform and cannot be rebound.
    PlatformReservedChord,
}

impl M5ConflictReason {
    /// Every conflict reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SameChordDifferentCommand,
        Self::HigherLayerShadowed,
        Self::SequencePrefixCollision,
        Self::ContextScopeOverlap,
        Self::ImportedBindingCollision,
        Self::PlatformReservedChord,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameChordDifferentCommand => "same_chord_different_command",
            Self::HigherLayerShadowed => "higher_layer_shadowed",
            Self::SequencePrefixCollision => "sequence_prefix_collision",
            Self::ContextScopeOverlap => "context_scope_overlap",
            Self::ImportedBindingCollision => "imported_binding_collision",
            Self::PlatformReservedChord => "platform_reserved_chord",
        }
    }
}

/// Controlled import-translation state an import-bridge row reports for one
/// foreign-keymap binding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ImportTranslationState {
    /// The binding translated exactly to a native command and chord.
    TranslatedExact,
    /// The binding translated to an approximate native equivalent.
    TranslatedApproximated,
    /// The source key had no native command mapping.
    UnmappedSourceKey,
    /// The translated binding collides with an existing native binding.
    ConflictWithExisting,
    /// The binding was rejected as unsafe (authority-widening / reserved).
    RejectedUnsafe,
    /// The binding needs manual review before it can be adopted.
    RequiresManualReview,
}

impl M5ImportTranslationState {
    /// Every import-translation state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TranslatedExact,
        Self::TranslatedApproximated,
        Self::UnmappedSourceKey,
        Self::ConflictWithExisting,
        Self::RejectedUnsafe,
        Self::RequiresManualReview,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TranslatedExact => "translated_exact",
            Self::TranslatedApproximated => "translated_approximated",
            Self::UnmappedSourceKey => "unmapped_source_key",
            Self::ConflictWithExisting => "conflict_with_existing",
            Self::RejectedUnsafe => "rejected_unsafe",
            Self::RequiresManualReview => "requires_manual_review",
        }
    }
}

/// Controlled stale-target invalidation state. A menu item, context action, or
/// keybinding whose target moved, was removed, or lost its context must report
/// one of these rather than silently misfire.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StaleTargetState {
    /// The target command / object is live and current.
    TargetLive,
    /// The target moved and the affordance rebound to it.
    TargetMovedRebound,
    /// The target was removed and the affordance is unavailable.
    TargetRemovedUnavailable,
    /// The affordance's when-context was lost (no valid focus/selection).
    TargetContextLost,
    /// The target was replaced by a deprecation successor.
    TargetReplacedByDeprecation,
}

impl M5StaleTargetState {
    /// Every stale-target state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TargetLive,
        Self::TargetMovedRebound,
        Self::TargetRemovedUnavailable,
        Self::TargetContextLost,
        Self::TargetReplacedByDeprecation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TargetLive => "target_live",
            Self::TargetMovedRebound => "target_moved_rebound",
            Self::TargetRemovedUnavailable => "target_removed_unavailable",
            Self::TargetContextLost => "target_context_lost",
            Self::TargetReplacedByDeprecation => "target_replaced_by_deprecation",
        }
    }
}

/// Controlled why-unavailable explanation class. A disabled-command explainer
/// (and any surface that greys out a command) reports one of these named reasons
/// rather than hiding the command or showing bare "unavailable".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5UnavailableReason {
    /// No active selection the command can operate on.
    NoActiveSelection,
    /// Focus is required in a different surface first.
    FocusRequiredElsewhere,
    /// A preview / approval step must be completed before apply.
    PreviewApprovalRequired,
    /// A policy or legal control blocks the command.
    PolicyBlocked,
    /// A required capability is missing on this build/client.
    CapabilityMissing,
    /// The command requires a higher scope / entitlement.
    HigherScopeRequired,
    /// The command is experimental and not claimed for this milestone.
    ExperimentalNotClaimed,
    /// The command is deprecated; a replacement id must be shown.
    DeprecatedUseReplacement,
    /// An upstream dependency object is unavailable.
    UpstreamDependencyUnavailable,
}

impl M5UnavailableReason {
    /// Every unavailable reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::NoActiveSelection,
        Self::FocusRequiredElsewhere,
        Self::PreviewApprovalRequired,
        Self::PolicyBlocked,
        Self::CapabilityMissing,
        Self::HigherScopeRequired,
        Self::ExperimentalNotClaimed,
        Self::DeprecatedUseReplacement,
        Self::UpstreamDependencyUnavailable,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoActiveSelection => "no_active_selection",
            Self::FocusRequiredElsewhere => "focus_required_elsewhere",
            Self::PreviewApprovalRequired => "preview_approval_required",
            Self::PolicyBlocked => "policy_blocked",
            Self::CapabilityMissing => "capability_missing",
            Self::HigherScopeRequired => "higher_scope_required",
            Self::ExperimentalNotClaimed => "experimental_not_claimed",
            Self::DeprecatedUseReplacement => "deprecated_use_replacement",
            Self::UpstreamDependencyUnavailable => "upstream_dependency_unavailable",
        }
    }
}

/// Mandatory label a claimed discoverability surface must be able to show. The
/// first four are hard requirements on every surface per the guardrails.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5RequiredLabel {
    /// The canonical command id.
    CommandId,
    /// The resolved shortcut source layer.
    SourceLayer,
    /// The typed disabled / why-unavailable reason.
    DisabledReason,
    /// The lifecycle / deprecation truth.
    LifecycleOrDeprecation,
    /// The canonical primary label.
    PrimaryLabel,
    /// The preview / approval requirement.
    PreviewOrApproval,
}

impl M5RequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::CommandId,
        Self::SourceLayer,
        Self::DisabledReason,
        Self::LifecycleOrDeprecation,
        Self::PrimaryLabel,
        Self::PreviewOrApproval,
    ];

    /// The four labels every claimed surface must be able to show.
    pub const MANDATORY: [Self; 4] = [
        Self::CommandId,
        Self::SourceLayer,
        Self::DisabledReason,
        Self::LifecycleOrDeprecation,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandId => "command_id",
            Self::SourceLayer => "source_layer",
            Self::DisabledReason => "disabled_reason",
            Self::LifecycleOrDeprecation => "lifecycle_or_deprecation",
            Self::PrimaryLabel => "primary_label",
            Self::PreviewOrApproval => "preview_or_approval",
        }
    }
}

/// Cross-modality parity surface that must be able to explain the same command
/// semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ParitySurface {
    /// Keyboard navigation / invocation.
    Keyboard,
    /// Screen-reader announcement path.
    ScreenReader,
    /// Pointer (mouse) interaction.
    Pointer,
    /// Touch interaction.
    Touch,
    /// CLI / headless help output.
    CliHelp,
    /// Support / export packet.
    SupportExport,
}

impl M5ParitySurface {
    /// Every parity surface, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Keyboard,
        Self::ScreenReader,
        Self::Pointer,
        Self::Touch,
        Self::CliHelp,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Keyboard => "keyboard",
            Self::ScreenReader => "screen_reader",
            Self::Pointer => "pointer",
            Self::Touch => "touch",
            Self::CliHelp => "cli_help",
            Self::SupportExport => "support_export",
        }
    }
}

/// Qualification class for an M5 discoverability surface row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurfaceQualificationClass {
    /// Surface qualifies for the Stable claim.
    Stable,
    /// Surface is narrowed to Beta.
    Beta,
    /// Surface is narrowed to Preview.
    Preview,
    /// Surface is experimental and not claimed.
    Experimental,
    /// Surface is unavailable on this build.
    Unavailable,
    /// Surface is held pending upstream resolution.
    Held,
}

impl M5SurfaceQualificationClass {
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

    /// Whether the surface may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a discoverability surface below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DiscoverabilityDowngradeTrigger {
    /// A surface invented an alternate label for a stable command.
    AlternateLabelInvented,
    /// A surface dropped the canonical command id.
    CommandIdMissing,
    /// A surface hid the resolved shortcut source layer.
    SourceLayerHidden,
    /// A surface hid the typed disabled / why-unavailable reason.
    DisabledReasonHidden,
    /// A surface hid the lifecycle / deprecation truth.
    LifecycleOrDeprecationHidden,
    /// A surface masked a preview / approval requirement.
    PreviewApprovalMasked,
    /// A surface widened authority beyond the canonical command.
    AuthorityWidened,
    /// A conflict sheet left the winner ambiguous.
    ConflictWinnerAmbiguous,
    /// A stale target was not invalidated.
    StaleTargetNotInvalidated,
    /// An import-bridge translation was untruthful.
    ImportTranslationUntruthful,
    /// A required cross-modality parity surface was dropped.
    ParitySurfaceDropped,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5DiscoverabilityDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::AlternateLabelInvented,
        Self::CommandIdMissing,
        Self::SourceLayerHidden,
        Self::DisabledReasonHidden,
        Self::LifecycleOrDeprecationHidden,
        Self::PreviewApprovalMasked,
        Self::AuthorityWidened,
        Self::ConflictWinnerAmbiguous,
        Self::StaleTargetNotInvalidated,
        Self::ImportTranslationUntruthful,
        Self::ParitySurfaceDropped,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AlternateLabelInvented => "alternate_label_invented",
            Self::CommandIdMissing => "command_id_missing",
            Self::SourceLayerHidden => "source_layer_hidden",
            Self::DisabledReasonHidden => "disabled_reason_hidden",
            Self::LifecycleOrDeprecationHidden => "lifecycle_or_deprecation_hidden",
            Self::PreviewApprovalMasked => "preview_approval_masked",
            Self::AuthorityWidened => "authority_widened",
            Self::ConflictWinnerAmbiguous => "conflict_winner_ambiguous",
            Self::StaleTargetNotInvalidated => "stale_target_not_invalidated",
            Self::ImportTranslationUntruthful => "import_translation_untruthful",
            Self::ParitySurfaceDropped => "parity_surface_dropped",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// The canonical command-record binding a discoverability surface projects from.
///
/// Every field is drawn from the frozen M5 command descriptor; the surface may
/// not reinterpret any of them.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5CanonicalCommandBinding {
    /// Field name carrying the canonical command id (e.g. `command_id`).
    pub command_id_field: String,
    /// Canonical primary-label ref.
    pub primary_label_ref: String,
    /// Canonical help-anchor ref the surface can reopen the command from.
    pub help_anchor_ref: String,
    /// Descriptor revision the surface projects against.
    pub descriptor_revision_ref: String,
    /// Pinned lifecycle label.
    pub lifecycle_label: M5LifecycleLabel,
    /// Pinned preview class.
    pub preview_class: M5PreviewClass,
    /// Pinned disabled-reason mode.
    pub disabled_reason_mode: M5DisabledReasonMode,
}

impl M5CanonicalCommandBinding {
    /// `true` when every required ref field is present.
    fn is_complete(&self) -> bool {
        !self.command_id_field.trim().is_empty()
            && !self.primary_label_ref.trim().is_empty()
            && !self.help_anchor_ref.trim().is_empty()
            && !self.descriptor_revision_ref.trim().is_empty()
    }
}

/// One row in the matrix: one governed discoverability surface family bound to
/// its canonical command record and the surface-specific truth it must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiscoverabilitySurfaceRow {
    /// Governed surface family.
    pub surface_family: M5CommandSurfaceFamily,
    /// Qualification class earned by this surface.
    pub qualification: M5SurfaceQualificationClass,
    /// Owner role accountable for keeping this surface governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Canonical command-record binding this surface projects from.
    pub canonical_command_binding: M5CanonicalCommandBinding,
    /// Mandatory labels this surface must be able to show (must include the four
    /// [`M5RequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5RequiredLabel>,
    /// Shortcut-source classes this surface resolves / explains.
    pub shortcut_source_classes: Vec<M5ShortcutSourceClass>,
    /// Conflict reasons this surface reports.
    pub conflict_reasons: Vec<M5ConflictReason>,
    /// Import-translation states this surface reports.
    pub import_translation_states: Vec<M5ImportTranslationState>,
    /// Stale-target invalidation states this surface honours.
    pub stale_target_states: Vec<M5StaleTargetState>,
    /// Why-unavailable explanation classes this surface reports.
    pub unavailable_reasons: Vec<M5UnavailableReason>,
    /// M5 feature families whose commands this surface exposes.
    pub feature_families: Vec<M5FeatureFamily>,
    /// Cross-modality parity surfaces that explain the same semantics.
    pub parity_surfaces: Vec<M5ParitySurface>,
    /// Discovery channels that consume this surface's projection.
    pub consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Downgrade triggers that apply to this surface.
    pub downgrade_triggers: Vec<M5DiscoverabilityDowngradeTrigger>,
    /// Proof packet refs that keep this surface current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this surface.
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this surface never invents an alternate label. MUST be
    /// `false`.
    pub invents_alternate_label: bool,
    /// Hard invariant: this surface never masks a preview / approval
    /// requirement. MUST be `false`.
    pub masks_preview_or_approval: bool,
    /// Hard invariant: this surface never widens authority. MUST be `false`.
    pub widens_authority: bool,
    /// Hard invariant: this surface never hides a disabled-state reason. MUST be
    /// `false`.
    pub hides_disabled_reason: bool,
}

impl M5DiscoverabilitySurfaceRow {
    /// `true` when the row declares all four mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5RequiredLabel> = self.required_labels.iter().copied().collect();
        M5RequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.invents_alternate_label
            && !self.masks_preview_or_approval
            && !self.widens_authority
            && !self.hides_disabled_reason
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiscoverabilityVocabularySet {
    /// Command-surface-family tokens.
    pub surface_families: Vec<String>,
    /// Shortcut-source-class tokens.
    pub shortcut_source_classes: Vec<String>,
    /// Conflict-reason tokens.
    pub conflict_reasons: Vec<String>,
    /// Import-translation-state tokens.
    pub import_translation_states: Vec<String>,
    /// Stale-target-state tokens.
    pub stale_target_states: Vec<String>,
    /// Why-unavailable-reason tokens.
    pub unavailable_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Parity-surface tokens.
    pub parity_surfaces: Vec<String>,
}

impl M5DiscoverabilityVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            surface_families: tokens(&M5CommandSurfaceFamily::ALL, |v| v.as_str()),
            shortcut_source_classes: tokens(&M5ShortcutSourceClass::ALL, |v| v.as_str()),
            conflict_reasons: tokens(&M5ConflictReason::ALL, |v| v.as_str()),
            import_translation_states: tokens(&M5ImportTranslationState::ALL, |v| v.as_str()),
            stale_target_states: tokens(&M5StaleTargetState::ALL, |v| v.as_str()),
            unavailable_reasons: tokens(&M5UnavailableReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5RequiredLabel::ALL, |v| v.as_str()),
            parity_surfaces: tokens(&M5ParitySurface::ALL, |v| v.as_str()),
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
pub struct M5DiscoverabilityGovernanceReview {
    /// Every surface projects from one canonical command record.
    pub all_surfaces_project_one_command_record: bool,
    /// No surface invents a second naming system.
    pub no_surface_invents_alternate_label: bool,
    /// No surface widens authority beyond the canonical command.
    pub no_surface_widens_authority: bool,
    /// No surface hides a disabled-state reason.
    pub no_surface_hides_disabled_reason: bool,
    /// No surface masks a preview / approval requirement.
    pub no_surface_masks_preview_or_approval: bool,
    /// Every surface can show command id, source layer, disabled reason, and
    /// lifecycle/deprecation truth.
    pub every_surface_shows_mandatory_labels: bool,
    /// Keybinding winners and losers are named from one shortcut-source set.
    pub keybinding_winners_and_losers_named: bool,
    /// Import-bridge outcomes report a controlled translation state.
    pub import_bridge_outcomes_controlled: bool,
    /// Stale targets are invalidated rather than silently misfiring.
    pub stale_targets_invalidated: bool,
    /// Keyboard, screen-reader, pointer, touch, CLI/help, and support/export can
    /// all explain the same command semantics.
    pub cross_modality_parity_preserved: bool,
    /// Later M5 rows cannot invent parallel discoverability vocabulary.
    pub later_rows_cannot_invent_parallel_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiscoverabilityConsumerProjection {
    /// The command palette consumes the shared discoverability matrix.
    pub command_palette_consumes_matrix: bool,
    /// Keybinding help shows the resolved source layer and conflicts.
    pub keybinding_help_shows_source_and_conflicts: bool,
    /// Help search / docs use the controlled surface vocabulary.
    pub help_search_uses_controlled_vocabulary: bool,
    /// Onboarding / tour references quote canonical command ids.
    pub onboarding_tour_quotes_command_ids: bool,
    /// CLI / headless help explains the same command semantics.
    pub cli_headless_explains_same_semantics: bool,
    /// AI automation reads a single canonical command source.
    pub ai_automation_reads_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiscoverabilityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the surface.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the discoverability lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiscoverabilityReleasePosture {
    /// Ref of the supporting release packet for the lane.
    pub release_packet_ref: String,
    /// Ref of the supporting command-parity audit for the lane.
    pub command_parity_audit_ref: String,
    /// True when support/export parity is required for every surface.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every surface.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5DiscoverabilityMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5DiscoverabilityMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5DiscoverabilitySurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DiscoverabilityVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DiscoverabilityGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DiscoverabilityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DiscoverabilityProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DiscoverabilityReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 discoverability-affordance matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DiscoverabilityMatrixPacket {
    /// Record kind; must equal [`M5_DISCOVERABILITY_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_DISCOVERABILITY_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Surface rows.
    pub surface_rows: Vec<M5DiscoverabilitySurfaceRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5DiscoverabilityVocabularySet,
    /// Governance-review block.
    pub governance_review: M5DiscoverabilityGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5DiscoverabilityConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DiscoverabilityProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5DiscoverabilityReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5DiscoverabilityMatrixPacket {
    /// Builds an M5 discoverability matrix packet from stable-lane input.
    pub fn new(input: M5DiscoverabilityMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_DISCOVERABILITY_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_DISCOVERABILITY_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            surface_rows: input.surface_rows,
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

    /// Validates the M5 discoverability matrix invariants.
    pub fn validate(&self) -> Vec<M5DiscoverabilityMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_DISCOVERABILITY_MATRIX_RECORD_KIND {
            violations.push(M5DiscoverabilityMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_DISCOVERABILITY_MATRIX_SCHEMA_VERSION {
            violations.push(M5DiscoverabilityMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5DiscoverabilityMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_surface_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 discoverability matrix packet serializes"),
        ) {
            violations.push(M5DiscoverabilityMatrixViolation::RawMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 discoverability matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,qualification,owner,command_id_field,lifecycle_label,preview_class,disabled_reason_mode,required_labels,consumer_surfaces\n",
        );
        for row in &self.surface_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                csv_field(&row.canonical_command_binding.command_id_field),
                row.canonical_command_binding.lifecycle_label.as_str(),
                row.canonical_command_binding.preview_class.as_str(),
                row.canonical_command_binding.disabled_reason_mode.as_str(),
                join_tokens(&row.required_labels, |v| v.as_str()),
                join_tokens(&row.consumer_surfaces, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_surfaces = self
            .surface_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Menu-Affordance, Keybinding-Resolver, and Command-Documentation Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Surface families: {} ({} stable)\n",
            self.surface_rows.len(),
            stable_surfaces
        ));
        out.push_str(&format!(
            "- Shortcut-source classes: {}\n",
            self.vocabulary_set.shortcut_source_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Why-unavailable reasons: {}\n",
            self.vocabulary_set.unavailable_reasons.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Surface families\n\n");
        for row in &self.surface_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.surface_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Command id field: `{}`\n",
                row.canonical_command_binding.command_id_field
            ));
            out.push_str(&format!(
                "  - Lifecycle / preview: `{}` / `{}`\n",
                row.canonical_command_binding.lifecycle_label.as_str(),
                row.canonical_command_binding.preview_class.as_str()
            ));
            out.push_str(&format!(
                "  - Required labels: {}\n",
                row.required_labels
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            out.push_str(&format!(
                "  - Why-unavailable: {}\n",
                row.unavailable_reasons
                    .iter()
                    .map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 discoverability matrix export.
#[derive(Debug)]
pub enum M5DiscoverabilityMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5DiscoverabilityMatrixViolation>),
}

impl fmt::Display for M5DiscoverabilityMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 discoverability matrix export parse failed: {error}"
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
                    "m5 discoverability matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5DiscoverabilityMatrixArtifactError {}

/// Validation failures emitted by [`M5DiscoverabilityMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5DiscoverabilityMatrixViolation {
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
    /// A required governed surface family is missing from the matrix.
    RequiredSurfaceMissing,
    /// A surface row is incomplete.
    SurfaceRowIncomplete,
    /// A surface row's canonical command binding is incomplete.
    CommandBindingIncomplete,
    /// A surface row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A shortcut-resolving surface declares no shortcut-source classes.
    ShortcutSourceMissing,
    /// A conflict-reviewing surface declares no conflict reasons.
    ConflictReasonMissing,
    /// An import-bridge surface declares no import-translation states.
    ImportTranslationMissing,
    /// A surface declares no stale-target states.
    StaleTargetMissing,
    /// A surface declares no why-unavailable reasons.
    UnavailableReasonMissing,
    /// A surface declares no feature families.
    FeatureFamilyMissing,
    /// A surface declares no parity surfaces.
    ParitySurfaceMissing,
    /// A surface declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A surface declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A surface claiming Stable is missing required proof packet refs.
    StableSurfaceMissingProof,
    /// A surface violates a hard invariant (alt label, masked approval, widened
    /// authority, or hidden disabled reason).
    SurfaceInvariantViolated,
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

impl M5DiscoverabilityMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::SurfaceRowIncomplete => "surface_row_incomplete",
            Self::CommandBindingIncomplete => "command_binding_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::ShortcutSourceMissing => "shortcut_source_missing",
            Self::ConflictReasonMissing => "conflict_reason_missing",
            Self::ImportTranslationMissing => "import_translation_missing",
            Self::StaleTargetMissing => "stale_target_missing",
            Self::UnavailableReasonMissing => "unavailable_reason_missing",
            Self::FeatureFamilyMissing => "feature_family_missing",
            Self::ParitySurfaceMissing => "parity_surface_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableSurfaceMissingProof => "stable_surface_missing_proof",
            Self::SurfaceInvariantViolated => "surface_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 discoverability matrix export.
pub fn current_stable_m5_discoverability_matrix_export(
) -> Result<M5DiscoverabilityMatrixPacket, M5DiscoverabilityMatrixArtifactError> {
    let packet: M5DiscoverabilityMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-discoverability-affordances-proof/support_export.json"
    )))
    .map_err(M5DiscoverabilityMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5DiscoverabilityMatrixArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5DiscoverabilityMatrixPacket,
    violations: &mut Vec<M5DiscoverabilityMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_DISCOVERABILITY_SCHEMA_REF,
        M5_DISCOVERABILITY_DOC_REF,
        M5_DISCOVERABILITY_COMMAND_DESCRIPTOR_REF,
        M5_DISCOVERABILITY_KEYBINDING_RESOLVER_REF,
        M5_DISCOVERABILITY_MENU_ITEM_REF,
        M5_DISCOVERABILITY_LEADER_OVERLAY_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5DiscoverabilityMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5DiscoverabilityMatrixPacket,
    violations: &mut Vec<M5DiscoverabilityMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5DiscoverabilityMatrixViolation::VocabularySetDrift);
    }
}

fn validate_surface_rows(
    packet: &M5DiscoverabilityMatrixPacket,
    violations: &mut Vec<M5DiscoverabilityMatrixViolation>,
) {
    let present: BTreeSet<M5CommandSurfaceFamily> = packet
        .surface_rows
        .iter()
        .map(|row| row.surface_family)
        .collect();
    for required in M5CommandSurfaceFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5DiscoverabilityMatrixViolation::RequiredSurfaceMissing);
            return;
        }
    }

    for row in &packet.surface_rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5DiscoverabilityMatrixViolation::SurfaceRowIncomplete);
        }
        if !row.canonical_command_binding.is_complete() {
            violations.push(M5DiscoverabilityMatrixViolation::CommandBindingIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5DiscoverabilityMatrixViolation::MandatoryLabelMissing);
        }
        if row.surface_family.resolves_shortcuts() && row.shortcut_source_classes.is_empty() {
            violations.push(M5DiscoverabilityMatrixViolation::ShortcutSourceMissing);
        }
        if row.surface_family.reviews_conflicts() && row.conflict_reasons.is_empty() {
            violations.push(M5DiscoverabilityMatrixViolation::ConflictReasonMissing);
        }
        if row.surface_family.translates_imports() && row.import_translation_states.is_empty() {
            violations.push(M5DiscoverabilityMatrixViolation::ImportTranslationMissing);
        }
        if row.stale_target_states.is_empty() {
            violations.push(M5DiscoverabilityMatrixViolation::StaleTargetMissing);
        }
        if row.unavailable_reasons.is_empty() {
            violations.push(M5DiscoverabilityMatrixViolation::UnavailableReasonMissing);
        }
        if row.feature_families.is_empty() {
            violations.push(M5DiscoverabilityMatrixViolation::FeatureFamilyMissing);
        }
        if row.parity_surfaces.is_empty() {
            violations.push(M5DiscoverabilityMatrixViolation::ParitySurfaceMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5DiscoverabilityMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5DiscoverabilityMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5DiscoverabilityMatrixViolation::StableSurfaceMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5DiscoverabilityMatrixViolation::SurfaceInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5DiscoverabilityMatrixPacket,
    violations: &mut Vec<M5DiscoverabilityMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.all_surfaces_project_one_command_record,
        review.no_surface_invents_alternate_label,
        review.no_surface_widens_authority,
        review.no_surface_hides_disabled_reason,
        review.no_surface_masks_preview_or_approval,
        review.every_surface_shows_mandatory_labels,
        review.keybinding_winners_and_losers_named,
        review.import_bridge_outcomes_controlled,
        review.stale_targets_invalidated,
        review.cross_modality_parity_preserved,
        review.later_rows_cannot_invent_parallel_vocabulary,
    ] {
        if !ok {
            violations.push(M5DiscoverabilityMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5DiscoverabilityMatrixPacket,
    violations: &mut Vec<M5DiscoverabilityMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.command_palette_consumes_matrix,
        projection.keybinding_help_shows_source_and_conflicts,
        projection.help_search_uses_controlled_vocabulary,
        projection.onboarding_tour_quotes_command_ids,
        projection.cli_headless_explains_same_semantics,
        projection.ai_automation_reads_single_source,
    ] {
        if !ok {
            violations.push(M5DiscoverabilityMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5DiscoverabilityMatrixPacket,
    violations: &mut Vec<M5DiscoverabilityMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5DiscoverabilityMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5DiscoverabilityMatrixPacket,
    violations: &mut Vec<M5DiscoverabilityMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.command_parity_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5DiscoverabilityMatrixViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never
/// introduces a stray comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items
        .iter()
        .map(|item| to_token(item))
        .collect::<Vec<_>>()
        .join("|")
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
