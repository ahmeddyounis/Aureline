//! Focus-return, asynchronous-update safety, roving-tabindex, and stable-item-identity
//! contract for the claimed M5 shell zones, dense collections, overlays, multi-window
//! layouts, and follow/presentation flows.
//!
//! Where the frozen dynamic-surface matrix
//! ([`crate::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix`])
//! freezes *which* focus-return dispositions an accessibility object may admit, and
//! the live-announcement grammar ([`crate::announcement_grammar`]) governs *how* a
//! focus or selection change is narrated, this module materializes the concrete
//! *focus and selection contract* a keyboard or assistive user can rely on. One
//! [`M5FocusZoneContract`] row per governed focus zone — modal dialogs, sheets,
//! command palettes, popovers, rename fields, inspector promotions, dense
//! collections, streamed lists, shell zones, multi-window layouts, and
//! follow/presentation modes — binds a stable zone id to:
//!
//! - an explicit [`M5FocusReturnRule`] with a real return target and a safe fallback
//!   disposition for when the invoking object no longer exists, so focus never
//!   teleports to an unrelated surface or vanishes on an async update or overlay
//!   teardown;
//! - an [`M5StableIdentityRule`] that preserves focus and selection by stable item
//!   identity — never row index — across virtualization, refresh, streaming inserts,
//!   filtering, sort changes, and multi-window restore/layout adjustments; and
//! - an optional [`M5RovingTabindexRule`] for dense collections that pins a single
//!   tab stop, predictable arrow/home/end/page navigation, and a disclosed (never
//!   silent) multi-selection narrowing.
//!
//! The contract is the single M5 source for assistive-tech *focus and selection*
//! truth: shell, search/palette, review, data-grid, notification, and presentation
//! surfaces consume these rows rather than improvising per-surface focus handling;
//! diagnostics, support exports, docs/help, and assistive-tech conformance packets
//! reuse the same rows so a focus-teleport or selection-drift regression is debuggable
//! from the support export alone. The guardrail is hard: no overlay, sheet, or
//! collection may claim keyboard completeness unless it states and proves its
//! focus-return and stable-item-identity behavior. When a zone's bridge or proof state
//! goes stale the claimed contract auto-narrows rather than implying silent
//! keyboard/assistive completeness.
//!
//! The controlled focus-return-disposition vocabulary is reused verbatim from the
//! frozen matrix, and the durable-fallback-surface vocabulary from the announcement
//! grammar, rather than minting parallel tokens. Only the focus-shaped vocabularies
//! this lane adds (zone kind, interaction model, async-update class, identity
//! strategy, and collection navigation key) are minted here and frozen in a
//! self-describing [`M5FocusSelectionVocabularySet`]. Raw provider payloads,
//! credentials, secret material, screenshots, and untranslated free-text prose stay
//! outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/a11y/m5-focus-selection.schema.json`](../../../../../schemas/a11y/m5-focus-selection.schema.json).
//! The contract doc is
//! [`docs/a11y/m5-focus-and-selection.md`](../../../../../docs/a11y/m5-focus-and-selection.md).
//! The protected fixture directory is
//! [`fixtures/a11y/m5-focus-return/`](../../../../../fixtures/a11y/m5-focus-return/).

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_focus_selection_contract,
    seeded_m5_focus_selection_contract_bridge_unavailable_narrowed,
    seeded_m5_focus_selection_contract_proof_stale_narrowed, M5_FOCUS_SELECTION_CONTRACT_PACKET_ID,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// The announcement grammar owns the canonical durable-fallback-surface vocabulary;
// route every zone's safe-working-context fallback through it rather than minting a
// parallel synonym.
use crate::announcement_grammar as grammar;
// The frozen matrix owns the canonical focus-return-disposition vocabulary, the
// shared state vocabularies, qualification classes, downgrade triggers, consumer
// surfaces, and proof/release posture.
use crate::freeze_the_m5_accessibility_bridge_live_announcement_focus_return_and_non_visual_dynamic_surface_matrix as matrix;

pub use grammar::{M5DurableFallbackRef, M5DurableFallbackSurface};
pub use matrix::{
    A11yFocusReturnDisposition, A11yNonVisualFidelity, M5DynamicSurfaceA11yConsumerSurface,
    M5DynamicSurfaceA11yDowngradeTrigger, M5DynamicSurfaceA11yProofFreshness,
    M5DynamicSurfaceA11yQualificationClass, M5DynamicSurfaceA11yReleasePosture,
    M5DynamicSurfaceA11yVocabularySet,
};

/// Stable record-kind tag carried by [`M5FocusSelectionContractPacket`].
pub const M5_FOCUS_SELECTION_RECORD_KIND: &str = "m5_focus_and_selection_contract";

/// Schema version for M5 focus-and-selection contracts.
pub const M5_FOCUS_SELECTION_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_FOCUS_SELECTION_SCHEMA_REF: &str = "schemas/a11y/m5-focus-selection.schema.json";

/// Repo-relative path of the M5 focus-and-selection contract doc.
pub const M5_FOCUS_SELECTION_DOC_REF: &str = "docs/a11y/m5-focus-and-selection.md";

/// Repo-relative path of the frozen dynamic-surface accessibility matrix that owns
/// this lane's shared controlled vocabularies and qualification classes.
pub const M5_FOCUS_SELECTION_MATRIX_REF: &str = "schemas/a11y/m5-dynamic-surface-a11y.schema.json";

/// Repo-relative path of the per-surface accessibility descriptors this lane's
/// focus-return targets resolve against.
pub const M5_FOCUS_SELECTION_SURFACE_DESCRIPTOR_REF: &str =
    "schemas/a11y/m5-surface-descriptors.schema.json";

/// Repo-relative path of the frozen focus / zoom / pointer-independence contract.
pub const M5_FOCUS_SELECTION_FOCUS_CONTRACT_REF: &str =
    "docs/accessibility/focus_zoom_and_pointer_independence_contract.md";

/// Repo-relative path of the frozen screen-reader announcement / live-region contract
/// a focus or selection change narrates through.
pub const M5_FOCUS_SELECTION_SCREEN_READER_CONTRACT_REF: &str =
    "docs/accessibility/screen_reader_and_live_region_contract.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_FOCUS_SELECTION_FIXTURE_DIR: &str = "fixtures/a11y/m5-focus-return";

/// Repo-relative path of the checked support-export artifact.
pub const M5_FOCUS_SELECTION_ARTIFACT_REF: &str =
    "artifacts/a11y/m5-focus-return-proof/support_export.json";

/// Repo-relative path of the checked Markdown governance summary.
pub const M5_FOCUS_SELECTION_SUMMARY_REF: &str =
    "artifacts/a11y/m5-focus-return-proof/focus-return-proof.md";

/// Stable prefix every focus-zone id carries.
pub const M5_FOCUS_ZONE_ID_PREFIX: &str = "focus-zone:";

/// One governed focus zone the contract must cover.
///
/// These are exactly the claimed M5 surfaces whose focus movement and dense-surface
/// navigation must be explicit: transient overlays (dialog, sheet, palette, popover,
/// rename field, inspector promotion), dense collections and streamed lists, broad
/// shell zones, multi-window layouts, and follow/presentation modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FocusZoneKind {
    /// A modal dialog.
    ModalDialog,
    /// A sheet or drawer.
    Sheet,
    /// A command palette / quick-open surface.
    CommandPalette,
    /// A popover or menu.
    Popover,
    /// An inline rename field.
    RenameField,
    /// An inspector promotion (a detail panel promoted to a working surface).
    InspectorPromotion,
    /// A dense, virtualized collection (list / grid / tree).
    DenseCollection,
    /// A streaming-insert list (log, search results, output).
    StreamedList,
    /// A broad shell layout zone (a primary region or panel group).
    ShellZone,
    /// A multi-window restore / layout adjustment.
    MultiWindowLayout,
    /// A follow / presentation mode.
    FollowPresentation,
}

impl M5FocusZoneKind {
    /// Every governed zone kind, in declaration order.
    pub const ALL: [Self; 11] = [
        Self::ModalDialog,
        Self::Sheet,
        Self::CommandPalette,
        Self::Popover,
        Self::RenameField,
        Self::InspectorPromotion,
        Self::DenseCollection,
        Self::StreamedList,
        Self::ShellZone,
        Self::MultiWindowLayout,
        Self::FollowPresentation,
    ];

    /// Stable token recorded in the contract.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModalDialog => "modal_dialog",
            Self::Sheet => "sheet",
            Self::CommandPalette => "command_palette",
            Self::Popover => "popover",
            Self::RenameField => "rename_field",
            Self::InspectorPromotion => "inspector_promotion",
            Self::DenseCollection => "dense_collection",
            Self::StreamedList => "streamed_list",
            Self::ShellZone => "shell_zone",
            Self::MultiWindowLayout => "multi_window_layout",
            Self::FollowPresentation => "follow_presentation",
        }
    }

    /// The interaction model this zone kind belongs to.
    pub const fn interaction_model(self) -> M5FocusInteractionModel {
        match self {
            Self::ModalDialog
            | Self::Sheet
            | Self::CommandPalette
            | Self::Popover
            | Self::RenameField
            | Self::InspectorPromotion => M5FocusInteractionModel::TransientOverlay,
            Self::DenseCollection | Self::StreamedList => M5FocusInteractionModel::DenseCollection,
            Self::ShellZone => M5FocusInteractionModel::ShellZone,
            Self::MultiWindowLayout => M5FocusInteractionModel::MultiWindowLayout,
            Self::FollowPresentation => M5FocusInteractionModel::FollowPresentation,
        }
    }
}

/// Class of focus interaction a zone follows, grouping zone kinds by the contract
/// invariant they must satisfy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FocusInteractionModel {
    /// A transient overlay that must return focus to its invoker on teardown.
    TransientOverlay,
    /// A dense collection that must use a roving single tab stop and preserve item
    /// identity across virtualization and refresh.
    DenseCollection,
    /// A persistent shell zone that must restore focus across layout adjustments.
    ShellZone,
    /// A multi-window layout that must restore focus and identity across window
    /// restore.
    MultiWindowLayout,
    /// A follow / presentation mode that must preserve context and return focus on
    /// exit.
    FollowPresentation,
}

impl M5FocusInteractionModel {
    /// Every interaction model, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::TransientOverlay,
        Self::DenseCollection,
        Self::ShellZone,
        Self::MultiWindowLayout,
        Self::FollowPresentation,
    ];

    /// Stable token recorded in the contract.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TransientOverlay => "transient_overlay",
            Self::DenseCollection => "dense_collection",
            Self::ShellZone => "shell_zone",
            Self::MultiWindowLayout => "multi_window_layout",
            Self::FollowPresentation => "follow_presentation",
        }
    }

    /// True when this model must carry a roving-tabindex rule.
    pub const fn requires_roving_tabindex(self) -> bool {
        matches!(self, Self::DenseCollection)
    }

    /// The async-update classes a zone in this model must preserve focus and selection
    /// across.
    pub fn required_async_classes(self) -> &'static [M5AsyncUpdateClass] {
        use M5AsyncUpdateClass as A;
        match self {
            Self::TransientOverlay => &[A::OverlayTeardown],
            Self::DenseCollection => &[
                A::Virtualization,
                A::Refresh,
                A::StreamingInsert,
                A::Filtering,
                A::SortChange,
            ],
            Self::ShellZone => &[A::Refresh, A::LayoutAdjustment],
            Self::MultiWindowLayout => &[A::MultiWindowRestore, A::LayoutAdjustment],
            Self::FollowPresentation => &[A::Refresh, A::LayoutAdjustment],
        }
    }
}

/// Class of asynchronous update or layout change a zone's focus and selection must
/// survive without teleporting, vanishing, or drifting to a row index.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AsyncUpdateClass {
    /// Rows recycled by virtualization.
    Virtualization,
    /// A full data refresh.
    Refresh,
    /// A streaming insert / append.
    StreamingInsert,
    /// A filter applied or cleared.
    Filtering,
    /// A sort order changed.
    SortChange,
    /// A multi-window restore.
    MultiWindowRestore,
    /// A layout adjustment (pane resize, detach, or rearrange).
    LayoutAdjustment,
    /// A transient overlay torn down.
    OverlayTeardown,
}

impl M5AsyncUpdateClass {
    /// Every async-update class, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Virtualization,
        Self::Refresh,
        Self::StreamingInsert,
        Self::Filtering,
        Self::SortChange,
        Self::MultiWindowRestore,
        Self::LayoutAdjustment,
        Self::OverlayTeardown,
    ];

    /// Stable token recorded in the contract.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Virtualization => "virtualization",
            Self::Refresh => "refresh",
            Self::StreamingInsert => "streaming_insert",
            Self::Filtering => "filtering",
            Self::SortChange => "sort_change",
            Self::MultiWindowRestore => "multi_window_restore",
            Self::LayoutAdjustment => "layout_adjustment",
            Self::OverlayTeardown => "overlay_teardown",
        }
    }
}

/// Strategy a zone uses to track item identity across an async update.
///
/// Every strategy except [`Self::RowIndexOnly`] preserves a stable per-item identity
/// so focus and selection survive virtualization and refresh. `row_index_only` is the
/// explicit anti-pattern the spec calls out — a protected zone must never use it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5IdentityStrategy {
    /// A stable per-item key issued by the data source.
    StableKey,
    /// A stable content hash of the item.
    ContentHash,
    /// A stable path or URI identity.
    PathOrUri,
    /// Row-index-only identity (forbidden; degrades into focus loss on reorder).
    RowIndexOnly,
}

impl M5IdentityStrategy {
    /// Every identity strategy, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::StableKey,
        Self::ContentHash,
        Self::PathOrUri,
        Self::RowIndexOnly,
    ];

    /// Stable token recorded in the contract.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableKey => "stable_key",
            Self::ContentHash => "content_hash",
            Self::PathOrUri => "path_or_uri",
            Self::RowIndexOnly => "row_index_only",
        }
    }

    /// True when this strategy preserves a stable per-item identity (never row index).
    pub const fn is_stable(self) -> bool {
        !matches!(self, Self::RowIndexOnly)
    }
}

/// A keyboard navigation key a roving-tabindex collection must handle predictably.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CollectionNavKey {
    /// Up / down arrow row movement.
    ArrowUpDown,
    /// Left / right arrow column movement.
    ArrowLeftRight,
    /// Home / End jump to the first / last item.
    HomeEnd,
    /// Page Up / Page Down paged movement.
    PageUpDown,
    /// Type-ahead first-letter navigation.
    TypeAhead,
}

impl M5CollectionNavKey {
    /// Every navigation key, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ArrowUpDown,
        Self::ArrowLeftRight,
        Self::HomeEnd,
        Self::PageUpDown,
        Self::TypeAhead,
    ];

    /// Stable token recorded in the contract.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ArrowUpDown => "arrow_up_down",
            Self::ArrowLeftRight => "arrow_left_right",
            Self::HomeEnd => "home_end",
            Self::PageUpDown => "page_up_down",
            Self::TypeAhead => "type_ahead",
        }
    }
}

/// Explicit focus-return rule for a zone.
///
/// `primary_disposition` is the focus-return path when the invoking object still
/// exists; `safe_fallback_disposition` is the path when the invoking object no longer
/// exists. Both return focus to a real owner — focus never teleports to an unrelated
/// surface or vanishes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FocusReturnRule {
    /// Stable id of the explicit return target (matrix-/grammar-resolvable).
    pub return_target_ref: String,
    /// Focus-return disposition when the invoking object still exists (matrix-owned).
    pub primary_disposition: A11yFocusReturnDisposition,
    /// Focus-return disposition when the invoking object no longer exists
    /// (matrix-owned). Must locate a new real owner — never the exact prior owner.
    pub safe_fallback_disposition: A11yFocusReturnDisposition,
    /// True when the return (for placeholder / announced re-entry) is announced.
    pub announces_return: bool,
}

/// Stable-item-identity rule that preserves focus and selection across async updates.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5StableIdentityRule {
    /// Identity strategy used to track items (must be stable — never row index).
    pub identity_strategy: M5IdentityStrategy,
    /// True when focus is preserved by stable identity across the async classes.
    pub preserves_focus: bool,
    /// True when selection is preserved by stable identity across the async classes.
    pub preserves_selection: bool,
    /// Async-update classes the identity is preserved across.
    pub preserved_across: Vec<M5AsyncUpdateClass>,
}

/// Roving-tabindex rule for a dense collection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5RovingTabindexRule {
    /// True when the collection exposes a single tab stop (roving tabindex).
    pub single_tab_stop: bool,
    /// Predictable navigation keys the collection handles.
    pub navigation_keys: Vec<M5CollectionNavKey>,
    /// True when a multi-selection narrowing is announced, never silent.
    pub multi_selection_narrowing_announced: bool,
}

/// One focus-zone contract row for a claimed M5 surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FocusZoneContract {
    /// Stable zone id, unique within the contract; carries [`M5_FOCUS_ZONE_ID_PREFIX`].
    pub zone_id: String,
    /// Governed zone kind.
    pub zone_kind: M5FocusZoneKind,
    /// Interaction model (must match the zone kind's model).
    pub interaction_model: M5FocusInteractionModel,
    /// Human-readable zone label.
    pub label: String,
    /// Owner role accountable for keeping this zone's focus contract current.
    pub owner_role: String,
    /// Qualification class earned by this zone's contract.
    pub qualification: M5DynamicSurfaceA11yQualificationClass,
    /// Non-visual fidelity the zone currently delivers (matrix-owned).
    pub non_visual_fidelity: A11yNonVisualFidelity,
    /// True when the zone claims keyboard completeness (guardrail-gated).
    pub keyboard_complete_claim: bool,
    /// Explicit focus-return rule.
    pub focus_return: M5FocusReturnRule,
    /// Stable-item-identity rule.
    pub stable_identity: M5StableIdentityRule,
    /// Roving-tabindex rule; required for, and only for, a dense collection.
    pub roving_tabindex: Option<M5RovingTabindexRule>,
    /// Durable fallback surface the zone returns focus to (grammar-owned).
    pub durable_fallback: M5DurableFallbackRef,
    /// Downgrade triggers that can narrow this zone below its claim.
    pub downgrade_triggers: Vec<M5DynamicSurfaceA11yDowngradeTrigger>,
    /// Assistive-tech proof packet refs that keep this zone current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this zone.
    pub source_contract_refs: Vec<String>,
    /// Consumer surfaces that project this zone's focus contract.
    pub consumer_surfaces: Vec<M5DynamicSurfaceA11yConsumerSurface>,
}

impl M5FocusZoneContract {
    /// True when the zone states and proves its focus-return behavior: an explicit
    /// real return target, an interactive primary disposition, and a safe fallback
    /// that locates a new owner when the invoker is gone.
    fn states_focus_return(&self) -> bool {
        !self.focus_return.return_target_ref.trim().is_empty()
            && is_interactive_return(self.focus_return.primary_disposition)
            && is_safe_invoker_gone_fallback(self.focus_return.safe_fallback_disposition)
    }

    /// True when the zone states and proves its stable-item-identity behavior: a
    /// stable (non-row-index) strategy that preserves focus and selection across the
    /// async classes its interaction model requires.
    fn states_stable_identity(&self) -> bool {
        self.stable_identity.identity_strategy.is_stable()
            && self.stable_identity.preserves_focus
            && self.stable_identity.preserves_selection
            && self.preserves_required_async_classes()
    }

    /// True when the zone preserves identity across every async class its model needs.
    fn preserves_required_async_classes(&self) -> bool {
        let present: BTreeSet<M5AsyncUpdateClass> = self
            .stable_identity
            .preserved_across
            .iter()
            .copied()
            .collect();
        self.interaction_model
            .required_async_classes()
            .iter()
            .all(|required| present.contains(required))
    }

    /// True when a roving-tabindex rule is present and well-formed for a collection.
    fn states_roving_tabindex(&self) -> bool {
        match &self.roving_tabindex {
            Some(rule) => {
                rule.single_tab_stop
                    && rule.multi_selection_narrowing_announced
                    && rule
                        .navigation_keys
                        .contains(&M5CollectionNavKey::ArrowUpDown)
                    && rule.navigation_keys.contains(&M5CollectionNavKey::HomeEnd)
            }
            None => false,
        }
    }

    /// True when the zone can support a keyboard-complete claim: it states focus
    /// return and stable identity, plus roving tabindex when it is a dense collection.
    fn supports_keyboard_complete(&self) -> bool {
        let roving_ok = if self.interaction_model.requires_roving_tabindex() {
            self.states_roving_tabindex()
        } else {
            true
        };
        self.states_focus_return() && self.states_stable_identity() && roving_ok
    }
}

/// Self-describing controlled-vocabulary set for the focus-shaped tokens this lane
/// mints (the focus-return-disposition tokens live in the matrix; the
/// durable-fallback tokens live in the grammar).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FocusSelectionVocabularySet {
    /// Zone-kind tokens.
    pub zone_kinds: Vec<String>,
    /// Interaction-model tokens.
    pub interaction_models: Vec<String>,
    /// Async-update-class tokens.
    pub async_update_classes: Vec<String>,
    /// Identity-strategy tokens.
    pub identity_strategies: Vec<String>,
    /// Collection navigation-key tokens.
    pub collection_nav_keys: Vec<String>,
}

impl M5FocusSelectionVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            zone_kinds: M5FocusZoneKind::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            interaction_models: M5FocusInteractionModel::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            async_update_classes: M5AsyncUpdateClass::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            identity_strategies: M5IdentityStrategy::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            collection_nav_keys: M5CollectionNavKey::ALL
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
        }
    }

    /// Returns true when this set matches the canonical token lists exactly.
    pub fn matches_canonical(&self) -> bool {
        *self == Self::canonical()
    }
}

/// Assistive-technology conformance review block for the focus-and-selection lane.
///
/// Every flag is a hard invariant; all must hold for the contract to validate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FocusSelectionConformanceReview {
    /// Transient surfaces declare an explicit focus-return target.
    pub transient_surfaces_declare_explicit_focus_return: bool,
    /// Focus never teleports to an unrelated surface or vanishes on an async update.
    pub focus_never_teleports_or_vanishes_on_async_update: bool,
    /// A safe fallback applies when the invoking object no longer exists.
    pub safe_fallback_when_invoking_object_gone: bool,
    /// Focus and selection are preserved by stable item identity, never row index.
    pub focus_and_selection_preserved_by_stable_item_identity: bool,
    /// Virtualization / refresh / restore never degrade into row-index focus loss.
    pub no_row_index_based_focus_loss_or_selection_drift: bool,
    /// Dense collections use a roving single tab stop.
    pub dense_collections_use_roving_single_tab_stop: bool,
    /// Arrow / Home / End / Page navigation is predictable.
    pub predictable_arrow_home_end_page_navigation: bool,
    /// A multi-selection narrowing is announced, never silent.
    pub no_silent_multi_selection_narrowing: bool,
    /// Overlays return the user to a safe working context.
    pub overlays_return_to_safe_working_context: bool,
    /// A keyboard-complete claim requires proven focus return and stable identity.
    pub keyboard_complete_requires_focus_return_and_stable_identity: bool,
    /// Claimed zones auto-narrow when bridge or proof state goes stale.
    pub claimed_zones_auto_narrow_when_bridge_or_proof_stale: bool,
    /// Downgrade narrows the claim rather than hiding the zone.
    pub downgrade_narrows_instead_of_hides: bool,
}

/// Consumer projection block: who routes focus and selection through this contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FocusSelectionConsumerProjection {
    /// Shell returns focus on overlay teardown via this contract.
    pub shell_returns_focus_on_overlay_teardown: bool,
    /// Search / command palette returns focus to its invoker via this contract.
    pub search_palette_returns_focus_to_invoker: bool,
    /// Review preserves row identity across refresh via this contract.
    pub review_preserves_row_identity_across_refresh: bool,
    /// Data grid uses roving tabindex via this contract.
    pub data_grid_uses_roving_tabindex: bool,
    /// Notifications route focus-return targets via this contract.
    pub notifications_route_focus_return_targets: bool,
    /// Presentation / follow mode preserves context via this contract.
    pub presentation_follow_mode_preserves_context: bool,
    /// Multi-window restore preserves item identity via this contract.
    pub multi_window_restore_preserves_identity: bool,
    /// Support export reuses this contract.
    pub support_export_reuses_contract: bool,
    /// Docs / help reuse this contract.
    pub docs_help_reuse_contract: bool,
    /// Assistive-tech conformance packets reuse this contract.
    pub at_conformance_packets_reuse_contract: bool,
}

/// Constructor input for [`M5FocusSelectionContractPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5FocusSelectionContractPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable contract label.
    pub contract_label: String,
    /// Focus-zone contract rows.
    pub zones: Vec<M5FocusZoneContract>,
    /// Shared (matrix-owned) controlled-vocabulary set, reused to prove the
    /// focus-return-disposition tokens come from the frozen matrix.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Focus-shaped controlled-vocabulary set.
    pub focus_vocabulary_set: M5FocusSelectionVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5FocusSelectionConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FocusSelectionConsumerProjection,
    /// Proof freshness block (reused from the matrix lane).
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture (reused from the matrix lane).
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 focus-and-selection contract packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5FocusSelectionContractPacket {
    /// Record kind; must equal [`M5_FOCUS_SELECTION_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_FOCUS_SELECTION_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable contract label.
    pub contract_label: String,
    /// Focus-zone contract rows.
    pub zones: Vec<M5FocusZoneContract>,
    /// Shared (matrix-owned) controlled-vocabulary set.
    pub shared_vocabulary_set: M5DynamicSurfaceA11yVocabularySet,
    /// Focus-shaped controlled-vocabulary set.
    pub focus_vocabulary_set: M5FocusSelectionVocabularySet,
    /// Conformance review block.
    pub conformance_review: M5FocusSelectionConformanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5FocusSelectionConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5DynamicSurfaceA11yProofFreshness,
    /// Release and mirror/offline parity posture.
    pub release_posture: M5DynamicSurfaceA11yReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5FocusSelectionContractPacket {
    /// Builds a focus-and-selection contract packet from seed input.
    pub fn new(input: M5FocusSelectionContractPacketInput) -> Self {
        Self {
            record_kind: M5_FOCUS_SELECTION_RECORD_KIND.to_owned(),
            schema_version: M5_FOCUS_SELECTION_SCHEMA_VERSION,
            packet_id: input.packet_id,
            contract_label: input.contract_label,
            zones: input.zones,
            shared_vocabulary_set: input.shared_vocabulary_set,
            focus_vocabulary_set: input.focus_vocabulary_set,
            conformance_review: input.conformance_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            release_posture: input.release_posture,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Total number of governed focus zones.
    pub fn zone_count(&self) -> usize {
        self.zones.len()
    }

    /// Validates the focus-and-selection contract invariants.
    pub fn validate(&self) -> Vec<M5FocusSelectionViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_FOCUS_SELECTION_RECORD_KIND {
            violations.push(M5FocusSelectionViolation::WrongRecordKind);
        }
        if self.schema_version != M5_FOCUS_SELECTION_SCHEMA_VERSION {
            violations.push(M5FocusSelectionViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.contract_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5FocusSelectionViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_sets(self, &mut violations);
        validate_zones(self, &mut violations);
        validate_conformance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("m5 focus selection contract serializes"),
        ) {
            violations.push(M5FocusSelectionViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 focus selection contract serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable = self
            .zones
            .iter()
            .filter(|z| z.qualification.is_stable())
            .count();
        let keyboard_complete = self
            .zones
            .iter()
            .filter(|z| z.keyboard_complete_claim)
            .count();
        let mut out = String::new();
        out.push_str("# M5 Focus-Return and Stable-Selection Contract\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.contract_label));
        out.push_str(&format!(
            "- Zones: {} ({} stable, {} keyboard-complete)\n",
            self.zones.len(),
            stable,
            keyboard_complete
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Focus zones\n\n");
        for zone in &self.zones {
            out.push_str(&format!(
                "- **{}** (`{}` / `{}`): `{}`, fidelity `{}`{}\n",
                zone.zone_id,
                zone.zone_kind.as_str(),
                zone.interaction_model.as_str(),
                zone.qualification.as_str(),
                zone.non_visual_fidelity.as_str(),
                if zone.keyboard_complete_claim {
                    ", keyboard-complete"
                } else {
                    ""
                }
            ));
            out.push_str(&format!("  - Owner: {}\n", zone.owner_role));
            out.push_str(&format!(
                "  - Focus return: primary {} / fallback {} -> `{}`\n",
                zone.focus_return.primary_disposition.as_str(),
                zone.focus_return.safe_fallback_disposition.as_str(),
                zone.focus_return.return_target_ref
            ));
            out.push_str(&format!(
                "  - Identity: `{}`, preserved across {}\n",
                zone.stable_identity.identity_strategy.as_str(),
                zone.stable_identity
                    .preserved_across
                    .iter()
                    .map(|a| a.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
            if let Some(roving) = &zone.roving_tabindex {
                out.push_str(&format!(
                    "  - Roving tabindex: single-tab-stop {}, keys {}\n",
                    roving.single_tab_stop,
                    roving
                        .navigation_keys
                        .iter()
                        .map(|k| k.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            out.push_str(&format!(
                "  - Safe working context: {} (`{}`)\n",
                zone.durable_fallback.surface.as_str(),
                zone.durable_fallback.surface_ref
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in focus-and-selection export.
#[derive(Debug)]
pub enum M5FocusSelectionArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5FocusSelectionViolation>),
}

impl fmt::Display for M5FocusSelectionArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "m5 focus selection export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "m5 focus selection export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5FocusSelectionArtifactError {}

/// Validation failures emitted by [`M5FocusSelectionContractPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5FocusSelectionViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// A frozen vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A governed zone kind has no contract row.
    RequiredZoneKindMissing,
    /// Two rows cover the same zone kind in a way that breaks coverage.
    DuplicateZoneId,
    /// A zone row is incomplete.
    ZoneIncomplete,
    /// A zone id is missing the governed prefix.
    ZoneIdPrefixMissing,
    /// A zone's interaction model does not match its zone kind.
    InteractionModelMismatch,
    /// A zone's focus-return rule has no explicit return target.
    FocusReturnTargetMissing,
    /// An interactive zone's primary disposition is non-interactive.
    InteractiveZoneFocusNotApplicable,
    /// A zone's safe fallback does not locate a new owner when the invoker is gone.
    FocusReturnUnsafeFallback,
    /// An announced-placeholder return is not flagged as announced.
    FocusReturnPlaceholderNotAnnounced,
    /// A zone's identity strategy is row-index-only.
    StableIdentityUsesRowIndex,
    /// A zone does not preserve focus or selection by stable identity.
    StableIdentityDoesNotPreserveFocusOrSelection,
    /// A zone does not preserve identity across a required async-update class.
    StableIdentityMissingRequiredAsyncClass,
    /// A dense collection is missing its roving-tabindex rule.
    RovingTabindexMissingForDenseCollection,
    /// A non-collection zone carries a roving-tabindex rule.
    RovingTabindexPresentForNonCollection,
    /// A roving-tabindex rule does not pin a single tab stop.
    RovingTabindexNotSingleTabStop,
    /// A roving-tabindex rule is missing predictable navigation keys.
    RovingTabindexMissingNavigationKeys,
    /// A multi-selection narrowing is silent (not announced).
    SilentMultiSelectionNarrowing,
    /// A zone claims keyboard completeness without proving focus return and identity.
    KeyboardCompleteWithoutFocusReturnAndIdentity,
    /// A zone has no reopenable durable fallback safe-working-context surface.
    ZoneDurableFallbackMissing,
    /// A zone's non-visual fidelity is not an accessible class.
    ZoneNonVisualFidelityInvalid,
    /// A zone claiming Stable is missing required proof packet refs.
    StableZoneMissingProof,
    /// A zone has no downgrade triggers.
    DowngradeTriggersMissing,
    /// A zone has no consumer surfaces.
    ConsumerSurfacesMissing,
    /// Conformance review does not satisfy required invariants.
    ConformanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release/mirror-offline parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl M5FocusSelectionViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredZoneKindMissing => "required_zone_kind_missing",
            Self::DuplicateZoneId => "duplicate_zone_id",
            Self::ZoneIncomplete => "zone_incomplete",
            Self::ZoneIdPrefixMissing => "zone_id_prefix_missing",
            Self::InteractionModelMismatch => "interaction_model_mismatch",
            Self::FocusReturnTargetMissing => "focus_return_target_missing",
            Self::InteractiveZoneFocusNotApplicable => "interactive_zone_focus_not_applicable",
            Self::FocusReturnUnsafeFallback => "focus_return_unsafe_fallback",
            Self::FocusReturnPlaceholderNotAnnounced => "focus_return_placeholder_not_announced",
            Self::StableIdentityUsesRowIndex => "stable_identity_uses_row_index",
            Self::StableIdentityDoesNotPreserveFocusOrSelection => {
                "stable_identity_does_not_preserve_focus_or_selection"
            }
            Self::StableIdentityMissingRequiredAsyncClass => {
                "stable_identity_missing_required_async_class"
            }
            Self::RovingTabindexMissingForDenseCollection => {
                "roving_tabindex_missing_for_dense_collection"
            }
            Self::RovingTabindexPresentForNonCollection => {
                "roving_tabindex_present_for_non_collection"
            }
            Self::RovingTabindexNotSingleTabStop => "roving_tabindex_not_single_tab_stop",
            Self::RovingTabindexMissingNavigationKeys => "roving_tabindex_missing_navigation_keys",
            Self::SilentMultiSelectionNarrowing => "silent_multi_selection_narrowing",
            Self::KeyboardCompleteWithoutFocusReturnAndIdentity => {
                "keyboard_complete_without_focus_return_and_identity"
            }
            Self::ZoneDurableFallbackMissing => "zone_durable_fallback_missing",
            Self::ZoneNonVisualFidelityInvalid => "zone_non_visual_fidelity_invalid",
            Self::StableZoneMissingProof => "stable_zone_missing_proof",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::ConformanceReviewIncomplete => "conformance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable focus-and-selection export.
pub fn current_stable_m5_focus_selection_export(
) -> Result<M5FocusSelectionContractPacket, M5FocusSelectionArtifactError> {
    let packet: M5FocusSelectionContractPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/a11y/m5-focus-return-proof/support_export.json"
    )))
    .map_err(M5FocusSelectionArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5FocusSelectionArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5FocusSelectionContractPacket,
    violations: &mut Vec<M5FocusSelectionViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_FOCUS_SELECTION_SCHEMA_REF,
        M5_FOCUS_SELECTION_DOC_REF,
        M5_FOCUS_SELECTION_MATRIX_REF,
        M5_FOCUS_SELECTION_SURFACE_DESCRIPTOR_REF,
        M5_FOCUS_SELECTION_FOCUS_CONTRACT_REF,
        M5_FOCUS_SELECTION_SCREEN_READER_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5FocusSelectionViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_sets(
    packet: &M5FocusSelectionContractPacket,
    violations: &mut Vec<M5FocusSelectionViolation>,
) {
    if !packet.shared_vocabulary_set.matches_canonical()
        || !packet.focus_vocabulary_set.matches_canonical()
    {
        violations.push(M5FocusSelectionViolation::VocabularySetDrift);
    }
}

fn validate_zones(
    packet: &M5FocusSelectionContractPacket,
    violations: &mut Vec<M5FocusSelectionViolation>,
) {
    let present: BTreeSet<M5FocusZoneKind> = packet.zones.iter().map(|z| z.zone_kind).collect();
    for required in M5FocusZoneKind::ALL {
        if !present.contains(&required) {
            violations.push(M5FocusSelectionViolation::RequiredZoneKindMissing);
            break;
        }
    }

    let mut seen_zone_ids: BTreeSet<&str> = BTreeSet::new();
    for zone in &packet.zones {
        if !seen_zone_ids.insert(zone.zone_id.as_str()) {
            violations.push(M5FocusSelectionViolation::DuplicateZoneId);
        }
        validate_zone(zone, violations);
    }
}

fn validate_zone(zone: &M5FocusZoneContract, violations: &mut Vec<M5FocusSelectionViolation>) {
    if zone.zone_id.trim().is_empty()
        || zone.label.trim().is_empty()
        || zone.owner_role.trim().is_empty()
        || zone.source_contract_refs.is_empty()
    {
        violations.push(M5FocusSelectionViolation::ZoneIncomplete);
    }

    if !zone.zone_id.starts_with(M5_FOCUS_ZONE_ID_PREFIX) {
        violations.push(M5FocusSelectionViolation::ZoneIdPrefixMissing);
    }

    if zone.zone_kind.interaction_model() != zone.interaction_model {
        violations.push(M5FocusSelectionViolation::InteractionModelMismatch);
    }

    validate_focus_return(zone, violations);
    validate_stable_identity(zone, violations);
    validate_roving_tabindex(zone, violations);

    // Guardrail: no zone may claim keyboard completeness unless it states and proves
    // its focus-return and stable-item-identity behavior (plus roving tabindex for a
    // dense collection).
    if zone.keyboard_complete_claim && !zone.supports_keyboard_complete() {
        violations.push(M5FocusSelectionViolation::KeyboardCompleteWithoutFocusReturnAndIdentity);
    }

    // Every zone must return focus to a reopenable durable safe-working-context.
    if zone.durable_fallback.surface_ref.trim().is_empty() || !zone.durable_fallback.reopenable {
        violations.push(M5FocusSelectionViolation::ZoneDurableFallbackMissing);
    }

    if !is_accessible_fidelity(zone.non_visual_fidelity) {
        violations.push(M5FocusSelectionViolation::ZoneNonVisualFidelityInvalid);
    }
    if zone.qualification.is_stable() && zone.required_proof_packet_refs.is_empty() {
        violations.push(M5FocusSelectionViolation::StableZoneMissingProof);
    }
    if zone.downgrade_triggers.is_empty() {
        violations.push(M5FocusSelectionViolation::DowngradeTriggersMissing);
    }
    if zone.consumer_surfaces.is_empty() {
        violations.push(M5FocusSelectionViolation::ConsumerSurfacesMissing);
    }
}

fn validate_focus_return(
    zone: &M5FocusZoneContract,
    violations: &mut Vec<M5FocusSelectionViolation>,
) {
    let rule = &zone.focus_return;
    if rule.return_target_ref.trim().is_empty() {
        violations.push(M5FocusSelectionViolation::FocusReturnTargetMissing);
    }
    // Every governed zone is interactive: the primary disposition must return focus to
    // a real owner, never declare focus return not-applicable.
    if !is_interactive_return(rule.primary_disposition) {
        violations.push(M5FocusSelectionViolation::InteractiveZoneFocusNotApplicable);
    }
    // When the invoking object is gone the fallback must locate a new real owner — it
    // can never return to the exact prior owner, keep a vanished owner, or be
    // non-applicable. This is the rule that keeps focus from teleporting or vanishing.
    if !is_safe_invoker_gone_fallback(rule.safe_fallback_disposition) {
        violations.push(M5FocusSelectionViolation::FocusReturnUnsafeFallback);
    }
    // An announced-placeholder re-entry must actually be announced.
    if rule.safe_fallback_disposition == A11yFocusReturnDisposition::ReturnedPlaceholderAnnounced
        && !rule.announces_return
    {
        violations.push(M5FocusSelectionViolation::FocusReturnPlaceholderNotAnnounced);
    }
}

fn validate_stable_identity(
    zone: &M5FocusZoneContract,
    violations: &mut Vec<M5FocusSelectionViolation>,
) {
    let rule = &zone.stable_identity;
    if !rule.identity_strategy.is_stable() {
        violations.push(M5FocusSelectionViolation::StableIdentityUsesRowIndex);
    }
    if !rule.preserves_focus || !rule.preserves_selection {
        violations.push(M5FocusSelectionViolation::StableIdentityDoesNotPreserveFocusOrSelection);
    }
    if !zone.preserves_required_async_classes() {
        violations.push(M5FocusSelectionViolation::StableIdentityMissingRequiredAsyncClass);
    }
}

fn validate_roving_tabindex(
    zone: &M5FocusZoneContract,
    violations: &mut Vec<M5FocusSelectionViolation>,
) {
    let requires = zone.interaction_model.requires_roving_tabindex();
    match (&zone.roving_tabindex, requires) {
        (None, true) => {
            violations.push(M5FocusSelectionViolation::RovingTabindexMissingForDenseCollection);
        }
        (Some(_), false) => {
            violations.push(M5FocusSelectionViolation::RovingTabindexPresentForNonCollection);
        }
        (Some(rule), true) => {
            if !rule.single_tab_stop {
                violations.push(M5FocusSelectionViolation::RovingTabindexNotSingleTabStop);
            }
            // Predictable navigation requires at least arrow and home/end movement.
            if !rule
                .navigation_keys
                .contains(&M5CollectionNavKey::ArrowUpDown)
                || !rule.navigation_keys.contains(&M5CollectionNavKey::HomeEnd)
            {
                violations.push(M5FocusSelectionViolation::RovingTabindexMissingNavigationKeys);
            }
            if !rule.multi_selection_narrowing_announced {
                violations.push(M5FocusSelectionViolation::SilentMultiSelectionNarrowing);
            }
        }
        (None, false) => {}
    }
}

/// True when a disposition returns focus to a real owner for an interactive zone.
fn is_interactive_return(disposition: A11yFocusReturnDisposition) -> bool {
    !matches!(
        disposition,
        A11yFocusReturnDisposition::FocusNotApplicableNonInteractive
    )
}

/// True when a disposition locates a new real owner once the invoker no longer exists.
///
/// The exact prior owner is gone, a denied-loss "keep prior owner" no longer has a
/// prior owner to keep, and a non-interactive surface has no owner — only the three
/// "locate a new owner" dispositions are safe fallbacks.
fn is_safe_invoker_gone_fallback(disposition: A11yFocusReturnDisposition) -> bool {
    matches!(
        disposition,
        A11yFocusReturnDisposition::ReturnedNearestSafeAncestor
            | A11yFocusReturnDisposition::ReturnedCurrentBatchOrDetailOwner
            | A11yFocusReturnDisposition::ReturnedPlaceholderAnnounced
    )
}

/// True when a fidelity class still conveys non-visual truth for a covered zone.
fn is_accessible_fidelity(fidelity: A11yNonVisualFidelity) -> bool {
    matches!(
        fidelity,
        A11yNonVisualFidelity::FullAccessible
            | A11yNonVisualFidelity::DegradedAccessible
            | A11yNonVisualFidelity::SummaryOnly
    )
}

fn validate_conformance_review(
    packet: &M5FocusSelectionContractPacket,
    violations: &mut Vec<M5FocusSelectionViolation>,
) {
    let review = &packet.conformance_review;
    for ok in [
        review.transient_surfaces_declare_explicit_focus_return,
        review.focus_never_teleports_or_vanishes_on_async_update,
        review.safe_fallback_when_invoking_object_gone,
        review.focus_and_selection_preserved_by_stable_item_identity,
        review.no_row_index_based_focus_loss_or_selection_drift,
        review.dense_collections_use_roving_single_tab_stop,
        review.predictable_arrow_home_end_page_navigation,
        review.no_silent_multi_selection_narrowing,
        review.overlays_return_to_safe_working_context,
        review.keyboard_complete_requires_focus_return_and_stable_identity,
        review.claimed_zones_auto_narrow_when_bridge_or_proof_stale,
        review.downgrade_narrows_instead_of_hides,
    ] {
        if !ok {
            violations.push(M5FocusSelectionViolation::ConformanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5FocusSelectionContractPacket,
    violations: &mut Vec<M5FocusSelectionViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.shell_returns_focus_on_overlay_teardown,
        projection.search_palette_returns_focus_to_invoker,
        projection.review_preserves_row_identity_across_refresh,
        projection.data_grid_uses_roving_tabindex,
        projection.notifications_route_focus_return_targets,
        projection.presentation_follow_mode_preserves_context,
        projection.multi_window_restore_preserves_identity,
        projection.support_export_reuses_contract,
        projection.docs_help_reuse_contract,
        projection.at_conformance_packets_reuse_contract,
    ] {
        if !ok {
            violations.push(M5FocusSelectionViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5FocusSelectionContractPacket,
    violations: &mut Vec<M5FocusSelectionViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5FocusSelectionViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5FocusSelectionContractPacket,
    violations: &mut Vec<M5FocusSelectionViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.mirror_offline_packet_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.mirror_offline_parity_required
        || !posture.stable_promotion_blocks_without_mapped_proof
    {
        violations.push(M5FocusSelectionViolation::ReleasePostureIncomplete);
    }
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
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
