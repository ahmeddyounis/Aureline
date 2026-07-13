//! Keyboard / screen-reader / high-zoom / high-contrast / snapped-width / CLI / export parity, and honest
//! automatic claim narrowing for the M5 shell-metric / minimum-size / density-mode / responsive-geometry /
//! collapse-priority shell-geometry families.
//!
//! This module is the M05-1162 accessibility-power-and-auto-narrowing capstone over the frozen M5
//! shell-metric / density matrix ([`crate::m5_shell_metric_density_matrix`]). Where the freeze matrix
//! defines the five governed shell-geometry families, and the 1157-1160 implementation lanes resolve their
//! per-surface shell-metric, minimum-size, density, responsive, and collapse truth, this lane certifies —
//! per geometry family — that shell-metric / minimum-size / density / responsive / collapse claims stay
//! **keyboard-reachable, screen-reader-announced, high-zoom-legible, high-contrast-safe, snapped-width-safe,
//! CLI/export-safe, and self-narrowing** rather than presenting a metric that disappears under zoom, a
//! density change that rearranges information architecture, a responsive collapse that drops recovery state,
//! a hit target that shrinks below the supported minimum, or a private width that fractures the shell as
//! still a stable, trusted shell-geometry surface:
//!
//! - **Keyboard / screen-reader / high-zoom / high-contrast / snapped-width / CLI reach.** Every family
//!   exposes a keyboard-reachable, screen-reader-announced, high-zoom-reflowing, high-contrast-legible,
//!   snapped-width-safe, and CLI/headless-reachable path into the same geometry identity, semantic role,
//!   registry reference, size metric, density mode, and responsive class the rendered surface shows — never a
//!   pointer-only affordance, an off-screen zone, an unlabeled control, or a metric that only lives in a
//!   screenshot and strands assistive-tech or headless-CLI users. Structure-heavy families (the shell-metric
//!   registry's zone table, the minimum-size registry's hit-target table, the collapse-priority ordered
//!   stack) additionally bind their structured layout to a flat list / textual / CLI path.
//! - **Export parity.** The support / release / CLI export reconstructs each family's meaning from typed
//!   tokens and opaque refs **without a raw payload**, preserving the same geometry identity, semantic role,
//!   registry reference, size metric, density mode, and responsive class shown in-product so support, help,
//!   and release proof can reconstruct which shell-geometry truth class was active without leaking a raw
//!   pixel value, private width, layout blob, or renderer-only screenshot.
//! - **Honest auto-narrowing.** When a shell-metric registry's evidence is stale, a density mode's
//!   presentation-only safety cannot be confirmed, a responsive window class's recovery-state preservation is
//!   unconfirmed, or a collapse priority's boundary can only be partially disclosed, the family's claim
//!   auto-narrows from `trusted_geometry_surface` / `reviewable_geometry_surface` to a
//!   shell-metric-unverified / density-mode-unverified / responsive-geometry-unverified /
//!   collapse-priority-disclosed projection, discloses the narrowing with a precise trigger and binding
//!   dimension, and preserves the canonical geometry identity / last-known registry reference. The underlying
//!   shell-metric / minimum-size / density / responsive / collapse truth is never dropped opaquely. A family
//!   with every dimension intact must NOT carry a spurious narrowing, and a workspace-starving / recovery-state
//!   dropping / hit-target-shrinking / overlay-only-hiding state can never keep a trusted, stable geometry
//!   claim — geometry meaning is never conveyed by a private width, an off-screen zone, or an unlabeled
//!   control alone.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the shell UI, the editor UI, the
//!   review UI, the notebook UI, the data UI, the settings UI, the CLI export, the support export, and the
//!   product UI so product, help, and release publication stay aligned on downgrade behavior rather than
//!   drifting in copy — a trusted-looking geometry surface can never outrun the shell-metric / density /
//!   responsive / collapse evidence it is being viewed away from.
//!
//! Each [`ShellGeometryAccessibilityRow`] keys on one
//! [`crate::m5_shell_metric_density_matrix::M5ShellGeometryFamily`] and reuses that frozen family vocabulary
//! plus the frozen [`M5ShellGeometryRequiredLabel`], [`M5ShellGeometryDowngradeTrigger`], and shared
//! [`M5ShellGeometryConsumerSurface`] consumer surfaces rather than minting parallel synonyms, so the
//! certified labels stay byte-identical to the matrix and the sibling shell-geometry packets.
//!
//! The packet is metadata-only: raw pixel values, private widths, layout blobs, credentials, secrets, and
//! endpoint refs never cross this boundary; the packet carries only typed class tokens, opaque geometry
//! refs, booleans, and controlled labels so support, release, and diagnostics exports can reconstruct
//! exactly which shell-geometry truth class was active without leaking sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen shell-geometry vocabulary — the capstone certifies the freeze matrix's families, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_shell_metric_density_matrix::{
    M5ShellGeometryConsumerSurface, M5ShellGeometryDowngradeTrigger, M5ShellGeometryFamily,
    M5ShellGeometryRequiredLabel, M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF,
};

/// Schema version stamped on the M05-1162 shell-metric-density accessibility parity packet.
pub const SHELL_METRIC_DENSITY_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ShellGeometryAccessibilityPacket`].
pub const SHELL_METRIC_DENSITY_A11Y_RECORD_KIND: &str =
    "m5_shell_metric_density_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`ShellGeometryAccessibilityRow`].
pub const SHELL_METRIC_DENSITY_A11Y_ROW_RECORD_KIND: &str =
    "m5_shell_metric_density_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const SHELL_METRIC_DENSITY_A11Y_SCHEMA_REF: &str =
    "schemas/shell/m5-shell-metric-density-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const SHELL_METRIC_DENSITY_A11Y_DOC_REF: &str =
    "docs/design-system/m5_shell_metric_density_accessibility_parity.md";

/// Repo-relative path of the frozen shell-metric-density matrix this lane certifies.
pub const SHELL_METRIC_DENSITY_A11Y_MATRIX_REF: &str = M5_SHELL_METRIC_DENSITY_MATRIX_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const SHELL_METRIC_DENSITY_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-shell-metric-density-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const SHELL_METRIC_DENSITY_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-shell-metric-density-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const SHELL_METRIC_DENSITY_A11Y_CSV_REF: &str =
    "artifacts/release/m5-shell-metric-density-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const SHELL_METRIC_DENSITY_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-shell-metric-density-accessibility-parity.md";

/// The reusable shell-geometry families that render a dense, structured surface (the shell-metric registry's
/// zone table, the minimum-size registry's hit-target table, the collapse-priority ordered stack) and
/// therefore MUST bind their structured layout to an equivalent flat list / textual / CLI path so the
/// structure is navigable non-visually.
const fn family_is_structure_heavy(family: M5ShellGeometryFamily) -> bool {
    matches!(
        family,
        M5ShellGeometryFamily::ShellMetric
            | M5ShellGeometryFamily::MinimumSize
            | M5ShellGeometryFamily::CollapsePriority
    )
}

/// The shell-geometry-truth dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(family: M5ShellGeometryFamily) -> M5ShellGeometryClaimDimension {
    match family {
        M5ShellGeometryFamily::ShellMetric => M5ShellGeometryClaimDimension::ShellMetricClarity,
        M5ShellGeometryFamily::MinimumSize => M5ShellGeometryClaimDimension::MinimumSizeClarity,
        M5ShellGeometryFamily::DensityMode => M5ShellGeometryClaimDimension::DensityModeClarity,
        M5ShellGeometryFamily::ResponsiveGeometry => {
            M5ShellGeometryClaimDimension::ResponsiveGeometryClarity
        }
        M5ShellGeometryFamily::CollapsePriority => {
            M5ShellGeometryClaimDimension::CollapsePriorityClarity
        }
    }
}

/// A rendered fallback modality for a shell-geometry family.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryFallbackModality {
    /// A rich, structured (zone table / hit-target table / collapse-order stack) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5ShellGeometryFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured surface
    /// (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Textual | Self::Cli)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Structured => "structured",
            Self::List => "list",
            Self::Textual => "textual",
            Self::Cli => "cli",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same geometry may
/// render at desktop-full capability or narrow to a companion, read-only browser, headless CLI, docs export,
/// or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryRenderingSurface {
    /// The full-capability desktop surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A docs / help export projection.
    DocsExport,
    /// A support / release / evaluation export.
    SupportExport,
}

impl M5ShellGeometryRenderingSurface {
    /// Returns true when the surface narrows the geometry below the desktop full-capability baseline and
    /// therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CompanionApp => "companion_app",
            Self::BrowserReadonly => "browser_readonly",
            Self::CliHeadless => "cli_headless",
            Self::DocsExport => "docs_export",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / high-zoom / high-contrast / snapped-width / CLI reach for a geometry's
/// non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellGeometryNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// An off-screen / pointer-only / view-only surface that traps keyboard / assistive-tech / headless-CLI
    /// users (red).
    ViewOnlyTrap,
}

impl ShellGeometryNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / CLI users.
    pub const fn never_traps(self) -> bool {
        !matches!(self, Self::ViewOnlyTrap)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedReducedButReachable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReachableAndLabeled => "reachable_and_labeled",
            Self::DisclosedReducedButReachable => "disclosed_reduced_but_reachable",
            Self::ViewOnlyTrap => "view_only_trap",
        }
    }
}

/// Whether an export-safe summary preserves the geometry meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellGeometryExportSummaryState {
    /// The geometry meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl ShellGeometryExportSummaryState {
    /// Returns true when the export never falls back to leaking a raw payload.
    pub const fn never_requires_raw_payload(self) -> bool {
        !matches!(self, Self::RequiresRawPayload)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutRawPayload => "reconstructable_without_raw_payload",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::RequiresRawPayload => "requires_raw_payload",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellGeometryNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced geometry, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Geometry, state, or tokens dropped without disclosure (red).
    SilentlyDropped,
}

impl ShellGeometryNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or tokens.
    pub const fn never_drops_silently(self) -> bool {
        !matches!(self, Self::SilentlyDropped)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedNarrowed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ParityPreserved => "parity_preserved",
            Self::DisclosedNarrowed => "disclosed_narrowed",
            Self::SilentlyDropped => "silently_dropped",
        }
    }
}

/// The shell-geometry claim ceiling a family asserts: how strong a trusted / stable posture it lets a surface
/// present. Auto-narrowing lowers this ceiling when a shell-metric / density-mode / responsive-geometry /
/// collapse-priority dimension weakens so a stale metric, an unconfirmed density change, an unconfirmed
/// responsive collapse, or a partially-disclosed collapse boundary can never keep an old
/// `TrustedGeometrySurface` or `ReviewableGeometrySurface` label — geometry meaning is never conveyed by a
/// private width, an off-screen zone, or an unlabeled control alone.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryA11yClaim {
    /// Trusted geometry surface: a fully current, registry-bound, minimum-honoring, presentation-only-density,
    /// recovery-state-preserving, workspace-dominant geometry — the strongest claim, a shell-geometry surface
    /// Aureline can present as exactly trusted and stable right now.
    TrustedGeometrySurface,
    /// Reviewable geometry surface: a self-sufficient, inspectable read-only geometry projection (a static
    /// zone-metric / registry reference a user can inspect) that is not itself an authoritative, live-rendering
    /// surface.
    ReviewableGeometrySurface,
    /// Density-mode-unverified projection: the density mode's presentation-only safety cannot be confirmed; the
    /// geometry stays a density-mode-unverified projection that keeps the last-known information architecture
    /// explicit, never a density change shown as safe when it may rearrange information architecture, focus
    /// order, or trust visibility.
    DensityModeUnverifiedProjection,
    /// Responsive-geometry-unverified projection: a responsive window class's recovery-state preservation
    /// cannot be confirmed; the geometry stays a responsive-geometry-unverified projection that keeps the
    /// task-identity and recovery-critical state inspectable, never a collapse shown as recovery-safe when it
    /// may drop recovery-critical state.
    ResponsiveGeometryUnverifiedProjection,
    /// Collapse-priority-disclosed projection: a collapse boundary can only be partially disclosed; the
    /// geometry stays a collapse-priority-disclosed projection that discloses the partial collapse boundary,
    /// never a private width shown as workspace-dominant when the shell may fracture.
    CollapsePriorityDisclosedProjection,
}

impl M5ShellGeometryA11yClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 5] = [
        Self::TrustedGeometrySurface,
        Self::ReviewableGeometrySurface,
        Self::DensityModeUnverifiedProjection,
        Self::ResponsiveGeometryUnverifiedProjection,
        Self::CollapsePriorityDisclosedProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedGeometrySurface => 4,
            Self::ReviewableGeometrySurface => 3,
            Self::DensityModeUnverifiedProjection => 2,
            Self::ResponsiveGeometryUnverifiedProjection => 1,
            Self::CollapsePriorityDisclosedProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, stable geometry surface.
    pub const fn asserts_trusted_surface(self) -> bool {
        matches!(self, Self::TrustedGeometrySurface)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) surface.
    pub const fn asserts_self_sufficient_surface(self) -> bool {
        matches!(
            self,
            Self::TrustedGeometrySurface | Self::ReviewableGeometrySurface
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedGeometrySurface => "trusted_geometry_surface",
            Self::ReviewableGeometrySurface => "reviewable_geometry_surface",
            Self::DensityModeUnverifiedProjection => "density_mode_unverified_projection",
            Self::ResponsiveGeometryUnverifiedProjection => {
                "responsive_geometry_unverified_projection"
            }
            Self::CollapsePriorityDisclosedProjection => "collapse_priority_disclosed_projection",
        }
    }
}

/// The shell-metric / minimum-size / density-mode / responsive-geometry / collapse-priority dimension whose
/// state governs how far a geometry may claim to be a fully trusted, stable shell surface. The dimensions map
/// 1:1 to the five frozen shell-geometry families so every family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryClaimDimension {
    /// Shell-metric clarity: does the shell-zone metric stay bound to the single shell-metric registry with
    /// its default / minimum / recommended size honored (shell-metric)?
    ShellMetricClarity,
    /// Minimum-size clarity: does the hit target stay at or above the supported minimum for tab width, resize
    /// handle, and icon-only target (minimum-size)?
    MinimumSizeClarity,
    /// Density-mode clarity: does the density mode change presentation only rather than rearranging the
    /// information architecture, focus order, or trust visibility (density-mode)?
    DensityModeClarity,
    /// Responsive-geometry clarity: does the responsive window class preserve task identity and
    /// recovery-critical state under snapped or narrow widths (responsive-geometry)?
    ResponsiveGeometryClarity,
    /// Collapse-priority clarity: does the collapse follow one declared priority that keeps the main workspace
    /// dominant rather than fracturing the shell with a private width (collapse-priority)?
    CollapsePriorityClarity,
}

impl M5ShellGeometryClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::ShellMetricClarity,
        Self::MinimumSizeClarity,
        Self::DensityModeClarity,
        Self::ResponsiveGeometryClarity,
        Self::CollapsePriorityClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ShellMetricClarity => "shell_metric_clarity",
            Self::MinimumSizeClarity => "minimum_size_clarity",
            Self::DensityModeClarity => "density_mode_clarity",
            Self::ResponsiveGeometryClarity => "responsive_geometry_clarity",
            Self::CollapsePriorityClarity => "collapse_priority_clarity",
        }
    }
}

/// The observed condition of one shell-geometry-truth dimension. Anything weaker than
/// [`Self::FullyQualified`] imposes a narrowing ceiling on the geometry's claim. The unconfirmed states the
/// lane must auto-narrow on as *weakened evidence* — an unconfirmed density mode and an unconfirmed responsive
/// collapse — are the states that [`Self::cannot_be_shown_trusted`] flags. A partially-disclosed collapse
/// boundary is an honest disclosed-absence operation (a partial collapse boundary shown honestly with an
/// inspectable note), not a truth overstatement, so it is deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ShellGeometryConditionState {
    /// Fully current, registry-bound, minimum-honoring, presentation-only-density, recovery-state-preserving,
    /// workspace-dominant — imposes no ceiling.
    FullyQualified,
    /// The density mode's presentation-only safety cannot be confirmed — claim drops to a
    /// density-mode-unverified projection.
    DensityModeUnconfirmed,
    /// The responsive window class's recovery-state preservation cannot be confirmed — claim drops to a
    /// responsive-geometry-unverified projection.
    ResponsiveGeometryUnconfirmed,
    /// The collapse boundary can only be partially disclosed — claim drops to a collapse-priority-disclosed
    /// projection.
    CollapsePriorityDisclosedPartial,
}

impl M5ShellGeometryConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullyQualified,
        Self::DensityModeUnconfirmed,
        Self::ResponsiveGeometryUnconfirmed,
        Self::CollapsePriorityDisclosedPartial,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully trusted,
    /// stable geometry surface and must never be shown as such. A partially-disclosed collapse boundary is an
    /// honest disclosed-absence operation (a partial collapse boundary shown honestly with an inspectable
    /// note), not a truth overstatement, so it is deliberately excluded here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::DensityModeUnconfirmed | Self::ResponsiveGeometryUnconfirmed
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5ShellGeometryA11yClaim {
        match self {
            Self::FullyQualified => M5ShellGeometryA11yClaim::TrustedGeometrySurface,
            Self::DensityModeUnconfirmed => {
                M5ShellGeometryA11yClaim::DensityModeUnverifiedProjection
            }
            Self::ResponsiveGeometryUnconfirmed => {
                M5ShellGeometryA11yClaim::ResponsiveGeometryUnverifiedProjection
            }
            Self::CollapsePriorityDisclosedPartial => {
                M5ShellGeometryA11yClaim::CollapsePriorityDisclosedProjection
            }
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each state maps
    /// to the on-topic frozen trigger the freeze matrix already governs, so the certified reason stays
    /// byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5ShellGeometryDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5ShellGeometryDowngradeTrigger::ProofStale,
            Self::DensityModeUnconfirmed => {
                M5ShellGeometryDowngradeTrigger::DensityChangedCommandOrFocusOrTrust
            }
            Self::ResponsiveGeometryUnconfirmed => {
                M5ShellGeometryDowngradeTrigger::ResponsiveCollapseDroppedRecoveryState
            }
            Self::CollapsePriorityDisclosedPartial => M5ShellGeometryDowngradeTrigger::ProofStale,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::DensityModeUnconfirmed => "density_mode_unconfirmed",
            Self::ResponsiveGeometryUnconfirmed => "responsive_geometry_unconfirmed",
            Self::CollapsePriorityDisclosedPartial => "collapse_priority_disclosed_partial",
        }
    }
}

/// One shell-geometry-truth dimension's observed condition on a geometry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5ShellGeometryClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5ShellGeometryConditionState,
}

/// An honest claim auto-narrow block. When a shell-geometry-truth dimension weakens, the geometry's claim
/// lowers to the permitted ceiling, names the binding dimension and frozen trigger, and preserves the
/// canonical geometry identity / last-known registry reference rather than silently dropping it — the
/// underlying shell-metric / minimum-size / density / responsive / collapse truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryClaimAutoNarrow {
    /// The claim the geometry is narrowed to.
    pub narrowed_to: M5ShellGeometryA11yClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling constraint).
    pub binding_dimension: M5ShellGeometryClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5ShellGeometryDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical geometry identity and last-known registry reference are preserved rather than dropped;
    /// must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying shell-metric / minimum-size / density / responsive / collapse truth is preserved (never
    /// dropped) across the narrowing; must hold so shell-metric-unverified, density-mode-unverified,
    /// responsive-geometry-unverified, and collapse-priority-disclosed states never fail opaquely.
    pub preserves_truth_continuity: bool,
}

impl ShellGeometryClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and shell-metric / density /
    /// responsive / collapse truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a geometry's accessible fallback: the same truth must be copyable as text / JSON
/// / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl ShellGeometryCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at least one
    /// export field is named, and a raw-payload-only export is prohibited.
    pub fn is_complete(&self) -> bool {
        self.raw_payload_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5ShellGeometryRenderingSurface,
    /// How the surface discloses its reduced geometry.
    pub state: ShellGeometryNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The geometry affordances reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a shell-geometry accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShellGeometryAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / high-contrast / snapped-width / CLI / export parity with no
    /// narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl ShellGeometryAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one shell-geometry family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryAccessibilityRow {
    /// Record kind; must equal [`SHELL_METRIC_DENSITY_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`SHELL_METRIC_DENSITY_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen geometry family this row certifies.
    pub geometry_family: M5ShellGeometryFamily,
    /// Ref to the frozen canonical per-domain schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the geometry this row represents; stays visible on every surface, so this is never
    /// empty.
    pub geometry_context_ref: String,
    /// Rendered modalities offered; a structure-heavy family must also offer a non-visual (list / textual /
    /// CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5ShellGeometryFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical geometry identity, semantic role, registry
    /// reference, size metric, density mode, and responsive class as the rendered geometry; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: ShellGeometryNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: ShellGeometryNonVisualReachState,
    /// High-zoom (200–400% reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: ShellGeometryNonVisualReachState,
    /// High-contrast / larger-text legibility of the non-visual path.
    pub high_contrast_reach: ShellGeometryNonVisualReachState,
    /// Snapped / narrow window-width safety of the non-visual path.
    pub snapped_width_reach: ShellGeometryNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: ShellGeometryNonVisualReachState,
    /// Whether the export-safe summary preserves geometry meaning.
    pub export_summary: ShellGeometryExportSummaryState,
    /// Ref to the export-safe summary object for this geometry.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: ShellGeometryCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_ready_claim: M5ShellGeometryA11yClaim,
    /// The observed condition of each modeled shell-geometry-truth dimension.
    #[serde(default)]
    pub claim_conditions: Vec<ShellGeometryClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<ShellGeometryClaimAutoNarrow>,
    /// Whether the underlying shell-metric / minimum-size / density / responsive / collapse truth is preserved
    /// on this geometry regardless of narrowing; must hold so every unverified projection never fails
    /// opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this geometry is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5ShellGeometryRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<ShellGeometryRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ShellGeometryRequiredLabel>,
    /// Semantic consumer surfaces this geometry is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5ShellGeometryConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ShellGeometryAccessibilityRow {
    /// Returns true when this family renders a dense, structured surface and must bind to a flat non-visual
    /// path.
    pub const fn is_structure_heavy(&self) -> bool {
        family_is_structure_heavy(self.geometry_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model that
    /// dimension.
    pub fn condition_for(
        &self,
        dimension: M5ShellGeometryClaimDimension,
    ) -> M5ShellGeometryConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5ShellGeometryConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the family's
    /// full claim.
    pub fn permitted_claim(&self) -> M5ShellGeometryA11yClaim {
        let mut permitted = self.full_ready_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows below
    /// the family's full claim.
    pub fn binding_condition(&self) -> Option<&ShellGeometryClaimConditionEntry> {
        let mut binding: Option<(&ShellGeometryClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_ready_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition, rank)),
            }
        }
        binding.map(|(condition, _)| condition)
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any.
    pub fn binding_dimension(&self) -> Option<M5ShellGeometryClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this geometry effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5ShellGeometryA11yClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale shell metric, an unconfirmed density change, an unconfirmed
    /// responsive collapse, or a partially-disclosed collapse boundary can no longer keep an old
    /// `TrustedGeometrySurface` / `ReviewableGeometrySurface` label. The effective claim never exceeds the
    /// permitted ceiling; when a dimension narrows below the full claim, an honest narrow block is present,
    /// narrows to exactly the permitted ceiling, binds to the ceiling-imposing dimension with its frozen
    /// trigger, and preserves canonical identity and truth. When nothing narrows, no spurious narrow block is
    /// present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_condition()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding.dimension
                    && narrow.trigger == binding.state.default_trigger()
                    && binding.state.is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / trusted honesty: a stale-shell-metric / unconfirmed-density / unconfirmed-responsive state never
    /// keeps a trusted claim — geometry meaning is never conveyed by a private width, an off-screen zone, or
    /// an unlabeled control alone. When such a state is modeled, the effective claim must not assert
    /// `TrustedGeometrySurface`.
    pub fn trusted_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_trusted());
        !(has_unprovable_state && self.effective_claim().asserts_trusted_surface())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / high-contrast / snapped-width / CLI trap, a structure-heavy
    /// family offers a non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.geometry_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.high_contrast_reach.never_traps()
            && self.snapped_width_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the geometry meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: every unverified projection preserves the underlying shell-metric / minimum-size /
    /// density / responsive / collapse truth. The row must assert `truth_preserved`, and any narrow block must
    /// preserve truth continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the geometry carries an honest claim
    /// narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.high_contrast_reach.is_disclosed_reduction()
            || self.snapped_width_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced geometry and
    /// keeps its labels, so product / help / release publication stay aligned on the same narrowed state.
    pub fn narrowing_disclosed(&self) -> bool {
        // Every declared narrowed rendering surface has a disclosure entry.
        for surface in &self.rendering_surfaces {
            if surface.is_narrowed()
                && !self
                    .narrowing_disclosures
                    .iter()
                    .any(|d| d.rendering_surface == *surface)
            {
                return false;
            }
        }
        // Every disclosure never silently drops and preserves labels on a narrowed surface.
        self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        })
    }

    /// Whether the row models its family's primary weakening dimension.
    pub fn models_primary_dimension(&self) -> bool {
        let primary = family_primary_dimension(self.geometry_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5ShellGeometryRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> ShellGeometryAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return ShellGeometryAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            ShellGeometryAccessibilityStatus::NarrowedDisclosed
        } else {
            ShellGeometryAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == SHELL_METRIC_DENSITY_A11Y_ROW_RECORD_KIND
            && self.schema_version == SHELL_METRIC_DENSITY_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.geometry_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} \
high_zoom={high_zoom} high_contrast={high_contrast} snapped_width={snapped_width} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.geometry_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            high_contrast = self.high_contrast_reach.as_str(),
            snapped_width = self.snapped_width_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1162 shell-metric-density accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub structure_heavy_family_count: usize,
    pub all_structure_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_trusted_honesty_holds: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_truth_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`ShellGeometryAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShellGeometryAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<ShellGeometryAccessibilityRow>,
}

/// Checked-in M05-1162 shell-metric-density accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ShellGeometryAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<ShellGeometryAccessibilityRow>,
    pub summary: ShellGeometryAccessibilitySummary,
}

impl ShellGeometryAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: ShellGeometryAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: SHELL_METRIC_DENSITY_A11Y_SCHEMA_VERSION,
            record_kind: SHELL_METRIC_DENSITY_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: ShellGeometryAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                structure_heavy_family_count: 0,
                all_structure_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_trusted_honesty_holds: false,
                all_export_summaries_preserve_meaning: false,
                all_truth_preserved: false,
                all_narrowing_disclosed: false,
                green_count: 0,
                yellow_count: 0,
                red_count: 0,
                rendering_surface_count: 0,
                consumer_surface_count: 0,
            },
        };
        packet.summary = packet.computed_summary();
        packet
    }

    /// Families represented by some row in this packet.
    pub fn represented_families(&self) -> BTreeSet<M5ShellGeometryFamily> {
        self.rows.iter().map(|r| r.geometry_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5ShellGeometryClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5ShellGeometryConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5ShellGeometryA11yClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5ShellGeometryConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ShellGeometryAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5ShellGeometryConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&ShellGeometryAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                ShellGeometryAccessibilityStatus::Parity => green += 1,
                ShellGeometryAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                ShellGeometryAccessibilityStatus::Stranded => red += 1,
            }
        }

        ShellGeometryAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            structure_heavy_family_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(ShellGeometryAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(ShellGeometryAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(ShellGeometryAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(ShellGeometryAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(ShellGeometryAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(ShellGeometryAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ShellGeometryAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != SHELL_METRIC_DENSITY_A11Y_SCHEMA_VERSION {
            violations.push(ShellGeometryAccessibilityViolation::SchemaVersion {
                expected: SHELL_METRIC_DENSITY_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != SHELL_METRIC_DENSITY_A11Y_RECORD_KIND {
            violations.push(ShellGeometryAccessibilityViolation::RecordKind {
                expected: SHELL_METRIC_DENSITY_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ShellGeometryAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ShellGeometryAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.geometry_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(ShellGeometryAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    ShellGeometryAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.geometry_family),
                    },
                );
            }

            // Each row must preserve every mandatory geometry label.
            if !row.preserves_mandatory_labels() {
                violations.push(ShellGeometryAccessibilityViolation::MissingMandatoryLabel {
                    id: row.row_id.clone(),
                });
            }

            // A structure-heavy family must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5ShellGeometryFallbackModality::Structured)
            {
                violations.push(
                    ShellGeometryAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(ShellGeometryAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC / trusted honesty: a stale-shell-metric / unconfirmed-density / unconfirmed-responsive state
            // never keeps a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(
                    ShellGeometryAccessibilityViolation::WeakStateShownAsTrusted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(ShellGeometryAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    ShellGeometryAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve shell-metric / density / responsive / collapse truth.
            if !row.preserves_truth_continuity() {
                violations.push(ShellGeometryAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    ShellGeometryAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(ShellGeometryAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == ShellGeometryAccessibilityStatus::Stranded {
                violations.push(ShellGeometryAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5ShellGeometryFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(ShellGeometryAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5ShellGeometryClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    ShellGeometryAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5ShellGeometryConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    ShellGeometryAccessibilityViolation::MissingConditionStateCoverage { state },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → collapse-priority-disclosed) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5ShellGeometryA11yClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(ShellGeometryAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        // Trusted honesty must be proven with at least one stale-shell-metric / unconfirmed-density /
        // unconfirmed-responsive row in the packet, so the "cannot-prove never shown as trusted" guarantee is
        // exercised end-to-end.
        if !has_unprovable_row {
            violations.push(ShellGeometryAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the shell, editor, review, notebook, data,
        // settings, CLI-export, support-export, and product surfaces — so every consumer surface is exercised
        // at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5ShellGeometryConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    ShellGeometryAccessibilityViolation::MissingConsumerSurfaceCoverage { surface },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ShellGeometryAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("shell-metric-density accessibility parity packet serializes"),
        ) {
            violations.push(ShellGeometryAccessibilityViolation::RawGeometryMaterialInExport);
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
            .expect("shell-metric-density accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,geometry_family,keyboard_reach,screen_reader_reach,high_zoom_reach,high_contrast_reach,snapped_width_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{high_zoom},{high_contrast},{snapped_width},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.geometry_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                high_contrast = row.high_contrast_reach.as_str(),
                snapped_width = row.snapped_width_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_ready_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, help, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Shell-Metric-Density Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5ShellGeometryFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.geometry_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_ready_claim.as_str(),
                    narrow.narrowed_to.as_str(),
                    narrow.binding_dimension.as_str(),
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in shell-metric-density accessibility parity export.
pub fn current_m5_shell_metric_density_a11y_export(
) -> Result<ShellGeometryAccessibilityPacket, ShellGeometryAccessibilityArtifactError> {
    let packet: ShellGeometryAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-shell-metric-density-accessibility-parity/support_export.json"
    )))
    .map_err(ShellGeometryAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ShellGeometryAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in shell-metric-density accessibility parity export.
#[derive(Debug)]
pub enum ShellGeometryAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ShellGeometryAccessibilityViolation>),
}

impl fmt::Display for ShellGeometryAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "shell-metric-density accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "shell-metric-density accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ShellGeometryAccessibilityArtifactError {}

/// Validation failure for M05-1162 shell-metric-density accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShellGeometryAccessibilityViolation {
    SchemaVersion {
        expected: u32,
        actual: u32,
    },
    RecordKind {
        expected: String,
        actual: String,
    },
    MissingIdentity,
    DuplicateId {
        id: String,
    },
    IncompleteRow {
        id: String,
    },
    MissingPrimaryDimension {
        id: String,
        dimension: M5ShellGeometryClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    StructureHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    WeakStateShownAsTrusted {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresRawPayload {
        id: String,
    },
    TruthDropped {
        id: String,
    },
    NarrowingDropsContextSilently {
        id: String,
    },
    MissingConsumerParity {
        id: String,
    },
    StrandedRow {
        id: String,
    },
    MissingFamilyCoverage {
        family: M5ShellGeometryFamily,
    },
    MissingDimensionCoverage {
        dimension: M5ShellGeometryClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5ShellGeometryConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5ShellGeometryA11yClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5ShellGeometryConsumerSurface,
    },
    SummaryMismatch,
    RawGeometryMaterialInExport,
}

impl ShellGeometryAccessibilityViolation {
    /// Stable token for CLI / support handoff.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::SchemaVersion { .. } => "schema_version",
            Self::RecordKind { .. } => "record_kind",
            Self::MissingIdentity => "missing_identity",
            Self::DuplicateId { .. } => "duplicate_id",
            Self::IncompleteRow { .. } => "incomplete_row",
            Self::MissingPrimaryDimension { .. } => "missing_primary_dimension",
            Self::MissingMandatoryLabel { .. } => "missing_mandatory_label",
            Self::StructureHeavyMissingStructured { .. } => "structure_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::WeakStateShownAsTrusted { .. } => "weak_state_shown_as_trusted",
            Self::AssistiveTechStranded { .. } => "assistive_tech_stranded",
            Self::ExportRequiresRawPayload { .. } => "export_requires_raw_payload",
            Self::TruthDropped { .. } => "truth_dropped",
            Self::NarrowingDropsContextSilently { .. } => "narrowing_drops_context_silently",
            Self::MissingConsumerParity { .. } => "missing_consumer_parity",
            Self::StrandedRow { .. } => "stranded_row",
            Self::MissingFamilyCoverage { .. } => "missing_family_coverage",
            Self::MissingDimensionCoverage { .. } => "missing_dimension_coverage",
            Self::MissingConditionStateCoverage { .. } => "missing_condition_state_coverage",
            Self::MissingClaimTierCoverage { .. } => "missing_claim_tier_coverage",
            Self::TrustedHonestyUnproven => "trusted_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawGeometryMaterialInExport => "raw_geometry_material_in_export",
        }
    }
}

impl fmt::Display for ShellGeometryAccessibilityViolation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SchemaVersion { expected, actual } => {
                write!(
                    f,
                    "schema version mismatch: expected {expected}, got {actual}"
                )
            }
            Self::RecordKind { expected, actual } => {
                write!(f, "record kind mismatch: expected {expected}, got {actual}")
            }
            Self::MissingIdentity => write!(f, "packet identity fields are missing"),
            Self::DuplicateId { id } => write!(f, "duplicate row id: {id}"),
            Self::IncompleteRow { id } => write!(f, "incomplete accessibility row: {id}"),
            Self::MissingPrimaryDimension { id, dimension } => {
                write!(
                    f,
                    "row {id} does not model its family's primary dimension {}",
                    dimension.as_str()
                )
            }
            Self::MissingMandatoryLabel { id } => {
                write!(f, "row {id} drops a mandatory geometry label")
            }
            Self::StructureHeavyMissingStructured { id } => {
                write!(
                    f,
                    "structure-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a trusted / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::WeakStateShownAsTrusted { id } => {
                write!(
                    f,
                    "row {id} shows a stale-shell-metric / unconfirmed-density / unconfirmed-responsive state as a trusted geometry surface"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-zoom / high-contrast / snapped-width / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresRawPayload { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without leaking a raw payload"
                )
            }
            Self::TruthDropped { id } => {
                write!(
                    f,
                    "row {id} does not preserve shell-metric / minimum-size / density / responsive / collapse truth across narrowing"
                )
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows a rendering surface without disclosing it"
                )
            }
            Self::MissingConsumerParity { id } => {
                write!(f, "row {id} is missing secondary consumer parity")
            }
            Self::StrandedRow { id } => write!(f, "row {id} is stranded (red) and may not ship"),
            Self::MissingFamilyCoverage { family } => {
                write!(
                    f,
                    "geometry family {family:?} is not certified in the packet"
                )
            }
            Self::MissingDimensionCoverage { dimension } => {
                write!(
                    f,
                    "claim dimension {} is not exercised in the packet",
                    dimension.as_str()
                )
            }
            Self::MissingConditionStateCoverage { state } => {
                write!(
                    f,
                    "condition state {} is not exercised in the packet",
                    state.as_str()
                )
            }
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::TrustedHonestyUnproven => {
                write!(
                    f,
                    "no stale-shell-metric / unconfirmed-density / unconfirmed-responsive row is present to prove the trusted-honesty guarantee"
                )
            }
            Self::MissingConsumerSurfaceCoverage { surface } => {
                write!(
                    f,
                    "consumer surface {} does not ingest any row in the packet",
                    surface.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawGeometryMaterialInExport => {
                write!(f, "export contains raw geometry material")
            }
        }
    }
}

impl Error for ShellGeometryAccessibilityViolation {}

/// Whether a narrowed label is a generic non-answer rather than a precise label.
fn label_is_generic(label: &str) -> bool {
    let trimmed = label.trim();
    if trimmed.is_empty() {
        return true;
    }
    let lower = trimmed.to_lowercase();
    matches!(
        lower.as_str(),
        "unsupported"
            | "not supported"
            | "unavailable"
            | "not available"
            | "n/a"
            | "error"
            | "failed"
            | "degraded"
            | "narrowed"
            | "fallback"
            | "reduced"
            | "blocked"
            | "unresolved"
            | "partial"
            | "stale"
            | "incomplete"
            | "not comparable"
            | "restricted"
            | "collapsed"
            | "ellipsis"
            | "mixed"
            | "expired"
            | "inferred"
            | "unverified"
            | "trusted"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// The canonical packet id for the checked-in stable export.
pub const SHELL_METRIC_DENSITY_A11Y_PACKET_ID: &str =
    "m5-shell-metric-density-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in shell-metric-density accessibility parity packet. This is the one source
/// of truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_shell_metric_density_a11y_packet() -> ShellGeometryAccessibilityPacket {
    ShellGeometryAccessibilityPacket::new(ShellGeometryAccessibilityPacketInput {
        packet_id: SHELL_METRIC_DENSITY_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-13T00:00:00Z".to_owned(),
        matrix_ref: SHELL_METRIC_DENSITY_A11Y_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:shell-metric-density-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5ShellGeometryRequiredLabel> {
    M5ShellGeometryRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> ShellGeometryCopyExportParity {
    ShellGeometryCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5ShellGeometryClaimDimension,
    state: M5ShellGeometryConditionState,
) -> ShellGeometryClaimConditionEntry {
    ShellGeometryClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the general product
/// UI — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5ShellGeometryConsumerSurface]) -> Vec<M5ShellGeometryConsumerSurface> {
    let mut out = vec![
        M5ShellGeometryConsumerSurface::SupportExport,
        M5ShellGeometryConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full label
/// and summary parity on the narrower surfaces; a narrowed row discloses the reduced geometry it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: ShellGeometryNarrowingDisclosureState,
) -> Vec<ShellGeometryRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        ShellGeometryRenderingNarrowingDisclosure {
            rendering_surface: M5ShellGeometryRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_resize_affordance".to_owned()],
        },
        ShellGeometryRenderingNarrowingDisclosure {
            rendering_surface: M5ShellGeometryRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_drag_affordance".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<ShellGeometryRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        ShellGeometryNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced geometry while
/// preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<ShellGeometryRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        ShellGeometryNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5ShellGeometryRenderingSurface> {
    vec![
        M5ShellGeometryRenderingSurface::DesktopFull,
        M5ShellGeometryRenderingSurface::CliHeadless,
        M5ShellGeometryRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5ShellGeometryFallbackModality> {
    vec![
        M5ShellGeometryFallbackModality::List,
        M5ShellGeometryFallbackModality::Textual,
        M5ShellGeometryFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5ShellGeometryFallbackModality> {
    vec![
        M5ShellGeometryFallbackModality::Structured,
        M5ShellGeometryFallbackModality::List,
        M5ShellGeometryFallbackModality::Textual,
        M5ShellGeometryFallbackModality::Cli,
    ]
}

const REACHABLE: ShellGeometryNonVisualReachState =
    ShellGeometryNonVisualReachState::ReachableAndLabeled;
const REDUCED: ShellGeometryNonVisualReachState =
    ShellGeometryNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<ShellGeometryAccessibilityRow> {
    vec![
        // Minimum size (tab / resize / icon-only hit targets at or above the supported minimum) — the
        // minimum-size family keeps every hit target at or above its supported minimum with a labeled
        // pointer-and-keyboard-reachable affordance, so it is a trusted geometry surface reachable on every
        // surface with no narrowing (green). Structure-heavy: its hit-target registry binds to a flat list /
        // textual path.
        ShellGeometryAccessibilityRow {
            record_kind: SHELL_METRIC_DENSITY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: SHELL_METRIC_DENSITY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:minimum-size-hit-targets-meet-supported-minimum".to_owned(),
            geometry_family: M5ShellGeometryFamily::MinimumSize,
            source_family_schema_ref: M5ShellGeometryFamily::MinimumSize
                .canonical_domain_schema_ref()
                .to_owned(),
            geometry_context_ref: "editor:minimum-size:0001".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            snapped_width_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ShellGeometryExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:minimum-size-hit-targets-meet-supported-minimum:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "geometry_identity",
                "semantic_role",
                "registry_reference",
                "hit_target_minimum",
            ]),
            full_ready_claim: M5ShellGeometryA11yClaim::TrustedGeometrySurface,
            claim_conditions: vec![condition(
                M5ShellGeometryClaimDimension::MinimumSizeClarity,
                M5ShellGeometryConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "geometry_identity",
                "semantic_role",
                "registry_reference",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ShellGeometryConsumerSurface::EditorUi,
                M5ShellGeometryConsumerSurface::ReviewUi,
            ]),
            source_refs: vec![
                "UX Style Guide §9.1–§9.7 — Spacing / sizing / minimum hit targets".to_owned(),
                SHELL_METRIC_DENSITY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("minimum-size-hit-targets-meet-supported-minimum"),
        },
        // Shell metric (zone metrics bound to the single shell-metric registry) — the shell-metric family
        // keeps every shell-zone size bound to the one shell-metric registry with its default / minimum /
        // recommended size honored, so it is a self-sufficient reviewable geometry surface a user can inspect,
        // but its narrower non-visual traversal discloses a reduced high-zoom reflow walk (yellow).
        // Structure-heavy: its zone-metric registry binds to a flat list / textual path.
        ShellGeometryAccessibilityRow {
            record_kind: SHELL_METRIC_DENSITY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: SHELL_METRIC_DENSITY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:shell-metric-zone-metrics-bound-to-registry".to_owned(),
            geometry_family: M5ShellGeometryFamily::ShellMetric,
            source_family_schema_ref: M5ShellGeometryFamily::ShellMetric
                .canonical_domain_schema_ref()
                .to_owned(),
            geometry_context_ref: "shell:shell-metric:0002".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            high_contrast_reach: REACHABLE,
            snapped_width_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ShellGeometryExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:shell-metric-zone-metrics-bound-to-registry:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "geometry_identity",
                "semantic_role",
                "registry_reference",
                "size_metric",
            ]),
            full_ready_claim: M5ShellGeometryA11yClaim::ReviewableGeometrySurface,
            claim_conditions: vec![condition(
                M5ShellGeometryClaimDimension::ShellMetricClarity,
                M5ShellGeometryConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "geometry_identity",
                "semantic_role",
                "size_metric",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ShellGeometryConsumerSurface::ShellUi,
                M5ShellGeometryConsumerSurface::DataUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.1–§6.2 — Canonical shell layout / default shell metrics".to_owned(),
                SHELL_METRIC_DENSITY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("shell-metric-zone-metrics-bound-to-registry"),
        },
        // Density mode (presentation-only safety unconfirmed) — the density mode's presentation-only safety
        // cannot be confirmed, so it auto-narrows to a density-mode-unverified projection that keeps the
        // last-known information architecture explicit, never a density change shown as safe when it may
        // rearrange information architecture, focus order, or trust visibility (yellow). Its condensed
        // presentation narrows the high-contrast path to a disclosed reduction.
        ShellGeometryAccessibilityRow {
            record_kind: SHELL_METRIC_DENSITY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: SHELL_METRIC_DENSITY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:density-mode-presentation-only-unconfirmed".to_owned(),
            geometry_family: M5ShellGeometryFamily::DensityMode,
            source_family_schema_ref: M5ShellGeometryFamily::DensityMode
                .canonical_domain_schema_ref()
                .to_owned(),
            geometry_context_ref: "shell:density-mode:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REDUCED,
            snapped_width_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ShellGeometryExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:density-mode-presentation-only-unconfirmed:a11y".to_owned(),
            copy_export: copy_export(&[
                "geometry_identity",
                "semantic_role",
                "registry_reference",
                "density_mode",
            ]),
            full_ready_claim: M5ShellGeometryA11yClaim::TrustedGeometrySurface,
            claim_conditions: vec![condition(
                M5ShellGeometryClaimDimension::DensityModeClarity,
                M5ShellGeometryConditionState::DensityModeUnconfirmed,
            )],
            claim_narrow: Some(ShellGeometryClaimAutoNarrow {
                narrowed_to: M5ShellGeometryA11yClaim::DensityModeUnverifiedProjection,
                binding_dimension: M5ShellGeometryClaimDimension::DensityModeClarity,
                trigger: M5ShellGeometryDowngradeTrigger::DensityChangedCommandOrFocusOrTrust,
                narrowed_label:
                    "This density mode cannot confirm that it changes presentation only — shown as a density-mode-unverified projection that keeps the last-known information architecture explicit, never presenting a density change as safe when it may rearrange information architecture, focus order, or trust visibility"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "geometry_identity",
                "semantic_role",
                "density_mode",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ShellGeometryConsumerSurface::NotebookUi,
                M5ShellGeometryConsumerSurface::SettingsUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.3–§6.4 — Density modes".to_owned(),
                SHELL_METRIC_DENSITY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("density-mode-presentation-only-unconfirmed"),
        },
        // Responsive geometry (recovery-state preservation unconfirmed) — the responsive window class's
        // recovery-state preservation cannot be confirmed, so it auto-narrows to a
        // responsive-geometry-unverified projection that keeps the task-identity and recovery-critical state
        // inspectable, never a collapse shown as recovery-safe when it may drop recovery-critical state
        // (yellow). Its snapped-width behavior narrows the snapped-width path to a disclosed reduction.
        ShellGeometryAccessibilityRow {
            record_kind: SHELL_METRIC_DENSITY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: SHELL_METRIC_DENSITY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:responsive-geometry-recovery-state-unconfirmed".to_owned(),
            geometry_family: M5ShellGeometryFamily::ResponsiveGeometry,
            source_family_schema_ref: M5ShellGeometryFamily::ResponsiveGeometry
                .canonical_domain_schema_ref()
                .to_owned(),
            geometry_context_ref: "shell:responsive-geometry:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            snapped_width_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: ShellGeometryExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:responsive-geometry-recovery-state-unconfirmed:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "geometry_identity",
                "semantic_role",
                "registry_reference",
                "responsive_class",
            ]),
            full_ready_claim: M5ShellGeometryA11yClaim::TrustedGeometrySurface,
            claim_conditions: vec![condition(
                M5ShellGeometryClaimDimension::ResponsiveGeometryClarity,
                M5ShellGeometryConditionState::ResponsiveGeometryUnconfirmed,
            )],
            claim_narrow: Some(ShellGeometryClaimAutoNarrow {
                narrowed_to: M5ShellGeometryA11yClaim::ResponsiveGeometryUnverifiedProjection,
                binding_dimension: M5ShellGeometryClaimDimension::ResponsiveGeometryClarity,
                trigger: M5ShellGeometryDowngradeTrigger::ResponsiveCollapseDroppedRecoveryState,
                narrowed_label:
                    "This responsive window class cannot confirm that it preserves recovery-critical state — shown as a responsive-geometry-unverified projection that keeps the task identity and recovery-critical state inspectable, never presenting a snapped-width collapse as recovery-safe when it may drop recovery-critical state"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "geometry_identity",
                "semantic_role",
                "responsive_class",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ShellGeometryConsumerSurface::CliExport,
                M5ShellGeometryConsumerSurface::ShellUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.5–§6.6 — Responsive window classes / no-fracture geometry".to_owned(),
                SHELL_METRIC_DENSITY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("responsive-geometry-recovery-state-unconfirmed"),
        },
        // Collapse priority (boundary disclosed partial) — the collapse boundary can only disclose a partial
        // collapse boundary, so it auto-narrows to a collapse-priority-disclosed projection that discloses the
        // partial boundary alongside the last-known collapse order, never a private width shown as
        // workspace-dominant when the shell may fracture (yellow). Structure-heavy: its collapse-order stack
        // binds to a flat list / textual path. A partial boundary disclosure is an honest disclosed-absence
        // operation, not a trusted overstatement.
        ShellGeometryAccessibilityRow {
            record_kind: SHELL_METRIC_DENSITY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: SHELL_METRIC_DENSITY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:collapse-priority-boundary-disclosed-partial".to_owned(),
            geometry_family: M5ShellGeometryFamily::CollapsePriority,
            source_family_schema_ref: M5ShellGeometryFamily::CollapsePriority
                .canonical_domain_schema_ref()
                .to_owned(),
            geometry_context_ref: "settings:collapse-priority:0005".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            high_contrast_reach: REACHABLE,
            snapped_width_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: ShellGeometryExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:collapse-priority-boundary-disclosed-partial:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "geometry_identity",
                "semantic_role",
                "registry_reference",
                "collapse_priority_order",
            ]),
            full_ready_claim: M5ShellGeometryA11yClaim::TrustedGeometrySurface,
            claim_conditions: vec![condition(
                M5ShellGeometryClaimDimension::CollapsePriorityClarity,
                M5ShellGeometryConditionState::CollapsePriorityDisclosedPartial,
            )],
            claim_narrow: Some(ShellGeometryClaimAutoNarrow {
                narrowed_to: M5ShellGeometryA11yClaim::CollapsePriorityDisclosedProjection,
                binding_dimension: M5ShellGeometryClaimDimension::CollapsePriorityClarity,
                trigger: M5ShellGeometryDowngradeTrigger::ProofStale,
                narrowed_label:
                    "This collapse priority can only disclose a partial collapse boundary — shown as a collapse-priority-disclosed projection that discloses the partial boundary alongside the last-known collapse order, never presenting a private collapse width as workspace-dominant when the shell may fracture below its minimum"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "geometry_identity",
                "semantic_role",
                "collapse_priority_order",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5ShellGeometryConsumerSurface::DataUi,
                M5ShellGeometryConsumerSurface::SettingsUi,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.6 — Collapse priorities / no-fracture geometry".to_owned(),
                SHELL_METRIC_DENSITY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-13T00:00:00Z".to_owned(),
            evidence_refs: ev("collapse-priority-boundary-disclosed-partial"),
        },
    ]
}
