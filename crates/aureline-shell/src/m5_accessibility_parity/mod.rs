//! Accessibility, high-zoom, high-contrast, reduced-motion, touch, and support-export parity
//! certified across the M5 status, hover/peek, splitter, and progress shell primitives.
//!
//! The [frozen shell-primitives matrix][matrix] already freezes the ten governed shell
//! primitives — the status-bar item and overflow menu, the tooltip/hovercard/peek/pinned
//! preview transient-inspect surfaces, the splitter handle and pane-resize preset, and the
//! ambient progress indicator and durable job row — into one export-safe packet: their
//! status-item classes, overflow behaviors, representation classes, promotion states,
//! pane-resize states, progress states, source/provider/freshness labels, non-visual
//! accessibility routes, the mandatory labels every primitive must be able to show, and the
//! downgrade triggers that narrow them below a claim. This lane is the **accessibility parity
//! certification capstone** on top of that matrix: for every claimed non-default accessibility
//! condition — keyboard reach, focus return, screen-reader narration, touch / context-action,
//! high-zoom, high-contrast, and reduced-motion — it certifies that the status, hover/peek,
//! splitter, and progress primitives stay keyboard- and screen-reader-reachable with focus
//! that returns after dismiss; stay legible and stable under high-zoom and high-contrast;
//! keep durable text alternatives when motion is reduced and touch / context-action
//! alternatives where a pointer affordance would otherwise be required; and reconstruct their
//! accessibility posture and primitive state from reusable fixtures and a support export
//! rather than ad hoc visual or screenshot checks.
//!
//! Three records carry the truth:
//!
//! - the per-condition **certification row** ([`AccessibilityParityRow`]): one row per
//!   [`M5AccessibilityCondition`] naming the ten primitives it certifies, the status-item
//!   classes / overflow behaviors / representation classes / promotion states / pane-resize
//!   states / progress states / source-freshness labels / accessibility routes / required
//!   labels / shell zones / consumer surfaces / downgrade triggers pulled from the frozen
//!   matrix, its non-visual-reach / zoom-contrast-stability / motion-touch-alternative /
//!   accessibility-export posture, any active waiver, and a derived green/yellow/red
//!   [`AccessibilityParityStatus`].
//! - the release **certification packet** ([`AccessibilityParityPacket`]): the full set of
//!   rows with derived per-row status, aggregate green/yellow/red counts, the active waivers,
//!   the exact certification causes ([`AccessibilityParityCause`]), and the blocking findings
//!   the lane refuses to ship with.
//! - the **certification dashboard** ([`AccessibilityParityDashboard`]): a light projection
//!   the shell / accessibility bridge / release automation reads to auto-narrow a claimed
//!   condition when its accessibility parity proof falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the
//! moment it discloses a reduced non-visual reach detail, a reduced zoom/contrast detail, a
//! waivered reduced motion/touch alternative, or a partial support-export capture; it drops to
//! `red` if a primitive's truth is reachable only by pointer or hover, is truncated or
//! unreadable under high-zoom or high-contrast, is conveyed by motion or a pointer affordance
//! only, is absent from the support-export capture, keeps any critical truth pointer- or
//! hover-only, or its accessibility routes / required labels are incomplete. That derivation
//! is the auto-narrowing the acceptance criteria require.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw
//! local paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids,
//! closed vocabulary, counts, refs, and short labels. The status-item-class, overflow-behavior,
//! representation-class, promotion-state, pane-resize-state, progress-state, source-freshness,
//! accessibility-route, required-label, shell-zone, consumer-surface, downgrade-trigger, and
//! qualification vocabulary is re-exported by reference from the already frozen [matrix]; each
//! row pulls its primitive bindings straight from the matrix's seeded ten primitive rows, so
//! this lane mints no parallel shell vocabulary and cannot certify an accessibility posture the
//! matrix does not freeze. Only the certification-specific vocabulary
//! ([`M5AccessibilityCondition`], [`M5AccessibilityParityProofDimension`],
//! [`AccessibilityParityStatus`], [`NonVisualReachState`], [`ZoomContrastStabilityState`],
//! [`MotionTouchAlternativeState`], [`AccessibilityExportState`],
//! [`AccessibilityParityWaiver`], [`AccessibilityParityCause`], [`AccessibilityParityFinding`])
//! is new.
//!
//! [matrix]: crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_status_bar_transient_inspect_pane_control_and_durable_progress_component_matrix as matrix;

pub use matrix::{
    M5AccessibilityRoute, M5OverflowBehavior, M5PaneResizeState, M5PrimitiveQualificationClass,
    M5PrimitiveRequiredLabel, M5ProgressState, M5PromotionState, M5RepresentationClass,
    M5ShellConsumerSurface, M5ShellPrimitiveDowngradeTrigger, M5ShellPrimitiveFamily,
    M5ShellZoneSlot, M5SourceFreshnessLabel, M5StatusItemClass,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_accessibility_parity_packet,
    seeded_m5_accessibility_parity_packet_focus_return_pointer_or_hover_only_blocked,
    seeded_m5_accessibility_parity_packet_high_zoom_unreadable_blocked,
    seeded_m5_accessibility_parity_packet_keyboard_reach_pointer_or_hover_only_blocked,
    seeded_m5_accessibility_parity_packet_reduced_motion_motion_only_blocked,
    seeded_m5_accessibility_parity_packet_touch_export_absent_blocked, SEED_BUILD_IDENTITY_REF,
    SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_ACCESSIBILITY_PARITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_ACCESSIBILITY_PARITY_SHARED_CONTRACT_REF: &str = "shell:m5_accessibility_parity:v1";

/// Stable record kind for [`AccessibilityParityPacket`] payloads.
pub const M5_ACCESSIBILITY_PARITY_PACKET_RECORD_KIND: &str =
    "shell_m5_accessibility_parity_packet_record";

/// Stable record kind for [`AccessibilityParityDashboard`] payloads.
pub const M5_ACCESSIBILITY_PARITY_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_accessibility_parity_dashboard_record";

/// Stable record kind for [`AccessibilityParitySupportExport`] payloads.
pub const M5_ACCESSIBILITY_PARITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_accessibility_parity_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_ACCESSIBILITY_PARITY_PACKET_ID: &str = "m5-accessibility-parity:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_ACCESSIBILITY_PARITY_DASHBOARD_ID: &str =
    "m5-accessibility-parity-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_ACCESSIBILITY_PARITY_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-accessibility-parity:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_ACCESSIBILITY_PARITY_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-accessibility-parity.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_ACCESSIBILITY_PARITY_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-accessibility-parity.md";

/// Published certification-packet artifact ref.
pub const M5_ACCESSIBILITY_PARITY_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-accessibility-parity-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_ACCESSIBILITY_PARITY_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-accessibility-parity-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_ACCESSIBILITY_PARITY_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-accessibility-parity-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_ACCESSIBILITY_PARITY_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-accessibility-parity-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_ACCESSIBILITY_PARITY_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_accessibility_parity_contract.md";

/// Repo-relative ref to the frozen shell-primitives matrix schema.
pub const M5_ACCESSIBILITY_PARITY_MATRIX_SCHEMA_REF: &str = matrix::M5_SHELL_PRIMITIVES_SCHEMA_REF;

/// The six non-visual accessibility routes every certified primitive must offer. The union
/// across the ten frozen primitives covers the full [`M5AccessibilityRoute::ALL`] set:
/// keyboard-focusable, screen-reader-announced, non-hover-reachable, pointer-optional,
/// high-contrast-safe, and support-exportable.
pub const ACCESSIBILITY_PARITY_REQUIRED_ROUTES: [M5AccessibilityRoute; 6] =
    M5AccessibilityRoute::ALL;

/// The six labels every certified primitive must be able to show. The union across the ten
/// frozen primitives covers the full [`M5PrimitiveRequiredLabel::ALL`] set: identity, state,
/// keyboard route, source provider, freshness, and reopen path.
pub const ACCESSIBILITY_PARITY_REQUIRED_LABELS: [M5PrimitiveRequiredLabel; 6] =
    M5PrimitiveRequiredLabel::ALL;

/// One of the claimed non-default accessibility conditions the parity proof must cover, in
/// canonical order. Each condition is a claimed M5 shell accessibility mode under which the
/// status, hover/peek, splitter, and progress primitives must stay reachable, legible, and
/// diagnosable; the lane certifies none beyond them and refuses to ship if any is missing.
/// These are exactly the fixture-coverage cases the acceptance criteria require: keyboard
/// reach, focus return, narration, touch / context-action, high-zoom, high-contrast, and
/// reduced-motion.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AccessibilityCondition {
    /// Keyboard-only reach: every primitive is operable without a pointer.
    KeyboardReach,
    /// Focus return: focus lands on and returns to the right place after dismiss.
    FocusReturn,
    /// Screen-reader narration: every primitive is announced to a screen reader.
    ScreenReaderNarration,
    /// Touch / context-action: pointer affordances have touch / context-menu alternatives.
    TouchContextAction,
    /// High-zoom / large-text rendering.
    HighZoom,
    /// High-contrast rendering.
    HighContrast,
    /// Reduced-motion rendering.
    ReducedMotion,
}

impl M5AccessibilityCondition {
    /// Every governed accessibility condition, in canonical order.
    pub const ALL: [Self; 7] = [
        Self::KeyboardReach,
        Self::FocusReturn,
        Self::ScreenReaderNarration,
        Self::TouchContextAction,
        Self::HighZoom,
        Self::HighContrast,
        Self::ReducedMotion,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardReach => "keyboard_reach",
            Self::FocusReturn => "focus_return",
            Self::ScreenReaderNarration => "screen_reader_narration",
            Self::TouchContextAction => "touch_context_action",
            Self::HighZoom => "high_zoom",
            Self::HighContrast => "high_contrast",
            Self::ReducedMotion => "reduced_motion",
        }
    }

    /// Short, reviewer-facing label for the condition.
    pub const fn label(self) -> &'static str {
        match self {
            Self::KeyboardReach => "Keyboard-only reach",
            Self::FocusReturn => "Focus return after dismiss",
            Self::ScreenReaderNarration => "Screen-reader narration",
            Self::TouchContextAction => "Touch / context-action alternatives",
            Self::HighZoom => "High-zoom / large-text rendering",
            Self::HighContrast => "High-contrast rendering",
            Self::ReducedMotion => "Reduced-motion rendering",
        }
    }
}

/// One of the four certification dimensions each accessibility condition is certified across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AccessibilityParityProofDimension {
    /// Non-visual reach (keyboard focus, screen-reader narration, focus return).
    NonVisualReach,
    /// Zoom / contrast stability (legible and stable under high-zoom and high-contrast).
    ZoomContrastStability,
    /// Motion / touch alternatives (durable text when motion is reduced, touch alternatives
    /// where a pointer affordance would otherwise be required).
    MotionTouchAlternative,
    /// Accessibility export (the accessibility posture and primitive state reconstruct from
    /// fixtures and a support export).
    AccessibilityExport,
}

impl M5AccessibilityParityProofDimension {
    /// Every certification dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NonVisualReach,
        Self::ZoomContrastStability,
        Self::MotionTouchAlternative,
        Self::AccessibilityExport,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonVisualReach => "non_visual_reach",
            Self::ZoomContrastStability => "zoom_contrast_stability",
            Self::MotionTouchAlternative => "motion_touch_alternative",
            Self::AccessibilityExport => "accessibility_export",
        }
    }
}

/// The derived certification light a governed accessibility condition carries.
///
/// `green` means the status, hover/peek, splitter, and progress primitives stay keyboard- and
/// screen-reader-reachable with focus that returns, stay legible and stable under high-zoom
/// and high-contrast, keep durable text and touch alternatives, and reconstruct their
/// accessibility posture from fixtures and a support export. `yellow` is a disclosed narrowing
/// (a reduced non-visual reach detail, a reduced zoom/contrast detail, a waivered reduced
/// motion/touch alternative, or a partial support-export capture). `red` is blocked: a
/// primitive's truth is reachable only by pointer or hover, is truncated or unreadable under
/// high-zoom or high-contrast, is conveyed by motion or a pointer affordance only, is absent
/// from the support-export capture, keeps critical truth pointer- or hover-only, or its
/// accessibility routes / required labels are incomplete — and the condition may not keep a
/// shell-maturity claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityParityStatus {
    /// Full standing: reachable, legible, motion/touch-safe, reconstructable.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl AccessibilityParityStatus {
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

/// How the condition keeps every primitive keyboard- and screen-reader-reachable, with focus
/// that returns after dismiss and no truth left behind pointer hover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NonVisualReachState {
    /// Every primitive is keyboard-focusable, announced to a screen reader, and returns focus
    /// to the right place after dismiss; hover-dependent information has a keyboard/focus
    /// alternative and splitter controls expose visible focus and keyboard step sizes.
    KeyboardFocusAndNarrationReachable,
    /// Under this condition a non-visual reach detail is disclosedly reduced (a longer
    /// narration abbreviates, or a focus-return lands one level up) while every primitive
    /// stays keyboard- and screen-reader-reachable and the reduction is disclosed.
    DisclosedReducedReachDetail,
    /// A primitive's truth is reachable only by pointer or hover — no keyboard focus, no
    /// screen-reader narration, or focus does not return after dismiss — always a blocker.
    TruthReachableByPointerOrHoverOnly,
}

impl NonVisualReachState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardFocusAndNarrationReachable => "keyboard_focus_and_narration_reachable",
            Self::DisclosedReducedReachDetail => "disclosed_reduced_reach_detail",
            Self::TruthReachableByPointerOrHoverOnly => "truth_reachable_by_pointer_or_hover_only",
        }
    }

    /// `true` when every primitive stays keyboard/screen-reader reachable.
    pub const fn is_reachable(self) -> bool {
        matches!(self, Self::KeyboardFocusAndNarrationReachable)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedReachDetail)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::TruthReachableByPointerOrHoverOnly)
    }
}

/// How the condition keeps every primitive legible and stable under high-zoom and
/// high-contrast rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoomContrastStabilityState {
    /// Every primitive stays legible and keeps a stable placement and meaning under high-zoom
    /// and high-contrast; nothing truncates, clips, or reflows away a truth-bearing item.
    LegibleStableUnderZoomAndContrast,
    /// Under this condition a zoom/contrast detail is disclosedly reduced (a label wraps to a
    /// shorter form, or a decorative accent drops) while every primitive stays legible and the
    /// reduction is disclosed.
    DisclosedReducedZoomContrastDetail,
    /// A primitive is truncated, clipped, or unreadable under high-zoom or high-contrast, so a
    /// truth-bearing item is lost or illegible — always a blocker.
    TruncatedOrUnreadableUnderZoomOrContrast,
}

impl ZoomContrastStabilityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LegibleStableUnderZoomAndContrast => "legible_stable_under_zoom_and_contrast",
            Self::DisclosedReducedZoomContrastDetail => "disclosed_reduced_zoom_contrast_detail",
            Self::TruncatedOrUnreadableUnderZoomOrContrast => {
                "truncated_or_unreadable_under_zoom_or_contrast"
            }
        }
    }

    /// `true` when every primitive stays legible and stable.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::LegibleStableUnderZoomAndContrast)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedZoomContrastDetail)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::TruncatedOrUnreadableUnderZoomOrContrast)
    }
}

/// How the condition keeps a durable text alternative for anything conveyed by motion and a
/// touch / context-action alternative for anything a pointer affordance would otherwise
/// require.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MotionTouchAlternativeState {
    /// Progress rows keep a durable text alternative when motion is reduced, and every pointer
    /// affordance (hover reveal, splitter drag) has a touch / context-action alternative.
    DurableTextAndTouchAlternativesPresent,
    /// Under this condition an alternative is disclosedly reduced (a coarser touch target, or
    /// a summarized text alternative) while a durable text and touch path stays present and
    /// the reduction is disclosed and waivered.
    DisclosedReducedAlternativeDetail,
    /// Critical state or progress is conveyed by motion only (a spinner) or by a pointer
    /// affordance only, with no durable text or touch alternative — always a blocker.
    MotionOnlyOrPointerOnlyAffordance,
}

impl MotionTouchAlternativeState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableTextAndTouchAlternativesPresent => {
                "durable_text_and_touch_alternatives_present"
            }
            Self::DisclosedReducedAlternativeDetail => "disclosed_reduced_alternative_detail",
            Self::MotionOnlyOrPointerOnlyAffordance => "motion_only_or_pointer_only_affordance",
        }
    }

    /// `true` when durable text and touch alternatives are present.
    pub const fn is_present(self) -> bool {
        matches!(self, Self::DurableTextAndTouchAlternativesPresent)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedAlternativeDetail)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::MotionOnlyOrPointerOnlyAffordance)
    }
}

/// How the condition's accessibility posture and primitive state are reconstructable from
/// reusable fixtures and a support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AccessibilityExportState {
    /// The support export and the reusable fixtures reconstruct the condition's accessibility
    /// posture — the visible/overflowed status state, the preview representation/freshness, the
    /// pane-control posture, and the progress/job-row state — so an accessibility regression can
    /// be diagnosed without a live screenshot.
    AccessibilityPostureAndStateReconstructable,
    /// The support export reconstructs the accessibility posture and discloses a partial
    /// capture (some low-priority primitive detail is trimmed) while the reduction is disclosed.
    DisclosedPartialCapture,
    /// The condition's accessibility state is absent from the support-export capture, so an
    /// accessibility regression cannot be explained without a live screenshot — always a
    /// blocker.
    AccessibilityStateAbsentFromCapture,
}

impl AccessibilityExportState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AccessibilityPostureAndStateReconstructable => {
                "accessibility_posture_and_state_reconstructable"
            }
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::AccessibilityStateAbsentFromCapture => "accessibility_state_absent_from_capture",
        }
    }

    /// `true` when the fixtures and export reconstruct the accessibility posture.
    pub const fn is_reconstructable(self) -> bool {
        matches!(self, Self::AccessibilityPostureAndStateReconstructable)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::AccessibilityStateAbsentFromCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red reduced motion/touch
/// alternative stay yellow rather than blocked — never lets a pointer/hover-only truth, an
/// illegible zoom/contrast surface, a motion-only affordance, or a missing export hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityParityWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed accessibility condition the waiver applies to.
    pub condition: M5AccessibilityCondition,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl AccessibilityParityWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed condition's certification.
///
/// The trigger token mirrors the frozen [`M5ShellPrimitiveDowngradeTrigger`] vocabulary so a
/// cause never mints a parallel reason synonym.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityParityCause {
    /// The governed accessibility condition the cause applies to.
    pub condition: M5AccessibilityCondition,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5ShellPrimitiveDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed
    /// cause is a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl AccessibilityParityCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed accessibility condition, certified across non-visual reach, zoom/contrast
/// stability, motion/touch alternatives, and accessibility export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityParityRow {
    /// The governed accessibility condition being certified.
    pub condition: M5AccessibilityCondition,
    /// The primitives this condition certifies (all ten frozen shell primitives). Pulled from
    /// the matrix.
    pub driven_primitive_families: Vec<M5ShellPrimitiveFamily>,
    /// The frozen qualification class across the ten primitives (the most-narrowed). Pulled
    /// from the matrix.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Owner role accountable for keeping this condition certified.
    pub owner_role: String,
    /// Short condition label.
    pub condition_label: String,
    /// Shell zones the certified primitives attach to (union across the ten). Pulled from the
    /// matrix.
    pub certified_shell_zone_slots: Vec<M5ShellZoneSlot>,
    /// Status-item classes the certified primitives project (union). Pulled from the matrix.
    pub certified_status_item_classes: Vec<M5StatusItemClass>,
    /// Overflow behaviors the certified primitives honour (union). Pulled from the matrix.
    pub certified_overflow_behaviors: Vec<M5OverflowBehavior>,
    /// Representation classes the certified transient surfaces show (union). Pulled from the
    /// matrix.
    pub certified_representation_classes: Vec<M5RepresentationClass>,
    /// Promotion states the certified promoting surfaces honour (union). Pulled from the
    /// matrix.
    pub certified_promotion_states: Vec<M5PromotionState>,
    /// Pane-resize states the certified pane controls honour (union). Pulled from the matrix.
    pub certified_pane_resize_states: Vec<M5PaneResizeState>,
    /// Progress states the certified progress rows honour (union). Pulled from the matrix.
    pub certified_progress_states: Vec<M5ProgressState>,
    /// Source/provider/freshness labels the certified primitives can show (union). Pulled from
    /// the matrix.
    pub certified_source_freshness_labels: Vec<M5SourceFreshnessLabel>,
    /// Non-visual accessibility routes (union). Pulled from the matrix.
    pub accessibility_routes: Vec<M5AccessibilityRoute>,
    /// Mandatory labels every certified primitive must be able to show (union). Pulled from
    /// the matrix.
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// Shell subsystems this condition stays aligned across (union). Pulled from the matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this condition (union). Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5ShellPrimitiveDowngradeTrigger>,
    /// Non-visual-reach posture.
    pub non_visual_reach: NonVisualReachState,
    /// Zoom/contrast-stability posture.
    pub zoom_contrast_stability: ZoomContrastStabilityState,
    /// Motion/touch-alternative posture.
    pub motion_touch_alternative: MotionTouchAlternativeState,
    /// Accessibility-export posture.
    pub accessibility_export: AccessibilityExportState,
    /// Hard invariant: no critical truth is kept pointer- or hover-only. `false` is a blocker.
    pub never_pointer_or_hover_only: bool,
    /// Active waiver, when a disclosed reduced motion/touch alternative is in force.
    pub active_waiver: Option<AccessibilityParityWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: AccessibilityParityStatus,
    /// The exact certification causes that narrowed or blocked this row.
    pub certification_causes: Vec<AccessibilityParityCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl AccessibilityParityRow {
    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when every non-visual accessibility route the matrix freezes is certified — the
    /// lint that prevents a condition from shipping without keyboard-focusable,
    /// screen-reader-announced, non-hover-reachable, pointer-optional, high-contrast-safe, and
    /// support-exportable routes on the certified primitives.
    pub fn accessibility_routes_complete(&self) -> bool {
        let present: BTreeSet<M5AccessibilityRoute> =
            self.accessibility_routes.iter().copied().collect();
        ACCESSIBILITY_PARITY_REQUIRED_ROUTES
            .iter()
            .all(|route| present.contains(route))
    }

    /// `true` when every required label the matrix freezes is certified — the lint that
    /// prevents a condition from shipping without identity, state, keyboard-route,
    /// source-provider, freshness, and reopen-path labels on the certified primitives.
    pub fn required_labels_complete(&self) -> bool {
        let present: BTreeSet<M5PrimitiveRequiredLabel> =
            self.required_labels.iter().copied().collect();
        ACCESSIBILITY_PARITY_REQUIRED_LABELS
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        self.non_visual_reach.is_blocked()
            || self.zoom_contrast_stability.is_blocked()
            || self.motion_touch_alternative.is_blocked()
            || self.accessibility_export.is_blocked()
            || !self.never_pointer_or_hover_only
            || !self.accessibility_routes_complete()
            || !self.required_labels_complete()
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.non_visual_reach.is_disclosed()
            || self.zoom_contrast_stability.is_disclosed()
            || self.motion_touch_alternative.is_disclosed()
            || self.accessibility_export.is_disclosed()
    }

    /// Recomputes the derived status from the four axes and the pointer/hover-only invariant.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing
    /// forces `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> AccessibilityParityStatus {
        if self.has_hard_blocker() {
            AccessibilityParityStatus::Red
        } else if self.has_narrowing() {
            AccessibilityParityStatus::Yellow
        } else {
            AccessibilityParityStatus::Green
        }
    }

    /// Recomputes the exact certification causes for the row, in deterministic order
    /// (non-visual reach, zoom/contrast, motion/touch, export, pointer/hover-only invariant).
    pub fn recompute_causes(&self) -> Vec<AccessibilityParityCause> {
        let mut causes = Vec::new();
        if !self.non_visual_reach.is_reachable() {
            causes.push(AccessibilityParityCause {
                condition: self.condition,
                trigger: M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth,
                disclosed: self.non_visual_reach.is_disclosed(),
                detail: if self.non_visual_reach.is_disclosed() {
                    "Under this condition a non-visual reach detail is disclosedly reduced (a longer \
                     narration abbreviates, or a focus-return lands one level up) while every \
                     primitive stays keyboard- and screen-reader-reachable; the reduction is \
                     disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "A primitive's truth is reachable only by pointer or hover — no keyboard focus, \
                     no screen-reader narration, or focus does not return after dismiss."
                        .to_owned()
                },
            });
        }
        if !self.zoom_contrast_stability.is_stable() {
            causes.push(AccessibilityParityCause {
                condition: self.condition,
                trigger: M5ShellPrimitiveDowngradeTrigger::VanityItemReflow,
                disclosed: self.zoom_contrast_stability.is_disclosed(),
                detail: if self.zoom_contrast_stability.is_disclosed() {
                    "Under this condition a zoom/contrast detail is disclosedly reduced (a label \
                     wraps to a shorter form, or a decorative accent drops) while every primitive \
                     stays legible; the reduction is disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "A primitive is truncated, clipped, or unreadable under high-zoom or \
                     high-contrast, so a truth-bearing item is lost or illegible."
                        .to_owned()
                },
            });
        }
        if !self.motion_touch_alternative.is_present() {
            causes.push(AccessibilityParityCause {
                condition: self.condition,
                trigger: M5ShellPrimitiveDowngradeTrigger::SpinnerOnlyState,
                disclosed: self.motion_touch_alternative.is_disclosed(),
                detail: if self.motion_touch_alternative.is_disclosed() {
                    "Under this condition an alternative is disclosedly reduced (a coarser touch \
                     target, or a summarized text alternative) while a durable text and touch path \
                     stays present; the reduction is disclosed and waivered, and the row is narrowed \
                     below green."
                        .to_owned()
                } else {
                    "Critical state or progress is conveyed by motion only (a spinner) or by a \
                     pointer affordance only, with no durable text or touch alternative."
                        .to_owned()
                },
            });
        }
        if !self.accessibility_export.is_reconstructable() {
            causes.push(AccessibilityParityCause {
                condition: self.condition,
                trigger: M5ShellPrimitiveDowngradeTrigger::ProofStale,
                disclosed: self.accessibility_export.is_disclosed(),
                detail: if self.accessibility_export.is_disclosed() {
                    "The support export reconstructs the accessibility posture and discloses a \
                     partial capture (some low-priority primitive detail is trimmed) while the \
                     reduction is disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "The condition's accessibility state is absent from the support-export capture, \
                     so an accessibility regression cannot be explained without a live screenshot."
                        .to_owned()
                },
            });
        }
        if !self.never_pointer_or_hover_only {
            causes.push(AccessibilityParityCause {
                condition: self.condition,
                trigger: M5ShellPrimitiveDowngradeTrigger::HoverOnlyCriticalTruth,
                disclosed: false,
                detail: "A primitive keeps a critical truth or progress visible only through a \
                         pointer hover or a pointer-only affordance, with no keyboard/focus or \
                         touch alternative."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced motion/touch alternative may only stay yellow (rather than red)
    /// when a waiver discloses it.
    pub fn requires_waiver(&self) -> bool {
        self.motion_touch_alternative.is_disclosed()
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<AccessibilityParityFinding> {
        let mut findings = Vec::new();
        let condition = self.condition.as_str().to_owned();

        if self.non_visual_reach.is_blocked() {
            findings.push(AccessibilityParityFinding::ReachPointerOrHoverOnly {
                condition: condition.clone(),
            });
        }
        if self.zoom_contrast_stability.is_blocked() {
            findings.push(AccessibilityParityFinding::ZoomContrastUnreadable {
                condition: condition.clone(),
            });
        }
        if self.motion_touch_alternative.is_blocked() {
            findings.push(AccessibilityParityFinding::MotionOrPointerOnlyAffordance {
                condition: condition.clone(),
            });
        }
        if self.accessibility_export.is_blocked() {
            findings.push(
                AccessibilityParityFinding::AccessibilityStateAbsentFromCapture {
                    condition: condition.clone(),
                },
            );
        }
        if !self.never_pointer_or_hover_only {
            findings.push(
                AccessibilityParityFinding::CriticalTruthPointerOrHoverOnly {
                    condition: condition.clone(),
                },
            );
        }
        if !self.accessibility_routes_complete() {
            findings.push(AccessibilityParityFinding::AccessibilityRoutesIncomplete {
                condition: condition.clone(),
            });
        }
        if !self.required_labels_complete() {
            findings.push(AccessibilityParityFinding::RequiredLabelsIncomplete {
                condition: condition.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, AccessibilityParityStatus::Green) && !self.has_reason() {
            findings.push(AccessibilityParityFinding::NarrowedRowWithoutReason {
                condition: condition.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active
        // waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(AccessibilityParityFinding::NarrowedRowWithoutWaiver {
                condition: condition.clone(),
            });
        }
        // An attached waiver must still be active and must point at this condition.
        if let Some(waiver) = &self.active_waiver {
            if waiver.condition != self.condition {
                findings.push(AccessibilityParityFinding::WaiverConditionMismatch {
                    condition: condition.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(AccessibilityParityFinding::WaiverExpired {
                    condition: condition.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(AccessibilityParityFinding::RowStatusStale {
                condition: condition.clone(),
            });
        }
        if self.certification_causes != self.recompute_causes() {
            findings.push(AccessibilityParityFinding::RowCausesStale { condition });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} reach={} zoom_contrast={} motion_touch={} export={} no_pointer_hover_only={} waiver={}",
            self.condition.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.non_visual_reach.as_str(),
            self.zoom_contrast_stability.as_str(),
            self.motion_touch_alternative.as_str(),
            self.accessibility_export.as_str(),
            self.never_pointer_or_hover_only,
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the accessibility parity proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum AccessibilityParityFinding {
    /// A governed accessibility condition has no certification row.
    ConditionMissing {
        /// The missing condition token.
        condition: String,
    },
    /// A condition keeps a primitive's truth reachable only by pointer or hover.
    ReachPointerOrHoverOnly {
        /// The condition token.
        condition: String,
    },
    /// A condition has a primitive truncated or unreadable under high-zoom or high-contrast.
    ZoomContrastUnreadable {
        /// The condition token.
        condition: String,
    },
    /// A condition conveys critical state by motion only or a pointer affordance only.
    MotionOrPointerOnlyAffordance {
        /// The condition token.
        condition: String,
    },
    /// A condition's accessibility state is absent from the support-export capture.
    AccessibilityStateAbsentFromCapture {
        /// The condition token.
        condition: String,
    },
    /// A condition keeps a critical truth pointer- or hover-only (hard invariant).
    CriticalTruthPointerOrHoverOnly {
        /// The condition token.
        condition: String,
    },
    /// A condition does not certify every frozen accessibility route.
    AccessibilityRoutesIncomplete {
        /// The condition token.
        condition: String,
    },
    /// A condition does not certify every frozen required label.
    RequiredLabelsIncomplete {
        /// The condition token.
        condition: String,
    },
    /// A narrowed or blocked row does not disclose why.
    NarrowedRowWithoutReason {
        /// The condition token.
        condition: String,
    },
    /// A waiver-requiring narrowing carries no active waiver.
    NarrowedRowWithoutWaiver {
        /// The condition token.
        condition: String,
    },
    /// An attached waiver does not point at the row's condition.
    WaiverConditionMismatch {
        /// The condition token.
        condition: String,
        /// The mismatched waiver id.
        waiver_id: String,
    },
    /// An attached waiver is past its expiry.
    WaiverExpired {
        /// The condition token.
        condition: String,
        /// The expired waiver id.
        waiver_id: String,
    },
    /// The declared derived status does not match the recomputed status.
    RowStatusStale {
        /// The condition token.
        condition: String,
    },
    /// The declared certification causes do not match the recomputed causes.
    RowCausesStale {
        /// The condition token.
        condition: String,
    },
    /// One of the declared status counts does not match the rows.
    StatusCountsStale,
    /// The declared covered conditions do not match the rows.
    CoverageStale,
    /// The export carries raw boundary material (url/path/credential/token).
    RawBoundaryMaterialInExport,
}

impl AccessibilityParityFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ConditionMissing { .. } => "condition_missing",
            Self::ReachPointerOrHoverOnly { .. } => "reach_pointer_or_hover_only",
            Self::ZoomContrastUnreadable { .. } => "zoom_contrast_unreadable",
            Self::MotionOrPointerOnlyAffordance { .. } => "motion_or_pointer_only_affordance",
            Self::AccessibilityStateAbsentFromCapture { .. } => {
                "accessibility_state_absent_from_capture"
            }
            Self::CriticalTruthPointerOrHoverOnly { .. } => "critical_truth_pointer_or_hover_only",
            Self::AccessibilityRoutesIncomplete { .. } => "accessibility_routes_incomplete",
            Self::RequiredLabelsIncomplete { .. } => "required_labels_incomplete",
            Self::NarrowedRowWithoutReason { .. } => "narrowed_row_without_reason",
            Self::NarrowedRowWithoutWaiver { .. } => "narrowed_row_without_waiver",
            Self::WaiverConditionMismatch { .. } => "waiver_condition_mismatch",
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
            Self::ConditionMissing { condition }
            | Self::ReachPointerOrHoverOnly { condition }
            | Self::ZoomContrastUnreadable { condition }
            | Self::MotionOrPointerOnlyAffordance { condition }
            | Self::AccessibilityStateAbsentFromCapture { condition }
            | Self::CriticalTruthPointerOrHoverOnly { condition }
            | Self::AccessibilityRoutesIncomplete { condition }
            | Self::RequiredLabelsIncomplete { condition }
            | Self::NarrowedRowWithoutReason { condition }
            | Self::NarrowedRowWithoutWaiver { condition }
            | Self::WaiverConditionMismatch { condition, .. }
            | Self::WaiverExpired { condition, .. }
            | Self::RowStatusStale { condition }
            | Self::RowCausesStale { condition } => condition,
            Self::StatusCountsStale => "status_counts",
            Self::CoverageStale => "coverage",
            Self::RawBoundaryMaterialInExport => "export",
        }
    }
}

/// The release certification packet shared by the shell / accessibility bridge / release
/// automation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityParityPacket {
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
    /// The certification dimensions every condition is certified across.
    pub required_proof_dimensions: Vec<M5AccessibilityParityProofDimension>,
    /// The accessibility routes every condition must certify.
    pub required_accessibility_routes: Vec<M5AccessibilityRoute>,
    /// The required labels every condition must certify.
    pub required_labels: Vec<M5PrimitiveRequiredLabel>,
    /// Per-condition certification rows, in canonical order.
    pub rows: Vec<AccessibilityParityRow>,
    /// Governed conditions certified, in canonical (sorted) order.
    pub covered_conditions: Vec<String>,
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
    pub active_waivers: Vec<AccessibilityParityWaiver>,
    /// Every exact certification cause, in row then cause order.
    pub certification_causes: Vec<AccessibilityParityCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<AccessibilityParityFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Shell / release automation refs that consume this packet to auto-narrow claimed
    /// conditions.
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

impl AccessibilityParityPacket {
    /// Returns the certification row for `condition`, if present.
    pub fn row(&self, condition: M5AccessibilityCondition) -> Option<&AccessibilityParityRow> {
        self.rows.iter().find(|row| row.condition == condition)
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
                waiver.condition.as_str(),
                waiver.expires_at
            ));
        }
        for cause in &self.certification_causes {
            lines.push(format!(
                "  cause {} {} disclosed={}",
                cause.condition.as_str(),
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
    pub fn dashboard(&self) -> AccessibilityParityDashboard {
        AccessibilityParityDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 accessibility-parity packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per condition.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "condition,status,qualification,non_visual_reach,zoom_contrast_stability,motion_touch_alternative,accessibility_export,never_pointer_or_hover_only,accessibility_routes,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.condition.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.non_visual_reach.as_str(),
                row.zoom_contrast_stability.as_str(),
                row.motion_touch_alternative.as_str(),
                row.accessibility_export.as_str(),
                row.never_pointer_or_hover_only,
                join_tokens(&row.accessibility_routes, |r| r.as_str()),
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
        out.push_str("# M5 shell-primitive accessibility parity\n\n");
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_accessibility_parity`](../../crates/aureline-shell/src/m5_accessibility_parity/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_accessibility_parity -- markdown > \\\n  artifacts/shell/m5-accessibility-parity.md\n",
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
            "| Condition | Status | Qualification | Reach | Zoom/contrast | Motion/touch | Export | No-pointer/hover-only | Waiver |\n\
             | --------- | ------ | ------------- | ----- | ------------- | ------------ | ------ | --------------------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.condition_label,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.non_visual_reach.as_str(),
                row.zoom_contrast_stability.as_str(),
                row.motion_touch_alternative.as_str(),
                row.accessibility_export.as_str(),
                row.never_pointer_or_hover_only,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&AccessibilityParityRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, AccessibilityParityStatus::Green))
            .collect();
        if narrowed.is_empty() {
            out.push_str(
                "None — every governed accessibility condition is certified at full standing.\n\n",
            );
        } else {
            for row in narrowed {
                out.push_str(&format!(
                    "- `{}` (`{}`) — {}\n",
                    row.condition.as_str(),
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
                    cause.condition.as_str(),
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
                    waiver.condition.as_str(),
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_accessibility_parity -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_accessibility_parity_fixtures\n");
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityParityDashboardRow {
    /// The governed condition.
    pub condition: M5AccessibilityCondition,
    /// Short condition label.
    pub condition_label: String,
    /// Derived green/yellow/red status.
    pub status: AccessibilityParityStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5PrimitiveQualificationClass,
    /// Non-visual-reach posture.
    pub non_visual_reach: NonVisualReachState,
    /// Zoom/contrast-stability posture.
    pub zoom_contrast_stability: ZoomContrastStabilityState,
    /// Motion/touch-alternative posture.
    pub motion_touch_alternative: MotionTouchAlternativeState,
    /// Accessibility-export posture.
    pub accessibility_export: AccessibilityExportState,
    /// `true` when no critical truth is pointer- or hover-only.
    pub never_pointer_or_hover_only: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the shell / accessibility bridge / release automation
/// reads to auto-narrow claimed accessibility conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityParityDashboard {
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
    pub rows: Vec<AccessibilityParityDashboardRow>,
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

impl AccessibilityParityDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &AccessibilityParityPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| AccessibilityParityDashboardRow {
                condition: row.condition,
                condition_label: row.condition_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                non_visual_reach: row.non_visual_reach,
                zoom_contrast_stability: row.zoom_contrast_stability,
                motion_touch_alternative: row.motion_touch_alternative,
                accessibility_export: row.accessibility_export,
                never_pointer_or_hover_only: row.never_pointer_or_hover_only,
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
            record_kind: M5_ACCESSIBILITY_PARITY_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_ACCESSIBILITY_PARITY_SCHEMA_VERSION,
            dashboard_id: M5_ACCESSIBILITY_PARITY_DASHBOARD_ID.to_owned(),
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
        serde_json::to_string_pretty(self).expect("m5 accessibility-parity dashboard serializes")
    }
}

/// Support-export wrapper for the accessibility parity certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityParitySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: AccessibilityParityPacket,
    /// Dashboard quoted in full.
    pub dashboard: AccessibilityParityDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl AccessibilityParitySupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each condition, and each
    /// active waiver id is quoted as a case id so a support reviewer — or the shell
    /// automation — can name the same condition and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: AccessibilityParityPacket,
    ) -> Self {
        let mut case_ids = vec![
            packet.packet_id.clone(),
            packet.matrix_packet_ref.clone(),
            packet.build_identity_ref.clone(),
        ];
        for row in &packet.rows {
            case_ids.push(row.condition.as_str().to_owned());
            if let Some(waiver) = &row.active_waiver {
                case_ids.push(waiver.waiver_id.clone());
            }
        }
        let dashboard = packet.dashboard();
        Self {
            record_kind: M5_ACCESSIBILITY_PARITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_ACCESSIBILITY_PARITY_SCHEMA_VERSION,
            shared_contract_ref: M5_ACCESSIBILITY_PARITY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_accessibility_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessibilityParityInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen shell-primitives matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-condition certification rows.
    pub rows: Vec<AccessibilityParityRow>,
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

/// Builds an [`AccessibilityParityPacket`] from the exact build identity, the frozen matrix
/// ref, and the per-condition certification rows.
///
/// Each row's derived status and certification causes, the aggregate counts, the active
/// waivers, and the blocking findings are recomputed here so the packet is the single source
/// of truth and the auto-narrowing cannot be asserted.
pub fn build_m5_accessibility_parity_packet(
    input: AccessibilityParityInput,
) -> AccessibilityParityPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<AccessibilityParityRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.certification_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<AccessibilityParityFinding> = Vec::new();

    // Every governed condition must carry a certification row.
    let present: BTreeSet<M5AccessibilityCondition> =
        rows.iter().map(|row| row.condition).collect();
    for condition in M5AccessibilityCondition::ALL {
        if !present.contains(&condition) {
            blocking_findings.push(AccessibilityParityFinding::ConditionMissing {
                condition: condition.as_str().to_owned(),
            });
        }
    }
    for row in &rows {
        blocking_findings.extend(row.compute_findings(&generated_at));
    }

    let covered_conditions: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|condition| condition.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };

    let row_count = rows.len();
    let green_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AccessibilityParityStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AccessibilityParityStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, AccessibilityParityStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(AccessibilityParityFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<AccessibilityParityWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let certification_causes: Vec<AccessibilityParityCause> = rows
        .iter()
        .flat_map(|row| row.certification_causes.clone())
        .collect();

    let mut packet = AccessibilityParityPacket {
        record_kind: M5_ACCESSIBILITY_PARITY_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_ACCESSIBILITY_PARITY_SCHEMA_VERSION,
        shared_contract_ref: M5_ACCESSIBILITY_PARITY_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_ACCESSIBILITY_PARITY_PACKET_ID.to_owned(),
        source_schema_ref: M5_ACCESSIBILITY_PARITY_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Accessibility parity certified across the M5 status, hover/peek, splitter, and \
                   progress shell primitives for every claimed non-default condition: keyboard \
                   reach, focus return, screen-reader narration, touch / context-action, high-zoom, \
                   high-contrast, and reduced-motion each keep the primitives keyboard- and \
                   screen-reader-reachable with focus that returns, legible and stable under \
                   high-zoom and high-contrast, backed by durable text and touch alternatives, and \
                   reconstructable from reusable fixtures and a support export — with each row's \
                   green/yellow/red claim auto-narrowed from its non-visual-reach, \
                   zoom-contrast-stability, motion-touch-alternative, and accessibility-export \
                   posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_ACCESSIBILITY_PARITY_MATRIX_SCHEMA_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_proof_dimensions: M5AccessibilityParityProofDimension::ALL.to_vec(),
        required_accessibility_routes: ACCESSIBILITY_PARITY_REQUIRED_ROUTES.to_vec(),
        required_labels: ACCESSIBILITY_PARITY_REQUIRED_LABELS.to_vec(),
        rows,
        covered_conditions,
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
            "shell_frame.accessibility_bridge.shell_primitive_parity_registry".to_owned(),
            "release_automation.auto_narrow.accessibility_parity_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.accessibility_parity".to_owned(),
            "artifacts/release/m5-accessibility-parity-proof/packet.json".to_owned(),
        ],
        help_docs_refs: vec![M5_ACCESSIBILITY_PARITY_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-accessibility-parity".to_owned()],
        published_report_ref: M5_ACCESSIBILITY_PARITY_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_ACCESSIBILITY_PARITY_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_ACCESSIBILITY_PARITY_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_ACCESSIBILITY_PARITY_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(AccessibilityParityFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_accessibility_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum AccessibilityParityValidationError {
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
    /// The rows do not cover all seven governed conditions.
    CoverageIncomplete,
    /// The declared covered conditions do not match the rows.
    CoverageStale,
    /// The declared required proof dimensions are not the canonical set.
    RequiredDimensionsStale,
    /// The declared required accessibility routes are not the canonical set.
    RequiredAccessibilityRoutesStale,
    /// The declared required labels are not the canonical set.
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

/// Validates a packet against the accessibility parity invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed accessibility
/// condition carries a current certification row; each row's status is the derived
/// auto-narrowed value, never asserted; a green row cannot keep a claim while a primitive's
/// truth is reachable only by pointer or hover, is truncated or unreadable under high-zoom or
/// high-contrast, is conveyed by motion or a pointer affordance only, is dropped from capture,
/// keeps critical truth pointer- or hover-only, or its accessibility routes / required labels
/// are incomplete; and a disclosed narrowing is backed by a reason and, where required, an
/// active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_accessibility_parity_packet(
    packet: &AccessibilityParityPacket,
) -> Result<(), Vec<AccessibilityParityValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(AccessibilityParityValidationError::NoRows);
    }
    if packet.record_kind != M5_ACCESSIBILITY_PARITY_PACKET_RECORD_KIND {
        errors.push(AccessibilityParityValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_ACCESSIBILITY_PARITY_SCHEMA_VERSION {
        errors.push(AccessibilityParityValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(AccessibilityParityValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(AccessibilityParityValidationError::MatrixPacketRefMissing);
    }
    if packet.required_proof_dimensions != M5AccessibilityParityProofDimension::ALL {
        errors.push(AccessibilityParityValidationError::RequiredDimensionsStale);
    }
    if packet.required_accessibility_routes != ACCESSIBILITY_PARITY_REQUIRED_ROUTES {
        errors.push(AccessibilityParityValidationError::RequiredAccessibilityRoutesStale);
    }
    if packet.required_labels != ACCESSIBILITY_PARITY_REQUIRED_LABELS {
        errors.push(AccessibilityParityValidationError::RequiredLabelsStale);
    }

    let present: BTreeSet<M5AccessibilityCondition> =
        packet.rows.iter().map(|row| row.condition).collect();
    let coverage_complete = M5AccessibilityCondition::ALL
        .iter()
        .all(|condition| present.contains(condition));
    if !coverage_complete || packet.rows.len() != M5AccessibilityCondition::ALL.len() {
        errors.push(AccessibilityParityValidationError::CoverageIncomplete);
    }

    let covered: Vec<String> = {
        let mut covered: Vec<String> = present
            .iter()
            .map(|condition| condition.as_str().to_owned())
            .collect();
        covered.sort();
        covered
    };
    if covered != packet.covered_conditions {
        errors.push(AccessibilityParityValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AccessibilityParityStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AccessibilityParityStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), AccessibilityParityStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(AccessibilityParityValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<AccessibilityParityWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(AccessibilityParityValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<AccessibilityParityCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.certification_causes {
        errors.push(AccessibilityParityValidationError::CertificationCausesStale);
    }

    let mut recomputed: Vec<AccessibilityParityFinding> = Vec::new();
    for condition in M5AccessibilityCondition::ALL {
        if !present.contains(&condition) {
            recomputed.push(AccessibilityParityFinding::ConditionMissing {
                condition: condition.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(AccessibilityParityFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(AccessibilityParityFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(AccessibilityParityValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(AccessibilityParityValidationError::BlockingFindingPresent {
            class: finding.class_token().to_owned(),
            subject_ref: finding.subject_ref().to_owned(),
        });
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(AccessibilityParityValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(AccessibilityParityValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(AccessibilityParityValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(AccessibilityParityValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
