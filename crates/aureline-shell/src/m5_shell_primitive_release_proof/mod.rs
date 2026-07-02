//! Status-bar, transient-inspect, pane-control, and durable-progress-component truth certified
//! and published as one release-evidence proof for every claimed M5 shell primitive.
//!
//! The [frozen shell-primitives matrix][matrix] already freezes the ten governed shell
//! primitives — the status-bar item and overflow menu, the tooltip/hovercard/peek/pinned-preview
//! transient-inspect surfaces, the splitter handle and pane-resize preset, and the ambient
//! progress indicator and durable job row — into one export-safe packet: their status-item
//! classes, overflow behaviors, representation classes, promotion states, pane-resize states,
//! progress states, source/provider/freshness labels, non-visual accessibility routes, the
//! mandatory labels every primitive must be able to show, and the downgrade triggers that narrow
//! them below a claim. This lane is the **release-proof publication capstone** on top of that
//! matrix: it emits one certification row per claimed shell primitive so that every claimed M5
//! shell-facing surface has a current proof for its status/peek/splitter/progress truth or is
//! automatically narrowed, ties each row to the release evidence index, and lets a shell-primitives
//! regression be detected mechanically before a beta/stable claim widens.
//!
//! Each row groups its primitive under one of four **truth pillars** — ambient instrumentation,
//! transient inspect, pane control, or durable progress — and certifies four release-truth axes:
//!
//! - **primitive truth** — the primitive's typed state truth is certified and current, never
//!   collapsed into a generic spinner or anonymous chrome.
//! - **representation / freshness** — the source, provider, freshness, and representation truth of
//!   what the primitive shows is preserved after it compacts, pins, or promotes; a stale/cached
//!   preview never reads as live canonical content.
//! - **interaction reach** — the primitive stays keyboard- and touch-reachable and resizes
//!   precisely and serializably; no critical truth or resize affordance is pointer- or hover-only.
//! - **exported proof parity** — the exported release, docs, and support surfaces reflect the
//!   current proof; a stale or divergent export auto-narrows the primitive.
//!
//! Three records carry the truth:
//!
//! - the per-primitive **certification row** ([`ShellPrimitiveReleaseRow`]): one row per
//!   [`M5ShellPrimitiveFamily`] naming its truth pillar, the status-item classes / overflow
//!   behaviors / representation classes / promotion states / pane-resize states / progress states
//!   / source-freshness labels / accessibility routes / required labels / shell zone / consumer
//!   surfaces / downgrade triggers pulled from the frozen matrix, the rendering profiles it is
//!   certified across, its four release-truth postures, any active waiver, and a derived
//!   green/yellow/red [`ShellPrimitiveReleaseStatus`].
//! - the release **certification packet** ([`ShellPrimitiveReleasePacket`]): the full set of rows
//!   with derived per-row status, aggregate green/yellow/red counts, the active waivers, the exact
//!   certification causes ([`ShellPrimitiveReleaseCause`]), and the blocking findings the lane
//!   refuses to ship with.
//! - the **certification dashboard** ([`ShellPrimitiveReleaseDashboard`]): a light projection the
//!   shell / release automation / evidence index reads to auto-narrow a claimed primitive when its
//!   release proof falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment
//! it discloses a reduced truth scope, a partial representation, a reduced (waivered) interaction
//! reach, or a partial export refresh; it drops to `red` if a primitive's state collapses into a
//! spinner, its source/freshness is hidden or a stale preview reads as live, its truth or resize
//! is pointer-/hover-only, its exported proof is stale or divergent, it keeps any critical truth
//! hover-/spinner-/pointer-only, or its accessibility routes / required labels / profile coverage
//! are incomplete. That derivation is the auto-narrowing the acceptance criteria require.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local
//! paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed
//! vocabulary, counts, refs, and short labels. The status-item-class, overflow-behavior,
//! representation-class, promotion-state, pane-resize-state, progress-state, source-freshness,
//! accessibility-route, required-label, shell-zone, consumer-surface, downgrade-trigger, and
//! qualification vocabulary is re-exported by reference from the already frozen [matrix]; each row
//! pulls its primitive bindings straight from the matrix's seeded primitive row for that family, so
//! this lane mints no parallel shell vocabulary and cannot certify a primitive the matrix does not
//! freeze. Only the certification-specific vocabulary ([`M5ShellPrimitiveTruthPillar`],
//! [`M5ShellReleaseProfile`], [`M5ShellPrimitiveReleaseProofDimension`],
//! [`ShellPrimitiveReleaseStatus`], [`PrimitiveTruthState`], [`RepresentationFreshnessState`],
//! [`InteractionReachState`], [`ExportedProofParityState`], [`ShellPrimitiveReleaseWaiver`],
//! [`ShellPrimitiveReleaseCause`], [`ShellPrimitiveReleaseFinding`]) is new.
//!
//! [matrix]: crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix as matrix;

pub use matrix::{
    M5AccessibilityRoute, M5OverflowBehavior, M5PaneResizeState, M5PrimitiveQualificationClass,
    M5PrimitiveRequiredLabel, M5ProgressState, M5PromotionState, M5RepresentationClass,
    M5ResponsiveClass, M5ShellConsumerSurface, M5ShellPrimitiveDowngradeTrigger,
    M5ShellPrimitiveFamily, M5ShellSurfaceFamily, M5ShellZoneSlot, M5SourceFreshnessLabel,
    M5StatusItemClass, M5WindowClass,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_shell_primitive_release_proof_packet,
    seeded_m5_shell_primitive_release_proof_packet_hovercard_source_freshness_hidden_blocked,
    seeded_m5_shell_primitive_release_proof_packet_job_row_exported_proof_stale_blocked,
    seeded_m5_shell_primitive_release_proof_packet_progress_hover_spinner_only_blocked,
    seeded_m5_shell_primitive_release_proof_packet_splitter_pointer_only_resize_blocked,
    seeded_m5_shell_primitive_release_proof_packet_status_bar_truth_collapsed_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_SHARED_CONTRACT_REF: &str =
    "shell:m5_shell_primitive_release_proof:v1";

/// Stable record kind for [`ShellPrimitiveReleasePacket`] payloads.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_PACKET_RECORD_KIND: &str =
    "shell_m5_shell_primitive_release_proof_packet_record";

/// Stable record kind for [`ShellPrimitiveReleaseDashboard`] payloads.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_shell_primitive_release_proof_dashboard_record";

/// Stable record kind for [`ShellPrimitiveReleaseSupportExport`] payloads.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_shell_primitive_release_proof_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_PACKET_ID: &str =
    "m5-shell-primitive-release-proof:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_DASHBOARD_ID: &str =
    "m5-shell-primitive-release-proof-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-shell-primitive-release-proof:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-shell-primitive-release-proof.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-shell-primitive-release-proof.md";

/// Published certification-packet artifact ref.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-shell-primitive-release-proof-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-shell-primitive-release-proof-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-shell-primitive-release-proof-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-shell-primitive-release-proof-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_shell_primitive_release_proof_contract.md";

/// Repo-relative ref to the frozen shell-primitives matrix schema.
pub const M5_SHELL_PRIMITIVE_RELEASE_PROOF_MATRIX_SCHEMA_REF: &str =
    matrix::M5_SHELL_PRIMITIVES_SCHEMA_REF;

/// The six non-visual accessibility routes every certified primitive must offer — the full
/// [`M5AccessibilityRoute::ALL`] set: keyboard-focusable, screen-reader-announced,
/// non-hover-reachable, pointer-optional, high-contrast-safe, and support-exportable.
pub const RELEASE_PROOF_REQUIRED_ROUTES: [M5AccessibilityRoute; 6] = M5AccessibilityRoute::ALL;

/// The three labels every certified primitive must be able to show — the mandatory
/// [`M5PrimitiveRequiredLabel::MANDATORY`] set: identity, state, and keyboard route. A single
/// primitive family only guarantees the mandatory three (source/provider, freshness, and reopen
/// path are declared by the families that carry them).
pub const RELEASE_PROOF_REQUIRED_LABELS: [M5PrimitiveRequiredLabel; 3] =
    M5PrimitiveRequiredLabel::MANDATORY;

/// One of the four truth pillars a governed shell primitive belongs to. Each pillar is one of the
/// four shell-primitive concerns the batch certifies together — status-bar / ambient
/// instrumentation, tooltip/hovercard/peek transient inspect, splitter/pane-resize pane control,
/// and progress/job-row durable progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellPrimitiveTruthPillar {
    /// Status-bar item and overflow menu ambient instrumentation.
    AmbientInstrumentation,
    /// Tooltip / hovercard / peek / pinned-preview transient inspect.
    TransientInspect,
    /// Splitter handle and pane-resize preset pane control.
    PaneControl,
    /// Progress indicator and durable job-row durable progress.
    DurableProgress,
}

impl M5ShellPrimitiveTruthPillar {
    /// Every truth pillar, in canonical order.
    pub const ALL: [Self; 4] = [
        Self::AmbientInstrumentation,
        Self::TransientInspect,
        Self::PaneControl,
        Self::DurableProgress,
    ];

    /// The truth pillar a governed primitive family belongs to.
    pub const fn from_family(family: M5ShellPrimitiveFamily) -> Self {
        if family.is_ambient() {
            Self::AmbientInstrumentation
        } else if family.is_transient_inspect() {
            Self::TransientInspect
        } else if family.is_pane_control() {
            Self::PaneControl
        } else {
            Self::DurableProgress
        }
    }

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AmbientInstrumentation => "ambient_instrumentation",
            Self::TransientInspect => "transient_inspect",
            Self::PaneControl => "pane_control",
            Self::DurableProgress => "durable_progress",
        }
    }

    /// Short, reviewer-facing label for the pillar.
    pub const fn label(self) -> &'static str {
        match self {
            Self::AmbientInstrumentation => "Ambient instrumentation",
            Self::TransientInspect => "Transient inspect",
            Self::PaneControl => "Pane control",
            Self::DurableProgress => "Durable progress",
        }
    }
}

/// One of the claimed M5 rendering profiles the release proof must cover, in canonical order. Each
/// profile is a claimed M5 shell rendering condition under which the status, hover/peek, splitter,
/// and progress primitives must keep their certified truth; the lane certifies none beyond them and
/// refuses to ship if any is missing. The compact, high-zoom, high-contrast, and reduced-motion
/// profiles are exactly the profile-coverage cases the implementation requirements name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellReleaseProfile {
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
    /// High-contrast rendering.
    HighContrast,
    /// Reduced-motion rendering.
    ReducedMotion,
}

impl M5ShellReleaseProfile {
    /// Every governed rendering profile, in canonical order.
    pub const ALL: [Self; 7] = [
        Self::Standard,
        Self::Compact,
        Self::Expanded,
        Self::MultiWindow,
        Self::HighZoom,
        Self::HighContrast,
        Self::ReducedMotion,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Standard => "standard",
            Self::Compact => "compact",
            Self::Expanded => "expanded",
            Self::MultiWindow => "multi_window",
            Self::HighZoom => "high_zoom",
            Self::HighContrast => "high_contrast",
            Self::ReducedMotion => "reduced_motion",
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
            Self::HighContrast => "High-contrast rendering",
            Self::ReducedMotion => "Reduced-motion rendering",
        }
    }
}

/// One of the four certification dimensions each primitive is certified across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellPrimitiveReleaseProofDimension {
    /// Primitive truth (the typed state truth is certified and current).
    PrimitiveTruth,
    /// Representation / freshness (source, provider, freshness, and representation preserved).
    RepresentationFreshness,
    /// Interaction reach (keyboard/touch reach and precise serializable resize).
    InteractionReach,
    /// Exported proof parity (exported surfaces reflect the current proof).
    ExportedProofParity,
}

impl M5ShellPrimitiveReleaseProofDimension {
    /// Every certification dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::PrimitiveTruth,
        Self::RepresentationFreshness,
        Self::InteractionReach,
        Self::ExportedProofParity,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimitiveTruth => "primitive_truth",
            Self::RepresentationFreshness => "representation_freshness",
            Self::InteractionReach => "interaction_reach",
            Self::ExportedProofParity => "exported_proof_parity",
        }
    }
}

/// The derived certification light a governed shell primitive carries.
///
/// `green` means the primitive's typed state truth is certified and current, its
/// source/representation/freshness truth is preserved, it stays keyboard- and touch-reachable and
/// resizes precisely and serializably, and its exported proof reflects the current state. `yellow`
/// is a disclosed narrowing (a reduced truth scope, a partial representation, a waivered reduced
/// interaction reach, or a partial export refresh). `red` is blocked: the primitive's state
/// collapses into a spinner, its source/freshness is hidden or a stale preview reads as live, its
/// truth or resize is pointer-/hover-only, its exported proof is stale or divergent, it keeps a
/// critical truth hover-/spinner-/pointer-only, or its routes / labels / profile coverage are
/// incomplete — and the primitive may not keep a shell-maturity claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellPrimitiveReleaseStatus {
    /// Full standing: truth current, representation preserved, reachable, export parity kept.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl ShellPrimitiveReleaseStatus {
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

/// How the primitive's typed state truth is certified and current across claimed surfaces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PrimitiveTruthState {
    /// The primitive's typed state truth (ambient status, inspect representation, pane-control
    /// posture, or progress/job-row state) is certified and current, never collapsed into a
    /// generic spinner or anonymous chrome.
    PrimitiveTruthCertifiedAndCurrent,
    /// A disclosed reduction: a low-priority slice of the typed state truth is presented at a
    /// coarser scope (a grouped summary in place of per-item detail) while the primary state stays
    /// current and named; the reduction is disclosed.
    DisclosedReducedTruthScope,
    /// The primitive's typed state truth collapsed into a generic spinner or anonymous chrome, or
    /// was lost when the surface compacted — always a blocker.
    PrimitiveTruthCollapsedOrLost,
}

impl PrimitiveTruthState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PrimitiveTruthCertifiedAndCurrent => "primitive_truth_certified_and_current",
            Self::DisclosedReducedTruthScope => "disclosed_reduced_truth_scope",
            Self::PrimitiveTruthCollapsedOrLost => "primitive_truth_collapsed_or_lost",
        }
    }

    /// `true` when the primitive's state truth is certified and current.
    pub const fn is_certified(self) -> bool {
        matches!(self, Self::PrimitiveTruthCertifiedAndCurrent)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedTruthScope)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::PrimitiveTruthCollapsedOrLost)
    }
}

/// How the source, provider, freshness, and representation truth of what the primitive shows is
/// preserved after it compacts, pins, or promotes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RepresentationFreshnessState {
    /// The source/provider/freshness and representation truth is preserved across compact, pin, and
    /// promote; a cached or stale value is labelled so it never reads as live canonical content, and
    /// a pure layout control keeps its identity and state representation.
    SourceFreshnessRepresentationPreserved,
    /// A disclosed reduction: a low-priority representation detail is trimmed (a provenance strip
    /// abbreviates) while the source, freshness, and representation truth stay preserved; the
    /// reduction is disclosed.
    DisclosedPartialRepresentation,
    /// The source/provider/freshness truth is hidden after compact/pin/promote, or a stale/cached
    /// preview reads as live canonical content — always a blocker.
    SourceOrFreshnessHiddenOrStale,
}

impl RepresentationFreshnessState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceFreshnessRepresentationPreserved => {
                "source_freshness_representation_preserved"
            }
            Self::DisclosedPartialRepresentation => "disclosed_partial_representation",
            Self::SourceOrFreshnessHiddenOrStale => "source_or_freshness_hidden_or_stale",
        }
    }

    /// `true` when the representation/freshness truth is preserved.
    pub const fn is_preserved(self) -> bool {
        matches!(self, Self::SourceFreshnessRepresentationPreserved)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedPartialRepresentation)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::SourceOrFreshnessHiddenOrStale)
    }
}

/// How the primitive stays keyboard- and touch-reachable and resizes precisely and serializably.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum InteractionReachState {
    /// The primitive is keyboard-focusable and touch-reachable, and any pane control resizes
    /// precisely, keyboard-addressably, and serializably; no critical truth or resize affordance is
    /// pointer- or hover-only.
    KeyboardTouchReachAndResizeCertified,
    /// A disclosed reduction: a coarser touch target or a reduced keyboard resize step while a
    /// keyboard/touch path stays present and precise; the reduction is disclosed and waivered.
    DisclosedReducedReachOrResize,
    /// A primitive's critical truth or its resize affordance is reachable only by pointer or hover,
    /// or a resize is brittle / not serializable — always a blocker.
    PointerOrHoverOnlyOrBrittleResize,
}

impl InteractionReachState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardTouchReachAndResizeCertified => {
                "keyboard_touch_reach_and_resize_certified"
            }
            Self::DisclosedReducedReachOrResize => "disclosed_reduced_reach_or_resize",
            Self::PointerOrHoverOnlyOrBrittleResize => "pointer_or_hover_only_or_brittle_resize",
        }
    }

    /// `true` when keyboard/touch reach and precise serializable resize are certified.
    pub const fn is_certified(self) -> bool {
        matches!(self, Self::KeyboardTouchReachAndResizeCertified)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedReachOrResize)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::PointerOrHoverOnlyOrBrittleResize)
    }
}

/// How the exported release, docs, and support surfaces reflect the primitive's current proof.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportedProofParityState {
    /// The exported release, docs, and support surfaces reflect the primitive's current proof, so a
    /// regression is detectable mechanically without a live screenshot.
    ExportedSurfacesReflectCurrentProof,
    /// A disclosed reduction: the export reflects the current proof and discloses a partial refresh
    /// (some low-priority primitive detail is trimmed) while the reduction is disclosed.
    DisclosedPartialExportRefresh,
    /// The exported proof is stale or divergent from the current primitive state, so a regression
    /// cannot be explained without a live screenshot — always a blocker.
    ExportedProofStaleOrDivergent,
}

impl ExportedProofParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExportedSurfacesReflectCurrentProof => "exported_surfaces_reflect_current_proof",
            Self::DisclosedPartialExportRefresh => "disclosed_partial_export_refresh",
            Self::ExportedProofStaleOrDivergent => "exported_proof_stale_or_divergent",
        }
    }

    /// `true` when the exported surfaces reflect the current proof.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::ExportedSurfacesReflectCurrentProof)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedPartialExportRefresh)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::ExportedProofStaleOrDivergent)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red reduced interaction reach stay
/// yellow rather than blocked — never lets a collapsed state, a hidden source/freshness, a
/// pointer/hover-only truth, or a stale export hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellPrimitiveReleaseWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed primitive family the waiver applies to.
    pub primitive_family: M5ShellPrimitiveFamily,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl ShellPrimitiveReleaseWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed primitive's certification.
///
/// The trigger token mirrors the frozen [`M5ShellPrimitiveDowngradeTrigger`] vocabulary so a cause
/// never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellPrimitiveReleaseCause {
    /// The governed primitive family the cause applies to.
    pub primitive_family: M5ShellPrimitiveFamily,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5ShellPrimitiveDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause is
    /// a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl ShellPrimitiveReleaseCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed shell primitive, certified across primitive truth, representation/freshness,
/// interaction reach, and exported proof parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellPrimitiveReleaseRow {
    /// The governed primitive family being certified.
    pub primitive_family: M5ShellPrimitiveFamily,
    /// The truth pillar this primitive belongs to (derived from the family).
    pub truth_pillar: M5ShellPrimitiveTruthPillar,
    /// The frozen qualification class for this primitive. Pulled from the matrix.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Owner role accountable for keeping this primitive certified. Pulled from the matrix.
    pub owner_role: String,
    /// Short primitive label.
    pub primitive_label: String,
    /// Scope summary. Pulled from the matrix.
    pub scope_summary: String,
    /// Canonical shell zone this primitive attaches to. Pulled from the matrix.
    pub shell_zone_slot: M5ShellZoneSlot,
    /// Responsive classes this primitive survives. Pulled from the matrix.
    pub responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes this primitive keeps continuity across. Pulled from the matrix.
    pub window_classes: Vec<M5WindowClass>,
    /// Claimed M5 surface families that render / consume this primitive. Pulled from the matrix.
    pub surface_families: Vec<M5ShellSurfaceFamily>,
    /// Rendering profiles this primitive is certified across.
    pub certified_profiles: Vec<M5ShellReleaseProfile>,
    /// Status-item classes this primitive projects. Pulled from the matrix.
    pub certified_status_item_classes: Vec<M5StatusItemClass>,
    /// Overflow behaviors this primitive honours. Pulled from the matrix.
    pub certified_overflow_behaviors: Vec<M5OverflowBehavior>,
    /// Representation classes this primitive shows. Pulled from the matrix.
    pub certified_representation_classes: Vec<M5RepresentationClass>,
    /// Promotion states this primitive honours. Pulled from the matrix.
    pub certified_promotion_states: Vec<M5PromotionState>,
    /// Pane-resize states this primitive honours. Pulled from the matrix.
    pub certified_pane_resize_states: Vec<M5PaneResizeState>,
    /// Progress states this primitive honours. Pulled from the matrix.
    pub certified_progress_states: Vec<M5ProgressState>,
    /// Source/provider/freshness labels this primitive can show. Pulled from the matrix.
    pub certified_source_freshness_labels: Vec<M5SourceFreshnessLabel>,
    /// Non-visual accessibility routes. Pulled from the matrix.
    pub accessibility_routes: Vec<M5AccessibilityRoute>,
    /// Mandatory labels this primitive must be able to show. Pulled from the matrix.
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// Shell subsystems that consume this primitive. Pulled from the matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this primitive. Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ShellPrimitiveDowngradeTrigger>,
    /// Primitive-truth posture.
    pub primitive_truth: PrimitiveTruthState,
    /// Representation/freshness posture.
    pub representation_freshness: RepresentationFreshnessState,
    /// Interaction-reach posture.
    pub interaction_reach: InteractionReachState,
    /// Exported-proof-parity posture.
    pub exported_proof_parity: ExportedProofParityState,
    /// Hard invariant: no critical truth is kept hover-, spinner-, or pointer-only. `false` is a
    /// blocker.
    pub never_hover_spinner_or_pointer_only: bool,
    /// Active waiver, when a disclosed reduced interaction reach is in force.
    pub active_waiver: Option<ShellPrimitiveReleaseWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: ShellPrimitiveReleaseStatus,
    /// The exact certification causes that narrowed or blocked this row.
    pub certification_causes: Vec<ShellPrimitiveReleaseCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl ShellPrimitiveReleaseRow {
    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when every non-visual accessibility route the matrix freezes is certified — the lint
    /// that prevents a primitive from shipping without keyboard-focusable, screen-reader-announced,
    /// non-hover-reachable, pointer-optional, high-contrast-safe, and support-exportable routes.
    pub fn accessibility_routes_complete(&self) -> bool {
        let present: BTreeSet<M5AccessibilityRoute> =
            self.accessibility_routes.iter().copied().collect();
        RELEASE_PROOF_REQUIRED_ROUTES
            .iter()
            .all(|route| present.contains(route))
    }

    /// `true` when every mandatory label is certified — the lint that prevents a primitive from
    /// shipping without identity, state, and keyboard-route labels.
    pub fn required_labels_complete(&self) -> bool {
        let present: BTreeSet<M5PrimitiveRequiredLabel> =
            self.required_labels.iter().copied().collect();
        RELEASE_PROOF_REQUIRED_LABELS
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when every governed rendering profile is certified — the lint that prevents a
    /// primitive from shipping without compact/high-zoom/high-contrast/reduced-motion (and the
    /// standard/expanded/multi-window) profile coverage.
    pub fn profiles_complete(&self) -> bool {
        let present: BTreeSet<M5ShellReleaseProfile> =
            self.certified_profiles.iter().copied().collect();
        M5ShellReleaseProfile::ALL
            .iter()
            .all(|profile| present.contains(profile))
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        self.primitive_truth.is_blocked()
            || self.representation_freshness.is_blocked()
            || self.interaction_reach.is_blocked()
            || self.exported_proof_parity.is_blocked()
            || !self.never_hover_spinner_or_pointer_only
            || !self.accessibility_routes_complete()
            || !self.required_labels_complete()
            || !self.profiles_complete()
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.primitive_truth.is_disclosed()
            || self.representation_freshness.is_disclosed()
            || self.interaction_reach.is_disclosed()
            || self.exported_proof_parity.is_disclosed()
    }

    /// Recomputes the derived status from the four axes and the hover/spinner/pointer-only
    /// invariant.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> ShellPrimitiveReleaseStatus {
        if self.has_hard_blocker() {
            ShellPrimitiveReleaseStatus::Red
        } else if self.has_narrowing() {
            ShellPrimitiveReleaseStatus::Yellow
        } else {
            ShellPrimitiveReleaseStatus::Green
        }
    }

    /// Recomputes the exact certification causes for the row, in deterministic order (primitive
    /// truth, representation/freshness, interaction reach, exported proof, hover/spinner/pointer-only
    /// invariant).
    pub fn recompute_causes(&self) -> Vec<ShellPrimitiveReleaseCause> {
        let mut causes = Vec::new();
        if !self.primitive_truth.is_certified() {
            causes.push(ShellPrimitiveReleaseCause {
                primitive_family: self.primitive_family,
                trigger: M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState,
                disclosed: self.primitive_truth.is_disclosed(),
                detail: if self.primitive_truth.is_disclosed() {
                    "A low-priority slice of the primitive's typed state truth is presented at a \
                     coarser scope (a grouped summary in place of per-item detail) while the \
                     primary state stays current and named; the reduction is disclosed and the row \
                     is narrowed below green."
                        .to_owned()
                } else {
                    "The primitive's typed state truth collapsed into a generic spinner or \
                     anonymous chrome, or was lost when the surface compacted."
                        .to_owned()
                },
            });
        }
        if !self.representation_freshness.is_preserved() {
            causes.push(ShellPrimitiveReleaseCause {
                primitive_family: self.primitive_family,
                trigger: M5ShellPrimitiveDowngradeTrigger::SourceFreshnessHidden,
                disclosed: self.representation_freshness.is_disclosed(),
                detail: if self.representation_freshness.is_disclosed() {
                    "A low-priority representation detail is trimmed (a provenance strip \
                     abbreviates) while the source, freshness, and representation truth stay \
                     preserved; the reduction is disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "The primitive's source/provider/freshness truth is hidden after \
                     compact/pin/promote, or a stale/cached preview reads as live canonical content."
                        .to_owned()
                },
            });
        }
        if !self.interaction_reach.is_certified() {
            causes.push(ShellPrimitiveReleaseCause {
                primitive_family: self.primitive_family,
                trigger: M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth,
                disclosed: self.interaction_reach.is_disclosed(),
                detail: if self.interaction_reach.is_disclosed() {
                    "A coarser touch target or a reduced keyboard resize step is served while a \
                     keyboard/touch path stays present and precise; the reduction is disclosed \
                     behind a waiver and the row is narrowed below green."
                        .to_owned()
                } else {
                    "A primitive's critical truth or its resize affordance is reachable only by \
                     pointer or hover, or a resize is brittle / not serializable."
                        .to_owned()
                },
            });
        }
        if !self.exported_proof_parity.is_current() {
            causes.push(ShellPrimitiveReleaseCause {
                primitive_family: self.primitive_family,
                trigger: M5ShellPrimitiveDowngradeTrigger::ProofStale,
                disclosed: self.exported_proof_parity.is_disclosed(),
                detail: if self.exported_proof_parity.is_disclosed() {
                    "The export reflects the current proof and discloses a partial refresh (some \
                     low-priority primitive detail is trimmed) while the reduction is disclosed and \
                     the row is narrowed below green."
                        .to_owned()
                } else {
                    "The exported proof is stale or divergent from the current primitive state, so \
                     a regression cannot be explained without a live screenshot."
                        .to_owned()
                },
            });
        }
        if !self.never_hover_spinner_or_pointer_only {
            causes.push(ShellPrimitiveReleaseCause {
                primitive_family: self.primitive_family,
                trigger: M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth,
                disclosed: false,
                detail: "A primitive keeps a critical truth or progress visible only through a \
                         hover reveal, a transient spinner, or a pointer-only affordance, with no \
                         keyboard/focus or touch alternative."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced interaction reach may only stay yellow (rather than red) when a waiver
    /// discloses it.
    pub fn requires_waiver(&self) -> bool {
        self.interaction_reach.is_disclosed()
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<ShellPrimitiveReleaseFinding> {
        let mut findings = Vec::new();
        let family = self.primitive_family.as_str().to_owned();

        if self.primitive_truth.is_blocked() {
            findings.push(ShellPrimitiveReleaseFinding::PrimitiveTruthCollapsed {
                family: family.clone(),
            });
        }
        if self.representation_freshness.is_blocked() {
            findings.push(ShellPrimitiveReleaseFinding::SourceOrFreshnessHidden {
                family: family.clone(),
            });
        }
        if self.interaction_reach.is_blocked() {
            findings.push(
                ShellPrimitiveReleaseFinding::InteractionPointerOrHoverOnly {
                    family: family.clone(),
                },
            );
        }
        if self.exported_proof_parity.is_blocked() {
            findings.push(ShellPrimitiveReleaseFinding::ExportedProofStale {
                family: family.clone(),
            });
        }
        if !self.never_hover_spinner_or_pointer_only {
            findings.push(
                ShellPrimitiveReleaseFinding::CriticalTruthHoverSpinnerOrPointerOnly {
                    family: family.clone(),
                },
            );
        }
        if !self.accessibility_routes_complete() {
            findings.push(
                ShellPrimitiveReleaseFinding::AccessibilityRoutesIncomplete {
                    family: family.clone(),
                },
            );
        }
        if !self.required_labels_complete() {
            findings.push(ShellPrimitiveReleaseFinding::RequiredLabelsIncomplete {
                family: family.clone(),
            });
        }
        if !self.profiles_complete() {
            findings.push(ShellPrimitiveReleaseFinding::ProfilesIncomplete {
                family: family.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, ShellPrimitiveReleaseStatus::Green) && !self.has_reason() {
            findings.push(ShellPrimitiveReleaseFinding::NarrowedRowWithoutReason {
                family: family.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active
        // waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(ShellPrimitiveReleaseFinding::NarrowedRowWithoutWaiver {
                family: family.clone(),
            });
        }
        // An attached waiver must still be active and must point at this family.
        if let Some(waiver) = &self.active_waiver {
            if waiver.primitive_family != self.primitive_family {
                findings.push(ShellPrimitiveReleaseFinding::WaiverFamilyMismatch {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(ShellPrimitiveReleaseFinding::WaiverExpired {
                    family: family.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(ShellPrimitiveReleaseFinding::RowStatusStale {
                family: family.clone(),
            });
        }
        if self.certification_causes != self.recompute_causes() {
            findings.push(ShellPrimitiveReleaseFinding::RowCausesStale { family });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} pillar={} status={} qual={} truth={} representation={} reach={} export={} no_hover_spinner_pointer_only={} waiver={}",
            self.primitive_family.as_str(),
            self.truth_pillar.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.primitive_truth.as_str(),
            self.representation_freshness.as_str(),
            self.interaction_reach.as_str(),
            self.exported_proof_parity.as_str(),
            self.never_hover_spinner_or_pointer_only,
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the release proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum ShellPrimitiveReleaseFinding {
    /// A governed primitive family has no certification row.
    PrimitiveMissing {
        /// The missing family token.
        family: String,
    },
    /// A primitive's typed state truth collapsed into a spinner or was lost.
    PrimitiveTruthCollapsed {
        /// The family token.
        family: String,
    },
    /// A primitive hides its source/provider/freshness or a stale preview reads as live.
    SourceOrFreshnessHidden {
        /// The family token.
        family: String,
    },
    /// A primitive keeps a truth or resize affordance pointer- or hover-only.
    InteractionPointerOrHoverOnly {
        /// The family token.
        family: String,
    },
    /// A primitive's exported proof is stale or divergent from its current state.
    ExportedProofStale {
        /// The family token.
        family: String,
    },
    /// A primitive keeps a critical truth hover-, spinner-, or pointer-only (hard invariant).
    CriticalTruthHoverSpinnerOrPointerOnly {
        /// The family token.
        family: String,
    },
    /// A primitive does not certify every frozen accessibility route.
    AccessibilityRoutesIncomplete {
        /// The family token.
        family: String,
    },
    /// A primitive does not certify every mandatory required label.
    RequiredLabelsIncomplete {
        /// The family token.
        family: String,
    },
    /// A primitive does not certify every governed rendering profile.
    ProfilesIncomplete {
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
    /// The declared certification causes do not match the recomputed causes.
    RowCausesStale {
        /// The family token.
        family: String,
    },
    /// The certified rows do not cover all four truth pillars.
    TruthPillarsIncomplete,
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl ShellPrimitiveReleaseFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::PrimitiveMissing { .. } => "primitive_missing",
            Self::PrimitiveTruthCollapsed { .. } => "primitive_truth_collapsed",
            Self::SourceOrFreshnessHidden { .. } => "source_or_freshness_hidden",
            Self::InteractionPointerOrHoverOnly { .. } => "interaction_pointer_or_hover_only",
            Self::ExportedProofStale { .. } => "exported_proof_stale",
            Self::CriticalTruthHoverSpinnerOrPointerOnly { .. } => {
                "critical_truth_hover_spinner_or_pointer_only"
            }
            Self::AccessibilityRoutesIncomplete { .. } => "accessibility_routes_incomplete",
            Self::RequiredLabelsIncomplete { .. } => "required_labels_incomplete",
            Self::ProfilesIncomplete { .. } => "profiles_incomplete",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverFamilyMismatch { .. } => "waiver_family_mismatch",
            Self::WaiverExpired { .. } => "waiver_expired",
            Self::RowStatusStale { .. } => "row_status_stale",
            Self::RowCausesStale { .. } => "row_causes_stale",
            Self::TruthPillarsIncomplete => "truth_pillars_incomplete",
            Self::StatusCountsStale => "status_counts_stale",
            Self::CoverageStale => "coverage_stale",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }

    /// The owning subject ref the finding points at.
    pub fn subject_ref(&self) -> &str {
        match self {
            Self::PrimitiveMissing { family }
            | Self::PrimitiveTruthCollapsed { family }
            | Self::SourceOrFreshnessHidden { family }
            | Self::InteractionPointerOrHoverOnly { family }
            | Self::ExportedProofStale { family }
            | Self::CriticalTruthHoverSpinnerOrPointerOnly { family }
            | Self::AccessibilityRoutesIncomplete { family }
            | Self::RequiredLabelsIncomplete { family }
            | Self::ProfilesIncomplete { family }
            | Self::NarrowedRowWithoutReason { family }
            | Self::NarrowedRowWithoutWaiver { family }
            | Self::WaiverFamilyMismatch { family, .. }
            | Self::WaiverExpired { family, .. }
            | Self::RowStatusStale { family }
            | Self::RowCausesStale { family } => family,
            Self::TruthPillarsIncomplete => "truth_pillars",
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release certification packet shared by the shell / release automation / evidence index.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellPrimitiveReleasePacket {
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
    /// The certification dimensions every primitive is certified across.
    pub required_proof_dimensions: Vec<M5ShellPrimitiveReleaseProofDimension>,
    /// The accessibility routes every primitive must certify.
    pub required_accessibility_routes: Vec<M5AccessibilityRoute>,
    /// The mandatory labels every primitive must certify.
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// The rendering profiles every primitive must certify.
    pub required_profiles: Vec<M5ShellReleaseProfile>,
    /// The truth pillars the rows must cover.
    pub required_truth_pillars: Vec<M5ShellPrimitiveTruthPillar>,
    /// Per-primitive certification rows, in canonical order.
    pub rows: Vec<ShellPrimitiveReleaseRow>,
    /// Governed families certified, in canonical (sorted) order.
    pub covered_families: Vec<String>,
    /// Truth pillars certified, in canonical (sorted) order.
    pub covered_truth_pillars: Vec<String>,
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
    pub active_waivers: Vec<ShellPrimitiveReleaseWaiver>,
    /// Every exact certification cause, in row then cause order.
    pub certification_causes: Vec<ShellPrimitiveReleaseCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<ShellPrimitiveReleaseFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Shell / release automation refs that consume this packet to auto-narrow claimed primitives.
    pub shell_automation_refs: Vec<String>,
    /// Release / evidence-center refs that route the packet into the release evidence index.
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

impl ShellPrimitiveReleasePacket {
    /// Returns the certification row for `family`, if present.
    pub fn row(&self, family: M5ShellPrimitiveFamily) -> Option<&ShellPrimitiveReleaseRow> {
        self.rows.iter().find(|row| row.primitive_family == family)
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
                waiver.primitive_family.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.certification_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.primitive_family.as_str(),
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
    pub fn dashboard(&self) -> ShellPrimitiveReleaseDashboard {
        ShellPrimitiveReleaseDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self)
            .expect("m5 shell-primitive-release-proof packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per primitive.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "primitive_family,truth_pillar,status,qualification,primitive_truth,representation_freshness,interaction_reach,exported_proof_parity,never_hover_spinner_or_pointer_only,profiles,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{},{}\n",
                row.primitive_family.as_str(),
                row.truth_pillar.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.primitive_truth.as_str(),
                row.representation_freshness.as_str(),
                row.interaction_reach.as_str(),
                row.exported_proof_parity.as_str(),
                row.never_hover_spinner_or_pointer_only,
                join_tokens(&row.certified_profiles, |p| p.as_str()),
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
        out.push_str("# M5 shell-primitive release proof\n\n");
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_shell_primitive_release_proof`](../../crates/aureline-shell/src/m5_shell_primitive_release_proof/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_primitive_release_proof -- markdown > \\\n  artifacts/shell/m5-shell-primitive-release-proof.md\n",
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
            "- Truth pillars covered: {}\n",
            self.covered_truth_pillars.join(", ")
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

        out.push_str("## Rendering profiles\n\n");
        for profile in &self.required_profiles {
            out.push_str(&format!("- `{}` — {}\n", profile.as_str(), profile.label()));
        }
        out.push('\n');

        out.push_str("## Certification rows\n\n");
        out.push_str(
            "| Primitive | Pillar | Status | Qualification | Truth | Representation | Reach | Export | No-hover/spinner/pointer-only | Waiver |\n\
             | --------- | ------ | ------ | ------------- | ----- | -------------- | ----- | ------ | ----------------------------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.primitive_label,
                row.truth_pillar.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.primitive_truth.as_str(),
                row.representation_freshness.as_str(),
                row.interaction_reach.as_str(),
                row.exported_proof_parity.as_str(),
                row.never_hover_spinner_or_pointer_only,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&ShellPrimitiveReleaseRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, ShellPrimitiveReleaseStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every governed shell primitive is certified at full standing.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.primitive_family.as_str(),
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
                    cause.primitive_family.as_str(),
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
                    waiver.primitive_family.as_str(),
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_shell_primitive_release_proof -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_shell_primitive_release_proof_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellPrimitiveReleaseDashboardRow {
    /// The governed primitive family.
    pub primitive_family: M5ShellPrimitiveFamily,
    /// The truth pillar.
    pub truth_pillar: M5ShellPrimitiveTruthPillar,
    /// Short primitive label.
    pub primitive_label: String,
    /// Derived green/yellow/red status.
    pub status: ShellPrimitiveReleaseStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Primitive-truth posture.
    pub primitive_truth: PrimitiveTruthState,
    /// Representation/freshness posture.
    pub representation_freshness: RepresentationFreshnessState,
    /// Interaction-reach posture.
    pub interaction_reach: InteractionReachState,
    /// Exported-proof-parity posture.
    pub exported_proof_parity: ExportedProofParityState,
    /// `true` when no critical truth is hover-, spinner-, or pointer-only.
    pub never_hover_spinner_or_pointer_only: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the shell / release automation / evidence index reads to
/// auto-narrow claimed shell primitives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellPrimitiveReleaseDashboard {
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
    pub rows: Vec<ShellPrimitiveReleaseDashboardRow>,
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

impl ShellPrimitiveReleaseDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &ShellPrimitiveReleasePacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| ShellPrimitiveReleaseDashboardRow {
                primitive_family: row.primitive_family,
                truth_pillar: row.truth_pillar,
                primitive_label: row.primitive_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                primitive_truth: row.primitive_truth,
                representation_freshness: row.representation_freshness,
                interaction_reach: row.interaction_reach,
                exported_proof_parity: row.exported_proof_parity,
                never_hover_spinner_or_pointer_only: row.never_hover_spinner_or_pointer_only,
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
            record_kind: M5_SHELL_PRIMITIVE_RELEASE_PROOF_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_SHELL_PRIMITIVE_RELEASE_PROOF_SCHEMA_VERSION,
            dashboard_id: M5_SHELL_PRIMITIVE_RELEASE_PROOF_DASHBOARD_ID.to_owned(),
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
            .expect("m5 shell-primitive-release-proof dashboard serializes")
    }
}

/// Support-export wrapper for the shell-primitive release-proof certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellPrimitiveReleaseSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: ShellPrimitiveReleasePacket,
    /// Dashboard quoted in full.
    pub dashboard: ShellPrimitiveReleaseDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl ShellPrimitiveReleaseSupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each family, and each active
    /// waiver id is quoted as a case id so a support reviewer — or the shell automation — can name
    /// the same primitive and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: ShellPrimitiveReleasePacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.primitive_family.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_SHELL_PRIMITIVE_RELEASE_PROOF_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_SHELL_PRIMITIVE_RELEASE_PROOF_SCHEMA_VERSION,
            shared_contract_ref: M5_SHELL_PRIMITIVE_RELEASE_PROOF_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_shell_primitive_release_proof_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellPrimitiveReleaseInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-primitives matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-primitive certification rows.
    pub rows: Vec<ShellPrimitiveReleaseRow>,
    /// Deterministic generated-at value.
    pub generated_at: String,
}

/// Joins tokens for a CSV cell with a `|` separator so a single cell never introduces a stray
/// comma.
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

/// Builds a [`ShellPrimitiveReleasePacket`] from the exact build identity, the frozen matrix ref,
/// and the per-primitive certification rows.
///
/// Each row's derived status and certification causes, the aggregate counts, the active waivers,
/// and the blocking findings are recomputed here so the packet is the single source of truth and
/// the auto-narrowing cannot be asserted.
pub fn build_m5_shell_primitive_release_proof_packet(
    input: ShellPrimitiveReleaseInput,
) -> ShellPrimitiveReleasePacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<ShellPrimitiveReleaseRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.certification_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<ShellPrimitiveReleaseFinding> = Vec::new();

    // Every governed primitive family must carry a certification row.
    let present: BTreeSet<M5ShellPrimitiveFamily> =
        rows.iter().map(|row| row.primitive_family).collect();
    for family in M5ShellPrimitiveFamily::ALL {
        if !present.contains(&family) {
            blocking_findings.push(ShellPrimitiveReleaseFinding::PrimitiveMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    // Every truth pillar must be covered by at least one certified row.
    let covered_pillars: BTreeSet<M5ShellPrimitiveTruthPillar> =
        rows.iter().map(|row| row.truth_pillar).collect();
    if !M5ShellPrimitiveTruthPillar::ALL
        .iter()
        .all(|pillar| covered_pillars.contains(pillar))
    {
        blocking_findings.push(ShellPrimitiveReleaseFinding::TruthPillarsIncomplete);
    }

    let covered_families: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|family| family.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let covered_truth_pillars: Vec<String> = {
        let mut covered: Vec<String> = covered_pillars
            .iter()
            .map(|pillar| pillar.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ShellPrimitiveReleaseStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ShellPrimitiveReleaseStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, ShellPrimitiveReleaseStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(ShellPrimitiveReleaseFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<ShellPrimitiveReleaseWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let certification_causes: Vec<ShellPrimitiveReleaseCause> = rows
        .iter()
        .flat_map(|row| row.certification_causes.clone())
        .collect();

    let mut packet = ShellPrimitiveReleasePacket {
        record_kind: M5_SHELL_PRIMITIVE_RELEASE_PROOF_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_SHELL_PRIMITIVE_RELEASE_PROOF_SCHEMA_VERSION,
        shared_contract_ref: M5_SHELL_PRIMITIVE_RELEASE_PROOF_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_SHELL_PRIMITIVE_RELEASE_PROOF_PACKET_ID.to_owned(),
        source_schema_ref: M5_SHELL_PRIMITIVE_RELEASE_PROOF_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Status-bar, transient-inspect, pane-control, and durable-progress-component \
                   truth certified and published as one release-evidence proof for every claimed \
                   M5 shell primitive: each of the ten governed primitives keeps its typed state \
                   truth current, its source/representation/freshness truth preserved after \
                   compact/pin/promote, its keyboard/touch reach and precise serializable resize, \
                   and its exported release/docs/support proof current — across the standard, \
                   compact, expanded, multi-window, high-zoom, high-contrast, and reduced-motion \
                   profiles — with each row's green/yellow/red claim auto-narrowed from its \
                   primitive-truth, representation/freshness, interaction-reach, and \
                   exported-proof-parity posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_SHELL_PRIMITIVE_RELEASE_PROOF_MATRIX_SCHEMA_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_proof_dimensions: M5ShellPrimitiveReleaseProofDimension::ALL.to_vec(),
        required_accessibility_routes: RELEASE_PROOF_REQUIRED_ROUTES.to_vec(),
        required_labels: RELEASE_PROOF_REQUIRED_LABELS.to_vec(),
        required_profiles: M5ShellReleaseProfile::ALL.to_vec(),
        required_truth_pillars: M5ShellPrimitiveTruthPillar::ALL.to_vec(),
        rows,
        covered_families,
        covered_truth_pillars,
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
            "shell_frame.release_automation.shell_primitive_release_registry".to_owned(),
            "release_automation.auto_narrow.shell_primitive_release_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.shell_primitive_release_proof".to_owned(),
            "artifacts/release/m5-shell-primitive-release-proof-proof/packet.json".to_owned(),
        ],
        help_docs_refs: vec![M5_SHELL_PRIMITIVE_RELEASE_PROOF_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-shell-primitive-release-proof".to_owned()],
        published_report_ref: M5_SHELL_PRIMITIVE_RELEASE_PROOF_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_SHELL_PRIMITIVE_RELEASE_PROOF_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_SHELL_PRIMITIVE_RELEASE_PROOF_PUBLISHED_DASHBOARD_REF
            .to_owned(),
        published_doc_ref: M5_SHELL_PRIMITIVE_RELEASE_PROOF_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(ShellPrimitiveReleaseFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_shell_primitive_release_proof_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum ShellPrimitiveReleaseValidationError {
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
    /// The rows do not cover all ten governed primitive families.
    CoverageIncomplete,
    /// The declared covered families do not match the rows.
    CoverageStale,
    /// The rows do not cover all four truth pillars.
    TruthPillarsIncomplete,
    /// The declared covered truth pillars do not match the rows.
    CoveredTruthPillarsStale,
    /// The declared required proof dimensions are not the canonical set.
    RequiredDimensionsStale,
    /// The declared required accessibility routes are not the canonical set.
    RequiredAccessibilityRoutesStale,
    /// The declared required labels are not the canonical set.
    RequiredLabelsStale,
    /// The declared required profiles are not the canonical set.
    RequiredProfilesStale,
    /// The declared required truth pillars are not the canonical set.
    RequiredTruthPillarsStale,
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

/// Validates a packet against the shell-primitive release-proof invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed shell primitive
/// carries a current certification row; every truth pillar is covered; each row's status is the
/// derived auto-narrowed value, never asserted; a green row cannot keep a claim while its state
/// collapses into a spinner, its source/freshness is hidden or a stale preview reads as live, its
/// truth or resize is pointer-/hover-only, its exported proof is stale or divergent, it keeps
/// critical truth hover-/spinner-/pointer-only, or its accessibility routes / required labels /
/// profile coverage are incomplete; and a disclosed narrowing is backed by a reason and, where
/// required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_shell_primitive_release_proof_packet(
    packet: &ShellPrimitiveReleasePacket,
) -> Result<(), Vec<ShellPrimitiveReleaseValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(ShellPrimitiveReleaseValidationError::NoRows);
    }
    if packet.record_kind != M5_SHELL_PRIMITIVE_RELEASE_PROOF_PACKET_RECORD_KIND {
        errors.push(ShellPrimitiveReleaseValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_SHELL_PRIMITIVE_RELEASE_PROOF_SCHEMA_VERSION {
        errors.push(ShellPrimitiveReleaseValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(ShellPrimitiveReleaseValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(ShellPrimitiveReleaseValidationError::MatrixPacketRefMissing);
    }
    if packet.required_proof_dimensions != M5ShellPrimitiveReleaseProofDimension::ALL {
        errors.push(ShellPrimitiveReleaseValidationError::RequiredDimensionsStale);
    }
    if packet.required_accessibility_routes != RELEASE_PROOF_REQUIRED_ROUTES {
        errors.push(ShellPrimitiveReleaseValidationError::RequiredAccessibilityRoutesStale);
    }
    if packet.required_labels != RELEASE_PROOF_REQUIRED_LABELS {
        errors.push(ShellPrimitiveReleaseValidationError::RequiredLabelsStale);
    }
    if packet.required_profiles != M5ShellReleaseProfile::ALL {
        errors.push(ShellPrimitiveReleaseValidationError::RequiredProfilesStale);
    }
    if packet.required_truth_pillars != M5ShellPrimitiveTruthPillar::ALL {
        errors.push(ShellPrimitiveReleaseValidationError::RequiredTruthPillarsStale);
    }

    let present: BTreeSet<M5ShellPrimitiveFamily> =
        packet.rows.iter().map(|row| row.primitive_family).collect();
    let coverage_complete = M5ShellPrimitiveFamily::ALL
        .iter()
        .all(|family| present.contains(family));
    if !coverage_complete || packet.rows.len() != M5ShellPrimitiveFamily::ALL.len() {
        errors.push(ShellPrimitiveReleaseValidationError::CoverageIncomplete);
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
        errors.push(ShellPrimitiveReleaseValidationError::CoverageStale);
    }

    let covered_pillars: BTreeSet<M5ShellPrimitiveTruthPillar> =
        packet.rows.iter().map(|row| row.truth_pillar).collect();
    if !M5ShellPrimitiveTruthPillar::ALL
        .iter()
        .all(|pillar| covered_pillars.contains(pillar))
    {
        errors.push(ShellPrimitiveReleaseValidationError::TruthPillarsIncomplete);
    }
    let covered_pillar_tokens: Vec<String> = {
        let mut covered: Vec<String> = covered_pillars
            .iter()
            .map(|pillar| pillar.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered_pillar_tokens != packet.covered_truth_pillars {
        errors.push(ShellPrimitiveReleaseValidationError::CoveredTruthPillarsStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ShellPrimitiveReleaseStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ShellPrimitiveReleaseStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), ShellPrimitiveReleaseStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(ShellPrimitiveReleaseValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<ShellPrimitiveReleaseWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(ShellPrimitiveReleaseValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<ShellPrimitiveReleaseCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.certification_causes {
        errors.push(ShellPrimitiveReleaseValidationError::CertificationCausesStale);
    }

    let mut recomputed: Vec<ShellPrimitiveReleaseFinding> = Vec::new();
    for family in M5ShellPrimitiveFamily::ALL {
        if !present.contains(&family) {
            recomputed.push(ShellPrimitiveReleaseFinding::PrimitiveMissing {
                family: family.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if !M5ShellPrimitiveTruthPillar::ALL
        .iter()
        .all(|pillar| covered_pillars.contains(pillar))
    {
        recomputed.push(ShellPrimitiveReleaseFinding::TruthPillarsIncomplete);
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(ShellPrimitiveReleaseFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(ShellPrimitiveReleaseFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(ShellPrimitiveReleaseValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(
            ShellPrimitiveReleaseValidationError::BlockingFindingPresent {
                class: finding.class_token().to_owned(),
                subject_ref: finding.subject_ref().to_owned(),
            },
        );
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(ShellPrimitiveReleaseValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(ShellPrimitiveReleaseValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(ShellPrimitiveReleaseValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(ShellPrimitiveReleaseValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
