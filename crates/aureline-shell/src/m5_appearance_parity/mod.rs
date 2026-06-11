//! Appearance and density qualification audit for the M5 depth surfaces.
//!
//! The M5 depth lanes ship new panes — notebook cell chrome, result-grid
//! rows, profiler and trace panels, pipeline cards, preview-route badges,
//! docs/browser panes, companion surfaces, sync status, and offboarding —
//! that are easy to certify against a single default desktop theme and
//! quietly broken under the other appearance and density rows Aureline
//! already claims. This module carries the stable v1 shell promise forward
//! into those lanes: every marketed M5 surface MUST stay legible,
//! controllable, and semantically consistent across the supported
//! appearance and density rows, and it MUST never let an appearance mode
//! hide trust, severity, or lifecycle state, lose focus visibility, or
//! corrupt layout on a live appearance change.
//!
//! The audit projects, for each registered M5 surface, the canonical
//! appearance descriptor against the qualification result the surface
//! actually certifies for each of the eight appearance rows the M5 lanes
//! must pass:
//!
//! - `theme_dark`
//! - `theme_light`
//! - `theme_high_contrast`
//! - `density_compact`
//! - `density_standard`
//! - `density_comfortable`
//! - `reduced_motion`
//! - `live_appearance_change`
//!
//! The resulting [`M5AppearanceQualificationReport`] is the canonical truth
//! object for the M5 appearance-and-density qualification lane. It is
//! consumed by:
//!
//! - the live shell design-QA inspector (so the in-product audit quotes the
//!   same per-row findings the CLI prints);
//! - the headless inspector (`aureline_shell_m5_appearance_parity`), which
//!   is the only mint-from-truth path for the JSON fixtures checked in under
//!   `fixtures/ux/m5/dark-light-hc-density-reduced-motion/`;
//! - the support-export wrapper that lets a reviewer pivot from a support
//!   case to the row that flagged a stale or red appearance result;
//! - the markdown audit under
//!   `artifacts/ux/m5/appearance-parity/m5_appearance_parity_audit.md`
//!   (rendered from the same seed); and
//! - the cross-surface hardening matrix and release-center packets, which
//!   ingest the audit directly when qualifying or narrowing a marketed M5
//!   row whose appearance evidence is stale or red.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every registered M5 surface must declare a qualification binding for
//!    each of the eight appearance rows.
//! 2. Every surface must carry a canonical appearance anchor, a non-empty
//!    accessibility note, and a flag asserting it rides the shared
//!    appearance-session model; a missing anchor, missing note, or a surface
//!    that paints its own appearance outside the session model is a blocker.
//! 3. A qualified row must carry the captured evidence the row requires —
//!    a screenshot pack, focus visibility, preserved state semantics, and
//!    keyboard/screen-reader checks for every row; a contrast result for the
//!    theme rows; a motion downgrade for `reduced_motion`; and intact layout
//!    for `live_appearance_change`. A red result (contrast below threshold,
//!    focus not visible, state semantics lost, a failed keyboard or
//!    screen-reader check, a lost reopen target, a hidden high-risk boundary
//!    cue, an un-downgraded motion treatment, or a corrupted live change) is
//!    a blocker.
//! 4. A surface that renders an appearance row through ad-hoc local styling
//!    outside the shared appearance-session model
//!    (`unqualified_local_appearance`), and a marketed row claimed with no
//!    captured evidence (`missing_evidence`), are blockers.
//! 5. Stale appearance evidence on a marketed row is a blocker, so release
//!    tooling can narrow a marketed M5 row instead of shipping it as
//!    implicitly stable.
//! 6. At least one surface must qualify each of the eight appearance rows so
//!    the audit cannot regress into a single default-theme view.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under
//! `fixtures/ux/m5/dark-light-hc-density-reduced-motion/` are bit-for-bit
//! equal to the seeded report returned by
//! [`seeded_m5_appearance_qualification_audit`].

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version exported with every M5 appearance-qualification record.
pub const M5_APPEARANCE_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every M5 appearance row.
pub const M5_APPEARANCE_SHARED_CONTRACT_REF: &str = "shell:m5_appearance_parity:v1";

/// Stable record kind for [`M5AppearanceQualificationReport`] payloads.
pub const M5_APPEARANCE_REPORT_RECORD_KIND: &str =
    "shell_m5_appearance_qualification_report_record";

/// Stable record kind for [`M5AppearanceQualificationRow`] payloads.
pub const M5_APPEARANCE_ROW_RECORD_KIND: &str = "shell_m5_appearance_qualification_row_record";

/// Stable record kind for [`M5AppearanceSupportExport`] payloads.
pub const M5_APPEARANCE_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_appearance_qualification_support_export_record";

/// Stable report id quoted across surfaces.
pub const M5_APPEARANCE_REPORT_ID: &str = "shell:m5_appearance_parity:audit:v1";

/// Stable support-export id quoted in the published wrapper.
pub const M5_APPEARANCE_SUPPORT_EXPORT_ID: &str = "support-export:m5-appearance-parity:001";

/// Source schema ref for the canonical appearance-qualification contract.
pub const M5_APPEARANCE_SOURCE_SCHEMA_REF: &str =
    "schemas/ux/m5-appearance-qualification.schema.json";

/// Path of the published markdown audit artifact.
pub const M5_APPEARANCE_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/appearance-parity/m5_appearance_parity_audit.md";

/// Path of the published companion doc.
pub const M5_APPEARANCE_PUBLISHED_DOC_REF: &str = "docs/m5/appearance-and-density-parity.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-11T00:00:00Z";

/// One M5 depth surface whose appearance rows the audit qualifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AppearanceSurfaceFamily {
    /// Notebook cell chrome (gutter, run state, output frame).
    NotebookCellChrome,
    /// Data / API result-grid rows.
    ResultGridRow,
    /// Profiler capture and flame panel.
    ProfilerPanel,
    /// Trace capture and replay panel.
    TracePanel,
    /// Review / pipeline status cards.
    PipelineCard,
    /// Live preview-route badges.
    PreviewRouteBadge,
    /// Embedded docs / browser panes.
    DocsBrowserPane,
    /// Companion / cross-device surfaces.
    CompanionSurface,
    /// Workspace sync status surfaces.
    SyncStatusSurface,
    /// Offboarding / export-and-wipe surfaces.
    OffboardingSurface,
}

impl M5AppearanceSurfaceFamily {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotebookCellChrome => "notebook_cell_chrome",
            Self::ResultGridRow => "result_grid_row",
            Self::ProfilerPanel => "profiler_panel",
            Self::TracePanel => "trace_panel",
            Self::PipelineCard => "pipeline_card",
            Self::PreviewRouteBadge => "preview_route_badge",
            Self::DocsBrowserPane => "docs_browser_pane",
            Self::CompanionSurface => "companion_surface",
            Self::SyncStatusSurface => "sync_status_surface",
            Self::OffboardingSurface => "offboarding_surface",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::NotebookCellChrome => "Notebook cell chrome",
            Self::ResultGridRow => "Result-grid row",
            Self::ProfilerPanel => "Profiler panel",
            Self::TracePanel => "Trace panel",
            Self::PipelineCard => "Pipeline card",
            Self::PreviewRouteBadge => "Preview-route badge",
            Self::DocsBrowserPane => "Docs / browser pane",
            Self::CompanionSurface => "Companion surface",
            Self::SyncStatusSurface => "Sync status surface",
            Self::OffboardingSurface => "Offboarding surface",
        }
    }
}

/// The appearance dimension an appearance row belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AppearanceDimension {
    /// Color theme (dark, light, high-contrast).
    Theme,
    /// Density (compact, standard, comfortable).
    Density,
    /// Motion (reduced-motion downgrade).
    Motion,
    /// Live appearance change applied at runtime.
    LiveChange,
}

impl M5AppearanceDimension {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Theme => "theme",
            Self::Density => "density",
            Self::Motion => "motion",
            Self::LiveChange => "live_change",
        }
    }
}

/// One of the eight appearance rows the audit certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AppearanceRow {
    /// Dark theme.
    ThemeDark,
    /// Light theme.
    ThemeLight,
    /// High-contrast theme.
    ThemeHighContrast,
    /// Compact density.
    DensityCompact,
    /// Standard density.
    DensityStandard,
    /// Comfortable density.
    DensityComfortable,
    /// Reduced-motion preference.
    ReducedMotion,
    /// Live appearance change applied while the surface is mounted.
    LiveAppearanceChange,
}

impl M5AppearanceRow {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThemeDark => "theme_dark",
            Self::ThemeLight => "theme_light",
            Self::ThemeHighContrast => "theme_high_contrast",
            Self::DensityCompact => "density_compact",
            Self::DensityStandard => "density_standard",
            Self::DensityComfortable => "density_comfortable",
            Self::ReducedMotion => "reduced_motion",
            Self::LiveAppearanceChange => "live_appearance_change",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::ThemeDark => "Dark theme",
            Self::ThemeLight => "Light theme",
            Self::ThemeHighContrast => "High-contrast theme",
            Self::DensityCompact => "Compact density",
            Self::DensityStandard => "Standard density",
            Self::DensityComfortable => "Comfortable density",
            Self::ReducedMotion => "Reduced motion",
            Self::LiveAppearanceChange => "Live appearance change",
        }
    }

    /// Returns the eight required appearance rows in canonical order.
    pub const fn required_rows() -> [Self; 8] {
        [
            Self::ThemeDark,
            Self::ThemeLight,
            Self::ThemeHighContrast,
            Self::DensityCompact,
            Self::DensityStandard,
            Self::DensityComfortable,
            Self::ReducedMotion,
            Self::LiveAppearanceChange,
        ]
    }

    /// Canonical appearance dimension this row certifies.
    pub const fn canonical_dimension(self) -> M5AppearanceDimension {
        match self {
            Self::ThemeDark | Self::ThemeLight | Self::ThemeHighContrast => {
                M5AppearanceDimension::Theme
            }
            Self::DensityCompact | Self::DensityStandard | Self::DensityComfortable => {
                M5AppearanceDimension::Density
            }
            Self::ReducedMotion => M5AppearanceDimension::Motion,
            Self::LiveAppearanceChange => M5AppearanceDimension::LiveChange,
        }
    }

    /// `true` for the theme rows, which must carry a contrast result.
    pub const fn requires_contrast(self) -> bool {
        matches!(
            self,
            Self::ThemeDark | Self::ThemeLight | Self::ThemeHighContrast
        )
    }

    /// `true` for the row that must prove a motion downgrade.
    pub const fn requires_motion_treatment(self) -> bool {
        matches!(self, Self::ReducedMotion)
    }

    /// `true` for the row that must prove layout stays intact across a live
    /// appearance change.
    pub const fn requires_layout_integrity(self) -> bool {
        matches!(self, Self::LiveAppearanceChange)
    }
}

/// Qualification status a surface reports for one appearance row.
///
/// Only `Qualified` rows project captured evidence and are drift/red
/// checked. `ExplicitlyNarrowed`, `NotApplicable`, `PlatformOmitted`, and
/// `DeclaredCaptureGap` rows are accepted as long as they carry a
/// `narrowing_reason`. `UnqualifiedLocalAppearance` (ad-hoc styling outside
/// the appearance-session model) and `MissingEvidence` are blocking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5QualificationStatus {
    /// The row is qualified with captured appearance evidence.
    Qualified,
    /// The surface narrows this row; a `narrowing_reason` MUST be set.
    ExplicitlyNarrowed,
    /// The row does not apply to this surface; a reason MUST be set.
    NotApplicable,
    /// The row is not surfaced on this client/platform; a reason MUST be set.
    PlatformOmitted,
    /// An extension/provider surface declares a known capture gap honestly;
    /// a reason MUST be set.
    DeclaredCaptureGap,
    /// The surface renders this row through ad-hoc local styling outside the
    /// shared appearance-session model. Always a blocker.
    UnqualifiedLocalAppearance,
    /// A marketed row is claimed with no captured evidence. Always a blocker.
    MissingEvidence,
}

impl M5QualificationStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::ExplicitlyNarrowed => "explicitly_narrowed",
            Self::NotApplicable => "not_applicable",
            Self::PlatformOmitted => "platform_omitted",
            Self::DeclaredCaptureGap => "declared_capture_gap",
            Self::UnqualifiedLocalAppearance => "unqualified_local_appearance",
            Self::MissingEvidence => "missing_evidence",
        }
    }

    /// `true` for statuses that require a `narrowing_reason`.
    pub const fn requires_narrowing_reason(self) -> bool {
        matches!(
            self,
            Self::ExplicitlyNarrowed
                | Self::NotApplicable
                | Self::PlatformOmitted
                | Self::DeclaredCaptureGap
        )
    }

    /// `true` for the status that projects captured evidence.
    pub const fn projects_evidence(self) -> bool {
        matches!(self, Self::Qualified)
    }
}

/// Contrast result captured for a theme row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ContrastResult {
    /// Meets WCAG AA contrast.
    MeetsAa,
    /// Meets WCAG AAA contrast.
    MeetsAaa,
    /// The surface is not color-dependent for this row.
    NotColorDependent,
    /// Contrast is below the required threshold. Always a blocker.
    BelowThreshold,
}

impl M5ContrastResult {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MeetsAa => "meets_aa",
            Self::MeetsAaa => "meets_aaa",
            Self::NotColorDependent => "not_color_dependent",
            Self::BelowThreshold => "below_threshold",
        }
    }
}

/// Focus-visibility result captured for an appearance row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FocusVisibility {
    /// A visible focus ring is preserved under this row.
    VisibleFocusRing,
    /// Focus is not visible under this row. Always a blocker.
    NotVisible,
}

impl M5FocusVisibility {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::VisibleFocusRing => "visible_focus_ring",
            Self::NotVisible => "not_visible",
        }
    }
}

/// Motion treatment captured for the reduced-motion row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5MotionTreatment {
    /// The surface still animates. A blocker on the reduced-motion row.
    Animated,
    /// Motion is reduced to a minimal transition.
    Reduced,
    /// Motion is removed entirely (a static treatment).
    Static,
}

impl M5MotionTreatment {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Animated => "animated",
            Self::Reduced => "reduced",
            Self::Static => "static",
        }
    }
}

/// Whether the controlled state vocabulary is preserved under a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5StateSemantics {
    /// State meaning (trust, severity, lifecycle) is preserved.
    Preserved,
    /// State meaning is lost or corrupted under this row. Always a blocker.
    Lost,
}

impl M5StateSemantics {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Lost => "lost",
        }
    }
}

/// Outcome of a captured keyboard or screen-reader check.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5CheckOutcome {
    /// The check passed.
    Pass,
    /// The check failed. Always a blocker.
    Fail,
    /// The check did not apply and was not run.
    NotRun,
}

impl M5CheckOutcome {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::NotRun => "not_run",
        }
    }
}

/// Whether the exact-target reopen affordance survives a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReopenAffordance {
    /// The exact-target reopen affordance is preserved.
    ExactTargetPreserved,
    /// The exact-target reopen affordance is lost. Always a blocker.
    Lost,
    /// The surface has no reopen affordance to preserve.
    NotApplicable,
}

impl M5ReopenAffordance {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactTargetPreserved => "exact_target_preserved",
            Self::Lost => "lost",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether a high-risk boundary cue survives a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BoundaryCue {
    /// The high-risk boundary cue is present.
    Present,
    /// The high-risk boundary cue is hidden. Always a blocker.
    Hidden,
    /// The surface carries no high-risk boundary cue.
    NotApplicable,
}

impl M5BoundaryCue {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Present => "present",
            Self::Hidden => "hidden",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether layout stays intact across a live appearance change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LayoutIntegrity {
    /// Layout stays intact across the live appearance change.
    Intact,
    /// Layout is corrupted by the live appearance change. Always a blocker.
    Corrupted,
}

impl M5LayoutIntegrity {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Intact => "intact",
            Self::Corrupted => "corrupted",
        }
    }
}

/// Freshness of the captured appearance evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EvidenceFreshness {
    /// The evidence is current.
    Fresh,
    /// The evidence is stale. A blocker on a marketed row.
    Stale,
}

impl M5EvidenceFreshness {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Fresh => "fresh",
            Self::Stale => "stale",
        }
    }
}

/// How much trust, lifecycle, or severity meaning the surface conveys.
///
/// A surface that conveys lifecycle, trust, or severity meaning is
/// "high-salience": no appearance row may hide that meaning, so the audit
/// requires a present high-risk boundary cue and preserved state semantics
/// on every qualified row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SemanticSalience {
    /// Purely decorative; carries no semantic meaning.
    DecorativeOnly,
    /// Informational only; no trust, lifecycle, or severity meaning.
    Informational,
    /// Conveys lifecycle state (preview, stale, pending).
    LifecycleBearing,
    /// Conveys trust or identity (companion presence, boundary).
    TrustBearing,
    /// Conveys severity or risk (blocked, destructive, failed).
    SeverityBearing,
}

impl M5SemanticSalience {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DecorativeOnly => "decorative_only",
            Self::Informational => "informational",
            Self::LifecycleBearing => "lifecycle_bearing",
            Self::TrustBearing => "trust_bearing",
            Self::SeverityBearing => "severity_bearing",
        }
    }

    /// `true` for salience classes that must never hide their meaning.
    pub const fn is_high_salience(self) -> bool {
        matches!(
            self,
            Self::LifecycleBearing | Self::TrustBearing | Self::SeverityBearing
        )
    }
}

/// Lifecycle label retained on the canonical surface descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5SurfaceLifecycle {
    /// Generally available.
    Stable,
    /// Beta lane; visibility and narrowing can change.
    Beta,
    /// Deprecated; surfaces must point at the replacement.
    Deprecated,
}

impl M5SurfaceLifecycle {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Beta => "beta",
            Self::Deprecated => "deprecated",
        }
    }
}

/// Canonical descriptor for one M5 surface's appearance contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AppearanceDescriptor {
    /// Stable surface id (e.g. `surface:notebook.cell_chrome`).
    pub surface_id: String,
    /// Surface family the descriptor belongs to.
    pub surface_family: M5AppearanceSurfaceFamily,
    /// Descriptor revision the audit was produced against.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Canonical appearance anchor ref the audit can reopen the surface's
    /// design-QA entry from.
    pub appearance_anchor_ref: String,
    /// Accessibility note retained on the descriptor. MUST be non-empty.
    pub accessibility_note: String,
    /// Pinned semantic salience.
    pub semantic_salience: M5SemanticSalience,
    /// Pinned surface lifecycle label.
    pub lifecycle_label: M5SurfaceLifecycle,
    /// `true` when the surface is marketed on desktop appearance rows and
    /// therefore must pass the claimed matrix or narrow accordingly.
    pub marketed_on_desktop_rows: bool,
    /// `true` once the surface rides the shared appearance-session model and
    /// does not paint its own appearance. MUST be `true`.
    pub registered_on_appearance_session: bool,
}

impl M5AppearanceDescriptor {
    /// `true` when this surface's salience makes it high-salience for the
    /// audit.
    pub const fn is_high_salience(&self) -> bool {
        self.semantic_salience.is_high_salience()
    }
}

/// Per-row qualification binding a surface reports for one appearance row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AppearanceBinding {
    /// Appearance row this binding covers.
    pub row: M5AppearanceRow,
    /// Appearance dimension projected for the row. MUST equal the row's
    /// canonical dimension.
    pub dimension: M5AppearanceDimension,
    /// Qualification status the surface reports.
    pub qualification_status: M5QualificationStatus,
    /// `true` when the surface is marketed on this row.
    pub marketed_on_row: bool,
    /// Captured screenshot-pack ref (`None` for non-qualified rows).
    pub projected_screenshot_pack_ref: Option<String>,
    /// Captured contrast result (`None` unless the row requires it).
    pub projected_contrast_result: Option<M5ContrastResult>,
    /// Captured focus-visibility result (`None` for non-qualified rows).
    pub projected_focus_visibility: Option<M5FocusVisibility>,
    /// Captured motion treatment (`None` unless the row requires it).
    pub projected_motion_treatment: Option<M5MotionTreatment>,
    /// Captured state-semantics result (`None` for non-qualified rows).
    pub projected_state_semantics: Option<M5StateSemantics>,
    /// Captured keyboard-check outcome (`None` for non-qualified rows).
    pub projected_keyboard_check: Option<M5CheckOutcome>,
    /// Captured screen-reader-check outcome (`None` for non-qualified rows).
    pub projected_screen_reader_check: Option<M5CheckOutcome>,
    /// Captured exact-target reopen affordance (`None` for non-qualified
    /// rows).
    pub projected_reopen_affordance: Option<M5ReopenAffordance>,
    /// Captured high-risk boundary cue (`None` for non-qualified rows).
    pub projected_boundary_cue: Option<M5BoundaryCue>,
    /// Captured layout integrity (`None` unless the row requires it).
    pub projected_layout_integrity: Option<M5LayoutIntegrity>,
    /// Freshness of the captured evidence (`None` for non-qualified rows).
    pub evidence_freshness: Option<M5EvidenceFreshness>,
    /// Timestamp the evidence was captured (`None` for non-qualified rows).
    pub evidence_captured_at: Option<String>,
    /// Narrowing reason set when `qualification_status` requires one.
    pub narrowing_reason: Option<String>,
    /// Reviewer-facing free-form note retained on the row.
    pub note: Option<String>,
}

/// Blocking finding class the validator emits.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "class", rename_all = "snake_case")]
pub enum M5AppearanceBlockingFinding {
    /// A surface renders an appearance row through ad-hoc local styling
    /// outside the shared appearance-session model.
    UnqualifiedLocalAppearance {
        /// Surface that exposes the gap.
        surface_id: String,
        /// Row that exposes the gap.
        row: M5AppearanceRow,
    },
    /// A marketed row is claimed with no captured evidence.
    MissingEvidence {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// A qualified row is missing its captured screenshot pack.
    MissingScreenshotPack {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// A theme row's captured contrast is below threshold.
    ContrastBelowThreshold {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// A row loses focus visibility.
    FocusNotVisible {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// A row loses the controlled state vocabulary.
    StateSemanticsLost {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// A row loses the exact-target reopen affordance.
    ReopenTargetLost {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// A row hides the high-risk boundary cue.
    BoundaryCueHidden {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// The reduced-motion row still animates.
    MotionNotDowngraded {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// A captured keyboard check failed.
    KeyboardCheckFailed {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// A captured screen-reader check failed.
    ScreenReaderCheckFailed {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// A live appearance change corrupts layout.
    LiveChangeLayoutCorruption {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// A live appearance change loses focus.
    LiveChangeFocusLoss {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// A live appearance change corrupts state semantics.
    LiveChangeStateCorruption {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// A marketed row carries stale appearance evidence.
    StaleEvidenceOnMarketedRow {
        surface_id: String,
        row: M5AppearanceRow,
    },
    /// A binding projects an appearance dimension that disagrees with the
    /// row's canonical dimension.
    DimensionDrift {
        surface_id: String,
        row: M5AppearanceRow,
        /// Projected dimension.
        projected_dimension: M5AppearanceDimension,
    },
    /// A non-qualified row is missing the `narrowing_reason`.
    MissingNarrowingReason {
        surface_id: String,
        row: M5AppearanceRow,
        qualification_status: M5QualificationStatus,
    },
    /// A qualified row is missing a captured-evidence field it requires.
    MissingProjection {
        surface_id: String,
        row: M5AppearanceRow,
        /// Name of the missing projection field.
        field: String,
    },
    /// The descriptor carries no canonical appearance anchor.
    DescriptorMissingAppearanceAnchor { surface_id: String },
    /// The descriptor carries no accessibility note.
    MissingAccessibilityNote { surface_id: String },
    /// The surface paints its own appearance outside the appearance-session
    /// model.
    SurfaceNotOnAppearanceSession { surface_id: String },
}

impl M5AppearanceBlockingFinding {
    /// Stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::UnqualifiedLocalAppearance { .. } => "unqualified_local_appearance",
            Self::MissingEvidence { .. } => "missing_evidence",
            Self::MissingScreenshotPack { .. } => "missing_screenshot_pack",
            Self::ContrastBelowThreshold { .. } => "contrast_below_threshold",
            Self::FocusNotVisible { .. } => "focus_not_visible",
            Self::StateSemanticsLost { .. } => "state_semantics_lost",
            Self::ReopenTargetLost { .. } => "reopen_target_lost",
            Self::BoundaryCueHidden { .. } => "boundary_cue_hidden",
            Self::MotionNotDowngraded { .. } => "motion_not_downgraded",
            Self::KeyboardCheckFailed { .. } => "keyboard_check_failed",
            Self::ScreenReaderCheckFailed { .. } => "screen_reader_check_failed",
            Self::LiveChangeLayoutCorruption { .. } => "live_change_layout_corruption",
            Self::LiveChangeFocusLoss { .. } => "live_change_focus_loss",
            Self::LiveChangeStateCorruption { .. } => "live_change_state_corruption",
            Self::StaleEvidenceOnMarketedRow { .. } => "stale_evidence_on_marketed_row",
            Self::DimensionDrift { .. } => "dimension_drift",
            Self::MissingNarrowingReason { .. } => "missing_narrowing_reason",
            Self::MissingProjection { .. } => "missing_projection",
            Self::DescriptorMissingAppearanceAnchor { .. } => {
                "descriptor_missing_appearance_anchor"
            }
            Self::MissingAccessibilityNote { .. } => "missing_accessibility_note",
            Self::SurfaceNotOnAppearanceSession { .. } => "surface_not_on_appearance_session",
        }
    }

    /// Returns the surface id this finding is attached to.
    pub fn surface_id(&self) -> &str {
        match self {
            Self::UnqualifiedLocalAppearance { surface_id, .. }
            | Self::MissingEvidence { surface_id, .. }
            | Self::MissingScreenshotPack { surface_id, .. }
            | Self::ContrastBelowThreshold { surface_id, .. }
            | Self::FocusNotVisible { surface_id, .. }
            | Self::StateSemanticsLost { surface_id, .. }
            | Self::ReopenTargetLost { surface_id, .. }
            | Self::BoundaryCueHidden { surface_id, .. }
            | Self::MotionNotDowngraded { surface_id, .. }
            | Self::KeyboardCheckFailed { surface_id, .. }
            | Self::ScreenReaderCheckFailed { surface_id, .. }
            | Self::LiveChangeLayoutCorruption { surface_id, .. }
            | Self::LiveChangeFocusLoss { surface_id, .. }
            | Self::LiveChangeStateCorruption { surface_id, .. }
            | Self::StaleEvidenceOnMarketedRow { surface_id, .. }
            | Self::DimensionDrift { surface_id, .. }
            | Self::MissingNarrowingReason { surface_id, .. }
            | Self::MissingProjection { surface_id, .. }
            | Self::DescriptorMissingAppearanceAnchor { surface_id }
            | Self::MissingAccessibilityNote { surface_id }
            | Self::SurfaceNotOnAppearanceSession { surface_id } => surface_id,
        }
    }

    /// Returns the row this finding is attached to, when row-scoped.
    pub fn row(&self) -> Option<M5AppearanceRow> {
        match self {
            Self::UnqualifiedLocalAppearance { row, .. }
            | Self::MissingEvidence { row, .. }
            | Self::MissingScreenshotPack { row, .. }
            | Self::ContrastBelowThreshold { row, .. }
            | Self::FocusNotVisible { row, .. }
            | Self::StateSemanticsLost { row, .. }
            | Self::ReopenTargetLost { row, .. }
            | Self::BoundaryCueHidden { row, .. }
            | Self::MotionNotDowngraded { row, .. }
            | Self::KeyboardCheckFailed { row, .. }
            | Self::ScreenReaderCheckFailed { row, .. }
            | Self::LiveChangeLayoutCorruption { row, .. }
            | Self::LiveChangeFocusLoss { row, .. }
            | Self::LiveChangeStateCorruption { row, .. }
            | Self::StaleEvidenceOnMarketedRow { row, .. }
            | Self::DimensionDrift { row, .. }
            | Self::MissingNarrowingReason { row, .. }
            | Self::MissingProjection { row, .. } => Some(*row),
            Self::DescriptorMissingAppearanceAnchor { .. }
            | Self::MissingAccessibilityNote { .. }
            | Self::SurfaceNotOnAppearanceSession { .. } => None,
        }
    }
}

/// One per-surface appearance-qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AppearanceQualificationRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the surface.
    pub descriptor: M5AppearanceDescriptor,
    /// Row-by-row qualification bindings, in canonical row order.
    pub bindings: Vec<M5AppearanceBinding>,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<M5AppearanceBlockingFinding>,
    /// `true` when the surface's descriptor classifies it as high-salience.
    pub high_salience: bool,
    /// `true` when the surface is marketed on desktop appearance rows.
    pub marketed: bool,
}

/// Per-class blocking-finding count summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AppearanceFindingSummary {
    /// Total blocking findings across the audit.
    pub total_blocking_findings: usize,
    /// Number of `unqualified_local_appearance` findings.
    pub unqualified_local_appearance: usize,
    /// Number of `missing_evidence` findings.
    pub missing_evidence: usize,
    /// Number of `missing_screenshot_pack` findings.
    pub missing_screenshot_pack: usize,
    /// Number of `contrast_below_threshold` findings.
    pub contrast_below_threshold: usize,
    /// Number of `focus_not_visible` findings.
    pub focus_not_visible: usize,
    /// Number of `state_semantics_lost` findings.
    pub state_semantics_lost: usize,
    /// Number of `reopen_target_lost` findings.
    pub reopen_target_lost: usize,
    /// Number of `boundary_cue_hidden` findings.
    pub boundary_cue_hidden: usize,
    /// Number of `motion_not_downgraded` findings.
    pub motion_not_downgraded: usize,
    /// Number of `keyboard_check_failed` findings.
    pub keyboard_check_failed: usize,
    /// Number of `screen_reader_check_failed` findings.
    pub screen_reader_check_failed: usize,
    /// Number of `live_change_layout_corruption` findings.
    pub live_change_layout_corruption: usize,
    /// Number of `live_change_focus_loss` findings.
    pub live_change_focus_loss: usize,
    /// Number of `live_change_state_corruption` findings.
    pub live_change_state_corruption: usize,
    /// Number of `stale_evidence_on_marketed_row` findings.
    pub stale_evidence_on_marketed_row: usize,
    /// Number of `dimension_drift` findings.
    pub dimension_drift: usize,
    /// Number of `missing_narrowing_reason` findings.
    pub missing_narrowing_reason: usize,
    /// Number of `missing_projection` findings.
    pub missing_projection: usize,
    /// Number of `descriptor_missing_appearance_anchor` findings.
    pub descriptor_missing_appearance_anchor: usize,
    /// Number of `missing_accessibility_note` findings.
    pub missing_accessibility_note: usize,
    /// Number of `surface_not_on_appearance_session` findings.
    pub surface_not_on_appearance_session: usize,
}

impl M5AppearanceFindingSummary {
    fn empty() -> Self {
        Self {
            total_blocking_findings: 0,
            unqualified_local_appearance: 0,
            missing_evidence: 0,
            missing_screenshot_pack: 0,
            contrast_below_threshold: 0,
            focus_not_visible: 0,
            state_semantics_lost: 0,
            reopen_target_lost: 0,
            boundary_cue_hidden: 0,
            motion_not_downgraded: 0,
            keyboard_check_failed: 0,
            screen_reader_check_failed: 0,
            live_change_layout_corruption: 0,
            live_change_focus_loss: 0,
            live_change_state_corruption: 0,
            stale_evidence_on_marketed_row: 0,
            dimension_drift: 0,
            missing_narrowing_reason: 0,
            missing_projection: 0,
            descriptor_missing_appearance_anchor: 0,
            missing_accessibility_note: 0,
            surface_not_on_appearance_session: 0,
        }
    }

    fn record(&mut self, finding: &M5AppearanceBlockingFinding) {
        self.total_blocking_findings += 1;
        match finding {
            M5AppearanceBlockingFinding::UnqualifiedLocalAppearance { .. } => {
                self.unqualified_local_appearance += 1
            }
            M5AppearanceBlockingFinding::MissingEvidence { .. } => self.missing_evidence += 1,
            M5AppearanceBlockingFinding::MissingScreenshotPack { .. } => {
                self.missing_screenshot_pack += 1
            }
            M5AppearanceBlockingFinding::ContrastBelowThreshold { .. } => {
                self.contrast_below_threshold += 1
            }
            M5AppearanceBlockingFinding::FocusNotVisible { .. } => self.focus_not_visible += 1,
            M5AppearanceBlockingFinding::StateSemanticsLost { .. } => {
                self.state_semantics_lost += 1
            }
            M5AppearanceBlockingFinding::ReopenTargetLost { .. } => self.reopen_target_lost += 1,
            M5AppearanceBlockingFinding::BoundaryCueHidden { .. } => self.boundary_cue_hidden += 1,
            M5AppearanceBlockingFinding::MotionNotDowngraded { .. } => {
                self.motion_not_downgraded += 1
            }
            M5AppearanceBlockingFinding::KeyboardCheckFailed { .. } => {
                self.keyboard_check_failed += 1
            }
            M5AppearanceBlockingFinding::ScreenReaderCheckFailed { .. } => {
                self.screen_reader_check_failed += 1
            }
            M5AppearanceBlockingFinding::LiveChangeLayoutCorruption { .. } => {
                self.live_change_layout_corruption += 1
            }
            M5AppearanceBlockingFinding::LiveChangeFocusLoss { .. } => {
                self.live_change_focus_loss += 1
            }
            M5AppearanceBlockingFinding::LiveChangeStateCorruption { .. } => {
                self.live_change_state_corruption += 1
            }
            M5AppearanceBlockingFinding::StaleEvidenceOnMarketedRow { .. } => {
                self.stale_evidence_on_marketed_row += 1
            }
            M5AppearanceBlockingFinding::DimensionDrift { .. } => self.dimension_drift += 1,
            M5AppearanceBlockingFinding::MissingNarrowingReason { .. } => {
                self.missing_narrowing_reason += 1
            }
            M5AppearanceBlockingFinding::MissingProjection { .. } => self.missing_projection += 1,
            M5AppearanceBlockingFinding::DescriptorMissingAppearanceAnchor { .. } => {
                self.descriptor_missing_appearance_anchor += 1
            }
            M5AppearanceBlockingFinding::MissingAccessibilityNote { .. } => {
                self.missing_accessibility_note += 1
            }
            M5AppearanceBlockingFinding::SurfaceNotOnAppearanceSession { .. } => {
                self.surface_not_on_appearance_session += 1
            }
        }
    }
}

/// Per-row coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AppearanceCoverageSummary {
    /// Row this summary covers.
    pub row: M5AppearanceRow,
    /// Number of `qualified` rows on this appearance row.
    pub qualified_rows: usize,
    /// Number of `explicitly_narrowed` rows on this appearance row.
    pub explicitly_narrowed_rows: usize,
    /// Number of `not_applicable` rows on this appearance row.
    pub not_applicable_rows: usize,
    /// Number of `platform_omitted` rows on this appearance row.
    pub platform_omitted_rows: usize,
    /// Number of `declared_capture_gap` rows on this appearance row.
    pub declared_capture_gap_rows: usize,
    /// Number of `unqualified_local_appearance` rows on this appearance row.
    pub unqualified_local_appearance_rows: usize,
    /// Number of `missing_evidence` rows on this appearance row.
    pub missing_evidence_rows: usize,
}

impl M5AppearanceCoverageSummary {
    fn narrowed_rows(&self) -> usize {
        self.explicitly_narrowed_rows
            + self.not_applicable_rows
            + self.platform_omitted_rows
            + self.declared_capture_gap_rows
    }
}

/// A single appearance-anchor index entry the audit publishes so design QA,
/// docs, and release surfaces can reopen each M5 surface by its anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AppearanceAnchorEntry {
    /// Surface family the anchor belongs to.
    pub surface_family: M5AppearanceSurfaceFamily,
    /// Surface id the anchor reopens.
    pub surface_id: String,
    /// Canonical appearance anchor ref.
    pub appearance_anchor_ref: String,
}

/// One marketed row release tooling should narrow because its appearance
/// evidence is stale or red.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5NarrowableRow {
    /// Surface id that must narrow.
    pub surface_id: String,
    /// Appearance row that must narrow.
    pub row: M5AppearanceRow,
    /// Stable reason the row is narrowable.
    pub reason: String,
}

/// M5 appearance-and-density qualification audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AppearanceQualificationReport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable report id quoted across surfaces.
    pub report_id: String,
    /// Source schema ref for the canonical contract.
    pub source_schema_ref: String,
    /// Required appearance rows, in canonical order.
    pub required_rows: Vec<M5AppearanceRow>,
    /// Per-surface qualification rows, sorted by `descriptor.surface_id`.
    pub rows: Vec<M5AppearanceQualificationRow>,
    /// Per-row coverage summary, in canonical row order.
    pub row_coverage: Vec<M5AppearanceCoverageSummary>,
    /// Per-class blocking-finding summary.
    pub findings_summary: M5AppearanceFindingSummary,
    /// Canonical appearance-anchor index, sorted by surface id.
    pub appearance_anchor_index: Vec<M5AppearanceAnchorEntry>,
    /// Number of registered M5 surfaces present.
    pub registered_surface_count: usize,
    /// Number of high-salience surfaces present.
    pub high_salience_surface_count: usize,
    /// Number of surfaces marketed on desktop appearance rows.
    pub marketed_surface_count: usize,
    /// Total appearance rows checked.
    pub appearance_rows_checked: usize,
    /// Marketed rows release tooling should narrow because their evidence is
    /// stale or red.
    pub narrowable_marketed_rows: Vec<M5NarrowableRow>,
    /// `true` when there are zero blocking findings.
    pub report_clean: bool,
    /// Markdown publication ref this audit is rendered to.
    pub published_report_ref: String,
    /// Companion doc publication ref.
    pub published_doc_ref: String,
    /// Docs/help refs the audit can be reopened from.
    pub docs_help_refs: Vec<String>,
    /// Support/export refs the audit can be reopened from.
    pub support_export_refs: Vec<String>,
    /// Timestamp captured when the audit was generated.
    pub generated_at: String,
}

impl M5AppearanceQualificationReport {
    /// Returns `true` when every required row is qualified by at least one
    /// surface.
    pub fn every_required_row_qualified(&self) -> bool {
        for row in M5AppearanceRow::required_rows() {
            let any_qualified = self.rows.iter().any(|surface| {
                surface.bindings.iter().any(|binding| {
                    binding.row == row
                        && binding.qualification_status == M5QualificationStatus::Qualified
                })
            });
            if !any_qualified {
                return false;
            }
        }
        true
    }

    /// Builds compact text rows for headless review.
    pub fn compact_lines(&self) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!(
            "audit: surfaces={}, high_salience={}, marketed={}, rows={}, blocking={}, clean={}",
            self.registered_surface_count,
            self.high_salience_surface_count,
            self.marketed_surface_count,
            self.appearance_rows_checked,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for coverage in &self.row_coverage {
            lines.push(format!(
                "{}: qualified={}, narrowed={}, unqualified={}, missing_evidence={}",
                coverage.row.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_local_appearance_rows,
                coverage.missing_evidence_rows,
            ));
        }
        for surface in &self.rows {
            for finding in &surface.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {} -- {}",
                    finding.class_token(),
                    finding.surface_id(),
                    finding
                        .row()
                        .map(M5AppearanceRow::as_str)
                        .unwrap_or("surface"),
                ));
            }
        }
        for narrowable in &self.narrowable_marketed_rows {
            lines.push(format!(
                "narrowable: {} -- {} -- {}",
                narrowable.surface_id,
                narrowable.row.as_str(),
                narrowable.reason,
            ));
        }
        lines
    }

    /// Renders the markdown audit artifact.
    pub fn render_markdown(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 appearance and density qualification audit\n");
        out.push('\n');
        out.push_str(
            "Generated from the seeded audit in\n\
             [`crate::m5_appearance_parity`](../../../../crates/aureline-shell/src/m5_appearance_parity/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_parity -- report-md > \\\n  artifacts/ux/m5/appearance-parity/m5_appearance_parity_audit.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Registered M5 surfaces: `{}`\n",
            self.registered_surface_count
        ));
        out.push_str(&format!(
            "- High-salience surfaces: `{}`\n",
            self.high_salience_surface_count
        ));
        out.push_str(&format!(
            "- Marketed surfaces: `{}`\n",
            self.marketed_surface_count
        ));
        out.push_str(&format!(
            "- Appearance rows checked: `{}`\n",
            self.appearance_rows_checked
        ));
        out.push_str(&format!(
            "- Blocking findings: `{}`\n",
            self.findings_summary.total_blocking_findings
        ));
        out.push_str(&format!(
            "- Narrowable marketed rows: `{}`\n",
            self.narrowable_marketed_rows.len()
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

        out.push_str("## Per-row coverage\n\n");
        out.push_str(
            "| Appearance row | Qualified | Narrowed | Unqualified | Missing evidence |\n\
             | -------------- | --------: | -------: | ----------: | ---------------: |\n",
        );
        for coverage in &self.row_coverage {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                coverage.row.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_local_appearance_rows,
                coverage.missing_evidence_rows,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        out.push_str(&format!(
            "| `unqualified_local_appearance` | {} |\n",
            self.findings_summary.unqualified_local_appearance
        ));
        out.push_str(&format!(
            "| `missing_evidence` | {} |\n",
            self.findings_summary.missing_evidence
        ));
        out.push_str(&format!(
            "| `missing_screenshot_pack` | {} |\n",
            self.findings_summary.missing_screenshot_pack
        ));
        out.push_str(&format!(
            "| `contrast_below_threshold` | {} |\n",
            self.findings_summary.contrast_below_threshold
        ));
        out.push_str(&format!(
            "| `focus_not_visible` | {} |\n",
            self.findings_summary.focus_not_visible
        ));
        out.push_str(&format!(
            "| `state_semantics_lost` | {} |\n",
            self.findings_summary.state_semantics_lost
        ));
        out.push_str(&format!(
            "| `reopen_target_lost` | {} |\n",
            self.findings_summary.reopen_target_lost
        ));
        out.push_str(&format!(
            "| `boundary_cue_hidden` | {} |\n",
            self.findings_summary.boundary_cue_hidden
        ));
        out.push_str(&format!(
            "| `motion_not_downgraded` | {} |\n",
            self.findings_summary.motion_not_downgraded
        ));
        out.push_str(&format!(
            "| `keyboard_check_failed` | {} |\n",
            self.findings_summary.keyboard_check_failed
        ));
        out.push_str(&format!(
            "| `screen_reader_check_failed` | {} |\n",
            self.findings_summary.screen_reader_check_failed
        ));
        out.push_str(&format!(
            "| `live_change_layout_corruption` | {} |\n",
            self.findings_summary.live_change_layout_corruption
        ));
        out.push_str(&format!(
            "| `live_change_focus_loss` | {} |\n",
            self.findings_summary.live_change_focus_loss
        ));
        out.push_str(&format!(
            "| `live_change_state_corruption` | {} |\n",
            self.findings_summary.live_change_state_corruption
        ));
        out.push_str(&format!(
            "| `stale_evidence_on_marketed_row` | {} |\n",
            self.findings_summary.stale_evidence_on_marketed_row
        ));
        out.push_str(&format!(
            "| `dimension_drift` | {} |\n",
            self.findings_summary.dimension_drift
        ));
        out.push_str(&format!(
            "| `missing_narrowing_reason` | {} |\n",
            self.findings_summary.missing_narrowing_reason
        ));
        out.push_str(&format!(
            "| `missing_projection` | {} |\n",
            self.findings_summary.missing_projection
        ));
        out.push_str(&format!(
            "| `descriptor_missing_appearance_anchor` | {} |\n",
            self.findings_summary.descriptor_missing_appearance_anchor
        ));
        out.push_str(&format!(
            "| `missing_accessibility_note` | {} |\n",
            self.findings_summary.missing_accessibility_note
        ));
        out.push_str(&format!(
            "| `surface_not_on_appearance_session` | {} |\n\n",
            self.findings_summary.surface_not_on_appearance_session
        ));

        out.push_str("## Appearance anchor index\n\n");
        out.push_str(
            "| Surface family | Surface | Appearance anchor |\n| -------------- | ------- | ----------------- |\n",
        );
        for entry in &self.appearance_anchor_index {
            out.push_str(&format!(
                "| {} | `{}` | `{}` |\n",
                entry.surface_family.display_label(),
                entry.surface_id,
                entry.appearance_anchor_ref,
            ));
        }
        out.push('\n');

        out.push_str("## Per-surface rows\n\n");
        for surface in &self.rows {
            out.push_str(&format!(
                "### `{}` ({}, {})\n\n",
                surface.descriptor.surface_id,
                surface.descriptor.surface_family.as_str(),
                surface.descriptor.lifecycle_label.as_str()
            ));
            out.push_str(&format!(
                "- Descriptor revision: `{}`\n",
                surface.descriptor.descriptor_revision_ref
            ));
            out.push_str(&format!(
                "- Semantic salience: `{}`\n",
                surface.descriptor.semantic_salience.as_str()
            ));
            out.push_str(&format!(
                "- Appearance anchor: `{}`\n",
                surface.descriptor.appearance_anchor_ref
            ));
            out.push_str(&format!(
                "- Marketed on desktop rows: `{}`\n",
                if surface.marketed { "yes" } else { "no" }
            ));
            out.push_str(&format!(
                "- High-salience: `{}`\n\n",
                if surface.high_salience { "yes" } else { "no" }
            ));

            out.push_str(
                "| Appearance row | Status | Contrast | Focus | Motion | State | Reopen | Boundary | Freshness | Narrowing reason |\n\
                 | -------------- | ------ | -------- | ----- | ------ | ----- | ------ | -------- | --------- | ---------------- |\n",
            );
            for binding in &surface.bindings {
                let contrast = binding
                    .projected_contrast_result
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let focus = binding
                    .projected_focus_visibility
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let motion = binding
                    .projected_motion_treatment
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let state = binding
                    .projected_state_semantics
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let reopen = binding
                    .projected_reopen_affordance
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let boundary = binding
                    .projected_boundary_cue
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let freshness = binding
                    .evidence_freshness
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let narrowing = binding.narrowing_reason.as_deref().unwrap_or("-");
                out.push_str(&format!(
                    "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                    binding.row.display_label(),
                    binding.qualification_status.as_str(),
                    contrast,
                    focus,
                    motion,
                    state,
                    reopen,
                    boundary,
                    freshness,
                    narrowing,
                ));
            }
            out.push('\n');

            if surface.blocking_findings.is_empty() {
                out.push_str("Findings: none.\n\n");
            } else {
                out.push_str("Findings:\n\n");
                for finding in &surface.blocking_findings {
                    out.push_str(&format!(
                        "- `{}` on `{}`\n",
                        finding.class_token(),
                        finding
                            .row()
                            .map(M5AppearanceRow::as_str)
                            .unwrap_or("surface"),
                    ));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_appearance_parity -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_appearance_parity_fixtures\n");
        out.push_str("python3 tools/ci/m5/appearance_parity_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the M5 appearance-qualification audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5AppearanceSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Audit report quoted in full.
    pub report: M5AppearanceQualificationReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl M5AppearanceSupportExport {
    /// Builds the support-export wrapper for an audit report.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: M5AppearanceQualificationReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for surface in &report.rows {
            case_ids.push(surface.descriptor.surface_id.clone());
            case_ids.push(surface.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: M5_APPEARANCE_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_APPEARANCE_SCHEMA_VERSION,
            shared_contract_ref: M5_APPEARANCE_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Computes the per-surface blocking findings from a descriptor and its row
/// bindings.
fn compute_surface_findings(
    descriptor: &M5AppearanceDescriptor,
    bindings: &[M5AppearanceBinding],
    high_salience: bool,
) -> Vec<M5AppearanceBlockingFinding> {
    let mut findings = Vec::new();

    // Descriptor-level (surface-scoped) findings.
    if descriptor.appearance_anchor_ref.trim().is_empty() {
        findings.push(
            M5AppearanceBlockingFinding::DescriptorMissingAppearanceAnchor {
                surface_id: descriptor.surface_id.clone(),
            },
        );
    }
    if descriptor.accessibility_note.trim().is_empty() {
        findings.push(M5AppearanceBlockingFinding::MissingAccessibilityNote {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if !descriptor.registered_on_appearance_session {
        findings.push(M5AppearanceBlockingFinding::SurfaceNotOnAppearanceSession {
            surface_id: descriptor.surface_id.clone(),
        });
    }

    for binding in bindings {
        let row = binding.row;
        let surface_id = descriptor.surface_id.clone();

        // A binding's dimension must match its row's canonical dimension.
        if binding.dimension != row.canonical_dimension() {
            findings.push(M5AppearanceBlockingFinding::DimensionDrift {
                surface_id: surface_id.clone(),
                row,
                projected_dimension: binding.dimension,
            });
        }

        match binding.qualification_status {
            M5QualificationStatus::UnqualifiedLocalAppearance => {
                findings.push(M5AppearanceBlockingFinding::UnqualifiedLocalAppearance {
                    surface_id: surface_id.clone(),
                    row,
                });
            }
            M5QualificationStatus::MissingEvidence => {
                findings.push(M5AppearanceBlockingFinding::MissingEvidence {
                    surface_id: surface_id.clone(),
                    row,
                });
            }
            M5QualificationStatus::Qualified => {
                compute_qualified_findings(binding, high_salience, &surface_id, &mut findings);
            }
            status if status.requires_narrowing_reason() => {
                let reason_ok = binding
                    .narrowing_reason
                    .as_deref()
                    .map(str::trim)
                    .map(str::is_empty)
                    == Some(false);
                if !reason_ok {
                    findings.push(M5AppearanceBlockingFinding::MissingNarrowingReason {
                        surface_id: surface_id.clone(),
                        row,
                        qualification_status: status,
                    });
                }
            }
            _ => {}
        }
    }
    findings
}

/// Computes the blocking findings for one qualified appearance binding.
fn compute_qualified_findings(
    binding: &M5AppearanceBinding,
    high_salience: bool,
    surface_id: &str,
    findings: &mut Vec<M5AppearanceBlockingFinding>,
) {
    let row = binding.row;

    // Required captured-evidence projections.
    if binding.projected_screenshot_pack_ref.is_none() {
        findings.push(M5AppearanceBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_screenshot_pack_ref".to_owned(),
        });
    }
    if binding.projected_focus_visibility.is_none() {
        findings.push(M5AppearanceBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_focus_visibility".to_owned(),
        });
    }
    if binding.projected_state_semantics.is_none() {
        findings.push(M5AppearanceBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_state_semantics".to_owned(),
        });
    }
    if binding.projected_keyboard_check.is_none() {
        findings.push(M5AppearanceBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_keyboard_check".to_owned(),
        });
    }
    if binding.projected_screen_reader_check.is_none() {
        findings.push(M5AppearanceBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_screen_reader_check".to_owned(),
        });
    }
    if binding.projected_reopen_affordance.is_none() {
        findings.push(M5AppearanceBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_reopen_affordance".to_owned(),
        });
    }
    if binding.evidence_freshness.is_none() {
        findings.push(M5AppearanceBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "evidence_freshness".to_owned(),
        });
    }
    if row.requires_contrast() && binding.projected_contrast_result.is_none() {
        findings.push(M5AppearanceBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_contrast_result".to_owned(),
        });
    }
    if row.requires_motion_treatment() && binding.projected_motion_treatment.is_none() {
        findings.push(M5AppearanceBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_motion_treatment".to_owned(),
        });
    }
    if row.requires_layout_integrity() && binding.projected_layout_integrity.is_none() {
        findings.push(M5AppearanceBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_layout_integrity".to_owned(),
        });
    }
    if high_salience && binding.projected_boundary_cue.is_none() {
        findings.push(M5AppearanceBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_boundary_cue".to_owned(),
        });
    }

    let is_live_change = row.requires_layout_integrity();

    // Red captured results.
    if binding.projected_screenshot_pack_ref.is_none() {
        findings.push(M5AppearanceBlockingFinding::MissingScreenshotPack {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_contrast_result == Some(M5ContrastResult::BelowThreshold) {
        findings.push(M5AppearanceBlockingFinding::ContrastBelowThreshold {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_focus_visibility == Some(M5FocusVisibility::NotVisible) {
        if is_live_change {
            findings.push(M5AppearanceBlockingFinding::LiveChangeFocusLoss {
                surface_id: surface_id.to_owned(),
                row,
            });
        } else {
            findings.push(M5AppearanceBlockingFinding::FocusNotVisible {
                surface_id: surface_id.to_owned(),
                row,
            });
        }
    }
    if binding.projected_state_semantics == Some(M5StateSemantics::Lost) {
        if is_live_change {
            findings.push(M5AppearanceBlockingFinding::LiveChangeStateCorruption {
                surface_id: surface_id.to_owned(),
                row,
            });
        } else {
            findings.push(M5AppearanceBlockingFinding::StateSemanticsLost {
                surface_id: surface_id.to_owned(),
                row,
            });
        }
    }
    if binding.projected_keyboard_check == Some(M5CheckOutcome::Fail) {
        findings.push(M5AppearanceBlockingFinding::KeyboardCheckFailed {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_screen_reader_check == Some(M5CheckOutcome::Fail) {
        findings.push(M5AppearanceBlockingFinding::ScreenReaderCheckFailed {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_reopen_affordance == Some(M5ReopenAffordance::Lost) {
        findings.push(M5AppearanceBlockingFinding::ReopenTargetLost {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_boundary_cue == Some(M5BoundaryCue::Hidden) {
        findings.push(M5AppearanceBlockingFinding::BoundaryCueHidden {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if row.requires_motion_treatment()
        && binding.projected_motion_treatment == Some(M5MotionTreatment::Animated)
    {
        findings.push(M5AppearanceBlockingFinding::MotionNotDowngraded {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if is_live_change && binding.projected_layout_integrity == Some(M5LayoutIntegrity::Corrupted) {
        findings.push(M5AppearanceBlockingFinding::LiveChangeLayoutCorruption {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.marketed_on_row && binding.evidence_freshness == Some(M5EvidenceFreshness::Stale) {
        findings.push(M5AppearanceBlockingFinding::StaleEvidenceOnMarketedRow {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
}

/// Computes the per-row and per-class summaries from finished surfaces.
fn summarize_report(
    surfaces: &[M5AppearanceQualificationRow],
) -> (Vec<M5AppearanceCoverageSummary>, M5AppearanceFindingSummary) {
    let mut summary = M5AppearanceFindingSummary::empty();
    let mut coverage: Vec<M5AppearanceCoverageSummary> = M5AppearanceRow::required_rows()
        .into_iter()
        .map(|row| M5AppearanceCoverageSummary {
            row,
            qualified_rows: 0,
            explicitly_narrowed_rows: 0,
            not_applicable_rows: 0,
            platform_omitted_rows: 0,
            declared_capture_gap_rows: 0,
            unqualified_local_appearance_rows: 0,
            missing_evidence_rows: 0,
        })
        .collect();

    for surface in surfaces {
        for binding in &surface.bindings {
            if let Some(coverage_row) = coverage.iter_mut().find(|entry| entry.row == binding.row) {
                match binding.qualification_status {
                    M5QualificationStatus::Qualified => coverage_row.qualified_rows += 1,
                    M5QualificationStatus::ExplicitlyNarrowed => {
                        coverage_row.explicitly_narrowed_rows += 1
                    }
                    M5QualificationStatus::NotApplicable => coverage_row.not_applicable_rows += 1,
                    M5QualificationStatus::PlatformOmitted => {
                        coverage_row.platform_omitted_rows += 1
                    }
                    M5QualificationStatus::DeclaredCaptureGap => {
                        coverage_row.declared_capture_gap_rows += 1
                    }
                    M5QualificationStatus::UnqualifiedLocalAppearance => {
                        coverage_row.unqualified_local_appearance_rows += 1
                    }
                    M5QualificationStatus::MissingEvidence => {
                        coverage_row.missing_evidence_rows += 1
                    }
                }
            }
        }
        for finding in &surface.blocking_findings {
            summary.record(finding);
        }
    }

    (coverage, summary)
}

/// Computes the marketed rows release tooling should narrow because their
/// appearance evidence is stale or red.
fn compute_narrowable_rows(surfaces: &[M5AppearanceQualificationRow]) -> Vec<M5NarrowableRow> {
    let mut narrowable = Vec::new();
    for surface in surfaces {
        if !surface.marketed {
            continue;
        }
        for finding in &surface.blocking_findings {
            if let Some(row) = finding.row() {
                narrowable.push(M5NarrowableRow {
                    surface_id: surface.descriptor.surface_id.clone(),
                    row,
                    reason: format!("blocking_finding:{}", finding.class_token()),
                });
            }
        }
    }
    narrowable
}

/// Builds an [`M5AppearanceQualificationRow`] from a descriptor and its row
/// bindings, computing the per-surface blocking findings.
pub fn build_m5_appearance_row(
    descriptor: M5AppearanceDescriptor,
    bindings: Vec<M5AppearanceBinding>,
) -> M5AppearanceQualificationRow {
    let high_salience = descriptor.is_high_salience();
    let marketed = descriptor.marketed_on_desktop_rows;
    let blocking_findings = compute_surface_findings(&descriptor, &bindings, high_salience);

    M5AppearanceQualificationRow {
        record_kind: M5_APPEARANCE_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_APPEARANCE_SCHEMA_VERSION,
        shared_contract_ref: M5_APPEARANCE_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        bindings,
        blocking_findings,
        high_salience,
        marketed,
    }
}

/// Builds a full [`M5AppearanceQualificationReport`] from per-surface rows.
pub fn build_m5_appearance_qualification_audit(
    surfaces: Vec<M5AppearanceQualificationRow>,
) -> M5AppearanceQualificationReport {
    let mut surfaces = surfaces;
    surfaces.sort_by(|left, right| left.descriptor.surface_id.cmp(&right.descriptor.surface_id));

    let registered_surface_count = surfaces.len();
    let high_salience_surface_count = surfaces.iter().filter(|row| row.high_salience).count();
    let marketed_surface_count = surfaces.iter().filter(|row| row.marketed).count();
    let appearance_rows_checked = surfaces.iter().map(|row| row.bindings.len()).sum::<usize>();

    let (row_coverage, findings_summary) = summarize_report(&surfaces);
    let narrowable_marketed_rows = compute_narrowable_rows(&surfaces);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut appearance_anchor_index: Vec<M5AppearanceAnchorEntry> = surfaces
        .iter()
        .map(|surface| M5AppearanceAnchorEntry {
            surface_family: surface.descriptor.surface_family,
            surface_id: surface.descriptor.surface_id.clone(),
            appearance_anchor_ref: surface.descriptor.appearance_anchor_ref.clone(),
        })
        .collect();
    appearance_anchor_index.sort_by(|left, right| left.surface_id.cmp(&right.surface_id));

    M5AppearanceQualificationReport {
        record_kind: M5_APPEARANCE_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_APPEARANCE_SCHEMA_VERSION,
        shared_contract_ref: M5_APPEARANCE_SHARED_CONTRACT_REF.to_owned(),
        report_id: M5_APPEARANCE_REPORT_ID.to_owned(),
        source_schema_ref: M5_APPEARANCE_SOURCE_SCHEMA_REF.to_owned(),
        required_rows: M5AppearanceRow::required_rows().to_vec(),
        rows: surfaces,
        row_coverage,
        findings_summary,
        appearance_anchor_index,
        registered_surface_count,
        high_salience_surface_count,
        marketed_surface_count,
        appearance_rows_checked,
        narrowable_marketed_rows,
        report_clean,
        published_report_ref: M5_APPEARANCE_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: M5_APPEARANCE_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            M5_APPEARANCE_PUBLISHED_DOC_REF.to_owned(),
            "docs/m5/component-state-parity.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-appearance-parity".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_m5_appearance_qualification`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum M5AppearanceValidationError {
    /// The audit has no registered surfaces.
    NoRegisteredSurfaces,
    /// A required appearance row has no qualified surface.
    RequiredRowNotQualified { row: String },
    /// A surface is missing a required appearance row from its binding set.
    MissingRequiredRow { surface_id: String, row: String },
    /// A blocking finding remains on the surface.
    BlockingFindingPresent {
        surface_id: String,
        row: String,
        class: String,
    },
    /// The published markdown report ref is empty.
    PublishedReportRefMissing,
    /// The companion doc ref is empty.
    PublishedDocRefMissing,
    /// A surface's descriptor revision ref is empty.
    MissingDescriptorRevisionRef { surface_id: String },
}

/// Validates an audit report against the M5 appearance acceptance
/// invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_m5_appearance_qualification(
    report: &M5AppearanceQualificationReport,
) -> Result<(), Vec<M5AppearanceValidationError>> {
    let mut errors = Vec::new();

    if report.rows.is_empty() {
        errors.push(M5AppearanceValidationError::NoRegisteredSurfaces);
    }

    for required in M5AppearanceRow::required_rows() {
        let any_qualified = report.rows.iter().any(|surface| {
            surface.bindings.iter().any(|binding| {
                binding.row == required
                    && binding.qualification_status == M5QualificationStatus::Qualified
            })
        });
        if !any_qualified {
            errors.push(M5AppearanceValidationError::RequiredRowNotQualified {
                row: required.as_str().to_owned(),
            });
        }
    }

    for surface in &report.rows {
        for required in M5AppearanceRow::required_rows() {
            if !surface
                .bindings
                .iter()
                .any(|binding| binding.row == required)
            {
                errors.push(M5AppearanceValidationError::MissingRequiredRow {
                    surface_id: surface.descriptor.surface_id.clone(),
                    row: required.as_str().to_owned(),
                });
            }
        }
        if surface.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(M5AppearanceValidationError::MissingDescriptorRevisionRef {
                surface_id: surface.descriptor.surface_id.clone(),
            });
        }
        for finding in &surface.blocking_findings {
            errors.push(M5AppearanceValidationError::BlockingFindingPresent {
                surface_id: finding.surface_id().to_owned(),
                row: finding
                    .row()
                    .map(|row| row.as_str().to_owned())
                    .unwrap_or_else(|| "surface".to_owned()),
                class: finding.class_token().to_owned(),
            });
        }
    }

    if report.published_report_ref.trim().is_empty() {
        errors.push(M5AppearanceValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(M5AppearanceValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_m5_appearance_qualification_audit`].
struct SurfaceSeed {
    surface_id: &'static str,
    surface_family: M5AppearanceSurfaceFamily,
    descriptor_revision_ref: &'static str,
    primary_label_ref: &'static str,
    appearance_anchor_ref: &'static str,
    accessibility_note: &'static str,
    semantic_salience: M5SemanticSalience,
    lifecycle_label: M5SurfaceLifecycle,
    reopen: M5ReopenAffordance,
    boundary_cue: M5BoundaryCue,
    bindings: &'static [BindingSeed],
}

struct BindingSeed {
    row: M5AppearanceRow,
    qualification_status: M5QualificationStatus,
    narrowing_reason: Option<&'static str>,
    note: Option<&'static str>,
}

/// Helper: a qualified row with captured evidence.
const fn qualified(row: M5AppearanceRow) -> BindingSeed {
    BindingSeed {
        row,
        qualification_status: M5QualificationStatus::Qualified,
        narrowing_reason: None,
        note: None,
    }
}

/// Helper: an honestly-declared capture gap with a documented reason.
const fn declared_capture_gap(row: M5AppearanceRow, reason: &'static str) -> BindingSeed {
    BindingSeed {
        row,
        qualification_status: M5QualificationStatus::DeclaredCaptureGap,
        narrowing_reason: Some(reason),
        note: None,
    }
}

/// Contrast result projected for a qualified theme row.
const fn contrast_for_row(row: M5AppearanceRow) -> M5ContrastResult {
    match row {
        M5AppearanceRow::ThemeHighContrast => M5ContrastResult::MeetsAaa,
        _ => M5ContrastResult::MeetsAa,
    }
}

const SURFACE_SEEDS: &[SurfaceSeed] = &[
    // Notebook cell chrome. Lifecycle-bearing; reopen-able; carries a cue.
    SurfaceSeed {
        surface_id: "surface:notebook.cell_chrome",
        surface_family: M5AppearanceSurfaceFamily::NotebookCellChrome,
        descriptor_revision_ref: "surface-rev:notebook.cell_chrome:2026.06.01-01",
        primary_label_ref: "label:notebook.cell_chrome:primary",
        appearance_anchor_ref: "appearance:anchor:notebook:cell_chrome",
        accessibility_note: "Run, error, and stale chrome stay legible and labelled across every theme, density, and reduced motion.",
        semantic_salience: M5SemanticSalience::LifecycleBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenAffordance::ExactTargetPreserved,
        boundary_cue: M5BoundaryCue::Present,
        bindings: &[
            qualified(M5AppearanceRow::ThemeDark),
            qualified(M5AppearanceRow::ThemeLight),
            qualified(M5AppearanceRow::ThemeHighContrast),
            qualified(M5AppearanceRow::DensityCompact),
            qualified(M5AppearanceRow::DensityStandard),
            qualified(M5AppearanceRow::DensityComfortable),
            qualified(M5AppearanceRow::ReducedMotion),
            qualified(M5AppearanceRow::LiveAppearanceChange),
        ],
    },
    // Result-grid rows. Lifecycle-bearing; dense data rows.
    SurfaceSeed {
        surface_id: "surface:data_api.result_grid_row",
        surface_family: M5AppearanceSurfaceFamily::ResultGridRow,
        descriptor_revision_ref: "surface-rev:data_api.result_grid_row:2026.06.01-01",
        primary_label_ref: "label:data_api.result_grid_row:primary",
        appearance_anchor_ref: "appearance:anchor:data_api:result_grid_row",
        accessibility_note: "Cached, stale, and partial row badges stay readable at every density and contrast.",
        semantic_salience: M5SemanticSalience::LifecycleBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenAffordance::ExactTargetPreserved,
        boundary_cue: M5BoundaryCue::Present,
        bindings: &[
            qualified(M5AppearanceRow::ThemeDark),
            qualified(M5AppearanceRow::ThemeLight),
            qualified(M5AppearanceRow::ThemeHighContrast),
            qualified(M5AppearanceRow::DensityCompact),
            qualified(M5AppearanceRow::DensityStandard),
            qualified(M5AppearanceRow::DensityComfortable),
            qualified(M5AppearanceRow::ReducedMotion),
            qualified(M5AppearanceRow::LiveAppearanceChange),
        ],
    },
    // Profiler panel. Informational; diagnostic timeline.
    SurfaceSeed {
        surface_id: "surface:profiler.capture_panel",
        surface_family: M5AppearanceSurfaceFamily::ProfilerPanel,
        descriptor_revision_ref: "surface-rev:profiler.capture_panel:2026.06.01-01",
        primary_label_ref: "label:profiler.capture_panel:primary",
        appearance_anchor_ref: "appearance:anchor:profiler:capture_panel",
        accessibility_note: "Flame intensities keep a legible legend across themes; reduced motion stills the playhead sweep.",
        semantic_salience: M5SemanticSalience::Informational,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenAffordance::NotApplicable,
        boundary_cue: M5BoundaryCue::NotApplicable,
        bindings: &[
            qualified(M5AppearanceRow::ThemeDark),
            qualified(M5AppearanceRow::ThemeLight),
            qualified(M5AppearanceRow::ThemeHighContrast),
            qualified(M5AppearanceRow::DensityCompact),
            qualified(M5AppearanceRow::DensityStandard),
            qualified(M5AppearanceRow::DensityComfortable),
            qualified(M5AppearanceRow::ReducedMotion),
            qualified(M5AppearanceRow::LiveAppearanceChange),
        ],
    },
    // Trace panel. Informational; replay timeline.
    SurfaceSeed {
        surface_id: "surface:trace.replay_panel",
        surface_family: M5AppearanceSurfaceFamily::TracePanel,
        descriptor_revision_ref: "surface-rev:trace.replay_panel:2026.06.01-01",
        primary_label_ref: "label:trace.replay_panel:primary",
        appearance_anchor_ref: "appearance:anchor:trace:replay_panel",
        accessibility_note: "Replay timeline labels survive every theme and density; reduced motion stills the scrub animation.",
        semantic_salience: M5SemanticSalience::Informational,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenAffordance::NotApplicable,
        boundary_cue: M5BoundaryCue::NotApplicable,
        bindings: &[
            qualified(M5AppearanceRow::ThemeDark),
            qualified(M5AppearanceRow::ThemeLight),
            qualified(M5AppearanceRow::ThemeHighContrast),
            qualified(M5AppearanceRow::DensityCompact),
            qualified(M5AppearanceRow::DensityStandard),
            qualified(M5AppearanceRow::DensityComfortable),
            qualified(M5AppearanceRow::ReducedMotion),
            qualified(M5AppearanceRow::LiveAppearanceChange),
        ],
    },
    // Pipeline card. Severity-bearing; status card.
    SurfaceSeed {
        surface_id: "surface:review.pipeline_card",
        surface_family: M5AppearanceSurfaceFamily::PipelineCard,
        descriptor_revision_ref: "surface-rev:review.pipeline_card:2026.06.01-01",
        primary_label_ref: "label:review.pipeline_card:primary",
        appearance_anchor_ref: "appearance:anchor:review:pipeline_card",
        accessibility_note: "Pass, fail, and blocked cards keep an icon and label in every theme, density, and reduced motion.",
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenAffordance::ExactTargetPreserved,
        boundary_cue: M5BoundaryCue::Present,
        bindings: &[
            qualified(M5AppearanceRow::ThemeDark),
            qualified(M5AppearanceRow::ThemeLight),
            qualified(M5AppearanceRow::ThemeHighContrast),
            qualified(M5AppearanceRow::DensityCompact),
            qualified(M5AppearanceRow::DensityStandard),
            qualified(M5AppearanceRow::DensityComfortable),
            qualified(M5AppearanceRow::ReducedMotion),
            qualified(M5AppearanceRow::LiveAppearanceChange),
        ],
    },
    // Preview-route badge. Lifecycle-bearing; live preview badge.
    SurfaceSeed {
        surface_id: "surface:preview.route_badge",
        surface_family: M5AppearanceSurfaceFamily::PreviewRouteBadge,
        descriptor_revision_ref: "surface-rev:preview.route_badge:2026.06.01-01",
        primary_label_ref: "label:preview.route_badge:primary",
        appearance_anchor_ref: "appearance:anchor:preview:route_badge",
        accessibility_note: "Preview-only and stale badges spell out the state in text across every theme and density.",
        semantic_salience: M5SemanticSalience::LifecycleBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenAffordance::ExactTargetPreserved,
        boundary_cue: M5BoundaryCue::Present,
        bindings: &[
            qualified(M5AppearanceRow::ThemeDark),
            qualified(M5AppearanceRow::ThemeLight),
            qualified(M5AppearanceRow::ThemeHighContrast),
            qualified(M5AppearanceRow::DensityCompact),
            qualified(M5AppearanceRow::DensityStandard),
            qualified(M5AppearanceRow::DensityComfortable),
            qualified(M5AppearanceRow::ReducedMotion),
            qualified(M5AppearanceRow::LiveAppearanceChange),
        ],
    },
    // Docs / browser pane. Informational; embedded provider declares a gap.
    SurfaceSeed {
        surface_id: "surface:docs_browser.pane",
        surface_family: M5AppearanceSurfaceFamily::DocsBrowserPane,
        descriptor_revision_ref: "surface-rev:docs_browser.pane:2026.06.01-01",
        primary_label_ref: "label:docs_browser.pane:primary",
        appearance_anchor_ref: "appearance:anchor:docs_browser:pane",
        accessibility_note: "Shell chrome around embedded content qualifies on every row; provider-only repaint gaps are declared, not hidden.",
        semantic_salience: M5SemanticSalience::Informational,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenAffordance::ExactTargetPreserved,
        boundary_cue: M5BoundaryCue::NotApplicable,
        bindings: &[
            qualified(M5AppearanceRow::ThemeDark),
            qualified(M5AppearanceRow::ThemeLight),
            qualified(M5AppearanceRow::ThemeHighContrast),
            qualified(M5AppearanceRow::DensityCompact),
            qualified(M5AppearanceRow::DensityStandard),
            qualified(M5AppearanceRow::DensityComfortable),
            qualified(M5AppearanceRow::ReducedMotion),
            declared_capture_gap(
                M5AppearanceRow::LiveAppearanceChange,
                "embedded_provider_repaints_on_its_own_cadence_so_live_change_capture_is_provider_attributed",
            ),
        ],
    },
    // Companion surface. Trust-bearing; provider-backed; declares a gap.
    SurfaceSeed {
        surface_id: "surface:companion.surface",
        surface_family: M5AppearanceSurfaceFamily::CompanionSurface,
        descriptor_revision_ref: "surface-rev:companion.surface:2026.06.01-01",
        primary_label_ref: "label:companion.surface:primary",
        appearance_anchor_ref: "appearance:anchor:companion:surface",
        accessibility_note: "Presence and handoff cues keep an icon and label across themes; provider-only motion gaps are declared honestly.",
        semantic_salience: M5SemanticSalience::TrustBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenAffordance::ExactTargetPreserved,
        boundary_cue: M5BoundaryCue::Present,
        bindings: &[
            qualified(M5AppearanceRow::ThemeDark),
            qualified(M5AppearanceRow::ThemeLight),
            qualified(M5AppearanceRow::ThemeHighContrast),
            qualified(M5AppearanceRow::DensityCompact),
            qualified(M5AppearanceRow::DensityStandard),
            qualified(M5AppearanceRow::DensityComfortable),
            declared_capture_gap(
                M5AppearanceRow::ReducedMotion,
                "companion_provider_drives_its_own_motion_so_the_reduced_motion_downgrade_is_provider_attributed",
            ),
            qualified(M5AppearanceRow::LiveAppearanceChange),
        ],
    },
    // Sync status surface. Severity-bearing; conflict status.
    SurfaceSeed {
        surface_id: "surface:sync.status_surface",
        surface_family: M5AppearanceSurfaceFamily::SyncStatusSurface,
        descriptor_revision_ref: "surface-rev:sync.status_surface:2026.06.01-01",
        primary_label_ref: "label:sync.status_surface:primary",
        appearance_anchor_ref: "appearance:anchor:sync:status_surface",
        accessibility_note: "Sync-pending and conflict states keep an icon and label across every theme, density, and reduced motion.",
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenAffordance::NotApplicable,
        boundary_cue: M5BoundaryCue::Present,
        bindings: &[
            qualified(M5AppearanceRow::ThemeDark),
            qualified(M5AppearanceRow::ThemeLight),
            qualified(M5AppearanceRow::ThemeHighContrast),
            qualified(M5AppearanceRow::DensityCompact),
            qualified(M5AppearanceRow::DensityStandard),
            qualified(M5AppearanceRow::DensityComfortable),
            qualified(M5AppearanceRow::ReducedMotion),
            qualified(M5AppearanceRow::LiveAppearanceChange),
        ],
    },
    // Offboarding surface. Severity-bearing; destructive lifecycle.
    SurfaceSeed {
        surface_id: "surface:offboarding.surface",
        surface_family: M5AppearanceSurfaceFamily::OffboardingSurface,
        descriptor_revision_ref: "surface-rev:offboarding.surface:2026.06.01-01",
        primary_label_ref: "label:offboarding.surface:primary",
        appearance_anchor_ref: "appearance:anchor:offboarding:surface",
        accessibility_note: "Destructive and blocked states keep an icon and label across every theme, density, and reduced motion.",
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenAffordance::NotApplicable,
        boundary_cue: M5BoundaryCue::Present,
        bindings: &[
            qualified(M5AppearanceRow::ThemeDark),
            qualified(M5AppearanceRow::ThemeLight),
            qualified(M5AppearanceRow::ThemeHighContrast),
            qualified(M5AppearanceRow::DensityCompact),
            qualified(M5AppearanceRow::DensityStandard),
            qualified(M5AppearanceRow::DensityComfortable),
            qualified(M5AppearanceRow::ReducedMotion),
            qualified(M5AppearanceRow::LiveAppearanceChange),
        ],
    },
];

fn build_binding_from_seed(seed: &SurfaceSeed, binding_seed: &BindingSeed) -> M5AppearanceBinding {
    let row = binding_seed.row;
    let qualified = binding_seed.qualification_status.projects_evidence();
    let high_salience = seed.semantic_salience.is_high_salience();
    let marketed_on_row = !matches!(
        binding_seed.qualification_status,
        M5QualificationStatus::NotApplicable | M5QualificationStatus::PlatformOmitted
    );

    M5AppearanceBinding {
        row,
        dimension: row.canonical_dimension(),
        qualification_status: binding_seed.qualification_status,
        marketed_on_row,
        projected_screenshot_pack_ref: qualified
            .then(|| format!("capture:{}:{}", seed.surface_id, row.as_str())),
        projected_contrast_result: (qualified && row.requires_contrast())
            .then(|| contrast_for_row(row)),
        projected_focus_visibility: qualified.then_some(M5FocusVisibility::VisibleFocusRing),
        projected_motion_treatment: (qualified && row.requires_motion_treatment())
            .then_some(M5MotionTreatment::Reduced),
        projected_state_semantics: qualified.then_some(M5StateSemantics::Preserved),
        projected_keyboard_check: qualified.then_some(M5CheckOutcome::Pass),
        projected_screen_reader_check: qualified.then_some(M5CheckOutcome::Pass),
        projected_reopen_affordance: qualified.then_some(seed.reopen),
        projected_boundary_cue: (qualified && high_salience).then_some(seed.boundary_cue),
        projected_layout_integrity: (qualified && row.requires_layout_integrity())
            .then_some(M5LayoutIntegrity::Intact),
        evidence_freshness: qualified.then_some(M5EvidenceFreshness::Fresh),
        evidence_captured_at: qualified.then(|| GENERATED_AT.to_owned()),
        narrowing_reason: binding_seed.narrowing_reason.map(str::to_owned),
        note: binding_seed.note.map(str::to_owned),
    }
}

fn build_surface_from_seed(seed: &SurfaceSeed) -> M5AppearanceQualificationRow {
    let descriptor = M5AppearanceDescriptor {
        surface_id: seed.surface_id.to_owned(),
        surface_family: seed.surface_family,
        descriptor_revision_ref: seed.descriptor_revision_ref.to_owned(),
        primary_label_ref: seed.primary_label_ref.to_owned(),
        appearance_anchor_ref: seed.appearance_anchor_ref.to_owned(),
        accessibility_note: seed.accessibility_note.to_owned(),
        semantic_salience: seed.semantic_salience,
        lifecycle_label: seed.lifecycle_label,
        marketed_on_desktop_rows: true,
        registered_on_appearance_session: true,
    };
    let bindings: Vec<M5AppearanceBinding> = seed
        .bindings
        .iter()
        .map(|binding_seed| build_binding_from_seed(seed, binding_seed))
        .collect();
    build_m5_appearance_row(descriptor, bindings)
}

/// Seeded audit builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/ux/m5/dark-light-hc-density-reduced-motion/`.
pub fn seeded_m5_appearance_qualification_audit() -> M5AppearanceQualificationReport {
    let surfaces = SURFACE_SEEDS.iter().map(build_surface_from_seed).collect();
    build_m5_appearance_qualification_audit(surfaces)
}
