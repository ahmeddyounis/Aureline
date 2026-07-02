//! Status-bar item priority classes, stable placement, overflow discoverability,
//! and inspector back-links certified across every claimed M5 status context.
//!
//! The [frozen shell-primitives matrix][matrix] already freezes the ambient
//! status-bar primitives — the status-bar item and the status overflow menu — into
//! one export-safe packet: their status-item classes, overflow / severe-state
//! displacement behaviors, source/provider/freshness labels, accessibility routes,
//! and the mandatory labels every ambient instrument must be able to show. This
//! lane is the **status-bar certification capstone** on top of that matrix: for
//! every claimed M5 status context — the notebook, data/API, remote, preview,
//! review, profiler, and incident lanes plus the desktop base — it certifies that
//! the status bar carries recovery-critical, execution/context, ongoing-work, and
//! ambient-metadata items in stable priority slots that never jitter or reflow
//! around a spinner or vanity item, that everything visible or overflowed stays
//! reachable through keyboard search, a status menu, or a palette route, that every
//! item links back to the narrowest useful inspector or command rather than a
//! generic settings detour, and that a support/export packet can reconstruct the
//! visible and overflowed items, their owning subsystems, and any critical-state
//! displacement without a screenshot.
//!
//! Three records carry the truth:
//!
//! - the per-context **certification row** ([`StatusBarCertificationRow`]): one row
//!   per [`M5StatusContext`] naming the ambient status primitives it drives, the
//!   priority classes it keeps in stable slots, the reach routes every item and
//!   overflow entry resolves through, the status-item classes / overflow behaviors /
//!   freshness labels / accessibility routes / consumer surfaces / downgrade
//!   triggers pulled from the frozen matrix, its placement / overflow /
//!   inspector-backlink / support-export posture, any active waiver, and a derived
//!   green/yellow/red [`StatusBarCertificationStatus`].
//! - the release **certification packet** ([`StatusBarCertificationPacket`]): the
//!   full set of rows with derived per-row status, aggregate green/yellow/red
//!   counts, the active waivers, the exact certification causes
//!   ([`StatusBarCertificationCause`]), and the blocking findings the lane refuses
//!   to ship with.
//! - the **certification dashboard** ([`StatusBarCertificationDashboard`]): a light
//!   projection the status bar / attention router / release automation reads to
//!   auto-narrow a claimed status context when its certification proof falls out of
//!   policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to
//! `yellow` the moment it discloses a compact priority compaction (backed by a
//! waiver), a reduced overflow route, a grouped inspector back-link, or a partial
//! support-export capture; it drops to `red` if its priority slots jitter or reflow
//! around a vanity item, its overflow becomes hover- or pointer-only, an item's
//! inspector back-link is missing or dumps into a generic settings detour, a
//! critical-state displacement is absent from the support/export capture, a status
//! item is reachable only through hover, its priority-class order is not the
//! canonical recovery-critical-first order, or its reach routes / status-item
//! classes are incomplete. That derivation is the auto-narrowing the acceptance
//! criteria require.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw
//! URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials —
//! only stable ids, closed vocabulary, counts, refs, and short labels. The
//! status-item-class, overflow-behavior, source-freshness, accessibility-route,
//! consumer-surface, downgrade-trigger, required-label, and qualification vocabulary
//! is re-exported by reference from the already frozen [matrix]; each row pulls its
//! ambient bindings straight from that matrix's seeded status-bar-item row, so this
//! lane mints no parallel shell vocabulary and cannot certify an ambient posture the
//! matrix does not freeze. Only the certification-specific vocabulary
//! ([`M5StatusContext`], [`M5StatusPriorityClass`], [`M5StatusReachRoute`],
//! [`M5StatusBarProofDimension`], [`StatusBarCertificationStatus`],
//! [`PlacementStabilityState`], [`OverflowDiscoverabilityState`],
//! [`InspectorBacklinkState`], [`SupportExportParityState`],
//! [`StatusBarCertificationWaiver`], [`StatusBarCertificationCause`],
//! [`StatusBarCertificationFinding`]) is new.
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
    seeded_m5_status_bar_certification_packet,
    seeded_m5_status_bar_certification_packet_data_api_overflow_hover_only_blocked,
    seeded_m5_status_bar_certification_packet_desktop_base_hover_only_blocked,
    seeded_m5_status_bar_certification_packet_notebook_vanity_reflow_blocked,
    seeded_m5_status_bar_certification_packet_preview_capture_absent_blocked,
    seeded_m5_status_bar_certification_packet_review_backlink_missing_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_STATUS_BAR_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_STATUS_BAR_CERTIFICATION_SHARED_CONTRACT_REF: &str =
    "shell:m5_status_bar_certification:v1";

/// Stable record kind for [`StatusBarCertificationPacket`] payloads.
pub const M5_STATUS_BAR_CERTIFICATION_PACKET_RECORD_KIND: &str =
    "shell_m5_status_bar_certification_packet_record";

/// Stable record kind for [`StatusBarCertificationDashboard`] payloads.
pub const M5_STATUS_BAR_CERTIFICATION_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_status_bar_certification_dashboard_record";

/// Stable record kind for [`StatusBarCertificationSupportExport`] payloads.
pub const M5_STATUS_BAR_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_status_bar_certification_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_STATUS_BAR_CERTIFICATION_PACKET_ID: &str = "m5-status-bar-certification:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_STATUS_BAR_CERTIFICATION_DASHBOARD_ID: &str =
    "m5-status-bar-certification-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_STATUS_BAR_CERTIFICATION_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-status-bar-certification:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_STATUS_BAR_CERTIFICATION_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-status-bar-certification.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_STATUS_BAR_CERTIFICATION_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-status-bar-certification.md";

/// Published certification-packet artifact ref.
pub const M5_STATUS_BAR_CERTIFICATION_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-status-bar-certification-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_STATUS_BAR_CERTIFICATION_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-status-bar-certification-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_STATUS_BAR_CERTIFICATION_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-status-bar-certification-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_STATUS_BAR_CERTIFICATION_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-status-bar-certification-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_STATUS_BAR_CERTIFICATION_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_status_bar_certification_contract.md";

/// Repo-relative ref to the frozen shell-primitives matrix schema.
pub const M5_STATUS_BAR_CERTIFICATION_MATRIX_SCHEMA_REF: &str =
    matrix::M5_SHELL_PRIMITIVES_SCHEMA_REF;

/// One of the claimed M5 status contexts the certification proof must cover, in
/// canonical order. Each context is a claimed M5 shell lane whose status bar carries
/// ambient instrumentation; the lane certifies none beyond them and refuses to ship
/// if any is missing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StatusContext {
    /// Notebook / cell-runtime lane.
    NotebookLane,
    /// Data grid / API-run lane.
    DataApiLane,
    /// Remote / connected-target lane.
    RemoteLane,
    /// Preview lane (render, diff, media).
    PreviewLane,
    /// Review / change-request lane.
    ReviewLane,
    /// Profiler / performance-capture lane.
    ProfilerLane,
    /// Incident / operations-response lane.
    IncidentLane,
    /// Desktop base lane (the ambient default workspace status bar).
    DesktopBaseLane,
}

impl M5StatusContext {
    /// Every governed status context, in canonical order.
    pub const ALL: [Self; 8] = [
        Self::NotebookLane,
        Self::DataApiLane,
        Self::RemoteLane,
        Self::PreviewLane,
        Self::ReviewLane,
        Self::ProfilerLane,
        Self::IncidentLane,
        Self::DesktopBaseLane,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookLane => "notebook_lane",
            Self::DataApiLane => "data_api_lane",
            Self::RemoteLane => "remote_lane",
            Self::PreviewLane => "preview_lane",
            Self::ReviewLane => "review_lane",
            Self::ProfilerLane => "profiler_lane",
            Self::IncidentLane => "incident_lane",
            Self::DesktopBaseLane => "desktop_base_lane",
        }
    }

    /// Short, reviewer-facing label for the context's status bar.
    pub const fn label(self) -> &'static str {
        match self {
            Self::NotebookLane => "Notebook lane status bar",
            Self::DataApiLane => "Data / API lane status bar",
            Self::RemoteLane => "Remote lane status bar",
            Self::PreviewLane => "Preview lane status bar",
            Self::ReviewLane => "Review lane status bar",
            Self::ProfilerLane => "Profiler lane status bar",
            Self::IncidentLane => "Incident lane status bar",
            Self::DesktopBaseLane => "Desktop base status bar",
        }
    }
}

/// The status-bar item priority class — the tier that decides which items hold a
/// stable slot and which yield when the bar compacts. The declaration order is the
/// canonical, recovery-critical-first placement order every context must preserve.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StatusPriorityClass {
    /// Recovery-critical state (highest priority; never displaced by ambient noise).
    RecoveryCritical,
    /// Execution / context truth (the active target, mode, or deployment profile).
    ExecutionContext,
    /// Ongoing work (background jobs, sync, progress attribution).
    OngoingWork,
    /// Ambient metadata (lowest priority; the first to compact into overflow).
    AmbientMetadata,
}

impl M5StatusPriorityClass {
    /// Every priority class, in canonical recovery-critical-first order.
    pub const ALL: [Self; 4] = [
        Self::RecoveryCritical,
        Self::ExecutionContext,
        Self::OngoingWork,
        Self::AmbientMetadata,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RecoveryCritical => "recovery_critical",
            Self::ExecutionContext => "execution_context",
            Self::OngoingWork => "ongoing_work",
            Self::AmbientMetadata => "ambient_metadata",
        }
    }

    /// Placement rank; `0` is the highest priority (pinned first, displaced last).
    pub const fn priority_rank(self) -> u8 {
        match self {
            Self::RecoveryCritical => 0,
            Self::ExecutionContext => 1,
            Self::OngoingWork => 2,
            Self::AmbientMetadata => 3,
        }
    }
}

/// A route through which every visible or overflowed status item must stay
/// reachable — never hover-only or pointer-only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StatusReachRoute {
    /// Reachable through keyboard search / focus traversal.
    KeyboardSearch,
    /// Reachable through the status-bar overflow menu.
    StatusMenu,
    /// Reachable through the command palette using the same label.
    PaletteRoute,
}

impl M5StatusReachRoute {
    /// Every reach route, in declaration order.
    pub const ALL: [Self; 3] = [Self::KeyboardSearch, Self::StatusMenu, Self::PaletteRoute];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardSearch => "keyboard_search",
            Self::StatusMenu => "status_menu",
            Self::PaletteRoute => "palette_route",
        }
    }
}

/// One of the four certification dimensions each status context is certified across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StatusBarProofDimension {
    /// Priority-class placement stability (no jitter, no vanity reflow).
    PlacementStability,
    /// Overflow discoverability (keyboard / menu / palette reach).
    OverflowDiscoverability,
    /// Inspector / command back-links (narrowest useful target).
    InspectorBacklink,
    /// Support / export parity (visible + overflowed items reconstructable).
    SupportExportParity,
}

impl M5StatusBarProofDimension {
    /// Every certification dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PlacementStability,
        Self::OverflowDiscoverability,
        Self::InspectorBacklink,
        Self::SupportExportParity,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PlacementStability => "placement_stability",
            Self::OverflowDiscoverability => "overflow_discoverability",
            Self::InspectorBacklink => "inspector_backlink",
            Self::SupportExportParity => "support_export_parity",
        }
    }
}

/// The derived certification light a governed status context carries.
///
/// `green` means the context keeps priority items in stable slots, every item and
/// overflow entry is keyboard/menu/palette-reachable, every item back-links to its
/// narrowest inspector, and the support export reconstructs all of it. `yellow` is a
/// disclosed narrowing (a waivered compact priority compaction, a reduced overflow
/// route, a grouped inspector back-link, or a partial support-export capture).
/// `red` is blocked: the priority slots jitter or reflow around a vanity item, the
/// overflow is hover- or pointer-only, an item's back-link is missing or dumps into
/// a generic settings detour, a critical-state displacement is absent from capture,
/// a status item is hover-only, or the priority order / reach routes / status-item
/// classes are incomplete — and the context may not keep a shell-maturity claim
/// until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatusBarCertificationStatus {
    /// Full standing: stable priority slots, reachable overflow, working back-links.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl StatusBarCertificationStatus {
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

/// How the context keeps its priority-class items in stable slots.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlacementStabilityState {
    /// Priority items hold stable slots; recovery-critical displaces vanity, never a
    /// truth-bearing peer, and the bar never jitters during active work.
    StablePrioritySlotsNoJitter,
    /// Under compact width, lower-priority items compact into a disclosed, waivered
    /// summary while recovery-critical / execution-context items stay pinned.
    DisclosedCompactPriorityCompaction,
    /// The priority slots jitter, or the bar reflows around a spinner / vanity item,
    /// or a severe state displaces a truth-bearing peer — always a blocker.
    UnstableSlotsOrVanityReflow,
}

impl PlacementStabilityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StablePrioritySlotsNoJitter => "stable_priority_slots_no_jitter",
            Self::DisclosedCompactPriorityCompaction => "disclosed_compact_priority_compaction",
            Self::UnstableSlotsOrVanityReflow => "unstable_slots_or_vanity_reflow",
        }
    }

    /// `true` when priority slots are stable.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::StablePrioritySlotsNoJitter)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedCompactPriorityCompaction)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::UnstableSlotsOrVanityReflow)
    }
}

/// How every visible and overflowed status item stays reachable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OverflowDiscoverabilityState {
    /// Every item and overflow entry is reachable through keyboard search, the
    /// status menu, and a palette route using the same label and explanation.
    KeyboardMenuPaletteReachable,
    /// One reach route is temporarily reduced but at least one non-hover route
    /// remains and the reduction is disclosed.
    DisclosedReducedOverflowRoute,
    /// The overflow is reachable only through hover or pointer — always a blocker.
    OverflowHoverOrPointerOnly,
}

impl OverflowDiscoverabilityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardMenuPaletteReachable => "keyboard_menu_palette_reachable",
            Self::DisclosedReducedOverflowRoute => "disclosed_reduced_overflow_route",
            Self::OverflowHoverOrPointerOnly => "overflow_hover_or_pointer_only",
        }
    }

    /// `true` when every reach route resolves.
    pub const fn is_reachable(self) -> bool {
        matches!(self, Self::KeyboardMenuPaletteReachable)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedOverflowRoute)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::OverflowHoverOrPointerOnly)
    }
}

/// How each status item links back to its narrowest useful inspector or command.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InspectorBacklinkState {
    /// Every item links back to the narrowest useful inspector or command.
    EveryItemBacklinksToNarrowestInspector,
    /// Some items share one disclosed grouped back-link rather than an individual
    /// narrowest target.
    DisclosedGroupedBacklink,
    /// An item's back-link is missing or dumps into a generic settings detour —
    /// always a blocker.
    BacklinkMissingOrGenericDetour,
}

impl InspectorBacklinkState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EveryItemBacklinksToNarrowestInspector => {
                "every_item_backlinks_to_narrowest_inspector"
            }
            Self::DisclosedGroupedBacklink => "disclosed_grouped_backlink",
            Self::BacklinkMissingOrGenericDetour => "backlink_missing_or_generic_detour",
        }
    }

    /// `true` when every item back-links to its narrowest inspector.
    pub const fn is_narrowest(self) -> bool {
        matches!(self, Self::EveryItemBacklinksToNarrowestInspector)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedGroupedBacklink)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::BacklinkMissingOrGenericDetour)
    }
}

/// How the support/export packet reconstructs the status bar without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportParityState {
    /// The export reconstructs the visible and overflowed items, their owning
    /// subsystems, and the current critical-state displacement.
    VisibleAndOverflowedItemsReconstructable,
    /// The export reconstructs the visible items and discloses a partial capture of
    /// the overflowed set.
    DisclosedPartialCapture,
    /// A critical-state displacement is absent from the capture — always a blocker.
    CriticalDisplacementAbsentFromCapture,
}

impl SupportExportParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VisibleAndOverflowedItemsReconstructable => {
                "visible_and_overflowed_items_reconstructable"
            }
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::CriticalDisplacementAbsentFromCapture => {
                "critical_displacement_absent_from_capture"
            }
        }
    }

    /// `true` when the export reconstructs everything.
    pub const fn is_reconstructable(self) -> bool {
        matches!(self, Self::VisibleAndOverflowedItemsReconstructable)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::CriticalDisplacementAbsentFromCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red placement narrowing
/// stay yellow rather than blocked — never lets an unstable reflow, a hover-only
/// overflow, a missing back-link, or an absent capture hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarCertificationWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed context the waiver applies to.
    pub context: M5StatusContext,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl StatusBarCertificationWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed context's certification.
///
/// The trigger token mirrors the frozen [`M5ShellPrimitiveDowngradeTrigger`]
/// vocabulary so a cause never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarCertificationCause {
    /// The governed context the cause applies to.
    pub context: M5StatusContext,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5ShellPrimitiveDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a
    /// non-disclosed cause is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl StatusBarCertificationCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed status context, certified across placement stability, overflow
/// discoverability, inspector back-links, and support-export parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarCertificationRow {
    /// The governed context being certified.
    pub context: M5StatusContext,
    /// The ambient status primitives this context drives. Pulled from the matrix.
    pub driven_primitive_families: Vec<M5ShellPrimitiveFamily>,
    /// The frozen qualification class of the driven status-bar-item primitive.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Owner role accountable for keeping this context certified.
    pub owner_role: String,
    /// Short context-status-bar label.
    pub context_label: String,
    /// The canonical shell zone the status bar attaches to. Pulled from the matrix.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// The priority classes kept in stable slots, in canonical recovery-critical
    /// order.
    pub certified_priority_classes: Vec<M5StatusPriorityClass>,
    /// The reach routes every item and overflow entry resolves through.
    pub certified_reach_routes: Vec<M5StatusReachRoute>,
    /// Status-item classes this context's status bar projects. Pulled from the
    /// matrix status-bar-item row.
    pub certified_status_item_classes: Vec<M5StatusItemClass>,
    /// Overflow / severe-state displacement behaviors. Pulled from the matrix.
    pub overflow_behaviors: Vec<M5OverflowBehavior>,
    /// Source / provider / freshness labels the items can show. Pulled from the
    /// matrix.
    pub source_freshness_labels: Vec<M5SourceFreshnessLabel>,
    /// Non-visual accessibility routes. Pulled from the matrix.
    pub accessibility_routes: Vec<M5AccessibilityRoute>,
    /// Mandatory labels every item must be able to show. Pulled from the matrix.
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// Shell subsystems this context stays aligned across. Pulled from the matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this context. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ShellPrimitiveDowngradeTrigger>,
    /// Placement-stability posture.
    pub placement_stability: PlacementStabilityState,
    /// Overflow-discoverability posture.
    pub overflow_discoverability: OverflowDiscoverabilityState,
    /// Inspector-backlink posture.
    pub inspector_backlink: InspectorBacklinkState,
    /// Support-export-parity posture.
    pub support_export_parity: SupportExportParityState,
    /// Hard invariant: every status item stays reachable without pointer hover.
    /// `false` is a blocker.
    pub keyboard_reachable_without_hover: bool,
    /// Active waiver, when a disclosed placement compaction is in force.
    pub active_waiver: Option<StatusBarCertificationWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: StatusBarCertificationStatus,
    /// The exact certification causes that narrowed or blocked this row.
    pub certification_causes: Vec<StatusBarCertificationCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl StatusBarCertificationRow {
    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when the certified priority classes are the canonical
    /// recovery-critical-first order (all four, in order) — the stable-placement
    /// lint.
    pub fn priority_order_well_formed(&self) -> bool {
        self.certified_priority_classes == M5StatusPriorityClass::ALL
    }

    /// `true` when every reach route is certified — the lint that prevents a
    /// hover-only or pointer-only overflow from shipping as stable.
    pub fn reach_routes_complete(&self) -> bool {
        let present: BTreeSet<M5StatusReachRoute> =
            self.certified_reach_routes.iter().copied().collect();
        M5StatusReachRoute::ALL
            .iter()
            .all(|route| present.contains(route))
    }

    /// `true` when every status-item class the matrix freezes is certified.
    pub fn status_item_classes_complete(&self) -> bool {
        let present: BTreeSet<M5StatusItemClass> =
            self.certified_status_item_classes.iter().copied().collect();
        M5StatusItemClass::ALL
            .iter()
            .all(|class| present.contains(class))
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        self.placement_stability.is_blocked()
            || self.overflow_discoverability.is_blocked()
            || self.inspector_backlink.is_blocked()
            || self.support_export_parity.is_blocked()
            || !self.keyboard_reachable_without_hover
            || !self.priority_order_well_formed()
            || !self.reach_routes_complete()
            || !self.status_item_classes_complete()
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.placement_stability.is_disclosed()
            || self.overflow_discoverability.is_disclosed()
            || self.inspector_backlink.is_disclosed()
            || self.support_export_parity.is_disclosed()
    }

    /// Recomputes the derived status from the four axes and the keyboard invariant.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest
    /// narrowing forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> StatusBarCertificationStatus {
        if self.has_hard_blocker() {
            StatusBarCertificationStatus::Red
        } else if self.has_narrowing() {
            StatusBarCertificationStatus::Yellow
        } else {
            StatusBarCertificationStatus::Green
        }
    }

    /// Recomputes the exact certification causes for the row, in deterministic order
    /// (placement, overflow, inspector back-link, support export, keyboard reach).
    pub fn recompute_causes(&self) -> Vec<StatusBarCertificationCause> {
        let mut causes = Vec::new();
        if !self.placement_stability.is_stable() {
            causes.push(StatusBarCertificationCause {
                context: self.context,
                trigger: if self.placement_stability.is_disclosed() {
                    M5ShellPrimitiveDowngradeTrigger::VanityItemReflow
                } else {
                    M5ShellPrimitiveDowngradeTrigger::SevereStateDisplacedTruth
                },
                disclosed: self.placement_stability.is_disclosed(),
                detail: if self.placement_stability.is_disclosed() {
                    "Under compact width the status bar performs a disclosed, waivered priority \
                     compaction that drops only ambient-metadata items; recovery-critical and \
                     execution-context items stay pinned in their stable slots."
                        .to_owned()
                } else {
                    "The status bar's priority slots jitter or reflow around a spinner / vanity \
                     item, or a severe state displaced a truth-bearing peer instead of a vanity \
                     item."
                        .to_owned()
                },
            });
        }
        if !self.overflow_discoverability.is_reachable() {
            causes.push(StatusBarCertificationCause {
                context: self.context,
                trigger: M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth,
                disclosed: self.overflow_discoverability.is_disclosed(),
                detail: if self.overflow_discoverability.is_disclosed() {
                    "One overflow reach route is temporarily reduced; at least one non-hover route \
                     (keyboard search, status menu, or palette) still resolves and the reduction \
                     is disclosed."
                        .to_owned()
                } else {
                    "The status overflow is reachable only through hover or pointer, with no \
                     keyboard-search, status-menu, or palette route."
                        .to_owned()
                },
            });
        }
        if !self.inspector_backlink.is_narrowest() {
            causes.push(StatusBarCertificationCause {
                context: self.context,
                trigger: M5ShellPrimitiveDowngradeTrigger::GroupedProgressUnattributed,
                disclosed: self.inspector_backlink.is_disclosed(),
                detail: if self.inspector_backlink.is_disclosed() {
                    "Some status items share one disclosed grouped inspector back-link rather than \
                     an individual narrowest target."
                        .to_owned()
                } else {
                    "A status item's inspector back-link is missing or dumps into a generic \
                     settings detour rather than the narrowest useful inspector or command."
                        .to_owned()
                },
            });
        }
        if !self.support_export_parity.is_reconstructable() {
            causes.push(StatusBarCertificationCause {
                context: self.context,
                trigger: M5ShellPrimitiveDowngradeTrigger::ProofStale,
                disclosed: self.support_export_parity.is_disclosed(),
                detail: if self.support_export_parity.is_disclosed() {
                    "The support export reconstructs the visible items and discloses a partial \
                     capture of the overflowed set."
                        .to_owned()
                } else {
                    "A critical-state displacement is absent from the support-export capture, so \
                     the ambient shell state cannot be reconstructed without a screenshot."
                        .to_owned()
                },
            });
        }
        if !self.keyboard_reachable_without_hover {
            causes.push(StatusBarCertificationCause {
                context: self.context,
                trigger: M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth,
                disclosed: false,
                detail: "A status item keeps critical truth reachable only through pointer hover."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed compact priority compaction may only stay yellow (rather than red)
    /// when a waiver discloses it.
    pub fn requires_waiver(&self) -> bool {
        self.placement_stability.is_disclosed()
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<StatusBarCertificationFinding> {
        let mut findings = Vec::new();
        let context = self.context.as_str().to_owned();

        if self.placement_stability.is_blocked() {
            findings.push(StatusBarCertificationFinding::UnstablePlacement {
                context: context.clone(),
            });
        }
        if self.overflow_discoverability.is_blocked() {
            findings.push(
                StatusBarCertificationFinding::OverflowNotKeyboardReachable {
                    context: context.clone(),
                },
            );
        }
        if self.inspector_backlink.is_blocked() {
            findings.push(StatusBarCertificationFinding::InspectorBacklinkMissing {
                context: context.clone(),
            });
        }
        if self.support_export_parity.is_blocked() {
            findings.push(
                StatusBarCertificationFinding::CriticalDisplacementAbsentFromCapture {
                    context: context.clone(),
                },
            );
        }
        if !self.keyboard_reachable_without_hover {
            findings.push(StatusBarCertificationFinding::KeyboardReachabilityLost {
                context: context.clone(),
            });
        }
        if !self.priority_order_well_formed() {
            findings.push(StatusBarCertificationFinding::PriorityOrderStale {
                context: context.clone(),
            });
        }
        if !self.reach_routes_complete() {
            findings.push(StatusBarCertificationFinding::ReachRoutesIncomplete {
                context: context.clone(),
            });
        }
        if !self.status_item_classes_complete() {
            findings.push(StatusBarCertificationFinding::StatusItemClassesIncomplete {
                context: context.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, StatusBarCertificationStatus::Green) && !self.has_reason() {
            findings.push(StatusBarCertificationFinding::NarrowedRowWithoutReason {
                context: context.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry
        // an active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(StatusBarCertificationFinding::NarrowedRowWithoutWaiver {
                context: context.clone(),
            });
        }
        // An attached waiver must still be active and must point at this context.
        if let Some(waiver) = &self.active_waiver {
            if waiver.context != self.context {
                findings.push(StatusBarCertificationFinding::WaiverContextMismatch {
                    context: context.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(StatusBarCertificationFinding::WaiverExpired {
                    context: context.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(StatusBarCertificationFinding::RowStatusStale {
                context: context.clone(),
            });
        }
        if self.certification_causes != self.recompute_causes() {
            findings.push(StatusBarCertificationFinding::RowCausesStale { context });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} placement={} overflow={} backlink={} export={} kbd={} waiver={}",
            self.context.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.placement_stability.as_str(),
            self.overflow_discoverability.as_str(),
            self.inspector_backlink.as_str(),
            self.support_export_parity.as_str(),
            self.keyboard_reachable_without_hover,
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the status-bar certification proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum StatusBarCertificationFinding {
    /// A governed status context has no certification row.
    ContextMissing {
        /// The missing context token.
        context: String,
    },
    /// A context's priority slots jitter or reflow around a vanity item.
    UnstablePlacement {
        /// The context token.
        context: String,
    },
    /// A context's overflow is reachable only through hover or pointer.
    OverflowNotKeyboardReachable {
        /// The context token.
        context: String,
    },
    /// A context has an item whose inspector back-link is missing or generic.
    InspectorBacklinkMissing {
        /// The context token.
        context: String,
    },
    /// A context's critical-state displacement is absent from the support export.
    CriticalDisplacementAbsentFromCapture {
        /// The context token.
        context: String,
    },
    /// A context keeps critical truth reachable only through pointer hover.
    KeyboardReachabilityLost {
        /// The context token.
        context: String,
    },
    /// A context's certified priority classes are not the canonical order.
    PriorityOrderStale {
        /// The context token.
        context: String,
    },
    /// A context does not certify every reach route.
    ReachRoutesIncomplete {
        /// The context token.
        context: String,
    },
    /// A context does not certify every frozen status-item class.
    StatusItemClassesIncomplete {
        /// The context token.
        context: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The context token.
        context: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The context token.
        context: String,
    },
    /// An attached waiver does not point at the row's context.
    WaiverContextMismatch {
        /// The context token.
        context: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The context token.
        context: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The context token.
        context: String,
    },
    /// The declared certification causes do not match the recomputed causes.
    RowCausesStale {
        /// The context token.
        context: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered contexts do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl StatusBarCertificationFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ContextMissing { .. } => "context_missing",
            Self::UnstablePlacement { .. } => "unstable_placement",
            Self::OverflowNotKeyboardReachable { .. } => "overflow_not_keyboard_reachable",
            Self::InspectorBacklinkMissing { .. } => "inspector_backlink_missing",
            Self::CriticalDisplacementAbsentFromCapture { .. } => {
                "critical_displacement_absent_from_capture"
            }
            Self::KeyboardReachabilityLost { .. } => "keyboard_reachability_lost",
            Self::PriorityOrderStale { .. } => "priority_order_stale",
            Self::ReachRoutesIncomplete { .. } => "reach_routes_incomplete",
            Self::StatusItemClassesIncomplete { .. } => "status_item_classes_incomplete",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverContextMismatch { .. } => "waiver_context_mismatch",
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
            Self::ContextMissing { context }
            | Self::UnstablePlacement { context }
            | Self::OverflowNotKeyboardReachable { context }
            | Self::InspectorBacklinkMissing { context }
            | Self::CriticalDisplacementAbsentFromCapture { context }
            | Self::KeyboardReachabilityLost { context }
            | Self::PriorityOrderStale { context }
            | Self::ReachRoutesIncomplete { context }
            | Self::StatusItemClassesIncomplete { context }
            | Self::NarrowedRowWithoutReason { context }
            | Self::NarrowedRowWithoutWaiver { context }
            | Self::WaiverContextMismatch { context, .. }
            | Self::WaiverExpired { context, .. }
            | Self::RowStatusStale { context }
            | Self::RowCausesStale { context } => context,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release certification packet shared by the status bar / attention router /
/// release automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarCertificationPacket {
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
    /// The certification dimensions every context is certified across.
    pub required_proof_dimensions: Vec<M5StatusBarProofDimension>,
    /// The priority classes every context keeps in stable slots.
    pub required_priority_classes: Vec<M5StatusPriorityClass>,
    /// The reach routes every item and overflow entry must resolve through.
    pub required_reach_routes: Vec<M5StatusReachRoute>,
    /// Per-context certification rows, in canonical order.
    pub rows: Vec<StatusBarCertificationRow>,
    /// Governed contexts certified, in canonical (sorted) order.
    pub covered_contexts: Vec<String>,
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
    pub active_waivers: Vec<StatusBarCertificationWaiver>,
    /// Every exact certification cause, in row then cause order.
    pub certification_causes: Vec<StatusBarCertificationCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<StatusBarCertificationFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Shell / release automation refs that consume this packet to auto-narrow
    /// claimed status contexts.
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

impl StatusBarCertificationPacket {
    /// Returns the certification row for `context`, if present.
    pub fn row(&self, context: M5StatusContext) -> Option<&StatusBarCertificationRow> {
        self.rows.iter().find(|row| row.context == context)
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
                waiver.context.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.certification_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.context.as_str(),
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

    /// Projects the light certification dashboard the status-bar automation consumes.
    pub fn dashboard(&self) -> StatusBarCertificationDashboard {
        StatusBarCertificationDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 status-bar certification packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per context.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "context,status,qualification,shell_zone_slot,placement_stability,overflow_discoverability,inspector_backlink,support_export_parity,keyboard_reachable,priority_classes,reach_routes,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{},{}\n",
                row.context.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.shell_zone_slot.as_str(),
                row.placement_stability.as_str(),
                row.overflow_discoverability.as_str(),
                row.inspector_backlink.as_str(),
                row.support_export_parity.as_str(),
                row.keyboard_reachable_without_hover,
                join_tokens(&row.certified_priority_classes, |c| c.as_str()),
                join_tokens(&row.certified_reach_routes, |r| r.as_str()),
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
            "# M5 status-bar item priority, placement, overflow & inspector back-links\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_status_bar_certification`](../../crates/aureline-shell/src/m5_status_bar_certification/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_status_bar_certification -- markdown > \\\n  artifacts/shell/m5-status-bar-certification.md\n",
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

        out.push_str("## Priority classes (stable placement order)\n\n");
        for class in &self.required_priority_classes {
            out.push_str(&format!(
                "- `{}` (rank {})\n",
                class.as_str(),
                class.priority_rank()
            ));
        }
        out.push('\n');

        out.push_str("## Certification rows\n\n");
        out.push_str(
            "| Context | Status | Qualification | Placement | Overflow | Back-link | Export | Keyboard | Waiver |\n\
             | ------- | ------ | ------------- | --------- | -------- | --------- | ------ | -------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.context_label,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.placement_stability.as_str(),
                row.overflow_discoverability.as_str(),
                row.inspector_backlink.as_str(),
                row.support_export_parity.as_str(),
                row.keyboard_reachable_without_hover,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&StatusBarCertificationRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, StatusBarCertificationStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str("None — every governed status context is certified at full standing.\n\n");
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.context.as_str(),
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
                    cause.context.as_str(),
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
                    waiver.context.as_str(),
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_status_bar_certification -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_status_bar_certification_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarCertificationDashboardRow {
    /// The governed context.
    pub context: M5StatusContext,
    /// Short context-status-bar label.
    pub context_label: String,
    /// Derived green/yellow/red status.
    pub status: StatusBarCertificationStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Placement-stability posture.
    pub placement_stability: PlacementStabilityState,
    /// Overflow-discoverability posture.
    pub overflow_discoverability: OverflowDiscoverabilityState,
    /// Inspector-backlink posture.
    pub inspector_backlink: InspectorBacklinkState,
    /// Support-export-parity posture.
    pub support_export_parity: SupportExportParityState,
    /// `true` when every status item is keyboard-reachable without hover.
    pub keyboard_reachable_without_hover: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the status bar / attention router / release
/// automation reads to auto-narrow claimed status contexts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarCertificationDashboard {
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
    pub rows: Vec<StatusBarCertificationDashboardRow>,
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

impl StatusBarCertificationDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &StatusBarCertificationPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| StatusBarCertificationDashboardRow {
                context: row.context,
                context_label: row.context_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                placement_stability: row.placement_stability,
                overflow_discoverability: row.overflow_discoverability,
                inspector_backlink: row.inspector_backlink,
                support_export_parity: row.support_export_parity,
                keyboard_reachable_without_hover: row.keyboard_reachable_without_hover,
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
            record_kind: M5_STATUS_BAR_CERTIFICATION_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_STATUS_BAR_CERTIFICATION_SCHEMA_VERSION,
            dashboard_id: M5_STATUS_BAR_CERTIFICATION_DASHBOARD_ID.to_owned(),
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
            .expect("m5 status-bar certification dashboard serializes")
    }
}

/// Support-export wrapper for the status-bar certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StatusBarCertificationSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: StatusBarCertificationPacket,
    /// Dashboard quoted in full.
    pub dashboard: StatusBarCertificationDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl StatusBarCertificationSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each context, and
    /// each active waiver id is quoted as a case id so a support reviewer — or the
    /// status-bar automation — can name the same context and waiver the runtime
    /// certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: StatusBarCertificationPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.context.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_STATUS_BAR_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_STATUS_BAR_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: M5_STATUS_BAR_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_status_bar_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatusBarCertificationInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-primitives matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-context certification rows.
    pub rows: Vec<StatusBarCertificationRow>,
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

/// Builds a [`StatusBarCertificationPacket`] from the exact build identity, the
/// frozen matrix ref, and the per-context certification rows.
///
/// Each row's derived status and certification causes, the aggregate counts, the
/// active waivers, and the blocking findings are recomputed here so the packet is
/// the single source of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_status_bar_certification_packet(
    input: StatusBarCertificationInput,
) -> StatusBarCertificationPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is
    // self-consistent and the auto-narrowing is the single source of truth.
    let rows: Vec<StatusBarCertificationRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.certification_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<StatusBarCertificationFinding> = Vec::new();

    // Every governed context must carry a certification row.
    let present: BTreeSet<M5StatusContext> = rows.iter().map(|row| row.context).collect();
    for context in M5StatusContext::ALL {
        if !present.contains(&context) {
            blocking_findings.push(StatusBarCertificationFinding::ContextMissing {
                context: context.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_contexts: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|context| context.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, StatusBarCertificationStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, StatusBarCertificationStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, StatusBarCertificationStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(StatusBarCertificationFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<StatusBarCertificationWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let certification_causes: Vec<StatusBarCertificationCause> = rows
        .iter()
        .flat_map(|row| row.certification_causes.clone())
        .collect();

    let mut packet = StatusBarCertificationPacket {
        record_kind: M5_STATUS_BAR_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_STATUS_BAR_CERTIFICATION_SCHEMA_VERSION,
        shared_contract_ref: M5_STATUS_BAR_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_STATUS_BAR_CERTIFICATION_PACKET_ID.to_owned(),
        source_schema_ref: M5_STATUS_BAR_CERTIFICATION_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Status-bar item priority classes, stable placement, overflow discoverability, \
                   and inspector back-links certified across every claimed M5 status context: \
                   notebook, data/API, remote, preview, review, profiler, incident, and desktop \
                   base each keep recovery-critical, execution-context, ongoing-work, and \
                   ambient-metadata items in stable priority slots, keep every visible and \
                   overflowed item keyboard/menu/palette-reachable, link every item back to its \
                   narrowest inspector, and reconstruct the bar from a support export — with each \
                   row's green/yellow/red claim auto-narrowed from its placement, overflow, \
                   back-link, and support-export posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_STATUS_BAR_CERTIFICATION_MATRIX_SCHEMA_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_proof_dimensions: M5StatusBarProofDimension::ALL.to_vec(),
        required_priority_classes: M5StatusPriorityClass::ALL.to_vec(),
        required_reach_routes: M5StatusReachRoute::ALL.to_vec(),
        rows,
        covered_contexts,
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
            "shell_frame.status_bar.priority_registry".to_owned(),
            "release_automation.auto_narrow.status_bar_certification_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.status_bar_certification".to_owned(),
            "artifacts/release/m5-status-bar-certification-proof/packet.json".to_owned(),
        ],
        help_docs_refs: vec![M5_STATUS_BAR_CERTIFICATION_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-status-bar-certification".to_owned()],
        published_report_ref: M5_STATUS_BAR_CERTIFICATION_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_STATUS_BAR_CERTIFICATION_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_STATUS_BAR_CERTIFICATION_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_STATUS_BAR_CERTIFICATION_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(StatusBarCertificationFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_status_bar_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum StatusBarCertificationValidationError {
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
    /// The rows do not cover all eight governed contexts.
    CoverageIncomplete,
    /// The declared covered contexts do not match the rows.
    CoverageStale,
    /// The declared required proof dimensions are not the canonical set.
    RequiredDimensionsStale,
    /// The declared required priority classes are not the canonical set.
    RequiredPriorityClassesStale,
    /// The declared required reach routes are not the canonical set.
    RequiredReachRoutesStale,
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

/// Validates a packet against the status-bar certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed
/// status context carries a current certification row; each row's status is the
/// derived auto-narrowed value, never asserted; a green row cannot keep a claim while
/// its priority slots jitter, its overflow is hover-only, an item's back-link is
/// missing, a critical displacement is absent from capture, an item is hover-only, or
/// its priority order / reach routes / status-item classes are incomplete; and a
/// disclosed narrowing is backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_status_bar_certification_packet(
    packet: &StatusBarCertificationPacket,
) -> Result<(), Vec<StatusBarCertificationValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(StatusBarCertificationValidationError::NoRows);
    }
    if packet.record_kind != M5_STATUS_BAR_CERTIFICATION_PACKET_RECORD_KIND {
        errors.push(StatusBarCertificationValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_STATUS_BAR_CERTIFICATION_SCHEMA_VERSION {
        errors.push(StatusBarCertificationValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(StatusBarCertificationValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(StatusBarCertificationValidationError::MatrixPacketRefMissing);
    }
    if packet.required_proof_dimensions != M5StatusBarProofDimension::ALL {
        errors.push(StatusBarCertificationValidationError::RequiredDimensionsStale);
    }
    if packet.required_priority_classes != M5StatusPriorityClass::ALL {
        errors.push(StatusBarCertificationValidationError::RequiredPriorityClassesStale);
    }
    if packet.required_reach_routes != M5StatusReachRoute::ALL {
        errors.push(StatusBarCertificationValidationError::RequiredReachRoutesStale);
    }

    let present: BTreeSet<M5StatusContext> = packet.rows.iter().map(|row| row.context).collect();
    let coverage_complete = M5StatusContext::ALL
        .iter()
        .all(|context| present.contains(context));
    if !coverage_complete || packet.rows.len() != M5StatusContext::ALL.len() {
        errors.push(StatusBarCertificationValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|context| context.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_contexts {
        errors.push(StatusBarCertificationValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), StatusBarCertificationStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), StatusBarCertificationStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), StatusBarCertificationStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(StatusBarCertificationValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<StatusBarCertificationWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(StatusBarCertificationValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<StatusBarCertificationCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.certification_causes {
        errors.push(StatusBarCertificationValidationError::CertificationCausesStale);
    }

    let mut recomputed: Vec<StatusBarCertificationFinding> = Vec::new();
    for context in M5StatusContext::ALL {
        if !present.contains(&context) {
            recomputed.push(StatusBarCertificationFinding::ContextMissing {
                context: context.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(StatusBarCertificationFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(StatusBarCertificationFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(StatusBarCertificationValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(
            StatusBarCertificationValidationError::BlockingFindingPresent {
                class: finding.class_token().to_owned(),
                subject_ref: finding.subject_ref().to_owned(),
            },
        );
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(StatusBarCertificationValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(StatusBarCertificationValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(StatusBarCertificationValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(StatusBarCertificationValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
