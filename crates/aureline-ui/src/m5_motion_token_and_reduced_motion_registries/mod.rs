//! Implemented M5 motion-token and reduced-motion registries.
//!
//! The frozen [motion / layer / iconography matrix][matrix] names Aureline's seven visual-interaction
//! families and locks their controlled vocabulary. This module is the first implement lane over that
//! matrix: it turns the two families that carry the *motion* grammar — the **motion token** (duration /
//! easing families that clarify origin, continuity, and completion) and the **reduced motion** clamp — into
//! registry resolvers that produce export-safe, honest projections, so a user can trust that transitions
//! clarify where content came from and when it settled without ever delaying typing, palette input, or a
//! diagnostic, that motion respects reduced-motion / power-saver / thermal clamps with a static fallback
//! that preserves meaning, and that no menu, palette, inline-editor, or typing-adjacent surface animates in
//! a way that delays input or shifts layout.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement canonical motion duration / easing families plus allow / deny rules for which surface
//!   classes may animate and which must remain effectively instant.** [`resolve_motion_entry`] refuses to
//!   read as a clean, protected-path-safe motion entry unless it names a canonical token, a classified
//!   [motion surface class][M5MotionSurfaceClass], a motion role, and a reduced-motion fallback, covers all
//!   three motion clamps, respects input priority on protected paths, introduces no layout shift, and
//!   traces to a canonical token rather than an inlined raw duration; otherwise it degrades.
//! * **Require reduced-motion clamps and no-layout-shift behavior for menu, palette, diagnostic,
//!   inline-editor, and typing-adjacent surfaces.** Every motion entry names an [`M5ReducedMotionFallback`]
//!   and degrades to [`M5MotionEntryDegradeReason::ProtectedPathDelayedByMotion`],
//!   [`M5MotionEntryDegradeReason::LayoutShiftIntroduced`], or
//!   [`M5MotionEntryDegradeReason::ReducedMotionFallbackMissing`] when motion would otherwise delay input,
//!   shift layout, or carry the only cue. [`resolve_reduced_motion_entry`] does the same for a
//!   reduced-motion role and refuses to let meaning ride on motion alone or leave a clamp uncovered.
//! * **Wire first shell, dialog, panel, and embedded-surface consumers plus fixtures that catch
//!   protected-path animation regressions.** Each registry row carries the render [surface
//!   context][M5MotionSurfaceContext] so a protected-path animation regression degrades honestly, and the
//!   acceptance-criteria gate proves a protected-path delay or raw-duration regression is caught before
//!   release evidence turns green.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5VisualInteractionRole`] role
//! vocabulary, the [`M5MotionTokenRole`] motion-role vocabulary, and the [`M5ReducedMotionRole`]
//! reduced-motion-role vocabulary — so shell, dialog, panel, embedded, and support surfaces can never fork
//! their own motion or reduced-motion meaning. Raw secret values and private endpoints stay outside the
//! export boundary.
//!
//! [matrix]: crate::m5_motion_layer_iconography_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_motion_reduced_motion_registries,
    seeded_m5_motion_reduced_motion_registries_onboarding_ui_preview_narrowed,
    seeded_m5_motion_reduced_motion_registries_shell_ui_beta_narrowed,
    M5_MOTION_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_motion_layer_iconography_matrix::{
    M5MotionTokenRole, M5ReducedMotionRole, M5VisualInteractionAccessibilityRoute,
    M5VisualInteractionConsumerSurface, M5VisualInteractionDeploymentLine,
    M5VisualInteractionDowngradeTrigger, M5VisualInteractionFamily,
    M5VisualInteractionQualificationClass, M5VisualInteractionRequiredLabel,
    M5VisualInteractionRole, M5_MOTION_AND_REDUCED_MOTION_SCHEMA_REF,
    M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF, M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5MotionRegistriesPacket`].
pub const M5_MOTION_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_motion_token_and_reduced_motion_registries";

/// Schema version for M5 motion / reduced-motion registry records.
pub const M5_MOTION_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_MOTION_REGISTRIES_SCHEMA_REF: &str =
    "schemas/design-system/m5-motion-token-and-reduced-motion-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_MOTION_REGISTRIES_DOC_REF: &str =
    "docs/design-system/m5_motion_token_and_reduced_motion_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_MOTION_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-motion-token-and-reduced-motion-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_MOTION_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-motion-token-and-reduced-motion-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_MOTION_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-motion-token-and-reduced-motion-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_MOTION_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-motion-token-and-reduced-motion-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so
/// no lane invents a parallel surface set.
pub type M5MotionRegistriesConsumerSurface = M5VisualInteractionConsumerSurface;

/// One of the three motion clamps every motion / reduced-motion entry must cover so its behavior is
/// explicit under reduced-motion, power-saver, and thermal pressure. Minted by this lane because the frozen
/// matrix names the reduced-motion *rule* but not the concrete clamp set an entry must cover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MotionClamp {
    /// The reduced-motion clamp.
    ReducedMotion,
    /// The power-saver clamp.
    PowerSaver,
    /// The thermal clamp.
    Thermal,
}

impl M5MotionClamp {
    /// Every motion clamp, in declaration order. A clean entry must cover all three.
    pub const ALL: [Self; 3] = [Self::ReducedMotion, Self::PowerSaver, Self::Thermal];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReducedMotion => "reduced_motion",
            Self::PowerSaver => "power_saver",
            Self::Thermal => "thermal",
        }
    }
}

/// Controlled surface class a motion entry maps, so the surfaces that must remain effectively instant
/// (command palette, menu navigation, typing caret, inline editor, diagnostics) never animate in a way that
/// delays input, and the surfaces that may animate (dialogs, panels, overlays, notifications, onboarding,
/// focus, progress) share one canonical grammar. Minted by this lane because the frozen matrix carries the
/// seven high-level interaction roles but not the finer surface classes the motion acceptance criteria
/// require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MotionSurfaceClass {
    /// Command-palette input (protected: must remain effectively instant).
    CommandPaletteInput,
    /// Menu navigation (protected: must remain effectively instant).
    MenuNavigation,
    /// Typing caret / cursor (protected: must remain effectively instant).
    TypingCaret,
    /// Inline editor (protected: must remain effectively instant).
    InlineEditor,
    /// Diagnostic surface (protected: must remain effectively instant).
    DiagnosticSurface,
    /// Dialog entrance (may animate).
    DialogEntrance,
    /// Panel transition (may animate).
    PanelTransition,
    /// Overlay reveal (may animate).
    OverlayReveal,
    /// Notification entrance (may animate).
    NotificationEntrance,
    /// Tooltip reveal (may animate).
    TooltipReveal,
    /// Progress indicator (may animate).
    ProgressIndicator,
    /// Onboarding sequence (may animate).
    OnboardingSequence,
    /// Focus transition (may animate).
    FocusTransition,
    /// The surface class is unclassified, which is disallowed.
    SurfaceClassUnclassified,
}

impl M5MotionSurfaceClass {
    /// Every motion surface class, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::CommandPaletteInput,
        Self::MenuNavigation,
        Self::TypingCaret,
        Self::InlineEditor,
        Self::DiagnosticSurface,
        Self::DialogEntrance,
        Self::PanelTransition,
        Self::OverlayReveal,
        Self::NotificationEntrance,
        Self::TooltipReveal,
        Self::ProgressIndicator,
        Self::OnboardingSequence,
        Self::FocusTransition,
        Self::SurfaceClassUnclassified,
    ];

    /// The protected-path surface classes the acceptance criteria require to remain effectively instant.
    pub const PROTECTED_PATHS: [Self; 5] = [
        Self::CommandPaletteInput,
        Self::MenuNavigation,
        Self::TypingCaret,
        Self::InlineEditor,
        Self::DiagnosticSurface,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandPaletteInput => "command_palette_input",
            Self::MenuNavigation => "menu_navigation",
            Self::TypingCaret => "typing_caret",
            Self::InlineEditor => "inline_editor",
            Self::DiagnosticSurface => "diagnostic_surface",
            Self::DialogEntrance => "dialog_entrance",
            Self::PanelTransition => "panel_transition",
            Self::OverlayReveal => "overlay_reveal",
            Self::NotificationEntrance => "notification_entrance",
            Self::TooltipReveal => "tooltip_reveal",
            Self::ProgressIndicator => "progress_indicator",
            Self::OnboardingSequence => "onboarding_sequence",
            Self::FocusTransition => "focus_transition",
            Self::SurfaceClassUnclassified => "surface_class_unclassified",
        }
    }

    /// Whether the surface class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::SurfaceClassUnclassified)
    }

    /// Whether this is a protected-path surface class that must remain effectively instant.
    pub const fn is_protected_path(self) -> bool {
        matches!(
            self,
            Self::CommandPaletteInput
                | Self::MenuNavigation
                | Self::TypingCaret
                | Self::InlineEditor
                | Self::DiagnosticSurface
        )
    }
}

/// Controlled static fallback a motion entry pairs with animation so meaning survives when motion is
/// clamped: an instant state change, an opacity crossfade, a static indicator, a textual status, or a
/// screen-reader announcement. Minted by this lane, tracking the reduced-motion / no-layout-shift fallback
/// the motion acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReducedMotionFallback {
    /// An instant state change carries the meaning when motion is clamped.
    InstantStateChange,
    /// An opacity crossfade (no positional motion) carries the meaning when motion is clamped.
    OpacityCrossfade,
    /// A static indicator carries the meaning when motion is clamped.
    StaticIndicator,
    /// A textual status carries the meaning when motion is clamped.
    TextualStatus,
    /// A screen-reader announcement carries the meaning when motion is clamped.
    ScreenReaderAnnouncement,
    /// No static fallback is paired with the motion, which is disallowed.
    NoneDisallowed,
}

impl M5ReducedMotionFallback {
    /// Every static fallback, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::InstantStateChange,
        Self::OpacityCrossfade,
        Self::StaticIndicator,
        Self::TextualStatus,
        Self::ScreenReaderAnnouncement,
        Self::NoneDisallowed,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InstantStateChange => "instant_state_change",
            Self::OpacityCrossfade => "opacity_crossfade",
            Self::StaticIndicator => "static_indicator",
            Self::TextualStatus => "textual_status",
            Self::ScreenReaderAnnouncement => "screen_reader_announcement",
            Self::NoneDisallowed => "none_disallowed",
        }
    }

    /// Whether a static fallback is present (never the disallowed none sentinel).
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::NoneDisallowed)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a motion or
/// reduced-motion token's behavior stays stable whether it appears in the shell, a dialog, a panel, an
/// embedded surface, or a notification. Minted by this lane, tracking the first-consumer surfaces the
/// implementation requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MotionSurfaceContext {
    /// The shell surface.
    Shell,
    /// The dialog surface.
    Dialog,
    /// The panel surface.
    Panel,
    /// The embedded / browser-handoff surface.
    Embedded,
    /// The notification surface.
    Notification,
    /// The render context cannot currently be resolved.
    ContextUnknown,
}

impl M5MotionSurfaceContext {
    /// Every render context, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Shell,
        Self::Dialog,
        Self::Panel,
        Self::Embedded,
        Self::Notification,
        Self::ContextUnknown,
    ];

    /// The five first-consumer contexts the implementation requirement names.
    pub const FIRST_CONSUMERS: [Self; 5] = [
        Self::Shell,
        Self::Dialog,
        Self::Panel,
        Self::Embedded,
        Self::Notification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Shell => "shell",
            Self::Dialog => "dialog",
            Self::Panel => "panel",
            Self::Embedded => "embedded",
            Self::Notification => "notification",
            Self::ContextUnknown => "context_unknown",
        }
    }

    /// Whether the render context is resolved (not the unknown sentinel).
    pub const fn is_resolved(self) -> bool {
        !matches!(self, Self::ContextUnknown)
    }
}

/// One mandatory rendered part a motion or reduced-motion entry must be able to show, so no behavior,
/// clamp, or token fact is left implicit behind an animation curve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MotionRegistryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical token reference the entry points at.
    TokenReference,
    /// The motion surface class the entry maps (motion entry).
    MotionSurfaceClass,
    /// The clamp coverage (reduced-motion / power-saver / thermal).
    ClampCoverage,
    /// The reduced-motion fallback paired with the animation (motion entry).
    ReducedMotionFallback,
    /// The motion role named by the entry (motion entry).
    MotionRole,
    /// The reduced-motion role named by the entry (reduced-motion entry).
    ReducedMotionRole,
    /// The render / surface context (both entries).
    SurfaceContext,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the token (both entries).
    PlainLanguageMeaning,
}

impl M5MotionRegistryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::TokenReference,
        Self::MotionSurfaceClass,
        Self::ClampCoverage,
        Self::ReducedMotionFallback,
        Self::MotionRole,
        Self::ReducedMotionRole,
        Self::SurfaceContext,
        Self::KeyboardRoute,
        Self::PlainLanguageMeaning,
    ];

    /// The three parts every claimed entry must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::TokenReference];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::TokenReference => "token_reference",
            Self::MotionSurfaceClass => "motion_surface_class",
            Self::ClampCoverage => "clamp_coverage",
            Self::ReducedMotionFallback => "reduced_motion_fallback",
            Self::MotionRole => "motion_role",
            Self::ReducedMotionRole => "reduced_motion_role",
            Self::SurfaceContext => "surface_context",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect behavior,
/// clamp coverage, or a degraded motion / reduced-motion token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MotionRegistryNextAction {
    /// Expand the motion's plain-language meaning.
    ExpandMotionMeaning,
    /// Inspect the motion surface class the entry maps.
    InspectSurfaceClass,
    /// Complete the reduced-motion / power-saver / thermal clamp coverage.
    CompleteClampCoverage,
    /// Trace the entry back to its canonical token.
    TraceCanonicalToken,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5MotionRegistryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandMotionMeaning,
        Self::InspectSurfaceClass,
        Self::CompleteClampCoverage,
        Self::TraceCanonicalToken,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandMotionMeaning => "expand_motion_meaning",
            Self::InspectSurfaceClass => "inspect_surface_class",
            Self::CompleteClampCoverage => "complete_clamp_coverage",
            Self::TraceCanonicalToken => "trace_canonical_token",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MotionRegistryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The interaction families covered.
    InteractionFamilies,
    /// The motion surface classes carried.
    MotionSurfaceClasses,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The clamp profiles covered.
    ClampProfiles,
    /// The reduced-motion fallbacks paired.
    ReducedMotionFallbacks,
    /// The render / surface context.
    SurfaceContext,
    /// The reduced-motion roles named.
    ReducedMotionRoles,
    /// The accountable owner role.
    OwnerRole,
}

impl M5MotionRegistryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::InteractionFamilies,
        Self::MotionSurfaceClasses,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ClampProfiles,
        Self::ReducedMotionFallbacks,
        Self::SurfaceContext,
        Self::ReducedMotionRoles,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::InteractionFamilies,
        Self::MotionSurfaceClasses,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::InteractionFamilies => "interaction_families",
            Self::MotionSurfaceClasses => "motion_surface_classes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ClampProfiles => "clamp_profiles",
            Self::ReducedMotionFallbacks => "reduced_motion_fallbacks",
            Self::SurfaceContext => "surface_context",
            Self::ReducedMotionRoles => "reduced_motion_roles",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a motion entry degraded below a clean, protected-path-safe state. The degrade-first ladder
/// returns one of these instead of ever letting a protected-path-delaying, layout-shifting, raw-duration,
/// or clamp-incomplete entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MotionEntryDegradeReason {
    /// The canonical token name is unstated; a user cannot trace what the motion means.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The motion surface class is unclassified (not in the preserved taxonomy).
    SurfaceClassUnclassified,
    /// The motion delays input on a protected path rather than respecting input priority.
    ProtectedPathDelayedByMotion,
    /// No reduced-motion static fallback is paired with the animation.
    ReducedMotionFallbackMissing,
    /// A raw duration value is inlined instead of tracing to a canonical token.
    RawDurationValueInlined,
    /// The reduced-motion / power-saver / thermal clamp coverage is incomplete.
    ClampCoverageIncomplete,
    /// The motion introduces a layout shift on a protected or typing-adjacent surface.
    LayoutShiftIntroduced,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5MotionEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::SurfaceClassUnclassified,
        Self::ProtectedPathDelayedByMotion,
        Self::ReducedMotionFallbackMissing,
        Self::RawDurationValueInlined,
        Self::ClampCoverageIncomplete,
        Self::LayoutShiftIntroduced,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::SurfaceClassUnclassified => "surface_class_unclassified",
            Self::ProtectedPathDelayedByMotion => "protected_path_delayed_by_motion",
            Self::ReducedMotionFallbackMissing => "reduced_motion_fallback_missing",
            Self::RawDurationValueInlined => "raw_duration_value_inlined",
            Self::ClampCoverageIncomplete => "clamp_coverage_incomplete",
            Self::LayoutShiftIntroduced => "layout_shift_introduced",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5MotionRegistryNextAction {
        match self {
            Self::TokenNameUnstated | Self::RawDurationValueInlined => {
                M5MotionRegistryNextAction::TraceCanonicalToken
            }
            Self::SurfaceClassUnclassified | Self::LayoutShiftIntroduced => {
                M5MotionRegistryNextAction::InspectSurfaceClass
            }
            Self::ProtectedPathDelayedByMotion | Self::ReducedMotionFallbackMissing => {
                M5MotionRegistryNextAction::ExpandMotionMeaning
            }
            Self::ClampCoverageIncomplete => M5MotionRegistryNextAction::CompleteClampCoverage,
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5MotionRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualInteractionDowngradeTrigger {
        match self {
            Self::ProtectedPathDelayedByMotion | Self::LayoutShiftIntroduced => {
                M5VisualInteractionDowngradeTrigger::MotionDelayedProtectedInput
            }
            Self::ReducedMotionFallbackMissing | Self::ClampCoverageIncomplete => {
                M5VisualInteractionDowngradeTrigger::MotionMeaningLostUnderReducedMotion
            }
            Self::TokenNameUnstated | Self::RawDurationValueInlined => {
                M5VisualInteractionDowngradeTrigger::TokenReferenceUnstated
            }
            Self::SurfaceClassUnclassified | Self::SurfaceContextUnresolved => {
                M5VisualInteractionDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualInteractionDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a reduced-motion entry degraded below a clean, clamp-safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReducedMotionEntryDegradeReason {
    /// The canonical token name is unstated.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// Meaning rides on motion alone with no static fallback, or no canonical token is named.
    MotionOnlyMeaningWithoutFallback,
    /// The reduced-motion / power-saver / thermal clamp coverage is incomplete.
    ClampCoverageIncomplete,
    /// The static fallback does not preserve the same meaning as the motion.
    StaticFallbackNotEquivalent,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ReducedMotionEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::MotionOnlyMeaningWithoutFallback,
        Self::ClampCoverageIncomplete,
        Self::StaticFallbackNotEquivalent,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::MotionOnlyMeaningWithoutFallback => "motion_only_meaning_without_fallback",
            Self::ClampCoverageIncomplete => "clamp_coverage_incomplete",
            Self::StaticFallbackNotEquivalent => "static_fallback_not_equivalent",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5MotionRegistryNextAction {
        match self {
            Self::TokenNameUnstated | Self::MotionOnlyMeaningWithoutFallback => {
                M5MotionRegistryNextAction::TraceCanonicalToken
            }
            Self::ClampCoverageIncomplete => M5MotionRegistryNextAction::CompleteClampCoverage,
            Self::StaticFallbackNotEquivalent => M5MotionRegistryNextAction::InspectSurfaceClass,
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5MotionRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualInteractionDowngradeTrigger {
        match self {
            Self::TokenNameUnstated | Self::MotionOnlyMeaningWithoutFallback => {
                M5VisualInteractionDowngradeTrigger::TokenReferenceUnstated
            }
            Self::ClampCoverageIncomplete | Self::StaticFallbackNotEquivalent => {
                M5VisualInteractionDowngradeTrigger::MotionMeaningLostUnderReducedMotion
            }
            Self::SurfaceContextUnresolved => {
                M5VisualInteractionDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualInteractionDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_motion_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MotionEntryResolutionInput {
    /// Stable identity of the motion-registry entry.
    pub entry_id: String,
    /// The canonical token name (e.g. `motion.duration.standard`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualInteractionRole,
    /// The motion role (from the frozen matrix vocabulary).
    pub motion_role: M5MotionTokenRole,
    /// The motion surface class this entry maps.
    pub surface_class: M5MotionSurfaceClass,
    /// The static fallback paired with the animation.
    pub reduced_motion_fallback: M5ReducedMotionFallback,
    /// The render / surface context.
    pub surface_context: M5MotionSurfaceContext,
    /// The motion clamps this entry covers (must cover reduced-motion / power-saver / thermal).
    pub clamp_coverage: Vec<M5MotionClamp>,
    /// True when the motion respects input priority and never delays a protected path.
    pub respects_input_priority: bool,
    /// True when the motion introduces no layout shift.
    pub preserves_no_layout_shift: bool,
    /// True when the entry traces to a canonical token (never an inlined raw duration value).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe motion-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedMotionEntry {
    /// Stable identity of the motion-registry entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role demands an accessible fallback (motion / overlay / icon / illustration /
    /// attention).
    pub semantic_role_demands_accessible_fallback: bool,
    /// The motion-role token named by the entry.
    pub motion_role: String,
    /// Whether the motion role names the disallowed protected-input-delaying token.
    pub motion_role_delays_protected_input: bool,
    /// The motion-surface-class token named by the entry.
    pub surface_class: String,
    /// Whether the motion surface class is classified into the preserved taxonomy.
    pub surface_class_is_classified: bool,
    /// Whether this is a protected-path surface class that must remain effectively instant.
    pub surface_class_is_protected_path: bool,
    /// The reduced-motion-fallback token named by the entry.
    pub reduced_motion_fallback: String,
    /// Whether a reduced-motion static fallback is present.
    pub reduced_motion_fallback_present: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The clamp tokens covered by the entry.
    pub clamp_coverage: Vec<String>,
    /// Whether the entry covers all three motion clamps.
    pub covers_all_clamps: bool,
    /// Whether the motion respects input priority and never delays a protected path.
    pub respects_input_priority: bool,
    /// Whether the motion introduces no layout shift.
    pub preserves_no_layout_shift: bool,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean, protected-path-safe state.
    pub degrade_reason: Option<M5MotionEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5MotionRegistryNextAction,
    /// Whether the motion stays safe on protected paths (clean entry naming every fact).
    pub motion_safe_on_protected_paths: bool,
}

impl M5ResolvedMotionEntry {
    /// Whether this motion entry reads as a clean, protected-path-safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_reduced_motion_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ReducedMotionEntryResolutionInput {
    /// Stable identity of the reduced-motion entry.
    pub entry_id: String,
    /// The canonical token name; empty means unstated.
    pub token_name: String,
    /// The reduced-motion role (from the frozen matrix vocabulary).
    pub reduced_motion_role: M5ReducedMotionRole,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualInteractionRole,
    /// The render / surface context.
    pub surface_context: M5MotionSurfaceContext,
    /// The motion clamps this entry covers (must cover reduced-motion / power-saver / thermal).
    pub clamp_coverage: Vec<M5MotionClamp>,
    /// True when the entry traces to a canonical token (never meaning riding on motion alone).
    pub references_canonical_token: bool,
    /// True when the static fallback preserves the same meaning as the motion.
    pub static_fallback_preserves_meaning: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe reduced-motion projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedReducedMotionEntry {
    /// Stable identity of the reduced-motion entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The reduced-motion-role token named by the entry.
    pub reduced_motion_role: String,
    /// Whether the reduced-motion role names the disallowed motion-only-meaning token.
    pub reduced_motion_role_is_motion_only: bool,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The clamp tokens covered by the entry.
    pub clamp_coverage: Vec<String>,
    /// Whether the entry covers all three motion clamps.
    pub covers_all_clamps: bool,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Whether the static fallback preserves the same meaning as the motion.
    pub static_fallback_preserves_meaning: bool,
    /// Degrade reason, if the entry could not read as a clean, clamp-safe state.
    pub degrade_reason: Option<M5ReducedMotionEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5MotionRegistryNextAction,
    /// Whether the fallback preserves meaning across every clamp (clean entry naming every fact).
    pub fallback_preserves_meaning_across_clamps: bool,
}

impl M5ResolvedReducedMotionEntry {
    /// Whether this reduced-motion entry reads as a clean, clamp-safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5MotionResolutionError {
    /// The motion-entry id was empty.
    EmptyMotionEntryId,
    /// The reduced-motion-entry id was empty.
    EmptyReducedMotionEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5MotionResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyMotionEntryId => "empty_motion_entry_id",
            Self::EmptyReducedMotionEntryId => "empty_reduced_motion_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5MotionResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 motion / reduced-motion registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5MotionResolutionError {}

fn clamp_tokens(clamps: &[M5MotionClamp]) -> Vec<String> {
    clamps.iter().map(|c| c.as_str().to_owned()).collect()
}

fn covers_all_clamps(clamps: &[M5MotionClamp]) -> bool {
    let present: BTreeSet<M5MotionClamp> = clamps.iter().copied().collect();
    M5MotionClamp::ALL
        .iter()
        .all(|clamp| present.contains(clamp))
}

/// Resolves a motion-registry entry so it stays safe on protected paths: the entry names its canonical
/// token, semantic role, motion role, surface class, and a reduced-motion fallback, covers all three
/// clamps, respects input priority, introduces no layout shift, and traces to a canonical token rather than
/// an inlined raw duration value.
pub fn resolve_motion_entry(
    input: M5MotionEntryResolutionInput,
) -> Result<M5ResolvedMotionEntry, M5MotionResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5MotionResolutionError::EmptyMotionEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5MotionResolutionError::ForbiddenMaterial);
    }

    let motion_role_delays_protected_input = matches!(
        input.motion_role,
        M5MotionTokenRole::MotionDelaysProtectedInputDisallowed
    );
    let all_clamps = covers_all_clamps(&input.clamp_coverage);

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5MotionEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5MotionEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.surface_class.is_classified() {
        Some(M5MotionEntryDegradeReason::SurfaceClassUnclassified)
    } else if motion_role_delays_protected_input || !input.respects_input_priority {
        Some(M5MotionEntryDegradeReason::ProtectedPathDelayedByMotion)
    } else if !input.reduced_motion_fallback.is_present() {
        Some(M5MotionEntryDegradeReason::ReducedMotionFallbackMissing)
    } else if !input.references_canonical_token {
        Some(M5MotionEntryDegradeReason::RawDurationValueInlined)
    } else if !all_clamps {
        Some(M5MotionEntryDegradeReason::ClampCoverageIncomplete)
    } else if !input.preserves_no_layout_shift {
        Some(M5MotionEntryDegradeReason::LayoutShiftIntroduced)
    } else if !input.proof_fresh {
        Some(M5MotionEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5MotionRegistryNextAction::ExpandMotionMeaning,
    };

    Ok(M5ResolvedMotionEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_demands_accessible_fallback: input
            .semantic_role
            .demands_accessible_fallback(),
        motion_role: input.motion_role.as_str().to_owned(),
        motion_role_delays_protected_input,
        surface_class: input.surface_class.as_str().to_owned(),
        surface_class_is_classified: input.surface_class.is_classified(),
        surface_class_is_protected_path: input.surface_class.is_protected_path(),
        reduced_motion_fallback: input.reduced_motion_fallback.as_str().to_owned(),
        reduced_motion_fallback_present: input.reduced_motion_fallback.is_present(),
        surface_context: input.surface_context.as_str().to_owned(),
        clamp_coverage: clamp_tokens(&input.clamp_coverage),
        covers_all_clamps: all_clamps,
        respects_input_priority: input.respects_input_priority,
        preserves_no_layout_shift: input.preserves_no_layout_shift,
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        motion_safe_on_protected_paths: degrade_reason.is_none(),
    })
}

/// Resolves a reduced-motion entry so it stays honest under every clamp: the entry names its canonical
/// token, reduced-motion role, semantic role, and surface context, covers all three clamps, keeps a static
/// fallback that preserves meaning, and traces to a canonical token rather than letting meaning ride on
/// motion alone.
pub fn resolve_reduced_motion_entry(
    input: M5ReducedMotionEntryResolutionInput,
) -> Result<M5ResolvedReducedMotionEntry, M5MotionResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5MotionResolutionError::EmptyReducedMotionEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5MotionResolutionError::ForbiddenMaterial);
    }

    let reduced_motion_role_is_motion_only = matches!(
        input.reduced_motion_role,
        M5ReducedMotionRole::MotionOnlyMeaningDisallowed
    );
    let all_clamps = covers_all_clamps(&input.clamp_coverage);

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5ReducedMotionEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ReducedMotionEntryDegradeReason::SurfaceContextUnresolved)
    } else if reduced_motion_role_is_motion_only || !input.references_canonical_token {
        Some(M5ReducedMotionEntryDegradeReason::MotionOnlyMeaningWithoutFallback)
    } else if !all_clamps {
        Some(M5ReducedMotionEntryDegradeReason::ClampCoverageIncomplete)
    } else if !input.static_fallback_preserves_meaning {
        Some(M5ReducedMotionEntryDegradeReason::StaticFallbackNotEquivalent)
    } else if !input.proof_fresh {
        Some(M5ReducedMotionEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5MotionRegistryNextAction::TraceCanonicalToken,
    };

    Ok(M5ResolvedReducedMotionEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        reduced_motion_role: input.reduced_motion_role.as_str().to_owned(),
        reduced_motion_role_is_motion_only,
        semantic_role: input.semantic_role.as_str().to_owned(),
        surface_context: input.surface_context.as_str().to_owned(),
        clamp_coverage: clamp_tokens(&input.clamp_coverage),
        covers_all_clamps: all_clamps,
        references_canonical_token: input.references_canonical_token,
        static_fallback_preserves_meaning: input.static_fallback_preserves_meaning,
        degrade_reason,
        next_action,
        fallback_preserves_meaning_across_clamps: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved motion and reduced-motion entries it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MotionRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5MotionRegistriesConsumerSurface,
    /// Qualification class earned by this row.
    pub qualification: M5VisualInteractionQualificationClass,
    /// Owner role accountable for keeping this row honest.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Deployment lines this row keeps the same truth across.
    pub deployment_lines: Vec<M5VisualInteractionDeploymentLine>,
    /// Mandatory labels this row must be able to show.
    pub required_labels: Vec<M5VisualInteractionRequiredLabel>,
    /// Non-visual accessibility routes offered.
    pub accessibility_routes: Vec<M5VisualInteractionAccessibilityRoute>,
    /// Anatomy parts this row must be able to show (must include the mandatory three).
    pub anatomy_parts: Vec<M5MotionRegistryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5MotionRegistryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5VisualInteractionDowngradeTrigger>,
    /// Resolved motion-registry examples.
    pub motion_entries: Vec<M5ResolvedMotionEntry>,
    /// Resolved reduced-motion examples.
    pub reduced_motion_entries: Vec<M5ResolvedReducedMotionEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical motion domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: motion never delays input on a protected path. MUST be `false`.
    pub motion_delays_protected_input: bool,
    /// Hard invariant: a raw duration value is never inlined instead of a canonical token. MUST be `false`.
    pub raw_duration_value_inlined_instead_of_token: bool,
    /// Hard invariant: motion never introduces a layout shift on a protected surface. MUST be `false`.
    pub layout_shift_on_protected_surface: bool,
    /// Hard invariant: the reduced-motion / power-saver / thermal clamp coverage is never incomplete. MUST
    /// be `false`.
    pub clamp_coverage_incomplete: bool,
}

impl M5MotionRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5MotionRegistryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5MotionRegistryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5MotionRegistryExportField> =
            self.export_fields.iter().copied().collect();
        M5MotionRegistryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.motion_delays_protected_input
            && !self.raw_duration_value_inlined_instead_of_token
            && !self.layout_shift_on_protected_surface
            && !self.clamp_coverage_incomplete
    }

    /// True when a clean motion entry preserves protected-path safety: it traces to a canonical token,
    /// respects input priority, never names the disallowed protected-input-delaying role, pairs a
    /// reduced-motion fallback, keeps a classified surface class, covers all three clamps, and introduces no
    /// layout shift.
    fn motion_is_honest(ex: &M5ResolvedMotionEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && ex.respects_input_priority
                && !ex.motion_role_delays_protected_input
                && ex.reduced_motion_fallback_present
                && ex.surface_class_is_classified
                && ex.covers_all_clamps
                && ex.preserves_no_layout_shift)
    }

    /// True when a clean reduced-motion entry preserves clamp safety: it traces to a canonical token, never
    /// names the disallowed motion-only role, covers all three clamps, and keeps a static fallback that
    /// preserves meaning.
    fn reduced_is_honest(ex: &M5ResolvedReducedMotionEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && !ex.reduced_motion_role_is_motion_only
                && ex.covers_all_clamps
                && ex.static_fallback_preserves_meaning)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.motion_entries.iter().all(Self::motion_is_honest)
            && self
                .reduced_motion_entries
                .iter()
                .all(Self::reduced_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MotionRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Motion-role tokens (bound from the frozen matrix).
    pub motion_roles: Vec<String>,
    /// Reduced-motion-role tokens (bound from the frozen matrix).
    pub reduced_motion_roles: Vec<String>,
    /// Motion-clamp tokens (minted by this lane).
    pub motion_clamps: Vec<String>,
    /// Motion-surface-class tokens (minted by this lane).
    pub motion_surface_classes: Vec<String>,
    /// Reduced-motion-fallback tokens (minted by this lane).
    pub reduced_motion_fallbacks: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Motion-entry degrade-reason tokens.
    pub motion_degrade_reasons: Vec<String>,
    /// Reduced-motion-entry degrade-reason tokens.
    pub reduced_motion_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5MotionRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5VisualInteractionRole::ALL, |v| v.as_str()),
            motion_roles: tokens(&M5MotionTokenRole::ALL, |v| v.as_str()),
            reduced_motion_roles: tokens(&M5ReducedMotionRole::ALL, |v| v.as_str()),
            motion_clamps: tokens(&M5MotionClamp::ALL, |v| v.as_str()),
            motion_surface_classes: tokens(&M5MotionSurfaceClass::ALL, |v| v.as_str()),
            reduced_motion_fallbacks: tokens(&M5ReducedMotionFallback::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5MotionSurfaceContext::ALL, |v| v.as_str()),
            motion_degrade_reasons: tokens(&M5MotionEntryDegradeReason::ALL, |v| v.as_str()),
            reduced_motion_degrade_reasons: tokens(&M5ReducedMotionEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5MotionRegistryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5MotionRegistryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5MotionRegistryExportField::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5VisualInteractionConsumerSurface::ALL, |v| v.as_str()),
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
pub struct M5MotionRegistriesGovernanceReview {
    /// The motion registry names a canonical token, motion role, and surface class for every entry.
    pub motion_registry_names_token_role_and_surface_class: bool,
    /// Duration and easing families clarify origin, continuity, and completion.
    pub duration_easing_families_clarify_origin_and_completion: bool,
    /// Motion never delays input on a protected path.
    pub motion_never_delays_protected_input: bool,
    /// Every motion entry covers the reduced-motion / power-saver / thermal clamps.
    pub every_motion_entry_covers_all_clamps: bool,
    /// Reduced-motion, power-saver, and thermal clamps are respected with a static fallback.
    pub reduced_motion_power_saver_thermal_clamps_respected: bool,
    /// Reduced-motion entries name a static fallback rather than letting meaning ride on motion alone.
    pub reduced_motion_names_static_fallback_not_motion_only: bool,
    /// Motion introduces no layout shift on protected or typing-adjacent surfaces.
    pub motion_preserves_no_layout_shift: bool,
    /// Protected-path animation regressions are caught by fixtures before release evidence turns green.
    pub protected_path_animation_caught_before_release: bool,
    /// The first shell / dialog / panel / embedded consumers use the canonical motion grammar.
    pub first_consumers_use_canonical_motion_grammar: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MotionRegistriesConsumerProjection {
    /// The shell surface consumes the shared motion / reduced-motion registries.
    pub shell_consumes_shared_registries: bool,
    /// The dialog surface consumes the shared motion / reduced-motion registries.
    pub dialog_consumes_shared_registries: bool,
    /// The panel surface consumes the shared motion / reduced-motion registries.
    pub panel_consumes_shared_registries: bool,
    /// The embedded and notification surfaces consume the shared motion / reduced-motion registries.
    pub embedded_and_notification_consume_shared_registries: bool,
    /// Motion behavior traces back to one canonical motion domain contract.
    pub motion_meaning_traces_to_single_domain_contract: bool,
    /// Support / export reads a single canonical motion / reduced-motion registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MotionRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MotionRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting visual-interaction audit for the lane.
    pub interaction_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5MotionRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MotionRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5MotionRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MotionRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MotionRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MotionRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MotionRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MotionRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 motion-token and reduced-motion registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MotionRegistriesPacket {
    /// Record kind; must equal [`M5_MOTION_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_MOTION_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5MotionRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5MotionRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5MotionRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5MotionRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5MotionRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5MotionRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5MotionRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5MotionRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_MOTION_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_MOTION_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5MotionRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_MOTION_REGISTRIES_RECORD_KIND {
            violations.push(M5MotionRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_MOTION_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5MotionRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5MotionRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5MotionRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 motion / reduced-motion registries packet serializes"),
        ) {
            violations.push(M5MotionRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 motion / reduced-motion registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,motion_entries,reduced_motion_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .motion_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.reduced_motion_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.motion_entries.len(),
                row.reduced_motion_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Motion-Token and Reduced-Motion Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Motion surface classes: {}\n",
            self.vocabulary_set.motion_surface_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Motion clamps: {}\n",
            self.vocabulary_set.motion_clamps.join(", ")
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
                "  - Motion entries: {} / reduced-motion entries: {}\n",
                row.motion_entries.len(),
                row.reduced_motion_entries.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5MotionRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5MotionRegistriesViolation>),
}

impl fmt::Display for M5MotionRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 motion / reduced-motion registries export parse failed: {error}"
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
                    "m5 motion / reduced-motion registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5MotionRegistriesArtifactError {}

/// Validation failures emitted by [`M5MotionRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5MotionRegistriesViolation {
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
    /// A registry row does not point at the canonical motion domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (protected-path-delaying, layout-shifting,
    /// raw-duration, clamp-incomplete, or a reduced-motion entry that rides on motion alone or drops its
    /// fallback).
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
    /// First-consumer canonical adoption is not proven: clean motion entries do not cover the canonical
    /// semantic-role families or the first shell / dialog / panel / embedded / notification surfaces, no
    /// raw-duration example degrades, or a clean entry inlines a raw duration.
    FirstConsumersUseCanonicalMotionGrammarNotProven,
    /// Protected-path safety across clamps is not proven: clean motion entries do not cover the protected
    /// command-palette / menu / typing / inline-editor / diagnostic surface classes with full clamp
    /// coverage while respecting input priority and preserving no layout shift, no clamp-incomplete or
    /// protected-path-delay example degrades, or a clean entry delays input or shifts layout.
    ProtectedPathSafetyAcrossClampsNotProven,
    /// Protected-path animation drift is not detectable: no raw-duration motion example and no
    /// motion-only reduced-motion example degrade, clean entries do not trace to a canonical token, or a
    /// clean entry inlines a raw value.
    ProtectedPathAnimationDriftNotDetectableNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5MotionRegistriesViolation {
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
            Self::FirstConsumersUseCanonicalMotionGrammarNotProven => {
                "first_consumers_use_canonical_motion_grammar_not_proven"
            }
            Self::ProtectedPathSafetyAcrossClampsNotProven => {
                "protected_path_safety_across_clamps_not_proven"
            }
            Self::ProtectedPathAnimationDriftNotDetectableNotProven => {
                "protected_path_animation_drift_not_detectable_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_motion_reduced_motion_registries_export(
) -> Result<M5MotionRegistriesPacket, M5MotionRegistriesArtifactError> {
    let packet: M5MotionRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-motion-token-and-reduced-motion-registries-proof/support_export.json"
    )))
    .map_err(M5MotionRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5MotionRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5MotionRegistriesPacket,
    violations: &mut Vec<M5MotionRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_MOTION_REGISTRIES_SCHEMA_REF,
        M5_MOTION_REGISTRIES_DOC_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF,
        M5_MOTION_AND_REDUCED_MOTION_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5MotionRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5MotionRegistriesPacket,
    violations: &mut Vec<M5MotionRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5MotionRegistriesViolation::NoRegistryRows);
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
            violations.push(M5MotionRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5MotionRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5MotionRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_MOTION_AND_REDUCED_MOTION_SCHEMA_REF) {
            violations.push(M5MotionRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.motion_entries.is_empty() || row.reduced_motion_entries.is_empty() {
            violations.push(M5MotionRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5MotionRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5MotionRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5MotionRegistriesPacket,
    violations: &mut Vec<M5MotionRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.motion_registry_names_token_role_and_surface_class,
        review.duration_easing_families_clarify_origin_and_completion,
        review.motion_never_delays_protected_input,
        review.every_motion_entry_covers_all_clamps,
        review.reduced_motion_power_saver_thermal_clamps_respected,
        review.reduced_motion_names_static_fallback_not_motion_only,
        review.motion_preserves_no_layout_shift,
        review.protected_path_animation_caught_before_release,
        review.first_consumers_use_canonical_motion_grammar,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5MotionRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5MotionRegistriesPacket,
    violations: &mut Vec<M5MotionRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_shared_registries,
        projection.dialog_consumes_shared_registries,
        projection.panel_consumes_shared_registries,
        projection.embedded_and_notification_consume_shared_registries,
        projection.motion_meaning_traces_to_single_domain_contract,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(M5MotionRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5MotionRegistriesPacket,
    violations: &mut Vec<M5MotionRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5MotionRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5MotionRegistriesPacket,
    violations: &mut Vec<M5MotionRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.interaction_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5MotionRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely
/// asserted by governance bools.
fn validate_acceptance_criteria(
    packet: &M5MotionRegistriesPacket,
    violations: &mut Vec<M5MotionRegistriesViolation>,
) {
    let motions = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.motion_entries.iter())
    };
    let reduced = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.reduced_motion_entries.iter())
    };

    // AC1: the first claimed consumers use one canonical motion grammar instead of feature-local
    // transitions. Clean motion entries cover the motion / attention semantic-role families and the first
    // shell / dialog / panel / embedded / notification surfaces, a raw-duration example degrades, and no
    // clean entry inlines a raw duration.
    let clean_semantic_roles: BTreeSet<String> = motions()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.semantic_role.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = motions()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let semantic_families_covered = ["motion", "attention"]
        .iter()
        .all(|r| clean_semantic_roles.contains(*r));
    let first_surfaces_covered = M5MotionSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let raw_duration_degrades = motions()
        .any(|ex| ex.degrade_reason == Some(M5MotionEntryDegradeReason::RawDurationValueInlined));
    let no_clean_raw_motion = !motions().any(|ex| ex.is_clean() && !ex.references_canonical_token);
    if !(semantic_families_covered
        && first_surfaces_covered
        && raw_duration_degrades
        && no_clean_raw_motion)
    {
        violations
            .push(M5MotionRegistriesViolation::FirstConsumersUseCanonicalMotionGrammarNotProven);
    }

    // AC2: protected input paths are not delayed by decorative motion and reduced-motion behavior is
    // explicit and testable. Clean motion entries cover every protected-path surface class with full clamp
    // coverage while respecting input priority and preserving no layout shift, a clamp-incomplete example
    // degrades, a protected-path-delay example degrades, and no clean entry delays input or shifts layout.
    let clean_protected_classes: BTreeSet<String> = motions()
        .filter(|ex| {
            ex.is_clean()
                && ex.surface_class_is_protected_path
                && ex.covers_all_clamps
                && ex.respects_input_priority
                && ex.preserves_no_layout_shift
        })
        .map(|ex| ex.surface_class.clone())
        .collect();
    let protected_classes_covered = M5MotionSurfaceClass::PROTECTED_PATHS
        .iter()
        .all(|s| clean_protected_classes.contains(s.as_str()));
    let clamp_incomplete_degrades = motions()
        .any(|ex| ex.degrade_reason == Some(M5MotionEntryDegradeReason::ClampCoverageIncomplete));
    let protected_delayed_degrades = motions().any(|ex| {
        ex.degrade_reason == Some(M5MotionEntryDegradeReason::ProtectedPathDelayedByMotion)
    });
    let no_clean_unsafe = !motions()
        .any(|ex| ex.is_clean() && (!ex.respects_input_priority || !ex.preserves_no_layout_shift));
    if !(protected_classes_covered
        && clamp_incomplete_degrades
        && protected_delayed_degrades
        && no_clean_unsafe)
    {
        violations.push(M5MotionRegistriesViolation::ProtectedPathSafetyAcrossClampsNotProven);
    }

    // AC3: animation regressions are detectable by fixtures or release evidence before promotion. A
    // raw-duration motion example and a motion-only reduced-motion example both degrade, at least one clean
    // motion and one clean reduced-motion entry trace to a canonical token, and no clean entry inlines a
    // raw value.
    let motion_only_degrades = reduced().any(|ex| {
        ex.degrade_reason
            == Some(M5ReducedMotionEntryDegradeReason::MotionOnlyMeaningWithoutFallback)
    });
    let traceable_motion = motions().any(|ex| ex.is_clean() && ex.references_canonical_token);
    let traceable_reduced = reduced().any(|ex| ex.is_clean() && ex.references_canonical_token);
    let no_clean_raw_reduced = !reduced().any(|ex| ex.is_clean() && !ex.references_canonical_token);
    if !(raw_duration_degrades
        && motion_only_degrades
        && traceable_motion
        && traceable_reduced
        && no_clean_raw_motion
        && no_clean_raw_reduced)
    {
        violations
            .push(M5MotionRegistriesViolation::ProtectedPathAnimationDriftNotDetectableNotProven);
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

/// The two interaction families this lane implements, for downstream reference.
pub const IMPLEMENTED_FAMILIES: [M5VisualInteractionFamily; 2] = [
    M5VisualInteractionFamily::MotionToken,
    M5VisualInteractionFamily::ReducedMotion,
];
