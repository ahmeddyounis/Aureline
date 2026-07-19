//! Keyboard, screen-reader, and CLI/export parity plus honest auto-narrowing for
//! the frozen M5 manifest / build-confidence components.
//!
//! This module is the M05-818 accessibility-hardening capstone over the frozen
//! M5 manifest / build component matrix
//! ([`crate::freeze_the_m5_manifest_editor_schema_validator_resource_link_build_adapter_target_graph_and_fallback_confidence_component_matrix`]).
//! Where the freeze matrix defines the reusable manifest-header / schema-validator
//! / target-context-chip / resource-link / resource-explorer / adapter-badge /
//! target-graph / capability-matrix / raw-event / fallback-confidence primitives
//! and the 813-816 implementation lanes resolve their per-target truth, this lane
//! certifies — per component family — that a manifest / build lane stays
//! **inspectable and honestly narrowed rather than falsely executable** whenever
//! schema freshness, connector state, adapter capability, or target-graph truth is
//! partial, stale, or policy-blocked:
//!
//! - **Keyboard / screen-reader / CLI reach.** Target-context chips, resource rows,
//!   adapter badges, target-graph rows, and fallback-confidence drawers are each
//!   keyboard-complete, screen-reader-reachable, and reachable from a headless CLI /
//!   export surface — never a view-only graph or matrix that strands assistive-tech
//!   or headless users.
//! - **List / table / textual fallback for visual-heavy families.** The target-graph
//!   row and capability matrix bind their visual surface to an equivalent list /
//!   table / textual path so a user is never trapped in a graph-only workflow.
//! - **Auto-narrowed interactive claims.** When schema freshness, connector state,
//!   adapter capability, or target-graph truth weakens, an interactive claim
//!   auto-narrows to review-required / read-only / inspect-only and can never
//!   present as fully executable or fully current.
//! - **Export preserves target identity and confidence.** The CLI / support / release
//!   export carries the same target IDs, target-context refs, schema-freshness,
//!   adapter-source, and confidence states shown in-product, and never relies on a
//!   screenshot to carry meaning.
//! - **Honest auto-narrowing.** A narrowed component discloses why with a precise
//!   frozen downgrade trigger and preserves the key target context rather than
//!   silently dropping it, and the same narrowed state surfaces in UI, docs / help,
//!   release packets, and support exports.
//!
//! Each [`ComponentAccessibilityRow`] keys on one
//! [`crate::M5ManifestBuildComponentFamily`] and reuses that frozen family
//! vocabulary plus [`crate::M5ManifestBuildRequiredLabel`],
//! [`crate::M5ManifestBuildDowngradeTrigger`], [`crate::M5SchemaFreshness`],
//! [`crate::M5AdapterSourceKind`], and [`crate::M5DiscoveryConfidence`] rather than
//! minting parallel synonyms, so the certified labels and confidence states stay
//! byte-identical to the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw manifests, adapter payloads, credentials, and
//! provider bodies never cross this boundary; the packet carries only typed class
//! tokens, opaque summary / evidence refs, booleans, target IDs, and redacted
//! labels so support and diagnostics exports can reconstruct exactly what an
//! accessible, honestly-narrowed fallback would have shown without leaking source.
//!
//! The boundary schema is
//! [`schemas/ui/m5-manifest-build-component-accessibility-fallback.schema.json`](../../../../schemas/ui/m5-manifest-build-component-accessibility-fallback.schema.json).
//! The contract doc is
//! [`docs/infra/m5_manifest_build_component_accessibility_fallback_contract.md`](../../../../docs/infra/m5_manifest_build_component_accessibility_fallback_contract.md).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    M5AdapterSourceKind, M5DiscoveryConfidence, M5ManifestBuildComponentFamily,
    M5ManifestBuildDowngradeTrigger, M5ManifestBuildRequiredLabel, M5SchemaFreshness,
};

/// Schema version stamped on the M05-818 accessibility fallback packet.
pub const MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`ComponentAccessibilityPacket`].
pub const MANIFEST_BUILD_A11Y_FALLBACK_RECORD_KIND: &str =
    "m5_manifest_build_component_accessibility_fallback_packet";

/// Stable record-kind tag carried by each [`ComponentAccessibilityRow`].
pub const MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND: &str =
    "m5_manifest_build_component_accessibility_fallback_row";

/// Repo-relative path of the boundary schema.
pub const MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_REF: &str =
    "schemas/ui/m5-manifest-build-component-accessibility-fallback.schema.json";

/// Repo-relative path of the contract doc.
pub const MANIFEST_BUILD_A11Y_FALLBACK_DOC_REF: &str =
    "docs/infra/m5_manifest_build_component_accessibility_fallback_contract.md";

/// Repo-relative path of the frozen manifest / build component matrix this lane
/// certifies.
pub const MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF: &str =
    "schemas/ui/m5-manifest-build-component-matrix.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const MANIFEST_BUILD_A11Y_FALLBACK_FIXTURE_DIR: &str =
    "fixtures/ui/m5-manifest-build-component-accessibility-fallback";

/// Repo-relative path of the checked support-export artifact (the `include_str!`
/// canonical).
pub const MANIFEST_BUILD_A11Y_FALLBACK_ARTIFACT_REF: &str =
    "artifacts/release/m5-manifest-build-component-accessibility-fallback-proof/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const MANIFEST_BUILD_A11Y_FALLBACK_CSV_REF: &str =
    "artifacts/release/m5-manifest-build-component-accessibility-fallback-proof/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const MANIFEST_BUILD_A11Y_FALLBACK_REPORT_REF: &str =
    "artifacts/release/m5-manifest-build-component-accessibility-fallback-proof/report.md";

/// The reusable component families that render a target graph / capability matrix
/// and therefore MUST bind to an equivalent non-visual (list / table / textual)
/// path.
const fn family_is_visual_heavy(family: M5ManifestBuildComponentFamily) -> bool {
    matches!(
        family,
        M5ManifestBuildComponentFamily::TargetGraphRow
            | M5ManifestBuildComponentFamily::CapabilityMatrix
    )
}

/// The reusable component families that expose an interactive / executable
/// affordance (apply, open-live, run / build / test, invoke-capability) and
/// therefore MUST auto-narrow their claim when truth weakens rather than present as
/// fully executable.
const fn family_is_actionable(family: M5ManifestBuildComponentFamily) -> bool {
    matches!(
        family,
        M5ManifestBuildComponentFamily::ManifestEditorHeader
            | M5ManifestBuildComponentFamily::ResourceLinkRow
            | M5ManifestBuildComponentFamily::ResourceExplorerRow
            | M5ManifestBuildComponentFamily::TargetGraphRow
            | M5ManifestBuildComponentFamily::CapabilityMatrix
    )
}

/// A rendered fallback modality for a component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestBuildFallbackModality {
    /// The visual target-graph / capability-matrix surface.
    Graph,
    /// A flat list projection.
    List,
    /// A tabular projection.
    Table,
    /// A textual / CLI-first projection.
    Textual,
}

impl M5ManifestBuildFallbackModality {
    /// Returns true when the modality is reachable without interpreting a visual
    /// graph / matrix (i.e. a keyboard / screen-reader / CLI path).
    pub const fn is_non_visual(self) -> bool {
        matches!(self, Self::List | Self::Table | Self::Textual)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Graph => "graph",
            Self::List => "list",
            Self::Table => "table",
            Self::Textual => "textual",
        }
    }
}

/// A rendering-surface capability tier. Distinct from the semantic consumer
/// surface: the same component may render at desktop-full capability or narrow to a
/// companion, read-only browser, handoff packet, headless CLI, or support export.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestBuildRenderingSurface {
    /// The full-capability desktop shell.
    DesktopFull,
    /// The companion app.
    CompanionApp,
    /// A read-only browser projection.
    BrowserReadonly,
    /// A handoff packet.
    HandoffPacket,
    /// A headless CLI surface.
    CliHeadless,
    /// A support export.
    SupportExport,
}

impl M5ManifestBuildRenderingSurface {
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
            Self::HandoffPacket => "handoff_packet",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }
}

/// The semantic consumer surface a manifest / build component is embedded in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5ManifestBuildConsumerSurface {
    /// The manifest-authoring workspace.
    ManifestWorkspace,
    /// The live-resource explorer.
    ResourceExplorer,
    /// The run / test / debug build launcher.
    BuildLauncher,
    /// The target-graph panel.
    TargetGraphPanel,
    /// Incident / support triage.
    IncidentSupport,
    /// Docs / help.
    DocsHelp,
    /// A support export.
    SupportExport,
    /// A release-proof surface.
    ReleaseProof,
}

impl M5ManifestBuildConsumerSurface {
    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ManifestWorkspace => "manifest_workspace",
            Self::ResourceExplorer => "resource_explorer",
            Self::BuildLauncher => "build_launcher",
            Self::TargetGraphPanel => "target_graph_panel",
            Self::IncidentSupport => "incident_support",
            Self::DocsHelp => "docs_help",
            Self::SupportExport => "support_export",
            Self::ReleaseProof => "release_proof",
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
    /// A view-only graph / matrix that traps keyboard, assistive-tech, or headless
    /// users (red).
    ViewOnlyTrap,
}

impl NonVisualReachState {
    /// Returns true when the state never strands keyboard / assistive-tech / CLI
    /// users.
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

/// The interactive-claim tier a component grants. When schema, connector, adapter,
/// or target-graph truth weakens, an interactive claim auto-narrows down this
/// ladder so a stale or partial lane can never present as fully executable.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrowedClaimTier {
    /// The lane can execute (apply / run / build / test / open-live) — permitted
    /// only when truth is fully current.
    FullyExecutable,
    /// The lane can propose an action but requires explicit review before it runs.
    ReviewRequired,
    /// The lane is read-only: it can render and copy, but not act.
    ReadOnly,
    /// The lane is inspect-only: it can show identity / context / confidence, but
    /// exposes no action.
    InspectOnly,
}

impl NarrowedClaimTier {
    /// Claim rank, higher is more capable; used to reason about narrowing.
    pub const fn rank(self) -> u8 {
        match self {
            Self::FullyExecutable => 3,
            Self::ReviewRequired => 2,
            Self::ReadOnly => 1,
            Self::InspectOnly => 0,
        }
    }

    /// Returns true when this tier presents as fully executable.
    pub const fn is_fully_executable(self) -> bool {
        matches!(self, Self::FullyExecutable)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyExecutable => "fully_executable",
            Self::ReviewRequired => "review_required",
            Self::ReadOnly => "read_only",
            Self::InspectOnly => "inspect_only",
        }
    }
}

/// Whether a component's granted interactive claim matches the truth that backs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimAffordanceState {
    /// The granted claim equals the component's baseline and the truth supports it
    /// (green).
    MatchesTruth,
    /// The claim auto-narrowed below its baseline because truth weakened, and the
    /// narrowing is disclosed (yellow).
    AutoNarrowedDisclosed,
    /// The granted claim exceeds what the weakened truth supports — it presents as
    /// fully executable or fully current when it is not (red).
    Overclaimed,
}

impl ClaimAffordanceState {
    /// Returns true when the claim never presents beyond what its truth supports.
    pub const fn never_overclaims(self) -> bool {
        !matches!(self, Self::Overclaimed)
    }

    /// Returns true when the state carries a disclosed reduction.
    pub const fn is_disclosed_reduction(self) -> bool {
        matches!(self, Self::AutoNarrowedDisclosed)
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MatchesTruth => "matches_truth",
            Self::AutoNarrowedDisclosed => "auto_narrowed_disclosed",
            Self::Overclaimed => "overclaimed",
        }
    }
}

/// The truth signals that back a component's interactive claim. When any signal is
/// weak — schema not fresh, connector lost, adapter non-native / non-confident, or a
/// policy block — the claim can no longer present as fully executable / current.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ClaimTruthSignals {
    /// Backing schema freshness (reused frozen vocabulary).
    pub schema_freshness: M5SchemaFreshness,
    /// Backing adapter source, present only for adapter-bearing families (reused
    /// frozen vocabulary).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub adapter_source: Option<M5AdapterSourceKind>,
    /// Discovery confidence for the backing truth (reused frozen vocabulary).
    pub discovery_confidence: M5DiscoveryConfidence,
    /// Whether the live connector is currently reachable.
    pub connector_live: bool,
    /// Whether a policy / capability block is currently narrowing the action.
    pub policy_blocked: bool,
}

impl ClaimTruthSignals {
    /// Whether every backing signal is current and authoritative: a fresh schema, a
    /// live connector, no policy block, high confidence, and — where an adapter
    /// backs the truth — a native adapter source.
    pub fn is_fully_current(&self) -> bool {
        self.schema_freshness.is_current()
            && self.discovery_confidence == M5DiscoveryConfidence::High
            && self.connector_live
            && !self.policy_blocked
            && self
                .adapter_source
                .map_or(true, M5AdapterSourceKind::is_native)
    }

    /// Whether any backing signal is partial, stale, lost, or policy-blocked.
    pub fn is_weak(&self) -> bool {
        !self.is_fully_current()
    }

    /// The maximum interactive-claim tier the current truth can support. Weak truth
    /// caps the claim at review-required so a stale / partial lane can never present
    /// as fully executable.
    pub fn max_supported_tier(&self) -> NarrowedClaimTier {
        if self.is_fully_current() {
            NarrowedClaimTier::FullyExecutable
        } else {
            NarrowedClaimTier::ReviewRequired
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

/// Copy / export parity for a component's accessible fallback: the same list /
/// table / textual truth must be copyable as text / JSON / Markdown, and a
/// screenshot is never the only export.
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
    pub rendering_surface: M5ManifestBuildRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: NarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// An honest auto-narrowing block. When a component narrows under a weakened-truth
/// or accessibility state, it names why with a precise frozen downgrade trigger and
/// preserves the key target context rather than silently dropping it.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AccessibilityAutoNarrow {
    /// The frozen downgrade trigger (reused vocabulary) that caused the narrowing.
    pub trigger: M5ManifestBuildDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The key target context is preserved rather than dropped; must hold.
    pub preserves_target_context: bool,
}

impl AccessibilityAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves target context and
    /// carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_target_context && !label_is_generic(&self.narrowed_label)
    }
}

/// Derived qualification status for a component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ComponentAccessibilityStatus {
    /// Full reach / claim / export parity (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech / CLI, overclaims execution, needs a screenshot, or
    /// drops state silently (red).
    Stranded,
}

impl ComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility fallback / auto-narrowing row for one manifest / build component
/// family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentAccessibilityRow {
    /// Record kind; must equal [`MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5ManifestBuildComponentFamily,
    /// Ref to the frozen matrix family schema this row certifies.
    pub source_family_schema_ref: String,
    /// The target identity this component acts on; must survive to the export.
    pub target_id: String,
    /// The target-context ref pinned to the component; must survive to the export.
    pub target_context_ref: String,
    /// Rendered modalities offered; a visual-heavy family must also offer a
    /// non-visual path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5ManifestBuildFallbackModality>,
    /// The non-visual path reaches the same target-backed truth (target / context /
    /// state) as the visual path; must hold.
    pub reaches_target_backed_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: NonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: NonVisualReachState,
    /// Headless CLI / export reach into the non-visual path.
    pub cli_reach: NonVisualReachState,
    /// The truth signals that back this component's interactive claim.
    pub truth_signals: ClaimTruthSignals,
    /// The interactive-claim tier this component offers when truth is fully current.
    pub baseline_claim: NarrowedClaimTier,
    /// The interactive-claim tier actually granted given current truth.
    pub granted_claim: NarrowedClaimTier,
    /// Whether the granted claim matches, honestly narrows below, or overclaims the
    /// backing truth.
    pub claim_affordance: ClaimAffordanceState,
    /// The actions still offered at the granted (possibly narrowed) claim tier.
    #[serde(default)]
    pub granted_actions: Vec<String>,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: ExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: CopyExportParity,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5ManifestBuildRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<RenderingNarrowingDisclosure>,
    /// The honest auto-narrow block, present only when the component is narrowed by
    /// a weakened-truth / accessibility state.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auto_narrow: Option<AccessibilityAutoNarrow>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5ManifestBuildRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in.
    #[serde(default)]
    pub consumer_surfaces: Vec<M5ManifestBuildConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl ComponentAccessibilityRow {
    /// Returns true when this family renders a target graph / capability matrix and
    /// must bind to a non-visual path.
    pub const fn is_visual_heavy(&self) -> bool {
        family_is_visual_heavy(self.component_family)
    }

    /// Returns true when this family exposes an interactive / executable affordance.
    pub const fn is_actionable(&self) -> bool {
        family_is_actionable(self.component_family)
    }

    /// Returns true when at least one non-visual (list / table / textual) fallback
    /// modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// AC1: a stale / partial lane can no longer present as fully executable or
    /// fully current. The granted claim never exceeds what the truth supports, the
    /// declared affordance matches the actual narrowing, and any weak-truth claim is
    /// capped below fully executable.
    pub fn claim_is_honest(&self) -> bool {
        let max = self.truth_signals.max_supported_tier();
        // The granted claim never exceeds what the current truth supports.
        if self.granted_claim.rank() > max.rank() {
            return false;
        }
        // The granted claim never exceeds its own declared baseline.
        if self.granted_claim.rank() > self.baseline_claim.rank() {
            return false;
        }
        // The declared affordance must match the actual narrowing.
        let narrowed = self.granted_claim.rank() < self.baseline_claim.rank();
        match self.claim_affordance {
            ClaimAffordanceState::Overclaimed => false,
            ClaimAffordanceState::AutoNarrowedDisclosed => narrowed,
            ClaimAffordanceState::MatchesTruth => !narrowed,
        }
    }

    /// AC2: assistive-tech and headless CLI modes reach the same target-backed truth
    /// as the visual path — nothing is a view-only trap, and a visual-heavy family
    /// offers a non-visual fallback.
    pub fn reaches_target_backed_truth_via_at(&self) -> bool {
        self.reaches_target_backed_truth
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_visual_heavy() || self.has_non_visual_fallback())
    }

    /// AC2: the export preserves the target identity, context, and confidence states
    /// shown in-product without a screenshot.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_screenshot_only()
            && !self.export_summary_ref.trim().is_empty()
            && !self.target_id.trim().is_empty()
            && !self.target_context_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state.
    pub fn is_reduced(&self) -> bool {
        self.keyboard_reach.is_disclosed_reduction()
            || self.screen_reader_reach.is_disclosed_reduction()
            || self.cli_reach.is_disclosed_reduction()
            || self.claim_affordance.is_disclosed_reduction()
            || self.export_summary.is_disclosed_reduction()
            || self
                .narrowing_disclosures
                .iter()
                .any(|d| d.state.is_disclosed_reduction())
    }

    /// AC3: every narrower rendering surface discloses its reduced interactivity and
    /// keeps its labels, and any reduced state auto-narrows honestly (a precise
    /// frozen trigger + preserved target context) rather than silently dropping key
    /// context.
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
        // Every disclosure never silently drops and preserves labels.
        let disclosures_ok = self.narrowing_disclosures.iter().all(|d| {
            d.state.never_drops_silently()
                && (!d.rendering_surface.is_narrowed() || !d.preserved_labels.is_empty())
        });
        if !disclosures_ok {
            return false;
        }
        // A reduced component must auto-narrow honestly; a full-parity component
        // must not carry a spurious auto-narrow block.
        match (&self.auto_narrow, self.is_reduced()) {
            (Some(narrow), true) => narrow.is_honest(),
            (Some(_), false) => false,
            (None, true) => false,
            (None, false) => true,
        }
    }

    /// Whether the row carries the mandatory required-label subset.
    pub fn preserves_mandatory_labels(&self) -> bool {
        M5ManifestBuildRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> ComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.reaches_target_backed_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.narrowing_disclosed()
            || !self.preserves_mandatory_labels()
        {
            return ComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            ComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            ComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND
            && self.schema_version == MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.target_id.trim().is_empty()
            && !self.target_context_ref.trim().is_empty()
            && !self.fallback_modalities.is_empty()
            && !self.observed_at.trim().is_empty()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
    }

    /// Deterministic governed chip line for this row.
    pub fn chip_tokens(&self) -> String {
        format!(
            "family={family} target={target} keyboard={keyboard} screen_reader={screen_reader} \
cli={cli} claim={claim} granted={granted} export={export} status={status}",
            family = self.component_family.as_str(),
            target = self.target_id,
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            cli = self.cli_reach.as_str(),
            claim = self.claim_affordance.as_str(),
            granted = self.granted_claim.as_str(),
            export = self.export_summary.as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-818 accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentAccessibilitySummary {
    pub family_count: usize,
    pub visual_heavy_family_count: usize,
    pub all_visual_heavy_have_non_visual_fallback: bool,
    pub all_reach_target_backed_truth_via_at: bool,
    pub all_claims_honest: bool,
    pub all_export_summaries_preserve_meaning: bool,
    pub all_narrowing_disclosed: bool,
    pub field_triage_and_publication_aligned: bool,
    pub green_count: usize,
    pub yellow_count: usize,
    pub red_count: usize,
    pub rendering_surface_count: usize,
    pub consumer_surface_count: usize,
}

/// Constructor input for [`ComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<ComponentAccessibilityRow>,
}

/// Checked-in M05-818 accessibility fallback packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<ComponentAccessibilityRow>,
    pub summary: ComponentAccessibilitySummary,
}

impl ComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed
    /// summary.
    pub fn new(input: ComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION,
            record_kind: MANIFEST_BUILD_A11Y_FALLBACK_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: ComponentAccessibilitySummary {
                family_count: 0,
                visual_heavy_family_count: 0,
                all_visual_heavy_have_non_visual_fallback: false,
                all_reach_target_backed_truth_via_at: false,
                all_claims_honest: false,
                all_export_summaries_preserve_meaning: false,
                all_narrowing_disclosed: false,
                field_triage_and_publication_aligned: false,
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
    pub fn represented_families(&self) -> BTreeSet<M5ManifestBuildComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Consumer surfaces represented by some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5ManifestBuildConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// AC3: claim publication (release / docs) and field triage (incident support)
    /// stay aligned on downgrade behavior — the packet reaches all three surfaces.
    pub fn field_triage_and_publication_aligned(&self) -> bool {
        let surfaces = self.represented_consumer_surfaces();
        surfaces.contains(&M5ManifestBuildConsumerSurface::IncidentSupport)
            && surfaces.contains(&M5ManifestBuildConsumerSurface::ReleaseProof)
            && surfaces.contains(&M5ManifestBuildConsumerSurface::DocsHelp)
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> ComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let visual_heavy: Vec<&ComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_visual_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                ComponentAccessibilityStatus::Parity => green += 1,
                ComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                ComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        ComponentAccessibilitySummary {
            family_count: self.rows.len(),
            visual_heavy_family_count: visual_heavy.len(),
            all_visual_heavy_have_non_visual_fallback: visual_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_target_backed_truth_via_at: self
                .rows
                .iter()
                .all(ComponentAccessibilityRow::reaches_target_backed_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(ComponentAccessibilityRow::claim_is_honest),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(ComponentAccessibilityRow::export_preserves_meaning),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(ComponentAccessibilityRow::narrowing_disclosed),
            field_triage_and_publication_aligned: self.field_triage_and_publication_aligned(),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<ComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION {
            violations.push(ComponentAccessibilityViolation::SchemaVersion {
                expected: MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != MANIFEST_BUILD_A11Y_FALLBACK_RECORD_KIND {
            violations.push(ComponentAccessibilityViolation::RecordKind {
                expected: MANIFEST_BUILD_A11Y_FALLBACK_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(ComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(ComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);

            if !row.is_complete() {
                violations.push(ComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // A visual-heavy family must render a graph *and* a non-visual path.
            if row.is_visual_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5ManifestBuildFallbackModality::Graph)
            {
                violations.push(ComponentAccessibilityViolation::VisualHeavyMissingGraph {
                    id: row.row_id.clone(),
                });
            }

            // AC1: interactive claim auto-narrows honestly, never overclaims.
            if !row.claim_is_honest() {
                violations.push(ComponentAccessibilityViolation::OverclaimedExecutable {
                    id: row.row_id.clone(),
                });
            }

            // AC2: assistive-tech / CLI reach the same target-backed truth.
            if !row.reaches_target_backed_truth_via_at() {
                violations.push(ComponentAccessibilityViolation::AssistiveTechStranded {
                    id: row.row_id.clone(),
                });
            }

            // AC2: export preserves target IDs / context / confidence without a
            // screenshot.
            if !row.export_preserves_meaning() {
                violations.push(ComponentAccessibilityViolation::ExportDropsTargetTruth {
                    id: row.row_id.clone(),
                });
            }

            // Mandatory labels (identity / target-context / truth-class / keyboard).
            if !row.preserves_mandatory_labels() {
                violations.push(ComponentAccessibilityViolation::MissingMandatoryLabels {
                    id: row.row_id.clone(),
                });
            }

            // AC3: narrowing disclosed and auto-narrow honest.
            if !row.narrowing_disclosed() {
                violations.push(
                    ComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(ComponentAccessibilityViolation::MissingConsumerParity {
                    id: row.row_id.clone(),
                });
            }

            // No red rows may ship.
            if row.status() == ComponentAccessibilityStatus::Stranded {
                violations.push(ComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5ManifestBuildComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(ComponentAccessibilityViolation::MissingFamilyCoverage { family });
            }
        }

        // AC3: claim publication and field triage stay aligned.
        if !self.field_triage_and_publication_aligned() {
            violations.push(ComponentAccessibilityViolation::TriagePublicationMisaligned);
        }

        if self.summary != self.computed_summary() {
            violations.push(ComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self).expect("accessibility fallback packet serializes"),
        ) {
            violations.push(ComponentAccessibilityViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("accessibility fallback packet serializes")
    }

    /// Deterministic CSV of the certified rows for release / support handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,target_id,keyboard_reach,screen_reader_reach,cli_reach,claim_affordance,granted_claim,export_summary,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{target},{keyboard},{screen_reader},{cli},{claim},{granted},{export},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                target = row.target_id,
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                cli = row.cli_reach.as_str(),
                claim = row.claim_affordance.as_str(),
                granted = row.granted_claim.as_str(),
                export = row.export_summary.as_str(),
                status = row.status().as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, docs, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Manifest / Build Component Accessibility Fallback\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5ManifestBuildComponentFamily::ALL.len(),
        ));
        out.push_str(&format!(
            "- Status: {} green / {} yellow / {} red\n",
            self.summary.green_count, self.summary.yellow_count, self.summary.red_count,
        ));
        out.push_str(&format!(
            "- Claim publication + field triage aligned: {}\n",
            self.summary.field_triage_and_publication_aligned,
        ));
        out.push_str("\n## Rows\n\n");
        for row in &self.rows {
            out.push_str(&format!(
                "- **{}** ({}) — {}\n",
                row.row_id,
                row.component_family.as_str(),
                row.chip_tokens(),
            ));
            if let Some(narrow) = &row.auto_narrow {
                out.push_str(&format!(
                    "  - Auto-narrow: trigger={} — {}\n",
                    narrow.trigger.as_str(),
                    narrow.narrowed_label,
                ));
            }
        }
        out
    }
}

/// Reads and validates the checked-in accessibility fallback export.
pub fn current_m5_manifest_build_a11y_fallback_export(
) -> Result<ComponentAccessibilityPacket, ComponentAccessibilityArtifactError> {
    let packet: ComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-manifest-build-component-accessibility-fallback-proof/support_export.json"
    )))
    .map_err(ComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ComponentAccessibilityArtifactError::Validation(violations))
    }
}

/// Errors emitted when reading the checked-in accessibility fallback export.
#[derive(Debug)]
pub enum ComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ComponentAccessibilityViolation>),
}

impl fmt::Display for ComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(f, "accessibility fallback export parse failed: {error}")
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "accessibility fallback export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for ComponentAccessibilityArtifactError {}

/// Validation failure for M05-818 accessibility fallback packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComponentAccessibilityViolation {
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
    VisualHeavyMissingGraph {
        id: String,
    },
    OverclaimedExecutable {
        id: String,
    },
    AssistiveTechStranded {
        id: String,
    },
    ExportDropsTargetTruth {
        id: String,
    },
    MissingMandatoryLabels {
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
        family: M5ManifestBuildComponentFamily,
    },
    TriagePublicationMisaligned,
    SummaryMismatch,
    RawBoundaryMaterialInExport,
}

impl fmt::Display for ComponentAccessibilityViolation {
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
            Self::VisualHeavyMissingGraph { id } => {
                write!(f, "visual-heavy row {id} does not render a graph modality")
            }
            Self::OverclaimedExecutable { id } => {
                write!(
                    f,
                    "row {id} overclaims execution: its granted claim exceeds what the current truth supports"
                )
            }
            Self::AssistiveTechStranded { id } => {
                write!(
                    f,
                    "row {id} strands assistive-tech / CLI users from the target-backed truth"
                )
            }
            Self::ExportDropsTargetTruth { id } => {
                write!(
                    f,
                    "row {id} export cannot preserve target identity / context / confidence without a screenshot"
                )
            }
            Self::MissingMandatoryLabels { id } => {
                write!(f, "row {id} is missing a mandatory required label")
            }
            Self::NarrowingDropsContextSilently { id } => {
                write!(
                    f,
                    "row {id} narrows without disclosing or auto-narrowing honestly"
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
            Self::TriagePublicationMisaligned => {
                write!(
                    f,
                    "claim publication (release / docs) and field triage (incident support) are not aligned across the packet"
                )
            }
            Self::SummaryMismatch => write!(f, "computed summary does not match stored summary"),
            Self::RawBoundaryMaterialInExport => {
                write!(f, "export contains raw boundary material")
            }
        }
    }
}

impl Error for ComponentAccessibilityViolation {}

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

/// Builds the canonical, checked-in accessibility fallback packet. This is the one
/// source of truth shared by the tests, the emit bin, and the on-disk support
/// export so all three stay byte-aligned.
pub fn seeded_m5_manifest_build_a11y_fallback_packet() -> ComponentAccessibilityPacket {
    ComponentAccessibilityPacket::new(ComponentAccessibilityPacketInput {
        packet_id: "m5-manifest-build-component-accessibility-fallback:stable:0001".to_owned(),
        as_of: "2026-07-04T00:00:00Z".to_owned(),
        matrix_ref: MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:manifest-build-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5ManifestBuildRequiredLabel> {
    M5ManifestBuildRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> CopyExportParity {
    CopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        screenshot_only_prohibited: true,
    }
}

/// A fully-current truth-signal block for a green, executable-when-current row.
fn current_signals(adapter: Option<M5AdapterSourceKind>) -> ClaimTruthSignals {
    ClaimTruthSignals {
        schema_freshness: M5SchemaFreshness::Fresh,
        adapter_source: adapter,
        discovery_confidence: M5DiscoveryConfidence::High,
        connector_live: true,
        policy_blocked: false,
    }
}

fn seeded_rows() -> Vec<ComponentAccessibilityRow> {
    vec![
        // Manifest-editor header — actionable; the backing schema is stale, so the
        // apply claim auto-narrows from fully-executable to review-required (yellow).
        ComponentAccessibilityRow {
            record_kind: MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:manifest-editor-header".to_owned(),
            component_family: M5ManifestBuildComponentFamily::ManifestEditorHeader,
            source_family_schema_ref: MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            target_id: "target:cluster/prod-us-east/deploy.yaml".to_owned(),
            target_context_ref: "context:cluster/prod-us-east".to_owned(),
            fallback_modalities: vec![
                M5ManifestBuildFallbackModality::Textual,
                M5ManifestBuildFallbackModality::List,
            ],
            reaches_target_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            truth_signals: ClaimTruthSignals {
                schema_freshness: M5SchemaFreshness::Stale,
                adapter_source: None,
                discovery_confidence: M5DiscoveryConfidence::High,
                connector_live: true,
                policy_blocked: false,
            },
            baseline_claim: NarrowedClaimTier::FullyExecutable,
            granted_claim: NarrowedClaimTier::ReviewRequired,
            claim_affordance: ClaimAffordanceState::AutoNarrowedDisclosed,
            granted_actions: vec![
                "edit_manifest".to_owned(),
                "request_review_before_apply".to_owned(),
            ],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:manifest-editor-header:a11y".to_owned(),
            copy_export: copy_export(&["target_id", "target_context_ref", "schema_freshness"]),
            rendering_surfaces: vec![
                M5ManifestBuildRenderingSurface::DesktopFull,
                M5ManifestBuildRenderingSurface::CliHeadless,
                M5ManifestBuildRenderingSurface::SupportExport,
            ],
            narrowing_disclosures: vec![
                RenderingNarrowingDisclosure {
                    rendering_surface: M5ManifestBuildRenderingSurface::CliHeadless,
                    state: NarrowingDisclosureState::DisclosedNarrowed,
                    preserved_labels: vec![
                        "identity".to_owned(),
                        "target_context".to_owned(),
                        "truth_class".to_owned(),
                    ],
                    reduced_interactions: vec!["direct_apply".to_owned()],
                },
                RenderingNarrowingDisclosure {
                    rendering_surface: M5ManifestBuildRenderingSurface::SupportExport,
                    state: NarrowingDisclosureState::DisclosedNarrowed,
                    preserved_labels: vec!["identity".to_owned(), "target_context".to_owned()],
                    reduced_interactions: vec!["interactive_edit".to_owned()],
                },
            ],
            auto_narrow: Some(AccessibilityAutoNarrow {
                trigger: M5ManifestBuildDowngradeTrigger::SchemaStale,
                narrowed_label:
                    "The manifest schema is stale for this target, so apply is narrowed to review-required and the target context stays pinned rather than presenting as directly executable".to_owned(),
                preserves_target_context: true,
            }),
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5ManifestBuildConsumerSurface::ManifestWorkspace,
                M5ManifestBuildConsumerSurface::DocsHelp,
            ],
            source_refs: vec![MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("manifest-editor-header"),
        },
        // Schema-validator row — read-only display; fresh schema, keyboard/SR/CLI
        // reachable (green).
        ComponentAccessibilityRow {
            record_kind: MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:schema-validator-row".to_owned(),
            component_family: M5ManifestBuildComponentFamily::SchemaValidatorRow,
            source_family_schema_ref: MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            target_id: "target:cluster/prod-us-east/deploy.yaml".to_owned(),
            target_context_ref: "context:cluster/prod-us-east".to_owned(),
            fallback_modalities: vec![
                M5ManifestBuildFallbackModality::List,
                M5ManifestBuildFallbackModality::Textual,
            ],
            reaches_target_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            truth_signals: current_signals(None),
            baseline_claim: NarrowedClaimTier::ReadOnly,
            granted_claim: NarrowedClaimTier::ReadOnly,
            claim_affordance: ClaimAffordanceState::MatchesTruth,
            granted_actions: vec!["copy_validation_state".to_owned()],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:schema-validator-row:a11y".to_owned(),
            copy_export: copy_export(&["target_id", "schema_freshness", "validation_state"]),
            rendering_surfaces: vec![
                M5ManifestBuildRenderingSurface::DesktopFull,
                M5ManifestBuildRenderingSurface::CliHeadless,
            ],
            narrowing_disclosures: vec![RenderingNarrowingDisclosure {
                rendering_surface: M5ManifestBuildRenderingSurface::CliHeadless,
                state: NarrowingDisclosureState::ParityPreserved,
                preserved_labels: vec![
                    "identity".to_owned(),
                    "target_context".to_owned(),
                    "truth_class".to_owned(),
                ],
                reduced_interactions: vec![],
            }],
            auto_narrow: None,
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5ManifestBuildConsumerSurface::ManifestWorkspace,
                M5ManifestBuildConsumerSurface::DocsHelp,
            ],
            source_refs: vec![MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("schema-validator-row"),
        },
        // Target-context chip group — inspect-only display; keyboard/SR/CLI
        // reachable (green).
        ComponentAccessibilityRow {
            record_kind: MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:target-context-chip-group".to_owned(),
            component_family: M5ManifestBuildComponentFamily::TargetContextChipGroup,
            source_family_schema_ref: MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            target_id: "target:cluster/prod-us-east".to_owned(),
            target_context_ref: "context:cluster/prod-us-east".to_owned(),
            fallback_modalities: vec![
                M5ManifestBuildFallbackModality::List,
                M5ManifestBuildFallbackModality::Textual,
            ],
            reaches_target_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            truth_signals: current_signals(None),
            baseline_claim: NarrowedClaimTier::InspectOnly,
            granted_claim: NarrowedClaimTier::InspectOnly,
            claim_affordance: ClaimAffordanceState::MatchesTruth,
            granted_actions: vec![],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:target-context-chip-group:a11y".to_owned(),
            copy_export: copy_export(&["target_id", "target_context_ref", "truth_class"]),
            rendering_surfaces: vec![
                M5ManifestBuildRenderingSurface::DesktopFull,
                M5ManifestBuildRenderingSurface::HandoffPacket,
            ],
            narrowing_disclosures: vec![RenderingNarrowingDisclosure {
                rendering_surface: M5ManifestBuildRenderingSurface::HandoffPacket,
                state: NarrowingDisclosureState::ParityPreserved,
                preserved_labels: vec![
                    "identity".to_owned(),
                    "target_context".to_owned(),
                    "truth_class".to_owned(),
                ],
                reduced_interactions: vec![],
            }],
            auto_narrow: None,
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5ManifestBuildConsumerSurface::ManifestWorkspace,
                M5ManifestBuildConsumerSurface::IncidentSupport,
            ],
            source_refs: vec![MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("target-context-chip-group"),
        },
        // Resource-link row — actionable; the live connector is lost, so open-live
        // auto-narrows to read-only (rendered truth only) (yellow).
        ComponentAccessibilityRow {
            record_kind: MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:resource-link-row".to_owned(),
            component_family: M5ManifestBuildComponentFamily::ResourceLinkRow,
            source_family_schema_ref: MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            target_id: "target:cluster/prod-us-east/svc/api-gateway".to_owned(),
            target_context_ref: "context:cluster/prod-us-east".to_owned(),
            fallback_modalities: vec![
                M5ManifestBuildFallbackModality::List,
                M5ManifestBuildFallbackModality::Textual,
            ],
            reaches_target_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            truth_signals: ClaimTruthSignals {
                schema_freshness: M5SchemaFreshness::Fresh,
                adapter_source: None,
                discovery_confidence: M5DiscoveryConfidence::Medium,
                connector_live: false,
                policy_blocked: false,
            },
            baseline_claim: NarrowedClaimTier::FullyExecutable,
            granted_claim: NarrowedClaimTier::ReadOnly,
            claim_affordance: ClaimAffordanceState::AutoNarrowedDisclosed,
            granted_actions: vec!["view_rendered_resource".to_owned(), "copy_link".to_owned()],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:resource-link-row:a11y".to_owned(),
            copy_export: copy_export(&["target_id", "link_class", "resource_freshness"]),
            rendering_surfaces: vec![
                M5ManifestBuildRenderingSurface::DesktopFull,
                M5ManifestBuildRenderingSurface::BrowserReadonly,
            ],
            narrowing_disclosures: vec![RenderingNarrowingDisclosure {
                rendering_surface: M5ManifestBuildRenderingSurface::BrowserReadonly,
                state: NarrowingDisclosureState::DisclosedNarrowed,
                preserved_labels: vec![
                    "identity".to_owned(),
                    "target_context".to_owned(),
                    "truth_class".to_owned(),
                ],
                reduced_interactions: vec!["open_live_resource".to_owned()],
            }],
            auto_narrow: Some(AccessibilityAutoNarrow {
                trigger: M5ManifestBuildDowngradeTrigger::ConnectorLoss,
                narrowed_label:
                    "The live connector is lost for this resource, so the link opens the rendered truth read-only and marks the live view unavailable rather than implying a live open".to_owned(),
                preserves_target_context: true,
            }),
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5ManifestBuildConsumerSurface::ResourceExplorer,
                M5ManifestBuildConsumerSurface::IncidentSupport,
            ],
            source_refs: vec![MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("resource-link-row"),
        },
        // Resource-explorer row — actionable; fully current live truth, so it stays
        // fully executable (green).
        ComponentAccessibilityRow {
            record_kind: MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:resource-explorer-row".to_owned(),
            component_family: M5ManifestBuildComponentFamily::ResourceExplorerRow,
            source_family_schema_ref: MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            target_id: "target:cluster/staging/svc/worker".to_owned(),
            target_context_ref: "context:cluster/staging".to_owned(),
            fallback_modalities: vec![
                M5ManifestBuildFallbackModality::Table,
                M5ManifestBuildFallbackModality::Textual,
            ],
            reaches_target_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            truth_signals: current_signals(None),
            baseline_claim: NarrowedClaimTier::FullyExecutable,
            granted_claim: NarrowedClaimTier::FullyExecutable,
            claim_affordance: ClaimAffordanceState::MatchesTruth,
            granted_actions: vec!["open_live_resource".to_owned(), "act_on_resource".to_owned()],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:resource-explorer-row:a11y".to_owned(),
            copy_export: copy_export(&["target_id", "resource_freshness", "discovery_confidence"]),
            rendering_surfaces: vec![
                M5ManifestBuildRenderingSurface::DesktopFull,
                M5ManifestBuildRenderingSurface::CliHeadless,
            ],
            narrowing_disclosures: vec![RenderingNarrowingDisclosure {
                rendering_surface: M5ManifestBuildRenderingSurface::CliHeadless,
                state: NarrowingDisclosureState::ParityPreserved,
                preserved_labels: vec![
                    "identity".to_owned(),
                    "target_context".to_owned(),
                    "truth_class".to_owned(),
                ],
                reduced_interactions: vec![],
            }],
            auto_narrow: None,
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5ManifestBuildConsumerSurface::ResourceExplorer,
                M5ManifestBuildConsumerSurface::ReleaseProof,
            ],
            source_refs: vec![MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("resource-explorer-row"),
        },
        // Adapter-source badge — read-only display of provenance; native adapter,
        // keyboard/SR/CLI reachable (green).
        ComponentAccessibilityRow {
            record_kind: MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:adapter-source-badge".to_owned(),
            component_family: M5ManifestBuildComponentFamily::AdapterSourceBadge,
            source_family_schema_ref: MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            target_id: "target:build///app:server".to_owned(),
            target_context_ref: "context:workspace/monorepo".to_owned(),
            fallback_modalities: vec![
                M5ManifestBuildFallbackModality::Textual,
                M5ManifestBuildFallbackModality::List,
            ],
            reaches_target_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            truth_signals: current_signals(Some(M5AdapterSourceKind::NativeBuildServer)),
            baseline_claim: NarrowedClaimTier::InspectOnly,
            granted_claim: NarrowedClaimTier::InspectOnly,
            claim_affordance: ClaimAffordanceState::MatchesTruth,
            granted_actions: vec![],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:adapter-source-badge:a11y".to_owned(),
            copy_export: copy_export(&["target_id", "adapter_source", "discovery_confidence"]),
            rendering_surfaces: vec![
                M5ManifestBuildRenderingSurface::DesktopFull,
                M5ManifestBuildRenderingSurface::SupportExport,
            ],
            narrowing_disclosures: vec![RenderingNarrowingDisclosure {
                rendering_surface: M5ManifestBuildRenderingSurface::SupportExport,
                state: NarrowingDisclosureState::ParityPreserved,
                preserved_labels: vec![
                    "identity".to_owned(),
                    "target_context".to_owned(),
                    "adapter_source".to_owned(),
                ],
                reduced_interactions: vec![],
            }],
            auto_narrow: None,
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5ManifestBuildConsumerSurface::BuildLauncher,
                M5ManifestBuildConsumerSurface::ReleaseProof,
            ],
            source_refs: vec![MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("adapter-source-badge"),
        },
        // Target-graph row — visual-heavy + actionable; a heuristic adapter with low
        // confidence auto-narrows run to review-required and the screen-reader path
        // discloses a graph-summary reduction (yellow).
        ComponentAccessibilityRow {
            record_kind: MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:target-graph-row".to_owned(),
            component_family: M5ManifestBuildComponentFamily::TargetGraphRow,
            source_family_schema_ref: MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            target_id: "target:build///app:server_test".to_owned(),
            target_context_ref: "context:workspace/monorepo".to_owned(),
            fallback_modalities: vec![
                M5ManifestBuildFallbackModality::Graph,
                M5ManifestBuildFallbackModality::Table,
                M5ManifestBuildFallbackModality::Textual,
            ],
            reaches_target_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::DisclosedReducedButReachable,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            truth_signals: ClaimTruthSignals {
                schema_freshness: M5SchemaFreshness::Fresh,
                adapter_source: Some(M5AdapterSourceKind::HeuristicParse),
                discovery_confidence: M5DiscoveryConfidence::Low,
                connector_live: true,
                policy_blocked: false,
            },
            baseline_claim: NarrowedClaimTier::FullyExecutable,
            granted_claim: NarrowedClaimTier::ReviewRequired,
            claim_affordance: ClaimAffordanceState::AutoNarrowedDisclosed,
            granted_actions: vec![
                "inspect_target_node".to_owned(),
                "request_review_before_run".to_owned(),
            ],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:target-graph-row:a11y".to_owned(),
            copy_export: copy_export(&["target_id", "adapter_source", "node_kind", "discovery_confidence"]),
            rendering_surfaces: vec![
                M5ManifestBuildRenderingSurface::DesktopFull,
                M5ManifestBuildRenderingSurface::CompanionApp,
                M5ManifestBuildRenderingSurface::CliHeadless,
            ],
            narrowing_disclosures: vec![
                RenderingNarrowingDisclosure {
                    rendering_surface: M5ManifestBuildRenderingSurface::CompanionApp,
                    state: NarrowingDisclosureState::DisclosedNarrowed,
                    preserved_labels: vec![
                        "identity".to_owned(),
                        "target_context".to_owned(),
                        "adapter_source".to_owned(),
                    ],
                    reduced_interactions: vec!["run_target".to_owned()],
                },
                RenderingNarrowingDisclosure {
                    rendering_surface: M5ManifestBuildRenderingSurface::CliHeadless,
                    state: NarrowingDisclosureState::DisclosedNarrowed,
                    preserved_labels: vec!["identity".to_owned(), "target_context".to_owned()],
                    reduced_interactions: vec!["rendered_graph".to_owned()],
                },
            ],
            auto_narrow: Some(AccessibilityAutoNarrow {
                trigger: M5ManifestBuildDowngradeTrigger::LowConfidenceDiscovery,
                narrowed_label:
                    "The target graph came from a low-confidence heuristic parse, so the run action narrows to review-required and the screen reader gets a labeled node/edge table instead of the canvas".to_owned(),
                preserves_target_context: true,
            }),
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5ManifestBuildConsumerSurface::TargetGraphPanel,
                M5ManifestBuildConsumerSurface::BuildLauncher,
                M5ManifestBuildConsumerSurface::IncidentSupport,
            ],
            source_refs: vec![MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("target-graph-row"),
        },
        // Capability matrix — visual-heavy + actionable; a policy block narrows the
        // capability to inspect-only (yellow).
        ComponentAccessibilityRow {
            record_kind: MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:capability-matrix".to_owned(),
            component_family: M5ManifestBuildComponentFamily::CapabilityMatrix,
            source_family_schema_ref: MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            target_id: "target:build///app:server".to_owned(),
            target_context_ref: "context:workspace/monorepo".to_owned(),
            fallback_modalities: vec![
                M5ManifestBuildFallbackModality::Graph,
                M5ManifestBuildFallbackModality::Table,
                M5ManifestBuildFallbackModality::Textual,
            ],
            reaches_target_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            truth_signals: ClaimTruthSignals {
                schema_freshness: M5SchemaFreshness::Fresh,
                adapter_source: Some(M5AdapterSourceKind::NativeBuildServer),
                discovery_confidence: M5DiscoveryConfidence::High,
                connector_live: true,
                policy_blocked: true,
            },
            baseline_claim: NarrowedClaimTier::FullyExecutable,
            granted_claim: NarrowedClaimTier::InspectOnly,
            claim_affordance: ClaimAffordanceState::AutoNarrowedDisclosed,
            granted_actions: vec!["inspect_capability_cell".to_owned()],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:capability-matrix:a11y".to_owned(),
            copy_export: copy_export(&["target_id", "capability_state", "adapter_source"]),
            rendering_surfaces: vec![
                M5ManifestBuildRenderingSurface::DesktopFull,
                M5ManifestBuildRenderingSurface::CliHeadless,
            ],
            narrowing_disclosures: vec![RenderingNarrowingDisclosure {
                rendering_surface: M5ManifestBuildRenderingSurface::CliHeadless,
                state: NarrowingDisclosureState::DisclosedNarrowed,
                preserved_labels: vec![
                    "identity".to_owned(),
                    "target_context".to_owned(),
                    "adapter_source".to_owned(),
                ],
                reduced_interactions: vec!["invoke_capability".to_owned()],
            }],
            auto_narrow: Some(AccessibilityAutoNarrow {
                trigger: M5ManifestBuildDowngradeTrigger::PolicyBlock,
                narrowed_label:
                    "A policy block prevents invoking this capability, so the matrix cell narrows to inspect-only and keeps the supported/partial/unsupported state visible rather than offering a blocked run".to_owned(),
                preserves_target_context: true,
            }),
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5ManifestBuildConsumerSurface::BuildLauncher,
                M5ManifestBuildConsumerSurface::DocsHelp,
            ],
            source_refs: vec![MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("capability-matrix"),
        },
        // Raw-event drawer — read-only redacted display; keyboard/SR/CLI reachable
        // (green).
        ComponentAccessibilityRow {
            record_kind: MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:raw-event-drawer".to_owned(),
            component_family: M5ManifestBuildComponentFamily::RawEventDrawer,
            source_family_schema_ref: MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            target_id: "target:build///app:server".to_owned(),
            target_context_ref: "context:workspace/monorepo".to_owned(),
            fallback_modalities: vec![
                M5ManifestBuildFallbackModality::List,
                M5ManifestBuildFallbackModality::Textual,
            ],
            reaches_target_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            truth_signals: current_signals(Some(M5AdapterSourceKind::NativeBuildEvent)),
            baseline_claim: NarrowedClaimTier::ReadOnly,
            granted_claim: NarrowedClaimTier::ReadOnly,
            claim_affordance: ClaimAffordanceState::MatchesTruth,
            granted_actions: vec!["copy_redacted_event".to_owned()],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:raw-event-drawer:a11y".to_owned(),
            copy_export: copy_export(&["target_id", "event_channel", "adapter_source"]),
            rendering_surfaces: vec![
                M5ManifestBuildRenderingSurface::DesktopFull,
                M5ManifestBuildRenderingSurface::SupportExport,
            ],
            narrowing_disclosures: vec![RenderingNarrowingDisclosure {
                rendering_surface: M5ManifestBuildRenderingSurface::SupportExport,
                state: NarrowingDisclosureState::ParityPreserved,
                preserved_labels: vec![
                    "identity".to_owned(),
                    "target_context".to_owned(),
                    "adapter_source".to_owned(),
                ],
                reduced_interactions: vec![],
            }],
            auto_narrow: None,
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5ManifestBuildConsumerSurface::BuildLauncher,
                M5ManifestBuildConsumerSurface::IncidentSupport,
            ],
            source_refs: vec![MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("raw-event-drawer"),
        },
        // Fallback-confidence drawer — read-only display of structured-vs-heuristic
        // posture; keyboard/SR/CLI reachable (green).
        ComponentAccessibilityRow {
            record_kind: MANIFEST_BUILD_A11Y_FALLBACK_ROW_RECORD_KIND.to_owned(),
            schema_version: MANIFEST_BUILD_A11Y_FALLBACK_SCHEMA_VERSION,
            row_id: "a11y:fallback-confidence-drawer".to_owned(),
            component_family: M5ManifestBuildComponentFamily::FallbackConfidenceDrawer,
            source_family_schema_ref: MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned(),
            target_id: "target:build///app:server".to_owned(),
            target_context_ref: "context:workspace/monorepo".to_owned(),
            fallback_modalities: vec![
                M5ManifestBuildFallbackModality::List,
                M5ManifestBuildFallbackModality::Textual,
            ],
            reaches_target_backed_truth: true,
            keyboard_reach: NonVisualReachState::ReachableAndLabeled,
            screen_reader_reach: NonVisualReachState::ReachableAndLabeled,
            cli_reach: NonVisualReachState::ReachableAndLabeled,
            truth_signals: current_signals(Some(M5AdapterSourceKind::HeuristicParse)),
            baseline_claim: NarrowedClaimTier::ReadOnly,
            granted_claim: NarrowedClaimTier::ReadOnly,
            claim_affordance: ClaimAffordanceState::MatchesTruth,
            granted_actions: vec!["copy_confidence_posture".to_owned()],
            export_summary: ExportSummaryState::ReconstructableWithoutScreenshot,
            export_summary_ref: "summary:fallback-confidence-drawer:a11y".to_owned(),
            copy_export: copy_export(&["target_id", "fallback_state", "adapter_source"]),
            rendering_surfaces: vec![
                M5ManifestBuildRenderingSurface::DesktopFull,
                M5ManifestBuildRenderingSurface::HandoffPacket,
            ],
            narrowing_disclosures: vec![RenderingNarrowingDisclosure {
                rendering_surface: M5ManifestBuildRenderingSurface::HandoffPacket,
                state: NarrowingDisclosureState::ParityPreserved,
                preserved_labels: vec![
                    "identity".to_owned(),
                    "target_context".to_owned(),
                    "adapter_source".to_owned(),
                ],
                reduced_interactions: vec![],
            }],
            auto_narrow: None,
            required_labels: all_required_labels(),
            consumer_surfaces: vec![
                M5ManifestBuildConsumerSurface::BuildLauncher,
                M5ManifestBuildConsumerSurface::ReleaseProof,
            ],
            source_refs: vec![MANIFEST_BUILD_A11Y_FALLBACK_COMPONENT_MATRIX_REF.to_owned()],
            observed_at: "2026-07-04T00:00:00Z".to_owned(),
            evidence_refs: ev("fallback-confidence-drawer"),
        },
    ]
}
