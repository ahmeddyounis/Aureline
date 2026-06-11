//! Desktop-platform, multi-window, power-state, and handoff qualification
//! audit for the M5 depth surfaces.
//!
//! The M5 depth lanes ship new panes — notebook cell chrome, result-grid
//! rows, profiler and trace panels, pipeline cards, preview-route badges,
//! docs/browser panes, companion surfaces, sync status, offboarding, and
//! incident packets — that are easy to certify on a single window, a single
//! monitor, one DPI class, and one uninterrupted happy path, then quietly
//! break continuity the moment a user hits the real desktop conditions
//! Aureline already claims. This module carries the stable v1 shell promise
//! forward into those lanes: every marketed M5 surface MUST behave like a
//! first-class desktop citizen — preserving authoritative-object reopen,
//! placeholder honesty, layout continuity, interruption safety, and
//! authority/handoff context — across the desktop scenario rows the M5 lanes
//! must pass, and it MUST never let a window, display, power-state, or
//! handoff change silently lose target identity or authority context.
//!
//! The audit projects, for each registered M5 surface, the canonical desktop
//! descriptor against the qualification result the surface actually certifies
//! for each of the nine desktop scenario rows the M5 lanes must pass:
//!
//! - `multi_window`
//! - `multi_monitor`
//! - `mixed_dpi`
//! - `suspend_resume`
//! - `battery_saver`
//! - `thermal_pressure`
//! - `deep_link`
//! - `file_association`
//! - `system_open_return`
//!
//! The resulting [`M5DesktopQualificationReport`] is the canonical truth
//! object for the M5 desktop-and-handoff qualification lane. It is consumed
//! by:
//!
//! - the live shell platform-conformance inspector (so the in-product audit
//!   quotes the same per-row findings the CLI prints);
//! - the headless inspector (`aureline_shell_m5_desktop_conformance`), which
//!   is the only mint-from-truth path for the JSON fixtures checked in under
//!   `fixtures/platform/m5_depth_surfaces/`;
//! - the support-export wrapper that lets a reviewer pivot from a support
//!   case to the row that flagged a stale or red desktop result;
//! - the markdown audit under
//!   `artifacts/ux/m5/desktop-conformance/m5_desktop_conformance_audit.md`
//!   (rendered from the same seed); and
//! - the cross-surface hardening matrix and release-center packets, which
//!   ingest the audit directly when qualifying or narrowing a marketed M5 row
//!   whose desktop evidence is stale or red.
//!
//! Acceptance invariants enforced by the validator:
//!
//! 1. Every registered M5 surface must declare a qualification binding for
//!    each of the nine desktop scenario rows.
//! 2. Every surface must carry a canonical reopen anchor, a non-empty
//!    continuity note, at least one claimed desktop profile, and a flag
//!    asserting it rides the shared platform-conformance harness; a missing
//!    anchor, missing note, no claimed profile, or a surface that drives its
//!    own window/restore path outside the harness is a blocker.
//! 3. A qualified row must carry the captured evidence the row requires — an
//!    evidence pack, reopen fidelity, layout continuity, interruption safety,
//!    and placeholder honesty for every row; a background-adaptation result
//!    on the power rows; and a handoff-reason and authority-context result on
//!    the handoff rows. A red result (a lost reopen target, broken layout,
//!    corrupted interruption, a misleading placeholder, a lost authority
//!    context, background work that is not throttled before corruption, a
//!    dropped handoff reason, or a hidden high-risk boundary cue) is a
//!    blocker.
//! 4. A surface that drives a row through an ad-hoc local window/restore path
//!    outside the shared platform-conformance harness
//!    (`unqualified_local_platform_path`), and a marketed row claimed with no
//!    captured evidence (`missing_evidence`), are blockers.
//! 5. Stale desktop evidence on a marketed row is a blocker, so release
//!    tooling can narrow a marketed M5 row on the affected profiles instead
//!    of shipping it as implicitly stable.
//! 6. At least one surface must qualify each of the nine scenario rows so the
//!    audit cannot regress into a single-window, single-monitor, one-DPI,
//!    happy-path view.
//!
//! All identifiers, refs, and label strings are deterministic so the
//! checked-in fixtures under `fixtures/platform/m5_depth_surfaces/` are
//! bit-for-bit equal to the seeded report returned by
//! [`seeded_m5_desktop_qualification_audit`].

use serde::{Deserialize, Serialize};

#[cfg(test)]
mod tests;

/// Schema version exported with every M5 desktop-qualification record.
pub const M5_DESKTOP_SCHEMA_VERSION: u32 = 1;

/// Stable shared contract ref consumed by every M5 desktop row.
pub const M5_DESKTOP_SHARED_CONTRACT_REF: &str = "shell:m5_desktop_conformance:v1";

/// Stable record kind for [`M5DesktopQualificationReport`] payloads.
pub const M5_DESKTOP_REPORT_RECORD_KIND: &str = "shell_m5_desktop_qualification_report_record";

/// Stable record kind for [`M5DesktopQualificationRow`] payloads.
pub const M5_DESKTOP_ROW_RECORD_KIND: &str = "shell_m5_desktop_qualification_row_record";

/// Stable record kind for [`M5DesktopSupportExport`] payloads.
pub const M5_DESKTOP_SUPPORT_EXPORT_RECORD_KIND: &str =
    "shell_m5_desktop_qualification_support_export_record";

/// Stable report id quoted across surfaces.
pub const M5_DESKTOP_REPORT_ID: &str = "shell:m5_desktop_conformance:audit:v1";

/// Stable support-export id quoted in the published wrapper.
pub const M5_DESKTOP_SUPPORT_EXPORT_ID: &str = "support-export:m5-desktop-conformance:001";

/// Source schema ref for the canonical desktop-qualification contract.
pub const M5_DESKTOP_SOURCE_SCHEMA_REF: &str =
    "schemas/platform/m5-surface-desktop-qualification.schema.json";

/// Path of the published markdown audit artifact.
pub const M5_DESKTOP_PUBLISHED_REPORT_REF: &str =
    "artifacts/ux/m5/desktop-conformance/m5_desktop_conformance_audit.md";

/// Path of the published companion doc.
pub const M5_DESKTOP_PUBLISHED_DOC_REF: &str = "docs/m5/desktop-and-handoff-parity.md";

/// Generation timestamp captured in every seeded record.
const GENERATED_AT: &str = "2026-06-11T00:00:00Z";

/// One M5 depth surface whose desktop scenario rows the audit qualifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DesktopSurfaceFamily {
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
    /// Incident / support packet surfaces.
    IncidentPacket,
}

impl M5DesktopSurfaceFamily {
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
            Self::IncidentPacket => "incident_packet",
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
            Self::IncidentPacket => "Incident packet",
        }
    }
}

/// A claimed desktop platform profile the audit certifies a surface on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DesktopProfile {
    /// macOS desktop profile.
    Macos,
    /// Windows desktop profile.
    Windows,
    /// Linux desktop profile.
    Linux,
}

impl M5DesktopProfile {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Macos => "macos",
            Self::Windows => "windows",
            Self::Linux => "linux",
        }
    }

    /// Returns the three claimed desktop profiles in canonical order.
    pub const fn all() -> [Self; 3] {
        [Self::Macos, Self::Windows, Self::Linux]
    }
}

/// The desktop dimension a scenario row belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DesktopDimension {
    /// Window topology (multi-window, multi-monitor, mixed-DPI).
    WindowTopology,
    /// Power state (suspend/resume, battery saver, thermal pressure).
    PowerState,
    /// Handoff routing (deep link, file association, system-open return).
    Handoff,
}

impl M5DesktopDimension {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WindowTopology => "window_topology",
            Self::PowerState => "power_state",
            Self::Handoff => "handoff",
        }
    }
}

/// One of the nine desktop scenario rows the audit certifies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DesktopRow {
    /// The surface stays coherent across multiple top-level windows.
    MultiWindow,
    /// The surface stays coherent across multiple monitors.
    MultiMonitor,
    /// The surface stays coherent across mixed-DPI displays.
    MixedDpi,
    /// The surface reopens truthfully after a suspend/resume cycle.
    SuspendResume,
    /// The surface adapts under battery saver without corrupting work.
    BatterySaver,
    /// The surface adapts under thermal pressure without corrupting work.
    ThermalPressure,
    /// The surface reopens the exact target from a deep link.
    DeepLink,
    /// The surface reopens the exact target from a file association.
    FileAssociation,
    /// The surface returns truthfully from a system-open or browser return.
    SystemOpenReturn,
}

impl M5DesktopRow {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MultiWindow => "multi_window",
            Self::MultiMonitor => "multi_monitor",
            Self::MixedDpi => "mixed_dpi",
            Self::SuspendResume => "suspend_resume",
            Self::BatterySaver => "battery_saver",
            Self::ThermalPressure => "thermal_pressure",
            Self::DeepLink => "deep_link",
            Self::FileAssociation => "file_association",
            Self::SystemOpenReturn => "system_open_return",
        }
    }

    /// Reviewer-facing label.
    pub const fn display_label(self) -> &'static str {
        match self {
            Self::MultiWindow => "Multi-window",
            Self::MultiMonitor => "Multi-monitor",
            Self::MixedDpi => "Mixed-DPI",
            Self::SuspendResume => "Suspend/resume",
            Self::BatterySaver => "Battery saver",
            Self::ThermalPressure => "Thermal pressure",
            Self::DeepLink => "Deep link",
            Self::FileAssociation => "File association",
            Self::SystemOpenReturn => "System-open return",
        }
    }

    /// Returns the nine required scenario rows in canonical order.
    pub const fn required_rows() -> [Self; 9] {
        [
            Self::MultiWindow,
            Self::MultiMonitor,
            Self::MixedDpi,
            Self::SuspendResume,
            Self::BatterySaver,
            Self::ThermalPressure,
            Self::DeepLink,
            Self::FileAssociation,
            Self::SystemOpenReturn,
        ]
    }

    /// Canonical desktop dimension this row certifies.
    pub const fn canonical_dimension(self) -> M5DesktopDimension {
        match self {
            Self::MultiWindow | Self::MultiMonitor | Self::MixedDpi => {
                M5DesktopDimension::WindowTopology
            }
            Self::SuspendResume | Self::BatterySaver | Self::ThermalPressure => {
                M5DesktopDimension::PowerState
            }
            Self::DeepLink | Self::FileAssociation | Self::SystemOpenReturn => {
                M5DesktopDimension::Handoff
            }
        }
    }

    /// `true` for the power rows that must prove background work is throttled
    /// before it can corrupt foreground work.
    pub const fn requires_background_adaptation(self) -> bool {
        matches!(self, Self::BatterySaver | Self::ThermalPressure)
    }

    /// `true` for the handoff rows that must preserve a handoff reason.
    pub const fn requires_handoff_reason(self) -> bool {
        matches!(
            self,
            Self::DeepLink | Self::FileAssociation | Self::SystemOpenReturn
        )
    }

    /// `true` for the handoff rows that must preserve an authority context.
    pub const fn requires_authority_context(self) -> bool {
        matches!(
            self,
            Self::DeepLink | Self::FileAssociation | Self::SystemOpenReturn
        )
    }
}

/// Qualification status a surface reports for one desktop scenario row.
///
/// Only `Qualified` rows project captured evidence and are drift/red
/// checked. `ExplicitlyNarrowed`, `NotApplicable`, `PlatformOmitted`, and
/// `DeclaredCaptureGap` rows are accepted as long as they carry a
/// `narrowing_reason`. `UnqualifiedLocalPlatformPath` (an ad-hoc
/// window/restore path outside the shared harness) and `MissingEvidence` are
/// blocking.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DesktopQualificationStatus {
    /// The row is qualified with captured desktop evidence.
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
    /// The surface drives this row through an ad-hoc local window/restore
    /// path outside the shared platform-conformance harness. Always a
    /// blocker.
    UnqualifiedLocalPlatformPath,
    /// A marketed row is claimed with no captured evidence. Always a blocker.
    MissingEvidence,
}

impl M5DesktopQualificationStatus {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Qualified => "qualified",
            Self::ExplicitlyNarrowed => "explicitly_narrowed",
            Self::NotApplicable => "not_applicable",
            Self::PlatformOmitted => "platform_omitted",
            Self::DeclaredCaptureGap => "declared_capture_gap",
            Self::UnqualifiedLocalPlatformPath => "unqualified_local_platform_path",
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

/// Whether the authoritative-object reopen affordance survives a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ReopenFidelity {
    /// The exact-target reopen affordance is preserved.
    ExactTargetPreserved,
    /// The exact-target reopen affordance is lost. Always a blocker.
    Lost,
    /// The surface has no reopen affordance to preserve.
    NotApplicable,
}

impl M5ReopenFidelity {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactTargetPreserved => "exact_target_preserved",
            Self::Lost => "lost",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether layout stays continuous across a desktop topology or power change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5LayoutContinuity {
    /// Layout stays continuous across the change.
    Preserved,
    /// Layout is broken by the change. Always a blocker.
    Broken,
}

impl M5LayoutContinuity {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Broken => "broken",
        }
    }
}

/// Whether typing, save, and reopen survive the interruption a row models.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5InterruptionSafety {
    /// Foreground work (typing, save, reopen) is preserved.
    Safe,
    /// Foreground work is corrupted by the interruption. Always a blocker.
    Corrupted,
}

impl M5InterruptionSafety {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Safe => "safe",
            Self::Corrupted => "corrupted",
        }
    }
}

/// Whether a placeholder shown when a target is gone is honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5PlaceholderHonesty {
    /// The placeholder honestly names the missing or expired target.
    Honest,
    /// The placeholder pretends a missing target is live. Always a blocker.
    Misleading,
    /// The surface shows no placeholder for this row.
    NotApplicable,
}

impl M5PlaceholderHonesty {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Honest => "honest",
            Self::Misleading => "misleading",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether the authority / identity context survives a handoff row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5AuthorityContext {
    /// The authority / identity context is preserved across the handoff.
    Preserved,
    /// The authority / identity context is lost. Always a blocker.
    Lost,
    /// The surface carries no authority context to preserve.
    NotApplicable,
}

impl M5AuthorityContext {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Lost => "lost",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether battery or thermal adaptation throttles background work before it
/// can corrupt foreground work.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BackgroundAdaptation {
    /// Background work is throttled before it can corrupt foreground work.
    ThrottledBeforeCorruption,
    /// Background work is not throttled and can corrupt work. Always a
    /// blocker on a power row.
    NotThrottled,
    /// The surface runs no background work to throttle.
    NotApplicable,
}

impl M5BackgroundAdaptation {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ThrottledBeforeCorruption => "throttled_before_corruption",
            Self::NotThrottled => "not_throttled",
            Self::NotApplicable => "not_applicable",
        }
    }
}

/// Whether the handoff reason survives a deep-link / file-association /
/// system-open return row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5HandoffReason {
    /// The handoff reason is preserved through the route.
    Preserved,
    /// The handoff reason is dropped. Always a blocker on a handoff row.
    Dropped,
    /// The surface carries no handoff reason for this row.
    NotApplicable,
}

impl M5HandoffReason {
    /// Stable schema token.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Preserved => "preserved",
            Self::Dropped => "dropped",
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

/// Freshness of the captured desktop evidence.
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
/// "high-salience": no desktop scenario may hide that meaning, so the audit
/// requires a present high-risk boundary cue on every qualified row.
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

/// Canonical descriptor for one M5 surface's desktop contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesktopDescriptor {
    /// Stable surface id (e.g. `surface:notebook.cell_chrome`).
    pub surface_id: String,
    /// Surface family the descriptor belongs to.
    pub surface_family: M5DesktopSurfaceFamily,
    /// Descriptor revision the audit was produced against.
    pub descriptor_revision_ref: String,
    /// Canonical primary label ref.
    pub primary_label_ref: String,
    /// Canonical authoritative-object reopen anchor ref the audit can reopen
    /// the surface's exact target from after a topology/power/handoff change.
    pub reopen_anchor_ref: String,
    /// Continuity note retained on the descriptor. MUST be non-empty.
    pub continuity_note: String,
    /// Pinned semantic salience.
    pub semantic_salience: M5SemanticSalience,
    /// Pinned surface lifecycle label.
    pub lifecycle_label: M5SurfaceLifecycle,
    /// Claimed desktop profiles. MUST be non-empty.
    pub claimed_desktop_profiles: Vec<M5DesktopProfile>,
    /// `true` when the surface is marketed on desktop scenario rows and
    /// therefore must pass the claimed matrix or narrow accordingly.
    pub marketed_on_desktop_rows: bool,
    /// `true` once the surface rides the shared platform-conformance harness
    /// and does not drive its own window/restore path. MUST be `true`.
    pub registered_on_platform_conformance: bool,
}

impl M5DesktopDescriptor {
    /// `true` when this surface's salience makes it high-salience for the
    /// audit.
    pub const fn is_high_salience(&self) -> bool {
        self.semantic_salience.is_high_salience()
    }
}

/// Per-row qualification binding a surface reports for one scenario row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesktopBinding {
    /// Scenario row this binding covers.
    pub row: M5DesktopRow,
    /// Desktop dimension projected for the row. MUST equal the row's
    /// canonical dimension.
    pub dimension: M5DesktopDimension,
    /// Qualification status the surface reports.
    pub qualification_status: M5DesktopQualificationStatus,
    /// `true` when the surface is marketed on this row.
    pub marketed_on_row: bool,
    /// Captured evidence-pack ref (`None` for non-qualified rows).
    pub projected_evidence_pack_ref: Option<String>,
    /// Captured reopen fidelity (`None` for non-qualified rows).
    pub projected_reopen_fidelity: Option<M5ReopenFidelity>,
    /// Captured layout continuity (`None` for non-qualified rows).
    pub projected_layout_continuity: Option<M5LayoutContinuity>,
    /// Captured interruption safety (`None` for non-qualified rows).
    pub projected_interruption_safety: Option<M5InterruptionSafety>,
    /// Captured placeholder honesty (`None` for non-qualified rows).
    pub projected_placeholder_honesty: Option<M5PlaceholderHonesty>,
    /// Captured authority context (`None` unless the row requires it).
    pub projected_authority_context: Option<M5AuthorityContext>,
    /// Captured background adaptation (`None` unless the row requires it).
    pub projected_background_adaptation: Option<M5BackgroundAdaptation>,
    /// Captured handoff reason (`None` unless the row requires it).
    pub projected_handoff_reason: Option<M5HandoffReason>,
    /// Captured high-risk boundary cue (`None` for non-qualified rows).
    pub projected_boundary_cue: Option<M5BoundaryCue>,
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
pub enum M5DesktopBlockingFinding {
    /// A surface drives a row through an ad-hoc local window/restore path
    /// outside the shared platform-conformance harness.
    UnqualifiedLocalPlatformPath {
        /// Surface that exposes the gap.
        surface_id: String,
        /// Row that exposes the gap.
        row: M5DesktopRow,
    },
    /// A marketed row is claimed with no captured evidence.
    MissingEvidence {
        surface_id: String,
        row: M5DesktopRow,
    },
    /// A qualified row is missing its captured evidence pack.
    MissingEvidencePack {
        surface_id: String,
        row: M5DesktopRow,
    },
    /// A row loses the authoritative-object reopen target.
    ReopenTargetLost {
        surface_id: String,
        row: M5DesktopRow,
    },
    /// A row breaks layout continuity.
    LayoutContinuityBroken {
        surface_id: String,
        row: M5DesktopRow,
    },
    /// A row corrupts foreground work (typing, save, reopen).
    InterruptionUnsafe {
        surface_id: String,
        row: M5DesktopRow,
    },
    /// A row shows a placeholder that pretends a missing target is live.
    PlaceholderMisleading {
        surface_id: String,
        row: M5DesktopRow,
    },
    /// A handoff row loses the authority / identity context.
    AuthorityContextLost {
        surface_id: String,
        row: M5DesktopRow,
    },
    /// A power row leaves background work un-throttled.
    BackgroundNotThrottled {
        surface_id: String,
        row: M5DesktopRow,
    },
    /// A handoff row drops the handoff reason.
    HandoffReasonDropped {
        surface_id: String,
        row: M5DesktopRow,
    },
    /// A row hides the high-risk boundary cue.
    BoundaryCueHidden {
        surface_id: String,
        row: M5DesktopRow,
    },
    /// A marketed row carries stale desktop evidence.
    StaleEvidenceOnMarketedRow {
        surface_id: String,
        row: M5DesktopRow,
    },
    /// A binding projects a desktop dimension that disagrees with the row's
    /// canonical dimension.
    DimensionDrift {
        surface_id: String,
        row: M5DesktopRow,
        /// Projected dimension.
        projected_dimension: M5DesktopDimension,
    },
    /// A non-qualified row is missing the `narrowing_reason`.
    MissingNarrowingReason {
        surface_id: String,
        row: M5DesktopRow,
        qualification_status: M5DesktopQualificationStatus,
    },
    /// A qualified row is missing a captured-evidence field it requires.
    MissingProjection {
        surface_id: String,
        row: M5DesktopRow,
        /// Name of the missing projection field.
        field: String,
    },
    /// The descriptor carries no canonical reopen anchor.
    DescriptorMissingReopenAnchor { surface_id: String },
    /// The descriptor carries no continuity note.
    MissingContinuityNote { surface_id: String },
    /// The descriptor claims no desktop profile.
    MissingClaimedProfiles { surface_id: String },
    /// The surface drives its own window/restore path outside the shared
    /// platform-conformance harness.
    SurfaceNotOnPlatformConformance { surface_id: String },
}

impl M5DesktopBlockingFinding {
    /// Stable schema token for the finding class.
    pub fn class_token(&self) -> &'static str {
        match self {
            Self::UnqualifiedLocalPlatformPath { .. } => "unqualified_local_platform_path",
            Self::MissingEvidence { .. } => "missing_evidence",
            Self::MissingEvidencePack { .. } => "missing_evidence_pack",
            Self::ReopenTargetLost { .. } => "reopen_target_lost",
            Self::LayoutContinuityBroken { .. } => "layout_continuity_broken",
            Self::InterruptionUnsafe { .. } => "interruption_unsafe",
            Self::PlaceholderMisleading { .. } => "placeholder_misleading",
            Self::AuthorityContextLost { .. } => "authority_context_lost",
            Self::BackgroundNotThrottled { .. } => "background_not_throttled",
            Self::HandoffReasonDropped { .. } => "handoff_reason_dropped",
            Self::BoundaryCueHidden { .. } => "boundary_cue_hidden",
            Self::StaleEvidenceOnMarketedRow { .. } => "stale_evidence_on_marketed_row",
            Self::DimensionDrift { .. } => "dimension_drift",
            Self::MissingNarrowingReason { .. } => "missing_narrowing_reason",
            Self::MissingProjection { .. } => "missing_projection",
            Self::DescriptorMissingReopenAnchor { .. } => "descriptor_missing_reopen_anchor",
            Self::MissingContinuityNote { .. } => "missing_continuity_note",
            Self::MissingClaimedProfiles { .. } => "missing_claimed_profiles",
            Self::SurfaceNotOnPlatformConformance { .. } => "surface_not_on_platform_conformance",
        }
    }

    /// Returns the surface id this finding is attached to.
    pub fn surface_id(&self) -> &str {
        match self {
            Self::UnqualifiedLocalPlatformPath { surface_id, .. }
            | Self::MissingEvidence { surface_id, .. }
            | Self::MissingEvidencePack { surface_id, .. }
            | Self::ReopenTargetLost { surface_id, .. }
            | Self::LayoutContinuityBroken { surface_id, .. }
            | Self::InterruptionUnsafe { surface_id, .. }
            | Self::PlaceholderMisleading { surface_id, .. }
            | Self::AuthorityContextLost { surface_id, .. }
            | Self::BackgroundNotThrottled { surface_id, .. }
            | Self::HandoffReasonDropped { surface_id, .. }
            | Self::BoundaryCueHidden { surface_id, .. }
            | Self::StaleEvidenceOnMarketedRow { surface_id, .. }
            | Self::DimensionDrift { surface_id, .. }
            | Self::MissingNarrowingReason { surface_id, .. }
            | Self::MissingProjection { surface_id, .. }
            | Self::DescriptorMissingReopenAnchor { surface_id }
            | Self::MissingContinuityNote { surface_id }
            | Self::MissingClaimedProfiles { surface_id }
            | Self::SurfaceNotOnPlatformConformance { surface_id } => surface_id,
        }
    }

    /// Returns the row this finding is attached to, when row-scoped.
    pub fn row(&self) -> Option<M5DesktopRow> {
        match self {
            Self::UnqualifiedLocalPlatformPath { row, .. }
            | Self::MissingEvidence { row, .. }
            | Self::MissingEvidencePack { row, .. }
            | Self::ReopenTargetLost { row, .. }
            | Self::LayoutContinuityBroken { row, .. }
            | Self::InterruptionUnsafe { row, .. }
            | Self::PlaceholderMisleading { row, .. }
            | Self::AuthorityContextLost { row, .. }
            | Self::BackgroundNotThrottled { row, .. }
            | Self::HandoffReasonDropped { row, .. }
            | Self::BoundaryCueHidden { row, .. }
            | Self::StaleEvidenceOnMarketedRow { row, .. }
            | Self::DimensionDrift { row, .. }
            | Self::MissingNarrowingReason { row, .. }
            | Self::MissingProjection { row, .. } => Some(*row),
            Self::DescriptorMissingReopenAnchor { .. }
            | Self::MissingContinuityNote { .. }
            | Self::MissingClaimedProfiles { .. }
            | Self::SurfaceNotOnPlatformConformance { .. } => None,
        }
    }
}

/// One per-surface desktop-qualification row.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesktopQualificationRow {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the row.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, and support export.
    pub shared_contract_ref: String,
    /// Canonical descriptor for the surface.
    pub descriptor: M5DesktopDescriptor,
    /// Row-by-row qualification bindings, in canonical row order.
    pub bindings: Vec<M5DesktopBinding>,
    /// Blocking findings emitted against this row.
    pub blocking_findings: Vec<M5DesktopBlockingFinding>,
    /// `true` when the surface's descriptor classifies it as high-salience.
    pub high_salience: bool,
    /// `true` when the surface is marketed on desktop scenario rows.
    pub marketed: bool,
}

/// Per-class blocking-finding count summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesktopFindingSummary {
    /// Total blocking findings across the audit.
    pub total_blocking_findings: usize,
    /// Number of `unqualified_local_platform_path` findings.
    pub unqualified_local_platform_path: usize,
    /// Number of `missing_evidence` findings.
    pub missing_evidence: usize,
    /// Number of `missing_evidence_pack` findings.
    pub missing_evidence_pack: usize,
    /// Number of `reopen_target_lost` findings.
    pub reopen_target_lost: usize,
    /// Number of `layout_continuity_broken` findings.
    pub layout_continuity_broken: usize,
    /// Number of `interruption_unsafe` findings.
    pub interruption_unsafe: usize,
    /// Number of `placeholder_misleading` findings.
    pub placeholder_misleading: usize,
    /// Number of `authority_context_lost` findings.
    pub authority_context_lost: usize,
    /// Number of `background_not_throttled` findings.
    pub background_not_throttled: usize,
    /// Number of `handoff_reason_dropped` findings.
    pub handoff_reason_dropped: usize,
    /// Number of `boundary_cue_hidden` findings.
    pub boundary_cue_hidden: usize,
    /// Number of `stale_evidence_on_marketed_row` findings.
    pub stale_evidence_on_marketed_row: usize,
    /// Number of `dimension_drift` findings.
    pub dimension_drift: usize,
    /// Number of `missing_narrowing_reason` findings.
    pub missing_narrowing_reason: usize,
    /// Number of `missing_projection` findings.
    pub missing_projection: usize,
    /// Number of `descriptor_missing_reopen_anchor` findings.
    pub descriptor_missing_reopen_anchor: usize,
    /// Number of `missing_continuity_note` findings.
    pub missing_continuity_note: usize,
    /// Number of `missing_claimed_profiles` findings.
    pub missing_claimed_profiles: usize,
    /// Number of `surface_not_on_platform_conformance` findings.
    pub surface_not_on_platform_conformance: usize,
}

impl M5DesktopFindingSummary {
    fn empty() -> Self {
        Self {
            total_blocking_findings: 0,
            unqualified_local_platform_path: 0,
            missing_evidence: 0,
            missing_evidence_pack: 0,
            reopen_target_lost: 0,
            layout_continuity_broken: 0,
            interruption_unsafe: 0,
            placeholder_misleading: 0,
            authority_context_lost: 0,
            background_not_throttled: 0,
            handoff_reason_dropped: 0,
            boundary_cue_hidden: 0,
            stale_evidence_on_marketed_row: 0,
            dimension_drift: 0,
            missing_narrowing_reason: 0,
            missing_projection: 0,
            descriptor_missing_reopen_anchor: 0,
            missing_continuity_note: 0,
            missing_claimed_profiles: 0,
            surface_not_on_platform_conformance: 0,
        }
    }

    fn record(&mut self, finding: &M5DesktopBlockingFinding) {
        self.total_blocking_findings += 1;
        match finding {
            M5DesktopBlockingFinding::UnqualifiedLocalPlatformPath { .. } => {
                self.unqualified_local_platform_path += 1
            }
            M5DesktopBlockingFinding::MissingEvidence { .. } => self.missing_evidence += 1,
            M5DesktopBlockingFinding::MissingEvidencePack { .. } => self.missing_evidence_pack += 1,
            M5DesktopBlockingFinding::ReopenTargetLost { .. } => self.reopen_target_lost += 1,
            M5DesktopBlockingFinding::LayoutContinuityBroken { .. } => {
                self.layout_continuity_broken += 1
            }
            M5DesktopBlockingFinding::InterruptionUnsafe { .. } => self.interruption_unsafe += 1,
            M5DesktopBlockingFinding::PlaceholderMisleading { .. } => {
                self.placeholder_misleading += 1
            }
            M5DesktopBlockingFinding::AuthorityContextLost { .. } => {
                self.authority_context_lost += 1
            }
            M5DesktopBlockingFinding::BackgroundNotThrottled { .. } => {
                self.background_not_throttled += 1
            }
            M5DesktopBlockingFinding::HandoffReasonDropped { .. } => {
                self.handoff_reason_dropped += 1
            }
            M5DesktopBlockingFinding::BoundaryCueHidden { .. } => self.boundary_cue_hidden += 1,
            M5DesktopBlockingFinding::StaleEvidenceOnMarketedRow { .. } => {
                self.stale_evidence_on_marketed_row += 1
            }
            M5DesktopBlockingFinding::DimensionDrift { .. } => self.dimension_drift += 1,
            M5DesktopBlockingFinding::MissingNarrowingReason { .. } => {
                self.missing_narrowing_reason += 1
            }
            M5DesktopBlockingFinding::MissingProjection { .. } => self.missing_projection += 1,
            M5DesktopBlockingFinding::DescriptorMissingReopenAnchor { .. } => {
                self.descriptor_missing_reopen_anchor += 1
            }
            M5DesktopBlockingFinding::MissingContinuityNote { .. } => {
                self.missing_continuity_note += 1
            }
            M5DesktopBlockingFinding::MissingClaimedProfiles { .. } => {
                self.missing_claimed_profiles += 1
            }
            M5DesktopBlockingFinding::SurfaceNotOnPlatformConformance { .. } => {
                self.surface_not_on_platform_conformance += 1
            }
        }
    }
}

/// Per-row coverage summary.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesktopCoverageSummary {
    /// Row this summary covers.
    pub row: M5DesktopRow,
    /// Number of `qualified` rows on this scenario row.
    pub qualified_rows: usize,
    /// Number of `explicitly_narrowed` rows on this scenario row.
    pub explicitly_narrowed_rows: usize,
    /// Number of `not_applicable` rows on this scenario row.
    pub not_applicable_rows: usize,
    /// Number of `platform_omitted` rows on this scenario row.
    pub platform_omitted_rows: usize,
    /// Number of `declared_capture_gap` rows on this scenario row.
    pub declared_capture_gap_rows: usize,
    /// Number of `unqualified_local_platform_path` rows on this scenario row.
    pub unqualified_local_platform_path_rows: usize,
    /// Number of `missing_evidence` rows on this scenario row.
    pub missing_evidence_rows: usize,
}

impl M5DesktopCoverageSummary {
    fn narrowed_rows(&self) -> usize {
        self.explicitly_narrowed_rows
            + self.not_applicable_rows
            + self.platform_omitted_rows
            + self.declared_capture_gap_rows
    }
}

/// A single reopen-anchor index entry the audit publishes so platform QA,
/// docs, and release surfaces can reopen each M5 surface by its anchor.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesktopReopenAnchorEntry {
    /// Surface family the anchor belongs to.
    pub surface_family: M5DesktopSurfaceFamily,
    /// Surface id the anchor reopens.
    pub surface_id: String,
    /// Canonical reopen anchor ref.
    pub reopen_anchor_ref: String,
}

/// One marketed row release tooling should narrow because its desktop
/// evidence is stale or red.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesktopNarrowableRow {
    /// Surface id that must narrow.
    pub surface_id: String,
    /// Scenario row that must narrow.
    pub row: M5DesktopRow,
    /// Stable reason the row is narrowable.
    pub reason: String,
}

/// M5 desktop-and-handoff qualification audit report.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesktopQualificationReport {
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
    /// Required scenario rows, in canonical order.
    pub required_rows: Vec<M5DesktopRow>,
    /// Union of claimed desktop profiles across all surfaces, sorted.
    pub claimed_desktop_profiles: Vec<M5DesktopProfile>,
    /// Per-surface qualification rows, sorted by `descriptor.surface_id`.
    pub rows: Vec<M5DesktopQualificationRow>,
    /// Per-row coverage summary, in canonical row order.
    pub row_coverage: Vec<M5DesktopCoverageSummary>,
    /// Per-class blocking-finding summary.
    pub findings_summary: M5DesktopFindingSummary,
    /// Canonical reopen-anchor index, sorted by surface id.
    pub reopen_anchor_index: Vec<M5DesktopReopenAnchorEntry>,
    /// Number of registered M5 surfaces present.
    pub registered_surface_count: usize,
    /// Number of high-salience surfaces present.
    pub high_salience_surface_count: usize,
    /// Number of surfaces marketed on desktop scenario rows.
    pub marketed_surface_count: usize,
    /// Total scenario rows checked.
    pub desktop_rows_checked: usize,
    /// Marketed rows release tooling should narrow because their evidence is
    /// stale or red.
    pub narrowable_marketed_rows: Vec<M5DesktopNarrowableRow>,
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

impl M5DesktopQualificationReport {
    /// Returns `true` when every required row is qualified by at least one
    /// surface.
    pub fn every_required_row_qualified(&self) -> bool {
        for row in M5DesktopRow::required_rows() {
            let any_qualified = self.rows.iter().any(|surface| {
                surface.bindings.iter().any(|binding| {
                    binding.row == row
                        && binding.qualification_status == M5DesktopQualificationStatus::Qualified
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
            self.desktop_rows_checked,
            self.findings_summary.total_blocking_findings,
            self.report_clean,
        ));
        for coverage in &self.row_coverage {
            lines.push(format!(
                "{}: qualified={}, narrowed={}, unqualified={}, missing_evidence={}",
                coverage.row.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_local_platform_path_rows,
                coverage.missing_evidence_rows,
            ));
        }
        for surface in &self.rows {
            for finding in &surface.blocking_findings {
                lines.push(format!(
                    "blocker: {} -- {} -- {}",
                    finding.class_token(),
                    finding.surface_id(),
                    finding.row().map(M5DesktopRow::as_str).unwrap_or("surface"),
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
        out.push_str("# M5 desktop and handoff qualification audit\n");
        out.push('\n');
        out.push_str(
            "Generated from the seeded audit in\n\
             [`crate::m5_desktop_conformance`](../../../../crates/aureline-shell/src/m5_desktop_conformance/mod.rs).\n\
             Regenerate with:\n\n",
        );
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_desktop_conformance -- report-md > \\\n  artifacts/ux/m5/desktop-conformance/m5_desktop_conformance_audit.md\n",
        );
        out.push_str("```\n\n");

        out.push_str(&format!("- Report id: `{}`\n", self.report_id));
        out.push_str(&format!(
            "- Source schema ref: `{}`\n",
            self.source_schema_ref
        ));
        out.push_str(&format!(
            "- Claimed desktop profiles: {}\n",
            self.claimed_desktop_profiles
                .iter()
                .map(|profile| format!("`{}`", profile.as_str()))
                .collect::<Vec<_>>()
                .join(", ")
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
            "- Desktop rows checked: `{}`\n",
            self.desktop_rows_checked
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
            if self.report_clean { "clean" } else { "blocked" }
        ));
        out.push_str(&format!("- Generated at: `{}`\n\n", self.generated_at));

        out.push_str("## Per-row coverage\n\n");
        out.push_str(
            "| Scenario row | Qualified | Narrowed | Unqualified | Missing evidence |\n\
             | ------------ | --------: | -------: | ----------: | ---------------: |\n",
        );
        for coverage in &self.row_coverage {
            out.push_str(&format!(
                "| {} | {} | {} | {} | {} |\n",
                coverage.row.display_label(),
                coverage.qualified_rows,
                coverage.narrowed_rows(),
                coverage.unqualified_local_platform_path_rows,
                coverage.missing_evidence_rows,
            ));
        }
        out.push('\n');

        out.push_str("## Findings summary\n\n");
        out.push_str("| Class | Count |\n| ----- | ----: |\n");
        out.push_str(&format!(
            "| `unqualified_local_platform_path` | {} |\n",
            self.findings_summary.unqualified_local_platform_path
        ));
        out.push_str(&format!(
            "| `missing_evidence` | {} |\n",
            self.findings_summary.missing_evidence
        ));
        out.push_str(&format!(
            "| `missing_evidence_pack` | {} |\n",
            self.findings_summary.missing_evidence_pack
        ));
        out.push_str(&format!(
            "| `reopen_target_lost` | {} |\n",
            self.findings_summary.reopen_target_lost
        ));
        out.push_str(&format!(
            "| `layout_continuity_broken` | {} |\n",
            self.findings_summary.layout_continuity_broken
        ));
        out.push_str(&format!(
            "| `interruption_unsafe` | {} |\n",
            self.findings_summary.interruption_unsafe
        ));
        out.push_str(&format!(
            "| `placeholder_misleading` | {} |\n",
            self.findings_summary.placeholder_misleading
        ));
        out.push_str(&format!(
            "| `authority_context_lost` | {} |\n",
            self.findings_summary.authority_context_lost
        ));
        out.push_str(&format!(
            "| `background_not_throttled` | {} |\n",
            self.findings_summary.background_not_throttled
        ));
        out.push_str(&format!(
            "| `handoff_reason_dropped` | {} |\n",
            self.findings_summary.handoff_reason_dropped
        ));
        out.push_str(&format!(
            "| `boundary_cue_hidden` | {} |\n",
            self.findings_summary.boundary_cue_hidden
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
            "| `descriptor_missing_reopen_anchor` | {} |\n",
            self.findings_summary.descriptor_missing_reopen_anchor
        ));
        out.push_str(&format!(
            "| `missing_continuity_note` | {} |\n",
            self.findings_summary.missing_continuity_note
        ));
        out.push_str(&format!(
            "| `missing_claimed_profiles` | {} |\n",
            self.findings_summary.missing_claimed_profiles
        ));
        out.push_str(&format!(
            "| `surface_not_on_platform_conformance` | {} |\n\n",
            self.findings_summary.surface_not_on_platform_conformance
        ));

        out.push_str("## Reopen anchor index\n\n");
        out.push_str(
            "| Surface family | Surface | Reopen anchor |\n| -------------- | ------- | ------------- |\n",
        );
        for entry in &self.reopen_anchor_index {
            out.push_str(&format!(
                "| {} | `{}` | `{}` |\n",
                entry.surface_family.display_label(),
                entry.surface_id,
                entry.reopen_anchor_ref,
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
                "- Reopen anchor: `{}`\n",
                surface.descriptor.reopen_anchor_ref
            ));
            out.push_str(&format!(
                "- Claimed profiles: {}\n",
                surface
                    .descriptor
                    .claimed_desktop_profiles
                    .iter()
                    .map(|profile| format!("`{}`", profile.as_str()))
                    .collect::<Vec<_>>()
                    .join(", ")
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
                "| Scenario row | Status | Reopen | Layout | Interruption | Placeholder | Authority | Background | Handoff | Boundary | Freshness | Narrowing reason |\n\
                 | ------------ | ------ | ------ | ------ | ------------ | ----------- | --------- | ---------- | ------- | -------- | --------- | ---------------- |\n",
            );
            for binding in &surface.bindings {
                let reopen = binding
                    .projected_reopen_fidelity
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let layout = binding
                    .projected_layout_continuity
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let interruption = binding
                    .projected_interruption_safety
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let placeholder = binding
                    .projected_placeholder_honesty
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let authority = binding
                    .projected_authority_context
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let background = binding
                    .projected_background_adaptation
                    .map(|value| value.as_str())
                    .unwrap_or("-");
                let handoff = binding
                    .projected_handoff_reason
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
                    "| {} | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                    binding.row.display_label(),
                    binding.qualification_status.as_str(),
                    reopen,
                    layout,
                    interruption,
                    placeholder,
                    authority,
                    background,
                    handoff,
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
                        finding.row().map(M5DesktopRow::as_str).unwrap_or("surface"),
                    ));
                }
                out.push('\n');
            }
        }

        out.push_str("## Verification\n\n");
        out.push_str("```sh\n");
        out.push_str(
            "cargo run -q -p aureline-shell --bin aureline_shell_m5_desktop_conformance -- validate\n",
        );
        out.push_str("cargo test -p aureline-shell --test m5_desktop_conformance_fixtures\n");
        out.push_str("python3 tools/ci/m5/desktop_conformance_check.py\n");
        out.push_str("```\n");
        out
    }
}

/// Support-export wrapper for the M5 desktop-qualification audit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5DesktopSupportExport {
    /// Record discriminator.
    pub record_kind: String,
    /// Schema version exported with the record.
    pub schema_version: u32,
    /// Shared contract ref consumed by UI, CLI, docs, and support export.
    pub shared_contract_ref: String,
    /// Stable support-export id.
    pub support_export_id: String,
    /// Audit report quoted in full.
    pub report: M5DesktopQualificationReport,
    /// Stable case ids reviewers pivot on.
    pub case_ids: Vec<String>,
}

impl M5DesktopSupportExport {
    /// Builds the support-export wrapper for an audit report.
    pub fn from_report(
        support_export_id: impl Into<String>,
        report: M5DesktopQualificationReport,
    ) -> Self {
        let mut case_ids = vec![report.report_id.clone()];
        for surface in &report.rows {
            case_ids.push(surface.descriptor.surface_id.clone());
            case_ids.push(surface.descriptor.descriptor_revision_ref.clone());
        }
        Self {
            record_kind: M5_DESKTOP_SUPPORT_EXPORT_RECORD_KIND.to_owned(),
            schema_version: M5_DESKTOP_SCHEMA_VERSION,
            shared_contract_ref: M5_DESKTOP_SHARED_CONTRACT_REF.to_owned(),
            support_export_id: support_export_id.into(),
            report,
            case_ids,
        }
    }
}

/// Computes the per-surface blocking findings from a descriptor and its row
/// bindings.
fn compute_surface_findings(
    descriptor: &M5DesktopDescriptor,
    bindings: &[M5DesktopBinding],
    high_salience: bool,
) -> Vec<M5DesktopBlockingFinding> {
    let mut findings = Vec::new();

    // Descriptor-level (surface-scoped) findings.
    if descriptor.reopen_anchor_ref.trim().is_empty() {
        findings.push(M5DesktopBlockingFinding::DescriptorMissingReopenAnchor {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if descriptor.continuity_note.trim().is_empty() {
        findings.push(M5DesktopBlockingFinding::MissingContinuityNote {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if descriptor.claimed_desktop_profiles.is_empty() {
        findings.push(M5DesktopBlockingFinding::MissingClaimedProfiles {
            surface_id: descriptor.surface_id.clone(),
        });
    }
    if !descriptor.registered_on_platform_conformance {
        findings.push(M5DesktopBlockingFinding::SurfaceNotOnPlatformConformance {
            surface_id: descriptor.surface_id.clone(),
        });
    }

    for binding in bindings {
        let row = binding.row;
        let surface_id = descriptor.surface_id.clone();

        // A binding's dimension must match its row's canonical dimension.
        if binding.dimension != row.canonical_dimension() {
            findings.push(M5DesktopBlockingFinding::DimensionDrift {
                surface_id: surface_id.clone(),
                row,
                projected_dimension: binding.dimension,
            });
        }

        match binding.qualification_status {
            M5DesktopQualificationStatus::UnqualifiedLocalPlatformPath => {
                findings.push(M5DesktopBlockingFinding::UnqualifiedLocalPlatformPath {
                    surface_id: surface_id.clone(),
                    row,
                });
            }
            M5DesktopQualificationStatus::MissingEvidence => {
                findings.push(M5DesktopBlockingFinding::MissingEvidence {
                    surface_id: surface_id.clone(),
                    row,
                });
            }
            M5DesktopQualificationStatus::Qualified => {
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
                    findings.push(M5DesktopBlockingFinding::MissingNarrowingReason {
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

/// Computes the blocking findings for one qualified desktop binding.
fn compute_qualified_findings(
    binding: &M5DesktopBinding,
    high_salience: bool,
    surface_id: &str,
    findings: &mut Vec<M5DesktopBlockingFinding>,
) {
    let row = binding.row;

    // Required captured-evidence projections (every qualified row).
    if binding.projected_evidence_pack_ref.is_none() {
        findings.push(M5DesktopBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_evidence_pack_ref".to_owned(),
        });
    }
    if binding.projected_reopen_fidelity.is_none() {
        findings.push(M5DesktopBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_reopen_fidelity".to_owned(),
        });
    }
    if binding.projected_layout_continuity.is_none() {
        findings.push(M5DesktopBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_layout_continuity".to_owned(),
        });
    }
    if binding.projected_interruption_safety.is_none() {
        findings.push(M5DesktopBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_interruption_safety".to_owned(),
        });
    }
    if binding.projected_placeholder_honesty.is_none() {
        findings.push(M5DesktopBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_placeholder_honesty".to_owned(),
        });
    }
    if binding.evidence_freshness.is_none() {
        findings.push(M5DesktopBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "evidence_freshness".to_owned(),
        });
    }
    if row.requires_background_adaptation() && binding.projected_background_adaptation.is_none() {
        findings.push(M5DesktopBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_background_adaptation".to_owned(),
        });
    }
    if row.requires_handoff_reason() && binding.projected_handoff_reason.is_none() {
        findings.push(M5DesktopBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_handoff_reason".to_owned(),
        });
    }
    if row.requires_authority_context() && binding.projected_authority_context.is_none() {
        findings.push(M5DesktopBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_authority_context".to_owned(),
        });
    }
    if high_salience && binding.projected_boundary_cue.is_none() {
        findings.push(M5DesktopBlockingFinding::MissingProjection {
            surface_id: surface_id.to_owned(),
            row,
            field: "projected_boundary_cue".to_owned(),
        });
    }

    // Missing evidence pack is also a dedicated class.
    if binding.projected_evidence_pack_ref.is_none() {
        findings.push(M5DesktopBlockingFinding::MissingEvidencePack {
            surface_id: surface_id.to_owned(),
            row,
        });
    }

    // Red captured results.
    if binding.projected_reopen_fidelity == Some(M5ReopenFidelity::Lost) {
        findings.push(M5DesktopBlockingFinding::ReopenTargetLost {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_layout_continuity == Some(M5LayoutContinuity::Broken) {
        findings.push(M5DesktopBlockingFinding::LayoutContinuityBroken {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_interruption_safety == Some(M5InterruptionSafety::Corrupted) {
        findings.push(M5DesktopBlockingFinding::InterruptionUnsafe {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_placeholder_honesty == Some(M5PlaceholderHonesty::Misleading) {
        findings.push(M5DesktopBlockingFinding::PlaceholderMisleading {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_authority_context == Some(M5AuthorityContext::Lost) {
        findings.push(M5DesktopBlockingFinding::AuthorityContextLost {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if row.requires_background_adaptation()
        && binding.projected_background_adaptation == Some(M5BackgroundAdaptation::NotThrottled)
    {
        findings.push(M5DesktopBlockingFinding::BackgroundNotThrottled {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if row.requires_handoff_reason()
        && binding.projected_handoff_reason == Some(M5HandoffReason::Dropped)
    {
        findings.push(M5DesktopBlockingFinding::HandoffReasonDropped {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.projected_boundary_cue == Some(M5BoundaryCue::Hidden) {
        findings.push(M5DesktopBlockingFinding::BoundaryCueHidden {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
    if binding.marketed_on_row && binding.evidence_freshness == Some(M5EvidenceFreshness::Stale) {
        findings.push(M5DesktopBlockingFinding::StaleEvidenceOnMarketedRow {
            surface_id: surface_id.to_owned(),
            row,
        });
    }
}

/// Computes the per-row and per-class summaries from finished surfaces.
fn summarize_report(
    surfaces: &[M5DesktopQualificationRow],
) -> (Vec<M5DesktopCoverageSummary>, M5DesktopFindingSummary) {
    let mut summary = M5DesktopFindingSummary::empty();
    let mut coverage: Vec<M5DesktopCoverageSummary> = M5DesktopRow::required_rows()
        .into_iter()
        .map(|row| M5DesktopCoverageSummary {
            row,
            qualified_rows: 0,
            explicitly_narrowed_rows: 0,
            not_applicable_rows: 0,
            platform_omitted_rows: 0,
            declared_capture_gap_rows: 0,
            unqualified_local_platform_path_rows: 0,
            missing_evidence_rows: 0,
        })
        .collect();

    for surface in surfaces {
        for binding in &surface.bindings {
            if let Some(coverage_row) = coverage.iter_mut().find(|entry| entry.row == binding.row) {
                match binding.qualification_status {
                    M5DesktopQualificationStatus::Qualified => coverage_row.qualified_rows += 1,
                    M5DesktopQualificationStatus::ExplicitlyNarrowed => {
                        coverage_row.explicitly_narrowed_rows += 1
                    }
                    M5DesktopQualificationStatus::NotApplicable => {
                        coverage_row.not_applicable_rows += 1
                    }
                    M5DesktopQualificationStatus::PlatformOmitted => {
                        coverage_row.platform_omitted_rows += 1
                    }
                    M5DesktopQualificationStatus::DeclaredCaptureGap => {
                        coverage_row.declared_capture_gap_rows += 1
                    }
                    M5DesktopQualificationStatus::UnqualifiedLocalPlatformPath => {
                        coverage_row.unqualified_local_platform_path_rows += 1
                    }
                    M5DesktopQualificationStatus::MissingEvidence => {
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
/// desktop evidence is stale or red.
fn compute_narrowable_rows(surfaces: &[M5DesktopQualificationRow]) -> Vec<M5DesktopNarrowableRow> {
    let mut narrowable = Vec::new();
    for surface in surfaces {
        if !surface.marketed {
            continue;
        }
        for finding in &surface.blocking_findings {
            if let Some(row) = finding.row() {
                narrowable.push(M5DesktopNarrowableRow {
                    surface_id: surface.descriptor.surface_id.clone(),
                    row,
                    reason: format!("blocking_finding:{}", finding.class_token()),
                });
            }
        }
    }
    narrowable
}

/// Builds an [`M5DesktopQualificationRow`] from a descriptor and its row
/// bindings, computing the per-surface blocking findings.
pub fn build_m5_desktop_row(
    descriptor: M5DesktopDescriptor,
    bindings: Vec<M5DesktopBinding>,
) -> M5DesktopQualificationRow {
    let high_salience = descriptor.is_high_salience();
    let marketed = descriptor.marketed_on_desktop_rows;
    let blocking_findings = compute_surface_findings(&descriptor, &bindings, high_salience);

    M5DesktopQualificationRow {
        record_kind: M5_DESKTOP_ROW_RECORD_KIND.to_owned(),
        schema_version: M5_DESKTOP_SCHEMA_VERSION,
        shared_contract_ref: M5_DESKTOP_SHARED_CONTRACT_REF.to_owned(),
        descriptor,
        bindings,
        blocking_findings,
        high_salience,
        marketed,
    }
}

/// Builds a full [`M5DesktopQualificationReport`] from per-surface rows.
pub fn build_m5_desktop_qualification_audit(
    surfaces: Vec<M5DesktopQualificationRow>,
) -> M5DesktopQualificationReport {
    let mut surfaces = surfaces;
    surfaces.sort_by(|left, right| left.descriptor.surface_id.cmp(&right.descriptor.surface_id));

    let registered_surface_count = surfaces.len();
    let high_salience_surface_count = surfaces.iter().filter(|row| row.high_salience).count();
    let marketed_surface_count = surfaces.iter().filter(|row| row.marketed).count();
    let desktop_rows_checked = surfaces.iter().map(|row| row.bindings.len()).sum::<usize>();

    let (row_coverage, findings_summary) = summarize_report(&surfaces);
    let narrowable_marketed_rows = compute_narrowable_rows(&surfaces);
    let report_clean = findings_summary.total_blocking_findings == 0;

    let mut profile_set: Vec<M5DesktopProfile> = Vec::new();
    for surface in &surfaces {
        for profile in &surface.descriptor.claimed_desktop_profiles {
            if !profile_set.contains(profile) {
                profile_set.push(*profile);
            }
        }
    }
    profile_set.sort();

    let mut reopen_anchor_index: Vec<M5DesktopReopenAnchorEntry> = surfaces
        .iter()
        .map(|surface| M5DesktopReopenAnchorEntry {
            surface_family: surface.descriptor.surface_family,
            surface_id: surface.descriptor.surface_id.clone(),
            reopen_anchor_ref: surface.descriptor.reopen_anchor_ref.clone(),
        })
        .collect();
    reopen_anchor_index.sort_by(|left, right| left.surface_id.cmp(&right.surface_id));

    M5DesktopQualificationReport {
        record_kind: M5_DESKTOP_REPORT_RECORD_KIND.to_owned(),
        schema_version: M5_DESKTOP_SCHEMA_VERSION,
        shared_contract_ref: M5_DESKTOP_SHARED_CONTRACT_REF.to_owned(),
        report_id: M5_DESKTOP_REPORT_ID.to_owned(),
        source_schema_ref: M5_DESKTOP_SOURCE_SCHEMA_REF.to_owned(),
        required_rows: M5DesktopRow::required_rows().to_vec(),
        claimed_desktop_profiles: profile_set,
        rows: surfaces,
        row_coverage,
        findings_summary,
        reopen_anchor_index,
        registered_surface_count,
        high_salience_surface_count,
        marketed_surface_count,
        desktop_rows_checked,
        narrowable_marketed_rows,
        report_clean,
        published_report_ref: M5_DESKTOP_PUBLISHED_REPORT_REF.to_owned(),
        published_doc_ref: M5_DESKTOP_PUBLISHED_DOC_REF.to_owned(),
        docs_help_refs: vec![
            M5_DESKTOP_PUBLISHED_DOC_REF.to_owned(),
            "docs/m5/durable-progress-and-reopen.md".to_owned(),
        ],
        support_export_refs: vec!["support:m5-desktop-conformance".to_owned()],
        generated_at: GENERATED_AT.to_owned(),
    }
}

/// Validation error produced by [`validate_m5_desktop_qualification`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "error", rename_all = "snake_case")]
pub enum M5DesktopValidationError {
    /// The audit has no registered surfaces.
    NoRegisteredSurfaces,
    /// A required scenario row has no qualified surface.
    RequiredRowNotQualified { row: String },
    /// A surface is missing a required scenario row from its binding set.
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

/// Validates an audit report against the M5 desktop acceptance invariants.
///
/// # Errors
/// Returns the full list of detected invariant violations.
pub fn validate_m5_desktop_qualification(
    report: &M5DesktopQualificationReport,
) -> Result<(), Vec<M5DesktopValidationError>> {
    let mut errors = Vec::new();

    if report.rows.is_empty() {
        errors.push(M5DesktopValidationError::NoRegisteredSurfaces);
    }

    for required in M5DesktopRow::required_rows() {
        let any_qualified = report.rows.iter().any(|surface| {
            surface.bindings.iter().any(|binding| {
                binding.row == required
                    && binding.qualification_status == M5DesktopQualificationStatus::Qualified
            })
        });
        if !any_qualified {
            errors.push(M5DesktopValidationError::RequiredRowNotQualified {
                row: required.as_str().to_owned(),
            });
        }
    }

    for surface in &report.rows {
        for required in M5DesktopRow::required_rows() {
            if !surface
                .bindings
                .iter()
                .any(|binding| binding.row == required)
            {
                errors.push(M5DesktopValidationError::MissingRequiredRow {
                    surface_id: surface.descriptor.surface_id.clone(),
                    row: required.as_str().to_owned(),
                });
            }
        }
        if surface.descriptor.descriptor_revision_ref.trim().is_empty() {
            errors.push(M5DesktopValidationError::MissingDescriptorRevisionRef {
                surface_id: surface.descriptor.surface_id.clone(),
            });
        }
        for finding in &surface.blocking_findings {
            errors.push(M5DesktopValidationError::BlockingFindingPresent {
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
        errors.push(M5DesktopValidationError::PublishedReportRefMissing);
    }
    if report.published_doc_ref.trim().is_empty() {
        errors.push(M5DesktopValidationError::PublishedDocRefMissing);
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

/// Seed row used by [`seeded_m5_desktop_qualification_audit`].
struct SurfaceSeed {
    surface_id: &'static str,
    surface_family: M5DesktopSurfaceFamily,
    descriptor_revision_ref: &'static str,
    primary_label_ref: &'static str,
    reopen_anchor_ref: &'static str,
    continuity_note: &'static str,
    semantic_salience: M5SemanticSalience,
    lifecycle_label: M5SurfaceLifecycle,
    reopen: M5ReopenFidelity,
    placeholder: M5PlaceholderHonesty,
    authority: M5AuthorityContext,
    boundary_cue: M5BoundaryCue,
    bindings: &'static [BindingSeed],
}

struct BindingSeed {
    row: M5DesktopRow,
    qualification_status: M5DesktopQualificationStatus,
    narrowing_reason: Option<&'static str>,
    note: Option<&'static str>,
}

/// Helper: a qualified row with captured evidence.
const fn qualified(row: M5DesktopRow) -> BindingSeed {
    BindingSeed {
        row,
        qualification_status: M5DesktopQualificationStatus::Qualified,
        narrowing_reason: None,
        note: None,
    }
}

/// Helper: an honestly-declared capture gap with a documented reason.
const fn declared_capture_gap(row: M5DesktopRow, reason: &'static str) -> BindingSeed {
    BindingSeed {
        row,
        qualification_status: M5DesktopQualificationStatus::DeclaredCaptureGap,
        narrowing_reason: Some(reason),
        note: None,
    }
}

const ALL_QUALIFIED: &[BindingSeed] = &[
    qualified(M5DesktopRow::MultiWindow),
    qualified(M5DesktopRow::MultiMonitor),
    qualified(M5DesktopRow::MixedDpi),
    qualified(M5DesktopRow::SuspendResume),
    qualified(M5DesktopRow::BatterySaver),
    qualified(M5DesktopRow::ThermalPressure),
    qualified(M5DesktopRow::DeepLink),
    qualified(M5DesktopRow::FileAssociation),
    qualified(M5DesktopRow::SystemOpenReturn),
];

const SURFACE_SEEDS: &[SurfaceSeed] = &[
    // Notebook cell chrome. Lifecycle-bearing; reopen-able; carries a cue.
    SurfaceSeed {
        surface_id: "surface:notebook.cell_chrome",
        surface_family: M5DesktopSurfaceFamily::NotebookCellChrome,
        descriptor_revision_ref: "surface-rev:notebook.cell_chrome:2026.06.01-01",
        primary_label_ref: "label:notebook.cell_chrome:primary",
        reopen_anchor_ref: "reopen:anchor:notebook:cell_chrome",
        continuity_note: "A kernel that disappears on resume reopens the exact cell with an honest stale placeholder, never a silent blank.",
        semantic_salience: M5SemanticSalience::LifecycleBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenFidelity::ExactTargetPreserved,
        placeholder: M5PlaceholderHonesty::Honest,
        authority: M5AuthorityContext::Preserved,
        boundary_cue: M5BoundaryCue::Present,
        bindings: ALL_QUALIFIED,
    },
    // Result-grid rows. Lifecycle-bearing; dense data rows.
    SurfaceSeed {
        surface_id: "surface:data_api.result_grid_row",
        surface_family: M5DesktopSurfaceFamily::ResultGridRow,
        descriptor_revision_ref: "surface-rev:data_api.result_grid_row:2026.06.01-01",
        primary_label_ref: "label:data_api.result_grid_row:primary",
        reopen_anchor_ref: "reopen:anchor:data_api:result_grid_row",
        continuity_note: "Cached and partial rows keep their honesty across monitors and DPI classes and reopen the same query target after suspend.",
        semantic_salience: M5SemanticSalience::LifecycleBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenFidelity::ExactTargetPreserved,
        placeholder: M5PlaceholderHonesty::Honest,
        authority: M5AuthorityContext::Preserved,
        boundary_cue: M5BoundaryCue::Present,
        bindings: ALL_QUALIFIED,
    },
    // Profiler panel. Informational; diagnostic timeline.
    SurfaceSeed {
        surface_id: "surface:profiler.capture_panel",
        surface_family: M5DesktopSurfaceFamily::ProfilerPanel,
        descriptor_revision_ref: "surface-rev:profiler.capture_panel:2026.06.01-01",
        primary_label_ref: "label:profiler.capture_panel:primary",
        reopen_anchor_ref: "reopen:anchor:profiler:capture_panel",
        continuity_note: "A capture reopens by id after suspend; thermal pressure throttles the sampler before it can drop frames into the wrong target.",
        semantic_salience: M5SemanticSalience::Informational,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenFidelity::ExactTargetPreserved,
        placeholder: M5PlaceholderHonesty::Honest,
        authority: M5AuthorityContext::NotApplicable,
        boundary_cue: M5BoundaryCue::NotApplicable,
        bindings: ALL_QUALIFIED,
    },
    // Trace panel. Informational; replay timeline.
    SurfaceSeed {
        surface_id: "surface:trace.replay_panel",
        surface_family: M5DesktopSurfaceFamily::TracePanel,
        descriptor_revision_ref: "surface-rev:trace.replay_panel:2026.06.01-01",
        primary_label_ref: "label:trace.replay_panel:primary",
        reopen_anchor_ref: "reopen:anchor:trace:replay_panel",
        continuity_note: "A replay reopens the exact captured trace after an interruption; battery saver slows replay decode before it corrupts the scrub head.",
        semantic_salience: M5SemanticSalience::Informational,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenFidelity::ExactTargetPreserved,
        placeholder: M5PlaceholderHonesty::Honest,
        authority: M5AuthorityContext::NotApplicable,
        boundary_cue: M5BoundaryCue::NotApplicable,
        bindings: ALL_QUALIFIED,
    },
    // Pipeline card. Severity-bearing; status card.
    SurfaceSeed {
        surface_id: "surface:review.pipeline_card",
        surface_family: M5DesktopSurfaceFamily::PipelineCard,
        descriptor_revision_ref: "surface-rev:review.pipeline_card:2026.06.01-01",
        primary_label_ref: "label:review.pipeline_card:primary",
        reopen_anchor_ref: "reopen:anchor:review:pipeline_card",
        continuity_note: "A pipeline action reopens the exact run from a deep link with its authority context and keeps the blocked cue across every display.",
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenFidelity::ExactTargetPreserved,
        placeholder: M5PlaceholderHonesty::Honest,
        authority: M5AuthorityContext::Preserved,
        boundary_cue: M5BoundaryCue::Present,
        bindings: ALL_QUALIFIED,
    },
    // Preview-route badge. Lifecycle-bearing; live preview badge.
    SurfaceSeed {
        surface_id: "surface:preview.route_badge",
        surface_family: M5DesktopSurfaceFamily::PreviewRouteBadge,
        descriptor_revision_ref: "surface-rev:preview.route_badge:2026.06.01-01",
        primary_label_ref: "label:preview.route_badge:primary",
        reopen_anchor_ref: "reopen:anchor:preview:route_badge",
        continuity_note: "An expired preview route reopens to an honest expired placeholder rather than pretending the route is still live.",
        semantic_salience: M5SemanticSalience::LifecycleBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenFidelity::ExactTargetPreserved,
        placeholder: M5PlaceholderHonesty::Honest,
        authority: M5AuthorityContext::Preserved,
        boundary_cue: M5BoundaryCue::Present,
        bindings: ALL_QUALIFIED,
    },
    // Docs / browser pane. Informational; embedded provider declares a gap.
    SurfaceSeed {
        surface_id: "surface:docs_browser.pane",
        surface_family: M5DesktopSurfaceFamily::DocsBrowserPane,
        descriptor_revision_ref: "surface-rev:docs_browser.pane:2026.06.01-01",
        primary_label_ref: "label:docs_browser.pane:primary",
        reopen_anchor_ref: "reopen:anchor:docs_browser:pane",
        continuity_note: "Shell chrome around embedded content qualifies on every row; the embedded provider's own OS file-handler registration is declared, not claimed.",
        semantic_salience: M5SemanticSalience::Informational,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenFidelity::ExactTargetPreserved,
        placeholder: M5PlaceholderHonesty::Honest,
        authority: M5AuthorityContext::NotApplicable,
        boundary_cue: M5BoundaryCue::NotApplicable,
        bindings: &[
            qualified(M5DesktopRow::MultiWindow),
            qualified(M5DesktopRow::MultiMonitor),
            qualified(M5DesktopRow::MixedDpi),
            qualified(M5DesktopRow::SuspendResume),
            qualified(M5DesktopRow::BatterySaver),
            qualified(M5DesktopRow::ThermalPressure),
            qualified(M5DesktopRow::DeepLink),
            declared_capture_gap(
                M5DesktopRow::FileAssociation,
                "embedded_provider_owns_os_file_handler_registration_so_the_association_capture_is_provider_attributed",
            ),
            qualified(M5DesktopRow::SystemOpenReturn),
        ],
    },
    // Companion surface. Trust-bearing; provider-backed; declares a gap.
    SurfaceSeed {
        surface_id: "surface:companion.surface",
        surface_family: M5DesktopSurfaceFamily::CompanionSurface,
        descriptor_revision_ref: "surface-rev:companion.surface:2026.06.01-01",
        primary_label_ref: "label:companion.surface:primary",
        reopen_anchor_ref: "reopen:anchor:companion:surface",
        continuity_note: "Presence and handoff cues keep their authority context across a system-open return; the companion provider's own thermal budget is declared honestly.",
        semantic_salience: M5SemanticSalience::TrustBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenFidelity::ExactTargetPreserved,
        placeholder: M5PlaceholderHonesty::Honest,
        authority: M5AuthorityContext::Preserved,
        boundary_cue: M5BoundaryCue::Present,
        bindings: &[
            qualified(M5DesktopRow::MultiWindow),
            qualified(M5DesktopRow::MultiMonitor),
            qualified(M5DesktopRow::MixedDpi),
            qualified(M5DesktopRow::SuspendResume),
            qualified(M5DesktopRow::BatterySaver),
            declared_capture_gap(
                M5DesktopRow::ThermalPressure,
                "companion_provider_owns_its_own_thermal_budget_so_the_throttle_capture_is_provider_attributed",
            ),
            qualified(M5DesktopRow::DeepLink),
            qualified(M5DesktopRow::FileAssociation),
            qualified(M5DesktopRow::SystemOpenReturn),
        ],
    },
    // Sync status surface. Severity-bearing; conflict status.
    SurfaceSeed {
        surface_id: "surface:sync.status_surface",
        surface_family: M5DesktopSurfaceFamily::SyncStatusSurface,
        descriptor_revision_ref: "surface-rev:sync.status_surface:2026.06.01-01",
        primary_label_ref: "label:sync.status_surface:primary",
        reopen_anchor_ref: "reopen:anchor:sync:status_surface",
        continuity_note: "Sync-pending and conflict states survive suspend/resume and keep their authority context when reopened from a deep link.",
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenFidelity::ExactTargetPreserved,
        placeholder: M5PlaceholderHonesty::Honest,
        authority: M5AuthorityContext::Preserved,
        boundary_cue: M5BoundaryCue::Present,
        bindings: ALL_QUALIFIED,
    },
    // Offboarding surface. Severity-bearing; destructive lifecycle.
    SurfaceSeed {
        surface_id: "surface:offboarding.surface",
        surface_family: M5DesktopSurfaceFamily::OffboardingSurface,
        descriptor_revision_ref: "surface-rev:offboarding.surface:2026.06.01-01",
        primary_label_ref: "label:offboarding.surface:primary",
        reopen_anchor_ref: "reopen:anchor:offboarding:surface",
        continuity_note: "A destructive export-and-wipe flow reopens its exact step after an interruption and keeps its authority context across a system-open return.",
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenFidelity::ExactTargetPreserved,
        placeholder: M5PlaceholderHonesty::Honest,
        authority: M5AuthorityContext::Preserved,
        boundary_cue: M5BoundaryCue::Present,
        bindings: ALL_QUALIFIED,
    },
    // Incident packet. Severity-bearing; support handoff.
    SurfaceSeed {
        surface_id: "surface:incident.packet",
        surface_family: M5DesktopSurfaceFamily::IncidentPacket,
        descriptor_revision_ref: "surface-rev:incident.packet:2026.06.01-01",
        primary_label_ref: "label:incident.packet:primary",
        reopen_anchor_ref: "reopen:anchor:incident:packet",
        continuity_note: "An incident packet reopens the exact captured packet from a deep link with its authority context and survives suspend/resume intact.",
        semantic_salience: M5SemanticSalience::SeverityBearing,
        lifecycle_label: M5SurfaceLifecycle::Beta,
        reopen: M5ReopenFidelity::ExactTargetPreserved,
        placeholder: M5PlaceholderHonesty::Honest,
        authority: M5AuthorityContext::Preserved,
        boundary_cue: M5BoundaryCue::Present,
        bindings: ALL_QUALIFIED,
    },
];

fn build_binding_from_seed(seed: &SurfaceSeed, binding_seed: &BindingSeed) -> M5DesktopBinding {
    let row = binding_seed.row;
    let qualified = binding_seed.qualification_status.projects_evidence();
    let high_salience = seed.semantic_salience.is_high_salience();
    let marketed_on_row = !matches!(
        binding_seed.qualification_status,
        M5DesktopQualificationStatus::NotApplicable | M5DesktopQualificationStatus::PlatformOmitted
    );

    M5DesktopBinding {
        row,
        dimension: row.canonical_dimension(),
        qualification_status: binding_seed.qualification_status,
        marketed_on_row,
        projected_evidence_pack_ref: qualified
            .then(|| format!("drill:{}:{}", seed.surface_id, row.as_str())),
        projected_reopen_fidelity: qualified.then_some(seed.reopen),
        projected_layout_continuity: qualified.then_some(M5LayoutContinuity::Preserved),
        projected_interruption_safety: qualified.then_some(M5InterruptionSafety::Safe),
        projected_placeholder_honesty: qualified.then_some(seed.placeholder),
        projected_authority_context: (qualified && row.requires_authority_context())
            .then_some(seed.authority),
        projected_background_adaptation: (qualified && row.requires_background_adaptation())
            .then_some(M5BackgroundAdaptation::ThrottledBeforeCorruption),
        projected_handoff_reason: (qualified && row.requires_handoff_reason())
            .then_some(M5HandoffReason::Preserved),
        projected_boundary_cue: (qualified && high_salience).then_some(seed.boundary_cue),
        evidence_freshness: qualified.then_some(M5EvidenceFreshness::Fresh),
        evidence_captured_at: qualified.then(|| GENERATED_AT.to_owned()),
        narrowing_reason: binding_seed.narrowing_reason.map(str::to_owned),
        note: binding_seed.note.map(str::to_owned),
    }
}

fn build_surface_from_seed(seed: &SurfaceSeed) -> M5DesktopQualificationRow {
    let descriptor = M5DesktopDescriptor {
        surface_id: seed.surface_id.to_owned(),
        surface_family: seed.surface_family,
        descriptor_revision_ref: seed.descriptor_revision_ref.to_owned(),
        primary_label_ref: seed.primary_label_ref.to_owned(),
        reopen_anchor_ref: seed.reopen_anchor_ref.to_owned(),
        continuity_note: seed.continuity_note.to_owned(),
        semantic_salience: seed.semantic_salience,
        lifecycle_label: seed.lifecycle_label,
        claimed_desktop_profiles: M5DesktopProfile::all().to_vec(),
        marketed_on_desktop_rows: true,
        registered_on_platform_conformance: true,
    };
    let bindings: Vec<M5DesktopBinding> = seed
        .bindings
        .iter()
        .map(|binding_seed| build_binding_from_seed(seed, binding_seed))
        .collect();
    build_m5_desktop_row(descriptor, bindings)
}

/// Seeded audit builder used by the headless inspector and the integration
/// test. The seed mirrors the JSON fixtures checked in under
/// `fixtures/platform/m5_depth_surfaces/`.
pub fn seeded_m5_desktop_qualification_audit() -> M5DesktopQualificationReport {
    let surfaces = SURFACE_SEEDS.iter().map(build_surface_from_seed).collect();
    build_m5_desktop_qualification_audit(surfaces)
}
