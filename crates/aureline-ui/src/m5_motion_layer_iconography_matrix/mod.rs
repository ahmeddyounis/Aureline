//! Frozen M5 motion-token, reduced-motion, opacity / scrim, layer-order, portal-ownership,
//! iconography, and illustration-boundary visual-interaction matrix.
//!
//! This module locks Aureline's motion, overlay, layering, symbol, and illustration grammar into one
//! export-safe packet. Every claimed M5 surface that still describes its own animation curves,
//! reduced-motion behavior, scrim / opacity rules, z-order tiers, portal ownership, icon semantics, or
//! illustration limits — across the desktop shell, dialogs, onboarding, notifications, and embedded
//! surfaces — is named once here and constrained by the same shared interaction-role taxonomy (motion,
//! overlay, layer, portal, icon, illustration, attention), the same never-delay-protected-input rule, the
//! same reduced-motion / power-saver / thermal clamp respect, the same scrim orientation-and-contrast
//! rule, the same single z-order model with owning-surface attachment, the same semantic-and-labeled icon
//! rule, and the same illustration-stays-secondary rule regardless of the feature family that renders it.
//!
//! The matrix does not re-open notification routing, dialog semantics, or browser-boundary authority — it
//! is the shared reusable visual-interaction contract those flows consume, and it binds back to the
//! already-landed design-system foundations and publication packets instead of leaving the grammar split
//! across prose and screenshots. The controlled vocabularies are frozen in one self-describing
//! [`M5VisualInteractionVocabularySet`] rather than minted per feature. The single controlled
//! interaction-role vocabulary consumers bind to — motion, overlay, layer, portal, icon, illustration, and
//! attention — keeps motion from delaying input on protected paths, keeps scrims from erasing workspace
//! orientation or contrast, keeps extension and private overlays from bypassing the shared z-order model,
//! keeps uncommon or destructive icons from shipping unlabeled, and keeps illustrations from masquerading
//! as trust, severity, or operational truth. Raw secret values and private endpoints stay outside the
//! export boundary.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_motion_layer_iconography_matrix,
    seeded_m5_motion_layer_iconography_matrix_illustration_preview_narrowed,
    seeded_m5_motion_layer_iconography_matrix_reduced_motion_beta_narrowed,
    M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5MotionLayerIconographyMatrixPacket`].
pub const M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_RECORD_KIND: &str =
    "freeze_m5_motion_token_opacity_scrim_layer_order_portal_iconography_and_illustration_boundary_matrix";

/// Schema version for M5 motion / layer / iconography matrix records.
pub const M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined visual-interaction matrix schema.
pub const M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF: &str =
    "schemas/design-system/m5-motion-layer-iconography-matrix.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF: &str =
    "docs/design-system/m5_motion_layer_iconography_contract.md";

/// Repo-relative path of the canonical motion / reduced-motion domain schema.
pub const M5_MOTION_AND_REDUCED_MOTION_SCHEMA_REF: &str =
    "schemas/design-system/m5-motion-and-reduced-motion.schema.json";

/// Repo-relative path of the canonical opacity / scrim domain schema.
pub const M5_OPACITY_SCRIM_SCHEMA_REF: &str = "schemas/design-system/m5-opacity-scrim.schema.json";

/// Repo-relative path of the canonical layer-order / portal domain schema.
pub const M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF: &str =
    "schemas/design-system/m5-layer-order-and-portal.schema.json";

/// Repo-relative path of the canonical iconography / illustration domain schema.
pub const M5_ICONOGRAPHY_AND_ILLUSTRATION_SCHEMA_REF: &str =
    "schemas/design-system/m5-iconography-and-illustration.schema.json";

/// Repo-relative path of the already-landed design-system foundations artifact the matrix binds to.
pub const M5_DESIGN_SYSTEM_FOUNDATIONS_SCHEMA_REF: &str =
    "schemas/design-system/m5-foundations.schema.json";

/// Repo-relative path of the already-landed design-system publication (foundation-package) schema the
/// matrix binds to.
pub const M5_DESIGN_SYSTEM_FOUNDATION_PACKAGE_SCHEMA_REF: &str =
    "schemas/design-system/m5-foundation-package.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_MOTION_LAYER_ICONOGRAPHY_FIXTURE_DIR: &str = "fixtures/ui/m5-motion-layer-iconography";

/// Repo-relative path of the checked support-export artifact.
pub const M5_MOTION_LAYER_ICONOGRAPHY_ARTIFACT_REF: &str =
    "artifacts/release/m5-motion-layer-iconography-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_MOTION_LAYER_ICONOGRAPHY_CSV_REF: &str =
    "artifacts/release/m5-motion-layer-iconography-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_MOTION_LAYER_ICONOGRAPHY_REPORT_REF: &str =
    "artifacts/design-system/m5-motion-layer-iconography.md";

/// One of the seven governed visual-interaction families this matrix freezes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionFamily {
    /// Motion tokens: duration / easing families that clarify origin, continuity, and completion.
    MotionToken,
    /// Reduced-motion behavior: reduced-motion, power-saver, and thermal clamps with static fallbacks.
    ReducedMotion,
    /// Opacity / scrim classes that preserve orientation and contrast.
    OpacityScrim,
    /// Z-order tiers that follow one shared layering model.
    LayerOrder,
    /// Portal ownership that keeps overlays attached to their owning surface.
    PortalOwnership,
    /// Iconography: semantic, labeled icon categories.
    Iconography,
    /// Illustration boundaries that keep illustrations secondary to operational truth.
    Illustration,
}

impl M5VisualInteractionFamily {
    /// Every governed interaction family, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::MotionToken,
        Self::ReducedMotion,
        Self::OpacityScrim,
        Self::LayerOrder,
        Self::PortalOwnership,
        Self::Iconography,
        Self::Illustration,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MotionToken => "motion_token",
            Self::ReducedMotion => "reduced_motion",
            Self::OpacityScrim => "opacity_scrim",
            Self::LayerOrder => "layer_order",
            Self::PortalOwnership => "portal_ownership",
            Self::Iconography => "iconography",
            Self::Illustration => "illustration",
        }
    }

    /// The canonical per-domain schema ref a downstream surface points at instead of restating this
    /// family's motion, scrim, layering, icon, or illustration meaning by hand.
    pub const fn canonical_domain_schema_ref(self) -> &'static str {
        match self {
            Self::MotionToken | Self::ReducedMotion => M5_MOTION_AND_REDUCED_MOTION_SCHEMA_REF,
            Self::OpacityScrim => M5_OPACITY_SCRIM_SCHEMA_REF,
            Self::LayerOrder | Self::PortalOwnership => M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF,
            Self::Iconography | Self::Illustration => M5_ICONOGRAPHY_AND_ILLUSTRATION_SCHEMA_REF,
        }
    }

    /// `true` when this family must name a controlled motion-token role.
    pub const fn declares_motion_roles(self) -> bool {
        matches!(self, Self::MotionToken)
    }

    /// `true` when this family must name a controlled reduced-motion role.
    pub const fn declares_reduced_motion_roles(self) -> bool {
        matches!(self, Self::ReducedMotion)
    }

    /// `true` when this family must name a controlled opacity / scrim role.
    pub const fn declares_opacity_scrim_roles(self) -> bool {
        matches!(self, Self::OpacityScrim)
    }

    /// `true` when this family must name a controlled layer-order role.
    pub const fn declares_layer_order_roles(self) -> bool {
        matches!(self, Self::LayerOrder)
    }

    /// `true` when this family must name a controlled portal-ownership role.
    pub const fn declares_portal_ownership_roles(self) -> bool {
        matches!(self, Self::PortalOwnership)
    }

    /// `true` when this family must name a controlled iconography role.
    pub const fn declares_iconography_roles(self) -> bool {
        matches!(self, Self::Iconography)
    }

    /// `true` when this family must name a controlled illustration role.
    pub const fn declares_illustration_roles(self) -> bool {
        matches!(self, Self::Illustration)
    }
}

/// The single controlled interaction-role vocabulary every desktop, dialog, onboarding, notification, or
/// embedded consumer binds to. These are the exact acceptance-criteria tokens that keep `motion`,
/// `overlay`, `layer`, `portal`, `icon`, `illustration`, and `attention` meaning the same thing everywhere
/// the visual-interaction grammar ships. No feature family invents a parallel word for any of these roles,
/// and the meaning-bearing roles may never be conveyed by motion, decoration, or an unlabeled symbol alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionRole {
    /// Motion / animation role (origin, continuity, completion).
    Motion,
    /// Overlay / scrim role.
    Overlay,
    /// Layer / z-order role.
    Layer,
    /// Portal / owning-surface attachment role.
    Portal,
    /// Icon / symbol role.
    Icon,
    /// Illustration / decorative role.
    Illustration,
    /// Attention-routing role.
    Attention,
}

impl M5VisualInteractionRole {
    /// Every interaction role token, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Motion,
        Self::Overlay,
        Self::Layer,
        Self::Portal,
        Self::Icon,
        Self::Illustration,
        Self::Attention,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Motion => "motion",
            Self::Overlay => "overlay",
            Self::Layer => "layer",
            Self::Portal => "portal",
            Self::Icon => "icon",
            Self::Illustration => "illustration",
            Self::Attention => "attention",
        }
    }

    /// Whether this role carries meaning that must never be conveyed by motion, decoration, or an
    /// unlabeled symbol alone and must always pair with a reduced-motion-safe, labeled, or announced
    /// fallback (`motion`, `overlay`, `icon`, `illustration`, `attention`).
    pub const fn demands_accessible_fallback(self) -> bool {
        matches!(
            self,
            Self::Motion | Self::Overlay | Self::Icon | Self::Illustration | Self::Attention
        )
    }
}

/// Controlled motion-token role — how motion is named, so duration and easing families clarify origin,
/// continuity, and completion and never delay input on a protected path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MotionTokenRole {
    /// A duration family (fast / standard / slow tiers).
    DurationFamily,
    /// An easing family (entrance / exit / emphasis curves).
    EasingFamily,
    /// Motion that clarifies origin and continuity.
    OriginContinuityCue,
    /// Motion that clarifies completion.
    CompletionCue,
    /// Motion that respects input priority on protected paths.
    RespectsInputPriority,
    /// Motion that delays input on a protected path, which is disallowed.
    MotionDelaysProtectedInputDisallowed,
}

impl M5MotionTokenRole {
    /// Every motion-token role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DurationFamily,
        Self::EasingFamily,
        Self::OriginContinuityCue,
        Self::CompletionCue,
        Self::RespectsInputPriority,
        Self::MotionDelaysProtectedInputDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurationFamily => "duration_family",
            Self::EasingFamily => "easing_family",
            Self::OriginContinuityCue => "origin_continuity_cue",
            Self::CompletionCue => "completion_cue",
            Self::RespectsInputPriority => "respects_input_priority",
            Self::MotionDelaysProtectedInputDisallowed => {
                "motion_delays_protected_input_disallowed"
            }
        }
    }
}

/// Controlled reduced-motion role — how reduced-motion behavior is named, so reduced-motion, power-saver,
/// and thermal clamps are honored and no motion carries the only cue.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReducedMotionRole {
    /// The reduced-motion clamp.
    ReducedMotionClamp,
    /// The power-saver clamp.
    PowerSaverClamp,
    /// The thermal clamp.
    ThermalClamp,
    /// A static fallback that preserves the same meaning without motion.
    StaticFallbackEquivalent,
    /// Respects the user's reduced-motion preference.
    RespectsUserPreference,
    /// Meaning conveyed by motion alone, which is disallowed.
    MotionOnlyMeaningDisallowed,
}

impl M5ReducedMotionRole {
    /// Every reduced-motion role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ReducedMotionClamp,
        Self::PowerSaverClamp,
        Self::ThermalClamp,
        Self::StaticFallbackEquivalent,
        Self::RespectsUserPreference,
        Self::MotionOnlyMeaningDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReducedMotionClamp => "reduced_motion_clamp",
            Self::PowerSaverClamp => "power_saver_clamp",
            Self::ThermalClamp => "thermal_clamp",
            Self::StaticFallbackEquivalent => "static_fallback_equivalent",
            Self::RespectsUserPreference => "respects_user_preference",
            Self::MotionOnlyMeaningDisallowed => "motion_only_meaning_disallowed",
        }
    }
}

/// Controlled opacity / scrim role — how scrims and opacity levels are named, so a scrim preserves
/// workspace orientation and text contrast and always offers a dismiss affordance.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OpacityScrimRole {
    /// A scrim layer.
    ScrimLayer,
    /// An opacity level.
    OpacityLevel,
    /// Orientation is preserved under the scrim.
    OrientationPreserved,
    /// Contrast is preserved under the scrim.
    ContrastPreserved,
    /// A dismiss affordance for the scrim.
    DismissAffordance,
    /// A scrim that erases orientation or contrast, which is disallowed.
    ScrimErasesOrientationOrContrastDisallowed,
}

impl M5OpacityScrimRole {
    /// Every opacity / scrim role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ScrimLayer,
        Self::OpacityLevel,
        Self::OrientationPreserved,
        Self::ContrastPreserved,
        Self::DismissAffordance,
        Self::ScrimErasesOrientationOrContrastDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ScrimLayer => "scrim_layer",
            Self::OpacityLevel => "opacity_level",
            Self::OrientationPreserved => "orientation_preserved",
            Self::ContrastPreserved => "contrast_preserved",
            Self::DismissAffordance => "dismiss_affordance",
            Self::ScrimErasesOrientationOrContrastDisallowed => {
                "scrim_erases_orientation_or_contrast_disallowed"
            }
        }
    }
}

/// Controlled layer-order role — how z-order tiers are named, so every menu, popover, dialog, and toast
/// stacks under one shared model no private overlay can bypass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LayerOrderRole {
    /// The base workspace-content tier.
    BaseContentTier,
    /// The overlay (menu / popover) tier.
    OverlayTier,
    /// The dialog / modal tier.
    DialogTier,
    /// The notification / toast tier.
    NotificationTier,
    /// A single shared z-order model.
    SingleZOrderModel,
    /// A private layer that bypasses the shared z-order, which is disallowed.
    PrivateLayerBypassDisallowed,
}

impl M5LayerOrderRole {
    /// Every layer-order role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::BaseContentTier,
        Self::OverlayTier,
        Self::DialogTier,
        Self::NotificationTier,
        Self::SingleZOrderModel,
        Self::PrivateLayerBypassDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BaseContentTier => "base_content_tier",
            Self::OverlayTier => "overlay_tier",
            Self::DialogTier => "dialog_tier",
            Self::NotificationTier => "notification_tier",
            Self::SingleZOrderModel => "single_z_order_model",
            Self::PrivateLayerBypassDisallowed => "private_layer_bypass_disallowed",
        }
    }
}

/// Controlled portal-ownership role — how portals attach to their owning surface, so every overlay
/// contains focus, dismisses with its owner, and stacks under the shared z-order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortalOwnershipRole {
    /// A portal attached to its owning surface.
    OwningSurfaceAttachment,
    /// A portal that contains focus within its owning scope.
    FocusScopeContainment,
    /// A portal that dismisses when its owner dismisses.
    OwnerDrivenDismissal,
    /// A portal that stacks under the shared z-order model.
    StacksUnderSharedModel,
    /// An extension / embedded portal governed by the shared model.
    ExtensionPortalGoverned,
    /// A portal detached from its owning surface, which is disallowed.
    DetachedPortalDisallowed,
}

impl M5PortalOwnershipRole {
    /// Every portal-ownership role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OwningSurfaceAttachment,
        Self::FocusScopeContainment,
        Self::OwnerDrivenDismissal,
        Self::StacksUnderSharedModel,
        Self::ExtensionPortalGoverned,
        Self::DetachedPortalDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwningSurfaceAttachment => "owning_surface_attachment",
            Self::FocusScopeContainment => "focus_scope_containment",
            Self::OwnerDrivenDismissal => "owner_driven_dismissal",
            Self::StacksUnderSharedModel => "stacks_under_shared_model",
            Self::ExtensionPortalGoverned => "extension_portal_governed",
            Self::DetachedPortalDisallowed => "detached_portal_disallowed",
        }
    }
}

/// Controlled iconography role — how icon categories are named, so status, action, and navigation icons
/// stay semantic and carry a text label for uncommon or destructive actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IconographyRole {
    /// A status icon (info / success / warning / danger).
    StatusIcon,
    /// A common-action icon.
    ActionIcon,
    /// A navigation icon.
    NavigationIcon,
    /// An icon labeled for an uncommon or destructive action.
    LabeledForUncommonOrDestructive,
    /// An icon that stays semantic, not purely decorative.
    SemanticNotDecorative,
    /// An unlabeled icon for an uncommon or destructive action, which is disallowed.
    UnlabeledUncommonOrDestructiveDisallowed,
}

impl M5IconographyRole {
    /// Every iconography role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StatusIcon,
        Self::ActionIcon,
        Self::NavigationIcon,
        Self::LabeledForUncommonOrDestructive,
        Self::SemanticNotDecorative,
        Self::UnlabeledUncommonOrDestructiveDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StatusIcon => "status_icon",
            Self::ActionIcon => "action_icon",
            Self::NavigationIcon => "navigation_icon",
            Self::LabeledForUncommonOrDestructive => "labeled_for_uncommon_or_destructive",
            Self::SemanticNotDecorative => "semantic_not_decorative",
            Self::UnlabeledUncommonOrDestructiveDisallowed => {
                "unlabeled_uncommon_or_destructive_disallowed"
            }
        }
    }
}

/// Controlled illustration role — how illustration limits are named, so empty-state, onboarding, and
/// decorative illustrations stay secondary and never impersonate operational, safety, or security truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IllustrationRole {
    /// An empty-state illustration.
    EmptyStateIllustration,
    /// An onboarding illustration.
    OnboardingIllustration,
    /// A decorative accent.
    DecorativeAccent,
    /// An illustration kept secondary to content.
    SecondaryToContent,
    /// An illustration that never impersonates operational state.
    NeverImpersonatesState,
    /// An illustration used as operational truth, which is disallowed.
    IllustrationAsOperationalTruthDisallowed,
}

impl M5IllustrationRole {
    /// Every illustration role, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::EmptyStateIllustration,
        Self::OnboardingIllustration,
        Self::DecorativeAccent,
        Self::SecondaryToContent,
        Self::NeverImpersonatesState,
        Self::IllustrationAsOperationalTruthDisallowed,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyStateIllustration => "empty_state_illustration",
            Self::OnboardingIllustration => "onboarding_illustration",
            Self::DecorativeAccent => "decorative_accent",
            Self::SecondaryToContent => "secondary_to_content",
            Self::NeverImpersonatesState => "never_impersonates_state",
            Self::IllustrationAsOperationalTruthDisallowed => {
                "illustration_as_operational_truth_disallowed"
            }
        }
    }
}

/// Claimed M5 surface family that renders / consumes a visual-interaction family. No family may invent a
/// parallel surface taxonomy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionSurfaceFamily {
    /// The desktop shell surface.
    Desktop,
    /// The dialog surface.
    Dialog,
    /// The onboarding surface.
    Onboarding,
    /// The notification surface.
    Notification,
    /// The embedded / browser-handoff surface.
    Embedded,
    /// The support export.
    SupportExport,
}

impl M5VisualInteractionSurfaceFamily {
    /// Every surface family, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Desktop,
        Self::Dialog,
        Self::Onboarding,
        Self::Notification,
        Self::Embedded,
        Self::SupportExport,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Desktop => "desktop",
            Self::Dialog => "dialog",
            Self::Onboarding => "onboarding",
            Self::Notification => "notification",
            Self::Embedded => "embedded",
            Self::SupportExport => "support_export",
        }
    }
}

/// Deployment line a family must survive with the same truth, so a family's motion, scrim, layering, icon,
/// or illustration meaning never silently narrows or widens between deployment shapes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionDeploymentLine {
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

impl M5VisualInteractionDeploymentLine {
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

/// Subsystem that consumes a family's projection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionConsumerSurface {
    /// The shell UI.
    ShellUi,
    /// The editor UI.
    EditorUi,
    /// The help UI.
    HelpUi,
    /// The marketplace / extensions UI.
    MarketplaceUi,
    /// The onboarding UI.
    OnboardingUi,
    /// The settings UI.
    SettingsUi,
    /// The CLI / export path.
    CliExport,
    /// The support export.
    SupportExport,
    /// The general product UI.
    ProductUi,
}

impl M5VisualInteractionConsumerSurface {
    /// Every consumer surface, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ShellUi,
        Self::EditorUi,
        Self::HelpUi,
        Self::MarketplaceUi,
        Self::OnboardingUi,
        Self::SettingsUi,
        Self::CliExport,
        Self::SupportExport,
        Self::ProductUi,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellUi => "shell_ui",
            Self::EditorUi => "editor_ui",
            Self::HelpUi => "help_ui",
            Self::MarketplaceUi => "marketplace_ui",
            Self::OnboardingUi => "onboarding_ui",
            Self::SettingsUi => "settings_ui",
            Self::CliExport => "cli_export",
            Self::SupportExport => "support_export",
            Self::ProductUi => "product_ui",
        }
    }
}

/// Non-visual / accessibility route every family must offer so no motion, scrim, layering, icon, or
/// illustration meaning is hover-only, pointer-only, motion-only, or visually encoded alone. Records the
/// keyboard, screen-reader, high-zoom, reduced-motion, CLI/export, and support-packet requirements up
/// front.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionAccessibilityRoute {
    /// Reachable and operable by keyboard focus.
    KeyboardFocusable,
    /// Announced to a screen reader (via a non-visual cue / label).
    ScreenReaderAnnounced,
    /// Reflows legibly at high zoom.
    HighZoomReflow,
    /// Legible and usable with reduced motion.
    ReducedMotionSafe,
    /// Reachable and inspectable through the CLI / export path.
    CliExportable,
    /// Present in the support / export packet, never renderer-only.
    SupportPacketPresent,
}

impl M5VisualInteractionAccessibilityRoute {
    /// Every accessibility route, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::KeyboardFocusable,
        Self::ScreenReaderAnnounced,
        Self::HighZoomReflow,
        Self::ReducedMotionSafe,
        Self::CliExportable,
        Self::SupportPacketPresent,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusable => "keyboard_focusable",
            Self::ScreenReaderAnnounced => "screen_reader_announced",
            Self::HighZoomReflow => "high_zoom_reflow",
            Self::ReducedMotionSafe => "reduced_motion_safe",
            Self::CliExportable => "cli_exportable",
            Self::SupportPacketPresent => "support_packet_present",
        }
    }
}

/// Reason a visual-interaction family has degraded below its qualified state. Required on every row so a
/// stale, unresolved, or narrowed fallback is never left implicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionDegradedReason {
    /// Proof has gone stale.
    ProofStale,
    /// The motion-token source is unavailable.
    MotionTokensUnavailable,
    /// Scrim contrast / orientation metrics are unavailable.
    ScrimMetricsUnavailable,
    /// The shared layer / z-order model is unavailable.
    LayerModelUnavailable,
    /// The icon-label source is unavailable.
    IconLabelSourceUnavailable,
    /// The illustration boundary is unverified.
    IllustrationBoundaryUnverified,
}

impl M5VisualInteractionDegradedReason {
    /// Every degraded reason, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ProofStale,
        Self::MotionTokensUnavailable,
        Self::ScrimMetricsUnavailable,
        Self::LayerModelUnavailable,
        Self::IconLabelSourceUnavailable,
        Self::IllustrationBoundaryUnverified,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::MotionTokensUnavailable => "motion_tokens_unavailable",
            Self::ScrimMetricsUnavailable => "scrim_metrics_unavailable",
            Self::LayerModelUnavailable => "layer_model_unavailable",
            Self::IconLabelSourceUnavailable => "icon_label_source_unavailable",
            Self::IllustrationBoundaryUnverified => "illustration_boundary_unverified",
        }
    }
}

/// Mandatory label a claimed visual-interaction family must be able to show. The first three are hard
/// requirements on every family; the remaining three close the acceptance-criteria ambiguity about the
/// motion profile, the layer tier, and the accessible fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionRequiredLabel {
    /// The family's stable identity.
    Identity,
    /// The family's interaction role.
    SemanticRole,
    /// The canonical token reference the family points at.
    TokenReference,
    /// The motion profile (full / reduced-motion) the family covers.
    MotionProfile,
    /// The z-order / layer tier the family applies to.
    LayerTier,
    /// The accessible fallback (label / static / announced) paired with a visual cue.
    AccessibleFallback,
}

impl M5VisualInteractionRequiredLabel {
    /// Every declared label, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::Identity,
        Self::SemanticRole,
        Self::TokenReference,
        Self::MotionProfile,
        Self::LayerTier,
        Self::AccessibleFallback,
    ];

    /// The three labels every claimed family must be able to show.
    pub const MANDATORY: [Self; 3] = [Self::Identity, Self::SemanticRole, Self::TokenReference];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Identity => "identity",
            Self::SemanticRole => "semantic_role",
            Self::TokenReference => "token_reference",
            Self::MotionProfile => "motion_profile",
            Self::LayerTier => "layer_tier",
            Self::AccessibleFallback => "accessible_fallback",
        }
    }
}

/// Qualification class for an M5 visual-interaction row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionQualificationClass {
    /// Family qualifies for the Stable claim.
    Stable,
    /// Family is narrowed to Beta.
    Beta,
    /// Family is narrowed to Preview.
    Preview,
    /// Family is experimental and not claimed.
    Experimental,
    /// Family is unavailable on this build.
    Unavailable,
    /// Family is held pending upstream resolution.
    Held,
}

impl M5VisualInteractionQualificationClass {
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

    /// Whether the family may carry a public Stable claim.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::Stable)
    }
}

/// Downgrade trigger that narrows a visual-interaction family below its claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5VisualInteractionDowngradeTrigger {
    /// Motion delayed input on a protected path.
    MotionDelayedProtectedInput,
    /// Motion meaning was lost under reduced motion.
    MotionMeaningLostUnderReducedMotion,
    /// A scrim erased workspace orientation or contrast.
    ScrimErasedOrientationOrContrast,
    /// An overlay bypassed the shared z-order model.
    OverlayBypassedSharedZOrder,
    /// A portal detached from its owning surface.
    PortalDetachedFromOwningSurface,
    /// An uncommon or destructive action used an unlabeled icon.
    UnlabeledIconForUncommonOrDestructiveAction,
    /// An illustration impersonated operational state.
    IllustrationImpersonatedOperationalState,
    /// An icon's semantics became ambiguous.
    IconSemanticsAmbiguous,
    /// A family left its layer tier unstated.
    LayerTierUnstated,
    /// A family left its interaction role unstated.
    SemanticRoleUnstated,
    /// A family left its canonical token reference unstated.
    TokenReferenceUnstated,
    /// The proof packet has gone stale.
    ProofStale,
}

impl M5VisualInteractionDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::MotionDelayedProtectedInput,
        Self::MotionMeaningLostUnderReducedMotion,
        Self::ScrimErasedOrientationOrContrast,
        Self::OverlayBypassedSharedZOrder,
        Self::PortalDetachedFromOwningSurface,
        Self::UnlabeledIconForUncommonOrDestructiveAction,
        Self::IllustrationImpersonatedOperationalState,
        Self::IconSemanticsAmbiguous,
        Self::LayerTierUnstated,
        Self::SemanticRoleUnstated,
        Self::TokenReferenceUnstated,
        Self::ProofStale,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MotionDelayedProtectedInput => "motion_delayed_protected_input",
            Self::MotionMeaningLostUnderReducedMotion => "motion_meaning_lost_under_reduced_motion",
            Self::ScrimErasedOrientationOrContrast => "scrim_erased_orientation_or_contrast",
            Self::OverlayBypassedSharedZOrder => "overlay_bypassed_shared_z_order",
            Self::PortalDetachedFromOwningSurface => "portal_detached_from_owning_surface",
            Self::UnlabeledIconForUncommonOrDestructiveAction => {
                "unlabeled_icon_for_uncommon_or_destructive_action"
            }
            Self::IllustrationImpersonatedOperationalState => {
                "illustration_impersonated_operational_state"
            }
            Self::IconSemanticsAmbiguous => "icon_semantics_ambiguous",
            Self::LayerTierUnstated => "layer_tier_unstated",
            Self::SemanticRoleUnstated => "semantic_role_unstated",
            Self::TokenReferenceUnstated => "token_reference_unstated",
            Self::ProofStale => "proof_stale",
        }
    }
}

/// One row in the matrix: one governed visual-interaction family bound to the surface-specific truth it
/// must project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualInteractionRow {
    /// Governed interaction family.
    pub interaction_family: M5VisualInteractionFamily,
    /// Qualification class earned by this family.
    pub qualification: M5VisualInteractionQualificationClass,
    /// Owner role accountable for keeping this family governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this family.
    pub surface_families: Vec<M5VisualInteractionSurfaceFamily>,
    /// Deployment lines this family keeps the same truth across.
    pub deployment_lines: Vec<M5VisualInteractionDeploymentLine>,
    /// Mandatory labels this family must be able to show (must include the three
    /// [`M5VisualInteractionRequiredLabel::MANDATORY`] labels).
    pub required_labels: Vec<M5VisualInteractionRequiredLabel>,
    /// Interaction roles this family can carry (the frozen AC vocabulary; required on every family).
    pub semantic_roles: Vec<M5VisualInteractionRole>,
    /// Motion-token roles this family names (motion-token family only).
    pub motion_roles: Vec<M5MotionTokenRole>,
    /// Reduced-motion roles this family names (reduced-motion family only).
    pub reduced_motion_roles: Vec<M5ReducedMotionRole>,
    /// Opacity / scrim roles this family names (opacity-scrim family only).
    pub opacity_scrim_roles: Vec<M5OpacityScrimRole>,
    /// Layer-order roles this family names (layer-order family only).
    pub layer_order_roles: Vec<M5LayerOrderRole>,
    /// Portal-ownership roles this family names (portal-ownership family only).
    pub portal_ownership_roles: Vec<M5PortalOwnershipRole>,
    /// Iconography roles this family names (iconography family only).
    pub iconography_roles: Vec<M5IconographyRole>,
    /// Illustration roles this family names (illustration family only).
    pub illustration_roles: Vec<M5IllustrationRole>,
    /// Degraded reasons this family can name (required on every family).
    pub degraded_reasons: Vec<M5VisualInteractionDegradedReason>,
    /// Non-visual accessibility routes this family offers.
    pub accessibility_routes: Vec<M5VisualInteractionAccessibilityRoute>,
    /// Subsystems that consume this family's projection.
    pub consumer_surfaces: Vec<M5VisualInteractionConsumerSurface>,
    /// Downgrade triggers that apply to this family.
    pub downgrade_triggers: Vec<M5VisualInteractionDowngradeTrigger>,
    /// Proof packet refs that keep this family current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this family (must include its own canonical domain schema so
    /// downstream surfaces have one target to point at).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: this family never delays input on a protected path with motion. MUST be `false`.
    pub delays_protected_input_with_motion: bool,
    /// Hard invariant: this family never lets a scrim erase workspace orientation or contrast. MUST be
    /// `false`.
    pub scrim_erases_orientation_or_contrast: bool,
    /// Hard invariant: this family never lets an overlay bypass the shared z-order model. MUST be
    /// `false`.
    pub overlay_bypasses_shared_z_order: bool,
    /// Hard invariant: this family never uses an unlabeled icon for an uncommon or destructive action.
    /// MUST be `false`.
    pub uses_unlabeled_icon_for_uncommon_or_destructive_action: bool,
    /// Hard invariant: this family never lets an illustration impersonate operational or security truth.
    /// MUST be `false`.
    pub lets_illustration_impersonate_operational_or_security_truth: bool,
}

impl M5VisualInteractionRow {
    /// `true` when the row declares all mandatory labels.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5VisualInteractionRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5VisualInteractionRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.delays_protected_input_with_motion
            && !self.scrim_erases_orientation_or_contrast
            && !self.overlay_bypasses_shared_z_order
            && !self.uses_unlabeled_icon_for_uncommon_or_destructive_action
            && !self.lets_illustration_impersonate_operational_or_security_truth
    }
}

/// Self-describing controlled-vocabulary set frozen by the matrix.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualInteractionVocabularySet {
    /// Interaction-family tokens.
    pub interaction_families: Vec<String>,
    /// Interaction-role tokens.
    pub semantic_roles: Vec<String>,
    /// Motion-token-role tokens.
    pub motion_roles: Vec<String>,
    /// Reduced-motion-role tokens.
    pub reduced_motion_roles: Vec<String>,
    /// Opacity / scrim-role tokens.
    pub opacity_scrim_roles: Vec<String>,
    /// Layer-order-role tokens.
    pub layer_order_roles: Vec<String>,
    /// Portal-ownership-role tokens.
    pub portal_ownership_roles: Vec<String>,
    /// Iconography-role tokens.
    pub iconography_roles: Vec<String>,
    /// Illustration-role tokens.
    pub illustration_roles: Vec<String>,
    /// Surface-family tokens.
    pub surface_families: Vec<String>,
    /// Deployment-line tokens.
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens.
    pub accessibility_routes: Vec<String>,
    /// Degraded-reason tokens.
    pub degraded_reasons: Vec<String>,
    /// Required-label tokens.
    pub required_labels: Vec<String>,
    /// Downgrade-trigger tokens.
    pub downgrade_triggers: Vec<String>,
}

impl M5VisualInteractionVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            interaction_families: tokens(&M5VisualInteractionFamily::ALL, |v| v.as_str()),
            semantic_roles: tokens(&M5VisualInteractionRole::ALL, |v| v.as_str()),
            motion_roles: tokens(&M5MotionTokenRole::ALL, |v| v.as_str()),
            reduced_motion_roles: tokens(&M5ReducedMotionRole::ALL, |v| v.as_str()),
            opacity_scrim_roles: tokens(&M5OpacityScrimRole::ALL, |v| v.as_str()),
            layer_order_roles: tokens(&M5LayerOrderRole::ALL, |v| v.as_str()),
            portal_ownership_roles: tokens(&M5PortalOwnershipRole::ALL, |v| v.as_str()),
            iconography_roles: tokens(&M5IconographyRole::ALL, |v| v.as_str()),
            illustration_roles: tokens(&M5IllustrationRole::ALL, |v| v.as_str()),
            surface_families: tokens(&M5VisualInteractionSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5VisualInteractionDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5VisualInteractionConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5VisualInteractionAccessibilityRoute::ALL, |v| {
                v.as_str()
            }),
            degraded_reasons: tokens(&M5VisualInteractionDegradedReason::ALL, |v| v.as_str()),
            required_labels: tokens(&M5VisualInteractionRequiredLabel::ALL, |v| v.as_str()),
            downgrade_triggers: tokens(&M5VisualInteractionDowngradeTrigger::ALL, |v| v.as_str()),
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
pub struct M5VisualInteractionGovernanceReview {
    /// Motion clarifies origin, continuity, and completion.
    pub motion_clarifies_origin_continuity_completion: bool,
    /// Motion never delays input on a protected path.
    pub motion_never_delays_protected_input: bool,
    /// Reduced-motion, power-saver, and thermal clamps are respected.
    pub reduced_motion_power_saver_thermal_clamps_respected: bool,
    /// Scrims preserve orientation and contrast.
    pub scrims_preserve_orientation_and_contrast: bool,
    /// Layers follow one shared z-order model.
    pub layers_follow_single_z_order_model: bool,
    /// Portals attach to their owning surface.
    pub portals_attach_to_owning_surface: bool,
    /// Extension / embedded overlays cannot bypass the shared z-order.
    pub extension_overlays_cannot_bypass_shared_z_order: bool,
    /// Icons stay semantic and labeled.
    pub icons_stay_semantic_and_labeled: bool,
    /// Uncommon or destructive icons carry a text label.
    pub uncommon_or_destructive_icons_are_labeled: bool,
    /// Illustrations remain secondary to content.
    pub illustrations_remain_secondary: bool,
    /// Illustrations never impersonate trust or severity.
    pub illustrations_never_impersonate_trust_or_severity: bool,
    /// Motion tokens bind to the appearance-session / design-system tokens.
    pub motion_tokens_bind_to_appearance_session: bool,
    /// Every family keeps the same truth across every deployment line.
    pub every_family_declares_deployment_lines: bool,
    /// Every family declares a non-visual accessibility route.
    pub every_family_declares_accessibility_route: bool,
    /// Support / export reads a single canonical visual-interaction source.
    pub support_export_reads_single_visual_interaction_source: bool,
    /// Later M5 rows cannot invent parallel motion / layer / icon vocabulary.
    pub later_rows_cannot_invent_parallel_motion_layer_icon_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualInteractionConsumerProjection {
    /// Shell and dialog consume the shared motion and layer grammar.
    pub shell_and_dialog_consume_shared_motion_and_layer_grammar: bool,
    /// Onboarding and notification consume the shared icon and illustration language.
    pub onboarding_and_notification_consume_shared_icon_and_illustration_language: bool,
    /// Embedded surfaces consume the shared z-order and portal model.
    pub embedded_surfaces_consume_shared_z_order_and_portal_model: bool,
    /// Motion / layer / icon consumers read a single token source.
    pub motion_layer_icon_consumers_read_single_token_source: bool,
    /// The appearance session binds to the shared motion tokens.
    pub appearance_session_binds_to_shared_motion_tokens: bool,
    /// Support / export reads a single canonical visual-interaction source.
    pub support_export_reads_single_visual_interaction_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualInteractionProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the family.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the visual-interaction lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5VisualInteractionReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting visual-interaction audit for the lane.
    pub interaction_audit_ref: String,
    /// True when support/export parity is required for every family.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every family.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5MotionLayerIconographyMatrixPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5MotionLayerIconographyMatrixPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Interaction rows.
    pub interaction_rows: Vec<M5VisualInteractionRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5VisualInteractionVocabularySet,
    /// Governance-review block.
    pub governance_review: M5VisualInteractionGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5VisualInteractionConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5VisualInteractionProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5VisualInteractionReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe frozen M5 motion / layer / iconography matrix packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5MotionLayerIconographyMatrixPacket {
    /// Record kind; must equal [`M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Interaction rows.
    pub interaction_rows: Vec<M5VisualInteractionRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5VisualInteractionVocabularySet,
    /// Governance-review block.
    pub governance_review: M5VisualInteractionGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5VisualInteractionConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5VisualInteractionProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5VisualInteractionReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5MotionLayerIconographyMatrixPacket {
    /// Builds an M5 motion / layer / iconography matrix packet from stable-lane input.
    pub fn new(input: M5MotionLayerIconographyMatrixPacketInput) -> Self {
        Self {
            record_kind: M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_RECORD_KIND.to_owned(),
            schema_version: M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            interaction_rows: input.interaction_rows,
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

    /// Validates the M5 motion / layer / iconography matrix invariants.
    pub fn validate(&self) -> Vec<M5MotionLayerIconographyMatrixViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_RECORD_KIND {
            violations.push(M5MotionLayerIconographyMatrixViolation::WrongRecordKind);
        }
        if self.schema_version != M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_VERSION {
            violations.push(M5MotionLayerIconographyMatrixViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5MotionLayerIconographyMatrixViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_interaction_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("m5 motion / layer / iconography matrix serializes"),
        ) {
            violations.push(M5MotionLayerIconographyMatrixViolation::RawMaterialInExport);
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
            .expect("m5 motion / layer / iconography matrix packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per governed family.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "interaction_family,qualification,owner,canonical_schema,surface_families,deployment_lines,required_labels,consumer_surfaces,downgrade_triggers\n",
        );
        for row in &self.interaction_rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.interaction_family.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.interaction_family.canonical_domain_schema_ref(),
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
        let stable_families = self
            .interaction_rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str(
            "# M5 Motion-Token, Reduced-Motion, Opacity / Scrim, Layer-Order, Portal-Ownership, Iconography, and Illustration-Boundary Visual-Interaction Matrix\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Interaction families: {} ({} stable)\n",
            self.interaction_rows.len(),
            stable_families
        ));
        out.push_str(&format!(
            "- Interaction roles: {}\n",
            self.vocabulary_set.semantic_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Motion roles: {}\n",
            self.vocabulary_set.motion_roles.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Interaction families\n\n");
        for row in &self.interaction_rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.interaction_family.as_str(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!(
                "  - Canonical schema: `{}`\n",
                row.interaction_family.canonical_domain_schema_ref()
            ));
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

/// Errors emitted when reading the checked-in M5 motion / layer / iconography matrix export.
#[derive(Debug)]
pub enum M5MotionLayerIconographyMatrixArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5MotionLayerIconographyMatrixViolation>),
}

impl fmt::Display for M5MotionLayerIconographyMatrixArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 motion / layer / iconography matrix export parse failed: {error}"
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
                    "m5 motion / layer / iconography matrix export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5MotionLayerIconographyMatrixArtifactError {}

/// Validation failures emitted by [`M5MotionLayerIconographyMatrixPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5MotionLayerIconographyMatrixViolation {
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
    /// A required governed interaction family is missing from the matrix.
    RequiredFamilyMissing,
    /// An interaction row is incomplete.
    InteractionRowIncomplete,
    /// An interaction row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// An interaction row does not point at its own canonical domain schema.
    DomainSchemaRefMissing,
    /// A family declares no interaction roles.
    SemanticRoleMissing,
    /// The motion-token family declares no motion roles.
    MotionRoleMissing,
    /// The reduced-motion family declares no reduced-motion roles.
    ReducedMotionRoleMissing,
    /// The opacity-scrim family declares no opacity / scrim roles.
    OpacityScrimRoleMissing,
    /// The layer-order family declares no layer-order roles.
    LayerOrderRoleMissing,
    /// The portal-ownership family declares no portal-ownership roles.
    PortalOwnershipRoleMissing,
    /// The iconography family declares no iconography roles.
    IconographyRoleMissing,
    /// The illustration family declares no illustration roles.
    IllustrationRoleMissing,
    /// A family declares no degraded reasons.
    DegradedReasonMissing,
    /// A family declares no surface families.
    SurfaceFamilyMissing,
    /// A family declares no deployment lines.
    DeploymentLineMissing,
    /// A family declares no accessibility routes.
    AccessibilityRouteMissing,
    /// A family declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A family declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A family claiming Stable is missing required proof packet refs.
    StableFamilyMissingProof,
    /// A family violates a hard invariant (motion delaying protected input, a scrim erasing orientation
    /// or contrast, an overlay bypassing the shared z-order, an unlabeled uncommon / destructive icon, or
    /// an illustration impersonating operational / security truth).
    InteractionInvariantViolated,
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

impl M5MotionLayerIconographyMatrixViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredFamilyMissing => "required_family_missing",
            Self::InteractionRowIncomplete => "interaction_row_incomplete",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::DomainSchemaRefMissing => "domain_schema_ref_missing",
            Self::SemanticRoleMissing => "semantic_role_missing",
            Self::MotionRoleMissing => "motion_role_missing",
            Self::ReducedMotionRoleMissing => "reduced_motion_role_missing",
            Self::OpacityScrimRoleMissing => "opacity_scrim_role_missing",
            Self::LayerOrderRoleMissing => "layer_order_role_missing",
            Self::PortalOwnershipRoleMissing => "portal_ownership_role_missing",
            Self::IconographyRoleMissing => "iconography_role_missing",
            Self::IllustrationRoleMissing => "illustration_role_missing",
            Self::DegradedReasonMissing => "degraded_reason_missing",
            Self::SurfaceFamilyMissing => "surface_family_missing",
            Self::DeploymentLineMissing => "deployment_line_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StableFamilyMissingProof => "stable_family_missing_proof",
            Self::InteractionInvariantViolated => "interaction_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 motion / layer / iconography matrix export.
pub fn current_stable_m5_motion_layer_iconography_matrix_export(
) -> Result<M5MotionLayerIconographyMatrixPacket, M5MotionLayerIconographyMatrixArtifactError> {
    let packet: M5MotionLayerIconographyMatrixPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-motion-layer-iconography-proof/support_export.json"
    )))
    .map_err(M5MotionLayerIconographyMatrixArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5MotionLayerIconographyMatrixArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5MotionLayerIconographyMatrixPacket,
    violations: &mut Vec<M5MotionLayerIconographyMatrixViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF,
        M5_MOTION_AND_REDUCED_MOTION_SCHEMA_REF,
        M5_OPACITY_SCRIM_SCHEMA_REF,
        M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF,
        M5_ICONOGRAPHY_AND_ILLUSTRATION_SCHEMA_REF,
        M5_DESIGN_SYSTEM_FOUNDATIONS_SCHEMA_REF,
        M5_DESIGN_SYSTEM_FOUNDATION_PACKAGE_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5MotionLayerIconographyMatrixViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5MotionLayerIconographyMatrixPacket,
    violations: &mut Vec<M5MotionLayerIconographyMatrixViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5MotionLayerIconographyMatrixViolation::VocabularySetDrift);
    }
}

fn validate_interaction_rows(
    packet: &M5MotionLayerIconographyMatrixPacket,
    violations: &mut Vec<M5MotionLayerIconographyMatrixViolation>,
) {
    let present: BTreeSet<M5VisualInteractionFamily> = packet
        .interaction_rows
        .iter()
        .map(|row| row.interaction_family)
        .collect();
    for required in M5VisualInteractionFamily::ALL {
        if !present.contains(&required) {
            violations.push(M5MotionLayerIconographyMatrixViolation::RequiredFamilyMissing);
            return;
        }
    }

    for row in &packet.interaction_rows {
        let family = row.interaction_family;
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5MotionLayerIconographyMatrixViolation::InteractionRowIncomplete);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5MotionLayerIconographyMatrixViolation::MandatoryLabelMissing);
        }
        if !row
            .source_contract_refs
            .iter()
            .any(|r| r == family.canonical_domain_schema_ref())
        {
            violations.push(M5MotionLayerIconographyMatrixViolation::DomainSchemaRefMissing);
        }
        if row.semantic_roles.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::SemanticRoleMissing);
        }
        if family.declares_motion_roles() && row.motion_roles.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::MotionRoleMissing);
        }
        if family.declares_reduced_motion_roles() && row.reduced_motion_roles.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::ReducedMotionRoleMissing);
        }
        if family.declares_opacity_scrim_roles() && row.opacity_scrim_roles.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::OpacityScrimRoleMissing);
        }
        if family.declares_layer_order_roles() && row.layer_order_roles.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::LayerOrderRoleMissing);
        }
        if family.declares_portal_ownership_roles() && row.portal_ownership_roles.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::PortalOwnershipRoleMissing);
        }
        if family.declares_iconography_roles() && row.iconography_roles.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::IconographyRoleMissing);
        }
        if family.declares_illustration_roles() && row.illustration_roles.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::IllustrationRoleMissing);
        }
        if row.degraded_reasons.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::DegradedReasonMissing);
        }
        if row.surface_families.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::SurfaceFamilyMissing);
        }
        if row.deployment_lines.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::DeploymentLineMissing);
        }
        if row.accessibility_routes.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::DowngradeTriggersMissing);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5MotionLayerIconographyMatrixViolation::StableFamilyMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5MotionLayerIconographyMatrixViolation::InteractionInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5MotionLayerIconographyMatrixPacket,
    violations: &mut Vec<M5MotionLayerIconographyMatrixViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.motion_clarifies_origin_continuity_completion,
        review.motion_never_delays_protected_input,
        review.reduced_motion_power_saver_thermal_clamps_respected,
        review.scrims_preserve_orientation_and_contrast,
        review.layers_follow_single_z_order_model,
        review.portals_attach_to_owning_surface,
        review.extension_overlays_cannot_bypass_shared_z_order,
        review.icons_stay_semantic_and_labeled,
        review.uncommon_or_destructive_icons_are_labeled,
        review.illustrations_remain_secondary,
        review.illustrations_never_impersonate_trust_or_severity,
        review.motion_tokens_bind_to_appearance_session,
        review.every_family_declares_deployment_lines,
        review.every_family_declares_accessibility_route,
        review.support_export_reads_single_visual_interaction_source,
        review.later_rows_cannot_invent_parallel_motion_layer_icon_vocabulary,
    ] {
        if !ok {
            violations.push(M5MotionLayerIconographyMatrixViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5MotionLayerIconographyMatrixPacket,
    violations: &mut Vec<M5MotionLayerIconographyMatrixViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_and_dialog_consume_shared_motion_and_layer_grammar,
        projection.onboarding_and_notification_consume_shared_icon_and_illustration_language,
        projection.embedded_surfaces_consume_shared_z_order_and_portal_model,
        projection.motion_layer_icon_consumers_read_single_token_source,
        projection.appearance_session_binds_to_shared_motion_tokens,
        projection.support_export_reads_single_visual_interaction_source,
    ] {
        if !ok {
            violations.push(M5MotionLayerIconographyMatrixViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5MotionLayerIconographyMatrixPacket,
    violations: &mut Vec<M5MotionLayerIconographyMatrixViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5MotionLayerIconographyMatrixViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5MotionLayerIconographyMatrixPacket,
    violations: &mut Vec<M5MotionLayerIconographyMatrixViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.interaction_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5MotionLayerIconographyMatrixViolation::ReleasePostureIncomplete);
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

/// Heuristic that rejects obviously forbidden raw material in export-safe JSON. The controlled
/// vocabulary deliberately uses motion / scrim / layer / icon / illustration words; what is rejected is a
/// raw secret *value* shape — a pasted passphrase, a bearer token, a raw endpoint URL, or a PEM key block.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
