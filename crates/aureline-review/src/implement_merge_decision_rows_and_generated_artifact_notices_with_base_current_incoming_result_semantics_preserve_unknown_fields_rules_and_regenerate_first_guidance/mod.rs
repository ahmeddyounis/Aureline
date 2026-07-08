//! Merge decision rows and generated-artifact notices carrying
//! Base / Current / Incoming / Result semantics, conflict class, preserve-unknown-fields
//! notes, manual / regenerate / accept-current / accept-incoming / accept-both
//! guidance, generated-from relations, stale-or-diverged state, and
//! regenerate-first-versus-write-back safety.
//!
//! This module narrows the `merge_decision_row` and `generated_artifact_notice`
//! components frozen in
//! [`crate::freeze_the_m5_structured_artifact_review_component_matrix`] into
//! implemented, export-safe review controls. Every [`MergeDecisionRow`] answers,
//! from the component alone, which structured object is in conflict, what the
//! Base, Current, Incoming, and Result summaries are, what conflict kind and
//! conflict class the conflict belongs to, whether unknown fields are preserved,
//! and which resolution guidance is safe — so a generated, lockfile, manifest, or
//! policy-owned conflict never masquerades as an ordinary line merge, and the
//! flow states clearly when regenerate-first or manual resolution is safer than a
//! direct write-back. Every [`GeneratedArtifactNotice`] names its generated-from
//! relation and source-of-truth pointer, its stale / diverged state, its
//! last-generated version or time, its regenerate / open-source actions, and its
//! compare-only-versus-write-back restriction, so a generated artifact is never
//! hand-edited behind generic file chrome.
//!
//! The two controls are joined by artifact reference: every merge decision row
//! whose conflict is a generated-artifact conflict is accompanied by a
//! generated-artifact notice for the same artifact, so the regenerate-first path
//! is always visible where a generated artifact is being resolved.
//!
//! The fidelity-narrowing vocabulary ([`M5ArtifactFidelityState`]) and rollback
//! posture ([`M5ArtifactComponentRollbackPosture`]) are reused directly from the
//! frozen matrix so schema state and write-back safety read the same everywhere.
//! The packet references the upstream artifact-component-matrix, notebook-merge,
//! and generated-artifact-descriptor contracts by id rather than embedding their
//! content. Raw artifact bodies, raw diffs, credentials, and live provider
//! responses stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/ui/m5-merge-decision-generated-notice-controls.schema.json`](../../../../schemas/ui/m5-merge-decision-generated-notice-controls.schema.json).
//! The contract doc is
//! [`docs/review/m5/implement_merge_decision_rows_and_generated_artifact_notices.md`](../../../../docs/review/m5/implement_merge_decision_rows_and_generated_artifact_notices.md).
//! The protected fixture directory is
//! [`fixtures/ui/m5-merge-decision-generated-notice-controls/`](../../../../fixtures/ui/m5-merge-decision-generated-notice-controls/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::freeze_the_m5_structured_artifact_review_component_matrix::{
    M5ArtifactComponent, M5ArtifactComponentRollbackPosture, M5ArtifactFidelityState,
    M5_ARTIFACT_COMPONENT_MATRIX_GENERATED_NOTICE_CONTRACT_REF,
    M5_ARTIFACT_COMPONENT_MATRIX_MERGE_DECISION_CONTRACT_REF,
    M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`MergeGeneratedControlsPacket`].
pub const MERGE_GENERATED_CONTROLS_RECORD_KIND: &str =
    "merge_decision_rows_and_generated_artifact_notices";

/// Schema version for merge-decision / generated-notice control records.
pub const MERGE_GENERATED_CONTROLS_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const MERGE_GENERATED_CONTROLS_SCHEMA_REF: &str =
    "schemas/ui/m5-merge-decision-generated-notice-controls.schema.json";

/// Repo-relative path of the contract doc.
pub const MERGE_GENERATED_CONTROLS_DOC_REF: &str =
    "docs/review/m5/implement_merge_decision_rows_and_generated_artifact_notices.md";

/// Repo-relative path of the protected fixture directory.
pub const MERGE_GENERATED_CONTROLS_FIXTURE_DIR: &str =
    "fixtures/ui/m5-merge-decision-generated-notice-controls";

/// Repo-relative path of the checked support-export artifact.
pub const MERGE_GENERATED_CONTROLS_ARTIFACT_REF: &str =
    "artifacts/release/m5-merge-decision-generated-notice-controls-proof/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const MERGE_GENERATED_CONTROLS_SUMMARY_REF: &str =
    "artifacts/release/m5-merge-decision-generated-notice-controls-proof/summary.md";

/// The class a merge conflict belongs to.
///
/// This is the core honesty axis for the merge decision row: a generated,
/// lockfile, manifest, or policy-owned conflict is its own explicit class rather
/// than being folded into an ordinary line merge, so it can never be resolved as
/// if picking a side of a text diff were safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeConflictClass {
    /// An ordinary line-level text merge; picking a side or both is safe.
    OrdinaryLineMerge,
    /// A generated artifact conflict; regenerating from source is safer.
    GeneratedArtifactConflict,
    /// A lockfile conflict; regenerating the lockfile is safer than hand-merging.
    LockfileConflict,
    /// A manifest conflict; manual reconciliation is safer than picking a side.
    ManifestConflict,
    /// A policy-owned conflict; manual reconciliation under policy is required.
    PolicyOwnedConflict,
}

impl MergeConflictClass {
    /// Every conflict class, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::OrdinaryLineMerge,
        Self::GeneratedArtifactConflict,
        Self::LockfileConflict,
        Self::ManifestConflict,
        Self::PolicyOwnedConflict,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OrdinaryLineMerge => "ordinary_line_merge",
            Self::GeneratedArtifactConflict => "generated_artifact_conflict",
            Self::LockfileConflict => "lockfile_conflict",
            Self::ManifestConflict => "manifest_conflict",
            Self::PolicyOwnedConflict => "policy_owned_conflict",
        }
    }

    /// Whether this is an ordinary line merge that may be resolved by picking a side.
    pub const fn is_ordinary_line_merge(self) -> bool {
        matches!(self, Self::OrdinaryLineMerge)
    }

    /// Whether this class refers to a generated artifact whose notice must accompany it.
    pub const fn is_generated_artifact(self) -> bool {
        matches!(self, Self::GeneratedArtifactConflict)
    }
}

/// A resolution guidance option a merge decision row may offer.
///
/// Base / Current / Incoming / Result semantics stay explicit: "accept current"
/// keeps the current (ours) side, "accept incoming" keeps the incoming (theirs)
/// side, "accept both" keeps both, "manual" resolves by hand, and "regenerate
/// from source" rebuilds the artifact from its source of truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeResolutionGuidance {
    /// Resolve the conflict by hand.
    Manual,
    /// Regenerate the artifact from its source of truth.
    RegenerateFromSource,
    /// Accept the current (ours) side.
    AcceptCurrent,
    /// Accept the incoming (theirs) side.
    AcceptIncoming,
    /// Accept both sides.
    AcceptBoth,
}

impl MergeResolutionGuidance {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::RegenerateFromSource => "regenerate_from_source",
            Self::AcceptCurrent => "accept_current",
            Self::AcceptIncoming => "accept_incoming",
            Self::AcceptBoth => "accept_both",
        }
    }

    /// Whether this guidance resolves the conflict by picking a side, the way an
    /// ordinary line merge is resolved.
    pub const fn is_direct_side_accept(self) -> bool {
        matches!(
            self,
            Self::AcceptCurrent | Self::AcceptIncoming | Self::AcceptBoth
        )
    }
}

/// Whether a generated artifact is up to date, stale, or diverged from its source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedArtifactState {
    /// The artifact is in sync with its source of truth.
    UpToDate,
    /// The source changed since the artifact was last generated.
    Stale,
    /// The artifact was edited and has diverged from what its source would generate.
    Diverged,
    /// The relationship to the source cannot be determined here.
    GenerationUnknown,
}

impl GeneratedArtifactState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::UpToDate,
        Self::Stale,
        Self::Diverged,
        Self::GenerationUnknown,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UpToDate => "up_to_date",
            Self::Stale => "stale",
            Self::Diverged => "diverged",
            Self::GenerationUnknown => "generation_unknown",
        }
    }

    /// Whether the artifact has diverged from its source.
    pub const fn is_diverged(self) -> bool {
        matches!(self, Self::Diverged)
    }
}

/// The write-back restriction a generated-artifact notice enforces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedWriteBackRestriction {
    /// Compare-only; the artifact is never written back from here.
    CompareOnly,
    /// Regenerate-only; the artifact is rebuilt from source, never hand-edited.
    RegenerateOnly,
    /// Write-back is permitted and stays individually attributable.
    WriteBackAllowed,
}

impl GeneratedWriteBackRestriction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CompareOnly => "compare_only",
            Self::RegenerateOnly => "regenerate_only",
            Self::WriteBackAllowed => "write_back_allowed",
        }
    }

    /// Whether the reused rollback posture matches this write-back restriction, so
    /// a compare-only or regenerate-only artifact is never silently promoted to a
    /// writable posture.
    pub const fn is_consistent_with_posture(
        self,
        posture: M5ArtifactComponentRollbackPosture,
    ) -> bool {
        match self {
            Self::CompareOnly => matches!(
                posture,
                M5ArtifactComponentRollbackPosture::CompareOnlyNoWriteBack
                    | M5ArtifactComponentRollbackPosture::ReadOnlyNoMutation
            ),
            Self::RegenerateOnly => matches!(
                posture,
                M5ArtifactComponentRollbackPosture::RegenerateOnlyNoManualEdit
            ),
            Self::WriteBackAllowed => matches!(
                posture,
                M5ArtifactComponentRollbackPosture::WriteBackAttributable
            ),
        }
    }
}

/// An action a generated-artifact notice may offer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeneratedNoticeAction {
    /// Regenerate the artifact from its source of truth.
    Regenerate,
    /// Open the source of truth this artifact was generated from.
    OpenSource,
    /// Compare the artifact against what its source would generate.
    CompareAgainstSource,
    /// View the generation lineage.
    ViewLineage,
    /// Dismiss the notice.
    Dismiss,
}

impl GeneratedNoticeAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Regenerate => "regenerate",
            Self::OpenSource => "open_source",
            Self::CompareAgainstSource => "compare_against_source",
            Self::ViewLineage => "view_lineage",
            Self::Dismiss => "dismiss",
        }
    }
}

/// Downgrade trigger that can narrow this lane below its claimed qualification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeGeneratedControlsDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// No schema recognizes the artifact class.
    SchemaUnrecognized,
    /// A generated artifact drifted from its source.
    GeneratedArtifactDrifted,
    /// Compare-only safety is enforced; write-back is unavailable.
    CompareOnlyEnforced,
    /// Regenerate-first is enforced; direct write-back is unavailable.
    RegenerateFirstEnforced,
    /// Content was redacted and narrows visible detail.
    RedactionApplied,
    /// Control trust narrowed.
    TrustNarrowing,
    /// An upstream dependency component narrowed.
    UpstreamDependencyNarrowed,
}

impl MergeGeneratedControlsDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::SchemaUnrecognized,
        Self::GeneratedArtifactDrifted,
        Self::CompareOnlyEnforced,
        Self::RegenerateFirstEnforced,
        Self::RedactionApplied,
        Self::TrustNarrowing,
        Self::UpstreamDependencyNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::SchemaUnrecognized => "schema_unrecognized",
            Self::GeneratedArtifactDrifted => "generated_artifact_drifted",
            Self::CompareOnlyEnforced => "compare_only_enforced",
            Self::RegenerateFirstEnforced => "regenerate_first_enforced",
            Self::RedactionApplied => "redaction_applied",
            Self::TrustNarrowing => "trust_narrowing",
            Self::UpstreamDependencyNarrowed => "upstream_dependency_narrowed",
        }
    }
}

/// Consumer surface that must reuse these controls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MergeGeneratedControlsConsumerSurface {
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

impl MergeGeneratedControlsConsumerSurface {
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

/// Disclosures a merge decision row must carry, derived from its conflict class.
///
/// A generated or lockfile conflict makes regenerate-first the safer path; a
/// manifest or policy-owned conflict makes manual reconciliation the safer path;
/// only an ordinary line merge is safe to resolve by picking a side directly.
/// Every non-ordinary class must preserve unknown fields explicitly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MergeDecisionDisclosure {
    /// Whether regenerating from source is safer than a direct write-back.
    pub regenerate_first_safer: bool,
    /// Whether manual reconciliation is safer than picking a side.
    pub manual_resolution_safer: bool,
    /// Whether a direct write-back by picking a side is safe.
    pub direct_write_back_safe: bool,
    /// Whether the row must carry a preserve-unknown-fields note.
    pub needs_preserve_unknown_fields_note: bool,
}

/// Resolves the disclosures a merge decision row must carry from its conflict class.
pub fn resolve_merge_decision_disclosure(
    conflict_class: MergeConflictClass,
) -> MergeDecisionDisclosure {
    let regenerate_first_safer = matches!(
        conflict_class,
        MergeConflictClass::GeneratedArtifactConflict | MergeConflictClass::LockfileConflict
    );
    let manual_resolution_safer = matches!(
        conflict_class,
        MergeConflictClass::ManifestConflict | MergeConflictClass::PolicyOwnedConflict
    );
    MergeDecisionDisclosure {
        regenerate_first_safer,
        manual_resolution_safer,
        direct_write_back_safe: conflict_class.is_ordinary_line_merge(),
        needs_preserve_unknown_fields_note: !conflict_class.is_ordinary_line_merge(),
    }
}

/// Disclosures a generated-artifact notice must carry, derived from its state.
///
/// A stale or diverged artifact must offer a regenerate action and make
/// regenerate-first the recommended path; a diverged artifact must additionally
/// carry a note explaining the divergence from its source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GeneratedNoticeDisclosure {
    /// Whether the notice must offer a regenerate action.
    pub needs_regenerate_action: bool,
    /// Whether the notice must carry a divergence note.
    pub needs_divergence_note: bool,
    /// Whether regenerate-first is the recommended path.
    pub regenerate_first_recommended: bool,
}

/// Resolves the disclosures a generated-artifact notice must carry from its state.
pub fn resolve_generated_notice_disclosure(
    state: GeneratedArtifactState,
) -> GeneratedNoticeDisclosure {
    let needs_regenerate_action = matches!(
        state,
        GeneratedArtifactState::Stale | GeneratedArtifactState::Diverged
    );
    GeneratedNoticeDisclosure {
        needs_regenerate_action,
        needs_divergence_note: state.is_diverged(),
        regenerate_first_recommended: needs_regenerate_action,
    }
}

/// A merge decision row carrying Base / Current / Incoming / Result semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeDecisionRow {
    /// Frozen component this row implements; must be `merge_decision_row`.
    pub component: M5ArtifactComponent,
    /// Stable row id.
    pub row_id: String,
    /// Stable artifact reference; shared with a generated-artifact notice when generated.
    pub artifact_ref: String,
    /// Object identity / key-path in conflict; required and non-empty.
    pub object_path: String,
    /// The conflict class: ordinary / generated / lockfile / manifest / policy-owned.
    pub conflict_class: MergeConflictClass,
    /// Human-readable conflict kind (for example "both modified"); required and non-empty.
    pub conflict_kind: String,
    /// Base-side summary; required and non-empty.
    pub base_summary: String,
    /// Current (ours) side summary; required and non-empty.
    pub current_summary: String,
    /// Incoming (theirs) side summary; required and non-empty.
    pub incoming_summary: String,
    /// Result-side summary after resolution; required and non-empty.
    pub result_summary: String,
    /// Preserve-unknown-fields note; required and non-empty for non-ordinary classes.
    pub preserve_unknown_fields_note: String,
    /// Resolution guidance options offered on this row.
    pub available_guidance: Vec<MergeResolutionGuidance>,
    /// The recommended, safest resolution guidance for this conflict class.
    pub recommended_guidance: MergeResolutionGuidance,
    /// Write-back safety note stating when regenerate-first or manual is safer; required.
    pub write_back_safety_note: String,
    /// Schema fidelity state, reused from the frozen component matrix.
    pub schema_fidelity: M5ArtifactFidelityState,
    /// Raw-context jump action; required and non-empty.
    pub raw_context_action: String,
    /// Rollback / write-back posture, reused from the frozen component matrix.
    pub rollback_posture: M5ArtifactComponentRollbackPosture,
    /// Row fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this row.
    pub source_contract_refs: Vec<String>,
}

impl MergeDecisionRow {
    /// Disclosures this row must carry, derived from its conflict class.
    pub fn disclosure(&self) -> MergeDecisionDisclosure {
        resolve_merge_decision_disclosure(self.conflict_class)
    }

    /// Whether this row offers the given resolution guidance.
    pub fn offers(&self, guidance: MergeResolutionGuidance) -> bool {
        self.available_guidance.contains(&guidance)
    }
}

/// A generated-artifact notice naming the generated-from relation and restrictions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GeneratedArtifactNotice {
    /// Frozen component this notice implements; must be `generated_artifact_notice`.
    pub component: M5ArtifactComponent,
    /// Stable notice id.
    pub notice_id: String,
    /// Stable artifact reference; shared with the paired merge decision row when in conflict.
    pub artifact_ref: String,
    /// Human-readable artifact-class label.
    pub artifact_class_label: String,
    /// Generated-from relation; required and non-empty.
    pub generated_from_relation: String,
    /// Source-of-truth pointer; required and non-empty.
    pub source_of_truth_ref: String,
    /// Whether the artifact is up to date, stale, diverged, or unknown.
    pub generation_state: GeneratedArtifactState,
    /// Last-generated version or time label; required and non-empty.
    pub last_generated_label: String,
    /// Divergence note; required and non-empty when the artifact has diverged.
    pub divergence_note: String,
    /// Actions offered on this notice.
    pub available_actions: Vec<GeneratedNoticeAction>,
    /// The compare-only-versus-write-back restriction.
    pub write_back_restriction: GeneratedWriteBackRestriction,
    /// Write-back restriction note; required and non-empty.
    pub write_back_restriction_note: String,
    /// Schema fidelity state, reused from the frozen component matrix.
    pub schema_fidelity: M5ArtifactFidelityState,
    /// Raw-context jump action; required and non-empty.
    pub raw_context_action: String,
    /// Rollback / write-back posture, reused from the frozen component matrix.
    pub rollback_posture: M5ArtifactComponentRollbackPosture,
    /// Notice fields the surface projects, in display order.
    pub fields_shown: Vec<String>,
    /// Source contract refs consumed by this notice.
    pub source_contract_refs: Vec<String>,
}

impl GeneratedArtifactNotice {
    /// Disclosures this notice must carry, derived from its generation state.
    pub fn disclosure(&self) -> GeneratedNoticeDisclosure {
        resolve_generated_notice_disclosure(self.generation_state)
    }

    /// Whether this notice offers the given action.
    pub fn offers(&self, action: GeneratedNoticeAction) -> bool {
        self.available_actions.contains(&action)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeGeneratedControlsTrustReview {
    /// Base / Current / Incoming / Result semantics stay distinct on a merge row.
    pub base_current_incoming_result_distinct: bool,
    /// A generated / lockfile / manifest / policy conflict is never an ordinary line merge.
    pub special_conflict_never_ordinary_line_merge: bool,
    /// Unknown fields are preserved explicitly for structured conflicts.
    pub unknown_fields_preserved_explicitly: bool,
    /// Regenerate-first or manual is stated when it is safer than write-back.
    pub regenerate_or_manual_stated_when_safer: bool,
    /// Generated-from and source-of-truth relations are always explicit.
    pub generated_from_relation_always_explicit: bool,
    /// Stale / diverged state is always disclosed.
    pub stale_or_diverged_state_disclosed: bool,
    /// A raw-context jump action is always reachable from both controls.
    pub raw_context_always_reachable: bool,
    /// Compare-only artifacts are never silently promoted to writable state.
    pub compare_only_never_silently_writable: bool,
    /// Downgrade narrows the claim rather than hiding the control.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified controls automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl MergeGeneratedControlsTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.base_current_incoming_result_distinct
            && self.special_conflict_never_ordinary_line_merge
            && self.unknown_fields_preserved_explicitly
            && self.regenerate_or_manual_stated_when_safer
            && self.generated_from_relation_always_explicit
            && self.stale_or_diverged_state_disclosed
            && self.raw_context_always_reachable
            && self.compare_only_never_silently_writable
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeGeneratedControlsConsumerProjection {
    /// Merge row shows Base / Current / Incoming / Result and conflict class.
    pub merge_row_shows_bcir_and_conflict_class: bool,
    /// Generated notice shows generated-from relation and restriction.
    pub generated_notice_shows_relation_and_restriction: bool,
    /// Raw context is reachable from both the row and the notice.
    pub raw_context_reachable_from_both: bool,
    /// Regenerate-first guidance is shown where it is safer than write-back.
    pub regenerate_first_guidance_shown: bool,
    /// CLI / headless shows control truth.
    pub cli_headless_shows_truth: bool,
    /// Support export shows control truth.
    pub support_export_shows_truth: bool,
    /// Help / About shows control truth.
    pub help_about_shows_truth: bool,
}

impl MergeGeneratedControlsConsumerProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.merge_row_shows_bcir_and_conflict_class
            && self.generated_notice_shows_relation_and_restriction
            && self.raw_context_reachable_from_both
            && self.regenerate_first_guidance_shown
            && self.cli_headless_shows_truth
            && self.support_export_shows_truth
            && self.help_about_shows_truth
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeGeneratedControlsProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`MergeGeneratedControlsPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MergeGeneratedControlsPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Merge decision rows.
    pub merge_decision_rows: Vec<MergeDecisionRow>,
    /// Generated-artifact notices.
    pub generated_artifact_notices: Vec<GeneratedArtifactNotice>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<MergeGeneratedControlsDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<MergeGeneratedControlsConsumerSurface>,
    /// Trust review block.
    pub trust_review: MergeGeneratedControlsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: MergeGeneratedControlsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: MergeGeneratedControlsProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe merge-decision / generated-notice controls packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MergeGeneratedControlsPacket {
    /// Record kind; must equal [`MERGE_GENERATED_CONTROLS_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`MERGE_GENERATED_CONTROLS_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Merge decision rows.
    pub merge_decision_rows: Vec<MergeDecisionRow>,
    /// Generated-artifact notices.
    pub generated_artifact_notices: Vec<GeneratedArtifactNotice>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<MergeGeneratedControlsDowngradeTrigger>,
    /// Consumer surfaces that must reuse these controls.
    pub consumer_surfaces: Vec<MergeGeneratedControlsConsumerSurface>,
    /// Trust review block.
    pub trust_review: MergeGeneratedControlsTrustReview,
    /// Consumer projection block.
    pub consumer_projection: MergeGeneratedControlsConsumerProjection,
    /// Proof freshness block.
    pub proof_freshness: MergeGeneratedControlsProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl MergeGeneratedControlsPacket {
    /// Builds a merge-decision / generated-notice controls packet from stable-lane input.
    pub fn new(input: MergeGeneratedControlsPacketInput) -> Self {
        Self {
            record_kind: MERGE_GENERATED_CONTROLS_RECORD_KIND.to_owned(),
            schema_version: MERGE_GENERATED_CONTROLS_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            merge_decision_rows: input.merge_decision_rows,
            generated_artifact_notices: input.generated_artifact_notices,
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

    /// Validates the merge-decision / generated-notice controls invariants.
    pub fn validate(&self) -> Vec<MergeGeneratedControlsViolation> {
        let mut violations = Vec::new();

        if self.record_kind != MERGE_GENERATED_CONTROLS_RECORD_KIND {
            violations.push(MergeGeneratedControlsViolation::WrongRecordKind);
        }
        if self.schema_version != MERGE_GENERATED_CONTROLS_SCHEMA_VERSION {
            violations.push(MergeGeneratedControlsViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(MergeGeneratedControlsViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(MergeGeneratedControlsViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(MergeGeneratedControlsViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_merge_decision_rows(self, &mut violations);
        validate_generated_artifact_notices(self, &mut violations);
        validate_pairing(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(MergeGeneratedControlsViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(MergeGeneratedControlsViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(MergeGeneratedControlsViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("merge generated controls packet serializes"),
        ) {
            violations.push(MergeGeneratedControlsViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("merge generated controls packet serializes")
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let special_conflicts = self
            .merge_decision_rows
            .iter()
            .filter(|row| !row.conflict_class.is_ordinary_line_merge())
            .count();
        let stale_or_diverged = self
            .generated_artifact_notices
            .iter()
            .filter(|notice| notice.disclosure().needs_regenerate_action)
            .count();

        let mut out = String::new();
        out.push_str("# Merge Decision Rows & Generated-Artifact Notices\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Merge decision rows: {} ({} non-ordinary conflict classes)\n",
            self.merge_decision_rows.len(),
            special_conflicts
        ));
        out.push_str(&format!(
            "- Generated-artifact notices: {} ({} stale or diverged)\n",
            self.generated_artifact_notices.len(),
            stale_or_diverged
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Merge decision rows\n\n");
        for row in &self.merge_decision_rows {
            out.push_str(&format!(
                "- **{}** [`{}`]: {} conflict ({}) — recommends {}\n",
                row.object_path,
                row.artifact_ref,
                row.conflict_class.as_str(),
                row.conflict_kind,
                row.recommended_guidance.as_str()
            ));
        }

        out.push_str("\n## Generated-artifact notices\n\n");
        for notice in &self.generated_artifact_notices {
            out.push_str(&format!(
                "- **{}** [`{}`]: {} ({}) — from `{}`, restriction {}\n",
                notice.artifact_class_label,
                notice.artifact_ref,
                notice.generation_state.as_str(),
                notice.last_generated_label,
                notice.generated_from_relation,
                notice.write_back_restriction.as_str()
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in merge/generated-controls export.
#[derive(Debug)]
pub enum MergeGeneratedControlsArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<MergeGeneratedControlsViolation>),
}

impl fmt::Display for MergeGeneratedControlsArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "merge generated controls export parse failed: {error}"
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
                    "merge generated controls export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for MergeGeneratedControlsArtifactError {}

/// Validation failures emitted by [`MergeGeneratedControlsPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MergeGeneratedControlsViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No merge decision rows are present.
    MergeDecisionRowsMissing,
    /// A merge decision row is incomplete.
    MergeDecisionRowIncomplete,
    /// A merge decision row carries the wrong frozen component class.
    MergeDecisionRowWrongComponentClass,
    /// A merge decision row does not name its object identity / key-path.
    ObjectIdentityMissing,
    /// A merge decision row does not name its conflict kind.
    ConflictKindMissing,
    /// A merge decision row does not preserve Base / Current / Incoming / Result semantics.
    MergeSemanticsMissing,
    /// A non-ordinary merge conflict does not carry a preserve-unknown-fields note.
    PreserveUnknownFieldsNoteMissing,
    /// A merge decision row does not carry a write-back safety note.
    WriteBackSafetyNoteMissing,
    /// A merge decision row offers no resolution guidance.
    ResolutionGuidanceMissing,
    /// A merge decision row's recommended guidance is not among the offered options.
    RecommendedGuidanceNotOffered,
    /// A regenerate-first-safer conflict does not offer regenerate-from-source guidance.
    RegenerateFirstGuidanceMissing,
    /// A manual-resolution-safer conflict does not offer manual guidance.
    ManualResolutionGuidanceMissing,
    /// A non-ordinary conflict is recommended for a direct side-accept, as if ordinary.
    OrdinaryMergeMisrepresented,
    /// The merge rows do not cover ordinary, generated, and policy-owned conflict classes.
    MergeConflictClassCoverageMissing,
    /// A structure row or generated notice does not carry a raw-context jump action.
    RawContextActionMissing,
    /// No generated-artifact notices are present.
    GeneratedArtifactNoticesMissing,
    /// A generated-artifact notice is incomplete.
    GeneratedArtifactNoticeIncomplete,
    /// A generated-artifact notice carries the wrong frozen component class.
    GeneratedArtifactNoticeWrongComponentClass,
    /// A generated-artifact notice does not name its generated-from relation.
    GeneratedFromRelationMissing,
    /// A generated-artifact notice does not name its source-of-truth pointer.
    SourceOfTruthPointerMissing,
    /// A generated-artifact notice does not carry a last-generated version or time.
    LastGeneratedLabelMissing,
    /// A stale or diverged notice does not offer a regenerate action.
    RegenerateActionMissing,
    /// A generated-artifact notice does not offer an open-source action.
    OpenSourceActionMissing,
    /// A diverged notice does not carry a divergence note.
    DivergenceNoteMissing,
    /// A generated-artifact notice does not carry a write-back restriction note.
    WriteBackRestrictionNoteMissing,
    /// A notice's write-back restriction disagrees with its rollback posture.
    WriteBackRestrictionInconsistent,
    /// The generated notices do not cover up-to-date, stale, and diverged states.
    GeneratedArtifactStateCoverageMissing,
    /// A generated-artifact conflict merge row has no accompanying generated-artifact notice.
    GeneratedConflictNoticeMissing,
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

impl MergeGeneratedControlsViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::MergeDecisionRowsMissing => "merge_decision_rows_missing",
            Self::MergeDecisionRowIncomplete => "merge_decision_row_incomplete",
            Self::MergeDecisionRowWrongComponentClass => "merge_decision_row_wrong_component_class",
            Self::ObjectIdentityMissing => "object_identity_missing",
            Self::ConflictKindMissing => "conflict_kind_missing",
            Self::MergeSemanticsMissing => "merge_semantics_missing",
            Self::PreserveUnknownFieldsNoteMissing => "preserve_unknown_fields_note_missing",
            Self::WriteBackSafetyNoteMissing => "write_back_safety_note_missing",
            Self::ResolutionGuidanceMissing => "resolution_guidance_missing",
            Self::RecommendedGuidanceNotOffered => "recommended_guidance_not_offered",
            Self::RegenerateFirstGuidanceMissing => "regenerate_first_guidance_missing",
            Self::ManualResolutionGuidanceMissing => "manual_resolution_guidance_missing",
            Self::OrdinaryMergeMisrepresented => "ordinary_merge_misrepresented",
            Self::MergeConflictClassCoverageMissing => "merge_conflict_class_coverage_missing",
            Self::RawContextActionMissing => "raw_context_action_missing",
            Self::GeneratedArtifactNoticesMissing => "generated_artifact_notices_missing",
            Self::GeneratedArtifactNoticeIncomplete => "generated_artifact_notice_incomplete",
            Self::GeneratedArtifactNoticeWrongComponentClass => {
                "generated_artifact_notice_wrong_component_class"
            }
            Self::GeneratedFromRelationMissing => "generated_from_relation_missing",
            Self::SourceOfTruthPointerMissing => "source_of_truth_pointer_missing",
            Self::LastGeneratedLabelMissing => "last_generated_label_missing",
            Self::RegenerateActionMissing => "regenerate_action_missing",
            Self::OpenSourceActionMissing => "open_source_action_missing",
            Self::DivergenceNoteMissing => "divergence_note_missing",
            Self::WriteBackRestrictionNoteMissing => "write_back_restriction_note_missing",
            Self::WriteBackRestrictionInconsistent => "write_back_restriction_inconsistent",
            Self::GeneratedArtifactStateCoverageMissing => {
                "generated_artifact_state_coverage_missing"
            }
            Self::GeneratedConflictNoticeMissing => "generated_conflict_notice_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable merge/generated-controls export.
pub fn current_merge_generated_controls_export(
) -> Result<MergeGeneratedControlsPacket, MergeGeneratedControlsArtifactError> {
    let packet: MergeGeneratedControlsPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/release/m5-merge-decision-generated-notice-controls-proof/support_export.json"
    )))
    .map_err(MergeGeneratedControlsArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(MergeGeneratedControlsArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &MergeGeneratedControlsPacket,
    violations: &mut Vec<MergeGeneratedControlsViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        MERGE_GENERATED_CONTROLS_SCHEMA_REF,
        MERGE_GENERATED_CONTROLS_DOC_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_SCHEMA_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_MERGE_DECISION_CONTRACT_REF,
        M5_ARTIFACT_COMPONENT_MATRIX_GENERATED_NOTICE_CONTRACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(MergeGeneratedControlsViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_merge_decision_rows(
    packet: &MergeGeneratedControlsPacket,
    violations: &mut Vec<MergeGeneratedControlsViolation>,
) {
    if packet.merge_decision_rows.is_empty() {
        violations.push(MergeGeneratedControlsViolation::MergeDecisionRowsMissing);
        return;
    }

    let mut classes: BTreeSet<MergeConflictClass> = BTreeSet::new();

    for row in &packet.merge_decision_rows {
        classes.insert(row.conflict_class);

        if row.row_id.trim().is_empty()
            || row.artifact_ref.trim().is_empty()
            || row.fields_shown.is_empty()
            || row.source_contract_refs.is_empty()
        {
            violations.push(MergeGeneratedControlsViolation::MergeDecisionRowIncomplete);
        }
        if row.component != M5ArtifactComponent::MergeDecisionRow {
            violations.push(MergeGeneratedControlsViolation::MergeDecisionRowWrongComponentClass);
        }
        if row.object_path.trim().is_empty() {
            violations.push(MergeGeneratedControlsViolation::ObjectIdentityMissing);
        }
        if row.conflict_kind.trim().is_empty() {
            violations.push(MergeGeneratedControlsViolation::ConflictKindMissing);
        }
        if row.base_summary.trim().is_empty()
            || row.current_summary.trim().is_empty()
            || row.incoming_summary.trim().is_empty()
            || row.result_summary.trim().is_empty()
        {
            violations.push(MergeGeneratedControlsViolation::MergeSemanticsMissing);
        }
        if row.write_back_safety_note.trim().is_empty() {
            violations.push(MergeGeneratedControlsViolation::WriteBackSafetyNoteMissing);
        }
        if row.raw_context_action.trim().is_empty() {
            violations.push(MergeGeneratedControlsViolation::RawContextActionMissing);
        }
        if row.available_guidance.is_empty() {
            violations.push(MergeGeneratedControlsViolation::ResolutionGuidanceMissing);
        } else if !row.offers(row.recommended_guidance) {
            violations.push(MergeGeneratedControlsViolation::RecommendedGuidanceNotOffered);
        }

        let disclosure = row.disclosure();

        if disclosure.needs_preserve_unknown_fields_note
            && row.preserve_unknown_fields_note.trim().is_empty()
        {
            violations.push(MergeGeneratedControlsViolation::PreserveUnknownFieldsNoteMissing);
        }
        if disclosure.regenerate_first_safer
            && !row.offers(MergeResolutionGuidance::RegenerateFromSource)
        {
            violations.push(MergeGeneratedControlsViolation::RegenerateFirstGuidanceMissing);
        }
        if disclosure.manual_resolution_safer && !row.offers(MergeResolutionGuidance::Manual) {
            violations.push(MergeGeneratedControlsViolation::ManualResolutionGuidanceMissing);
        }
        // A non-ordinary conflict resolved by picking a side is masquerading as an
        // ordinary line merge: regenerate-first or manual must be recommended instead.
        if !disclosure.direct_write_back_safe && row.recommended_guidance.is_direct_side_accept() {
            violations.push(MergeGeneratedControlsViolation::OrdinaryMergeMisrepresented);
        }
    }

    for required in [
        MergeConflictClass::OrdinaryLineMerge,
        MergeConflictClass::GeneratedArtifactConflict,
        MergeConflictClass::PolicyOwnedConflict,
    ] {
        if !classes.contains(&required) {
            violations.push(MergeGeneratedControlsViolation::MergeConflictClassCoverageMissing);
            break;
        }
    }
}

fn validate_generated_artifact_notices(
    packet: &MergeGeneratedControlsPacket,
    violations: &mut Vec<MergeGeneratedControlsViolation>,
) {
    if packet.generated_artifact_notices.is_empty() {
        violations.push(MergeGeneratedControlsViolation::GeneratedArtifactNoticesMissing);
        return;
    }

    let mut states: BTreeSet<GeneratedArtifactState> = BTreeSet::new();

    for notice in &packet.generated_artifact_notices {
        states.insert(notice.generation_state);

        if notice.notice_id.trim().is_empty()
            || notice.artifact_ref.trim().is_empty()
            || notice.artifact_class_label.trim().is_empty()
            || notice.fields_shown.is_empty()
            || notice.source_contract_refs.is_empty()
        {
            violations.push(MergeGeneratedControlsViolation::GeneratedArtifactNoticeIncomplete);
        }
        if notice.component != M5ArtifactComponent::GeneratedArtifactNotice {
            violations
                .push(MergeGeneratedControlsViolation::GeneratedArtifactNoticeWrongComponentClass);
        }
        if notice.generated_from_relation.trim().is_empty() {
            violations.push(MergeGeneratedControlsViolation::GeneratedFromRelationMissing);
        }
        if notice.source_of_truth_ref.trim().is_empty() {
            violations.push(MergeGeneratedControlsViolation::SourceOfTruthPointerMissing);
        }
        if notice.last_generated_label.trim().is_empty() {
            violations.push(MergeGeneratedControlsViolation::LastGeneratedLabelMissing);
        }
        if notice.write_back_restriction_note.trim().is_empty() {
            violations.push(MergeGeneratedControlsViolation::WriteBackRestrictionNoteMissing);
        }
        if notice.raw_context_action.trim().is_empty() {
            violations.push(MergeGeneratedControlsViolation::RawContextActionMissing);
        }
        if !notice.offers(GeneratedNoticeAction::OpenSource) {
            violations.push(MergeGeneratedControlsViolation::OpenSourceActionMissing);
        }
        if !notice
            .write_back_restriction
            .is_consistent_with_posture(notice.rollback_posture)
        {
            violations.push(MergeGeneratedControlsViolation::WriteBackRestrictionInconsistent);
        }

        let disclosure = notice.disclosure();
        if disclosure.needs_regenerate_action && !notice.offers(GeneratedNoticeAction::Regenerate) {
            violations.push(MergeGeneratedControlsViolation::RegenerateActionMissing);
        }
        if disclosure.needs_divergence_note && notice.divergence_note.trim().is_empty() {
            violations.push(MergeGeneratedControlsViolation::DivergenceNoteMissing);
        }
    }

    for required in [
        GeneratedArtifactState::UpToDate,
        GeneratedArtifactState::Stale,
        GeneratedArtifactState::Diverged,
    ] {
        if !states.contains(&required) {
            violations.push(MergeGeneratedControlsViolation::GeneratedArtifactStateCoverageMissing);
            break;
        }
    }
}

fn validate_pairing(
    packet: &MergeGeneratedControlsPacket,
    violations: &mut Vec<MergeGeneratedControlsViolation>,
) {
    let notice_refs: BTreeSet<&str> = packet
        .generated_artifact_notices
        .iter()
        .map(|notice| notice.artifact_ref.as_str())
        .collect();
    // Every generated-artifact conflict merge row must be accompanied by a
    // generated-artifact notice for the same artifact, so the regenerate-first
    // path is always visible where a generated artifact is being resolved.
    for row in &packet.merge_decision_rows {
        if row.conflict_class.is_generated_artifact()
            && !notice_refs.contains(row.artifact_ref.as_str())
        {
            violations.push(MergeGeneratedControlsViolation::GeneratedConflictNoticeMissing);
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
