//! Keyboard, screen-reader, CLI, and export parity plus automatic claim narrowing
//! for the nine shared M5 structured-artifact review components.
//!
//! This module is the accessibility / headless / export capstone over the
//! structured-artifact review components frozen in
//! [`crate::freeze_the_m5_structured_artifact_review_component_matrix`], implemented
//! by the artifact-identity / diff-mode, structure-row / compare-summary,
//! merge-decision / generated-notice, and rendered-compare / media-rail /
//! redaction-trust-badge lanes, and adopted by the shared consumers in
//! [`crate::add_shared_diff_toolbar_merge_sheet_review_workspace_help_support_and_export_consumers_so_artifact_review_components_keep_mode_risk_and_provenance_language_aligned`].
//! Where the consumer lane proves mode / risk / provenance parity across desktop
//! surfaces, this lane proves the harder claim: that artifact class, diff mode,
//! structure, rendered-compare fallback, and merge-decision state is exposed just as
//! honestly in assistive, headless, and exported forms as it is on the desktop — and
//! that a claim-bearing component automatically narrows the moment its parser/schema
//! certainty, render trust, write-back safety, or metadata availability stops being
//! trustworthy.
//!
//! The honesty axes are two. First, parity across forms: every claimed component must
//! expose a keyboard label, a screen-reader label, a CLI enum token, an export enum
//! token, and a human-readable explanation field, and must render on the desktop, the
//! headless CLI, and the support export alike. No component may be pointer-only,
//! export-opaque, or semantically stronger on the desktop than it is in CLI or support
//! output.
//!
//! Second, automatic narrowing: each component carries a claim about how much
//! structured or rendered fidelity it asserts, drawn from
//! [`ArtifactReviewClaimTier`]. When parser/schema certainty is uncertain, when render
//! trust is unavailable, when merge/write-back safety is unavailable, or when metadata
//! availability is stale or policy-blocked, the claim must narrow to the ceiling
//! permitted by that condition ([`ArtifactReviewClaimCondition::permitted_ceiling`]),
//! disclose the narrowing through an explicit trigger and next action, keep the
//! raw/export-safe fallback explicit, never promote a compare-only artifact to a
//! writable state, and keep redacted or withheld metadata labeled. A component may
//! never keep asserting full structured fidelity while one of those conditions holds.
//!
//! The packet references upstream component and consumer contracts by id rather than
//! embedding their content. Raw artifact payloads, credentials, and live provider
//! responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-structured-artifact-review-component-accessibility-parity.schema.json`](../../../../schemas/ui/m5-structured-artifact-review-component-accessibility-parity.schema.json).
//! The contract doc is
//! [`docs/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_parser_schema_render_trust_or_write_back_safety_is_unavailable_across_claimed_m5_structured_artifact_review_components.md`](../../../../docs/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_parser_schema_render_trust_or_write_back_safety_is_unavailable_across_claimed_m5_structured_artifact_review_components.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-structured-artifact-review-component-accessibility-parity/`](../../../../fixtures/ui/m5-structured-artifact-review-component-accessibility-parity/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_structured_artifact_review_component_matrix::M5ArtifactComponent;

/// Stable record-kind tag carried by [`ArtifactReviewAccessibilityPacket`].
pub const M5_ARTIFACT_REVIEW_ACCESSIBILITY_RECORD_KIND: &str =
    "structured_artifact_review_component_accessibility_parity_truth";

/// Schema version for structured-artifact review accessibility parity records.
pub const M5_ARTIFACT_REVIEW_ACCESSIBILITY_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const M5_ARTIFACT_REVIEW_ACCESSIBILITY_SCHEMA_REF: &str =
    "schemas/ui/m5-structured-artifact-review-component-accessibility-parity.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_ARTIFACT_REVIEW_ACCESSIBILITY_DOC_REF: &str =
    "docs/review/m5/implement_keyboard_screen_reader_cli_export_parity_and_automatic_narrowing_when_parser_schema_render_trust_or_write_back_safety_is_unavailable_across_claimed_m5_structured_artifact_review_components.md";

/// Repo-relative path of the frozen component matrix these claims exercise.
pub const M5_ARTIFACT_REVIEW_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF: &str =
    "schemas/ui/m5-structured-artifact-review-component-matrix.schema.json";

/// Repo-relative path of the shared-consumer parity contract this capstone extends.
pub const M5_ARTIFACT_REVIEW_ACCESSIBILITY_CONSUMER_CONTRACT_REF: &str =
    "schemas/ui/m5-structured-artifact-review-component-consumer.schema.json";

/// Repo-relative path of the artifact-identity / diff-mode controls contract.
pub const M5_ARTIFACT_REVIEW_ACCESSIBILITY_IDENTITY_DIFF_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-artifact-identity-diff-mode-controls.schema.json";

/// Repo-relative path of the structure-row / compare-summary controls contract.
pub const M5_ARTIFACT_REVIEW_ACCESSIBILITY_STRUCTURE_COMPARE_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-structure-compare-summary-controls.schema.json";

/// Repo-relative path of the merge-decision / generated-notice controls contract.
pub const M5_ARTIFACT_REVIEW_ACCESSIBILITY_MERGE_GENERATED_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-merge-decision-generated-notice-controls.schema.json";

/// Repo-relative path of the rendered-compare / media-rail / redaction-trust controls contract.
pub const M5_ARTIFACT_REVIEW_ACCESSIBILITY_MEDIA_TRUST_CONTROLS_CONTRACT_REF: &str =
    "schemas/ui/m5-rendered-compare-media-trust-controls.schema.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_ARTIFACT_REVIEW_ACCESSIBILITY_FIXTURE_DIR: &str =
    "fixtures/ui/m5-structured-artifact-review-component-accessibility-parity";

/// Repo-relative path of the checked support-export artifact.
pub const M5_ARTIFACT_REVIEW_ACCESSIBILITY_ARTIFACT_REF: &str =
    "artifacts/release/m5-structured-artifact-review-accessibility-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const M5_ARTIFACT_REVIEW_ACCESSIBILITY_SUMMARY_REF: &str =
    "artifacts/release/m5-structured-artifact-review-accessibility-proof/summary.md";

/// Canonical component contract that a row must point at for a given component.
///
/// Each of the nine shared components resolves to the checked-in schema of the
/// implement lane that produced it: the artifact-identity / diff-mode controls, the
/// structure-row / compare-summary controls, the merge-decision / generated-notice
/// controls, and the rendered-compare / media-rail / redaction-trust-badge controls.
pub const fn component_canonical_schema_ref(component: M5ArtifactComponent) -> &'static str {
    match component {
        M5ArtifactComponent::ArtifactIdentityBar | M5ArtifactComponent::DiffModeSwitcher => {
            M5_ARTIFACT_REVIEW_ACCESSIBILITY_IDENTITY_DIFF_CONTROLS_CONTRACT_REF
        }
        M5ArtifactComponent::StructureRow | M5ArtifactComponent::CompareSummaryCard => {
            M5_ARTIFACT_REVIEW_ACCESSIBILITY_STRUCTURE_COMPARE_CONTROLS_CONTRACT_REF
        }
        M5ArtifactComponent::MergeDecisionRow | M5ArtifactComponent::GeneratedArtifactNotice => {
            M5_ARTIFACT_REVIEW_ACCESSIBILITY_MERGE_GENERATED_CONTROLS_CONTRACT_REF
        }
        M5ArtifactComponent::RenderedCompareViewer
        | M5ArtifactComponent::MediaMetadataRail
        | M5ArtifactComponent::RedactionOrTrustBadgeSet => {
            M5_ARTIFACT_REVIEW_ACCESSIBILITY_MEDIA_TRUST_CONTROLS_CONTRACT_REF
        }
    }
}

/// The condition governing how much structured/rendered fidelity a component may claim.
///
/// [`StructuredTruthTrusted`](Self::StructuredTruthTrusted) is the baseline where the
/// full structured-fidelity claim is permitted. The other four are the weakening
/// conditions named by the spec: an uncertain parser/schema, unavailable render trust,
/// unavailable merge/write-back safety, and stale or policy-blocked metadata
/// availability. Each weakening condition pins the claim to a ceiling.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewClaimCondition {
    /// Parser/schema, render trust, write-back safety, and metadata are all trusted.
    StructuredTruthTrusted,
    /// Parser/schema certainty is uncertain; structured coverage is partial.
    ParserSchemaUncertain,
    /// Render trust is unavailable; only a raw/export-safe fallback is trustworthy.
    RenderTrustUnavailable,
    /// Merge/write-back safety is unavailable; the artifact stays compare-only.
    WriteBackSafetyUnavailable,
    /// Metadata availability is stale, unavailable, or policy-blocked.
    MetadataUnavailable,
}

impl ArtifactReviewClaimCondition {
    /// Every condition, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::StructuredTruthTrusted,
        Self::ParserSchemaUncertain,
        Self::RenderTrustUnavailable,
        Self::WriteBackSafetyUnavailable,
        Self::MetadataUnavailable,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredTruthTrusted => "structured_truth_trusted",
            Self::ParserSchemaUncertain => "parser_schema_uncertain",
            Self::RenderTrustUnavailable => "render_trust_unavailable",
            Self::WriteBackSafetyUnavailable => "write_back_safety_unavailable",
            Self::MetadataUnavailable => "metadata_unavailable",
        }
    }

    /// Whether this condition weakens the structured-fidelity claim (everything but trusted).
    pub const fn is_weakening(self) -> bool {
        !matches!(self, Self::StructuredTruthTrusted)
    }

    /// The strongest claim tier this condition still permits.
    pub const fn permitted_ceiling(self) -> ArtifactReviewClaimTier {
        match self {
            Self::StructuredTruthTrusted => ArtifactReviewClaimTier::FullStructuredFidelity,
            Self::WriteBackSafetyUnavailable => ArtifactReviewClaimTier::StructuredCompareOnly,
            Self::ParserSchemaUncertain => ArtifactReviewClaimTier::PartialStructure,
            Self::RenderTrustUnavailable => ArtifactReviewClaimTier::RawFallbackDisclosed,
            Self::MetadataUnavailable => ArtifactReviewClaimTier::MetadataWithheld,
        }
    }

    /// The downgrade trigger a weakening condition must disclose, if any.
    pub const fn default_trigger(self) -> Option<ArtifactReviewAccessibilityDowngradeTrigger> {
        match self {
            Self::StructuredTruthTrusted => None,
            Self::ParserSchemaUncertain => {
                Some(ArtifactReviewAccessibilityDowngradeTrigger::ParserSchemaUncertain)
            }
            Self::RenderTrustUnavailable => {
                Some(ArtifactReviewAccessibilityDowngradeTrigger::RenderTrustUnavailable)
            }
            Self::WriteBackSafetyUnavailable => {
                Some(ArtifactReviewAccessibilityDowngradeTrigger::WriteBackSafetyUnavailable)
            }
            Self::MetadataUnavailable => {
                Some(ArtifactReviewAccessibilityDowngradeTrigger::MetadataAvailabilityUnavailable)
            }
        }
    }

    /// The next action a weakening condition's narrow disclosure must offer.
    pub const fn next_action(self) -> ArtifactReviewClaimNextAction {
        match self {
            Self::StructuredTruthTrusted => ArtifactReviewClaimNextAction::ContinueStructuredReview,
            Self::ParserSchemaUncertain => ArtifactReviewClaimNextAction::ReparseAgainstSchema,
            Self::RenderTrustUnavailable => ArtifactReviewClaimNextAction::ReviewRawSafeFallback,
            Self::WriteBackSafetyUnavailable => ArtifactReviewClaimNextAction::KeepCompareOnly,
            Self::MetadataUnavailable => ArtifactReviewClaimNextAction::RestoreMetadataAccess,
        }
    }
}

/// A component's claim about how much structured or rendered fidelity it asserts.
///
/// Ordered strongest to weakest. [`FullStructuredFidelity`](Self::FullStructuredFidelity)
/// is the only tier that asserts full semantic and rendered fidelity with write-back
/// safety; the rest are the honest fallbacks a weakening condition narrows to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewClaimTier {
    /// Full semantic and rendered fidelity with write-back safety.
    FullStructuredFidelity,
    /// Full structure, but compare-only: write-back is unavailable.
    StructuredCompareOnly,
    /// Structured mode covers only part of the artifact; parser/schema is uncertain.
    PartialStructure,
    /// An explicitly labeled raw/export-safe fallback; render trust is unavailable.
    RawFallbackDisclosed,
    /// Metadata or content is withheld or redacted under the export/redaction posture.
    MetadataWithheld,
}

impl ArtifactReviewClaimTier {
    /// Every tier, in declaration order (strongest first).
    pub const ALL: [Self; 5] = [
        Self::FullStructuredFidelity,
        Self::StructuredCompareOnly,
        Self::PartialStructure,
        Self::RawFallbackDisclosed,
        Self::MetadataWithheld,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FullStructuredFidelity => "full_structured_fidelity",
            Self::StructuredCompareOnly => "structured_compare_only",
            Self::PartialStructure => "partial_structure",
            Self::RawFallbackDisclosed => "raw_fallback_disclosed",
            Self::MetadataWithheld => "metadata_withheld",
        }
    }

    /// Strength rank, higher is stronger. Used for the ceiling comparison.
    pub const fn rank(self) -> u8 {
        match self {
            Self::FullStructuredFidelity => 5,
            Self::StructuredCompareOnly => 4,
            Self::PartialStructure => 3,
            Self::RawFallbackDisclosed => 2,
            Self::MetadataWithheld => 1,
        }
    }

    /// Whether this tier asserts full semantic and rendered fidelity.
    pub const fn asserts_full_structured_fidelity(self) -> bool {
        matches!(self, Self::FullStructuredFidelity)
    }
}

/// A rendering form the claim must reach with identical semantics.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewRenderingSurface {
    /// The full desktop surface.
    DesktopFull,
    /// The headless CLI.
    CliHeadless,
    /// The support export.
    SupportExport,
}

impl ArtifactReviewRenderingSurface {
    /// Every rendering surface, in declaration order.
    pub const ALL: [Self; 3] = [Self::DesktopFull, Self::CliHeadless, Self::SupportExport];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopFull => "desktop_full",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
        }
    }
}

/// The next action a narrow disclosure offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewClaimNextAction {
    /// Re-parse the artifact against a recognized schema.
    ReparseAgainstSchema,
    /// Review the explicit raw/export-safe fallback.
    ReviewRawSafeFallback,
    /// Keep the artifact compare-only; do not write back.
    KeepCompareOnly,
    /// Restore metadata access before relying on it.
    RestoreMetadataAccess,
    /// Continue the structured review.
    ContinueStructuredReview,
}

impl ArtifactReviewClaimNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReparseAgainstSchema => "reparse_against_schema",
            Self::ReviewRawSafeFallback => "review_raw_safe_fallback",
            Self::KeepCompareOnly => "keep_compare_only",
            Self::RestoreMetadataAccess => "restore_metadata_access",
            Self::ContinueStructuredReview => "continue_structured_review",
        }
    }
}

/// Downgrade trigger that can narrow this accessibility lane below its full claim.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewAccessibilityDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Parser/schema certainty is uncertain.
    ParserSchemaUncertain,
    /// Render trust is unavailable.
    RenderTrustUnavailable,
    /// Merge/write-back safety is unavailable.
    WriteBackSafetyUnavailable,
    /// Metadata availability is stale, unavailable, or policy-blocked.
    MetadataAvailabilityUnavailable,
    /// A claim was overstated relative to its permitted ceiling.
    ClaimOverstated,
    /// Parity across desktop, CLI, or export was dropped.
    ParityDropped,
    /// Consumer trust narrowed.
    TrustNarrowing,
}

impl ArtifactReviewAccessibilityDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ParserSchemaUncertain,
        Self::RenderTrustUnavailable,
        Self::WriteBackSafetyUnavailable,
        Self::MetadataAvailabilityUnavailable,
        Self::ClaimOverstated,
        Self::ParityDropped,
        Self::TrustNarrowing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ParserSchemaUncertain => "parser_schema_uncertain",
            Self::RenderTrustUnavailable => "render_trust_unavailable",
            Self::WriteBackSafetyUnavailable => "write_back_safety_unavailable",
            Self::MetadataAvailabilityUnavailable => "metadata_availability_unavailable",
            Self::ClaimOverstated => "claim_overstated",
            Self::ParityDropped => "parity_dropped",
            Self::TrustNarrowing => "trust_narrowing",
        }
    }
}

/// The disclosures an accessibility row must carry, derived from its condition.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactReviewClaimResolution {
    /// The strongest claim tier the condition permits.
    pub permitted_ceiling: ArtifactReviewClaimTier,
    /// Whether the condition requires an explicit narrow disclosure.
    pub requires_narrowing: bool,
    /// The downgrade trigger the narrow disclosure must name, if any.
    pub expected_trigger: Option<ArtifactReviewAccessibilityDowngradeTrigger>,
    /// The next action the narrow disclosure must offer.
    pub expected_next_action: ArtifactReviewClaimNextAction,
    /// Whether the row must carry an explicit raw/export-safe fallback note.
    pub needs_raw_fallback_note: bool,
    /// Whether the row must carry an explicit compare-only note.
    pub needs_compare_only_note: bool,
    /// Whether the row must carry an explicit redaction / withheld-metadata note.
    pub needs_redaction_note: bool,
}

/// Resolves the claim narrowing an accessibility row must carry from its condition.
///
/// Trusted structured truth keeps the full structured-fidelity claim. Each weakening
/// condition pins the claim to a ceiling, demands an explicit narrow disclosure naming
/// its trigger and next action, and keeps an explicit raw/export-safe fallback so the
/// artifact can always be reviewed. An unavailable write-back safety additionally
/// demands an explicit compare-only note rather than silently promoting the artifact to
/// a writable state, and unavailable metadata demands an explicit redaction note rather
/// than hiding withheld content behind generic chrome.
pub const fn resolve_artifact_review_claim_narrowing(
    condition: ArtifactReviewClaimCondition,
) -> ArtifactReviewClaimResolution {
    ArtifactReviewClaimResolution {
        permitted_ceiling: condition.permitted_ceiling(),
        requires_narrowing: condition.is_weakening(),
        expected_trigger: condition.default_trigger(),
        expected_next_action: condition.next_action(),
        needs_raw_fallback_note: condition.is_weakening(),
        needs_compare_only_note: matches!(
            condition,
            ArtifactReviewClaimCondition::WriteBackSafetyUnavailable
        ),
        needs_redaction_note: matches!(
            condition,
            ArtifactReviewClaimCondition::MetadataUnavailable
        ),
    }
}

/// The explicit narrow disclosure a claim-narrowed row shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewClaimNarrowing {
    /// The downgrade trigger the narrowing discloses.
    pub trigger: ArtifactReviewAccessibilityDowngradeTrigger,
    /// The claim tier the narrowing pins the component to.
    pub narrowed_to: ArtifactReviewClaimTier,
    /// Note naming the truth preserved through the narrowing (never omitted).
    pub preserved_truth_note: String,
    /// The next action offered.
    pub next_action: ArtifactReviewClaimNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// One accessibility row: a claimed component under one condition, exposed across
/// keyboard, screen-reader, CLI, and export forms.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewAccessibilityRow {
    /// Stable row id.
    pub row_id: String,
    /// Which shared component this row claims.
    pub component: M5ArtifactComponent,
    /// The condition governing the claim.
    pub condition: ArtifactReviewClaimCondition,
    /// The claim tier the component effectively asserts.
    pub effective_claim: ArtifactReviewClaimTier,
    /// Keyboard reach / operation label (never empty).
    pub keyboard_label: String,
    /// Screen-reader label (never empty).
    pub screen_reader_label: String,
    /// CLI enum token (never empty).
    pub cli_enum_token: String,
    /// Export enum token (never empty).
    pub export_enum_token: String,
    /// Human-readable explanation field (never empty).
    pub explanation_field: String,
    /// The rendering surfaces this row reaches (must cover all three).
    pub rendering_surfaces: Vec<ArtifactReviewRenderingSurface>,
    /// The explicit narrow disclosure; required and complete when the claim narrows.
    pub narrowing: Option<ArtifactReviewClaimNarrowing>,
    /// Raw/export-safe fallback note; required and non-empty when the claim narrows.
    pub raw_fallback_note: String,
    /// Compare-only note; required and non-empty when write-back safety is unavailable.
    pub compare_only_note: String,
    /// Redaction / withheld-metadata note; required and non-empty when metadata is unavailable.
    pub redaction_note: String,
    /// Guardrail: this component is reachable only by pointer.
    pub is_pointer_only: bool,
    /// Guardrail: this component omits itself from the export.
    pub is_export_opaque: bool,
    /// Guardrail: this component claims more on the desktop than in CLI or export.
    pub desktop_stronger_than_cli: bool,
    /// Source contract refs this row points at.
    pub source_contract_refs: Vec<String>,
}

impl ArtifactReviewAccessibilityRow {
    /// The disclosures this row must carry, derived from its condition.
    pub const fn resolution(&self) -> ArtifactReviewClaimResolution {
        resolve_artifact_review_claim_narrowing(self.condition)
    }

    /// Whether this row narrows below the full structured-fidelity claim.
    pub const fn is_narrowed(&self) -> bool {
        self.condition.is_weakening()
    }

    /// Whether this row reaches all three rendering surfaces.
    pub fn covers_all_rendering_surfaces(&self) -> bool {
        ArtifactReviewRenderingSurface::ALL
            .iter()
            .all(|surface| self.rendering_surfaces.contains(surface))
    }

    /// Whether every accessibility field is present.
    pub fn accessibility_fields_present(&self) -> bool {
        !self.keyboard_label.trim().is_empty()
            && !self.screen_reader_label.trim().is_empty()
            && !self.cli_enum_token.trim().is_empty()
            && !self.export_enum_token.trim().is_empty()
            && !self.explanation_field.trim().is_empty()
    }

    /// Whether every guardrail row-invariant is false, as required.
    pub const fn guardrails_hold(&self) -> bool {
        !self.is_pointer_only && !self.is_export_opaque && !self.desktop_stronger_than_cli
    }

    /// Whether this row points at the canonical component schema and matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let component_ref = component_canonical_schema_ref(self.component);
        self.source_contract_refs
            .iter()
            .any(|reference| reference == component_ref)
            && self.source_contract_refs.iter().any(|reference| {
                reference == M5_ARTIFACT_REVIEW_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF
            })
    }

    /// Whether the effective claim is honest under the row's condition: it never
    /// exceeds the permitted ceiling, and a weakening condition narrows the claim
    /// down to exactly that ceiling.
    pub fn claim_is_honest(&self) -> bool {
        let resolution = self.resolution();
        let ceiling = resolution.permitted_ceiling;
        if self.effective_claim.rank() > ceiling.rank() {
            return false;
        }
        if resolution.requires_narrowing {
            self.effective_claim == ceiling
                && self
                    .narrowing
                    .as_ref()
                    .is_some_and(|narrowing| narrowing.narrowed_to == ceiling)
        } else {
            self.effective_claim == ceiling && self.narrowing.is_none()
        }
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewAccessibilityTrustReview {
    /// Every claim is keyboard-reachable.
    pub keyboard_reachable_on_every_claim: bool,
    /// Every claim carries a screen-reader label.
    pub screen_reader_labeled_on_every_claim: bool,
    /// Every claim exposes a CLI enum token.
    pub cli_enum_exposed_on_every_claim: bool,
    /// Every claim exposes an export enum token.
    pub export_enum_exposed_on_every_claim: bool,
    /// Every claim carries an explanation field.
    pub explanation_field_present_on_every_claim: bool,
    /// No component is pointer-only.
    pub no_component_pointer_only: bool,
    /// No component is export-opaque.
    pub no_component_export_opaque: bool,
    /// No component claims more on the desktop than in CLI or export.
    pub desktop_never_stronger_than_cli: bool,
    /// The claim narrows whenever structured fidelity weakens.
    pub claim_narrows_when_structured_fidelity_weakens: bool,
    /// Structured fidelity is never overstated while a weakening condition holds.
    pub structured_fidelity_never_overstated_under_weakening: bool,
    /// The raw/export-safe fallback is kept explicit when fidelity narrows.
    pub raw_or_export_safe_fallback_kept_explicit: bool,
    /// A compare-only artifact is never promoted to a writable state.
    pub compare_only_never_promoted_to_writable_state: bool,
}

impl ArtifactReviewAccessibilityTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.keyboard_reachable_on_every_claim
            && self.screen_reader_labeled_on_every_claim
            && self.cli_enum_exposed_on_every_claim
            && self.export_enum_exposed_on_every_claim
            && self.explanation_field_present_on_every_claim
            && self.no_component_pointer_only
            && self.no_component_export_opaque
            && self.desktop_never_stronger_than_cli
            && self.claim_narrows_when_structured_fidelity_weakens
            && self.structured_fidelity_never_overstated_under_weakening
            && self.raw_or_export_safe_fallback_kept_explicit
            && self.compare_only_never_promoted_to_writable_state
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewAccessibilityProjection {
    /// Keyboard and screen-reader labels are exposed.
    pub exposes_keyboard_and_screen_reader_labels: bool,
    /// CLI and export enums are exposed.
    pub exposes_cli_and_export_enums: bool,
    /// Explanation fields are exposed.
    pub exposes_explanation_fields: bool,
    /// The claim auto-narrows when parser/schema certainty is uncertain.
    pub auto_narrows_on_uncertain_parser_schema: bool,
    /// The claim auto-narrows when render trust is unavailable.
    pub auto_narrows_on_unavailable_render_trust: bool,
    /// The claim auto-narrows when write-back safety is unavailable.
    pub auto_narrows_on_unavailable_write_back_safety: bool,
    /// The claim auto-narrows when metadata availability is unavailable.
    pub auto_narrows_on_unavailable_metadata: bool,
    /// Desktop, CLI, and export semantics are identical.
    pub desktop_cli_export_semantics_identical: bool,
    /// Narrowing prevents overstated structured fidelity.
    pub narrowing_prevents_overstated_structured_fidelity: bool,
    /// Every component is reachable non-visually.
    pub every_component_reachable_non_visually: bool,
}

impl ArtifactReviewAccessibilityProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.exposes_keyboard_and_screen_reader_labels
            && self.exposes_cli_and_export_enums
            && self.exposes_explanation_fields
            && self.auto_narrows_on_uncertain_parser_schema
            && self.auto_narrows_on_unavailable_render_trust
            && self.auto_narrows_on_unavailable_write_back_safety
            && self.auto_narrows_on_unavailable_metadata
            && self.desktop_cli_export_semantics_identical
            && self.narrowing_prevents_overstated_structured_fidelity
            && self.every_component_reachable_non_visually
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewAccessibilityProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ArtifactReviewAccessibilityPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactReviewAccessibilityPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Accessibility rows.
    pub accessibility_rows: Vec<ArtifactReviewAccessibilityRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ArtifactReviewAccessibilityDowngradeTrigger>,
    /// Rendering surfaces this packet covers.
    pub rendering_surfaces: Vec<ArtifactReviewRenderingSurface>,
    /// Trust review block.
    pub trust_review: ArtifactReviewAccessibilityTrustReview,
    /// Consumer projection block.
    pub projection: ArtifactReviewAccessibilityProjection,
    /// Proof freshness block.
    pub proof_freshness: ArtifactReviewAccessibilityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe structured-artifact review accessibility parity packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewAccessibilityPacket {
    /// Record kind; must equal [`M5_ARTIFACT_REVIEW_ACCESSIBILITY_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_ARTIFACT_REVIEW_ACCESSIBILITY_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Accessibility rows.
    pub accessibility_rows: Vec<ArtifactReviewAccessibilityRow>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ArtifactReviewAccessibilityDowngradeTrigger>,
    /// Rendering surfaces this packet covers.
    pub rendering_surfaces: Vec<ArtifactReviewRenderingSurface>,
    /// Trust review block.
    pub trust_review: ArtifactReviewAccessibilityTrustReview,
    /// Consumer projection block.
    pub projection: ArtifactReviewAccessibilityProjection,
    /// Proof freshness block.
    pub proof_freshness: ArtifactReviewAccessibilityProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ArtifactReviewAccessibilityPacket {
    /// Builds a structured-artifact review accessibility packet from stable-lane input.
    pub fn new(input: ArtifactReviewAccessibilityPacketInput) -> Self {
        Self {
            record_kind: M5_ARTIFACT_REVIEW_ACCESSIBILITY_RECORD_KIND.to_owned(),
            schema_version: M5_ARTIFACT_REVIEW_ACCESSIBILITY_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            accessibility_rows: input.accessibility_rows,
            downgrade_triggers: input.downgrade_triggers,
            rendering_surfaces: input.rendering_surfaces,
            trust_review: input.trust_review,
            projection: input.projection,
            proof_freshness: input.proof_freshness,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Validates the structured-artifact review accessibility parity invariants.
    pub fn validate(&self) -> Vec<ArtifactReviewAccessibilityViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_ARTIFACT_REVIEW_ACCESSIBILITY_RECORD_KIND {
            violations.push(ArtifactReviewAccessibilityViolation::WrongRecordKind);
        }
        if self.schema_version != M5_ARTIFACT_REVIEW_ACCESSIBILITY_SCHEMA_VERSION {
            violations.push(ArtifactReviewAccessibilityViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ArtifactReviewAccessibilityViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ArtifactReviewAccessibilityViolation::DowngradeTriggersMissing);
        }
        if self.rendering_surfaces.is_empty() {
            violations.push(ArtifactReviewAccessibilityViolation::RenderingSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_rows(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(ArtifactReviewAccessibilityViolation::TrustReviewIncomplete);
        }
        if !self.projection.all_hold() {
            violations.push(ArtifactReviewAccessibilityViolation::ProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ArtifactReviewAccessibilityViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self)
                .expect("structured-artifact review accessibility packet serializes"),
        ) {
            violations.push(ArtifactReviewAccessibilityViolation::RawBoundaryMaterialInExport);
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
            .expect("structured-artifact review accessibility packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .accessibility_rows
            .iter()
            .filter(|row| row.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str("# Structured-Artifact Review Accessibility, Headless, and Export Parity\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Accessibility rows: {} ({} claim-narrowed)\n",
            self.accessibility_rows.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Accessibility rows\n\n");
        for row in &self.accessibility_rows {
            out.push_str(&format!(
                "- **{}** [`{}`]: condition `{}`, claim `{}`\n",
                row.component.as_str(),
                row.row_id,
                row.condition.as_str(),
                row.effective_claim.as_str(),
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in structured-artifact review accessibility export.
#[derive(Debug)]
pub enum ArtifactReviewAccessibilityArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ArtifactReviewAccessibilityViolation>),
}

impl fmt::Display for ArtifactReviewAccessibilityArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "structured-artifact review accessibility export parse failed: {error}"
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
                    "structured-artifact review accessibility export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ArtifactReviewAccessibilityArtifactError {}

/// Validation failures emitted by [`ArtifactReviewAccessibilityPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactReviewAccessibilityViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No accessibility rows are present.
    AccessibilityRowsMissing,
    /// An accessibility row is incomplete.
    RowIncomplete,
    /// A row is missing its keyboard label.
    KeyboardLabelMissing,
    /// A row is missing its screen-reader label.
    ScreenReaderLabelMissing,
    /// A row is missing its CLI enum token.
    CliEnumTokenMissing,
    /// A row is missing its export enum token.
    ExportEnumTokenMissing,
    /// A row is missing its explanation field.
    ExplanationFieldMissing,
    /// A row does not reach all three rendering surfaces.
    RenderingSurfaceCoverageMissing,
    /// A component is reachable only by pointer.
    PointerOnlyComponent,
    /// A component omits itself from the export.
    ExportOpaqueComponent,
    /// A component claims more on the desktop than in CLI or export.
    DesktopStrongerThanCli,
    /// A row's effective claim exceeds the ceiling its condition permits.
    ClaimCeilingExceeded,
    /// A weakening condition is missing its explicit narrow disclosure.
    ClaimNarrowingMissing,
    /// A baseline condition unexpectedly carries a narrow disclosure.
    ClaimNarrowingUnexpected,
    /// A narrow disclosure pins the claim to the wrong tier.
    NarrowedToMismatch,
    /// A narrow disclosure names the wrong trigger.
    NarrowTriggerMismatch,
    /// A narrow disclosure offers the wrong next action.
    NarrowNextActionMismatch,
    /// A narrow disclosure is missing its preserved-truth note.
    NarrowPreservedTruthMissing,
    /// A narrow disclosure is missing its next-action copy.
    NarrowNextActionMissing,
    /// A row that must keep the raw/export-safe fallback explicit is missing its note.
    RawFallbackNoteMissing,
    /// A row that must keep the compare-only posture explicit is missing its note.
    CompareOnlyNoteMissing,
    /// A row that must keep redacted/withheld metadata explicit is missing its note.
    RedactionNoteMissing,
    /// A row does not point at the canonical component and matrix contracts.
    CanonicalContractReferenceMissing,
    /// Not every shared component appears among the rows.
    ComponentCoverageMissing,
    /// Not every claim condition appears among the rows.
    ConditionCoverageMissing,
    /// Not every claim tier appears as an effective claim.
    ClaimTierCoverageMissing,
    /// No downgrade triggers are present.
    DowngradeTriggersMissing,
    /// No rendering surfaces are present.
    RenderingSurfacesMissing,
    /// Trust review does not satisfy required invariants.
    TrustReviewIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ProjectionIncomplete,
    /// Proof freshness block is incomplete.
    ProofFreshnessIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl ArtifactReviewAccessibilityViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::AccessibilityRowsMissing => "accessibility_rows_missing",
            Self::RowIncomplete => "row_incomplete",
            Self::KeyboardLabelMissing => "keyboard_label_missing",
            Self::ScreenReaderLabelMissing => "screen_reader_label_missing",
            Self::CliEnumTokenMissing => "cli_enum_token_missing",
            Self::ExportEnumTokenMissing => "export_enum_token_missing",
            Self::ExplanationFieldMissing => "explanation_field_missing",
            Self::RenderingSurfaceCoverageMissing => "rendering_surface_coverage_missing",
            Self::PointerOnlyComponent => "pointer_only_component",
            Self::ExportOpaqueComponent => "export_opaque_component",
            Self::DesktopStrongerThanCli => "desktop_stronger_than_cli",
            Self::ClaimCeilingExceeded => "claim_ceiling_exceeded",
            Self::ClaimNarrowingMissing => "claim_narrowing_missing",
            Self::ClaimNarrowingUnexpected => "claim_narrowing_unexpected",
            Self::NarrowedToMismatch => "narrowed_to_mismatch",
            Self::NarrowTriggerMismatch => "narrow_trigger_mismatch",
            Self::NarrowNextActionMismatch => "narrow_next_action_mismatch",
            Self::NarrowPreservedTruthMissing => "narrow_preserved_truth_missing",
            Self::NarrowNextActionMissing => "narrow_next_action_missing",
            Self::RawFallbackNoteMissing => "raw_fallback_note_missing",
            Self::CompareOnlyNoteMissing => "compare_only_note_missing",
            Self::RedactionNoteMissing => "redaction_note_missing",
            Self::CanonicalContractReferenceMissing => "canonical_contract_reference_missing",
            Self::ComponentCoverageMissing => "component_coverage_missing",
            Self::ConditionCoverageMissing => "condition_coverage_missing",
            Self::ClaimTierCoverageMissing => "claim_tier_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::RenderingSurfacesMissing => "rendering_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ProjectionIncomplete => "projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable structured-artifact review accessibility export.
pub fn current_artifact_review_accessibility_export(
) -> Result<ArtifactReviewAccessibilityPacket, ArtifactReviewAccessibilityArtifactError> {
    let packet: ArtifactReviewAccessibilityPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-structured-artifact-review-accessibility-proof/support_export.json"
    )))
    .map_err(ArtifactReviewAccessibilityArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ArtifactReviewAccessibilityArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &ArtifactReviewAccessibilityPacket,
    violations: &mut Vec<ArtifactReviewAccessibilityViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_SCHEMA_REF,
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_DOC_REF,
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_COMPONENT_MATRIX_CONTRACT_REF,
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_CONSUMER_CONTRACT_REF,
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_IDENTITY_DIFF_CONTROLS_CONTRACT_REF,
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_STRUCTURE_COMPARE_CONTROLS_CONTRACT_REF,
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_MERGE_GENERATED_CONTROLS_CONTRACT_REF,
        M5_ARTIFACT_REVIEW_ACCESSIBILITY_MEDIA_TRUST_CONTROLS_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ArtifactReviewAccessibilityViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_rows(
    packet: &ArtifactReviewAccessibilityPacket,
    violations: &mut Vec<ArtifactReviewAccessibilityViolation>,
) {
    if packet.accessibility_rows.is_empty() {
        violations.push(ArtifactReviewAccessibilityViolation::AccessibilityRowsMissing);
        return;
    }

    let mut seen_components: BTreeSet<M5ArtifactComponent> = BTreeSet::new();
    let mut seen_conditions: BTreeSet<ArtifactReviewClaimCondition> = BTreeSet::new();
    let mut seen_tiers: BTreeSet<ArtifactReviewClaimTier> = BTreeSet::new();

    for row in &packet.accessibility_rows {
        if row.row_id.trim().is_empty() || row.source_contract_refs.is_empty() {
            violations.push(ArtifactReviewAccessibilityViolation::RowIncomplete);
        }

        if row.keyboard_label.trim().is_empty() {
            violations.push(ArtifactReviewAccessibilityViolation::KeyboardLabelMissing);
        }
        if row.screen_reader_label.trim().is_empty() {
            violations.push(ArtifactReviewAccessibilityViolation::ScreenReaderLabelMissing);
        }
        if row.cli_enum_token.trim().is_empty() {
            violations.push(ArtifactReviewAccessibilityViolation::CliEnumTokenMissing);
        }
        if row.export_enum_token.trim().is_empty() {
            violations.push(ArtifactReviewAccessibilityViolation::ExportEnumTokenMissing);
        }
        if row.explanation_field.trim().is_empty() {
            violations.push(ArtifactReviewAccessibilityViolation::ExplanationFieldMissing);
        }

        if !row.covers_all_rendering_surfaces() {
            violations.push(ArtifactReviewAccessibilityViolation::RenderingSurfaceCoverageMissing);
        }

        // AC1 guardrails: parity across desktop, CLI, and export.
        if row.is_pointer_only {
            violations.push(ArtifactReviewAccessibilityViolation::PointerOnlyComponent);
        }
        if row.is_export_opaque {
            violations.push(ArtifactReviewAccessibilityViolation::ExportOpaqueComponent);
        }
        if row.desktop_stronger_than_cli {
            violations.push(ArtifactReviewAccessibilityViolation::DesktopStrongerThanCli);
        }

        let resolution = row.resolution();
        let ceiling = resolution.permitted_ceiling;

        // AC2 core: a claim may never exceed the ceiling its condition permits.
        if row.effective_claim.rank() > ceiling.rank() {
            violations.push(ArtifactReviewAccessibilityViolation::ClaimCeilingExceeded);
        }

        // Narrow-disclosure presence and completeness.
        if resolution.requires_narrowing {
            match &row.narrowing {
                None => {
                    violations.push(ArtifactReviewAccessibilityViolation::ClaimNarrowingMissing);
                }
                Some(narrowing) => {
                    if narrowing.narrowed_to != ceiling {
                        violations.push(ArtifactReviewAccessibilityViolation::NarrowedToMismatch);
                    }
                    if Some(narrowing.trigger) != resolution.expected_trigger {
                        violations
                            .push(ArtifactReviewAccessibilityViolation::NarrowTriggerMismatch);
                    }
                    if narrowing.next_action != resolution.expected_next_action {
                        violations
                            .push(ArtifactReviewAccessibilityViolation::NarrowNextActionMismatch);
                    }
                    if narrowing.preserved_truth_note.trim().is_empty() {
                        violations.push(
                            ArtifactReviewAccessibilityViolation::NarrowPreservedTruthMissing,
                        );
                    }
                    if narrowing.next_action_label.trim().is_empty() {
                        violations
                            .push(ArtifactReviewAccessibilityViolation::NarrowNextActionMissing);
                    }
                }
            }
        } else if row.narrowing.is_some() {
            violations.push(ArtifactReviewAccessibilityViolation::ClaimNarrowingUnexpected);
        }

        if resolution.needs_raw_fallback_note && row.raw_fallback_note.trim().is_empty() {
            violations.push(ArtifactReviewAccessibilityViolation::RawFallbackNoteMissing);
        }
        if resolution.needs_compare_only_note && row.compare_only_note.trim().is_empty() {
            violations.push(ArtifactReviewAccessibilityViolation::CompareOnlyNoteMissing);
        }
        if resolution.needs_redaction_note && row.redaction_note.trim().is_empty() {
            violations.push(ArtifactReviewAccessibilityViolation::RedactionNoteMissing);
        }

        if !row.points_at_canonical_contracts() {
            violations
                .push(ArtifactReviewAccessibilityViolation::CanonicalContractReferenceMissing);
        }

        seen_components.insert(row.component);
        seen_conditions.insert(row.condition);
        seen_tiers.insert(row.effective_claim);
    }

    // Coverage: every component, every condition, and every claim tier must appear.
    for component in M5ArtifactComponent::ALL {
        if !seen_components.contains(&component) {
            violations.push(ArtifactReviewAccessibilityViolation::ComponentCoverageMissing);
            break;
        }
    }
    for condition in ArtifactReviewClaimCondition::ALL {
        if !seen_conditions.contains(&condition) {
            violations.push(ArtifactReviewAccessibilityViolation::ConditionCoverageMissing);
            break;
        }
    }
    for tier in ArtifactReviewClaimTier::ALL {
        if !seen_tiers.contains(&tier) {
            violations.push(ArtifactReviewAccessibilityViolation::ClaimTierCoverageMissing);
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
