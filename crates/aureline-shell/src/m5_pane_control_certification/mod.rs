//! Splitter and resizable-pane control precision, proportion-safe persistence,
//! default-size reset/restore, and support-export truth certified across every
//! claimed M5 multi-pane layout.
//!
//! The [frozen shell-primitives matrix][matrix] already freezes the pane-control
//! primitives — the splitter handle and the named pane-resize preset — into one
//! export-safe packet: their pane-resize states, accessibility routes, the mandatory
//! labels every pane control must be able to show, and the downgrade triggers that
//! narrow them below a claim. This lane is the **pane-control certification capstone**
//! on top of that matrix: for every claimed M5 multi-pane layout — the notebook, data,
//! review/change, docs, profiler, and incident lanes — it certifies that a splitter or
//! resizable pane can be resized precisely with both pointer and keyboard inputs
//! (enlarged logical hit targets, hover/focus strengthen states, keyboard step-size
//! controls, and double-click / default-size restore); that resize intent persists as
//! proportions or named presets rather than brittle pixel positions, and that
//! compact/expanded or monitor-topology changes preserve that intent safely; that
//! reset-to-default and restore after crash or display/topology changes stay lossless
//! and non-destructive; and that current pane proportions and recent resize actions are
//! reconstructable from a support export without screenshots or manual reproduction.
//!
//! Three records carry the truth:
//!
//! - the per-layout **certification row** ([`PaneControlCertificationRow`]): one row per
//!   [`M5PaneLayout`] naming the pane-control primitives it drives, the pane-resize
//!   states / required labels / accessibility routes / consumer surfaces / downgrade
//!   triggers pulled from the frozen matrix, its resize-control-precision /
//!   proportion-persistence / reset-restore / resize-export posture, any active waiver,
//!   and a derived green/yellow/red [`PaneControlCertificationStatus`].
//! - the release **certification packet** ([`PaneControlCertificationPacket`]): the full
//!   set of rows with derived per-row status, aggregate green/yellow/red counts, the
//!   active waivers, the exact certification causes ([`PaneControlCertificationCause`]),
//!   and the blocking findings the lane refuses to ship with.
//! - the **certification dashboard** ([`PaneControlCertificationDashboard`]): a light
//!   projection the shell / layout engine / release automation reads to auto-narrow a
//!   claimed layout when its pane-control proof falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow`
//! the moment it discloses a reduced hit target or keyboard step, a reduced persistence
//! fidelity, a reduced restore fidelity (backed by a waiver), or a partial
//! support-export capture; it drops to `red` if a pane is pointer-only or resizes
//! through a brittle hit target, resize intent persists only as brittle pixels, restore
//! after display/topology change is lost or destructive, the resize state is absent from
//! the support-export capture, a pane is resizable by pointer only, or its pane-resize
//! states / required labels are incomplete. That derivation is the auto-narrowing the
//! acceptance criteria require.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw
//! URLs, raw local paths, raw usernames, raw hostnames, tokens, or credentials — only
//! stable ids, closed vocabulary, counts, refs, and short labels. The pane-resize-state,
//! accessibility-route, required-label, consumer-surface, downgrade-trigger, and
//! qualification vocabulary is re-exported by reference from the already frozen
//! [matrix]; each row pulls its pane-control bindings straight from that matrix's seeded
//! splitter-handle and pane-resize-preset rows, so this lane mints no parallel shell
//! vocabulary and cannot certify a pane-control posture the matrix does not freeze. Only
//! the certification-specific vocabulary ([`M5PaneLayout`],
//! [`M5PaneControlProofDimension`], [`PaneControlCertificationStatus`],
//! [`ResizeControlPrecisionState`], [`ProportionPersistenceState`],
//! [`ResetRestoreState`], [`ResizeExportState`], [`PaneControlCertificationWaiver`],
//! [`PaneControlCertificationCause`], [`PaneControlCertificationFinding`]) is new.
//!
//! Unlike the ambient, transient-inspect, and progress primitives, pane controls carry
//! **no** source/provider/freshness truth — a splitter has no cached-versus-live value —
//! so this lane certifies neither freshness labels nor representation/promotion truth.
//! Its required-label completeness lint checks the four pane-control labels the matrix
//! freezes ([`PANE_CONTROL_REQUIRED_LABELS`]: identity, state, keyboard-route, and
//! reopen-path) rather than the full six-label set.
//!
//! [matrix]: crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix as matrix;

pub use matrix::{
    M5AccessibilityRoute, M5PaneResizeState, M5PrimitiveQualificationClass,
    M5PrimitiveRequiredLabel, M5ShellConsumerSurface, M5ShellPrimitiveDowngradeTrigger,
    M5ShellPrimitiveFamily, M5ShellZoneSlot,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_pane_control_certification_packet,
    seeded_m5_pane_control_certification_packet_data_pixel_only_persistence_blocked,
    seeded_m5_pane_control_certification_packet_docs_resize_absent_from_capture_blocked,
    seeded_m5_pane_control_certification_packet_incident_pointer_only_resizable_blocked,
    seeded_m5_pane_control_certification_packet_notebook_pointer_only_resize_blocked,
    seeded_m5_pane_control_certification_packet_review_restore_destructive_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_PANE_CONTROL_CERTIFICATION_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_PANE_CONTROL_CERTIFICATION_SHARED_CONTRACT_REF: &str =
    "shell:m5_pane_control_certification:v1";

/// Stable record kind for [`PaneControlCertificationPacket`] payloads.
pub const M5_PANE_CONTROL_CERTIFICATION_PACKET_RECORD_KIND: &str =
    "shell_m5_pane_control_certification_packet_record";

/// Stable record kind for [`PaneControlCertificationDashboard`] payloads.
pub const M5_PANE_CONTROL_CERTIFICATION_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_pane_control_certification_dashboard_record";

/// Stable record kind for [`PaneControlCertificationSupportExport`] payloads.
pub const M5_PANE_CONTROL_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_pane_control_certification_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_PANE_CONTROL_CERTIFICATION_PACKET_ID: &str =
    "m5-pane-control-certification:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_PANE_CONTROL_CERTIFICATION_DASHBOARD_ID: &str =
    "m5-pane-control-certification-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_PANE_CONTROL_CERTIFICATION_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-pane-control-certification:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_PANE_CONTROL_CERTIFICATION_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-pane-control-certification.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_PANE_CONTROL_CERTIFICATION_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-pane-control-certification.md";

/// Published certification-packet artifact ref.
pub const M5_PANE_CONTROL_CERTIFICATION_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-pane-control-certification-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_PANE_CONTROL_CERTIFICATION_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-pane-control-certification-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_PANE_CONTROL_CERTIFICATION_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-pane-control-certification-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_PANE_CONTROL_CERTIFICATION_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-pane-control-certification-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_PANE_CONTROL_CERTIFICATION_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_pane_control_certification_contract.md";

/// Repo-relative ref to the frozen shell-primitives matrix schema.
pub const M5_PANE_CONTROL_CERTIFICATION_MATRIX_SCHEMA_REF: &str =
    matrix::M5_SHELL_PRIMITIVES_SCHEMA_REF;

/// The four labels every pane control must be able to show. Pane controls carry no
/// source/provider or freshness truth, so this is a strict subset of
/// [`M5PrimitiveRequiredLabel::ALL`]: identity, state, keyboard route, and reopen path.
pub const PANE_CONTROL_REQUIRED_LABELS: [M5PrimitiveRequiredLabel; 4] = [
    M5PrimitiveRequiredLabel::Identity,
    M5PrimitiveRequiredLabel::State,
    M5PrimitiveRequiredLabel::KeyboardRoute,
    M5PrimitiveRequiredLabel::ReopenPath,
];

/// One of the claimed M5 multi-pane layouts the certification proof must cover, in
/// canonical order. Each layout is a claimed M5 shell lane whose surfaces render
/// splitters and resizable panes; the lane certifies none beyond them and refuses to
/// ship if any is missing. Detached windows, compact-width sheets, and restoration after
/// crash or display changes are certified within each layout's row rather than as
/// separate layouts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PaneLayout {
    /// Notebook lane: cell list, editor, and output panes.
    Notebook,
    /// Data / API-run lane: query, grid, and inspector panes.
    Data,
    /// Review / change-request lane: tree, diff, and comment panes.
    Review,
    /// Docs / help lane: navigation, article, and preview panes.
    Docs,
    /// Profiler / performance-capture lane: timeline, flame-graph, and detail panes.
    Profiler,
    /// Incident / operator-console lane: signal, log, and action panes.
    Incident,
}

impl M5PaneLayout {
    /// Every governed pane layout, in canonical order.
    pub const ALL: [Self; 6] = [
        Self::Notebook,
        Self::Data,
        Self::Review,
        Self::Docs,
        Self::Profiler,
        Self::Incident,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Notebook => "notebook",
            Self::Data => "data",
            Self::Review => "review",
            Self::Docs => "docs",
            Self::Profiler => "profiler",
            Self::Incident => "incident",
        }
    }

    /// Short, reviewer-facing label for the layout's pane-control surfaces.
    pub const fn label(self) -> &'static str {
        match self {
            Self::Notebook => "Notebook cell / editor / output splitters",
            Self::Data => "Data grid query / grid / inspector splitters",
            Self::Review => "Review tree / diff / comment splitters",
            Self::Docs => "Docs nav / article / preview splitters",
            Self::Profiler => "Profiler timeline / flame-graph / detail splitters",
            Self::Incident => "Incident signal / log / action splitters",
        }
    }
}

/// One of the four certification dimensions each pane layout is certified across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PaneControlProofDimension {
    /// Resize-control precision (hit targets, focus states, keyboard step, default reset).
    ResizeControlPrecision,
    /// Proportion-safe persistence (proportions/presets, not brittle pixels).
    ProportionPersistence,
    /// Reset / restore (default reset + restore after crash / display / topology change).
    ResetRestore,
    /// Resize-state export (proportions + recent actions reconstructable from support).
    ResizeStateExport,
}

impl M5PaneControlProofDimension {
    /// Every certification dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::ResizeControlPrecision,
        Self::ProportionPersistence,
        Self::ResetRestore,
        Self::ResizeStateExport,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResizeControlPrecision => "resize_control_precision",
            Self::ProportionPersistence => "proportion_persistence",
            Self::ResetRestore => "reset_restore",
            Self::ResizeStateExport => "resize_state_export",
        }
    }
}

/// The derived certification light a governed pane layout carries.
///
/// `green` means the layout's splitters and resizable panes are resized precisely with
/// pointer and keyboard, persist resize intent as proportions or named presets, reset to
/// a default and restore after display/topology change losslessly, and export current
/// proportions and recent resize actions. `yellow` is a disclosed narrowing (a reduced
/// hit target or keyboard step, a reduced persistence fidelity, a waivered reduced
/// restore fidelity, or a partial support-export capture). `red` is blocked: a pane is
/// pointer-only or resizes through a brittle hit target, resize intent persists only as
/// brittle pixels, restore is lost or destructive, the resize state is absent from
/// capture, a pane is resizable by pointer only, or the pane-resize states / required
/// labels are incomplete — and the layout may not keep a shell-maturity claim until
/// repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PaneControlCertificationStatus {
    /// Full standing: precise resize, proportion-safe persistence, lossless restore.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl PaneControlCertificationStatus {
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

/// How the layout's splitters and panes are resized precisely with pointer and keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResizeControlPrecisionState {
    /// Every splitter offers an enlarged logical hit target, hover/focus strengthen
    /// states, keyboard step-size controls, and a double-click / default-size restore —
    /// resizable precisely by both pointer and keyboard.
    PrecisePointerAndKeyboardResize,
    /// Under compact width one precision affordance is disclosedly reduced (the enlarged
    /// hit target shrinks or a coarse keyboard step is used) while pointer and keyboard
    /// resize both still resolve and the default-size restore stays reachable.
    DisclosedReducedHitTargetOrStep,
    /// A pane is resizable only through the pointer, or its hit target is so brittle it
    /// cannot be reliably grabbed — always a blocker.
    PointerOnlyOrBrittleHitTarget,
}

impl ResizeControlPrecisionState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrecisePointerAndKeyboardResize => "precise_pointer_and_keyboard_resize",
            Self::DisclosedReducedHitTargetOrStep => "disclosed_reduced_hit_target_or_step",
            Self::PointerOnlyOrBrittleHitTarget => "pointer_only_or_brittle_hit_target",
        }
    }

    /// `true` when resize is precise on both pointer and keyboard.
    pub const fn is_precise(self) -> bool {
        matches!(self, Self::PrecisePointerAndKeyboardResize)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedHitTargetOrStep)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::PointerOnlyOrBrittleHitTarget)
    }
}

/// How the layout persists resize intent as proportions or presets rather than pixels.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProportionPersistenceState {
    /// Resize intent is persisted as proportions or named presets, and compact/expanded
    /// or monitor-topology changes preserve that intent safely.
    ProportionsOrPresetsPersisted,
    /// The persistence fidelity is disclosedly reduced under one topology (a preset
    /// snaps to the nearest safe ratio) while the intent stays serialized as proportions
    /// rather than pixels and the reduction is disclosed.
    DisclosedReducedPersistenceFidelity,
    /// Resize intent persists only as brittle pixel positions, so a compact/expanded or
    /// monitor-topology change loses or corrupts the layout — always a blocker.
    BrittlePixelOnlyPersistence,
}

impl ProportionPersistenceState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProportionsOrPresetsPersisted => "proportions_or_presets_persisted",
            Self::DisclosedReducedPersistenceFidelity => "disclosed_reduced_persistence_fidelity",
            Self::BrittlePixelOnlyPersistence => "brittle_pixel_only_persistence",
        }
    }

    /// `true` when resize intent persists as proportions or presets.
    pub const fn is_persisted(self) -> bool {
        matches!(self, Self::ProportionsOrPresetsPersisted)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedPersistenceFidelity)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::BrittlePixelOnlyPersistence)
    }
}

/// How the layout resets to a default size and restores after crash / display / topology
/// changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResetRestoreState {
    /// A double-click or command resets a pane to its named default size, and restore
    /// after crash, display change, or monitor-topology change reconstructs the resize
    /// intent without loss and without a destructive collapse.
    DefaultResetAndTopologyRestore,
    /// The restore fidelity is disclosedly reduced (a detached window falls back to a
    /// safe default layout after a display change) while the reset-to-default path and a
    /// non-destructive restore still resolve; the reduction is disclosed and waivered.
    DisclosedReducedRestoreFidelity,
    /// Restore after a crash or display/topology change is lost or destructive — a pane
    /// vanishes with no reopen path or the layout collapses to an unusable state —
    /// always a blocker.
    RestoreLostOrDestructive,
}

impl ResetRestoreState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DefaultResetAndTopologyRestore => "default_reset_and_topology_restore",
            Self::DisclosedReducedRestoreFidelity => "disclosed_reduced_restore_fidelity",
            Self::RestoreLostOrDestructive => "restore_lost_or_destructive",
        }
    }

    /// `true` when reset and restore are lossless and non-destructive.
    pub const fn is_restored(self) -> bool {
        matches!(self, Self::DefaultResetAndTopologyRestore)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedRestoreFidelity)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::RestoreLostOrDestructive)
    }
}

/// How current pane proportions and recent resize actions are reconstructable from a
/// support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResizeExportState {
    /// The support export reconstructs current pane proportions and recent resize actions
    /// against the serialized ratios so a layout bug can be diagnosed without screenshots
    /// or manual reproduction.
    ProportionsAndActionsReconstructable,
    /// The support export reconstructs current proportions and discloses a partial
    /// capture of the recent resize-action log while it is still being trimmed.
    DisclosedPartialCapture,
    /// Current pane proportions or the recent resize-action log are absent from the
    /// support-export capture, so a layout bug cannot be explained without a screenshot —
    /// always a blocker.
    ResizeStateAbsentFromCapture,
}

impl ResizeExportState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProportionsAndActionsReconstructable => "proportions_and_actions_reconstructable",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::ResizeStateAbsentFromCapture => "resize_state_absent_from_capture",
        }
    }

    /// `true` when the export reconstructs proportions and actions.
    pub const fn is_reconstructable(self) -> bool {
        matches!(self, Self::ProportionsAndActionsReconstructable)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::ResizeStateAbsentFromCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red restore narrowing stay
/// yellow rather than blocked — never lets a pointer-only pane, a brittle pixel
/// persistence, a destructive restore, or a missing export hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneControlCertificationWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed layout the waiver applies to.
    pub layout: M5PaneLayout,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl PaneControlCertificationWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed layout's certification.
///
/// The trigger token mirrors the frozen [`M5ShellPrimitiveDowngradeTrigger`] vocabulary
/// so a cause never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneControlCertificationCause {
    /// The governed layout the cause applies to.
    pub layout: M5PaneLayout,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5ShellPrimitiveDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a
    /// non-disclosed cause is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl PaneControlCertificationCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed pane layout, certified across resize-control precision, proportion-safe
/// persistence, reset/restore, and resize-state export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneControlCertificationRow {
    /// The governed layout being certified.
    pub layout: M5PaneLayout,
    /// The pane-control primitives this layout drives. Pulled from the matrix.
    pub driven_primitive_families: Vec<M5ShellPrimitiveFamily>,
    /// The frozen qualification class of the driven pane-control primitives (the
    /// most-narrowed of the two). Pulled from the matrix.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Owner role accountable for keeping this layout certified.
    pub owner_role: String,
    /// Short layout-surface label.
    pub layout_label: String,
    /// The canonical shell zone the pane-control surfaces attach to. Pulled from the
    /// matrix.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Pane-resize states these surfaces honour (union across the two pane families).
    /// Pulled from the matrix.
    pub certified_pane_resize_states: Vec<M5PaneResizeState>,
    /// Non-visual accessibility routes. Pulled from the matrix.
    pub accessibility_routes: Vec<M5AccessibilityRoute>,
    /// Mandatory labels every pane control must be able to show. Pulled from the matrix.
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// Shell subsystems this layout stays aligned across. Pulled from the matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this layout. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ShellPrimitiveDowngradeTrigger>,
    /// Resize-control-precision posture.
    pub resize_control_precision: ResizeControlPrecisionState,
    /// Proportion-persistence posture.
    pub proportion_persistence: ProportionPersistenceState,
    /// Reset-restore posture.
    pub reset_restore: ResetRestoreState,
    /// Resize-export posture.
    pub resize_export: ResizeExportState,
    /// Hard invariant: a pane is never resizable by pointer only. `false` is a blocker.
    pub pane_never_pointer_only_resizable: bool,
    /// Active waiver, when a disclosed restore-fidelity reduction is in force.
    pub active_waiver: Option<PaneControlCertificationWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: PaneControlCertificationStatus,
    /// The exact certification causes that narrowed or blocked this row.
    pub certification_causes: Vec<PaneControlCertificationCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl PaneControlCertificationRow {
    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when every pane-resize state the matrix freezes is certified — the lint
    /// that prevents a pane control from shipping without its full idle / dragging /
    /// keyboard-step / snapped / reset / clamped / collapsed transition set.
    pub fn pane_resize_states_complete(&self) -> bool {
        let present: BTreeSet<M5PaneResizeState> =
            self.certified_pane_resize_states.iter().copied().collect();
        M5PaneResizeState::ALL
            .iter()
            .all(|state| present.contains(state))
    }

    /// `true` when every pane-control required label is certified — the lint that
    /// prevents a pane control from shipping without identity, state, keyboard-route, and
    /// reopen-path labels. Pane controls carry no source/provider or freshness label, so
    /// the required set is [`PANE_CONTROL_REQUIRED_LABELS`], not the full six.
    pub fn required_labels_complete(&self) -> bool {
        let present: BTreeSet<M5PrimitiveRequiredLabel> =
            self.required_labels.iter().copied().collect();
        PANE_CONTROL_REQUIRED_LABELS
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        self.resize_control_precision.is_blocked()
            || self.proportion_persistence.is_blocked()
            || self.reset_restore.is_blocked()
            || self.resize_export.is_blocked()
            || !self.pane_never_pointer_only_resizable
            || !self.pane_resize_states_complete()
            || !self.required_labels_complete()
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.resize_control_precision.is_disclosed()
            || self.proportion_persistence.is_disclosed()
            || self.reset_restore.is_disclosed()
            || self.resize_export.is_disclosed()
    }

    /// Recomputes the derived status from the four axes and the pointer-only invariant.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest
    /// narrowing forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> PaneControlCertificationStatus {
        if self.has_hard_blocker() {
            PaneControlCertificationStatus::Red
        } else if self.has_narrowing() {
            PaneControlCertificationStatus::Yellow
        } else {
            PaneControlCertificationStatus::Green
        }
    }

    /// Recomputes the exact certification causes for the row, in deterministic order
    /// (precision, persistence, reset/restore, export, pointer-only invariant).
    pub fn recompute_causes(&self) -> Vec<PaneControlCertificationCause> {
        let mut causes = Vec::new();
        if !self.resize_control_precision.is_precise() {
            causes.push(PaneControlCertificationCause {
                layout: self.layout,
                trigger: M5ShellPrimitiveDowngradeTrigger::PointerOnlyResize,
                disclosed: self.resize_control_precision.is_disclosed(),
                detail: if self.resize_control_precision.is_disclosed() {
                    "Under compact width one precision affordance (the enlarged hit target or a \
                     fine keyboard step) is disclosedly reduced while pointer and keyboard resize \
                     both still resolve and the default-size restore stays reachable; the \
                     reduction is disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "A pane is resizable only through the pointer, or its hit target is so brittle \
                     it cannot be reliably grabbed, so precise resize is not available."
                        .to_owned()
                },
            });
        }
        if !self.proportion_persistence.is_persisted() {
            causes.push(PaneControlCertificationCause {
                layout: self.layout,
                trigger: M5ShellPrimitiveDowngradeTrigger::ResizeStateNotSerializable,
                disclosed: self.proportion_persistence.is_disclosed(),
                detail: if self.proportion_persistence.is_disclosed() {
                    "The persistence fidelity is disclosedly reduced under one topology (a preset \
                     snaps to the nearest safe ratio) while the intent stays serialized as \
                     proportions rather than pixels; the reduction is disclosed and the row is \
                     narrowed below green."
                        .to_owned()
                } else {
                    "Resize intent persists only as brittle pixel positions, so a compact/expanded \
                     or monitor-topology change loses or corrupts the layout."
                        .to_owned()
                },
            });
        }
        if !self.reset_restore.is_restored() {
            causes.push(PaneControlCertificationCause {
                layout: self.layout,
                trigger: M5ShellPrimitiveDowngradeTrigger::ResizeStateNotSerializable,
                disclosed: self.reset_restore.is_disclosed(),
                detail: if self.reset_restore.is_disclosed() {
                    "The restore fidelity is disclosedly reduced (a detached window falls back to a \
                     safe default layout after a display change) while the reset-to-default path \
                     and a non-destructive restore still resolve; the reduction is disclosed and \
                     waivered, and the row is narrowed below green."
                        .to_owned()
                } else {
                    "Restore after a crash or display/topology change is lost or destructive — a \
                     pane vanishes with no reopen path or the layout collapses to an unusable \
                     state."
                        .to_owned()
                },
            });
        }
        if !self.resize_export.is_reconstructable() {
            causes.push(PaneControlCertificationCause {
                layout: self.layout,
                trigger: M5ShellPrimitiveDowngradeTrigger::ProofStale,
                disclosed: self.resize_export.is_disclosed(),
                detail: if self.resize_export.is_disclosed() {
                    "The support export reconstructs current pane proportions and discloses a \
                     partial capture of the recent resize-action log while it is still being \
                     trimmed; the partial capture is disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "Current pane proportions or the recent resize-action log are absent from the \
                     support-export capture, so a layout bug cannot be explained without a \
                     screenshot or manual reproduction."
                        .to_owned()
                },
            });
        }
        if !self.pane_never_pointer_only_resizable {
            causes.push(PaneControlCertificationCause {
                layout: self.layout,
                trigger: M5ShellPrimitiveDowngradeTrigger::PointerOnlyResize,
                disclosed: false,
                detail:
                    "A pane is resizable by pointer only, with no keyboard step-size route, so \
                         its resize affordance is pointer-only."
                        .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced restore fidelity may only stay yellow (rather than red) when a
    /// waiver discloses it.
    pub fn requires_waiver(&self) -> bool {
        self.reset_restore.is_disclosed()
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<PaneControlCertificationFinding> {
        let mut findings = Vec::new();
        let layout = self.layout.as_str().to_owned();

        if self.resize_control_precision.is_blocked() {
            findings.push(PaneControlCertificationFinding::ResizeControlNotPrecise {
                layout: layout.clone(),
            });
        }
        if self.proportion_persistence.is_blocked() {
            findings.push(
                PaneControlCertificationFinding::PersistenceBrittlePixelOnly {
                    layout: layout.clone(),
                },
            );
        }
        if self.reset_restore.is_blocked() {
            findings.push(PaneControlCertificationFinding::RestoreLostOrDestructive {
                layout: layout.clone(),
            });
        }
        if self.resize_export.is_blocked() {
            findings.push(
                PaneControlCertificationFinding::ResizeStateAbsentFromCapture {
                    layout: layout.clone(),
                },
            );
        }
        if !self.pane_never_pointer_only_resizable {
            findings.push(PaneControlCertificationFinding::PanePointerOnlyResizable {
                layout: layout.clone(),
            });
        }
        if !self.pane_resize_states_complete() {
            findings.push(
                PaneControlCertificationFinding::PaneResizeStatesIncomplete {
                    layout: layout.clone(),
                },
            );
        }
        if !self.required_labels_complete() {
            findings.push(PaneControlCertificationFinding::RequiredLabelsIncomplete {
                layout: layout.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, PaneControlCertificationStatus::Green) && !self.has_reason() {
            findings.push(PaneControlCertificationFinding::NarrowedRowWithoutReason {
                layout: layout.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an
        // active waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(PaneControlCertificationFinding::NarrowedRowWithoutWaiver {
                layout: layout.clone(),
            });
        }
        // An attached waiver must still be active and must point at this layout.
        if let Some(waiver) = &self.active_waiver {
            if waiver.layout != self.layout {
                findings.push(PaneControlCertificationFinding::WaiverLayoutMismatch {
                    layout: layout.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(PaneControlCertificationFinding::WaiverExpired {
                    layout: layout.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(PaneControlCertificationFinding::RowStatusStale {
                layout: layout.clone(),
            });
        }
        if self.certification_causes != self.recompute_causes() {
            findings.push(PaneControlCertificationFinding::RowCausesStale { layout });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} precision={} persistence={} restore={} export={} keyboard_resizable={} waiver={}",
            self.layout.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.resize_control_precision.as_str(),
            self.proportion_persistence.as_str(),
            self.reset_restore.as_str(),
            self.resize_export.as_str(),
            self.pane_never_pointer_only_resizable,
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the pane-control certification proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum PaneControlCertificationFinding {
    /// A governed pane layout has no certification row.
    LayoutMissing {
        /// The missing layout token.
        layout: String,
    },
    /// A layout's pane is pointer-only or resizes through a brittle hit target.
    ResizeControlNotPrecise {
        /// The layout token.
        layout: String,
    },
    /// A layout persists resize intent only as brittle pixels.
    PersistenceBrittlePixelOnly {
        /// The layout token.
        layout: String,
    },
    /// A layout's restore after display/topology change is lost or destructive.
    RestoreLostOrDestructive {
        /// The layout token.
        layout: String,
    },
    /// A layout's resize state is absent from the support-export capture.
    ResizeStateAbsentFromCapture {
        /// The layout token.
        layout: String,
    },
    /// A layout has a pane resizable by pointer only.
    PanePointerOnlyResizable {
        /// The layout token.
        layout: String,
    },
    /// A layout does not certify every frozen pane-resize state.
    PaneResizeStatesIncomplete {
        /// The layout token.
        layout: String,
    },
    /// A layout does not certify every pane-control required label.
    RequiredLabelsIncomplete {
        /// The layout token.
        layout: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The layout token.
        layout: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The layout token.
        layout: String,
    },
    /// An attached waiver does not point at the row's layout.
    WaiverLayoutMismatch {
        /// The layout token.
        layout: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The layout token.
        layout: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The layout token.
        layout: String,
    },
    /// The declared certification causes do not match the recomputed causes.
    RowCausesStale {
        /// The layout token.
        layout: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered layouts do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl PaneControlCertificationFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::LayoutMissing { .. } => "layout_missing",
            Self::ResizeControlNotPrecise { .. } => "resize_control_not_precise",
            Self::PersistenceBrittlePixelOnly { .. } => "persistence_brittle_pixel_only",
            Self::RestoreLostOrDestructive { .. } => "restore_lost_or_destructive",
            Self::ResizeStateAbsentFromCapture { .. } => "resize_state_absent_from_capture",
            Self::PanePointerOnlyResizable { .. } => "pane_pointer_only_resizable",
            Self::PaneResizeStatesIncomplete { .. } => "pane_resize_states_incomplete",
            Self::RequiredLabelsIncomplete { .. } => "required_labels_incomplete",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverLayoutMismatch { .. } => "waiver_layout_mismatch",
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
            Self::LayoutMissing { layout }
            | Self::ResizeControlNotPrecise { layout }
            | Self::PersistenceBrittlePixelOnly { layout }
            | Self::RestoreLostOrDestructive { layout }
            | Self::ResizeStateAbsentFromCapture { layout }
            | Self::PanePointerOnlyResizable { layout }
            | Self::PaneResizeStatesIncomplete { layout }
            | Self::RequiredLabelsIncomplete { layout }
            | Self::NarrowedRowWithoutReason { layout }
            | Self::NarrowedRowWithoutWaiver { layout }
            | Self::WaiverLayoutMismatch { layout, .. }
            | Self::WaiverExpired { layout, .. }
            | Self::RowStatusStale { layout }
            | Self::RowCausesStale { layout } => layout,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release certification packet shared by the shell / layout engine / release
/// automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneControlCertificationPacket {
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
    /// The certification dimensions every layout is certified across.
    pub required_proof_dimensions: Vec<M5PaneControlProofDimension>,
    /// The pane-resize states every layout must certify.
    pub required_pane_resize_states: Vec<M5PaneResizeState>,
    /// The required labels every layout must certify.
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// Per-layout certification rows, in canonical order.
    pub rows: Vec<PaneControlCertificationRow>,
    /// Governed layouts certified, in canonical (sorted) order.
    pub covered_layouts: Vec<String>,
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
    pub active_waivers: Vec<PaneControlCertificationWaiver>,
    /// Every exact certification cause, in row then cause order.
    pub certification_causes: Vec<PaneControlCertificationCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<PaneControlCertificationFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Shell / release automation refs that consume this packet to auto-narrow claimed
    /// layouts.
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

impl PaneControlCertificationPacket {
    /// Returns the certification row for `layout`, if present.
    pub fn row(&self, layout: M5PaneLayout) -> Option<&PaneControlCertificationRow> {
        self.rows.iter().find(|row| row.layout == layout)
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
                waiver.layout.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.certification_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.layout.as_str(),
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
    pub fn dashboard(&self) -> PaneControlCertificationDashboard {
        PaneControlCertificationDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 pane-control certification packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per layout.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "layout,status,qualification,shell_zone_slot,resize_control_precision,proportion_persistence,reset_restore,resize_export,pane_never_pointer_only_resizable,pane_resize_states,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.layout.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.shell_zone_slot.as_str(),
                row.resize_control_precision.as_str(),
                row.proportion_persistence.as_str(),
                row.reset_restore.as_str(),
                row.resize_export.as_str(),
                row.pane_never_pointer_only_resizable,
                join_tokens(&row.certified_pane_resize_states, |s| s.as_str()),
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
            "# M5 splitter & resizable-pane control precision, persistence, restore & export\n\n",
        );
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_pane_control_certification`](../../crates/aureline-shell/src/m5_pane_control_certification/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_pane_control_certification -- markdown > \\\n  artifacts/shell/m5-pane-control-certification.md\n",
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
            "| Layout | Status | Qualification | Precision | Persistence | Restore | Export | Keyboard-resizable | Waiver |\n\
             | ------ | ------ | ------------- | --------- | ----------- | ------- | ------ | ------------------ | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.layout_label,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.resize_control_precision.as_str(),
                row.proportion_persistence.as_str(),
                row.reset_restore.as_str(),
                row.resize_export.as_str(),
                row.pane_never_pointer_only_resizable,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&PaneControlCertificationRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, PaneControlCertificationStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str("None — every governed pane layout is certified at full standing.\n\n");
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.layout.as_str(),
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
                    cause.layout.as_str(),
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
                    waiver.layout.as_str(),
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_pane_control_certification -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_pane_control_certification_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneControlCertificationDashboardRow {
    /// The governed layout.
    pub layout: M5PaneLayout,
    /// Short layout-surface label.
    pub layout_label: String,
    /// Derived green/yellow/red status.
    pub status: PaneControlCertificationStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Resize-control-precision posture.
    pub resize_control_precision: ResizeControlPrecisionState,
    /// Proportion-persistence posture.
    pub proportion_persistence: ProportionPersistenceState,
    /// Reset-restore posture.
    pub reset_restore: ResetRestoreState,
    /// Resize-export posture.
    pub resize_export: ResizeExportState,
    /// `true` when a pane is never resizable by pointer only.
    pub pane_never_pointer_only_resizable: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the shell / layout engine / release automation reads
/// to auto-narrow claimed pane layouts.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneControlCertificationDashboard {
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
    pub rows: Vec<PaneControlCertificationDashboardRow>,
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

impl PaneControlCertificationDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &PaneControlCertificationPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| PaneControlCertificationDashboardRow {
                layout: row.layout,
                layout_label: row.layout_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                resize_control_precision: row.resize_control_precision,
                proportion_persistence: row.proportion_persistence,
                reset_restore: row.reset_restore,
                resize_export: row.resize_export,
                pane_never_pointer_only_resizable: row.pane_never_pointer_only_resizable,
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
            record_kind: M5_PANE_CONTROL_CERTIFICATION_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_PANE_CONTROL_CERTIFICATION_SCHEMA_VERSION,
            dashboard_id: M5_PANE_CONTROL_CERTIFICATION_DASHBOARD_ID.to_owned(),
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
            .expect("m5 pane-control certification dashboard serializes")
    }
}

/// Support-export wrapper for the pane-control certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PaneControlCertificationSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: PaneControlCertificationPacket,
    /// Dashboard quoted in full.
    pub dashboard: PaneControlCertificationDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl PaneControlCertificationSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each layout, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the shell
    /// automation — can name the same layout and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: PaneControlCertificationPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.layout.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_PANE_CONTROL_CERTIFICATION_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_PANE_CONTROL_CERTIFICATION_SCHEMA_VERSION,
            shared_contract_ref: M5_PANE_CONTROL_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_pane_control_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PaneControlCertificationInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-primitives matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-layout certification rows.
    pub rows: Vec<PaneControlCertificationRow>,
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

/// Builds a [`PaneControlCertificationPacket`] from the exact build identity, the frozen
/// matrix ref, and the per-layout certification rows.
///
/// Each row's derived status and certification causes, the aggregate counts, the active
/// waivers, and the blocking findings are recomputed here so the packet is the single
/// source of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_pane_control_certification_packet(
    input: PaneControlCertificationInput,
) -> PaneControlCertificationPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent
    // and the auto-narrowing is the single source of truth.
    let rows: Vec<PaneControlCertificationRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.certification_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<PaneControlCertificationFinding> = Vec::new();

    // Every governed layout must carry a certification row.
    let present: BTreeSet<M5PaneLayout> = rows.iter().map(|row| row.layout).collect();
    for layout in M5PaneLayout::ALL {
        if !present.contains(&layout) {
            blocking_findings.push(PaneControlCertificationFinding::LayoutMissing {
                layout: layout.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_layouts: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|layout| layout.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, PaneControlCertificationStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, PaneControlCertificationStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, PaneControlCertificationStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(PaneControlCertificationFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<PaneControlCertificationWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let certification_causes: Vec<PaneControlCertificationCause> = rows
        .iter()
        .flat_map(|row| row.certification_causes.clone())
        .collect();

    let mut packet = PaneControlCertificationPacket {
        record_kind: M5_PANE_CONTROL_CERTIFICATION_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_PANE_CONTROL_CERTIFICATION_SCHEMA_VERSION,
        shared_contract_ref: M5_PANE_CONTROL_CERTIFICATION_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_PANE_CONTROL_CERTIFICATION_PACKET_ID.to_owned(),
        source_schema_ref: M5_PANE_CONTROL_CERTIFICATION_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Splitter and resizable-pane control precision, proportion-safe persistence, \
                   default-size reset/restore, and support-export truth certified across every \
                   claimed M5 multi-pane layout: notebook, data, review, docs, profiler, and \
                   incident each resize precisely with pointer and keyboard, persist resize intent \
                   as proportions or named presets rather than brittle pixels, reset to a default \
                   and restore losslessly after crash or display/topology change, and reconstruct \
                   current proportions and recent resize actions from a support export — with each \
                   row's green/yellow/red claim auto-narrowed from its precision, persistence, \
                   restore, and export posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_PANE_CONTROL_CERTIFICATION_MATRIX_SCHEMA_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_proof_dimensions: M5PaneControlProofDimension::ALL.to_vec(),
        required_pane_resize_states: M5PaneResizeState::ALL.to_vec(),
        required_labels: PANE_CONTROL_REQUIRED_LABELS.to_vec(),
        rows,
        covered_layouts,
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
            "shell_frame.pane_control.proportion_registry".to_owned(),
            "release_automation.auto_narrow.pane_control_certification_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.pane_control_certification".to_owned(),
            "artifacts/release/m5-pane-control-certification-proof/packet.json".to_owned(),
        ],
        help_docs_refs: vec![M5_PANE_CONTROL_CERTIFICATION_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-pane-control-certification".to_owned()],
        published_report_ref: M5_PANE_CONTROL_CERTIFICATION_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_PANE_CONTROL_CERTIFICATION_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_PANE_CONTROL_CERTIFICATION_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_PANE_CONTROL_CERTIFICATION_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(PaneControlCertificationFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_pane_control_certification_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum PaneControlCertificationValidationError {
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
    /// The rows do not cover all six governed layouts.
    CoverageIncomplete,
    /// The declared covered layouts do not match the rows.
    CoverageStale,
    /// The declared required proof dimensions are not the canonical set.
    RequiredDimensionsStale,
    /// The declared required pane-resize states are not the canonical set.
    RequiredPaneResizeStatesStale,
    /// The declared required labels are not the pane-control set.
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

/// Validates a packet against the pane-control certification invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed pane
/// layout carries a current certification row; each row's status is the derived
/// auto-narrowed value, never asserted; a green row cannot keep a claim while a pane is
/// pointer-only or brittle, persists only as pixels, loses or destroys restore, drops its
/// resize state from capture, is resizable by pointer only, or leaves its pane-resize
/// states / required labels incomplete; and a disclosed narrowing is backed by a reason
/// and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_pane_control_certification_packet(
    packet: &PaneControlCertificationPacket,
) -> Result<(), Vec<PaneControlCertificationValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(PaneControlCertificationValidationError::NoRows);
    }
    if packet.record_kind != M5_PANE_CONTROL_CERTIFICATION_PACKET_RECORD_KIND {
        errors.push(PaneControlCertificationValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_PANE_CONTROL_CERTIFICATION_SCHEMA_VERSION {
        errors.push(PaneControlCertificationValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(PaneControlCertificationValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(PaneControlCertificationValidationError::MatrixPacketRefMissing);
    }
    if packet.required_proof_dimensions != M5PaneControlProofDimension::ALL {
        errors.push(PaneControlCertificationValidationError::RequiredDimensionsStale);
    }
    if packet.required_pane_resize_states != M5PaneResizeState::ALL {
        errors.push(PaneControlCertificationValidationError::RequiredPaneResizeStatesStale);
    }
    if packet.required_labels != PANE_CONTROL_REQUIRED_LABELS {
        errors.push(PaneControlCertificationValidationError::RequiredLabelsStale);
    }

    let present: BTreeSet<M5PaneLayout> = packet.rows.iter().map(|row| row.layout).collect();
    let coverage_complete = M5PaneLayout::ALL
        .iter()
        .all(|layout| present.contains(layout));
    if !coverage_complete || packet.rows.len() != M5PaneLayout::ALL.len() {
        errors.push(PaneControlCertificationValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|layout| layout.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_layouts {
        errors.push(PaneControlCertificationValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| {
            matches!(
                row.recompute_status(),
                PaneControlCertificationStatus::Green
            )
        })
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| {
            matches!(
                row.recompute_status(),
                PaneControlCertificationStatus::Yellow
            )
        })
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), PaneControlCertificationStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(PaneControlCertificationValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<PaneControlCertificationWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(PaneControlCertificationValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<PaneControlCertificationCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.certification_causes {
        errors.push(PaneControlCertificationValidationError::CertificationCausesStale);
    }

    let mut recomputed: Vec<PaneControlCertificationFinding> = Vec::new();
    for layout in M5PaneLayout::ALL {
        if !present.contains(&layout) {
            recomputed.push(PaneControlCertificationFinding::LayoutMissing {
                layout: layout.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(PaneControlCertificationFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(PaneControlCertificationFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(PaneControlCertificationValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(
            PaneControlCertificationValidationError::BlockingFindingPresent {
                class: finding.class_token().to_owned(),
                subject_ref: finding.subject_ref().to_owned(),
            },
        );
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(PaneControlCertificationValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(PaneControlCertificationValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(PaneControlCertificationValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(PaneControlCertificationValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
