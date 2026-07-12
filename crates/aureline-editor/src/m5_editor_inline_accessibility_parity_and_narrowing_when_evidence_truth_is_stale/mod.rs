//! Keyboard / screen-reader / high-zoom / reduced-motion / CLI / export parity, and honest automatic
//! claim narrowing for the M5 editor-tab / gutter / diagnostic-decoration / code-action-chip /
//! diff-view / review-thread / AI-message-card / evidence-timeline inline components.
//!
//! This module is the M05-1122 accessibility-and-auto-narrowing capstone over the frozen M5
//! editor-inline component matrix ([`crate::m5_editor_inline_component_matrix`]). Where the freeze
//! matrix defines the reusable editor tab, gutter, diagnostic decoration, code-action chip, diff view,
//! review thread, AI message card, and evidence timeline primitives, and the 1117-1120 implementation
//! lanes resolve their per-surface truth, this lane certifies — per component family — that inline
//! editor / review / AI claims stay **keyboard-complete, assistive-tech-reachable, high-zoom /
//! reduced-motion-safe, CLI/export-safe, and self-narrowing** rather than presenting a drifted comment
//! anchor, a stale diagnostic severity, an unattributed source, a low confidence, an unverified approval
//! state, or a partial evidence lineage as still a trusted, apply-ready inline surface:
//!
//! - **Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach.** Every family exposes a
//!   keyboard-complete, screen-reader-reachable, high-zoom-legible, reduced-motion-safe, and
//!   CLI/headless-reachable path into the same inline identity, state / disposition, anchor durability
//!   and freshness, severity / source, fix posture, confidence, approval state, and evidence lineage the
//!   rich component shows — never a color-only badge, a hover-only chip, or a motion-only cue that
//!   strands assistive-tech or headless-CLI users. Structure-heavy families (the gutter's layered
//!   markers, the diff view's hunks, the evidence timeline's lineage) additionally bind their structured
//!   layout to a flat list / textual path.
//! - **Export parity.** The support / release / CLI export reconstructs each component's meaning from
//!   typed tokens and opaque refs **without a raw payload**, preserving the same identity, state, anchor,
//!   severity / source, fix posture, confidence, approval, and evidence lineage shown in-product so
//!   support, help, and release proof can reconstruct exactly what the user was actually shown without
//!   leaking a raw document body, diff hunk, message transcript, or evidence blob.
//! - **Honest auto-narrowing.** When an anchor durability signal is stale / drifted, a severity or source
//!   attribution is stale, a fix posture is only inferred, a confidence or source is stale, an approval
//!   state is unverified, or an evidence lineage is only partial, the component's claim auto-narrows from
//!   `trusted_inline_result` / `reviewable_inline_result` to an anchor-unverified / severity-unverified /
//!   fix-posture-unverified / confidence-unverified / approval-unverified / evidence-lineage projection,
//!   discloses the narrowing with a precise trigger and binding dimension, and preserves the canonical
//!   component identity / last-known state. The underlying editor / review / AI truth is never dropped
//!   opaquely. A component with every dimension intact must NOT carry a spurious narrowing, and a
//!   drifted-anchor / stale-severity / inferred-fix / stale-confidence / unverified-approval state can
//!   never keep a trusted, apply-ready claim — an inferred fix never masquerades as an exact one, and a
//!   drifted comment anchor never reads as a durably anchored review.
//! - **Cross-surface disclosure.** The same narrowed state surfaces in the editor UI, the diff UI, the
//!   review UI, the notebook UI, the AI UI, the diagnostics UI, the CLI export, the support export, and
//!   the product UI so product, help, and release publication stay aligned on downgrade behavior rather
//!   than drifting in copy — a trusted-looking surface can never outrun the anchor / severity / confidence
//!   / approval / evidence evidence it is being viewed away from.
//!
//! Each [`EditorInlineComponentAccessibilityRow`] keys on one
//! [`crate::m5_editor_inline_component_matrix::M5EditorInlineComponentFamily`] and reuses that frozen
//! family vocabulary plus the frozen [`M5EditorInlineRequiredLabel`], [`M5EditorInlineDowngradeTrigger`],
//! and shared [`M5EditorInlineConsumerSurface`] consumer surfaces rather than minting parallel synonyms,
//! so the certified labels stay byte-identical to the matrix and the sibling primitive packets.
//!
//! The packet is metadata-only: raw document bodies, diff hunks, message transcripts, evidence blobs,
//! credentials, secrets, and endpoint refs never cross this boundary; the packet carries only typed class
//! tokens, opaque component refs, booleans, and controlled labels so support, release, and diagnostics
//! exports can reconstruct exactly what an accessible fallback would have shown without leaking sensitive
//! material or a raw payload.

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

// Reused frozen component vocabulary — the capstone certifies the freeze matrix's families, required
// labels, downgrade triggers, and consumer surfaces rather than mint parallel ones.
use crate::m5_editor_inline_component_matrix::{
    M5EditorInlineComponentFamily, M5EditorInlineConsumerSurface, M5EditorInlineDowngradeTrigger,
    M5EditorInlineRequiredLabel, M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF,
};

/// Schema version stamped on the M05-1122 editor-inline component accessibility parity packet.
pub const EDITOR_INLINE_A11Y_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by [`EditorInlineComponentAccessibilityPacket`].
pub const EDITOR_INLINE_A11Y_RECORD_KIND: &str =
    "m5_editor_inline_component_accessibility_parity_packet";

/// Stable record-kind tag carried by each [`EditorInlineComponentAccessibilityRow`].
pub const EDITOR_INLINE_A11Y_ROW_RECORD_KIND: &str =
    "m5_editor_inline_component_accessibility_parity_row";

/// Repo-relative path of the boundary schema.
pub const EDITOR_INLINE_A11Y_SCHEMA_REF: &str =
    "schemas/ui/m5-editor-inline-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const EDITOR_INLINE_A11Y_DOC_REF: &str =
    "docs/editor/m5_editor_inline_component_accessibility_parity.md";

/// Repo-relative path of the frozen editor-inline component matrix this lane certifies.
pub const EDITOR_INLINE_A11Y_COMPONENT_MATRIX_REF: &str = M5_EDITOR_INLINE_COMPONENT_SCHEMA_REF;

/// Repo-relative path of the protected fixture directory.
pub const EDITOR_INLINE_A11Y_FIXTURE_DIR: &str =
    "fixtures/ui/m5-editor-inline-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact (the `include_str!` canonical).
pub const EDITOR_INLINE_A11Y_ARTIFACT_REF: &str =
    "artifacts/release/m5-editor-inline-component-accessibility-parity/support_export.json";

/// Repo-relative path of the checked machine-readable matrix CSV.
pub const EDITOR_INLINE_A11Y_CSV_REF: &str =
    "artifacts/release/m5-editor-inline-component-accessibility-parity/matrix.csv";

/// Repo-relative path of the checked Markdown report.
pub const EDITOR_INLINE_A11Y_REPORT_REF: &str =
    "artifacts/release/m5-editor-inline-component-accessibility-parity.md";

/// The reusable component families that render a dense, layered structure (the gutter's stacked
/// markers, the diff view's hunks, the evidence timeline's lineage) and therefore MUST bind their
/// structured layout to an equivalent flat list / textual path so the structure is navigable
/// non-visually.
const fn family_is_structure_heavy(family: M5EditorInlineComponentFamily) -> bool {
    matches!(
        family,
        M5EditorInlineComponentFamily::Gutter
            | M5EditorInlineComponentFamily::DiffView
            | M5EditorInlineComponentFamily::EvidenceTimeline
    )
}

/// The editor / review / AI dimension whose weakening a family primarily discloses. Every row must
/// model at least this dimension so its key weakening axis is covered.
const fn family_primary_dimension(
    family: M5EditorInlineComponentFamily,
) -> M5EditorInlineComponentClaimDimension {
    match family {
        M5EditorInlineComponentFamily::EditorTab => {
            M5EditorInlineComponentClaimDimension::StateClarity
        }
        M5EditorInlineComponentFamily::Gutter => {
            M5EditorInlineComponentClaimDimension::MarkerLayerClarity
        }
        M5EditorInlineComponentFamily::DiagnosticDecoration => {
            M5EditorInlineComponentClaimDimension::SeveritySourceFreshnessClarity
        }
        M5EditorInlineComponentFamily::CodeActionChip => {
            M5EditorInlineComponentClaimDimension::FixPostureClarity
        }
        M5EditorInlineComponentFamily::DiffView => {
            M5EditorInlineComponentClaimDimension::ChangeAnchorClarity
        }
        M5EditorInlineComponentFamily::ReviewThread => {
            M5EditorInlineComponentClaimDimension::ReviewApprovalClarity
        }
        M5EditorInlineComponentFamily::AiMessageCard => {
            M5EditorInlineComponentClaimDimension::ConfidenceSourceClarity
        }
        M5EditorInlineComponentFamily::EvidenceTimeline => {
            M5EditorInlineComponentClaimDimension::EvidenceLineageClarity
        }
    }
}

/// A rendered fallback modality for an editor-inline component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineComponentFallbackModality {
    /// A rich, structured (layered markers / hunks / lineage) projection.
    Structured,
    /// A flat list projection.
    List,
    /// A textual / label-first projection.
    Textual,
    /// A CLI / headless text projection.
    Cli,
}

impl M5EditorInlineComponentFallbackModality {
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
pub enum M5EditorInlineComponentRenderingSurface {
    /// The full-capability desktop editor surface.
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

impl M5EditorInlineComponentRenderingSurface {
    /// Returns true when the surface narrows interactivity below the desktop full-capability baseline
    /// and therefore must disclose its reduction.
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

/// Keyboard / screen-reader / high-zoom / reduced-motion / CLI reach for a component's non-visual path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorInlineComponentNonVisualReachState {
    /// Fully traversable and labeled with no loss.
    ReachableAndLabeled,
    /// Reachable and labeled, but with a disclosed reduction (yellow).
    DisclosedReducedButReachable,
    /// A view-only / hover-only / color-only surface that traps keyboard / assistive-tech /
    /// headless-CLI users (red).
    ViewOnlyTrap,
}

impl EditorInlineComponentNonVisualReachState {
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
pub enum EditorInlineComponentExportSummaryState {
    /// The component meaning reconstructs from the metadata summary without a raw payload.
    ReconstructableWithoutRawPayload,
    /// Partial capture, but disclosed (yellow).
    DisclosedPartialCapture,
    /// The export can only carry meaning by dumping a raw payload (red).
    RequiresRawPayload,
}

impl EditorInlineComponentExportSummaryState {
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

/// Whether a narrower rendering surface discloses its reduced interactivity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorInlineComponentNarrowingDisclosureState {
    /// Full label and summary parity with the desktop surface.
    ParityPreserved,
    /// Reduced interactivity, disclosed with preserved labels (yellow).
    DisclosedNarrowed,
    /// Interactivity, state, or actions dropped without disclosure (red).
    SilentlyDropped,
}

impl EditorInlineComponentNarrowingDisclosureState {
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

/// The inline claim ceiling a component asserts: how strong a trusted / apply-ready posture it lets a
/// surface present. Auto-narrowing lowers this ceiling when an editor / review / AI dimension weakens so
/// a drifted anchor, a stale severity, an inferred fix, a stale confidence, an unverified approval, or a
/// partial evidence lineage can never keep an old `TrustedInlineResult` or `ReviewableInlineResult`
/// label — an inferred fix never masquerades as an exact one.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineComponentClaim {
    /// Trusted inline result: a fully current, durably anchored, attributed, confidence-clear,
    /// approval-clear, evidence-complete inline surface — the strongest claim, a surface Aureline can
    /// present as exactly trusted and apply-ready right now.
    TrustedInlineResult,
    /// Reviewable inline result: a self-sufficient, reviewable read-only structure (a gutter / diff /
    /// evidence timeline a user can review) that is not itself an authoritative trusted apply surface.
    ReviewableInlineResult,
    /// Anchor-unverified projection: the anchor durability signal is stale / drifted; the surface stays
    /// an anchor-unverified projection with its last-known identity preserved, never a durably anchored
    /// result.
    AnchorUnverifiedProjection,
    /// Severity-unverified projection: the severity / source attribution is stale; the surface stays a
    /// severity-unverified projection with its last-known severity preserved, never a freshly-verified
    /// diagnostic.
    SeverityUnverifiedProjection,
    /// Fix-posture-unverified projection: the fix posture is only inferred; the surface stays a
    /// fix-posture-unverified projection that names it an inferred fix, never an exact, safe-to-apply
    /// change.
    FixPostureUnverifiedProjection,
    /// Confidence-unverified projection: the confidence / source context is stale; the surface stays a
    /// confidence-unverified projection that discloses the last-known confidence, never a fully-verified
    /// answer.
    ConfidenceUnverifiedProjection,
    /// Approval-unverified projection: the review approval / outdated-versus-resolved state is
    /// unverified; the surface stays an approval-unverified projection that keeps the last-known thread
    /// state, never a resolved, approved review.
    ApprovalUnverifiedProjection,
    /// Evidence-lineage projection: the evidence lineage is only partial / redacted; the surface stays
    /// an evidence-lineage projection that discloses the partial / redacted lineage, never a
    /// fully-captured evidence trail.
    EvidenceLineageProjection,
}

impl M5EditorInlineComponentClaim {
    /// Every claim tier, strongest first.
    pub const ALL: [Self; 8] = [
        Self::TrustedInlineResult,
        Self::ReviewableInlineResult,
        Self::AnchorUnverifiedProjection,
        Self::SeverityUnverifiedProjection,
        Self::FixPostureUnverifiedProjection,
        Self::ConfidenceUnverifiedProjection,
        Self::ApprovalUnverifiedProjection,
        Self::EvidenceLineageProjection,
    ];

    /// Capability rank; a higher rank asserts a stronger posture. Narrowing lowers rank.
    pub const fn capability_rank(self) -> u8 {
        match self {
            Self::TrustedInlineResult => 7,
            Self::ReviewableInlineResult => 6,
            Self::AnchorUnverifiedProjection => 5,
            Self::SeverityUnverifiedProjection => 4,
            Self::FixPostureUnverifiedProjection => 3,
            Self::ConfidenceUnverifiedProjection => 2,
            Self::ApprovalUnverifiedProjection => 1,
            Self::EvidenceLineageProjection => 0,
        }
    }

    /// Returns true when this claim asserts a fully trusted, apply-ready inline surface.
    pub const fn asserts_trusted_inline_result(self) -> bool {
        matches!(self, Self::TrustedInlineResult)
    }

    /// Returns true when this claim asserts a fully self-sufficient (trusted or reviewable) result.
    pub const fn asserts_self_sufficient_result(self) -> bool {
        matches!(
            self,
            Self::TrustedInlineResult | Self::ReviewableInlineResult
        )
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TrustedInlineResult => "trusted_inline_result",
            Self::ReviewableInlineResult => "reviewable_inline_result",
            Self::AnchorUnverifiedProjection => "anchor_unverified_projection",
            Self::SeverityUnverifiedProjection => "severity_unverified_projection",
            Self::FixPostureUnverifiedProjection => "fix_posture_unverified_projection",
            Self::ConfidenceUnverifiedProjection => "confidence_unverified_projection",
            Self::ApprovalUnverifiedProjection => "approval_unverified_projection",
            Self::EvidenceLineageProjection => "evidence_lineage_projection",
        }
    }
}

/// The editor / review / AI dimension whose state governs how far a component may claim to be a fully
/// trusted, apply-ready inline surface. The dimensions map 1:1 to the eight frozen component families so
/// every family carries an honest narrowing path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineComponentClaimDimension {
    /// State clarity: is the tab's modified / preview / pinned / read-only / shared / generated / remote
    /// state fully stated (editor tab)?
    StateClarity,
    /// Marker-layer clarity: is the gutter's breakpoint / change-marker / diagnostic / fold layering
    /// fully stated without color alone (gutter)?
    MarkerLayerClarity,
    /// Severity / source / freshness clarity: are the diagnostic's problem severity, source attribution,
    /// and freshness fully stated (diagnostic decoration)?
    SeveritySourceFreshnessClarity,
    /// Fix-posture clarity: is the code action's exact-versus-inferred fix posture fully stated (code
    /// action chip)?
    FixPostureClarity,
    /// Change / anchor clarity: are the diff's change kinds and hunk anchor durability fully stated (diff
    /// view)?
    ChangeAnchorClarity,
    /// Review-approval clarity: is the review thread's comment-anchor durability and outdated-versus-
    /// resolved approval state fully stated (review thread)?
    ReviewApprovalClarity,
    /// Confidence / source clarity: are the AI message's source context and confidence fully stated (AI
    /// message card)?
    ConfidenceSourceClarity,
    /// Evidence-lineage clarity: is the evidence timeline's inspectable, export-safe lineage fully stated
    /// (evidence timeline)?
    EvidenceLineageClarity,
}

impl M5EditorInlineComponentClaimDimension {
    /// Every dimension, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::StateClarity,
        Self::MarkerLayerClarity,
        Self::SeveritySourceFreshnessClarity,
        Self::FixPostureClarity,
        Self::ChangeAnchorClarity,
        Self::ReviewApprovalClarity,
        Self::ConfidenceSourceClarity,
        Self::EvidenceLineageClarity,
    ];

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StateClarity => "state_clarity",
            Self::MarkerLayerClarity => "marker_layer_clarity",
            Self::SeveritySourceFreshnessClarity => "severity_source_freshness_clarity",
            Self::FixPostureClarity => "fix_posture_clarity",
            Self::ChangeAnchorClarity => "change_anchor_clarity",
            Self::ReviewApprovalClarity => "review_approval_clarity",
            Self::ConfidenceSourceClarity => "confidence_source_clarity",
            Self::EvidenceLineageClarity => "evidence_lineage_clarity",
        }
    }
}

/// The observed condition of one editor / review / AI dimension. Anything weaker than
/// [`Self::FullyQualified`] imposes a narrowing ceiling on the component's claim. The stale / drifted /
/// inferred / unverified states the lane must auto-narrow on as *weakened evidence* — a drifted anchor,
/// a stale severity / source, an inferred fix, a stale confidence, and an unverified approval — are the
/// states that [`Self::cannot_be_shown_trusted`] flags. A partial evidence lineage is an honest
/// disclosed-absence operation (a partial / redacted lineage shown honestly with inspectable structure),
/// not a truth overstatement, so it is deliberately excluded there.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum M5EditorInlineComponentConditionState {
    /// Fully current, durably anchored, attributed, confidence-clear, approval-clear — imposes no
    /// ceiling.
    FullyQualified,
    /// The anchor durability signal is stale / drifted — claim drops to an anchor-unverified projection.
    AnchorDurabilityStale,
    /// The severity / source attribution is stale — claim drops to a severity-unverified projection.
    SeveritySourceStale,
    /// The fix posture is only inferred, not verified exact — claim drops to a fix-posture-unverified
    /// projection.
    FixPostureInferred,
    /// The confidence / source context is stale — claim drops to a confidence-unverified projection.
    ConfidenceStale,
    /// The review approval / outdated-versus-resolved state is unverified — claim drops to an
    /// approval-unverified projection.
    ApprovalUnverified,
    /// The evidence lineage is only partial / redacted — claim drops to an evidence-lineage projection.
    EvidenceLineagePartial,
}

impl M5EditorInlineComponentConditionState {
    /// Every condition state, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::FullyQualified,
        Self::AnchorDurabilityStale,
        Self::SeveritySourceStale,
        Self::FixPostureInferred,
        Self::ConfidenceStale,
        Self::ApprovalUnverified,
        Self::EvidenceLineagePartial,
    ];

    /// Returns true when the dimension is weaker than fully qualified and therefore imposes a narrowing
    /// ceiling.
    pub const fn is_weak(self) -> bool {
        !matches!(self, Self::FullyQualified)
    }

    /// Returns true when the condition reflects weakened evidence that cannot be shown as a fully
    /// trusted, apply-ready inline surface and must never be shown as such. A partial evidence lineage is
    /// an honest disclosed-absence operation (a partial / redacted lineage shown honestly with
    /// inspectable structure), not a truth overstatement, so it is deliberately excluded here.
    pub const fn cannot_be_shown_trusted(self) -> bool {
        matches!(
            self,
            Self::AnchorDurabilityStale
                | Self::SeveritySourceStale
                | Self::FixPostureInferred
                | Self::ConfidenceStale
                | Self::ApprovalUnverified
        )
    }

    /// The strongest claim this condition state permits.
    pub const fn permitted_ceiling(self) -> M5EditorInlineComponentClaim {
        match self {
            Self::FullyQualified => M5EditorInlineComponentClaim::TrustedInlineResult,
            Self::AnchorDurabilityStale => M5EditorInlineComponentClaim::AnchorUnverifiedProjection,
            Self::SeveritySourceStale => M5EditorInlineComponentClaim::SeverityUnverifiedProjection,
            Self::FixPostureInferred => {
                M5EditorInlineComponentClaim::FixPostureUnverifiedProjection
            }
            Self::ConfidenceStale => M5EditorInlineComponentClaim::ConfidenceUnverifiedProjection,
            Self::ApprovalUnverified => M5EditorInlineComponentClaim::ApprovalUnverifiedProjection,
            Self::EvidenceLineagePartial => M5EditorInlineComponentClaim::EvidenceLineageProjection,
        }
    }

    /// The frozen downgrade trigger this condition names when its weakness binds a narrowing. Each
    /// state maps to the on-topic frozen trigger the freeze matrix already governs, so the certified
    /// reason stays byte-identical to the matrix.
    pub const fn default_trigger(self) -> M5EditorInlineDowngradeTrigger {
        match self {
            // The fully-qualified baseline never narrows; kept for exhaustiveness.
            Self::FullyQualified => M5EditorInlineDowngradeTrigger::ProofStale,
            Self::AnchorDurabilityStale => M5EditorInlineDowngradeTrigger::AnchorStateUnstated,
            Self::SeveritySourceStale => {
                M5EditorInlineDowngradeTrigger::DiagnosticFreshnessUnstated
            }
            Self::FixPostureInferred => M5EditorInlineDowngradeTrigger::InferredFixShownAsExact,
            Self::ConfidenceStale => M5EditorInlineDowngradeTrigger::AiConfidenceUnstated,
            Self::ApprovalUnverified => M5EditorInlineDowngradeTrigger::OutdatedAndResolvedBlurred,
            Self::EvidenceLineagePartial => M5EditorInlineDowngradeTrigger::ProofStale,
        }
    }

    /// Stable token recorded in the row.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullyQualified => "fully_qualified",
            Self::AnchorDurabilityStale => "anchor_durability_stale",
            Self::SeveritySourceStale => "severity_source_stale",
            Self::FixPostureInferred => "fix_posture_inferred",
            Self::ConfidenceStale => "confidence_stale",
            Self::ApprovalUnverified => "approval_unverified",
            Self::EvidenceLineagePartial => "evidence_lineage_partial",
        }
    }
}

/// One editor / review / AI dimension's observed condition on a component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineComponentClaimConditionEntry {
    /// Which dimension this entry describes.
    pub dimension: M5EditorInlineComponentClaimDimension,
    /// The observed condition state of the dimension.
    pub state: M5EditorInlineComponentConditionState,
}

/// An honest claim auto-narrow block. When an editor / review / AI dimension weakens, the component's
/// claim lowers to the permitted ceiling, names the binding dimension and frozen trigger, and preserves
/// the canonical component identity / last-known state rather than silently dropping it — the underlying
/// editor / review / AI truth is never erased opaquely.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineComponentClaimAutoNarrow {
    /// The claim the component is narrowed to.
    pub narrowed_to: M5EditorInlineComponentClaim,
    /// The dimension whose weakness bound the narrowing (the one imposing the strongest ceiling
    /// constraint).
    pub binding_dimension: M5EditorInlineComponentClaimDimension,
    /// The frozen downgrade trigger (reused vocabulary) the narrowing names.
    pub trigger: M5EditorInlineDowngradeTrigger,
    /// A precise, non-generic label safe to render.
    pub narrowed_label: String,
    /// The canonical component identity and last-known state are preserved rather than dropped; must
    /// hold.
    pub preserves_canonical_identity: bool,
    /// The underlying editor / review / AI truth is preserved (never dropped) across the narrowing; must
    /// hold so anchor-unverified, severity-unverified, fix-posture-unverified, confidence-unverified,
    /// approval-unverified, and evidence-lineage states never fail opaquely.
    pub preserves_truth_continuity: bool,
}

impl EditorInlineComponentClaimAutoNarrow {
    /// Whether the auto-narrow block is honest: it preserves canonical identity and editor / review / AI
    /// truth and carries a precise, non-generic label.
    pub fn is_honest(&self) -> bool {
        self.preserves_canonical_identity
            && self.preserves_truth_continuity
            && !label_is_generic(&self.narrowed_label)
    }
}

/// Copy / export parity for a component's accessible fallback: the same truth must be copyable as
/// text / JSON / Markdown, and a raw payload is never the only export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineComponentCopyExportParity {
    /// The copy / export formats offered (must include text, json, markdown).
    #[serde(default)]
    pub formats: Vec<String>,
    /// The named export fields the summary carries.
    #[serde(default)]
    pub export_fields: Vec<String>,
    /// A raw payload is never the only export; must always hold.
    pub raw_payload_only_prohibited: bool,
}

impl EditorInlineComponentCopyExportParity {
    /// Whether the copy / export parity is complete: text / JSON / Markdown are all offered, at least
    /// one export field is named, and a raw-payload-only export is prohibited.
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
pub struct EditorInlineComponentRenderingNarrowingDisclosure {
    /// The rendering surface being narrowed.
    pub rendering_surface: M5EditorInlineComponentRenderingSurface,
    /// How the surface discloses its reduced interactivity.
    pub state: EditorInlineComponentNarrowingDisclosureState,
    /// The labels preserved across the narrowing.
    #[serde(default)]
    pub preserved_labels: Vec<String>,
    /// The interactions reduced on the narrowed surface.
    #[serde(default)]
    pub reduced_interactions: Vec<String>,
}

/// Derived qualification status for an editor-inline-component accessibility row.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EditorInlineComponentAccessibilityStatus {
    /// Full keyboard / screen-reader / high-zoom / reduced-motion / CLI / export parity with no
    /// narrowing (green).
    Parity,
    /// Reduced but fully disclosed, reachable, and honestly auto-narrowed (yellow).
    NarrowedDisclosed,
    /// Strands assistive tech, needs a raw payload, over-claims trusted, or drops state silently (red).
    Stranded,
}

impl EditorInlineComponentAccessibilityStatus {
    /// Stable token recorded in the summary / CSV.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Parity => "parity",
            Self::NarrowedDisclosed => "narrowed_disclosed",
            Self::Stranded => "stranded",
        }
    }
}

/// Accessibility / auto-narrowing parity row for one editor-inline-component family.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineComponentAccessibilityRow {
    /// Record kind; must equal [`EDITOR_INLINE_A11Y_ROW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`EDITOR_INLINE_A11Y_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable row id.
    pub row_id: String,
    /// The frozen component family this row certifies.
    pub component_family: M5EditorInlineComponentFamily,
    /// Ref to the frozen per-component schema this row certifies.
    pub source_family_schema_ref: String,
    /// Opaque ref to the editor / review / AI component this row represents; stays visible on every
    /// surface, so this is never empty.
    pub component_context_ref: String,
    /// Rendered modalities offered; a structure-heavy family must also offer a non-visual (list /
    /// textual / CLI) path.
    #[serde(default)]
    pub fallback_modalities: Vec<M5EditorInlineComponentFallbackModality>,
    /// The non-visual / CLI path reaches the same canonical identity, state, anchor, severity / source,
    /// fix posture, confidence, approval, and evidence lineage as the rich surface; must hold.
    pub reaches_canonical_truth: bool,
    /// Keyboard reach into the non-visual path.
    pub keyboard_reach: EditorInlineComponentNonVisualReachState,
    /// Screen-reader reach into the non-visual path.
    pub screen_reader_reach: EditorInlineComponentNonVisualReachState,
    /// High-zoom (reflow / magnification) legibility of the non-visual path.
    pub high_zoom_reach: EditorInlineComponentNonVisualReachState,
    /// Reduced-motion behavior of the non-visual path.
    pub reduced_motion_reach: EditorInlineComponentNonVisualReachState,
    /// CLI / headless reach into the non-visual path.
    pub cli_reach: EditorInlineComponentNonVisualReachState,
    /// Whether the export-safe summary preserves component meaning.
    pub export_summary: EditorInlineComponentExportSummaryState,
    /// Ref to the export-safe summary object for this component.
    pub export_summary_ref: String,
    /// The copy / export parity of the accessible fallback.
    pub copy_export: EditorInlineComponentCopyExportParity,
    /// The full claim this family asserts when every dimension is intact.
    pub full_ready_claim: M5EditorInlineComponentClaim,
    /// The observed condition of each modeled editor / review / AI dimension.
    #[serde(default)]
    pub claim_conditions: Vec<EditorInlineComponentClaimConditionEntry>,
    /// The honest auto-narrow block, present only when some dimension weakens below the family's full
    /// claim.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub claim_narrow: Option<EditorInlineComponentClaimAutoNarrow>,
    /// Whether the underlying editor / review / AI truth is preserved on this component regardless of
    /// narrowing; must hold so anchor-unverified, severity-unverified, fix-posture-unverified,
    /// confidence-unverified, approval-unverified, and evidence-lineage states never fail opaquely.
    pub truth_preserved: bool,
    /// Rendering surfaces this component is certified on.
    #[serde(default)]
    pub rendering_surfaces: Vec<M5EditorInlineComponentRenderingSurface>,
    /// Per-surface narrowing disclosures.
    #[serde(default)]
    pub narrowing_disclosures: Vec<EditorInlineComponentRenderingNarrowingDisclosure>,
    /// The required labels the accessible fallback preserves (reused vocabulary).
    #[serde(default)]
    pub required_labels: Vec<M5EditorInlineRequiredLabel>,
    /// Semantic consumer surfaces this component is embedded in (reused vocabulary).
    #[serde(default)]
    pub consumer_surfaces: Vec<M5EditorInlineConsumerSurface>,
    /// Source contract refs backing this row.
    #[serde(default)]
    pub source_refs: Vec<String>,
    /// ISO 8601 UTC timestamp the accessibility posture was observed.
    pub observed_at: String,
    /// Evidence packet refs backing this row.
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

impl EditorInlineComponentAccessibilityRow {
    /// Returns true when this family renders a dense, layered structure and must bind to a flat
    /// non-visual path.
    pub const fn is_structure_heavy(&self) -> bool {
        family_is_structure_heavy(self.component_family)
    }

    /// Returns true when at least one non-visual (list / textual / CLI) fallback modality is offered.
    pub fn has_non_visual_fallback(&self) -> bool {
        self.fallback_modalities.iter().any(|m| m.is_non_visual())
    }

    /// The condition state observed for one dimension, or `FullyQualified` when the row does not model
    /// that dimension.
    pub fn condition_for(
        &self,
        dimension: M5EditorInlineComponentClaimDimension,
    ) -> M5EditorInlineComponentConditionState {
        self.claim_conditions
            .iter()
            .find(|c| c.dimension == dimension)
            .map(|c| c.state)
            .unwrap_or(M5EditorInlineComponentConditionState::FullyQualified)
    }

    /// Whether any modeled dimension is weaker than fully qualified.
    pub fn has_weak_dimension(&self) -> bool {
        self.claim_conditions.iter().any(|c| c.state.is_weak())
    }

    /// The strongest claim permitted after applying every modeled dimension's ceiling, capped at the
    /// family's full claim.
    pub fn permitted_claim(&self) -> M5EditorInlineComponentClaim {
        let mut permitted = self.full_ready_claim;
        for condition in &self.claim_conditions {
            let ceiling = condition.state.permitted_ceiling();
            if ceiling.capability_rank() < permitted.capability_rank() {
                permitted = ceiling;
            }
        }
        permitted
    }

    /// The condition entry imposing the strongest (lowest-rank) ceiling, if any weak dimension narrows
    /// below the family's full claim.
    pub fn binding_condition(&self) -> Option<&EditorInlineComponentClaimConditionEntry> {
        let mut binding: Option<(&EditorInlineComponentClaimConditionEntry, u8)> = None;
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
    pub fn binding_dimension(&self) -> Option<M5EditorInlineComponentClaimDimension> {
        self.binding_condition().map(|c| c.dimension)
    }

    /// The claim this component effectively asserts after narrowing.
    pub fn effective_claim(&self) -> M5EditorInlineComponentClaim {
        match &self.claim_narrow {
            Some(narrow) => narrow.narrowed_to,
            None => self.full_ready_claim,
        }
    }

    /// AC / auto-narrowing honesty: a drifted anchor, a stale severity, an inferred fix, a stale
    /// confidence, an unverified approval, or a partial evidence lineage can no longer keep an old
    /// `TrustedInlineResult` / `ReviewableInlineResult` label. The effective claim never exceeds the
    /// permitted ceiling; when a dimension narrows below the full claim, an honest narrow block is
    /// present, narrows to exactly the permitted ceiling, binds to the ceiling-imposing dimension with
    /// its frozen trigger, and preserves canonical identity and truth. When nothing narrows, no spurious
    /// narrow block is present.
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

    /// AC / trusted honesty: a drifted-anchor / stale-severity / inferred-fix / stale-confidence /
    /// unverified-approval state never keeps a trusted claim — an inferred fix never masquerades as an
    /// exact one. When such a state is modeled, the effective claim must not assert
    /// `TrustedInlineResult`.
    pub fn trusted_honesty_holds(&self) -> bool {
        let has_unprovable_state = self
            .claim_conditions
            .iter()
            .any(|c| c.state.cannot_be_shown_trusted());
        !(has_unprovable_state && self.effective_claim().asserts_trusted_inline_result())
    }

    /// AC / assistive-tech reach: accessibility and export surfaces reach the same canonical truth — no
    /// keyboard / screen-reader / high-zoom / reduced-motion / CLI trap, a structure-heavy family offers
    /// a non-visual fallback, and the export reconstructs meaning without a raw payload.
    pub fn reaches_canonical_truth_via_at(&self) -> bool {
        self.reaches_canonical_truth
            && !self.component_context_ref.trim().is_empty()
            && self.keyboard_reach.never_traps()
            && self.screen_reader_reach.never_traps()
            && self.high_zoom_reach.never_traps()
            && self.reduced_motion_reach.never_traps()
            && self.cli_reach.never_traps()
            && (!self.is_structure_heavy() || self.has_non_visual_fallback())
    }

    /// The export preserves the component meaning without leaking a raw payload.
    pub fn export_preserves_meaning(&self) -> bool {
        self.export_summary.never_requires_raw_payload()
            && !self.export_summary_ref.trim().is_empty()
            && self.copy_export.is_complete()
    }

    /// AC / no-loss: anchor-unverified, severity-unverified, fix-posture-unverified,
    /// confidence-unverified, approval-unverified, and evidence-lineage states preserve the underlying
    /// editor / review / AI truth. The row must assert `truth_preserved`, and any narrow block must
    /// preserve truth continuity too.
    pub fn preserves_truth_continuity(&self) -> bool {
        self.truth_preserved
            && self
                .claim_narrow
                .as_ref()
                .map(|n| n.preserves_truth_continuity)
                .unwrap_or(true)
    }

    /// Whether any axis is in a disclosed-reduction (yellow) state or the component carries an honest
    /// claim narrow.
    pub fn is_reduced(&self) -> bool {
        self.claim_narrow.is_some()
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
    /// interactivity and keeps its labels, so product / help / release publication stay aligned on the
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
        M5EditorInlineRequiredLabel::MANDATORY
            .iter()
            .all(|label| self.required_labels.contains(label))
    }

    /// Derived qualification status.
    pub fn status(&self) -> EditorInlineComponentAccessibilityStatus {
        if !self.claim_is_honest()
            || !self.trusted_honesty_holds()
            || !self.reaches_canonical_truth_via_at()
            || !self.export_preserves_meaning()
            || !self.preserves_truth_continuity()
            || !self.narrowing_disclosed()
            || !self.models_primary_dimension()
            || !self.preserves_mandatory_labels()
        {
            return EditorInlineComponentAccessibilityStatus::Stranded;
        }
        if self.is_reduced() {
            EditorInlineComponentAccessibilityStatus::NarrowedDisclosed
        } else {
            EditorInlineComponentAccessibilityStatus::Parity
        }
    }

    /// Whether the row's identity and evidence fields are complete.
    pub fn is_complete(&self) -> bool {
        self.record_kind == EDITOR_INLINE_A11Y_ROW_RECORD_KIND
            && self.schema_version == EDITOR_INLINE_A11Y_SCHEMA_VERSION
            && !self.row_id.trim().is_empty()
            && !self.source_family_schema_ref.trim().is_empty()
            && !self.component_context_ref.trim().is_empty()
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
high_zoom={high_zoom} reduced_motion={reduced_motion} cli={cli} export={export} \
full_claim={full} effective_claim={effective} status={status}",
            family = self.component_family.as_str(),
            keyboard = self.keyboard_reach.as_str(),
            screen_reader = self.screen_reader_reach.as_str(),
            high_zoom = self.high_zoom_reach.as_str(),
            reduced_motion = self.reduced_motion_reach.as_str(),
            cli = self.cli_reach.as_str(),
            export = self.export_summary.as_str(),
            full = self.full_ready_claim.as_str(),
            effective = self.effective_claim().as_str(),
            status = self.status().as_str(),
        )
    }
}

/// Rolled-up summary of an M05-1122 editor-inline-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineComponentAccessibilitySummary {
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

/// Constructor input for [`EditorInlineComponentAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorInlineComponentAccessibilityPacketInput {
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    pub rows: Vec<EditorInlineComponentAccessibilityRow>,
}

/// Checked-in M05-1122 editor-inline-component accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EditorInlineComponentAccessibilityPacket {
    pub schema_version: u32,
    pub record_kind: String,
    pub packet_id: String,
    pub as_of: String,
    pub matrix_ref: String,
    #[serde(default)]
    pub rows: Vec<EditorInlineComponentAccessibilityRow>,
    pub summary: EditorInlineComponentAccessibilitySummary,
}

impl EditorInlineComponentAccessibilityPacket {
    /// Builds a packet, stamping the record kind, schema version, and computed summary.
    pub fn new(input: EditorInlineComponentAccessibilityPacketInput) -> Self {
        let mut packet = Self {
            schema_version: EDITOR_INLINE_A11Y_SCHEMA_VERSION,
            record_kind: EDITOR_INLINE_A11Y_RECORD_KIND.to_owned(),
            packet_id: input.packet_id,
            as_of: input.as_of,
            matrix_ref: input.matrix_ref,
            rows: input.rows,
            summary: EditorInlineComponentAccessibilitySummary {
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
    pub fn represented_families(&self) -> BTreeSet<M5EditorInlineComponentFamily> {
        self.rows.iter().map(|r| r.component_family).collect()
    }

    /// Dimensions exercised by some row's claim conditions.
    pub fn exercised_dimensions(&self) -> BTreeSet<M5EditorInlineComponentClaimDimension> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.dimension))
            .collect()
    }

    /// Condition states exercised by some row's claim conditions.
    pub fn exercised_condition_states(&self) -> BTreeSet<M5EditorInlineComponentConditionState> {
        self.rows
            .iter()
            .flat_map(|r| r.claim_conditions.iter().map(|c| c.state))
            .collect()
    }

    /// Claim tiers that appear as an effective claim across the rows.
    pub fn represented_effective_claims(&self) -> BTreeSet<M5EditorInlineComponentClaim> {
        self.rows.iter().map(|r| r.effective_claim()).collect()
    }

    /// Consumer surfaces ingesting some row in this packet.
    pub fn represented_consumer_surfaces(&self) -> BTreeSet<M5EditorInlineConsumerSurface> {
        self.rows
            .iter()
            .flat_map(|r| r.consumer_surfaces.iter().copied())
            .collect()
    }

    /// Computes summary fields from the packet contents.
    pub fn computed_summary(&self) -> EditorInlineComponentAccessibilitySummary {
        let mut rendering = BTreeSet::new();
        let mut consumers: BTreeSet<M5EditorInlineConsumerSurface> = BTreeSet::new();
        for row in &self.rows {
            rendering.extend(row.rendering_surfaces.iter().copied());
            consumers.extend(row.consumer_surfaces.iter().copied());
        }

        let structure_heavy: Vec<&EditorInlineComponentAccessibilityRow> = self
            .rows
            .iter()
            .filter(|row| row.is_structure_heavy())
            .collect();

        let mut green = 0;
        let mut yellow = 0;
        let mut red = 0;
        for row in &self.rows {
            match row.status() {
                EditorInlineComponentAccessibilityStatus::Parity => green += 1,
                EditorInlineComponentAccessibilityStatus::NarrowedDisclosed => yellow += 1,
                EditorInlineComponentAccessibilityStatus::Stranded => red += 1,
            }
        }

        EditorInlineComponentAccessibilitySummary {
            row_count: self.rows.len(),
            family_count: self.represented_families().len(),
            structure_heavy_family_count: structure_heavy.len(),
            all_structure_heavy_have_non_visual_fallback: structure_heavy
                .iter()
                .all(|row| row.has_non_visual_fallback()),
            all_reach_canonical_truth_via_at: self
                .rows
                .iter()
                .all(EditorInlineComponentAccessibilityRow::reaches_canonical_truth_via_at),
            all_claims_honest: self
                .rows
                .iter()
                .all(EditorInlineComponentAccessibilityRow::claim_is_honest),
            all_trusted_honesty_holds: self
                .rows
                .iter()
                .all(EditorInlineComponentAccessibilityRow::trusted_honesty_holds),
            all_export_summaries_preserve_meaning: self
                .rows
                .iter()
                .all(EditorInlineComponentAccessibilityRow::export_preserves_meaning),
            all_truth_preserved: self
                .rows
                .iter()
                .all(EditorInlineComponentAccessibilityRow::preserves_truth_continuity),
            all_narrowing_disclosed: self
                .rows
                .iter()
                .all(EditorInlineComponentAccessibilityRow::narrowing_disclosed),
            green_count: green,
            yellow_count: yellow,
            red_count: red,
            rendering_surface_count: rendering.len(),
            consumer_surface_count: consumers.len(),
        }
    }

    /// Validates the packet and returns every contract violation.
    pub fn validate(&self) -> Vec<EditorInlineComponentAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.schema_version != EDITOR_INLINE_A11Y_SCHEMA_VERSION {
            violations.push(EditorInlineComponentAccessibilityViolation::SchemaVersion {
                expected: EDITOR_INLINE_A11Y_SCHEMA_VERSION,
                actual: self.schema_version,
            });
        }
        if self.record_kind != EDITOR_INLINE_A11Y_RECORD_KIND {
            violations.push(EditorInlineComponentAccessibilityViolation::RecordKind {
                expected: EDITOR_INLINE_A11Y_RECORD_KIND.to_owned(),
                actual: self.record_kind.clone(),
            });
        }
        if self.packet_id.trim().is_empty()
            || self.as_of.trim().is_empty()
            || self.matrix_ref.trim().is_empty()
        {
            violations.push(EditorInlineComponentAccessibilityViolation::MissingIdentity);
        }

        let mut row_ids = BTreeSet::new();
        let mut seen_families = BTreeSet::new();
        let mut has_unprovable_row = false;
        for row in &self.rows {
            if !row_ids.insert(row.row_id.clone()) {
                violations.push(EditorInlineComponentAccessibilityViolation::DuplicateId {
                    id: row.row_id.clone(),
                });
            }
            seen_families.insert(row.component_family);
            if row
                .claim_conditions
                .iter()
                .any(|c| c.state.cannot_be_shown_trusted())
            {
                has_unprovable_row = true;
            }

            if !row.is_complete() {
                violations.push(EditorInlineComponentAccessibilityViolation::IncompleteRow {
                    id: row.row_id.clone(),
                });
            }

            // Each row must model its family's primary weakening dimension.
            if !row.models_primary_dimension() {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::MissingPrimaryDimension {
                        id: row.row_id.clone(),
                        dimension: family_primary_dimension(row.component_family),
                    },
                );
            }

            // Each row must preserve every mandatory editor / review / AI label.
            if !row.preserves_mandatory_labels() {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::MissingMandatoryLabel {
                        id: row.row_id.clone(),
                    },
                );
            }

            // A structure-heavy family must render a structured projection *and* a non-visual path.
            if row.is_structure_heavy()
                && !row
                    .fallback_modalities
                    .contains(&M5EditorInlineComponentFallbackModality::Structured)
            {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::StructureHeavyMissingStructured {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: claim never over-asserts a trusted / reviewable surface for a weakened one.
            if !row.claim_is_honest() {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::ClaimOverAsserted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / trusted honesty: a drifted-anchor / stale-severity / inferred-fix / stale-confidence /
            // unverified-approval state never keeps a trusted claim.
            if !row.trusted_honesty_holds() {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::WeakStateShownAsTrusted {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: assistive-tech / CLI reach the same canonical truth.
            if !row.reaches_canonical_truth_via_at() {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::AssistiveTechStranded {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC: export preserves meaning without leaking a raw payload.
            if !row.export_preserves_meaning() {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::ExportRequiresRawPayload {
                        id: row.row_id.clone(),
                    },
                );
            }

            // AC / no-loss: weakened states preserve editor / review / AI truth.
            if !row.preserves_truth_continuity() {
                violations.push(EditorInlineComponentAccessibilityViolation::TruthDropped {
                    id: row.row_id.clone(),
                });
            }

            // Narrowing disclosed on every narrowed rendering surface.
            if !row.narrowing_disclosed() {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::NarrowingDropsContextSilently {
                        id: row.row_id.clone(),
                    },
                );
            }

            // Consumer parity: at least two consumer surfaces ingest the row.
            if row.consumer_surfaces.len() < 2 {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::MissingConsumerParity {
                        id: row.row_id.clone(),
                    },
                );
            }

            // No red rows may ship.
            if row.status() == EditorInlineComponentAccessibilityStatus::Stranded {
                violations.push(EditorInlineComponentAccessibilityViolation::StrandedRow {
                    id: row.row_id.clone(),
                });
            }
        }

        // Coverage: every frozen family is certified at least once.
        for family in M5EditorInlineComponentFamily::ALL {
            if !seen_families.contains(&family) {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::MissingFamilyCoverage { family },
                );
            }
        }

        // Coverage: every weakening dimension is exercised somewhere.
        let exercised = self.exercised_dimensions();
        for dimension in M5EditorInlineComponentClaimDimension::ALL {
            if !exercised.contains(&dimension) {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::MissingDimensionCoverage {
                        dimension,
                    },
                );
            }
        }

        // Coverage: every condition state (the fully-qualified baseline plus each spec narrowing axis)
        // is exercised somewhere, so the full narrowing spectrum is proven end-to-end.
        let states = self.exercised_condition_states();
        for state in M5EditorInlineComponentConditionState::ALL {
            if !states.contains(&state) {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::MissingConditionStateCoverage {
                        state,
                    },
                );
            }
        }

        // Coverage: every claim tier appears as an effective claim, so the full narrowing spectrum
        // (trusted → … → evidence-lineage) is proven end-to-end.
        let effective = self.represented_effective_claims();
        for claim in M5EditorInlineComponentClaim::ALL {
            if !effective.contains(&claim) {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::MissingClaimTierCoverage { claim },
                );
            }
        }

        // Trusted honesty must be proven with at least one drifted-anchor / stale-severity / inferred-fix
        // / stale-confidence / unverified-approval row in the packet, so the "cannot-prove never shown as
        // trusted" guarantee is exercised end-to-end.
        if !has_unprovable_row {
            violations.push(EditorInlineComponentAccessibilityViolation::TrustedHonestyUnproven);
        }

        // Cross-surface: the same narrowed state must reach the editor, diff, review, notebook, AI,
        // diagnostics, CLI-export, support-export, and product surfaces — so every consumer surface is
        // exercised at least once across the packet.
        let consumers = self.represented_consumer_surfaces();
        for surface in M5EditorInlineConsumerSurface::ALL {
            if !consumers.contains(&surface) {
                violations.push(
                    EditorInlineComponentAccessibilityViolation::MissingConsumerSurfaceCoverage {
                        surface,
                    },
                );
            }
        }

        if self.summary != self.computed_summary() {
            violations.push(EditorInlineComponentAccessibilityViolation::SummaryMismatch);
        }

        if json_contains_forbidden_material(
            &serde_json::to_value(self)
                .expect("editor-inline-component accessibility parity packet serializes"),
        ) {
            violations.push(EditorInlineComponentAccessibilityViolation::RawInlineMaterialInExport);
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
            .expect("editor-inline-component accessibility parity packet serializes")
    }

    /// Deterministic CSV of the certified rows for support / release handoff.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "row_id,component_family,keyboard_reach,screen_reader_reach,high_zoom_reach,reduced_motion_reach,cli_reach,export_summary,full_claim,effective_claim,status\n",
        );
        for row in &self.rows {
            out.push_str(&format!(
                "{id},{family},{keyboard},{screen_reader},{high_zoom},{reduced_motion},{cli},{export},{full},{effective},{status}\n",
                id = row.row_id,
                family = row.component_family.as_str(),
                keyboard = row.keyboard_reach.as_str(),
                screen_reader = row.screen_reader_reach.as_str(),
                high_zoom = row.high_zoom_reach.as_str(),
                reduced_motion = row.reduced_motion_reach.as_str(),
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
        out.push_str("# M5 Editor-Inline-Component Accessibility & Auto-Narrowing\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- As of: `{}`\n", self.as_of));
        out.push_str(&format!(
            "- Families: {} certified across {} / {} frozen families\n",
            self.summary.family_count,
            self.represented_families().len(),
            M5EditorInlineComponentFamily::ALL.len(),
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

/// Reads and validates the checked-in editor-inline-component accessibility parity export.
pub fn current_m5_editor_inline_component_a11y_export(
) -> Result<EditorInlineComponentAccessibilityPacket, EditorInlineComponentAccessibilityArtifactError>
{
    let packet: EditorInlineComponentAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-editor-inline-component-accessibility-parity/support_export.json"
    )))
    .map_err(EditorInlineComponentAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(EditorInlineComponentAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

/// Errors emitted when reading the checked-in editor-inline-component accessibility parity export.
#[derive(Debug)]
pub enum EditorInlineComponentAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<EditorInlineComponentAccessibilityViolation>),
}

impl fmt::Display for EditorInlineComponentAccessibilityArtifactError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    f,
                    "editor-inline-component accessibility parity export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                write!(
                    f,
                    "editor-inline-component accessibility parity export failed validation: {} violation(s)",
                    violations.len()
                )
            }
        }
    }
}

impl Error for EditorInlineComponentAccessibilityArtifactError {}

/// Validation failure for M05-1122 editor-inline-component accessibility parity packets.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EditorInlineComponentAccessibilityViolation {
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
        dimension: M5EditorInlineComponentClaimDimension,
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
        family: M5EditorInlineComponentFamily,
    },
    MissingDimensionCoverage {
        dimension: M5EditorInlineComponentClaimDimension,
    },
    MissingConditionStateCoverage {
        state: M5EditorInlineComponentConditionState,
    },
    MissingClaimTierCoverage {
        claim: M5EditorInlineComponentClaim,
    },
    TrustedHonestyUnproven,
    MissingConsumerSurfaceCoverage {
        surface: M5EditorInlineConsumerSurface,
    },
    SummaryMismatch,
    RawInlineMaterialInExport,
}

impl EditorInlineComponentAccessibilityViolation {
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
            Self::RawInlineMaterialInExport => "raw_inline_material_in_export",
        }
    }
}

impl fmt::Display for EditorInlineComponentAccessibilityViolation {
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
                write!(f, "row {id} drops a mandatory editor / review / AI label")
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
                    "row {id} shows a drifted-anchor / stale-severity / inferred-fix / stale-confidence / unverified-approval state as a trusted inline result"
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
                    "row {id} does not preserve editor / review / AI truth across narrowing"
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
            Self::TrustedHonestyUnproven => {
                write!(
                    f,
                    "no drifted-anchor / stale-severity / inferred-fix / stale-confidence / unverified-approval row is present to prove the trusted-honesty guarantee"
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
            Self::RawInlineMaterialInExport => {
                write!(f, "export contains raw editor / review / AI material")
            }
        }
    }
}

impl Error for EditorInlineComponentAccessibilityViolation {}

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
pub const EDITOR_INLINE_A11Y_PACKET_ID: &str =
    "m5-editor-inline-component-accessibility-parity:stable:0001";

/// Builds the canonical, checked-in editor-inline-component accessibility parity packet. This is the one
/// source of truth shared by the tests and the on-disk support export so both stay byte-aligned.
pub fn seeded_m5_editor_inline_component_a11y_packet() -> EditorInlineComponentAccessibilityPacket {
    EditorInlineComponentAccessibilityPacket::new(EditorInlineComponentAccessibilityPacketInput {
        packet_id: EDITOR_INLINE_A11Y_PACKET_ID.to_owned(),
        as_of: "2026-07-12T00:00:00Z".to_owned(),
        matrix_ref: EDITOR_INLINE_A11Y_COMPONENT_MATRIX_REF.to_owned(),
        rows: seeded_rows(),
    })
}

fn ev(id: &str) -> Vec<String> {
    vec![format!("evidence:editor-inline-component-a11y:{id}")]
}

fn all_required_labels() -> Vec<M5EditorInlineRequiredLabel> {
    M5EditorInlineRequiredLabel::ALL.to_vec()
}

fn copy_export(fields: &[&str]) -> EditorInlineComponentCopyExportParity {
    EditorInlineComponentCopyExportParity {
        formats: vec!["text".to_owned(), "json".to_owned(), "markdown".to_owned()],
        export_fields: fields.iter().map(|f| (*f).to_owned()).collect(),
        raw_payload_only_prohibited: true,
    }
}

fn condition(
    dimension: M5EditorInlineComponentClaimDimension,
    state: M5EditorInlineComponentConditionState,
) -> EditorInlineComponentClaimConditionEntry {
    EditorInlineComponentClaimConditionEntry { dimension, state }
}

/// The two consumer surfaces every row ships to at minimum — support / release export and the general
/// product UI — so the narrowed state always reaches headless field triage.
fn base_consumers(extra: &[M5EditorInlineConsumerSurface]) -> Vec<M5EditorInlineConsumerSurface> {
    let mut out = vec![
        M5EditorInlineConsumerSurface::SupportExport,
        M5EditorInlineConsumerSurface::ProductUi,
    ];
    out.extend_from_slice(extra);
    out
}

/// Disclosures for the CLI-headless and support-export surfaces. A green (full parity) row keeps full
/// label and summary parity on the narrower surfaces; a narrowed row discloses the reduced interactions
/// it drops there.
fn surface_disclosures(
    labels: &[&str],
    state: EditorInlineComponentNarrowingDisclosureState,
) -> Vec<EditorInlineComponentRenderingNarrowingDisclosure> {
    let preserved: Vec<String> = labels.iter().map(|l| (*l).to_owned()).collect();
    vec![
        EditorInlineComponentRenderingNarrowingDisclosure {
            rendering_surface: M5EditorInlineComponentRenderingSurface::CliHeadless,
            state,
            preserved_labels: preserved.clone(),
            reduced_interactions: vec!["pointer_interaction".to_owned()],
        },
        EditorInlineComponentRenderingNarrowingDisclosure {
            rendering_surface: M5EditorInlineComponentRenderingSurface::SupportExport,
            state,
            preserved_labels: preserved,
            reduced_interactions: vec!["live_animated_overlay".to_owned()],
        },
    ]
}

/// Disclosures for a full-parity (green) row: the narrower surfaces preserve full label and summary
/// parity.
fn parity_surfaces(labels: &[&str]) -> Vec<EditorInlineComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        EditorInlineComponentNarrowingDisclosureState::ParityPreserved,
    )
}

/// Disclosures for a narrowed (yellow) row: the narrower surfaces disclose their reduced interactions
/// while preserving labels.
fn narrowed_surfaces(labels: &[&str]) -> Vec<EditorInlineComponentRenderingNarrowingDisclosure> {
    surface_disclosures(
        labels,
        EditorInlineComponentNarrowingDisclosureState::DisclosedNarrowed,
    )
}

fn rendering_surfaces() -> Vec<M5EditorInlineComponentRenderingSurface> {
    vec![
        M5EditorInlineComponentRenderingSurface::DesktopFull,
        M5EditorInlineComponentRenderingSurface::CliHeadless,
        M5EditorInlineComponentRenderingSurface::SupportExport,
    ]
}

fn non_visual_modalities() -> Vec<M5EditorInlineComponentFallbackModality> {
    vec![
        M5EditorInlineComponentFallbackModality::List,
        M5EditorInlineComponentFallbackModality::Textual,
        M5EditorInlineComponentFallbackModality::Cli,
    ]
}

fn structured_modalities() -> Vec<M5EditorInlineComponentFallbackModality> {
    vec![
        M5EditorInlineComponentFallbackModality::Structured,
        M5EditorInlineComponentFallbackModality::List,
        M5EditorInlineComponentFallbackModality::Textual,
        M5EditorInlineComponentFallbackModality::Cli,
    ]
}

const REACHABLE: EditorInlineComponentNonVisualReachState =
    EditorInlineComponentNonVisualReachState::ReachableAndLabeled;
const REDUCED: EditorInlineComponentNonVisualReachState =
    EditorInlineComponentNonVisualReachState::DisclosedReducedButReachable;

fn seeded_rows() -> Vec<EditorInlineComponentAccessibilityRow> {
    vec![
        // Editor tab (state fully stated) — the tab's modified / preview / pinned / read-only / shared /
        // generated / remote state is fully stated without relying on color alone, so it is a trusted
        // inline result reachable on every surface with no narrowing (green).
        EditorInlineComponentAccessibilityRow {
            record_kind: EDITOR_INLINE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EDITOR_INLINE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:editor-tab-state-stated".to_owned(),
            component_family: M5EditorInlineComponentFamily::EditorTab,
            source_family_schema_ref: M5EditorInlineComponentFamily::EditorTab
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "editor:editor-tab:0001".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: EditorInlineComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:editor-tab-state-stated:a11y".to_owned(),
            copy_export: copy_export(&[
                "component_identity",
                "open_document_context",
                "per_tab_item_state",
                "read_only_and_remote_state",
            ]),
            full_ready_claim: M5EditorInlineComponentClaim::TrustedInlineResult,
            claim_conditions: vec![condition(
                M5EditorInlineComponentClaimDimension::StateClarity,
                M5EditorInlineComponentConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: parity_surfaces(&[
                "component_identity",
                "open_document_context",
                "per_tab_item_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5EditorInlineConsumerSurface::EditorUi,
                M5EditorInlineConsumerSurface::DiffUi,
            ]),
            source_refs: vec![
                "UX Design System IDE component — Editor tab".to_owned(),
                EDITOR_INLINE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("editor-tab-state-stated"),
        },
        // Gutter (marker layering fully stated) — structure-heavy (stacked breakpoint / change-marker /
        // diagnostic / fold layers); the layering is fully stated without color alone, so it is a
        // reviewable inline result that binds its stacked markers to a flat list / textual path, but its
        // dense stacking narrows the screen-reader traversal to a disclosed linear walk (yellow).
        EditorInlineComponentAccessibilityRow {
            record_kind: EDITOR_INLINE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EDITOR_INLINE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:gutter-marker-layering-stated".to_owned(),
            component_family: M5EditorInlineComponentFamily::Gutter,
            source_family_schema_ref: M5EditorInlineComponentFamily::Gutter
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "editor:gutter:0002".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REDUCED,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: EditorInlineComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:gutter-marker-layering-stated:a11y".to_owned(),
            copy_export: copy_export(&[
                "gutter_identity",
                "marker_layer_stack",
                "breakpoint_and_change_marker_state",
                "layer_precedence",
            ]),
            full_ready_claim: M5EditorInlineComponentClaim::ReviewableInlineResult,
            claim_conditions: vec![condition(
                M5EditorInlineComponentClaimDimension::MarkerLayerClarity,
                M5EditorInlineComponentConditionState::FullyQualified,
            )],
            claim_narrow: None,
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "gutter_identity",
                "marker_layer_stack",
                "breakpoint_and_change_marker_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5EditorInlineConsumerSurface::EditorUi,
                M5EditorInlineConsumerSurface::DiagnosticsUi,
            ]),
            source_refs: vec![
                "UX Design System IDE component — Gutter".to_owned(),
                EDITOR_INLINE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("gutter-marker-layering-stated"),
        },
        // Diff view (hunk anchor stale) — structure-heavy; the diff's hunk anchors are stale / drifted,
        // so it auto-narrows to an anchor-unverified projection that keeps the last-known hunk identity
        // and change kinds visible, never a stably-anchored, current diff (yellow).
        EditorInlineComponentAccessibilityRow {
            record_kind: EDITOR_INLINE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EDITOR_INLINE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:diff-view-hunk-anchor-stale".to_owned(),
            component_family: M5EditorInlineComponentFamily::DiffView,
            source_family_schema_ref: M5EditorInlineComponentFamily::DiffView
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "review:diff-view:0003".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REDUCED,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: EditorInlineComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:diff-view-hunk-anchor-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "diff_identity",
                "change_kinds",
                "hunk_identity",
                "last_known_anchor",
            ]),
            full_ready_claim: M5EditorInlineComponentClaim::TrustedInlineResult,
            claim_conditions: vec![condition(
                M5EditorInlineComponentClaimDimension::ChangeAnchorClarity,
                M5EditorInlineComponentConditionState::AnchorDurabilityStale,
            )],
            claim_narrow: Some(EditorInlineComponentClaimAutoNarrow {
                narrowed_to: M5EditorInlineComponentClaim::AnchorUnverifiedProjection,
                binding_dimension: M5EditorInlineComponentClaimDimension::ChangeAnchorClarity,
                trigger: M5EditorInlineDowngradeTrigger::AnchorStateUnstated,
                narrowed_label:
                    "This diff view's hunk anchors are stale or have drifted — shown as an anchor-unverified projection that keeps the last-known hunk identity and change kinds visible, never presenting a re-anchored hunk as a stably-anchored, current diff"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "diff_identity",
                "change_kinds",
                "hunk_identity",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5EditorInlineConsumerSurface::DiffUi,
                M5EditorInlineConsumerSurface::ReviewUi,
            ]),
            source_refs: vec![
                "UX Design System IDE component — Diff view".to_owned(),
                EDITOR_INLINE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("diff-view-hunk-anchor-stale"),
        },
        // Diagnostic decoration (severity / source stale) — the diagnostic's severity or source
        // attribution is stale, so it auto-narrows to a severity-unverified projection that keeps the
        // last-known severity and source visible, never a freshly-verified diagnostic (yellow).
        EditorInlineComponentAccessibilityRow {
            record_kind: EDITOR_INLINE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EDITOR_INLINE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:diagnostic-decoration-severity-source-stale".to_owned(),
            component_family: M5EditorInlineComponentFamily::DiagnosticDecoration,
            source_family_schema_ref: M5EditorInlineComponentFamily::DiagnosticDecoration
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "diagnostics:diagnostic-decoration:0004".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: EditorInlineComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:diagnostic-decoration-severity-source-stale:a11y"
                .to_owned(),
            copy_export: copy_export(&[
                "decoration_identity",
                "severity",
                "source_attribution",
                "freshness_state",
            ]),
            full_ready_claim: M5EditorInlineComponentClaim::TrustedInlineResult,
            claim_conditions: vec![condition(
                M5EditorInlineComponentClaimDimension::SeveritySourceFreshnessClarity,
                M5EditorInlineComponentConditionState::SeveritySourceStale,
            )],
            claim_narrow: Some(EditorInlineComponentClaimAutoNarrow {
                narrowed_to: M5EditorInlineComponentClaim::SeverityUnverifiedProjection,
                binding_dimension:
                    M5EditorInlineComponentClaimDimension::SeveritySourceFreshnessClarity,
                trigger: M5EditorInlineDowngradeTrigger::DiagnosticFreshnessUnstated,
                narrowed_label:
                    "This diagnostic's severity or source attribution is stale — shown as a severity-unverified projection that keeps the last-known severity and source visible, never presenting an unattributed or stale problem as a freshly-verified diagnostic"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "decoration_identity",
                "severity",
                "source_attribution",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5EditorInlineConsumerSurface::DiagnosticsUi,
                M5EditorInlineConsumerSurface::EditorUi,
            ]),
            source_refs: vec![
                "UX Design System IDE component — Diagnostic decoration".to_owned(),
                EDITOR_INLINE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("diagnostic-decoration-severity-source-stale"),
        },
        // Code action chip (fix posture inferred) — the code action's fix posture is inferred, not
        // verified exact, so it auto-narrows to a fix-posture-unverified projection that names it an
        // inferred fix requiring review, never presenting an inferred fix as exact (yellow).
        EditorInlineComponentAccessibilityRow {
            record_kind: EDITOR_INLINE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EDITOR_INLINE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:code-action-chip-fix-posture-inferred".to_owned(),
            component_family: M5EditorInlineComponentFamily::CodeActionChip,
            source_family_schema_ref: M5EditorInlineComponentFamily::CodeActionChip
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "editor:code-action-chip:0005".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: EditorInlineComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:code-action-chip-fix-posture-inferred:a11y".to_owned(),
            copy_export: copy_export(&[
                "chip_identity",
                "fix_posture",
                "apply_state",
                "requires_review_note",
            ]),
            full_ready_claim: M5EditorInlineComponentClaim::TrustedInlineResult,
            claim_conditions: vec![condition(
                M5EditorInlineComponentClaimDimension::FixPostureClarity,
                M5EditorInlineComponentConditionState::FixPostureInferred,
            )],
            claim_narrow: Some(EditorInlineComponentClaimAutoNarrow {
                narrowed_to: M5EditorInlineComponentClaim::FixPostureUnverifiedProjection,
                binding_dimension: M5EditorInlineComponentClaimDimension::FixPostureClarity,
                trigger: M5EditorInlineDowngradeTrigger::InferredFixShownAsExact,
                narrowed_label:
                    "This code action's fix posture is inferred, not verified exact — shown as a fix-posture-unverified projection that names it an inferred fix requiring review, never presenting an inferred fix as an exact, safe-to-apply change"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "chip_identity",
                "fix_posture",
                "apply_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5EditorInlineConsumerSurface::EditorUi,
                M5EditorInlineConsumerSurface::DiagnosticsUi,
            ]),
            source_refs: vec![
                "UX Design System IDE component — Code action chip".to_owned(),
                EDITOR_INLINE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("code-action-chip-fix-posture-inferred"),
        },
        // AI message card (confidence / source stale) — the AI message's confidence or source context is
        // stale, so it auto-narrows to a confidence-unverified projection that discloses the last-known
        // confidence and source, never a fully-verified result (yellow).
        EditorInlineComponentAccessibilityRow {
            record_kind: EDITOR_INLINE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EDITOR_INLINE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:ai-message-card-confidence-source-stale".to_owned(),
            component_family: M5EditorInlineComponentFamily::AiMessageCard,
            source_family_schema_ref: M5EditorInlineComponentFamily::AiMessageCard
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "ai:ai-message-card:0006".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REDUCED,
            cli_reach: REACHABLE,
            export_summary: EditorInlineComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:ai-message-card-confidence-source-stale:a11y".to_owned(),
            copy_export: copy_export(&[
                "card_identity",
                "source_context",
                "confidence",
                "available_actions",
            ]),
            full_ready_claim: M5EditorInlineComponentClaim::TrustedInlineResult,
            claim_conditions: vec![condition(
                M5EditorInlineComponentClaimDimension::ConfidenceSourceClarity,
                M5EditorInlineComponentConditionState::ConfidenceStale,
            )],
            claim_narrow: Some(EditorInlineComponentClaimAutoNarrow {
                narrowed_to: M5EditorInlineComponentClaim::ConfidenceUnverifiedProjection,
                binding_dimension: M5EditorInlineComponentClaimDimension::ConfidenceSourceClarity,
                trigger: M5EditorInlineDowngradeTrigger::AiConfidenceUnstated,
                narrowed_label:
                    "This AI message's confidence or source context is stale — shown as a confidence-unverified projection that discloses the last-known confidence and source, never presenting a low-confidence or stale-source answer as a fully-verified result"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "card_identity",
                "source_context",
                "confidence",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5EditorInlineConsumerSurface::AiUi,
                M5EditorInlineConsumerSurface::NotebookUi,
            ]),
            source_refs: vec![
                "UX Design System IDE component — AI message card".to_owned(),
                EDITOR_INLINE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("ai-message-card-confidence-source-stale"),
        },
        // Review thread (approval unverified) — the review thread's comment-anchor durability and
        // outdated-versus-resolved approval state cannot be distinguished, so it auto-narrows to an
        // approval-unverified projection that keeps the last-known thread state and comment anchor
        // visible, never blurring an outdated thread into a resolved, approved one (yellow).
        EditorInlineComponentAccessibilityRow {
            record_kind: EDITOR_INLINE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EDITOR_INLINE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:review-thread-approval-unverified".to_owned(),
            component_family: M5EditorInlineComponentFamily::ReviewThread,
            source_family_schema_ref: M5EditorInlineComponentFamily::ReviewThread
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "review:review-thread:0007".to_owned(),
            fallback_modalities: non_visual_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: EditorInlineComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:review-thread-approval-unverified:a11y".to_owned(),
            copy_export: copy_export(&[
                "thread_identity",
                "comment_anchor",
                "outdated_versus_resolved_state",
                "last_known_thread_state",
            ]),
            full_ready_claim: M5EditorInlineComponentClaim::TrustedInlineResult,
            claim_conditions: vec![condition(
                M5EditorInlineComponentClaimDimension::ReviewApprovalClarity,
                M5EditorInlineComponentConditionState::ApprovalUnverified,
            )],
            claim_narrow: Some(EditorInlineComponentClaimAutoNarrow {
                narrowed_to: M5EditorInlineComponentClaim::ApprovalUnverifiedProjection,
                binding_dimension: M5EditorInlineComponentClaimDimension::ReviewApprovalClarity,
                trigger: M5EditorInlineDowngradeTrigger::OutdatedAndResolvedBlurred,
                narrowed_label:
                    "This review thread's approval state cannot be distinguished as outdated versus resolved — shown as an approval-unverified projection that keeps the last-known thread state and comment anchor visible, never blurring an outdated thread into a resolved, approved one"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "thread_identity",
                "comment_anchor",
                "outdated_versus_resolved_state",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5EditorInlineConsumerSurface::ReviewUi,
                M5EditorInlineConsumerSurface::CliExport,
            ]),
            source_refs: vec![
                "UX Design System IDE component — Review thread".to_owned(),
                EDITOR_INLINE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("review-thread-approval-unverified"),
        },
        // Evidence timeline (evidence lineage partial) — structure-heavy; the evidence lineage is only
        // partially captured or redacted, so it auto-narrows to an evidence-lineage projection that
        // discloses the partial / redacted lineage alongside the inspectable structure, never hiding the
        // missing evidence in an opaque log (yellow). A partial evidence lineage is an honest
        // disclosed-absence operation, not a trusted overstatement.
        EditorInlineComponentAccessibilityRow {
            record_kind: EDITOR_INLINE_A11Y_ROW_RECORD_KIND.to_owned(),
            schema_version: EDITOR_INLINE_A11Y_SCHEMA_VERSION,
            row_id: "a11y:evidence-timeline-lineage-partial".to_owned(),
            component_family: M5EditorInlineComponentFamily::EvidenceTimeline,
            source_family_schema_ref: M5EditorInlineComponentFamily::EvidenceTimeline
                .canonical_component_schema_ref()
                .to_owned(),
            component_context_ref: "ai:evidence-timeline:0008".to_owned(),
            fallback_modalities: structured_modalities(),
            reaches_canonical_truth: true,
            keyboard_reach: REACHABLE,
            screen_reader_reach: REACHABLE,
            high_zoom_reach: REACHABLE,
            reduced_motion_reach: REACHABLE,
            cli_reach: REACHABLE,
            export_summary: EditorInlineComponentExportSummaryState::ReconstructableWithoutRawPayload,
            export_summary_ref: "summary:evidence-timeline-lineage-partial:a11y".to_owned(),
            copy_export: copy_export(&[
                "timeline_identity",
                "evidence_lineage",
                "inspectable_structure",
                "partial_or_redacted_note",
            ]),
            full_ready_claim: M5EditorInlineComponentClaim::TrustedInlineResult,
            claim_conditions: vec![condition(
                M5EditorInlineComponentClaimDimension::EvidenceLineageClarity,
                M5EditorInlineComponentConditionState::EvidenceLineagePartial,
            )],
            claim_narrow: Some(EditorInlineComponentClaimAutoNarrow {
                narrowed_to: M5EditorInlineComponentClaim::EvidenceLineageProjection,
                binding_dimension: M5EditorInlineComponentClaimDimension::EvidenceLineageClarity,
                trigger: M5EditorInlineDowngradeTrigger::ProofStale,
                narrowed_label:
                    "This evidence timeline's lineage is only partially captured or redacted — shown as an evidence-lineage projection that discloses the partial / redacted lineage alongside the inspectable structure, never hiding the missing evidence in an opaque log"
                        .to_owned(),
                preserves_canonical_identity: true,
                preserves_truth_continuity: true,
            }),
            truth_preserved: true,
            rendering_surfaces: rendering_surfaces(),
            narrowing_disclosures: narrowed_surfaces(&[
                "timeline_identity",
                "evidence_lineage",
                "inspectable_structure",
            ]),
            required_labels: all_required_labels(),
            consumer_surfaces: base_consumers(&[
                M5EditorInlineConsumerSurface::AiUi,
                M5EditorInlineConsumerSurface::NotebookUi,
            ]),
            source_refs: vec![
                "UX Design System IDE component — Evidence timeline".to_owned(),
                EDITOR_INLINE_A11Y_DOC_REF.to_owned(),
            ],
            observed_at: "2026-07-12T00:00:00Z".to_owned(),
            evidence_refs: ev("evidence-timeline-lineage-partial"),
        },
    ]
}
