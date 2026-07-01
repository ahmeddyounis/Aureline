//! Canonical compact / standard / expanded responsive-collapse certification for
//! every claimed M5 surface family.
//!
//! The [frozen shell-zone matrix][matrix] already binds each claimed M5 surface
//! family — notebook, data grid, profiler, pipeline, docs, preview, review,
//! incident, companion, and operator — to the responsive classes it must survive
//! and the ordered collapse ladder (docked → sheet → overflow → placeholder) it
//! falls through as width narrows. This lane is the responsive-collapse capstone on
//! top of that matrix: for every governed family it certifies that its
//! compact / standard / expanded presentation stays **identity-stable**, that the
//! docked-to-sheet transition preserves the same object identity and task state,
//! that no essential action becomes hover-only or route-breaking as width narrows,
//! and that 400% zoom and high-contrast layouts keep the same route semantics and
//! task state.
//!
//! Three records carry the truth:
//!
//! - the per-family **collapse row** ([`ResponsiveCollapseRow`]): one row per
//!   [`M5ShellSurfaceFamily`] naming the responsive classes it declares, the ordered
//!   collapse ladder pulled from the matrix, its per-class presentation
//!   ([`ResponsiveClassPresentation`]) with the placement it lands in and whether
//!   identity and essential actions survive there, its collapse-ladder /
//!   identity-continuity / critical-action-reach / zoom-contrast-parity posture, any
//!   active waiver, and a derived green/yellow/red [`ResponsiveCollapseStatus`].
//! - the release **collapse packet** ([`ResponsiveCollapsePacket`]): the full set of
//!   rows with derived per-row status, aggregate green/yellow/red counts, the active
//!   waivers, the exact collapse causes ([`ResponsiveCollapseCause`]), and the
//!   blocking findings the lane refuses to ship with.
//! - the **collapse dashboard** ([`ResponsiveCollapseDashboard`]): a light projection
//!   the shell / windowing / layout / release automation reads to auto-narrow a
//!   claimed surface when its responsive proof falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to
//! `yellow` the moment its frozen qualification is below Stable, its collapse ladder
//! takes a disclosed narrowing step, its docked-to-sheet transition rehydrates task
//! state through a disclosed, waivered path, a low-frequency action moves to a
//! disclosed keyboard-reachable overflow, or its zoom/contrast layout is disclosed as
//! narrowed; it drops to `red` if responsive collapse changes the task identity, the
//! ladder does not terminate in an identity-preserving placeholder, the
//! docked-to-sheet transition loses identity or task state, critical state is hidden
//! instead of overflowed, an essential action becomes hover-only or route-breaking,
//! zoom/high-contrast diverges the route semantics, or a per-class presentation lands
//! in a placement the family never declared. That derivation is the auto-narrowing
//! the acceptance criteria require, and the presentation/ladder checks are the lint
//! that prevents a later collapse regression from shipping as stable.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw
//! URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials —
//! only stable ids, closed vocabulary, counts, refs, and short labels. The surface
//! family, responsive-class, collapse-placement, qualification, downgrade-trigger,
//! and consumer-surface vocabulary is re-exported by reference from the already
//! frozen [matrix]; the certified rows are pulled straight from that matrix's seeded
//! packet, so this lane mints no parallel shell vocabulary and cannot certify a
//! family the matrix does not freeze. Only the responsive-collapse-specific
//! vocabulary ([`ResponsiveCollapseStatus`], [`CollapseLadderState`],
//! [`IdentityContinuityState`], [`CriticalActionReachState`],
//! [`ZoomContrastParityState`], [`ResponsiveClassPresentation`],
//! [`ResponsiveCollapseWaiver`], [`ResponsiveCollapseCause`],
//! [`ResponsiveCollapseFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix as matrix;

pub use matrix::{
    M5FallbackPlacement, M5ResponsiveClass, M5ShellConsumerSurface, M5ShellDowngradeTrigger,
    M5ShellQualificationClass, M5ShellSurfaceFamily, M5ShellZoneSlot,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_responsive_collapse_packet,
    seeded_m5_responsive_collapse_packet_companion_ladder_missing_placeholder_blocked,
    seeded_m5_responsive_collapse_packet_docs_zoom_route_divergence_blocked,
    seeded_m5_responsive_collapse_packet_notebook_collapse_identity_blocked,
    seeded_m5_responsive_collapse_packet_profiler_critical_state_hidden_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_RESPONSIVE_COLLAPSE_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_RESPONSIVE_COLLAPSE_SHARED_CONTRACT_REF: &str = "shell:m5_responsive_collapse:v1";

/// Stable record kind for [`ResponsiveCollapsePacket`] payloads.
pub const M5_RESPONSIVE_COLLAPSE_PACKET_RECORD_KIND: &str =
    "shell_m5_responsive_collapse_packet_record";

/// Stable record kind for [`ResponsiveCollapseDashboard`] payloads.
pub const M5_RESPONSIVE_COLLAPSE_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_responsive_collapse_dashboard_record";

/// Stable record kind for [`ResponsiveCollapseSupportExport`] payloads.
pub const M5_RESPONSIVE_COLLAPSE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_responsive_collapse_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_RESPONSIVE_COLLAPSE_PACKET_ID: &str = "m5-responsive-collapse:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_RESPONSIVE_COLLAPSE_DASHBOARD_ID: &str =
    "m5-responsive-collapse-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_RESPONSIVE_COLLAPSE_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-responsive-collapse:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_RESPONSIVE_COLLAPSE_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-responsive-collapse.schema.json";

/// Published markdown report ref reviewers reopen the responsive proof from.
pub const M5_RESPONSIVE_COLLAPSE_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-responsive-collapse.md";

/// Published collapse-packet artifact ref.
pub const M5_RESPONSIVE_COLLAPSE_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-responsive-collapse-proof/packet.json";

/// Published collapse-dashboard artifact ref.
pub const M5_RESPONSIVE_COLLAPSE_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-responsive-collapse-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_RESPONSIVE_COLLAPSE_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-responsive-collapse-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_RESPONSIVE_COLLAPSE_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-responsive-collapse-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_RESPONSIVE_COLLAPSE_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_responsive_collapse_contract.md";

/// Repo-relative ref to the frozen shell-zone matrix schema.
pub const M5_RESPONSIVE_COLLAPSE_MATRIX_SCHEMA_REF: &str = matrix::M5_SHELL_ZONE_MATRIX_SCHEMA_REF;

/// Repo-relative ref to the frozen responsive-class schema.
pub const M5_RESPONSIVE_COLLAPSE_RESPONSIVE_CLASS_SCHEMA_REF: &str =
    matrix::M5_SHELL_RESPONSIVE_CLASS_SCHEMA_REF;

/// Every governed surface family the responsive proof must cover, in canonical
/// order. These are exactly the families the frozen shell-zone matrix freezes; the
/// lane certifies none beyond them and refuses to ship if any is missing.
pub const REQUIRED_FAMILIES: [M5ShellSurfaceFamily; 10] = M5ShellSurfaceFamily::ALL;

/// The derived responsive-collapse light a governed surface family carries.
///
/// `green` means the family stays identity-stable across compact/standard/expanded,
/// its docked-to-sheet transition preserves identity and task state, no essential
/// action becomes hover-only or route-breaking, and zoom/high-contrast keeps the same
/// route semantics. `yellow` is a disclosed narrowing (the family is honestly
/// narrowed below Stable, takes a disclosed collapse-ladder step, rehydrates state
/// through a disclosed waivered path, moves an action to a disclosed keyboard-
/// reachable overflow, or discloses a zoom/contrast narrowing). `red` is blocked:
/// collapse changes the task identity, the ladder loses its placeholder terminal, the
/// transition loses identity or task state, critical state is hidden, an essential
/// action is hover-only or route-broken, zoom/contrast diverges the route semantics,
/// or a per-class presentation lands outside the declared ladder — and it may not
/// keep a shell-maturity claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponsiveCollapseStatus {
    /// Full standing: identity-stable across every class, no route/action loss.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl ResponsiveCollapseStatus {
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

/// How the collapse ladder behaves as the surface falls through it.
///
/// `identity_stable_ladder` means every ladder step preserves the task identity and
/// moves optional detail before critical content. `disclosed_ladder_narrowing` is a
/// disclosed step that trims optional detail early — a yellow narrowing.
/// `ladder_changes_identity` means a collapse step changed what task the surface
/// represents — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CollapseLadderState {
    /// Every ladder step preserves the task identity.
    IdentityStableLadder,
    /// A disclosed collapse step trims optional detail before critical content.
    DisclosedLadderNarrowing,
    /// A collapse step changed the task identity — a blocker.
    LadderChangesIdentity,
}

impl CollapseLadderState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityStableLadder => "identity_stable_ladder",
            Self::DisclosedLadderNarrowing => "disclosed_ladder_narrowing",
            Self::LadderChangesIdentity => "ladder_changes_identity",
        }
    }

    /// `true` when the ladder is identity-stable at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::IdentityStableLadder)
    }

    /// `true` when the ladder took a disclosed narrowing step.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedLadderNarrowing)
    }
}

/// How the docked-to-sheet (and back) transition preserves object identity and task
/// state.
///
/// `identity_and_state_preserved` means the same object and task state survive the
/// transition. `disclosed_state_rehydration` means the transition rehydrates task
/// state through a disclosed, waivered path — a yellow narrowing.
/// `identity_or_state_lost_on_transition` means the transition changed the object or
/// dropped task state — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityContinuityState {
    /// The same object and task state survive the docked-to-sheet transition.
    IdentityAndStatePreserved,
    /// The transition rehydrates task state through a disclosed, waivered path.
    DisclosedStateRehydration,
    /// The transition changed the object or dropped task state — a blocker.
    IdentityOrStateLostOnTransition,
}

impl IdentityContinuityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityAndStatePreserved => "identity_and_state_preserved",
            Self::DisclosedStateRehydration => "disclosed_state_rehydration",
            Self::IdentityOrStateLostOnTransition => "identity_or_state_lost_on_transition",
        }
    }

    /// `true` when identity and task state fully survive the transition.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::IdentityAndStatePreserved)
    }

    /// `true` when the transition took a disclosed rehydration path.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedStateRehydration)
    }
}

/// How critical state and essential actions stay reachable as width narrows.
///
/// `all_critical_and_actions_reachable` means critical state stays visible or moves
/// to a keyboard-reachable overflow and every essential action stays route-reachable.
/// `disclosed_overflow_reach` means a low-frequency action moved to a disclosed
/// keyboard-reachable overflow or drawer — a yellow narrowing. `critical_state_hidden`
/// (critical state hidden instead of overflowed) and
/// `essential_action_hover_only_or_route_broken` (an essential action became
/// hover-only or lost its route) are always blockers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CriticalActionReachState {
    /// Critical state stays visible/overflowed and every action is route-reachable.
    AllCriticalAndActionsReachable,
    /// A low-frequency action moved to a disclosed keyboard-reachable overflow.
    DisclosedOverflowReach,
    /// Critical state was hidden instead of overflowed — a blocker.
    CriticalStateHidden,
    /// An essential action became hover-only or lost its route — a blocker.
    EssentialActionHoverOnlyOrRouteBroken,
}

impl CriticalActionReachState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AllCriticalAndActionsReachable => "all_critical_and_actions_reachable",
            Self::DisclosedOverflowReach => "disclosed_overflow_reach",
            Self::CriticalStateHidden => "critical_state_hidden",
            Self::EssentialActionHoverOnlyOrRouteBroken => {
                "essential_action_hover_only_or_route_broken"
            }
        }
    }

    /// `true` when critical state and every essential action stay reachable.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::AllCriticalAndActionsReachable)
    }

    /// `true` when a low-frequency action took a disclosed overflow route.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedOverflowReach)
    }
}

/// How 400% zoom and high-contrast layouts preserve route semantics and task state.
///
/// `routes_stable_at_zoom_and_contrast` means zoom and high contrast expose the same
/// routes and task state. `disclosed_zoom_narrowing` means the high-zoom layout
/// discloses a narrowed presentation while keeping the same routes — a yellow
/// narrowing. `route_semantics_diverge_at_zoom` means zoom or high contrast changed
/// the route semantics — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoomContrastParityState {
    /// Zoom and high contrast expose the same routes and task state.
    RoutesStableAtZoomAndContrast,
    /// The high-zoom layout discloses a narrowed presentation, same routes.
    DisclosedZoomNarrowing,
    /// Zoom or high contrast changed the route semantics — a blocker.
    RouteSemanticsDivergeAtZoom,
}

impl ZoomContrastParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RoutesStableAtZoomAndContrast => "routes_stable_at_zoom_and_contrast",
            Self::DisclosedZoomNarrowing => "disclosed_zoom_narrowing",
            Self::RouteSemanticsDivergeAtZoom => "route_semantics_diverge_at_zoom",
        }
    }

    /// `true` when zoom and high contrast preserve route semantics fully.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::RoutesStableAtZoomAndContrast)
    }

    /// `true` when the high-zoom layout took a disclosed narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedZoomNarrowing)
    }
}

/// The presentation a family lands in at one responsive class.
///
/// The placement must be a member of the family's declared collapse ladder, and both
/// `identity_preserved` and `essential_actions_reachable` must hold; a false value or
/// an undeclared placement is a blocker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsiveClassPresentation {
    /// The responsive class this presentation describes.
    pub responsive_class: M5ResponsiveClass,
    /// The collapse placement the surface lands in at this class.
    pub placement: M5FallbackPlacement,
    /// `true` when the surface keeps the same object identity at this class.
    pub identity_preserved: bool,
    /// `true` when every essential action stays route-reachable at this class.
    pub essential_actions_reachable: bool,
}

impl ResponsiveClassPresentation {
    /// `true` when the presentation preserves identity and keeps actions reachable.
    pub const fn is_stable(&self) -> bool {
        self.identity_preserved && self.essential_actions_reachable
    }
}

/// Short, reviewer-facing label for a governed family's collapsing surface.
pub const fn surface_label(family: M5ShellSurfaceFamily) -> &'static str {
    match family {
        M5ShellSurfaceFamily::Notebook => "Notebook editor / cell surface",
        M5ShellSurfaceFamily::DataGrid => "Tabular data grid surface",
        M5ShellSurfaceFamily::Profiler => "Profiler / performance surface",
        M5ShellSurfaceFamily::Pipeline => "Pipeline / workflow graph surface",
        M5ShellSurfaceFamily::Docs => "Documentation reader surface",
        M5ShellSurfaceFamily::Preview => "Preview surface (render, diff, media)",
        M5ShellSurfaceFamily::Review => "Review / change-request surface",
        M5ShellSurfaceFamily::Incident => "Incident / operations-response surface",
        M5ShellSurfaceFamily::Companion => "Companion assistant surface",
        M5ShellSurfaceFamily::Operator => "Operator / control-plane surface",
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red posture stay narrowed
/// (yellow) rather than blocked — never lets a collapse identity change, a lost
/// transition, a hidden critical state, or a diverged zoom route hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsiveCollapseWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed family the waiver applies to.
    pub family: M5ShellSurfaceFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row
    /// blocks.
    pub expires_at: String,
}

impl ResponsiveCollapseWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed family's responsive claim.
///
/// The trigger token mirrors the frozen [`M5ShellDowngradeTrigger`] vocabulary so a
/// cause never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsiveCollapseCause {
    /// The governed family the cause applies to.
    pub family: M5ShellSurfaceFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5ShellDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a
    /// non-disclosed cause is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl ResponsiveCollapseCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed surface family, certified across its collapse-ladder,
/// identity-continuity, critical-action-reach, and zoom-contrast-parity posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsiveCollapseRow {
    /// The governed family being certified.
    pub family: M5ShellSurfaceFamily,
    /// The family's frozen qualification class from the shell-zone matrix.
    pub matrix_qualification: M5ShellQualificationClass,
    /// Owner role accountable for keeping this family governed.
    pub owner_role: String,
    /// Short collapsing-surface label.
    pub surface_label: String,
    /// Canonical shell slot from the matrix.
    pub canonical_slot: M5ShellZoneSlot,
    /// Declared fallback slot from the matrix.
    pub fallback_slot: M5ShellZoneSlot,
    /// Responsive classes this family must survive. Pulled from the matrix.
    pub declared_responsive_classes: Vec<M5ResponsiveClass>,
    /// Ordered responsive collapse ladder; terminates in `placeholder`. Pulled from
    /// the matrix.
    pub collapse_ladder: Vec<M5FallbackPlacement>,
    /// Per-class presentation, one per declared responsive class.
    pub class_presentations: Vec<ResponsiveClassPresentation>,
    /// Collapse-ladder posture.
    pub collapse_ladder_state: CollapseLadderState,
    /// Docked-to-sheet identity-continuity posture.
    pub identity_continuity: IdentityContinuityState,
    /// Critical-state / essential-action reach posture.
    pub critical_action_reach: CriticalActionReachState,
    /// Zoom / high-contrast route-parity posture.
    pub zoom_contrast_parity: ZoomContrastParityState,
    /// Consumer surfaces this family must stay aligned across. Pulled from the
    /// matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this family. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ShellDowngradeTrigger>,
    /// Active waiver, when a disclosed identity-continuity narrowing is in force.
    pub active_waiver: Option<ResponsiveCollapseWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: ResponsiveCollapseStatus,
    /// The exact collapse causes that narrowed or blocked this row.
    pub collapse_causes: Vec<ResponsiveCollapseCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl ResponsiveCollapseRow {
    /// `true` when this family's frozen qualification keeps a Stable claim.
    pub fn is_stable_qualified(&self) -> bool {
        self.matrix_qualification.is_stable()
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the collapse ladder terminates in an identity-preserving
    /// placeholder — the guarantee that a surface never dead-ends when it can no
    /// longer dock.
    pub fn ladder_terminates_in_placeholder(&self) -> bool {
        matches!(
            self.collapse_ladder.last(),
            Some(M5FallbackPlacement::Placeholder)
        )
    }

    /// `true` when the collapse ladder is ordered from most-docked to
    /// most-collapsed (docked → sheet → overflow → placeholder), so optional detail
    /// always sheds before critical content.
    pub fn ladder_is_ordered(&self) -> bool {
        self.collapse_ladder
            .windows(2)
            .all(|pair| pair[0] < pair[1])
    }

    /// `true` when the per-class presentations cover exactly the declared responsive
    /// classes — no class the family must survive is left uncertified and none is
    /// invented.
    pub fn presentations_cover_declared_classes(&self) -> bool {
        let declared: BTreeSet<M5ResponsiveClass> =
            self.declared_responsive_classes.iter().copied().collect();
        let present: BTreeSet<M5ResponsiveClass> = self
            .class_presentations
            .iter()
            .map(|presentation| presentation.responsive_class)
            .collect();
        declared == present && present.len() == self.class_presentations.len()
    }

    /// `true` when every presentation lands in a placement the family declared — the
    /// lint that prevents a collapse into an undeclared placement from shipping.
    pub fn presentations_placements_declared(&self) -> bool {
        self.class_presentations
            .iter()
            .all(|presentation| self.collapse_ladder.contains(&presentation.placement))
    }

    /// `true` when the presentations are monotonic: the compact class lands at or
    /// below (more collapsed than) standard, which lands at or below expanded.
    pub fn presentations_monotonic(&self) -> bool {
        let placement_for = |class: M5ResponsiveClass| {
            self.class_presentations
                .iter()
                .find(|presentation| presentation.responsive_class == class)
                .map(|presentation| presentation.placement)
        };
        match (
            placement_for(M5ResponsiveClass::CompactDesktop),
            placement_for(M5ResponsiveClass::StandardDesktop),
            placement_for(M5ResponsiveClass::ExpandedDesktop),
        ) {
            (Some(compact), Some(standard), Some(expanded)) => {
                compact >= standard && standard >= expanded
            }
            // A family that does not declare all three classes is checked by the
            // coverage lint instead.
            _ => true,
        }
    }

    /// `true` when every presentation preserves identity and keeps actions reachable.
    pub fn presentations_stable(&self) -> bool {
        self.class_presentations
            .iter()
            .all(ResponsiveClassPresentation::is_stable)
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.ladder_terminates_in_placeholder() || !self.ladder_is_ordered() {
            return true;
        }
        if matches!(
            self.collapse_ladder_state,
            CollapseLadderState::LadderChangesIdentity
        ) {
            return true;
        }
        if matches!(
            self.identity_continuity,
            IdentityContinuityState::IdentityOrStateLostOnTransition
        ) {
            return true;
        }
        if matches!(
            self.critical_action_reach,
            CriticalActionReachState::CriticalStateHidden
                | CriticalActionReachState::EssentialActionHoverOnlyOrRouteBroken
        ) {
            return true;
        }
        if matches!(
            self.zoom_contrast_parity,
            ZoomContrastParityState::RouteSemanticsDivergeAtZoom
        ) {
            return true;
        }
        if !self.presentations_cover_declared_classes()
            || !self.presentations_placements_declared()
            || !self.presentations_monotonic()
            || !self.presentations_stable()
        {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        !self.is_stable_qualified()
            || self.collapse_ladder_state.is_disclosed_narrowing()
            || self.identity_continuity.is_disclosed_narrowing()
            || self.critical_action_reach.is_disclosed_narrowing()
            || self.zoom_contrast_parity.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the collapse posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest
    /// narrowing forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> ResponsiveCollapseStatus {
        if self.has_hard_blocker() {
            ResponsiveCollapseStatus::Red
        } else if self.has_narrowing() {
            ResponsiveCollapseStatus::Yellow
        } else {
            ResponsiveCollapseStatus::Green
        }
    }

    /// Recomputes the exact collapse causes for the row, in deterministic order
    /// (qualification, ladder, identity continuity, critical/action reach, zoom).
    pub fn recompute_causes(&self) -> Vec<ResponsiveCollapseCause> {
        let mut causes = Vec::new();
        if !self.is_stable_qualified() {
            causes.push(ResponsiveCollapseCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                disclosed: true,
                detail: format!(
                    "Frozen shell-zone matrix qualifies this family at `{}`, below a Stable shell claim.",
                    self.matrix_qualification.as_str()
                ),
            });
        }
        if !self.ladder_terminates_in_placeholder() || !self.ladder_is_ordered() {
            causes.push(ResponsiveCollapseCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::PlaceholderLostIdentityOrReopen,
                disclosed: false,
                detail: "The collapse ladder is not ordered docked→sheet→overflow→placeholder or \
                         does not terminate in an identity-preserving placeholder."
                    .to_owned(),
            });
        }
        match self.collapse_ladder_state {
            CollapseLadderState::IdentityStableLadder => {}
            CollapseLadderState::DisclosedLadderNarrowing => causes.push(ResponsiveCollapseCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                disclosed: true,
                detail: "A disclosed collapse step trims optional detail before critical content \
                         while preserving the task identity."
                    .to_owned(),
            }),
            CollapseLadderState::LadderChangesIdentity => causes.push(ResponsiveCollapseCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::CollapseChangedTaskIdentity,
                disclosed: false,
                detail: "A responsive collapse step changed the task identity of the surface."
                    .to_owned(),
            }),
        }
        match self.identity_continuity {
            IdentityContinuityState::IdentityAndStatePreserved => {}
            IdentityContinuityState::DisclosedStateRehydration => {
                causes.push(ResponsiveCollapseCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail: "The docked-to-sheet transition rehydrates task state through a \
                             disclosed, waivered path while preserving the object identity."
                        .to_owned(),
                });
            }
            IdentityContinuityState::IdentityOrStateLostOnTransition => {
                causes.push(ResponsiveCollapseCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::CollapseChangedTaskIdentity,
                    disclosed: false,
                    detail: "The docked-to-sheet transition changed the object or dropped task \
                             state."
                        .to_owned(),
                });
            }
        }
        match self.critical_action_reach {
            CriticalActionReachState::AllCriticalAndActionsReachable => {}
            CriticalActionReachState::DisclosedOverflowReach => {
                causes.push(ResponsiveCollapseCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail:
                        "A low-frequency action moved to a disclosed keyboard-reachable overflow \
                         or drawer before primary navigation was starved."
                            .to_owned(),
                })
            }
            CriticalActionReachState::CriticalStateHidden => causes.push(ResponsiveCollapseCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::CriticalStateHiddenOnCollapse,
                disclosed: false,
                detail: "Critical state was hidden on collapse instead of moving to a \
                         keyboard-reachable overflow."
                    .to_owned(),
            }),
            CriticalActionReachState::EssentialActionHoverOnlyOrRouteBroken => {
                causes.push(ResponsiveCollapseCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::CriticalStateHiddenOnCollapse,
                    disclosed: false,
                    detail: "An essential action became hover-only or lost its route as width \
                             narrowed."
                        .to_owned(),
                });
            }
        }
        match self.zoom_contrast_parity {
            ZoomContrastParityState::RoutesStableAtZoomAndContrast => {}
            ZoomContrastParityState::DisclosedZoomNarrowing => {
                causes.push(ResponsiveCollapseCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                    disclosed: true,
                    detail:
                        "The 400% zoom / high-contrast layout discloses a narrowed presentation \
                         while exposing the same routes and task state."
                            .to_owned(),
                })
            }
            ZoomContrastParityState::RouteSemanticsDivergeAtZoom => {
                causes.push(ResponsiveCollapseCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::CollapseChangedTaskIdentity,
                    disclosed: false,
                    detail: "The 400% zoom / high-contrast layout diverged the route semantics \
                             from the standard layout."
                        .to_owned(),
                });
            }
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay
    /// publishable.
    ///
    /// A disclosed docked-to-sheet state rehydration may only stay yellow (rather
    /// than red) when a waiver discloses it.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.identity_continuity,
            IdentityContinuityState::DisclosedStateRehydration
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<ResponsiveCollapseFinding> {
        let mut findings = Vec::new();
        let family = self.family.as_str().to_owned();

        if !self.ladder_terminates_in_placeholder() {
            findings.push(
                ResponsiveCollapseFinding::LadderMissingPlaceholderTerminal {
                    family: family.clone(),
                },
            );
        }
        if !self.ladder_is_ordered() {
            findings.push(ResponsiveCollapseFinding::LadderNotOrdered {
                family: family.clone(),
            });
        }
        if matches!(
            self.collapse_ladder_state,
            CollapseLadderState::LadderChangesIdentity
        ) {
            findings.push(ResponsiveCollapseFinding::LadderChangesIdentity {
                family: family.clone(),
            });
        }
        if matches!(
            self.identity_continuity,
            IdentityContinuityState::IdentityOrStateLostOnTransition
        ) {
            findings.push(ResponsiveCollapseFinding::IdentityOrStateLostOnTransition {
                family: family.clone(),
            });
        }
        if matches!(
            self.critical_action_reach,
            CriticalActionReachState::CriticalStateHidden
        ) {
            findings.push(ResponsiveCollapseFinding::CriticalStateHidden {
                family: family.clone(),
            });
        }
        if matches!(
            self.critical_action_reach,
            CriticalActionReachState::EssentialActionHoverOnlyOrRouteBroken
        ) {
            findings.push(
                ResponsiveCollapseFinding::EssentialActionHoverOnlyOrRouteBroken {
                    family: family.clone(),
                },
            );
        }
        if matches!(
            self.zoom_contrast_parity,
            ZoomContrastParityState::RouteSemanticsDivergeAtZoom
        ) {
            findings.push(ResponsiveCollapseFinding::RouteSemanticsDivergeAtZoom {
                family: family.clone(),
            });
        }
        if !self.presentations_cover_declared_classes() {
            findings.push(
                ResponsiveCollapseFinding::PresentationClassCoverageMismatch {
                    family: family.clone(),
                },
            );
        }
        if !self.presentations_placements_declared() {
            findings.push(ResponsiveCollapseFinding::PresentationPlacementUndeclared {
                family: family.clone(),
            });
        }
        if !self.presentations_monotonic() {
            findings.push(ResponsiveCollapseFinding::PresentationLadderNonMonotonic {
                family: family.clone(),
            });
        }
        if !self.presentations_stable() {
            findings.push(
                ResponsiveCollapseFinding::PresentationIdentityOrActionLost {
                    family: family.clone(),
                },
            );
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, ResponsiveCollapseStatus::Green) && !self.has_reason() {
            findings.push(ResponsiveCollapseFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry
        // an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(ResponsiveCollapseFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.family != self.family {
                findings.push(ResponsiveCollapseFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(ResponsiveCollapseFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(ResponsiveCollapseFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.collapse_causes != self.recompute_causes() {
            findings.push(ResponsiveCollapseFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} ladder={} identity={} action={} zoom={} waiver={}",
            self.family.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.collapse_ladder_state.as_str(),
            self.identity_continuity.as_str(),
            self.critical_action_reach.as_str(),
            self.zoom_contrast_parity.as_str(),
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the responsive-collapse proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum ResponsiveCollapseFinding {
    /// A governed surface family has no collapse row.
    FamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row's collapse ladder does not terminate in an identity-preserving
    /// placeholder.
    LadderMissingPlaceholderTerminal {
        /// The family token.
        family: String,
    },
    /// A row's collapse ladder is not ordered most-docked to most-collapsed.
    LadderNotOrdered {
        /// The family token.
        family: String,
    },
    /// A row's responsive collapse changed the task identity.
    LadderChangesIdentity {
        /// The family token.
        family: String,
    },
    /// A row's docked-to-sheet transition lost identity or task state.
    IdentityOrStateLostOnTransition {
        /// The family token.
        family: String,
    },
    /// A row hid critical state on collapse instead of overflowing it.
    CriticalStateHidden {
        /// The family token.
        family: String,
    },
    /// A row's essential action became hover-only or route-broken.
    EssentialActionHoverOnlyOrRouteBroken {
        /// The family token.
        family: String,
    },
    /// A row's zoom / high-contrast layout diverged the route semantics.
    RouteSemanticsDivergeAtZoom {
        /// The family token.
        family: String,
    },
    /// A row's per-class presentations do not cover exactly the declared classes.
    PresentationClassCoverageMismatch {
        /// The family token.
        family: String,
    },
    /// A row's presentation lands in a placement the family never declared.
    PresentationPlacementUndeclared {
        /// The family token.
        family: String,
    },
    /// A row's presentations are not monotonic across compact/standard/expanded.
    PresentationLadderNonMonotonic {
        /// The family token.
        family: String,
    },
    /// A row's presentation loses identity or an essential action at some class.
    PresentationIdentityOrActionLost {
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
    /// The declared collapse causes do not match the recomputed causes.
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

impl ResponsiveCollapseFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::FamilyMissing { .. } => "family_missing",
            Self::LadderMissingPlaceholderTerminal { .. } => "ladder_missing_placeholder_terminal",
            Self::LadderNotOrdered { .. } => "ladder_not_ordered",
            Self::LadderChangesIdentity { .. } => "ladder_changes_identity",
            Self::IdentityOrStateLostOnTransition { .. } => "identity_or_state_lost_on_transition",
            Self::CriticalStateHidden { .. } => "critical_state_hidden",
            Self::EssentialActionHoverOnlyOrRouteBroken { .. } => {
                "essential_action_hover_only_or_route_broken"
            }
            Self::RouteSemanticsDivergeAtZoom { .. } => "route_semantics_diverge_at_zoom",
            Self::PresentationClassCoverageMismatch { .. } => {
                "presentation_class_coverage_mismatch"
            }
            Self::PresentationPlacementUndeclared { .. } => "presentation_placement_undeclared",
            Self::PresentationLadderNonMonotonic { .. } => "presentation_ladder_non_monotonic",
            Self::PresentationIdentityOrActionLost { .. } => "presentation_identity_or_action_lost",
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
            Self::FamilyMissing { family }
            | Self::LadderMissingPlaceholderTerminal { family }
            | Self::LadderNotOrdered { family }
            | Self::LadderChangesIdentity { family }
            | Self::IdentityOrStateLostOnTransition { family }
            | Self::CriticalStateHidden { family }
            | Self::EssentialActionHoverOnlyOrRouteBroken { family }
            | Self::RouteSemanticsDivergeAtZoom { family }
            | Self::PresentationClassCoverageMismatch { family }
            | Self::PresentationPlacementUndeclared { family }
            | Self::PresentationLadderNonMonotonic { family }
            | Self::PresentationIdentityOrActionLost { family }
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

/// The release responsive-collapse packet shared by the shell / windowing / layout /
/// release automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsiveCollapsePacket {
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
    /// The frozen shell-zone matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen shell-zone matrix schema.
    pub matrix_schema_ref: String,
    /// Repo-relative ref to the frozen responsive-class schema.
    pub responsive_class_schema_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// Per-family collapse rows, in canonical order.
    pub rows: Vec<ResponsiveCollapseRow>,
    /// Governed families certified, in canonical (sorted) order.
    pub covered_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (identity-stable) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<ResponsiveCollapseWaiver>,
    /// Every exact collapse cause, in row then cause order.
    pub collapse_causes: Vec<ResponsiveCollapseCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<ResponsiveCollapseFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Shell / release automation refs that consume this packet to auto-narrow
    /// claimed surfaces.
    pub shell_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet.
    pub release_center_refs: Vec<String>,
    /// Docs / help refs the packet reopens from.
    pub help_docs_refs: Vec<String>,
    /// Support / export refs that preserve the packet.
    pub support_export_refs: Vec<String>,
    /// Published markdown report ref.
    pub published_report_ref: String,
    /// Published collapse-packet ref.
    pub published_packet_ref: String,
    /// Published collapse-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl ResponsiveCollapsePacket {
    /// Returns the collapse row for `family`, if present.
    pub fn row(&self, family: M5ShellSurfaceFamily) -> Option<&ResponsiveCollapseRow> {
        self.rows.iter().find(|row| row.family == family)
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
                waiver.family.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.collapse_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.family.as_str(),
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

    /// Projects the light collapse dashboard the shell automation consumes.
    pub fn dashboard(&self) -> ResponsiveCollapseDashboard {
        ResponsiveCollapseDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 responsive-collapse packet serializes")
    }

    /// Deterministic, machine-readable collapse CSV: one row per family naming its
    /// status, qualification, ladder, per-dimension posture, and waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "family,status,qualification,canonical_slot,fallback_slot,collapse_ladder,compact_placement,collapse_ladder_state,identity_continuity,critical_action_reach,zoom_contrast_parity,waiver\n",
        );
        for row in &self.rows {
            let compact_placement = row
                .class_presentations
                .iter()
                .find(|p| p.responsive_class == M5ResponsiveClass::CompactDesktop)
                .map(|p| p.placement.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.family.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.canonical_slot.as_str(),
                row.fallback_slot.as_str(),
                join_tokens(&row.collapse_ladder, |p| p.as_str()),
                compact_placement,
                row.collapse_ladder_state.as_str(),
                row.identity_continuity.as_str(),
                row.critical_action_reach.as_str(),
                row.zoom_contrast_parity.as_str(),
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
        out.push_str("# M5 responsive collapse: compact / standard / expanded parity\n\n");
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_responsive_collapse`](../../crates/aureline-shell/src/m5_responsive_collapse/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse -- markdown > \\\n  artifacts/shell/m5-responsive-collapse.md\n",
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
        out.push_str(&format!("- Rows certified: {}\n", self.row_count));
        out.push_str(&format!(
            "- Green (identity-stable): {}\n",
            self.green_row_count
        ));
        out.push_str(&format!(
            "- Yellow (auto-narrowed): {}\n",
            self.yellow_row_count
        ));
        out.push_str(&format!("- Red (blocked): {}\n", self.red_row_count));
        out.push_str(&format!(
            "- All rows publishable: `{}`\n",
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

        out.push_str("## Collapse rows\n\n");
        out.push_str(
            "| Surface | Status | Qualification | Collapse ladder | Ladder | Identity | Action reach | Zoom/contrast | Waiver |\n\
             | ------- | ------ | ------------- | --------------- | ------ | -------- | ------------ | ------------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.surface_label,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                join_tokens(&row.collapse_ladder, |p| p.as_str()),
                row.collapse_ladder_state.as_str(),
                row.identity_continuity.as_str(),
                row.critical_action_reach.as_str(),
                row.zoom_contrast_parity.as_str(),
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Per-class presentation\n\n");
        out.push_str(
            "| Surface | Compact | Standard | Expanded |\n\
             | ------- | ------- | -------- | -------- |\n",
        );
        for row in &self.rows {
            let placement = |class: M5ResponsiveClass| {
                row.class_presentations
                    .iter()
                    .find(|p| p.responsive_class == class)
                    .map(|p| p.placement.as_str())
                    .unwrap_or("—")
            };
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` |\n",
                row.surface_label,
                placement(M5ResponsiveClass::CompactDesktop),
                placement(M5ResponsiveClass::StandardDesktop),
                placement(M5ResponsiveClass::ExpandedDesktop),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&ResponsiveCollapseRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, ResponsiveCollapseStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every governed family stays identity-stable across compact/standard/expanded.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.family.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact collapse causes\n\n");
        if self.collapse_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.collapse_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.family.as_str(),
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
                    waiver.family.as_str(),
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_responsive_collapse -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_responsive_collapse_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light collapse dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsiveCollapseDashboardRow {
    /// The governed family.
    pub family: M5ShellSurfaceFamily,
    /// Short collapsing-surface label.
    pub surface_label: String,
    /// Derived green/yellow/red status.
    pub status: ResponsiveCollapseStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5ShellQualificationClass,
    /// The collapse placement the surface lands in at compact width.
    pub compact_placement: Option<M5FallbackPlacement>,
    /// Collapse-ladder posture.
    pub collapse_ladder_state: CollapseLadderState,
    /// Identity-continuity posture.
    pub identity_continuity: IdentityContinuityState,
    /// Critical-state / essential-action reach posture.
    pub critical_action_reach: CriticalActionReachState,
    /// Zoom / high-contrast route-parity posture.
    pub zoom_contrast_parity: ZoomContrastParityState,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light collapse dashboard the shell / windowing / layout / release automation
/// reads to auto-narrow claimed surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsiveCollapseDashboard {
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
    pub rows: Vec<ResponsiveCollapseDashboardRow>,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow rows.
    pub yellow_row_count: usize,
    /// Number of red rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Shell / release automation refs that consume the dashboard.
    pub shell_automation_refs: Vec<String>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl ResponsiveCollapseDashboard {
    /// Projects the dashboard from a collapse packet.
    pub fn from_packet(packet: &ResponsiveCollapsePacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| ResponsiveCollapseDashboardRow {
                family: row.family,
                surface_label: row.surface_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                compact_placement: row
                    .class_presentations
                    .iter()
                    .find(|p| p.responsive_class == M5ResponsiveClass::CompactDesktop)
                    .map(|p| p.placement),
                collapse_ladder_state: row.collapse_ladder_state,
                identity_continuity: row.identity_continuity,
                critical_action_reach: row.critical_action_reach,
                zoom_contrast_parity: row.zoom_contrast_parity,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .collapse_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_RESPONSIVE_COLLAPSE_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_RESPONSIVE_COLLAPSE_SCHEMA_VERSION,
            dashboard_id: M5_RESPONSIVE_COLLAPSE_DASHBOARD_ID.to_owned(),
            source_packet_ref: packet.packet_id.clone(),
            source_schema_ref: packet.source_schema_ref.clone(),
            rows,
            green_row_count: packet.green_row_count,
            yellow_row_count: packet.yellow_row_count,
            red_row_count: packet.red_row_count,
            all_rows_publishable: packet.all_rows_publishable,
            shell_automation_refs: packet.shell_automation_refs.clone(),
            generated_at: packet.generated_at.clone(),
        }
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only dashboard fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 responsive-collapse dashboard serializes")
    }
}

/// Support-export wrapper for the responsive-collapse packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResponsiveCollapseSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: ResponsiveCollapsePacket,
    /// Dashboard quoted in full.
    pub dashboard: ResponsiveCollapseDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl ResponsiveCollapseSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each family, and
    /// each active waiver id is quoted as a case id so a support reviewer — or the
    /// shell automation — can name the same family and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: ResponsiveCollapsePacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.family.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_RESPONSIVE_COLLAPSE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_RESPONSIVE_COLLAPSE_SCHEMA_VERSION,
            shared_contract_ref: M5_RESPONSIVE_COLLAPSE_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_responsive_collapse_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResponsiveCollapseInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-zone matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family collapse rows.
    pub rows: Vec<ResponsiveCollapseRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
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

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The collapse packet carries only closed vocabulary, refs, and short labels, so
/// raw URLs, credentials, or tokens must never appear.
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

/// Builds a [`ResponsiveCollapsePacket`] from the exact build identity, the frozen
/// matrix ref, and the per-family collapse rows.
///
/// Each row's derived status and collapse causes, the aggregate counts, the active
/// waivers, and the blocking findings are recomputed here so the packet is the
/// single source of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_responsive_collapse_packet(
    input: ResponsiveCollapseInput,
) -> ResponsiveCollapsePacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is
    // self-consistent and the auto-narrowing is the single source of truth.
    let rows: Vec<ResponsiveCollapseRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.collapse_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<ResponsiveCollapseFinding> = Vec::new();

    // Every governed family must carry a collapse row.
    let present: BTreeSet<M5ShellSurfaceFamily> = rows.iter().map(|row| row.family).collect();
    for family in REQUIRED_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(ResponsiveCollapseFinding::FamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_families: Vec<String> = {
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
        .filter(|row| matches!(row.derived_status, ResponsiveCollapseStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ResponsiveCollapseStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ResponsiveCollapseStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(ResponsiveCollapseFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<ResponsiveCollapseWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let collapse_causes: Vec<ResponsiveCollapseCause> = rows
        .iter()
        .flat_map(|row| row.collapse_causes.clone())
        .collect();

    let mut packet = ResponsiveCollapsePacket {
        record_kind: M5_RESPONSIVE_COLLAPSE_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_RESPONSIVE_COLLAPSE_SCHEMA_VERSION,
        shared_contract_ref: M5_RESPONSIVE_COLLAPSE_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_RESPONSIVE_COLLAPSE_PACKET_ID.to_owned(),
        source_schema_ref: M5_RESPONSIVE_COLLAPSE_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Compact/standard/expanded responsive-collapse parity for every claimed M5 \
                   surface family: notebook, data grid, profiler, pipeline, docs, preview, review, \
                   incident, companion, and operator each certified to stay identity-stable across \
                   the collapse ladder, preserve object identity and task state on the \
                   docked-to-sheet transition, keep critical state and essential actions reachable \
                   as width narrows, and hold the same route semantics at 400% zoom and high \
                   contrast, with each row's green/yellow/red claim auto-narrowed from its \
                   collapse-ladder, identity-continuity, action-reach, and zoom-parity posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_RESPONSIVE_COLLAPSE_MATRIX_SCHEMA_REF.to_owned(),
        responsive_class_schema_ref: M5_RESPONSIVE_COLLAPSE_RESPONSIVE_CLASS_SCHEMA_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        rows,
        covered_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        collapse_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        shell_automation_refs: vec![
            "shell_frame.responsive_collapse_registry".to_owned(),
            "release_automation.auto_narrow.responsive_collapse_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.responsive_collapse".to_owned(),
            M5_RESPONSIVE_COLLAPSE_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_RESPONSIVE_COLLAPSE_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-responsive-collapse".to_owned()],
        published_report_ref: M5_RESPONSIVE_COLLAPSE_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_RESPONSIVE_COLLAPSE_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_RESPONSIVE_COLLAPSE_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_RESPONSIVE_COLLAPSE_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("collapse packet serializes"),
    ) {
        blocking_findings.push(ResponsiveCollapseFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_responsive_collapse_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum ResponsiveCollapseValidationError {
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
    /// The rows do not cover all ten governed families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared collapse causes do not match the recomputed causes.
    CollapseCausesStale,
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

/// Validates a packet against the responsive-collapse invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed
/// family carries a current collapse row; each row's status is the derived
/// auto-narrowed value, never asserted; a green row cannot keep a claim while its
/// collapse changes identity, its ladder loses the placeholder terminal, its
/// docked-to-sheet transition loses state, critical state is hidden, an essential
/// action becomes hover-only or route-broken, zoom/contrast diverges the routes, or a
/// per-class presentation lands outside the declared ladder; and a disclosed
/// narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_responsive_collapse_packet(
    packet: &ResponsiveCollapsePacket,
) -> Result<(), Vec<ResponsiveCollapseValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(ResponsiveCollapseValidationError::NoRows);
    }
    if packet.record_kind != M5_RESPONSIVE_COLLAPSE_PACKET_RECORD_KIND {
        errors.push(ResponsiveCollapseValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_RESPONSIVE_COLLAPSE_SCHEMA_VERSION {
        errors.push(ResponsiveCollapseValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(ResponsiveCollapseValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(ResponsiveCollapseValidationError::MatrixPacketRefMissing);
    }

    let present: BTreeSet<M5ShellSurfaceFamily> =
        packet.rows.iter().map(|row| row.family).collect();
    let coverage_complete = REQUIRED_FAMILIES
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_FAMILIES.len() {
        errors.push(ResponsiveCollapseValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_families {
        errors.push(ResponsiveCollapseValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ResponsiveCollapseStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ResponsiveCollapseStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ResponsiveCollapseStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(ResponsiveCollapseValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<ResponsiveCollapseWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(ResponsiveCollapseValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<ResponsiveCollapseCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.collapse_causes {
        errors.push(ResponsiveCollapseValidationError::CollapseCausesStale);
    }

    let mut recomputed: Vec<ResponsiveCollapseFinding> = Vec::new();
    for family in REQUIRED_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(ResponsiveCollapseFinding::FamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(ResponsiveCollapseFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("collapse packet serializes"),
    ) {
        recomputed.push(ResponsiveCollapseFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(ResponsiveCollapseValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(ResponsiveCollapseValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(ResponsiveCollapseValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(ResponsiveCollapseValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(ResponsiveCollapseValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(ResponsiveCollapseValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
