//! One reusable M5 design-system primitive — the selection-or-lock-state contract — so every
//! claimed M5 tab, tree, dense list, grid/table, badge, settings row, and inspector entry renders
//! its `Selected`, `Current`, `Disabled`, `Read-only`, and `Locked` states the same way, with the
//! semantic distinctions the acceptance criteria demand: a merely selected item never reads as the
//! actively current one, an inspectable-but-read-only item never collapses into a silently
//! disabled one, and an explicit trust/policy lock is never hidden behind a plain disabled
//! treatment. Whenever a state is explainable — a policy lock, a trust block, a missing capability,
//! or a read-only inspection path — the contract surfaces its owner, its cause, and its recovery
//! action instead of a silent, color-only style change.
//!
//! Aureline's frozen shared-component-state-taxonomy component matrix
//! ([`crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix`])
//! names the selection-or-lock-state contract as one of its four governed component-state families
//! and freezes its controlled vocabulary — the selection/lock subset of the shared taxonomy
//! (`selected`, `current`, `disabled`, `read_only`, `locked`), the lock owner classes it can name
//! (`policy_lock`, `trust_lock`, `permission_lock`, `ownership_lock`, `source_lock`, `no_lock`),
//! the state cause classes it can name, plus the surface families, deployment lines, consumer
//! surfaces, non-visual accessibility routes, mandatory labels, qualification classes, and
//! downgrade triggers. This module *implements* that contract as one reusable resolver so a user —
//! pointer, keyboard, or assistive-tech operator alike — always gets the same explicit
//! selection-or-lock behavior from a tab, a tree node, a dense list row, a grid cell, a badge, a
//! settings row, or an inspector entry, instead of one-off styling accidents on individual
//! surfaces.
//!
//! The module has one resolver:
//!
//! 1. [`resolve_selection_or_lock_state_contract`] — takes one item's kind, the selection-or-lock
//!    state it is entering (one of `selected`, `current`, `disabled`, `read_only`, `locked`), the
//!    lock owner and state cause behind the state, whether a recovery path is available, whether a
//!    read-only item stays inspectable, the high-contrast context, its opaque stable item identity,
//!    the opaque shared state-style token reference that renders it, and the opaque owner/cause/
//!    recovery disclosure reference, and produces one
//!    [`M5ResolvedSelectionOrLockStateContract`] carrying the derived presentation posture
//!    (selected / current / disabled / read-only / locked treatment), the required non-color cues
//!    that carry the state beyond hue, the required disclosures the state must publish (state
//!    cause, owner / block reason, recovery action), and the hard guarantees that `selected` and
//!    `current` never collapse, read-only preserves inspectability, a lock is never hidden behind
//!    disabled, and the state stays keyboard- and screen-reader-explainable. It refuses to model a
//!    lock as a plain disabled control, refuses a locked state with no owner, refuses a read-only
//!    state that has lost its inspectability, and refuses an explainable state with no owner /
//!    cause / recovery detail.
//!
//! A single parity matrix — [`M5SelectionOrLockStateContractPacket`] — binds one row per claimed
//! M5 collection surface (the tab, the tree item, the dense list row, the grid/table row, the
//! badge, the settings row, and the inspector entry) to the shared selection-or-lock anatomy, the
//! same selection-or-lock states, presentation postures, non-color cues, required disclosures,
//! lock owner classes, state cause classes, export fields, mandatory labels, and non-visual
//! accessibility routes, so the selected / current / read-only / disabled / locked vocabulary and
//! its owner-reason-recovery rules stay identical across desktop, headless/export, and support
//! consumers.
//!
//! The selection/lock state class ([`M5SharedComponentStateClass`]), the lock owner class
//! ([`M5LockOwnerClass`]), the state cause class ([`M5StateCauseClass`]), the state disclosure
//! trigger ([`M5StateDisclosureTrigger`]), the surface family
//! ([`M5ComponentStateSurfaceFamily`]), the deployment line
//! ([`M5ComponentStateDeploymentLine`]), the consumer surface
//! ([`M5ComponentStateConsumerSurface`]), the accessibility route
//! ([`M5ComponentStateAccessibilityRoute`]), the required label
//! ([`M5ComponentStateRequiredLabel`]), the qualification class
//! ([`M5ComponentStateQualificationClass`]), and the downgrade trigger
//! ([`M5ComponentStateDowngradeTrigger`]) are reused verbatim from the frozen matrix. This module
//! mints new vocabulary only for what that matrix left implicit about the selection-or-lock
//! rendering itself: its claimed item kinds, its anatomy parts, its derived presentation posture,
//! its non-color cues, and its export fields. No M5 collection surface invents a second
//! selection-or-lock grammar.
//!
//! Raw local paths, credentials, and private endpoints stay outside the export boundary; every
//! item identity, state-style token reference, and owner/cause/recovery disclosure reference is
//! carried only as an opaque, export-safe representation.

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_selection_or_lock_state_contract_badge_beta_narrowed,
    seeded_m5_selection_or_lock_state_contract_inspector_entry_preview_narrowed,
    seeded_m5_selection_or_lock_state_contract_packet,
    M5_SELECTION_OR_LOCK_STATE_CONTRACT_PACKET_ID,
};

// The selection/lock state class, lock owner class, state cause class, state disclosure trigger,
// surface family, deployment line, consumer surface, accessibility route, required label,
// qualification class, and downgrade triggers are frozen once, in the shared-component-state-taxonomy
// component matrix. This primitive reuses them verbatim so it never invents a parallel
// selection-or-lock vocabulary.
pub use crate::freeze_the_m5_shared_component_state_taxonomy_interactive_state_selection_or_lock_state_and_degraded_state_application_component_matrix::{
    M5ComponentStateAccessibilityRoute, M5ComponentStateConsumerSurface,
    M5ComponentStateDeploymentLine, M5ComponentStateDowngradeTrigger,
    M5ComponentStateQualificationClass, M5ComponentStateRequiredLabel,
    M5ComponentStateSurfaceFamily, M5LockOwnerClass, M5SharedComponentStateClass,
    M5SharedComponentStateFamily, M5StateCauseClass, M5StateDisclosureTrigger,
};

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Stable record-kind tag carried by [`M5SelectionOrLockStateContractPacket`].
pub const M5_SELECTION_OR_LOCK_STATE_CONTRACT_RECORD_KIND: &str =
    "implement_m5_selected_current_read_only_disabled_and_locked_state_parity_with_owner_reason_recovery_truth_across_claimed_m5_tabs_trees_lists_tables_badges_and_inspectors";

/// Schema version for M5 selection-or-lock-state-contract records.
pub const M5_SELECTION_OR_LOCK_STATE_CONTRACT_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the selection-or-lock-state-contract boundary schema.
pub const M5_SELECTION_OR_LOCK_STATE_CONTRACT_SCHEMA_REF: &str =
    "schemas/ui/m5-selection-lock-state-contract.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_SELECTION_OR_LOCK_STATE_CONTRACT_DOC_REF: &str =
    "docs/design-system/m5_selection_or_lock_state_contract_primitive.md";

/// Repo-relative path of the frozen shared-component-state-taxonomy component matrix this
/// primitive narrows from.
pub const M5_SELECTION_OR_LOCK_STATE_CONTRACT_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-shared-component-state-taxonomy-component-matrix.schema.json";

/// Repo-relative path of the focus/selection accessibility contract the `selected` and `current`
/// states bind their distinct-selection and current-location semantics against.
pub const M5_SELECTION_OR_LOCK_STATE_CONTRACT_FOCUS_SELECTION_REF: &str =
    "schemas/a11y/m5-focus-selection.schema.json";

/// Repo-relative path of the state-class recovery contract the explainable states bind their
/// owner / cause / recovery disclosure against.
pub const M5_SELECTION_OR_LOCK_STATE_CONTRACT_STATE_RECOVERY_REF: &str =
    "schemas/state/state_class_recovery.schema.json";

/// Repo-relative path of the operational-surface-state contract the `disabled`, `read_only`, and
/// `locked` states bind their blocked / inspectable / locked posture against.
pub const M5_SELECTION_OR_LOCK_STATE_CONTRACT_OPERATIONAL_SURFACE_STATE_REF: &str =
    "schemas/accessibility/operational_surface_state.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_SELECTION_OR_LOCK_STATE_CONTRACT_FIXTURE_DIR: &str =
    "fixtures/ui/m5-selection-lock-state-contract-primitive";

/// Repo-relative path of the checked support-export artifact.
pub const M5_SELECTION_OR_LOCK_STATE_CONTRACT_ARTIFACT_REF: &str =
    "artifacts/release/m5-selection-lock-state-contract-primitive-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const M5_SELECTION_OR_LOCK_STATE_CONTRACT_CSV_REF: &str =
    "artifacts/release/m5-selection-lock-state-contract-primitive-proof/matrix.csv";

/// Repo-relative path of the checked Markdown design report.
pub const M5_SELECTION_OR_LOCK_STATE_CONTRACT_REPORT_REF: &str =
    "artifacts/design/m5-selection-lock-state-contract-primitive.md";

/// One claimed M5 collection surface that renders the shared selection-or-lock-state contract.
/// These are the surfaces the implementation requirements name — tabs, trees, dense lists,
/// grids/tables, badges, settings rows, and inspector entries — so the same selected / current /
/// read-only / disabled / locked grammar works across every claimed collection surface.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SelectionOrLockItemKind {
    /// A tab in a tab strip.
    Tab,
    /// A tree node in a navigation or outline tree.
    TreeItem,
    /// A row in a dense list.
    ListRow,
    /// A row / cell in a grid or table.
    TableRow,
    /// A status badge.
    Badge,
    /// A settings row in a settings or capability sheet.
    SettingsRow,
    /// An inspector entry in a detail / property inspector.
    InspectorEntry,
}

impl M5SelectionOrLockItemKind {
    /// Every claimed item kind, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::Tab,
        Self::TreeItem,
        Self::ListRow,
        Self::TableRow,
        Self::Badge,
        Self::SettingsRow,
        Self::InspectorEntry,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Tab => "tab",
            Self::TreeItem => "tree_item",
            Self::ListRow => "list_row",
            Self::TableRow => "table_row",
            Self::Badge => "badge",
            Self::SettingsRow => "settings_row",
            Self::InspectorEntry => "inspector_entry",
        }
    }

    /// Review-safe label for evidence packets and docs.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Tab => "Tab",
            Self::TreeItem => "Tree Item",
            Self::ListRow => "Dense List Row",
            Self::TableRow => "Grid / Table Row",
            Self::Badge => "Badge",
            Self::SettingsRow => "Settings Row",
            Self::InspectorEntry => "Inspector Entry",
        }
    }
}

/// The derived presentation posture of a selection-or-lock state — the resolver's verdict about how
/// an item's `selected`, `current`, `disabled`, `read_only`, or `locked` state is rendered. Derived
/// one-to-one from the state so no selection-or-lock state collapses into another: a merely
/// selected item is always distinguishable from the actively current one, and a locked item is
/// always distinguishable from a plain disabled one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SelectionOrLockPresentation {
    /// The durable-selection treatment.
    SelectedTreatment,
    /// The current-location / live-context treatment.
    CurrentTreatment,
    /// The silently-unavailable disabled treatment.
    DisabledTreatment,
    /// The inspectable-but-not-editable read-only treatment.
    ReadOnlyTreatment,
    /// The explicit policy / trust / permission / ownership / source lock treatment.
    LockedTreatment,
}

impl M5SelectionOrLockPresentation {
    /// Every presentation posture, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::SelectedTreatment,
        Self::CurrentTreatment,
        Self::DisabledTreatment,
        Self::ReadOnlyTreatment,
        Self::LockedTreatment,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectedTreatment => "selected_treatment",
            Self::CurrentTreatment => "current_treatment",
            Self::DisabledTreatment => "disabled_treatment",
            Self::ReadOnlyTreatment => "read_only_treatment",
            Self::LockedTreatment => "locked_treatment",
        }
    }

    /// The presentation posture for one selection-or-lock state, or `None` when the state is not one
    /// of the five governed selection-or-lock states.
    pub const fn from_state(state: M5SharedComponentStateClass) -> Option<Self> {
        match state {
            M5SharedComponentStateClass::Selected => Some(Self::SelectedTreatment),
            M5SharedComponentStateClass::Current => Some(Self::CurrentTreatment),
            M5SharedComponentStateClass::Disabled => Some(Self::DisabledTreatment),
            M5SharedComponentStateClass::ReadOnly => Some(Self::ReadOnlyTreatment),
            M5SharedComponentStateClass::Locked => Some(Self::LockedTreatment),
            _ => None,
        }
    }

    /// True when this posture is one of the explainable postures — disabled, read-only, or locked —
    /// so its owner / cause / recovery detail must be surfaced.
    pub const fn is_explainable(self) -> bool {
        matches!(
            self,
            Self::DisabledTreatment | Self::ReadOnlyTreatment | Self::LockedTreatment
        )
    }
}

/// One non-color cue a selection-or-lock state renders so its meaning is never carried by hue
/// alone. Every derived presentation posture publishes at least one of these, enforcing the
/// no-color-only signaling rule and keeping the selection/current and read-only/disabled/locked
/// distinctions legible without color.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SelectionOrLockCue {
    /// A selection marker / check carries the durable-selection state.
    SelectionMarker,
    /// A current-location indicator (leading marker, "you are here") carries the current state,
    /// distinct from the selection marker.
    CurrentLocationIndicator,
    /// A dimmed treatment paired with an explicit disabled reason carries the disabled state — never
    /// a bare color change.
    DisabledDimWithReason,
    /// A read-only glyph paired with an inspectable affordance carries the read-only state.
    ReadOnlyGlyphInspectable,
    /// A lock glyph paired with its owner carries the locked state.
    LockGlyphWithOwner,
    /// A recovery affordance names the path out of an explainable state.
    RecoveryAffordance,
}

impl M5SelectionOrLockCue {
    /// Every non-color cue, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::SelectionMarker,
        Self::CurrentLocationIndicator,
        Self::DisabledDimWithReason,
        Self::ReadOnlyGlyphInspectable,
        Self::LockGlyphWithOwner,
        Self::RecoveryAffordance,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SelectionMarker => "selection_marker",
            Self::CurrentLocationIndicator => "current_location_indicator",
            Self::DisabledDimWithReason => "disabled_dim_with_reason",
            Self::ReadOnlyGlyphInspectable => "read_only_glyph_inspectable",
            Self::LockGlyphWithOwner => "lock_glyph_with_owner",
            Self::RecoveryAffordance => "recovery_affordance",
        }
    }
}

/// Controlled selection-or-lock-state anatomy part the shared contract surfaces. The parts in
/// [`M5SelectionOrLockAnatomyPart::MANDATORY`] are required on every item so the state identity,
/// the presentation posture, the non-color cue set, the state cause, and the non-visual keyboard
/// route are never hidden.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SelectionOrLockAnatomyPart {
    /// The typed state identity cue.
    StateIdentityCue,
    /// The derived presentation-posture cue.
    PresentationPostureCue,
    /// The non-color cue-set cue.
    NonColorCueSetCue,
    /// The state-cause cue (why the state applies).
    StateCauseCue,
    /// The owner / block-reason cue (who gates the state).
    OwnerOrBlockReasonCue,
    /// The recovery-action cue (the path out of the state).
    RecoveryActionCue,
    /// The inspectability-guarantee cue (a read-only item stays inspectable).
    InspectabilityCue,
    /// The non-visual keyboard-route cue.
    KeyboardRouteCue,
}

impl M5SelectionOrLockAnatomyPart {
    /// Every anatomy part, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::StateIdentityCue,
        Self::PresentationPostureCue,
        Self::NonColorCueSetCue,
        Self::StateCauseCue,
        Self::OwnerOrBlockReasonCue,
        Self::RecoveryActionCue,
        Self::InspectabilityCue,
        Self::KeyboardRouteCue,
    ];

    /// The anatomy parts every item must render.
    pub const MANDATORY: [Self; 5] = [
        Self::StateIdentityCue,
        Self::PresentationPostureCue,
        Self::NonColorCueSetCue,
        Self::StateCauseCue,
        Self::KeyboardRouteCue,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateIdentityCue => "state_identity_cue",
            Self::PresentationPostureCue => "presentation_posture_cue",
            Self::NonColorCueSetCue => "non_color_cue_set_cue",
            Self::StateCauseCue => "state_cause_cue",
            Self::OwnerOrBlockReasonCue => "owner_or_block_reason_cue",
            Self::RecoveryActionCue => "recovery_action_cue",
            Self::InspectabilityCue => "inspectability_cue",
            Self::KeyboardRouteCue => "keyboard_route_cue",
        }
    }
}

/// A field the selection-or-lock-state export carries so its truth is reconstructable. The fields in
/// [`M5SelectionOrLockExportField::MANDATORY`] are required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SelectionOrLockExportField {
    /// The item kind.
    ItemKind,
    /// The selection-or-lock state.
    SelectionOrLockState,
    /// The derived presentation posture.
    Presentation,
    /// The required non-color cues.
    NonColorCues,
    /// The state cause.
    StateCause,
    /// The lock owner.
    LockOwner,
    /// Whether a recovery path is available.
    RecoveryAvailable,
    /// The shared state-style token reference.
    StateStyleRef,
}

impl M5SelectionOrLockExportField {
    /// Every export field, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::ItemKind,
        Self::SelectionOrLockState,
        Self::Presentation,
        Self::NonColorCues,
        Self::StateCause,
        Self::LockOwner,
        Self::RecoveryAvailable,
        Self::StateStyleRef,
    ];

    /// The export fields every item must carry.
    pub const MANDATORY: [Self; 5] = [
        Self::ItemKind,
        Self::SelectionOrLockState,
        Self::Presentation,
        Self::NonColorCues,
        Self::StateCause,
    ];

    /// Stable token recorded in the matrix.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ItemKind => "item_kind",
            Self::SelectionOrLockState => "selection_or_lock_state",
            Self::Presentation => "presentation",
            Self::NonColorCues => "non_color_cues",
            Self::StateCause => "state_cause",
            Self::LockOwner => "lock_owner",
            Self::RecoveryAvailable => "recovery_available",
            Self::StateStyleRef => "state_style_ref",
        }
    }
}

/// The five governed selection-or-lock states, in the frozen taxonomy's declaration order. Reused
/// from the selection-or-lock-state family's canonical partition of the shared taxonomy so this
/// primitive never re-lists a private selection-or-lock-state set.
pub fn selection_or_lock_states() -> Vec<M5SharedComponentStateClass> {
    M5SharedComponentStateFamily::SelectionOrLockState
        .governed_states()
        .to_vec()
}

// ---- selection-or-lock-state resolver -----------------------------------

/// The full input to the selection-or-lock-state-contract resolver for one item state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SelectionOrLockResolutionInput {
    /// The claimed item kind.
    pub item_kind: M5SelectionOrLockItemKind,
    /// The selection-or-lock state the item is entering (one of the five governed states).
    pub selection_or_lock_state: M5SharedComponentStateClass,
    /// The lock owner behind the state (`no_lock` when no lock is in effect).
    pub lock_owner: M5LockOwnerClass,
    /// The cause of the state (why it applies).
    pub state_cause: M5StateCauseClass,
    /// True when a recovery path out of the state is available.
    pub recovery_available: bool,
    /// True when the item stays inspectable in this state (required for a read-only item).
    pub inspectable: bool,
    /// True when a high-contrast mode is active, so the state stays legible without hue.
    pub high_contrast_active: bool,
    /// The opaque stable item identity (must be non-empty).
    pub item_identity_ref: String,
    /// The opaque shared state-style token reference that renders this state (must be non-empty).
    pub state_style_ref: String,
    /// The opaque owner / cause / recovery disclosure reference (must be non-empty when the state is
    /// explainable: disabled, read-only, or locked).
    pub disclosure_ref: String,
}

/// The resolved selection-or-lock-state-contract truth for one item state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ResolvedSelectionOrLockStateContract {
    /// The item kind.
    pub item_kind: M5SelectionOrLockItemKind,
    /// The selection-or-lock state.
    pub selection_or_lock_state: M5SharedComponentStateClass,
    /// The derived presentation posture.
    pub presentation: M5SelectionOrLockPresentation,
    /// The required non-color cues that carry this state beyond hue.
    pub required_non_color_cues: Vec<M5SelectionOrLockCue>,
    /// The disclosures this state must publish (state cause, owner / block reason, recovery action,
    /// and never a silent style-only change).
    pub required_disclosures: Vec<M5StateDisclosureTrigger>,
    /// The lock owner behind the state, preserved exactly from the input.
    pub lock_owner: M5LockOwnerClass,
    /// The cause of the state, preserved exactly from the input.
    pub state_cause: M5StateCauseClass,
    /// True when a recovery path is available, preserved from the input.
    pub recovery_available: bool,
    /// True when the item stays inspectable in this state, preserved from the input.
    pub inspectable: bool,
    /// True when high-contrast is active, preserved from the input.
    pub high_contrast_active: bool,
    /// The opaque stable item identity, preserved exactly from the input.
    pub item_identity_ref: String,
    /// The opaque shared state-style token reference, preserved exactly from the input.
    pub state_style_ref: String,
    /// The opaque owner / cause / recovery disclosure reference, preserved exactly from the input.
    pub disclosure_ref: String,
    /// True when this state is explainable (disabled, read-only, or locked) and therefore must
    /// surface its owner / cause / recovery detail.
    pub explainable: bool,
    /// True when this state must name a lock owner (the locked state).
    pub owner_disclosed: bool,
    /// `selected` and `current` never collapse into one another. ALWAYS `true`.
    pub selected_and_current_stay_distinct: bool,
    /// A read-only item preserves its inspectability rather than reading as disabled. ALWAYS
    /// `true`.
    pub read_only_preserves_inspectability: bool,
    /// A lock is never hidden behind a plain disabled treatment. ALWAYS `true`.
    pub lock_never_hidden_behind_disabled: bool,
    /// State meaning is never carried by color alone. ALWAYS `true`.
    pub no_color_only_signaling: bool,
    /// Whenever a state is explainable, its owner / source / recovery detail is surfaced. ALWAYS
    /// `true`.
    pub names_owner_and_recovery_when_explainable: bool,
    /// The state stays keyboard- and screen-reader-explainable. ALWAYS `true`.
    pub keyboard_and_screen_reader_explainable: bool,
    /// The state semantics are driven by the shared contract and its token hooks, not a one-off
    /// implementation choice. ALWAYS `true`.
    pub driven_by_shared_state_contract: bool,
}

/// Errors returned by [`resolve_selection_or_lock_state_contract`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum M5SelectionOrLockResolutionError {
    /// The item identity ref was empty.
    EmptyItemIdentity,
    /// The state-style token ref was empty.
    EmptyStateStyleRef,
    /// The state was not one of the five governed selection-or-lock states.
    NonSelectionOrLockState,
    /// A locked state named no lock owner, so the lock would not be explainable.
    LockWithoutOwner,
    /// A disabled state carried a lock owner, masking a lock that should be modeled as `locked`.
    DisabledMaskingLock,
    /// A read-only state lost its inspectability, so it would read as disabled.
    ReadOnlyNotInspectable,
    /// An explainable state carried no owner / cause / recovery disclosure detail.
    MissingDisclosureDetail,
    /// A descriptor carried forbidden material.
    ForbiddenStateMaterial,
}

impl M5SelectionOrLockResolutionError {
    /// Stable token for tests and diagnostics.
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::EmptyItemIdentity => "empty_item_identity",
            Self::EmptyStateStyleRef => "empty_state_style_ref",
            Self::NonSelectionOrLockState => "non_selection_or_lock_state",
            Self::LockWithoutOwner => "lock_without_owner",
            Self::DisabledMaskingLock => "disabled_masking_lock",
            Self::ReadOnlyNotInspectable => "read_only_not_inspectable",
            Self::MissingDisclosureDetail => "missing_disclosure_detail",
            Self::ForbiddenStateMaterial => "forbidden_state_material",
        }
    }
}

impl fmt::Display for M5SelectionOrLockResolutionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "selection or lock state contract resolution error: {}",
            self.as_str()
        )
    }
}

impl Error for M5SelectionOrLockResolutionError {}

/// Resolves one selection-or-lock-state contract from an item's kind, the selection-or-lock state
/// it is entering, and the owner / cause / recovery context behind it.
///
/// The presentation posture is derived one-to-one from the state so no state collapses into
/// another: `selected` renders the durable-selection treatment, `current` renders the
/// current-location treatment, `disabled` renders the silently-unavailable treatment, `read_only`
/// renders the inspectable-but-not-editable treatment, and `locked` renders the explicit lock
/// treatment. Each posture publishes a non-empty non-color cue set so the state is never carried by
/// color alone, and a required-disclosure set so an explainable state always names its cause, its
/// owner / block reason, and its recovery action. The resolver refuses a locked state that names no
/// owner, refuses a disabled state that carries a lock owner (which should be modeled as `locked`),
/// refuses a read-only state that has lost its inspectability (which would read as disabled), and
/// refuses an explainable state that carries no disclosure detail.
pub fn resolve_selection_or_lock_state_contract(
    input: &M5SelectionOrLockResolutionInput,
) -> Result<M5ResolvedSelectionOrLockStateContract, M5SelectionOrLockResolutionError> {
    if input.item_identity_ref.trim().is_empty() {
        return Err(M5SelectionOrLockResolutionError::EmptyItemIdentity);
    }
    if input.state_style_ref.trim().is_empty() {
        return Err(M5SelectionOrLockResolutionError::EmptyStateStyleRef);
    }
    if value_repr_is_forbidden(&input.item_identity_ref)
        || value_repr_is_forbidden(&input.state_style_ref)
        || value_repr_is_forbidden(&input.disclosure_ref)
    {
        return Err(M5SelectionOrLockResolutionError::ForbiddenStateMaterial);
    }

    let presentation = M5SelectionOrLockPresentation::from_state(input.selection_or_lock_state)
        .ok_or(M5SelectionOrLockResolutionError::NonSelectionOrLockState)?;

    let state = input.selection_or_lock_state;
    let has_lock = input.lock_owner != M5LockOwnerClass::NoLock;

    // A lock is never hidden behind a plain disabled treatment: a locked state must name its owner,
    // and a disabled state must never carry a lock owner (it should be modeled as `locked`).
    if state == M5SharedComponentStateClass::Locked && !has_lock {
        return Err(M5SelectionOrLockResolutionError::LockWithoutOwner);
    }
    if state == M5SharedComponentStateClass::Disabled && has_lock {
        return Err(M5SelectionOrLockResolutionError::DisabledMaskingLock);
    }
    // A read-only state preserves inspectability rather than reading as disabled.
    if state == M5SharedComponentStateClass::ReadOnly && !input.inspectable {
        return Err(M5SelectionOrLockResolutionError::ReadOnlyNotInspectable);
    }
    // An explainable state always carries owner / cause / recovery detail.
    if presentation.is_explainable() && input.disclosure_ref.trim().is_empty() {
        return Err(M5SelectionOrLockResolutionError::MissingDisclosureDetail);
    }

    let required_non_color_cues = derive_non_color_cues(presentation);
    let required_disclosures = derive_required_disclosures(presentation);
    let owner_disclosed = presentation == M5SelectionOrLockPresentation::LockedTreatment;

    Ok(M5ResolvedSelectionOrLockStateContract {
        item_kind: input.item_kind,
        selection_or_lock_state: state,
        presentation,
        required_non_color_cues,
        required_disclosures,
        lock_owner: input.lock_owner,
        state_cause: input.state_cause,
        recovery_available: input.recovery_available,
        inspectable: input.inspectable,
        high_contrast_active: input.high_contrast_active,
        item_identity_ref: input.item_identity_ref.clone(),
        state_style_ref: input.state_style_ref.clone(),
        disclosure_ref: input.disclosure_ref.clone(),
        explainable: presentation.is_explainable(),
        owner_disclosed,
        // The acceptance criteria: selected and current never collapse, read-only preserves
        // inspectability, a lock is never hidden behind disabled, the state is never color-only,
        // owner / source / recovery is surfaced when explainable, the state stays keyboard- and
        // screen-reader-explainable, and the semantics are driven by the shared contract.
        selected_and_current_stay_distinct: true,
        read_only_preserves_inspectability: true,
        lock_never_hidden_behind_disabled: true,
        no_color_only_signaling: true,
        names_owner_and_recovery_when_explainable: true,
        keyboard_and_screen_reader_explainable: true,
        driven_by_shared_state_contract: true,
    })
}

/// Derives the non-color cue set for a presentation posture. Every posture publishes at least one
/// non-color cue, so state meaning is never carried by hue alone, and every explainable posture
/// additionally publishes a recovery affordance.
fn derive_non_color_cues(presentation: M5SelectionOrLockPresentation) -> Vec<M5SelectionOrLockCue> {
    use M5SelectionOrLockCue as Cue;
    use M5SelectionOrLockPresentation as Posture;

    match presentation {
        Posture::SelectedTreatment => vec![Cue::SelectionMarker],
        Posture::CurrentTreatment => vec![Cue::CurrentLocationIndicator],
        Posture::DisabledTreatment => vec![Cue::DisabledDimWithReason, Cue::RecoveryAffordance],
        Posture::ReadOnlyTreatment => vec![Cue::ReadOnlyGlyphInspectable, Cue::RecoveryAffordance],
        Posture::LockedTreatment => vec![Cue::LockGlyphWithOwner, Cue::RecoveryAffordance],
    }
}

/// Derives the required-disclosure set for a presentation posture. Every posture forbids a silent
/// style-only change; every explainable posture additionally requires the state cause and the
/// recovery action; the locked posture requires the owner, and the disabled and locked postures
/// require the block reason.
fn derive_required_disclosures(
    presentation: M5SelectionOrLockPresentation,
) -> Vec<M5StateDisclosureTrigger> {
    use M5SelectionOrLockPresentation as Posture;
    use M5StateDisclosureTrigger as Trigger;

    match presentation {
        Posture::SelectedTreatment | Posture::CurrentTreatment => {
            vec![Trigger::SilentStyleOnlyForbidden]
        }
        Posture::DisabledTreatment => vec![
            Trigger::StateCauseRequired,
            Trigger::BlockReasonRequired,
            Trigger::RecoveryActionRequired,
            Trigger::SilentStyleOnlyForbidden,
        ],
        Posture::ReadOnlyTreatment => vec![
            Trigger::StateCauseRequired,
            Trigger::RecoveryActionRequired,
            Trigger::SilentStyleOnlyForbidden,
        ],
        Posture::LockedTreatment => vec![
            Trigger::StateCauseRequired,
            Trigger::OwnerRequired,
            Trigger::BlockReasonRequired,
            Trigger::RecoveryActionRequired,
            Trigger::SilentStyleOnlyForbidden,
        ],
    }
}

// ---- worked cases -------------------------------------------------------

/// One worked selection-or-lock-state resolution carried in the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SelectionOrLockResolutionCase {
    /// The resolver input.
    pub input: M5SelectionOrLockResolutionInput,
    /// The resolved truth. Must equal `resolve_selection_or_lock_state_contract(&input)`.
    pub resolved: M5ResolvedSelectionOrLockStateContract,
}

impl M5SelectionOrLockResolutionCase {
    /// Builds a case by resolving `input`.
    ///
    /// # Panics
    ///
    /// Panics if `input` does not resolve; seed inputs are always valid.
    pub fn resolved(input: M5SelectionOrLockResolutionInput) -> Self {
        let resolved = resolve_selection_or_lock_state_contract(&input)
            .expect("seed selection or lock state contract case is valid");
        Self { input, resolved }
    }

    /// True when the stored resolution matches a fresh resolve of the input.
    pub fn is_self_consistent(&self) -> bool {
        resolve_selection_or_lock_state_contract(&self.input).as_ref() == Ok(&self.resolved)
    }

    /// True when the resolved case preserves the input item identity, state-style reference, and
    /// disclosure reference exactly.
    pub fn preserves_identity(&self) -> bool {
        self.resolved.item_identity_ref == self.input.item_identity_ref
            && self.resolved.state_style_ref == self.input.state_style_ref
            && self.resolved.disclosure_ref == self.input.disclosure_ref
    }

    /// True when the resolved case keeps selection and current distinct, preserves read-only
    /// inspectability, never hides a lock behind disabled, never signals by color alone, names
    /// owner / recovery when explainable, stays keyboard- and screen-reader-explainable, and is
    /// driven by the shared contract.
    pub fn preserves_guarantees(&self) -> bool {
        !self.resolved.required_non_color_cues.is_empty()
            && !self.resolved.required_disclosures.is_empty()
            && self.resolved.selected_and_current_stay_distinct
            && self.resolved.read_only_preserves_inspectability
            && self.resolved.lock_never_hidden_behind_disabled
            && self.resolved.no_color_only_signaling
            && self.resolved.names_owner_and_recovery_when_explainable
            && self.resolved.keyboard_and_screen_reader_explainable
            && self.resolved.driven_by_shared_state_contract
    }
}

/// One row in the primitive matrix: one claimed M5 collection surface bound to the shared
/// selection-or-lock anatomy, selection-or-lock states, presentation postures, non-color cues,
/// required disclosures, lock owner classes, state cause classes, export fields, mandatory labels,
/// and accessibility routes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SelectionOrLockItemRow {
    /// Claimed item kind.
    pub item_kind: M5SelectionOrLockItemKind,
    /// Qualification class earned by this item.
    pub qualification: M5ComponentStateQualificationClass,
    /// Owner role accountable for keeping this item governed.
    pub owner_role: String,
    /// Human-readable scope summary.
    pub scope_summary: String,
    /// Claimed M5 surface families that render / consume this item.
    pub surface_families: Vec<M5ComponentStateSurfaceFamily>,
    /// Deployment lines this item keeps the same truth across.
    pub deployment_lines: Vec<M5ComponentStateDeploymentLine>,
    /// Anatomy parts this item renders (must include the mandatory parts).
    pub anatomy_parts: Vec<M5SelectionOrLockAnatomyPart>,
    /// Selection-or-lock states this item distinguishes.
    pub selection_or_lock_states: Vec<M5SharedComponentStateClass>,
    /// Presentation postures this item distinguishes.
    pub presentations: Vec<M5SelectionOrLockPresentation>,
    /// Non-color cues this item renders.
    pub non_color_cues: Vec<M5SelectionOrLockCue>,
    /// Required disclosures this item publishes.
    pub required_disclosures: Vec<M5StateDisclosureTrigger>,
    /// Lock owner classes this item can name behind a locked state.
    pub lock_owner_classes: Vec<M5LockOwnerClass>,
    /// State cause classes this item can name behind an explainable state.
    pub state_cause_classes: Vec<M5StateCauseClass>,
    /// Export fields this item carries (must include the mandatory fields).
    pub export_fields: Vec<M5SelectionOrLockExportField>,
    /// Non-visual accessibility routes this item offers.
    pub accessibility_routes: Vec<M5ComponentStateAccessibilityRoute>,
    /// Mandatory labels this item can show (must include the mandatory labels).
    pub required_labels: Vec<M5ComponentStateRequiredLabel>,
    /// Subsystems that consume this projection.
    pub consumer_surfaces: Vec<M5ComponentStateConsumerSurface>,
    /// Downgrade triggers that apply to this item.
    pub downgrade_triggers: Vec<M5ComponentStateDowngradeTrigger>,
    /// Proof packet refs that keep this row current.
    pub required_proof_packet_refs: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
    /// Worked selection-or-lock resolutions proving the resolver on this item.
    pub state_examples: Vec<M5SelectionOrLockResolutionCase>,
    /// Hard invariant: this item never collapses `selected` and `current`. MUST be `false`.
    pub collapses_selected_and_current: bool,
    /// Hard invariant: this item never hides a lock behind a disabled treatment. MUST be `false`.
    pub hides_lock_behind_disabled: bool,
    /// Hard invariant: this item never drops read-only inspectability. MUST be `false`.
    pub drops_read_only_inspectability: bool,
    /// Hard invariant: this item never invents a private state name. MUST be `false`.
    pub invents_private_state_name: bool,
}

impl M5SelectionOrLockItemRow {
    /// True when the row declares every mandatory anatomy part.
    fn declares_mandatory_anatomy(&self) -> bool {
        let present: BTreeSet<M5SelectionOrLockAnatomyPart> =
            self.anatomy_parts.iter().copied().collect();
        M5SelectionOrLockAnatomyPart::MANDATORY
            .iter()
            .all(|part| present.contains(part))
    }

    /// True when the row declares every mandatory export field.
    fn declares_mandatory_export(&self) -> bool {
        let present: BTreeSet<M5SelectionOrLockExportField> =
            self.export_fields.iter().copied().collect();
        M5SelectionOrLockExportField::MANDATORY
            .iter()
            .all(|field| present.contains(field))
    }

    /// True when the row declares every mandatory label.
    fn declares_mandatory_labels(&self) -> bool {
        let present: BTreeSet<M5ComponentStateRequiredLabel> =
            self.required_labels.iter().copied().collect();
        M5ComponentStateRequiredLabel::MANDATORY
            .iter()
            .all(|label| present.contains(label))
    }

    /// True when the row's hard invariants hold.
    fn honours_invariants(&self) -> bool {
        !self.collapses_selected_and_current
            && !self.hides_lock_behind_disabled
            && !self.drops_read_only_inspectability
            && !self.invents_private_state_name
    }
}

/// Self-describing controlled-vocabulary set carried by this primitive.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SelectionOrLockVocabularySet {
    /// Item-kind tokens.
    pub item_kinds: Vec<String>,
    /// Anatomy-part tokens.
    pub anatomy_parts: Vec<String>,
    /// Presentation-posture tokens.
    pub presentations: Vec<String>,
    /// Non-color-cue tokens.
    pub non_color_cues: Vec<String>,
    /// Export-field tokens.
    pub export_fields: Vec<String>,
    /// Selection-or-lock-state tokens (reused from the frozen matrix).
    pub selection_or_lock_states: Vec<String>,
    /// Required-disclosure tokens (reused from the frozen matrix).
    pub required_disclosures: Vec<String>,
    /// Lock-owner-class tokens (reused from the frozen matrix).
    pub lock_owner_classes: Vec<String>,
    /// State-cause-class tokens (reused from the frozen matrix).
    pub state_cause_classes: Vec<String>,
    /// Surface-family tokens (reused from the frozen matrix).
    pub surface_families: Vec<String>,
    /// Deployment-line tokens (reused from the frozen matrix).
    pub deployment_lines: Vec<String>,
    /// Consumer-surface tokens (reused from the frozen matrix).
    pub consumer_surfaces: Vec<String>,
    /// Accessibility-route tokens (reused from the frozen matrix).
    pub accessibility_routes: Vec<String>,
    /// Required-label tokens (reused from the frozen matrix).
    pub required_labels: Vec<String>,
}

impl M5SelectionOrLockVocabularySet {
    /// Builds the canonical vocabulary set from the typed `ALL` arrays.
    pub fn canonical() -> Self {
        Self {
            item_kinds: tokens(&M5SelectionOrLockItemKind::ALL, |v| v.as_str()),
            anatomy_parts: tokens(&M5SelectionOrLockAnatomyPart::ALL, |v| v.as_str()),
            presentations: tokens(&M5SelectionOrLockPresentation::ALL, |v| v.as_str()),
            non_color_cues: tokens(&M5SelectionOrLockCue::ALL, |v| v.as_str()),
            export_fields: tokens(&M5SelectionOrLockExportField::ALL, |v| v.as_str()),
            selection_or_lock_states: selection_or_lock_states()
                .iter()
                .map(|v| v.as_str().to_owned())
                .collect(),
            required_disclosures: tokens(&M5StateDisclosureTrigger::ALL, |v| v.as_str()),
            lock_owner_classes: tokens(&M5LockOwnerClass::ALL, |v| v.as_str()),
            state_cause_classes: tokens(&M5StateCauseClass::ALL, |v| v.as_str()),
            surface_families: tokens(&M5ComponentStateSurfaceFamily::ALL, |v| v.as_str()),
            deployment_lines: tokens(&M5ComponentStateDeploymentLine::ALL, |v| v.as_str()),
            consumer_surfaces: tokens(&M5ComponentStateConsumerSurface::ALL, |v| v.as_str()),
            accessibility_routes: tokens(&M5ComponentStateAccessibilityRoute::ALL, |v| v.as_str()),
            required_labels: tokens(&M5ComponentStateRequiredLabel::ALL, |v| v.as_str()),
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
pub struct M5SelectionOrLockGovernanceReview {
    /// Items distinguish selected, current, disabled, read-only, and locked explicitly.
    pub items_distinguish_selected_current_read_only_disabled_locked: bool,
    /// `selected` and `current` never collapse into one another.
    pub selected_and_current_never_collapse: bool,
    /// A read-only state never collapses into a plain disabled state.
    pub read_only_never_collapses_into_disabled: bool,
    /// A lock is never hidden behind a plain disabled state.
    pub locked_never_hidden_behind_disabled: bool,
    /// State meaning is never carried by color alone.
    pub state_meaning_never_color_only: bool,
    /// Owner / source / recovery detail is surfaced whenever a state is explainable.
    pub owner_source_recovery_surfaced_when_explainable: bool,
    /// States stay keyboard- and screen-reader-explainable.
    pub states_keyboard_and_screen_reader_explainable: bool,
    /// State semantics are driven by the shared contract and its token hooks.
    pub states_driven_by_shared_contract_and_tokens: bool,
    /// No item uses one-off, per-surface selection-or-lock styling.
    pub no_one_off_per_surface_styling: bool,
    /// Selection-or-lock states keep the same truth across every deployment line.
    pub states_stable_across_deployment_lines: bool,
    /// Selection-or-lock states keep the same truth across desktop, headless/export, and support
    /// consumers.
    pub states_stable_across_consumer_surfaces: bool,
    /// Every item declares a non-visual accessibility route.
    pub every_item_declares_accessibility_route: bool,
    /// The support / export packet reconstructs selection-or-lock-state truth.
    pub support_export_reconstructs_state_truth: bool,
    /// Later M5 rows cannot invent parallel selection-or-lock vocabulary.
    pub later_rows_cannot_invent_parallel_state_vocabulary: bool,
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SelectionOrLockConsumerProjection {
    /// Items consume the shared selection-or-lock vocabulary.
    pub items_consume_state_vocabulary: bool,
    /// The presentation-posture resolver reads a single canonical source.
    pub presentation_reads_single_source: bool,
    /// The required-disclosure derivation reads a single canonical source.
    pub disclosure_set_reads_single_source: bool,
    /// Support / export reads a single canonical source.
    pub support_export_reads_single_source: bool,
    /// Headless and desktop items read a single canonical source.
    pub headless_and_desktop_read_single_source: bool,
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SelectionOrLockProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the primitive.
    pub auto_narrow_on_stale: bool,
}

/// Release and support parity posture for the selection-or-lock-state contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SelectionOrLockReleasePosture {
    /// Ref of the supporting release packet.
    pub release_packet_ref: String,
    /// Ref of the supporting selection-or-lock-state audit.
    pub selection_or_lock_state_audit_ref: String,
    /// True when support / export parity is required for every item.
    pub support_export_parity_required: bool,
    /// True when accessibility parity is required for every item.
    pub accessibility_parity_required: bool,
}

/// Constructor input for [`M5SelectionOrLockStateContractPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5SelectionOrLockStateContractPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Item rows.
    pub rows: Vec<M5SelectionOrLockItemRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SelectionOrLockVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SelectionOrLockGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SelectionOrLockConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SelectionOrLockProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SelectionOrLockReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe M5 selection-or-lock-state-contract primitive packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5SelectionOrLockStateContractPacket {
    /// Record kind; must equal [`M5_SELECTION_OR_LOCK_STATE_CONTRACT_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_SELECTION_OR_LOCK_STATE_CONTRACT_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable matrix label.
    pub matrix_label: String,
    /// Item rows.
    pub rows: Vec<M5SelectionOrLockItemRow>,
    /// Frozen controlled-vocabulary set.
    pub vocabulary_set: M5SelectionOrLockVocabularySet,
    /// Governance-review block.
    pub governance_review: M5SelectionOrLockGovernanceReview,
    /// Consumer projection block.
    pub consumer_projection: M5SelectionOrLockConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: M5SelectionOrLockProofFreshness,
    /// Release and support parity posture.
    pub release_posture: M5SelectionOrLockReleasePosture,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5SelectionOrLockStateContractPacket {
    /// Builds an M5 selection-or-lock-state-contract-primitive packet from stable-lane input.
    pub fn new(input: M5SelectionOrLockStateContractPacketInput) -> Self {
        Self {
            record_kind: M5_SELECTION_OR_LOCK_STATE_CONTRACT_RECORD_KIND.to_owned(),
            schema_version: M5_SELECTION_OR_LOCK_STATE_CONTRACT_SCHEMA_VERSION,
            packet_id: input.packet_id,
            matrix_label: input.matrix_label,
            rows: input.rows,
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

    /// Validates the M5 selection-or-lock-state-contract-primitive invariants.
    pub fn validate(&self) -> Vec<M5SelectionOrLockStateContractViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_SELECTION_OR_LOCK_STATE_CONTRACT_RECORD_KIND {
            violations.push(M5SelectionOrLockStateContractViolation::WrongRecordKind);
        }
        if self.schema_version != M5_SELECTION_OR_LOCK_STATE_CONTRACT_SCHEMA_VERSION {
            violations.push(M5SelectionOrLockStateContractViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.matrix_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5SelectionOrLockStateContractViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_vocabulary_set(self, &mut violations);
        validate_rows(self, &mut violations);
        validate_selection_or_lock_state_coverage(self, &mut violations);
        validate_presentation_coverage(self, &mut violations);
        validate_cue_coverage(self, &mut violations);
        validate_disclosure_coverage(self, &mut violations);
        validate_guarantees(self, &mut violations);
        validate_identity_preservation(self, &mut violations);
        validate_governance_review(self, &mut violations);
        validate_consumer_projection(self, &mut violations);
        validate_proof_freshness(self, &mut violations);
        validate_release_posture(self, &mut violations);

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("m5 selection or lock state contract primitive packet serializes"),
        ) {
            violations.push(M5SelectionOrLockStateContractViolation::RawMaterialInExport);
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
            .expect("m5 selection or lock state contract primitive packet serializes")
    }

    /// Deterministic, machine-readable matrix CSV: one row per item kind.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "item_kind,qualification,owner,anatomy,selection_or_lock_states,presentations,non_color_cues,required_disclosures,state_examples\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                row.item_kind.as_str(),
                row.qualification.as_str(),
                csv_field(&row.owner_role),
                join_tokens(&row.anatomy_parts, |v| v.as_str()),
                join_state_tokens(&row.selection_or_lock_states),
                join_tokens(&row.presentations, |v| v.as_str()),
                join_tokens(&row.non_color_cues, |v| v.as_str()),
                join_tokens(&row.required_disclosures, |v| v.as_str()),
                row.state_examples.len(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let stable_rows = self
            .rows
            .iter()
            .filter(|row| row.qualification.is_stable())
            .count();
        let mut out = String::new();
        out.push_str("# M5 Selection-or-Lock-State Contract Primitive\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.matrix_label));
        out.push_str(&format!(
            "- Items: {} ({} stable)\n",
            self.rows.len(),
            stable_rows
        ));
        out.push_str(&format!(
            "- Presentations: {}\n",
            self.vocabulary_set.presentations.join(", ")
        ));
        out.push_str(&format!(
            "- Non-color cues: {}\n",
            self.vocabulary_set.non_color_cues.join(", ")
        ));
        out.push_str(&format!(
            "- Selection-or-lock states: {}\n",
            self.vocabulary_set.selection_or_lock_states.join(", ")
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));
        out.push_str("\n## Items\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}**: `{}`\n",
                row.item_kind.label(),
                row.qualification.as_str()
            ));
            out.push_str(&format!("  - Owner: {}\n", row.owner_role));
            out.push_str(&format!("  - Scope: {}\n", row.scope_summary));
            out.push_str(&format!(
                "  - Worked states: {}\n",
                row.state_examples.len()
            ));
            for case in &row.state_examples {
                out.push_str(&format!(
                    "    - `{}` (`{}`) → `{}` (non-color cues {}, lock-owner `{}`, explainable `{}`, recovery `{}`)\n",
                    case.resolved.item_identity_ref,
                    case.resolved.selection_or_lock_state.as_str(),
                    case.resolved.presentation.as_str(),
                    case.resolved.required_non_color_cues.len(),
                    case.resolved.lock_owner.as_str(),
                    case.resolved.explainable,
                    case.resolved.recovery_available,
                ));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in M5 selection-or-lock-state-contract-primitive export.
#[derive(Debug)]
pub enum M5SelectionOrLockStateContractArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5SelectionOrLockStateContractViolation>),
}

impl fmt::Display for M5SelectionOrLockStateContractArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "m5 selection or lock state contract primitive export parse failed: {error}"
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
                    "m5 selection or lock state contract primitive export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for M5SelectionOrLockStateContractArtifactError {}

/// Validation failures emitted by [`M5SelectionOrLockStateContractPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5SelectionOrLockStateContractViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// The controlled vocabulary set drifted from the canonical token lists.
    VocabularySetDrift,
    /// A required item kind is missing from the matrix.
    RequiredItemMissing,
    /// An item row is incomplete.
    RowIncomplete,
    /// A row omits one of the mandatory anatomy parts.
    MandatoryAnatomyMissing,
    /// A row omits one of the mandatory export fields.
    MandatoryExportMissing,
    /// A row omits one of the mandatory labels.
    MandatoryLabelMissing,
    /// A row declares no accessibility routes, or misses keyboard focus or non-color encoding.
    AccessibilityRouteMissing,
    /// A row declares no consumer surfaces.
    ConsumerSurfacesMissing,
    /// A row declares no downgrade triggers.
    DowngradeTriggersMissing,
    /// A row declares no worked state resolutions.
    StateExampleMissing,
    /// A worked resolution case does not match a fresh resolve of its input.
    ExampleResolutionDrift,
    /// A row claiming Stable is missing required proof packet refs.
    StableItemMissingProof,
    /// The worked resolutions do not exercise every selection-or-lock state.
    SelectionOrLockStateCoverageUnproven,
    /// The worked resolutions do not exercise every presentation posture.
    PresentationCoverageUnproven,
    /// The worked resolutions do not exercise every non-color cue.
    CueCoverageUnproven,
    /// The worked resolutions do not exercise every required disclosure.
    DisclosureCoverageUnproven,
    /// A worked resolution does not hold the selected-current-distinct, read-only-inspectable,
    /// lock-not-hidden, no-color-only, owner-recovery, and keyboard/screen-reader guarantees.
    GuaranteesUnproven,
    /// A worked resolution does not preserve its exact item identity, state-style, and disclosure
    /// reference.
    IdentityPreservationUnproven,
    /// A row violates a hard invariant.
    RowInvariantViolated,
    /// Governance review does not satisfy required invariants.
    GovernanceReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Release / support parity posture is incomplete.
    ReleasePostureIncomplete,
    /// Export contains raw sensitive material.
    RawMaterialInExport,
}

impl M5SelectionOrLockStateContractViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::VocabularySetDrift => "vocabulary_set_drift",
            Self::RequiredItemMissing => "required_item_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::MandatoryAnatomyMissing => "mandatory_anatomy_missing",
            Self::MandatoryExportMissing => "mandatory_export_missing",
            Self::MandatoryLabelMissing => "mandatory_label_missing",
            Self::AccessibilityRouteMissing => "accessibility_route_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::StateExampleMissing => "state_example_missing",
            Self::ExampleResolutionDrift => "example_resolution_drift",
            Self::StableItemMissingProof => "stable_item_missing_proof",
            Self::SelectionOrLockStateCoverageUnproven => {
                "selection_or_lock_state_coverage_unproven"
            }
            Self::PresentationCoverageUnproven => "presentation_coverage_unproven",
            Self::CueCoverageUnproven => "cue_coverage_unproven",
            Self::DisclosureCoverageUnproven => "disclosure_coverage_unproven",
            Self::GuaranteesUnproven => "guarantees_unproven",
            Self::IdentityPreservationUnproven => "identity_preservation_unproven",
            Self::RowInvariantViolated => "row_invariant_violated",
            Self::GovernanceReviewIncomplete => "governance_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::ReleasePostureIncomplete => "release_posture_incomplete",
            Self::RawMaterialInExport => "raw_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable M5 selection-or-lock-state-contract-primitive export.
pub fn current_stable_m5_selection_or_lock_state_contract_export(
) -> Result<M5SelectionOrLockStateContractPacket, M5SelectionOrLockStateContractArtifactError> {
    let packet: M5SelectionOrLockStateContractPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-selection-lock-state-contract-primitive-proof/support_export.json"
    )))
    .map_err(M5SelectionOrLockStateContractArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5SelectionOrLockStateContractArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5SelectionOrLockStateContractPacket,
    violations: &mut Vec<M5SelectionOrLockStateContractViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_SCHEMA_REF,
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_DOC_REF,
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_COMPONENT_MATRIX_REF,
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_FOCUS_SELECTION_REF,
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_STATE_RECOVERY_REF,
        M5_SELECTION_OR_LOCK_STATE_CONTRACT_OPERATIONAL_SURFACE_STATE_REF,
    ] {
        if !refs.contains(required) {
            violations.push(M5SelectionOrLockStateContractViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_vocabulary_set(
    packet: &M5SelectionOrLockStateContractPacket,
    violations: &mut Vec<M5SelectionOrLockStateContractViolation>,
) {
    if !packet.vocabulary_set.matches_canonical() {
        violations.push(M5SelectionOrLockStateContractViolation::VocabularySetDrift);
    }
}

fn validate_rows(
    packet: &M5SelectionOrLockStateContractPacket,
    violations: &mut Vec<M5SelectionOrLockStateContractViolation>,
) {
    let present: BTreeSet<M5SelectionOrLockItemKind> =
        packet.rows.iter().map(|row| row.item_kind).collect();
    for required in M5SelectionOrLockItemKind::ALL {
        if !present.contains(&required) {
            violations.push(M5SelectionOrLockStateContractViolation::RequiredItemMissing);
            return;
        }
    }

    for row in &packet.rows {
        if row.owner_role.trim().is_empty()
            || row.scope_summary.trim().is_empty()
            || row.source_contract_refs.is_empty()
            || row.anatomy_parts.is_empty()
            || row.surface_families.is_empty()
            || row.deployment_lines.is_empty()
            || row.selection_or_lock_states.is_empty()
            || row.presentations.is_empty()
            || row.non_color_cues.is_empty()
            || row.required_disclosures.is_empty()
            || row.lock_owner_classes.is_empty()
            || row.state_cause_classes.is_empty()
            || row.export_fields.is_empty()
            || row.required_labels.is_empty()
        {
            violations.push(M5SelectionOrLockStateContractViolation::RowIncomplete);
        }
        if !row.declares_mandatory_anatomy() {
            violations.push(M5SelectionOrLockStateContractViolation::MandatoryAnatomyMissing);
        }
        if !row.declares_mandatory_export() {
            violations.push(M5SelectionOrLockStateContractViolation::MandatoryExportMissing);
        }
        if !row.declares_mandatory_labels() {
            violations.push(M5SelectionOrLockStateContractViolation::MandatoryLabelMissing);
        }
        if row.accessibility_routes.is_empty()
            || !row
                .accessibility_routes
                .contains(&M5ComponentStateAccessibilityRoute::KeyboardFocusable)
            || !row
                .accessibility_routes
                .contains(&M5ComponentStateAccessibilityRoute::NonColorEncoded)
        {
            violations.push(M5SelectionOrLockStateContractViolation::AccessibilityRouteMissing);
        }
        if row.consumer_surfaces.is_empty() {
            violations.push(M5SelectionOrLockStateContractViolation::ConsumerSurfacesMissing);
        }
        if row.downgrade_triggers.is_empty() {
            violations.push(M5SelectionOrLockStateContractViolation::DowngradeTriggersMissing);
        }
        if row.state_examples.is_empty() {
            violations.push(M5SelectionOrLockStateContractViolation::StateExampleMissing);
        }
        if row
            .state_examples
            .iter()
            .any(|case| !case.is_self_consistent())
        {
            violations.push(M5SelectionOrLockStateContractViolation::ExampleResolutionDrift);
        }
        if row.qualification.is_stable() && row.required_proof_packet_refs.is_empty() {
            violations.push(M5SelectionOrLockStateContractViolation::StableItemMissingProof);
        }
        if !row.honours_invariants() {
            violations.push(M5SelectionOrLockStateContractViolation::RowInvariantViolated);
        }
    }
}

/// Every selection-or-lock state must be exercised by some worked resolution — the implementation
/// requirement that selected, current, disabled, read-only, and locked states are all wired
/// explicitly.
fn validate_selection_or_lock_state_coverage(
    packet: &M5SelectionOrLockStateContractPacket,
    violations: &mut Vec<M5SelectionOrLockStateContractViolation>,
) {
    let exercised: BTreeSet<M5SharedComponentStateClass> = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .map(|case| case.resolved.selection_or_lock_state)
        .collect();
    let covered = selection_or_lock_states()
        .iter()
        .all(|state| exercised.contains(state));
    if !covered {
        violations
            .push(M5SelectionOrLockStateContractViolation::SelectionOrLockStateCoverageUnproven);
    }
}

/// Every presentation posture must be exercised by some worked resolution.
fn validate_presentation_coverage(
    packet: &M5SelectionOrLockStateContractPacket,
    violations: &mut Vec<M5SelectionOrLockStateContractViolation>,
) {
    let exercised: BTreeSet<M5SelectionOrLockPresentation> = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .map(|case| case.resolved.presentation)
        .collect();
    let covered = M5SelectionOrLockPresentation::ALL
        .iter()
        .all(|posture| exercised.contains(posture));
    if !covered {
        violations.push(M5SelectionOrLockStateContractViolation::PresentationCoverageUnproven);
    }
}

/// Every non-color cue must be exercised by some worked resolution — the acceptance criterion that
/// state meaning never depends on color alone.
fn validate_cue_coverage(
    packet: &M5SelectionOrLockStateContractPacket,
    violations: &mut Vec<M5SelectionOrLockStateContractViolation>,
) {
    let cases = || packet.rows.iter().flat_map(|row| row.state_examples.iter());
    let covered = M5SelectionOrLockCue::ALL
        .iter()
        .all(|cue| cases().any(|case| case.resolved.required_non_color_cues.contains(cue)));
    if !covered {
        violations.push(M5SelectionOrLockStateContractViolation::CueCoverageUnproven);
    }
}

/// Every required disclosure must be exercised by some worked resolution — the requirement that an
/// explainable state always names its cause, owner / block reason, and recovery action.
fn validate_disclosure_coverage(
    packet: &M5SelectionOrLockStateContractPacket,
    violations: &mut Vec<M5SelectionOrLockStateContractViolation>,
) {
    let cases = || packet.rows.iter().flat_map(|row| row.state_examples.iter());
    let covered = M5StateDisclosureTrigger::ALL
        .iter()
        .all(|trigger| cases().any(|case| case.resolved.required_disclosures.contains(trigger)));
    if !covered {
        violations.push(M5SelectionOrLockStateContractViolation::DisclosureCoverageUnproven);
    }
}

/// Every worked resolution must hold the selected-current-distinct, read-only-inspectable,
/// lock-not-hidden, no-color-only, owner-recovery, and keyboard/screen-reader guarantees — the core
/// acceptance criteria.
fn validate_guarantees(
    packet: &M5SelectionOrLockStateContractPacket,
    violations: &mut Vec<M5SelectionOrLockStateContractViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .all(|case| case.preserves_guarantees());
    if !preserved {
        violations.push(M5SelectionOrLockStateContractViolation::GuaranteesUnproven);
    }
}

/// Every worked resolution must preserve its exact item identity, state-style, and disclosure
/// reference — the invariant that the contract never rewrites what it renders or discloses.
fn validate_identity_preservation(
    packet: &M5SelectionOrLockStateContractPacket,
    violations: &mut Vec<M5SelectionOrLockStateContractViolation>,
) {
    let preserved = packet
        .rows
        .iter()
        .flat_map(|row| row.state_examples.iter())
        .all(|case| case.preserves_identity());
    if !preserved {
        violations.push(M5SelectionOrLockStateContractViolation::IdentityPreservationUnproven);
    }
}

fn validate_governance_review(
    packet: &M5SelectionOrLockStateContractPacket,
    violations: &mut Vec<M5SelectionOrLockStateContractViolation>,
) {
    let review = &packet.governance_review;
    for ok in [
        review.items_distinguish_selected_current_read_only_disabled_locked,
        review.selected_and_current_never_collapse,
        review.read_only_never_collapses_into_disabled,
        review.locked_never_hidden_behind_disabled,
        review.state_meaning_never_color_only,
        review.owner_source_recovery_surfaced_when_explainable,
        review.states_keyboard_and_screen_reader_explainable,
        review.states_driven_by_shared_contract_and_tokens,
        review.no_one_off_per_surface_styling,
        review.states_stable_across_deployment_lines,
        review.states_stable_across_consumer_surfaces,
        review.every_item_declares_accessibility_route,
        review.support_export_reconstructs_state_truth,
        review.later_rows_cannot_invent_parallel_state_vocabulary,
    ] {
        if !ok {
            violations.push(M5SelectionOrLockStateContractViolation::GovernanceReviewIncomplete);
            return;
        }
    }
}

fn validate_consumer_projection(
    packet: &M5SelectionOrLockStateContractPacket,
    violations: &mut Vec<M5SelectionOrLockStateContractViolation>,
) {
    let projection = &packet.consumer_projection;
    for ok in [
        projection.items_consume_state_vocabulary,
        projection.presentation_reads_single_source,
        projection.disclosure_set_reads_single_source,
        projection.support_export_reads_single_source,
        projection.headless_and_desktop_read_single_source,
    ] {
        if !ok {
            violations.push(M5SelectionOrLockStateContractViolation::ConsumerProjectionIncomplete);
            return;
        }
    }
}

fn validate_proof_freshness(
    packet: &M5SelectionOrLockStateContractPacket,
    violations: &mut Vec<M5SelectionOrLockStateContractViolation>,
) {
    if packet.proof_freshness.proof_freshness_slo_hours == 0
        || packet.proof_freshness.last_proof_refresh.trim().is_empty()
    {
        violations.push(M5SelectionOrLockStateContractViolation::ProofFreshnessIncomplete);
    }
}

fn validate_release_posture(
    packet: &M5SelectionOrLockStateContractPacket,
    violations: &mut Vec<M5SelectionOrLockStateContractViolation>,
) {
    let posture = &packet.release_posture;
    if posture.release_packet_ref.trim().is_empty()
        || posture.selection_or_lock_state_audit_ref.trim().is_empty()
        || !posture.support_export_parity_required
        || !posture.accessibility_parity_required
    {
        violations.push(M5SelectionOrLockStateContractViolation::ReleasePostureIncomplete);
    }
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
fn join_tokens<T, F>(items: &[T], to_token: F) -> String
where
    F: Fn(&T) -> &'static str,
{
    items.iter().map(to_token).collect::<Vec<_>>().join("|")
}

/// Joins selection-or-lock-state tokens for a CSV cell with a `|` separator.
fn join_state_tokens(items: &[M5SharedComponentStateClass]) -> String {
    items
        .iter()
        .map(|state| state.as_str())
        .collect::<Vec<_>>()
        .join("|")
}

/// Quotes a free-text CSV field when it contains a comma or quote.
fn csv_field(value: &str) -> String {
    if value.contains(',') || value.contains('"') || value.contains('\n') {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// True when a single representation carries obviously forbidden material.
fn value_repr_is_forbidden(value: &str) -> bool {
    let lower = value.to_lowercase();
    lower.contains("api_key")
        || lower.contains("password")
        || lower.contains("secret")
        || lower.contains("bearer ")
        || lower.contains("://")
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => value_repr_is_forbidden(s),
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}
