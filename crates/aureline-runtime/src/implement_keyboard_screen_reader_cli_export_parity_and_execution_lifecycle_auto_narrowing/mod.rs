//! Keyboard / screen-reader / CLI / export parity and honest auto-narrowing for
//! the M5 execution-lifecycle components.
//!
//! This module is the M05-826 accessibility-and-auto-narrowing capstone over the
//! frozen M5 execution-lifecycle component matrix
//! ([`crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix`]).
//! Where the freeze matrix defines the reusable run/attempt-header,
//! input-request-prompt, artifact-publish-row, rerun-comparison-sheet,
//! debug-session-header, thread/process-tree, and dump/crash-artifact-card
//! primitives and the 821-824 implementation lanes resolve their per-surface
//! truth, this lane certifies — per component family — that execution-lifecycle
//! claims stay **keyboard-complete, assistive-tech-reachable, CLI/export-safe, and
//! self-narrowing** rather than presenting a stale or partial lane as fully current,
//! fully controllable, or fully attributable:
//!
//! - **Keyboard / screen-reader / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, and CLI/headless-reachable path
//!   into the same run/attempt identity, target boundary, and outcome / artifact /
//!   mapping state the rich surface shows — never a view-only strip that strands
//!   assistive-tech or headless users. Hierarchy-heavy families (the thread /
//!   process tree) additionally bind their tree to a flat list / textual path.
//! - **Export parity.** The support / release export reconstructs each component's
//!   meaning from typed tokens and opaque refs without a screenshot, preserving the
//!   same run/attempt IDs, target boundaries, and artifact / mapping states shown
//!   in-product.
//! - **Honest auto-narrowing.** When attempt lineage, input state, artifact
//!   freshness, mapping quality, or target identity is partial, stale, unavailable,
//!   or policy-blocked, the component's interactive claim auto-narrows to
//!   review-required / read-only / inspect-only, discloses the narrowing with a
//!   precise trigger and binding dimension, and preserves the canonical run/attempt
//!   identity rather than silently dropping it. A component with every dimension
//!   intact must NOT carry a spurious narrowing.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in UI,
//!   docs/help, release packets, and support exports so claim publication and field
//!   triage stay aligned on execution-lifecycle downgrade behavior.
//!
//! Each [`ExecutionAccessibilityRow`] keys on one
//! [`crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::M5ExecutionComponentFamily`]
//! and reuses that frozen family vocabulary plus the frozen
//! [`M5ExecutionRequiredLabel`] and [`M5ExecutionDowngradeTrigger`] and the shared
//! [`M5RunAttemptSurfaceFamily`] rather than minting parallel synonyms, so the
//! certified labels stay byte-identical to the matrix and the sibling primitive
//! packets.
//!
//! The packet is metadata-only: raw run logs, process memory, dump payloads,
//! symbol blobs, credentials, and provider cursors never cross this boundary; the
//! packet carries only typed class tokens, opaque summary / evidence refs,
//! booleans, and redacted labels so support and diagnostics exports can reconstruct
//! exactly what an accessible fallback would have shown without leaking execution
//! state.
//!
//! The boundary schema is
//! [`schemas/ui/m5-execution-lifecycle-accessibility-fallback.schema.json`](../../../../schemas/ui/m5-execution-lifecycle-accessibility-fallback.schema.json).
//! The contract doc is
//! [`docs/run-test-debug/m5_execution_lifecycle_accessibility_fallback.md`](../../../../docs/run-test-debug/m5_execution_lifecycle_accessibility_fallback.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_run_attempt_input_request_artifact_publish_rerun_review_and_debug_hierarchy_component_matrix::{
    M5ExecutionComponentFamily, M5ExecutionDowngradeTrigger, M5ExecutionRequiredLabel,
};
use crate::implement_the_m5_run_attempt_header_and_attempt_selector_primitive::M5RunAttemptSurfaceFamily;

/// Schema version stamped on the M05-826 execution accessibility fallback packet.
pub const EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ExecutionAccessibilityPacket`].
pub const EXECUTION_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_execution_lifecycle_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`ExecutionAccessibilityRow`].
pub const EXECUTION_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_execution_lifecycle_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const EXECUTION_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-execution-lifecycle-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const EXECUTION_A11Y_FALLBACK_DOC_REF: &str =
    "docs/run-test-debug/m5_execution_lifecycle_accessibility_fallback.md";

/// Repo-relative path of the frozen execution-lifecycle component matrix this lane
/// certifies.
pub const EXECUTION_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-execution-lifecycle-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const EXECUTION_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-execution-lifecycle-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const EXECUTION_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-execution-lifecycle-accessibility-fallback-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const EXECUTION_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-execution-lifecycle-accessibility-fallback-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const EXECUTION_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-execution-lifecycle-accessibility-fallback-proof/report.md";

/// The reusable component families that render a non-linear hierarchy (a thread /
/// process tree) and therefore MUST bind their tree to an equivalent flat list /
/// textual path so the hierarchy is navigable non-visually.
const fn family_is_hierarchy_heavy(family: M5ExecutionComponentFamily) -> bool {
    matches!(family, M5ExecutionComponentFamily::ThreadProcessTree)
}

/// The execution-lifecycle dimension whose weakening a family primarily discloses.
/// Every row must model at least this dimension so its key weakening axis is
/// covered.
const fn family_primary_dimension(
    family: M5ExecutionComponentFamily,
) -> M5ExecutionClaimDimension {
    match family {
        M5ExecutionComponentFamily::RunAttemptHeader
        | M5ExecutionComponentFamily::RerunComparisonSheet => {
            M5ExecutionClaimDimension::AttemptLineage
        }
        M5ExecutionComponentFamily::InputRequestPrompt => M5ExecutionClaimDimension::InputState,
        M5ExecutionComponentFamily::ArtifactPublishRow => {
            M5ExecutionClaimDimension::ArtifactFreshness
        }
        M5ExecutionComponentFamily::DebugSessionHeader
        | M5ExecutionComponentFamily::ThreadProcessTree => {
            M5ExecutionClaimDimension::TargetIdentity
        }
        M5ExecutionComponentFamily::DumpCrashArtifactCard => {
            M5ExecutionClaimDimension::MappingQuality
        }
    }
}

/// A rendered fallback modality for an execution-lifecycle component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5FallbackModality {
    /// A rich, structured (tree / graph) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / source-first projection.
    Textual,
    /// A CLI / headless line projection.
    Cli,
}

impl M5FallbackModality {
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

/// A rendering-surface capability tier. Distinct from the semantic consumer
/// surface: the same component may render at desktop-full capability or narrow to
/// a companion, read-only browser, headless CLI, handoff packet, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionRenderingSurface {
    /// The full-capability desktop runtime surface.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A headless CLI surface.
    CliHeadless,
    /// A handoff packet.
    HandoffPacket,
    /// A support export.
    SupportExport,
}

impl M5ExecutionRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop
    /// full-capability baseline and therefore must disclose its reduction.
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

/// The interactive-claim ceiling a component asserts: how much control it lets a
/// user exert. Auto-narrowing lowers this ceiling when an execution dimension
/// weakens so a stale or partial lane can never present as fully controllable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionInteractiveClaim {
    /// Full live control: dispatch, answer input, continue / pause, re-open live.
    FullInteractive,
    /// Action is allowed but gated behind an explicit review step before dispatch.
    ReviewRequired,
    /// The component is read-only: copy / export is allowed, action is not.
    ReadOnly,
    /// The component is inspect-only: captured evidence may be viewed / navigated,
    /// nothing may be acted on or copied as a live handle.
    InspectOnly,
}

impl M5ExecutionInteractiveClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 4] = [
        Self::FullInteractive,
        Self::ReviewRequired,
        Self::ReadOnly,
        Self::InspectOnly,
    ];

    /// Capability rank; a higher rank asserts more control. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::FullInteractive => 3,
            Self::ReviewRequired => 2,
            Self::ReadOnly => 1,
            Self::InspectOnly => 0,
        }
    }

    /// Returns true when this claim asserts an action affordance (dispatch / answer
    /// / continue) rather than a view-only posture.
    pub const fn asserts_control(self) -> bool {
        matches!(self, Self::FullInteractive | Self::ReviewRequired)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullInteractive => "full_interactive",
            Self::ReviewRequired => "review_required",
            Self::ReadOnly => "read_only",
            Self::InspectOnly => "inspect_only",
        }
    }
}

/// The execution-lifecycle dimension whose state governs how far a component may
/// claim to be current, controllable, or attributable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ExecutionClaimDimension {
    /// Run / attempt lineage: is the attempt identity fully resolved?
    AttemptLineage,
    /// Input state: is an outstanding input request answerable and its consequence
    /// known?
    InputState,
    /// Artifact freshness: is a produced artifact current and its retention intact?
    ArtifactFreshness,
    /// Mapping / symbolication quality: does captured evidence map cleanly to source?
    MappingQuality,
    /// Target identity: is the run / debug target boundary fully resolved and
    /// reachable?
    TargetIdentity,
}

impl M5ExecutionClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::AttemptLineage,
        Self::InputState,
        Self::ArtifactFreshness,
        Self::MappingQuality,
        Self::TargetIdentity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AttemptLineage => "attempt_lineage",
            Self::InputState => "input_state",
            Self::ArtifactFreshness => "artifact_freshness",
            Self::MappingQuality => "mapping_quality",
            Self::TargetIdentity => "target_identity",
        }
    }

    /// The frozen downgrade trigger this dimension binds to when it weakens.
    pub const fn default_trigger(self) -> M5ExecutionDowngradeTrigger {
        match self {
            Self::AttemptLineage => M5ExecutionDowngradeTrigger::RunAttemptIdentityUnresolved,
            Self::InputState => M5ExecutionDowngradeTrigger::InputConsequenceUnknown,
            Self::ArtifactFreshness => M5ExecutionDowngradeTrigger::ArtifactRetentionExpired,
            Self::MappingQuality => M5ExecutionDowngradeTrigger::SymbolsUnavailable,
            Self::TargetIdentity => M5ExecutionDowngradeTrigger::ConnectorLost,
        }
    }
}

/// The strength state of one execution-lifecycle dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ClaimConditionState {
    /// Fully resolved / current / attributable — imposes no ceiling.
    Intact,
    /// Partially resolved — action allowed only behind explicit review.
    Partial,
    /// Stale — superseded by a newer run / source; read-only.
    Stale,
    /// Unavailable — the live handle / connector is gone; inspect-only.
    Unavailable,
    /// Policy-blocked — action is denied by policy; inspect-only.
    PolicyBlocked,
}

impl M5ClaimConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::Intact,
        Self::Partial,
        Self::Stale,
        Self::Unavailable,
        Self::PolicyBlocked,
    ];

    /// Returns true when the dimension is weaker than intact and therefore imposes
    /// a narrowing ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::Intact)
    }

    /// The strongest interactive claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5ExecutionInteractiveClaim {
        match self {
            Self::Intact => M5ExecutionInteractiveClaim::FullInteractive,
            Self::Partial => M5ExecutionInteractiveClaim::ReviewRequired,
            Self::Stale => M5ExecutionInteractiveClaim::ReadOnly,
            Self::Unavailable | Self::PolicyBlocked => M5ExecutionInteractiveClaim::InspectOnly,
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

/// One execution-lifecycle dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5ExecutionClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5ClaimConditionState,
}

/// An honest interactive-claim auto-narrow block. When an execution dimension
/// weakens, the component's interactive claim lowers to the permitted ceiling, names
/// the binding dimension and frozen trigger, and preserves the canonical run/attempt
/// identity rather than silently dropping it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimAutoNarrow {
    /// The interactive claim the component is narrowed to.
    pub narrowed_to: M5ExecutionInteractiveClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the
    /// strongest ceiling constraint).
    pub binding_dimension: M5ExecutionClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5ExecutionDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical run/attempt identity, target boundary, and artifact / mapping
    /// state are preserved rather than dropped; must hold.
    pub preserves_canonical_identity: bool,
}

impl ClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and
    /// carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must
/// be copyable as text / JSON / Markdown, and a screenshot is never the only export.
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
    /// offered, at least one export field is named, and screenshots are prohibited
    /// as the sole export.
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
    pub rendering_surface: M5ExecutionRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: NarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for an execution accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ExecutionAccessibilityStatus {
    /// Full keyboard / screen-reader / CLI / export parity with no narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a screenshot, over-claims control, or drops
    /// state silently (red).
    Stranded,
}

impl ExecutionAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one execution-lifecycle component
/// family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionAccessibilityRow {
    /// Record kind; must equal [`EXECUTION_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5ExecutionComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the run / attempt / execution context this component acts on;
    /// stays visible on every surface, so this is never empty.
    pub execution_context_ref: String,
    /// Rendered modalities offered; a hierarchy-heavy family must also offer a
    /// non-visual (list / textual / cli) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5FallbackModality>,
    /// The non-visual / CLI path reaches the same canonical run/attempt identity,
    /// target boundary, and outcome / artifact / mapping state as the rich surface;
    /// must hold.
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
    /// The full interactive claim this family asserts when every dimension is
    /// intact.
    pub full_interactive_claim: M5ExecutionInteractiveClaim,
    /// The observed condition of each modeled execution dimension.
    #[serde(default)]
    pub claim_conditions: Vec<ClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below
    /// the family's full claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<ClaimAutoNarrow>,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5ExecutionRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<RenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ExecutionRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5RunAttemptSurfaceFamily>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ExecutionAccessibilityRow {
    /// Returns true when this family renders a non-linear hierarchy and must bind to
    /// a flat non-visual path.
    pub const fn is_hierarchy_heavy(&self) -> bool {
        family_is_hierarchy_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / cli) fallback
    /// modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `Intact` when the row does
    /// not model that dimension.
    pub fn condition_for(&self, dimension: M5ExecutionClaimDimension) -> M5ClaimConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5ClaimConditionState::Intact)
    }

    /// Whether any modeled dimension is weaker than intact.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest interactive claim permitted after applying every modeled
    /// dimension's ceiling, capped at the family's full claim.
    pub fn permitted_claim(&self) -> M5ExecutionInteractiveClaim {
        let mut permitted = self.full_interactive_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The dimension imposing the strongest (lowest-rank) ceiling, if any weak
    /// dimension narrows below the family's full claim.
    pub fn binding_dimension(&self) -> Option<M5ExecutionClaimDimension> {
        let mut binding: Option<(M5ExecutionClaimDimension, u8)> = None;
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
    pub fn effective_claim(&self) -> M5ExecutionInteractiveClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_interactive_claim,
        }
    }

    /// AC1: a stale or partial lane can no longer present as fully controllable.
    /// The effective claim never exceeds the permitted ceiling; when a dimension
    /// narrows below the full claim, an honest narrow block is present, narrows to
    /// exactly the permitted ceiling, binds to the ceiling-imposing dimension with
    /// its frozen trigger, and preserves canonical identity. When nothing narrows,
    /// no spurious narrow block is present.
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
    /// keyboard / screen-reader / CLI trap, a hierarchy-heavy family offers a
    /// non-visual fallback, and the export reconstructs meaning without a screenshot.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.execution_context_ref.trim().is_empty()
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
    /// keeps its labels, so claim publication and field triage stay aligned on the
    /// same narrowed state.
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
    pub fn status(&self) -> ExecutionAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
        {
            return ExecutionAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            ExecutionAccessibilityStatus::NarrowedDisclosed
        } else {
            ExecutionAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == EXECUTION_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.execution_context_ref.trim().is_empty()
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

/// Rolled-up summary of an M05-826 execution accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionAccessibilitySummary {
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

/// Constructor input for [`ExecutionAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExecutionAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<ExecutionAccessibilityRow>,
}

/// Checked-in M05-826 execution accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ExecutionAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<ExecutionAccessibilityRow>,
    pub summary: ExecutionAccessibilitySummary,
}

impl ExecutionAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: ExecutionAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: EXECUTION_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: ExecutionAccessibilitySummary {
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
    pub fn represented_families(&self) -> BTreeSet<M5ExecutionComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5ExecutionClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Interactive claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5ExecutionInteractiveClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ExecutionAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let hierarchy_heavy: Vec<&ExecutionAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_hierarchy_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                ExecutionAccessibilityStatus::Parity => green += 1,
                ExecutionAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                ExecutionAccessibilityStatus::Stranded => red += 1,
            }
        }

        ExecutionAccessibilitySummary {
            family_count: self.rows.len(),
            hierarchy_heavy_family_count: hierarchy_heavy.len(),
            all_hierarchy_heavy_have_non_visual_fallback: hierarchy_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(ExecutionAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(ExecutionAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(ExecutionAccessibilityRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(ExecutionAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ExecutionAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(ExecutionAccessibilityViolation::SchemaVersion {
                expected: EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EXECUTION_A11Y_FALLBACK_RECORD_KIND {
            violations.push(ExecutionAccessibilityViolation::RecordKind {
                expected: EXECUTION_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ExecutionAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ExecutionAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(ExecutionAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(ExecutionAccessibilityViolation::MissingPrimaryDimension {
                    id: row.row_id.clone(),
                    dimension: family_primary_dimension(row.component_family),
                });
            }

            // A hierarchy-heavy family must render a structured tree *and* a
            // non-visual path.
            if row.is_hierarchy_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5FallbackModality::Structured)
            {
                violations.push(ExecutionAccessibilityViolation::HierarchyHeavyMissingStructured {
                    id: row.row_id.clone(),
                });
            }

            // AC1: claim never over-asserts control for a weakened lane.
            if !row.claim_is_honest() {
                violations.push(ExecutionAccessibilityViolation::ClaimOverAsserted {
                    id: row.row_id.clone(),
                });
            }

            // AC2: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(ExecutionAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // Export preserves meaning without a screenshot.
            if !row.export_preserves_meaning() {
                violations.push(ExecutionAccessibilityViolation::ExportRequiresScreenshot {
                    id: row.row_id.clone(),
                });
            }

            // AC3: narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    ExecutionAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(ExecutionAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == ExecutionAccessibilityStatus::Stranded {
                violations.push(ExecutionAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5ExecutionComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations
                    .push(ExecutionAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5ExecutionClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations
                    .push(ExecutionAccessibilityViolation::MissingDimensionCoverage { dimension });
            }
        }

        // Coverage: every interactive claim tier appears as an effective claim, so
        // the full narrowing spectrum (full → review → read-only → inspect-only) is
        // proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5ExecutionInteractiveClaim::ALL {
            if !effective.contains(&claim) {
                violations
                    .push(ExecutionAccessibilityViolation::MissingClaimTierCoverage { claim });
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(ExecutionAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("execution accessibility fallback packet serializes"),
        ) {
            violations.push(ExecutionAccessibilityViolation::RawBoundaryMaterialInExport);
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
            .expect("execution accessibility fallback packet serializes")
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
        out.push_str("# M5 Execution-Lifecycle Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5ExecutionComponentFamily::ALL.len(),
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

/// Reads and validates the checked-in execution accessibility fallback export.
pub fn current_m5_execution_a11y_fallback_export(
) -> Result<ExecutionAccessibilityPacket, ExecutionAccessibilityArtifactError> {
    let packet: ExecutionAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-execution-lifecycle-accessibility-fallback-proof/support_export.json"
    )))
    .map_err(ExecutionAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ExecutionAccessibilityArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in execution accessibility fallback
/// export.
#[derive(Debug)]
pub enum ExecutionAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ExecutionAccessibilityViolation>),
}

impl fmt::Display for ExecutionAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "execution accessibility fallback export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "execution accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ExecutionAccessibilityArtifactError {}

/// Validation failure for M05-826 execution accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionAccessibilityViolation {
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
        dimension: M5ExecutionClaimDimension,
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
        family: M5ExecutionComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5ExecutionClaimDimension,
    },
    MissingClaimTierCoverage {
        claim: M5ExecutionInteractiveClaim,
    },
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for ExecutionAccessibilityViolation {
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
            Self::HierarchyHeavyMissingStructured { id } => {
                write!(
                    f,
                    "hierarchy-heavy row {id} does not render a structured modality"
                )
            }
            Self::ClaimOverAsserted { id } => {
                write!(
                    f,
                    "row {id} over-asserts interactive control for a weakened lane, or narrows spuriously"
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
                write!(f, "row {id} narrows a rendering surface without disclosing it")
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

impl Error for ExecutionAccessibilityViolation {}

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

/// Builds the canonical, checked-in execution accessibility fallback packet. This
/// is the one source of truth shared by the tests, the example dump, and the
/// on-disk support export so all three stay byte-aligned.
pub fn seeded_m5_execution_a11y_fallback_packet() -> ExecutionAccessibilityPacket {
    ExecutionAccessibilityPacket::new(ExecutionAccessibilityPacketInput {
        packet_id: "m5-execution-lifecycle-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-04T00:00:00Z".to_owned(),
        matrix_ref: EXECUTION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:execution-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5ExecutionRequiredLabel> {
    M5ExecutionRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

fn condition(
    dimension: M5ExecutionClaimDimension,
    state: M5ClaimConditionState,
) -> ClaimConditionEntry {
    ClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum, plus a support/export
/// surface so the narrowed state reaches field triage.
fn base_consumers(extra: &[M5RunAttemptSurfaceFamily]) -> Vec<M5RunAttemptSurfaceFamily> {
    let mut out = vec![
        M5RunAttemptSurfaceFamily::HistoryActivityCenter,
        M5RunAttemptSurfaceFamily::SupportExportReplay,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full
/// parity) row keeps full label and summary parity on the narrower surfaces; a
/// narrowed row discloses the reduced interactions it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: NarrowingDisclosureState,
) -> Vec<RenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        RenderingNarrowingDisclosure {
            rendering_surface: M5ExecutionRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        RenderingNarrowingDisclosure {
            rendering_surface: M5ExecutionRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_action".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full
/// label and summary parity.
fn parity_surfaces(labels: &[&str]) -> Vec<RenderingNarrowingDisclosure> {
    surface_disclosures(labels, NarrowingDisclosureState::ParityPreserved)
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their
/// reduced interactions while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<RenderingNarrowingDisclosure> {
    surface_disclosures(labels, NarrowingDisclosureState::DisclosedNarrowed)
}

fn rendering_surfaces() -> Vec<M5ExecutionRenderingSurface> {
    vec![
        M5ExecutionRenderingSurface::DesktopFull,
        M5ExecutionRenderingSurface::CliHeadless,
        M5ExecutionRenderingSurface::SupportExport,
    ]
}

fn seeded_rows() -> Vec<ExecutionAccessibilityRow> {
    vec![
        // Run/attempt header — attempt lineage intact; full attempt-selection
        // control, fully reachable on every surface (green).
        ExecutionAccessibilityRow {
            record_kind: EXECUTION_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:run-attempt-header".to_owned(),
            component_family: M5ExecutionComponentFamily::RunAttemptHeader,
            source_family_schema_ref: EXECUTION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            execution_context_ref: "run:attempt:0001".to_owned(),
            fallback_modalities: vec![
                M5FallbackModality::Structured,
                M5FallbackModality::List,
                M5FallbackModality::Textual,
                M5FallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:run-attempt-header:a11y".to_owned(),
            copy_export: copy_export(&["run_id", "attempt_id", "outcome", "target_ref"]),
            full_interactive_claim: M5ExecutionInteractiveClaim::FullInteractive,
            claim_conditions: vec![condition(
                M5ExecutionClaimDimension::AttemptLineage,
                M5ClaimConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&["run_id", "attempt_id", "outcome"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5RunAttemptSurfaceFamily::TaskRunPane,
                M5RunAttemptSurfaceFamily::TestRunPane,
            ]),
            source_refs: vec![
                "UI/UX Spec §14.4".to_owned(),
                EXECUTION_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("run-attempt-header"),
        },
        // Input-request prompt — the approval is policy-blocked, so the prompt
        // auto-narrows to inspect-only: the consequence is shown but the answer
        // affordance is withdrawn (yellow).
        ExecutionAccessibilityRow {
            record_kind: EXECUTION_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:input-request-prompt".to_owned(),
            component_family: M5ExecutionComponentFamily::InputRequestPrompt,
            source_family_schema_ref: EXECUTION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            execution_context_ref: "run:attempt:0002".to_owned(),
            fallback_modalities: vec![
                M5FallbackModality::List,
                M5FallbackModality::Textual,
                M5FallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::DisclosedReducedButReachable,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:input-request-prompt:a11y".to_owned(),
            copy_export: copy_export(&["run_id", "prompt_kind", "consequence", "deadline"]),
            full_interactive_claim: M5ExecutionInteractiveClaim::FullInteractive,
            claim_conditions: vec![condition(
                M5ExecutionClaimDimension::InputState,
                M5ClaimConditionState::PolicyBlocked,
            )],
            claim_narrow: Some(ClaimAutoNarrow {
                narrowed_to: M5ExecutionInteractiveClaim::InspectOnly,
                binding_dimension: M5ExecutionClaimDimension::InputState,
                trigger: M5ExecutionDowngradeTrigger::InputConsequenceUnknown,
                narrowed_label: "Approval blocked by policy — prompt is inspect-only".to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["run_id", "prompt_kind", "consequence"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5RunAttemptSurfaceFamily::TaskRunPane,
                M5RunAttemptSurfaceFamily::AiMediatedExecution,
            ]),
            source_refs: vec![
                "UI/UX Spec §14.4".to_owned(),
                EXECUTION_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("input-request-prompt"),
        },
        // Artifact-publish row — retention expired, so the produced artifact
        // auto-narrows to read-only: its lineage and metadata remain copyable but it
        // can no longer be re-opened live (yellow).
        ExecutionAccessibilityRow {
            record_kind: EXECUTION_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:artifact-publish-row".to_owned(),
            component_family: M5ExecutionComponentFamily::ArtifactPublishRow,
            source_family_schema_ref: EXECUTION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            execution_context_ref: "run:attempt:0003".to_owned(),
            fallback_modalities: vec![
                M5FallbackModality::List,
                M5FallbackModality::Textual,
                M5FallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:artifact-publish-row:a11y".to_owned(),
            copy_export: copy_export(&["artifact_id", "producing_run_ref", "retention", "freshness"]),
            full_interactive_claim: M5ExecutionInteractiveClaim::FullInteractive,
            claim_conditions: vec![condition(
                M5ExecutionClaimDimension::ArtifactFreshness,
                M5ClaimConditionState::Stale,
            )],
            claim_narrow: Some(ClaimAutoNarrow {
                narrowed_to: M5ExecutionInteractiveClaim::ReadOnly,
                binding_dimension: M5ExecutionClaimDimension::ArtifactFreshness,
                trigger: M5ExecutionDowngradeTrigger::ArtifactRetentionExpired,
                narrowed_label: "Artifact retention expired — lineage copyable, re-open disabled"
                    .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "artifact_id",
                "producing_run_ref",
                "retention",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5RunAttemptSurfaceFamily::PublishFlow,
                M5RunAttemptSurfaceFamily::PreviewFlow,
            ]),
            source_refs: vec![
                "TDD §8.32".to_owned(),
                EXECUTION_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("artifact-publish-row"),
        },
        // Rerun comparison sheet — attempt lineage intact; the sheet's full claim is
        // review-required by nature (it discloses exact-vs-current-context diffs
        // before dispatch), so it stays green without narrowing.
        ExecutionAccessibilityRow {
            record_kind: EXECUTION_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:rerun-comparison-sheet".to_owned(),
            component_family: M5ExecutionComponentFamily::RerunComparisonSheet,
            source_family_schema_ref: EXECUTION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            execution_context_ref: "run:attempt:0004".to_owned(),
            fallback_modalities: vec![
                M5FallbackModality::List,
                M5FallbackModality::Textual,
                M5FallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:rerun-comparison-sheet:a11y".to_owned(),
            copy_export: copy_export(&["prior_attempt_ref", "rerun_mode", "context_diff"]),
            full_interactive_claim: M5ExecutionInteractiveClaim::ReviewRequired,
            claim_conditions: vec![condition(
                M5ExecutionClaimDimension::AttemptLineage,
                M5ClaimConditionState::Intact,
            )],
            claim_narrow: None,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "prior_attempt_ref",
                "rerun_mode",
                "context_diff",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5RunAttemptSurfaceFamily::TaskRunPane,
                M5RunAttemptSurfaceFamily::TestRunPane,
            ]),
            source_refs: vec![
                "TDD §9.20".to_owned(),
                EXECUTION_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("rerun-comparison-sheet"),
        },
        // Debug session header — target identity partial (the attach target is
        // ambiguous), so live control auto-narrows to review-required (yellow).
        ExecutionAccessibilityRow {
            record_kind: EXECUTION_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:debug-session-header".to_owned(),
            component_family: M5ExecutionComponentFamily::DebugSessionHeader,
            source_family_schema_ref: EXECUTION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            execution_context_ref: "run:attempt:0005".to_owned(),
            fallback_modalities: vec![
                M5FallbackModality::List,
                M5FallbackModality::Textual,
                M5FallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:debug-session-header:a11y".to_owned(),
            copy_export: copy_export(&["session_id", "session_mode", "target_ref", "truth_class"]),
            full_interactive_claim: M5ExecutionInteractiveClaim::FullInteractive,
            claim_conditions: vec![condition(
                M5ExecutionClaimDimension::TargetIdentity,
                M5ClaimConditionState::Partial,
            )],
            claim_narrow: Some(ClaimAutoNarrow {
                narrowed_to: M5ExecutionInteractiveClaim::ReviewRequired,
                binding_dimension: M5ExecutionClaimDimension::TargetIdentity,
                trigger: M5ExecutionDowngradeTrigger::ConnectorLost,
                narrowed_label: "Attach target ambiguous — control gated behind target review"
                    .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["session_id", "session_mode", "target_ref"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5RunAttemptSurfaceFamily::TaskRunPane,
                M5RunAttemptSurfaceFamily::CompanionSummary,
            ]),
            source_refs: vec![
                "UI/UX Spec §14.5".to_owned(),
                EXECUTION_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("debug-session-header"),
        },
        // Thread / process tree — hierarchy-heavy; the live connector is gone, so
        // the tree auto-narrows to inspect-only captured evidence and binds its tree
        // to a flat list / textual path (yellow).
        ExecutionAccessibilityRow {
            record_kind: EXECUTION_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:thread-process-tree".to_owned(),
            component_family: M5ExecutionComponentFamily::ThreadProcessTree,
            source_family_schema_ref: EXECUTION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            execution_context_ref: "run:attempt:0006".to_owned(),
            fallback_modalities: vec![
                M5FallbackModality::Structured,
                M5FallbackModality::List,
                M5FallbackModality::Textual,
                M5FallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::DisclosedReducedButReachable,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:thread-process-tree:a11y".to_owned(),
            copy_export: copy_export(&["session_id", "node_id", "parent_ref", "run_state"]),
            full_interactive_claim: M5ExecutionInteractiveClaim::FullInteractive,
            claim_conditions: vec![condition(
                M5ExecutionClaimDimension::TargetIdentity,
                M5ClaimConditionState::Unavailable,
            )],
            claim_narrow: Some(ClaimAutoNarrow {
                narrowed_to: M5ExecutionInteractiveClaim::InspectOnly,
                binding_dimension: M5ExecutionClaimDimension::TargetIdentity,
                trigger: M5ExecutionDowngradeTrigger::ConnectorLost,
                narrowed_label: "Live connector lost — tree is captured, inspect-only".to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&["session_id", "node_id", "parent_ref"]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5RunAttemptSurfaceFamily::TaskRunPane,
                M5RunAttemptSurfaceFamily::CompanionSummary,
            ]),
            source_refs: vec![
                "UI/UX Spec §14.5".to_owned(),
                EXECUTION_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("thread-process-tree"),
        },
        // Dump / crash artifact card — captured by nature (full claim is read-only);
        // symbols are unavailable, so mapping quality narrows it to inspect-only
        // (yellow). It never implies live control.
        ExecutionAccessibilityRow {
            record_kind: EXECUTION_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: EXECUTION_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:dump-crash-artifact-card".to_owned(),
            component_family: M5ExecutionComponentFamily::DumpCrashArtifactCard,
            source_family_schema_ref: EXECUTION_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            execution_context_ref: "run:attempt:0007".to_owned(),
            fallback_modalities: vec![
                M5FallbackModality::List,
                M5FallbackModality::Textual,
                M5FallbackModality::Cli,
            ],
            reaches_canonical_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:dump-crash-artifact-card:a11y".to_owned(),
            copy_export: copy_export(&[
                "dump_ref",
                "producing_run_ref",
                "symbolication",
                "capture_time",
            ]),
            full_interactive_claim: M5ExecutionInteractiveClaim::ReadOnly,
            claim_conditions: vec![condition(
                M5ExecutionClaimDimension::MappingQuality,
                M5ClaimConditionState::Unavailable,
            )],
            claim_narrow: Some(ClaimAutoNarrow {
                narrowed_to: M5ExecutionInteractiveClaim::InspectOnly,
                binding_dimension: M5ExecutionClaimDimension::MappingQuality,
                trigger: M5ExecutionDowngradeTrigger::SymbolsUnavailable,
                narrowed_label: "Symbols unavailable — frames shown unsymbolicated, inspect-only"
                    .to_owned(),
                preserves_canonical_identity: true,
            }),
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "dump_ref",
                "producing_run_ref",
                "symbolication",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5RunAttemptSurfaceFamily::TaskRunPane,
                M5RunAttemptSurfaceFamily::AiMediatedExecution,
            ]),
            source_refs: vec![
                "TAD crash capture / symbolication".to_owned(),
                EXECUTION_A11Y_FALLBACK_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("dump-crash-artifact-card"),
        },
    ]
}
