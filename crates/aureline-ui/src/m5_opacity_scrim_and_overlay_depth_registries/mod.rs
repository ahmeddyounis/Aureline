//! Implemented M5 opacity / scrim and overlay-depth (layer-order) registries.
//!
//! The frozen [motion / layer / iconography matrix][matrix] names Aureline's seven visual-interaction
//! families and locks their controlled vocabulary. This module is the overlay-depth implement lane over
//! that matrix: it turns the two families that carry the *overlay depth* grammar — the **opacity / scrim**
//! primitives (lightweight versus blocking overlays that must keep the workspace orientable and legible)
//! and the **layer-order** z-order tiers (one shared blocking-versus-nonblocking depth model no private
//! overlay may bypass) — into registry resolvers that produce export-safe, honest projections, so a user can
//! trust that a scrim never turns the workspace into an unreadable backdrop, that text stays legible beneath
//! every overlay, that blocking overlays always offer a dismiss affordance, that overlays and portals stack
//! under one shared z-order model, and that scrims narrow honestly (drop blur, simplify) under
//! reduced-motion, power-saver, and thermal runtime pressure without hiding why behavior changed.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Define opacity / scrim primitives for lightweight versus blocking overlays, with text-contrast and
//!   background-orientation rules.** [`resolve_scrim_entry`] refuses to read as a clean, orientation-safe
//!   scrim entry unless it names a canonical token, a classified [overlay depth class][M5OverlayDepthClass],
//!   an opacity / scrim role, and a contrast treatment, covers all three runtime clamps, preserves workspace
//!   orientation, preserves text contrast, and traces to a canonical token rather than an inlined raw
//!   opacity value; otherwise it degrades.
//! * **Bind overlay depth to runtime state so reduced-motion, power-saver, and thermal pressure narrow
//!   behavior honestly.** Every scrim entry names an [`M5ScrimContrastTreatment`] and covers the three
//!   runtime clamps, and degrades to [`M5ScrimEntryDegradeReason::OrientationErasedByScrim`],
//!   [`M5ScrimEntryDegradeReason::TextContrastLost`], or
//!   [`M5ScrimEntryDegradeReason::ClampCoverageIncomplete`] when a scrim would otherwise erase orientation,
//!   drop text contrast, or leave a runtime clamp uncovered. [`resolve_overlay_depth_entry`] does the same
//!   for a layer-order tier and refuses to let a private overlay bypass the shared z-order model or drop out
//!   of the single shared stacking model.
//! * **Wire first shell, dialog, panel, embedded, and notification consumers plus fixtures that catch
//!   context-legibility and depth-truth regressions.** Each registry row carries the render [surface
//!   context][M5OverlaySurfaceContext] so a scrim or overlay-depth regression degrades honestly, and the
//!   acceptance-criteria gate proves the first claimed overlays carry correct blocking-versus-nonblocking
//!   depth truth before release evidence turns green.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5VisualInteractionRole`] role
//! vocabulary, the [`M5OpacityScrimRole`] scrim-role vocabulary, and the [`M5LayerOrderRole`] layer-order
//! vocabulary — so shell, dialog, panel, embedded, notification, and support surfaces can never fork their
//! own scrim or layering meaning. Raw secret values and private endpoints stay outside the export boundary.
//!
//! [matrix]: crate::m5_motion_layer_iconography_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_opacity_scrim_overlay_depth_registries,
    seeded_m5_opacity_scrim_overlay_depth_registries_onboarding_ui_preview_narrowed,
    seeded_m5_opacity_scrim_overlay_depth_registries_shell_ui_beta_narrowed,
    M5_OVERLAY_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_motion_layer_iconography_matrix::{
    M5LayerOrderRole, M5OpacityScrimRole, M5VisualInteractionAccessibilityRoute,
    M5VisualInteractionConsumerSurface, M5VisualInteractionDeploymentLine,
    M5VisualInteractionDowngradeTrigger, M5VisualInteractionFamily,
    M5VisualInteractionQualificationClass, M5VisualInteractionRequiredLabel,
    M5VisualInteractionRole, M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF,
    M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF, M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
    M5_OPACITY_SCRIM_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5OverlayRegistriesPacket`].
pub const M5_OVERLAY_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_opacity_scrim_and_overlay_depth_registries";

/// Schema version for M5 opacity / scrim and overlay-depth registry records.
pub const M5_OVERLAY_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_OVERLAY_REGISTRIES_SCHEMA_REF: &str =
    "schemas/design-system/m5-opacity-scrim-and-overlay-depth-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_OVERLAY_REGISTRIES_DOC_REF: &str =
    "docs/design-system/m5_opacity_scrim_and_overlay_depth_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_OVERLAY_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-opacity-scrim-and-overlay-depth-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_OVERLAY_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-opacity-scrim-and-overlay-depth-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_OVERLAY_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-opacity-scrim-and-overlay-depth-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_OVERLAY_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-opacity-scrim-and-overlay-depth-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5OverlayRegistriesConsumerSurface = M5VisualInteractionConsumerSurface;

/// One of the three runtime clamps every scrim / overlay-depth entry must cover so its behavior is explicit
/// under reduced-motion, power-saver, and thermal pressure. Minted by this lane because the frozen matrix
/// names the reduced-motion *rule* but not the concrete runtime-clamp set an overlay entry must narrow for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverlayRuntimeClamp {
    /// The reduced-motion clamp.
    ReducedMotion,
    /// The power-saver clamp.
    PowerSaver,
    /// The thermal clamp.
    Thermal,
}

impl M5OverlayRuntimeClamp {
    /// Every runtime clamp, in declaration order. A clean entry must cover all three.
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

/// Controlled overlay depth class a scrim or overlay-depth entry maps, so a blocking overlay (modal dialog,
/// sheet, confirm scrim, wizard step, credential prompt) always offers a dismiss affordance and preserves
/// orientation while a lightweight overlay (tooltip, popover, toast, hover preview, drawer, panel, menu, HUD)
/// stays non-blocking, and every overlay carries an honest blocking-versus-nonblocking depth truth. Minted by
/// this lane because the frozen matrix carries the high-level interaction roles but not the finer depth
/// classes the overlay acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverlayDepthClass {
    /// A blocking modal dialog (blocking: must dismiss and preserve orientation).
    BlockingModalDialog,
    /// A blocking sheet (blocking: must dismiss and preserve orientation).
    BlockingSheet,
    /// A blocking confirm scrim (blocking: must dismiss and preserve orientation).
    BlockingConfirmScrim,
    /// A blocking wizard step (blocking: must dismiss and preserve orientation).
    BlockingWizardStep,
    /// A blocking credential prompt (blocking: must dismiss and preserve orientation).
    BlockingCredentialPrompt,
    /// A lightweight tooltip (non-blocking).
    LightweightTooltip,
    /// A lightweight popover (non-blocking).
    LightweightPopover,
    /// A transient toast (non-blocking).
    TransientToast,
    /// A hover preview (non-blocking).
    HoverPreview,
    /// An inline drawer (non-blocking).
    InlineDrawer,
    /// A side panel (non-blocking).
    SidePanel,
    /// A context menu (non-blocking).
    ContextMenu,
    /// A status HUD (non-blocking).
    StatusHud,
    /// The overlay depth class is unclassified, which is disallowed.
    DepthClassUnclassified,
}

impl M5OverlayDepthClass {
    /// Every overlay depth class, in declaration order.
    pub const ALL: [Self; 14] = [
        Self::BlockingModalDialog,
        Self::BlockingSheet,
        Self::BlockingConfirmScrim,
        Self::BlockingWizardStep,
        Self::BlockingCredentialPrompt,
        Self::LightweightTooltip,
        Self::LightweightPopover,
        Self::TransientToast,
        Self::HoverPreview,
        Self::InlineDrawer,
        Self::SidePanel,
        Self::ContextMenu,
        Self::StatusHud,
        Self::DepthClassUnclassified,
    ];

    /// The blocking depth classes the acceptance criteria require to offer a dismiss affordance and preserve
    /// orientation.
    pub const BLOCKING_CLASSES: [Self; 5] = [
        Self::BlockingModalDialog,
        Self::BlockingSheet,
        Self::BlockingConfirmScrim,
        Self::BlockingWizardStep,
        Self::BlockingCredentialPrompt,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockingModalDialog => "blocking_modal_dialog",
            Self::BlockingSheet => "blocking_sheet",
            Self::BlockingConfirmScrim => "blocking_confirm_scrim",
            Self::BlockingWizardStep => "blocking_wizard_step",
            Self::BlockingCredentialPrompt => "blocking_credential_prompt",
            Self::LightweightTooltip => "lightweight_tooltip",
            Self::LightweightPopover => "lightweight_popover",
            Self::TransientToast => "transient_toast",
            Self::HoverPreview => "hover_preview",
            Self::InlineDrawer => "inline_drawer",
            Self::SidePanel => "side_panel",
            Self::ContextMenu => "context_menu",
            Self::StatusHud => "status_hud",
            Self::DepthClassUnclassified => "depth_class_unclassified",
        }
    }

    /// Whether the depth class is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::DepthClassUnclassified)
    }

    /// Whether this is a blocking depth class that must offer a dismiss affordance and preserve orientation.
    pub const fn is_blocking(self) -> bool {
        matches!(
            self,
            Self::BlockingModalDialog
                | Self::BlockingSheet
                | Self::BlockingConfirmScrim
                | Self::BlockingWizardStep
                | Self::BlockingCredentialPrompt
        )
    }
}

/// Controlled contrast treatment a scrim entry pairs with its opacity so text stays legible and the
/// workspace stays orientable beneath the overlay: a dim backdrop with readable text, a blur with a contrast
/// floor, a solid panel behind text, a high-contrast border, or a screen-reader context announcement. Minted
/// by this lane, tracking the text-contrast / orientation rule the scrim acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScrimContrastTreatment {
    /// A dim backdrop that keeps text readable carries the contrast.
    DimBackdropReadableText,
    /// A blur with a contrast floor carries the contrast.
    BlurWithContrastFloor,
    /// A solid panel behind text carries the contrast.
    SolidPanelBehindText,
    /// A high-contrast border carries the contrast.
    HighContrastBorder,
    /// A screen-reader context announcement carries the orientation.
    ScreenReaderContext,
    /// No contrast treatment is paired with the scrim, which is disallowed.
    NoneDisallowed,
}

impl M5ScrimContrastTreatment {
    /// Every contrast treatment, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DimBackdropReadableText,
        Self::BlurWithContrastFloor,
        Self::SolidPanelBehindText,
        Self::HighContrastBorder,
        Self::ScreenReaderContext,
        Self::NoneDisallowed,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DimBackdropReadableText => "dim_backdrop_readable_text",
            Self::BlurWithContrastFloor => "blur_with_contrast_floor",
            Self::SolidPanelBehindText => "solid_panel_behind_text",
            Self::HighContrastBorder => "high_contrast_border",
            Self::ScreenReaderContext => "screen_reader_context",
            Self::NoneDisallowed => "none_disallowed",
        }
    }

    /// Whether a contrast treatment is present (never the disallowed none sentinel).
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::NoneDisallowed)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a scrim or overlay's
/// depth truth stays stable whether it appears in the shell, a dialog, a panel, an embedded surface, or a
/// notification. Minted by this lane, tracking the first-consumer surfaces the implementation requirement
/// names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverlaySurfaceContext {
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

impl M5OverlaySurfaceContext {
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

/// One mandatory rendered part a scrim or overlay-depth entry must be able to show, so no depth, clamp, or
/// token fact is left implicit behind an opacity value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverlayRegistryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical token reference the entry points at.
    TokenReference,
    /// The overlay depth class the entry maps (both entries).
    DepthClass,
    /// The clamp coverage (reduced-motion / power-saver / thermal).
    ClampCoverage,
    /// The contrast treatment paired with the opacity (scrim entry).
    ContrastTreatment,
    /// The opacity / scrim role named by the entry (scrim entry).
    ScrimRole,
    /// The layer-order role named by the entry (overlay-depth entry).
    LayerOrderRole,
    /// The render / surface context (both entries).
    SurfaceContext,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the token (both entries).
    PlainLanguageMeaning,
}

impl M5OverlayRegistryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::TokenReference,
        Self::DepthClass,
        Self::ClampCoverage,
        Self::ContrastTreatment,
        Self::ScrimRole,
        Self::LayerOrderRole,
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
            Self::DepthClass => "depth_class",
            Self::ClampCoverage => "clamp_coverage",
            Self::ContrastTreatment => "contrast_treatment",
            Self::ScrimRole => "scrim_role",
            Self::LayerOrderRole => "layer_order_role",
            Self::SurfaceContext => "surface_context",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect depth,
/// clamp coverage, or a degraded scrim / overlay-depth token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverlayRegistryNextAction {
    /// Expand the overlay's plain-language meaning.
    ExpandOverlayMeaning,
    /// Inspect the overlay depth class the entry maps.
    InspectDepthClass,
    /// Complete the reduced-motion / power-saver / thermal clamp coverage.
    CompleteClampCoverage,
    /// Trace the entry back to its canonical token.
    TraceCanonicalToken,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5OverlayRegistryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandOverlayMeaning,
        Self::InspectDepthClass,
        Self::CompleteClampCoverage,
        Self::TraceCanonicalToken,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandOverlayMeaning => "expand_overlay_meaning",
            Self::InspectDepthClass => "inspect_depth_class",
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
pub enum M5OverlayRegistryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The interaction families covered.
    InteractionFamilies,
    /// The overlay depth classes carried.
    DepthClasses,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// The clamp profiles covered.
    ClampProfiles,
    /// The contrast treatments paired.
    ContrastTreatments,
    /// The render / surface context.
    SurfaceContext,
    /// The layer-order roles named.
    LayerOrderRoles,
    /// The accountable owner role.
    OwnerRole,
}

impl M5OverlayRegistryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::InteractionFamilies,
        Self::DepthClasses,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ClampProfiles,
        Self::ContrastTreatments,
        Self::SurfaceContext,
        Self::LayerOrderRoles,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::InteractionFamilies,
        Self::DepthClasses,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::InteractionFamilies => "interaction_families",
            Self::DepthClasses => "depth_classes",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ClampProfiles => "clamp_profiles",
            Self::ContrastTreatments => "contrast_treatments",
            Self::SurfaceContext => "surface_context",
            Self::LayerOrderRoles => "layer_order_roles",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a scrim entry degraded below a clean, orientation-safe state. The degrade-first ladder returns one
/// of these instead of ever letting an orientation-erasing, contrast-losing, raw-opacity, or clamp-incomplete
/// entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ScrimEntryDegradeReason {
    /// The canonical token name is unstated; a user cannot trace what the scrim means.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The overlay depth class is unclassified (not in the preserved taxonomy).
    DepthClassUnclassified,
    /// The scrim erases workspace orientation rather than keeping it orientable.
    OrientationErasedByScrim,
    /// No contrast treatment is paired with the scrim.
    ContrastCueMissing,
    /// A raw opacity value is inlined instead of tracing to a canonical token.
    RawOpacityValueInlined,
    /// The reduced-motion / power-saver / thermal clamp coverage is incomplete.
    ClampCoverageIncomplete,
    /// The scrim drops text contrast so text beneath the overlay is no longer legible.
    TextContrastLost,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5ScrimEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::DepthClassUnclassified,
        Self::OrientationErasedByScrim,
        Self::ContrastCueMissing,
        Self::RawOpacityValueInlined,
        Self::ClampCoverageIncomplete,
        Self::TextContrastLost,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::DepthClassUnclassified => "depth_class_unclassified",
            Self::OrientationErasedByScrim => "orientation_erased_by_scrim",
            Self::ContrastCueMissing => "contrast_cue_missing",
            Self::RawOpacityValueInlined => "raw_opacity_value_inlined",
            Self::ClampCoverageIncomplete => "clamp_coverage_incomplete",
            Self::TextContrastLost => "text_contrast_lost",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5OverlayRegistryNextAction {
        match self {
            Self::TokenNameUnstated | Self::RawOpacityValueInlined => {
                M5OverlayRegistryNextAction::TraceCanonicalToken
            }
            Self::DepthClassUnclassified | Self::TextContrastLost => {
                M5OverlayRegistryNextAction::InspectDepthClass
            }
            Self::OrientationErasedByScrim | Self::ContrastCueMissing => {
                M5OverlayRegistryNextAction::ExpandOverlayMeaning
            }
            Self::ClampCoverageIncomplete => M5OverlayRegistryNextAction::CompleteClampCoverage,
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5OverlayRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualInteractionDowngradeTrigger {
        match self {
            Self::OrientationErasedByScrim | Self::ContrastCueMissing | Self::TextContrastLost => {
                M5VisualInteractionDowngradeTrigger::ScrimErasedOrientationOrContrast
            }
            Self::ClampCoverageIncomplete => {
                M5VisualInteractionDowngradeTrigger::MotionMeaningLostUnderReducedMotion
            }
            Self::TokenNameUnstated | Self::RawOpacityValueInlined => {
                M5VisualInteractionDowngradeTrigger::TokenReferenceUnstated
            }
            Self::DepthClassUnclassified => M5VisualInteractionDowngradeTrigger::LayerTierUnstated,
            Self::SurfaceContextUnresolved => {
                M5VisualInteractionDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualInteractionDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason an overlay-depth entry degraded below a clean, shared-z-order-safe state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5OverlayDepthEntryDegradeReason {
    /// The canonical token name is unstated.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The overlay depth class is unclassified (not in the preserved taxonomy).
    DepthClassUnclassified,
    /// A private layer bypasses the shared z-order model, or no canonical token is named.
    PrivateLayerBypassWithoutSharedModel,
    /// The reduced-motion / power-saver / thermal clamp coverage is incomplete.
    ClampCoverageIncomplete,
    /// The overlay does not stack under the single shared z-order model.
    NotStackedUnderSharedModel,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5OverlayDepthEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::DepthClassUnclassified,
        Self::PrivateLayerBypassWithoutSharedModel,
        Self::ClampCoverageIncomplete,
        Self::NotStackedUnderSharedModel,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::DepthClassUnclassified => "depth_class_unclassified",
            Self::PrivateLayerBypassWithoutSharedModel => {
                "private_layer_bypass_without_shared_model"
            }
            Self::ClampCoverageIncomplete => "clamp_coverage_incomplete",
            Self::NotStackedUnderSharedModel => "not_stacked_under_shared_model",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5OverlayRegistryNextAction {
        match self {
            Self::TokenNameUnstated | Self::PrivateLayerBypassWithoutSharedModel => {
                M5OverlayRegistryNextAction::TraceCanonicalToken
            }
            Self::DepthClassUnclassified | Self::NotStackedUnderSharedModel => {
                M5OverlayRegistryNextAction::InspectDepthClass
            }
            Self::ClampCoverageIncomplete => M5OverlayRegistryNextAction::CompleteClampCoverage,
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5OverlayRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualInteractionDowngradeTrigger {
        match self {
            Self::PrivateLayerBypassWithoutSharedModel | Self::NotStackedUnderSharedModel => {
                M5VisualInteractionDowngradeTrigger::OverlayBypassedSharedZOrder
            }
            Self::ClampCoverageIncomplete => {
                M5VisualInteractionDowngradeTrigger::MotionMeaningLostUnderReducedMotion
            }
            Self::TokenNameUnstated => M5VisualInteractionDowngradeTrigger::TokenReferenceUnstated,
            Self::DepthClassUnclassified => M5VisualInteractionDowngradeTrigger::LayerTierUnstated,
            Self::SurfaceContextUnresolved => {
                M5VisualInteractionDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualInteractionDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_scrim_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ScrimEntryResolutionInput {
    /// Stable identity of the scrim-registry entry.
    pub entry_id: String,
    /// The canonical token name (e.g. `scrim.blocking.modal`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualInteractionRole,
    /// The opacity / scrim role (from the frozen matrix vocabulary).
    pub scrim_role: M5OpacityScrimRole,
    /// The overlay depth class this entry maps.
    pub depth_class: M5OverlayDepthClass,
    /// The contrast treatment paired with the opacity.
    pub contrast_treatment: M5ScrimContrastTreatment,
    /// The render / surface context.
    pub surface_context: M5OverlaySurfaceContext,
    /// The runtime clamps this entry covers (must cover reduced-motion / power-saver / thermal).
    pub clamp_coverage: Vec<M5OverlayRuntimeClamp>,
    /// True when the scrim keeps the workspace orientable and never erases orientation.
    pub preserves_orientation: bool,
    /// True when the scrim keeps text beneath the overlay legible (contrast preserved).
    pub preserves_text_contrast: bool,
    /// True when the entry traces to a canonical token (never an inlined raw opacity value).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe scrim-registry projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedScrimEntry {
    /// Stable identity of the scrim-registry entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role demands an accessible fallback (motion / overlay / icon / illustration /
    /// attention).
    pub semantic_role_demands_accessible_fallback: bool,
    /// The scrim-role token named by the entry.
    pub scrim_role: String,
    /// Whether the scrim role names the disallowed orientation-or-contrast-erasing token.
    pub scrim_role_erases_orientation_or_contrast: bool,
    /// The overlay-depth-class token named by the entry.
    pub depth_class: String,
    /// Whether the overlay depth class is classified into the preserved taxonomy.
    pub depth_class_is_classified: bool,
    /// Whether this is a blocking depth class that must offer a dismiss affordance and preserve orientation.
    pub depth_class_is_blocking: bool,
    /// The contrast-treatment token named by the entry.
    pub contrast_treatment: String,
    /// Whether a contrast treatment is present.
    pub contrast_treatment_present: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The clamp tokens covered by the entry.
    pub clamp_coverage: Vec<String>,
    /// Whether the entry covers all three runtime clamps.
    pub covers_all_clamps: bool,
    /// Whether the scrim keeps the workspace orientable.
    pub preserves_orientation: bool,
    /// Whether the scrim keeps text beneath the overlay legible.
    pub preserves_text_contrast: bool,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean, orientation-safe state.
    pub degrade_reason: Option<M5ScrimEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5OverlayRegistryNextAction,
    /// Whether the scrim keeps the workspace orientable and legible (clean entry naming every fact).
    pub overlay_orientation_preserved: bool,
}

impl M5ResolvedScrimEntry {
    /// Whether this scrim entry reads as a clean, orientation-safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_overlay_depth_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5OverlayDepthEntryResolutionInput {
    /// Stable identity of the overlay-depth entry.
    pub entry_id: String,
    /// The canonical token name; empty means unstated.
    pub token_name: String,
    /// The layer-order role (from the frozen matrix vocabulary).
    pub layer_order_role: M5LayerOrderRole,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualInteractionRole,
    /// The overlay depth class this entry maps.
    pub depth_class: M5OverlayDepthClass,
    /// The render / surface context.
    pub surface_context: M5OverlaySurfaceContext,
    /// The runtime clamps this entry covers (must cover reduced-motion / power-saver / thermal).
    pub clamp_coverage: Vec<M5OverlayRuntimeClamp>,
    /// True when the entry traces to a canonical token (never a private layer bypassing the shared model).
    pub references_canonical_token: bool,
    /// True when the overlay stacks under the single shared z-order model.
    pub stacks_under_shared_model: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe overlay-depth projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedOverlayDepthEntry {
    /// Stable identity of the overlay-depth entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The layer-order-role token named by the entry.
    pub layer_order_role: String,
    /// Whether the layer-order role names the disallowed private-layer-bypass token.
    pub layer_order_role_is_private_bypass: bool,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// The overlay-depth-class token named by the entry.
    pub depth_class: String,
    /// Whether the overlay depth class is classified into the preserved taxonomy.
    pub depth_class_is_classified: bool,
    /// Whether this is a blocking depth class.
    pub depth_class_is_blocking: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// The clamp tokens covered by the entry.
    pub clamp_coverage: Vec<String>,
    /// Whether the entry covers all three runtime clamps.
    pub covers_all_clamps: bool,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Whether the overlay stacks under the single shared z-order model.
    pub stacks_under_shared_model: bool,
    /// Degrade reason, if the entry could not read as a clean, shared-z-order-safe state.
    pub degrade_reason: Option<M5OverlayDepthEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5OverlayRegistryNextAction,
    /// Whether the depth truth holds across every clamp (clean entry naming every fact).
    pub depth_truth_holds_across_clamps: bool,
}

impl M5ResolvedOverlayDepthEntry {
    /// Whether this overlay-depth entry reads as a clean, shared-z-order-safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5OverlayResolutionError {
    /// The scrim-entry id was empty.
    EmptyScrimEntryId,
    /// The overlay-depth-entry id was empty.
    EmptyOverlayDepthEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5OverlayResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyScrimEntryId => "empty_scrim_entry_id",
            Self::EmptyOverlayDepthEntryId => "empty_overlay_depth_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5OverlayResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 opacity / scrim and overlay-depth registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5OverlayResolutionError {}

fn clamp_tokens(clamps: &[M5OverlayRuntimeClamp]) -> Vec<String> {
    clamps.iter().map(|c| c.as_str().to_owned()).collect()
}

fn covers_all_clamps(clamps: &[M5OverlayRuntimeClamp]) -> bool {
    let present: BTreeSet<M5OverlayRuntimeClamp> = clamps.iter().copied().collect();
    M5OverlayRuntimeClamp::ALL
        .iter()
        .all(|clamp| present.contains(clamp))
}

/// Resolves a scrim-registry entry so it stays orientation-safe: the entry names its canonical token,
/// semantic role, scrim role, depth class, and a contrast treatment, covers all three runtime clamps,
/// preserves workspace orientation, preserves text contrast, and traces to a canonical token rather than an
/// inlined raw opacity value.
pub fn resolve_scrim_entry(
    input: M5ScrimEntryResolutionInput,
) -> Result<M5ResolvedScrimEntry, M5OverlayResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5OverlayResolutionError::EmptyScrimEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5OverlayResolutionError::ForbiddenMaterial);
    }

    let scrim_role_erases_orientation_or_contrast = matches!(
        input.scrim_role,
        M5OpacityScrimRole::ScrimErasesOrientationOrContrastDisallowed
    );
    let all_clamps = covers_all_clamps(&input.clamp_coverage);

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5ScrimEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5ScrimEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.depth_class.is_classified() {
        Some(M5ScrimEntryDegradeReason::DepthClassUnclassified)
    } else if scrim_role_erases_orientation_or_contrast || !input.preserves_orientation {
        Some(M5ScrimEntryDegradeReason::OrientationErasedByScrim)
    } else if !input.contrast_treatment.is_present() {
        Some(M5ScrimEntryDegradeReason::ContrastCueMissing)
    } else if !input.references_canonical_token {
        Some(M5ScrimEntryDegradeReason::RawOpacityValueInlined)
    } else if !all_clamps {
        Some(M5ScrimEntryDegradeReason::ClampCoverageIncomplete)
    } else if !input.preserves_text_contrast {
        Some(M5ScrimEntryDegradeReason::TextContrastLost)
    } else if !input.proof_fresh {
        Some(M5ScrimEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5OverlayRegistryNextAction::ExpandOverlayMeaning,
    };

    Ok(M5ResolvedScrimEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_demands_accessible_fallback: input
            .semantic_role
            .demands_accessible_fallback(),
        scrim_role: input.scrim_role.as_str().to_owned(),
        scrim_role_erases_orientation_or_contrast,
        depth_class: input.depth_class.as_str().to_owned(),
        depth_class_is_classified: input.depth_class.is_classified(),
        depth_class_is_blocking: input.depth_class.is_blocking(),
        contrast_treatment: input.contrast_treatment.as_str().to_owned(),
        contrast_treatment_present: input.contrast_treatment.is_present(),
        surface_context: input.surface_context.as_str().to_owned(),
        clamp_coverage: clamp_tokens(&input.clamp_coverage),
        covers_all_clamps: all_clamps,
        preserves_orientation: input.preserves_orientation,
        preserves_text_contrast: input.preserves_text_contrast,
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        overlay_orientation_preserved: degrade_reason.is_none(),
    })
}

/// Resolves an overlay-depth entry so it stays honest under the shared z-order model: the entry names its
/// canonical token, layer-order role, semantic role, depth class, and surface context, covers all three
/// runtime clamps, stacks under the single shared z-order model, and traces to a canonical token rather than
/// letting a private layer bypass the shared model.
pub fn resolve_overlay_depth_entry(
    input: M5OverlayDepthEntryResolutionInput,
) -> Result<M5ResolvedOverlayDepthEntry, M5OverlayResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5OverlayResolutionError::EmptyOverlayDepthEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5OverlayResolutionError::ForbiddenMaterial);
    }

    let layer_order_role_is_private_bypass = matches!(
        input.layer_order_role,
        M5LayerOrderRole::PrivateLayerBypassDisallowed
    );
    let all_clamps = covers_all_clamps(&input.clamp_coverage);

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5OverlayDepthEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5OverlayDepthEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.depth_class.is_classified() {
        Some(M5OverlayDepthEntryDegradeReason::DepthClassUnclassified)
    } else if layer_order_role_is_private_bypass || !input.references_canonical_token {
        Some(M5OverlayDepthEntryDegradeReason::PrivateLayerBypassWithoutSharedModel)
    } else if !all_clamps {
        Some(M5OverlayDepthEntryDegradeReason::ClampCoverageIncomplete)
    } else if !input.stacks_under_shared_model {
        Some(M5OverlayDepthEntryDegradeReason::NotStackedUnderSharedModel)
    } else if !input.proof_fresh {
        Some(M5OverlayDepthEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5OverlayRegistryNextAction::TraceCanonicalToken,
    };

    Ok(M5ResolvedOverlayDepthEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        layer_order_role: input.layer_order_role.as_str().to_owned(),
        layer_order_role_is_private_bypass,
        semantic_role: input.semantic_role.as_str().to_owned(),
        depth_class: input.depth_class.as_str().to_owned(),
        depth_class_is_classified: input.depth_class.is_classified(),
        depth_class_is_blocking: input.depth_class.is_blocking(),
        surface_context: input.surface_context.as_str().to_owned(),
        clamp_coverage: clamp_tokens(&input.clamp_coverage),
        covers_all_clamps: all_clamps,
        references_canonical_token: input.references_canonical_token,
        stacks_under_shared_model: input.stacks_under_shared_model,
        degrade_reason,
        next_action,
        depth_truth_holds_across_clamps: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved scrim and overlay-depth entries it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverlayRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5OverlayRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5OverlayRegistryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5OverlayRegistryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5VisualInteractionDowngradeTrigger>,
    /// Resolved scrim-registry examples.
    pub scrim_entries: Vec<M5ResolvedScrimEntry>,
    /// Resolved overlay-depth examples.
    pub overlay_depth_entries: Vec<M5ResolvedOverlayDepthEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical scrim and layer domain schemas).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: a scrim never erases workspace orientation or contrast. MUST be `false`.
    pub scrim_erases_orientation_or_contrast: bool,
    /// Hard invariant: a raw opacity value is never inlined instead of a canonical token. MUST be `false`.
    pub raw_opacity_value_inlined_instead_of_token: bool,
    /// Hard invariant: an overlay never bypasses the shared z-order model. MUST be `false`.
    pub overlay_bypasses_shared_z_order: bool,
    /// Hard invariant: the reduced-motion / power-saver / thermal clamp coverage is never incomplete. MUST be
    /// `false`.
    pub runtime_clamp_coverage_incomplete: bool,
}

impl M5OverlayRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5OverlayRegistryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5OverlayRegistryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5OverlayRegistryExportField> =
            self.export_fields.iter().copied().collect();
        M5OverlayRegistryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.scrim_erases_orientation_or_contrast
            && !self.raw_opacity_value_inlined_instead_of_token
            && !self.overlay_bypasses_shared_z_order
            && !self.runtime_clamp_coverage_incomplete
    }

    /// True when a clean scrim entry preserves orientation safety: it traces to a canonical token, preserves
    /// orientation, never names the disallowed orientation-erasing role, pairs a contrast treatment, keeps a
    /// classified depth class, covers all three clamps, and preserves text contrast.
    fn scrim_is_honest(ex: &M5ResolvedScrimEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && ex.preserves_orientation
                && !ex.scrim_role_erases_orientation_or_contrast
                && ex.contrast_treatment_present
                && ex.depth_class_is_classified
                && ex.covers_all_clamps
                && ex.preserves_text_contrast)
    }

    /// True when a clean overlay-depth entry preserves shared-z-order safety: it traces to a canonical token,
    /// never names the disallowed private-bypass role, keeps a classified depth class, covers all three
    /// clamps, and stacks under the shared model.
    fn depth_is_honest(ex: &M5ResolvedOverlayDepthEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && !ex.layer_order_role_is_private_bypass
                && ex.depth_class_is_classified
                && ex.covers_all_clamps
                && ex.stacks_under_shared_model)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.scrim_entries.iter().all(Self::scrim_is_honest)
            && self.overlay_depth_entries.iter().all(Self::depth_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverlayRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Opacity / scrim-role tokens (bound from the frozen matrix).
    pub scrim_roles: Vec<String>,
    /// Layer-order-role tokens (bound from the frozen matrix).
    pub layer_order_roles: Vec<String>,
    /// Runtime-clamp tokens (minted by this lane).
    pub runtime_clamps: Vec<String>,
    /// Overlay-depth-class tokens (minted by this lane).
    pub overlay_depth_classes: Vec<String>,
    /// Contrast-treatment tokens (minted by this lane).
    pub contrast_treatments: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Scrim-entry degrade-reason tokens.
    pub scrim_degrade_reasons: Vec<String>,
    /// Overlay-depth-entry degrade-reason tokens.
    pub overlay_depth_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5OverlayRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5VisualInteractionRole::ALL, |v| v.as_str()),
            scrim_roles: tokens(&M5OpacityScrimRole::ALL, |v| v.as_str()),
            layer_order_roles: tokens(&M5LayerOrderRole::ALL, |v| v.as_str()),
            runtime_clamps: tokens(&M5OverlayRuntimeClamp::ALL, |v| v.as_str()),
            overlay_depth_classes: tokens(&M5OverlayDepthClass::ALL, |v| v.as_str()),
            contrast_treatments: tokens(&M5ScrimContrastTreatment::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5OverlaySurfaceContext::ALL, |v| v.as_str()),
            scrim_degrade_reasons: tokens(&M5ScrimEntryDegradeReason::ALL, |v| v.as_str()),
            overlay_depth_degrade_reasons: tokens(&M5OverlayDepthEntryDegradeReason::ALL, |v| {
                v.as_str()
            }),
            anatomy_parts: tokens(&M5OverlayRegistryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5OverlayRegistryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5OverlayRegistryExportField::ALL, |v| v.as_str()),
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
pub struct M5OverlayRegistriesGovernanceReview {
    /// The scrim registry names a canonical token, scrim role, and depth class for every entry.
    pub scrim_registry_names_token_role_and_depth_class: bool,
    /// Opacity / scrim classes distinguish lightweight from blocking overlays.
    pub opacity_scrim_classes_distinguish_lightweight_from_blocking: bool,
    /// A scrim never erases workspace orientation.
    pub scrim_never_erases_orientation: bool,
    /// Every scrim and overlay entry covers the reduced-motion / power-saver / thermal clamps.
    pub every_entry_covers_all_runtime_clamps: bool,
    /// Reduced-motion, power-saver, and thermal clamps narrow overlay behavior honestly.
    pub runtime_clamps_narrow_overlay_behavior_honestly: bool,
    /// Scrims name a contrast treatment so text beneath the overlay stays legible.
    pub scrims_name_contrast_treatment_not_unreadable_backdrop: bool,
    /// Overlays and portals stack under one shared z-order model no private layer bypasses.
    pub overlays_stack_under_one_shared_z_order_model: bool,
    /// The first claimed overlays carry correct blocking-versus-nonblocking depth truth before release.
    pub blocking_versus_nonblocking_depth_truth_caught_before_release: bool,
    /// The first shell / dialog / panel / embedded consumers use the canonical overlay grammar.
    pub first_consumers_use_canonical_overlay_grammar: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverlayRegistriesConsumerProjection {
    /// The shell surface consumes the shared scrim / overlay-depth registries.
    pub shell_consumes_shared_registries: bool,
    /// The dialog surface consumes the shared scrim / overlay-depth registries.
    pub dialog_consumes_shared_registries: bool,
    /// The panel surface consumes the shared scrim / overlay-depth registries.
    pub panel_consumes_shared_registries: bool,
    /// The embedded and notification surfaces consume the shared scrim / overlay-depth registries.
    pub embedded_and_notification_consume_shared_registries: bool,
    /// Overlay depth behavior traces back to the canonical opacity/scrim and layer/portal domain contracts.
    pub overlay_meaning_traces_to_domain_contracts: bool,
    /// Support / export reads a single canonical scrim / overlay-depth registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverlayRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverlayRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting visual-interaction audit for the lane.
    pub interaction_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5OverlayRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5OverlayRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5OverlayRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5OverlayRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5OverlayRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5OverlayRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5OverlayRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5OverlayRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 opacity / scrim and overlay-depth registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5OverlayRegistriesPacket {
    /// Record kind; must equal [`M5_OVERLAY_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_OVERLAY_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5OverlayRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5OverlayRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5OverlayRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5OverlayRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5OverlayRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5OverlayRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5OverlayRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5OverlayRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_OVERLAY_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_OVERLAY_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5OverlayRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_OVERLAY_REGISTRIES_RECORD_KIND {
            violations.push(M5OverlayRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_OVERLAY_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5OverlayRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5OverlayRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5OverlayRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 opacity / scrim and overlay-depth registries packet serializes"),
        ) {
            violations.push(M5OverlayRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 opacity / scrim and overlay-depth registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,scrim_entries,overlay_depth_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .scrim_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.overlay_depth_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.scrim_entries.len(),
                row.overlay_depth_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Opacity / Scrim and Overlay-Depth Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Overlay depth classes: {}\n",
            self.vocabulary_set.overlay_depth_classes.join(", ")
        ));
        out.push_str(&format!(
            "- Runtime clamps: {}\n",
            self.vocabulary_set.runtime_clamps.join(", ")
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
                "  - Scrim entries: {} / overlay-depth entries: {}\n",
                row.scrim_entries.len(),
                row.overlay_depth_entries.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5OverlayRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5OverlayRegistriesViolation>),
}

impl fmt::Display for M5OverlayRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 opacity / scrim and overlay-depth registries export parse failed: {error}"
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
                    "m5 opacity / scrim and overlay-depth registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5OverlayRegistriesArtifactError {}

/// Validation failures emitted by [`M5OverlayRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5OverlayRegistriesViolation {
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
    /// A registry row does not point at the canonical scrim and layer domain schemas.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (orientation-erasing, contrast-losing, raw-opacity,
    /// clamp-incomplete, or an overlay-depth entry that bypasses the shared z-order or drops out of it).
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
    /// First-consumer canonical adoption is not proven: clean scrim entries do not cover the canonical
    /// semantic-role families or the first shell / dialog / panel / embedded / notification surfaces, no
    /// raw-opacity example degrades, or a clean entry inlines a raw opacity value.
    FirstConsumersUseCanonicalOverlayGrammarNotProven,
    /// Scrim contrast / orientation preservation is not proven: clean scrim entries do not cover the blocking
    /// depth classes with full clamp coverage while preserving orientation and text contrast, no
    /// clamp-incomplete or orientation-erased example degrades, or a clean entry erases orientation or drops
    /// text contrast.
    ScrimsPreserveContrastAndOrientationNotProven,
    /// Blocking-versus-nonblocking depth truth is not proven: clean overlay-depth entries do not cover the
    /// blocking depth classes and at least one non-blocking class, no private-bypass or not-stacked example
    /// degrades, clean entries do not trace to a canonical token, or a clean entry bypasses the shared model.
    BlockingVersusNonBlockingDepthTruthNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5OverlayRegistriesViolation {
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
            Self::FirstConsumersUseCanonicalOverlayGrammarNotProven => {
                "first_consumers_use_canonical_overlay_grammar_not_proven"
            }
            Self::ScrimsPreserveContrastAndOrientationNotProven => {
                "scrims_preserve_contrast_and_orientation_not_proven"
            }
            Self::BlockingVersusNonBlockingDepthTruthNotProven => {
                "blocking_versus_nonblocking_depth_truth_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_opacity_scrim_overlay_depth_registries_export(
) -> Result<M5OverlayRegistriesPacket, M5OverlayRegistriesArtifactError> {
    let packet: M5OverlayRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-opacity-scrim-and-overlay-depth-registries-proof/support_export.json"
    )))
    .map_err(M5OverlayRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5OverlayRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5OverlayRegistriesPacket,
    violations: &mut Vec<M5OverlayRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_OVERLAY_REGISTRIES_SCHEMA_REF,
        M5_OVERLAY_REGISTRIES_DOC_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF,
        M5_OPACITY_SCRIM_SCHEMA_REF,
        M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5OverlayRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5OverlayRegistriesPacket,
    violations: &mut Vec<M5OverlayRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5OverlayRegistriesViolation::NoRegistryRows);
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
            violations.push(M5OverlayRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5OverlayRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5OverlayRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_OPACITY_SCRIM_SCHEMA_REF)
            || !refs.contains(M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF)
        {
            violations.push(M5OverlayRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.scrim_entries.is_empty() || row.overlay_depth_entries.is_empty() {
            violations.push(M5OverlayRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5OverlayRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5OverlayRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5OverlayRegistriesPacket,
    violations: &mut Vec<M5OverlayRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.scrim_registry_names_token_role_and_depth_class,
        review.opacity_scrim_classes_distinguish_lightweight_from_blocking,
        review.scrim_never_erases_orientation,
        review.every_entry_covers_all_runtime_clamps,
        review.runtime_clamps_narrow_overlay_behavior_honestly,
        review.scrims_name_contrast_treatment_not_unreadable_backdrop,
        review.overlays_stack_under_one_shared_z_order_model,
        review.blocking_versus_nonblocking_depth_truth_caught_before_release,
        review.first_consumers_use_canonical_overlay_grammar,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5OverlayRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5OverlayRegistriesPacket,
    violations: &mut Vec<M5OverlayRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_shared_registries,
        projection.dialog_consumes_shared_registries,
        projection.panel_consumes_shared_registries,
        projection.embedded_and_notification_consume_shared_registries,
        projection.overlay_meaning_traces_to_domain_contracts,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(M5OverlayRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5OverlayRegistriesPacket,
    violations: &mut Vec<M5OverlayRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5OverlayRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5OverlayRegistriesPacket,
    violations: &mut Vec<M5OverlayRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.interaction_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5OverlayRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted
/// by governance bools.
fn validate_acceptance_criteria(
    packet: &M5OverlayRegistriesPacket,
    violations: &mut Vec<M5OverlayRegistriesViolation>,
) {
    let scrims = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.scrim_entries.iter())
    };
    let depths = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.overlay_depth_entries.iter())
    };

    // AC1: the first claimed consumers use one canonical overlay grammar instead of feature-local scrims.
    // Clean scrim entries cover the overlay / attention semantic-role families and the first shell / dialog /
    // panel / embedded / notification surfaces, a raw-opacity example degrades, and no clean entry inlines a
    // raw opacity value.
    let clean_semantic_roles: BTreeSet<String> = scrims()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.semantic_role.clone())
        .collect();
    let clean_surfaces: BTreeSet<String> = scrims()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .collect();
    let semantic_families_covered = ["overlay", "attention"]
        .iter()
        .all(|r| clean_semantic_roles.contains(*r));
    let first_surfaces_covered = M5OverlaySurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let raw_opacity_degrades = scrims()
        .any(|ex| ex.degrade_reason == Some(M5ScrimEntryDegradeReason::RawOpacityValueInlined));
    let no_clean_raw_scrim = !scrims().any(|ex| ex.is_clean() && !ex.references_canonical_token);
    if !(semantic_families_covered
        && first_surfaces_covered
        && raw_opacity_degrades
        && no_clean_raw_scrim)
    {
        violations
            .push(M5OverlayRegistriesViolation::FirstConsumersUseCanonicalOverlayGrammarNotProven);
    }

    // AC2: scrims and overlay-depth classes preserve contrast and orientation. Clean scrim entries cover
    // every blocking depth class with full clamp coverage while preserving orientation and text contrast, a
    // clamp-incomplete example degrades, an orientation-erased example degrades, and no clean entry erases
    // orientation or drops text contrast.
    let clean_blocking_classes: BTreeSet<String> = scrims()
        .filter(|ex| {
            ex.is_clean()
                && ex.depth_class_is_blocking
                && ex.covers_all_clamps
                && ex.preserves_orientation
                && ex.preserves_text_contrast
        })
        .map(|ex| ex.depth_class.clone())
        .collect();
    let blocking_classes_covered = M5OverlayDepthClass::BLOCKING_CLASSES
        .iter()
        .all(|s| clean_blocking_classes.contains(s.as_str()));
    let clamp_incomplete_degrades = scrims()
        .any(|ex| ex.degrade_reason == Some(M5ScrimEntryDegradeReason::ClampCoverageIncomplete));
    let orientation_erased_degrades = scrims()
        .any(|ex| ex.degrade_reason == Some(M5ScrimEntryDegradeReason::OrientationErasedByScrim));
    let no_clean_unsafe = !scrims()
        .any(|ex| ex.is_clean() && (!ex.preserves_orientation || !ex.preserves_text_contrast));
    if !(blocking_classes_covered
        && clamp_incomplete_degrades
        && orientation_erased_degrades
        && no_clean_unsafe)
    {
        violations
            .push(M5OverlayRegistriesViolation::ScrimsPreserveContrastAndOrientationNotProven);
    }

    // AC3: the first claimed overlays show correct blocking-versus-nonblocking depth truth. Clean
    // overlay-depth entries cover every blocking depth class and at least one non-blocking class with full
    // clamp coverage while stacking under the shared z-order model, a private-bypass example degrades, a
    // not-stacked example degrades, clean entries trace to a canonical token, and no clean entry bypasses the
    // shared model.
    let clean_blocking_depth: BTreeSet<String> = depths()
        .filter(|ex| ex.is_clean() && ex.depth_class_is_blocking && ex.stacks_under_shared_model)
        .map(|ex| ex.depth_class.clone())
        .collect();
    let blocking_depth_covered = M5OverlayDepthClass::BLOCKING_CLASSES
        .iter()
        .all(|s| clean_blocking_depth.contains(s.as_str()));
    let nonblocking_depth_covered = depths().any(|ex| ex.is_clean() && !ex.depth_class_is_blocking);
    let private_bypass_degrades = depths().any(|ex| {
        ex.degrade_reason
            == Some(M5OverlayDepthEntryDegradeReason::PrivateLayerBypassWithoutSharedModel)
    });
    let not_stacked_degrades = depths().any(|ex| {
        ex.degrade_reason == Some(M5OverlayDepthEntryDegradeReason::NotStackedUnderSharedModel)
    });
    let traceable_depth = depths().any(|ex| ex.is_clean() && ex.references_canonical_token);
    let no_clean_bypass = !depths().any(|ex| ex.is_clean() && !ex.stacks_under_shared_model);
    if !(blocking_depth_covered
        && nonblocking_depth_covered
        && private_bypass_degrades
        && not_stacked_degrades
        && traceable_depth
        && no_clean_bypass)
    {
        violations.push(M5OverlayRegistriesViolation::BlockingVersusNonBlockingDepthTruthNotProven);
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
    M5VisualInteractionFamily::OpacityScrim,
    M5VisualInteractionFamily::LayerOrder,
];
