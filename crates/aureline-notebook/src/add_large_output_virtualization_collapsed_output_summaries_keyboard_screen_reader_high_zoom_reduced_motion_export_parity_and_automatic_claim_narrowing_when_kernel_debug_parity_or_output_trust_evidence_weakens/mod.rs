//! Large-output virtualization, collapsed-output summaries, keyboard / screen-reader / high-zoom /
//! reduced-motion / CLI / export parity, and honest automatic claim narrowing for the M5
//! notebook-document-header / kernel-state-strip / kernel-picker-row / kernel-origin-pill /
//! output-trust-banner / output-provenance-chip-group / restart-consequence-card /
//! kernel-recovery-card notebook components.
//!
//! This module is the M05-1089 accessibility-and-auto-narrowing capstone over the frozen M5
//! notebook-kernel-output component matrix
//! ([`crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix`]).
//! Where the freeze matrix defines the reusable notebook document header, kernel state strip, kernel
//! picker row, kernel origin pill, output trust banner, output provenance chip group, restart
//! consequence card, and kernel recovery card primitives, and the 1085-1088 implementation lanes
//! resolve their per-surface truth, this lane certifies — per component family — that notebook
//! claims stay **keyboard-complete, assistive-tech-reachable, high-zoom / reduced-motion-safe,
//! CLI/export-safe, virtualization-honest, and self-narrowing** rather than presenting a kernel with
//! partial parity, an unsupported debugger, a stale output, a degraded kernel origin, a severed
//! environment provenance, a kernel-free strip, or a dense / collapsed output as still a fully live,
//! trusted, provenanced result:
//!
//! - **Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, high-zoom-legible, reduced-motion-safe, and
//!   CLI/headless-reachable path into the same document identity, selected kernel origin / class,
//!   execution / connection state, output trust class, output freshness, provenance lineage, and
//!   restart / recovery consequence the rich component shows — never a hover-only chip that strands
//!   assistive-tech or headless-CLI users. Hierarchy-heavy families (the output provenance chip
//!   group's nested source / transform / lineage chips) additionally bind their group to a flat list
//!   / textual path.
//! - **Large-output virtualization & collapsed-output summaries.** When a dense output is virtualized
//!   or collapsed to a summary, the component keeps the run identity, output trust class, and
//!   stale-versus-live truth attached to the truncated view rather than degrading it into an
//!   anonymous blob — a virtualized or collapsed output is never rendered as a trust-less,
//!   attribution-less payload.
//! - **Export parity.** The support / release / CLI export reconstructs each component's meaning from
//!   typed tokens and opaque refs **without a raw payload**, preserving the same document identity,
//!   kernel origin / class, output trust / freshness, provenance, and restart / recovery posture
//!   shown in-product so support, docs, and release proof can reconstruct exactly what the user was
//!   actually shown without leaking a raw kernel session or output payload.
//! - **Honest auto-narrowing.** When kernel parity is partial, a debugger is unsupported, an output's
//!   trust evidence is stale, a kernel origin is degraded, an environment's provenance is severed, or
//!   no kernel is available, the component's claim auto-narrows from `live_trusted_result` /
//!   `reviewable_result` to a partial-kernel-parity / debugger-unsupported / degraded-origin /
//!   stale-output / unprovenanced-environment / no-kernel projection, discloses the narrowing with a
//!   precise trigger and binding dimension, and preserves the canonical document identity / kernel
//!   origin / output provenance. The underlying notebook truth is never dropped opaquely. A component
//!   with every dimension intact must NOT carry a spurious narrowing, and a partial-parity /
//!   stale-output / degraded-origin / severed-provenance state can never keep a live-trusted claim —
//!   a stale output never masquerades as live truth.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the notebook UI, the
//!   kernel-manager UI, the output-viewer UI, the debugger UI, the AI-context UI, the review UI, the
//!   CLI surface, the support export, and the product UI so product, docs, and release publication
//!   stay aligned on downgrade behavior rather than drifting in copy — a live-looking surface can
//!   never outrun the kernel / output / trust proof it is being viewed away from.
//!
//! Each [`NotebookComponentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix::M5NotebookKernelOutputComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen [`M5NotebookKernelOutputRequiredLabel`],
//! [`M5NotebookKernelOutputDowngradeTrigger`], and shared [`M5NotebookKernelOutputConsumerSurface`]
//! consumer surfaces rather than minting parallel synonyms, so the certified labels stay
//! byte-identical to the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw kernel sessions, output payloads, credentials, tokens, request
//! bodies, and endpoint secrets never cross this boundary; the packet carries only typed class
//! tokens, opaque notebook refs, booleans, and controlled labels so support, release, and
//! diagnostics exports can reconstruct exactly what an accessible fallback would have shown without
//! leaking sensitive material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families,
// required labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::freeze_the_m5_notebook_document_header_kernel_state_strip_kernel_picker_row_kernel_origin_pill_output_trust_banner_output_provenance_chip_group_restart_consequence_card_and_kernel_recovery_card_component_matrix::{
    M5NotebookKernelOutputComponentFamily, M5NotebookKernelOutputConsumerSurface,
    M5NotebookKernelOutputDowngradeTrigger, M5NotebookKernelOutputRequiredLabel,
    M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1089 notebook-component accessibility parity packet.
pub const NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`NotebookComponentAccessibilityPacket`].
pub const NOTEBOOK_KERNEL_OUTPUT_A11Y_RECORD_KIND: &str =
    "m5_notebook_kernel_output_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`NotebookComponentAccessibilityRow`].
pub const NOTEBOOK_KERNEL_OUTPUT_A11Y_ROW_RECORD_KIND: &str =
    "m5_notebook_kernel_output_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-notebook-kernel-output-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const NOTEBOOK_KERNEL_OUTPUT_A11Y_DOC_REF: &str =
    "docs/notebooks/m5_notebook_kernel_output_component_accessibility_parity.md";

/// Repo-relative path of the frozen notebook-kernel-output component matrix this lane certifies.
pub const NOTEBOOK_KERNEL_OUTPUT_A11Y_COMPONENT_MATRIX_REF: &str =
    M5_NOTEBOOK_KERNEL_OUTPUT_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const NOTEBOOK_KERNEL_OUTPUT_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-notebook-kernel-output-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const NOTEBOOK_KERNEL_OUTPUT_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-notebook-kernel-output-component-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const NOTEBOOK_KERNEL_OUTPUT_A11Y_CSV_REF: &str =
    "artifacts/release/m5-notebook-kernel-output-component-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const NOTEBOOK_KERNEL_OUTPUT_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-notebook-kernel-output-component-accessibility-parity.md";

/// The reusable component families that render a non-linear hierarchy (the output provenance chip
/// group's nested source / transform / derived-output lineage chips) and therefore MUST bind their
/// group to an equivalent flat list / textual path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5NotebookKernelOutputComponentFamily) -> bool {
    matches!(
        family,
        M5NotebookKernelOutputComponentFamily::OutputProvenanceChipGroup
    )
}

/// The notebook dimension whose weakening a family primarily discloses. Every row must model at
/// least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5NotebookKernelOutputComponentFamily,
) -> M5NotebookComponentClaimDimension {
    match family {
        M5NotebookKernelOutputComponentFamily::NotebookDocumentHeader => {
            M5NotebookComponentClaimDimension::DocumentIdentity
        }
        M5NotebookKernelOutputComponentFamily::KernelStateStrip => {
            M5NotebookComponentClaimDimension::KernelLiveness
        }
        M5NotebookKernelOutputComponentFamily::KernelPickerRow => {
            M5NotebookComponentClaimDimension::KernelParity
        }
        M5NotebookKernelOutputComponentFamily::KernelOriginPill => {
            M5NotebookComponentClaimDimension::KernelOriginProvenance
        }
        M5NotebookKernelOutputComponentFamily::OutputTrustBanner => {
            M5NotebookComponentClaimDimension::OutputTrustEvidence
        }
        M5NotebookKernelOutputComponentFamily::OutputProvenanceChipGroup => {
            M5NotebookComponentClaimDimension::OutputProvenance
        }
        M5NotebookKernelOutputComponentFamily::RestartConsequenceCard => {
            M5NotebookComponentClaimDimension::RestartConsequenceClarity
        }
        M5NotebookKernelOutputComponentFamily::KernelRecoveryCard => {
            M5NotebookComponentClaimDimension::RecoveryContinuity
        }
    }
}

/// A rendered fallback modality for a notebook component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookComponentFallbackModality {
    /// A rich, structured (nested source / transform / derived-output lineage) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5NotebookComponentFallbackModality {
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

/// A rendering-surface capability tier. Distinct from the semantic consumer surface: the same
/// component may render at desktop-full capability or narrow to a companion, read-only browser,
/// headless CLI, docs export, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookComponentRenderingSurface {
    /// The full-capability desktop notebook surface.
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

impl M5NotebookComponentRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability
    /// baseline and therefore must disclose its reduction.
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

/// Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach for a component's non-visual
/// path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only surface that traps keyboard / assistive-tech / headless-CLI users
    /// (red).
    ViewOnlyTrap,
}

impl NotebookComponentNonVisualReachState {
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

/// Whether an export-safe summary preserves the component meaning without leaking a raw payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookComponentExportSummaryState {
    /// The component meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl NotebookComponentExportSummaryState {
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

/// How a dense output is presented when it is too large to render in full. Large-output
/// virtualization and collapsed-output summaries must keep the run identity, output trust class, and
/// stale-versus-live truth attached to the truncated view — a virtualized or collapsed output is
/// never allowed to degrade into an anonymous, trust-less blob.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookOutputVirtualizationState {
    /// The output is small enough to render in full; no virtualization is applied.
    NotVirtualized,
    /// A large output is windowed / virtualized while keeping run identity, trust class, and
    /// stale-versus-live truth attached to the visible window (yellow).
    VirtualizedAttributed,
    /// A large output is collapsed to a summary that keeps run identity, trust class, and
    /// stale-versus-live truth (yellow).
    CollapsedSummarized,
    /// A truncated / virtualized output that lost its run identity or trust label and became an
    /// anonymous blob (red).
    AnonymousBlob,
}

impl M5NotebookOutputVirtualizationState {
    /// Every virtualization state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::NotVirtualized,
        Self::VirtualizedAttributed,
        Self::CollapsedSummarized,
        Self::AnonymousBlob,
    ];

    /// Returns true when the state keeps the output attributable and trust-labeled rather than an
    /// anonymous blob.
    pub const fn stays_attributable(self) -> bool {
        !matches!(self, Self::AnonymousBlob)
    }

    /// Returns true when a large output is being truncated (virtualized or collapsed) and therefore
    /// carries a disclosed reduction.
    pub const fn is_truncating(self) -> bool {
        matches!(
            self,
            Self::VirtualizedAttributed | Self::CollapsedSummarized
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotVirtualized => "not_virtualized",
            Self::VirtualizedAttributed => "virtualized_attributed",
            Self::CollapsedSummarized => "collapsed_summarized",
            Self::AnonymousBlob => "anonymous_blob",
        }
    }
}

/// A large-output virtualization / collapsed-output disclosure: whether the truncated view keeps the
/// run identity, output trust class, and stale-versus-live truth of the full output.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookOutputVirtualizationDisclosure {
    /// How the dense output is presented.
    pub state: M5NotebookOutputVirtualizationState,
    /// The run / execution identity stays attached to the virtualized or collapsed view.
    pub preserves_run_identity: bool,
    /// The output trust class stays attached to the virtualized or collapsed view.
    pub preserves_trust_class: bool,
    /// The stale-versus-live truth stays attached to the virtualized or collapsed view.
    pub preserves_stale_live_truth: bool,
}

impl NotebookOutputVirtualizationDisclosure {
    /// A non-virtualized full output; nothing is truncated.
    pub fn full() -> Self {
        Self {
            state: M5NotebookOutputVirtualizationState::NotVirtualized,
            preserves_run_identity: true,
            preserves_trust_class: true,
            preserves_stale_live_truth: true,
        }
    }

    /// Whether the truncated view stays attributable and trust-labeled. A non-virtualized output is
    /// trivially attributable; a virtualized or collapsed output must keep the run identity, trust
    /// class, and stale-versus-live truth; an anonymous blob never qualifies.
    pub fn stays_attributable_and_trust_labeled(&self) -> bool {
        self.state.stays_attributable()
            && (matches!(
                self.state,
                M5NotebookOutputVirtualizationState::NotVirtualized
            ) || (self.preserves_run_identity
                && self.preserves_trust_class
                && self.preserves_stale_live_truth))
    }

    /// Whether this disclosure reflects a truncated (virtualized or collapsed) output.
    pub const fn is_truncating(&self) -> bool {
        self.state.is_truncating()
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl NotebookComponentNarrowingDisclosureState {
    /// Returns true when the surface never silently drops state or actions.
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

/// The notebook claim ceiling a component asserts: how strong a live / trusted / provenanced posture
/// it lets a surface present. Auto-narrowing lowers this ceiling when a notebook dimension weakens so
/// a partial kernel parity, an unsupported debugger, a degraded kernel origin, a stale output, a
/// severed environment provenance, or a kernel-free strip can never keep an old `LiveTrustedResult`
/// or `ReviewableResult` label — a stale output never masquerades as live truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookComponentClaim {
    /// Live trusted result: a fully identified, live-kerneled, trusted, provenanced result — the
    /// strongest claim, a surface Aureline can present as exactly true right now.
    LiveTrustedResult,
    /// Reviewable result: a self-sufficient, reviewable read-only notebook / output view (a result a
    /// user can review) that is not itself a certified live-trusted path.
    ReviewableResult,
    /// Partial-kernel-parity projection: the selected kernel's parity is only partially resolved;
    /// the surface stays a partial-kernel-parity projection that names the resolved axes, never a
    /// fully live-trusted result.
    PartialKernelParityProjection,
    /// Debugger-unsupported projection: a debugger is unsupported on this kernel / profile; the
    /// surface stays a debugger-unsupported projection with its supported affordances preserved,
    /// never a fully-debuggable live result.
    DebuggerUnsupportedProjection,
    /// Degraded-origin projection: the kernel origin is degraded (unstated or approximate); the
    /// surface stays a degraded-origin projection with its last-known origin preserved, never an
    /// exact-origin live result.
    DegradedOriginProjection,
    /// Stale-output projection: an output's trust evidence is stale; the surface stays a
    /// stale-output projection with its last-known freshness preserved, never a live output.
    StaleOutputProjection,
    /// Unprovenanced-environment projection: the environment provenance is severed; the surface
    /// stays an unprovenanced-environment projection with its last-known environment preserved,
    /// never a fully-provenanced environment.
    UnprovenancedEnvironmentProjection,
    /// No-kernel projection: no kernel is available; the surface stays a kernel-free, edit-only
    /// no-kernel projection, never a live-executing result.
    NoKernelProjection,
}

impl M5NotebookComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::LiveTrustedResult,
        Self::ReviewableResult,
        Self::PartialKernelParityProjection,
        Self::DebuggerUnsupportedProjection,
        Self::DegradedOriginProjection,
        Self::StaleOutputProjection,
        Self::UnprovenancedEnvironmentProjection,
        Self::NoKernelProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::LiveTrustedResult => 7,
            Self::ReviewableResult => 6,
            Self::PartialKernelParityProjection => 5,
            Self::DebuggerUnsupportedProjection => 4,
            Self::DegradedOriginProjection => 3,
            Self::StaleOutputProjection => 2,
            Self::UnprovenancedEnvironmentProjection => 1,
            Self::NoKernelProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully live, trusted result.
    pub const fn asserts_live_trusted_result(self) -> bool {
        matches!(self, Self::LiveTrustedResult)
    }

    /// Returns true when this claim asserts a fully self-sufficient (live-trusted or reviewable)
    /// result.
    pub const fn asserts_trustworthy_result(self) -> bool {
        matches!(self, Self::LiveTrustedResult | Self::ReviewableResult)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTrustedResult => "live_trusted_result",
            Self::ReviewableResult => "reviewable_result",
            Self::PartialKernelParityProjection => "partial_kernel_parity_projection",
            Self::DebuggerUnsupportedProjection => "debugger_unsupported_projection",
            Self::DegradedOriginProjection => "degraded_origin_projection",
            Self::StaleOutputProjection => "stale_output_projection",
            Self::UnprovenancedEnvironmentProjection => "unprovenanced_environment_projection",
            Self::NoKernelProjection => "no_kernel_projection",
        }
    }
}

/// The notebook dimension whose state governs how far a component may claim to be a live, trusted
/// result. The dimensions map 1:1 to the eight frozen component families so every family carries an
/// honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookComponentClaimDimension {
    /// Document identity: is the canonical .ipynb identity fully stated?
    DocumentIdentity,
    /// Kernel liveness: is a kernel live and executing, or absent?
    KernelLiveness,
    /// Kernel parity: is the selected kernel's parity fully resolved, or partial?
    KernelParity,
    /// Kernel-origin provenance: is the kernel origin exact, or degraded?
    KernelOriginProvenance,
    /// Output trust evidence: is the output's trust evidence live, or stale?
    OutputTrustEvidence,
    /// Output provenance: is the output's provenance lineage intact, or severed?
    OutputProvenance,
    /// Restart-consequence clarity: is the restart / debugger consequence fully stated?
    RestartConsequenceClarity,
    /// Recovery continuity: is the kernel recovery continuity intact, or does it require a fresh
    /// environment?
    RecoveryContinuity,
}

impl M5NotebookComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::DocumentIdentity,
        Self::KernelLiveness,
        Self::KernelParity,
        Self::KernelOriginProvenance,
        Self::OutputTrustEvidence,
        Self::OutputProvenance,
        Self::RestartConsequenceClarity,
        Self::RecoveryContinuity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DocumentIdentity => "document_identity",
            Self::KernelLiveness => "kernel_liveness",
            Self::KernelParity => "kernel_parity",
            Self::KernelOriginProvenance => "kernel_origin_provenance",
            Self::OutputTrustEvidence => "output_trust_evidence",
            Self::OutputProvenance => "output_provenance",
            Self::RestartConsequenceClarity => "restart_consequence_clarity",
            Self::RecoveryContinuity => "recovery_continuity",
        }
    }
}

/// The observed condition of one notebook dimension. Anything weaker than
/// [`Self::LiveTrusted`] imposes a narrowing ceiling on the component's claim. The
/// stale / partial / degraded / severed states the lane must auto-narrow on as *weakened evidence*
/// — a partial kernel parity, a degraded kernel origin, a stale output, and a severed environment
/// provenance — are the states that [`Self::cannot_be_shown_live_trusted`] flags. An unsupported
/// debugger and a kernel-free strip are honest capability / offline operations, not truth
/// overstatements, so they are deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5NotebookComponentConditionState {
    /// Fully identified, live-kerneled, trusted, provenanced — imposes no ceiling.
    LiveTrusted,
    /// The selected kernel's parity is only partially resolved — claim drops to a
    /// partial-kernel-parity projection.
    KernelParityPartial,
    /// A debugger is unsupported on this kernel / profile — claim drops to a debugger-unsupported
    /// projection.
    DebuggerUnsupported,
    /// The kernel origin is degraded (unstated or approximate) — claim drops to a degraded-origin
    /// projection.
    KernelOriginDegraded,
    /// An output's trust evidence is stale — claim drops to a stale-output projection.
    OutputTrustStale,
    /// The environment provenance is severed — claim drops to an unprovenanced-environment
    /// projection.
    EnvironmentProvenanceSevered,
    /// No kernel is available — claim drops to a kernel-free no-kernel projection.
    KernelUnavailable,
}

impl M5NotebookComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::LiveTrusted,
        Self::KernelParityPartial,
        Self::DebuggerUnsupported,
        Self::KernelOriginDegraded,
        Self::OutputTrustStale,
        Self::EnvironmentProvenanceSevered,
        Self::KernelUnavailable,
    ];

    /// Returns true when the dimension is weaker than live-trusted and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::LiveTrusted)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully
    /// live, trusted result and must never be shown as such. An unsupported debugger and a
    /// kernel-free strip are honest capability / offline operations, not truth overstatements, so
    /// they are deliberately excluded here.
    pub const fn cannot_be_shown_live_trusted(self) -> bool {
        matches!(
            self,
            Self::KernelParityPartial
                | Self::KernelOriginDegraded
                | Self::OutputTrustStale
                | Self::EnvironmentProvenanceSevered
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5NotebookComponentClaim {
        match self {
            Self::LiveTrusted => M5NotebookComponentClaim::LiveTrustedResult,
            Self::KernelParityPartial => M5NotebookComponentClaim::PartialKernelParityProjection,
            Self::DebuggerUnsupported => M5NotebookComponentClaim::DebuggerUnsupportedProjection,
            Self::KernelOriginDegraded => M5NotebookComponentClaim::DegradedOriginProjection,
            Self::OutputTrustStale => M5NotebookComponentClaim::StaleOutputProjection,
            Self::EnvironmentProvenanceSevered => {
                M5NotebookComponentClaim::UnprovenancedEnvironmentProjection
            }
            Self::KernelUnavailable => M5NotebookComponentClaim::NoKernelProjection,
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each
    /// state maps to the on-topic frozen trigger the freeze matrix already governs, so the certified
    /// reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5NotebookKernelOutputDowngradeTrigger {
        match self {
            // The live baseline never narrows; kept for exhaustiveness.
            Self::LiveTrusted => M5NotebookKernelOutputDowngradeTrigger::ProofStale,
            Self::KernelParityPartial => {
                M5NotebookKernelOutputDowngradeTrigger::KernelClassCollapsed
            }
            Self::DebuggerUnsupported => M5NotebookKernelOutputDowngradeTrigger::ProofStale,
            Self::KernelOriginDegraded => {
                M5NotebookKernelOutputDowngradeTrigger::KernelOriginUnstated
            }
            Self::OutputTrustStale => {
                M5NotebookKernelOutputDowngradeTrigger::StaleOutputShownAsLive
            }
            Self::EnvironmentProvenanceSevered => {
                M5NotebookKernelOutputDowngradeTrigger::ProvenanceSevered
            }
            Self::KernelUnavailable => {
                M5NotebookKernelOutputDowngradeTrigger::ReconnectShownAsFresh
            }
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTrusted => "live_trusted",
            Self::KernelParityPartial => "kernel_parity_partial",
            Self::DebuggerUnsupported => "debugger_unsupported",
            Self::KernelOriginDegraded => "kernel_origin_degraded",
            Self::OutputTrustStale => "output_trust_stale",
            Self::EnvironmentProvenanceSevered => "environment_provenance_severed",
            Self::KernelUnavailable => "kernel_unavailable",
        }
    }
}

/// One notebook dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5NotebookComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5NotebookComponentConditionState,
}

/// An honest claim auto-narrow block. When a notebook dimension weakens, the component's claim
/// lowers to the permitted ceiling, names the binding dimension and frozen trigger, and preserves
/// the canonical document identity / kernel origin / output provenance rather than silently dropping
/// it — the underlying notebook truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookComponentClaimAutoNarrow {
    /// The claim the component is narrowed to.
    pub narrowed_to: M5NotebookComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5NotebookComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5NotebookKernelOutputDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical document identity, kernel origin, output provenance, and export scope are
    /// preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
    /// The underlying document / kernel / output truth is preserved (never dropped) across the
    /// narrowing; must hold so partial-parity, debugger-unsupported, degraded-origin, stale-output,
    /// unprovenanced-environment, and no-kernel states never fail opaquely.
    pub preserves_truth_continuity: bool,
}

impl NotebookComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and notebook truth
    /// and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl NotebookComponentCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at
    /// least one export field is named, and a raw-payload-only export is prohibited.
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
pub struct NotebookComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5NotebookComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: NotebookComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a notebook-component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NotebookComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / reduced-motion / CLI / export parity with no
    /// narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims live truth, drops an anonymous
    /// output blob, or drops state silently (red).
    Stranded,
}

impl NotebookComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one notebook-component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookComponentAccessibilityRow {
    /// Record kind; must equal [`NOTEBOOK_KERNEL_OUTPUT_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5NotebookKernelOutputComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the notebook / kernel / output object this component represents; stays visible
    /// on every surface, so this is never empty.
    pub notebook_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual (list /
    /// textual / CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5NotebookComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical document identity, kernel origin /
    /// class, output trust / freshness, provenance, and restart / recovery posture as the rich
    /// surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: NotebookComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: NotebookComponentNonVisualReachState,
    /// High-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: NotebookComponentNonVisualReachState,
    /// Reduced-motion behavior of the non-visual path.
    pub reduced_motion_reach: NotebookComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: NotebookComponentNonVisualReachState,
    /// Large-output virtualization / collapsed-output disclosure: whether a dense output stays
    /// attributable and trust-labeled when truncated.
    pub output_virtualization: NotebookOutputVirtualizationDisclosure,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: NotebookComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: NotebookComponentCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_notebook_claim: M5NotebookComponentClaim,
    /// The observed condition of each modeled notebook dimension.
    #[serde(default)]
    pub claim_conditions: Vec<NotebookComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's
    /// full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<NotebookComponentClaimAutoNarrow>,
    /// Whether the underlying notebook truth is preserved on this component regardless of narrowing;
    /// must hold so partial-parity, debugger-unsupported, degraded-origin, stale-output,
    /// unprovenanced-environment, and no-kernel states never fail opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5NotebookComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<NotebookComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5NotebookKernelOutputRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5NotebookKernelOutputConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl NotebookComponentAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat
    /// non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is
    /// offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `LiveTrusted` when the row does not model
    /// that dimension.
    pub fn condition_for(
        &self,
        dimension: M5NotebookComponentClaimDimension,
    ) -> M5NotebookComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5NotebookComponentConditionState::LiveTrusted)
    }

    /// Whether any modeled dimension is weaker than live-trusted.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// family's full claim.
    pub fn permitted_claim(&self) -> M5NotebookComponentClaim {
        let mut permitted = self.full_notebook_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_condition(&self) -> Option<&NotebookComponentClaimConditionEntry> {
        let mut binding: Option<(&NotebookComponentClaimConditionEntry, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_notebook_claim.capability_rank() {
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
    pub fn binding_dimension(&self) -> Option<M5NotebookComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5NotebookComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_notebook_claim,
        }
    }

    /// AC / auto-narrowing honesty: a partial kernel parity, an unsupported debugger, a degraded
    /// kernel origin, a stale output, a severed environment provenance, or a kernel-free strip can
    /// no longer keep an old `LiveTrustedResult` / `ReviewableResult` label. The effective claim
    /// never exceeds the permitted ceiling; when a dimension narrows below the full claim, an honest
    /// narrow block is present, narrows to exactly the permitted ceiling, binds to the
    /// ceiling-imposing dimension with its frozen trigger, and preserves canonical identity and
    /// truth. When nothing narrows, no spurious narrow block is present.
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

    /// AC / stale-output honesty: a partial-parity / degraded-origin / stale-output /
    /// severed-provenance state never keeps a live-trusted claim — a stale output never masquerades
    /// as live truth. When such a state is modeled, the effective claim must not assert
    /// `LiveTrustedResult`.
    pub fn live_truth_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_live_trusted());
        !(has_unprovable_state && self.effective_claim().asserts_live_trusted_result())
    }

    /// AC1 / virtualization honesty: a dense output that is virtualized or collapsed stays
    /// attributable and trust-labeled rather than becoming an anonymous blob.
    pub fn virtualized_output_stays_attributable(&self) -> bool {
        self.output_virtualization
            .stays_attributable_and_trust_labeled()
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth —
    /// no keyboard / screen-reader / high-zoom / reduced-motion / CLI trap, a hierarchy-heavy family
    /// offers a non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.notebook_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.reduced_motion_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: partial-parity, debugger-unsupported, degraded-origin, stale-output,
    /// unprovenanced-environment, and no-kernel states preserve the underlying notebook truth. The
    /// row must assert `truth_preserved`, and any narrow block must preserve truth continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state, the output is virtualized /
    /// collapsed, or the component carries an honest claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.output_virtualization.is_truncating()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.high_zoom_reach.is_disclosed_reduction()
            || self.reduced_motion_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so product / docs / release publication stay aligned on
    /// the same narrowed state.
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
        let primary = family_primary_dimension(self.component_family);
        self.claim_conditions.iter().any(|c| c.dimension == primary)
    }

    /// Whether every mandatory required label is preserved on the accessible fallback.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5NotebookKernelOutputRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> NotebookComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.live_truth_honesty_holds()
            || !self.virtualized_output_stays_attributable()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return NotebookComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            NotebookComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            NotebookComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == NOTEBOOK_KERNEL_OUTPUT_A11Y_ROW_RECORD_KIND
            && self.schema_version == NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.notebook_context_ref.trim().is_empty()
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
high_zoom={high_zoom} reduced_motion={reduced_motion} cli={cli} virtualization={virtualization} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            reduced_motion = self.reduced_motion_reach.as_str(),
            cli = self.cli_reach.as_str(),
            virtualization = self.output_virtualization.state.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_notebook_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1089 notebook-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookComponentAccessibilitySummary {
    pub row_count: usize,
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_live_truth_honesty_holds: bool,
    pub all_virtualized_outputs_attributable: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_truth_preserved: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`NotebookComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotebookComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<NotebookComponentAccessibilityRow>,
}

/// Checked-in M05-1089 notebook-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NotebookComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<NotebookComponentAccessibilityRow>,
    pub summary: NotebookComponentAccessibilitySummary,
}

impl NotebookComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: NotebookComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION,
            record_kind: NOTEBOOK_KERNEL_OUTPUT_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: NotebookComponentAccessibilitySummary {
                row_count: 0,
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_live_truth_honesty_holds: false,
                all_virtualized_outputs_attributable: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5NotebookKernelOutputComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5NotebookComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5NotebookComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5NotebookComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Virtualization states exercised by some row across the packet.
    pub fn exercised_virtualization_states(&self) -> BTreeSet<M5NotebookOutputVirtualizationState> {
        self.rows
            .iter()
            .map(|r| r.output_virtualization.state)
            .collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5NotebookKernelOutputConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> NotebookComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5NotebookKernelOutputConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&NotebookComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                NotebookComponentAccessibilityStatus::Parity => green += 1,
                NotebookComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                NotebookComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        NotebookComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(NotebookComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(NotebookComponentAccessibilityRow::claim_is_honest),
            all_live_truth_honesty_holds: self
                .rows
                .iter()
                .all(NotebookComponentAccessibilityRow::live_truth_honesty_holds),
            all_virtualized_outputs_attributable: self
                .rows
                .iter()
                .all(NotebookComponentAccessibilityRow::virtualized_output_stays_attributable),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(NotebookComponentAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(NotebookComponentAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(NotebookComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<NotebookComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION {
            violations.push(NotebookComponentAccessibilityViolation::SchemaVersion {
                expected: NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != NOTEBOOK_KERNEL_OUTPUT_A11Y_RECORD_KIND {
            violations.push(NotebookComponentAccessibilityViolation::RecordKind {
                expected: NOTEBOOK_KERNEL_OUTPUT_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(NotebookComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        let mut has_virtualized_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(NotebookComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_live_trusted())
            {
                has_unprovable_row = true;
            }
            if row.output_virtualization.is_truncating() {
                has_virtualized_row = true;
            }

            if !row.is_complete() {
                violations.push(NotebookComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    NotebookComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory notebook label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    NotebookComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured tree *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5NotebookComponentFallbackModality::Structured)
            {
                violations.push(
                    NotebookComponentAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC3: claim never over-asserts a live / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(NotebookComponentAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC / stale-output honesty: a partial-parity / degraded-origin / stale-output /
            // severed-provenance state never keeps a live-trusted claim.
            if !row.live_truth_honesty_holds() {
                violations.push(
                    NotebookComponentAccessibilityViolation::StaleStateShownAsLive {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: a dense virtualized / collapsed output stays attributable and trust-labeled.
            if !row.virtualized_output_stays_attributable() {
                violations.push(
                    NotebookComponentAccessibilityViolation::VirtualizedOutputAnonymous {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    NotebookComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC2: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    NotebookComponentAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: partial-parity, debugger-unsupported, degraded-origin, stale-output,
            // unprovenanced-environment, and no-kernel states preserve notebook truth.
            if !row.preserves_truth_continuity() {
                violations.push(NotebookComponentAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    NotebookComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    NotebookComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == NotebookComponentAccessibilityStatus::Stranded {
                violations.push(NotebookComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5NotebookKernelOutputComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    NotebookComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5NotebookComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    NotebookComponentAccessibilityViolation::MissingDimensionCoverage { dimension },
                );
            }
        }

        // Coverage: every condition state (the live baseline plus each spec narrowing axis) is
        // exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5NotebookComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    NotebookComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (live-trusted → … → no-kernel) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5NotebookComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    NotebookComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Live-truth honesty must be proven with at least one partial-parity / degraded-origin /
        // stale-output / severed-provenance row in the packet, so the "cannot-prove never shown as
        // live" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(NotebookComponentAccessibilityViolation::LiveTruthHonestyUnproven);
        }

        // Virtualization honesty must be proven with at least one virtualized / collapsed
        // large-output row, so AC1 is exercised end-to-end.
        if !has_virtualized_row {
            violations.push(NotebookComponentAccessibilityViolation::VirtualizationHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the notebook, kernel-manager,
        // output-viewer, debugger, AI-context, review, CLI, support-export, and product surfaces —
        // so every consumer surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5NotebookKernelOutputConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    NotebookComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(NotebookComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("notebook-component accessibility parity packet serializes"),
        ) {
            violations.push(NotebookComponentAccessibilityViolation::RawNotebookMaterialInExport);
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
            .expect("notebook-component accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,high_zoom_reach,reduced_motion_reach,cli_reach,virtualization,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{high_zoom},{reduced_motion},{cli},{virtualization},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                reduced_motion = row.reduced_motion_reach.as_str(),
                cli = row.cli_reach.as_str(),
                virtualization = row.output_virtualization.state.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_notebook_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Notebook-Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5NotebookKernelOutputComponentFamily::ALL.len(),
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
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.claim_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: {} → {} (dimension={}, trigger={}) — {}\n",
                    row.full_notebook_claim.as_str(),
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

/// Reads and validates the checked-in notebook-component accessibility parity export.
pub fn current_m5_notebook_kernel_output_component_a11y_export(
) -> Result<NotebookComponentAccessibilityPacket, NotebookComponentAccessibilityArtifactError> {
    let packet: NotebookComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-notebook-kernel-output-component-accessibility-parity/support_export.json"
    )))
    .map_err(NotebookComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(NotebookComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in notebook-component accessibility parity export.
#[derive(Debug)]
pub enum NotebookComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<NotebookComponentAccessibilityViolation>),
}

impl fmt::Display for NotebookComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "notebook-component accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "notebook-component accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for NotebookComponentAccessibilityArtifactError {}

/// Validation failure for M05-1089 notebook-component accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotebookComponentAccessibilityViolation {
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
        dimension: M5NotebookComponentClaimDimension,
    },
    MissingMandatoryLabel {
        id: String,
    },
    HierarchyHeavyMissingStructured {
        id: String,
    },
    ClaimOverAsserted {
        id: String,
    },
    StaleStateShownAsLive {
        id: String,
    },
    VirtualizedOutputAnonymous {
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
        family: M5NotebookKernelOutputComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5NotebookComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5NotebookComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5NotebookComponentClaim,
    },
    LiveTruthHonestyUnproven,
    VirtualizationHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5NotebookKernelOutputConsumerSurface,
    },
    SummaryMismatch,
    RawNotebookMaterialInExport,
}

impl NotebookComponentAccessibilityViolation {
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
            Self::HierarchyHeavyMissingStructured { .. } => "hierarchy_heavy_missing_structured",
            Self::ClaimOverAsserted { .. } => "claim_over_asserted",
            Self::StaleStateShownAsLive { .. } => "stale_state_shown_as_live",
            Self::VirtualizedOutputAnonymous { .. } => "virtualized_output_anonymous",
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
            Self::LiveTruthHonestyUnproven => "live_truth_honesty_unproven",
            Self::VirtualizationHonestyUnproven => "virtualization_honesty_unproven",
            Self::MissingConsumerSurfaceCoverage { .. } => "missing_consumer_surface_coverage",
            Self::SummaryMismatch => "summary_mismatch",
            Self::RawNotebookMaterialInExport => "raw_notebook_material_in_export",
        }
    }
}

impl fmt::Display for NotebookComponentAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory notebook label")
            }
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts a live / reviewable surface for a weakened one, or narrows spuriously"
                )
            }
            Self::StaleStateShownAsLive { id } => {
                write!(
                    f,
                    "row {id} shows a partial-parity / degraded-origin / stale-output / severed-provenance state as a live trusted result"
                )
            }
            Self::VirtualizedOutputAnonymous { id } => {
                write!(
                    f,
                    "row {id} virtualizes / collapses a dense output into an anonymous, trust-less blob"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / high-zoom / reduced-motion / CLI users from the canonical truth"
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
                    "row {id} does not preserve notebook truth across narrowing"
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
                    "component family {family:?} is not certified in the packet"
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
            Self::LiveTruthHonestyUnproven => {
                write!(
                    f,
                    "no partial-parity / degraded-origin / stale-output / severed-provenance row is present to prove the live-truth-honesty guarantee"
                )
            }
            Self::VirtualizationHonestyUnproven => {
                write!(
                    f,
                    "no virtualized / collapsed large-output row is present to prove the virtualization-honesty guarantee"
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
            Self::RawNotebookMaterialInExport => {
                write!(f, "export contains raw notebook material")
            }
        }
    }
}

impl Error for NotebookComponentAccessibilityViolation {}

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
            | "no kernel"
            | "offline"
            | "disconnected"
            | "severed"
            | "unprovenanced"
            | "reconnect"
            | "restart"
    )
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("api_key")
                || lower.contains("password")
                || lower.contains("secret")
                || lower.contains("-----begin")
                || lower.contains("bearer ")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_material),
        serde_json::Value::Object(map) => map.values().any(json_contains_forbidden_material),
        _ => false,
    }
}

/// Builds the canonical, checked-in notebook-component accessibility parity packet. This is the one
/// source of truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_notebook_kernel_output_component_a11y_packet(
) -> NotebookComponentAccessibilityPacket {
    NotebookComponentAccessibilityPacket::new(NotebookComponentAccessibilityPacketInput {
        packet_id: "m5-notebook-kernel-output-component-accessibility-parity:stable:0001"
            .to_owned(),
        as_of: "2026-07-11T00:00:00Z".to_owned(),
        matrix_ref: NOTEBOOK_KERNEL_OUTPUT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:notebook-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5NotebookKernelOutputRequiredLabel> {
    M5NotebookKernelOutputRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> NotebookComponentCopyExportParity {
    NotebookComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5NotebookComponentClaimDimension,
    state: M5NotebookComponentConditionState,
) -> NotebookComponentClaimConditionEntry {
    NotebookComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the CLI
/// surface — so the narrowed state always reaches headless field triage.
fn base_consumers(
    extra: &[M5NotebookKernelOutputConsumerSurface],
) -> Vec<M5NotebookKernelOutputConsumerSurface> {
    let mut out = vec![
        M5NotebookKernelOutputConsumerSurface::SupportExport,
        M5NotebookKernelOutputConsumerSurface::CliSurface,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: NotebookComponentNarrowingDisclosureState,
) -> Vec<NotebookComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        NotebookComponentRenderingNarrowingDisclosure {
            rendering_surface: M5NotebookComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        NotebookComponentRenderingNarrowingDisclosure {
            rendering_surface: M5NotebookComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_animated_overlay".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary
/// parity.
fn parity_surfaces(labels: &[&str]) -> Vec<NotebookComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        NotebookComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<NotebookComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        NotebookComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5NotebookComponentRenderingSurface> {
    vec![
        M5NotebookComponentRenderingSurface::DesktopFull,
        M5NotebookComponentRenderingSurface::CliHeadless,
        M5NotebookComponentRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5NotebookComponentFallbackModality> {
    vec![
        M5NotebookComponentFallbackModality::List,
        M5NotebookComponentFallbackModality::Textual,
        M5NotebookComponentFallbackModality::Cli,
    ]
}

/// A large output collapsed to an attributed summary that keeps run identity, trust class, and
/// stale-versus-live truth.
fn collapsed_attributed() -> NotebookOutputVirtualizationDisclosure {
    NotebookOutputVirtualizationDisclosure {
        state: M5NotebookOutputVirtualizationState::CollapsedSummarized,
        preserves_run_identity: true,
        preserves_trust_class: true,
        preserves_stale_live_truth: true,
    }
}

/// A large output virtualized (windowed) while keeping run identity, trust class, and
/// stale-versus-live truth attached to the visible window.
fn virtualized_attributed() -> NotebookOutputVirtualizationDisclosure {
    NotebookOutputVirtualizationDisclosure {
        state: M5NotebookOutputVirtualizationState::VirtualizedAttributed,
        preserves_run_identity: true,
        preserves_trust_class: true,
        preserves_stale_live_truth: true,
    }
}

fn seeded_rows() -> Vec<NotebookComponentAccessibilityRow> {
    vec![
        // Notebook document header (canonical .ipynb identity) — the document identity is fully
        // stated, so it is a live trusted result and reachable on every surface (green).
        NotebookComponentAccessibilityRow {
            record_kind: NOTEBOOK_KERNEL_OUTPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:notebook-document-header-live".to_owned(),
            component_family: M5NotebookKernelOutputComponentFamily::NotebookDocumentHeader,
            source_family_schema_ref: NOTEBOOK_KERNEL_OUTPUT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            notebook_context_ref: "notebook:notebook-document-header:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            output_virtualization: NotebookOutputVirtualizationDisclosure::full(),
            export_summary:
                NotebookComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:notebook-document-header-live:a11y".to_owned(),
            copy_export: copy_export(&[
                "document_identity",
                "canonical_ipynb_source",
                "selected_kernel_origin",
                "keyboard_route",
            ]),
            full_notebook_claim: M5NotebookComponentClaim::LiveTrustedResult,
            claim_conditions: vec![condition(
                M5NotebookComponentClaimDimension::DocumentIdentity,
                M5NotebookComponentConditionState::LiveTrusted,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "document_identity",
                "canonical_ipynb_source",
                "selected_kernel_origin",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NotebookKernelOutputConsumerSurface::NotebookUi,
                M5NotebookKernelOutputConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 notebook document header".to_owned(),
                NOTEBOOK_KERNEL_OUTPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("notebook-document-header-live"),
        },
        // Output provenance chip group (complete lineage) — hierarchy-heavy (nested source /
        // transform / derived-output lineage chips); the provenance is intact, so it is a reviewable
        // result that binds its nested chip group to a flat list / textual path and keeps its dense
        // lineage attributable via a collapsed-output summary (green).
        NotebookComponentAccessibilityRow {
            record_kind: NOTEBOOK_KERNEL_OUTPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:output-provenance-chip-group-complete".to_owned(),
            component_family: M5NotebookKernelOutputComponentFamily::OutputProvenanceChipGroup,
            source_family_schema_ref: NOTEBOOK_KERNEL_OUTPUT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            notebook_context_ref: "notebook:output-provenance-chip-group:0002".to_owned(),
            fallback_modalities: vec![
                M5NotebookComponentFallbackModality::Structured,
                M5NotebookComponentFallbackModality::List,
                M5NotebookComponentFallbackModality::Textual,
                M5NotebookComponentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach:
                NotebookComponentNonVisualReachState::DisclosedReducedButReachable,
            high_zoom_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            output_virtualization: collapsed_attributed(),
            export_summary:
                NotebookComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:output-provenance-chip-group-complete:a11y".to_owned(),
            copy_export: copy_export(&[
                "provenance_identity",
                "output_origin_class",
                "lineage_state",
                "collapsed_summary_note",
            ]),
            full_notebook_claim: M5NotebookComponentClaim::ReviewableResult,
            claim_conditions: vec![condition(
                M5NotebookComponentClaimDimension::OutputProvenance,
                M5NotebookComponentConditionState::LiveTrusted,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "provenance_identity",
                "output_origin_class",
                "lineage_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NotebookKernelOutputConsumerSurface::OutputViewerUi,
                M5NotebookKernelOutputConsumerSurface::ReviewUi,
            ]),
            source_refs: vec![
                "UI/UX Design System output provenance chips".to_owned(),
                NOTEBOOK_KERNEL_OUTPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("output-provenance-chip-group-complete"),
        },
        // Kernel state strip (no kernel) — no kernel is available, so the strip auto-narrows to a
        // kernel-free, edit-only no-kernel projection that keeps the document editable and its
        // identity visible, never implying live execution (yellow).
        NotebookComponentAccessibilityRow {
            record_kind: NOTEBOOK_KERNEL_OUTPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:kernel-state-strip-no-kernel".to_owned(),
            component_family: M5NotebookKernelOutputComponentFamily::KernelStateStrip,
            source_family_schema_ref: NOTEBOOK_KERNEL_OUTPUT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            notebook_context_ref: "notebook:kernel-state-strip:0003".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            output_virtualization: NotebookOutputVirtualizationDisclosure::full(),
            export_summary:
                NotebookComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:kernel-state-strip-no-kernel:a11y".to_owned(),
            copy_export: copy_export(&[
                "strip_identity",
                "kernel_liveness_state",
                "edit_parity_note",
                "keyboard_route",
            ]),
            full_notebook_claim: M5NotebookComponentClaim::LiveTrustedResult,
            claim_conditions: vec![condition(
                M5NotebookComponentClaimDimension::KernelLiveness,
                M5NotebookComponentConditionState::KernelUnavailable,
            )],
            claim_narrow: Some(NotebookComponentClaimAutoNarrow {
                narrowed_to: M5NotebookComponentClaim::NoKernelProjection,
                binding_dimension: M5NotebookComponentClaimDimension::KernelLiveness,
                trigger: M5NotebookKernelOutputDowngradeTrigger::ReconnectShownAsFresh,
                narrowed_label:
                    "No kernel is attached — shown as a kernel-free, edit-only projection that keeps the document editable and its identity visible, never implying a live-executing result"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "strip_identity",
                "kernel_liveness_state",
                "edit_parity_note",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NotebookKernelOutputConsumerSurface::KernelManagerUi,
                M5NotebookKernelOutputConsumerSurface::NotebookUi,
            ]),
            source_refs: vec![
                "TDD notebook kernel state strip".to_owned(),
                NOTEBOOK_KERNEL_OUTPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("kernel-state-strip-no-kernel"),
        },
        // Kernel picker row (parity partial) — the selected kernel's parity is only partially
        // resolved, so the row auto-narrows to a partial-kernel-parity projection that names the
        // resolved axes, never a fully live-trusted result (yellow).
        NotebookComponentAccessibilityRow {
            record_kind: NOTEBOOK_KERNEL_OUTPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:kernel-picker-row-partial-parity".to_owned(),
            component_family: M5NotebookKernelOutputComponentFamily::KernelPickerRow,
            source_family_schema_ref: NOTEBOOK_KERNEL_OUTPUT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            notebook_context_ref: "notebook:kernel-picker-row:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            output_virtualization: NotebookOutputVirtualizationDisclosure::full(),
            export_summary:
                NotebookComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:kernel-picker-row-partial-parity:a11y".to_owned(),
            copy_export: copy_export(&[
                "candidate_identity",
                "kernel_parity_state",
                "resolved_axes",
                "compatibility_note",
            ]),
            full_notebook_claim: M5NotebookComponentClaim::LiveTrustedResult,
            claim_conditions: vec![condition(
                M5NotebookComponentClaimDimension::KernelParity,
                M5NotebookComponentConditionState::KernelParityPartial,
            )],
            claim_narrow: Some(NotebookComponentClaimAutoNarrow {
                narrowed_to: M5NotebookComponentClaim::PartialKernelParityProjection,
                binding_dimension: M5NotebookComponentClaimDimension::KernelParity,
                trigger: M5NotebookKernelOutputDowngradeTrigger::KernelClassCollapsed,
                narrowed_label:
                    "This kernel's parity is only partially resolved — shown as a partial-kernel-parity projection that names the resolved compatibility axes, never as a fully live-trusted kernel match"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "candidate_identity",
                "kernel_parity_state",
                "resolved_axes",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NotebookKernelOutputConsumerSurface::KernelManagerUi,
                M5NotebookKernelOutputConsumerSurface::ProductUi,
            ]),
            source_refs: vec![
                "TDD kernel picker parity".to_owned(),
                NOTEBOOK_KERNEL_OUTPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("kernel-picker-row-partial-parity"),
        },
        // Kernel origin pill (origin degraded) — the kernel origin is degraded (unstated or
        // approximate), so the pill auto-narrows to a degraded-origin projection that keeps its
        // last-known origin visible, never an exact-origin live result (yellow).
        NotebookComponentAccessibilityRow {
            record_kind: NOTEBOOK_KERNEL_OUTPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:kernel-origin-pill-degraded".to_owned(),
            component_family: M5NotebookKernelOutputComponentFamily::KernelOriginPill,
            source_family_schema_ref: NOTEBOOK_KERNEL_OUTPUT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            notebook_context_ref: "notebook:kernel-origin-pill:0005".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: NotebookComponentNonVisualReachState::DisclosedReducedButReachable,
            reduced_motion_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            output_virtualization: NotebookOutputVirtualizationDisclosure::full(),
            export_summary:
                NotebookComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:kernel-origin-pill-degraded:a11y".to_owned(),
            copy_export: copy_export(&[
                "origin_identity",
                "kernel_origin_class",
                "origin_provenance_state",
                "last_known_origin",
            ]),
            full_notebook_claim: M5NotebookComponentClaim::LiveTrustedResult,
            claim_conditions: vec![condition(
                M5NotebookComponentClaimDimension::KernelOriginProvenance,
                M5NotebookComponentConditionState::KernelOriginDegraded,
            )],
            claim_narrow: Some(NotebookComponentClaimAutoNarrow {
                narrowed_to: M5NotebookComponentClaim::DegradedOriginProjection,
                binding_dimension: M5NotebookComponentClaimDimension::KernelOriginProvenance,
                trigger: M5NotebookKernelOutputDowngradeTrigger::KernelOriginUnstated,
                narrowed_label:
                    "This kernel's origin is degraded — shown as a degraded-origin projection that keeps the last-known origin class and locality visible, never as an exact-origin live result"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "origin_identity",
                "kernel_origin_class",
                "origin_provenance_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NotebookKernelOutputConsumerSurface::KernelManagerUi,
                M5NotebookKernelOutputConsumerSurface::AiContextUi,
            ]),
            source_refs: vec![
                "TAD kernel origin provenance".to_owned(),
                NOTEBOOK_KERNEL_OUTPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("kernel-origin-pill-degraded"),
        },
        // Output trust banner (output stale) — a large output's trust evidence is stale, so the
        // banner auto-narrows to a stale-output projection that keeps its last-known freshness
        // visible, never presenting the stale output as live — and it virtualizes the dense output
        // while keeping run identity and trust class attached (yellow).
        NotebookComponentAccessibilityRow {
            record_kind: NOTEBOOK_KERNEL_OUTPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:output-trust-banner-stale".to_owned(),
            component_family: M5NotebookKernelOutputComponentFamily::OutputTrustBanner,
            source_family_schema_ref: NOTEBOOK_KERNEL_OUTPUT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            notebook_context_ref: "notebook:output-trust-banner:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            output_virtualization: virtualized_attributed(),
            export_summary:
                NotebookComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:output-trust-banner-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "banner_identity",
                "output_trust_class",
                "output_freshness_state",
                "last_known_freshness",
            ]),
            full_notebook_claim: M5NotebookComponentClaim::LiveTrustedResult,
            claim_conditions: vec![condition(
                M5NotebookComponentClaimDimension::OutputTrustEvidence,
                M5NotebookComponentConditionState::OutputTrustStale,
            )],
            claim_narrow: Some(NotebookComponentClaimAutoNarrow {
                narrowed_to: M5NotebookComponentClaim::StaleOutputProjection,
                binding_dimension: M5NotebookComponentClaimDimension::OutputTrustEvidence,
                trigger: M5NotebookKernelOutputDowngradeTrigger::StaleOutputShownAsLive,
                narrowed_label:
                    "This output's trust evidence is stale — shown as a stale-output projection that keeps its trust class and last-known freshness visible, never presenting the stale output as live truth"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "banner_identity",
                "output_trust_class",
                "output_freshness_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NotebookKernelOutputConsumerSurface::OutputViewerUi,
                M5NotebookKernelOutputConsumerSurface::AiContextUi,
            ]),
            source_refs: vec![
                "UI/UX Spec v3.8 output trust banner".to_owned(),
                NOTEBOOK_KERNEL_OUTPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("output-trust-banner-stale"),
        },
        // Restart consequence card (debugger unsupported) — a debugger is unsupported on this
        // kernel / profile, so the card auto-narrows to a debugger-unsupported projection that keeps
        // its supported restart affordances visible, never a fully-debuggable live result (yellow).
        NotebookComponentAccessibilityRow {
            record_kind: NOTEBOOK_KERNEL_OUTPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:restart-consequence-card-debugger-unsupported".to_owned(),
            component_family: M5NotebookKernelOutputComponentFamily::RestartConsequenceCard,
            source_family_schema_ref: NOTEBOOK_KERNEL_OUTPUT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            notebook_context_ref: "notebook:restart-consequence-card:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach:
                NotebookComponentNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            output_virtualization: NotebookOutputVirtualizationDisclosure::full(),
            export_summary:
                NotebookComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:restart-consequence-card-debugger-unsupported:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "card_identity",
                "restart_consequence_state",
                "debugger_support_state",
                "keyboard_route",
            ]),
            full_notebook_claim: M5NotebookComponentClaim::ReviewableResult,
            claim_conditions: vec![condition(
                M5NotebookComponentClaimDimension::RestartConsequenceClarity,
                M5NotebookComponentConditionState::DebuggerUnsupported,
            )],
            claim_narrow: Some(NotebookComponentClaimAutoNarrow {
                narrowed_to: M5NotebookComponentClaim::DebuggerUnsupportedProjection,
                binding_dimension: M5NotebookComponentClaimDimension::RestartConsequenceClarity,
                trigger: M5NotebookKernelOutputDowngradeTrigger::ProofStale,
                narrowed_label:
                    "A debugger is not supported on this kernel — shown as a debugger-unsupported projection that keeps the supported restart consequences and stable route visible, never a fully-debuggable live result"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "card_identity",
                "restart_consequence_state",
                "debugger_support_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NotebookKernelOutputConsumerSurface::DebuggerUi,
                M5NotebookKernelOutputConsumerSurface::ReviewUi,
            ]),
            source_refs: vec![
                "TAD notebook restart / recovery architecture".to_owned(),
                NOTEBOOK_KERNEL_OUTPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("restart-consequence-card-debugger-unsupported"),
        },
        // Kernel recovery card (environment provenance severed) — choosing another kernel lands on
        // an environment whose provenance is severed, so the card auto-narrows to an
        // unprovenanced-environment projection that keeps its last-known environment visible, never
        // a fully-provenanced environment, and never implies a hidden rerun (yellow).
        NotebookComponentAccessibilityRow {
            record_kind: NOTEBOOK_KERNEL_OUTPUT_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: NOTEBOOK_KERNEL_OUTPUT_A11Y_SCHEMA_VERSION,
            row_id: "a11y:kernel-recovery-card-unprovenanced-environment".to_owned(),
            component_family: M5NotebookKernelOutputComponentFamily::KernelRecoveryCard,
            source_family_schema_ref: NOTEBOOK_KERNEL_OUTPUT_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            notebook_context_ref: "notebook:kernel-recovery-card:0008".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            high_zoom_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            reduced_motion_reach: NotebookComponentNonVisualReachState::ReachableAndLabeled,
            cli_reach: NotebookComponentNonVisualReachState::DisclosedReducedButReachable,
            output_virtualization: NotebookOutputVirtualizationDisclosure::full(),
            export_summary:
                NotebookComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:kernel-recovery-card-unprovenanced-environment:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "recovery_identity",
                "recovery_state",
                "environment_provenance_state",
                "no_rerun_note",
            ]),
            full_notebook_claim: M5NotebookComponentClaim::LiveTrustedResult,
            claim_conditions: vec![condition(
                M5NotebookComponentClaimDimension::RecoveryContinuity,
                M5NotebookComponentConditionState::EnvironmentProvenanceSevered,
            )],
            claim_narrow: Some(NotebookComponentClaimAutoNarrow {
                narrowed_to: M5NotebookComponentClaim::UnprovenancedEnvironmentProjection,
                binding_dimension: M5NotebookComponentClaimDimension::RecoveryContinuity,
                trigger: M5NotebookKernelOutputDowngradeTrigger::ProvenanceSevered,
                narrowed_label:
                    "The recovered environment's provenance is severed — shown as an unprovenanced-environment projection that keeps the last-known environment visible and never implies a hidden rerun, never a fully-provenanced live environment"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "recovery_identity",
                "recovery_state",
                "environment_provenance_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5NotebookKernelOutputConsumerSurface::KernelManagerUi,
                M5NotebookKernelOutputConsumerSurface::DebuggerUi,
            ]),
            source_refs: vec![
                "TAD supportability / recovery architecture".to_owned(),
                NOTEBOOK_KERNEL_OUTPUT_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("kernel-recovery-card-unprovenanced-environment"),
        },
    ]
}
