//! Keyboard, screen-reader, high-zoom, high-contrast, reduced-motion, density, and
//! support-export parity certified across the M5 settings-row, capability-sheet, and
//! evidence-chronology reusable trust components.
//!
//! The [frozen trust-chronology component matrix][matrix] already freezes Aureline's
//! highest-trust reusable components — the settings row, the permission/capability sheet, the
//! event/history row, the timeline group, the narrative summary card, and the chronology export
//! preview — into one export-safe packet: their settings-row states and source pills, the
//! capability consequence classes and scope states, the chronology verbs and provenance badges,
//! the chronology detail states and export fields, the non-visual accessibility routes, the
//! mandatory labels every component must be able to show, and the downgrade triggers that narrow
//! them below a claim. This lane is the **accessibility parity certification capstone** on top of
//! that matrix: for every claimed non-default accessibility condition — keyboard reach, focus
//! order, screen-reader narration, high-zoom, high-contrast, reduced-motion, and compact/comfortable
//! density — it certifies that the settings rows, capability sheets, event rows, timeline groups,
//! and export previews stay keyboard- and screen-reader-reachable with focus that lands and
//! returns in order; stay legible and stable under high-zoom, high-contrast, and compact density
//! with no truth left hover-only, color-only, or dropped when the surface compacts; keep a durable
//! static text alternative for anything conveyed by motion; and reconstruct the same source, state,
//! and chronology language from reusable fixtures and a support export rather than ad hoc visual or
//! screenshot checks.
//!
//! Three records carry the truth:
//!
//! - the per-condition **certification row** ([`TrustComponentParityRow`]): one row per
//!   [`M5TrustAccessibilityCondition`] naming the six components it certifies, the settings-row
//!   states / source pills / consequence classes / scope states / chronology verbs / provenance
//!   badges / detail states / export fields / accessibility routes / required labels / shell zones
//!   / responsive classes / window classes / surface families / consumer surfaces / downgrade
//!   triggers pulled from the frozen matrix, its non-visual-reach / zoom-contrast-density /
//!   motion-alternative / support-export-parity posture, any active waiver, and a derived
//!   green/yellow/red [`TrustComponentParityStatus`].
//! - the release **certification packet** ([`TrustComponentParityPacket`]): the full set of rows
//!   with derived per-row status, aggregate green/yellow/red counts, the active waivers, the exact
//!   certification causes ([`TrustComponentParityCause`]), and the blocking findings the lane
//!   refuses to ship with.
//! - the **certification dashboard** ([`TrustComponentParityDashboard`]): a light projection the
//!   shell / accessibility bridge / release automation reads to auto-narrow a claimed condition
//!   when its component accessibility parity proof falls out of policy.
//!
//! The row status is **derived**, never asserted: a row drops from `green` to `yellow` the moment
//! it discloses a reduced non-visual reach detail, a reduced zoom/contrast/density detail, a
//! waivered reduced motion alternative, or a partial support-export capture; it drops to `red` if a
//! component's truth is reachable only by pointer or hover, is truncated, color-only, or lost when
//! the surface compacts under high-zoom / high-contrast / compact density, is conveyed by motion
//! only, is absent from the support-export capture, keeps any critical truth hover-only /
//! color-only / compaction-lost, or its accessibility routes / required labels are incomplete. That
//! derivation is the auto-narrowing the acceptance criteria require.
//!
//! The records are inspectable, serde-serializable truth packets that carry no raw URLs, raw local
//! paths, raw usernames, raw hostnames, tokens, or credentials — only stable ids, closed
//! vocabulary, counts, refs, and short labels. The settings-row-state, source-pill,
//! consequence-class, scope-state, chronology-verb, provenance-badge, detail-state, export-field,
//! accessibility-route, required-label, shell-zone, responsive-class, window-class, surface-family,
//! consumer-surface, downgrade-trigger, and qualification vocabulary is re-exported by reference
//! from the already frozen [matrix]; each row pulls its component bindings straight from the
//! matrix's seeded six component rows, so this lane mints no parallel component vocabulary and
//! cannot certify an accessibility posture the matrix does not freeze. Only the
//! certification-specific vocabulary ([`M5TrustAccessibilityCondition`],
//! [`M5TrustComponentParityProofDimension`], [`TrustComponentParityStatus`], [`NonVisualReachState`],
//! [`ZoomContrastDensityState`], [`MotionAlternativeState`], [`SupportExportParityState`],
//! [`TrustComponentParityWaiver`], [`TrustComponentParityCause`], [`TrustComponentParityFinding`])
//! is new.
//!
//! [matrix]: crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_settings_row_capability_sheet_evidence_chronology_and_chronology_export_component_matrix as matrix;

pub use matrix::{
    M5CapabilityConsequenceClass, M5CapabilityScopeState, M5ChronologyDetailState,
    M5ChronologyExportField, M5ChronologyVerb, M5ProvenanceBadge, M5ResponsiveClass,
    M5SettingSourcePill, M5SettingsRowState, M5ShellConsumerSurface, M5ShellSurfaceFamily,
    M5ShellZoneSlot, M5TrustAccessibilityRoute, M5TrustComponentDowngradeTrigger,
    M5TrustComponentFamily, M5TrustQualificationClass, M5TrustRequiredLabel, M5WindowClass,
};

mod seed;
#[cfg(test)]
mod tests;

pub use seed::{
    seeded_m5_trust_component_accessibility_parity_packet,
    seeded_m5_trust_component_accessibility_parity_packet_focus_order_hover_color_only_blocked,
    seeded_m5_trust_component_accessibility_parity_packet_high_contrast_export_absent_blocked,
    seeded_m5_trust_component_accessibility_parity_packet_high_zoom_unreadable_blocked,
    seeded_m5_trust_component_accessibility_parity_packet_keyboard_reach_pointer_or_hover_only_blocked,
    seeded_m5_trust_component_accessibility_parity_packet_reduced_motion_motion_only_blocked,
    SEED_BUILD_IDENTITY_REF, SEED_RELEASE_CHANNEL_CLASS,
};

/// Schema version exported with every record.
pub const M5_TRUST_COMPONENT_PARITY_SCHEMA_VERSION: u32 = 1;

/// Shared contract ref consumed by every consumer.
pub const M5_TRUST_COMPONENT_PARITY_SHARED_CONTRACT_REF: &str =
    "shell:m5_trust_component_accessibility_parity:v1";

/// Stable record kind for [`TrustComponentParityPacket`] payloads.
pub const M5_TRUST_COMPONENT_PARITY_PACKET_RECORD_KIND: &str =
    "shell_m5_trust_component_accessibility_parity_packet_record";

/// Stable record kind for [`TrustComponentParityDashboard`] payloads.
pub const M5_TRUST_COMPONENT_PARITY_DASHBOARD_RECORD_KIND: &str =
    "shell_m5_trust_component_accessibility_parity_dashboard_record";

/// Stable record kind for [`TrustComponentParitySupportExport`] payloads.
pub const M5_TRUST_COMPONENT_PARITY_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_trust_component_accessibility_parity_support_export_record";

/// Stable packet id quoted across surfaces.
pub const M5_TRUST_COMPONENT_PARITY_PACKET_ID: &str =
    "m5-trust-component-accessibility-parity:stable:0001";

/// Stable dashboard id quoted across surfaces.
pub const M5_TRUST_COMPONENT_PARITY_DASHBOARD_ID: &str =
    "m5-trust-component-accessibility-parity-dashboard:stable:0001";

/// Stable support-export id.
pub const M5_TRUST_COMPONENT_PARITY_SUPPORT_EXPORT_ID: &str =
    "support-export:m5-trust-component-accessibility-parity:001";

/// Repo-relative ref to the boundary schema this packet conforms to.
pub const M5_TRUST_COMPONENT_PARITY_SOURCE_SCHEMA_REF: &str =
    "schemas/shell/m5-trust-component-accessibility-parity.schema.json";

/// Published markdown report ref reviewers reopen the certification proof from.
pub const M5_TRUST_COMPONENT_PARITY_PUBLISHED_REPORT_REF: &str =
    "artifacts/shell/m5-trust-component-accessibility-parity.md";

/// Published certification-packet artifact ref.
pub const M5_TRUST_COMPONENT_PARITY_PUBLISHED_PACKET_REF: &str =
    "artifacts/release/m5-trust-component-accessibility-parity-proof/packet.json";

/// Published certification-dashboard artifact ref.
pub const M5_TRUST_COMPONENT_PARITY_PUBLISHED_DASHBOARD_REF: &str =
    "artifacts/release/m5-trust-component-accessibility-parity-proof/dashboard.json";

/// Published support-export artifact ref.
pub const M5_TRUST_COMPONENT_PARITY_PUBLISHED_SUPPORT_EXPORT_REF: &str =
    "artifacts/release/m5-trust-component-accessibility-parity-proof/support_export.json";

/// Published matrix CSV artifact ref.
pub const M5_TRUST_COMPONENT_PARITY_PUBLISHED_CSV_REF: &str =
    "artifacts/release/m5-trust-component-accessibility-parity-proof/matrix.csv";

/// Published companion doc ref.
pub const M5_TRUST_COMPONENT_PARITY_PUBLISHED_DOC_REF: &str =
    "docs/shell/m5_trust_component_accessibility_parity_contract.md";

/// Repo-relative ref to the frozen trust-chronology component matrix schema.
pub const M5_TRUST_COMPONENT_PARITY_MATRIX_SCHEMA_REF: &str =
    matrix::M5_TRUST_COMPONENTS_SCHEMA_REF;

/// The six non-visual accessibility routes every certified component must offer. The union across
/// the six frozen components covers the full [`M5TrustAccessibilityRoute::ALL`] set:
/// keyboard-focusable, screen-reader-announced, non-hover-reachable, pointer-optional,
/// high-contrast-safe, and support-exportable.
pub const TRUST_COMPONENT_PARITY_REQUIRED_ROUTES: [M5TrustAccessibilityRoute; 6] =
    M5TrustAccessibilityRoute::ALL;

/// The six labels every certified component must be able to show. The union across the six frozen
/// components covers the full [`M5TrustRequiredLabel::ALL`] set: identity, state, keyboard route,
/// provenance, effective value, and audit/reopen path.
pub const TRUST_COMPONENT_PARITY_REQUIRED_LABELS: [M5TrustRequiredLabel; 6] =
    M5TrustRequiredLabel::ALL;

/// One of the claimed non-default accessibility conditions the parity proof must cover, in
/// canonical order. Each condition is a claimed M5 accessibility mode under which the settings row,
/// capability sheet, event row, timeline group, narrative card, and export preview must stay
/// reachable, legible, and diagnosable; the lane certifies none beyond them and refuses to ship if
/// any is missing. These are exactly the fixture-coverage cases the acceptance criteria require:
/// keyboard reach, focus order, narration, high-zoom, high-contrast, reduced-motion, and
/// compact/comfortable density.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustAccessibilityCondition {
    /// Keyboard-only reach: every component is operable without a pointer.
    KeyboardReach,
    /// Focus order: focus lands and returns in the right order after open / dismiss.
    FocusOrder,
    /// Screen-reader narration and live announcements: every component is announced.
    ScreenReaderNarration,
    /// High-zoom / large-text rendering.
    HighZoom,
    /// High-contrast rendering.
    HighContrast,
    /// Reduced-motion rendering.
    ReducedMotion,
    /// Compact / comfortable density compaction (dense enterprise usage).
    DensityCompaction,
}

impl M5TrustAccessibilityCondition {
    /// Every governed accessibility condition, in canonical order.
    pub const ALL: [Self; 7] = [
        Self::KeyboardReach,
        Self::FocusOrder,
        Self::ScreenReaderNarration,
        Self::HighZoom,
        Self::HighContrast,
        Self::ReducedMotion,
        Self::DensityCompaction,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyboardReach => "keyboard_reach",
            Self::FocusOrder => "focus_order",
            Self::ScreenReaderNarration => "screen_reader_narration",
            Self::HighZoom => "high_zoom",
            Self::HighContrast => "high_contrast",
            Self::ReducedMotion => "reduced_motion",
            Self::DensityCompaction => "density_compaction",
        }
    }

    /// Short, reviewer-facing label for the condition.
    pub const fn label(self) -> &'static str {
        match self {
            Self::KeyboardReach => "Keyboard-only reach",
            Self::FocusOrder => "Focus order after open / dismiss",
            Self::ScreenReaderNarration => "Screen-reader narration & live announcements",
            Self::HighZoom => "High-zoom / large-text rendering",
            Self::HighContrast => "High-contrast rendering",
            Self::ReducedMotion => "Reduced-motion rendering",
            Self::DensityCompaction => "Compact / comfortable density",
        }
    }
}

/// One of the four certification dimensions each accessibility condition is certified across.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5TrustComponentParityProofDimension {
    /// Non-visual reach (keyboard focus, screen-reader narration, focus order).
    NonVisualReach,
    /// Zoom / contrast / density stability (legible and stable under high-zoom, high-contrast, and
    /// compact density; no truth hover-only, color-only, or lost on compaction).
    ZoomContrastDensity,
    /// Motion alternative (durable static text when motion is reduced).
    MotionAlternative,
    /// Support-export parity (the same source, state, and chronology language reconstructs from
    /// fixtures and a support export).
    SupportExportParity,
}

impl M5TrustComponentParityProofDimension {
    /// Every certification dimension, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NonVisualReach,
        Self::ZoomContrastDensity,
        Self::MotionAlternative,
        Self::SupportExportParity,
    ];

    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NonVisualReach => "non_visual_reach",
            Self::ZoomContrastDensity => "zoom_contrast_density",
            Self::MotionAlternative => "motion_alternative",
            Self::SupportExportParity => "support_export_parity",
        }
    }
}

/// The derived certification light a governed accessibility condition carries.
///
/// `green` means the settings rows, capability sheets, event rows, timeline groups, and export
/// previews stay keyboard- and screen-reader-reachable with focus that returns in order, stay
/// legible and stable under high-zoom / high-contrast / compact density, keep durable static text
/// alternatives, and reconstruct their source/state/chronology language from fixtures and a support
/// export. `yellow` is a disclosed narrowing (a reduced non-visual reach detail, a reduced
/// zoom/contrast/density detail, a waivered reduced motion alternative, or a partial support-export
/// capture). `red` is blocked: a component's truth is reachable only by pointer or hover, is
/// truncated / color-only / lost when the surface compacts, is conveyed by motion only, is absent
/// from the support-export capture, keeps critical truth hover-only / color-only / compaction-lost,
/// or its accessibility routes / required labels are incomplete — and the condition may not keep a
/// trust-component-maturity claim until repaired.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrustComponentParityStatus {
    /// Full standing: reachable, legible, motion-safe, reconstructable.
    Green,
    /// The claim is honestly narrowed and the narrowing is disclosed.
    Yellow,
    /// The claim is blocked and may not be published until repaired.
    Red,
}

impl TrustComponentParityStatus {
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

/// How the condition keeps every component keyboard- and screen-reader-reachable, with focus that
/// lands and returns in order and no truth left behind pointer hover.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NonVisualReachState {
    /// Every component is keyboard-focusable, announced to a screen reader, and returns focus in
    /// order after open / dismiss; hover-dependent detail (a source pill explainer, a transitive
    /// scope popover, a chronology detail) has a keyboard/focus alternative and live announcements
    /// fire where the surface changes dynamically.
    KeyboardFocusAndNarrationReachable,
    /// Under this condition a non-visual reach detail is disclosedly reduced (a longer narration
    /// abbreviates, or a focus-return lands one level up) while every component stays keyboard- and
    /// screen-reader-reachable and the reduction is disclosed.
    DisclosedReducedReachDetail,
    /// A component's truth is reachable only by pointer or hover — no keyboard focus, no
    /// screen-reader narration, or focus does not return in order after dismiss — always a blocker.
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

    /// `true` when every component stays keyboard/screen-reader reachable.
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

/// How the condition keeps every component legible and stable under high-zoom, high-contrast, and
/// compact/comfortable density, with no truth left hover-only, color-only, or dropped on compaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ZoomContrastDensityState {
    /// Every component stays legible and keeps a stable placement and meaning under high-zoom,
    /// high-contrast, and compact density; source pills, lock states, consequence groups, verbs,
    /// provenance badges, and export fields keep a non-color-only affordance and nothing truncates,
    /// clips, or reflows away a truth-bearing item when the surface compacts.
    LegibleStableUnderZoomContrastDensity,
    /// Under this condition a zoom/contrast/density detail is disclosedly reduced (a label wraps to
    /// a shorter form, or a decorative accent drops in compact density) while every component stays
    /// legible, keeps a non-color-only affordance, and the reduction is disclosed.
    DisclosedReducedZoomContrastDensityDetail,
    /// A component is truncated, conveyed by color only, or lost when the surface compacts under
    /// high-zoom / high-contrast / compact density, so a truth-bearing item is illegible or
    /// dropped — always a blocker.
    TruncatedColorOnlyOrLostOnCompaction,
}

impl ZoomContrastDensityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LegibleStableUnderZoomContrastDensity => {
                "legible_stable_under_zoom_contrast_density"
            }
            Self::DisclosedReducedZoomContrastDensityDetail => {
                "disclosed_reduced_zoom_contrast_density_detail"
            }
            Self::TruncatedColorOnlyOrLostOnCompaction => {
                "truncated_color_only_or_lost_on_compaction"
            }
        }
    }

    /// `true` when every component stays legible and stable.
    pub const fn is_stable(self) -> bool {
        matches!(self, Self::LegibleStableUnderZoomContrastDensity)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedZoomContrastDensityDetail)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::TruncatedColorOnlyOrLostOnCompaction)
    }
}

/// How the condition keeps a durable static text alternative for anything a component would
/// otherwise convey by motion (a live-updating progress badge, an animated state transition).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MotionAlternativeState {
    /// Every component that would animate a state change or a live update keeps a durable static
    /// text alternative when motion is reduced (a labelled state instead of an animated transition,
    /// a text count instead of a pulsing badge).
    DurableTextAlternativePresent,
    /// Under this condition an alternative is disclosedly reduced (a summarized text alternative for
    /// a small set of high-frequency updates) while a durable text path stays present and the
    /// reduction is disclosed and waivered.
    DisclosedReducedAlternativeDetail,
    /// Critical state or a live update is conveyed by motion only, with no durable static text
    /// alternative when motion is suppressed — always a blocker.
    MotionOnlyAffordance,
}

impl MotionAlternativeState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DurableTextAlternativePresent => "durable_text_alternative_present",
            Self::DisclosedReducedAlternativeDetail => "disclosed_reduced_alternative_detail",
            Self::MotionOnlyAffordance => "motion_only_affordance",
        }
    }

    /// `true` when a durable static text alternative is present.
    pub const fn is_present(self) -> bool {
        matches!(self, Self::DurableTextAlternativePresent)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedReducedAlternativeDetail)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::MotionOnlyAffordance)
    }
}

/// How the condition's component source, state, and chronology language survives copied evidence,
/// saved packets, and issue/report flows — reconstructable from reusable fixtures and a support
/// export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SupportExportParityState {
    /// The support export and the reusable fixtures reconstruct the condition's component truth —
    /// the settings source and lock state, the capability consequence and scope, the chronology
    /// verb / provenance / detail, and the export fields — in the same language shown in-product, so
    /// a component accessibility regression can be diagnosed without a live screenshot.
    ComponentTruthReconstructable,
    /// The support export reconstructs the component truth and discloses a partial capture (some
    /// low-priority component detail is trimmed) while the reduction is disclosed.
    DisclosedPartialCapture,
    /// The condition's component state is absent from the support-export capture, so a component
    /// accessibility regression cannot be explained without a live screenshot — always a blocker.
    ComponentStateAbsentFromCapture,
}

impl SupportExportParityState {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ComponentTruthReconstructable => "component_truth_reconstructable",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::ComponentStateAbsentFromCapture => "component_state_absent_from_capture",
        }
    }

    /// `true` when the fixtures and export reconstruct the component truth.
    pub const fn is_reconstructable(self) -> bool {
        matches!(self, Self::ComponentTruthReconstructable)
    }

    /// `true` when the state is the disclosed narrowing.
    pub const fn is_disclosed(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// `true` when the state is a hard blocker.
    pub const fn is_blocked(self) -> bool {
        matches!(self, Self::ComponentStateAbsentFromCapture)
    }
}

/// A disclosed, time-bounded exception that lets a would-be-red reduced motion alternative stay
/// yellow rather than blocked — never lets a pointer/hover-only truth, an illegible
/// zoom/contrast/density surface, a motion-only affordance, or a missing export hide.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustComponentParityWaiver {
    /// Stable waiver id quoted in the packet and dashboard.
    pub waiver_id: String,
    /// The governed accessibility condition the waiver applies to.
    pub condition: M5TrustAccessibilityCondition,
    /// Why the narrowing is acceptable; always disclosed, never hidden.
    pub reason: String,
    /// Owner role accountable for retiring the waiver.
    pub owner_role: String,
    /// RFC 3339 expiry. After this the waiver is no longer active and the row blocks.
    pub expires_at: String,
}

impl TrustComponentParityWaiver {
    /// `true` when the waiver is still active at `as_of` (RFC 3339, UTC).
    pub fn is_active_at(&self, as_of: &str) -> bool {
        // RFC 3339 UTC timestamps sort lexicographically by instant.
        self.expires_at.as_str() > as_of
    }
}

/// One exact cause that narrowed or blocked a governed condition's certification.
///
/// The trigger token mirrors the frozen [`M5TrustComponentDowngradeTrigger`] vocabulary so a cause
/// never mints a parallel reason synonym. Because this lane spans every component family, its
/// accessibility failures are recorded against the two family-agnostic triggers the matrix freezes:
/// `AuditTruthLostOffPrimarySurface` (a reachable, legible, or motion-safe truth is lost off the
/// primary surface) and `ProofStale` (the support export can no longer reconstruct the truth).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustComponentParityCause {
    /// The governed accessibility condition the cause applies to.
    pub condition: M5TrustAccessibilityCondition,
    /// The frozen downgrade trigger that fired.
    pub trigger: M5TrustComponentDowngradeTrigger,
    /// `true` when the cause is disclosed (and, where required, waivered); a non-disclosed cause is
    /// a blocker.
    pub disclosed: bool,
    /// Short reviewer-facing detail for the cause.
    pub detail: String,
}

impl TrustComponentParityCause {
    /// Stable trigger token for the cause.
    pub fn cause_token(&self) -> &'static str {
        self.trigger.as_str()
    }
}

/// One governed accessibility condition, certified across non-visual reach, zoom/contrast/density
/// stability, motion alternatives, and support-export parity.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustComponentParityRow {
    /// The governed accessibility condition being certified.
    pub condition: M5TrustAccessibilityCondition,
    /// The components this condition certifies (all six frozen trust components). Pulled from the
    /// matrix.
    pub driven_component_families: Vec<M5TrustComponentFamily>,
    /// The frozen qualification class across the six components (the most-narrowed). Pulled from
    /// the matrix.
    pub matrix_qualification: M5TrustQualificationClass,
    /// Owner role accountable for keeping this condition certified.
    pub owner_role: String,
    /// Short condition label.
    pub condition_label: String,
    /// Shell zones the certified components attach to (union across the six). Pulled from the
    /// matrix.
    pub certified_shell_zone_slots: Vec<M5ShellZoneSlot>,
    /// Responsive classes the certified components survive (union). Pulled from the matrix.
    pub certified_responsive_classes: Vec<M5ResponsiveClass>,
    /// Window classes the certified components keep continuity across (union). Pulled from the
    /// matrix.
    pub certified_window_classes: Vec<M5WindowClass>,
    /// Claimed M5 surface families that render the certified components (union). Pulled from the
    /// matrix.
    pub certified_surface_families: Vec<M5ShellSurfaceFamily>,
    /// Settings-row states the certified settings row projects (union). Pulled from the matrix.
    pub certified_settings_row_states: Vec<M5SettingsRowState>,
    /// Source pills the certified settings row shows (union). Pulled from the matrix.
    pub certified_source_pills: Vec<M5SettingSourcePill>,
    /// Capability consequence classes the certified capability sheet groups by (union). Pulled from
    /// the matrix.
    pub certified_consequence_classes: Vec<M5CapabilityConsequenceClass>,
    /// Capability scope states the certified capability sheet honours (union). Pulled from the
    /// matrix.
    pub certified_capability_scope_states: Vec<M5CapabilityScopeState>,
    /// Chronology verbs the certified chronology components use (union). Pulled from the matrix.
    pub certified_chronology_verbs: Vec<M5ChronologyVerb>,
    /// Provenance badges the certified chronology components attribute (union). Pulled from the
    /// matrix.
    pub certified_provenance_badges: Vec<M5ProvenanceBadge>,
    /// Chronology detail states the certified grouping components honour (union). Pulled from the
    /// matrix.
    pub certified_chronology_detail_states: Vec<M5ChronologyDetailState>,
    /// Chronology export fields the certified export preview promises (union). Pulled from the
    /// matrix.
    pub certified_chronology_export_fields: Vec<M5ChronologyExportField>,
    /// Non-visual accessibility routes (union). Pulled from the matrix.
    pub accessibility_routes: Vec<M5TrustAccessibilityRoute>,
    /// Mandatory labels every certified component must be able to show (union). Pulled from the
    /// matrix.
    pub required_labels: Vec<M5TrustRequiredLabel>,
    /// Shell subsystems this condition stays aligned across (union). Pulled from the matrix.
    pub consumer_surfaces: Vec<M5ShellConsumerSurface>,
    /// Downgrade triggers that apply to this condition (union). Pulled from the matrix.
    pub applicable_downgrade_triggers: Vec<M5TrustComponentDowngradeTrigger>,
    /// Non-visual-reach posture.
    pub non_visual_reach: NonVisualReachState,
    /// Zoom/contrast/density-stability posture.
    pub zoom_contrast_density: ZoomContrastDensityState,
    /// Motion-alternative posture.
    pub motion_alternative: MotionAlternativeState,
    /// Support-export-parity posture.
    pub support_export_parity: SupportExportParityState,
    /// Hard invariant: no critical truth is kept hover-only, color-only, or lost on compaction.
    /// `false` is a blocker.
    pub never_hover_color_only_or_compaction_lost: bool,
    /// Active waiver, when a disclosed reduced motion alternative is in force.
    pub active_waiver: Option<TrustComponentParityWaiver>,
    /// Derived green/yellow/red status. Recomputed by the builder; never asserted.
    pub derived_status: TrustComponentParityStatus,
    /// The exact certification causes that narrowed or blocked this row.
    pub certification_causes: Vec<TrustComponentParityCause>,
    /// Required whenever the derived status is not green.
    pub narrowing_reason: Option<String>,
}

impl TrustComponentParityRow {
    /// `true` when an active waiver is attached.
    pub fn has_active_waiver(&self) -> bool {
        self.active_waiver.is_some()
    }

    /// `true` when every non-visual accessibility route the matrix freezes is certified — the lint
    /// that prevents a condition from shipping without keyboard-focusable, screen-reader-announced,
    /// non-hover-reachable, pointer-optional, high-contrast-safe, and support-exportable routes on
    /// the certified components.
    pub fn accessibility_routes_complete(&self) -> bool {
        let present: BTreeSet<M5TrustAccessibilityRoute> =
            self.accessibility_routes.iter().copied().collect();
        TRUST_COMPONENT_PARITY_REQUIRED_ROUTES
            .iter()
            .all(|route| present.contains(route))
    }

    /// `true` when every required label the matrix freezes is certified — the lint that prevents a
    /// condition from shipping without identity, state, keyboard-route, provenance, effective-value,
    /// and audit/reopen-path labels on the certified components.
    pub fn required_labels_complete(&self) -> bool {
        let present: BTreeSet<M5TrustRequiredLabel> =
            self.required_labels.iter().copied().collect();
        TRUST_COMPONENT_PARITY_REQUIRED_LABELS
            .iter()
            .all(|label| present.contains(label))
    }

    /// `true` when the row has a hard blocker that no waiver may mask.
    fn has_hard_blocker(&self) -> bool {
        self.non_visual_reach.is_blocked()
            || self.zoom_contrast_density.is_blocked()
            || self.motion_alternative.is_blocked()
            || self.support_export_parity.is_blocked()
            || !self.never_hover_color_only_or_compaction_lost
            || !self.accessibility_routes_complete()
            || !self.required_labels_complete()
    }

    /// `true` when the row is honestly narrowed (yellow rather than green).
    fn has_narrowing(&self) -> bool {
        self.non_visual_reach.is_disclosed()
            || self.zoom_contrast_density.is_disclosed()
            || self.motion_alternative.is_disclosed()
            || self.support_export_parity.is_disclosed()
    }

    /// Recomputes the derived status from the four axes and the hover/color/compaction invariant.
    ///
    /// This is the auto-narrowing rule: any hard blocker forces `red`, any honest narrowing forces
    /// `yellow`, otherwise `green`.
    pub fn recompute_status(&self) -> TrustComponentParityStatus {
        if self.has_hard_blocker() {
            TrustComponentParityStatus::Red
        } else if self.has_narrowing() {
            TrustComponentParityStatus::Yellow
        } else {
            TrustComponentParityStatus::Green
        }
    }

    /// Recomputes the exact certification causes for the row, in deterministic order (non-visual
    /// reach, zoom/contrast/density, motion, support-export, hover/color/compaction invariant).
    pub fn recompute_causes(&self) -> Vec<TrustComponentParityCause> {
        let mut causes = Vec::new();
        if !self.non_visual_reach.is_reachable() {
            causes.push(TrustComponentParityCause {
                condition: self.condition,
                trigger: M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface,
                disclosed: self.non_visual_reach.is_disclosed(),
                detail: if self.non_visual_reach.is_disclosed() {
                    "Under this condition a non-visual reach detail is disclosedly reduced (a longer \
                     narration abbreviates, or a focus-return lands one level up) while every \
                     component stays keyboard- and screen-reader-reachable; the reduction is \
                     disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "A component's truth is reachable only by pointer or hover — no keyboard focus, \
                     no screen-reader narration, or focus does not return in order after dismiss."
                        .to_owned()
                },
            });
        }
        if !self.zoom_contrast_density.is_stable() {
            causes.push(TrustComponentParityCause {
                condition: self.condition,
                trigger: M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface,
                disclosed: self.zoom_contrast_density.is_disclosed(),
                detail: if self.zoom_contrast_density.is_disclosed() {
                    "Under this condition a zoom/contrast/density detail is disclosedly reduced (a \
                     label wraps to a shorter form, or a decorative accent drops in compact density) \
                     while every component stays legible and keeps a non-color-only affordance; the \
                     reduction is disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "A component is truncated, conveyed by color only, or lost when the surface \
                     compacts under high-zoom / high-contrast / compact density, so a truth-bearing \
                     item is illegible or dropped."
                        .to_owned()
                },
            });
        }
        if !self.motion_alternative.is_present() {
            causes.push(TrustComponentParityCause {
                condition: self.condition,
                trigger: M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface,
                disclosed: self.motion_alternative.is_disclosed(),
                detail: if self.motion_alternative.is_disclosed() {
                    "Under this condition a motion alternative is disclosedly reduced (a summarized \
                     text alternative for a small set of high-frequency updates) while a durable \
                     text path stays present; the reduction is disclosed and waivered, and the row \
                     is narrowed below green."
                        .to_owned()
                } else {
                    "Critical state or a live update is conveyed by motion only, with no durable \
                     static text alternative when motion is suppressed."
                        .to_owned()
                },
            });
        }
        if !self.support_export_parity.is_reconstructable() {
            causes.push(TrustComponentParityCause {
                condition: self.condition,
                trigger: M5TrustComponentDowngradeTrigger::ProofStale,
                disclosed: self.support_export_parity.is_disclosed(),
                detail: if self.support_export_parity.is_disclosed() {
                    "The support export reconstructs the component truth and discloses a partial \
                     capture (some low-priority component detail is trimmed) while the reduction is \
                     disclosed and the row is narrowed below green."
                        .to_owned()
                } else {
                    "The condition's component state is absent from the support-export capture, so a \
                     component accessibility regression cannot be explained without a live \
                     screenshot."
                        .to_owned()
                },
            });
        }
        if !self.never_hover_color_only_or_compaction_lost {
            causes.push(TrustComponentParityCause {
                condition: self.condition,
                trigger: M5TrustComponentDowngradeTrigger::AuditTruthLostOffPrimarySurface,
                disclosed: false,
                detail: "A component keeps a critical setting, capability, or chronology truth \
                         visible only through a pointer hover, conveyed by color alone, or dropped \
                         when the surface compacts, with no keyboard/focus or durable text \
                         alternative."
                    .to_owned(),
            });
        }
        causes
    }

    /// `true` when the row's narrowing requires an active waiver to stay publishable.
    ///
    /// A disclosed reduced motion alternative may only stay yellow (rather than red) when a waiver
    /// discloses it.
    pub fn requires_waiver(&self) -> bool {
        self.motion_alternative.is_disclosed()
    }

    fn has_reason(&self) -> bool {
        self.narrowing_reason
            .as_deref()
            .map(str::trim)
            .map(|reason| !reason.is_empty())
            .unwrap_or(false)
    }

    fn compute_findings(&self, as_of: &str) -> Vec<TrustComponentParityFinding> {
        let mut findings = Vec::new();
        let condition = self.condition.as_str().to_owned();

        if self.non_visual_reach.is_blocked() {
            findings.push(TrustComponentParityFinding::ReachPointerOrHoverOnly {
                condition: condition.clone(),
            });
        }
        if self.zoom_contrast_density.is_blocked() {
            findings.push(TrustComponentParityFinding::ZoomContrastDensityUnreadable {
                condition: condition.clone(),
            });
        }
        if self.motion_alternative.is_blocked() {
            findings.push(TrustComponentParityFinding::MotionOnlyAffordance {
                condition: condition.clone(),
            });
        }
        if self.support_export_parity.is_blocked() {
            findings.push(
                TrustComponentParityFinding::ComponentStateAbsentFromCapture {
                    condition: condition.clone(),
                },
            );
        }
        if !self.never_hover_color_only_or_compaction_lost {
            findings.push(
                TrustComponentParityFinding::CriticalTruthHoverColorOnlyOrCompactionLost {
                    condition: condition.clone(),
                },
            );
        }
        if !self.accessibility_routes_complete() {
            findings.push(TrustComponentParityFinding::AccessibilityRoutesIncomplete {
                condition: condition.clone(),
            });
        }
        if !self.required_labels_complete() {
            findings.push(TrustComponentParityFinding::RequiredLabelsIncomplete {
                condition: condition.clone(),
            });
        }

        // A narrowed/blocked row must disclose why.
        let derived = self.recompute_status();
        if !matches!(derived, TrustComponentParityStatus::Green) && !self.has_reason() {
            findings.push(TrustComponentParityFinding::NarrowedRowWithoutReason {
                condition: condition.clone(),
            });
        }
        // A waiver-requiring narrowing that is not already a hard blocker must carry an active
        // waiver.
        if self.requires_waiver() && !self.has_hard_blocker() && !self.has_active_waiver() {
            findings.push(TrustComponentParityFinding::NarrowedRowWithoutWaiver {
                condition: condition.clone(),
            });
        }
        // An attached waiver must still be active and must point at this condition.
        if let Some(waiver) = &self.active_waiver {
            if waiver.condition != self.condition {
                findings.push(TrustComponentParityFinding::WaiverConditionMismatch {
                    condition: condition.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
            if !waiver.is_active_at(as_of) {
                findings.push(TrustComponentParityFinding::WaiverExpired {
                    condition: condition.clone(),
                    waiver_id: waiver.waiver_id.clone(),
                });
            }
        }
        // The declared derived fields must match the recomputed ones.
        if self.derived_status != derived {
            findings.push(TrustComponentParityFinding::RowStatusStale {
                condition: condition.clone(),
            });
        }
        if self.certification_causes != self.recompute_causes() {
            findings.push(TrustComponentParityFinding::RowCausesStale { condition });
        }

        findings
    }

    fn compact_line(&self) -> String {
        format!(
            "  {} status={} qual={} reach={} zoom_contrast_density={} motion={} export={} no_hover_color_only_or_compaction_lost={} waiver={}",
            self.condition.as_str(),
            self.derived_status.as_str(),
            self.matrix_qualification.as_str(),
            self.non_visual_reach.as_str(),
            self.zoom_contrast_density.as_str(),
            self.motion_alternative.as_str(),
            self.support_export_parity.as_str(),
            self.never_hover_color_only_or_compaction_lost,
            self.active_waiver
                .as_ref()
                .map(|w| w.waiver_id.as_str())
                .unwrap_or("none"),
        )
    }
}

/// A blocking finding the component accessibility parity proof refuses to ship with.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum TrustComponentParityFinding {
    /// A governed accessibility condition has no certification row.
    ConditionMissing {
        /// The missing condition token.
        condition: String,
    },
    /// A condition keeps a component's truth reachable only by pointer or hover.
    ReachPointerOrHoverOnly {
        /// The condition token.
        condition: String,
    },
    /// A condition has a component truncated, color-only, or lost under high-zoom / high-contrast /
    /// compact density.
    ZoomContrastDensityUnreadable {
        /// The condition token.
        condition: String,
    },
    /// A condition conveys critical state by motion only.
    MotionOnlyAffordance {
        /// The condition token.
        condition: String,
    },
    /// A condition's component state is absent from the support-export capture.
    ComponentStateAbsentFromCapture {
        /// The condition token.
        condition: String,
    },
    /// A condition keeps a critical truth hover-only, color-only, or compaction-lost (hard
    /// invariant).
    CriticalTruthHoverColorOnlyOrCompactionLost {
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

impl TrustComponentParityFinding {
    /// Stable class token for the finding.
    pub const fn class_token(&self) -> &'static str {
        match self {
            Self::ConditionMissing { .. } => "condition_missing",
            Self::ReachPointerOrHoverOnly { .. } => "reach_pointer_or_hover_only",
            Self::ZoomContrastDensityUnreadable { .. } => "zoom_contrast_density_unreadable",
            Self::MotionOnlyAffordance { .. } => "motion_only_affordance",
            Self::ComponentStateAbsentFromCapture { .. } => "component_state_absent_from_capture",
            Self::CriticalTruthHoverColorOnlyOrCompactionLost { .. } => {
                "critical_truth_hover_color_only_or_compaction_lost"
            }
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
            | Self::ZoomContrastDensityUnreadable { condition }
            | Self::MotionOnlyAffordance { condition }
            | Self::ComponentStateAbsentFromCapture { condition }
            | Self::CriticalTruthHoverColorOnlyOrCompactionLost { condition }
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
pub struct TrustComponentParityPacket {
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
    /// The frozen trust-chronology component matrix packet id this proof certifies.
    pub matrix_packet_ref: String,
    /// Repo-relative ref to the frozen trust-chronology component matrix schema.
    pub matrix_schema_ref: String,
    /// Exact-build identity ref the packet was generated against.
    pub build_identity_ref: String,
    /// Release-channel class the build was produced for.
    pub release_channel_class: String,
    /// The certification dimensions every condition is certified across.
    pub required_proof_dimensions: Vec<M5TrustComponentParityProofDimension>,
    /// The accessibility routes every condition must certify.
    pub required_accessibility_routes: Vec<M5TrustAccessibilityRoute>,
    /// The required labels every condition must certify.
    pub required_labels: Vec<M5TrustRequiredLabel>,
    /// Per-condition certification rows, in canonical order.
    pub rows: Vec<TrustComponentParityRow>,
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
    pub active_waivers: Vec<TrustComponentParityWaiver>,
    /// Every exact certification cause, in row then cause order.
    pub certification_causes: Vec<TrustComponentParityCause>,
    /// Every blocking finding, sorted by class then subject.
    pub blocking_findings: Vec<TrustComponentParityFinding>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Shell / release automation refs that consume this packet to auto-narrow claimed conditions.
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

impl TrustComponentParityPacket {
    /// Returns the certification row for `condition`, if present.
    pub fn row(
        &self,
        condition: M5TrustAccessibilityCondition,
    ) -> Option<&TrustComponentParityRow> {
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
    pub fn dashboard(&self) -> TrustComponentParityDashboard {
        TrustComponentParityDashboard::from_packet(self)
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("m5 trust-component-parity packet serializes")
    }

    /// Deterministic, machine-readable certification CSV: one row per condition.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::new();
        out.push_str(
            "condition,status,qualification,non_visual_reach,zoom_contrast_density,motion_alternative,support_export_parity,never_hover_color_only_or_compaction_lost,accessibility_routes,waiver\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{},{}\n",
                row.condition.as_str(),
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.non_visual_reach.as_str(),
                row.zoom_contrast_density.as_str(),
                row.motion_alternative.as_str(),
                row.support_export_parity.as_str(),
                row.never_hover_color_only_or_compaction_lost,
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
        out.push_str("# M5 trust-component accessibility parity\n\n");
        out.push_str(
            "Generated from the seeded packet in\n\
             [`crate::m5_trust_component_accessibility_parity`](../../crates/aureline-shell/src/m5_trust_component_accessibility_parity/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_component_accessibility_parity -- markdown > \\\n  artifacts/shell/m5-trust-component-accessibility-parity.md\n",
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
            "| Condition | Status | Qualification | Reach | Zoom/contrast/density | Motion | Support-export | No-hover/color-only/compaction-lost | Waiver |\n\
             | --------- | ------ | ------------- | ----- | --------------------- | ------ | -------------- | ----------------------------------- | ------ |\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                row.condition_label,
                row.derived_status.as_str(),
                row.matrix_qualification.as_str(),
                row.non_visual_reach.as_str(),
                row.zoom_contrast_density.as_str(),
                row.motion_alternative.as_str(),
                row.support_export_parity.as_str(),
                row.never_hover_color_only_or_compaction_lost,
                row.active_waiver
                    .as_ref()
                    .map(|w| format!("`{}`", w.waiver_id))
                    .unwrap_or_else(|| "—".to_owned()),
            ));
        }
        out.push('\n');

        out.push_str("## Auto-narrowed rows\n\n");
        let narrowed: Vec<&TrustComponentParityRow> = self
            .rows
            .iter()
            .filter(|row| !matches!(row.derived_status, TrustComponentParityStatus::Green))
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
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_trust_component_accessibility_parity -- validate\n",
        );
        out.push_str(
            "cargo test -p aureline-shell --test m5_trust_component_accessibility_parity_fixtures\n",
        );
        out.push_str("```\n");
        out
    }
}

/// One row of the light certification dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustComponentParityDashboardRow {
    /// The governed condition.
    pub condition: M5TrustAccessibilityCondition,
    /// Short condition label.
    pub condition_label: String,
    /// Derived green/yellow/red status.
    pub status: TrustComponentParityStatus,
    /// Frozen qualification class.
    pub matrix_qualification: M5TrustQualificationClass,
    /// Non-visual-reach posture.
    pub non_visual_reach: NonVisualReachState,
    /// Zoom/contrast/density-stability posture.
    pub zoom_contrast_density: ZoomContrastDensityState,
    /// Motion-alternative posture.
    pub motion_alternative: MotionAlternativeState,
    /// Support-export-parity posture.
    pub support_export_parity: SupportExportParityState,
    /// `true` when no critical truth is hover-only, color-only, or compaction-lost.
    pub never_hover_color_only_or_compaction_lost: bool,
    /// `true` when an active waiver is attached.
    pub has_active_waiver: bool,
    /// Active waiver id, when attached.
    pub waiver_id: Option<String>,
    /// Cause trigger tokens that narrowed/blocked this row.
    pub cause_tokens: Vec<String>,
    /// Disclosed narrowing reason, when not green.
    pub narrowing_reason: Option<String>,
}

/// The light certification dashboard the shell / accessibility bridge / release automation reads to
/// auto-narrow claimed accessibility conditions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustComponentParityDashboard {
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
    pub rows: Vec<TrustComponentParityDashboardRow>,
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

impl TrustComponentParityDashboard {
    /// Projects the dashboard from a certification packet.
    pub fn from_packet(packet: &TrustComponentParityPacket) -> Self {
        let rows = packet
            .rows
            .iter()
            .map(|row| TrustComponentParityDashboardRow {
                condition: row.condition,
                condition_label: row.condition_label.clone(),
                status: row.derived_status,
                matrix_qualification: row.matrix_qualification,
                non_visual_reach: row.non_visual_reach,
                zoom_contrast_density: row.zoom_contrast_density,
                motion_alternative: row.motion_alternative,
                support_export_parity: row.support_export_parity,
                never_hover_color_only_or_compaction_lost: row
                    .never_hover_color_only_or_compaction_lost,
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
            record_kind: M5_TRUST_COMPONENT_PARITY_DASHBOARD_RECORD_KIND.to_owned(),
            schema_version: M5_TRUST_COMPONENT_PARITY_SCHEMA_VERSION,
            dashboard_id: M5_TRUST_COMPONENT_PARITY_DASHBOARD_ID.to_owned(),
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
        serde_json::to_string_pretty(self).expect("m5 trust-component-parity dashboard serializes")
    }
}

/// Support-export wrapper for the component accessibility parity certification packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrustComponentParitySupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Packet quoted in full.
    pub packet: TrustComponentParityPacket,
    /// Dashboard quoted in full.
    pub dashboard: TrustComponentParityDashboard,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl TrustComponentParitySupportExport {
    /// Builds the support-export wrapper for a packet.
    ///
    /// The packet id, the matrix packet ref, the exact-build ref, each condition, and each active
    /// waiver id is quoted as a case id so a support reviewer — or the shell automation — can name
    /// the same condition and waiver the runtime certified.
    pub fn from_packet(
        support_export_id: impl Into<String>,
        packet: TrustComponentParityPacket,
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
            record_kind: M5_TRUST_COMPONENT_PARITY_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_TRUST_COMPONENT_PARITY_SCHEMA_VERSION,
            shared_contract_ref: M5_TRUST_COMPONENT_PARITY_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            packet,
            dashboard,
            case_ids,
        }
    }
}

/// Constructor input for [`build_m5_trust_component_accessibility_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustComponentParityInput {
    /// Exact-build identity ref.
    pub build_identity_ref: String,
    /// Release-channel class.
    pub release_channel_class: String,
    /// The frozen trust-chronology component matrix packet id being certified.
    pub matrix_packet_ref: String,
    /// Per-condition certification rows.
    pub rows: Vec<TrustComponentParityRow>,
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

/// Builds a [`TrustComponentParityPacket`] from the exact build identity, the frozen matrix ref,
/// and the per-condition certification rows.
///
/// Each row's derived status and certification causes, the aggregate counts, the active waivers,
/// and the blocking findings are recomputed here so the packet is the single source of truth and
/// the auto-narrowing cannot be asserted.
pub fn build_m5_trust_component_accessibility_parity_packet(
    input: TrustComponentParityInput,
) -> TrustComponentParityPacket {
    let generated_at = input.generated_at;

    // Recompute each row's derived status and causes so the packet is self-consistent and the
    // auto-narrowing is the single source of truth.
    let rows: Vec<TrustComponentParityRow> = input
        .rows
        .into_iter()
        .map(|mut row| {
            row.derived_status = row.recompute_status();
            row.certification_causes = row.recompute_causes();
            row
        })
        .collect();

    let mut blocking_findings: Vec<TrustComponentParityFinding> = Vec::new();

    // Every governed condition must carry a certification row.
    let present: BTreeSet<M5TrustAccessibilityCondition> =
        rows.iter().map(|row| row.condition).collect();
    for condition in M5TrustAccessibilityCondition::ALL {
        if !present.contains(&condition) {
            blocking_findings.push(TrustComponentParityFinding::ConditionMissing {
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
        .filter(|row| matches!(row.derived_status, TrustComponentParityStatus::Green))
        .count();
    let yellow_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, TrustComponentParityStatus::Yellow))
        .count();
    let red_row_count = rows
        .iter()
        .filter(|row| matches!(row.derived_status, TrustComponentParityStatus::Red))
        .count();
    let all_rows_publishable = red_row_count == 0;

    if green_row_count + yellow_row_count + red_row_count != row_count {
        blocking_findings.push(TrustComponentParityFinding::StatusCountsStale);
    }

    let mut active_waivers: Vec<TrustComponentParityWaiver> = rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    active_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));

    let certification_causes: Vec<TrustComponentParityCause> = rows
        .iter()
        .flat_map(|row| row.certification_causes.clone())
        .collect();

    let mut packet = TrustComponentParityPacket {
        record_kind: M5_TRUST_COMPONENT_PARITY_PACKET_RECORD_KIND.to_owned(),
        schema_version: M5_TRUST_COMPONENT_PARITY_SCHEMA_VERSION,
        shared_contract_ref: M5_TRUST_COMPONENT_PARITY_SHARED_CONTRACT_REF.to_owned(),
        packet_id: M5_TRUST_COMPONENT_PARITY_PACKET_ID.to_owned(),
        source_schema_ref: M5_TRUST_COMPONENT_PARITY_SOURCE_SCHEMA_REF.to_owned(),
        headline: "Accessibility parity certified across the M5 settings-row, capability-sheet, and \
                   evidence-chronology reusable trust components for every claimed non-default \
                   condition: keyboard reach, focus order, screen-reader narration, high-zoom, \
                   high-contrast, reduced-motion, and compact/comfortable density each keep the \
                   components keyboard- and screen-reader-reachable with focus that returns in \
                   order, legible and stable under high-zoom / high-contrast / compact density with \
                   no truth hover-only, color-only, or lost on compaction, backed by durable static \
                   text alternatives, and reconstructable from reusable fixtures and a support \
                   export — with each row's green/yellow/red claim auto-narrowed from its \
                   non-visual-reach, zoom-contrast-density, motion-alternative, and \
                   support-export-parity posture."
            .to_owned(),
        matrix_packet_ref: input.matrix_packet_ref,
        matrix_schema_ref: M5_TRUST_COMPONENT_PARITY_MATRIX_SCHEMA_REF.to_owned(),
        build_identity_ref: input.build_identity_ref,
        release_channel_class: input.release_channel_class,
        required_proof_dimensions: M5TrustComponentParityProofDimension::ALL.to_vec(),
        required_accessibility_routes: TRUST_COMPONENT_PARITY_REQUIRED_ROUTES.to_vec(),
        required_labels: TRUST_COMPONENT_PARITY_REQUIRED_LABELS.to_vec(),
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
            "shell_frame.accessibility_bridge.trust_component_parity_registry".to_owned(),
            "release_automation.auto_narrow.trust_component_parity_dashboard".to_owned(),
        ],
        release_center_refs: vec![
            "release_center.trust_component_accessibility_parity".to_owned(),
            "artifacts/release/m5-trust-component-accessibility-parity-proof/packet.json".to_owned(),
        ],
        help_docs_refs: vec![M5_TRUST_COMPONENT_PARITY_PUBLISHED_DOC_REF.to_owned()],
        support_export_refs: vec!["support:m5-trust-component-accessibility-parity".to_owned()],
        published_report_ref: M5_TRUST_COMPONENT_PARITY_PUBLISHED_REPORT_REF.to_owned(),
        published_packet_ref: M5_TRUST_COMPONENT_PARITY_PUBLISHED_PACKET_REF.to_owned(),
        published_dashboard_ref: M5_TRUST_COMPONENT_PARITY_PUBLISHED_DASHBOARD_REF.to_owned(),
        published_doc_ref: M5_TRUST_COMPONENT_PARITY_PUBLISHED_DOC_REF.to_owned(),
        generated_at,
    };

    // Guard the export boundary: no raw URL/path/credential/token may appear.
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(&packet).expect("certification packet serializes"),
    ) {
        blocking_findings.push(TrustComponentParityFinding::RawBoundaryMaterialInExport);
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

/// Validation error produced by [`validate_m5_trust_component_accessibility_parity_packet`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum TrustComponentParityValidationError {
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

/// Validates a packet against the component accessibility parity invariants.
///
/// The checks encode the track invariant and acceptance criteria: every governed accessibility
/// condition carries a current certification row; each row's status is the derived auto-narrowed
/// value, never asserted; a green row cannot keep a claim while a component's truth is reachable
/// only by pointer or hover, is truncated / color-only / lost on compaction, is conveyed by motion
/// only, is dropped from capture, keeps critical truth hover-only / color-only / compaction-lost,
/// or its accessibility routes / required labels are incomplete; and a disclosed narrowing is
/// backed by a reason and, where required, an active waiver.
///
/// # Errors
///
/// Returns the full list of detected invariant violations.
pub fn validate_m5_trust_component_accessibility_parity_packet(
    packet: &TrustComponentParityPacket,
) -> Result<(), Vec<TrustComponentParityValidationError>> {
    let mut errors = Vec::new();

    if packet.rows.is_empty() {
        errors.push(TrustComponentParityValidationError::NoRows);
    }
    if packet.record_kind != M5_TRUST_COMPONENT_PARITY_PACKET_RECORD_KIND {
        errors.push(TrustComponentParityValidationError::WrongRecordKind);
    }
    if packet.schema_version != M5_TRUST_COMPONENT_PARITY_SCHEMA_VERSION {
        errors.push(TrustComponentParityValidationError::WrongSchemaVersion);
    }
    if packet.build_identity_ref.trim().is_empty() {
        errors.push(TrustComponentParityValidationError::BuildIdentityRefMissing);
    }
    if packet.matrix_packet_ref.trim().is_empty() {
        errors.push(TrustComponentParityValidationError::MatrixPacketRefMissing);
    }
    if packet.required_proof_dimensions != M5TrustComponentParityProofDimension::ALL {
        errors.push(TrustComponentParityValidationError::RequiredDimensionsStale);
    }
    if packet.required_accessibility_routes != TRUST_COMPONENT_PARITY_REQUIRED_ROUTES {
        errors.push(TrustComponentParityValidationError::RequiredAccessibilityRoutesStale);
    }
    if packet.required_labels != TRUST_COMPONENT_PARITY_REQUIRED_LABELS {
        errors.push(TrustComponentParityValidationError::RequiredLabelsStale);
    }

    let present: BTreeSet<M5TrustAccessibilityCondition> =
        packet.rows.iter().map(|row| row.condition).collect();
    let coverage_complete = M5TrustAccessibilityCondition::ALL
        .iter()
        .all(|condition| present.contains(condition));
    if !coverage_complete || packet.rows.len() != M5TrustAccessibilityCondition::ALL.len() {
        errors.push(TrustComponentParityValidationError::CoverageIncomplete);
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
        errors.push(TrustComponentParityValidationError::CoverageStale);
    }

    let green = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), TrustComponentParityStatus::Green))
        .count();
    let yellow = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), TrustComponentParityStatus::Yellow))
        .count();
    let red = packet
        .rows
        .iter()
        .filter(|row| matches!(row.recompute_status(), TrustComponentParityStatus::Red))
        .count();
    if packet.row_count != packet.rows.len()
        || packet.green_row_count != green
        || packet.yellow_row_count != yellow
        || packet.red_row_count != red
        || packet.all_rows_publishable != (red == 0)
    {
        errors.push(TrustComponentParityValidationError::StatusCountsStale);
    }

    let mut expected_waivers: Vec<TrustComponentParityWaiver> = packet
        .rows
        .iter()
        .filter_map(|row| row.active_waiver.clone())
        .collect();
    expected_waivers.sort_by(|left, right| left.waiver_id.cmp(&right.waiver_id));
    if expected_waivers != packet.active_waivers {
        errors.push(TrustComponentParityValidationError::ActiveWaiversStale);
    }

    let expected_causes: Vec<TrustComponentParityCause> = packet
        .rows
        .iter()
        .flat_map(|row| row.recompute_causes())
        .collect();
    if expected_causes != packet.certification_causes {
        errors.push(TrustComponentParityValidationError::CertificationCausesStale);
    }

    let mut recomputed: Vec<TrustComponentParityFinding> = Vec::new();
    for condition in M5TrustAccessibilityCondition::ALL {
        if !present.contains(&condition) {
            recomputed.push(TrustComponentParityFinding::ConditionMissing {
                condition: condition.as_str().to_owned(),
            });
        }
    }
    for row in &packet.rows {
        recomputed.extend(row.compute_findings(&packet.generated_at));
    }
    if green + yellow + red != packet.rows.len() {
        recomputed.push(TrustComponentParityFinding::StatusCountsStale);
    }
    if json_contains_forbidden_boundary_material(
        &serde_json::to_value(packet).expect("certification packet serializes"),
    ) {
        recomputed.push(TrustComponentParityFinding::RawBoundaryMaterialInExport);
    }
    recomputed.sort_by(|left, right| {
        left.class_token()
            .cmp(right.class_token())
            .then_with(|| left.subject_ref().cmp(right.subject_ref()))
    });
    if recomputed != packet.blocking_findings {
        errors.push(TrustComponentParityValidationError::BlockingFindingsStale);
    }
    for finding in &packet.blocking_findings {
        errors.push(
            TrustComponentParityValidationError::BlockingFindingPresent {
                class: finding.class_token().to_owned(),
                subject_ref: finding.subject_ref().to_owned(),
            },
        );
    }

    if packet.published_report_ref.trim().is_empty() {
        errors.push(TrustComponentParityValidationError::PublishedReportRefMissing);
    }
    if packet.published_packet_ref.trim().is_empty() {
        errors.push(TrustComponentParityValidationError::PublishedPacketRefMissing);
    }
    if packet.published_dashboard_ref.trim().is_empty() {
        errors.push(TrustComponentParityValidationError::PublishedDashboardRefMissing);
    }
    if packet.published_doc_ref.trim().is_empty() {
        errors.push(TrustComponentParityValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
