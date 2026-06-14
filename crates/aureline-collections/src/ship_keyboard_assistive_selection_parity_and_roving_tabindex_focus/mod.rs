//! Keyboard and assistive-technology selection parity, roving-tabindex (or
//! `aria-activedescendant`) dense-collection focus, and offscreen-selection
//! durability for query-backed M5 dense surfaces.
//!
//! Where
//! [`crate::implement_selection_bars_range_anchor_and_stale_snapshot_guards`]
//! made the *selection state* of a dense surface a canonical product object
//! (membership by stable identity, range-anchor identity, hidden-selected counts,
//! and a stale-query-snapshot guard), this module makes that same selection model
//! **fully keyboard and screen-reader usable** so offscreen virtualization,
//! streaming inserts, and query-backed selection never collapse into a
//! pointer-only or inaccessible batch model.
//!
//! Each [`AssistiveSelectionProfile`] pins one [`DenseCollectionSurface`],
//! rendered as a [`CollectionViewKind`] under a [`CollectionDataMode`], to:
//!
//! - a [`RovingFocusModel`] — a roving-tabindex or `aria-activedescendant`
//!   dense-focus behavior with a single tabstop, focus tracked by **stable item
//!   id** rather than row index, a visible focus indicator, and arrow-key
//!   navigation — so focus has one predictable home and survives churn;
//! - the full set of [`AssistiveCommand`]s the track demands — select-current,
//!   extend-range, clear-selection, inspect-hidden-count, and open-batch-review —
//!   each with a keyboard binding, an accessible name, and an announcement, every
//!   one keyboard- and screen-reader-reachable and never pointer-only;
//! - a [`SelectionAnnouncement`] live-region contract that announces the selection
//!   count and, crucially, the **hidden-selected count** to assistive technology;
//! - per-event [`FocusChurnResilience`] records proving streaming inserts,
//!   background refreshes, sort/filter changes, and virtualization recycling never
//!   steal focus or corrupt durable selection; and
//! - an [`OffscreenSelectionDurability`] record proving selection of
//!   offscreen/virtualized members persists by stable identity with the
//!   hidden-selected count exposed to assistive technology.
//!
//! The lane carries the track guardrails: a row highlight never stands in for
//! durable selection, no selection control is pointer-only, streaming and
//! virtualization never steal focus, selection survives sort/filter/virtualization
//! by stable identity, the hidden-selected count is always exposed to assistive
//! technology, and broad-action review is keyboard reachable.
//! [`AssistiveSelectionProfile::reconstruction`] projects the same truth into a
//! redaction-aware [`AssistiveSelectionProfileReconstruction`] that diagnostics
//! and accessibility-evidence packets reuse instead of re-deriving parity from
//! raw rows.
//!
//! The boundary schema is
//! [`schemas/collections/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr.schema.json`](../../../../schemas/collections/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr.schema.json).
//! The contract doc is
//! [`docs/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr.md`](../../../../docs/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr.md).
//! The protected fixture directory is
//! [`fixtures/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr/`](../../../../fixtures/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_filter_ast_saved_view_column_preset_and_batch_action_descriptor_matrix::DenseCollectionSurface;
use crate::implement_selection_bars_range_anchor_and_stale_snapshot_guards::CollectionDataMode;
use crate::ship_result_scope_counters_and_hidden_narrowing_chips::CollectionViewKind;

/// Stable record-kind tag carried by [`AssistiveSelectionParityPacket`].
pub const ASSISTIVE_SELECTION_PARITY_RECORD_KIND: &str = "m5_assistive_selection_parity_packet";

/// Integer schema version for the assistive-selection-parity packet.
pub const ASSISTIVE_SELECTION_PARITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const ASSISTIVE_SELECTION_PARITY_SCHEMA_REF: &str =
    "schemas/collections/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr.schema.json";

/// Repo-relative path of the contract doc.
pub const ASSISTIVE_SELECTION_PARITY_DOC_REF: &str =
    "docs/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr.md";

/// Repo-relative path of the protected fixture directory.
pub const ASSISTIVE_SELECTION_PARITY_FIXTURE_DIR: &str =
    "fixtures/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr";

/// Repo-relative path of the checked support-export artifact.
pub const ASSISTIVE_SELECTION_PARITY_ARTIFACT_REF: &str =
    "artifacts/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const ASSISTIVE_SELECTION_PARITY_SUMMARY_REF: &str =
    "artifacts/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr.md";

/// The first real M5 dense surfaces this lane wires onto canonical assistive
/// selection profiles.
const REQUIRED_PARITY_SURFACES: [DenseCollectionSurface; 6] = [
    DenseCollectionSurface::PipelineRunList,
    DenseCollectionSurface::ReviewQueue,
    DenseCollectionSurface::IncidentList,
    DenseCollectionSurface::GraphList,
    DenseCollectionSurface::MarketplaceResults,
    DenseCollectionSurface::ProviderAdminTable,
];

/// Data modes the parity contract must survive. A profile must keep focus and
/// selection durable whether the collection is filtered/sorted, streaming, or
/// virtualized.
const REQUIRED_PARITY_DATA_MODES: [CollectionDataMode; 3] = [
    CollectionDataMode::FilteredSorted,
    CollectionDataMode::Streaming,
    CollectionDataMode::Virtualized,
];

/// Churn events focus and selection must survive without theft or corruption.
const REQUIRED_CHURN_EVENTS: [FocusChurnEvent; 4] = [
    FocusChurnEvent::StreamingInsert,
    FocusChurnEvent::BackgroundRefresh,
    FocusChurnEvent::SortOrFilterChange,
    FocusChurnEvent::VirtualizationRecycle,
];

/// How a dense surface implements single-home keyboard focus. Both behaviors are
/// "roving" in that exactly one item is the focus target at a time; the lane
/// accepts either so list/tree/table/queue surfaces are not forced onto one DOM
/// idiom.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FocusModelKind {
    /// Roving tabindex: the focused item carries `tabindex=0`, every other item
    /// `tabindex=-1`, and arrow keys move the single tabstop.
    RovingTabindex,
    /// A single container tabstop whose `aria-activedescendant` points at the
    /// active item; arrow keys move the active descendant.
    AriaActivedescendant,
}

impl FocusModelKind {
    /// Every focus model the lane recognizes.
    pub const ALL: [Self; 2] = [Self::RovingTabindex, Self::AriaActivedescendant];

    /// Stable token recorded in packets, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RovingTabindex => "roving_tabindex",
            Self::AriaActivedescendant => "aria_activedescendant",
        }
    }
}

/// The keyboard/assistive selection commands every claimed dense surface must
/// expose. These are the operations that must be reachable without a pointer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AssistiveCommandKind {
    /// Select (or toggle) the currently focused item.
    SelectCurrent,
    /// Extend the selection as a range from the anchor to the current focus.
    ExtendRange,
    /// Clear the entire selection.
    ClearSelection,
    /// Announce the hidden-selected count (selected items outside the current
    /// filter / offscreen).
    InspectHiddenCount,
    /// Open batch review for the current selection before a broad action runs.
    OpenBatchReview,
}

impl AssistiveCommandKind {
    /// Every required command, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SelectCurrent,
        Self::ExtendRange,
        Self::ClearSelection,
        Self::InspectHiddenCount,
        Self::OpenBatchReview,
    ];

    /// Stable token recorded in packets, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectCurrent => "select_current",
            Self::ExtendRange => "extend_range",
            Self::ClearSelection => "clear_selection",
            Self::InspectHiddenCount => "inspect_hidden_count",
            Self::OpenBatchReview => "open_batch_review",
        }
    }
}

/// A churn event that must not steal focus or corrupt durable selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FocusChurnEvent {
    /// New rows arrive live while a selection and focus are held.
    StreamingInsert,
    /// The dataset is refreshed in the background.
    BackgroundRefresh,
    /// The list is re-sorted or re-filtered.
    SortOrFilterChange,
    /// The virtualizer recycles row DOM as the viewport scrolls.
    VirtualizationRecycle,
}

impl FocusChurnEvent {
    /// Every churn event the lane recognizes.
    pub const ALL: [Self; 4] = [
        Self::StreamingInsert,
        Self::BackgroundRefresh,
        Self::SortOrFilterChange,
        Self::VirtualizationRecycle,
    ];

    /// Stable token recorded in packets, fixtures, and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StreamingInsert => "streaming_insert",
            Self::BackgroundRefresh => "background_refresh",
            Self::SortOrFilterChange => "sort_or_filter_change",
            Self::VirtualizationRecycle => "virtualization_recycle",
        }
    }
}

/// What happened to keyboard focus across a churn event. Both are correct
/// behaviors; what is forbidden is focus theft (jumping to the top, or being
/// lost), which is modeled by [`FocusChurnResilience::focus_not_stolen`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FocusDurabilityOutcome {
    /// The focused item is still present; focus stays on the same stable id.
    FocusHeldByIdentity,
    /// The focused item left the view; focus re-anchors on a precise visible
    /// neighbor, announced to assistive technology.
    FocusReanchoredVisible,
}

impl FocusDurabilityOutcome {
    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FocusHeldByIdentity => "focus_held_by_identity",
            Self::FocusReanchoredVisible => "focus_reanchored_visible",
        }
    }

    /// True when the focused item departed and focus had to re-anchor.
    pub const fn focused_item_departed(self) -> bool {
        matches!(self, Self::FocusReanchoredVisible)
    }
}

/// How an `aria-live` region announces selection changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LiveRegionPoliteness {
    /// Announced at the next graceful opportunity (`aria-live="polite"`).
    Polite,
    /// Announced immediately, interrupting (`aria-live="assertive"`).
    Assertive,
}

impl LiveRegionPoliteness {
    /// Stable token recorded in packets and exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Polite => "polite",
            Self::Assertive => "assertive",
        }
    }
}

/// A roving dense-focus model: one keyboard home that survives churn by stable
/// identity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RovingFocusModel {
    /// Roving-tabindex or `aria-activedescendant`.
    pub kind: FocusModelKind,
    /// True when exactly one item is the focus target at a time (required).
    pub single_tabstop: bool,
    /// True when the focus target is tracked by stable item id, not row index
    /// (required).
    pub focus_tracked_by_stable_identity: bool,
    /// True when a visible focus indicator is always rendered (required).
    pub focus_visible_indicator: bool,
    /// True when arrow keys move focus across the collection (required).
    pub arrow_key_navigation: bool,
    /// Precise label describing navigation behavior at the ends of the collection
    /// (wrap or clamp). Required and non-generic.
    pub navigation_bound_label: String,
}

impl RovingFocusModel {
    /// Whether the focus model is well formed: a single tabstop, tracked by stable
    /// identity, with a visible indicator, arrow-key navigation, and a precise
    /// navigation-bound label.
    pub fn is_valid(&self) -> bool {
        self.single_tabstop
            && self.focus_tracked_by_stable_identity
            && self.focus_visible_indicator
            && self.arrow_key_navigation
            && !label_is_generic(&self.navigation_bound_label)
    }
}

/// One keyboard/assistive command bound to a dense surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistiveCommand {
    /// Which command this is.
    pub kind: AssistiveCommandKind,
    /// Keyboard binding, e.g. `Space`, `Shift+ArrowDown`, `Escape`. Required and
    /// non-generic.
    pub keyboard_binding: String,
    /// Accessible name announced for the command. Required and non-generic.
    pub accessible_name: String,
    /// Live-region announcement emitted when the command runs. Required and
    /// non-generic.
    pub announcement: String,
    /// True when the command is reachable from the keyboard (required).
    pub keyboard_reachable: bool,
    /// True when the command is reachable and described to a screen reader
    /// (required).
    pub screen_reader_reachable: bool,
    /// True when the command is *only* operable with a pointer. Must be false — a
    /// row highlight or pointer-only control never stands in for a keyboard path.
    pub pointer_only: bool,
}

impl AssistiveCommand {
    /// Whether the command is well formed and not pointer-only.
    pub fn is_valid(&self) -> bool {
        !label_is_generic(&self.keyboard_binding)
            && !label_is_generic(&self.accessible_name)
            && !label_is_generic(&self.announcement)
            && self.keyboard_reachable
            && self.screen_reader_reachable
            && !self.pointer_only
            && self.exposes_hidden_count_when_inspecting()
    }

    /// The inspect-hidden-count command must actually name the hidden / outside /
    /// offscreen population in its announcement, so the user hears the scope a
    /// broad action would touch.
    pub fn exposes_hidden_count_when_inspecting(&self) -> bool {
        if self.kind != AssistiveCommandKind::InspectHiddenCount {
            return true;
        }
        let lower = self.announcement.to_lowercase();
        lower.contains("hidden") || lower.contains("outside") || lower.contains("offscreen")
    }
}

/// The `aria-live` announcement contract for selection changes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionAnnouncement {
    /// Politeness of the live region.
    pub politeness: LiveRegionPoliteness,
    /// True when selection-count changes are announced (required).
    pub announces_selection_count: bool,
    /// True when the hidden-selected count is announced to assistive technology
    /// (required).
    pub announces_hidden_selected_count: bool,
    /// True when opening batch review is announced.
    pub announces_batch_review_open: bool,
    /// A representative announcement string. Required and non-generic.
    pub sample_announcement: String,
}

impl SelectionAnnouncement {
    /// Whether the announcement contract is well formed: it announces the
    /// selection count and the hidden-selected count, and carries a precise
    /// sample.
    pub fn is_valid(&self) -> bool {
        self.announces_selection_count
            && self.announces_hidden_selected_count
            && !label_is_generic(&self.sample_announcement)
    }
}

/// How focus and selection behaved across one churn event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FocusChurnResilience {
    /// The churn event this record describes.
    pub event: FocusChurnEvent,
    /// What happened to keyboard focus.
    pub outcome: FocusDurabilityOutcome,
    /// True when durable selection was preserved by stable identity across the
    /// event (required).
    pub selection_preserved: bool,
    /// True when the event did not steal focus (no jump to top, no lost focus)
    /// (required).
    pub focus_not_stolen: bool,
    /// True when any focus or selection change was announced to assistive
    /// technology (required).
    pub change_announced: bool,
    /// Precise label describing the behavior. Required and non-generic; a departed
    /// focus must describe how it re-anchored.
    pub detail_label: String,
}

impl FocusChurnResilience {
    /// Whether the resilience record is well formed: selection preserved, focus
    /// not stolen, change announced, and a precise detail label.
    pub fn is_valid(&self) -> bool {
        self.selection_preserved
            && self.focus_not_stolen
            && self.change_announced
            && !label_is_generic(&self.detail_label)
    }
}

/// Durability of selection for offscreen / virtualized members.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OffscreenSelectionDurability {
    /// True when selection survives virtualization recycling (required).
    pub selection_survives_virtualization_recycle: bool,
    /// True when offscreen members are tracked by stable identity (required).
    pub offscreen_members_tracked_by_identity: bool,
    /// True when the hidden/offscreen-selected count is exposed to assistive
    /// technology (required).
    pub hidden_selected_count_exposed_to_at: bool,
    /// Count of selected items currently offscreen / virtualized away.
    pub offscreen_selected_count: u64,
    /// Precise label describing the offscreen-selection behavior. Required and
    /// non-generic.
    pub detail_label: String,
}

impl OffscreenSelectionDurability {
    /// Whether the offscreen-durability record is well formed.
    pub fn is_valid(&self) -> bool {
        self.selection_survives_virtualization_recycle
            && self.offscreen_members_tracked_by_identity
            && self.hidden_selected_count_exposed_to_at
            && !label_is_generic(&self.detail_label)
    }
}

/// Redaction-aware projection of one assistive selection profile for diagnostics
/// and accessibility-evidence packets. Carries only ids, tokens, labels, and
/// counts — never raw row bodies or provider payloads.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistiveSelectionProfileReconstruction {
    /// Profile id this reconstruction projects.
    pub profile_id: String,
    /// Surface token.
    pub surface_token: String,
    /// View-kind token.
    pub view_kind_token: String,
    /// Data-mode token.
    pub data_mode_token: String,
    /// Focus-model token.
    pub focus_model_token: String,
    /// True when focus is tracked by stable identity.
    pub focus_by_stable_identity: bool,
    /// Command tokens covered by this profile.
    pub command_tokens: Vec<String>,
    /// True when no command is pointer-only and every command is keyboard- and
    /// screen-reader-reachable.
    pub all_commands_keyboard_reachable: bool,
    /// True when the hidden-selected count is exposed to assistive technology.
    pub hidden_selected_count_exposed: bool,
    /// Churn-event tokens this profile proves resilient against.
    pub churn_event_tokens: Vec<String>,
    /// True when no churn event steals focus or corrupts selection.
    pub focus_and_selection_durable: bool,
    /// Count of selected items currently offscreen / virtualized away.
    pub offscreen_selected_count: u64,
}

/// One assistive selection parity profile for a dense M5 surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistiveSelectionProfile {
    /// Stable profile id.
    pub profile_id: String,
    /// Bound dense collection surface.
    pub surface: DenseCollectionSurface,
    /// How the surface is rendered.
    pub view_kind: CollectionViewKind,
    /// How the collection is materialized while focus and selection are held.
    pub data_mode: CollectionDataMode,
    /// Human-readable label summary.
    pub label_summary: String,
    /// Roving dense-focus model.
    pub focus_model: RovingFocusModel,
    /// Keyboard/assistive commands. Must cover every [`AssistiveCommandKind`].
    pub commands: Vec<AssistiveCommand>,
    /// Live-region selection announcement contract.
    pub announcement: SelectionAnnouncement,
    /// Per-event focus/selection churn resilience.
    pub churn_resilience: Vec<FocusChurnResilience>,
    /// Offscreen / virtualized selection durability.
    pub offscreen_durability: OffscreenSelectionDurability,
    /// Accessibility evidence packet refs backing this profile.
    pub evidence_refs: Vec<String>,
}

impl AssistiveSelectionProfile {
    /// Command kinds covered by this profile.
    pub fn covered_commands(&self) -> BTreeSet<AssistiveCommandKind> {
        self.commands.iter().map(|command| command.kind).collect()
    }

    /// Churn events this profile proves resilient against.
    pub fn covered_churn_events(&self) -> BTreeSet<FocusChurnEvent> {
        self.churn_resilience
            .iter()
            .map(|resilience| resilience.event)
            .collect()
    }

    /// Whether every required command is present.
    pub fn covers_required_commands(&self) -> bool {
        let covered = self.covered_commands();
        AssistiveCommandKind::ALL
            .iter()
            .all(|kind| covered.contains(kind))
    }

    /// Whether no command is pointer-only and every command is keyboard- and
    /// screen-reader-reachable.
    pub fn all_commands_keyboard_reachable(&self) -> bool {
        self.commands.iter().all(|command| {
            command.keyboard_reachable && command.screen_reader_reachable && !command.pointer_only
        })
    }

    /// Whether the hidden-selected count is exposed to assistive technology, in
    /// both the announcement contract and the offscreen-durability record.
    pub fn hidden_selected_count_exposed(&self) -> bool {
        self.announcement.announces_hidden_selected_count
            && self
                .offscreen_durability
                .hidden_selected_count_exposed_to_at
    }

    /// Whether no churn event steals focus or corrupts durable selection.
    pub fn focus_and_selection_durable(&self) -> bool {
        self.churn_resilience
            .iter()
            .all(|resilience| resilience.selection_preserved && resilience.focus_not_stolen)
    }

    /// Whether every dimension required to record this profile is present and
    /// internally consistent.
    pub fn is_complete(&self) -> bool {
        !self.profile_id.trim().is_empty()
            && !self.label_summary.trim().is_empty()
            && self.focus_model.is_valid()
            && !self.commands.is_empty()
            && self.commands.iter().all(AssistiveCommand::is_valid)
            && self.covers_required_commands()
            && self.announcement.is_valid()
            && !self.churn_resilience.is_empty()
            && self
                .churn_resilience
                .iter()
                .all(FocusChurnResilience::is_valid)
            && self.offscreen_durability.is_valid()
            && self.hidden_selected_count_exposed()
            && self.focus_and_selection_durable()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Projects the profile into a redaction-aware reconstruction for diagnostics
    /// and accessibility-evidence packets.
    pub fn reconstruction(&self) -> AssistiveSelectionProfileReconstruction {
        AssistiveSelectionProfileReconstruction {
            profile_id: self.profile_id.clone(),
            surface_token: self.surface.as_str().to_owned(),
            view_kind_token: self.view_kind.as_str().to_owned(),
            data_mode_token: self.data_mode.as_str().to_owned(),
            focus_model_token: self.focus_model.kind.as_str().to_owned(),
            focus_by_stable_identity: self.focus_model.focus_tracked_by_stable_identity,
            command_tokens: self
                .commands
                .iter()
                .map(|command| command.kind.as_str().to_owned())
                .collect(),
            all_commands_keyboard_reachable: self.all_commands_keyboard_reachable(),
            hidden_selected_count_exposed: self.hidden_selected_count_exposed(),
            churn_event_tokens: self
                .churn_resilience
                .iter()
                .map(|resilience| resilience.event.as_str().to_owned())
                .collect(),
            focus_and_selection_durable: self.focus_and_selection_durable(),
            offscreen_selected_count: self.offscreen_durability.offscreen_selected_count,
        }
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistiveSelectionGuardrails {
    /// No selection control is pointer-only; a row highlight never stands in for a
    /// keyboard path.
    pub no_pointer_only_selection_controls: bool,
    /// Streaming inserts and virtualization never steal focus.
    pub focus_not_stolen_by_streaming_or_virtualization: bool,
    /// Selection survives sort, filter, and virtualization by stable identity.
    pub selection_survives_sort_filter_virtualization_by_identity: bool,
    /// The hidden-selected count is always exposed to assistive technology.
    pub hidden_selected_count_exposed_to_assistive_tech: bool,
    /// Roving focus is tracked by stable identity, not row position.
    pub roving_focus_tracked_by_stable_identity: bool,
    /// Broad-action review is keyboard reachable.
    pub broad_action_review_keyboard_reachable: bool,
}

impl AssistiveSelectionGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.no_pointer_only_selection_controls
            && self.focus_not_stolen_by_streaming_or_virtualization
            && self.selection_survives_sort_filter_virtualization_by_identity
            && self.hidden_selected_count_exposed_to_assistive_tech
            && self.roving_focus_tracked_by_stable_identity
            && self.broad_action_review_keyboard_reachable
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistiveSelectionConsumerProjection {
    /// Product renders the assistive selection model from these records.
    pub product_renders_assistive_selection: bool,
    /// Diagnostics reconstruct parity from these records.
    pub diagnostics_reconstructs_parity: bool,
    /// Support/export reuses the parity projection.
    pub support_export_reuses_records: bool,
    /// Accessibility evidence and release proof reuse these records.
    pub accessibility_evidence_reuses_records: bool,
}

impl AssistiveSelectionConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.product_renders_assistive_selection
            && self.diagnostics_reconstructs_parity
            && self.support_export_reuses_records
            && self.accessibility_evidence_reuses_records
    }
}

/// Constructor input for [`AssistiveSelectionParityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssistiveSelectionParityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Per-surface assistive selection profiles.
    pub profiles: Vec<AssistiveSelectionProfile>,
    /// Guardrail invariants block.
    pub guardrails: AssistiveSelectionGuardrails,
    /// Consumer projection block.
    pub consumer_projection: AssistiveSelectionConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe assistive-selection-parity packet for the first real M5 dense
/// surfaces.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AssistiveSelectionParityPacket {
    /// Record kind; must equal [`ASSISTIVE_SELECTION_PARITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`ASSISTIVE_SELECTION_PARITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub packet_label: String,
    /// Per-surface assistive selection profiles.
    pub profiles: Vec<AssistiveSelectionProfile>,
    /// Guardrail invariants block.
    pub guardrails: AssistiveSelectionGuardrails,
    /// Consumer projection block.
    pub consumer_projection: AssistiveSelectionConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl AssistiveSelectionParityPacket {
    /// Builds an assistive-selection-parity packet.
    pub fn new(input: AssistiveSelectionParityPacketInput) -> Self {
        Self {
            record_kind: ASSISTIVE_SELECTION_PARITY_RECORD_KIND.to_owned(),
            schema_version: ASSISTIVE_SELECTION_PARITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            packet_label: input.packet_label,
            profiles: input.profiles,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Surfaces represented by some profile in this packet.
    pub fn represented_surfaces(&self) -> BTreeSet<DenseCollectionSurface> {
        self.profiles
            .iter()
            .map(|profile| profile.surface)
            .collect()
    }

    /// View kinds represented by some profile in this packet.
    pub fn represented_view_kinds(&self) -> BTreeSet<CollectionViewKind> {
        self.profiles
            .iter()
            .map(|profile| profile.view_kind)
            .collect()
    }

    /// Data modes represented by some profile in this packet.
    pub fn represented_data_modes(&self) -> BTreeSet<CollectionDataMode> {
        self.profiles
            .iter()
            .map(|profile| profile.data_mode)
            .collect()
    }

    /// Focus models represented by some profile in this packet.
    pub fn represented_focus_models(&self) -> BTreeSet<FocusModelKind> {
        self.profiles
            .iter()
            .map(|profile| profile.focus_model.kind)
            .collect()
    }

    /// Churn events demonstrated by some profile in this packet.
    pub fn demonstrated_churn_events(&self) -> BTreeSet<FocusChurnEvent> {
        self.profiles
            .iter()
            .flat_map(|profile| profile.covered_churn_events())
            .collect()
    }

    /// Count of profiles that demonstrate a non-zero offscreen selection.
    pub fn offscreen_selection_profile_count(&self) -> usize {
        self.profiles
            .iter()
            .filter(|profile| profile.offscreen_durability.offscreen_selected_count > 0)
            .count()
    }

    /// Reconstructions for every profile, used by diagnostics and accessibility
    /// evidence packets.
    pub fn reconstructions(&self) -> Vec<AssistiveSelectionProfileReconstruction> {
        self.profiles
            .iter()
            .map(AssistiveSelectionProfile::reconstruction)
            .collect()
    }

    /// Validates the assistive-selection-parity packet invariants.
    pub fn validate(&self) -> Vec<AssistiveSelectionParityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != ASSISTIVE_SELECTION_PARITY_RECORD_KIND {
            violations.push(AssistiveSelectionParityViolation::WrongRecordKind);
        }
        if self.schema_version != ASSISTIVE_SELECTION_PARITY_SCHEMA_VERSION {
            violations.push(AssistiveSelectionParityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.packet_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(AssistiveSelectionParityViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_profiles(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(AssistiveSelectionParityViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(AssistiveSelectionParityViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("assistive selection parity packet serializes"),
        ) {
            violations.push(AssistiveSelectionParityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("assistive selection parity packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Keyboard Assistive Selection Parity And Roving Focus\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.packet_label));
        out.push_str(&format!(
            "- Profiles: {} ({} with offscreen selection)\n",
            self.profiles.len(),
            self.offscreen_selection_profile_count()
        ));
        out.push_str(&format!(
            "- Surfaces: {} / {}\n",
            self.represented_surfaces().len(),
            REQUIRED_PARITY_SURFACES.len()
        ));
        out.push_str(&format!(
            "- View kinds: {} / {}\n",
            self.represented_view_kinds().len(),
            CollectionViewKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Data modes: {} / {}\n",
            self.represented_data_modes().len(),
            CollectionDataMode::ALL.len()
        ));
        out.push_str(&format!(
            "- Focus models: {} / {}\n",
            self.represented_focus_models().len(),
            FocusModelKind::ALL.len()
        ));
        out.push_str(&format!(
            "- Churn events: {} / {}\n",
            self.demonstrated_churn_events().len(),
            FocusChurnEvent::ALL.len()
        ));
        out.push_str("\n## Profiles\n\n");
        for profile in &self.profiles {
            out.push_str(&format!(
                "- **{}** ({} / {} / {}): {}\n",
                profile.profile_id,
                profile.surface.as_str(),
                profile.view_kind.as_str(),
                profile.data_mode.as_str(),
                profile.label_summary,
            ));
            out.push_str(&format!(
                "  - focus: `{}` (by_identity={})\n",
                profile.focus_model.kind.as_str(),
                profile.focus_model.focus_tracked_by_stable_identity,
            ));
            out.push_str(&format!(
                "  - commands: {}\n",
                profile
                    .commands
                    .iter()
                    .map(|command| command.kind.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
            out.push_str(&format!(
                "  - churn: {}\n",
                profile
                    .churn_resilience
                    .iter()
                    .map(|resilience| resilience.event.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
            ));
            out.push_str(&format!(
                "  - offscreen_selected={} hidden_count_exposed={}\n",
                profile.offscreen_durability.offscreen_selected_count,
                profile.hidden_selected_count_exposed(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in assistive-selection-parity export.
#[derive(Debug)]
pub enum AssistiveSelectionParityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<AssistiveSelectionParityViolation>),
}

impl fmt::Display for AssistiveSelectionParityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "assistive selection parity export parse failed: {error}"
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
                    "assistive selection parity export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for AssistiveSelectionParityArtifactError {}

/// Validation failures emitted by [`AssistiveSelectionParityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssistiveSelectionParityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A required real M5 surface is bound by no profile.
    RequiredSurfaceMissing,
    /// A required view kind (list, tree, table, queue) is represented by no
    /// profile.
    RequiredViewKindMissing,
    /// A required data mode (filtered/sorted, streaming, virtualized) is
    /// represented by no profile.
    RequiredDataModeMissing,
    /// Both focus models (roving tabindex, aria-activedescendant) are not
    /// demonstrated.
    RequiredFocusModelMissing,
    /// A required churn event is demonstrated by no profile.
    RequiredChurnEventMissing,
    /// No profile demonstrates a non-zero offscreen selection.
    OffscreenSelectionCaseMissing,
    /// No profile demonstrates a focus re-anchor after the focused item departs.
    FocusReanchorCaseMissing,
    /// A profile is incomplete.
    ProfileIncomplete,
    /// A profile is missing a required command.
    RequiredCommandMissing,
    /// A profile exposes a pointer-only or non-keyboard/non-screen-reader command.
    PointerOnlyControlPresent,
    /// A profile's roving focus is not tracked by stable identity.
    FocusNotByStableIdentity,
    /// A churn event steals focus.
    FocusStolenOnChurn,
    /// A churn event or offscreen record corrupts / drops durable selection.
    SelectionNotDurable,
    /// The hidden-selected count is not exposed to assistive technology.
    HiddenSelectedCountNotExposed,
    /// A profile lacks accessibility evidence refs.
    ProfileEvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl AssistiveSelectionParityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::RequiredSurfaceMissing => "required_surface_missing",
            Self::RequiredViewKindMissing => "required_view_kind_missing",
            Self::RequiredDataModeMissing => "required_data_mode_missing",
            Self::RequiredFocusModelMissing => "required_focus_model_missing",
            Self::RequiredChurnEventMissing => "required_churn_event_missing",
            Self::OffscreenSelectionCaseMissing => "offscreen_selection_case_missing",
            Self::FocusReanchorCaseMissing => "focus_reanchor_case_missing",
            Self::ProfileIncomplete => "profile_incomplete",
            Self::RequiredCommandMissing => "required_command_missing",
            Self::PointerOnlyControlPresent => "pointer_only_control_present",
            Self::FocusNotByStableIdentity => "focus_not_by_stable_identity",
            Self::FocusStolenOnChurn => "focus_stolen_on_churn",
            Self::SelectionNotDurable => "selection_not_durable",
            Self::HiddenSelectedCountNotExposed => "hidden_selected_count_not_exposed",
            Self::ProfileEvidenceMissing => "profile_evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in assistive-selection-parity export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_m5_assistive_selection_parity_export(
) -> Result<AssistiveSelectionParityPacket, AssistiveSelectionParityArtifactError> {
    let packet: AssistiveSelectionParityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/collections/m5/ship-keyboard-assistive-selection-parity-roving-tabindex-dense-collection-focus-and-offscr/support_export.json"
    )))
    .map_err(AssistiveSelectionParityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(AssistiveSelectionParityArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &AssistiveSelectionParityPacket,
    violations: &mut Vec<AssistiveSelectionParityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        ASSISTIVE_SELECTION_PARITY_SCHEMA_REF,
        ASSISTIVE_SELECTION_PARITY_DOC_REF,
        ASSISTIVE_SELECTION_PARITY_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(AssistiveSelectionParityViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &AssistiveSelectionParityPacket,
    violations: &mut Vec<AssistiveSelectionParityViolation>,
) {
    let surfaces = packet.represented_surfaces();
    for required in REQUIRED_PARITY_SURFACES {
        if !surfaces.contains(&required) {
            violations.push(AssistiveSelectionParityViolation::RequiredSurfaceMissing);
            break;
        }
    }

    let view_kinds = packet.represented_view_kinds();
    for required in CollectionViewKind::ALL {
        if !view_kinds.contains(&required) {
            violations.push(AssistiveSelectionParityViolation::RequiredViewKindMissing);
            break;
        }
    }

    let data_modes = packet.represented_data_modes();
    for required in REQUIRED_PARITY_DATA_MODES {
        if !data_modes.contains(&required) {
            violations.push(AssistiveSelectionParityViolation::RequiredDataModeMissing);
            break;
        }
    }

    let focus_models = packet.represented_focus_models();
    for required in FocusModelKind::ALL {
        if !focus_models.contains(&required) {
            violations.push(AssistiveSelectionParityViolation::RequiredFocusModelMissing);
            break;
        }
    }

    let churn_events = packet.demonstrated_churn_events();
    for required in REQUIRED_CHURN_EVENTS {
        if !churn_events.contains(&required) {
            violations.push(AssistiveSelectionParityViolation::RequiredChurnEventMissing);
            break;
        }
    }

    if packet.offscreen_selection_profile_count() == 0 {
        violations.push(AssistiveSelectionParityViolation::OffscreenSelectionCaseMissing);
    }

    if !packet.profiles.iter().any(|profile| {
        profile
            .churn_resilience
            .iter()
            .any(|resilience| resilience.outcome == FocusDurabilityOutcome::FocusReanchoredVisible)
    }) {
        violations.push(AssistiveSelectionParityViolation::FocusReanchorCaseMissing);
    }
}

fn validate_profiles(
    packet: &AssistiveSelectionParityPacket,
    violations: &mut Vec<AssistiveSelectionParityViolation>,
) {
    for profile in &packet.profiles {
        if !profile.is_complete() {
            violations.push(AssistiveSelectionParityViolation::ProfileIncomplete);
        }
        if !profile.covers_required_commands() {
            violations.push(AssistiveSelectionParityViolation::RequiredCommandMissing);
        }
        if !profile.all_commands_keyboard_reachable() {
            violations.push(AssistiveSelectionParityViolation::PointerOnlyControlPresent);
        }
        if !profile.focus_model.focus_tracked_by_stable_identity {
            violations.push(AssistiveSelectionParityViolation::FocusNotByStableIdentity);
        }
        if profile
            .churn_resilience
            .iter()
            .any(|resilience| !resilience.focus_not_stolen)
        {
            violations.push(AssistiveSelectionParityViolation::FocusStolenOnChurn);
        }
        let selection_durable = profile
            .churn_resilience
            .iter()
            .all(|resilience| resilience.selection_preserved)
            && profile
                .offscreen_durability
                .selection_survives_virtualization_recycle
            && profile
                .offscreen_durability
                .offscreen_members_tracked_by_identity;
        if !selection_durable {
            violations.push(AssistiveSelectionParityViolation::SelectionNotDurable);
        }
        if !profile.hidden_selected_count_exposed() {
            violations.push(AssistiveSelectionParityViolation::HiddenSelectedCountNotExposed);
        }
        if profile.evidence_refs.is_empty()
            || profile.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(AssistiveSelectionParityViolation::ProfileEvidenceMissing);
        }
    }
}

/// Whether a label is a generic non-answer rather than a precise label. A generic
/// provider error or bare token must never stand in for precise parity truth.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "provider error"
            | "request failed"
            | "failed"
            | "hidden"
            | "stale"
            | "changed"
            | "review"
            | "selected"
            | "key"
            | "action"
            | "button"
            | "press"
            | "click"
            | "focus"
            | "select"
            | "tbd"
            | "todo"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret_value")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
