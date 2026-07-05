//! Keyboard / screen-reader / CLI / export parity and honest auto-narrowing for
//! the M5 deployment/continuity components.
//!
//! This module is the M05-834 accessibility-and-auto-narrowing capstone over the
//! frozen M5 deployment/continuity component matrix
//! ([`crate::freeze_the_m5_deployment_continuity_component_matrix`]). Where the
//! freeze matrix defines the reusable install-profile card, side-by-side import
//! sheet, rollout-ring row, deployment summary card, residual-dependency row,
//! control-plane/data-plane status strip, mirror/offline artifact row,
//! mode-change review sheet, and channel-association review row primitives and the
//! 829-831 implementation lanes resolve their per-surface truth, this lane certifies
//! — per component family — that deployment claims stay **keyboard-complete,
//! assistive-tech-reachable, CLI/export-safe, and self-narrowing** rather than
//! presenting a stale or partial lane as fully healthy, fully self-hosted, or fully
//! current:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, and CLI/headless-reachable path
//!   into the same install / deployment identity, operating mode, and
//!   rollout / residual-dependency / plane / mirror state the rich surface shows —
//!   never a view-only strip that strands assistive-tech or headless users.
//!   Spatially arranged families (the control-plane/data-plane status strip)
//!   additionally bind their multi-cell layout to a flat list / textual path.
//! - **Export parity.** The support / release export reconstructs each component's
//!   meaning from typed tokens and opaque refs without a screenshot, preserving the
//!   same install / deployment IDs, operating-mode labels, and residual-dependency
//!   states shown in-product.
//! - **Honest auto-narrowing.** When rollout state, residual dependency,
//!   control-plane freshness, mirror verification, handler ownership, or state-root
//!   integrity is partial, stale, unavailable, or policy-blocked, the component's
//!   interactive claim auto-narrows to review-required / local-cached-only /
//!   inspect-only, discloses the narrowing with a precise trigger and binding
//!   dimension, and preserves the canonical install / deployment identity rather than
//!   silently dropping it. A component with every dimension intact must NOT carry a
//!   spurious narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in UI,
//!   docs/help, release packets, and support / admin exports so claim publication and
//!   field triage stay aligned on deployment/continuity downgrade behavior.
//!
//! Each [`DeploymentAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_deployment_continuity_component_matrix::M5DeploymentComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen
//! [`M5DeploymentRequiredLabel`] and [`M5DeploymentDowngradeTrigger`] and the shared
//! [`M5DeploymentSurfaceFamily`] rather than minting parallel synonyms, so the
//! certified labels stay byte-identical to the matrix and the sibling primitive
//! packets.
//!
//! The packet is metadata-only: raw config bytes, credentials, license keys, mirror
//! URLs, provider cursors, and raw device identifiers never cross this boundary; the
//! packet carries only typed class tokens, opaque install / channel / mirror / handler
//! refs, booleans, and redacted labels so support and diagnostics exports can
//! reconstruct exactly what an accessible fallback would have shown without leaking
//! deployment state.
//!
//! The boundary schema is
//! [`schemas/ui/m5-deployment-continuity-accessibility-fallback.schema.json`](../../../../schemas/ui/m5-deployment-continuity-accessibility-fallback.schema.json).
//! The contract doc is
//! [`docs/deployment/m5_deployment_continuity_accessibility_fallback.md`](../../../../docs/deployment/m5_deployment_continuity_accessibility_fallback.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_deployment_continuity_component_matrix::{
    M5DeploymentComponentFamily, M5DeploymentDowngradeTrigger, M5DeploymentRequiredLabel,
};
use crate::implement_the_m5_install_profile_side_by_side_import_and_rollout_ring_primitive::M5DeploymentSurfaceFamily;

/// Schema version stamped on the M05-834 deployment accessibility fallback packet.
pub const DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`DeploymentAccessibilityPacket`].
pub const DEPLOYMENT_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_deployment_continuity_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`DeploymentAccessibilityRow`].
pub const DEPLOYMENT_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_deployment_continuity_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const DEPLOYMENT_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-deployment-continuity-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const DEPLOYMENT_A11Y_FALLBACK_DOC_REF: &str =
    "docs/deployment/m5_deployment_continuity_accessibility_fallback.md";

/// Repo-relative path of the frozen deployment/continuity component matrix this lane
/// certifies.
pub const DEPLOYMENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-deployment-continuity-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const DEPLOYMENT_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-deployment-continuity-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const DEPLOYMENT_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-deployment-continuity-accessibility-fallback-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const DEPLOYMENT_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-deployment-continuity-accessibility-fallback-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const DEPLOYMENT_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-deployment-continuity-accessibility-fallback-proof/report.md";

/// The reusable component families that render a spatially arranged multi-cell layout
/// (a control-plane/data-plane status strip) and therefore MUST bind their layout to an
/// equivalent flat list / textual path so it is navigable non-visually.
const fn family_needs_non_visual_layout(family: M5DeploymentComponentFamily) -> bool {
    matches!(
        family,
        M5DeploymentComponentFamily::ControlPlaneDataPlaneStatusStrip
    )
}

/// The deployment/continuity dimension whose weakening a family primarily discloses.
/// Every row must model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5DeploymentComponentFamily,
) -> M5DeploymentClaimDimension {
    match family {
        M5DeploymentComponentFamily::InstallProfileCard
        | M5DeploymentComponentFamily::ModeChangeReviewSheet => {
            M5DeploymentClaimDimension::StateRootIntegrity
        }
        M5DeploymentComponentFamily::SideBySideImportSheet
        | M5DeploymentComponentFamily::ChannelAssociationReviewRow => {
            M5DeploymentClaimDimension::HandlerOwnership
        }
        M5DeploymentComponentFamily::RolloutRingRow => M5DeploymentClaimDimension::RolloutState,
        M5DeploymentComponentFamily::DeploymentSummaryCard
        | M5DeploymentComponentFamily::ControlPlaneDataPlaneStatusStrip => {
            M5DeploymentClaimDimension::ControlPlaneFreshness
        }
        M5DeploymentComponentFamily::ResidualDependencyRow => {
            M5DeploymentClaimDimension::ResidualDependency
        }
        M5DeploymentComponentFamily::MirrorOfflineArtifactRow => {
            M5DeploymentClaimDimension::MirrorVerification
        }
    }
}

/// A rendered fallback modality for a deployment/continuity component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentFallbackModality {
    /// A rich, structured (card / strip) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5DeploymentFallbackModality {
    /// Returns true when the modality is reachable without interpreting a rich,
    /// structured surface (i.e. a keyboard / screen-reader / headless path).
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

/// A rendering-surface capability tier. Distinct from the semantic consumer surface:
/// the same component may render at desktop-full capability or narrow to an admin
/// console, read-only browser, headless CLI, handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentRenderingSurface {
    /// The full-capability desktop / About surface.
    DesktopFull,
    /// The admin fleet console.
    AdminConsole,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A handoff packet.
    HandoffPacket,
    /// A support export.
    SupportExport,
}

impl M5DeploymentRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop
    /// full-capability baseline and therefore must disclose its reduction.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::DesktopFull)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::AdminConsole => "admin_console",
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
pub enum NonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only surface that traps keyboard / assistive-tech / headless users
    /// (red).
    ViewOnlyTrap,
}

impl NonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech /
    /// headless users.
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

/// Whether an export-safe summary preserves the component meaning without a
/// screenshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExportSummaryState {
    /// The component meaning reconstructs from the summary without a screenshot.
    ReconstructableWithoutScreenshot,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export relies on a screenshot to carry meaning (red).
    AbsentNeedsScreenshot,
}

impl ExportSummaryState {
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
pub enum NarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl NarrowingDisclosureState {
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

/// The interactive-claim ceiling a component asserts: how much control it lets a user
/// exert and how current / self-hosted it presents. Auto-narrowing lowers this ceiling
/// when a deployment dimension weakens so a stale or partial lane can never present as
/// fully healthy, fully self-hosted, or fully current.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentInteractiveClaim {
    /// Full, live, current control: the deployment presents as fully managed /
    /// self-hosted and current, with mutating actions live.
    FullyCurrentManaged,
    /// Action is allowed but gated behind an explicit review step before the durable
    /// boundary change is applied.
    ReviewRequired,
    /// The deployment narrows to a local-only / mirror-only / cached read posture:
    /// operating truth is copyable / exportable, but no live / managed / current claim
    /// is asserted.
    LocalCachedOnly,
    /// The component is inspect-only: captured / imported evidence may be viewed /
    /// navigated, nothing may be acted on or claimed as live.
    InspectOnly,
}

impl M5DeploymentInteractiveClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 4] = [
        Self::FullyCurrentManaged,
        Self::ReviewRequired,
        Self::LocalCachedOnly,
        Self::InspectOnly,
    ];

    /// Capability rank; a higher rank asserts more control / currency. Narrowing lowers
    /// rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::FullyCurrentManaged => 3,
            Self::ReviewRequired => 2,
            Self::LocalCachedOnly => 1,
            Self::InspectOnly => 0,
        }
    }

    /// Returns true when this claim asserts a mutating / current affordance rather than
    /// a read-only posture.
    pub const fn asserts_control(self) -> bool {
        matches!(self, Self::FullyCurrentManaged | Self::ReviewRequired)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyCurrentManaged => "fully_current_managed",
            Self::ReviewRequired => "review_required",
            Self::LocalCachedOnly => "local_cached_only",
            Self::InspectOnly => "inspect_only",
        }
    }
}

/// The deployment/continuity dimension whose state governs how far a component may
/// claim to be current, self-hosted, controllable, or uncaptured.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentClaimDimension {
    /// Rollout state: is the rollout ring promoted and its promotion lifecycle current?
    RolloutState,
    /// Residual dependency: is any remaining vendor dependency resolved so the
    /// self-hosted claim is complete?
    ResidualDependency,
    /// Control-plane freshness: is the managed control plane reachable and current?
    ControlPlaneFreshness,
    /// Mirror verification: is a mirrored / offline artifact fresh and signature-verified?
    MirrorVerification,
    /// Handler ownership: is the channel / handler association free of contest / capture?
    HandlerOwnership,
    /// State-root integrity: is the durable state root resolved and the mode-change
    /// boundary reviewed?
    StateRootIntegrity,
}

impl M5DeploymentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::RolloutState,
        Self::ResidualDependency,
        Self::ControlPlaneFreshness,
        Self::MirrorVerification,
        Self::HandlerOwnership,
        Self::StateRootIntegrity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RolloutState => "rollout_state",
            Self::ResidualDependency => "residual_dependency",
            Self::ControlPlaneFreshness => "control_plane_freshness",
            Self::MirrorVerification => "mirror_verification",
            Self::HandlerOwnership => "handler_ownership",
            Self::StateRootIntegrity => "state_root_integrity",
        }
    }

    /// The frozen downgrade trigger this dimension binds to when it weakens.
    pub const fn default_trigger(self) -> M5DeploymentDowngradeTrigger {
        match self {
            Self::RolloutState => M5DeploymentDowngradeTrigger::RolloutPaused,
            Self::ResidualDependency => M5DeploymentDowngradeTrigger::ResidualVendorDependency,
            Self::ControlPlaneFreshness => M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
            Self::MirrorVerification => M5DeploymentDowngradeTrigger::MirrorStale,
            Self::HandlerOwnership => M5DeploymentDowngradeTrigger::HandlerOwnershipContested,
            Self::StateRootIntegrity => M5DeploymentDowngradeTrigger::StateRootUnavailable,
        }
    }
}

/// The strength state of one deployment/continuity dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5DeploymentClaimConditionState {
    /// Fully resolved / current / uncaptured — imposes no ceiling.
    Intact,
    /// Partially resolved — action allowed only behind explicit review.
    Partial,
    /// Stale — superseded / mirrored / cached; local-cached-only.
    Stale,
    /// Unavailable — the live handle / plane / source is gone; inspect-only.
    Unavailable,
    /// Policy-blocked — action is denied by policy; inspect-only.
    PolicyBlocked,
}

impl M5DeploymentClaimConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Intact,
        Self::Partial,
        Self::Stale,
        Self::Unavailable,
        Self::PolicyBlocked,
    ];

    /// Returns true when the dimension is weaker than intact and therefore imposes a
    /// narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Intact)
    }

    /// The strongest interactive claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5DeploymentInteractiveClaim {
        match self {
            Self::Intact => M5DeploymentInteractiveClaim::FullyCurrentManaged,
            Self::Partial => M5DeploymentInteractiveClaim::ReviewRequired,
            Self::Stale => M5DeploymentInteractiveClaim::LocalCachedOnly,
            Self::Unavailable | Self::PolicyBlocked => M5DeploymentInteractiveClaim::InspectOnly,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Intact => "intact",
            Self::Partial => "partial",
            Self::Stale => "stale",
            Self::Unavailable => "unavailable",
            Self::PolicyBlocked => "policy_blocked",
        }
    }
}

/// One deployment/continuity dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5DeploymentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5DeploymentClaimConditionState,
}

/// An honest interactive-claim auto-narrow block. When a deployment dimension weakens,
/// the component's interactive claim lowers to the permitted ceiling, names the binding
/// dimension and frozen trigger, and preserves the canonical install / deployment
/// identity rather than silently dropping it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimAutoNarrow {
    /// The interactive claim the component is narrowed to.
    pub narrowed_to: M5DeploymentInteractiveClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest
    /// ceiling constraint).
    pub binding_dimension: M5DeploymentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5DeploymentDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical install / deployment identity, operating mode, and residual /
    /// mirror / plane state are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
}

impl ClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and
    /// carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be
/// copyable as text / JSON / Markdown, and a screenshot is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A screenshot is never the only export; must always hold.
    pub screenshot_only_prohibited: bool,
}

impl CopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all
    /// offered, at least one export field is named, and screenshots are prohibited as
    /// the sole export.
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
pub struct RenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5DeploymentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: NarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for a deployment accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeploymentAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims control, or drops state
    /// silently (red).
    Stranded,
}

impl DeploymentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one deployment/continuity component
/// family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentAccessibilityRow {
    /// Record kind; must equal [`DEPLOYMENT_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5DeploymentComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the install / tenant / deployment context this component acts on;
    /// stays visible on every surface, so this is never empty.
    pub operating_context_ref: String,
    /// Rendered modalities offered; a spatially arranged family must also offer a
    /// non-visual (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5DeploymentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical install / deployment
    /// identity, operating mode, and rollout / residual / plane / mirror state as the
    /// rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: NonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: NonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: NonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: ExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: CopyExportParity,
    /// The full interactive claim this family asserts when every dimension is intact.
    pub full_interactive_claim: M5DeploymentInteractiveClaim,
    /// The observed condition of each modeled deployment dimension.
    #[serde(default)]
    pub claim_conditions: Vec<ClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the
    /// family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<ClaimAutoNarrow>,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5DeploymentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<RenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5DeploymentRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5DeploymentSurfaceFamily>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl DeploymentAccessibilityRow {
    /// Returns true when this family renders a spatially arranged layout and must bind
    /// to a flat non-visual path.
    pub const fn needs_non_visual_layout(&self) -> bool {
        family_needs_non_visual_layout(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback
    /// modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `Intact` when the row does
    /// not model that dimension.
    pub fn condition_for(
        &self,
        dimension: M5DeploymentClaimDimension,
    ) -> M5DeploymentClaimConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5DeploymentClaimConditionState::Intact)
    }

    /// Whether any modeled dimension is weaker than intact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest interactive claim permitted after applying every modeled
    /// dimension's ceiling, capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5DeploymentInteractiveClaim {
        let mut permitted = self.full_interactive_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any weak dimension
    /// narrows below the family's full claim.
    pub fn binding_dimension(&self) -> Option<M5DeploymentClaimDimension> {
        let mut binding: Option<(M5DeploymentClaimDimension, u8)> = None;
        for condition in &self.claim_conditions {
            if !condition.state.is_weak() {
                continue;
            }
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() >= self.full_interactive_claim.capability_rank() {
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

    /// The interactive claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5DeploymentInteractiveClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_interactive_claim,
        }
    }

    /// AC1: a stale or limited lane can no longer present as fully healthy, fully
    /// self-hosted, or fully current. The effective claim never exceeds the permitted
    /// ceiling; when a dimension narrows below the full claim, an honest narrow block is
    /// present, narrows to exactly the permitted ceiling, binds to the ceiling-imposing
    /// dimension with its frozen trigger, and preserves canonical identity. When nothing
    /// narrows, no spurious narrow block is present.
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

    /// AC2: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / CLI trap, a spatially arranged family offers a
    /// non-visual fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.operating_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.needs_non_visual_layout() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component
    /// carries an honest claim narrow.
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

    /// AC3: every narrower rendering surface discloses its reduced interactivity and
    /// keeps its labels, so claim publication and field triage stay aligned on the same
    /// narrowed state.
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
        // Every disclosure never silently drops and preserves labels on a narrowed
        // surface.
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

    /// Derived qualification status.
    pub fn status(&self) -> DeploymentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
        {
            return DeploymentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            DeploymentAccessibilityStatus::NarrowedDisclosed
        } else {
            DeploymentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == DEPLOYMENT_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.operating_context_ref.trim().is_empty()
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
            full = self.full_interactive_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-834 deployment accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentAccessibilitySummary {
    pub family_count: usize,
    pub non_visual_layout_family_count: usize,
    pub all_non_visual_layout_have_non_visual_fallback: bool,
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

/// Constructor input for [`DeploymentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeploymentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<DeploymentAccessibilityRow>,
}

/// Checked-in M05-834 deployment accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeploymentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<DeploymentAccessibilityRow>,
    pub summary: DeploymentAccessibilitySummary,
}

impl DeploymentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: DeploymentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: DEPLOYMENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: DeploymentAccessibilitySummary {
                family_count: 0,
                non_visual_layout_family_count: 0,
                all_non_visual_layout_have_non_visual_fallback: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5DeploymentComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5DeploymentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Interactive claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5DeploymentInteractiveClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> DeploymentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let spatial: Vec<&DeploymentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.needs_non_visual_layout())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                DeploymentAccessibilityStatus::Parity => green += 1,
                DeploymentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                DeploymentAccessibilityStatus::Stranded => red += 1,
            }
        }

        DeploymentAccessibilitySummary {
            family_count: self.rows.len(),
            non_visual_layout_family_count: spatial.len(),
            all_non_visual_layout_have_non_visual_fallback: spatial
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(DeploymentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(DeploymentAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(DeploymentAccessibilityRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(DeploymentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<DeploymentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(DeploymentAccessibilityViolation::SchemaVersion {
                expected: DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != DEPLOYMENT_A11Y_FALLBACK_RECORD_KIND {
            violations.push(DeploymentAccessibilityViolation::RecordKind {
                expected: DEPLOYMENT_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(DeploymentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(DeploymentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(DeploymentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(DeploymentAccessibilityViolation::MissingPrimaryDimension {
                    id: row.row_id.clone(),
                    dimension: family_primary_dimension(row.component_family),
                });
            }

            // A spatially arranged family must render a structured layout *and* a
            // non-visual path.
            if row.needs_non_visual_layout()
                && !row
                    .fallback_modalities
                    .contains(&M5DeploymentFallbackModality::Structured)
            {
                violations.push(
                    DeploymentAccessibilityViolation::SpatialLayoutMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC1: claim never over-asserts control / currency for a weakened lane.
            if !row.claim_is_honest() {
                violations.push(DeploymentAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC2: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(DeploymentAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(DeploymentAccessibilityViolation::ExportRequiresScreenshot {
                    id: row.row_id.clone(),
                });
            }

            // AC3: narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    DeploymentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(DeploymentAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == DeploymentAccessibilityStatus::Stranded {
                violations.push(DeploymentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5DeploymentComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(DeploymentAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5DeploymentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations
                    .push(DeploymentAccessibilityViolation::MissingDimensionCoverage { dimension });
            }
        }

        // Coverage: every interactive claim tier appears as an effective claim, so the
        // full narrowing spectrum (full → review → local-cached → inspect-only) is
        // proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5DeploymentInteractiveClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(DeploymentAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(DeploymentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("deployment accessibility fallback packet serializes"),
        ) {
            violations.push(DeploymentAccessibilityViolation::RawBoundaryMaterialInExport);
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
            .expect("deployment accessibility fallback packet serializes")
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
                full = row.full_interactive_claim.as_str(),
                effective = row.effective_claim().as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown report for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Deployment/Continuity Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5DeploymentComponentFamily::ALL.len(),
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
                    row.full_interactive_claim.as_str(),
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

/// Reads and validates the checked-in deployment accessibility fallback export.
pub fn current_m5_deployment_a11y_fallback_export(
) -> Result<DeploymentAccessibilityPacket, DeploymentAccessibilityArtifactError> {
    let packet: DeploymentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-deployment-continuity-accessibility-fallback-proof/support_export.json"
    )))
    .map_err(DeploymentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(DeploymentAccessibilityArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in deployment accessibility fallback export.
#[derive(Debug)]
pub enum DeploymentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<DeploymentAccessibilityViolation>),
}

impl fmt::Display for DeploymentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "deployment accessibility fallback export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "deployment accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for DeploymentAccessibilityArtifactError {}

/// Validation failure for M05-834 deployment accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeploymentAccessibilityViolation {
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
        dimension: M5DeploymentClaimDimension,
    },
    SpatialLayoutMissingStructured {
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
        family: M5DeploymentComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5DeploymentClaimDimension,
    },
    MissingClaimTierCoverage {
        claim: M5DeploymentInteractiveClaim,
    },
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for DeploymentAccessibilityViolation {
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
            Self::SpatialLayoutMissingStructured { id } => {
                write!(
                    f,
                    "spatially arranged row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts control / currency for a weakened lane, or narrows spuriously"
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
                    "interactive claim tier {} does not appear as an effective claim",
                    claim.as_str()
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for DeploymentAccessibilityViolation {}

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

/// Builds the canonical, checked-in deployment accessibility fallback packet. This is
/// the one source of truth shared by the tests and the on-disk support export so both
/// stay byte-aligned.
pub fn seeded_m5_deployment_a11y_fallback_packet() -> DeploymentAccessibilityPacket {
    DeploymentAccessibilityPacket::new(DeploymentAccessibilityPacketInput {
        packet_id: "m5-deployment-continuity-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-04T00:00:00Z".to_owned(),
        matrix_ref: DEPLOYMENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:deployment-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5DeploymentRequiredLabel> {
    M5DeploymentRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5DeploymentClaimDimension,
    state: M5DeploymentClaimConditionState,
) -> ClaimConditionEntry {
    ClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum: the diagnostics deployment
/// pane and the support / export replay surface, so the narrowed state reaches field
/// triage.
fn base_consumers(extra: &[M5DeploymentSurfaceFamily]) -> Vec<M5DeploymentSurfaceFamily> {
    let mut out = vec![
        M5DeploymentSurfaceFamily::DiagnosticsDeployment,
        M5DeploymentSurfaceFamily::SupportExportReplay,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity)
/// row keeps full label and summary parity on the narrower surfaces; a narrowed row
/// discloses the reduced interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: NarrowingDisclosureState,
) -> Vec<RenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        RenderingNarrowingDisclosure {
            rendering_surface: M5DeploymentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        RenderingNarrowingDisclosure {
            rendering_surface: M5DeploymentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_action".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label
/// and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<RenderingNarrowingDisclosure> {
    surface_disclosures(labels, NarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced
/// interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<RenderingNarrowingDisclosure> {
    surface_disclosures(labels, NarrowingDisclosureState::DisclosedNarrowed)
}

fn rendering_surfaces() -> Vec<M5DeploymentRenderingSurface> {
    vec![
        M5DeploymentRenderingSurface::DesktopFull,
        M5DeploymentRenderingSurface::CliHeadless,
        M5DeploymentRenderingSurface::SupportExport,
    ]
}

#[allow(clippy::too_many_lines)]
fn seeded_rows() -> Vec<DeploymentAccessibilityRow> {
    vec![
        // Install-profile card — state-root integrity intact; the card presents a fully
        // current, fully attributable install, reachable on every surface (green).
        DeploymentAccessibilityRow {
            record_kind: DEPLOYMENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:install-profile-card".to_owned(),
            component_family: M5DeploymentComponentFamily::InstallProfileCard,
            source_family_schema_ref: DEPLOYMENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            operating_context_ref: "install:profile:0001".to_owned(),
            fallback_modalities: vec![
                M5DeploymentFallbackModality::Structured,
                M5DeploymentFallbackModality::List,
                M5DeploymentFallbackModality::Textual,
                M5DeploymentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:install-profile-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "install_id",
                "install_mode",
                "updater_owner",
                "state_root",
            ]),
            full_interactive_claim: M5DeploymentInteractiveClaim::FullyCurrentManaged,
            claim_conditions: vec![condition(
                M5DeploymentClaimDimension::StateRootIntegrity,
                M5DeploymentClaimConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&["install_id", "install_mode", "state_root"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DeploymentSurfaceFamily::AboutInstallCard,
                M5DeploymentSurfaceFamily::UpdateCenter,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.12".to_owned(),
                DEPLOYMENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("install-profile-card"),
        },
        // Rollout-ring row — rollout state partial (a ring is held awaiting promotion),
        // so promotion control auto-narrows to review-required (yellow).
        DeploymentAccessibilityRow {
            record_kind: DEPLOYMENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:rollout-ring-row".to_owned(),
            component_family: M5DeploymentComponentFamily::RolloutRingRow,
            source_family_schema_ref: DEPLOYMENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            operating_context_ref: "install:rollout:0002".to_owned(),
            fallback_modalities: vec![
                M5DeploymentFallbackModality::List,
                M5DeploymentFallbackModality::Textual,
                M5DeploymentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:rollout-ring-row:a11y".to_owned(),
            copy_export: copy_export(&["ring", "promotion_state", "target_scope", "rollback"]),
            full_interactive_claim: M5DeploymentInteractiveClaim::FullyCurrentManaged,
            claim_conditions: vec![condition(
                M5DeploymentClaimDimension::RolloutState,
                M5DeploymentClaimConditionState::Partial,
            )],
            claim_narrow: Some(ClaimAutoNarrow {
                narrowed_to: M5DeploymentInteractiveClaim::ReviewRequired,
                binding_dimension: M5DeploymentClaimDimension::RolloutState,
                trigger: M5DeploymentDowngradeTrigger::RolloutPaused,
                narrowed_label: "Rollout ring held — promotion gated behind explicit review"
                    .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["ring", "promotion_state", "target_scope"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DeploymentSurfaceFamily::UpdateCenter,
                M5DeploymentSurfaceFamily::AdminFleetConsole,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.12".to_owned(),
                DEPLOYMENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("rollout-ring-row"),
        },
        // Deployment summary card — control-plane freshness stale (the managed control
        // plane is mirrored / cached), so the managed claim auto-narrows to
        // local-cached-only (yellow).
        DeploymentAccessibilityRow {
            record_kind: DEPLOYMENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:deployment-summary-card".to_owned(),
            component_family: M5DeploymentComponentFamily::DeploymentSummaryCard,
            source_family_schema_ref: DEPLOYMENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            operating_context_ref: "install:deployment:0003".to_owned(),
            fallback_modalities: vec![
                M5DeploymentFallbackModality::List,
                M5DeploymentFallbackModality::Textual,
                M5DeploymentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:deployment-summary-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "tenant_region",
                "operating_mode",
                "control_plane",
                "data_plane",
            ]),
            full_interactive_claim: M5DeploymentInteractiveClaim::FullyCurrentManaged,
            claim_conditions: vec![condition(
                M5DeploymentClaimDimension::ControlPlaneFreshness,
                M5DeploymentClaimConditionState::Stale,
            )],
            claim_narrow: Some(ClaimAutoNarrow {
                narrowed_to: M5DeploymentInteractiveClaim::LocalCachedOnly,
                binding_dimension: M5DeploymentClaimDimension::ControlPlaneFreshness,
                trigger: M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
                narrowed_label:
                    "Control plane cached — deployment shown local-cached-only, not live-current"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "tenant_region",
                "operating_mode",
                "control_plane",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DeploymentSurfaceFamily::AboutInstallCard,
                M5DeploymentSurfaceFamily::AdminFleetConsole,
            ]),
            source_refs: vec![
                "UI/UX Spec §5.6".to_owned(),
                DEPLOYMENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("deployment-summary-card"),
        },
        // Residual-dependency row — residual dependency partial (a vendor dependency
        // remains for a self-hosted install), so the fully-self-hosted claim auto-narrows
        // to review-required (yellow).
        DeploymentAccessibilityRow {
            record_kind: DEPLOYMENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:residual-dependency-row".to_owned(),
            component_family: M5DeploymentComponentFamily::ResidualDependencyRow,
            source_family_schema_ref: DEPLOYMENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            operating_context_ref: "install:residual:0004".to_owned(),
            fallback_modalities: vec![
                M5DeploymentFallbackModality::List,
                M5DeploymentFallbackModality::Textual,
                M5DeploymentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::DisclosedReducedButReachable,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:residual-dependency-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "vendor_dependency",
                "dependency_class",
                "required",
                "freshness",
            ]),
            full_interactive_claim: M5DeploymentInteractiveClaim::FullyCurrentManaged,
            claim_conditions: vec![condition(
                M5DeploymentClaimDimension::ResidualDependency,
                M5DeploymentClaimConditionState::Partial,
            )],
            claim_narrow: Some(ClaimAutoNarrow {
                narrowed_to: M5DeploymentInteractiveClaim::ReviewRequired,
                binding_dimension: M5DeploymentClaimDimension::ResidualDependency,
                trigger: M5DeploymentDowngradeTrigger::ResidualVendorDependency,
                narrowed_label:
                    "Residual vendor dependency remains — self-hosted claim gated behind review"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "vendor_dependency",
                "dependency_class",
                "required",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DeploymentSurfaceFamily::AboutInstallCard,
                M5DeploymentSurfaceFamily::AdminFleetConsole,
            ]),
            source_refs: vec![
                "UI/UX Spec §5.6".to_owned(),
                DEPLOYMENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("residual-dependency-row"),
        },
        // Control-plane/data-plane status strip — spatially arranged; control-plane
        // freshness unavailable (the managed control plane is unreachable) while the
        // local runtime is unaffected, so the strip auto-narrows to inspect-only and
        // binds its two-plane layout to a flat list / textual path (yellow).
        DeploymentAccessibilityRow {
            record_kind: DEPLOYMENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:control-data-plane-status-strip".to_owned(),
            component_family: M5DeploymentComponentFamily::ControlPlaneDataPlaneStatusStrip,
            source_family_schema_ref: DEPLOYMENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            operating_context_ref: "install:planes:0005".to_owned(),
            fallback_modalities: vec![
                M5DeploymentFallbackModality::Structured,
                M5DeploymentFallbackModality::List,
                M5DeploymentFallbackModality::Textual,
                M5DeploymentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::DisclosedReducedButReachable,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:control-data-plane-status-strip:a11y".to_owned(),
            copy_export: copy_export(&[
                "control_plane",
                "data_plane",
                "local_runtime",
                "impairment",
            ]),
            full_interactive_claim: M5DeploymentInteractiveClaim::FullyCurrentManaged,
            claim_conditions: vec![condition(
                M5DeploymentClaimDimension::ControlPlaneFreshness,
                M5DeploymentClaimConditionState::Unavailable,
            )],
            claim_narrow: Some(ClaimAutoNarrow {
                narrowed_to: M5DeploymentInteractiveClaim::InspectOnly,
                binding_dimension: M5DeploymentClaimDimension::ControlPlaneFreshness,
                trigger: M5DeploymentDowngradeTrigger::ControlPlaneImpaired,
                narrowed_label:
                    "Control plane unreachable — status inspect-only; local runtime unaffected"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "control_plane",
                "data_plane",
                "local_runtime",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DeploymentSurfaceFamily::AdminFleetConsole,
                M5DeploymentSurfaceFamily::AboutInstallCard,
            ]),
            source_refs: vec![
                "TAD control-plane/data-plane decomposition".to_owned(),
                DEPLOYMENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("control-data-plane-status-strip"),
        },
        // Mirror/offline artifact row — mirror verification stale (the mirror is behind
        // the first-party source), so the artifact auto-narrows to local-cached-only:
        // provenance stays copyable but it is never shown as a current live source
        // (yellow).
        DeploymentAccessibilityRow {
            record_kind: DEPLOYMENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:mirror-offline-artifact-row".to_owned(),
            component_family: M5DeploymentComponentFamily::MirrorOfflineArtifactRow,
            source_family_schema_ref: DEPLOYMENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            operating_context_ref: "install:mirror:0006".to_owned(),
            fallback_modalities: vec![
                M5DeploymentFallbackModality::List,
                M5DeploymentFallbackModality::Textual,
                M5DeploymentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:mirror-offline-artifact-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "mirror_source",
                "freshness",
                "signature_state",
                "artifact_class",
            ]),
            full_interactive_claim: M5DeploymentInteractiveClaim::FullyCurrentManaged,
            claim_conditions: vec![condition(
                M5DeploymentClaimDimension::MirrorVerification,
                M5DeploymentClaimConditionState::Stale,
            )],
            claim_narrow: Some(ClaimAutoNarrow {
                narrowed_to: M5DeploymentInteractiveClaim::LocalCachedOnly,
                binding_dimension: M5DeploymentClaimDimension::MirrorVerification,
                trigger: M5DeploymentDowngradeTrigger::MirrorStale,
                narrowed_label:
                    "Mirror stale — artifact shown mirror-cached, never as a current live source"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "mirror_source",
                "freshness",
                "signature_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DeploymentSurfaceFamily::UpdateCenter,
                M5DeploymentSurfaceFamily::AdminFleetConsole,
            ]),
            source_refs: vec![
                "UI/UX Spec §5.6".to_owned(),
                DEPLOYMENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("mirror-offline-artifact-row"),
        },
        // Mode-change review sheet — state-root integrity intact; the sheet's full claim
        // is review-required by nature (it discloses cache and rollback consequences
        // before a durable boundary change), so it stays green without narrowing.
        DeploymentAccessibilityRow {
            record_kind: DEPLOYMENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:mode-change-review-sheet".to_owned(),
            component_family: M5DeploymentComponentFamily::ModeChangeReviewSheet,
            source_family_schema_ref: DEPLOYMENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            operating_context_ref: "install:mode-change:0007".to_owned(),
            fallback_modalities: vec![
                M5DeploymentFallbackModality::List,
                M5DeploymentFallbackModality::Textual,
                M5DeploymentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:mode-change-review-sheet:a11y".to_owned(),
            copy_export: copy_export(&["from_mode", "to_mode", "boundary_change", "rollback"]),
            full_interactive_claim: M5DeploymentInteractiveClaim::ReviewRequired,
            claim_conditions: vec![condition(
                M5DeploymentClaimDimension::StateRootIntegrity,
                M5DeploymentClaimConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&["from_mode", "to_mode", "boundary_change"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DeploymentSurfaceFamily::SideBySideReview,
                M5DeploymentSurfaceFamily::UpdateCenter,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.14".to_owned(),
                DEPLOYMENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("mode-change-review-sheet"),
        },
        // Side-by-side import sheet — handler ownership partial (an association is
        // contested across installs), so import auto-narrows to review-required: nothing
        // is captured last-writer-wins (yellow).
        DeploymentAccessibilityRow {
            record_kind: DEPLOYMENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:side-by-side-import-sheet".to_owned(),
            component_family: M5DeploymentComponentFamily::SideBySideImportSheet,
            source_family_schema_ref: DEPLOYMENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            operating_context_ref: "install:side-by-side:0008".to_owned(),
            fallback_modalities: vec![
                M5DeploymentFallbackModality::List,
                M5DeploymentFallbackModality::Textual,
                M5DeploymentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:side-by-side-import-sheet:a11y".to_owned(),
            copy_export: copy_export(&[
                "import_source",
                "handler_ownership",
                "isolation",
                "current_owner",
            ]),
            full_interactive_claim: M5DeploymentInteractiveClaim::FullyCurrentManaged,
            claim_conditions: vec![condition(
                M5DeploymentClaimDimension::HandlerOwnership,
                M5DeploymentClaimConditionState::Partial,
            )],
            claim_narrow: Some(ClaimAutoNarrow {
                narrowed_to: M5DeploymentInteractiveClaim::ReviewRequired,
                binding_dimension: M5DeploymentClaimDimension::HandlerOwnership,
                trigger: M5DeploymentDowngradeTrigger::HandlerOwnershipContested,
                narrowed_label:
                    "Handler ownership contested — import gated behind review, no default capture"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "import_source",
                "handler_ownership",
                "isolation",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DeploymentSurfaceFamily::SideBySideReview,
                M5DeploymentSurfaceFamily::AboutInstallCard,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.12".to_owned(),
                DEPLOYMENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("side-by-side-import-sheet"),
        },
        // Channel-association review row — handler ownership policy-blocked (a managed
        // policy pins the default handler), so the association auto-narrows to
        // inspect-only: the current owner is shown but the change affordance is withdrawn
        // (yellow).
        DeploymentAccessibilityRow {
            record_kind: DEPLOYMENT_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: DEPLOYMENT_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:channel-association-review-row".to_owned(),
            component_family: M5DeploymentComponentFamily::ChannelAssociationReviewRow,
            source_family_schema_ref: DEPLOYMENT_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            operating_context_ref: "install:channel-assoc:0009".to_owned(),
            fallback_modalities: vec![
                M5DeploymentFallbackModality::List,
                M5DeploymentFallbackModality::Textual,
                M5DeploymentFallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:channel-association-review-row:a11y".to_owned(),
            copy_export: copy_export(&[
                "channel",
                "handler_association",
                "current_owner",
                "review",
            ]),
            full_interactive_claim: M5DeploymentInteractiveClaim::FullyCurrentManaged,
            claim_conditions: vec![condition(
                M5DeploymentClaimDimension::HandlerOwnership,
                M5DeploymentClaimConditionState::PolicyBlocked,
            )],
            claim_narrow: Some(ClaimAutoNarrow {
                narrowed_to: M5DeploymentInteractiveClaim::InspectOnly,
                binding_dimension: M5DeploymentClaimDimension::HandlerOwnership,
                trigger: M5DeploymentDowngradeTrigger::HandlerOwnershipContested,
                narrowed_label:
                    "Handler pinned by policy — association inspect-only, current owner shown"
                        .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "channel",
                "handler_association",
                "current_owner",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5DeploymentSurfaceFamily::SideBySideReview,
                M5DeploymentSurfaceFamily::AdminFleetConsole,
            ]),
            source_refs: vec![
                "UI/UX Spec §6.12".to_owned(),
                DEPLOYMENT_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("channel-association-review-row"),
        },
    ]
}
