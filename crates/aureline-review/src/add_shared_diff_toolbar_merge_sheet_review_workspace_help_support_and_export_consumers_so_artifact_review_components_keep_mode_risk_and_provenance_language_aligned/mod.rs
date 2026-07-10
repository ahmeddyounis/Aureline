//! Shared diff-toolbar / merge-sheet / review-workspace / help / support / export
//! consumers that keep the nine reusable structured-artifact review components at
//! mode, risk, and provenance parity across every claimed M5 profile.
//!
//! This module is the closing consumer-adoption lane for the structured-artifact
//! review components frozen in
//! [`crate::freeze_the_m5_structured_artifact_review_component_matrix`] and
//! implemented by the artifact-identity / diff-mode, structure-row /
//! compare-summary, merge-decision / generated-notice, and rendered-compare /
//! media-rail / redaction-trust-badge lanes. It binds each shared component to the
//! diff toolbar, merge sheet, review workspace, Help surface, support packet, and
//! exported view that render it, and proves — by fixtures, not screenshots — that
//! the same artifact object presents the same artifact-class, canonical-source,
//! diff-mode, compare-risk, and generated-from language wherever it appears.
//!
//! The core honesty axes are two. First, parity: for a given artifact object, every
//! consumer surface must present identical parity facet values — the same
//! canonical-source label, the same primary mode/action, the same compare
//! risk/status language, and the same generated-from provenance relation. A surface
//! may narrow how much it shows when render, schema, or merge fidelity degrades, but
//! it may never reword the underlying language per surface, silently promote a
//! compare-only artifact to writable state, flatten a structured mode into a raw
//! fallback without explanation, or hide the generated-from / source-of-truth
//! relation behind generic file chrome. Second, disclosure: when a surface narrows,
//! it must do so through an explicit narrow banner that names the reason, the
//! preserved facets, and the next action — the raw / export-safe fallback and the
//! redaction posture stay explicit rather than collapsing the artifact out of view.
//!
//! Component reuse is proven rather than inferred: every one of the nine shared
//! components must be adopted by at least two distinct consumers, and Help, support,
//! and exported-view consumers must point at the canonical component contracts by
//! id. The render/schema fidelity vocabulary is reused directly from the frozen
//! matrix ([`M5ArtifactFidelityState`]) and the component identity from
//! [`M5ArtifactComponent`], so fidelity narrowing and component identity read the
//! same everywhere.
//!
//! The packet references upstream component contracts by id rather than embedding
//! their content. Raw artifact bodies, raw render payloads, raw media bytes,
//! credentials, and live provider responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-structured-artifact-review-component-consumer.schema.json`](../../../../schemas/ui/m5-structured-artifact-review-component-consumer.schema.json).
//! The contract doc is
//! [`docs/review/m5/add_shared_diff_toolbar_merge_sheet_review_workspace_help_support_and_export_consumers_so_artifact_review_components_keep_mode_risk_and_provenance_language_aligned.md`](../../../../docs/review/m5/add_shared_diff_toolbar_merge_sheet_review_workspace_help_support_and_export_consumers_so_artifact_review_components_keep_mode_risk_and_provenance_language_aligned.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-structured-artifact-review-component-consumers/`](../../../../fixtures/ui/m5-structured-artifact-review-component-consumers/).

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_structured_artifact_review_component_matrix::{
    M5ArtifactComponent, M5ArtifactFidelityState,
};

/// Stable record-kind tag carried by [`ArtifactReviewComponentConsumerPacket`].
pub const ARTIFACT_REVIEW_COMPONENT_CONSUMER_RECORD_KIND: &str =
    "artifact_review_component_consumer_parity_truth";

/// Schema version for artifact-review-component consumer parity records.
pub const ARTIFACT_REVIEW_COMPONENT_CONSUMER_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const ARTIFACT_REVIEW_COMPONENT_CONSUMER_SCHEMA_REF: &str =
    "schemas/ui/m5-structured-artifact-review-component-consumer.schema.json";

/// Repo-relative path of the contract doc.
pub const ARTIFACT_REVIEW_COMPONENT_CONSUMER_DOC_REF: &str =
    "docs/review/m5/add_shared_diff_toolbar_merge_sheet_review_workspace_help_support_and_export_consumers_so_artifact_review_components_keep_mode_risk_and_provenance_language_aligned.md";

/// Repo-relative path of the frozen component matrix these consumers adopt.
pub const ARTIFACT_REVIEW_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-structured-artifact-review-component-matrix.schema.json";

/// Repo-relative path of the artifact-identity-bar / diff-mode-switcher contract.
pub const ARTIFACT_REVIEW_COMPONENT_CONSUMER_IDENTITY_DIFF_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-artifact-identity-diff-mode-controls.schema.json";

/// Repo-relative path of the structure-row / compare-summary-card contract.
pub const ARTIFACT_REVIEW_COMPONENT_CONSUMER_STRUCTURE_COMPARE_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-structure-compare-summary-controls.schema.json";

/// Repo-relative path of the merge-decision-row / generated-artifact-notice contract.
pub const ARTIFACT_REVIEW_COMPONENT_CONSUMER_MERGE_GENERATED_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-merge-decision-generated-notice-controls.schema.json";

/// Repo-relative path of the rendered-compare / media-rail / redaction-trust-badge contract.
pub const ARTIFACT_REVIEW_COMPONENT_CONSUMER_MEDIA_TRUST_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-rendered-compare-media-trust-controls.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const ARTIFACT_REVIEW_COMPONENT_CONSUMER_FIXTURE_DIR: &str =
    "fixtures/ui/m5-structured-artifact-review-component-consumers";

/// Repo-relative path of the checked support-export artifact.
pub const ARTIFACT_REVIEW_COMPONENT_CONSUMER_ARTIFACT_REF: &str =
    "artifacts/release/m5-structured-artifact-review-consumers-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const ARTIFACT_REVIEW_COMPONENT_CONSUMER_SUMMARY_REF: &str =
    "artifacts/release/m5-structured-artifact-review-consumers-proof/summary.md";

/// Canonical component contract that a consumer must point at for a given component.
///
/// Each of the nine shared components resolves to the checked-in schema of the
/// implement lane that produced it: the artifact-identity / diff-mode controls, the
/// structure-row / compare-summary controls, the merge-decision / generated-notice
/// controls, and the rendered-compare / media-rail / redaction-trust-badge controls.
pub const fn component_canonical_schema_ref(component: M5ArtifactComponent) -> &'static str {
    match component {
        M5ArtifactComponent::ArtifactIdentityBar | M5ArtifactComponent::DiffModeSwitcher => {
            ARTIFACT_REVIEW_COMPONENT_CONSUMER_IDENTITY_DIFF_CONTROLS_CONTRACT_REF
        }
        M5ArtifactComponent::StructureRow | M5ArtifactComponent::CompareSummaryCard => {
            ARTIFACT_REVIEW_COMPONENT_CONSUMER_STRUCTURE_COMPARE_CONTROLS_CONTRACT_REF
        }
        M5ArtifactComponent::MergeDecisionRow | M5ArtifactComponent::GeneratedArtifactNotice => {
            ARTIFACT_REVIEW_COMPONENT_CONSUMER_MERGE_GENERATED_CONTROLS_CONTRACT_REF
        }
        M5ArtifactComponent::RenderedCompareViewer
        | M5ArtifactComponent::MediaMetadataRail
        | M5ArtifactComponent::RedactionOrTrustBadgeSet => {
            ARTIFACT_REVIEW_COMPONENT_CONSUMER_MEDIA_TRUST_CONTROLS_CONTRACT_REF
        }
    }
}

/// Consumer surface that must reuse the shared artifact-review components at parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewComponentConsumer {
    /// Diff / compare toolbar.
    DiffToolbar,
    /// Merge / conflict resolution sheet.
    MergeSheet,
    /// Review workspace surface.
    ReviewWorkspace,
    /// Help / About surface.
    HelpSurface,
    /// Support packet.
    SupportPacket,
    /// Exported review evidence / artifact view.
    ExportedView,
}

impl ArtifactReviewComponentConsumer {
    /// Every consumer, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::DiffToolbar,
        Self::MergeSheet,
        Self::ReviewWorkspace,
        Self::HelpSurface,
        Self::SupportPacket,
        Self::ExportedView,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiffToolbar => "diff_toolbar",
            Self::MergeSheet => "merge_sheet",
            Self::ReviewWorkspace => "review_workspace",
            Self::HelpSurface => "help_surface",
            Self::SupportPacket => "support_packet",
            Self::ExportedView => "exported_view",
        }
    }

    /// Whether this consumer is a Help, support, or exported-view surface that must
    /// point at the canonical component contracts by id.
    pub const fn is_help_support_or_export(self) -> bool {
        matches!(
            self,
            Self::HelpSurface | Self::SupportPacket | Self::ExportedView
        )
    }
}

/// A parity facet whose value must stay identical across surfaces for one object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewComponentParityFacet {
    /// The artifact-class / canonical-source label for the component.
    CanonicalSourceLabel,
    /// The primary diff-mode / action offered by the component.
    ModeAction,
    /// The compare risk / status language shown on the component.
    RiskStatusLanguage,
    /// The generated-from / source-of-truth provenance relation.
    ProvenanceRelation,
}

impl ArtifactReviewComponentParityFacet {
    /// Every parity facet, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::CanonicalSourceLabel,
        Self::ModeAction,
        Self::RiskStatusLanguage,
        Self::ProvenanceRelation,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CanonicalSourceLabel => "canonical_source_label",
            Self::ModeAction => "mode_action",
            Self::RiskStatusLanguage => "risk_status_language",
            Self::ProvenanceRelation => "provenance_relation",
        }
    }
}

/// How much of a shared component a consumer renders.
///
/// Narrowing changes how much is shown, never the underlying parity language: a
/// narrowed surface still carries the same canonical-source label, mode/action,
/// risk/status language, and provenance relation, and discloses the narrowing
/// through an explicit banner.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewComponentRenderMode {
    /// Full parity; the structured artifact renders faithfully.
    FullParity,
    /// Structured fidelity is narrowed; structure is partial or the render is untrusted.
    StructuredFidelityNarrowed,
    /// Structured mode is unavailable; an explicitly labeled raw / export-safe fallback shows.
    RawFallbackDisclosed,
    /// Content is redacted or withheld under the export/redaction posture.
    RedactionNarrowed,
}

impl ArtifactReviewComponentRenderMode {
    /// Every render mode, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::FullParity,
        Self::StructuredFidelityNarrowed,
        Self::RawFallbackDisclosed,
        Self::RedactionNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullParity => "full_parity",
            Self::StructuredFidelityNarrowed => "structured_fidelity_narrowed",
            Self::RawFallbackDisclosed => "raw_fallback_disclosed",
            Self::RedactionNarrowed => "redaction_narrowed",
        }
    }

    /// Whether this mode narrows below full parity.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::FullParity)
    }
}

/// Why a surface narrowed its rendering of a shared component.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewComponentNarrowReason {
    /// Structured coverage is partial or the render is not fully trusted.
    StructuredFidelityDegraded,
    /// No parser/schema recognizes the artifact; the raw / export-safe fallback shows.
    StructuredModeUnavailableRawFallback,
    /// Content is redacted or withheld under the export/redaction posture.
    ContentRedactedOrWithheld,
}

impl ArtifactReviewComponentNarrowReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredFidelityDegraded => "structured_fidelity_degraded",
            Self::StructuredModeUnavailableRawFallback => {
                "structured_mode_unavailable_raw_fallback"
            }
            Self::ContentRedactedOrWithheld => "content_redacted_or_withheld",
        }
    }
}

/// The next action a narrow banner offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewComponentNarrowNextAction {
    /// Open the raw / export-safe fallback view.
    OpenRawExportSafeFallback,
    /// Review the parser / schema state.
    ReviewParserSchemaState,
    /// Review the redaction / export posture.
    ReviewRedactionPosture,
}

impl ArtifactReviewComponentNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenRawExportSafeFallback => "open_raw_export_safe_fallback",
            Self::ReviewParserSchemaState => "review_parser_schema_state",
            Self::ReviewRedactionPosture => "review_redaction_posture",
        }
    }
}

/// Whether a binding preserves full parity or discloses a narrowed rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewComponentParityState {
    /// All parity facets are preserved and shown in full.
    FacetsPreserved,
    /// All parity facets are preserved, and a narrowing is explicitly disclosed.
    FacetsDisclosedNarrowed,
}

impl ArtifactReviewComponentParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FacetsPreserved => "facets_preserved",
            Self::FacetsDisclosedNarrowed => "facets_disclosed_narrowed",
        }
    }
}

/// Downgrade trigger that can narrow this consumer lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewComponentConsumerDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// The artifact parser is unavailable on this build.
    ParserUnavailable,
    /// No schema recognizes the artifact class.
    SchemaUnrecognized,
    /// The rendered preview is not trusted.
    RenderUntrusted,
    /// Redaction was applied and narrows visible content.
    RedactionApplied,
    /// Parity drift was detected between surfaces for the same object.
    ParityDriftDetected,
    /// Consumer trust narrowed.
    TrustNarrowing,
    /// An upstream shared component narrowed.
    UpstreamComponentNarrowed,
}

impl ArtifactReviewComponentConsumerDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ParserUnavailable,
        Self::SchemaUnrecognized,
        Self::RenderUntrusted,
        Self::RedactionApplied,
        Self::ParityDriftDetected,
        Self::TrustNarrowing,
        Self::UpstreamComponentNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ParserUnavailable => "parser_unavailable",
            Self::SchemaUnrecognized => "schema_unrecognized",
            Self::RenderUntrusted => "render_untrusted",
            Self::RedactionApplied => "redaction_applied",
            Self::ParityDriftDetected => "parity_drift_detected",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamComponentNarrowed => "upstream_component_narrowed",
        }
    }
}

/// The parity facet values a shared component presents for one artifact object.
///
/// These four values must be identical across every consumer surface that shows the
/// same artifact object. A surface may narrow how much it renders, but it may never
/// reword any of these values per surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewComponentParityFacetValues {
    /// Artifact-class / canonical-source label (never reworded per surface).
    pub canonical_source_label: String,
    /// Primary diff-mode / action (identical across surfaces).
    pub mode_action: String,
    /// Compare risk / status language (identical across surfaces).
    pub risk_status_language: String,
    /// Generated-from / source-of-truth provenance relation (identical across surfaces).
    pub provenance_relation: String,
}

impl ArtifactReviewComponentParityFacetValues {
    /// Whether every parity facet value is present.
    pub fn all_present(&self) -> bool {
        !self.canonical_source_label.trim().is_empty()
            && !self.mode_action.trim().is_empty()
            && !self.risk_status_language.trim().is_empty()
            && !self.provenance_relation.trim().is_empty()
    }
}

/// The explicit banner a narrowed surface shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewComponentNarrowBanner {
    /// Why the surface narrowed.
    pub reason: ArtifactReviewComponentNarrowReason,
    /// Note naming the preserved parity facets (never omitted).
    pub preserved_facets_note: String,
    /// The next action offered.
    pub next_action: ArtifactReviewComponentNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// Disclosures a consumer binding must carry, derived from its render fidelity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactReviewComponentRenderDisclosure {
    /// The render mode the fidelity state requires.
    pub expected_mode: ArtifactReviewComponentRenderMode,
    /// The narrow reason the render mode requires, if any.
    pub narrow_reason: Option<ArtifactReviewComponentNarrowReason>,
    /// Whether the binding must carry an explicit narrow banner.
    pub needs_narrow_banner: bool,
    /// Whether the binding must carry an explicit raw / export-safe fallback note.
    pub needs_raw_fallback_note: bool,
    /// Whether the binding must carry an explicit redaction-posture note.
    pub needs_redaction_note: bool,
}

/// Resolves the render disclosures a consumer binding must carry from its fidelity.
///
/// Faithful structure renders at full parity. Partial structure or an untrusted
/// render narrows structured fidelity while keeping every parity facet and an
/// explicit raw / export-safe fallback. A schema-unrecognized artifact or an
/// explicit raw fallback discloses the raw / export-safe view rather than pretending
/// to be faithful structure. Redacted or withheld content narrows through an
/// explicit redaction-posture note. In every case the artifact stays visible with
/// its canonical-source and compare-only posture intact.
pub fn resolve_artifact_component_render_disclosure(
    fidelity: M5ArtifactFidelityState,
) -> ArtifactReviewComponentRenderDisclosure {
    let (expected_mode, narrow_reason) = match fidelity {
        M5ArtifactFidelityState::StructuredFaithful => {
            (ArtifactReviewComponentRenderMode::FullParity, None)
        }
        M5ArtifactFidelityState::StructuredPartial | M5ArtifactFidelityState::RenderUntrusted => (
            ArtifactReviewComponentRenderMode::StructuredFidelityNarrowed,
            Some(ArtifactReviewComponentNarrowReason::StructuredFidelityDegraded),
        ),
        M5ArtifactFidelityState::SchemaUnrecognized | M5ArtifactFidelityState::RawFallback => (
            ArtifactReviewComponentRenderMode::RawFallbackDisclosed,
            Some(ArtifactReviewComponentNarrowReason::StructuredModeUnavailableRawFallback),
        ),
        M5ArtifactFidelityState::RedactedOrWithheld => (
            ArtifactReviewComponentRenderMode::RedactionNarrowed,
            Some(ArtifactReviewComponentNarrowReason::ContentRedactedOrWithheld),
        ),
    };

    // The raw / export-safe fallback must stay explicit whenever render or schema
    // fidelity narrows below faithful structure (spec guardrail). Redaction narrows
    // through its own redaction-posture note instead.
    let needs_raw_fallback_note = matches!(
        fidelity,
        M5ArtifactFidelityState::StructuredPartial
            | M5ArtifactFidelityState::RenderUntrusted
            | M5ArtifactFidelityState::SchemaUnrecognized
            | M5ArtifactFidelityState::RawFallback
    );
    let needs_redaction_note = matches!(fidelity, M5ArtifactFidelityState::RedactedOrWithheld);

    ArtifactReviewComponentRenderDisclosure {
        expected_mode,
        narrow_reason,
        needs_narrow_banner: expected_mode.is_narrowed(),
        needs_raw_fallback_note,
        needs_redaction_note,
    }
}

/// The parity state a render mode requires.
pub const fn parity_state_for_mode(
    mode: ArtifactReviewComponentRenderMode,
) -> ArtifactReviewComponentParityState {
    match mode {
        ArtifactReviewComponentRenderMode::FullParity => {
            ArtifactReviewComponentParityState::FacetsPreserved
        }
        ArtifactReviewComponentRenderMode::StructuredFidelityNarrowed
        | ArtifactReviewComponentRenderMode::RawFallbackDisclosed
        | ArtifactReviewComponentRenderMode::RedactionNarrowed => {
            ArtifactReviewComponentParityState::FacetsDisclosedNarrowed
        }
    }
}

/// One consumer binding: a shared component rendered on one consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewComponentConsumerBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable artifact-object id (shared across surfaces that show the same object).
    pub artifact_object_id: String,
    /// Human-readable artifact-object identity.
    pub artifact_object_label: String,
    /// Which shared component this binding renders.
    pub component: M5ArtifactComponent,
    /// Which consumer surface renders it.
    pub consumer: ArtifactReviewComponentConsumer,
    /// Render/schema fidelity state, reused from the frozen component matrix.
    pub render_fidelity: M5ArtifactFidelityState,
    /// How much of the component this surface renders.
    pub render_mode: ArtifactReviewComponentRenderMode,
    /// The parity facet values presented (identical across surfaces for one object).
    pub parity_facets: ArtifactReviewComponentParityFacetValues,
    /// Whether facets are preserved in full or a narrowing is disclosed.
    pub parity_state: ArtifactReviewComponentParityState,
    /// The explicit narrow banner; required and complete when the binding narrows.
    pub narrow_banner: Option<ArtifactReviewComponentNarrowBanner>,
    /// Raw / export-safe fallback note; required and non-empty when the disclosure demands it.
    pub raw_fallback_note: String,
    /// Redaction-posture note; required and non-empty when the disclosure demands it.
    pub redaction_note: String,
    /// Guardrail: this surface silently promotes a compare-only artifact to writable state.
    pub promotes_compare_only_to_writable_state: bool,
    /// Guardrail: this surface flattens a structured mode into a raw fallback without explanation.
    pub flattens_structured_mode_without_explanation: bool,
    /// Guardrail: this surface hides the generated-from relation behind generic file chrome.
    pub hides_generated_from_relation_behind_generic_chrome: bool,
    /// Guardrail: this surface drops the raw / export-safe fallback when fidelity narrows.
    pub drops_raw_or_export_safe_fallback: bool,
    /// Guardrail: this surface rewords the parity labels per surface.
    pub rewords_artifact_labels_per_surface: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl ArtifactReviewComponentConsumerBinding {
    /// Disclosures this binding must carry, derived from its render fidelity.
    pub fn disclosure(&self) -> ArtifactReviewComponentRenderDisclosure {
        resolve_artifact_component_render_disclosure(self.render_fidelity)
    }

    /// Whether this binding renders below full parity.
    pub fn is_narrowed(&self) -> bool {
        self.render_mode.is_narrowed()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub fn guardrails_hold(&self) -> bool {
        !self.promotes_compare_only_to_writable_state
            && !self.flattens_structured_mode_without_explanation
            && !self.hides_generated_from_relation_behind_generic_chrome
            && !self.drops_raw_or_export_safe_fallback
            && !self.rewords_artifact_labels_per_surface
    }

    /// Whether this binding points at the canonical component schema and matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let component_ref = component_canonical_schema_ref(self.component);
        self.source_contract_refs
            .iter()
            .any(|reference| reference == component_ref)
            && self.source_contract_refs.iter().any(|reference| {
                reference == ARTIFACT_REVIEW_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF
            })
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewComponentConsumerTrustReview {
    /// Component reuse is proven by fixtures rather than inferred from screenshots.
    pub component_reuse_proven_by_fixtures: bool,
    /// The same artifact object presents the same language across surfaces.
    pub same_object_same_language_across_surfaces: bool,
    /// Compare-only artifacts are never silently promoted to writable state.
    pub compare_only_never_silently_writable: bool,
    /// Structured modes are never flattened into raw fallback without explanation.
    pub structured_mode_never_flattened_without_explanation: bool,
    /// The generated-from / source-of-truth relation is never hidden behind generic chrome.
    pub generated_from_relation_never_hidden: bool,
    /// Canonical-source labels are identical across surfaces.
    pub canonical_source_labels_identical_across_surfaces: bool,
    /// Compare risk / status language is identical across surfaces.
    pub risk_status_language_identical_across_surfaces: bool,
    /// The raw / export-safe fallback stays explicit when fidelity narrows.
    pub raw_export_safe_fallback_kept_explicit: bool,
    /// The redaction / export posture stays explicit.
    pub redaction_posture_kept_explicit: bool,
    /// Help, support, and export consumers point at the canonical contracts.
    pub help_support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the component.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl ArtifactReviewComponentConsumerTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.component_reuse_proven_by_fixtures
            && self.same_object_same_language_across_surfaces
            && self.compare_only_never_silently_writable
            && self.structured_mode_never_flattened_without_explanation
            && self.generated_from_relation_never_hidden
            && self.canonical_source_labels_identical_across_surfaces
            && self.risk_status_language_identical_across_surfaces
            && self.raw_export_safe_fallback_kept_explicit
            && self.redaction_posture_kept_explicit
            && self.help_support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewComponentConsumerProjection {
    /// The diff toolbar reuses the shared components.
    pub diff_toolbar_reuses_shared_components: bool,
    /// The merge sheet reuses the shared components.
    pub merge_sheet_reuses_shared_components: bool,
    /// The review workspace reuses the shared components.
    pub review_workspace_reuses_shared_components: bool,
    /// The Help surface reuses the shared components.
    pub help_surface_reuses_shared_components: bool,
    /// The support packet reuses the shared components.
    pub support_packet_reuses_shared_components: bool,
    /// The exported view reuses the shared components.
    pub exported_view_reuses_shared_components: bool,
    /// Every component is adopted by two or more consumers.
    pub every_component_adopted_by_two_or_more_consumers: bool,
    /// Parity facets are identical for the same artifact object.
    pub parity_facets_identical_for_same_object: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export preserves canonical-source and compare-only posture.
    pub export_preserves_canonical_source_and_compare_only_posture: bool,
}

impl ArtifactReviewComponentConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.diff_toolbar_reuses_shared_components
            && self.merge_sheet_reuses_shared_components
            && self.review_workspace_reuses_shared_components
            && self.help_surface_reuses_shared_components
            && self.support_packet_reuses_shared_components
            && self.exported_view_reuses_shared_components
            && self.every_component_adopted_by_two_or_more_consumers
            && self.parity_facets_identical_for_same_object
            && self.narrowing_disclosed_not_hidden
            && self.export_preserves_canonical_source_and_compare_only_posture
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewComponentConsumerProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ArtifactReviewComponentConsumerPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactReviewComponentConsumerPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ArtifactReviewComponentConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ArtifactReviewComponentConsumerDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<ArtifactReviewComponentConsumer>,
    /// Trust review block.
    pub trust_review: ArtifactReviewComponentConsumerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ArtifactReviewComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ArtifactReviewComponentConsumerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe artifact-review-component consumer parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewComponentConsumerPacket {
    /// Record kind; must equal [`ARTIFACT_REVIEW_COMPONENT_CONSUMER_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`ARTIFACT_REVIEW_COMPONENT_CONSUMER_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Consumer bindings.
    pub consumer_bindings: Vec<ArtifactReviewComponentConsumerBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ArtifactReviewComponentConsumerDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<ArtifactReviewComponentConsumer>,
    /// Trust review block.
    pub trust_review: ArtifactReviewComponentConsumerTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ArtifactReviewComponentConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ArtifactReviewComponentConsumerProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ArtifactReviewComponentConsumerPacket {
    /// Builds an artifact-review-component consumer packet from stable-lane input.
    pub fn new(input: ArtifactReviewComponentConsumerPacketInput) -> Self {
        Self {
            record_kind: ARTIFACT_REVIEW_COMPONENT_CONSUMER_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_REVIEW_COMPONENT_CONSUMER_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            consumer_bindings: input.consumer_bindings,
            downgrade_triggers: input.downgrade_triggers,
            consumer_surfaces: input.consumer_surfaces,
            trust_review: input.trust_review,
            consumer_projection: input.consumer_projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the artifact-review-component consumer parity invariants.
    pub fn validate(&self) -> Vec<ArtifactReviewComponentConsumerViolation> {
        let mut violations = Vec::new();

        if self.record_kind != ARTIFACT_REVIEW_COMPONENT_CONSUMER_RECORD_KIND {
            violations.push(ArtifactReviewComponentConsumerViolation::WrongRecordKind);
        }
        if self.schema_version != ARTIFACT_REVIEW_COMPONENT_CONSUMER_SCHEMA_VERSION {
            violations.push(ArtifactReviewComponentConsumerViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ArtifactReviewComponentConsumerViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ArtifactReviewComponentConsumerViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ArtifactReviewComponentConsumerViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(ArtifactReviewComponentConsumerViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ArtifactReviewComponentConsumerViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ArtifactReviewComponentConsumerViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("artifact-review-component consumer packet serializes"),
        ) {
            violations.push(ArtifactReviewComponentConsumerViolation::RawBoundaryMaterialInExport);
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
            .expect("artifact-review-component consumer packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .consumer_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str(
            "# Shared Artifact-Review Component Consumers: Mode, Risk, and Provenance Parity\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Consumer bindings: {} ({} narrowed)\n",
            self.consumer_bindings.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Consumer bindings\n\n");
        for binding in &self.consumer_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: component `{}` on `{}`, mode `{}`\n",
                binding.artifact_object_label,
                binding.binding_id,
                binding.component.as_str(),
                binding.consumer.as_str(),
                binding.render_mode.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in artifact-review consumer export.
#[derive(Debug)]
pub enum ArtifactReviewComponentConsumerArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ArtifactReviewComponentConsumerViolation>),
}

impl fmt::Display for ArtifactReviewComponentConsumerArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "artifact-review-component consumer export parse failed: {error}"
                )
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(
                    formatter,
                    "artifact-review-component consumer export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ArtifactReviewComponentConsumerArtifactError {}

/// Validation failures emitted by [`ArtifactReviewComponentConsumerPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactReviewComponentConsumerViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No consumer bindings are present.
    ConsumerBindingsMissing,
    /// A consumer binding is incomplete.
    BindingIncomplete,
    /// A binding's parity facet values are incomplete.
    ParityFacetIncomplete,
    /// A binding's render mode does not match its render fidelity.
    RenderModeMismatch,
    /// A binding's parity state does not match its render mode.
    ParityStateMismatch,
    /// Two surfaces show the same artifact object with different parity language.
    ParityDriftAcrossSurfaces,
    /// A shared component is not adopted by at least two distinct consumers.
    ArtifactComponentReuseUnproven,
    /// A Help, support, or export binding does not point at the canonical contracts.
    HelpSupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit narrow banner.
    NarrowBannerMissing,
    /// A narrow banner's reason does not match the required narrow reason.
    NarrowReasonMismatch,
    /// A narrow banner is missing its preserved-facets note.
    NarrowBannerPreservedFacetsMissing,
    /// A narrow banner is missing its next-action copy.
    NarrowNextActionMissing,
    /// A binding that must keep a raw / export-safe fallback is missing its note.
    RawFallbackNoteMissing,
    /// A binding that needs an explicit redaction-posture note is missing it.
    RedactionNoteMissing,
    /// A binding silently promotes a compare-only artifact to writable state.
    CompareOnlyPromotedToWritable,
    /// A binding flattens a structured mode into a raw fallback without explanation.
    StructuredModeFlattenedWithoutExplanation,
    /// A binding hides the generated-from relation behind generic file chrome.
    GeneratedFromRelationHiddenBehindGenericChrome,
    /// A binding drops the raw / export-safe fallback when fidelity narrows.
    RawOrExportSafeFallbackDropped,
    /// A binding rewords the parity labels per surface.
    ArtifactLabelsRewordedPerSurface,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared component appears among the bindings.
    ComponentCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No consumer surfaces are present.
    ConsumerSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ArtifactReviewComponentConsumerViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::ConsumerBindingsMissing => "consumer_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::ParityFacetIncomplete => "parity_facet_incomplete",
            Self::RenderModeMismatch => "render_mode_mismatch",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::ParityDriftAcrossSurfaces => "parity_drift_across_surfaces",
            Self::ArtifactComponentReuseUnproven => "artifact_component_reuse_unproven",
            Self::HelpSupportExportReferenceMissing => "help_support_export_reference_missing",
            Self::NarrowBannerMissing => "narrow_banner_missing",
            Self::NarrowReasonMismatch => "narrow_reason_mismatch",
            Self::NarrowBannerPreservedFacetsMissing => "narrow_banner_preserved_facets_missing",
            Self::NarrowNextActionMissing => "narrow_next_action_missing",
            Self::RawFallbackNoteMissing => "raw_fallback_note_missing",
            Self::RedactionNoteMissing => "redaction_note_missing",
            Self::CompareOnlyPromotedToWritable => "compare_only_promoted_to_writable",
            Self::StructuredModeFlattenedWithoutExplanation => {
                "structured_mode_flattened_without_explanation"
            }
            Self::GeneratedFromRelationHiddenBehindGenericChrome => {
                "generated_from_relation_hidden_behind_generic_chrome"
            }
            Self::RawOrExportSafeFallbackDropped => "raw_or_export_safe_fallback_dropped",
            Self::ArtifactLabelsRewordedPerSurface => "artifact_labels_reworded_per_surface",
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable artifact-review consumer export.
pub fn current_artifact_review_component_consumer_export(
) -> Result<ArtifactReviewComponentConsumerPacket, ArtifactReviewComponentConsumerArtifactError> {
    let packet: ArtifactReviewComponentConsumerPacket =
        serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-structured-artifact-review-consumers-proof/support_export.json"
    )))
        .map_err(ArtifactReviewComponentConsumerArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ArtifactReviewComponentConsumerArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &ArtifactReviewComponentConsumerPacket,
    violations: &mut Vec<ArtifactReviewComponentConsumerViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_SCHEMA_REF,
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_DOC_REF,
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_COMPONENT_MATRIX_CONTRACT_REF,
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_IDENTITY_DIFF_CONTROLS_CONTRACT_REF,
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_STRUCTURE_COMPARE_CONTROLS_CONTRACT_REF,
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_MERGE_GENERATED_CONTROLS_CONTRACT_REF,
        ARTIFACT_REVIEW_COMPONENT_CONSUMER_MEDIA_TRUST_CONTROLS_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ArtifactReviewComponentConsumerViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &ArtifactReviewComponentConsumerPacket,
    violations: &mut Vec<ArtifactReviewComponentConsumerViolation>,
) {
    if packet.consumer_bindings.is_empty() {
        violations.push(ArtifactReviewComponentConsumerViolation::ConsumerBindingsMissing);
        return;
    }

    // Parity: the parity facet values must be identical for every binding that
    // renders the same artifact object.
    let mut object_facets: BTreeMap<&str, &ArtifactReviewComponentParityFacetValues> =
        BTreeMap::new();
    let mut parity_drift_reported = false;

    // Reuse: each component must be adopted by at least two distinct consumers.
    let mut component_consumers: BTreeMap<
        M5ArtifactComponent,
        BTreeSet<ArtifactReviewComponentConsumer>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<ArtifactReviewComponentConsumer> = BTreeSet::new();
    let mut seen_components: BTreeSet<M5ArtifactComponent> = BTreeSet::new();

    for binding in &packet.consumer_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.artifact_object_id.trim().is_empty()
            || binding.artifact_object_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(ArtifactReviewComponentConsumerViolation::BindingIncomplete);
        }
        if !binding.parity_facets.all_present() {
            violations.push(ArtifactReviewComponentConsumerViolation::ParityFacetIncomplete);
        }

        let disclosure = binding.disclosure();

        if binding.render_mode != disclosure.expected_mode {
            violations.push(ArtifactReviewComponentConsumerViolation::RenderModeMismatch);
        }
        if binding.parity_state != parity_state_for_mode(binding.render_mode) {
            violations.push(ArtifactReviewComponentConsumerViolation::ParityStateMismatch);
        }

        // Narrowing disclosure.
        if disclosure.needs_narrow_banner {
            match &binding.narrow_banner {
                None => {
                    violations.push(ArtifactReviewComponentConsumerViolation::NarrowBannerMissing);
                }
                Some(banner) => {
                    if Some(banner.reason) != disclosure.narrow_reason {
                        violations
                            .push(ArtifactReviewComponentConsumerViolation::NarrowReasonMismatch);
                    }
                    if banner.preserved_facets_note.trim().is_empty() {
                        violations.push(
                            ArtifactReviewComponentConsumerViolation::NarrowBannerPreservedFacetsMissing,
                        );
                    }
                    if banner.next_action_label.trim().is_empty() {
                        violations.push(
                            ArtifactReviewComponentConsumerViolation::NarrowNextActionMissing,
                        );
                    }
                }
            }
        } else if binding.narrow_banner.is_some() {
            // A full-parity binding must not carry a narrow banner.
            violations.push(ArtifactReviewComponentConsumerViolation::NarrowBannerMissing);
        }

        if disclosure.needs_raw_fallback_note && binding.raw_fallback_note.trim().is_empty() {
            violations.push(ArtifactReviewComponentConsumerViolation::RawFallbackNoteMissing);
        }
        if disclosure.needs_redaction_note && binding.redaction_note.trim().is_empty() {
            violations.push(ArtifactReviewComponentConsumerViolation::RedactionNoteMissing);
        }

        // Guardrail row-invariants (each must be false).
        if binding.promotes_compare_only_to_writable_state {
            violations
                .push(ArtifactReviewComponentConsumerViolation::CompareOnlyPromotedToWritable);
        }
        if binding.flattens_structured_mode_without_explanation {
            violations.push(
                ArtifactReviewComponentConsumerViolation::StructuredModeFlattenedWithoutExplanation,
            );
        }
        if binding.hides_generated_from_relation_behind_generic_chrome {
            violations.push(
                ArtifactReviewComponentConsumerViolation::GeneratedFromRelationHiddenBehindGenericChrome,
            );
        }
        if binding.drops_raw_or_export_safe_fallback {
            violations
                .push(ArtifactReviewComponentConsumerViolation::RawOrExportSafeFallbackDropped);
        }
        if binding.rewords_artifact_labels_per_surface {
            violations
                .push(ArtifactReviewComponentConsumerViolation::ArtifactLabelsRewordedPerSurface);
        }

        // Help / support / export consumers must point at the canonical contracts.
        if binding.consumer.is_help_support_or_export() && !binding.points_at_canonical_contracts()
        {
            violations
                .push(ArtifactReviewComponentConsumerViolation::HelpSupportExportReferenceMissing);
        }

        // Parity drift accumulation.
        match object_facets.get(binding.artifact_object_id.as_str()) {
            None => {
                object_facets.insert(binding.artifact_object_id.as_str(), &binding.parity_facets);
            }
            Some(existing) => {
                if **existing != binding.parity_facets && !parity_drift_reported {
                    violations
                        .push(ArtifactReviewComponentConsumerViolation::ParityDriftAcrossSurfaces);
                    parity_drift_reported = true;
                }
            }
        }

        component_consumers
            .entry(binding.component)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_components.insert(binding.component);
    }

    // Coverage: every consumer and every component must appear.
    for consumer in ArtifactReviewComponentConsumer::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(ArtifactReviewComponentConsumerViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for component in M5ArtifactComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(ArtifactReviewComponentConsumerViolation::ComponentCoverageMissing);
            break;
        }
    }

    // Reuse: every present component must be adopted by two or more distinct consumers.
    for consumers in component_consumers.values() {
        if consumers.len() < 2 {
            violations
                .push(ArtifactReviewComponentConsumerViolation::ArtifactComponentReuseUnproven);
            break;
        }
    }
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
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
