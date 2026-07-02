//! Ambient shell-instrumentation stability certified across every claimed M5 rendering
//! profile so counters, spinners, and multi-job summaries stay legible, compact, and
//! palette-searchable.
//!
//! The [frozen shell-primitives matrix][matrix] already freezes the ambient-instrumentation
//! primitives — the status-bar item, the status overflow menu, and the ambient progress
//! indicator — into one export-safe packet: their status-item classes, overflow behaviors,
//! source/provider/freshness labels, accessibility routes, the mandatory labels every
//! ambient surface must be able to show, and the downgrade triggers that narrow them below
//! a claim. This lane is the **ambient-instrumentation stability certification capstone** on
//! top of that matrix: for every claimed M5 rendering profile — standard, compact, expanded,
//! multi-window, high-zoom, reduced-motion, degraded-network, and degraded-power — it
//! certifies that counters, spinners, and multi-job summaries stay stable and never reflow
//! or flicker the status/header strip when many jobs or counters update at once; that
//! overflowed ambient items stay searchable from the command palette or status menus with
//! the same labels and explanations used in the visible instrumentation; that multiple
//! active jobs, repeated updates, and quick state changes group into one meaningful summary
//! rather than many flickering primitives; and that this ambient-stability behavior is
//! reconstructable from reusable fixtures and a support export rather than ad hoc visual
//! checks alone.
//!
//! Three records carry the truth:
//!
//! - the per-profile **certification row** ([`AmbientStabilityRow`]): one row per
//!   [`M5AmbientStabilityProfile`] naming the ambient primitives it drives, the status-item
//!   classes / overflow behaviors / source-freshness labels / required labels / accessibility
//!   routes / consumer surfaces / downgrade triggers pulled from the frozen matrix, its
//!   counter-stability / overflow-searchability / grouped-summary / stability-export posture,
//!   any active waiver, and a derived green/yellow/red [`AmbientStabilityStatus`].
//! - the release **certification packet** ([`AmbientStabilityPacket`]): the full set of rows
//!   with derived per-row status, aggregate green/yellow/red counts, the active waivers, the
//!   exact certification causes ([`AmbientStabilityCause`]), and the blocking findings the
//!   lane refuses to ship with.
//! - the **certification dashboard** ([`AmbientStabilityDashboard`]): a light projection the
//!   shell / status bar / release automation reads to auto-narrow a claimed profile when its
//!   ambient-stability proof falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the
//! moment it discloses a reduced counter detail, a reduced overflow-search detail, a coarse
//! grouping (backed by a waiver), or a partial support-export capture; it drops to `red` if
//! the status strip reflows or flickers when counters update, an overflowed item is
//! undiscoverable or relabeled away from the palette/status search, many flickering
//! primitives are shown instead of one grouped summary, the ambient-stability state is
//! absent from the support-export capture, the status bar reflows around a vanity item, or
//! its status-item classes / overflow behaviors / required labels are incomplete. That
//! derivation is the auto-narrowing the acceptance criteria require.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs,
//! raw local paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids,
//! closed vocabulary, counts, refs, and short labels. The status-item-class, overflow-behavior,
//! source-freshness, accessibility-route, required-label, consumer-surface, downgrade-trigger,
//! and qualification vocabulary is re-exported by reference from the already frozen [matrix];
//! each row pulls its ambient bindings straight from that matrix's seeded status-bar-item,
//! status-overflow-menu, and progress-indicator rows, so this lane mints no parallel shell
//! vocabulary and cannot certify an ambient-stability posture the matrix does not freeze.
//! Only the certification-specific vocabulary ([`M5AmbientStabilityProfile`],
//! [`M5AmbientStabilityProofDimension`], [`AmbientStabilityStatus`],
//! [`CounterSpinnerStabilityState`], [`OverflowSearchabilityState`], [`GroupedSummaryState`],
//! [`StabilityExportState`], [`AmbientStabilityWaiver`], [`AmbientStabilityCause`],
//! [`AmbientStabilityFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix as matrix;

pub use matrix::{
    M5AccessibilityRoute, M5OverflowBehavior, M5PrimitiveQualificationClass,
    M5PrimitiveRequiredLabel, M5ShellConsumerSurface, M5ShellPrimitiveDowngradeTrigger,
    M5ShellPrimitiveFamily, M5ShellZoneSlot, M5SourceFreshnessLabel, M5StatusItemClass,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_ambient_instrumentation_stability_packet,
    seeded_m5_ambient_instrumentation_stability_packet_compact_status_reflow_blocked,
    seeded_m5_ambient_instrumentation_stability_packet_degraded_network_export_absent_blocked,
    seeded_m5_ambient_instrumentation_stability_packet_expanded_overflow_undiscoverable_blocked,
    seeded_m5_ambient_instrumentation_stability_packet_high_zoom_vanity_reflow_blocked,
    seeded_m5_ambient_instrumentation_stability_packet_multi_window_flickering_primitives_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_SHARED_CONTRACT_REF: &str =
    "shell:m5_ambient_instrumentation_stability:v1";

/// Stable record kind for [`AmbientStabilityPacket`] payloads.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_PACKET_RECORD_KIND: &str =
    "shell_m5_ambient_instrumentation_stability_packet_record";

/// Stable record kind for [`AmbientStabilityDashboard`] payloads.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_ambient_instrumentation_stability_dashboard_record";

/// Stable record kind for [`AmbientStabilitySupportExport`] payloads.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_ambient_instrumentation_stability_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_PACKET_ID: &str =
    "m5-ambient-instrumentation-stability:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_DASHBOARD_ID: &str =
    "m5-ambient-instrumentation-stability-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-ambient-instrumentation-stability:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-ambient-instrumentation-stability.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-ambient-instrumentation-stability.md";

/// Published certification-packet artifact ref.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-ambient-instrumentation-stability-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-ambient-instrumentation-stability-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-ambient-instrumentation-stability-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-ambient-instrumentation-stability-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_ambient_instrumentation_stability_contract.md";

/// Repo-relative ref to the frozen shell-primitives matrix schema.
pub const M5_AMBIENT_INSTRUMENTATION_STABILITY_MATRIX_SCHEMA_REF: &str =
    matrix::M5_SHELL_PRIMITIVES_SCHEMA_REF;

/// The six labels every ambient-instrumentation surface must be able to show. The union
/// across the status-bar item, the status overflow menu, and the progress indicator covers
/// the full [`M5PrimitiveRequiredLabel::ALL`] set: identity, state, keyboard route, source
/// provider, freshness, and reopen path (counters/spinners label source and freshness; the
/// overflow menu carries a reopen path).
pub const AMBIENT_STABILITY_REQUIRED_LABELS: [M5PrimitiveRequiredLabel; 6] =
    M5PrimitiveRequiredLabel::ALL;

/// One of the claimed M5 rendering profiles the ambient-stability proof must cover, in
/// canonical order. Each profile is a claimed M5 shell rendering condition under which the
/// ambient instrumentation must stay legible, compact, overflow-safe, and searchable; the
/// lane certifies none beyond them and refuses to ship if any is missing. The compact,
/// high-zoom, reduced-motion, degraded-network, and degraded-power profiles are exactly the
/// fixture-coverage cases the acceptance criteria require.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AmbientStabilityProfile {
    /// Standard desktop density (the normal case).
    Standard,
    /// Compact / reduced-width density.
    Compact,
    /// Expanded / wide density.
    Expanded,
    /// Multi-window / detached shell.
    MultiWindow,
    /// High-zoom / large-text rendering.
    HighZoom,
    /// Reduced-motion rendering.
    ReducedMotion,
    /// Degraded-network conditions.
    DegradedNetwork,
    /// Degraded-power / low-power conditions.
    DegradedPower,
}

impl M5AmbientStabilityProfile {
    /// Every governed rendering profile, in canonical order.
    pub const ALL: [Self; 8] = [
        Self::Standard,
        Self::Compact,
        Self::Expanded,
        Self::MultiWindow,
        Self::HighZoom,
        Self::ReducedMotion,
        Self::DegradedNetwork,
        Self::DegradedPower,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Compact => "compact",
            Self::Expanded => "expanded",
            Self::MultiWindow => "multi_window",
            Self::HighZoom => "high_zoom",
            Self::ReducedMotion => "reduced_motion",
            Self::DegradedNetwork => "degraded_network",
            Self::DegradedPower => "degraded_power",
        }
    }

    /// Short, reviewer-facing label for the profile.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Standard => "Standard desktop density",
            Self::Compact => "Compact / reduced-width density",
            Self::Expanded => "Expanded / wide density",
            Self::MultiWindow => "Multi-window / detached shell",
            Self::HighZoom => "High-zoom / large-text rendering",
            Self::ReducedMotion => "Reduced-motion rendering",
            Self::DegradedNetwork => "Degraded-network conditions",
            Self::DegradedPower => "Degraded-power / low-power conditions",
        }
    }
}

/// One of the four certification dimensions each rendering profile is certified across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AmbientStabilityProofDimension {
    /// Counter / spinner / summary stability (no status reflow or flicker on update).
    CounterStability,
    /// Overflow searchability (overflowed items stay palette/status searchable with the
    /// same labels).
    OverflowSearchability,
    /// Grouped summary (many active jobs group into one summary, not many flickering
    /// primitives).
    GroupedSummary,
    /// Stability export (the ambient-stability state is reconstructable from fixtures and a
    /// support export).
    StabilityExport,
}

impl M5AmbientStabilityProofDimension {
    /// Every certification dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CounterStability,
        Self::OverflowSearchability,
        Self::GroupedSummary,
        Self::StabilityExport,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CounterStability => "counter_stability",
            Self::OverflowSearchability => "overflow_searchability",
            Self::GroupedSummary => "grouped_summary",
            Self::StabilityExport => "stability_export",
        }
    }
}

/// The derived certification light a governed rendering profile carries.
///
/// `green` means the profile's counters, spinners, and multi-job summaries stay stable
/// without reflowing or flickering the status strip, its overflowed items stay
/// palette/status searchable with the same labels, it groups many active jobs into one
/// meaningful summary, and its ambient-stability behavior reconstructs from fixtures and a
/// support export. `yellow` is a disclosed narrowing (a reduced counter detail, a reduced
/// overflow-search detail, a waivered coarse grouping, or a partial support-export capture).
/// `red` is blocked: the status strip reflows or flickers when counters update, an
/// overflowed item is undiscoverable or relabeled, many flickering primitives are shown
/// instead of a grouped summary, the ambient-stability state is absent from capture, the
/// status bar reflows around a vanity item, or its status-item classes / overflow behaviors
/// / required labels are incomplete — and the profile may not keep a shell-maturity claim
/// until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AmbientStabilityStatus {
    /// Full standing: stable, searchable, grouped, reconstructable.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl AmbientStabilityStatus {
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

/// How the profile keeps counters, spinners, and multi-job summaries stable without
/// reflowing or flickering the status / header strip when many jobs or counters update.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CounterSpinnerStabilityState {
    /// Counters, spinners, and summarized-work items keep a stable placement and meaning
    /// across this profile; the status strip does not rapidly reflow or flicker when
    /// multiple jobs or counters update at once.
    CounterSpinnerSummaryStableNoReflow,
    /// Under this profile a counter's detail is disclosedly reduced (a wide count
    /// abbreviates to a magnitude, or a spinner label shortens) while the item keeps its
    /// stable placement, identity, and meaning and the reduction is disclosed.
    DisclosedReducedCounterDetail,
    /// The status / header strip reflows or flickers when counters or spinners update, so
    /// ambient state jitters and layout churns — always a blocker.
    StatusReflowsOrFlickersOnUpdate,
}

impl CounterSpinnerStabilityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CounterSpinnerSummaryStableNoReflow => "counter_spinner_summary_stable_no_reflow",
            Self::DisclosedReducedCounterDetail => "disclosed_reduced_counter_detail",
            Self::StatusReflowsOrFlickersOnUpdate => "status_reflows_or_flickers_on_update",
        }
    }

    /// `true` when counters/spinners/summaries stay stable without reflow.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::CounterSpinnerSummaryStableNoReflow)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedCounterDetail)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::StatusReflowsOrFlickersOnUpdate)
    }
}

/// How the profile keeps overflowed ambient items searchable from the command palette or
/// status menus with the same labels and explanations used in the visible instrumentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverflowSearchabilityState {
    /// Every overflowed ambient item stays discoverable through the command-palette or
    /// status-menu search and preserves the same label and explanation the visible item
    /// used, without pointer hover.
    OverflowItemsPaletteSearchableSameLabels,
    /// Under this profile the overflow search shows a disclosedly reduced detail (a shorter
    /// explanation, or a grouped result) while every overflowed item stays discoverable and
    /// keeps its original label; the reduction is disclosed.
    DisclosedReducedOverflowSearchDetail,
    /// An overflowed ambient item is undiscoverable from the palette/status search or is
    /// relabeled away from the visible instrumentation, so a displaced truth is reachable
    /// only by pointer hover — always a blocker.
    OverflowItemUndiscoverableOrRelabeled,
}

impl OverflowSearchabilityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OverflowItemsPaletteSearchableSameLabels => {
                "overflow_items_palette_searchable_same_labels"
            }
            Self::DisclosedReducedOverflowSearchDetail => {
                "disclosed_reduced_overflow_search_detail"
            }
            Self::OverflowItemUndiscoverableOrRelabeled => {
                "overflow_item_undiscoverable_or_relabeled"
            }
        }
    }

    /// `true` when overflowed items stay palette-searchable with the same labels.
    pub const fn is_reachable(self) -> bool {
        matches!(self, Self::OverflowItemsPaletteSearchableSameLabels)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedOverflowSearchDetail)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::OverflowItemUndiscoverableOrRelabeled)
    }
}

/// How the profile groups multiple active jobs, repeated updates, and quick state changes
/// into one meaningful summary rather than many flickering primitives.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GroupedSummaryState {
    /// Multiple active jobs, repeated updates, and quick state changes fold into one
    /// meaningful summarized-work item (a summary chip with a count) rather than many
    /// flickering primitives.
    MultiJobGroupedIntoOneSummary,
    /// Under this profile the grouping is disclosedly coarse (a summary folds distinct job
    /// classes into one chip sooner than the standard threshold) while the summary stays
    /// meaningful and each job stays reachable; the coarse grouping is disclosed and
    /// waivered.
    DisclosedCoarseGrouping,
    /// Many flickering primitives are shown instead of one grouped summary, so the status
    /// strip churns as jobs update — always a blocker.
    ManyFlickeringPrimitivesInsteadOfSummary,
}

impl GroupedSummaryState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MultiJobGroupedIntoOneSummary => "multi_job_grouped_into_one_summary",
            Self::DisclosedCoarseGrouping => "disclosed_coarse_grouping",
            Self::ManyFlickeringPrimitivesInsteadOfSummary => {
                "many_flickering_primitives_instead_of_summary"
            }
        }
    }

    /// `true` when multiple active jobs group into one summary.
    pub const fn is_grouped(self) -> bool {
        matches!(self, Self::MultiJobGroupedIntoOneSummary)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedCoarseGrouping)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::ManyFlickeringPrimitivesInsteadOfSummary)
    }
}

/// How the profile's ambient-stability behavior is reconstructable from reusable fixtures
/// and a support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilityExportState {
    /// The support export and the reusable fixtures reconstruct the profile's ambient
    /// instrumentation — its status items, overflow entries, counters, and grouped
    /// summaries — so a reflow or a lost overflow item can be diagnosed without a live
    /// screenshot.
    StabilityFixturesAndExportReconstructable,
    /// The support export reconstructs the profile's ambient instrumentation and discloses a
    /// partial capture (some low-priority overflow entries are trimmed) while the reduction
    /// is disclosed.
    DisclosedPartialCapture,
    /// The profile's ambient-instrumentation state is absent from the support-export
    /// capture, so a reflow or a lost overflow item cannot be explained without a live
    /// screenshot — always a blocker.
    StabilityStateAbsentFromCapture,
}

impl StabilityExportState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StabilityFixturesAndExportReconstructable => {
                "stability_fixtures_and_export_reconstructable"
            }
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::StabilityStateAbsentFromCapture => "stability_state_absent_from_capture",
        }
    }

    /// `true` when the fixtures and export reconstruct the ambient state.
    pub const fn is_reconstructable(self) -> bool {
        matches!(self, Self::StabilityFixturesAndExportReconstructable)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::StabilityStateAbsentFromCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red grouped-summary narrowing
/// stay yellow rather than blocked — never lets a reflowing strip, an undiscoverable
/// overflow item, a flickering-primitive churn, or a missing export hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AmbientStabilityWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed rendering profile the waiver applies to.
    pub profile: M5AmbientStabilityProfile,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl AmbientStabilityWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed profile's certification.
///
/// The trigger token mirrors the frozen [`M5ShellPrimitiveDowngradeTrigger`] vocabulary so a
/// cause never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AmbientStabilityCause {
    /// The governed rendering profile the cause applies to.
    pub profile: M5AmbientStabilityProfile,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5ShellPrimitiveDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed
    /// cause is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl AmbientStabilityCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed rendering profile, certified across counter stability, overflow
/// searchability, grouped summary, and stability export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AmbientStabilityRow {
    /// The governed rendering profile being certified.
    pub profile: M5AmbientStabilityProfile,
    /// The ambient primitives this profile drives. Pulled from the matrix.
    pub driven_primitive_families: Vec<M5ShellPrimitiveFamily>,
    /// The frozen qualification class of the driven ambient primitives (the most-narrowed of
    /// the three). Pulled from the matrix.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Owner role accountable for keeping this profile certified.
    pub owner_role: String,
    /// Short profile label.
    pub profile_label: String,
    /// The canonical shell zone the ambient instrumentation attaches to. Pulled from the
    /// matrix.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Status-item classes these surfaces project (union across the ambient families).
    /// Pulled from the matrix.
    pub certified_status_item_classes: Vec<M5StatusItemClass>,
    /// Overflow behaviors these surfaces honour (union across the ambient families). Pulled
    /// from the matrix.
    pub certified_overflow_behaviors: Vec<M5OverflowBehavior>,
    /// Source/provider/freshness labels these surfaces can show (union across the ambient
    /// families). Pulled from the matrix.
    pub certified_source_freshness_labels: Vec<M5SourceFreshnessLabel>,
    /// Non-visual accessibility routes. Pulled from the matrix.
    pub accessibility_routes: Vec<M5AccessibilityRoute>,
    /// Mandatory labels every ambient surface must be able to show. Pulled from the matrix.
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// Shell subsystems this profile stays aligned across. Pulled from the matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this profile. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ShellPrimitiveDowngradeTrigger>,
    /// Counter/spinner/summary-stability posture.
    pub counter_stability: CounterSpinnerStabilityState,
    /// Overflow-searchability posture.
    pub overflow_searchability: OverflowSearchabilityState,
    /// Grouped-summary posture.
    pub grouped_summary: GroupedSummaryState,
    /// Stability-export posture.
    pub stability_export: StabilityExportState,
    /// Hard invariant: the status bar never reflows around a vanity item. `false` is a
    /// blocker.
    pub never_reflows_around_vanity_items: bool,
    /// Active waiver, when a disclosed coarse grouping is in force.
    pub active_waiver: Option<AmbientStabilityWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: AmbientStabilityStatus,
    /// The exact certification causes that narrowed or blocked this row.
    pub certification_causes: Vec<AmbientStabilityCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl AmbientStabilityRow {
    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when every status-item class the matrix freezes is certified — the lint that
    /// prevents an ambient surface from shipping without every ambient truth class
    /// (background work, connection target, deployment profile, sync freshness, problem
    /// count, mode, notification summary, capacity) staying legible under the profile.
    pub fn status_item_classes_complete(&self) -> bool {
        let present: BTreeSet<M5StatusItemClass> =
            self.certified_status_item_classes.iter().copied().collect();
        M5StatusItemClass::ALL
            .iter()
            .all(|class| present.contains(class))
    }

    /// `true` when every overflow behavior the matrix freezes is certified — the lint that
    /// prevents an ambient surface from shipping without a keyboard-reachable overflow,
    /// summary grouping, or priority-pin path.
    pub fn overflow_behaviors_complete(&self) -> bool {
        let present: BTreeSet<M5OverflowBehavior> =
            self.certified_overflow_behaviors.iter().copied().collect();
        M5OverflowBehavior::ALL
            .iter()
            .all(|behavior| present.contains(behavior))
    }

    /// `true` when every ambient-stability required label is certified — the lint that
    /// prevents an ambient surface from shipping without identity, state, keyboard-route,
    /// source-provider, freshness, and reopen-path labels.
    pub fn required_labels_complete(&self) -> bool {
        let present: BTreeSet<M5PrimitiveRequiredLabel> =
            self.required_labels.iter().copied().collect();
        AMBIENT_STABILITY_REQUIRED_LABELS
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        self.counter_stability.is_blocked()
            || self.overflow_searchability.is_blocked()
            || self.grouped_summary.is_blocked()
            || self.stability_export.is_blocked()
            || !self.never_reflows_around_vanity_items
            || !self.status_item_classes_complete()
            || !self.overflow_behaviors_complete()
            || !self.required_labels_complete()
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.counter_stability.is_disclosed()
            || self.overflow_searchability.is_disclosed()
            || self.grouped_summary.is_disclosed()
            || self.stability_export.is_disclosed()
    }

    /// Recomputes the derived status from the four axes and the vanity-reflow invariant.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing
    /// forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> AmbientStabilityStatus {
        if self.has_hard_blocker() {
            AmbientStabilityStatus::Red
        } else if self.has_narrowing() {
            AmbientStabilityStatus::Yellow
        } else {
            AmbientStabilityStatus::Green
        }
    }

    /// Recomputes the exact certification causes for the row, in deterministic order
    /// (counter stability, overflow searchability, grouped summary, export, vanity-reflow
    /// invariant).
    pub fn recompute_causes(&self) -> Vec<AmbientStabilityCause> {
        let mut causes = Vec::new();
        if !self.counter_stability.is_stable() {
            causes.push(AmbientStabilityCause {
                profile: self.profile,
                trigger: M5ShellPrimitiveDowngradeTrigger::VanityItemReflow,
                disclosed: self.counter_stability.is_disclosed(),
                detail: if self.counter_stability.is_disclosed() {
                    "Under this profile a counter's detail is disclosedly reduced (a wide count \
                     abbreviates to a magnitude, or a spinner label shortens) while the item keeps \
                     its stable placement, identity, and meaning; the reduction is disclosed and the \
                     row is narrowed below green."
                        .to_owned()
                } else {
                    "The status / header strip reflows or flickers when counters or spinners update, \
                     so ambient state jitters and layout churns."
                        .to_owned()
                },
            });
        }
        if !self.overflow_searchability.is_reachable() {
            causes.push(AmbientStabilityCause {
                profile: self.profile,
                trigger: M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth,
                disclosed: self.overflow_searchability.is_disclosed(),
                detail: if self.overflow_searchability.is_disclosed() {
                    "Under this profile the overflow search shows a disclosedly reduced detail (a \
                     shorter explanation or a grouped result) while every overflowed item stays \
                     discoverable and keeps its original label; the reduction is disclosed and the \
                     row is narrowed below green."
                        .to_owned()
                } else {
                    "An overflowed ambient item is undiscoverable from the palette/status search or \
                     is relabeled away from the visible instrumentation, so a displaced truth is \
                     reachable only by pointer hover."
                        .to_owned()
                },
            });
        }
        if !self.grouped_summary.is_grouped() {
            causes.push(AmbientStabilityCause {
                profile: self.profile,
                trigger: M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState,
                disclosed: self.grouped_summary.is_disclosed(),
                detail: if self.grouped_summary.is_disclosed() {
                    "Under this profile the grouping is disclosedly coarse (a summary folds distinct \
                     job classes into one chip sooner than the standard threshold) while the summary \
                     stays meaningful and each job stays reachable; the coarse grouping is disclosed \
                     and waivered, and the row is narrowed below green."
                        .to_owned()
                } else {
                    "Many flickering primitives are shown instead of one grouped summary, so the \
                     status strip churns as jobs update."
                        .to_owned()
                },
            });
        }
        if !self.stability_export.is_reconstructable() {
            causes.push(AmbientStabilityCause {
                profile: self.profile,
                trigger: M5ShellPrimitiveDowngradeTrigger::ProofStale,
                disclosed: self.stability_export.is_disclosed(),
                detail: if self.stability_export.is_disclosed() {
                    "The support export reconstructs the profile's ambient instrumentation and \
                     discloses a partial capture (some low-priority overflow entries are trimmed) \
                     while the reduction is disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "The profile's ambient-instrumentation state is absent from the support-export \
                     capture, so a reflow or a lost overflow item cannot be explained without a live \
                     screenshot."
                        .to_owned()
                },
            });
        }
        if !self.never_reflows_around_vanity_items {
            causes.push(AmbientStabilityCause {
                profile: self.profile,
                trigger: M5ShellPrimitiveDowngradeTrigger::VanityItemReflow,
                disclosed: false,
                detail: "The status bar reflows around a vanity or decorative item, displacing a \
                         truth-bearing peer, so the strip is not overflow-safe."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed coarse grouping may only stay yellow (rather than red) when a waiver
    /// discloses it.
    pub fn requires_waiver(&self) -> bool {
        self.grouped_summary.is_disclosed()
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<AmbientStabilityFinding> {
        let mut findings = Vec::new();
        let profile = self.profile.as_str().to_owned();

        if self.counter_stability.is_blocked() {
            findings.push(AmbientStabilityFinding::CountersReflow {
                profile: profile.clone(),
            });
        }
        if self.overflow_searchability.is_blocked() {
            findings.push(AmbientStabilityFinding::OverflowUndiscoverable {
                profile: profile.clone(),
            });
        }
        if self.grouped_summary.is_blocked() {
            findings.push(AmbientStabilityFinding::FlickeringPrimitives {
                profile: profile.clone(),
            });
        }
        if self.stability_export.is_blocked() {
            findings.push(AmbientStabilityFinding::StabilityStateAbsentFromCapture {
                profile: profile.clone(),
            });
        }
        if !self.never_reflows_around_vanity_items {
            findings.push(AmbientStabilityFinding::StatusReflowsAroundVanityItems {
                profile: profile.clone(),
            });
        }
        if !self.status_item_classes_complete() {
            findings.push(AmbientStabilityFinding::StatusItemClassesIncomplete {
                profile: profile.clone(),
            });
        }
        if !self.overflow_behaviors_complete() {
            findings.push(AmbientStabilityFinding::OverflowBehaviorsIncomplete {
                profile: profile.clone(),
            });
        }
        if !self.required_labels_complete() {
            findings.push(AmbientStabilityFinding::RequiredLabelsIncomplete {
                profile: profile.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, AmbientStabilityStatus::Green) && !self.has_reason() {
            findings.push(AmbientStabilityFinding::NarrowedRowWithoutReason {
                profile: profile.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an
        // active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(AmbientStabilityFinding::NarrowedRowWithoutWaiver {
                profile: profile.clone(),
            });
        }
        // An attached waiver must still be active and must point at this profile.
        if let Some(waiver) = &self.active_waiver {
            if waiver.profile != self.profile {
                findings.push(AmbientStabilityFinding::WaiverProfileMismatch {
                    profile: profile.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(AmbientStabilityFinding::WaiverExpired {
                    profile: profile.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(AmbientStabilityFinding::RowStatusStale {
                profile: profile.clone(),
            });
        }
        if self.certification_causes != self.recompute_causes() {
            findings.push(AmbientStabilityFinding::RowCausesStale { profile });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} counter={} overflow={} grouped={} export={} no_vanity_reflow={} waiver={}",
            self.profile.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.counter_stability.as_str(),
            self.overflow_searchability.as_str(),
            self.grouped_summary.as_str(),
            self.stability_export.as_str(),
            self.never_reflows_around_vanity_items,
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the ambient-instrumentation stability proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum AmbientStabilityFinding {
    /// A governed rendering profile has no certification row.
    ProfileMissing {
        /// The missing profile token.
        profile: String,
    },
    /// A profile's status / header strip reflows or flickers when counters update.
    CountersReflow {
        /// The profile token.
        profile: String,
    },
    /// A profile has an overflowed item undiscoverable from the palette/status search.
    OverflowUndiscoverable {
        /// The profile token.
        profile: String,
    },
    /// A profile shows many flickering primitives instead of one grouped summary.
    FlickeringPrimitives {
        /// The profile token.
        profile: String,
    },
    /// A profile's ambient-instrumentation state is absent from the support-export capture.
    StabilityStateAbsentFromCapture {
        /// The profile token.
        profile: String,
    },
    /// A profile's status bar reflows around a vanity item.
    StatusReflowsAroundVanityItems {
        /// The profile token.
        profile: String,
    },
    /// A profile does not certify every frozen status-item class.
    StatusItemClassesIncomplete {
        /// The profile token.
        profile: String,
    },
    /// A profile does not certify every frozen overflow behavior.
    OverflowBehaviorsIncomplete {
        /// The profile token.
        profile: String,
    },
    /// A profile does not certify every ambient-stability required label.
    RequiredLabelsIncomplete {
        /// The profile token.
        profile: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The profile token.
        profile: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The profile token.
        profile: String,
    },
    /// An attached waiver does not point at the row's profile.
    WaiverProfileMismatch {
        /// The profile token.
        profile: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The profile token.
        profile: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The profile token.
        profile: String,
    },
    /// The declared certification causes do not match the recomputed causes.
    RowCausesStale {
        /// The profile token.
        profile: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered profiles do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl AmbientStabilityFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ProfileMissing { .. } => "profile_missing",
            Self::CountersReflow { .. } => "counters_reflow",
            Self::OverflowUndiscoverable { .. } => "overflow_undiscoverable",
            Self::FlickeringPrimitives { .. } => "flickering_primitives",
            Self::StabilityStateAbsentFromCapture { .. } => "stability_state_absent_from_capture",
            Self::StatusReflowsAroundVanityItems { .. } => "status_reflows_around_vanity_items",
            Self::StatusItemClassesIncomplete { .. } => "status_item_classes_incomplete",
            Self::OverflowBehaviorsIncomplete { .. } => "overflow_behaviors_incomplete",
            Self::RequiredLabelsIncomplete { .. } => "required_labels_incomplete",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverProfileMismatch { .. } => "waiver_profile_mismatch",
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
            Self::ProfileMissing { profile }
            | Self::CountersReflow { profile }
            | Self::OverflowUndiscoverable { profile }
            | Self::FlickeringPrimitives { profile }
            | Self::StabilityStateAbsentFromCapture { profile }
            | Self::StatusReflowsAroundVanityItems { profile }
            | Self::StatusItemClassesIncomplete { profile }
            | Self::OverflowBehaviorsIncomplete { profile }
            | Self::RequiredLabelsIncomplete { profile }
            | Self::NarrowedRowWithoutReason { profile }
            | Self::NarrowedRowWithoutWaiver { profile }
            | Self::WaiverProfileMismatch { profile, .. }
            | Self::WaiverExpired { profile, .. }
            | Self::RowStatusStale { profile }
            | Self::RowCausesStale { profile } => profile,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release certification packet shared by the shell / status bar / release automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AmbientStabilityPacket {
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
    /// The frozen shell-primitives matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen shell-primitives matrix schema.
    pub matrix_schema_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The certification dimensions every profile is certified across.
    pub required_proof_dimensions: Vec<M5AmbientStabilityProofDimension>,
    /// The status-item classes every profile must certify.
    pub required_status_item_classes: Vec<M5StatusItemClass>,
    /// The overflow behaviors every profile must certify.
    pub required_overflow_behaviors: Vec<M5OverflowBehavior>,
    /// The required labels every profile must certify.
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// Per-profile certification rows, in canonical order.
    pub rows: Vec<AmbientStabilityRow>,
    /// Governed profiles certified, in canonical (sorted) order.
    pub covered_profiles: Vec<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of green rows.
    pub green_row_count: usize,
    /// Number of yellow (auto-narrowed, disclosed) rows.
    pub yellow_row_count: usize,
    /// Number of red (blocked) rows.
    pub red_row_count: usize,
    /// `true` when no row is blocked.
    pub all_rows_publishable: bool,
    /// Every active waiver in force, sorted by waiver id.
    pub active_waivers: Vec<AmbientStabilityWaiver>,
    /// Every exact certification cause, in row then cause order.
    pub certification_causes: Vec<AmbientStabilityCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<AmbientStabilityFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Shell / release automation refs that consume this packet to auto-narrow claimed
    /// profiles.
    pub shell_automation_refs: Vec<String>,
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

impl AmbientStabilityPacket {
    /// Returns the certification row for `profile`, if present.
    pub fn row(&self, profile: M5AmbientStabilityProfile) -> Option<&AmbientStabilityRow> {
        self.rows.iter().find(|row| row.profile == profile)
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
                waiver.profile.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.certification_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.profile.as_str(),
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

    /// Projects the light certification dashboard the shell automation consumes.
    pub fn dashboard(&self) -> AmbientStabilityDashboard {
        AmbientStabilityDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 ambient-instrumentation stability packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per profile.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "profile,status,qualification,shell_zone_slot,counter_stability,overflow_searchability,grouped_summary,stability_export,never_reflows_around_vanity_items,status_item_classes,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.profile.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.shell_zone_slot.as_str(),
                row.counter_stability.as_str(),
                row.overflow_searchability.as_str(),
                row.grouped_summary.as_str(),
                row.stability_export.as_str(),
                row.never_reflows_around_vanity_items,
                join_tokens(&row.certified_status_item_classes, |s| s.as_str()),
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
        out.push_str("# M5 ambient shell-instrumentation stability\n\n");
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_ambient_instrumentation_stability`](../../crates/aureline-shell/src/m5_ambient_instrumentation_stability/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_ambient_instrumentation_stability -- markdown > \\\n  artifacts/shell/m5-ambient-instrumentation-stability.md\n",
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
        out.push_str(&format!("- Green: {}\n", self.green_row_count));
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

        out.push_str("## Certification dimensions\n\n");
        for dimension in &self.required_proof_dimensions {
            out.push_str(&format!("- `{}`\n", dimension.as_str()));
        }
        out.push('\n');

        out.push_str("## Certification rows\n\n");
        out.push_str(
            "| Profile | Status | Qualification | Counter | Overflow | Grouped | Export | No-vanity-reflow | Waiver |\n\
             | ------- | ------ | ------------- | ------- | -------- | ------- | ------ | ---------------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.profile_label,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.counter_stability.as_str(),
                row.overflow_searchability.as_str(),
                row.grouped_summary.as_str(),
                row.stability_export.as_str(),
                row.never_reflows_around_vanity_items,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&AmbientStabilityRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, AmbientStabilityStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every governed rendering profile is certified at full standing.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.profile.as_str(),
                    row.derived_status.as_str(),
                    row.narrowing_reason.as_deref().unwrap_or("(undisclosed)"),
                ));
            }
            out.push('\n');
        }

        out.push_str("## Exact certification causes\n\n");
        if self.certification_causes.is_empty() {
            out.push_str("None.\n\n");
        } else {
            for cause in &self.certification_causes {
                out.push_str(&format!(
                    "- `{}` — `{}` (disclosed: `{}`) — {}\n",
                    cause.profile.as_str(),
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
                    waiver.profile.as_str(),
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_ambient_instrumentation_stability -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_ambient_instrumentation_stability_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AmbientStabilityDashboardRow {
    /// The governed profile.
    pub profile: M5AmbientStabilityProfile,
    /// Short profile label.
    pub profile_label: String,
    /// Derived green/yellow/red status.
    pub status: AmbientStabilityStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Counter-stability posture.
    pub counter_stability: CounterSpinnerStabilityState,
    /// Overflow-searchability posture.
    pub overflow_searchability: OverflowSearchabilityState,
    /// Grouped-summary posture.
    pub grouped_summary: GroupedSummaryState,
    /// Stability-export posture.
    pub stability_export: StabilityExportState,
    /// `true` when the status bar never reflows around a vanity item.
    pub never_reflows_around_vanity_items: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the shell / status bar / release automation reads to
/// auto-narrow claimed rendering profiles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AmbientStabilityDashboard {
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
    pub rows: Vec<AmbientStabilityDashboardRow>,
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

impl AmbientStabilityDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &AmbientStabilityPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| AmbientStabilityDashboardRow {
                profile: row.profile,
                profile_label: row.profile_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                counter_stability: row.counter_stability,
                overflow_searchability: row.overflow_searchability,
                grouped_summary: row.grouped_summary,
                stability_export: row.stability_export,
                never_reflows_around_vanity_items: row.never_reflows_around_vanity_items,
                has_active_waiver: row.has_active_waiver(),
                waiver_id: row.active_waiver.as_ref().map(|w| w.waiver_id.clone()),
                cause_tokens: row
                    .certification_causes
                    .iter()
                    .map(|cause| cause.cause_token().to_owned())
                    .collect(),
                narrowing_reason: row.narrowing_reason.clone(),
            })
            .collect();
        Self {
            record_kind: M5_AMBIENT_INSTRUMENTATION_STABILITY_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_AMBIENT_INSTRUMENTATION_STABILITY_SCHEMA_VERSION,
            dashboard_id: M5_AMBIENT_INSTRUMENTATION_STABILITY_DASHBOARD_ID.to_owned(),
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
        serde_json::to_string_pretty(self)
            .expect("m5 ambient-instrumentation stability dashboard serializes")
    }
}

/// Support-export wrapper for the ambient-instrumentation stability certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AmbientStabilitySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: AmbientStabilityPacket,
    /// Dashboard quoted in full.
    pub dashboard: AmbientStabilityDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl AmbientStabilitySupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each profile, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the shell
    /// automation — can name the same profile and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: AmbientStabilityPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.profile.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_AMBIENT_INSTRUMENTATION_STABILITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_AMBIENT_INSTRUMENTATION_STABILITY_SCHEMA_VERSION,
            shared_contract_ref: M5_AMBIENT_INSTRUMENTATION_STABILITY_SHARED_CONTRACT_REF
                .to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_ambient_instrumentation_stability_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AmbientStabilityInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-primitives matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-profile certification rows.
    pub rows: Vec<AmbientStabilityRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a
/// stray comma.
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

/// Builds an [`AmbientStabilityPacket`] from the exact build identity, the frozen matrix
/// ref, and the per-profile certification rows.
///
/// Each row's derived status and certification causes, the aggregate counts, the active
/// waivers, and the blocking findings are recomputed here so the packet is the single source
/// of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_ambient_instrumentation_stability_packet(
    input: AmbientStabilityInput,
) -> AmbientStabilityPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and
    // the auto-narrowing is the single source of truth.
    let rows: Vec<AmbientStabilityRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.certification_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<AmbientStabilityFinding> = Vec::new();

    // Every governed profile must carry a certification row.
    let present: BTreeSet<M5AmbientStabilityProfile> = rows.iter().map(|row| row.profile).collect();
    for profile in M5AmbientStabilityProfile::ALL {
        if !present.contains(&profile) {
            blocking_findings.push(AmbientStabilityFinding::ProfileMissing {
                profile: profile.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_profiles: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|profile| profile.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AmbientStabilityStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AmbientStabilityStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AmbientStabilityStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(AmbientStabilityFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<AmbientStabilityWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let certification_causes: Vec<AmbientStabilityCause> = rows
        .iter()
        .flat_map(|row| row.certification_causes.clone())
        .collect();

    let mut packet = AmbientStabilityPacket {
        record_kind: M5_AMBIENT_INSTRUMENTATION_STABILITY_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_AMBIENT_INSTRUMENTATION_STABILITY_SCHEMA_VERSION,
        shared_contract_ref: M5_AMBIENT_INSTRUMENTATION_STABILITY_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_AMBIENT_INSTRUMENTATION_STABILITY_PACKET_ID.to_owned(),
        source_schema_ref: M5_AMBIENT_INSTRUMENTATION_STABILITY_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Ambient shell-instrumentation stability certified across every claimed M5 \
                   rendering profile: standard, compact, expanded, multi-window, high-zoom, \
                   reduced-motion, degraded-network, and degraded-power each keep counters, \
                   spinners, and multi-job summaries stable without reflowing or flickering the \
                   status strip, keep overflowed items palette/status searchable with the same \
                   labels, group many active jobs into one meaningful summary rather than many \
                   flickering primitives, and reconstruct their ambient-stability behavior from \
                   reusable fixtures and a support export — with each row's green/yellow/red claim \
                   auto-narrowed from its counter-stability, overflow-searchability, \
                   grouped-summary, and stability-export posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_AMBIENT_INSTRUMENTATION_STABILITY_MATRIX_SCHEMA_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_proof_dimensions: M5AmbientStabilityProofDimension::ALL.to_vec(),
        required_status_item_classes: M5StatusItemClass::ALL.to_vec(),
        required_overflow_behaviors: M5OverflowBehavior::ALL.to_vec(),
        required_labels: AMBIENT_STABILITY_REQUIRED_LABELS.to_vec(),
        rows,
        covered_profiles,
        row_count,
        green_row_count,
        yellow_row_count,
        red_row_count,
        all_rows_publishable,
        active_waivers,
        certification_causes,
        blocking_findings: Vec::new(),
        report_clean: false,
        shell_automation_refs: vec![
            "shell_frame.status_bar.ambient_instrumentation_registry".to_owned(),
            "release_automation.auto_narrow.ambient_instrumentation_stability_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.ambient_instrumentation_stability".to_owned(),
            "artifacts/release/m5-ambient-instrumentation-stability-proof/packet.json".to_owned(),
        ],
        help_docs_refs: vec![M5_AMBIENT_INSTRUMENTATION_STABILITY_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-ambient-instrumentation-stability".to_owned()],
        published_report_ref: M5_AMBIENT_INSTRUMENTATION_STABILITY_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_AMBIENT_INSTRUMENTATION_STABILITY_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_AMBIENT_INSTRUMENTATION_STABILITY_PUBLISHED_DASHBOARD_REF
            .to_owned(),
        published_doc_ref: M5_AMBIENT_INSTRUMENTATION_STABILITY_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(AmbientStabilityFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_ambient_instrumentation_stability_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum AmbientStabilityValidationError {
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
    /// The rows do not cover all eight governed profiles.
    CoverageIncomplete,
    /// The declared covered profiles do not match the rows.
    CoverageStale,
    /// The declared required proof dimensions are not the canonical set.
    RequiredDimensionsStale,
    /// The declared required status-item classes are not the canonical set.
    RequiredStatusItemClassesStale,
    /// The declared required overflow behaviors are not the canonical set.
    RequiredOverflowBehaviorsStale,
    /// The declared required labels are not the ambient-stability set.
    RequiredLabelsStale,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared active waivers do not match the rows.
    ActiveWaiversStale,
    /// The declared certification causes do not match the recomputed causes.
    CertificationCausesStale,
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

/// Validates a packet against the ambient-instrumentation stability invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed rendering
/// profile carries a current certification row; each row's status is the derived
/// auto-narrowed value, never asserted; a green row cannot keep a claim while the status
/// strip reflows or flickers on update, an overflowed item is undiscoverable or relabeled,
/// many flickering primitives are shown instead of a grouped summary, the ambient-stability
/// state is dropped from capture, the status bar reflows around a vanity item, or its
/// status-item classes / overflow behaviors / required labels are incomplete; and a disclosed
/// narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_ambient_instrumentation_stability_packet(
    packet: &AmbientStabilityPacket,
) -> Result<(), Vec<AmbientStabilityValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(AmbientStabilityValidationError::NoRows);
    }
    if packet.record_kind != M5_AMBIENT_INSTRUMENTATION_STABILITY_PACKET_RECORD_KIND {
        errors.push(AmbientStabilityValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_AMBIENT_INSTRUMENTATION_STABILITY_SCHEMA_VERSION {
        errors.push(AmbientStabilityValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(AmbientStabilityValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(AmbientStabilityValidationError::MatrixPacketRefMissing);
    }
    if packet.required_proof_dimensions != M5AmbientStabilityProofDimension::ALL {
        errors.push(AmbientStabilityValidationError::RequiredDimensionsStale);
    }
    if packet.required_status_item_classes != M5StatusItemClass::ALL {
        errors.push(AmbientStabilityValidationError::RequiredStatusItemClassesStale);
    }
    if packet.required_overflow_behaviors != M5OverflowBehavior::ALL {
        errors.push(AmbientStabilityValidationError::RequiredOverflowBehaviorsStale);
    }
    if packet.required_labels != AMBIENT_STABILITY_REQUIRED_LABELS {
        errors.push(AmbientStabilityValidationError::RequiredLabelsStale);
    }

    let present: BTreeSet<M5AmbientStabilityProfile> =
        packet.rows.iter().map(|row| row.profile).collect();
    let coverage_complete = M5AmbientStabilityProfile::ALL
        .iter()
        .all(|profile| present.contains(profile));
    if !coverage_complete || packet.rows.len() != M5AmbientStabilityProfile::ALL.len() {
        errors.push(AmbientStabilityValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|profile| profile.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_profiles {
        errors.push(AmbientStabilityValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AmbientStabilityStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AmbientStabilityStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AmbientStabilityStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(AmbientStabilityValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<AmbientStabilityWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(AmbientStabilityValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<AmbientStabilityCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.certification_causes {
        errors.push(AmbientStabilityValidationError::CertificationCausesStale);
    }

    let mut recomputed: Vec<AmbientStabilityFinding> = Vec::new();
    for profile in M5AmbientStabilityProfile::ALL {
        if !present.contains(&profile) {
            recomputed.push(AmbientStabilityFinding::ProfileMissing {
                profile: profile.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(AmbientStabilityFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(AmbientStabilityFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(AmbientStabilityValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(AmbientStabilityValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(AmbientStabilityValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(AmbientStabilityValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(AmbientStabilityValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(AmbientStabilityValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
