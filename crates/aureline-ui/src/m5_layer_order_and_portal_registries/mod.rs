//! Implemented M5 layer-order (z-tier) and portal-ownership registries.
//!
//! The frozen [motion / layer / iconography matrix][matrix] names Aureline's seven visual-interaction
//! families and locks their controlled vocabulary. This module is the layer / portal implement lane over
//! that matrix: it turns the two families that carry the *overlay stack* grammar — the **layer-order**
//! z-tier registry (one canonical base / sticky / floating / menu / dialog / toast / critical ordering no
//! private overlay may bypass or hard-code above) and the **portal-ownership** registry (portals that attach
//! to their owning surface and restore safely) — into registry resolvers that produce export-safe, honest
//! projections, so a user can trust that every floating surface, menu, dialog, toast, and critical prompt
//! keeps one ordering model, that no first-party or extension surface hard-codes always-on-top behavior to
//! bypass the shared model, and that a portal stays attached to its owning window and tears down or restores
//! with its owner rather than stranding an orphaned overlay.
//!
//! Three implementation requirements drive the resolvers:
//!
//! * **Implement the canonical z-tier registry for base, sticky, floating, menu, dialog, toast, and critical
//!   overlays, with owning-surface attachment and restore-safe portal semantics.**
//!   [`resolve_layer_tier_entry`] refuses to read as a clean, shared-z-order-safe layer-tier entry unless it
//!   names a canonical token, a classified [z-tier][M5LayerTier], a layer-order role, and a surface context,
//!   stacks under the single shared z-order model, and traces to a canonical token rather than an inlined raw
//!   z-index; otherwise it degrades.
//! * **Prevent hard-coded always-on-top behavior in first-party or extension surfaces from bypassing the
//!   shared layer model.** Every layer-tier entry carries the `hardcodes_always_on_top` guard, and degrades
//!   to [`M5LayerTierEntryDegradeReason::AlwaysOnTopBypassesSharedModel`] when a private layer or a
//!   hard-coded always-on-top overlay would otherwise stack outside the shared model.
//! * **Wire first palette, hover / peek, dialog, toast, and embedded-boundary consumers plus fixtures for
//!   owning-window and portal-order continuity.** [`resolve_portal_entry`] refuses to read as a clean,
//!   owning-surface-attached portal entry unless it names a canonical token, a classified z-tier, a
//!   portal-ownership role, and an [attachment mode][M5PortalAttachmentMode], attaches to its owning surface,
//!   and restores safely when its owner changes; a detached portal degrades honestly. Each registry row
//!   carries the render [surface context][M5LayerPortalSurfaceContext] so a z-order or portal regression
//!   degrades honestly, and the acceptance-criteria gate proves the first claimed consumers obey one
//!   canonical layer-order model before release evidence turns green.
//!
//! The resolvers reuse the frozen matrix vocabulary directly — the [`M5VisualInteractionRole`] role
//! vocabulary, the [`M5LayerOrderRole`] layer-order vocabulary, and the [`M5PortalOwnershipRole`]
//! portal-ownership vocabulary — so shell, dialog, panel, embedded, notification, and support surfaces can
//! never fork their own z-order or portal meaning. Raw secret values and private endpoints stay outside the
//! export boundary.
//!
//! [matrix]: crate::m5_motion_layer_iconography_matrix

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_layer_order_and_portal_registries,
    seeded_m5_layer_order_and_portal_registries_onboarding_ui_preview_narrowed,
    seeded_m5_layer_order_and_portal_registries_shell_ui_beta_narrowed,
    M5_LAYER_PORTAL_REGISTRIES_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::m5_motion_layer_iconography_matrix::{
    M5LayerOrderRole, M5PortalOwnershipRole, M5VisualInteractionAccessibilityRoute,
    M5VisualInteractionConsumerSurface, M5VisualInteractionDeploymentLine,
    M5VisualInteractionDowngradeTrigger, M5VisualInteractionFamily,
    M5VisualInteractionQualificationClass, M5VisualInteractionRequiredLabel,
    M5VisualInteractionRole, M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF,
    M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF, M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5LayerPortalRegistriesPacket`].
pub const M5_LAYER_PORTAL_REGISTRIES_RECORD_KIND: &str =
    "implement_m5_layer_order_and_portal_registries";

/// Schema version for M5 layer-order and portal registry records.
pub const M5_LAYER_PORTAL_REGISTRIES_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the combined registries schema.
pub const M5_LAYER_PORTAL_REGISTRIES_SCHEMA_REF: &str =
    "schemas/design-system/m5-layer-order-and-portal-registries.schema.json";

/// Repo-relative path of the registries doc.
pub const M5_LAYER_PORTAL_REGISTRIES_DOC_REF: &str =
    "docs/design-system/m5_layer_order_and_portal_registries.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_LAYER_PORTAL_REGISTRIES_ARTIFACT_REF: &str =
    "artifacts/release/m5-layer-order-and-portal-registries-proof/support_export.json";

/// Repo-relative path of the checked machine-readable registries CSV.
pub const M5_LAYER_PORTAL_REGISTRIES_CSV_REF: &str =
    "artifacts/release/m5-layer-order-and-portal-registries-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_LAYER_PORTAL_REGISTRIES_REPORT_REF: &str =
    "artifacts/release/m5-layer-order-and-portal-registries-proof/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_LAYER_PORTAL_REGISTRIES_FIXTURE_DIR: &str =
    "fixtures/ui/m5-layer-order-and-portal-registries";

/// Consumer surface a registry row projects onto. Reuses the frozen matrix consumer-surface taxonomy so no
/// lane invents a parallel surface set.
pub type M5LayerPortalRegistriesConsumerSurface = M5VisualInteractionConsumerSurface;

/// Controlled z-order tier a layer-tier or portal entry maps, so every floating surface, menu, dialog,
/// toast, and critical prompt keeps one canonical ordering (base &lt; sticky &lt; floating &lt; menu &lt;
/// dialog &lt; toast &lt; critical) rather than competing through ad hoc z-index rules. Minted by this lane
/// because the frozen matrix carries the high-level layer-order role but not the concrete named tiers the
/// z-tier acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LayerTier {
    /// The base workspace-content tier.
    Base,
    /// The sticky (pinned header / affix) tier.
    Sticky,
    /// The floating (hover / peek / inline preview) tier.
    Floating,
    /// The menu (palette / context menu / popover) tier.
    Menu,
    /// The dialog / modal tier.
    Dialog,
    /// The transient toast / notification tier.
    Toast,
    /// The critical (blocking prompt / credential) tier.
    Critical,
    /// The z-tier is unclassified, which is disallowed.
    TierUnclassified,
}

impl M5LayerTier {
    /// Every z-tier, in declaration (ascending z-order) order.
    pub const ALL: [Self; 8] = [
        Self::Base,
        Self::Sticky,
        Self::Floating,
        Self::Menu,
        Self::Dialog,
        Self::Toast,
        Self::Critical,
        Self::TierUnclassified,
    ];

    /// The seven canonical tiers the z-tier registry names, in ascending z-order.
    pub const CANONICAL_TIERS: [Self; 7] = [
        Self::Base,
        Self::Sticky,
        Self::Floating,
        Self::Menu,
        Self::Dialog,
        Self::Toast,
        Self::Critical,
    ];

    /// The tiers the acceptance criteria require to stop competing through ad hoc z-order rules.
    pub const COMPETING_TIERS: [Self; 4] = [Self::Menu, Self::Dialog, Self::Toast, Self::Critical];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Base => "base",
            Self::Sticky => "sticky",
            Self::Floating => "floating",
            Self::Menu => "menu",
            Self::Dialog => "dialog",
            Self::Toast => "toast",
            Self::Critical => "critical",
            Self::TierUnclassified => "tier_unclassified",
        }
    }

    /// Whether the z-tier is classified (never the unclassified sentinel).
    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::TierUnclassified)
    }

    /// Whether this is a competing tier that must stack under the shared model rather than through ad hoc
    /// z-order rules.
    pub const fn is_competing(self) -> bool {
        matches!(
            self,
            Self::Menu | Self::Dialog | Self::Toast | Self::Critical
        )
    }

    /// The canonical z-index this tier occupies in the single shared ordering. The unclassified sentinel
    /// carries `0` but is gated out before ordering matters.
    pub const fn z_index(self) -> u32 {
        match self {
            Self::Base => 0,
            Self::Sticky => 1,
            Self::Floating => 2,
            Self::Menu => 3,
            Self::Dialog => 4,
            Self::Toast => 5,
            Self::Critical => 6,
            Self::TierUnclassified => 0,
        }
    }
}

/// Controlled portal attachment mode a portal entry pairs with its ownership role so it stays attached to its
/// owning surface, contains focus, tears down with its owner, or restores safely on reparent: anchored to the
/// owning window, tracked to an anchor element, contained within a focus scope, torn down with its owner, or
/// re-parented restore-safe. Minted by this lane, tracking the owning-surface-attachment / restore-safe rule
/// the portal acceptance criteria require by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortalAttachmentMode {
    /// The portal is anchored to its owning window.
    OwningWindowAnchored,
    /// The portal tracks an anchor element within its owning surface.
    AnchorElementTracked,
    /// The portal contains focus within its owning scope.
    FocusScopeContained,
    /// The portal tears down when its owner tears down.
    OwnerDrivenTeardown,
    /// The portal re-parents restore-safe when its owning surface changes.
    RestoreSafeReparent,
    /// No attachment mode is paired with the portal, which is disallowed.
    NoneDisallowed,
}

impl M5PortalAttachmentMode {
    /// Every attachment mode, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::OwningWindowAnchored,
        Self::AnchorElementTracked,
        Self::FocusScopeContained,
        Self::OwnerDrivenTeardown,
        Self::RestoreSafeReparent,
        Self::NoneDisallowed,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OwningWindowAnchored => "owning_window_anchored",
            Self::AnchorElementTracked => "anchor_element_tracked",
            Self::FocusScopeContained => "focus_scope_contained",
            Self::OwnerDrivenTeardown => "owner_driven_teardown",
            Self::RestoreSafeReparent => "restore_safe_reparent",
            Self::NoneDisallowed => "none_disallowed",
        }
    }

    /// Whether an attachment mode is present (never the disallowed none sentinel).
    pub const fn is_present(self) -> bool {
        !matches!(self, Self::NoneDisallowed)
    }
}

/// Controlled render context — which claimed M5 surface renders the registry entry, so a layer-tier or
/// portal's z-order truth stays stable whether it appears in the shell, a dialog, a panel, an embedded
/// surface, or a notification. Minted by this lane, tracking the first-consumer surfaces the implementation
/// requirement names directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LayerPortalSurfaceContext {
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

impl M5LayerPortalSurfaceContext {
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

/// One mandatory rendered part a layer-tier or portal entry must be able to show, so no tier, attachment, or
/// token fact is left implicit behind a raw z-index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LayerPortalRegistryAnatomyPart {
    /// The entry's stable identity.
    Identity,
    /// The entry's semantic role.
    SemanticRole,
    /// The canonical token reference the entry points at.
    TokenReference,
    /// The z-tier the entry maps (both entries).
    LayerTier,
    /// The canonical z-index the tier occupies (both entries).
    ZIndex,
    /// The portal attachment mode paired with the ownership role (portal entry).
    AttachmentMode,
    /// The layer-order role named by the entry (layer-tier entry).
    LayerOrderRole,
    /// The portal-ownership role named by the entry (portal entry).
    PortalOwnershipRole,
    /// The render / surface context (both entries).
    SurfaceContext,
    /// The non-visual keyboard / screen-reader route to the entry.
    KeyboardRoute,
    /// The plain-language meaning of the token (both entries).
    PlainLanguageMeaning,
}

impl M5LayerPortalRegistryAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::Identity,
        Self::SemanticRole,
        Self::TokenReference,
        Self::LayerTier,
        Self::ZIndex,
        Self::AttachmentMode,
        Self::LayerOrderRole,
        Self::PortalOwnershipRole,
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
            Self::LayerTier => "layer_tier",
            Self::ZIndex => "z_index",
            Self::AttachmentMode => "attachment_mode",
            Self::LayerOrderRole => "layer_order_role",
            Self::PortalOwnershipRole => "portal_ownership_role",
            Self::SurfaceContext => "surface_context",
            Self::KeyboardRoute => "keyboard_route",
            Self::PlainLanguageMeaning => "plain_language_meaning",
        }
    }
}

/// Next safe action a registry entry surfaces so a user is never left without a route to inspect a z-tier,
/// reattach an owning surface, or trace a degraded layer / portal token.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LayerPortalRegistryNextAction {
    /// Expand the layer / portal's plain-language meaning.
    ExpandLayerMeaning,
    /// Inspect the z-tier the entry maps.
    InspectLayerTier,
    /// Reattach the portal to its owning surface.
    ReattachOwningSurface,
    /// Trace the entry back to its canonical token.
    TraceCanonicalToken,
    /// Review a blocked / degraded entry.
    ReviewBlockedOrDegraded,
    /// No action is needed; the entry is clean.
    NoActionNeeded,
}

impl M5LayerPortalRegistryNextAction {
    /// Every next action, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::ExpandLayerMeaning,
        Self::InspectLayerTier,
        Self::ReattachOwningSurface,
        Self::TraceCanonicalToken,
        Self::ReviewBlockedOrDegraded,
        Self::NoActionNeeded,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExpandLayerMeaning => "expand_layer_meaning",
            Self::InspectLayerTier => "inspect_layer_tier",
            Self::ReattachOwningSurface => "reattach_owning_surface",
            Self::TraceCanonicalToken => "trace_canonical_token",
            Self::ReviewBlockedOrDegraded => "review_blocked_or_degraded",
            Self::NoActionNeeded => "no_action_needed",
        }
    }
}

/// Field a registry row exposes in the support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LayerPortalRegistryExportField {
    /// The consumer surface.
    ConsumerSurface,
    /// The interaction families covered.
    InteractionFamilies,
    /// The z-tiers carried.
    LayerTiers,
    /// The degrade reasons observed.
    DegradeReasons,
    /// The qualification class.
    Qualification,
    /// The semantic roles named.
    SemanticRoles,
    /// Whether entries stack under the single shared z-order model.
    ZOrderModel,
    /// The attachment modes paired.
    AttachmentModes,
    /// The render / surface context.
    SurfaceContext,
    /// The portal-ownership roles named.
    PortalRoles,
    /// The accountable owner role.
    OwnerRole,
}

impl M5LayerPortalRegistryExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ConsumerSurface,
        Self::InteractionFamilies,
        Self::LayerTiers,
        Self::DegradeReasons,
        Self::Qualification,
        Self::SemanticRoles,
        Self::ZOrderModel,
        Self::AttachmentModes,
        Self::SurfaceContext,
        Self::PortalRoles,
        Self::OwnerRole,
    ];

    /// The five mandatory export fields.
    pub const MANDATORY: [Self; 5] = [
        Self::ConsumerSurface,
        Self::InteractionFamilies,
        Self::LayerTiers,
        Self::DegradeReasons,
        Self::Qualification,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConsumerSurface => "consumer_surface",
            Self::InteractionFamilies => "interaction_families",
            Self::LayerTiers => "layer_tiers",
            Self::DegradeReasons => "degrade_reasons",
            Self::Qualification => "qualification",
            Self::SemanticRoles => "semantic_roles",
            Self::ZOrderModel => "z_order_model",
            Self::AttachmentModes => "attachment_modes",
            Self::SurfaceContext => "surface_context",
            Self::PortalRoles => "portal_roles",
            Self::OwnerRole => "owner_role",
        }
    }
}

/// Reason a layer-tier entry degraded below a clean, shared-z-order-safe state. The degrade-first ladder
/// returns one of these instead of ever letting an always-on-top, private-bypass, raw-z-index, or
/// not-stacked entry read as a clean pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LayerTierEntryDegradeReason {
    /// The canonical token name is unstated; a user cannot trace what the tier means.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The z-tier is unclassified (not in the canonical base..critical ordering).
    LayerTierUnclassified,
    /// A private layer or a hard-coded always-on-top overlay bypasses the shared z-order model.
    AlwaysOnTopBypassesSharedModel,
    /// A raw z-index value is inlined instead of tracing to a canonical token.
    RawZOrderValueInlined,
    /// The tier does not stack under the single shared z-order model.
    NotStackedUnderSharedModel,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5LayerTierEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::LayerTierUnclassified,
        Self::AlwaysOnTopBypassesSharedModel,
        Self::RawZOrderValueInlined,
        Self::NotStackedUnderSharedModel,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::LayerTierUnclassified => "layer_tier_unclassified",
            Self::AlwaysOnTopBypassesSharedModel => "always_on_top_bypasses_shared_model",
            Self::RawZOrderValueInlined => "raw_z_order_value_inlined",
            Self::NotStackedUnderSharedModel => "not_stacked_under_shared_model",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5LayerPortalRegistryNextAction {
        match self {
            Self::TokenNameUnstated | Self::RawZOrderValueInlined => {
                M5LayerPortalRegistryNextAction::TraceCanonicalToken
            }
            Self::LayerTierUnclassified | Self::NotStackedUnderSharedModel => {
                M5LayerPortalRegistryNextAction::InspectLayerTier
            }
            Self::AlwaysOnTopBypassesSharedModel => {
                M5LayerPortalRegistryNextAction::ExpandLayerMeaning
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5LayerPortalRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualInteractionDowngradeTrigger {
        match self {
            Self::AlwaysOnTopBypassesSharedModel | Self::NotStackedUnderSharedModel => {
                M5VisualInteractionDowngradeTrigger::OverlayBypassedSharedZOrder
            }
            Self::LayerTierUnclassified => M5VisualInteractionDowngradeTrigger::LayerTierUnstated,
            Self::TokenNameUnstated | Self::RawZOrderValueInlined => {
                M5VisualInteractionDowngradeTrigger::TokenReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5VisualInteractionDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualInteractionDowngradeTrigger::ProofStale,
        }
    }
}

/// Reason a portal entry degraded below a clean, owning-surface-attached state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PortalEntryDegradeReason {
    /// The canonical token name is unstated.
    TokenNameUnstated,
    /// The render / surface context cannot currently be resolved.
    SurfaceContextUnresolved,
    /// The z-tier is unclassified (not in the canonical base..critical ordering).
    LayerTierUnclassified,
    /// A portal detaches from its owning surface (orphaned or a disallowed detached role).
    PortalDetachedFromOwningSurface,
    /// No attachment mode is paired with the portal.
    AttachmentModeMissing,
    /// A raw z-index value is inlined instead of tracing to a canonical token.
    RawZOrderValueInlined,
    /// The portal does not restore safely when its owning surface changes.
    RestoreUnsafeOnOwnerChange,
    /// The supporting proof packet has gone stale.
    ProofStale,
}

impl M5PortalEntryDegradeReason {
    /// Every degrade reason, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::TokenNameUnstated,
        Self::SurfaceContextUnresolved,
        Self::LayerTierUnclassified,
        Self::PortalDetachedFromOwningSurface,
        Self::AttachmentModeMissing,
        Self::RawZOrderValueInlined,
        Self::RestoreUnsafeOnOwnerChange,
        Self::ProofStale,
    ];

    /// Stable token recorded in exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TokenNameUnstated => "token_name_unstated",
            Self::SurfaceContextUnresolved => "surface_context_unresolved",
            Self::LayerTierUnclassified => "layer_tier_unclassified",
            Self::PortalDetachedFromOwningSurface => "portal_detached_from_owning_surface",
            Self::AttachmentModeMissing => "attachment_mode_missing",
            Self::RawZOrderValueInlined => "raw_z_order_value_inlined",
            Self::RestoreUnsafeOnOwnerChange => "restore_unsafe_on_owner_change",
            Self::ProofStale => "proof_stale",
        }
    }

    /// Next safe action for this reason.
    pub const fn next_action(self) -> M5LayerPortalRegistryNextAction {
        match self {
            Self::TokenNameUnstated | Self::RawZOrderValueInlined => {
                M5LayerPortalRegistryNextAction::TraceCanonicalToken
            }
            Self::LayerTierUnclassified => M5LayerPortalRegistryNextAction::InspectLayerTier,
            Self::PortalDetachedFromOwningSurface
            | Self::AttachmentModeMissing
            | Self::RestoreUnsafeOnOwnerChange => {
                M5LayerPortalRegistryNextAction::ReattachOwningSurface
            }
            Self::SurfaceContextUnresolved | Self::ProofStale => {
                M5LayerPortalRegistryNextAction::ReviewBlockedOrDegraded
            }
        }
    }

    /// The matching downgrade trigger recorded on the frozen matrix.
    pub const fn downgrade_trigger(self) -> M5VisualInteractionDowngradeTrigger {
        match self {
            Self::PortalDetachedFromOwningSurface
            | Self::AttachmentModeMissing
            | Self::RestoreUnsafeOnOwnerChange => {
                M5VisualInteractionDowngradeTrigger::PortalDetachedFromOwningSurface
            }
            Self::LayerTierUnclassified => M5VisualInteractionDowngradeTrigger::LayerTierUnstated,
            Self::TokenNameUnstated | Self::RawZOrderValueInlined => {
                M5VisualInteractionDowngradeTrigger::TokenReferenceUnstated
            }
            Self::SurfaceContextUnresolved => {
                M5VisualInteractionDowngradeTrigger::SemanticRoleUnstated
            }
            Self::ProofStale => M5VisualInteractionDowngradeTrigger::ProofStale,
        }
    }
}

/// Input to [`resolve_layer_tier_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LayerTierEntryResolutionInput {
    /// Stable identity of the layer-tier entry.
    pub entry_id: String,
    /// The canonical token name (e.g. `layer.menu.palette`); empty means unstated.
    pub token_name: String,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualInteractionRole,
    /// The layer-order role (from the frozen matrix vocabulary).
    pub layer_order_role: M5LayerOrderRole,
    /// The z-tier this entry maps.
    pub layer_tier: M5LayerTier,
    /// The render / surface context.
    pub surface_context: M5LayerPortalSurfaceContext,
    /// True when the entry hard-codes always-on-top behavior (disallowed — must be `false` for a clean pass).
    pub hardcodes_always_on_top: bool,
    /// True when the tier stacks under the single shared z-order model.
    pub stacks_under_shared_model: bool,
    /// True when the entry traces to a canonical token (never an inlined raw z-index value).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe layer-tier projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedLayerTierEntry {
    /// Stable identity of the layer-tier entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role demands an accessible fallback.
    pub semantic_role_demands_accessible_fallback: bool,
    /// The layer-order-role token named by the entry.
    pub layer_order_role: String,
    /// Whether the layer-order role names the disallowed private-layer-bypass token.
    pub layer_order_role_is_private_bypass: bool,
    /// The z-tier token named by the entry.
    pub layer_tier: String,
    /// Whether the z-tier is classified into the canonical base..critical ordering.
    pub layer_tier_is_classified: bool,
    /// Whether this is a competing tier that must stack under the shared model.
    pub layer_tier_is_competing: bool,
    /// The canonical z-index the tier occupies.
    pub z_index: u32,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// Whether the entry hard-codes always-on-top behavior.
    pub hardcodes_always_on_top: bool,
    /// Whether the tier stacks under the single shared z-order model.
    pub stacks_under_shared_model: bool,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean, shared-z-order-safe state.
    pub degrade_reason: Option<M5LayerTierEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5LayerPortalRegistryNextAction,
    /// Whether the z-order model holds (clean entry naming every fact).
    pub z_order_model_holds: bool,
}

impl M5ResolvedLayerTierEntry {
    /// Whether this layer-tier entry reads as a clean, shared-z-order-safe state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Input to [`resolve_portal_entry`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5PortalEntryResolutionInput {
    /// Stable identity of the portal entry.
    pub entry_id: String,
    /// The canonical token name; empty means unstated.
    pub token_name: String,
    /// The portal-ownership role (from the frozen matrix vocabulary).
    pub portal_ownership_role: M5PortalOwnershipRole,
    /// The high-level semantic role (from the frozen matrix vocabulary).
    pub semantic_role: M5VisualInteractionRole,
    /// The z-tier this portal maps.
    pub layer_tier: M5LayerTier,
    /// The attachment mode paired with the ownership role.
    pub attachment_mode: M5PortalAttachmentMode,
    /// The render / surface context.
    pub surface_context: M5LayerPortalSurfaceContext,
    /// True when the portal attaches to its owning surface (never orphaned).
    pub attaches_to_owning_surface: bool,
    /// True when the portal restores safely when its owning surface changes.
    pub restore_safe: bool,
    /// True when the entry traces to a canonical token (never an inlined raw z-index value).
    pub references_canonical_token: bool,
    /// True when the supporting proof packet is fresh.
    pub proof_fresh: bool,
}

/// Resolved, export-safe portal projection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedPortalEntry {
    /// Stable identity of the portal entry.
    pub entry_id: String,
    /// The canonical token name named by the entry.
    pub token_name: String,
    /// The portal-ownership-role token named by the entry.
    pub portal_ownership_role: String,
    /// Whether the portal-ownership role names the disallowed detached-portal token.
    pub portal_role_is_detached: bool,
    /// The semantic-role token named by the entry.
    pub semantic_role: String,
    /// Whether the semantic role demands an accessible fallback.
    pub semantic_role_demands_accessible_fallback: bool,
    /// The z-tier token named by the entry.
    pub layer_tier: String,
    /// Whether the z-tier is classified into the canonical base..critical ordering.
    pub layer_tier_is_classified: bool,
    /// The canonical z-index the tier occupies.
    pub z_index: u32,
    /// The attachment-mode token named by the entry.
    pub attachment_mode: String,
    /// Whether an attachment mode is present.
    pub attachment_mode_present: bool,
    /// The render / surface-context token named by the entry.
    pub surface_context: String,
    /// Whether the portal attaches to its owning surface.
    pub attaches_to_owning_surface: bool,
    /// Whether the portal restores safely on owner change.
    pub restore_safe: bool,
    /// Whether the entry traces to a canonical token.
    pub references_canonical_token: bool,
    /// Degrade reason, if the entry could not read as a clean, owning-surface-attached state.
    pub degrade_reason: Option<M5PortalEntryDegradeReason>,
    /// Next safe action offered.
    pub next_action: M5LayerPortalRegistryNextAction,
    /// Whether the owning-surface attachment is preserved (clean entry naming every fact).
    pub owning_surface_attachment_preserved: bool,
}

impl M5ResolvedPortalEntry {
    /// Whether this portal entry reads as a clean, owning-surface-attached state.
    pub fn is_clean(&self) -> bool {
        self.degrade_reason.is_none()
    }
}

/// Error emitted when a resolver input carries invalid or forbidden material.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum M5LayerPortalResolutionError {
    /// The layer-tier-entry id was empty.
    EmptyLayerTierEntryId,
    /// The portal-entry id was empty.
    EmptyPortalEntryId,
    /// A field carried forbidden raw material (secret / endpoint).
    ForbiddenMaterial,
}

impl M5LayerPortalResolutionError {
    /// Stable token used in tests and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyLayerTierEntryId => "empty_layer_tier_entry_id",
            Self::EmptyPortalEntryId => "empty_portal_entry_id",
            Self::ForbiddenMaterial => "forbidden_material",
        }
    }
}

impl fmt::Display for M5LayerPortalResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "m5 layer-order and portal registry resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5LayerPortalResolutionError {}

/// Resolves a layer-tier entry so it stays honest under the shared z-order model: the entry names its
/// canonical token, semantic role, layer-order role, z-tier, and surface context, stacks under the single
/// shared z-order model, never hard-codes always-on-top behavior, and traces to a canonical token rather than
/// letting a private layer bypass the shared model.
pub fn resolve_layer_tier_entry(
    input: M5LayerTierEntryResolutionInput,
) -> Result<M5ResolvedLayerTierEntry, M5LayerPortalResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5LayerPortalResolutionError::EmptyLayerTierEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5LayerPortalResolutionError::ForbiddenMaterial);
    }

    let layer_order_role_is_private_bypass = matches!(
        input.layer_order_role,
        M5LayerOrderRole::PrivateLayerBypassDisallowed
    );

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5LayerTierEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5LayerTierEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.layer_tier.is_classified() {
        Some(M5LayerTierEntryDegradeReason::LayerTierUnclassified)
    } else if layer_order_role_is_private_bypass || input.hardcodes_always_on_top {
        Some(M5LayerTierEntryDegradeReason::AlwaysOnTopBypassesSharedModel)
    } else if !input.references_canonical_token {
        Some(M5LayerTierEntryDegradeReason::RawZOrderValueInlined)
    } else if !input.stacks_under_shared_model {
        Some(M5LayerTierEntryDegradeReason::NotStackedUnderSharedModel)
    } else if !input.proof_fresh {
        Some(M5LayerTierEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5LayerPortalRegistryNextAction::InspectLayerTier,
    };

    Ok(M5ResolvedLayerTierEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_demands_accessible_fallback: input
            .semantic_role
            .demands_accessible_fallback(),
        layer_order_role: input.layer_order_role.as_str().to_owned(),
        layer_order_role_is_private_bypass,
        layer_tier: input.layer_tier.as_str().to_owned(),
        layer_tier_is_classified: input.layer_tier.is_classified(),
        layer_tier_is_competing: input.layer_tier.is_competing(),
        z_index: input.layer_tier.z_index(),
        surface_context: input.surface_context.as_str().to_owned(),
        hardcodes_always_on_top: input.hardcodes_always_on_top,
        stacks_under_shared_model: input.stacks_under_shared_model,
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        z_order_model_holds: degrade_reason.is_none(),
    })
}

/// Resolves a portal entry so it stays attached to its owning surface: the entry names its canonical token,
/// portal-ownership role, semantic role, z-tier, attachment mode, and surface context, attaches to its owning
/// surface, restores safely when its owner changes, and traces to a canonical token rather than stranding a
/// detached, orphaned overlay.
pub fn resolve_portal_entry(
    input: M5PortalEntryResolutionInput,
) -> Result<M5ResolvedPortalEntry, M5LayerPortalResolutionError> {
    if input.entry_id.trim().is_empty() {
        return Err(M5LayerPortalResolutionError::EmptyPortalEntryId);
    }
    if string_is_forbidden(&input.entry_id) || string_is_forbidden(&input.token_name) {
        return Err(M5LayerPortalResolutionError::ForbiddenMaterial);
    }

    let portal_role_is_detached = matches!(
        input.portal_ownership_role,
        M5PortalOwnershipRole::DetachedPortalDisallowed
    );

    let degrade_reason = if input.token_name.trim().is_empty() {
        Some(M5PortalEntryDegradeReason::TokenNameUnstated)
    } else if !input.surface_context.is_resolved() {
        Some(M5PortalEntryDegradeReason::SurfaceContextUnresolved)
    } else if !input.layer_tier.is_classified() {
        Some(M5PortalEntryDegradeReason::LayerTierUnclassified)
    } else if portal_role_is_detached || !input.attaches_to_owning_surface {
        Some(M5PortalEntryDegradeReason::PortalDetachedFromOwningSurface)
    } else if !input.attachment_mode.is_present() {
        Some(M5PortalEntryDegradeReason::AttachmentModeMissing)
    } else if !input.references_canonical_token {
        Some(M5PortalEntryDegradeReason::RawZOrderValueInlined)
    } else if !input.restore_safe {
        Some(M5PortalEntryDegradeReason::RestoreUnsafeOnOwnerChange)
    } else if !input.proof_fresh {
        Some(M5PortalEntryDegradeReason::ProofStale)
    } else {
        None
    };

    let next_action = match degrade_reason {
        Some(reason) => reason.next_action(),
        None => M5LayerPortalRegistryNextAction::ExpandLayerMeaning,
    };

    Ok(M5ResolvedPortalEntry {
        entry_id: input.entry_id,
        token_name: input.token_name,
        portal_ownership_role: input.portal_ownership_role.as_str().to_owned(),
        portal_role_is_detached,
        semantic_role: input.semantic_role.as_str().to_owned(),
        semantic_role_demands_accessible_fallback: input
            .semantic_role
            .demands_accessible_fallback(),
        layer_tier: input.layer_tier.as_str().to_owned(),
        layer_tier_is_classified: input.layer_tier.is_classified(),
        z_index: input.layer_tier.z_index(),
        attachment_mode: input.attachment_mode.as_str().to_owned(),
        attachment_mode_present: input.attachment_mode.is_present(),
        surface_context: input.surface_context.as_str().to_owned(),
        attaches_to_owning_surface: input.attaches_to_owning_surface,
        restore_safe: input.restore_safe,
        references_canonical_token: input.references_canonical_token,
        degrade_reason,
        next_action,
        owning_surface_attachment_preserved: degrade_reason.is_none(),
    })
}

/// One registry row: one consumer surface bound to the resolved layer-tier and portal entries it must
/// project honestly.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LayerPortalRegistriesRow {
    /// Consumer surface this row projects onto.
    pub consumer_surface: M5LayerPortalRegistriesConsumerSurface,
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
    pub anatomy_parts: Vec<M5LayerPortalRegistryAnatomyPart>,
    /// Export fields exposed (must include the mandatory five).
    pub export_fields: Vec<M5LayerPortalRegistryExportField>,
    /// Downgrade triggers that apply to this row.
    pub downgrade_triggers: Vec<M5VisualInteractionDowngradeTrigger>,
    /// Resolved layer-tier examples.
    pub layer_tier_entries: Vec<M5ResolvedLayerTierEntry>,
    /// Resolved portal examples.
    pub portal_entries: Vec<M5ResolvedPortalEntry>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row (must include the canonical layer / portal domain schema).
    pub source_contract_refs: Vec<String>,
    /// Hard invariant: an overlay never hard-codes always-on-top behavior. MUST be `false`.
    pub overlay_hardcodes_always_on_top: bool,
    /// Hard invariant: a portal never detaches from its owning surface. MUST be `false`.
    pub portal_detaches_from_owning_surface: bool,
    /// Hard invariant: a raw z-index value is never inlined instead of a canonical token. MUST be `false`.
    pub raw_z_order_value_inlined_instead_of_token: bool,
    /// Hard invariant: a layer-order entry never bypasses the shared z-order model. MUST be `false`.
    pub layer_order_bypasses_shared_z_order_model: bool,
}

impl M5LayerPortalRegistriesRow {
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5LayerPortalRegistryAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5LayerPortalRegistryAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    fn declares_mandatory_export_fields(&self) -> bool {
        let present: BTreeSet<M5LayerPortalRegistryExportField> =
            self.export_fields.iter().copied().collect();
        M5LayerPortalRegistryExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    fn honours_invariants(&self) -> bool {
        !self.overlay_hardcodes_always_on_top
            && !self.portal_detaches_from_owning_surface
            && !self.raw_z_order_value_inlined_instead_of_token
            && !self.layer_order_bypasses_shared_z_order_model
    }

    /// True when a clean layer-tier entry preserves shared-z-order safety: it traces to a canonical token,
    /// never names the disallowed private-bypass role, never hard-codes always-on-top, keeps a classified
    /// tier, and stacks under the shared model.
    fn layer_tier_is_honest(ex: &M5ResolvedLayerTierEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && !ex.layer_order_role_is_private_bypass
                && !ex.hardcodes_always_on_top
                && ex.layer_tier_is_classified
                && ex.stacks_under_shared_model)
    }

    /// True when a clean portal entry preserves owning-surface attachment: it traces to a canonical token,
    /// never names the disallowed detached role, attaches to its owning surface, pairs an attachment mode,
    /// keeps a classified tier, and restores safely.
    fn portal_is_honest(ex: &M5ResolvedPortalEntry) -> bool {
        !ex.is_clean()
            || (ex.references_canonical_token
                && !ex.portal_role_is_detached
                && ex.attaches_to_owning_surface
                && ex.attachment_mode_present
                && ex.layer_tier_is_classified
                && ex.restore_safe)
    }

    /// True when every resolved example on this row is honest.
    fn examples_are_honest(&self) -> bool {
        self.layer_tier_entries
            .iter()
            .all(Self::layer_tier_is_honest)
            && self.portal_entries.iter().all(Self::portal_is_honest)
    }
}

/// Self-describing controlled-vocabulary set frozen by the registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LayerPortalRegistriesVocabularySet {
    /// Semantic-role tokens (bound from the frozen matrix).
    pub semantic_roles: Vec<String>,
    /// Layer-order-role tokens (bound from the frozen matrix).
    pub layer_order_roles: Vec<String>,
    /// Portal-ownership-role tokens (bound from the frozen matrix).
    pub portal_ownership_roles: Vec<String>,
    /// Z-tier tokens (minted by this lane).
    pub layer_tiers: Vec<String>,
    /// Attachment-mode tokens (minted by this lane).
    pub attachment_modes: Vec<String>,
    /// Surface-context tokens (minted by this lane).
    pub surface_contexts: Vec<String>,
    /// Layer-tier-entry degrade-reason tokens.
    pub layer_tier_degrade_reasons: Vec<String>,
    /// Portal-entry degrade-reason tokens.
    pub portal_degrade_reasons: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Next-action tokens.
    pub next_actions: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Consumer-surface tokens.
    pub consumer_surfaces: Vec<String>,
}

impl M5LayerPortalRegistriesVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            semantic_roles: tokens(&M5VisualInteractionRole::ALL, |v| v.as_str()),
            layer_order_roles: tokens(&M5LayerOrderRole::ALL, |v| v.as_str()),
            portal_ownership_roles: tokens(&M5PortalOwnershipRole::ALL, |v| v.as_str()),
            layer_tiers: tokens(&M5LayerTier::ALL, |v| v.as_str()),
            attachment_modes: tokens(&M5PortalAttachmentMode::ALL, |v| v.as_str()),
            surface_contexts: tokens(&M5LayerPortalSurfaceContext::ALL, |v| v.as_str()),
            layer_tier_degrade_reasons: tokens(&M5LayerTierEntryDegradeReason::ALL, |v| v.as_str()),
            portal_degrade_reasons: tokens(&M5PortalEntryDegradeReason::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5LayerPortalRegistryAnatomyPart::ALL, |v| v.as_str()),
            next_actions: tokens(&M5LayerPortalRegistryNextAction::ALL, |v| v.as_str()),
            export_fields: tokens(&M5LayerPortalRegistryExportField::ALL, |v| v.as_str()),
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
pub struct M5LayerPortalRegistriesGovernanceReview {
    /// The z-tier registry names a canonical token, layer-order role, and tier for every entry.
    pub layer_tier_registry_names_token_role_and_tier: bool,
    /// The z-tier registry distinguishes base / sticky / floating / menu / dialog / toast / critical.
    pub z_tier_registry_covers_canonical_ordering: bool,
    /// No first-party or extension overlay hard-codes always-on-top behavior.
    pub no_overlay_hardcodes_always_on_top: bool,
    /// Menus, toasts, dialogs, and critical prompts stack under one shared z-order model.
    pub competing_tiers_stack_under_one_shared_model: bool,
    /// Portals attach to their owning surface and restore safely.
    pub portals_attach_to_owning_surface_and_restore_safely: bool,
    /// Portals name an attachment mode so an owning-surface detach is caught.
    pub portals_name_attachment_mode_not_orphaned_overlay: bool,
    /// Overlays and portals stack under one shared z-order model no private layer bypasses.
    pub overlays_stack_under_one_shared_z_order_model: bool,
    /// Layer-order drift is caught by fixtures / diagnostics / release proof before stable promotion.
    pub layer_order_drift_caught_before_release: bool,
    /// The first palette / hover-peek / dialog / toast / embedded consumers use the canonical layer grammar.
    pub first_consumers_use_canonical_layer_grammar: bool,
    /// Every row declares the mandatory anatomy parts.
    pub every_row_declares_mandatory_anatomy: bool,
    /// Every row declares a non-visual accessibility route.
    pub every_row_declares_accessibility_route: bool,
    /// The lane reuses the frozen matrix vocabulary rather than inventing parallel wording.
    pub reuses_frozen_matrix_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LayerPortalRegistriesConsumerProjection {
    /// The shell surface consumes the shared layer / portal registries.
    pub shell_consumes_shared_registries: bool,
    /// The dialog surface consumes the shared layer / portal registries.
    pub dialog_consumes_shared_registries: bool,
    /// The panel surface consumes the shared layer / portal registries.
    pub panel_consumes_shared_registries: bool,
    /// The embedded and notification surfaces consume the shared layer / portal registries.
    pub embedded_and_notification_consume_shared_registries: bool,
    /// Layer / portal behavior traces back to the canonical layer-order-and-portal domain contract.
    pub layer_meaning_traces_to_domain_contract: bool,
    /// Support / export reads a single canonical layer / portal registry source.
    pub support_export_reads_single_registry_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LayerPortalRegistriesProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the registry.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the registries lane.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LayerPortalRegistriesReleasePosture {
    /// Ref of the supporting proof packet for the lane.
    pub proof_packet_ref: String,
    /// Ref of the supporting visual-interaction audit for the lane.
    pub interaction_audit_ref: String,
    /// True when support/export parity is required for every row.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every row.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5LayerPortalRegistriesPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5LayerPortalRegistriesPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5LayerPortalRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LayerPortalRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LayerPortalRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LayerPortalRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LayerPortalRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LayerPortalRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 layer-order and portal registries packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5LayerPortalRegistriesPacket {
    /// Record kind; must equal [`M5_LAYER_PORTAL_REGISTRIES_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_LAYER_PORTAL_REGISTRIES_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable registries label.
    pub registries_label: String,
    /// Registry rows.
    pub registry_rows: Vec<M5LayerPortalRegistriesRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5LayerPortalRegistriesVocabularySet,
    /// Governance-review block.
    pub governance_review: M5LayerPortalRegistriesGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5LayerPortalRegistriesConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5LayerPortalRegistriesProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5LayerPortalRegistriesReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5LayerPortalRegistriesPacket {
    /// Builds a registries packet from stable-lane input.
    pub fn new(input: M5LayerPortalRegistriesPacketInput) -> Self {
        Self {
            record_kind: M5_LAYER_PORTAL_REGISTRIES_RECORD_KIND.to_owned(),
            schema_version: M5_LAYER_PORTAL_REGISTRIES_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5LayerPortalRegistriesViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_LAYER_PORTAL_REGISTRIES_RECORD_KIND {
            violations.push(M5LayerPortalRegistriesViolation::WrongRecordKind);
        }
        if self.schema_version != M5_LAYER_PORTAL_REGISTRIES_SCHEMA_VERSION {
            violations.push(M5LayerPortalRegistriesViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.registries_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5LayerPortalRegistriesViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        if !self.vocabulary_set.matches_canonical() {
            violations.push(M5LayerPortalRegistriesViolation::VocabularySetDrift);
        }
        validate_registry_rows(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);
        validate_acceptance_criteria(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 layer-order and portal registries packet serializes"),
        ) {
            violations.push(M5LayerPortalRegistriesViolation::RawMaterialInExport);
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
            .expect("m5 layer-order and portal registries packet serializes")
    }

    /// Deterministic, machine-readable registries CSV: one row per consumer surface.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "consumer_surface,qualification,owner,layer_tier_entries,portal_entries,degrade_reasons,downgrade_triggers\n",
        );
        for row in &self.registry_rows {
            let degrades: Vec<&str> = row
                .layer_tier_entries
                .iter()
                .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str()))
                .chain(
                    row.portal_entries
                        .iter()
                        .filter_map(|ex| ex.degrade_reason.map(|r| r.as_str())),
                )
                .collect();
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                row.consumer_surface.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                row.layer_tier_entries.len(),
                row.portal_entries.len(),
                degrades.join("|"),
                join_tokens(&row.downgrade_triggers, |v| v.as_str()),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Layer-Order and Portal Registries\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.registries_label));
        out.push_str(&format!(
            "- Consumer surfaces: {}\n",
            self.registry_rows.len()
        ));
        out.push_str(&format!(
            "- Z-tiers: {}\n",
            self.vocabulary_set.layer_tiers.join(", ")
        ));
        out.push_str(&format!(
            "- Attachment modes: {}\n",
            self.vocabulary_set.attachment_modes.join(", ")
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
                "  - Layer-tier entries: {} / portal entries: {}\n",
                row.layer_tier_entries.len(),
                row.portal_entries.len()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in stable registries export.
#[derive(Debug)]
pub enum M5LayerPortalRegistriesArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5LayerPortalRegistriesViolation>),
}

impl fmt::Display for M5LayerPortalRegistriesArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 layer-order and portal registries export parse failed: {error}"
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
                    "m5 layer-order and portal registries export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5LayerPortalRegistriesArtifactError {}

/// Validation failures emitted by [`M5LayerPortalRegistriesPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5LayerPortalRegistriesViolation {
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
    /// A registry row does not point at the canonical layer / portal domain schema.
    DomainSchemaRefMissing,
    /// A registry row carries no resolved examples.
    ExamplesMissing,
    /// A registry row carries a dishonest clean example (always-on-top, private-bypass, raw-z-index,
    /// not-stacked layer-tier entry, or a detached / restore-unsafe portal entry).
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
    /// First-consumer canonical adoption is not proven: clean entries do not cover the canonical
    /// semantic-role families or the first shell / dialog / panel / embedded / notification surfaces, no
    /// raw-z-index example degrades, or a clean entry inlines a raw z-index value.
    FirstConsumersObeyCanonicalLayerModelNotProven,
    /// Competing-tier z-order truth is not proven: clean layer-tier entries do not cover the menu / dialog /
    /// toast / critical competing tiers while stacking under the shared model, no always-on-top or
    /// not-stacked example degrades, or a clean entry hard-codes always-on-top.
    CompetingTiersNoAdHocZOrderNotProven,
    /// Portal continuity and drift visibility is not proven: clean portal entries do not cover the first
    /// surfaces with owning-surface attachment and restore-safety, no detached / restore-unsafe /
    /// tier-drift example degrades, clean portals do not trace to a canonical token, or a clean portal
    /// detaches from its owning surface.
    PortalContinuityAndDriftVisibleNotProven,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5LayerPortalRegistriesViolation {
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
            Self::FirstConsumersObeyCanonicalLayerModelNotProven => {
                "first_consumers_obey_canonical_layer_model_not_proven"
            }
            Self::CompetingTiersNoAdHocZOrderNotProven => {
                "competing_tiers_no_ad_hoc_z_order_not_proven"
            }
            Self::PortalContinuityAndDriftVisibleNotProven => {
                "portal_continuity_and_drift_visible_not_proven"
            }
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable registries export.
pub fn current_stable_m5_layer_order_and_portal_registries_export(
) -> Result<M5LayerPortalRegistriesPacket, M5LayerPortalRegistriesArtifactError> {
    let packet: M5LayerPortalRegistriesPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-layer-order-and-portal-registries-proof/support_export.json"
    )))
    .map_err(M5LayerPortalRegistriesArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5LayerPortalRegistriesArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5LayerPortalRegistriesPacket,
    violations: &mut Vec<M5LayerPortalRegistriesViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_LAYER_PORTAL_REGISTRIES_SCHEMA_REF,
        M5_LAYER_PORTAL_REGISTRIES_DOC_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_SCHEMA_REF,
        M5_MOTION_LAYER_ICONOGRAPHY_MATRIX_DOC_REF,
        M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5LayerPortalRegistriesViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_registry_rows(
    packet: &M5LayerPortalRegistriesPacket,
    violations: &mut Vec<M5LayerPortalRegistriesViolation>,
) {
    if packet.registry_rows.is_empty() {
        violations.push(M5LayerPortalRegistriesViolation::NoRegistryRows);
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
            violations.push(M5LayerPortalRegistriesViolation::RegistryRowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5LayerPortalRegistriesViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export_fields() {
            violations.push(M5LayerPortalRegistriesViolation::MandatoryExportFieldMissing);
        }
        let refs: BTreeSet<&str> = row
            .source_contract_refs
            .iter()
            .map(String::as_str)
            .collect();
        if !refs.contains(M5_LAYER_ORDER_AND_PORTAL_SCHEMA_REF) {
            violations.push(M5LayerPortalRegistriesViolation::DomainSchemaRefMissing);
        }
        if row.layer_tier_entries.is_empty() || row.portal_entries.is_empty() {
            violations.push(M5LayerPortalRegistriesViolation::ExamplesMissing);
        }
        if !row.examples_are_honest() {
            violations.push(M5LayerPortalRegistriesViolation::DishonestExample);
        }
        if !row.honours_invariants() {
            violations.push(M5LayerPortalRegistriesViolation::RowInvariantViolated);
        }
    }
}

fn validate_governance_review(
    packet: &M5LayerPortalRegistriesPacket,
    violations: &mut Vec<M5LayerPortalRegistriesViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.layer_tier_registry_names_token_role_and_tier,
        review.z_tier_registry_covers_canonical_ordering,
        review.no_overlay_hardcodes_always_on_top,
        review.competing_tiers_stack_under_one_shared_model,
        review.portals_attach_to_owning_surface_and_restore_safely,
        review.portals_name_attachment_mode_not_orphaned_overlay,
        review.overlays_stack_under_one_shared_z_order_model,
        review.layer_order_drift_caught_before_release,
        review.first_consumers_use_canonical_layer_grammar,
        review.every_row_declares_mandatory_anatomy,
        review.every_row_declares_accessibility_route,
        review.reuses_frozen_matrix_vocabulary,
    ] {
        if !ok {
            violations.push(M5LayerPortalRegistriesViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5LayerPortalRegistriesPacket,
    violations: &mut Vec<M5LayerPortalRegistriesViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_consumes_shared_registries,
        projection.dialog_consumes_shared_registries,
        projection.panel_consumes_shared_registries,
        projection.embedded_and_notification_consume_shared_registries,
        projection.layer_meaning_traces_to_domain_contract,
        projection.support_export_reads_single_registry_source,
    ] {
        if !ok {
            violations.push(M5LayerPortalRegistriesViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5LayerPortalRegistriesPacket,
    violations: &mut Vec<M5LayerPortalRegistriesViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5LayerPortalRegistriesViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5LayerPortalRegistriesPacket,
    violations: &mut Vec<M5LayerPortalRegistriesViolation>,
) {
    let posture = &packet.release_posture;
    if posture.proof_packet_ref.trim().is_empty()
        || posture.interaction_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5LayerPortalRegistriesViolation::ReleasePostureIncomplete);
    }
}

/// Proves the three acceptance criteria are exercised by the packet's resolved examples, not merely asserted
/// by governance bools.
fn validate_acceptance_criteria(
    packet: &M5LayerPortalRegistriesPacket,
    violations: &mut Vec<M5LayerPortalRegistriesViolation>,
) {
    let tiers = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.layer_tier_entries.iter())
    };
    let portals = || {
        packet
            .registry_rows
            .iter()
            .flat_map(|row| row.portal_entries.iter())
    };

    // AC1: the first claimed consumers obey one canonical layer-order model instead of feature-local z-order
    // rules. Clean entries cover the layer / portal semantic-role families and the first shell / dialog /
    // panel / embedded / notification surfaces, a raw-z-index example degrades, and no clean entry inlines a
    // raw z-index value.
    let clean_semantic_roles: BTreeSet<String> = tiers()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.semantic_role.clone())
        .chain(
            portals()
                .filter(|ex| ex.is_clean())
                .map(|ex| ex.semantic_role.clone()),
        )
        .collect();
    let clean_surfaces: BTreeSet<String> = tiers()
        .filter(|ex| ex.is_clean())
        .map(|ex| ex.surface_context.clone())
        .chain(
            portals()
                .filter(|ex| ex.is_clean())
                .map(|ex| ex.surface_context.clone()),
        )
        .collect();
    let semantic_families_covered = ["layer", "portal"]
        .iter()
        .all(|r| clean_semantic_roles.contains(*r));
    let first_surfaces_covered = M5LayerPortalSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_surfaces.contains(s.as_str()));
    let raw_z_order_degrades = tiers()
        .any(|ex| ex.degrade_reason == Some(M5LayerTierEntryDegradeReason::RawZOrderValueInlined))
        || portals()
            .any(|ex| ex.degrade_reason == Some(M5PortalEntryDegradeReason::RawZOrderValueInlined));
    let no_clean_raw = !tiers().any(|ex| ex.is_clean() && !ex.references_canonical_token)
        && !portals().any(|ex| ex.is_clean() && !ex.references_canonical_token);
    if !(semantic_families_covered
        && first_surfaces_covered
        && raw_z_order_degrades
        && no_clean_raw)
    {
        violations
            .push(M5LayerPortalRegistriesViolation::FirstConsumersObeyCanonicalLayerModelNotProven);
    }

    // AC2: menus, toasts, dialogs, and critical prompts no longer compete through ad hoc z-order rules. Clean
    // layer-tier entries cover every competing tier while stacking under the shared model, an always-on-top
    // example degrades, a not-stacked example degrades, and no clean entry hard-codes always-on-top or fails
    // to stack under the shared model.
    let clean_competing_tiers: BTreeSet<String> = tiers()
        .filter(|ex| {
            ex.is_clean()
                && ex.layer_tier_is_competing
                && ex.stacks_under_shared_model
                && !ex.hardcodes_always_on_top
        })
        .map(|ex| ex.layer_tier.clone())
        .collect();
    let competing_covered = M5LayerTier::COMPETING_TIERS
        .iter()
        .all(|t| clean_competing_tiers.contains(t.as_str()));
    let always_on_top_degrades = tiers().any(|ex| {
        ex.degrade_reason == Some(M5LayerTierEntryDegradeReason::AlwaysOnTopBypassesSharedModel)
    });
    let not_stacked_degrades = tiers().any(|ex| {
        ex.degrade_reason == Some(M5LayerTierEntryDegradeReason::NotStackedUnderSharedModel)
    });
    let no_clean_ad_hoc = !tiers()
        .any(|ex| ex.is_clean() && (ex.hardcodes_always_on_top || !ex.stacks_under_shared_model));
    if !(competing_covered && always_on_top_degrades && not_stacked_degrades && no_clean_ad_hoc) {
        violations.push(M5LayerPortalRegistriesViolation::CompetingTiersNoAdHocZOrderNotProven);
    }

    // AC3: the first claimed consumers keep owning-window and portal-order continuity, and layer-order drift
    // is visible before stable promotion. Clean portal entries cover the first surfaces with owning-surface
    // attachment and restore-safety, a detached example degrades, a restore-unsafe example degrades, a
    // tier-drift (unclassified) example degrades, clean portals trace to a canonical token, and no clean
    // portal detaches from its owning surface.
    let clean_attach_surfaces: BTreeSet<String> = portals()
        .filter(|ex| ex.is_clean() && ex.attaches_to_owning_surface && ex.restore_safe)
        .map(|ex| ex.surface_context.clone())
        .collect();
    let attach_surfaces_covered = M5LayerPortalSurfaceContext::FIRST_CONSUMERS
        .iter()
        .all(|s| clean_attach_surfaces.contains(s.as_str()));
    let detached_degrades = portals().any(|ex| {
        ex.degrade_reason == Some(M5PortalEntryDegradeReason::PortalDetachedFromOwningSurface)
    });
    let restore_unsafe_degrades = portals().any(|ex| {
        ex.degrade_reason == Some(M5PortalEntryDegradeReason::RestoreUnsafeOnOwnerChange)
    });
    let tier_drift_degrades = tiers()
        .any(|ex| ex.degrade_reason == Some(M5LayerTierEntryDegradeReason::LayerTierUnclassified))
        || portals()
            .any(|ex| ex.degrade_reason == Some(M5PortalEntryDegradeReason::LayerTierUnclassified));
    let traceable_portal = portals().any(|ex| ex.is_clean() && ex.references_canonical_token);
    let no_clean_detached = !portals().any(|ex| ex.is_clean() && !ex.attaches_to_owning_surface);
    if !(attach_surfaces_covered
        && detached_degrades
        && restore_unsafe_degrades
        && tier_drift_degrades
        && traceable_portal
        && no_clean_detached)
    {
        violations.push(M5LayerPortalRegistriesViolation::PortalContinuityAndDriftVisibleNotProven);
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
    M5VisualInteractionFamily::LayerOrder,
    M5VisualInteractionFamily::PortalOwnership,
];
