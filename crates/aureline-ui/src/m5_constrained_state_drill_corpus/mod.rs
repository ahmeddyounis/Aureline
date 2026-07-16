//! Seeded constrained-state drill corpus: mixed-state fixtures and regression drills for the problematic
//! constrained-object transitions — symlink / alias save, generated-artifact drift, policy-locked managed mirrors,
//! projection export, captured snapshots inside the current workspace, and unsupported round trips.
//!
//! This module is the B150 fixture-corpus + regression-drill lane over the six constrained-current-object classes
//! frozen in [`crate::m5_constrained_file_state_matrix`]. Where the state-descriptor, badge-group, canonical-source,
//! write-review-sheet, and cross-actor-gate lanes make one honest constrained-object *loop* real, this lane seeds the
//! reusable corpus QA, release, and support pull that proves those loops stay honest under failure: every binding
//! seeds one constrained-object fixture (a read-only alias path, a generated / derived artifact, a policy-locked
//! managed mirror, a projection / virtual view, a managed source, or a captured snapshot) exercised by one of nine
//! drills that attempt a direct write, watch it be denied, and route to the exact reviewed fallback path (duplicate
//! to an editable copy, regenerate with preview, request approval, detach from the managed source, or create an
//! overlay patch) rather than a silent, lossy best-effort write.
//!
//! The three honesty axes mirror the row acceptance criteria.
//!
//! 1. **The fixture corpus covers at least one example of every supported state class plus five mixed-state
//!    combinations.** Every binding carries a primary [`M5ConstrainedFileStateObject`] and an optional co-applicable
//!    class, and the corpus covers all six classes as a primary and at least five distinct mixed-state combinations
//!    (read-only + generated, generated + policy-locked, policy-locked + managed, projection + captured-snapshot, and
//!    managed + captured-snapshot) so a drill can prove that when two state classes materially affect behaviour both
//!    stay visible instead of one badge hiding another.
//! 2. **Automated or scripted drills catch lossy fallback, hidden second-state, or cross-surface disagreement
//!    regressions.** Each binding derives its [`BlockedWriteReason`], its chosen [`WriteReviewFallbackAction`], its
//!    required [`M5ConstrainedFileStateWriteDisposition`], and its [`CheckpointUndoClass`] from its object class
//!    through the shared pure functions, so a lossy direct write, a masked second state, a fallback that does not
//!    match its reason, or a grammar that drifts across surfaces is mechanically rejected.
//! 3. **The first seeded support / export packet can replay a constrained write denial and the chosen fallback path
//!    from fixtures.** Every binding records a [`DenialExpectation`] that names the exact blocked-write reason, the
//!    chosen reviewed fallback, the required write disposition, the checkpoint / undo class, and the reviewed-fallback
//!    ref, and binds back to screenshots, an accessibility check, the CLI / support export, and the health dashboard
//!    through [`CorpusEvidenceBindings`], so the denial-and-fallback replay is discoverable from release and support
//!    evidence rather than an ad hoc local sample set.
//!
//! Every binding names the accessibility routes ([`M5ConstrainedFileStateAccessibilityRoute`]) through which the
//! state class, its canonical source, and its exact write target can be discovered without pointer-only chrome;
//! keyboard focus and screen-reader announcement are mandatory. No drill silently falls back to a lossy direct write,
//! lets one state class hide another, gives AI / automation / import / repair a hidden bypass, or presents a
//! constrained object as directly writable while hiding the recovery / regenerate path.
//!
//! The boundary schema is
//! [`schemas/program/m5-constrained-state-drill-corpus.schema.json`](../../../../schemas/program/m5-constrained-state-drill-corpus.schema.json).
//! The contract doc is
//! [`docs/support/m5_constrained_state_drill_corpus.md`](../../../../docs/support/m5_constrained_state_drill_corpus.md).
//! The protected fixture directory is
//! [`fixtures/editor/m5-constrained-state-drills/`](../../../../fixtures/editor/m5-constrained-state-drills/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_constrained_state_drill_corpus,
    seeded_m5_constrained_state_drill_corpus_mixed_state_narrowed,
    seeded_m5_constrained_state_drill_corpus_read_only_generated_narrowed,
};

use crate::m5_constrained_file_state_matrix::{
    M5ConstrainedFileStateAccessibilityRoute, M5ConstrainedFileStateConsumerSurface,
    M5ConstrainedFileStateObject, M5ConstrainedFileStateRole,
    M5ConstrainedFileStateWriteDisposition, M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF,
    M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
};
use crate::m5_cross_actor_constrained_write_enforcement::BlockedWriteReason;
use crate::m5_write_review_sheet_fallback_paths::{CheckpointUndoClass, WriteReviewFallbackAction};

/// Stable record-kind tag carried by [`M5ConstrainedStateDrillCorpusPacket`].
pub const M5_CONSTRAINED_STATE_DRILL_RECORD_KIND: &str = "m5_constrained_state_drill_corpus";

/// Schema version for constrained-state drill-corpus records.
pub const M5_CONSTRAINED_STATE_DRILL_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_CONSTRAINED_STATE_DRILL_PACKET_ID: &str = "m5-constrained-state-drill:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_CONSTRAINED_STATE_DRILL_SCHEMA_REF: &str =
    "schemas/program/m5-constrained-state-drill-corpus.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_CONSTRAINED_STATE_DRILL_DOC_REF: &str =
    "docs/support/m5_constrained_state_drill_corpus.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_CONSTRAINED_STATE_DRILL_ARTIFACT_REF: &str =
    "artifacts/support/m5-constrained-state-drills/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_CONSTRAINED_STATE_DRILL_CSV_REF: &str =
    "artifacts/support/m5-constrained-state-drills/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_CONSTRAINED_STATE_DRILL_REPORT_REF: &str =
    "artifacts/support/m5-constrained-state-drills/summary.md";

/// Repo-relative path of the checked health dashboard.
pub const M5_CONSTRAINED_STATE_DRILL_DASHBOARD_REF: &str =
    "dashboards/m5-constrained-state-drill-health.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_CONSTRAINED_STATE_DRILL_FIXTURE_DIR: &str =
    "fixtures/editor/m5-constrained-state-drills";

/// Record kind carried by the health dashboard.
pub const M5_CONSTRAINED_STATE_DRILL_DASHBOARD_RECORD_KIND: &str =
    "m5_constrained_state_drill_health";

/// Proof-freshness SLO in hours for this lane.
pub const M5_CONSTRAINED_STATE_DRILL_PROOF_SLO_HOURS: u32 = 720;

/// Write-disposition sentinel words a constrained grammar may never fall back to; a constrained-object fixture whose
/// state role must be present before it is surfaced as a constrained object must always keep a real write-constrained
/// disposition rather than implying the object is directly writable, editable, or unconstrained.
const WRITE_DISPOSITION_UNCONSTRAINED_SENTINELS: [&str; 4] =
    ["none", "directly_writable", "writable", "editable"];

/// Whether a consumer surface is an export / support path that must map an object class back to its canonical
/// contract by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5ConstrainedFileStateConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5ConstrainedFileStateConsumerSurface::SupportExportPacket
    )
}

/// Whether `token` is a member of the frozen [`M5ConstrainedFileStateRole`] vocabulary.
pub fn is_known_constrained_file_state_role_token(token: &str) -> bool {
    constrained_file_state_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5ConstrainedFileStateRole`], if it is one.
pub fn constrained_file_state_role_from_token(token: &str) -> Option<M5ConstrainedFileStateRole> {
    M5ConstrainedFileStateRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// One of the nine seeded drills the corpus exercises against a constrained-object fixture.
///
/// A drill fully determines the primary object class, the optional co-applicable class, the blocked-write reason, the
/// chosen reviewed fallback path, the required write disposition, the checkpoint / undo class, and the parity a
/// binding carries — a single [`DrillScenario`] resolves the whole disclosure through [`resolve_drill_disclosure`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillScenario {
    /// A read-only current object reached through a symlink / alias path: an in-place save is denied and the fallback
    /// duplicates it to an editable copy.
    SymlinkAliasSaveDenied,
    /// A generated / derived artifact that has drifted from its generator: a direct edit is denied and the fallback
    /// regenerates from the canonical source with a preview.
    GeneratedArtifactDriftDenied,
    /// A policy-locked object that is also a managed mirror: an in-place write is denied and the fallback requests
    /// approval, while both the policy-lock and managed-mirror facets stay visible.
    PolicyLockedManagedMirrorDenied,
    /// A projection / virtual view exported into the current workspace, whose backing source is a captured snapshot:
    /// a write is denied and the fallback creates an overlay patch, while both facets stay visible.
    ProjectionExportDenied,
    /// A captured snapshot opened inside the current workspace context: an in-place mutation is denied and the
    /// fallback duplicates it to an editable copy, leaving the snapshot restore-only.
    CapturedSnapshotInWorkspaceDenied,
    /// A managed source pushed through an unsupported round trip: a direct write is denied and the fallback detaches
    /// from the managed source.
    ManagedMirrorRoundTripDenied,
    /// A read-only object that is also a generated artifact: an in-place save is denied and the fallback duplicates to
    /// an editable copy, while both the read-only and generated facets stay visible.
    ReadOnlyGeneratedOverlayDenied,
    /// A generated artifact that is also policy-locked: a direct edit is denied and the fallback regenerates with a
    /// preview, while both the generated and policy-locked facets stay visible.
    GeneratedPolicyLockedRegenDenied,
    /// A managed source that also carries a captured snapshot: a direct write is denied and the fallback detaches from
    /// the managed source, while both the managed and captured-snapshot facets stay visible.
    ManagedCapturedSnapshotRestoreDenied,
}

impl DrillScenario {
    /// Every drill, in declaration order.
    pub const ALL: [Self; 9] = [
        Self::SymlinkAliasSaveDenied,
        Self::GeneratedArtifactDriftDenied,
        Self::PolicyLockedManagedMirrorDenied,
        Self::ProjectionExportDenied,
        Self::CapturedSnapshotInWorkspaceDenied,
        Self::ManagedMirrorRoundTripDenied,
        Self::ReadOnlyGeneratedOverlayDenied,
        Self::GeneratedPolicyLockedRegenDenied,
        Self::ManagedCapturedSnapshotRestoreDenied,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SymlinkAliasSaveDenied => "symlink_alias_save_denied",
            Self::GeneratedArtifactDriftDenied => "generated_artifact_drift_denied",
            Self::PolicyLockedManagedMirrorDenied => "policy_locked_managed_mirror_denied",
            Self::ProjectionExportDenied => "projection_export_denied",
            Self::CapturedSnapshotInWorkspaceDenied => "captured_snapshot_in_workspace_denied",
            Self::ManagedMirrorRoundTripDenied => "managed_mirror_round_trip_denied",
            Self::ReadOnlyGeneratedOverlayDenied => "read_only_generated_overlay_denied",
            Self::GeneratedPolicyLockedRegenDenied => "generated_policy_locked_regen_denied",
            Self::ManagedCapturedSnapshotRestoreDenied => {
                "managed_captured_snapshot_restore_denied"
            }
        }
    }

    /// A stable, human-facing default label for the drill.
    pub const fn stable_label(self) -> &'static str {
        match self {
            Self::SymlinkAliasSaveDenied => {
                "Symlink / alias save (denied, duplicate to editable copy)"
            }
            Self::GeneratedArtifactDriftDenied => {
                "Generated-artifact drift (denied, regenerate with preview)"
            }
            Self::PolicyLockedManagedMirrorDenied => {
                "Policy-locked managed mirror (denied, request approval)"
            }
            Self::ProjectionExportDenied => "Projection export (denied, create overlay patch)",
            Self::CapturedSnapshotInWorkspaceDenied => {
                "Captured snapshot in workspace (denied, duplicate to editable copy)"
            }
            Self::ManagedMirrorRoundTripDenied => {
                "Managed-mirror round trip (denied, detach from managed source)"
            }
            Self::ReadOnlyGeneratedOverlayDenied => {
                "Read-only + generated overlay (denied, duplicate to editable copy)"
            }
            Self::GeneratedPolicyLockedRegenDenied => {
                "Generated + policy-locked regenerate (denied, regenerate with preview)"
            }
            Self::ManagedCapturedSnapshotRestoreDenied => {
                "Managed + captured-snapshot restore (denied, detach from managed source)"
            }
        }
    }

    /// The primary constrained-object class this drill exercises.
    pub const fn primary_object_class(self) -> M5ConstrainedFileStateObject {
        match self {
            Self::SymlinkAliasSaveDenied | Self::ReadOnlyGeneratedOverlayDenied => {
                M5ConstrainedFileStateObject::ReadOnly
            }
            Self::GeneratedArtifactDriftDenied | Self::GeneratedPolicyLockedRegenDenied => {
                M5ConstrainedFileStateObject::Generated
            }
            Self::PolicyLockedManagedMirrorDenied => M5ConstrainedFileStateObject::PolicyLocked,
            Self::ProjectionExportDenied => M5ConstrainedFileStateObject::Projection,
            Self::CapturedSnapshotInWorkspaceDenied => {
                M5ConstrainedFileStateObject::CapturedSnapshot
            }
            Self::ManagedMirrorRoundTripDenied | Self::ManagedCapturedSnapshotRestoreDenied => {
                M5ConstrainedFileStateObject::Managed
            }
        }
    }

    /// The co-applicable (second) constrained-object class this drill exercises, if it is a mixed-state drill.
    pub const fn co_applicable_object_class(self) -> Option<M5ConstrainedFileStateObject> {
        match self {
            Self::ReadOnlyGeneratedOverlayDenied => Some(M5ConstrainedFileStateObject::Generated),
            Self::GeneratedPolicyLockedRegenDenied => {
                Some(M5ConstrainedFileStateObject::PolicyLocked)
            }
            Self::PolicyLockedManagedMirrorDenied => Some(M5ConstrainedFileStateObject::Managed),
            Self::ProjectionExportDenied | Self::ManagedCapturedSnapshotRestoreDenied => {
                Some(M5ConstrainedFileStateObject::CapturedSnapshot)
            }
            _ => None,
        }
    }

    /// Whether this drill exercises a mixed-state (two-class) transition.
    pub const fn is_mixed_state(self) -> bool {
        self.co_applicable_object_class().is_some()
    }
}

/// Whether a binding is a single-state or a mixed-state (both facets visible) denial.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillParity {
    /// A single constrained state class denied a direct write; one badge, one blocked-write reason.
    SingleStateDirectWriteDenied,
    /// Two constrained state classes both materially affect behaviour and both stay visible on the denial.
    MixedStateBothFacetsVisible,
}

impl DrillParity {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SingleStateDirectWriteDenied => "single_state_direct_write_denied",
            Self::MixedStateBothFacetsVisible => "mixed_state_both_facets_visible",
        }
    }
}

/// The action a drill surface may expose.
///
/// The set is deliberately closed and inspect / export / review-only: there is no direct-write, save-in-place, apply,
/// or sync action, so a drill surface can never silently mutate a constrained object. Every denial offers exactly the
/// reviewed fallback path rather than a lossy best-effort write.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillDenialAction {
    /// Inspect the blocked-write reason and the state-class badge, metadata-only.
    InspectBlockedWriteReason,
    /// Export the denial-and-fallback evidence record.
    ExportDenialEvidence,
    /// Open the reviewed fallback path (duplicate / regenerate / request-approval / detach / overlay) as a reviewed
    /// transition before any commit.
    OpenReviewedFallbackPath,
}

impl DrillDenialAction {
    /// The inspect / export base action set present on every drill surface.
    pub const BASE: [Self; 2] = [Self::InspectBlockedWriteReason, Self::ExportDenialEvidence];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectBlockedWriteReason => "inspect_blocked_write_reason",
            Self::ExportDenialEvidence => "export_denial_evidence",
            Self::OpenReviewedFallbackPath => "open_reviewed_fallback_path",
        }
    }
}

/// Downgrade trigger that can narrow this drill-corpus lane below its claimed coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConstrainedStateDrillDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Constrained-state grammar drifted between surfaces for the same fixture.
    GrammarDriftDetected,
    /// A drill dropped its constrained-state classification and began to present the object as directly writable.
    ConstrainedStateClassificationDropped,
    /// A drill's blocked-write reason drifted apart from its object class.
    BlockedReasonClassMismatch,
    /// A drill's chosen fallback path drifted apart from its blocked-write reason.
    FallbackReasonMismatch,
    /// A drill silently fell back to a lossy direct write instead of the reviewed fallback path.
    SilentLossyDirectWriteObserved,
    /// A mixed-state drill hid its co-applicable second state behind the primary badge.
    HiddenSecondStateObserved,
    /// A fixture's rendering disagreed across two consumer surfaces.
    CrossSurfaceDisagreementObserved,
    /// A binding lost its canonical-source or exact-write-target join.
    CanonicalSourceOrWriteTargetMissing,
    /// An AI / automation / import / repair path was given a hidden bypass around the constrained-state rules.
    AiAutomationBypassObserved,
    /// A binding lost its corpus evidence bindings (screenshots, accessibility, CLI export, dashboard).
    CorpusEvidenceBindingsMissing,
    /// An accessibility route for the state class, canonical source, or write target was dropped.
    AccessibilityRouteDropped,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream constrained-file-state contract narrowed.
    UpstreamConstrainedFileStateNarrowed,
}

impl ConstrainedStateDrillDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 15] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::GrammarDriftDetected,
        Self::ConstrainedStateClassificationDropped,
        Self::BlockedReasonClassMismatch,
        Self::FallbackReasonMismatch,
        Self::SilentLossyDirectWriteObserved,
        Self::HiddenSecondStateObserved,
        Self::CrossSurfaceDisagreementObserved,
        Self::CanonicalSourceOrWriteTargetMissing,
        Self::AiAutomationBypassObserved,
        Self::CorpusEvidenceBindingsMissing,
        Self::AccessibilityRouteDropped,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamConstrainedFileStateNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::GrammarDriftDetected => "grammar_drift_detected",
            Self::ConstrainedStateClassificationDropped => {
                "constrained_state_classification_dropped"
            }
            Self::BlockedReasonClassMismatch => "blocked_reason_class_mismatch",
            Self::FallbackReasonMismatch => "fallback_reason_mismatch",
            Self::SilentLossyDirectWriteObserved => "silent_lossy_direct_write_observed",
            Self::HiddenSecondStateObserved => "hidden_second_state_observed",
            Self::CrossSurfaceDisagreementObserved => "cross_surface_disagreement_observed",
            Self::CanonicalSourceOrWriteTargetMissing => "canonical_source_or_write_target_missing",
            Self::AiAutomationBypassObserved => "ai_automation_bypass_observed",
            Self::CorpusEvidenceBindingsMissing => "corpus_evidence_bindings_missing",
            Self::AccessibilityRouteDropped => "accessibility_route_dropped",
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamConstrainedFileStateNarrowed => {
                "upstream_constrained_file_state_narrowed"
            }
        }
    }
}

/// The controlled constrained-state grammar a fixture presents.
///
/// These six words describe the constrained-object side of a fixture and must be identical across every drill that
/// renders the same fixture. The state-role word must be a frozen [`M5ConstrainedFileStateRole`] token; the rest are
/// controlled words the fixture carries so it stays attributable to its state class, canonical source, and exact
/// write target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConstrainedStateGrammar {
    /// State-role word (must be a frozen [`M5ConstrainedFileStateRole`] token).
    pub state_role_word: String,
    /// The state-class badge label word.
    pub state_class_label_word: String,
    /// The blocked-write-reason word.
    pub blocked_write_reason_word: String,
    /// The canonical-source / owning-authority word the object relates back to.
    pub canonical_source_word: String,
    /// The exact-write-target word a write-capable action would touch.
    pub exact_write_target_word: String,
    /// The write-disposition (posture) word; must stay write-constrained, never an unconstrained sentinel.
    pub write_disposition_word: String,
}

impl ConstrainedStateGrammar {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.state_role_word.trim().is_empty()
            && !self.state_class_label_word.trim().is_empty()
            && !self.blocked_write_reason_word.trim().is_empty()
            && !self.canonical_source_word.trim().is_empty()
            && !self.exact_write_target_word.trim().is_empty()
            && !self.write_disposition_word.trim().is_empty()
    }

    /// Whether the state-role word is a member of the frozen role vocabulary.
    pub fn state_role_word_in_vocabulary(&self) -> bool {
        is_known_constrained_file_state_role_token(self.state_role_word.trim())
    }

    /// Whether the canonical-source and exact-write-target words that keep the object honest are both present.
    pub fn canonical_source_and_write_target_present(&self) -> bool {
        !self.canonical_source_word.trim().is_empty()
            && !self.exact_write_target_word.trim().is_empty()
    }

    /// Whether the profile honours the write-constrained rule: a state role that must be present before the object may
    /// be surfaced as a constrained object must pair it with a real write-constrained disposition word and never
    /// collapse to a directly-writable / editable / writable / none sentinel.
    pub fn write_disposition_constrained_satisfied(&self) -> bool {
        match constrained_file_state_role_from_token(self.state_role_word.trim()) {
            Some(role) if role.must_be_present_before_surfacing_as_constrained_object() => {
                let disposition = self.write_disposition_word.trim().to_lowercase();
                !disposition.is_empty()
                    && !WRITE_DISPOSITION_UNCONSTRAINED_SENTINELS.contains(&disposition.as_str())
            }
            _ => true,
        }
    }
}

/// The join that keeps a seeded fixture attributable to its canonical source and exact write target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CanonicalSourceJoin {
    /// Stable id / ref of the canonical source the constrained object relates back to.
    pub canonical_source_ref: String,
    /// Stable id / ref of the exact write target a write-capable action would touch.
    pub exact_write_target_ref: String,
    /// Stable id / ref of the owning authority (generator, policy owner, or managing owner).
    pub owning_authority_ref: String,
    /// Stable id / ref of the preserved-versus-lost sync-or-regenerate note.
    pub preserved_versus_lost_sync_ref: String,
}

impl CanonicalSourceJoin {
    /// Whether every join ref is present, so the fixture is fully attributable.
    pub fn all_present(&self) -> bool {
        !self.canonical_source_ref.trim().is_empty()
            && !self.exact_write_target_ref.trim().is_empty()
            && !self.owning_authority_ref.trim().is_empty()
            && !self.preserved_versus_lost_sync_ref.trim().is_empty()
    }
}

/// The denial expectation a drill records: its exact blocked-write reason, chosen fallback path, required write
/// disposition, checkpoint / undo class, and the controlled replay refs. This is the AC3 replay record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DenialExpectation {
    /// The blocked-write reason this drill names (a pure function of the primary object class).
    pub blocked_write_reason: BlockedWriteReason,
    /// The reviewed fallback path this drill routes to.
    pub chosen_fallback_path: WriteReviewFallbackAction,
    /// The write disposition the reviewed transition requires.
    pub required_write_disposition: M5ConstrainedFileStateWriteDisposition,
    /// The checkpoint / undo class the reviewed transition preserves.
    pub checkpoint_undo_class: CheckpointUndoClass,
    /// The reviewed-fallback packet ref; always present since every denial offers a reviewed transition.
    pub reviewed_fallback_ref: Option<String>,
    /// The co-applicable second-state ref; present only when the drill is mixed-state.
    pub co_applicable_state_ref: Option<String>,
    /// The explicit denial-and-fallback note (never omitted); names the exact reason and chosen fallback in plain
    /// words.
    pub denial_note: String,
}

/// The refs binding a drill back to release / support evidence: screenshots, an accessibility check, the CLI /
/// support export, and the health dashboard.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CorpusEvidenceBindings {
    /// The screenshot artifact ref for this drill.
    pub screenshot_ref: String,
    /// The accessibility-check artifact ref for this drill.
    pub accessibility_check_ref: String,
    /// The CLI / support export ref the corpus is minted into.
    pub cli_support_export_ref: String,
    /// The health dashboard ref that surfaces this corpus.
    pub health_dashboard_ref: String,
}

impl CorpusEvidenceBindings {
    /// Whether every corpus evidence ref is present.
    pub fn all_present(&self) -> bool {
        !self.screenshot_ref.trim().is_empty()
            && !self.accessibility_check_ref.trim().is_empty()
            && !self.cli_support_export_ref.trim().is_empty()
            && !self.health_dashboard_ref.trim().is_empty()
    }
}

/// Disclosures a drill binding must carry, derived from its drill scenario.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrillDisclosure {
    /// The primary constrained-object class the drill exercises.
    pub primary_object_class: M5ConstrainedFileStateObject,
    /// The co-applicable second class, if the drill is mixed-state.
    pub co_applicable_object_class: Option<M5ConstrainedFileStateObject>,
    /// The blocked-write reason the drill names.
    pub blocked_write_reason: BlockedWriteReason,
    /// The reviewed fallback path the drill routes to.
    pub chosen_fallback_path: WriteReviewFallbackAction,
    /// The write disposition the reviewed transition requires.
    pub required_write_disposition: M5ConstrainedFileStateWriteDisposition,
    /// The checkpoint / undo class the reviewed transition preserves.
    pub checkpoint_undo_class: CheckpointUndoClass,
    /// Whether the drill is mixed-state.
    pub is_mixed_state: bool,
    /// The parity the drill requires.
    pub parity: DrillParity,
    /// Whether the drill offers a direct-write action (always `false`; every constrained write is denied).
    pub offers_direct_write: bool,
}

/// Resolves the disclosures a drill binding must carry from its scenario.
///
/// Every drill denies the direct write and routes to exactly the reviewed fallback path keyed to the primary object
/// class through the shared pure functions ([`BlockedWriteReason::for_object_class`],
/// [`BlockedWriteReason::safe_next_step`], [`WriteReviewFallbackAction::required_write_disposition`], and
/// [`WriteReviewFallbackAction::required_checkpoint_undo_class`]). A mixed-state drill additionally names a
/// co-applicable second class and requires both facets to stay visible.
pub fn resolve_drill_disclosure(drill: DrillScenario) -> DrillDisclosure {
    let primary = drill.primary_object_class();
    let co_applicable = drill.co_applicable_object_class();
    let blocked_write_reason = BlockedWriteReason::for_object_class(primary);
    let chosen_fallback_path = blocked_write_reason.safe_next_step();
    DrillDisclosure {
        primary_object_class: primary,
        co_applicable_object_class: co_applicable,
        blocked_write_reason,
        chosen_fallback_path,
        required_write_disposition: chosen_fallback_path.required_write_disposition(),
        checkpoint_undo_class: chosen_fallback_path.required_checkpoint_undo_class(),
        is_mixed_state: co_applicable.is_some(),
        parity: if co_applicable.is_some() {
            DrillParity::MixedStateBothFacetsVisible
        } else {
            DrillParity::SingleStateDirectWriteDenied
        },
        offers_direct_write: false,
    }
}

/// One drill binding: a seeded constrained-object fixture exercised by one drill on one consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillCorpusBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable seeded-fixture id (shared across surfaces that show the same fixture).
    pub fixture_id: String,
    /// Human-readable seeded-fixture identity.
    pub fixture_label: String,
    /// Which primary constrained-object class this fixture belongs to.
    pub object_class: M5ConstrainedFileStateObject,
    /// The co-applicable second class, if this drill is mixed-state.
    pub co_applicable_object_class: Option<M5ConstrainedFileStateObject>,
    /// Which consumer surface renders it.
    pub consumer: M5ConstrainedFileStateConsumerSurface,
    /// The drill exercised on this fixture.
    pub drill: DrillScenario,
    /// A stable, human-facing drill label.
    pub drill_label: String,
    /// The blocked-write reason this drill names.
    pub blocked_write_reason: BlockedWriteReason,
    /// The reviewed fallback path this drill routes to.
    pub chosen_fallback_path: WriteReviewFallbackAction,
    /// The write disposition the reviewed transition requires.
    pub write_disposition: M5ConstrainedFileStateWriteDisposition,
    /// The checkpoint / undo class the reviewed transition preserves.
    pub checkpoint_undo_class: CheckpointUndoClass,
    /// The controlled constrained-state grammar presented (identical across surfaces for one fixture).
    pub constrained_grammar: ConstrainedStateGrammar,
    /// Whether this drill exercises a mixed-state transition.
    pub is_mixed_state: bool,
    /// Whether one or both state facets are visible.
    pub parity_state: DrillParity,
    /// The inspect / export / review-only action set allowed on this drill surface.
    pub allowed_actions: Vec<DrillDenialAction>,
    /// The accessibility routes through which the state class, canonical source, and write target can be discovered
    /// without pointer-only chrome.
    pub accessibility_routes: Vec<M5ConstrainedFileStateAccessibilityRoute>,
    /// The canonical-source / exact-write-target join keeping this fixture attributable.
    pub canonical_source_join: CanonicalSourceJoin,
    /// The denial expectation this drill records (the AC3 replay record).
    pub denial_expectation: DenialExpectation,
    /// The refs binding this drill back to release / support evidence.
    pub corpus_evidence: CorpusEvidenceBindings,
    /// The constrained state class is explicitly classified. MUST be `true`.
    pub constrained_state_explicitly_classified: bool,
    /// When mixed-state, both state facets are visible. MUST be `true`.
    pub both_state_facets_visible_when_mixed: bool,
    /// Guardrail: this drill lets one constrained state class hide another. MUST be `false`.
    pub lets_one_constrained_state_class_hide_another: bool,
    /// Guardrail: this drill silently falls back to a lossy direct write. MUST be `false`.
    pub silently_falls_back_to_lossy_direct_write: bool,
    /// Guardrail: this drill gives AI / automation / import / repair a hidden bypass. MUST be `false`.
    pub gives_ai_automation_import_or_repair_a_hidden_bypass: bool,
    /// Guardrail: this drill leaves the canonical source or exact write target unstated. MUST be `false`.
    pub leaves_canonical_source_or_exact_write_target_unstated: bool,
    /// Guardrail: this drill presents the object as directly writable or hides the recovery / regenerate path. MUST be
    /// `false`.
    pub presents_as_directly_writable_or_hides_recovery_path: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl DrillCorpusBinding {
    /// Disclosures this binding must carry, derived from its drill scenario.
    pub fn disclosure(&self) -> DrillDisclosure {
        resolve_drill_disclosure(self.drill)
    }

    /// Whether this drill exercises a mixed-state transition.
    pub const fn is_mixed(&self) -> bool {
        self.is_mixed_state
    }

    /// Whether the blocked-write reason matches the primary object class (the actor-independent reason vocabulary).
    pub fn blocked_reason_matches_class(&self) -> bool {
        self.blocked_write_reason == BlockedWriteReason::for_object_class(self.object_class)
    }

    /// Whether the chosen fallback path matches the blocked-write reason's safe next step.
    pub fn fallback_matches_reason(&self) -> bool {
        self.chosen_fallback_path == self.blocked_write_reason.safe_next_step()
    }

    /// Whether the write disposition matches the chosen fallback path's required disposition.
    pub fn disposition_matches_fallback(&self) -> bool {
        self.write_disposition == self.chosen_fallback_path.required_write_disposition()
    }

    /// Whether the checkpoint / undo class matches the chosen fallback path's required class.
    pub fn checkpoint_matches_fallback(&self) -> bool {
        self.checkpoint_undo_class == self.chosen_fallback_path.required_checkpoint_undo_class()
    }

    /// Whether every guardrail row-invariant holds (state explicitly classified, both facets visible when mixed, all
    /// guardrails false).
    pub const fn guardrails_hold(&self) -> bool {
        self.constrained_state_explicitly_classified
            && self.both_state_facets_visible_when_mixed
            && !self.lets_one_constrained_state_class_hide_another
            && !self.silently_falls_back_to_lossy_direct_write
            && !self.gives_ai_automation_import_or_repair_a_hidden_bypass
            && !self.leaves_canonical_source_or_exact_write_target_unstated
            && !self.presents_as_directly_writable_or_hides_recovery_path
    }

    /// Whether the inspect / export base action set is present.
    pub fn has_base_actions(&self) -> bool {
        DrillDenialAction::BASE
            .iter()
            .all(|action| self.allowed_actions.contains(action))
    }

    /// Whether no direct-write / save / apply / sync affordance leaked in (structurally guaranteed by the closed
    /// action enum, but checked so the invariant is explicit).
    pub fn action_set_is_closed(&self) -> bool {
        self.allowed_actions.iter().all(|action| {
            matches!(
                action,
                DrillDenialAction::InspectBlockedWriteReason
                    | DrillDenialAction::ExportDenialEvidence
                    | DrillDenialAction::OpenReviewedFallbackPath
            )
        })
    }

    /// Whether the reviewed-fallback action is offered (it is on every denial, so a constrained write always routes to
    /// a reviewed transition rather than a silent lossy write).
    pub fn reviewed_fallback_action_present(&self) -> bool {
        self.allowed_actions
            .contains(&DrillDenialAction::OpenReviewedFallbackPath)
    }

    /// Whether the mixed-state facets are internally consistent: the co-applicable class, the mixed flag, and the
    /// parity all agree, and a mixed-state binding keeps both facets visible.
    pub fn mixed_state_facets_consistent(&self) -> bool {
        let disclosure = self.disclosure();
        if self.is_mixed_state != disclosure.is_mixed_state {
            return false;
        }
        if self.co_applicable_object_class != disclosure.co_applicable_object_class {
            return false;
        }
        if self.parity_state != disclosure.parity {
            return false;
        }
        let denial_has_co_ref = self.denial_expectation.co_applicable_state_ref.is_some();
        if denial_has_co_ref != self.is_mixed_state {
            return false;
        }
        if self.is_mixed_state {
            self.both_state_facets_visible_when_mixed
                && !self.lets_one_constrained_state_class_hide_another
        } else {
            true
        }
    }

    /// Whether the binding renders its canonical source and exact write target instead of leaving them unstated.
    pub fn renders_canonical_source_and_write_target(&self) -> bool {
        self.canonical_source_join.all_present()
            && self
                .constrained_grammar
                .canonical_source_and_write_target_present()
            && !self.denial_expectation.denial_note.trim().is_empty()
            && !self.leaves_canonical_source_or_exact_write_target_unstated
    }

    /// Whether keyboard focus and screen-reader announcement are both discoverable.
    pub fn accessibility_state_discoverable(&self) -> bool {
        self.accessibility_routes
            .contains(&M5ConstrainedFileStateAccessibilityRoute::KeyboardFocusable)
            && self
                .accessibility_routes
                .contains(&M5ConstrainedFileStateAccessibilityRoute::ScreenReaderAnnounced)
    }

    /// Whether this binding points at the canonical per-domain schema and the matrix.
    pub fn points_at_canonical_contracts(&self) -> bool {
        let domain_ref = self.object_class.canonical_domain_schema_ref();
        self.source_contract_refs
            .iter()
            .any(|reference| reference == domain_ref)
            && self
                .source_contract_refs
                .iter()
                .any(|reference| reference == M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillCorpusTrustReview {
    /// The corpus covers every supported constrained-object class as a primary.
    pub covers_every_object_class_as_primary: bool,
    /// The corpus covers five or more distinct mixed-state combinations.
    pub covers_five_or_more_mixed_state_combinations: bool,
    /// The exact blocked-write reasons are mechanically distinguishable.
    pub exact_blocked_write_reasons_are_distinguishable: bool,
    /// Every object class is seeded by two or more consumers.
    pub every_object_class_seeded_by_two_or_more_consumers: bool,
    /// The same fixture presents the same constrained-state grammar across surfaces.
    pub constrained_grammar_identical_for_same_fixture: bool,
    /// Every state-role word is a frozen role token.
    pub state_role_words_stay_in_frozen_vocabulary: bool,
    /// Canonical source and exact write target are present on every binding.
    pub canonical_source_and_write_target_present_on_every_binding: bool,
    /// Every denial routes to the reviewed fallback path keyed to its state class.
    pub every_denial_routes_to_reviewed_fallback_keyed_to_state_class: bool,
    /// No drill silently falls back to a lossy direct write.
    pub no_drill_silently_falls_back_to_lossy_direct_write: bool,
    /// No mixed-state drill hides its second state.
    pub no_mixed_state_drill_hides_second_state: bool,
    /// No fixture disagrees across surfaces.
    pub no_cross_surface_disagreement: bool,
    /// No AI / automation / import / repair path gets a hidden bypass.
    pub no_ai_automation_import_or_repair_bypass: bool,
    /// The first support / export packet can replay a denial and chosen fallback path.
    pub support_export_can_replay_denial_and_fallback: bool,
    /// The corpus is bound to screenshots, accessibility checks, the CLI / support export, and dashboards.
    pub corpus_bound_to_screenshots_accessibility_cli_and_dashboards: bool,
    /// The corpus is referenced by release evidence and support drills, not an ad hoc sample set.
    pub corpus_referenced_by_release_and_support_not_ad_hoc: bool,
    /// Accessibility routes for the state class, canonical source, and write target are present.
    pub accessibility_routes_present_for_state_source_and_target: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the object class.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl DrillCorpusTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.covers_every_object_class_as_primary
            && self.covers_five_or_more_mixed_state_combinations
            && self.exact_blocked_write_reasons_are_distinguishable
            && self.every_object_class_seeded_by_two_or_more_consumers
            && self.constrained_grammar_identical_for_same_fixture
            && self.state_role_words_stay_in_frozen_vocabulary
            && self.canonical_source_and_write_target_present_on_every_binding
            && self.every_denial_routes_to_reviewed_fallback_keyed_to_state_class
            && self.no_drill_silently_falls_back_to_lossy_direct_write
            && self.no_mixed_state_drill_hides_second_state
            && self.no_cross_surface_disagreement
            && self.no_ai_automation_import_or_repair_bypass
            && self.support_export_can_replay_denial_and_fallback
            && self.corpus_bound_to_screenshots_accessibility_cli_and_dashboards
            && self.corpus_referenced_by_release_and_support_not_ad_hoc
            && self.accessibility_routes_present_for_state_source_and_target
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillCorpusProjection {
    /// The tab-chrome surface consumes the corpus.
    pub tab_chrome_consumes_corpus: bool,
    /// The breadcrumb-trail surface consumes the corpus.
    pub breadcrumb_trail_consumes_corpus: bool,
    /// The status-bar surface consumes the corpus.
    pub status_bar_consumes_corpus: bool,
    /// The command-palette surface consumes the corpus.
    pub command_palette_consumes_corpus: bool,
    /// The editor-banner surface consumes the corpus.
    pub editor_banner_consumes_corpus: bool,
    /// The diff / review-header surface consumes the corpus.
    pub diff_review_header_consumes_corpus: bool,
    /// The write-review-sheet surface consumes the corpus.
    pub write_review_sheet_consumes_corpus: bool,
    /// The AI / automation-path surface consumes the corpus.
    pub ai_automation_path_consumes_corpus: bool,
    /// The support / export-packet surface consumes the corpus.
    pub support_export_packet_consumes_corpus: bool,
    /// Every object class is stated by two or more consumers.
    pub every_object_class_stated_by_two_or_more_consumers: bool,
    /// Constrained-state grammar is identical for the same fixture.
    pub constrained_grammar_identical_for_same_fixture: bool,
    /// The constrained state is disclosed rather than hidden.
    pub constrained_state_disclosed_not_hidden: bool,
    /// Export maps a drill row back to one constrained-object class.
    pub drill_maps_back_to_one_constrained_object: bool,
}

impl DrillCorpusProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.tab_chrome_consumes_corpus
            && self.breadcrumb_trail_consumes_corpus
            && self.status_bar_consumes_corpus
            && self.command_palette_consumes_corpus
            && self.editor_banner_consumes_corpus
            && self.diff_review_header_consumes_corpus
            && self.write_review_sheet_consumes_corpus
            && self.ai_automation_path_consumes_corpus
            && self.support_export_packet_consumes_corpus
            && self.every_object_class_stated_by_two_or_more_consumers
            && self.constrained_grammar_identical_for_same_fixture
            && self.constrained_state_disclosed_not_hidden
            && self.drill_maps_back_to_one_constrained_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillCorpusProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5ConstrainedStateDrillCorpusPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5ConstrainedStateDrillCorpusPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Drill bindings.
    pub drill_bindings: Vec<DrillCorpusBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ConstrainedStateDrillDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ConstrainedFileStateConsumerSurface>,
    /// Trust review block.
    pub trust_review: DrillCorpusTrustReview,
    /// Consumer projection block.
    pub consumer_projection: DrillCorpusProjection,
    /// Proof freshness block.
    pub proof_freshness: DrillCorpusProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe constrained-state drill-corpus packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5ConstrainedStateDrillCorpusPacket {
    /// Record kind; must equal [`M5_CONSTRAINED_STATE_DRILL_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_CONSTRAINED_STATE_DRILL_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Drill bindings.
    pub drill_bindings: Vec<DrillCorpusBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<ConstrainedStateDrillDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5ConstrainedFileStateConsumerSurface>,
    /// Trust review block.
    pub trust_review: DrillCorpusTrustReview,
    /// Consumer projection block.
    pub consumer_projection: DrillCorpusProjection,
    /// Proof freshness block.
    pub proof_freshness: DrillCorpusProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5ConstrainedStateDrillCorpusPacket {
    /// Builds a drill-corpus packet from stable-lane input.
    pub fn new(input: M5ConstrainedStateDrillCorpusPacketInput) -> Self {
        Self {
            record_kind: M5_CONSTRAINED_STATE_DRILL_RECORD_KIND.to_owned(),
            schema_version: M5_CONSTRAINED_STATE_DRILL_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            drill_bindings: input.drill_bindings,
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

    /// Validates the drill-corpus invariants.
    pub fn validate(&self) -> Vec<M5ConstrainedStateDrillViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_CONSTRAINED_STATE_DRILL_RECORD_KIND {
            violations.push(M5ConstrainedStateDrillViolation::WrongRecordKind);
        }
        if self.schema_version != M5_CONSTRAINED_STATE_DRILL_SCHEMA_VERSION {
            violations.push(M5ConstrainedStateDrillViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5ConstrainedStateDrillViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5ConstrainedStateDrillViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5ConstrainedStateDrillViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5ConstrainedStateDrillViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5ConstrainedStateDrillViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5ConstrainedStateDrillViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("drill-corpus packet serializes"),
        ) {
            violations.push(M5ConstrainedStateDrillViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("drill-corpus packet serializes")
    }

    /// Deterministic matrix CSV, one row per drill binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "object_class,co_applicable_class,consumer,drill,blocked_write_reason,chosen_fallback_path,write_disposition,parity_state,fixture_id\n",
        );
        for binding in &self.drill_bindings {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{},{}\n",
                binding.object_class.as_str(),
                binding
                    .co_applicable_object_class
                    .map_or("none", |class| class.as_str()),
                binding.consumer.as_str(),
                binding.drill.as_str(),
                binding.blocked_write_reason.as_str(),
                binding.chosen_fallback_path.as_str(),
                binding.write_disposition.as_str(),
                binding.parity_state.as_str(),
                binding.fixture_id.replace(',', ";"),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mixed = self
            .drill_bindings
            .iter()
            .filter(|binding| binding.is_mixed_state)
            .count();

        let mut out = String::new();
        out.push_str(
            "# Constrained-State Drill Corpus: Mixed-State Fixtures and Regression Drills\n\n",
        );
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Drill bindings: {} ({} exercise a mixed-state transition)\n",
            self.drill_bindings.len(),
            mixed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Drill bindings\n\n");
        for binding in &self.drill_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: object `{}`{} on `{}`, drill `{}`, reason `{}`, fallback `{}`, disposition `{}`, parity `{}`\n",
                binding.fixture_label,
                binding.binding_id,
                binding.object_class.as_str(),
                binding
                    .co_applicable_object_class
                    .map_or(String::new(), |class| format!(" (+ `{}`)", class.as_str())),
                binding.consumer.as_str(),
                binding.drill.as_str(),
                binding.blocked_write_reason.as_str(),
                binding.chosen_fallback_path.as_str(),
                binding.write_disposition.as_str(),
                binding.parity_state.as_str(),
            ));
        }
        out
    }

    /// Deterministic health dashboard JSON, minted from truth, so release / support can surface this corpus.
    pub fn render_health_dashboard(&self) -> String {
        let dashboard = DrillHealthDashboard {
            record_kind: M5_CONSTRAINED_STATE_DRILL_DASHBOARD_RECORD_KIND,
            packet_id: &self.packet_id,
            support_export_ref: M5_CONSTRAINED_STATE_DRILL_ARTIFACT_REF,
            corpus_schema_ref: M5_CONSTRAINED_STATE_DRILL_SCHEMA_REF,
            matrix_schema_ref: M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
            drills: DrillScenario::ALL.iter().map(|d| d.as_str()).collect(),
            blocked_write_reasons: BlockedWriteReason::ALL.iter().map(|r| r.as_str()).collect(),
            fallback_paths: WriteReviewFallbackAction::ALL
                .iter()
                .map(|f| f.as_str())
                .collect(),
            mixed_state_combinations: mixed_state_combinations(&self.drill_bindings),
            fixture_families: M5ConstrainedFileStateObject::ALL
                .iter()
                .map(|object_class| DrillFixtureFamily {
                    object_class: object_class.as_str(),
                    canonical_schema: object_class.canonical_domain_schema_ref(),
                })
                .collect(),
        };
        serde_json::to_string_pretty(&dashboard).expect("drill dashboard serializes")
    }
}

/// The distinct mixed-state combinations present in the corpus, as `primary+co` tokens in a stable sorted order.
fn mixed_state_combinations(bindings: &[DrillCorpusBinding]) -> Vec<String> {
    let mut combos: BTreeSet<String> = BTreeSet::new();
    for binding in bindings {
        if let Some(co) = binding.co_applicable_object_class {
            combos.insert(format!("{}+{}", binding.object_class.as_str(), co.as_str()));
        }
    }
    combos.into_iter().collect()
}

#[derive(Serialize)]
struct DrillHealthDashboard<'a> {
    record_kind: &'a str,
    packet_id: &'a str,
    support_export_ref: &'a str,
    corpus_schema_ref: &'a str,
    matrix_schema_ref: &'a str,
    drills: Vec<&'a str>,
    blocked_write_reasons: Vec<&'a str>,
    fallback_paths: Vec<&'a str>,
    mixed_state_combinations: Vec<String>,
    fixture_families: Vec<DrillFixtureFamily<'a>>,
}

#[derive(Serialize)]
struct DrillFixtureFamily<'a> {
    object_class: &'a str,
    canonical_schema: &'a str,
}

/// Errors emitted when reading the checked-in drill-corpus export.
#[derive(Debug)]
pub enum M5ConstrainedStateDrillArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5ConstrainedStateDrillViolation>),
}

impl fmt::Display for M5ConstrainedStateDrillArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "drill-corpus export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "drill-corpus export failed validation: {tokens}")
            }
        }
    }
}

impl Error for M5ConstrainedStateDrillArtifactError {}

/// Validation failures emitted by [`M5ConstrainedStateDrillCorpusPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5ConstrainedStateDrillViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No drill bindings are present.
    DrillBindingsMissing,
    /// A drill binding is incomplete.
    BindingIncomplete,
    /// A binding's constrained-state grammar values are incomplete.
    GrammarFacetIncomplete,
    /// A binding's state-role word is not a frozen role token.
    StateRoleWordOutsideVocabulary,
    /// A binding's gate-role dropped its write-constrained disposition.
    WriteDispositionUnconstrainedForGateRole,
    /// A binding's object class does not match its drill.
    ObjectClassDrillMismatch,
    /// A binding's co-applicable class does not match its drill.
    CoApplicableClassMismatch,
    /// A binding's blocked-write reason does not match its object class.
    BlockedReasonClassMismatch,
    /// A binding's chosen fallback path does not match its blocked-write reason.
    FallbackReasonMismatch,
    /// A binding's write disposition does not match its chosen fallback path.
    WriteDispositionFallbackMismatch,
    /// A binding's checkpoint / undo class does not match its chosen fallback path.
    CheckpointFallbackMismatch,
    /// A binding's parity state does not match its drill.
    ParityStateMismatch,
    /// A binding's mixed-state facets are inconsistent.
    MixedStateFacetsInconsistent,
    /// Two surfaces show the same fixture with different constrained-state grammar.
    GrammarDriftAcrossSurfaces,
    /// A shared object class is not seeded by at least two distinct consumers.
    ObjectClassReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A binding is missing a stable drill label.
    DrillLabelMissing,
    /// A binding's canonical-source / exact-write-target join is incomplete.
    CanonicalSourceJoinIncomplete,
    /// A binding's reviewed-fallback ref presence does not match its drill.
    ReviewedFallbackRefMismatch,
    /// A binding's co-applicable-state ref presence does not match its drill.
    CoApplicableStateRefMismatch,
    /// A binding is missing its denial note.
    DenialNoteMissing,
    /// A binding's denial-expectation fields drift from the binding.
    DenialExpectationMismatch,
    /// A binding is missing its corpus evidence bindings.
    CorpusEvidenceBindingsMissing,
    /// A binding is missing the inspect / export base action set.
    BaseActionsMissing,
    /// A binding's action set is not the closed drill action set.
    ActionSetNotClosed,
    /// A binding does not offer the reviewed fallback path.
    ReviewedFallbackActionMissing,
    /// A binding leaves its canonical source or exact write target unstated.
    CanonicalSourceOrWriteTargetUnstated,
    /// A binding cannot discover its state via keyboard focus and screen-reader announcement.
    AccessibilityStateUndiscoverable,
    /// A binding's constrained state is not explicitly classified.
    ConstrainedStateNotClassified,
    /// A mixed-state binding does not keep both facets visible.
    SecondStateHidden,
    /// A binding lets one constrained state class hide another.
    LetsOneConstrainedStateClassHideAnother,
    /// A binding silently falls back to a lossy direct write.
    SilentlyFallsBackToLossyDirectWrite,
    /// A binding gives AI / automation / import / repair a hidden bypass.
    GivesAiAutomationImportOrRepairAHiddenBypass,
    /// A binding presents the object as directly writable or hides the recovery / regenerate path.
    PresentsAsDirectlyWritableOrHidesRecoveryPath,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every constrained-object class appears among the bindings.
    ObjectClassCoverageMissing,
    /// Not every drill appears among the bindings.
    DrillCoverageMissing,
    /// Fewer than five distinct mixed-state combinations appear.
    MixedStateComboCoverageInsufficient,
    /// Not every blocked-write reason appears among the bindings.
    BlockedWriteReasonCoverageMissing,
    /// Not every reviewed fallback path appears among the bindings.
    FallbackPathCoverageMissing,
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

impl M5ConstrainedStateDrillViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::DrillBindingsMissing => "drill_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::GrammarFacetIncomplete => "grammar_facet_incomplete",
            Self::StateRoleWordOutsideVocabulary => "state_role_word_outside_vocabulary",
            Self::WriteDispositionUnconstrainedForGateRole => {
                "write_disposition_unconstrained_for_gate_role"
            }
            Self::ObjectClassDrillMismatch => "object_class_drill_mismatch",
            Self::CoApplicableClassMismatch => "co_applicable_class_mismatch",
            Self::BlockedReasonClassMismatch => "blocked_reason_class_mismatch",
            Self::FallbackReasonMismatch => "fallback_reason_mismatch",
            Self::WriteDispositionFallbackMismatch => "write_disposition_fallback_mismatch",
            Self::CheckpointFallbackMismatch => "checkpoint_fallback_mismatch",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::MixedStateFacetsInconsistent => "mixed_state_facets_inconsistent",
            Self::GrammarDriftAcrossSurfaces => "grammar_drift_across_surfaces",
            Self::ObjectClassReuseUnproven => "object_class_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::DrillLabelMissing => "drill_label_missing",
            Self::CanonicalSourceJoinIncomplete => "canonical_source_join_incomplete",
            Self::ReviewedFallbackRefMismatch => "reviewed_fallback_ref_mismatch",
            Self::CoApplicableStateRefMismatch => "co_applicable_state_ref_mismatch",
            Self::DenialNoteMissing => "denial_note_missing",
            Self::DenialExpectationMismatch => "denial_expectation_mismatch",
            Self::CorpusEvidenceBindingsMissing => "corpus_evidence_bindings_missing",
            Self::BaseActionsMissing => "base_actions_missing",
            Self::ActionSetNotClosed => "action_set_not_closed",
            Self::ReviewedFallbackActionMissing => "reviewed_fallback_action_missing",
            Self::CanonicalSourceOrWriteTargetUnstated => {
                "canonical_source_or_write_target_unstated"
            }
            Self::AccessibilityStateUndiscoverable => "accessibility_state_undiscoverable",
            Self::ConstrainedStateNotClassified => "constrained_state_not_classified",
            Self::SecondStateHidden => "second_state_hidden",
            Self::LetsOneConstrainedStateClassHideAnother => {
                "lets_one_constrained_state_class_hide_another"
            }
            Self::SilentlyFallsBackToLossyDirectWrite => {
                "silently_falls_back_to_lossy_direct_write"
            }
            Self::GivesAiAutomationImportOrRepairAHiddenBypass => {
                "gives_ai_automation_import_or_repair_a_hidden_bypass"
            }
            Self::PresentsAsDirectlyWritableOrHidesRecoveryPath => {
                "presents_as_directly_writable_or_hides_recovery_path"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ObjectClassCoverageMissing => "object_class_coverage_missing",
            Self::DrillCoverageMissing => "drill_coverage_missing",
            Self::MixedStateComboCoverageInsufficient => "mixed_state_combo_coverage_insufficient",
            Self::BlockedWriteReasonCoverageMissing => "blocked_write_reason_coverage_missing",
            Self::FallbackPathCoverageMissing => "fallback_path_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable drill-corpus export.
pub fn current_stable_m5_constrained_state_drill_corpus_export(
) -> Result<M5ConstrainedStateDrillCorpusPacket, M5ConstrainedStateDrillArtifactError> {
    let packet: M5ConstrainedStateDrillCorpusPacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/support/m5-constrained-state-drills/support_export.json"
    )))
    .map_err(M5ConstrainedStateDrillArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5ConstrainedStateDrillArtifactError::Validation(violations))
    }
}

fn validate_source_contracts(
    packet: &M5ConstrainedStateDrillCorpusPacket,
    violations: &mut Vec<M5ConstrainedStateDrillViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_CONSTRAINED_STATE_DRILL_SCHEMA_REF,
        M5_CONSTRAINED_STATE_DRILL_DOC_REF,
        M5_CONSTRAINED_FILE_STATE_MATRIX_SCHEMA_REF,
        M5_CONSTRAINED_FILE_STATE_MATRIX_DOC_REF,
    ];
    // The six object classes map to three canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5ConstrainedFileStateObject::ALL {
        domains.insert(object_class.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5ConstrainedStateDrillViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5ConstrainedStateDrillCorpusPacket,
    violations: &mut Vec<M5ConstrainedStateDrillViolation>,
) {
    if packet.drill_bindings.is_empty() {
        violations.push(M5ConstrainedStateDrillViolation::DrillBindingsMissing);
        return;
    }

    // One vocabulary: the constrained-state grammar must be identical for every binding that renders the same fixture.
    let mut fixture_grammar: BTreeMap<&str, &ConstrainedStateGrammar> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object class must be seeded by at least two distinct consumers.
    let mut object_consumers: BTreeMap<
        M5ConstrainedFileStateObject,
        BTreeSet<M5ConstrainedFileStateConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5ConstrainedFileStateConsumerSurface> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5ConstrainedFileStateObject> = BTreeSet::new();
    let mut seen_drills: BTreeSet<DrillScenario> = BTreeSet::new();
    let mut seen_reasons: BTreeSet<BlockedWriteReason> = BTreeSet::new();
    let mut seen_fallbacks: BTreeSet<WriteReviewFallbackAction> = BTreeSet::new();
    let mut seen_mixed_combos: BTreeSet<(
        M5ConstrainedFileStateObject,
        M5ConstrainedFileStateObject,
    )> = BTreeSet::new();

    for binding in &packet.drill_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.fixture_id.trim().is_empty()
            || binding.fixture_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5ConstrainedStateDrillViolation::BindingIncomplete);
        }
        if binding.drill_label.trim().is_empty() {
            violations.push(M5ConstrainedStateDrillViolation::DrillLabelMissing);
        }
        if !binding.constrained_grammar.all_present() {
            violations.push(M5ConstrainedStateDrillViolation::GrammarFacetIncomplete);
        }
        if !binding.constrained_grammar.state_role_word_in_vocabulary() {
            violations.push(M5ConstrainedStateDrillViolation::StateRoleWordOutsideVocabulary);
        }
        if !binding
            .constrained_grammar
            .write_disposition_constrained_satisfied()
        {
            violations
                .push(M5ConstrainedStateDrillViolation::WriteDispositionUnconstrainedForGateRole);
        }

        let disclosure = binding.disclosure();

        if binding.object_class != disclosure.primary_object_class {
            violations.push(M5ConstrainedStateDrillViolation::ObjectClassDrillMismatch);
        }
        if binding.co_applicable_object_class != disclosure.co_applicable_object_class {
            violations.push(M5ConstrainedStateDrillViolation::CoApplicableClassMismatch);
        }
        if !binding.blocked_reason_matches_class() {
            violations.push(M5ConstrainedStateDrillViolation::BlockedReasonClassMismatch);
        }
        if !binding.fallback_matches_reason() {
            violations.push(M5ConstrainedStateDrillViolation::FallbackReasonMismatch);
        }
        if !binding.disposition_matches_fallback() {
            violations.push(M5ConstrainedStateDrillViolation::WriteDispositionFallbackMismatch);
        }
        if !binding.checkpoint_matches_fallback() {
            violations.push(M5ConstrainedStateDrillViolation::CheckpointFallbackMismatch);
        }
        if binding.parity_state != disclosure.parity {
            violations.push(M5ConstrainedStateDrillViolation::ParityStateMismatch);
        }
        if !binding.mixed_state_facets_consistent() {
            violations.push(M5ConstrainedStateDrillViolation::MixedStateFacetsInconsistent);
        }

        // Canonical-source / exact-write-target join: always present, always joined back.
        if !binding.canonical_source_join.all_present() {
            violations.push(M5ConstrainedStateDrillViolation::CanonicalSourceJoinIncomplete);
        }

        // Denial expectation refs and values match the drill disclosure.
        let expectation = &binding.denial_expectation;
        if expectation.blocked_write_reason != binding.blocked_write_reason
            || expectation.chosen_fallback_path != binding.chosen_fallback_path
            || expectation.required_write_disposition != binding.write_disposition
            || expectation.checkpoint_undo_class != binding.checkpoint_undo_class
        {
            violations.push(M5ConstrainedStateDrillViolation::DenialExpectationMismatch);
        }
        // The reviewed-fallback ref is always present (every denial offers a reviewed transition).
        if expectation.reviewed_fallback_ref.is_none() {
            violations.push(M5ConstrainedStateDrillViolation::ReviewedFallbackRefMismatch);
        }
        // The co-applicable-state ref is present exactly for mixed-state drills.
        if expectation.co_applicable_state_ref.is_some() != binding.is_mixed_state {
            violations.push(M5ConstrainedStateDrillViolation::CoApplicableStateRefMismatch);
        }
        if expectation.denial_note.trim().is_empty() {
            violations.push(M5ConstrainedStateDrillViolation::DenialNoteMissing);
        }

        // Corpus evidence bindings (screenshots, accessibility, CLI export, dashboard).
        if !binding.corpus_evidence.all_present() {
            violations.push(M5ConstrainedStateDrillViolation::CorpusEvidenceBindingsMissing);
        }

        // Action rules.
        if !binding.has_base_actions() {
            violations.push(M5ConstrainedStateDrillViolation::BaseActionsMissing);
        }
        if !binding.action_set_is_closed() {
            violations.push(M5ConstrainedStateDrillViolation::ActionSetNotClosed);
        }
        if !binding.reviewed_fallback_action_present() {
            violations.push(M5ConstrainedStateDrillViolation::ReviewedFallbackActionMissing);
        }

        // Canonical-source / write-target honesty.
        if !binding.renders_canonical_source_and_write_target() {
            violations.push(M5ConstrainedStateDrillViolation::CanonicalSourceOrWriteTargetUnstated);
        }

        // Accessibility discovery.
        if !binding.accessibility_state_discoverable() {
            violations.push(M5ConstrainedStateDrillViolation::AccessibilityStateUndiscoverable);
        }

        // Guardrail row-invariants.
        if !binding.constrained_state_explicitly_classified {
            violations.push(M5ConstrainedStateDrillViolation::ConstrainedStateNotClassified);
        }
        if binding.is_mixed_state && !binding.both_state_facets_visible_when_mixed {
            violations.push(M5ConstrainedStateDrillViolation::SecondStateHidden);
        }
        if binding.lets_one_constrained_state_class_hide_another {
            violations
                .push(M5ConstrainedStateDrillViolation::LetsOneConstrainedStateClassHideAnother);
        }
        if binding.silently_falls_back_to_lossy_direct_write {
            violations.push(M5ConstrainedStateDrillViolation::SilentlyFallsBackToLossyDirectWrite);
        }
        if binding.gives_ai_automation_import_or_repair_a_hidden_bypass {
            violations.push(
                M5ConstrainedStateDrillViolation::GivesAiAutomationImportOrRepairAHiddenBypass,
            );
        }
        if binding.presents_as_directly_writable_or_hides_recovery_path {
            violations.push(
                M5ConstrainedStateDrillViolation::PresentsAsDirectlyWritableOrHidesRecoveryPath,
            );
        }

        // Support / export consumers must map an object class back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5ConstrainedStateDrillViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match fixture_grammar.get(binding.fixture_id.as_str()) {
            None => {
                fixture_grammar.insert(binding.fixture_id.as_str(), &binding.constrained_grammar);
            }
            Some(existing) => {
                if **existing != binding.constrained_grammar && !drift_reported {
                    violations.push(M5ConstrainedStateDrillViolation::GrammarDriftAcrossSurfaces);
                    drift_reported = true;
                }
            }
        }

        object_consumers
            .entry(binding.object_class)
            .or_default()
            .insert(binding.consumer);
        seen_consumers.insert(binding.consumer);
        seen_objects.insert(binding.object_class);
        seen_drills.insert(binding.drill);
        seen_reasons.insert(binding.blocked_write_reason);
        seen_fallbacks.insert(binding.chosen_fallback_path);
        if let Some(co) = binding.co_applicable_object_class {
            seen_mixed_combos.insert((binding.object_class, co));
        }
    }

    // Coverage: every consumer surface, object class, and drill must appear.
    for consumer in M5ConstrainedFileStateConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5ConstrainedStateDrillViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for object_class in M5ConstrainedFileStateObject::ALL {
        if !seen_objects.contains(&object_class) {
            violations.push(M5ConstrainedStateDrillViolation::ObjectClassCoverageMissing);
            break;
        }
    }
    for drill in DrillScenario::ALL {
        if !seen_drills.contains(&drill) {
            violations.push(M5ConstrainedStateDrillViolation::DrillCoverageMissing);
            break;
        }
    }
    for reason in BlockedWriteReason::ALL {
        if !seen_reasons.contains(&reason) {
            violations.push(M5ConstrainedStateDrillViolation::BlockedWriteReasonCoverageMissing);
            break;
        }
    }
    for fallback in WriteReviewFallbackAction::ALL {
        if !seen_fallbacks.contains(&fallback) {
            violations.push(M5ConstrainedStateDrillViolation::FallbackPathCoverageMissing);
            break;
        }
    }

    // AC1: every object class as a primary and at least five distinct mixed-state combinations.
    if seen_mixed_combos.len() < 5 {
        violations.push(M5ConstrainedStateDrillViolation::MixedStateComboCoverageInsufficient);
    }

    // Reuse: every present object class must be seeded by two or more distinct consumers.
    for consumers in object_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5ConstrainedStateDrillViolation::ObjectClassReuseUnproven);
            break;
        }
    }
}

/// Heuristic that rejects obviously forbidden material in export-safe JSON.
fn json_contains_forbidden_boundary_material(value: &serde_json::Value) -> bool {
    match value {
        serde_json::Value::String(s) => {
            let lower = s.to_lowercase();
            lower.contains("password")
                || lower.contains("passphrase")
                || lower.contains("bearer ")
                || lower.contains("://")
                || lower.contains("-----begin")
        }
        serde_json::Value::Array(arr) => arr.iter().any(json_contains_forbidden_boundary_material),
        serde_json::Value::Object(map) => {
            map.values().any(json_contains_forbidden_boundary_material)
        }
        _ => false,
    }
}
