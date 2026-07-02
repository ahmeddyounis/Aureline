//! Menu, context-menu, and command-bar parity certification for every claimed M5 command surface.
//!
//! The [frozen discoverability matrix][matrix] already binds each governed M5 last-mile
//! command-discovery surface — menu items, menu groups, context menus, command bars, keybinding
//! resolver layers, conflict review sheets, import-bridge rows, disabled-command explainers,
//! leader/sequence help overlays, and command-documentation surfaces — to one canonical command record.
//! This lane is the **parity capstone** that certifies, for every one of those ten surface families,
//! that the same action keeps the same canonical label, the same shortcut truth, the same blocked-state
//! reason, and the same authority posture regardless of whether it is reached from a menu, a context
//! menu, a command bar, a keybinding sheet, a help page, a leader overlay, a palette row, or a
//! contextual affordance — so a surface that still invents an alternate label, drops a stale-target
//! guard, hides a command behind a contextual-only route, or cannot reconstruct its blocked-state reason
//! from durable evidence is automatically narrowed or blocked from a stable discoverability claim rather
//! than shipping an over-claim.
//!
//! For every surface family the lane certifies four things the acceptance criteria and implementation
//! requirements demand:
//!
//! - the surface projects the **canonical label, shortcut hint, and blocked-state reason** that match
//!   the command record rather than inventing a second naming system
//!   ([`CanonicalProjectionState`], acceptance criterion 1);
//! - the surface enforces **stale-target invalidation and destructive grouping** rather than leaving
//!   either to surface-local judgement ([`TargetGuardState`], acceptance criterion 2);
//! - **no claimed action exists only in a contextual affordance** without a matching palette / help /
//!   keyboard route or an explicit, disclosed architectural exception ([`RouteParityState`], acceptance
//!   criterion 3);
//! - and the surface's **command id, label, and blocked-state reason can be reconstructed from durable
//!   support/export evidence** without a screenshot ([`SupportExportParityState`], the support/export
//!   parity implementation requirement).
//!
//! Three records carry the truth:
//!
//! - the per-family **certification row** ([`CommandSurfaceParityRow`]): one row per
//!   [`M5CommandSurfaceFamily`] naming the canonical command binding it projects from, the parity
//!   surfaces it certifies (keyboard, screen-reader, pointer, touch, CLI/help, support/export), the
//!   affordance open modes it certifies fixtures for (pointer-opened, keyboard-opened, compact-layout,
//!   touch/context-action, and policy-blocked), the consumer surfaces it evaluated, its
//!   canonical-projection / target-guard / route-parity / support-export-parity posture, whether the
//!   same command semantics survive headless/CLI execution, any active waiver, and a derived
//!   green/yellow/red [`CommandSurfaceParityStatus`].
//! - the parity **certification packet** ([`CommandSurfaceParityPacket`]): the full set of rows with
//!   derived per-row status, aggregate green/yellow/red counts, the active waivers, the exact
//!   conformance causes ([`CommandSurfaceParityCause`]), and the blocking findings the lane refuses to
//!   ship with.
//! - the **certification dashboard** ([`CommandSurfaceParityDashboard`]): a light projection the command
//!   palette / Support Center / product UI / CLI / help / AI-automation reads to auto-narrow a surface's
//!   discoverability claim when its certification falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment a
//! surface discloses a reduced shortcut hint, discloses a deferred stale-target revalidation, keeps a
//! disclosed, waivered architectural route exception, or discloses a partial support/export capture; it
//! drops to `red` if a surface invents an alternate label or hides its blocked-state reason, leaves a
//! stale target un-invalidated or a destructive item un-separated, hides a claimed action behind a
//! contextual-only route with no palette/help/keyboard equivalent, cannot reconstruct its blocked-state
//! reason from durable evidence, loses the same command semantics in a headless/CLI execution, fails to
//! certify all six cross-modality parity surfaces, fails to certify all five affordance open modes, or
//! fails to certify every consumer surface the matrix declares for the family. That derivation is the
//! auto-narrowing the acceptance criteria require, and the parity-surface, open-mode, and
//! consumer-surface completeness checks are the conformance lints that gate a stable discoverability
//! claim when a surface diverges from the controlled command vocabulary or leaves a claimed modality
//! uncertified.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local
//! paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed vocabulary,
//! counts, refs, and short labels. The surface-family, canonical-command-binding, required-label,
//! stale-target-state, unavailable-reason, parity-surface, consumer-surface, feature-family,
//! downgrade-trigger, and qualification vocabulary is re-exported by reference from the already frozen
//! [matrix], and every family's canonical command binding, qualification, owner, required labels,
//! feature families, parity surfaces, declared consumer surfaces, stale-target states, unavailable
//! reasons, and applicable downgrade triggers are pulled straight from that matrix's seeded packet, so
//! this lane mints no parallel command vocabulary and cannot certify a surface the matrix does not
//! anchor. Only the parity-specific vocabulary ([`M5CommandParityDimension`], [`M5AffordanceOpenMode`],
//! [`CommandSurfaceParityStatus`], [`CanonicalProjectionState`], [`TargetGuardState`],
//! [`RouteParityState`], [`SupportExportParityState`], [`CommandSurfaceParityWaiver`],
//! [`CommandSurfaceParityCause`], [`CommandSurfaceParityFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_menu_keybinding_resolver_and_command_documentation_matrix as matrix;

pub use matrix::{
    M5CanonicalCommandBinding, M5CommandSurfaceFamily, M5DisabledReasonMode,
    M5DiscoverabilityDowngradeTrigger, M5DiscoveryChannel, M5FeatureFamily, M5LifecycleLabel,
    M5ParitySurface, M5PreviewClass, M5RequiredLabel, M5StaleTargetState,
    M5SurfaceQualificationClass, M5UnavailableReason,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_command_surface_parity_packet,
    seeded_m5_command_surface_parity_packet_command_bar_contextual_only_blocked,
    seeded_m5_command_surface_parity_packet_context_menu_stale_target_blocked,
    seeded_m5_command_surface_parity_packet_disabled_explainer_headless_parity_lost_blocked,
    seeded_m5_command_surface_parity_packet_documentation_capture_absent_blocked,
    seeded_m5_command_surface_parity_packet_menu_item_alternate_label_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_COMMAND_SURFACE_PARITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_COMMAND_SURFACE_PARITY_SHARED_CONTRACT_REF: &str =
    "commands:m5_command_surface_parity:v1";

/// Stable record kind for [`CommandSurfaceParityPacket`] payloads.
pub const M5_COMMAND_SURFACE_PARITY_PACKET_RECORD_KIND: &str =
    "commands_m5_command_surface_parity_packet_record";

/// Stable record kind for [`CommandSurfaceParityDashboard`] payloads.
pub const M5_COMMAND_SURFACE_PARITY_DASHBOARD_RECORD_KIND: &str =
    "commands_m5_command_surface_parity_dashboard_record";

/// Stable record kind for [`CommandSurfaceParitySupportExport`] payloads.
pub const M5_COMMAND_SURFACE_PARITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "commands_m5_command_surface_parity_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_COMMAND_SURFACE_PARITY_PACKET_ID: &str = "m5-command-surface-parity:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_COMMAND_SURFACE_PARITY_DASHBOARD_ID: &str =
    "m5-command-surface-parity-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_COMMAND_SURFACE_PARITY_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-command-surface-parity:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_COMMAND_SURFACE_PARITY_SOURCE_SCHEMA_REF: &str =
    "schemas/commands/m5-command-surface-parity.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_COMMAND_SURFACE_PARITY_PUBLISHED_REPORT_REF: &str =
    "artifacts/commands/m5-command-surface-parity.md";

/// Published certification-packet artifact ref.
pub const M5_COMMAND_SURFACE_PARITY_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-command-surface-parity-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_COMMAND_SURFACE_PARITY_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-command-surface-parity-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_COMMAND_SURFACE_PARITY_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-command-surface-parity-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_COMMAND_SURFACE_PARITY_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-command-surface-parity-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_COMMAND_SURFACE_PARITY_PUBLISHED_DOC_REF: &str =
    "docs/commands/m5_command_surface_parity_contract.md";

/// Repo-relative ref to the frozen discoverability boundary schema.
pub const M5_COMMAND_SURFACE_PARITY_MATRIX_SCHEMA_REF: &str = matrix::M5_DISCOVERABILITY_SCHEMA_REF;

/// Frozen discoverability contract doc this proof mirrors.
pub const M5_COMMAND_SURFACE_PARITY_MATRIX_DOC_REF: &str = matrix::M5_DISCOVERABILITY_DOC_REF;

/// Canonical command-descriptor schema every surface projects from.
pub const M5_COMMAND_SURFACE_PARITY_COMMAND_DESCRIPTOR_REF: &str =
    matrix::M5_DISCOVERABILITY_COMMAND_DESCRIPTOR_REF;

/// Every command-surface family the certification must cover, in canonical order. A certification that
/// covers fewer regresses into a partial view and blocks.
pub const REQUIRED_SURFACE_FAMILIES: [M5CommandSurfaceFamily; 10] = M5CommandSurfaceFamily::ALL;

/// Every parity dimension each family row certifies, in canonical order.
pub const REQUIRED_PARITY_DIMENSIONS: [M5CommandParityDimension; 4] = M5CommandParityDimension::ALL;

/// Every cross-modality parity surface each family row must certify, in canonical order.
pub const REQUIRED_PARITY_SURFACES: [M5ParitySurface; 6] = M5ParitySurface::ALL;

/// Every affordance open mode each family row must certify fixtures for, in canonical order.
pub const REQUIRED_OPEN_MODES: [M5AffordanceOpenMode; 5] = M5AffordanceOpenMode::ALL;

/// One of the four parity dimensions each surface-family row certifies.
///
/// These are exactly the four ways the acceptance criteria and implementation requirements demand a
/// claimed M5 command surface keep the same command truth across every reach: it projects the canonical
/// label, shortcut hint, and blocked-state reason; it enforces stale-target invalidation and destructive
/// grouping; it never hides a claimed action behind a contextual-only route; and its command id, label,
/// and blocked-state reason survive support/export capture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CommandParityDimension {
    /// Canonical label, shortcut hint, and blocked-state reason that match the command record.
    CanonicalProjection,
    /// Stale-target invalidation and destructive grouping enforced, not left to surface-local judgement.
    TargetGuard,
    /// No claimed action exists only in a contextual affordance without a matching route.
    RouteParity,
    /// Command id, label, and blocked-state reason reconstructable from durable support/export evidence.
    SupportExportParity,
}

impl M5CommandParityDimension {
    /// Every parity dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CanonicalProjection,
        Self::TargetGuard,
        Self::RouteParity,
        Self::SupportExportParity,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalProjection => "canonical_projection",
            Self::TargetGuard => "target_guard",
            Self::RouteParity => "route_parity",
            Self::SupportExportParity => "support_export_parity",
        }
    }
}

/// One of the five affordance open modes each family row must certify menu/context-menu fixtures for.
///
/// These are the exact cases the implementation requirements name: fixtures that cover a pointer-opened
/// menu, a keyboard-opened menu, a compact-layout presentation, a touch / long-press context action, and
/// a policy-blocked case — each with the same reason strings and shortcuts. A row that certifies fewer
/// leaves a claimed interaction path unproven and blocks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AffordanceOpenMode {
    /// A pointer- (mouse) opened menu / affordance.
    PointerOpened,
    /// A keyboard-opened menu / affordance.
    KeyboardOpened,
    /// A compact-layout presentation of the affordance.
    CompactLayout,
    /// A touch / long-press context action.
    TouchContextAction,
    /// A policy-blocked case (the command is disabled with a typed reason).
    PolicyBlocked,
}

impl M5AffordanceOpenMode {
    /// Every affordance open mode, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::PointerOpened,
        Self::KeyboardOpened,
        Self::CompactLayout,
        Self::TouchContextAction,
        Self::PolicyBlocked,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PointerOpened => "pointer_opened",
            Self::KeyboardOpened => "keyboard_opened",
            Self::CompactLayout => "compact_layout",
            Self::TouchContextAction => "touch_context_action",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// The derived parity certification light a command surface carries.
///
/// `green` means the surface projects the canonical label, shortcut hint, and blocked-state reason,
/// enforces stale-target invalidation and destructive grouping, keeps a palette/help/keyboard route for
/// every claimed action, and can reconstruct its command id, label, and blocked-state reason from
/// durable evidence — across all six cross-modality parity surfaces, all five affordance open modes, and
/// every declared consumer surface, with the same command semantics surviving a headless/CLI execution.
/// `yellow` is a disclosed narrowing (a disclosed reduced shortcut hint, a disclosed deferred
/// stale-target revalidation, a disclosed, waivered architectural route exception, or a disclosed
/// partial support/export capture). `red` is blocked: an invented alternate label or hidden
/// blocked-state reason, a stale target left un-invalidated or a destructive item left un-separated, a
/// contextual-only action with no route, a blocked-state reason absent from capture, a
/// headless/CLI semantics loss, an incomplete parity-surface / open-mode / consumer-surface set — and it
/// may not keep a discoverability claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CommandSurfaceParityStatus {
    /// Full standing: all four parity dimensions hold and headless parity is preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl CommandSurfaceParityStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Green => "green",
            Self::Yellow => "yellow",
            Self::Red => "red",
        }
    }

    /// `true` when the row keeps a publishable (green or yellow) claim.
    pub const fn is_publishable(self) -> bool {
        matches!(self, Self::Green | Self::Yellow)
    }
}

/// How the surface projects the canonical label, shortcut hint, and blocked-state reason.
///
/// `canonical_label_shortcut_reason_certified` means the surface shows the canonical primary label, the
/// resolved shortcut hint, and the typed blocked-state reason that match the command record.
/// `disclosed_reduced_shortcut_hint` means the surface takes a disclosed reduced shortcut hint on a
/// constrained layout — for example folding the resolved source-layer chip into a tooltip while the
/// chord, the canonical label, and the blocked-state reason stay visible (a yellow narrowing).
/// `alternate_label_or_reason_invented` means the surface invented an alternate label for a stable
/// command or hid the blocked-state reason, so the same action no longer reads the same across surfaces —
/// always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CanonicalProjectionState {
    /// Canonical label, shortcut hint, and blocked-state reason are certified.
    CanonicalLabelShortcutReasonCertified,
    /// The surface takes a disclosed reduced shortcut hint on a constrained layout.
    DisclosedReducedShortcutHint,
    /// The surface invented an alternate label or hid the blocked-state reason — a blocker.
    AlternateLabelOrReasonInvented,
}

impl CanonicalProjectionState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalLabelShortcutReasonCertified => {
                "canonical_label_shortcut_reason_certified"
            }
            Self::DisclosedReducedShortcutHint => "disclosed_reduced_shortcut_hint",
            Self::AlternateLabelOrReasonInvented => "alternate_label_or_reason_invented",
        }
    }

    /// `true` when canonical projection is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::CanonicalLabelShortcutReasonCertified)
    }

    /// `true` when the surface took a disclosed reduced-shortcut-hint narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedShortcutHint)
    }
}

/// How the surface enforces stale-target invalidation and destructive grouping.
///
/// `stale_target_and_destructive_grouping_certified` means the surface invalidates items whose target
/// moved, was removed, or lost its context, and keeps destructive items clearly separated from routine
/// ones. `disclosed_deferred_target_revalidation` means the surface takes a disclosed deferred
/// stale-target revalidation on a background-refresh surface — for example marking an item provisional
/// and revalidating it on next open while still keeping destructive grouping (a yellow narrowing).
/// `stale_target_not_invalidated_or_destructive_unseparated` means the surface let a stale target misfire
/// silently or mixed a destructive item into a routine group — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetGuardState {
    /// Stale-target invalidation and destructive grouping are certified.
    StaleTargetAndDestructiveGroupingCertified,
    /// The surface takes a disclosed deferred stale-target revalidation.
    DisclosedDeferredTargetRevalidation,
    /// The surface left a stale target un-invalidated or a destructive item un-separated — a blocker.
    StaleTargetNotInvalidatedOrDestructiveUnseparated,
}

impl TargetGuardState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StaleTargetAndDestructiveGroupingCertified => {
                "stale_target_and_destructive_grouping_certified"
            }
            Self::DisclosedDeferredTargetRevalidation => "disclosed_deferred_target_revalidation",
            Self::StaleTargetNotInvalidatedOrDestructiveUnseparated => {
                "stale_target_not_invalidated_or_destructive_unseparated"
            }
        }
    }

    /// `true` when the target guard is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::StaleTargetAndDestructiveGroupingCertified)
    }

    /// `true` when the surface took a disclosed deferred-revalidation narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedDeferredTargetRevalidation)
    }
}

/// How the surface keeps every claimed action on a palette / help / keyboard route.
///
/// `every_action_has_palette_help_keyboard_route` means every action the surface exposes is also
/// reachable from the command palette, help, and the keyboard — no action is contextual-only.
/// `disclosed_architectural_route_exception` means one action is contextual-only under a disclosed,
/// waivered architectural exception — for example an in-place affordance that only makes sense against a
/// live selection while its behaviour is still documented (a yellow narrowing that widens no authority,
/// so it **requires an active waiver**). `contextual_only_action_without_route` means an action exists
/// only in a contextual affordance with no palette/help/keyboard equivalent and no exception — always a
/// blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RouteParityState {
    /// Every claimed action has a palette / help / keyboard route.
    EveryActionHasPaletteHelpKeyboardRoute,
    /// One action is contextual-only under a disclosed, waivered architectural exception.
    DisclosedArchitecturalRouteException,
    /// An action exists only in a contextual affordance with no route — a blocker.
    ContextualOnlyActionWithoutRoute,
}

impl RouteParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EveryActionHasPaletteHelpKeyboardRoute => {
                "every_action_has_palette_help_keyboard_route"
            }
            Self::DisclosedArchitecturalRouteException => "disclosed_architectural_route_exception",
            Self::ContextualOnlyActionWithoutRoute => "contextual_only_action_without_route",
        }
    }

    /// `true` when route parity is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::EveryActionHasPaletteHelpKeyboardRoute)
    }

    /// `true` when the surface took a disclosed architectural-route-exception narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedArchitecturalRouteException)
    }
}

/// How the surface's command id, label, and blocked-state reason survive support/export capture.
///
/// `command_id_label_reason_reconstructable` means the command id, canonical label, and blocked-state
/// reason can be reconstructed from a durable support export without a screenshot.
/// `disclosed_partial_capture` means one legacy export surface takes a disclosed partial capture — for
/// example a legacy diagnostics export capturing the command id and reason but not the resolved shortcut
/// hint, while still disclosing the gap (a yellow narrowing). `blocked_reason_absent_from_capture` means
/// the blocked-state reason (or command id) is absent from durable evidence, so a support reviewer cannot
/// reconstruct why the command was blocked without a screenshot — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportParityState {
    /// Command id, label, and blocked-state reason are reconstructable from durable evidence.
    CommandIdLabelReasonReconstructable,
    /// One legacy export surface takes a disclosed partial capture.
    DisclosedPartialCapture,
    /// The blocked-state reason or command id is absent from durable evidence — a blocker.
    BlockedReasonAbsentFromCapture,
}

impl SupportExportParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CommandIdLabelReasonReconstructable => "command_id_label_reason_reconstructable",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::BlockedReasonAbsentFromCapture => "blocked_reason_absent_from_capture",
        }
    }

    /// `true` when support/export parity is certified at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::CommandIdLabelReasonReconstructable)
    }

    /// `true` when the surface took a disclosed partial-capture narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed (yellow) rather
/// than blocked — never lets an invented label, an un-invalidated stale target, a hidden route, or an
/// uncapturable blocked-state reason hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSurfaceParityWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The surface family the waiver applies to.
    pub surface_family: M5CommandSurfaceFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl CommandSurfaceParityWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a surface family's parity certification.
///
/// The trigger token mirrors the frozen [`M5DiscoverabilityDowngradeTrigger`] vocabulary so a cause
/// never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSurfaceParityCause {
    /// The surface family the cause applies to.
    pub surface_family: M5CommandSurfaceFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5DiscoverabilityDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause is a
    /// blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl CommandSurfaceParityCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One surface family, certified across its canonical-projection, target-guard, route-parity, and
/// support-export-parity dimensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSurfaceParityRow {
    /// The surface family being certified.
    pub surface_family: M5CommandSurfaceFamily,
    /// Short reviewer-facing family label.
    pub surface_label: String,
    /// Qualification class the matrix earned for the surface. Pulled from the matrix.
    pub qualification: M5SurfaceQualificationClass,
    /// Owner role accountable for keeping this surface's parity governed. Pulled from the matrix.
    pub owner_role: String,
    /// Short conformance scope summary.
    pub scope_summary: String,
    /// The canonical command-record binding this surface projects from. Pulled from the matrix.
    pub canonical_command_binding: M5CanonicalCommandBinding,
    /// Mandatory labels this surface must be able to show. Pulled from the matrix.
    pub required_labels: Vec<M5RequiredLabel>,
    /// The stale-target invalidation states this surface honours. Pulled from the matrix.
    pub stale_target_states: Vec<M5StaleTargetState>,
    /// The why-unavailable explanation classes this surface reports. Pulled from the matrix.
    pub unavailable_reasons: Vec<M5UnavailableReason>,
    /// M5 feature families whose commands this surface exposes. Pulled from the matrix.
    pub feature_families: Vec<M5FeatureFamily>,
    /// Cross-modality parity surfaces the matrix declares the surface must certify (must be all six).
    pub required_parity_surfaces: Vec<M5ParitySurface>,
    /// Cross-modality parity surfaces this certification evaluated. Pulled from the matrix.
    pub certified_parity_surfaces: Vec<M5ParitySurface>,
    /// The affordance open modes this row certifies fixtures for (must be all five).
    pub certified_open_modes: Vec<M5AffordanceOpenMode>,
    /// Consumer surfaces the matrix declares the surface must project to.
    pub required_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Consumer surfaces this certification evaluated. Pulled from the matrix.
    pub evaluated_consumer_surfaces: Vec<M5DiscoveryChannel>,
    /// Canonical-projection posture.
    pub canonical_projection: CanonicalProjectionState,
    /// Target-guard posture.
    pub target_guard: TargetGuardState,
    /// Route-parity posture.
    pub route_parity: RouteParityState,
    /// Support-export-parity posture.
    pub support_export_parity: SupportExportParityState,
    /// `true` when the same command semantics survive a headless / CLI execution; a hard invariant.
    pub headless_parity_preserved: bool,
    /// Downgrade triggers that apply to the surface. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5DiscoverabilityDowngradeTrigger>,
    /// Active waiver, when a disclosed architectural route exception is in force.
    pub active_waiver: Option<CommandSurfaceParityWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: CommandSurfaceParityStatus,
    /// The exact conformance causes that narrowed or blocked this row.
    pub conformance_causes: Vec<CommandSurfaceParityCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl CommandSurfaceParityRow {
    /// `true` when the row certified every consumer surface the matrix declares for the surface — no
    /// declared surface is left uncertified and none is invented.
    pub fn consumer_surfaces_complete(&self) -> bool {
        let mut evaluated: Vec<&str> = self
            .evaluated_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        let mut required: Vec<&str> = self
            .required_consumer_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        evaluated.sort_unstable();
        required.sort_unstable();
        !required.is_empty() && evaluated == required
    }

    /// `true` when the row certifies every one of the six cross-modality parity surfaces — the
    /// structural proof that keyboard, screen-reader, pointer, touch, CLI/help, and support/export can
    /// all explain the same command semantics.
    pub fn parity_surfaces_complete(&self) -> bool {
        let mut certified: Vec<&str> = self
            .certified_parity_surfaces
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_PARITY_SURFACES
            .iter()
            .map(|surface| surface.as_str())
            .collect();
        certified.sort_unstable();
        certified.dedup();
        required.sort_unstable();
        certified == required
    }

    /// `true` when the row certifies fixtures for every one of the five affordance open modes — the
    /// structural proof that pointer-opened, keyboard-opened, compact-layout, touch/context-action, and
    /// policy-blocked cases are all proven with the same reason strings and shortcuts.
    pub fn open_modes_complete(&self) -> bool {
        let mut certified: Vec<&str> = self
            .certified_open_modes
            .iter()
            .map(|mode| mode.as_str())
            .collect();
        let mut required: Vec<&str> = REQUIRED_OPEN_MODES
            .iter()
            .map(|mode| mode.as_str())
            .collect();
        certified.sort_unstable();
        certified.dedup();
        required.sort_unstable();
        certified == required
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.consumer_surfaces_complete() {
            return true;
        }
        if !self.parity_surfaces_complete() {
            return true;
        }
        if !self.open_modes_complete() {
            return true;
        }
        if !self.headless_parity_preserved {
            return true;
        }
        if matches!(
            self.canonical_projection,
            CanonicalProjectionState::AlternateLabelOrReasonInvented
        ) {
            return true;
        }
        if matches!(
            self.target_guard,
            TargetGuardState::StaleTargetNotInvalidatedOrDestructiveUnseparated
        ) {
            return true;
        }
        if matches!(
            self.route_parity,
            RouteParityState::ContextualOnlyActionWithoutRoute
        ) {
            return true;
        }
        if matches!(
            self.support_export_parity,
            SupportExportParityState::BlockedReasonAbsentFromCapture
        ) {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.canonical_projection.is_disclosed_narrowing()
            || self.target_guard.is_disclosed_narrowing()
            || self.route_parity.is_disclosed_narrowing()
            || self.support_export_parity.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the parity posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> CommandSurfaceParityStatus {
        if self.has_hard_blocker() {
            CommandSurfaceParityStatus::Red
        } else if self.has_narrowing() {
            CommandSurfaceParityStatus::Yellow
        } else {
            CommandSurfaceParityStatus::Green
        }
    }

    /// Recomputes the exact conformance causes for the row, in deterministic order (canonical
    /// projection, target guard, route parity, support/export parity, then structural completeness and
    /// headless parity).
    pub fn recompute_causes(&self) -> Vec<CommandSurfaceParityCause> {
        let mut causes = Vec::new();
        match self.canonical_projection {
            CanonicalProjectionState::CanonicalLabelShortcutReasonCertified => {}
            CanonicalProjectionState::DisclosedReducedShortcutHint => {
                causes.push(CommandSurfaceParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::SourceLayerHidden,
                    disclosed: true,
                    detail: "On a constrained layout the surface takes a disclosed reduced shortcut \
                             hint — the resolved source-layer chip is folded into a tooltip while the \
                             chord, the canonical label, and the blocked-state reason stay visible — so \
                             the shortcut truth is narrowed and disclosed rather than hidden."
                        .to_owned(),
                });
            }
            CanonicalProjectionState::AlternateLabelOrReasonInvented => {
                causes.push(CommandSurfaceParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::AlternateLabelInvented,
                    disclosed: false,
                    detail: "The surface invented an alternate label for a stable command or hid the \
                             typed blocked-state reason, so the same action no longer reads with the \
                             same label and reason across menus, palette, help, and keyboard."
                        .to_owned(),
                });
            }
        }
        match self.target_guard {
            TargetGuardState::StaleTargetAndDestructiveGroupingCertified => {}
            TargetGuardState::DisclosedDeferredTargetRevalidation => {
                causes.push(CommandSurfaceParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::StaleTargetNotInvalidated,
                    disclosed: true,
                    detail: "On a background-refresh surface the affordance takes a disclosed deferred \
                             stale-target revalidation — an item whose target may have moved is marked \
                             provisional and revalidated on next open while destructive items stay \
                             clearly grouped — so the guard is narrowed and disclosed rather than \
                             silently misfiring."
                        .to_owned(),
                });
            }
            TargetGuardState::StaleTargetNotInvalidatedOrDestructiveUnseparated => {
                causes.push(CommandSurfaceParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::StaleTargetNotInvalidated,
                    disclosed: false,
                    detail: "The surface let an item whose target moved or was removed misfire silently \
                             instead of invalidating it, or mixed a destructive item into a routine \
                             group, so a context menu was untruthful under a changing target."
                        .to_owned(),
                });
            }
        }
        match self.route_parity {
            RouteParityState::EveryActionHasPaletteHelpKeyboardRoute => {}
            RouteParityState::DisclosedArchitecturalRouteException => {
                causes.push(CommandSurfaceParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                    disclosed: true,
                    detail: "One action is contextual-only under a disclosed, waivered architectural \
                             exception — the in-place affordance only makes sense against a live \
                             selection and its behaviour is still documented in help — so the route \
                             parity is narrowed and disclosed rather than a hidden-only route that \
                             widens no authority."
                        .to_owned(),
                });
            }
            RouteParityState::ContextualOnlyActionWithoutRoute => {
                causes.push(CommandSurfaceParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                    disclosed: false,
                    detail: "A claimed action exists only in a contextual affordance with no matching \
                             palette, help, or keyboard route and no disclosed architectural exception, \
                             so the action cannot be discovered or invoked from any other surface."
                        .to_owned(),
                });
            }
        }
        match self.support_export_parity {
            SupportExportParityState::CommandIdLabelReasonReconstructable => {}
            SupportExportParityState::DisclosedPartialCapture => {
                causes.push(CommandSurfaceParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: true,
                    detail: "One legacy export surface takes a disclosed partial capture — a legacy \
                             diagnostics export captures the command id and blocked-state reason but not \
                             the resolved shortcut hint, while still disclosing the gap — so the \
                             support/export parity is narrowed and disclosed rather than absent."
                        .to_owned(),
                });
            }
            SupportExportParityState::BlockedReasonAbsentFromCapture => {
                causes.push(CommandSurfaceParityCause {
                    surface_family: self.surface_family,
                    trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                    disclosed: false,
                    detail:
                        "The command id, canonical label, or typed blocked-state reason is absent \
                             from durable support/export evidence, so a support reviewer cannot \
                             reconstruct what the command was or why it was blocked without a \
                             screenshot."
                            .to_owned(),
                });
            }
        }
        if !self.parity_surfaces_complete() {
            causes.push(CommandSurfaceParityCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                disclosed: false,
                detail: "The surface does not certify all six cross-modality parity surfaces — keyboard, \
                         screen-reader, pointer, touch, CLI/help, and support/export — so a claimed \
                         modality cannot explain the same command semantics."
                    .to_owned(),
            });
        }
        if !self.open_modes_complete() {
            causes.push(CommandSurfaceParityCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ProofStale,
                disclosed: false,
                detail: "The surface does not certify fixtures for all five affordance open modes — \
                         pointer-opened, keyboard-opened, compact-layout, touch/context-action, and \
                         policy-blocked — so a claimed interaction path is left unproven."
                    .to_owned(),
            });
        }
        if !self.headless_parity_preserved {
            causes.push(CommandSurfaceParityCause {
                surface_family: self.surface_family,
                trigger: M5DiscoverabilityDowngradeTrigger::ParitySurfaceDropped,
                disclosed: false,
                detail:
                    "A headless / CLI execution of this surface lost the shared command semantics, \
                         so the same command reports a different label, shortcut, or blocked-state \
                         reason depending on how it is reached."
                        .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed architectural route exception may only stay yellow (rather than red) when a waiver
    /// discloses it — making an action contextual-only is the sensitive narrowing.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.route_parity,
            RouteParityState::DisclosedArchitecturalRouteException
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<CommandSurfaceParityFinding> {
        let mut findings = Vec::new();
        let family = self.surface_family.as_str().to_owned();

        if !self.consumer_surfaces_complete() {
            findings.push(CommandSurfaceParityFinding::ConsumerSurfacesIncomplete {
                family: family.clone(),
            });
        }
        if !self.parity_surfaces_complete() {
            findings.push(CommandSurfaceParityFinding::ParitySurfacesIncomplete {
                family: family.clone(),
            });
        }
        if !self.open_modes_complete() {
            findings.push(CommandSurfaceParityFinding::OpenModesIncomplete {
                family: family.clone(),
            });
        }
        if !self.headless_parity_preserved {
            findings.push(CommandSurfaceParityFinding::HeadlessParityLost {
                family: family.clone(),
            });
        }
        if matches!(
            self.canonical_projection,
            CanonicalProjectionState::AlternateLabelOrReasonInvented
        ) {
            findings.push(CommandSurfaceParityFinding::CanonicalProjectionBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.target_guard,
            TargetGuardState::StaleTargetNotInvalidatedOrDestructiveUnseparated
        ) {
            findings.push(CommandSurfaceParityFinding::TargetGuardBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.route_parity,
            RouteParityState::ContextualOnlyActionWithoutRoute
        ) {
            findings.push(CommandSurfaceParityFinding::RouteParityBroken {
                family: family.clone(),
            });
        }
        if matches!(
            self.support_export_parity,
            SupportExportParityState::BlockedReasonAbsentFromCapture
        ) {
            findings.push(CommandSurfaceParityFinding::SupportExportParityBroken {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, CommandSurfaceParityStatus::Green) && !self.has_reason() {
            findings.push(CommandSurfaceParityFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(CommandSurfaceParityFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.surface_family != self.surface_family {
                findings.push(CommandSurfaceParityFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(CommandSurfaceParityFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(CommandSurfaceParityFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.conformance_causes != self.recompute_causes() {
            findings.push(CommandSurfaceParityFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} canonical={} target={} route={} export={} headless={} parity={} modes={} surfaces={} waiver={}",
            self.surface_family.as_str(),
            self.derived_status.as_str(),
            self.canonical_projection.as_str(),
            self.target_guard.as_str(),
            self.route_parity.as_str(),
            self.support_export_parity.as_str(),
            self.headless_parity_preserved,
            self.certified_parity_surfaces.len(),
            self.certified_open_modes.len(),
            self.evaluated_consumer_surfaces.len(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the parity certification refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum CommandSurfaceParityFinding {
    /// A surface family has no certification row.
    SurfaceFamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row did not certify every declared consumer surface.
    ConsumerSurfacesIncomplete {
        /// The family token.
        family: String,
    },
    /// A row does not certify all six cross-modality parity surfaces.
    ParitySurfacesIncomplete {
        /// The family token.
        family: String,
    },
    /// A row does not certify fixtures for all five affordance open modes.
    OpenModesIncomplete {
        /// The family token.
        family: String,
    },
    /// A headless / CLI execution lost the shared command semantics.
    HeadlessParityLost {
        /// The family token.
        family: String,
    },
    /// The surface invented an alternate label or hid its blocked-state reason.
    CanonicalProjectionBroken {
        /// The family token.
        family: String,
    },
    /// The surface left a stale target un-invalidated or a destructive item un-separated.
    TargetGuardBroken {
        /// The family token.
        family: String,
    },
    /// The surface hid a claimed action behind a contextual-only route.
    RouteParityBroken {
        /// The family token.
        family: String,
    },
    /// The surface's blocked-state reason is absent from durable evidence.
    SupportExportParityBroken {
        /// The family token.
        family: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The family token.
        family: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The family token.
        family: String,
    },
    /// An attached waiver does not point at the row's family.
    WaiverFamilyMismatch {
        /// The family token.
        family: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The family token.
        family: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The family token.
        family: String,
    },
    /// The declared conformance causes do not match the recomputed causes.
    RowCausesStale {
        /// The family token.
        family: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl CommandSurfaceParityFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::SurfaceFamilyMissing { .. } => "surface_family_missing",
            Self::ConsumerSurfacesIncomplete { .. } => "consumer_surfaces_incomplete",
            Self::ParitySurfacesIncomplete { .. } => "parity_surfaces_incomplete",
            Self::OpenModesIncomplete { .. } => "open_modes_incomplete",
            Self::HeadlessParityLost { .. } => "headless_parity_lost",
            Self::CanonicalProjectionBroken { .. } => "canonical_projection_broken",
            Self::TargetGuardBroken { .. } => "target_guard_broken",
            Self::RouteParityBroken { .. } => "route_parity_broken",
            Self::SupportExportParityBroken { .. } => "support_export_parity_broken",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverFamilyMismatch { .. } => "waiver_family_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::SurfaceFamilyMissing { family }
            | Self::ConsumerSurfacesIncomplete { family }
            | Self::ParitySurfacesIncomplete { family }
            | Self::OpenModesIncomplete { family }
            | Self::HeadlessParityLost { family }
            | Self::CanonicalProjectionBroken { family }
            | Self::TargetGuardBroken { family }
            | Self::RouteParityBroken { family }
            | Self::SupportExportParityBroken { family }
            | Self::NarrowedRowWithoutReason { family }
            | Self::NarrowedRowWithoutWaiver { family }
            | Self::WaiverFamilyMismatch { family, .. }
            | Self::WaiverExpired { family, .. }
            | Self::RowStatusStale { family }
            | Self::RowCausesStale { family } => family,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The parity certification packet shared by the command palette / Support Center / product UI / CLI /
/// help / AI-automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSurfaceParityPacket {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the packet.
    pub schema_version: u32,
    /// Shared contract ref consumed by every consumer.
    pub shared_contract_ref: String,
    /// Stable packet id used to pivot across surfaces.
    pub packet_id: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Reviewer-facing summary line printed above the rows.
    pub headline: String,
    /// The frozen discoverability matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen discoverability boundary schema.
    pub matrix_schema_ref: String,
    /// Frozen discoverability contract doc this proof mirrors.
    pub matrix_doc_ref: String,
    /// Canonical command-descriptor schema every surface projects from.
    pub command_descriptor_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The four parity dimensions every family row certifies.
    pub required_parity_dimensions: Vec<String>,
    /// The six cross-modality parity surfaces every family row must certify.
    pub required_parity_surfaces: Vec<String>,
    /// The five affordance open modes every family row must certify.
    pub required_open_modes: Vec<String>,
    /// The ten surface families the certification must cover.
    pub required_surface_families: Vec<String>,
    /// Per-family certification rows, in canonical order.
    pub rows: Vec<CommandSurfaceParityRow>,
    /// Surface families certified, in canonical (sorted) order.
    pub covered_surface_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (full-conformance) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked — the stable-claim gate.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<CommandSurfaceParityWaiver>,
    /// Every exact conformance cause, in row then cause order.
    pub conformance_causes: Vec<CommandSurfaceParityCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<CommandSurfaceParityFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Command / discoverability automation refs that consume this packet to auto-narrow a surface.
    pub command_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published certification-packet ref.
    pub published_packet_ref: String,
    /// Published certification-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl CommandSurfaceParityPacket {
    /// Returns the certification row for `family`, if present.
    pub fn row(&self, family: M5CommandSurfaceFamily) -> Option<&CommandSurfaceParityRow> {
        self.rows.iter().find(|row| row.surface_family == family)
    }

    /// Returns compact text lines for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = vec![
            format!(
                "packet: id={}, rows={}, green={}, yellow={}, red={}, clean={}",
                self.packet_id,
                self.row_count,
                self.green_row_count,
                self.yellow_row_count,
                self.red_row_count,
                self.report_clean,
            ),
            format!(
                "matrix={} build={} channel={} publishable={}",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.all_rows_publishable,
            ),
        ];
        for row in &self.rows {
            lines.push(row.compact_line());
        }
        for waiver in &self.active_waivers {
            lines.push(format!(
                "  waiver {} -> {} (expires {})",
                waiver.waiver_id,
                waiver.surface_family.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.conformance_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.surface_family.as_str(),
                cause.cause_token(),
                cause.disclosed
            ));
        }
        for finding in &self.blocking_findings {
            lines.push(format!(
                "  blocker: {} -- {}",
                finding.class_token(),
                finding.subject_ref()
            ));
        }
        lines
    }

    /// Projects the light certification dashboard the command automation consumes.
    pub fn dashboard(&self) -> CommandSurfaceParityDashboard {
        CommandSurfaceParityDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 command-surface-parity packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per surface family naming its status,
    /// the four parity postures, headless parity, the parity-surface / open-mode counts, the
    /// evaluated-surface count, and the waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "surface_family,status,canonical_projection,target_guard,route_parity,support_export_parity,headless_parity,parity_surfaces,open_modes,evaluated_surfaces,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.surface_family.as_str(),
                row.derived_status.as_str(),
                row.canonical_projection.as_str(),
                row.target_guard.as_str(),
                row.route_parity.as_str(),
                row.support_export_parity.as_str(),
                row.headless_parity_preserved,
                row.certified_parity_surfaces.len(),
                row.certified_open_modes.len(),
                row.evaluated_consumer_surfaces.len(),
                row.active_waiver
                    .as_ref()
                    .map(|w| w.waiver_id.as_str())
                    .unwrap_or("none"),
            ));
        }
        out
    }

    /// Renders the markdown report for the lane.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "# M5 command-surface parity: menu, context-menu, and command-bar parity with canonical labels, shortcuts, and blocked-state reasons across every claimed M5 surface\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_command_surface_parity`](../../crates/aureline-shell/src/m5_command_surface_parity/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_command_surface_parity -- markdown > \\\n  artifacts/commands/m5-command-surface-parity.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!(
            "- Release channel: `{}`\n",
            self.release_channel_class
        ));
        out.push_str(&format!(
            "- Required parity dimensions: {}\n",
            self.required_parity_dimensions
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Affordance open modes certified: {}\n",
            self.required_open_modes
                .iter()
                .map(|t| format!("`{t}`"))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        out.push_str(&format!(
            "- Surface families certified: {}\n",
            self.row_count
        ));
        out.push_str(&format!(
            "- Green (full conformance): {}\n",
            self.green_row_count
        ));
        out.push_str(&format!(
            "- Yellow (auto-narrowed): {}\n",
            self.yellow_row_count
        ));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable (stable-claim gate): `{}`\n",
            self.all_rows_publishable
        ));
        out.push_str(&format!(
            "- Blocking findings: {}\n",
            self.blocking_findings.len()
        ));
        out.push_str(&format!(
            "- Status: **{}**\n",
            if self.report_clean {
                "clean"
            } else {
                "blocked"
            }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Certification rows\n\n");
        out.push_str(
            "| Surface family | Status | Canonical projection | Target guard | Route parity | Support/export | Headless | Waiver |\n\
             | -------------- | ------ | -------------------- | ------------ | ------------ | -------------- | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.surface_label,
                row.derived_status.as_str(),
                row.canonical_projection.as_str(),
                row.target_guard.as_str(),
                row.route_parity.as_str(),
                row.support_export_parity.as_str(),
                row.headless_parity_preserved,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&CommandSurfaceParityRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, CommandSurfaceParityStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every claimed M5 command surface projects the canonical label, shortcut hint, and blocked-state reason, enforces stale-target invalidation and destructive grouping, keeps a palette/help/keyboard route for every action, and can reconstruct its command id, label, and blocked-state reason from durable evidence across every modality and open mode.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.surface_family.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact conformance causes\n\n");
        if self.conformance_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.conformance_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.surface_family.as_str(),
                    cause.cause_token(),
                    cause.disclosed,
                    cause.detail,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Active waivers\n\n");
        if self.active_waivers.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for waiver in &self.active_waivers {
                out.push_str(&format!(
                    "- `{}` (`{}`, owner: {}, expires `{}`) — {}\n",
                    waiver.waiver_id,
                    waiver.surface_family.as_str(),
                    waiver.owner_role,
                    waiver.expires_at,
                    waiver.reason,
                ));
            }
            out.push('\n');
        }

        out.push_str("## Findings\n\n");
        if self.blocking_findings.is_empty() {
            out.push_str("Findings: none.\n\n");
        } else {
            for finding in &self.blocking_findings {
                out.push_str(&format!(
                    "- `{}` — `{}`\n",
                    finding.class_token(),
                    finding.subject_ref()
                ));
            }
            out.push('\n');
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_command_surface_parity -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_command_surface_parity_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSurfaceParityDashboardRow {
    /// The surface family.
    pub surface_family: M5CommandSurfaceFamily,
    /// Short family label.
    pub surface_label: String,
    /// Qualification class earned by the surface.
    pub qualification: M5SurfaceQualificationClass,
    /// Derived green/yellow/red status.
    pub status: CommandSurfaceParityStatus,
    /// Number of cross-modality parity surfaces certified.
    pub certified_parity_surface_count: usize,
    /// Number of affordance open modes certified.
    pub certified_open_mode_count: usize,
    /// Number of declared consumer surfaces certified for this family.
    pub evaluated_surface_count: usize,
    /// Canonical-projection posture.
    pub canonical_projection: CanonicalProjectionState,
    /// Target-guard posture.
    pub target_guard: TargetGuardState,
    /// Route-parity posture.
    pub route_parity: RouteParityState,
    /// Support-export-parity posture.
    pub support_export_parity: SupportExportParityState,
    /// `true` when headless / CLI parity is preserved.
    pub headless_parity_preserved: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the command palette / Support Center / product UI / CLI / help /
/// AI-automation reads to auto-narrow a surface's discoverability claim.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSurfaceParityDashboard {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the dashboard.
    pub schema_version: u32,
    /// Stable dashboard id.
    pub dashboard_id: String,
    /// The packet id this dashboard projects.
    pub source_packet_ref: String,
    /// Repo-relative ref to the boundary schema.
    pub source_schema_ref: String,
    /// Dashboard rows, in canonical order.
    pub rows: Vec<CommandSurfaceParityDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Command / discoverability automation refs that consume the dashboard.
    pub command_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl CommandSurfaceParityDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &CommandSurfaceParityPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| CommandSurfaceParityDashboardRow {
                surface_family: row.surface_family,
                surface_label: row.surface_label.clone(),
                qualification: row.qualification,
                status: row.derived_status,
                certified_parity_surface_count: row.certified_parity_surfaces.len(),
                certified_open_mode_count: row.certified_open_modes.len(),
                evaluated_surface_count: row.evaluated_consumer_surfaces.len(),
                canonical_projection: row.canonical_projection,
                target_guard: row.target_guard,
                route_parity: row.route_parity,
                support_export_parity: row.support_export_parity,
                headless_parity_preserved: row.headless_parity_preserved,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .conformance_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_COMMAND_SURFACE_PARITY_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_COMMAND_SURFACE_PARITY_SCHEMA_VERSION,
            dashboard_id: M5_COMMAND_SURFACE_PARITY_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            command_automation_refs: packet.command_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 command-surface-parity dashboard serializes")
    }
}

/// Support-export wrapper for the parity certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CommandSurfaceParitySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: CommandSurfaceParityPacket,
    /// Dashboard quoted in full.
    pub dashboard: CommandSurfaceParityDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl CommandSurfaceParitySupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each surface family, and each active
    /// waiver id is quoted as a case id so a support reviewer — or the command automation — can name the
    /// same surface and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: CommandSurfaceParityPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.surface_family.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_COMMAND_SURFACE_PARITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_COMMAND_SURFACE_PARITY_SCHEMA_VERSION,
            shared_contract_ref: M5_COMMAND_SURFACE_PARITY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_command_surface_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSurfaceParityInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen discoverability matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family certification rows.
    pub rows: Vec<CommandSurfaceParityRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The certification packet carries only closed vocabulary, refs, and short labels, so raw URLs,
/// credentials, or tokens must never appear.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("bearer ")
                || lower.contains("://")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}

/// Builds a [`CommandSurfaceParityPacket`] from the exact build identity, the frozen matrix ref, and the
/// per-family certification rows.
///
/// Each row's derived status and conformance causes, the aggregate counts, the active waivers, and the
/// blocking findings are recomputed here so the packet is the single source of truth and the
/// auto-narrowing cannot be asserted.
pub fn build_m5_command_surface_parity_packet(
    input: CommandSurfaceParityInput,
) -> CommandSurfaceParityPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<CommandSurfaceParityRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.conformance_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<CommandSurfaceParityFinding> = Vec::new();

    // Every surface family must carry a certification row.
    let present: BTreeSet<M5CommandSurfaceFamily> =
        rows.iter().map(|row| row.surface_family).collect();
    for family in REQUIRED_SURFACE_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(CommandSurfaceParityFinding::SurfaceFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_surface_families: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, CommandSurfaceParityStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, CommandSurfaceParityStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, CommandSurfaceParityStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(CommandSurfaceParityFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<CommandSurfaceParityWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let conformance_causes: Vec<CommandSurfaceParityCause> = rows
        .iter()
        .flat_map(|row| row.conformance_causes.clone())
        .collect();

    let required_parity_dimensions: Vec<String> = REQUIRED_PARITY_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    let required_parity_surfaces: Vec<String> = REQUIRED_PARITY_SURFACES
        .iter()
        .map(|surface| surface.as_str().to_owned())
        .collect();
    let required_open_modes: Vec<String> = REQUIRED_OPEN_MODES
        .iter()
        .map(|mode| mode.as_str().to_owned())
        .collect();
    let required_surface_families: Vec<String> = REQUIRED_SURFACE_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();

    let mut packet = CommandSurfaceParityPacket {
        record_kind: M5_COMMAND_SURFACE_PARITY_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_COMMAND_SURFACE_PARITY_SCHEMA_VERSION,
        shared_contract_ref: M5_COMMAND_SURFACE_PARITY_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_COMMAND_SURFACE_PARITY_PACKET_ID.to_owned(),
        source_schema_ref: M5_COMMAND_SURFACE_PARITY_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Parity certification for every claimed M5 command surface: each of the ten governed \
                   surface families certified so the same action keeps the same canonical label, the \
                   same shortcut hint, the same blocked-state reason, and the same authority posture \
                   whether it is reached from a menu, a context menu, a command bar, a keybinding sheet, \
                   a help page, a leader overlay, a palette row, or a contextual affordance — across all \
                   six cross-modality parity surfaces and all five affordance open modes, with the same \
                   command semantics preserved in headless/CLI execution, each surface's green/yellow/red \
                   claim auto-narrowed from its four parity postures, and any surface that still invents \
                   an alternate label, drops a stale-target guard, hides an action behind a \
                   contextual-only route, or cannot reconstruct its blocked-state reason from durable \
                   evidence blocked from a stable claim."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_COMMAND_SURFACE_PARITY_MATRIX_SCHEMA_REF.to_owned(),
        matrix_doc_ref: M5_COMMAND_SURFACE_PARITY_MATRIX_DOC_REF.to_owned(),
        command_descriptor_ref: M5_COMMAND_SURFACE_PARITY_COMMAND_DESCRIPTOR_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_parity_dimensions,
        required_parity_surfaces,
        required_open_modes,
        required_surface_families,
        rows,
        covered_surface_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        conformance_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        command_automation_refs: vec![
            "command_status.surface_parity_registry".to_owned(),
            "discoverability_automation.auto_narrow.surface_parity_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.command_surface_parity".to_owned(),
            M5_COMMAND_SURFACE_PARITY_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_COMMAND_SURFACE_PARITY_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-command-surface-parity".to_owned()],
        published_report_ref: M5_COMMAND_SURFACE_PARITY_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_COMMAND_SURFACE_PARITY_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_COMMAND_SURFACE_PARITY_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_COMMAND_SURFACE_PARITY_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(CommandSurfaceParityFinding::RawBoundaryMaterialInExport);
    }

    blocking_findings.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    packet.report_clean = blocking_findings.is_empty();
    packet.blocking_findings = blocking_findings;

    packet
}

/// Validation error produced by [`validate_m5_command_surface_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum CommandSurfaceParityValidationError {
    /// The packet has no rows.
    NoRows,
    /// The packet's record kind is wrong.
    WrongRecordKind,
    /// The packet's schema version is wrong.
    WrongSchemaVersion,
    /// The packet's exact-build identity ref is empty.
    BuildIdentityRefMissing,
    /// The packet does not certify a frozen matrix packet.
    MatrixPacketRefMissing,
    /// The declared required parity dimensions do not match the lane constants.
    RequiredParityDimensionsStale,
    /// The declared required parity surfaces do not match the lane constants.
    RequiredParitySurfacesStale,
    /// The declared required open modes do not match the lane constants.
    RequiredOpenModesStale,
    /// The declared required surface families do not match the lane constants.
    RequiredSurfaceFamiliesStale,
    /// The rows do not cover all ten surface families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared conformance causes do not match the recomputed causes.
    ConformanceCausesStale,
    /// The declared blocking findings do not match the recomputed findings.
    BlockingFindingsStale,
    /// A blocking finding remains in the packet.
    BlockingFindingPresent {
        /// Finding class.
        class: String,
        /// Owning subject ref.
        subject_ref: String,
    },
    /// The published report ref is empty.
    PublishedReportRefMissing,
    /// The published packet ref is empty.
    PublishedPacketRefMissing,
    /// The published dashboard ref is empty.
    PublishedDashboardRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
}

/// Validates a packet against the parity certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every surface family carries a current
/// certification row; each row's status is the derived auto-narrowed value, never asserted; a green row
/// cannot keep a claim while it invents an alternate label or hides its blocked-state reason, leaves a
/// stale target un-invalidated or a destructive item un-separated, hides a claimed action behind a
/// contextual-only route, cannot reconstruct its blocked-state reason from durable evidence, loses
/// headless/CLI parity, fails to certify all six cross-modality parity surfaces, fails to certify all
/// five affordance open modes, or fails to certify every declared consumer surface; and a disclosed
/// narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_command_surface_parity_packet(
    packet: &CommandSurfaceParityPacket,
) -> Result<(), Vec<CommandSurfaceParityValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(CommandSurfaceParityValidationError::NoRows);
    }
    if packet.record_kind != M5_COMMAND_SURFACE_PARITY_PACKET_RECORD_KIND {
        errors.push(CommandSurfaceParityValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_COMMAND_SURFACE_PARITY_SCHEMA_VERSION {
        errors.push(CommandSurfaceParityValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(CommandSurfaceParityValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(CommandSurfaceParityValidationError::MatrixPacketRefMissing);
    }
    let expected_dimensions: Vec<String> = REQUIRED_PARITY_DIMENSIONS
        .iter()
        .map(|dimension| dimension.as_str().to_owned())
        .collect();
    if packet.required_parity_dimensions != expected_dimensions {
        errors.push(CommandSurfaceParityValidationError::RequiredParityDimensionsStale);
    }
    let expected_parity_surfaces: Vec<String> = REQUIRED_PARITY_SURFACES
        .iter()
        .map(|surface| surface.as_str().to_owned())
        .collect();
    if packet.required_parity_surfaces != expected_parity_surfaces {
        errors.push(CommandSurfaceParityValidationError::RequiredParitySurfacesStale);
    }
    let expected_open_modes: Vec<String> = REQUIRED_OPEN_MODES
        .iter()
        .map(|mode| mode.as_str().to_owned())
        .collect();
    if packet.required_open_modes != expected_open_modes {
        errors.push(CommandSurfaceParityValidationError::RequiredOpenModesStale);
    }
    let expected_families: Vec<String> = REQUIRED_SURFACE_FAMILIES
        .iter()
        .map(|family| family.as_str().to_owned())
        .collect();
    if packet.required_surface_families != expected_families {
        errors.push(CommandSurfaceParityValidationError::RequiredSurfaceFamiliesStale);
    }

    let present: BTreeSet<M5CommandSurfaceFamily> =
        packet.rows.iter().map(|row| row.surface_family).collect();
    let coverage_complete = REQUIRED_SURFACE_FAMILIES
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_SURFACE_FAMILIES.len() {
        errors.push(CommandSurfaceParityValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_surface_families {
        errors.push(CommandSurfaceParityValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), CommandSurfaceParityStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), CommandSurfaceParityStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), CommandSurfaceParityStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(CommandSurfaceParityValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<CommandSurfaceParityWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(CommandSurfaceParityValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<CommandSurfaceParityCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.conformance_causes {
        errors.push(CommandSurfaceParityValidationError::ConformanceCausesStale);
    }

    let mut recomputed: Vec<CommandSurfaceParityFinding> = Vec::new();
    for family in REQUIRED_SURFACE_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(CommandSurfaceParityFinding::SurfaceFamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(CommandSurfaceParityFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(CommandSurfaceParityFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(CommandSurfaceParityValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(
            CommandSurfaceParityValidationError::BlockingFindingPresent {
                class: finding.class_token().to_owned(),
                subject_ref: finding.subject_ref().to_owned(),
            },
        );
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(CommandSurfaceParityValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(CommandSurfaceParityValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(CommandSurfaceParityValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(CommandSurfaceParityValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
