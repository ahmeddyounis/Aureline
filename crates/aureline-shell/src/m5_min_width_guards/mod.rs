//! Canonical editor-min-width, compare-fallback, and no-unusable-narrow-pane
//! certification for every claimed M5 surface family.
//!
//! The [frozen shell-zone matrix][matrix] already binds each claimed M5 surface
//! family — notebook, data grid, profiler, pipeline, docs, preview, review,
//! incident, companion, and operator — to the responsive classes it must survive
//! and the occupant transitions (side-by-side, tabbed, sheeted, overflowed,
//! solo-docked) it may take as width narrows. This lane is the minimum-useful-size
//! capstone on top of that matrix: for every governed family it certifies that its
//! editor / primary pane **enforces a minimum useful width and height**, that when a
//! second group, compare, diff, or dense inspector would violate that minimum the
//! surface **falls back to a declared safe compare mode** (tabbed compare, staged
//! peek, sequential disclosure, or explicit user choice) instead of silently
//! producing an unusable narrow split, and that breadcrumbs, active object identity,
//! and recovery-critical status **stay visible** while the fallback is active.
//!
//! Three records carry the truth:
//!
//! - the per-family **guard row** ([`MinWidthGuardRow`]): one row per
//!   [`M5ShellSurfaceFamily`] naming the responsive classes and occupant transitions
//!   it declares, the declared safe compare-fallback strategies pulled from the
//!   matrix's occupant transitions, the minimum useful width and height it enforces,
//!   its per-class compare plan ([`MinWidthClassPlan`]) with the strategy it lands in
//!   and whether that plan meets the minimum and preserves identity and status, its
//!   min-size-enforcement / compare-fallback / status-continuity posture, any active
//!   waiver, and a derived green/yellow/red [`MinWidthGuardStatus`].
//! - the release **guard packet** ([`MinWidthGuardPacket`]): the full set of rows with
//!   derived per-row status, aggregate green/yellow/red counts, the active waivers,
//!   the exact guard causes ([`MinWidthGuardCause`]), and the blocking findings the
//!   lane refuses to ship with.
//! - the **guard dashboard** ([`MinWidthGuardDashboard`]): a light projection the
//!   shell / windowing / layout / release automation reads to auto-narrow a claimed
//!   surface when its minimum-size proof falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to
//! `yellow` the moment its frozen qualification is below Stable, its editor discloses
//! a reduced-but-still-usable minimum under compact width, its compare area discloses
//! a narrowed fallback, or its recovery-critical status is relocated to a disclosed,
//! waivered still-visible affordance; it drops to `red` if the editor or compare pane
//! can be forced below a usable minimum, a compare or diff produces a silent unusable
//! split with no fallback, breadcrumbs / identity / recovery-critical status are lost
//! while the fallback is active, the declared safe-fallback set does not terminate in
//! a universally-available safe mode, the row's primary fallback strategy is not one
//! it declared, its declared minimum falls below the absolute floor its enforcement
//! claims, or a per-class compare plan lands in a strategy the family never declared.
//! That derivation is the auto-narrowing the acceptance criteria require, and the
//! strategy-set and plan checks are the lint that prevents a later narrow-pane
//! regression from shipping as stable.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw
//! URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials —
//! only stable ids, closed vocabulary, counts, refs, pixel floors, and short labels.
//! The surface family, responsive-class, occupant-persistence, qualification,
//! downgrade-trigger, and consumer-surface vocabulary is re-exported by reference from
//! the already frozen [matrix]; the certified rows are pulled straight from that
//! matrix's seeded packet, so this lane mints no parallel shell vocabulary and cannot
//! certify a family the matrix does not freeze. Only the min-width-guard-specific
//! vocabulary ([`MinWidthGuardStatus`], [`MinSizeEnforcementState`],
//! [`CompareFallbackState`], [`StatusContinuityState`], [`M5CompareFallbackStrategy`],
//! [`MinWidthClassPlan`], [`MinWidthGuardWaiver`], [`MinWidthGuardCause`],
//! [`MinWidthGuardFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_shell_zone_responsive_class_and_multi_window_continuity_matrix as matrix;

pub use matrix::{
    M5FallbackPlacement, M5OccupantPersistence, M5ResponsiveClass, M5ShellConsumerSurface,
    M5ShellDowngradeTrigger, M5ShellQualificationClass, M5ShellSurfaceFamily, M5ShellZoneSlot,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_min_width_guards_packet,
    seeded_m5_min_width_guards_packet_companion_strategy_set_no_terminal_blocked,
    seeded_m5_min_width_guards_packet_datagrid_status_lost_blocked,
    seeded_m5_min_width_guards_packet_notebook_pane_below_minimum_blocked,
    seeded_m5_min_width_guards_packet_preview_silent_unusable_split_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_MIN_WIDTH_GUARDS_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_MIN_WIDTH_GUARDS_SHARED_CONTRACT_REF: &str = "shell:m5_min_width_guards:v1";

/// Stable record kind for [`MinWidthGuardPacket`] payloads.
pub const M5_MIN_WIDTH_GUARDS_PACKET_RECORD_KIND: &str = "shell_m5_min_width_guards_packet_record";

/// Stable record kind for [`MinWidthGuardDashboard`] payloads.
pub const M5_MIN_WIDTH_GUARDS_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_min_width_guards_dashboard_record";

/// Stable record kind for [`MinWidthGuardSupportExport`] payloads.
pub const M5_MIN_WIDTH_GUARDS_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_min_width_guards_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_MIN_WIDTH_GUARDS_PACKET_ID: &str = "m5-min-width-guards:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_MIN_WIDTH_GUARDS_DASHBOARD_ID: &str = "m5-min-width-guards-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_MIN_WIDTH_GUARDS_SUPPORT_EXPORT_ID: &str = "support-export:m5-min-width-guards:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_MIN_WIDTH_GUARDS_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-min-width-guards.schema.json";

/// Published markdown report ref reviewers reopen the min-size proof from.
pub const M5_MIN_WIDTH_GUARDS_PUBLISHED_REPORT_REF: &str = "artifacts/shell/m5-min-width-guards.md";

/// Published guard-packet artifact ref.
pub const M5_MIN_WIDTH_GUARDS_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-min-width-guards-proof/packet.json";

/// Published guard-dashboard artifact ref.
pub const M5_MIN_WIDTH_GUARDS_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-min-width-guards-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_MIN_WIDTH_GUARDS_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-min-width-guards-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_MIN_WIDTH_GUARDS_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-min-width-guards-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_MIN_WIDTH_GUARDS_PUBLISHED_DOC_REF: &str = "docs/shell/m5_min_width_guards_contract.md";

/// Repo-relative ref to the frozen shell-zone matrix schema.
pub const M5_MIN_WIDTH_GUARDS_MATRIX_SCHEMA_REF: &str = matrix::M5_SHELL_ZONE_MATRIX_SCHEMA_REF;

/// Repo-relative ref to the frozen responsive-class schema.
pub const M5_MIN_WIDTH_GUARDS_RESPONSIVE_CLASS_SCHEMA_REF: &str =
    matrix::M5_SHELL_RESPONSIVE_CLASS_SCHEMA_REF;

/// Absolute floor for a usable editor/compare pane width, in logical pixels. A pane
/// enforcing a disclosed reduced minimum may narrow to this floor but never below it.
pub const ABSOLUTE_MIN_USEFUL_WIDTH_PX: u32 = 320;

/// Absolute floor for a usable editor/compare pane height, in logical pixels.
pub const ABSOLUTE_MIN_USEFUL_HEIGHT_PX: u32 = 200;

/// Minimum useful width a pane claiming full min-size enforcement must reserve.
pub const STANDARD_MIN_USEFUL_WIDTH_PX: u32 = 480;

/// Minimum useful height a pane claiming full min-size enforcement must reserve.
pub const STANDARD_MIN_USEFUL_HEIGHT_PX: u32 = 320;

/// Every governed surface family the min-size proof must cover, in canonical order.
/// These are exactly the families the frozen shell-zone matrix freezes; the lane
/// certifies none beyond them and refuses to ship if any is missing.
pub const REQUIRED_FAMILIES: [M5ShellSurfaceFamily; 10] = M5ShellSurfaceFamily::ALL;

/// The derived min-width-guard light a governed surface family carries.
///
/// `green` means the family enforces a minimum useful width and height, falls back to
/// a declared safe compare mode before ever producing an unusable split, and keeps
/// breadcrumbs, identity, and recovery-critical status visible while the fallback is
/// active. `yellow` is a disclosed narrowing (the family is honestly narrowed below
/// Stable, discloses a reduced-but-usable minimum, discloses a narrowed compare
/// fallback, or relocates recovery-critical status to a disclosed, waivered
/// affordance). `red` is blocked: the editor or compare pane can be forced below a
/// usable minimum, a compare/diff produces a silent unusable split, breadcrumbs /
/// identity / recovery-critical status are lost under the fallback, the safe-fallback
/// set has no universal terminal, the primary fallback strategy is undeclared, the
/// declared minimum drops below the absolute floor, or a per-class plan lands in an
/// undeclared strategy — and it may not keep a shell-maturity claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MinWidthGuardStatus {
    /// Full standing: min useful size enforced, safe fallback, status preserved.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl MinWidthGuardStatus {
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

/// How the editor / primary pane enforces a minimum useful size.
///
/// `min_useful_size_enforced` means a hard minimum useful width and height is
/// reserved and any split that would violate it is prevented before it happens.
/// `disclosed_reduced_minimum` means the surface discloses a reduced-but-still-usable
/// minimum under compact width — a yellow narrowing. `pane_forced_below_usable_minimum`
/// means the pane can be dragged or split below a usable minimum — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MinSizeEnforcementState {
    /// A hard minimum useful width and height is reserved and enforced.
    MinUsefulSizeEnforced,
    /// The surface discloses a reduced-but-still-usable minimum under compact width.
    DisclosedReducedMinimum,
    /// The pane can be forced below a usable minimum — a blocker.
    PaneForcedBelowUsableMinimum,
}

impl MinSizeEnforcementState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MinUsefulSizeEnforced => "min_useful_size_enforced",
            Self::DisclosedReducedMinimum => "disclosed_reduced_minimum",
            Self::PaneForcedBelowUsableMinimum => "pane_forced_below_usable_minimum",
        }
    }

    /// `true` when a hard minimum useful size is enforced at full standing.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::MinUsefulSizeEnforced)
    }

    /// `true` when the surface disclosed a reduced-but-usable minimum.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedReducedMinimum)
    }
}

/// How a would-be-unusable compare / second-group / diff split falls back.
///
/// `safe_fallback_before_unusable_split` means the surface switches to a declared safe
/// compare mode (tabbed compare, staged peek, sequential disclosure, or explicit user
/// choice) before it would produce an unusable split. `disclosed_fallback_narrowing`
/// means the fallback trims a secondary pane's optional detail in a disclosed way — a
/// yellow narrowing. `silent_unusable_split` means the surface produces a silent
/// unusable narrow split with no fallback — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareFallbackState {
    /// The surface switches to a declared safe compare mode before an unusable split.
    SafeFallbackBeforeUnusableSplit,
    /// The fallback trims a secondary pane's optional detail in a disclosed way.
    DisclosedFallbackNarrowing,
    /// The surface produces a silent unusable narrow split — a blocker.
    SilentUnusableSplit,
}

impl CompareFallbackState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SafeFallbackBeforeUnusableSplit => "safe_fallback_before_unusable_split",
            Self::DisclosedFallbackNarrowing => "disclosed_fallback_narrowing",
            Self::SilentUnusableSplit => "silent_unusable_split",
        }
    }

    /// `true` when a safe fallback always precedes an unusable split.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::SafeFallbackBeforeUnusableSplit)
    }

    /// `true` when the fallback took a disclosed narrowing.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedFallbackNarrowing)
    }
}

/// How breadcrumbs, active object identity, and recovery-critical status survive while
/// the compare fallback is active.
///
/// `identity_breadcrumbs_status_preserved` means the active object, its breadcrumbs,
/// and recovery-critical status all stay visible through the fallback.
/// `disclosed_status_relocation` means recovery-critical status is relocated to a
/// disclosed, waivered still-visible affordance — a yellow narrowing.
/// `status_or_identity_lost_under_fallback` means breadcrumbs, identity, or
/// recovery-critical status is lost while the fallback is active — always a blocker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusContinuityState {
    /// Identity, breadcrumbs, and recovery-critical status all stay visible.
    IdentityBreadcrumbsStatusPreserved,
    /// Recovery-critical status is relocated to a disclosed, waivered affordance.
    DisclosedStatusRelocation,
    /// Breadcrumbs, identity, or recovery-critical status is lost — a blocker.
    StatusOrIdentityLostUnderFallback,
}

impl StatusContinuityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::IdentityBreadcrumbsStatusPreserved => "identity_breadcrumbs_status_preserved",
            Self::DisclosedStatusRelocation => "disclosed_status_relocation",
            Self::StatusOrIdentityLostUnderFallback => "status_or_identity_lost_under_fallback",
        }
    }

    /// `true` when identity, breadcrumbs, and status fully survive the fallback.
    pub const fn is_full(self) -> bool {
        matches!(self, Self::IdentityBreadcrumbsStatusPreserved)
    }

    /// `true` when recovery-critical status took a disclosed relocation.
    pub const fn is_disclosed_narrowing(self) -> bool {
        matches!(self, Self::DisclosedStatusRelocation)
    }
}

/// A safe compare-fallback strategy a surface may use when a side-by-side split would
/// violate the minimum useful size.
///
/// The strategies are ordered by the width they require (see [`Self::required_width_rank`]):
/// a real side-by-side split needs the most width, tabbed compare less, staged peek and
/// sequential disclosure less still, and explicit user choice the least — a surface can
/// always fall back to sequential disclosure or explicit user choice regardless of how
/// narrow it becomes, so those two are the universal safe terminals.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CompareFallbackStrategy {
    /// A real side-by-side split; needs the most width.
    SideBySideSplit,
    /// Compare targets share one pane as tabs.
    TabbedCompare,
    /// The secondary target opens as a staged, dismissable peek over the primary.
    StagedPeek,
    /// The targets are disclosed one at a time in a sequence.
    SequentialDisclosure,
    /// The surface explicitly asks the user which target to show; always available.
    ExplicitUserChoice,
}

impl M5CompareFallbackStrategy {
    /// Every strategy, in declaration (widest-to-narrowest) order.
    pub const ALL: [Self; 5] = [
        Self::SideBySideSplit,
        Self::TabbedCompare,
        Self::StagedPeek,
        Self::SequentialDisclosure,
        Self::ExplicitUserChoice,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SideBySideSplit => "side_by_side_split",
            Self::TabbedCompare => "tabbed_compare",
            Self::StagedPeek => "staged_peek",
            Self::SequentialDisclosure => "sequential_disclosure",
            Self::ExplicitUserChoice => "explicit_user_choice",
        }
    }

    /// Relative width the strategy requires. Higher needs more room; a strategy with
    /// rank `<= 1` is a universal safe terminal that fits at any usable width.
    pub const fn required_width_rank(self) -> u8 {
        match self {
            Self::SideBySideSplit => 4,
            Self::TabbedCompare => 3,
            Self::StagedPeek => 2,
            Self::SequentialDisclosure => 1,
            Self::ExplicitUserChoice => 0,
        }
    }

    /// `true` when the strategy fits at any usable width and can safely terminate a
    /// declared fallback set.
    pub const fn is_safe_terminal(self) -> bool {
        self.required_width_rank() <= 1
    }
}

/// The compare plan a family lands in at one responsive class.
///
/// The strategy must be a member of the family's declared safe-fallback set, both
/// `meets_min_useful_size` and `identity_and_status_preserved` must hold, and a
/// narrower responsive class may never require a strategy that needs more width than a
/// wider one; a false value, an undeclared strategy, or a non-monotonic plan is a
/// blocker.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinWidthClassPlan {
    /// The responsive class this plan describes.
    pub responsive_class: M5ResponsiveClass,
    /// The compare-fallback strategy the surface lands in at this class.
    pub strategy: M5CompareFallbackStrategy,
    /// `true` when the surface keeps its minimum useful pane size at this class.
    pub meets_min_useful_size: bool,
    /// `true` when identity, breadcrumbs, and recovery-critical status stay visible.
    pub identity_and_status_preserved: bool,
}

impl MinWidthClassPlan {
    /// `true` when the plan meets the minimum and preserves identity and status.
    pub const fn is_stable(&self) -> bool {
        self.meets_min_useful_size && self.identity_and_status_preserved
    }
}

/// Short, reviewer-facing label for a governed family's guarded surface.
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
/// (yellow) rather than blocked — never lets a pane below the usable minimum, a silent
/// unusable split, or a lost identity/status hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinWidthGuardWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed family the waiver applies to.
    pub family: M5ShellSurfaceFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl MinWidthGuardWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed family's min-width claim.
///
/// The trigger token mirrors the frozen [`M5ShellDowngradeTrigger`] vocabulary so a
/// cause never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinWidthGuardCause {
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

impl MinWidthGuardCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed surface family, certified across its min-size-enforcement,
/// compare-fallback, and status-continuity posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinWidthGuardRow {
    /// The governed family being certified.
    pub family: M5ShellSurfaceFamily,
    /// The family's frozen qualification class from the shell-zone matrix.
    pub matrix_qualification: M5ShellQualificationClass,
    /// Owner role accountable for keeping this family governed.
    pub owner_role: String,
    /// Short guarded-surface label.
    pub surface_label: String,
    /// Canonical shell slot from the matrix.
    pub canonical_slot: M5ShellZoneSlot,
    /// Declared fallback slot from the matrix.
    pub fallback_slot: M5ShellZoneSlot,
    /// Responsive classes this family must survive. Pulled from the matrix.
    pub declared_responsive_classes: Vec<M5ResponsiveClass>,
    /// Occupant transitions this family may take. Pulled from the matrix.
    pub declared_occupant_persistence: Vec<M5OccupantPersistence>,
    /// Declared safe compare-fallback strategies, widest-to-narrowest; terminates in a
    /// universal safe strategy. Derived from the matrix occupant transitions.
    pub declared_strategies: Vec<M5CompareFallbackStrategy>,
    /// The primary safe fallback strategy the surface uses when a split would violate
    /// the minimum useful size. Must be one of `declared_strategies`.
    pub compare_fallback_strategy: M5CompareFallbackStrategy,
    /// Minimum useful editor / primary pane width the family enforces, in pixels.
    pub min_useful_width_px: u32,
    /// Minimum useful editor / primary pane height the family enforces, in pixels.
    pub min_useful_height_px: u32,
    /// Per-class compare plan, one per declared responsive class.
    pub class_plans: Vec<MinWidthClassPlan>,
    /// Min-size-enforcement posture.
    pub min_size_enforcement: MinSizeEnforcementState,
    /// Compare-fallback posture.
    pub compare_fallback: CompareFallbackState,
    /// Breadcrumb / identity / recovery-status continuity posture.
    pub status_continuity: StatusContinuityState,
    /// Consumer surfaces this family must stay aligned across. Pulled from the matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this family. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ShellDowngradeTrigger>,
    /// Active waiver, when a disclosed status-relocation narrowing is in force.
    pub active_waiver: Option<MinWidthGuardWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: MinWidthGuardStatus,
    /// The exact guard causes that narrowed or blocked this row.
    pub guard_causes: Vec<MinWidthGuardCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl MinWidthGuardRow {
    /// `true` when this family's frozen qualification keeps a Stable claim.
    pub fn is_stable_qualified(&self) -> bool {
        self.matrix_qualification.is_stable()
    }

    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the declared safe-fallback set terminates in a universally-available
    /// safe strategy — the guarantee that a surface always has a safe compare mode no
    /// matter how narrow it becomes.
    pub fn strategy_set_has_safe_terminal(&self) -> bool {
        matches!(
            self.declared_strategies.last(),
            Some(strategy) if strategy.is_safe_terminal()
        )
    }

    /// `true` when the declared safe-fallback set is ordered widest-to-narrowest, so a
    /// narrower fallback is always available below a wider one.
    pub fn strategy_set_is_ordered(&self) -> bool {
        self.declared_strategies
            .windows(2)
            .all(|pair| pair[0].required_width_rank() > pair[1].required_width_rank())
    }

    /// `true` when the primary fallback strategy is one the family declared.
    pub fn primary_strategy_declared(&self) -> bool {
        self.declared_strategies
            .contains(&self.compare_fallback_strategy)
    }

    /// `true` when the per-class plans cover exactly the declared responsive classes —
    /// no class the family must survive is left uncertified and none is invented.
    pub fn plans_cover_declared_classes(&self) -> bool {
        let declared: BTreeSet<M5ResponsiveClass> =
            self.declared_responsive_classes.iter().copied().collect();
        let present: BTreeSet<M5ResponsiveClass> = self
            .class_plans
            .iter()
            .map(|plan| plan.responsive_class)
            .collect();
        declared == present && present.len() == self.class_plans.len()
    }

    /// `true` when every plan lands in a strategy the family declared — the lint that
    /// prevents a compare plan from using an undeclared strategy.
    pub fn plans_strategies_declared(&self) -> bool {
        self.class_plans
            .iter()
            .all(|plan| self.declared_strategies.contains(&plan.strategy))
    }

    /// `true` when the plans are monotonic: the compact class uses a strategy that
    /// needs at most as much width as standard, which needs at most as much as
    /// expanded.
    pub fn plans_monotonic(&self) -> bool {
        let rank_for = |class: M5ResponsiveClass| {
            self.class_plans
                .iter()
                .find(|plan| plan.responsive_class == class)
                .map(|plan| plan.strategy.required_width_rank())
        };
        match (
            rank_for(M5ResponsiveClass::CompactDesktop),
            rank_for(M5ResponsiveClass::StandardDesktop),
            rank_for(M5ResponsiveClass::ExpandedDesktop),
        ) {
            (Some(compact), Some(standard), Some(expanded)) => {
                compact <= standard && standard <= expanded
            }
            // A family that does not declare all three classes is checked by the
            // coverage lint instead.
            _ => true,
        }
    }

    /// `true` when every plan meets the minimum useful size and preserves identity and
    /// status.
    pub fn plans_stable(&self) -> bool {
        self.class_plans.iter().all(MinWidthClassPlan::is_stable)
    }

    /// `true` when the declared minimum useful size falls below the floor its
    /// enforcement posture claims. An enforced pane must reserve at least the standard
    /// minimum; a disclosed reduced minimum may narrow to the absolute floor but never
    /// below it.
    pub fn min_size_below_floor(&self) -> bool {
        match self.min_size_enforcement {
            MinSizeEnforcementState::MinUsefulSizeEnforced => {
                self.min_useful_width_px < STANDARD_MIN_USEFUL_WIDTH_PX
                    || self.min_useful_height_px < STANDARD_MIN_USEFUL_HEIGHT_PX
            }
            MinSizeEnforcementState::DisclosedReducedMinimum => {
                self.min_useful_width_px < ABSOLUTE_MIN_USEFUL_WIDTH_PX
                    || self.min_useful_height_px < ABSOLUTE_MIN_USEFUL_HEIGHT_PX
            }
            // Already a hard blocker via the enforcement state; the pixel floor is
            // not re-asserted for a pane that is honestly reported as unusable.
            MinSizeEnforcementState::PaneForcedBelowUsableMinimum => false,
        }
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        if !self.strategy_set_is_ordered() || !self.strategy_set_has_safe_terminal() {
            return true;
        }
        if !self.primary_strategy_declared() {
            return true;
        }
        if matches!(
            self.min_size_enforcement,
            MinSizeEnforcementState::PaneForcedBelowUsableMinimum
        ) {
            return true;
        }
        if matches!(self.compare_fallback, CompareFallbackState::SilentUnusableSplit) {
            return true;
        }
        if matches!(
            self.status_continuity,
            StatusContinuityState::StatusOrIdentityLostUnderFallback
        ) {
            return true;
        }
        if self.min_size_below_floor() {
            return true;
        }
        if !self.plans_cover_declared_classes()
            || !self.plans_strategies_declared()
            || !self.plans_monotonic()
            || !self.plans_stable()
        {
            return true;
        }
        false
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        !self.is_stable_qualified()
            || self.min_size_enforcement.is_disclosed_narrowing()
            || self.compare_fallback.is_disclosed_narrowing()
            || self.status_continuity.is_disclosed_narrowing()
    }

    /// Recomputes the derived status from the guard posture.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest
    /// narrowing forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> MinWidthGuardStatus {
        if self.has_hard_blocker() {
            MinWidthGuardStatus::Red
        } else if self.has_narrowing() {
            MinWidthGuardStatus::Yellow
        } else {
            MinWidthGuardStatus::Green
        }
    }

    /// Recomputes the exact guard causes for the row, in deterministic order
    /// (qualification, strategy set, min size, compare fallback, status continuity).
    pub fn recompute_causes(&self) -> Vec<MinWidthGuardCause> {
        let mut causes = Vec::new();
        if !self.is_stable_qualified() {
            causes.push(MinWidthGuardCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                disclosed: true,
                detail: format!(
                    "Frozen shell-zone matrix qualifies this family at `{}`, below a Stable shell claim.",
                    self.matrix_qualification.as_str()
                ),
            });
        }
        if !self.strategy_set_is_ordered() || !self.strategy_set_has_safe_terminal() {
            causes.push(MinWidthGuardCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::PlaceholderLostIdentityOrReopen,
                disclosed: false,
                detail: "The declared safe-fallback set is not ordered widest-to-narrowest or does \
                         not terminate in a universally-available safe compare mode."
                    .to_owned(),
            });
        }
        if !self.primary_strategy_declared() {
            causes.push(MinWidthGuardCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::PlaceholderLostIdentityOrReopen,
                disclosed: false,
                detail: "The primary compare-fallback strategy is not a member of the declared \
                         safe-fallback set."
                    .to_owned(),
            });
        }
        match self.min_size_enforcement {
            MinSizeEnforcementState::MinUsefulSizeEnforced => {}
            MinSizeEnforcementState::DisclosedReducedMinimum => causes.push(MinWidthGuardCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                disclosed: true,
                detail: "Under compact width the editor discloses a reduced-but-still-usable minimum \
                         useful size while staying above the absolute floor."
                    .to_owned(),
            }),
            MinSizeEnforcementState::PaneForcedBelowUsableMinimum => causes.push(MinWidthGuardCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::CriticalStateHiddenOnCollapse,
                disclosed: false,
                detail: "The editor or compare pane can be forced below a usable minimum size, \
                         producing a silent unusable narrow pane."
                    .to_owned(),
            }),
        }
        if self.min_size_below_floor() {
            causes.push(MinWidthGuardCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::CriticalStateHiddenOnCollapse,
                disclosed: false,
                detail: "The declared minimum useful size falls below the absolute floor its \
                         enforcement posture claims."
                    .to_owned(),
            });
        }
        match self.compare_fallback {
            CompareFallbackState::SafeFallbackBeforeUnusableSplit => {}
            CompareFallbackState::DisclosedFallbackNarrowing => causes.push(MinWidthGuardCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                disclosed: true,
                detail: "The compare fallback trims a secondary pane's optional detail in a \
                         disclosed way before an unusable split could occur."
                    .to_owned(),
            }),
            CompareFallbackState::SilentUnusableSplit => causes.push(MinWidthGuardCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::CriticalStateHiddenOnCollapse,
                disclosed: false,
                detail: "A compare or diff produced a silent unusable narrow split with no safe \
                         fallback."
                    .to_owned(),
            }),
        }
        match self.status_continuity {
            StatusContinuityState::IdentityBreadcrumbsStatusPreserved => {}
            StatusContinuityState::DisclosedStatusRelocation => causes.push(MinWidthGuardCause {
                family: self.family,
                trigger: M5ShellDowngradeTrigger::UpstreamDependencyNarrowed,
                disclosed: true,
                detail: "Recovery-critical status is relocated to a disclosed, waivered \
                         still-visible affordance while the compare fallback is active."
                    .to_owned(),
            }),
            StatusContinuityState::StatusOrIdentityLostUnderFallback => {
                causes.push(MinWidthGuardCause {
                    family: self.family,
                    trigger: M5ShellDowngradeTrigger::CollapseChangedTaskIdentity,
                    disclosed: false,
                    detail: "Breadcrumbs, active object identity, or recovery-critical status is \
                             lost while the compare fallback is active."
                        .to_owned(),
                });
            }
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed recovery-status relocation may only stay yellow (rather than red)
    /// when a waiver discloses it.
    pub fn requires_waiver(&self) -> bool {
        matches!(
            self.status_continuity,
            StatusContinuityState::DisclosedStatusRelocation
        )
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<MinWidthGuardFinding> {
        let mut findings = Vec::new();
        let family = self.family.as_str().to_owned();

        if !self.strategy_set_has_safe_terminal() {
            findings.push(MinWidthGuardFinding::StrategySetMissingSafeTerminal {
                family: family.clone(),
            });
        }
        if !self.strategy_set_is_ordered() {
            findings.push(MinWidthGuardFinding::StrategySetNotOrdered {
                family: family.clone(),
            });
        }
        if !self.primary_strategy_declared() {
            findings.push(MinWidthGuardFinding::PrimaryStrategyUndeclared {
                family: family.clone(),
            });
        }
        if matches!(
            self.min_size_enforcement,
            MinSizeEnforcementState::PaneForcedBelowUsableMinimum
        ) {
            findings.push(MinWidthGuardFinding::PaneForcedBelowUsableMinimum {
                family: family.clone(),
            });
        }
        if self.min_size_below_floor() {
            findings.push(MinWidthGuardFinding::MinSizeBelowFloor {
                family: family.clone(),
            });
        }
        if matches!(self.compare_fallback, CompareFallbackState::SilentUnusableSplit) {
            findings.push(MinWidthGuardFinding::SilentUnusableSplit {
                family: family.clone(),
            });
        }
        if matches!(
            self.status_continuity,
            StatusContinuityState::StatusOrIdentityLostUnderFallback
        ) {
            findings.push(MinWidthGuardFinding::StatusOrIdentityLostUnderFallback {
                family: family.clone(),
            });
        }
        if !self.plans_cover_declared_classes() {
            findings.push(MinWidthGuardFinding::PlanClassCoverageMismatch {
                family: family.clone(),
            });
        }
        if !self.plans_strategies_declared() {
            findings.push(MinWidthGuardFinding::PlanStrategyUndeclared {
                family: family.clone(),
            });
        }
        if !self.plans_monotonic() {
            findings.push(MinWidthGuardFinding::PlanLadderNonMonotonic {
                family: family.clone(),
            });
        }
        if !self.plans_stable() {
            findings.push(MinWidthGuardFinding::PlanMinSizeOrStatusLost {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, MinWidthGuardStatus::Green) && !self.has_reason() {
            findings.push(MinWidthGuardFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an
        // active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(MinWidthGuardFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.family != self.family {
                findings.push(MinWidthGuardFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(MinWidthGuardFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(MinWidthGuardFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.guard_causes != self.recompute_causes() {
            findings.push(MinWidthGuardFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} min_size={} fallback={} status_cont={} strategy={} min={}x{} waiver={}",
            self.family.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.min_size_enforcement.as_str(),
            self.compare_fallback.as_str(),
            self.status_continuity.as_str(),
            self.compare_fallback_strategy.as_str(),
            self.min_useful_width_px,
            self.min_useful_height_px,
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the min-width-guard proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum MinWidthGuardFinding {
    /// A governed surface family has no guard row.
    FamilyMissing {
        /// The missing family token.
        family: String,
    },
    /// A row's declared safe-fallback set does not terminate in a universal safe mode.
    StrategySetMissingSafeTerminal {
        /// The family token.
        family: String,
    },
    /// A row's declared safe-fallback set is not ordered widest-to-narrowest.
    StrategySetNotOrdered {
        /// The family token.
        family: String,
    },
    /// A row's primary fallback strategy is not in the declared safe-fallback set.
    PrimaryStrategyUndeclared {
        /// The family token.
        family: String,
    },
    /// A row's editor or compare pane can be forced below a usable minimum.
    PaneForcedBelowUsableMinimum {
        /// The family token.
        family: String,
    },
    /// A row's declared minimum useful size falls below the absolute floor.
    MinSizeBelowFloor {
        /// The family token.
        family: String,
    },
    /// A row produced a silent unusable narrow split with no fallback.
    SilentUnusableSplit {
        /// The family token.
        family: String,
    },
    /// A row lost breadcrumbs, identity, or recovery-critical status under fallback.
    StatusOrIdentityLostUnderFallback {
        /// The family token.
        family: String,
    },
    /// A row's per-class plans do not cover exactly the declared classes.
    PlanClassCoverageMismatch {
        /// The family token.
        family: String,
    },
    /// A row's plan lands in a strategy the family never declared.
    PlanStrategyUndeclared {
        /// The family token.
        family: String,
    },
    /// A row's plans are not monotonic across compact/standard/expanded.
    PlanLadderNonMonotonic {
        /// The family token.
        family: String,
    },
    /// A row's plan loses the minimum useful size or identity/status at some class.
    PlanMinSizeOrStatusLost {
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
    /// The declared guard causes do not match the recomputed causes.
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

impl MinWidthGuardFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::FamilyMissing { .. } => "family_missing",
            Self::StrategySetMissingSafeTerminal { .. } => "strategy_set_missing_safe_terminal",
            Self::StrategySetNotOrdered { .. } => "strategy_set_not_ordered",
            Self::PrimaryStrategyUndeclared { .. } => "primary_strategy_undeclared",
            Self::PaneForcedBelowUsableMinimum { .. } => "pane_forced_below_usable_minimum",
            Self::MinSizeBelowFloor { .. } => "min_size_below_floor",
            Self::SilentUnusableSplit { .. } => "silent_unusable_split",
            Self::StatusOrIdentityLostUnderFallback { .. } => {
                "status_or_identity_lost_under_fallback"
            }
            Self::PlanClassCoverageMismatch { .. } => "plan_class_coverage_mismatch",
            Self::PlanStrategyUndeclared { .. } => "plan_strategy_undeclared",
            Self::PlanLadderNonMonotonic { .. } => "plan_ladder_non_monotonic",
            Self::PlanMinSizeOrStatusLost { .. } => "plan_min_size_or_status_lost",
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
            | Self::StrategySetMissingSafeTerminal { family }
            | Self::StrategySetNotOrdered { family }
            | Self::PrimaryStrategyUndeclared { family }
            | Self::PaneForcedBelowUsableMinimum { family }
            | Self::MinSizeBelowFloor { family }
            | Self::SilentUnusableSplit { family }
            | Self::StatusOrIdentityLostUnderFallback { family }
            | Self::PlanClassCoverageMismatch { family }
            | Self::PlanStrategyUndeclared { family }
            | Self::PlanLadderNonMonotonic { family }
            | Self::PlanMinSizeOrStatusLost { family }
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

/// The release min-width-guard packet shared by the shell / windowing / layout /
/// release automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinWidthGuardPacket {
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
    /// Absolute minimum useful pane width floor, in pixels.
    pub absolute_min_useful_width_px: u32,
    /// Absolute minimum useful pane height floor, in pixels.
    pub absolute_min_useful_height_px: u32,
    /// Standard minimum useful pane width a fully-enforced pane must reserve.
    pub standard_min_useful_width_px: u32,
    /// Standard minimum useful pane height a fully-enforced pane must reserve.
    pub standard_min_useful_height_px: u32,
    /// Per-family guard rows, in canonical order.
    pub rows: Vec<MinWidthGuardRow>,
    /// Governed families certified, in canonical (sorted) order.
    pub covered_families: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green (fully-guarded) rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<MinWidthGuardWaiver>,
    /// Every exact guard cause, in row then cause order.
    pub guard_causes: Vec<MinWidthGuardCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<MinWidthGuardFinding>,
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
    /// Published guard-packet ref.
    pub published_packet_ref: String,
    /// Published guard-dashboard ref.
    pub published_dashboard_ref: String,
    /// Published companion doc ref.
    pub published_doc_ref: String,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

impl MinWidthGuardPacket {
    /// Returns the guard row for `family`, if present.
    pub fn row(&self, family: M5ShellSurfaceFamily) -> Option<&MinWidthGuardRow> {
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
                "matrix={} build={} channel={} floor={}x{} publishable={}",
                self.matrix_packet_ref,
                self.build_identity_ref,
                self.release_channel_class,
                self.absolute_min_useful_width_px,
                self.absolute_min_useful_height_px,
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
        for cause in &self.guard_causes {
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

    /// Projects the light guard dashboard the shell automation consumes.
    pub fn dashboard(&self) -> MinWidthGuardDashboard {
        MinWidthGuardDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 min-width-guards packet serializes")
    }

    /// Deterministic, machine-readable guard CSV: one row per family naming its status,
    /// qualification, min-size posture, fallback posture, strategy, minimum, and waiver.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "family,status,qualification,canonical_slot,fallback_slot,min_size_enforcement,min_useful_width_px,min_useful_height_px,compare_fallback,compare_fallback_strategy,compact_strategy,status_continuity,waiver\n",
        );
        for row in &self.rows {
            let compact_strategy = row
                .class_plans
                .iter()
                .find(|p| p.responsive_class == M5ResponsiveClass::CompactDesktop)
                .map(|p| p.strategy.as_str())
                .unwrap_or("none");
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.family.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.canonical_slot.as_str(),
                row.fallback_slot.as_str(),
                row.min_size_enforcement.as_str(),
                row.min_useful_width_px,
                row.min_useful_height_px,
                row.compare_fallback.as_str(),
                row.compare_fallback_strategy.as_str(),
                compact_strategy,
                row.status_continuity.as_str(),
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
        out.push_str("# M5 min-width guards: editor minimum, compare fallback, no unusable narrow pane\n\n");
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_min_width_guards`](../../crates/aureline-shell/src/m5_min_width_guards/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_min_width_guards -- markdown > \\\n  artifacts/shell/m5-min-width-guards.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Packet id: `{}`\n", self.packet_id));
        out.push_str(&format!("- Source schema ref: `{}`\n", self.source_schema_ref));
        out.push_str(&format!(
            "- Certifies matrix packet: `{}`\n",
            self.matrix_packet_ref
        ));
        out.push_str(&format!("- Exact build: `{}`\n", self.build_identity_ref));
        out.push_str(&format!("- Release channel: `{}`\n", self.release_channel_class));
        out.push_str(&format!(
            "- Absolute min useful size: `{}x{}` px\n",
            self.absolute_min_useful_width_px, self.absolute_min_useful_height_px
        ));
        out.push_str(&format!(
            "- Standard min useful size: `{}x{}` px\n",
            self.standard_min_useful_width_px, self.standard_min_useful_height_px
        ));
        out.push_str(&format!("- Rows certified: {}\n", self.row_count));
        out.push_str(&format!("- Green (fully-guarded): {}\n", self.green_row_count));
        out.push_str(&format!("- Yellow (auto-narrowed): {}\n", self.yellow_row_count));
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
            if self.report_clean { "clean" } else { "blocked" }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Guard rows\n\n");
        out.push_str(
            "| Surface | Status | Qualification | Min size | Enforcement | Fallback | Strategy | Status continuity | Waiver |\n\
             | ------- | ------ | ------------- | -------- | ----------- | -------- | -------- | ----------------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}x{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.surface_label,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.min_useful_width_px,
                row.min_useful_height_px,
                row.min_size_enforcement.as_str(),
                row.compare_fallback.as_str(),
                row.compare_fallback_strategy.as_str(),
                row.status_continuity.as_str(),
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Per-class compare plan\n\n");
        out.push_str(
            "| Surface | Compact | Standard | Expanded |\n\
             | ------- | ------- | -------- | -------- |\n",
        );
        for row in &self.rows {
            let strategy = |class: M5ResponsiveClass| {
                row.class_plans
                    .iter()
                    .find(|p| p.responsive_class == class)
                    .map(|p| p.strategy.as_str())
                    .unwrap_or("—")
            };
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` |\n",
                row.surface_label,
                strategy(M5ResponsiveClass::CompactDesktop),
                strategy(M5ResponsiveClass::StandardDesktop),
                strategy(M5ResponsiveClass::ExpandedDesktop),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&MinWidthGuardRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, MinWidthGuardStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every governed family enforces a minimum useful size and falls back safely.\n\n",
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

        out.push_str("## Exact guard causes\n\n");
        if self.guard_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.guard_causes {
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_min_width_guards -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_min_width_guards_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light guard dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinWidthGuardDashboardRow {
    /// The governed family.
    pub family: M5ShellSurfaceFamily,
    /// Short guarded-surface label.
    pub surface_label: String,
    /// Derived green/yellow/red status.
    pub status: MinWidthGuardStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5ShellQualificationClass,
    /// Minimum useful width the family enforces.
    pub min_useful_width_px: u32,
    /// Minimum useful height the family enforces.
    pub min_useful_height_px: u32,
    /// The compare-fallback strategy the surface lands in at compact width.
    pub compact_strategy: Option<M5CompareFallbackStrategy>,
    /// Min-size-enforcement posture.
    pub min_size_enforcement: MinSizeEnforcementState,
    /// Compare-fallback posture.
    pub compare_fallback: CompareFallbackState,
    /// Status-continuity posture.
    pub status_continuity: StatusContinuityState,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light guard dashboard the shell / windowing / layout / release automation reads
/// to auto-narrow claimed surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinWidthGuardDashboard {
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
    pub rows: Vec<MinWidthGuardDashboardRow>,
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

impl MinWidthGuardDashboard {
    /// Projects the dashboard from a guard packet.
    pub fn from_packet(packet: &MinWidthGuardPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| MinWidthGuardDashboardRow {
                family: row.family,
                surface_label: row.surface_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                min_useful_width_px: row.min_useful_width_px,
                min_useful_height_px: row.min_useful_height_px,
                compact_strategy: row
                    .class_plans
                    .iter()
                    .find(|p| p.responsive_class == M5ResponsiveClass::CompactDesktop)
                    .map(|p| p.strategy),
                min_size_enforcement: row.min_size_enforcement,
                compare_fallback: row.compare_fallback,
                status_continuity: row.status_continuity,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .guard_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_MIN_WIDTH_GUARDS_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_MIN_WIDTH_GUARDS_SCHEMA_VERSION,
            dashboard_id: M5_MIN_WIDTH_GUARDS_DASHBOARD_ID.to_owned(),
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
        serde_json::to_string_pretty(self).expect("m5 min-width-guards dashboard serializes")
    }
}

/// Support-export wrapper for the min-width-guard packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MinWidthGuardSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: MinWidthGuardPacket,
    /// Dashboard quoted in full.
    pub dashboard: MinWidthGuardDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl MinWidthGuardSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each family, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the shell
    /// automation — can name the same family and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: MinWidthGuardPacket,
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
            record_kind: M5_MIN_WIDTH_GUARDS_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_MIN_WIDTH_GUARDS_SCHEMA_VERSION,
            shared_contract_ref: M5_MIN_WIDTH_GUARDS_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_min_width_guards_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MinWidthGuardInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-zone matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-family guard rows.
    pub rows: Vec<MinWidthGuardRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
///
/// The guard packet carries only closed vocabulary, refs, pixel floors, and short
/// labels, so raw URLs, credentials, or tokens must never appear.
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

/// Builds a [`MinWidthGuardPacket`] from the exact build identity, the frozen matrix
/// ref, and the per-family guard rows.
///
/// Each row's derived status and guard causes, the aggregate counts, the active
/// waivers, and the blocking findings are recomputed here so the packet is the single
/// source of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_min_width_guards_packet(input: MinWidthGuardInput) -> MinWidthGuardPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent
    // and the auto-narrowing is the single source of truth.
    let rows: Vec<MinWidthGuardRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.guard_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<MinWidthGuardFinding> = Vec::new();

    // Every governed family must carry a guard row.
    let present: BTreeSet<M5ShellSurfaceFamily> = rows.iter().map(|row| row.family).collect();
    for family in REQUIRED_FAMILIES {
        if !present.contains(&family) {
            blocking_findings.push(MinWidthGuardFinding::FamilyMissing {
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
        .filter(|row| matches!(row.derived_status, MinWidthGuardStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, MinWidthGuardStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, MinWidthGuardStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(MinWidthGuardFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<MinWidthGuardWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let guard_causes: Vec<MinWidthGuardCause> = rows
        .iter()
        .flat_map(|row| row.guard_causes.clone())
        .collect();

    let mut packet = MinWidthGuardPacket {
        record_kind: M5_MIN_WIDTH_GUARDS_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_MIN_WIDTH_GUARDS_SCHEMA_VERSION,
        shared_contract_ref: M5_MIN_WIDTH_GUARDS_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_MIN_WIDTH_GUARDS_PACKET_ID.to_owned(),
        source_schema_ref: M5_MIN_WIDTH_GUARDS_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Editor-minimum, compare-fallback, and no-unusable-narrow-pane guards for every \
                   claimed M5 surface family: notebook, data grid, profiler, pipeline, docs, \
                   preview, review, incident, companion, and operator each certified to enforce a \
                   minimum useful editor width and height, fall back to a declared safe compare mode \
                   (tabbed compare, staged peek, sequential disclosure, or explicit user choice) \
                   before ever producing an unusable narrow split, and keep breadcrumbs, active \
                   object identity, and recovery-critical status visible while the fallback is \
                   active, with each row's green/yellow/red claim auto-narrowed from its \
                   min-size-enforcement, compare-fallback, and status-continuity posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_MIN_WIDTH_GUARDS_MATRIX_SCHEMA_REF.to_owned(),
        responsive_class_schema_ref: M5_MIN_WIDTH_GUARDS_RESPONSIVE_CLASS_SCHEMA_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        absolute_min_useful_width_px: ABSOLUTE_MIN_USEFUL_WIDTH_PX,
        absolute_min_useful_height_px: ABSOLUTE_MIN_USEFUL_HEIGHT_PX,
        standard_min_useful_width_px: STANDARD_MIN_USEFUL_WIDTH_PX,
        standard_min_useful_height_px: STANDARD_MIN_USEFUL_HEIGHT_PX,
        rows,
        covered_families,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        guard_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        shell_automation_refs: vec![
            "shell_frame.min_width_guard_registry".to_owned(),
            "release_automation.auto_narrow.min_width_guard_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.min_width_guards".to_owned(),
            M5_MIN_WIDTH_GUARDS_PUBLISHED_PACKET_REF.to_owned(),
        ],
        help_docs_refs: vec![M5_MIN_WIDTH_GUARDS_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-min-width-guards".to_owned()],
        published_report_ref: M5_MIN_WIDTH_GUARDS_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_MIN_WIDTH_GUARDS_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_MIN_WIDTH_GUARDS_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_MIN_WIDTH_GUARDS_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("guard packet serializes"),
    ) {
        blocking_findings.push(MinWidthGuardFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_min_width_guards_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum MinWidthGuardValidationError {
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
    /// The declared pixel floors do not match the lane constants.
    FloorConstantsStale,
    /// The rows do not cover all ten governed families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared guard causes do not match the recomputed causes.
    GuardCausesStale,
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

/// Validates a packet against the min-width-guard invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed
/// family carries a current guard row; each row's status is the derived auto-narrowed
/// value, never asserted; a green row cannot keep a claim while its editor can be
/// forced below a usable minimum, its compare produces a silent unusable split, its
/// breadcrumbs / identity / recovery-critical status is lost under fallback, its
/// safe-fallback set has no universal terminal, its primary strategy is undeclared, or
/// a per-class plan lands outside the declared set; and a disclosed narrowing is backed
/// by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_min_width_guards_packet(
    packet: &MinWidthGuardPacket,
) -> Result<(), Vec<MinWidthGuardValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(MinWidthGuardValidationError::NoRows);
    }
    if packet.record_kind != M5_MIN_WIDTH_GUARDS_PACKET_RECORD_KIND {
        errors.push(MinWidthGuardValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_MIN_WIDTH_GUARDS_SCHEMA_VERSION {
        errors.push(MinWidthGuardValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(MinWidthGuardValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(MinWidthGuardValidationError::MatrixPacketRefMissing);
    }
    if packet.absolute_min_useful_width_px != ABSOLUTE_MIN_USEFUL_WIDTH_PX
        || packet.absolute_min_useful_height_px != ABSOLUTE_MIN_USEFUL_HEIGHT_PX
        || packet.standard_min_useful_width_px != STANDARD_MIN_USEFUL_WIDTH_PX
        || packet.standard_min_useful_height_px != STANDARD_MIN_USEFUL_HEIGHT_PX
    {
        errors.push(MinWidthGuardValidationError::FloorConstantsStale);
    }

    let present: BTreeSet<M5ShellSurfaceFamily> =
        packet.rows.iter().map(|row| row.family).collect();
    let coverage_complete = REQUIRED_FAMILIES.iter().all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != REQUIRED_FAMILIES.len() {
        errors.push(MinWidthGuardValidationError::CoverageIncomplete);
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
        errors.push(MinWidthGuardValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), MinWidthGuardStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), MinWidthGuardStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), MinWidthGuardStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(MinWidthGuardValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<MinWidthGuardWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(MinWidthGuardValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<MinWidthGuardCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.guard_causes {
        errors.push(MinWidthGuardValidationError::GuardCausesStale);
    }

    let mut recomputed: Vec<MinWidthGuardFinding> = Vec::new();
    for family in REQUIRED_FAMILIES {
        if !present.contains(&family) {
            recomputed.push(MinWidthGuardFinding::FamilyMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(MinWidthGuardFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("guard packet serializes"),
    ) {
        recomputed.push(MinWidthGuardFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(MinWidthGuardValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(MinWidthGuardValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(MinWidthGuardValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(MinWidthGuardValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(MinWidthGuardValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(MinWidthGuardValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
