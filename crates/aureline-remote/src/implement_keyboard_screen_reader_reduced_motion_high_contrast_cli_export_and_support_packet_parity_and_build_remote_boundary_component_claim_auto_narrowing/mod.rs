//! Keyboard / screen-reader / reduced-motion / high-contrast / CLI / export /
//! support-packet parity and honest auto-narrowing for the M5 build/remote/managed-workspace
//! boundary components.
//!
//! This module is the M05-1081 accessibility-and-auto-narrowing capstone over the frozen M5
//! build/remote boundary component matrix
//! ([`crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix`]).
//! Where the freeze matrix defines the reusable adapter-confidence chip, discovery-diff card,
//! host-boundary strip, execution-origin receipt row, managed-workspace lifecycle card,
//! suspend/resume/rebuild review sheet, workspace-expiry banner, and local-safe continuation
//! card primitives, and the sibling implementation lanes resolve their per-surface truth, this
//! lane certifies — per component family — that discovery confidence, host ownership, execution
//! origin, lifecycle state, expiry timing, and continuity truth stays **keyboard-complete,
//! screen-reader-reachable, reduced-motion safe, high-contrast legible, CLI/export-safe, and
//! self-narrowing** rather than presenting a stale, unverified, unsupported, or partial boundary
//! state as still fresh first-party `full-truth`:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a keyboard-complete,
//!   screen-reader-reachable, and CLI/headless-reachable path into the same discovery confidence,
//!   host boundary, execution origin, lifecycle state, expiry timing, and continuity the rich
//!   surface shows — never a hover-only or menu-only chrome that strands assistive-tech or
//!   headless users. Hierarchy-heavy families (the review sheet's nested lifecycle / persistence /
//!   continuity / preserved-vs-lost grid) additionally bind their structured layout to a flat
//!   list / textual path.
//! - **Export parity.** The support / release export reconstructs each component's meaning from
//!   typed tokens and opaque refs without a screenshot, preserving the same discovery confidence,
//!   host boundary, execution origin, lifecycle state, expiry timing, and continuity shown
//!   in-product.
//! - **Honest auto-narrowing.** When the discovery confidence, host ownership, execution origin,
//!   lifecycle state, expiry timing, or continuity becomes partial, stale, unverified, or
//!   unsupported on a claimed profile, the component's boundary-support claim auto-narrows from
//!   `full-truth` / `resolved-truth` to degraded / stale / unverified / unsupported, discloses the
//!   narrowing with a precise frozen trigger and binding dimension, and preserves the canonical
//!   target / host / lifecycle / continuity identity rather than silently dropping it or letting a
//!   rebuilt, recreated, or expired workspace read as exact continuity. A component with every
//!   dimension intact must NOT carry a spurious narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the shell, run/test/debug,
//!   notebook, preview, companion, incident/diagnostics, and support/admin exports so claim
//!   publication and field triage stay aligned on build/remote-boundary downgrade behavior.
//!
//! Each [`BuildRemoteBoundaryAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix::M5BuildRemoteBoundaryComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen
//! [`M5BuildRemoteRequiredLabel`] and [`M5BuildRemoteDowngradeTrigger`] and the shared
//! [`M5BuildRemoteConsumerSurface`] consumer surfaces rather than minting parallel synonyms, so
//! the certified labels stay byte-identical to the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw provider tokens, cookies, credentials, and runtime bodies
//! never cross this boundary; the packet carries only typed class tokens, opaque summary /
//! evidence refs, booleans, and redacted labels so support and diagnostics exports can
//! reconstruct exactly what an accessible fallback would have shown without leaking remote state.
//!
//! The boundary schema is
//! [`schemas/ui/m5-build-remote-boundary-component-accessibility-parity.schema.json`](../../../../schemas/ui/m5-build-remote-boundary-component-accessibility-parity.schema.json).
//! The contract doc is
//! [`docs/remote/m5_build_remote_boundary_component_accessibility_parity.md`](../../../../docs/remote/m5_build_remote_boundary_component_accessibility_parity.md).

#[cfg(test)]
mod tests;

use std::collections::{BTreeSet, HashSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families,
// required labels, and downgrade triggers rather than mint parallel ones.
use crate::freeze_the_m5_adapter_confidence_chip_discovery_diff_card_host_boundary_strip_execution_origin_receipt_row_managed_workspace_lifecycle_card_suspend_resume_rebuild_review_sheet_workspace_expiry_banner_and_local_safe_continuation_card_component_matrix::{
    M5BuildRemoteBoundaryComponentFamily, M5BuildRemoteConsumerSurface, M5BuildRemoteDowngradeTrigger,
    M5BuildRemoteRequiredLabel, M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1081 build/remote-boundary-component accessibility parity
/// packet.
pub const BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`BuildRemoteBoundaryAccessibilityPacket`].
pub const BUILD_REMOTE_BOUNDARY_A11Y_RECORD_KIND: &str =
    "m5_build_remote_boundary_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`BuildRemoteBoundaryAccessibilityRow`].
pub const BUILD_REMOTE_BOUNDARY_A11Y_ROW_RECORD_KIND: &str =
    "m5_build_remote_boundary_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-build-remote-boundary-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const BUILD_REMOTE_BOUNDARY_A11Y_DOC_REF: &str =
    "docs/remote/m5_build_remote_boundary_component_accessibility_parity.md";

/// Repo-relative path of the frozen build/remote-boundary component matrix this lane certifies.
pub const BUILD_REMOTE_BOUNDARY_A11Y_COMPONENT_MATRIX_REF: &str =
    M5_BUILD_REMOTE_BOUNDARY_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const BUILD_REMOTE_BOUNDARY_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-build-remote-boundary-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const BUILD_REMOTE_BOUNDARY_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-build-remote-boundary-component-accessibility-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const BUILD_REMOTE_BOUNDARY_A11Y_CSV_REF: &str =
    "artifacts/release/m5-build-remote-boundary-component-accessibility-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const BUILD_REMOTE_BOUNDARY_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-build-remote-boundary-component-accessibility-proof/report.md";

/// The reusable component families that render a non-linear hierarchy (the suspend/resume/rebuild
/// review sheet's nested lifecycle / persistence / continuity / preserved-vs-lost grid) and
/// therefore MUST bind their grid to an equivalent flat list / textual path so the layout is
/// navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5BuildRemoteBoundaryComponentFamily) -> bool {
    matches!(
        family,
        M5BuildRemoteBoundaryComponentFamily::SuspendResumeRebuildReviewSheet
    )
}

/// The build/remote-boundary dimension whose weakening a family primarily discloses. Every row
/// must model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5BuildRemoteBoundaryComponentFamily,
) -> M5BuildRemoteClaimDimension {
    match family {
        M5BuildRemoteBoundaryComponentFamily::AdapterConfidenceChip
        | M5BuildRemoteBoundaryComponentFamily::DiscoveryDiffCard => {
            M5BuildRemoteClaimDimension::DiscoveryConfidenceTruth
        }
        M5BuildRemoteBoundaryComponentFamily::HostBoundaryStrip => {
            M5BuildRemoteClaimDimension::HostOwnershipTruth
        }
        M5BuildRemoteBoundaryComponentFamily::ExecutionOriginReceiptRow => {
            M5BuildRemoteClaimDimension::ExecutionOriginTruth
        }
        M5BuildRemoteBoundaryComponentFamily::ManagedWorkspaceLifecycleCard => {
            M5BuildRemoteClaimDimension::LifecycleStateTruth
        }
        M5BuildRemoteBoundaryComponentFamily::WorkspaceExpiryBanner => {
            M5BuildRemoteClaimDimension::ExpiryTimingTruth
        }
        M5BuildRemoteBoundaryComponentFamily::SuspendResumeRebuildReviewSheet
        | M5BuildRemoteBoundaryComponentFamily::LocalSafeContinuationCard => {
            M5BuildRemoteClaimDimension::ContinuityTruth
        }
    }
}

/// A rendered fallback modality for a build/remote-boundary component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteFallbackModality {
    /// A rich, structured (lifecycle / persistence / continuity / preserved-vs-lost grid)
    /// projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5BuildRemoteFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich, structured surface
    /// (i.e. a keyboard / screen-reader / headless path).
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
/// headless CLI, handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteRenderingSurface {
    /// The full-capability desktop shell surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A handoff packet.
    HandoffPacket,
    /// A support / admin export.
    SupportExport,
}

impl M5BuildRemoteRenderingSurface {
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
            Self::HandoffPacket => "handoff_packet",
            Self::SupportExport => "support_export",
        }
    }
}

/// Keyboard / screen-reader / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildRemoteNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only / menu-only surface that traps keyboard / assistive-tech /
    /// headless users (red).
    ViewOnlyTrap,
}

impl BuildRemoteNonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / headless users.
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

/// Whether an export-safe summary preserves the component meaning without a screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildRemoteExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl BuildRemoteExportSummaryState {
    /// Returns true when the export never falls back to a screenshot alone.
    pub const fn never_screenshot_only(self) -> bool {
        !matches!(self, Self::AbsentNeedsScreenshot)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::DisclosedPartialCapture)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReconstructableWithoutScreenshot => "reconstructable_without_screenshot",
            Self::DisclosedPartialCapture => "disclosed_partial_capture",
            Self::AbsentNeedsScreenshot => "absent_needs_screenshot",
        }
    }
}

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildRemoteNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl BuildRemoteNarrowingDisclosureState {
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

/// The boundary-support claim ceiling a component asserts: how strong a build/remote-boundary
/// truth it lets a surface present. Auto-narrowing lowers this ceiling when a boundary dimension
/// weakens so a stale, unverified, unsupported, or partial state can never keep an old fresh
/// first-party `full-truth` label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteAccessClaim {
    /// Full-truth: the component's live, first-party-local host / origin / lifecycle truth is
    /// fully reachable and current — the strongest claim.
    FullTruth,
    /// Resolved-truth: a resolved, self-sufficient boundary posture (a fully-labelled adapter
    /// chip, discovery diff, expiry banner, review sheet, or local-safe card) that is not itself a
    /// live-adapting first-party-local stream.
    ResolvedTruth,
    /// Degraded: usable, but with a disclosed reduction in discovery, host, lifecycle, or
    /// continuity confidence.
    Degraded,
    /// Stale: a stale snapshot is deliberately kept visible pending refresh, not a live current
    /// value.
    Stale,
    /// Unverified: continuity, host, or lifecycle evidence relative to the prior runtime cannot be
    /// verified.
    Unverified,
    /// Unsupported: the boundary truth is unsupported on this claimed profile and the component
    /// downgrades visibly rather than inheriting a stronger host/lifecycle claim.
    Unsupported,
}

impl M5BuildRemoteAccessClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 6] = [
        Self::FullTruth,
        Self::ResolvedTruth,
        Self::Degraded,
        Self::Stale,
        Self::Unverified,
        Self::Unsupported,
    ];

    /// Capability rank; a higher rank asserts a stronger boundary posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::FullTruth => 5,
            Self::ResolvedTruth => 4,
            Self::Degraded => 3,
            Self::Stale => 2,
            Self::Unverified => 1,
            Self::Unsupported => 0,
        }
    }

    /// Returns true when this claim asserts live, current, fresh first-party-local truth.
    pub const fn asserts_live_truth(self) -> bool {
        matches!(self, Self::FullTruth)
    }

    /// Returns true when this claim asserts a fully self-sufficient (live or resolved / current)
    /// boundary posture.
    pub const fn asserts_full_self_sufficiency(self) -> bool {
        matches!(self, Self::FullTruth | Self::ResolvedTruth)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullTruth => "full_truth",
            Self::ResolvedTruth => "resolved_truth",
            Self::Degraded => "degraded",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
            Self::Unsupported => "unsupported",
        }
    }
}

/// The build/remote-boundary dimension whose state governs how far a component may claim fresh,
/// current first-party-local truth. These are exactly the axes the spec requires auto-narrowing
/// on: discovery confidence / target identity, host ownership, execution origin, lifecycle state,
/// expiry timing, and continuity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteClaimDimension {
    /// Discovery-confidence truth: is the adapter/discovery confidence in the resolved target
    /// current, or has it drifted or gone heuristic?
    DiscoveryConfidenceTruth,
    /// Host-ownership truth: is the host that owns the execution resolved and named?
    HostOwnershipTruth,
    /// Execution-origin truth: is the origin locus where the work actually ran resolved and named?
    ExecutionOriginTruth,
    /// Lifecycle-state truth: is the managed-workspace lifecycle state resolved and named?
    LifecycleStateTruth,
    /// Expiry-timing truth: is the expiry timing that governs the workspace resolved and named?
    ExpiryTimingTruth,
    /// Continuity truth: is the claimed continuity relative to the prior runtime verified, or has
    /// the workspace rebuilt / recreated / expired / dropped to local-safe only?
    ContinuityTruth,
}

impl M5BuildRemoteClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DiscoveryConfidenceTruth,
        Self::HostOwnershipTruth,
        Self::ExecutionOriginTruth,
        Self::LifecycleStateTruth,
        Self::ExpiryTimingTruth,
        Self::ContinuityTruth,
    ];

    /// The frozen downgrade trigger this dimension names when its weakness binds a narrowing. Each
    /// dimension maps to the on-topic frozen trigger the freeze matrix already governs, so the
    /// certified reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5BuildRemoteDowngradeTrigger {
        match self {
            Self::DiscoveryConfidenceTruth => M5BuildRemoteDowngradeTrigger::DiscoveryDriftHidden,
            Self::HostOwnershipTruth => M5BuildRemoteDowngradeTrigger::HostBoundaryUnstated,
            Self::ExecutionOriginTruth => M5BuildRemoteDowngradeTrigger::ExecutionOriginUnstated,
            Self::LifecycleStateTruth => M5BuildRemoteDowngradeTrigger::LifecycleStateUnstated,
            Self::ExpiryTimingTruth => M5BuildRemoteDowngradeTrigger::ExpiryTimingUnstated,
            Self::ContinuityTruth => M5BuildRemoteDowngradeTrigger::ExactContinuityOverclaimed,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiscoveryConfidenceTruth => "discovery_confidence_truth",
            Self::HostOwnershipTruth => "host_ownership_truth",
            Self::ExecutionOriginTruth => "execution_origin_truth",
            Self::LifecycleStateTruth => "lifecycle_state_truth",
            Self::ExpiryTimingTruth => "expiry_timing_truth",
            Self::ContinuityTruth => "continuity_truth",
        }
    }
}

/// The observed condition of one build/remote-boundary dimension. Anything weaker than
/// [`Self::Intact`] imposes a narrowing ceiling on the component's support claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5BuildRemoteConditionState {
    /// Fully verified / current / live first-party-local — imposes no ceiling.
    Intact,
    /// Partially resolved — scope or confidence is reduced; support drops to degraded.
    Partial,
    /// Stale — a stale snapshot is deliberately kept visible pending refresh; support drops to
    /// stale.
    Stale,
    /// Unverified — continuity / host / lifecycle evidence cannot be verified; support drops to
    /// unverified.
    Unverified,
    /// Unsupported — the boundary truth is unsupported on this claimed profile; support drops to
    /// unsupported.
    Unsupported,
}

impl M5BuildRemoteConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Intact,
        Self::Partial,
        Self::Stale,
        Self::Unverified,
        Self::Unsupported,
    ];

    /// Returns true when the dimension is weaker than intact and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Intact)
    }

    /// The strongest boundary-support claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5BuildRemoteAccessClaim {
        match self {
            Self::Intact => M5BuildRemoteAccessClaim::FullTruth,
            Self::Partial => M5BuildRemoteAccessClaim::Degraded,
            Self::Stale => M5BuildRemoteAccessClaim::Stale,
            Self::Unverified => M5BuildRemoteAccessClaim::Unverified,
            Self::Unsupported => M5BuildRemoteAccessClaim::Unsupported,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Intact => "intact",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Unverified => "unverified",
            Self::Unsupported => "unsupported",
        }
    }
}

/// One build/remote-boundary dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5BuildRemoteClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5BuildRemoteConditionState,
}

/// An honest boundary-support-claim auto-narrow block. When a boundary dimension weakens, the
/// component's support claim lowers to the permitted ceiling, names the binding dimension and
/// frozen trigger, and preserves the canonical target / host / lifecycle / continuity identity
/// rather than silently dropping it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteClaimAutoNarrow {
    /// The support claim the component is narrowed to.
    pub narrowed_to: M5BuildRemoteAccessClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5BuildRemoteClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5BuildRemoteDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical target, host, execution origin, lifecycle, expiry, and continuity are
    /// preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
}

impl BuildRemoteClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and carries a
    /// precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl BuildRemoteCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at
    /// least one export field is named, and screenshots are prohibited as the sole export.
    pub fn is_complete(&self) -> bool {
        self.screenshot_only_prohibited
            && self.formats.iter().any(|f| f == "text")
            && self.formats.iter().any(|f| f == "json")
            && self.formats.iter().any(|f| f == "markdown")
            && !self.export_fields.is_empty()
    }
}

/// Per-rendering-surface narrowing disclosure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5BuildRemoteRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: BuildRemoteNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a build/remote-boundary-component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuildRemoteAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims boundary truth, or drops state
    /// silently (red).
    Stranded,
}

impl BuildRemoteAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one build/remote-boundary component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteBoundaryAccessibilityRow {
    /// Record kind; must equal [`BUILD_REMOTE_BOUNDARY_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5BuildRemoteBoundaryComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the target / host / workspace context this component acts on; stays visible
    /// on every surface, so this is never empty.
    pub boundary_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a non-visual (list /
    /// textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5BuildRemoteFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical discovery confidence, host boundary,
    /// execution origin, lifecycle state, expiry timing, and continuity as the rich surface; must
    /// hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: BuildRemoteNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: BuildRemoteNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: BuildRemoteNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: BuildRemoteExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: BuildRemoteCopyExportParity,
    /// The full support claim this family asserts when every dimension is intact.
    pub full_support_claim: M5BuildRemoteAccessClaim,
    /// The observed condition of each modeled boundary dimension.
    #[serde(default)]
    pub claim_conditions: Vec<BuildRemoteClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's
    /// full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<BuildRemoteClaimAutoNarrow>,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5BuildRemoteRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<BuildRemoteRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5BuildRemoteRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5BuildRemoteConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl BuildRemoteBoundaryAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to a flat
    /// non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback modality is
    /// offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `Intact` when the row does not model
    /// that dimension.
    pub fn condition_for(
        &self,
        dimension: M5BuildRemoteClaimDimension,
    ) -> M5BuildRemoteConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5BuildRemoteConditionState::Intact)
    }

    /// Whether any modeled dimension is weaker than intact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest support claim permitted after applying every modeled dimension's ceiling,
    /// capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5BuildRemoteAccessClaim {
        let mut permitted = self.full_support_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows
    /// below the family's full claim.
    pub fn binding_dimension(&self) -> Option<M5BuildRemoteClaimDimension> {
        let mut binding: Option<(M5BuildRemoteClaimDimension, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_support_claim.capability_rank() {
                // The dimension is weak but does not narrow below the full claim.
                continue;
            }
            let rank = ceiling.capability_rank();
            match binding {
                Some((_, best)) if best <= rank => {}
                _ => binding = Some((condition.dimension, rank)),
            }
        }
        binding.map(|(dimension, _)| dimension)
    }

    /// The support claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5BuildRemoteAccessClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_support_claim,
        }
    }

    /// AC / auto-narrowing honesty: a stale, unverified, unsupported, or partial boundary state can
    /// no longer keep an old fresh first-party `full-truth` / `resolved-truth` label. The effective
    /// claim never exceeds the permitted ceiling; when a dimension narrows below the full claim, an
    /// honest narrow block is present, narrows to exactly the permitted ceiling, binds to the
    /// ceiling-imposing dimension with its frozen trigger, and preserves canonical identity. When
    /// nothing narrows, no spurious narrow block is present.
    pub fn claim_is_honest(&self) -> bool {
        let permitted = self.permitted_claim();
        if self.effective_claim().capability_rank() > permitted.capability_rank() {
            return false;
        }
        match (&self.claim_narrow, self.binding_dimension()) {
            (Some(narrow), Some(binding)) => {
                narrow.is_honest()
                    && narrow.narrowed_to == permitted
                    && narrow.binding_dimension == binding
                    && narrow.trigger == binding.default_trigger()
                    && self.condition_for(binding).is_weak()
            }
            // A narrow block with no ceiling-imposing dimension is spurious.
            (Some(_), None) => false,
            // A ceiling-imposing dimension with no narrow block over-claims.
            (None, Some(_)) => false,
            (None, None) => true,
        }
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth
    /// — no keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a non-visual
    /// fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.boundary_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_hierarchy_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries an
    /// honest claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
            || self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC / cross-surface disclosure: every narrower rendering surface discloses its reduced
    /// interactivity and keeps its labels, so claim publication and field triage stay aligned on
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
        M5BuildRemoteRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> BuildRemoteAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return BuildRemoteAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            BuildRemoteAccessibilityStatus::NarrowedDisclosed
        } else {
            BuildRemoteAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == BUILD_REMOTE_BOUNDARY_A11Y_ROW_RECORD_KIND
            && self.schema_version == BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.boundary_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.claim_conditions.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} keyboard={keyboard} screen_reader={screen_reader} cli={cli} \
export={export} full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_support_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1081 build/remote-boundary-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteBoundaryAccessibilitySummary {
    pub family_count: usize,
    pub hierarchy_heavy_family_count: usize,
    pub all_hierarchy_heavy_have_non_visual_fallback: bool,
    pub all_reach_canonical_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_narrowing_disclosed: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`BuildRemoteBoundaryAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildRemoteBoundaryAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<BuildRemoteBoundaryAccessibilityRow>,
}

/// Checked-in M05-1081 build/remote-boundary-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BuildRemoteBoundaryAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<BuildRemoteBoundaryAccessibilityRow>,
    pub summary: BuildRemoteBoundaryAccessibilitySummary,
}

impl BuildRemoteBoundaryAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: BuildRemoteBoundaryAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION,
            record_kind: BUILD_REMOTE_BOUNDARY_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: BuildRemoteBoundaryAccessibilitySummary {
                family_count: 0,
                hierarchy_heavy_family_count: 0,
                all_hierarchy_heavy_have_non_visual_fallback: false,
                all_reach_canonical_truth_via_at: false,
                all_claims_honest: false,
                all_export_summaries_preserve_meaning: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5BuildRemoteBoundaryComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5BuildRemoteClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Support claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5BuildRemoteAccessClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> HashSet<M5BuildRemoteConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> BuildRemoteBoundaryAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: HashSet<M5BuildRemoteConsumerSurface> = HashSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&BuildRemoteBoundaryAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                BuildRemoteAccessibilityStatus::Parity => green += 1,
                BuildRemoteAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                BuildRemoteAccessibilityStatus::Stranded => red += 1,
            }
        }

        BuildRemoteBoundaryAccessibilitySummary {
            family_count: self.rows.len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(BuildRemoteBoundaryAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(BuildRemoteBoundaryAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(BuildRemoteBoundaryAccessibilityRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(BuildRemoteBoundaryAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<BuildRemoteBoundaryAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION {
            violations.push(BuildRemoteBoundaryAccessibilityViolation::SchemaVersion {
                expected: BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != BUILD_REMOTE_BOUNDARY_A11Y_RECORD_KIND {
            violations.push(BuildRemoteBoundaryAccessibilityViolation::RecordKind {
                expected: BUILD_REMOTE_BOUNDARY_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(BuildRemoteBoundaryAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(BuildRemoteBoundaryAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(BuildRemoteBoundaryAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    BuildRemoteBoundaryAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory build/remote-boundary label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    BuildRemoteBoundaryAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A hierarchy-heavy family must render a structured grid *and* a non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5BuildRemoteFallbackModality::Structured)
            {
                violations.push(
                    BuildRemoteBoundaryAccessibilityViolation::HierarchyHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts fresh first-party truth for a weakened one.
            if !row.claim_is_honest() {
                violations.push(
                    BuildRemoteBoundaryAccessibilityViolation::ClaimOverAsserted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    BuildRemoteBoundaryAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(
                    BuildRemoteBoundaryAccessibilityViolation::ExportRequiresScreenshot {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    BuildRemoteBoundaryAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    BuildRemoteBoundaryAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == BuildRemoteAccessibilityStatus::Stranded {
                violations.push(BuildRemoteBoundaryAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5BuildRemoteBoundaryComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    BuildRemoteBoundaryAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5BuildRemoteClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    BuildRemoteBoundaryAccessibilityViolation::MissingDimensionCoverage {
                        dimension,
                    },
                );
            }
        }

        // Coverage: every support claim tier appears as an effective claim, so the full narrowing
        // spectrum (full-truth → … → unsupported) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5BuildRemoteAccessClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    BuildRemoteBoundaryAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Cross-surface: the same narrowed state must reach the shell, run/test/debug, notebook,
        // preview, companion, incident/diagnostics, and support/admin exports — so every consumer
        // surface is exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5_BUILD_REMOTE_BOUNDARY_A11Y_CONSUMER_SURFACES {
            if !consumers.contains(&surface) {
                violations.push(
                    BuildRemoteBoundaryAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(BuildRemoteBoundaryAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("build/remote-boundary accessibility parity packet serializes"),
        ) {
            violations.push(BuildRemoteBoundaryAccessibilityViolation::RawRemoteMaterialInExport);
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
            .expect("build/remote-boundary accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                cli = row.cli_reach.as_str(),
                export = row.export_summary.as_str(),
                full = row.full_support_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Build/Remote-Boundary Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5BuildRemoteBoundaryComponentFamily::ALL.len(),
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
                    row.full_support_claim.as_str(),
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

/// Reads and validates the checked-in build/remote-boundary-component accessibility parity export.
pub fn current_m5_build_remote_boundary_a11y_export(
) -> Result<BuildRemoteBoundaryAccessibilityPacket, BuildRemoteBoundaryAccessibilityArtifactError> {
    let packet: BuildRemoteBoundaryAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-build-remote-boundary-component-accessibility-proof/support_export.json"
    )))
    .map_err(BuildRemoteBoundaryAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(BuildRemoteBoundaryAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in build/remote-boundary-component accessibility parity
/// export.
#[derive(Debug)]
pub enum BuildRemoteBoundaryAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<BuildRemoteBoundaryAccessibilityViolation>),
}

impl fmt::Display for BuildRemoteBoundaryAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "build/remote-boundary accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "build/remote-boundary accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for BuildRemoteBoundaryAccessibilityArtifactError {}

/// The consumer surfaces this lane requires the packet to exercise. The full
/// [`M5BuildRemoteConsumerSurface`] set — the support export and product surfaces plus the shell /
/// run-test-debug / notebook / preview / companion / incident surfaces where a build/remote-boundary
/// component is embedded.
pub const M5_BUILD_REMOTE_BOUNDARY_A11Y_CONSUMER_SURFACES: [M5BuildRemoteConsumerSurface; 8] =
    M5BuildRemoteConsumerSurface::ALL;

/// Validation failure for M05-1081 build/remote-boundary-component accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BuildRemoteBoundaryAccessibilityViolation {
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
        dimension: M5BuildRemoteClaimDimension,
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
    AssistiveTechStranded {
        id: String,
    },
    ExportRequiresScreenshot {
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
        family: M5BuildRemoteBoundaryComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5BuildRemoteClaimDimension,
    },
    MissingClaimTierCoverage {
        claim: M5BuildRemoteAccessClaim,
    },
    MissingConsumerSurfaceCoverage {
        surface: M5BuildRemoteConsumerSurface,
    },
    SummaryMismatch,
    RawRemoteMaterialInExport,
}

impl fmt::Display for BuildRemoteBoundaryAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory build/remote-boundary label")
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
                    "row {id} over-asserts fresh first-party truth for a weakened one, or narrows spuriously"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands keyboard / assistive-tech / CLI users from the canonical truth"
                )
            }
            Self::ExportRequiresScreenshot { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve meaning without a screenshot"
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
            Self::MissingClaimTierCoverage { claim } => {
                write!(
                    f,
                    "support claim tier {} does not appear as an effective claim",
                    claim.as_str()
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
            Self::RawRemoteMaterialInExport => {
                write!(f, "export contains raw remote material")
            }
        }
    }
}

impl Error for BuildRemoteBoundaryAccessibilityViolation {}

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
            | "offline"
            | "stale"
            | "blocked"
            | "unverified"
            | "loading"
            | "content"
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

/// Builds the canonical, checked-in build/remote-boundary-component accessibility parity packet.
/// This is the one source of truth shared by the tests, the artifact writer, and the on-disk
/// support export so all three stay byte-aligned.
pub fn seeded_m5_build_remote_boundary_a11y_packet() -> BuildRemoteBoundaryAccessibilityPacket {
    BuildRemoteBoundaryAccessibilityPacket::new(BuildRemoteBoundaryAccessibilityPacketInput {
        packet_id: "m5-build-remote-boundary-component-accessibility-parity:stable:0001".to_owned(),
        as_of: "2026-07-11T00:00:00Z".to_owned(),
        matrix_ref: BUILD_REMOTE_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:build-remote-boundary-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5BuildRemoteRequiredLabel> {
    M5BuildRemoteRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> BuildRemoteCopyExportParity {
    BuildRemoteCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5BuildRemoteClaimDimension,
    state: M5BuildRemoteConditionState,
) -> BuildRemoteClaimConditionEntry {
    BuildRemoteClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support/export replay and the general
/// product UI — so the narrowed state always reaches field triage.
fn base_consumers(extra: &[M5BuildRemoteConsumerSurface]) -> Vec<M5BuildRemoteConsumerSurface> {
    let mut out = vec![
        M5BuildRemoteConsumerSurface::SupportExport,
        M5BuildRemoteConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps
/// full label and summary parity on the narrower surfaces; a narrowed row discloses the reduced
/// interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: BuildRemoteNarrowingDisclosureState,
) -> Vec<BuildRemoteRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        BuildRemoteRenderingNarrowingDisclosure {
            rendering_surface: M5BuildRemoteRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        BuildRemoteRenderingNarrowingDisclosure {
            rendering_surface: M5BuildRemoteRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_action".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and
/// summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<BuildRemoteRenderingNarrowingDisclosure> {
    surface_disclosures(labels, BuildRemoteNarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<BuildRemoteRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        BuildRemoteNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5BuildRemoteRenderingSurface> {
    vec![
        M5BuildRemoteRenderingSurface::DesktopFull,
        M5BuildRemoteRenderingSurface::CliHeadless,
        M5BuildRemoteRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<BuildRemoteBoundaryAccessibilityRow> {
    vec![
        // Host-boundary strip — the host that owns the execution is resolved as the local desktop
        // host; the strip reports a live, first-party-local host truth reachable on every surface
        // (green).
        BuildRemoteBoundaryAccessibilityRow {
            record_kind: BUILD_REMOTE_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:host-boundary-strip".to_owned(),
            component_family: M5BuildRemoteBoundaryComponentFamily::HostBoundaryStrip,
            source_family_schema_ref: BUILD_REMOTE_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "build-remote-boundary:host-boundary-strip:0001".to_owned(),
            fallback_modalities: vec![
                M5BuildRemoteFallbackModality::List,
                M5BuildRemoteFallbackModality::Textual,
                M5BuildRemoteFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            cli_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            export_summary: BuildRemoteExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:host-boundary-strip:a11y".to_owned(),
            copy_export: copy_export(&[
                "host_kind",
                "owner_runtime",
                "locality_class",
                "reconnect_state",
            ]),
            full_support_claim: M5BuildRemoteAccessClaim::FullTruth,
            claim_conditions: vec![condition(
                M5BuildRemoteClaimDimension::HostOwnershipTruth,
                M5BuildRemoteConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&["host_kind", "owner_runtime", "locality_class"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5BuildRemoteConsumerSurface::ShellUi]),
            source_refs: vec![
                "TDD host-boundary / execution-context contracts".to_owned(),
                BUILD_REMOTE_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("host-boundary-strip"),
        },
        // Execution-origin receipt row — the origin locus where the work ran is fully resolved and
        // labelled; the receipt reports a ready, self-sufficient export-safe lineage (never a
        // live-adapting first-party-local stream) (green).
        BuildRemoteBoundaryAccessibilityRow {
            record_kind: BUILD_REMOTE_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:execution-origin-receipt-row".to_owned(),
            component_family: M5BuildRemoteBoundaryComponentFamily::ExecutionOriginReceiptRow,
            source_family_schema_ref: BUILD_REMOTE_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "build-remote-boundary:execution-origin-receipt-row:0002"
                .to_owned(),
            fallback_modalities: vec![
                M5BuildRemoteFallbackModality::List,
                M5BuildRemoteFallbackModality::Textual,
                M5BuildRemoteFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            cli_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            export_summary: BuildRemoteExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:execution-origin-receipt-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "origin_locus",
                "resolved_target_identity",
                "provenance",
                "export_safe_lineage",
            ]),
            full_support_claim: M5BuildRemoteAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5BuildRemoteClaimDimension::ExecutionOriginTruth,
                M5BuildRemoteConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "origin_locus",
                "resolved_target_identity",
                "provenance",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5BuildRemoteConsumerSurface::RunTestDebugUi]),
            source_refs: vec![
                "TAD deployment / remote-agent / execution-context architecture".to_owned(),
                BUILD_REMOTE_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("execution-origin-receipt-row"),
        },
        // Adapter-confidence chip — the adapter's confidence in the resolved target is only
        // partially resolved (the build adapter is still settling), so the chip auto-narrows to
        // degraded rather than reading as a fully-confident target and preserves the resolved
        // target identity (yellow).
        BuildRemoteBoundaryAccessibilityRow {
            record_kind: BUILD_REMOTE_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:adapter-confidence-chip".to_owned(),
            component_family: M5BuildRemoteBoundaryComponentFamily::AdapterConfidenceChip,
            source_family_schema_ref: BUILD_REMOTE_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "build-remote-boundary:adapter-confidence-chip:0003".to_owned(),
            fallback_modalities: vec![
                M5BuildRemoteFallbackModality::List,
                M5BuildRemoteFallbackModality::Textual,
                M5BuildRemoteFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            cli_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            export_summary: BuildRemoteExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:adapter-confidence-chip:a11y".to_owned(),
            copy_export: copy_export(&[
                "adapter_source_class",
                "confidence_band",
                "resolved_target_identity",
                "review_state",
            ]),
            full_support_claim: M5BuildRemoteAccessClaim::FullTruth,
            claim_conditions: vec![condition(
                M5BuildRemoteClaimDimension::DiscoveryConfidenceTruth,
                M5BuildRemoteConditionState::Partial,
            )],
            claim_narrow: Some(BuildRemoteClaimAutoNarrow {
                narrowed_to: M5BuildRemoteAccessClaim::Degraded,
                binding_dimension: M5BuildRemoteClaimDimension::DiscoveryConfidenceTruth,
                trigger: M5BuildRemoteDowngradeTrigger::DiscoveryDriftHidden,
                narrowed_label:
                    "Adapter confidence partially resolved — chip shown degraded until the build adapter's confidence in the resolved target settles"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "adapter_source_class",
                "confidence_band",
                "resolved_target_identity",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5BuildRemoteConsumerSurface::RunTestDebugUi]),
            source_refs: vec![
                "UI/UX Spec v3.8 build-intelligence confidence review findings".to_owned(),
                BUILD_REMOTE_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("adapter-confidence-chip"),
        },
        // Discovery-diff card — the discovery proof of the heuristic-vs-resolved target has gone
        // stale beyond the freshness window and is deliberately kept visible pending re-discovery,
        // so the card auto-narrows to stale rather than reading as a fresh resolved target
        // (yellow).
        BuildRemoteBoundaryAccessibilityRow {
            record_kind: BUILD_REMOTE_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:discovery-diff-card".to_owned(),
            component_family: M5BuildRemoteBoundaryComponentFamily::DiscoveryDiffCard,
            source_family_schema_ref: BUILD_REMOTE_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "build-remote-boundary:discovery-diff-card:0004".to_owned(),
            fallback_modalities: vec![
                M5BuildRemoteFallbackModality::List,
                M5BuildRemoteFallbackModality::Textual,
                M5BuildRemoteFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            cli_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            export_summary: BuildRemoteExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:discovery-diff-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "discovery_mode",
                "target_identity_drift",
                "changed_certainty",
                "review_before_switch",
            ]),
            full_support_claim: M5BuildRemoteAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5BuildRemoteClaimDimension::DiscoveryConfidenceTruth,
                M5BuildRemoteConditionState::Stale,
            )],
            claim_narrow: Some(BuildRemoteClaimAutoNarrow {
                narrowed_to: M5BuildRemoteAccessClaim::Stale,
                binding_dimension: M5BuildRemoteClaimDimension::DiscoveryConfidenceTruth,
                trigger: M5BuildRemoteDowngradeTrigger::DiscoveryDriftHidden,
                narrowed_label:
                    "Discovery proof stale — shown as a stale diff with its last-discovery time, not a fresh resolved target, pending re-discovery"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "discovery_mode",
                "target_identity_drift",
                "changed_certainty",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5BuildRemoteConsumerSurface::IncidentUi]),
            source_refs: vec![
                "TDD build-intelligence / target-discovery contracts".to_owned(),
                BUILD_REMOTE_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("discovery-diff-card"),
        },
        // Managed-workspace lifecycle card — the lifecycle state is only partially resolved
        // (reconnecting to the managed control plane), so the card auto-narrows to degraded rather
        // than reading as a fully-live managed workspace and preserves the workspace identity
        // (yellow).
        BuildRemoteBoundaryAccessibilityRow {
            record_kind: BUILD_REMOTE_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:managed-workspace-lifecycle-card".to_owned(),
            component_family: M5BuildRemoteBoundaryComponentFamily::ManagedWorkspaceLifecycleCard,
            source_family_schema_ref: BUILD_REMOTE_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "build-remote-boundary:managed-workspace-lifecycle-card:0005"
                .to_owned(),
            fallback_modalities: vec![
                M5BuildRemoteFallbackModality::List,
                M5BuildRemoteFallbackModality::Textual,
                M5BuildRemoteFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            cli_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            export_summary: BuildRemoteExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:managed-workspace-lifecycle-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "lifecycle_state",
                "persistence_class",
                "workspace_identity",
                "template_image_provenance",
            ]),
            full_support_claim: M5BuildRemoteAccessClaim::FullTruth,
            claim_conditions: vec![condition(
                M5BuildRemoteClaimDimension::LifecycleStateTruth,
                M5BuildRemoteConditionState::Partial,
            )],
            claim_narrow: Some(BuildRemoteClaimAutoNarrow {
                narrowed_to: M5BuildRemoteAccessClaim::Degraded,
                binding_dimension: M5BuildRemoteClaimDimension::LifecycleStateTruth,
                trigger: M5BuildRemoteDowngradeTrigger::LifecycleStateUnstated,
                narrowed_label:
                    "Lifecycle state partially resolved — card shown degraded while reconnecting to the managed control plane, not a fully-live workspace"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "lifecycle_state",
                "persistence_class",
                "workspace_identity",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5BuildRemoteConsumerSurface::CompanionUi]),
            source_refs: vec![
                "TDD managed-workspace lifecycle contracts".to_owned(),
                BUILD_REMOTE_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("managed-workspace-lifecycle-card"),
        },
        // Suspend/resume/rebuild review sheet — hierarchy-heavy (nested lifecycle / persistence /
        // continuity / preserved-vs-lost grid); the claimed continuity relative to the prior
        // runtime cannot be verified after a rebuild, so the sheet auto-narrows to unverified
        // rather than reading as exact continuity, binds its grid to a flat list / textual path,
        // and preserves the preserved-vs-lost identity (yellow).
        BuildRemoteBoundaryAccessibilityRow {
            record_kind: BUILD_REMOTE_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:suspend-resume-rebuild-review-sheet".to_owned(),
            component_family: M5BuildRemoteBoundaryComponentFamily::SuspendResumeRebuildReviewSheet,
            source_family_schema_ref: BUILD_REMOTE_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "build-remote-boundary:suspend-resume-rebuild-review-sheet:0006"
                .to_owned(),
            fallback_modalities: vec![
                M5BuildRemoteFallbackModality::Structured,
                M5BuildRemoteFallbackModality::List,
                M5BuildRemoteFallbackModality::Textual,
                M5BuildRemoteFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BuildRemoteNonVisualReachState::DisclosedReducedButReachable,
            cli_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            export_summary: BuildRemoteExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:suspend-resume-rebuild-review-sheet:a11y".to_owned(),
            copy_export: copy_export(&[
                "lifecycle_state",
                "changed_persistence_class",
                "preserved_vs_lost_state",
                "claimed_continuity",
            ]),
            full_support_claim: M5BuildRemoteAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5BuildRemoteClaimDimension::ContinuityTruth,
                M5BuildRemoteConditionState::Unverified,
            )],
            claim_narrow: Some(BuildRemoteClaimAutoNarrow {
                narrowed_to: M5BuildRemoteAccessClaim::Unverified,
                binding_dimension: M5BuildRemoteClaimDimension::ContinuityTruth,
                trigger: M5BuildRemoteDowngradeTrigger::ExactContinuityOverclaimed,
                narrowed_label:
                    "Continuity unverified after rebuild — shown as material-change, not exact continuity, with the preserved-vs-lost state named for review"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "lifecycle_state",
                "changed_persistence_class",
                "preserved_vs_lost_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5BuildRemoteConsumerSurface::IncidentUi]),
            source_refs: vec![
                "TAD continuity architecture / managed-workspace lifecycle".to_owned(),
                BUILD_REMOTE_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("suspend-resume-rebuild-review-sheet"),
        },
        // Workspace-expiry banner — the expiry timing that governs the workspace is stale (the
        // control-plane expiry clock has not refreshed), so the banner auto-narrows to stale rather
        // than reading as a fresh expiry deadline, and never reads as a generic disconnect
        // (yellow).
        BuildRemoteBoundaryAccessibilityRow {
            record_kind: BUILD_REMOTE_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:workspace-expiry-banner".to_owned(),
            component_family: M5BuildRemoteBoundaryComponentFamily::WorkspaceExpiryBanner,
            source_family_schema_ref: BUILD_REMOTE_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "build-remote-boundary:workspace-expiry-banner:0007".to_owned(),
            fallback_modalities: vec![
                M5BuildRemoteFallbackModality::List,
                M5BuildRemoteFallbackModality::Textual,
                M5BuildRemoteFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            cli_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            export_summary: BuildRemoteExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:workspace-expiry-banner:a11y".to_owned(),
            copy_export: copy_export(&[
                "expiry_class",
                "expiry_timing",
                "triggering_owner_source",
                "export_before_loss",
            ]),
            full_support_claim: M5BuildRemoteAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5BuildRemoteClaimDimension::ExpiryTimingTruth,
                M5BuildRemoteConditionState::Stale,
            )],
            claim_narrow: Some(BuildRemoteClaimAutoNarrow {
                narrowed_to: M5BuildRemoteAccessClaim::Stale,
                binding_dimension: M5BuildRemoteClaimDimension::ExpiryTimingTruth,
                trigger: M5BuildRemoteDowngradeTrigger::ExpiryTimingUnstated,
                narrowed_label:
                    "Expiry timing stale — shown as a stale expiry clock with its last-known deadline, not a fresh countdown or a generic disconnect, pending control-plane refresh"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "expiry_class",
                "expiry_timing",
                "triggering_owner_source",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5BuildRemoteConsumerSurface::PreviewUi]),
            source_refs: vec![
                "Milestones v3.1 durable truth and boundary governance".to_owned(),
                BUILD_REMOTE_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("workspace-expiry-banner"),
        },
        // Local-safe continuation card — managed continuity is unsupported on this claimed profile,
        // so the card downgrades visibly to unsupported rather than silently inheriting a stronger
        // managed-continuity claim, while preserving the local-safe continuation identity and never
        // implying exact continuity (yellow).
        BuildRemoteBoundaryAccessibilityRow {
            record_kind: BUILD_REMOTE_BOUNDARY_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: BUILD_REMOTE_BOUNDARY_A11Y_SCHEMA_VERSION,
            row_id: "a11y:local-safe-continuation-card".to_owned(),
            component_family: M5BuildRemoteBoundaryComponentFamily::LocalSafeContinuationCard,
            source_family_schema_ref: BUILD_REMOTE_BOUNDARY_A11Y_COMPONENT_MATRIX_REF.to_owned(),
            boundary_context_ref: "build-remote-boundary:local-safe-continuation-card:0008"
                .to_owned(),
            fallback_modalities: vec![
                M5BuildRemoteFallbackModality::List,
                M5BuildRemoteFallbackModality::Textual,
                M5BuildRemoteFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            cli_reach: BuildRemoteNonVisualReachState::ReachableAndLabeled,
            export_summary: BuildRemoteExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:local-safe-continuation-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "continuity_class",
                "preserved_files_context",
                "lost_live_state",
                "next_safe_actions",
            ]),
            full_support_claim: M5BuildRemoteAccessClaim::ResolvedTruth,
            claim_conditions: vec![condition(
                M5BuildRemoteClaimDimension::ContinuityTruth,
                M5BuildRemoteConditionState::Unsupported,
            )],
            claim_narrow: Some(BuildRemoteClaimAutoNarrow {
                narrowed_to: M5BuildRemoteAccessClaim::Unsupported,
                binding_dimension: M5BuildRemoteClaimDimension::ContinuityTruth,
                trigger: M5BuildRemoteDowngradeTrigger::ExactContinuityOverclaimed,
                narrowed_label:
                    "Managed continuity unsupported on this profile — shown as local-safe continuation only, never exact continuity, with preserved files and lost live state named"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "continuity_class",
                "preserved_files_context",
                "lost_live_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[M5BuildRemoteConsumerSurface::NotebookUi]),
            source_refs: vec![
                "UI/UX Spec v3.8 provisioned-workspace lifecycle truth review findings".to_owned(),
                BUILD_REMOTE_BOUNDARY_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-11T00:00:00Z".to_owned(),
            evidence_refs: ev("local-safe-continuation-card"),
        },
    ]
}
