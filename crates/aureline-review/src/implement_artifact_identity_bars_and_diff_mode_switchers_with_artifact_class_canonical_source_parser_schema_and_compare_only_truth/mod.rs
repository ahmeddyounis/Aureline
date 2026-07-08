//! Artifact identity bars and diff-mode switchers with artifact class,
//! canonical source, parser/schema state, and compare-only truth.
//!
//! This module narrows the `artifact_identity_bar` and `diff_mode_switcher`
//! components frozen in
//! [`crate::freeze_the_m5_structured_artifact_review_component_matrix`] into
//! implemented, export-safe review controls. Every [`ArtifactIdentityBar`]
//! answers, from the component alone, what artifact class the reader is looking
//! at, which canonical source owns it, what parser/schema state currently backs
//! it, whether the artifact is authored, generated, imported, or policy-owned,
//! and whether it is a writable target or compare-only. Every
//! [`DiffModeSwitcher`] enumerates which review lenses exist, which is active,
//! why any unavailable lens cannot be used, and keeps a raw/export-safe fallback
//! lens explicitly reachable so a narrowed render or schema is never flattened
//! without explanation.
//!
//! The two controls are paired by artifact reference: every artifact that shows
//! an identity bar also shows a diff-mode switcher, so canonical-source and
//! compare-only truth are never buried in a distant panel away from the lens
//! picker. The fidelity-narrowing vocabulary
//! ([`M5ArtifactFidelityState`]) and rollback posture
//! ([`M5ArtifactComponentRollbackPosture`]) are reused directly from the frozen
//! matrix so parser/schema state and write-back safety read the same everywhere.
//!
//! The packet references upstream artifact-component-matrix, artifact-provenance,
//! and cell-aware-diff contracts by id rather than embedding their content. Raw
//! artifact bodies, raw render payloads, raw media bytes, credentials, and live
//! provider responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-artifact-identity-diff-mode-controls.schema.json`](../../../../schemas/ui/m5-artifact-identity-diff-mode-controls.schema.json).
//! The contract doc is
//! [`docs/review/m5/implement_artifact_identity_bars_and_diff_mode_switchers.md`](../../../../docs/review/m5/implement_artifact_identity_bars_and_diff_mode_switchers.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-artifact-identity-diff-mode-controls/`](../../../../fixtures/ui/m5-artifact-identity-diff-mode-controls/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_structured_artifact_review_component_matrix::{
    M5ArtifactComponent, M5ArtifactComponentRollbackPosture, M5ArtifactFidelityState,
    M5_ARTIFACT_COMPONENT_MATRIX_DIFF_MODE_CONTRACT_REF,
    M5_ARTIFACT_COMPONENT_MATRIX_IDENTITY_BAR_CONTRACT_REF,
    M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`ArtifactReviewControlsPacket`].
pub const ARTIFACT_REVIEW_CONTROLS_RECORD_KIND: &str = "artifact_identity_and_diff_mode_controls";

/// Schema version for artifact identity / diff-mode control records.
pub const ARTIFACT_REVIEW_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const ARTIFACT_REVIEW_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-artifact-identity-diff-mode-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const ARTIFACT_REVIEW_CONTROLS_DOC_REF: &str =
    "docs/review/m5/implement_artifact_identity_bars_and_diff_mode_switchers.md";

/// Repo-relative path of the protected fixture directory.
pub const ARTIFACT_REVIEW_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-artifact-identity-diff-mode-controls";

/// Repo-relative path of the checked support-export artifact.
pub const ARTIFACT_REVIEW_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-artifact-identity-diff-mode-controls-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const ARTIFACT_REVIEW_CONTROLS_SUMMARY_REF: &str =
    "artifacts/release/m5-artifact-identity-diff-mode-controls-proof/summary.md";

/// Artifact origin class shown on an identity bar: the generated / imported /
/// authored / policy-owned identity axis.
///
/// This is a core honesty axis. Only an artifact authored in the repo is a
/// natural write-back target; a generated artifact must name its generated-from
/// relation and regenerate rather than accept manual structured edits, and an
/// imported or policy-owned artifact keeps a pointer to the canonical source of
/// truth that lives elsewhere.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactOriginClass {
    /// Authored in this repo; a natural write-back target.
    AuthoredInRepo,
    /// Generated from an upstream source; regenerate rather than hand-edit.
    GeneratedFromSource,
    /// Imported from an external system; canonical truth lives upstream.
    ImportedExternal,
    /// Owned by policy or a governance system; not freely writable here.
    PolicyOwned,
}

impl ArtifactOriginClass {
    /// Every origin class, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::AuthoredInRepo,
        Self::GeneratedFromSource,
        Self::ImportedExternal,
        Self::PolicyOwned,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AuthoredInRepo => "authored_in_repo",
            Self::GeneratedFromSource => "generated_from_source",
            Self::ImportedExternal => "imported_external",
            Self::PolicyOwned => "policy_owned",
        }
    }

    /// Whether this origin is a natural write-back target for the artifact itself.
    pub const fn is_authored(self) -> bool {
        matches!(self, Self::AuthoredInRepo)
    }

    /// Whether this origin is generated and must name its generated-from relation.
    pub const fn is_generated(self) -> bool {
        matches!(self, Self::GeneratedFromSource)
    }

    /// Whether this origin must point at a canonical source of truth living elsewhere.
    pub const fn needs_source_of_truth_pointer(self) -> bool {
        !matches!(self, Self::AuthoredInRepo)
    }
}

/// A review lens a diff-mode switcher may expose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffReviewLens {
    /// Structured / semantic diff against a known schema.
    StructuredSemantic,
    /// Rendered preview of the artifact.
    RenderedPreview,
    /// Raw / export-safe text fallback that is always reachable.
    RawTextFallback,
    /// Line-oriented side-by-side view.
    SideBySide,
    /// Three-way (base/ours/theirs) merge lens.
    ThreeWayMerge,
    /// Visual comparison for media-like artifacts.
    MediaVisual,
}

impl DiffReviewLens {
    /// Every lens, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::StructuredSemantic,
        Self::RenderedPreview,
        Self::RawTextFallback,
        Self::SideBySide,
        Self::ThreeWayMerge,
        Self::MediaVisual,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StructuredSemantic => "structured_semantic",
            Self::RenderedPreview => "rendered_preview",
            Self::RawTextFallback => "raw_text_fallback",
            Self::SideBySide => "side_by_side",
            Self::ThreeWayMerge => "three_way_merge",
            Self::MediaVisual => "media_visual",
        }
    }

    /// Whether this lens is the raw / export-safe fallback that must stay reachable.
    pub const fn is_raw_fallback(self) -> bool {
        matches!(self, Self::RawTextFallback)
    }
}

/// Availability state of a single diff-mode lens.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiffLensAvailability {
    /// The lens is available and selectable.
    Available,
    /// No schema recognizes the artifact, so the structured lens is unavailable.
    UnavailableSchemaUnrecognized,
    /// The parser for this artifact class is unavailable on this build.
    UnavailableParserMissing,
    /// A rendered lens exists but its render is not trusted, so it is withheld.
    UnavailableRenderUntrusted,
    /// The lens is unavailable because the content is redacted or withheld.
    UnavailableRedacted,
    /// Policy blocks this lens for this artifact.
    UnavailablePolicyBlocked,
}

impl DiffLensAvailability {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Available => "available",
            Self::UnavailableSchemaUnrecognized => "unavailable_schema_unrecognized",
            Self::UnavailableParserMissing => "unavailable_parser_missing",
            Self::UnavailableRenderUntrusted => "unavailable_render_untrusted",
            Self::UnavailableRedacted => "unavailable_redacted",
            Self::UnavailablePolicyBlocked => "unavailable_policy_blocked",
        }
    }

    /// Whether the lens is available and selectable.
    pub const fn is_available(self) -> bool {
        matches!(self, Self::Available)
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewControlsDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// The artifact parser is unavailable on this build.
    ParserUnavailable,
    /// No schema recognizes the artifact class.
    SchemaUnrecognized,
    /// A rendered lens is not trusted.
    RenderUntrusted,
    /// Compare-only safety is enforced; write-back is unavailable.
    CompareOnlyEnforced,
    /// The generated-from source drifted or the generated artifact is stale.
    GeneratedArtifactStale,
    /// Content was redacted and narrows visible lenses.
    RedactionApplied,
    /// Control trust narrowed.
    TrustNarrowing,
    /// An upstream dependency component narrowed.
    UpstreamDependencyNarrowed,
}

impl ArtifactReviewControlsDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 10] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::ParserUnavailable,
        Self::SchemaUnrecognized,
        Self::RenderUntrusted,
        Self::CompareOnlyEnforced,
        Self::GeneratedArtifactStale,
        Self::RedactionApplied,
        Self::TrustNarrowing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::ParserUnavailable => "parser_unavailable",
            Self::SchemaUnrecognized => "schema_unrecognized",
            Self::RenderUntrusted => "render_untrusted",
            Self::CompareOnlyEnforced => "compare_only_enforced",
            Self::GeneratedArtifactStale => "generated_artifact_stale",
            Self::RedactionApplied => "redaction_applied",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must reuse these controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactReviewControlsConsumerSurface {
    /// Diff / compare view.
    DiffCompareView,
    /// Merge / conflict resolution workspace.
    MergeConflictWorkspace,
    /// Notebook review surface.
    NotebookReview,
    /// Artifact browser (coverage, profile, crash, SBOM, lockfile adjuncts).
    ArtifactBrowser,
    /// CLI / headless replay or JSON output.
    CliHeadless,
    /// Support / export packet.
    SupportExport,
    /// Help / About surface.
    HelpAbout,
}

impl ArtifactReviewControlsConsumerSurface {
    /// Every surface, in declaration order.
    pub const ALL: [Self; 7] = [
        Self::DiffCompareView,
        Self::MergeConflictWorkspace,
        Self::NotebookReview,
        Self::ArtifactBrowser,
        Self::CliHeadless,
        Self::SupportExport,
        Self::HelpAbout,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiffCompareView => "diff_compare_view",
            Self::MergeConflictWorkspace => "merge_conflict_workspace",
            Self::NotebookReview => "notebook_review",
            Self::ArtifactBrowser => "artifact_browser",
            Self::CliHeadless => "cli_headless",
            Self::SupportExport => "support_export",
            Self::HelpAbout => "help_about",
        }
    }
}

/// Disclosures an identity bar must carry, derived from its origin and parser state.
///
/// This is the resolver output that anchors the honesty invariants: a generated,
/// imported, or policy-owned artifact never claims to be a plain writable target,
/// a non-authored artifact always points at its canonical source of truth, and a
/// narrowed parser/schema state always keeps a raw-fallback note explicit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ArtifactIdentityDisclosure {
    /// Whether the bar may assert the artifact is a writable target.
    pub asserts_writable_target: bool,
    /// Whether the bar must name its generated-from relation.
    pub needs_generated_from_relation: bool,
    /// Whether the bar must point at a canonical source of truth living elsewhere.
    pub needs_source_of_truth_pointer: bool,
    /// Whether the bar must keep an explicit raw / export-safe fallback note.
    pub needs_raw_fallback_note: bool,
}

/// Resolves the disclosures an identity bar must carry from its origin and parser state.
///
/// Writable-target status is derived, never asserted directly: only an artifact
/// authored in the repo whose parser/schema state is structured-faithful or
/// structured-partial is a writable target, so a generated, imported, policy-owned,
/// schema-unrecognized, untrusted, or redacted artifact can never masquerade as a
/// plain editable file. A narrowed parser/schema state always forces an explicit
/// raw-fallback note.
pub fn resolve_artifact_identity_disclosure(
    origin: ArtifactOriginClass,
    parser_schema_state: M5ArtifactFidelityState,
) -> ArtifactIdentityDisclosure {
    let parser_supports_structured = matches!(
        parser_schema_state,
        M5ArtifactFidelityState::StructuredFaithful | M5ArtifactFidelityState::StructuredPartial
    );
    ArtifactIdentityDisclosure {
        asserts_writable_target: origin.is_authored() && parser_supports_structured,
        needs_generated_from_relation: origin.is_generated(),
        needs_source_of_truth_pointer: origin.needs_source_of_truth_pointer(),
        needs_raw_fallback_note: !matches!(
            parser_schema_state,
            M5ArtifactFidelityState::StructuredFaithful
        ),
    }
}

/// One diff-mode option exposed by a switcher.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffModeOption {
    /// The review lens this option offers.
    pub lens: DiffReviewLens,
    /// Whether the lens is available, and why not when it is unavailable.
    pub availability: DiffLensAvailability,
    /// Reason shown when the lens is unavailable; required and non-empty then.
    pub unavailability_reason: String,
}

/// An artifact identity bar naming class, canonical source, parser state, and origin.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactIdentityBar {
    /// Frozen component this bar implements; must be `artifact_identity_bar`.
    pub component: M5ArtifactComponent,
    /// Stable bar id.
    pub bar_id: String,
    /// Stable artifact reference shared with the paired diff-mode switcher.
    pub artifact_ref: String,
    /// Human-readable artifact-class label (notebook, lockfile, SBOM, media, ...).
    pub artifact_class_label: String,
    /// Origin class: authored / generated / imported / policy-owned.
    pub origin_class: ArtifactOriginClass,
    /// Canonical-source-of-truth disclosure; required and non-empty.
    pub canonical_source_disclosure: String,
    /// Parser/schema state, reused from the frozen component matrix.
    pub parser_schema_state: M5ArtifactFidelityState,
    /// Whether the bar claims the artifact is a writable target; must match the origin/parser.
    pub claims_writable_target: bool,
    /// Generated-from relation; required and non-empty when the artifact is generated.
    pub generated_from_relation: String,
    /// Pointer to a canonical source of truth living elsewhere; required for non-authored artifacts.
    pub source_of_truth_pointer: String,
    /// Raw / export-safe fallback note; required when the parser/schema state narrows.
    pub raw_fallback_note: String,
    /// Rollback / write-back posture, reused from the frozen component matrix.
    pub rollback_posture: M5ArtifactComponentRollbackPosture,
    /// Bar fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this bar.
    pub source_contract_refs: Vec<String>,
}

impl ArtifactIdentityBar {
    /// Disclosures this bar must carry, derived from origin and parser state.
    pub fn disclosure(&self) -> ArtifactIdentityDisclosure {
        resolve_artifact_identity_disclosure(self.origin_class, self.parser_schema_state)
    }

    /// Whether the rollback posture is consistent with the writable-target claim.
    ///
    /// A writable target must keep write-back individually attributable; a
    /// compare-only artifact must never carry a write-back-attributable posture.
    pub fn rollback_posture_consistent(&self) -> bool {
        let writable = self.disclosure().asserts_writable_target;
        let write_back = matches!(
            self.rollback_posture,
            M5ArtifactComponentRollbackPosture::WriteBackAttributable
        );
        writable == write_back
    }
}

/// A diff-mode switcher enumerating review lenses, the active one, and unavailability reasons.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DiffModeSwitcher {
    /// Frozen component this switcher implements; must be `diff_mode_switcher`.
    pub component: M5ArtifactComponent,
    /// Stable switcher id.
    pub switcher_id: String,
    /// Stable artifact reference shared with the paired identity bar.
    pub artifact_ref: String,
    /// Human-readable artifact-class label.
    pub artifact_class_label: String,
    /// Diff-mode options, in display order.
    pub options: Vec<DiffModeOption>,
    /// The active lens; must be present and available among the options.
    pub active_lens: DiffReviewLens,
    /// Compare-only-versus-write-back safety disclosure; required and non-empty.
    pub compare_write_back_safety: String,
    /// Rollback / write-back posture, reused from the frozen component matrix.
    pub rollback_posture: M5ArtifactComponentRollbackPosture,
    /// Switcher fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this switcher.
    pub source_contract_refs: Vec<String>,
}

impl DiffModeSwitcher {
    /// Whether a raw / export-safe fallback lens is available.
    pub fn has_available_raw_fallback(&self) -> bool {
        self.options
            .iter()
            .any(|option| option.lens.is_raw_fallback() && option.availability.is_available())
    }

    /// Whether the active lens is present among the options and available.
    pub fn active_lens_available(&self) -> bool {
        self.options
            .iter()
            .any(|option| option.lens == self.active_lens && option.availability.is_available())
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewControlsTrustReview {
    /// Artifact class is always explicit on the identity bar.
    pub artifact_class_always_explicit: bool,
    /// Canonical source is never buried away from the compare surface.
    pub canonical_source_never_buried: bool,
    /// Generated-from / source-of-truth relations are never hidden.
    pub generated_from_relation_never_hidden: bool,
    /// Compare-only artifacts are never silently promoted to writable state.
    pub compare_only_never_silently_writable: bool,
    /// Parser / schema state stays explicit.
    pub parser_schema_state_explicit: bool,
    /// Available review lenses are enumerated rather than implied.
    pub review_lenses_enumerated: bool,
    /// Unavailable lenses always carry a reason.
    pub lens_unavailability_reason_explicit: bool,
    /// A raw / export-safe fallback lens is always reachable.
    pub raw_fallback_always_available: bool,
    /// Writable-target claims match the origin and parser state.
    pub writable_target_matches_origin: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified controls automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl ArtifactReviewControlsTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.artifact_class_always_explicit
            && self.canonical_source_never_buried
            && self.generated_from_relation_never_hidden
            && self.compare_only_never_silently_writable
            && self.parser_schema_state_explicit
            && self.review_lenses_enumerated
            && self.lens_unavailability_reason_explicit
            && self.raw_fallback_always_available
            && self.writable_target_matches_origin
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewControlsConsumerProjection {
    /// Identity bar shows artifact class and canonical source.
    pub identity_bar_shows_class_and_canonical_source: bool,
    /// Diff switcher shows available and unavailable lenses.
    pub diff_switcher_shows_available_and_unavailable_lenses: bool,
    /// Compare-only truth is shown inline with the controls.
    pub compare_only_truth_shown_inline: bool,
    /// Generated-from relation is shown on the identity bar.
    pub generated_from_relation_shown: bool,
    /// The raw / export-safe fallback lens is reachable.
    pub raw_fallback_reachable: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_truth: bool,
}

impl ArtifactReviewControlsConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.identity_bar_shows_class_and_canonical_source
            && self.diff_switcher_shows_available_and_unavailable_lenses
            && self.compare_only_truth_shown_inline
            && self.generated_from_relation_shown
            && self.raw_fallback_reachable
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.help_about_shows_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewControlsProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`ArtifactReviewControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArtifactReviewControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Artifact identity bars.
    pub identity_bars: Vec<ArtifactIdentityBar>,
    /// Diff-mode switchers.
    pub diff_switchers: Vec<DiffModeSwitcher>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ArtifactReviewControlsDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<ArtifactReviewControlsConsumerSurface>,
    /// Trust review block.
    pub trust_review: ArtifactReviewControlsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ArtifactReviewControlsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ArtifactReviewControlsProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe artifact identity / diff-mode controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ArtifactReviewControlsPacket {
    /// Record kind; must equal [`ARTIFACT_REVIEW_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`ARTIFACT_REVIEW_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Artifact identity bars.
    pub identity_bars: Vec<ArtifactIdentityBar>,
    /// Diff-mode switchers.
    pub diff_switchers: Vec<DiffModeSwitcher>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ArtifactReviewControlsDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<ArtifactReviewControlsConsumerSurface>,
    /// Trust review block.
    pub trust_review: ArtifactReviewControlsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: ArtifactReviewControlsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: ArtifactReviewControlsProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl ArtifactReviewControlsPacket {
    /// Builds an artifact identity / diff-mode controls packet from stable-lane input.
    pub fn new(input: ArtifactReviewControlsPacketInput) -> Self {
        Self {
            record_kind: ARTIFACT_REVIEW_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: ARTIFACT_REVIEW_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            identity_bars: input.identity_bars,
            diff_switchers: input.diff_switchers,
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

    /// Validates the artifact identity / diff-mode controls invariants.
    pub fn validate(&self) -> Vec<ArtifactReviewControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != ARTIFACT_REVIEW_CONTROLS_RECORD_KIND {
            violations.push(ArtifactReviewControlsViolation::WrongRecordKind);
        }
        if self.schema_version != ARTIFACT_REVIEW_CONTROLS_SCHEMA_VERSION {
            violations.push(ArtifactReviewControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(ArtifactReviewControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(ArtifactReviewControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(ArtifactReviewControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_identity_bars(self, &mut violations);
        validate_diff_switchers(self, &mut violations);
        validate_pairing(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(ArtifactReviewControlsViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(ArtifactReviewControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(ArtifactReviewControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("artifact review controls packet serializes"),
        ) {
            violations.push(ArtifactReviewControlsViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("artifact review controls packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let generated = self
            .identity_bars
            .iter()
            .filter(|bar| bar.origin_class.is_generated())
            .count();
        let writable = self
            .identity_bars
            .iter()
            .filter(|bar| bar.claims_writable_target)
            .count();

        let mut out = String::new();
        out.push_str("# Artifact Identity Bars & Diff-Mode Switchers\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Identity bars: {} ({} generated, {} writable targets)\n",
            self.identity_bars.len(),
            generated,
            writable
        ));
        out.push_str(&format!(
            "- Diff-mode switchers: {}\n",
            self.diff_switchers.len()
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Identity bars\n\n");
        for bar in &self.identity_bars {
            out.push_str(&format!(
                "- **{}** [`{}`]: origin `{}`, parser `{}`, writable `{}` — {}\n",
                bar.artifact_class_label,
                bar.artifact_ref,
                bar.origin_class.as_str(),
                bar.parser_schema_state.as_str(),
                bar.claims_writable_target,
                bar.canonical_source_disclosure
            ));
        }

        out.push_str("\n## Diff-mode switchers\n\n");
        for switcher in &self.diff_switchers {
            let lenses = switcher
                .options
                .iter()
                .map(|option| format!("{}={}", option.lens.as_str(), option.availability.as_str()))
                .collect::<Vec<_>>()
                .join(", ");
            out.push_str(&format!(
                "- **{}** [`{}`]: active `{}` — {}\n",
                switcher.artifact_class_label,
                switcher.artifact_ref,
                switcher.active_lens.as_str(),
                lenses
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in artifact-controls export.
#[derive(Debug)]
pub enum ArtifactReviewControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<ArtifactReviewControlsViolation>),
}

impl fmt::Display for ArtifactReviewControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "artifact review controls export parse failed: {error}"
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
                    "artifact review controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for ArtifactReviewControlsArtifactError {}

/// Validation failures emitted by [`ArtifactReviewControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ArtifactReviewControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No identity bars are present.
    IdentityBarsMissing,
    /// An identity bar is incomplete.
    IdentityBarIncomplete,
    /// An identity bar carries the wrong frozen component class.
    IdentityBarWrongComponentClass,
    /// An identity bar does not name its canonical source of truth.
    CanonicalSourceDisclosureMissing,
    /// A generated artifact does not name its generated-from relation.
    GeneratedFromRelationMissing,
    /// A non-authored artifact does not point at its canonical source of truth.
    SourceOfTruthPointerMissing,
    /// A narrowed parser/schema state does not keep an explicit raw-fallback note.
    RawFallbackNoteMissing,
    /// An identity bar misrepresents writable-target status relative to origin/parser.
    WritableTargetMisrepresented,
    /// The rollback posture is inconsistent with the writable-target claim.
    RollbackPostureInconsistent,
    /// The identity bars do not cover authored, generated, and imported origins.
    ArtifactOriginCoverageMissing,
    /// No diff-mode switchers are present.
    DiffSwitchersMissing,
    /// A diff-mode switcher is incomplete.
    DiffSwitcherIncomplete,
    /// A diff-mode switcher carries the wrong frozen component class.
    DiffSwitcherWrongComponentClass,
    /// A diff-mode switcher exposes no options.
    DiffModeOptionsMissing,
    /// A diff-mode switcher has no available raw / export-safe fallback lens.
    RawFallbackLensMissing,
    /// A diff-mode switcher's active lens is not present and available.
    ActiveLensUnavailable,
    /// An unavailable lens does not carry an unavailability reason.
    LensUnavailabilityReasonMissing,
    /// A diff-mode switcher does not name its compare-only-versus-write-back safety.
    CompareWriteBackSafetyMissing,
    /// The diff-mode switchers do not cover the structured and raw-fallback lenses.
    DiffLensCoverageMissing,
    /// An identity bar and a diff-mode switcher are not paired by artifact reference.
    ArtifactPairingIncomplete,
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

impl ArtifactReviewControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::IdentityBarsMissing => "identity_bars_missing",
            Self::IdentityBarIncomplete => "identity_bar_incomplete",
            Self::IdentityBarWrongComponentClass => "identity_bar_wrong_component_class",
            Self::CanonicalSourceDisclosureMissing => "canonical_source_disclosure_missing",
            Self::GeneratedFromRelationMissing => "generated_from_relation_missing",
            Self::SourceOfTruthPointerMissing => "source_of_truth_pointer_missing",
            Self::RawFallbackNoteMissing => "raw_fallback_note_missing",
            Self::WritableTargetMisrepresented => "writable_target_misrepresented",
            Self::RollbackPostureInconsistent => "rollback_posture_inconsistent",
            Self::ArtifactOriginCoverageMissing => "artifact_origin_coverage_missing",
            Self::DiffSwitchersMissing => "diff_switchers_missing",
            Self::DiffSwitcherIncomplete => "diff_switcher_incomplete",
            Self::DiffSwitcherWrongComponentClass => "diff_switcher_wrong_component_class",
            Self::DiffModeOptionsMissing => "diff_mode_options_missing",
            Self::RawFallbackLensMissing => "raw_fallback_lens_missing",
            Self::ActiveLensUnavailable => "active_lens_unavailable",
            Self::LensUnavailabilityReasonMissing => "lens_unavailability_reason_missing",
            Self::CompareWriteBackSafetyMissing => "compare_write_back_safety_missing",
            Self::DiffLensCoverageMissing => "diff_lens_coverage_missing",
            Self::ArtifactPairingIncomplete => "artifact_pairing_incomplete",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable artifact-controls export.
pub fn current_artifact_review_controls_export(
) -> Result<ArtifactReviewControlsPacket, ArtifactReviewControlsArtifactError> {
    let packet: ArtifactReviewControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-artifact-identity-diff-mode-controls-proof/support_export.json"
    )))
    .map_err(ArtifactReviewControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(ArtifactReviewControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &ArtifactReviewControlsPacket,
    violations: &mut Vec<ArtifactReviewControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        ARTIFACT_REVIEW_CONTROLS_SCHEMA_REF,
        ARTIFACT_REVIEW_CONTROLS_DOC_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_IDENTITY_BAR_CONTRACT_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_DIFF_MODE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(ArtifactReviewControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_identity_bars(
    packet: &ArtifactReviewControlsPacket,
    violations: &mut Vec<ArtifactReviewControlsViolation>,
) {
    if packet.identity_bars.is_empty() {
        violations.push(ArtifactReviewControlsViolation::IdentityBarsMissing);
        return;
    }

    let mut origins: BTreeSet<ArtifactOriginClass> = BTreeSet::new();

    for bar in &packet.identity_bars {
        origins.insert(bar.origin_class);

        if bar.bar_id.trim().is_empty()
            || bar.artifact_ref.trim().is_empty()
            || bar.artifact_class_label.trim().is_empty()
            || bar.fields_shown.is_empty()
            || bar.source_contract_refs.is_empty()
        {
            violations.push(ArtifactReviewControlsViolation::IdentityBarIncomplete);
        }
        if bar.component != M5ArtifactComponent::ArtifactIdentityBar {
            violations.push(ArtifactReviewControlsViolation::IdentityBarWrongComponentClass);
        }
        if bar.canonical_source_disclosure.trim().is_empty() {
            violations.push(ArtifactReviewControlsViolation::CanonicalSourceDisclosureMissing);
        }

        let disclosure = bar.disclosure();

        if bar.claims_writable_target != disclosure.asserts_writable_target {
            violations.push(ArtifactReviewControlsViolation::WritableTargetMisrepresented);
        }
        if disclosure.needs_generated_from_relation && bar.generated_from_relation.trim().is_empty()
        {
            violations.push(ArtifactReviewControlsViolation::GeneratedFromRelationMissing);
        }
        if disclosure.needs_source_of_truth_pointer && bar.source_of_truth_pointer.trim().is_empty()
        {
            violations.push(ArtifactReviewControlsViolation::SourceOfTruthPointerMissing);
        }
        if disclosure.needs_raw_fallback_note && bar.raw_fallback_note.trim().is_empty() {
            violations.push(ArtifactReviewControlsViolation::RawFallbackNoteMissing);
        }
        if !bar.rollback_posture_consistent() {
            violations.push(ArtifactReviewControlsViolation::RollbackPostureInconsistent);
        }
    }

    for required in [
        ArtifactOriginClass::AuthoredInRepo,
        ArtifactOriginClass::GeneratedFromSource,
        ArtifactOriginClass::ImportedExternal,
    ] {
        if !origins.contains(&required) {
            violations.push(ArtifactReviewControlsViolation::ArtifactOriginCoverageMissing);
            break;
        }
    }
}

fn validate_diff_switchers(
    packet: &ArtifactReviewControlsPacket,
    violations: &mut Vec<ArtifactReviewControlsViolation>,
) {
    if packet.diff_switchers.is_empty() {
        violations.push(ArtifactReviewControlsViolation::DiffSwitchersMissing);
        return;
    }

    let mut lenses: BTreeSet<DiffReviewLens> = BTreeSet::new();

    for switcher in &packet.diff_switchers {
        if switcher.switcher_id.trim().is_empty()
            || switcher.artifact_ref.trim().is_empty()
            || switcher.artifact_class_label.trim().is_empty()
            || switcher.fields_shown.is_empty()
            || switcher.source_contract_refs.is_empty()
        {
            violations.push(ArtifactReviewControlsViolation::DiffSwitcherIncomplete);
        }
        if switcher.component != M5ArtifactComponent::DiffModeSwitcher {
            violations.push(ArtifactReviewControlsViolation::DiffSwitcherWrongComponentClass);
        }
        if switcher.compare_write_back_safety.trim().is_empty() {
            violations.push(ArtifactReviewControlsViolation::CompareWriteBackSafetyMissing);
        }
        if switcher.options.is_empty() {
            violations.push(ArtifactReviewControlsViolation::DiffModeOptionsMissing);
        }

        for option in &switcher.options {
            lenses.insert(option.lens);
            if !option.availability.is_available() && option.unavailability_reason.trim().is_empty()
            {
                violations.push(ArtifactReviewControlsViolation::LensUnavailabilityReasonMissing);
            }
        }

        if !switcher.has_available_raw_fallback() {
            violations.push(ArtifactReviewControlsViolation::RawFallbackLensMissing);
        }
        if !switcher.active_lens_available() {
            violations.push(ArtifactReviewControlsViolation::ActiveLensUnavailable);
        }
    }

    for required in [
        DiffReviewLens::StructuredSemantic,
        DiffReviewLens::RawTextFallback,
    ] {
        if !lenses.contains(&required) {
            violations.push(ArtifactReviewControlsViolation::DiffLensCoverageMissing);
            break;
        }
    }
}

fn validate_pairing(
    packet: &ArtifactReviewControlsPacket,
    violations: &mut Vec<ArtifactReviewControlsViolation>,
) {
    let bar_refs: BTreeSet<&str> = packet
        .identity_bars
        .iter()
        .map(|bar| bar.artifact_ref.as_str())
        .collect();
    let switcher_refs: BTreeSet<&str> = packet
        .diff_switchers
        .iter()
        .map(|switcher| switcher.artifact_ref.as_str())
        .collect();
    if bar_refs != switcher_refs {
        violations.push(ArtifactReviewControlsViolation::ArtifactPairingIncomplete);
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
