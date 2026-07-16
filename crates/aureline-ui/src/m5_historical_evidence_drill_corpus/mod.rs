//! Seeded historical-reference drill corpus: archived-snapshot, imported / offline evidence, and live-target
//! handoff fixtures plus regression drills for retired lines, missing targets, stale captures, expired snapshots,
//! and evidence-only reopen paths.
//!
//! This module is the B149 fixture-corpus + regression-drill lane over the five non-live-evidence object classes
//! frozen in [`crate::m5_historical_reference_matrix`]. Where the archive-viewer, expiry-state, live-target-handoff,
//! and lineage-propagation lanes make one honest non-live evidence *loop* real, this lane seeds the reusable corpus
//! QA, release, and support pull to prove those loops stay honest under failure: a last-supported retirement
//! snapshot, a captured support / export evidence bundle, a runbook / incident archived packet, and an imported /
//! offline route packet — each with known provenance and handoff expectations — exercised by six drills that either
//! clear the live-target handoff or block it with an exact, named blocker (missing target, trust block, route
//! unavailable, expired snapshot, or imported / offline evidence only) and fall back to a satisfy-prerequisite or
//! metadata-only exit rather than a dead end.
//!
//! The three honesty axes mirror the row acceptance criteria.
//!
//! 1. **The seeded fixtures exercise at least four distinct historical-reference states and two distinct
//!    live-target handoff outcomes.** Every binding carries a [`HistoricalReferenceDrillState`] and a
//!    [`HandoffOutcome`], and the corpus covers all six states and all four outcomes so a drill can prove a
//!    preserved live-target-joinable snapshot, a missing-target metadata-only fallback, a retired-line reopen
//!    refusal, a stale-import trust block, an expired-snapshot metadata fallback, and an evidence-only reopen after
//!    version / schema drift.
//! 2. **QA / support automation can distinguish exact blockers.** Each binding names a [`DrillBlocker`] whose
//!    required [`HandoffOutcome`] is validated, and whose meaning maps into the live-target-handoff module's own
//!    [`HandoffBlockerReason`] vocabulary, so `missing_target`, `trust_block`, `route_unavailable`,
//!    `expired_snapshot`, and `imported_offline_evidence_only` are mechanically separable rather than a single
//!    generic failure.
//! 3. **The corpus is referenced by release evidence and support drills rather than living as an ad hoc local
//!    sample set.** Every binding binds back to screenshots, an accessibility check, the CLI / support export, and
//!    the health dashboard through [`CorpusEvidenceBindings`], and the packet points at the canonical matrix and
//!    per-domain schemas, so the corpus is discoverable from the checked-in release and support surfaces.
//!
//! Every binding names the accessibility routes ([`M5HistoricalReferenceAccessibilityRoute`]) through which the
//! non-live boundary, its provenance, and its handoff expectation can be discovered without pointer-only chrome;
//! keyboard focus and screen-reader announcement are mandatory. The historical side stays visibly non-live and
//! mutation blocked throughout, and no drill reopens a live target implicitly, dead-links an expired artifact, or
//! presents imported / offline evidence as current live truth.
//!
//! The boundary schema is
//! [`schemas/program/m5-historical-evidence-drill-corpus.schema.json`](../../../../schemas/program/m5-historical-evidence-drill-corpus.schema.json).
//! The contract doc is
//! [`docs/support/m5_historical_evidence_drill_corpus.md`](../../../../docs/support/m5_historical_evidence_drill_corpus.md).
//! The protected fixture directory is
//! [`fixtures/recovery/m5-historical-evidence-drills/`](../../../../fixtures/recovery/m5-historical-evidence-drills/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_historical_evidence_drill_corpus,
    seeded_m5_historical_evidence_drill_corpus_expired_snapshot_narrowed,
    seeded_m5_historical_evidence_drill_corpus_missing_target_narrowed,
};

use crate::m5_historical_reference_matrix::{
    M5HistoricalReferenceAccessibilityRoute, M5HistoricalReferenceConsumerSurface,
    M5HistoricalReferenceObject, M5HistoricalReferenceRole, M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF,
    M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
};
use crate::m5_live_target_handoff_packet_and_route_validation::{
    HandoffBlockerReason, HandoffOutcome,
};

/// Stable record-kind tag carried by [`M5HistoricalEvidenceDrillCorpusPacket`].
pub const M5_HISTORICAL_EVIDENCE_DRILL_RECORD_KIND: &str = "m5_historical_evidence_drill_corpus";

/// Schema version for historical-evidence drill-corpus records.
pub const M5_HISTORICAL_EVIDENCE_DRILL_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_HISTORICAL_EVIDENCE_DRILL_PACKET_ID: &str = "m5-historical-evidence-drill:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_HISTORICAL_EVIDENCE_DRILL_SCHEMA_REF: &str =
    "schemas/program/m5-historical-evidence-drill-corpus.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_HISTORICAL_EVIDENCE_DRILL_DOC_REF: &str =
    "docs/support/m5_historical_evidence_drill_corpus.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_HISTORICAL_EVIDENCE_DRILL_ARTIFACT_REF: &str =
    "artifacts/support/m5-historical-evidence-drills/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_HISTORICAL_EVIDENCE_DRILL_CSV_REF: &str =
    "artifacts/support/m5-historical-evidence-drills/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_HISTORICAL_EVIDENCE_DRILL_REPORT_REF: &str =
    "artifacts/support/m5-historical-evidence-drills/summary.md";

/// Repo-relative path of the checked health dashboard.
pub const M5_HISTORICAL_EVIDENCE_DRILL_DASHBOARD_REF: &str =
    "dashboards/m5-historical-evidence-drill-health.json";

/// Repo-relative path of the protected fixture directory.
pub const M5_HISTORICAL_EVIDENCE_DRILL_FIXTURE_DIR: &str =
    "fixtures/recovery/m5-historical-evidence-drills";

/// Record kind carried by the health dashboard.
pub const M5_HISTORICAL_EVIDENCE_DRILL_DASHBOARD_RECORD_KIND: &str =
    "m5_historical_evidence_drill_health";

/// Proof-freshness SLO in hours for this lane.
pub const M5_HISTORICAL_EVIDENCE_DRILL_PROOF_SLO_HOURS: u32 = 720;

/// Mutation-blocked-posture sentinel words a non-live grammar may never fall back to; a historical-reference
/// fixture whose historical role must be present before surfacing as non-live evidence must always keep a real
/// mutation-blocked posture rather than implying the object is editable, live, writable, or current.
const MUTATION_BLOCKED_POSTURE_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "editable",
    "live_object",
    "writable",
    "current_object",
];

/// Whether a consumer surface is an export / support path that must map an object class back to its canonical
/// contract by id.
pub const fn consumer_must_reference_canonical(
    consumer: M5HistoricalReferenceConsumerSurface,
) -> bool {
    matches!(
        consumer,
        M5HistoricalReferenceConsumerSurface::Support
            | M5HistoricalReferenceConsumerSurface::CliExport
    )
}

/// Whether `token` is a member of the frozen [`M5HistoricalReferenceRole`] vocabulary.
pub fn is_known_historical_reference_role_token(token: &str) -> bool {
    historical_reference_role_from_token(token).is_some()
}

/// Resolves `token` to a frozen [`M5HistoricalReferenceRole`], if it is one.
pub fn historical_reference_role_from_token(token: &str) -> Option<M5HistoricalReferenceRole> {
    M5HistoricalReferenceRole::ALL
        .iter()
        .copied()
        .find(|role| role.as_str() == token)
}

/// One of the six seeded drills the corpus exercises against a historical-reference fixture.
///
/// A drill fully determines the historical-reference state, the live-target handoff outcome, the exact blocker,
/// the parity, the discoverable actions, and the handoff-expectation refs a binding carries — a single
/// [`DrillScenario`] resolves the whole disclosure through [`resolve_drill_disclosure`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillScenario {
    /// The preserved, joinable baseline: every precondition clears and the drill offers an explicit
    /// open-current-live-object exit through the validated live-target handoff.
    PreservedLiveTargetHandoff,
    /// The current live object was removed; the handoff falls back to a metadata-only exit instead of a dead end.
    MissingLiveTarget,
    /// A retired line's live counterpart no longer exists; the reopen is refused and a satisfy-prerequisite
    /// (migration) fallback is offered rather than an implicit reopen.
    RetiredLineReopen,
    /// The imported evidence is stale; the handoff blocks on a trust prerequisite until fresh evidence is imported.
    StaleImportedEvidence,
    /// The snapshot's retention window closed and its content bytes are gone; metadata, capture time, and
    /// provenance render instead of a dead link.
    ExpiredSnapshotMetadataOnlyFallback,
    /// Version / schema drift means no live object can be reopened from the snapshot; only imported / offline
    /// evidence remains and it stays non-live.
    EvidenceOnlyReopenAfterVersionSchemaDrift,
}

impl DrillScenario {
    /// Every drill, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PreservedLiveTargetHandoff,
        Self::MissingLiveTarget,
        Self::RetiredLineReopen,
        Self::StaleImportedEvidence,
        Self::ExpiredSnapshotMetadataOnlyFallback,
        Self::EvidenceOnlyReopenAfterVersionSchemaDrift,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreservedLiveTargetHandoff => "preserved_live_target_handoff",
            Self::MissingLiveTarget => "missing_live_target",
            Self::RetiredLineReopen => "retired_line_reopen",
            Self::StaleImportedEvidence => "stale_imported_evidence",
            Self::ExpiredSnapshotMetadataOnlyFallback => "expired_snapshot_metadata_only_fallback",
            Self::EvidenceOnlyReopenAfterVersionSchemaDrift => {
                "evidence_only_reopen_after_version_schema_drift"
            }
        }
    }

    /// A stable, human-facing default label for the drill.
    pub const fn stable_label(self) -> &'static str {
        match self {
            Self::PreservedLiveTargetHandoff => "Preserved snapshot (live target joinable)",
            Self::MissingLiveTarget => "Missing live target (metadata-only fallback)",
            Self::RetiredLineReopen => "Retired-line reopen (refused, migration prerequisite)",
            Self::StaleImportedEvidence => "Stale imported evidence (trust prerequisite block)",
            Self::ExpiredSnapshotMetadataOnlyFallback => {
                "Expired snapshot (metadata-only fallback, no dead link)"
            }
            Self::EvidenceOnlyReopenAfterVersionSchemaDrift => {
                "Evidence-only reopen after version / schema drift"
            }
        }
    }

    /// Whether this drill clears the live-target handoff (the joinable baseline).
    pub const fn clears_handoff(self) -> bool {
        matches!(self, Self::PreservedLiveTargetHandoff)
    }
}

/// The historical-reference state a seeded fixture exercises; mechanically distinct from a live object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoricalReferenceDrillState {
    /// A preserved snapshot whose live target still exists and is joinable through a validated handoff.
    PreservedLiveTargetJoinable,
    /// A preserved snapshot whose live target was removed; only its metadata remains.
    MissingLiveTargetMetadataOnly,
    /// A retired line with no live counterpart to reopen.
    RetiredLineNoLiveCounterpart,
    /// Imported evidence that has gone stale relative to current trust / freshness.
    StaleImportedEvidence,
    /// An expired snapshot whose content is gone but whose metadata still renders.
    ExpiredSnapshotMetadataFallback,
    /// Imported / offline evidence only, with no reopenable live object after drift.
    ImportedOfflineEvidenceOnly,
}

impl HistoricalReferenceDrillState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::PreservedLiveTargetJoinable,
        Self::MissingLiveTargetMetadataOnly,
        Self::RetiredLineNoLiveCounterpart,
        Self::StaleImportedEvidence,
        Self::ExpiredSnapshotMetadataFallback,
        Self::ImportedOfflineEvidenceOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PreservedLiveTargetJoinable => "preserved_live_target_joinable",
            Self::MissingLiveTargetMetadataOnly => "missing_live_target_metadata_only",
            Self::RetiredLineNoLiveCounterpart => "retired_line_no_live_counterpart",
            Self::StaleImportedEvidence => "stale_imported_evidence",
            Self::ExpiredSnapshotMetadataFallback => "expired_snapshot_metadata_fallback",
            Self::ImportedOfflineEvidenceOnly => "imported_offline_evidence_only",
        }
    }

    /// Every seeded state is non-live evidence; the corpus never seeds a live object as a drill fixture.
    pub const fn is_non_live_evidence(self) -> bool {
        true
    }
}

/// The exact blocker a drill names so QA / support automation can mechanically separate failure modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillBlocker {
    /// No blocker: the handoff cleared and an open-current-live-object exit is offered.
    NoneCleared,
    /// The live target does not exist / was removed.
    MissingTarget,
    /// The trust posture is insufficient (revalidation / fresh evidence required).
    TrustBlock,
    /// The route to the live target is unavailable (retired line, no live counterpart route).
    RouteUnavailable,
    /// The snapshot expired; its retention window closed and the content is gone.
    ExpiredSnapshot,
    /// Only imported / offline evidence remains; no live object can be reopened.
    ImportedOfflineEvidenceOnly,
}

impl DrillBlocker {
    /// Every blocker, in declaration order.
    pub const ALL: [Self; 6] = [
        Self::NoneCleared,
        Self::MissingTarget,
        Self::TrustBlock,
        Self::RouteUnavailable,
        Self::ExpiredSnapshot,
        Self::ImportedOfflineEvidenceOnly,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoneCleared => "none_cleared",
            Self::MissingTarget => "missing_target",
            Self::TrustBlock => "trust_block",
            Self::RouteUnavailable => "route_unavailable",
            Self::ExpiredSnapshot => "expired_snapshot",
            Self::ImportedOfflineEvidenceOnly => "imported_offline_evidence_only",
        }
    }

    /// The live-target handoff outcome this blocker requires, so a binding's blocker and outcome can never drift
    /// apart.
    pub const fn required_outcome(self) -> HandoffOutcome {
        match self {
            Self::NoneCleared => HandoffOutcome::HandoffCleared,
            Self::MissingTarget => HandoffOutcome::BlockedTargetUnavailable,
            Self::TrustBlock => HandoffOutcome::BlockedNeedsPrerequisite,
            Self::RouteUnavailable => HandoffOutcome::BlockedNeedsPrerequisite,
            Self::ExpiredSnapshot => HandoffOutcome::BlockedByPolicy,
            Self::ImportedOfflineEvidenceOnly => HandoffOutcome::BlockedTargetUnavailable,
        }
    }

    /// The equivalent reason in the live-target-handoff module's own [`HandoffBlockerReason`] vocabulary; `None`
    /// only for the cleared baseline. Every mapped reason is a member of its required outcome's allowed reasons.
    pub const fn maps_to_handoff_blocker_reason(self) -> Option<HandoffBlockerReason> {
        match self {
            Self::NoneCleared => None,
            Self::MissingTarget => Some(HandoffBlockerReason::TargetDoesNotExist),
            Self::TrustBlock => Some(HandoffBlockerReason::TrustPostureInsufficient),
            Self::RouteUnavailable => Some(HandoffBlockerReason::RouteUnavailable),
            Self::ExpiredSnapshot => Some(HandoffBlockerReason::PolicyOrLifecycleBlocked),
            Self::ImportedOfflineEvidenceOnly => {
                Some(HandoffBlockerReason::RetiredCapabilityNoLiveCounterpart)
            }
        }
    }
}

/// The action a drill surface may expose.
///
/// The set is deliberately closed and analysis-only apart from the single validated pivot: there is no apply /
/// sync / restore action, and `OpenCurrentLiveObject` appears only when the drill clears the handoff, so a drill
/// surface can never reopen live state from an unvalidated or blocked snapshot.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillCorpusAction {
    /// Inspect the seeded historical-reference fixture metadata-only.
    InspectHistoricalFixture,
    /// Export the drill evidence record.
    ExportDrillEvidence,
    /// Open the current live object — only when the drill clears the handoff.
    OpenCurrentLiveObject,
}

impl DrillCorpusAction {
    /// The analysis-only base action set present on every drill surface.
    pub const BASE: [Self; 2] = [Self::InspectHistoricalFixture, Self::ExportDrillEvidence];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectHistoricalFixture => "inspect_historical_fixture",
            Self::ExportDrillEvidence => "export_drill_evidence",
            Self::OpenCurrentLiveObject => "open_current_live_object",
        }
    }
}

/// Whether a binding joins a live target or discloses a non-live boundary only.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillParity {
    /// The handoff cleared and an open-current-live-object action is offered.
    LiveTargetJoined,
    /// The non-live boundary is explicitly disclosed with a satisfy-prerequisite or metadata-only exit.
    NonLiveBoundaryDisclosed,
}

impl DrillParity {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTargetJoined => "live_target_joined",
            Self::NonLiveBoundaryDisclosed => "non_live_boundary_disclosed",
        }
    }
}

/// Downgrade trigger that can narrow this drill-corpus lane below its claimed coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoricalEvidenceDrillDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Non-live grammar drifted between surfaces for the same fixture.
    GrammarDriftDetected,
    /// A drill dropped its mutation-blocked posture and began to imply the evidence is live.
    MutationBlockedPostureDropped,
    /// A drill's exact blocker drifted apart from its live-target handoff outcome.
    BlockerOutcomeMismatch,
    /// A drill dead-linked an expired or removed artifact instead of rendering metadata.
    ExpiredArtifactDeadLinked,
    /// A drill reopened a live target without validating identity, trust, route, and authority.
    ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
    /// A drill presented imported / offline evidence as current live truth.
    PresentsNonLiveEvidenceAsCurrentLive,
    /// A binding lost its capture-context join.
    NonLiveEvidenceUnjoinedToCaptureContext,
    /// A binding lost its corpus evidence bindings (screenshots, accessibility, CLI export, dashboard).
    CorpusEvidenceBindingsMissing,
    /// An accessibility route for the non-live boundary, provenance, or handoff expectation was dropped.
    AccessibilityRouteDropped,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream historical-reference contract narrowed.
    UpstreamHistoricalReferenceNarrowed,
}

impl HistoricalEvidenceDrillDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 13] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::GrammarDriftDetected,
        Self::MutationBlockedPostureDropped,
        Self::BlockerOutcomeMismatch,
        Self::ExpiredArtifactDeadLinked,
        Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
        Self::PresentsNonLiveEvidenceAsCurrentLive,
        Self::NonLiveEvidenceUnjoinedToCaptureContext,
        Self::CorpusEvidenceBindingsMissing,
        Self::AccessibilityRouteDropped,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamHistoricalReferenceNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::GrammarDriftDetected => "grammar_drift_detected",
            Self::MutationBlockedPostureDropped => "mutation_blocked_posture_dropped",
            Self::BlockerOutcomeMismatch => "blocker_outcome_mismatch",
            Self::ExpiredArtifactDeadLinked => "expired_artifact_dead_linked",
            Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority => {
                "reopens_live_target_without_validating_identity_trust_route_and_authority"
            }
            Self::PresentsNonLiveEvidenceAsCurrentLive => {
                "presents_non_live_evidence_as_current_live"
            }
            Self::NonLiveEvidenceUnjoinedToCaptureContext => {
                "non_live_evidence_unjoined_to_capture_context"
            }
            Self::CorpusEvidenceBindingsMissing => "corpus_evidence_bindings_missing",
            Self::AccessibilityRouteDropped => "accessibility_route_dropped",
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamHistoricalReferenceNarrowed => "upstream_historical_reference_narrowed",
        }
    }
}

/// The controlled non-live grammar a historical-reference fixture presents.
///
/// These five words describe the non-live (historical) side of a fixture and must be identical across every drill
/// that renders the same fixture. The historical-role word must be a frozen [`M5HistoricalReferenceRole`] token;
/// the rest are controlled words the fixture carries so it stays attributable to its capture context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalEvidenceGrammar {
    /// Historical-role word (must be a frozen [`M5HistoricalReferenceRole`] token).
    pub historical_role_word: String,
    /// The captured-evidence / archived-snapshot label word.
    pub snapshot_label_word: String,
    /// The capture-time word the evidence is attributed to.
    pub capture_time_word: String,
    /// The provenance / capture-context word the evidence is attributed to.
    pub provenance_word: String,
    /// The mutation-blocked-posture word (read-only, non-authoritative-for-mutation).
    pub mutation_blocked_posture_word: String,
}

impl HistoricalEvidenceGrammar {
    /// Whether every grammar word is present.
    pub fn all_present(&self) -> bool {
        !self.historical_role_word.trim().is_empty()
            && !self.snapshot_label_word.trim().is_empty()
            && !self.capture_time_word.trim().is_empty()
            && !self.provenance_word.trim().is_empty()
            && !self.mutation_blocked_posture_word.trim().is_empty()
    }

    /// Whether the historical-role word is a member of the frozen role vocabulary.
    pub fn historical_role_word_in_vocabulary(&self) -> bool {
        is_known_historical_reference_role_token(self.historical_role_word.trim())
    }

    /// Whether the capture-time and provenance words that keep the evidence from dead-linking are both present.
    pub fn capture_context_present(&self) -> bool {
        !self.capture_time_word.trim().is_empty() && !self.provenance_word.trim().is_empty()
    }

    /// Whether the profile honours the mutation-blocked rule: a historical-side role that must be present before
    /// the object may be surfaced as non-live evidence must pair it with a real mutation-blocked posture word and
    /// never collapse to an editable / live / writable / current-object sentinel.
    pub fn mutation_blocked_posture_satisfied(&self) -> bool {
        match historical_reference_role_from_token(self.historical_role_word.trim()) {
            Some(role) if role.must_be_present_before_surfacing_as_non_live_evidence() => {
                let posture = self.mutation_blocked_posture_word.trim().to_lowercase();
                !posture.is_empty()
                    && !MUTATION_BLOCKED_POSTURE_ABSENT_SENTINELS.contains(&posture.as_str())
            }
            _ => true,
        }
    }
}

/// The join that keeps a seeded fixture attributable to its capture context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ProvenanceJoin {
    /// Stable id / ref of the source snapshot / archived descriptor.
    pub source_snapshot_descriptor_ref: String,
    /// Stable id / ref of the source capture context.
    pub capture_context_ref: String,
    /// Stable id / ref of the producer / build that captured the evidence.
    pub producer_build_ref: String,
    /// Stable id / ref of the provenance lineage chain.
    pub provenance_lineage_ref: String,
}

impl ProvenanceJoin {
    /// Whether every join ref is present, so the fixture is fully attributable.
    pub fn all_present(&self) -> bool {
        !self.source_snapshot_descriptor_ref.trim().is_empty()
            && !self.capture_context_ref.trim().is_empty()
            && !self.producer_build_ref.trim().is_empty()
            && !self.provenance_lineage_ref.trim().is_empty()
    }
}

/// The handoff expectation a drill records: its outcome, exact blocker, and the controlled exit refs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HandoffExpectation {
    /// The live-target handoff outcome this drill expects.
    pub expected_outcome: HandoffOutcome,
    /// The exact blocker this drill expects (`none_cleared` when the handoff clears).
    pub expected_blocker: DrillBlocker,
    /// The validated live-target handoff packet ref, present only when the handoff clears.
    pub live_target_handoff_ref: Option<String>,
    /// The metadata-only exit ref, present when the target is unavailable or policy-blocked.
    pub metadata_only_exit_ref: Option<String>,
    /// The satisfy-prerequisite-then-retry ref, present when a route / trust prerequisite is unmet.
    pub satisfy_prerequisite_ref: Option<String>,
    /// The explicit blocker / handoff note (never omitted); names the exact blocker in plain words.
    pub blocker_note: String,
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
    /// The historical-reference state the drill exercises.
    pub expected_state: HistoricalReferenceDrillState,
    /// The live-target handoff outcome the drill expects.
    pub expected_handoff_outcome: HandoffOutcome,
    /// The exact blocker the drill names.
    pub expected_blocker: DrillBlocker,
    /// The parity state the drill requires.
    pub parity: DrillParity,
    /// Whether the drill offers an open-current-live-object action.
    pub offers_open_live_target: bool,
    /// Whether the handoff expectation must carry a live-target handoff ref.
    pub requires_live_target_handoff_ref: bool,
    /// Whether the handoff expectation must carry a metadata-only exit ref.
    pub requires_metadata_only_exit_ref: bool,
    /// Whether the handoff expectation must carry a satisfy-prerequisite ref.
    pub requires_satisfy_prerequisite_ref: bool,
    /// Whether the seeded fixture's content is available in this drill.
    pub expects_content_available: bool,
}

/// Resolves the disclosures a drill binding must carry from its scenario.
///
/// The preserved baseline clears the handoff, offers an open-current-live-object action, and keeps its content
/// available. Every blocked drill narrows the actions, names an exact blocker, and either offers a
/// satisfy-prerequisite fallback (route / trust prerequisite unmet) or a metadata-only exit (target unavailable
/// or policy-blocked). The expired-snapshot drill additionally drops its content and still renders metadata
/// instead of a dead link. All keep the non-live grammar and join back to a source snapshot descriptor.
pub const fn resolve_drill_disclosure(drill: DrillScenario) -> DrillDisclosure {
    match drill {
        DrillScenario::PreservedLiveTargetHandoff => DrillDisclosure {
            expected_state: HistoricalReferenceDrillState::PreservedLiveTargetJoinable,
            expected_handoff_outcome: HandoffOutcome::HandoffCleared,
            expected_blocker: DrillBlocker::NoneCleared,
            parity: DrillParity::LiveTargetJoined,
            offers_open_live_target: true,
            requires_live_target_handoff_ref: true,
            requires_metadata_only_exit_ref: false,
            requires_satisfy_prerequisite_ref: false,
            expects_content_available: true,
        },
        DrillScenario::MissingLiveTarget => DrillDisclosure {
            expected_state: HistoricalReferenceDrillState::MissingLiveTargetMetadataOnly,
            expected_handoff_outcome: HandoffOutcome::BlockedTargetUnavailable,
            expected_blocker: DrillBlocker::MissingTarget,
            parity: DrillParity::NonLiveBoundaryDisclosed,
            offers_open_live_target: false,
            requires_live_target_handoff_ref: false,
            requires_metadata_only_exit_ref: true,
            requires_satisfy_prerequisite_ref: false,
            expects_content_available: true,
        },
        DrillScenario::RetiredLineReopen => DrillDisclosure {
            expected_state: HistoricalReferenceDrillState::RetiredLineNoLiveCounterpart,
            expected_handoff_outcome: HandoffOutcome::BlockedNeedsPrerequisite,
            expected_blocker: DrillBlocker::RouteUnavailable,
            parity: DrillParity::NonLiveBoundaryDisclosed,
            offers_open_live_target: false,
            requires_live_target_handoff_ref: false,
            requires_metadata_only_exit_ref: false,
            requires_satisfy_prerequisite_ref: true,
            expects_content_available: true,
        },
        DrillScenario::StaleImportedEvidence => DrillDisclosure {
            expected_state: HistoricalReferenceDrillState::StaleImportedEvidence,
            expected_handoff_outcome: HandoffOutcome::BlockedNeedsPrerequisite,
            expected_blocker: DrillBlocker::TrustBlock,
            parity: DrillParity::NonLiveBoundaryDisclosed,
            offers_open_live_target: false,
            requires_live_target_handoff_ref: false,
            requires_metadata_only_exit_ref: false,
            requires_satisfy_prerequisite_ref: true,
            expects_content_available: true,
        },
        DrillScenario::ExpiredSnapshotMetadataOnlyFallback => DrillDisclosure {
            expected_state: HistoricalReferenceDrillState::ExpiredSnapshotMetadataFallback,
            expected_handoff_outcome: HandoffOutcome::BlockedByPolicy,
            expected_blocker: DrillBlocker::ExpiredSnapshot,
            parity: DrillParity::NonLiveBoundaryDisclosed,
            offers_open_live_target: false,
            requires_live_target_handoff_ref: false,
            requires_metadata_only_exit_ref: true,
            requires_satisfy_prerequisite_ref: false,
            expects_content_available: false,
        },
        DrillScenario::EvidenceOnlyReopenAfterVersionSchemaDrift => DrillDisclosure {
            expected_state: HistoricalReferenceDrillState::ImportedOfflineEvidenceOnly,
            expected_handoff_outcome: HandoffOutcome::BlockedTargetUnavailable,
            expected_blocker: DrillBlocker::ImportedOfflineEvidenceOnly,
            parity: DrillParity::NonLiveBoundaryDisclosed,
            offers_open_live_target: false,
            requires_live_target_handoff_ref: false,
            requires_metadata_only_exit_ref: true,
            requires_satisfy_prerequisite_ref: false,
            expects_content_available: true,
        },
    }
}

/// One drill binding: a seeded historical-reference fixture exercised by one drill on one consumer surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillCorpusBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable seeded-fixture id (shared across surfaces that show the same fixture).
    pub fixture_id: String,
    /// Human-readable seeded-fixture identity.
    pub fixture_label: String,
    /// Which preserved-object class this fixture belongs to.
    pub object_class: M5HistoricalReferenceObject,
    /// Which consumer surface renders it.
    pub consumer: M5HistoricalReferenceConsumerSurface,
    /// The drill exercised on this fixture.
    pub drill: DrillScenario,
    /// A stable, human-facing drill label.
    pub drill_label: String,
    /// The historical-reference state this drill exercises.
    pub evidence_state: HistoricalReferenceDrillState,
    /// The live-target handoff outcome this drill expects.
    pub expected_handoff_outcome: HandoffOutcome,
    /// The exact blocker this drill names.
    pub expected_blocker: DrillBlocker,
    /// The controlled non-live grammar presented (identical across surfaces for one fixture).
    pub non_live_grammar: HistoricalEvidenceGrammar,
    /// Whether the seeded fixture's content is available.
    pub content_available: bool,
    /// Whether a live target is joined or a non-live boundary is disclosed.
    pub parity_state: DrillParity,
    /// The discoverable action set allowed on this drill surface.
    pub allowed_actions: Vec<DrillCorpusAction>,
    /// The accessibility routes through which the non-live boundary, provenance, and handoff expectation can be
    /// discovered without pointer-only chrome.
    pub accessibility_routes: Vec<M5HistoricalReferenceAccessibilityRoute>,
    /// The provenance join keeping this fixture attributable to its capture context.
    pub provenance_join: ProvenanceJoin,
    /// The handoff expectation this drill records.
    pub handoff_expectation: HandoffExpectation,
    /// The refs binding this drill back to release / support evidence.
    pub corpus_evidence: CorpusEvidenceBindings,
    /// The non-live boundary is explicitly called out. MUST be `true`.
    pub non_live_boundary_explicitly_called_out: bool,
    /// Guardrail: this drill lets non-live evidence look live by omission. MUST be `false`.
    pub looks_live_by_omission: bool,
    /// Guardrail: this drill reopens a live target without validating identity, trust, route, and authority.
    /// MUST be `false`.
    pub reopens_live_target_without_validating_identity_trust_route_and_authority: bool,
    /// Guardrail: this drill dead-links an expired or removed artifact instead of rendering metadata. MUST be
    /// `false`.
    pub dead_links_expired_or_removed_artifact: bool,
    /// Guardrail: this drill's non-live evidence is unjoined to its capture context. MUST be `false`.
    pub non_live_evidence_unjoined_to_capture_context: bool,
    /// Guardrail: this drill presents non-live evidence as current or reopens through an ambiguous route. MUST be
    /// `false`.
    pub presents_as_current_or_reopens_through_ambiguous_route: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl DrillCorpusBinding {
    /// Disclosures this binding must carry, derived from its drill scenario.
    pub const fn disclosure(&self) -> DrillDisclosure {
        resolve_drill_disclosure(self.drill)
    }

    /// Whether this drill clears the live-target handoff.
    pub const fn is_cleared_handoff(&self) -> bool {
        self.drill.clears_handoff()
    }

    /// Whether the expected blocker's required outcome matches the expected handoff outcome.
    pub fn blocker_matches_outcome(&self) -> bool {
        self.expected_blocker.required_outcome() == self.expected_handoff_outcome
    }

    /// Whether every guardrail row-invariant holds (non-live boundary called out, all guardrails false).
    pub const fn guardrails_hold(&self) -> bool {
        self.non_live_boundary_explicitly_called_out
            && !self.looks_live_by_omission
            && !self.reopens_live_target_without_validating_identity_trust_route_and_authority
            && !self.dead_links_expired_or_removed_artifact
            && !self.non_live_evidence_unjoined_to_capture_context
            && !self.presents_as_current_or_reopens_through_ambiguous_route
    }

    /// Whether the analysis-only base action set is present.
    pub fn has_base_actions(&self) -> bool {
        DrillCorpusAction::BASE
            .iter()
            .all(|action| self.allowed_actions.contains(action))
    }

    /// Whether no apply / sync / restore affordance leaked in (structurally guaranteed by the closed action enum,
    /// but checked so the invariant is explicit).
    pub fn action_set_is_closed(&self) -> bool {
        self.allowed_actions.iter().all(|action| {
            matches!(
                action,
                DrillCorpusAction::InspectHistoricalFixture
                    | DrillCorpusAction::ExportDrillEvidence
                    | DrillCorpusAction::OpenCurrentLiveObject
            )
        })
    }

    /// Whether the open-current-live-object action is present exactly when the drill clears the handoff.
    pub fn open_live_action_matches_drill(&self) -> bool {
        let offered = self.disclosure().offers_open_live_target;
        let present = self
            .allowed_actions
            .contains(&DrillCorpusAction::OpenCurrentLiveObject);
        offered == present
    }

    /// Whether the content-available flag matches what the drill expects.
    pub fn content_presence_matches_drill(&self) -> bool {
        self.content_available == self.disclosure().expects_content_available
    }

    /// Whether, when the content is unavailable, the binding still renders capture time, provenance, and a
    /// blocker note instead of degrading to a dead link.
    pub fn renders_metadata_instead_of_dead_link(&self) -> bool {
        if self.content_available {
            return true;
        }
        self.non_live_grammar.capture_context_present()
            && !self.handoff_expectation.blocker_note.trim().is_empty()
            && !self.dead_links_expired_or_removed_artifact
    }

    /// Whether keyboard focus and screen-reader announcement are both discoverable.
    pub fn accessibility_state_discoverable(&self) -> bool {
        self.accessibility_routes
            .contains(&M5HistoricalReferenceAccessibilityRoute::KeyboardFocusable)
            && self
                .accessibility_routes
                .contains(&M5HistoricalReferenceAccessibilityRoute::ScreenReaderAnnounced)
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
                .any(|reference| reference == M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF)
    }
}

/// Trust and provenance review block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillCorpusTrustReview {
    /// The seeded fixtures exercise four or more distinct historical-reference states.
    pub fixtures_seed_four_or_more_historical_reference_states: bool,
    /// The seeded fixtures exercise two or more distinct live-target handoff outcomes.
    pub fixtures_seed_two_or_more_live_target_handoff_outcomes: bool,
    /// The exact blockers are mechanically distinguishable.
    pub exact_blockers_are_distinguishable: bool,
    /// Every object class is seeded by two or more consumers.
    pub every_object_class_seeded_by_two_or_more_consumers: bool,
    /// The same fixture presents the same non-live grammar across surfaces.
    pub non_live_grammar_identical_for_same_fixture: bool,
    /// Every historical-role word is a frozen role token.
    pub historical_role_words_stay_in_frozen_vocabulary: bool,
    /// Capture time and provenance are present on every binding.
    pub capture_context_present_on_every_binding: bool,
    /// Metadata, provenance, and a blocker note render instead of a dead link when content is gone.
    pub metadata_provenance_and_boundary_render_instead_of_dead_link: bool,
    /// A retired-line reopen is refused rather than silently reopened.
    pub retired_line_reopen_is_refused_not_silently_reopened: bool,
    /// A missing target falls back to a metadata-only exit.
    pub missing_target_falls_back_to_metadata_only_exit: bool,
    /// A stale import blocks on a trust prerequisite.
    pub stale_import_blocks_on_trust_prerequisite: bool,
    /// An expired snapshot shows metadata rather than a dead link.
    pub expired_snapshot_shows_metadata_not_dead_link: bool,
    /// An evidence-only reopen after drift stays non-live.
    pub evidence_only_reopen_after_drift_stays_non_live: bool,
    /// An open-current-live-object action is offered only when the handoff clears.
    pub open_live_offered_only_when_handoff_clears: bool,
    /// The corpus is bound to screenshots, accessibility checks, the CLI / support export, and dashboards.
    pub corpus_bound_to_screenshots_accessibility_cli_and_dashboards: bool,
    /// The corpus is referenced by release evidence and support drills, not an ad hoc sample set.
    pub corpus_referenced_by_release_and_support_not_ad_hoc: bool,
    /// Accessibility routes for the non-live boundary, provenance, and handoff expectation are present.
    pub accessibility_routes_present_for_boundary_provenance_and_join: bool,
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
        self.fixtures_seed_four_or_more_historical_reference_states
            && self.fixtures_seed_two_or_more_live_target_handoff_outcomes
            && self.exact_blockers_are_distinguishable
            && self.every_object_class_seeded_by_two_or_more_consumers
            && self.non_live_grammar_identical_for_same_fixture
            && self.historical_role_words_stay_in_frozen_vocabulary
            && self.capture_context_present_on_every_binding
            && self.metadata_provenance_and_boundary_render_instead_of_dead_link
            && self.retired_line_reopen_is_refused_not_silently_reopened
            && self.missing_target_falls_back_to_metadata_only_exit
            && self.stale_import_blocks_on_trust_prerequisite
            && self.expired_snapshot_shows_metadata_not_dead_link
            && self.evidence_only_reopen_after_drift_stays_non_live
            && self.open_live_offered_only_when_handoff_clears
            && self.corpus_bound_to_screenshots_accessibility_cli_and_dashboards
            && self.corpus_referenced_by_release_and_support_not_ad_hoc
            && self.accessibility_routes_present_for_boundary_provenance_and_join
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillCorpusProjection {
    /// The shell / archive-viewer surface consumes the corpus.
    pub shell_consumes_corpus: bool,
    /// The help / docs surface consumes the corpus.
    pub help_docs_consumes_corpus: bool,
    /// The support surface consumes the corpus.
    pub support_consumes_corpus: bool,
    /// The review / incident surface consumes the corpus.
    pub review_incident_consumes_corpus: bool,
    /// The runbook-archive surface consumes the corpus.
    pub runbook_archive_consumes_corpus: bool,
    /// The release-center surface consumes the corpus.
    pub release_center_consumes_corpus: bool,
    /// The companion / export surface consumes the corpus.
    pub companion_export_consumes_corpus: bool,
    /// The program-governance surface consumes the corpus.
    pub program_governance_consumes_corpus: bool,
    /// The CLI / export path consumes the corpus.
    pub cli_export_consumes_corpus: bool,
    /// Every object class is stated by two or more consumers.
    pub every_object_class_stated_by_two_or_more_consumers: bool,
    /// Non-live grammar is identical for the same fixture.
    pub non_live_grammar_identical_for_same_fixture: bool,
    /// The non-live boundary is disclosed rather than hidden.
    pub non_live_boundary_disclosed_not_hidden: bool,
    /// Export maps a drill row back to one historical-reference object class.
    pub drill_maps_back_to_one_historical_reference_object: bool,
}

impl DrillCorpusProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.shell_consumes_corpus
            && self.help_docs_consumes_corpus
            && self.support_consumes_corpus
            && self.review_incident_consumes_corpus
            && self.runbook_archive_consumes_corpus
            && self.release_center_consumes_corpus
            && self.companion_export_consumes_corpus
            && self.program_governance_consumes_corpus
            && self.cli_export_consumes_corpus
            && self.every_object_class_stated_by_two_or_more_consumers
            && self.non_live_grammar_identical_for_same_fixture
            && self.non_live_boundary_disclosed_not_hidden
            && self.drill_maps_back_to_one_historical_reference_object
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

/// Constructor input for [`M5HistoricalEvidenceDrillCorpusPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5HistoricalEvidenceDrillCorpusPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Drill bindings.
    pub drill_bindings: Vec<DrillCorpusBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<HistoricalEvidenceDrillDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5HistoricalReferenceConsumerSurface>,
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

/// Export-safe historical-evidence drill-corpus packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoricalEvidenceDrillCorpusPacket {
    /// Record kind; must equal [`M5_HISTORICAL_EVIDENCE_DRILL_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_HISTORICAL_EVIDENCE_DRILL_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Drill bindings.
    pub drill_bindings: Vec<DrillCorpusBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<HistoricalEvidenceDrillDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5HistoricalReferenceConsumerSurface>,
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

impl M5HistoricalEvidenceDrillCorpusPacket {
    /// Builds a drill-corpus packet from stable-lane input.
    pub fn new(input: M5HistoricalEvidenceDrillCorpusPacketInput) -> Self {
        Self {
            record_kind: M5_HISTORICAL_EVIDENCE_DRILL_RECORD_KIND.to_owned(),
            schema_version: M5_HISTORICAL_EVIDENCE_DRILL_SCHEMA_VERSION,
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
    pub fn validate(&self) -> Vec<M5HistoricalEvidenceDrillViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_HISTORICAL_EVIDENCE_DRILL_RECORD_KIND {
            violations.push(M5HistoricalEvidenceDrillViolation::WrongRecordKind);
        }
        if self.schema_version != M5_HISTORICAL_EVIDENCE_DRILL_SCHEMA_VERSION {
            violations.push(M5HistoricalEvidenceDrillViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5HistoricalEvidenceDrillViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5HistoricalEvidenceDrillViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5HistoricalEvidenceDrillViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5HistoricalEvidenceDrillViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(M5HistoricalEvidenceDrillViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5HistoricalEvidenceDrillViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("drill-corpus packet serializes"),
        ) {
            violations.push(M5HistoricalEvidenceDrillViolation::RawBoundaryMaterialInExport);
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
            "object_class,consumer,drill,evidence_state,handoff_outcome,expected_blocker,content_available,fixture_id\n",
        );
        for binding in &self.drill_bindings {
            out.push_str(&format!(
                "{},{},{},{},{},{},{},{}\n",
                binding.object_class.as_str(),
                binding.consumer.as_str(),
                binding.drill.as_str(),
                binding.evidence_state.as_str(),
                binding.expected_handoff_outcome.as_str(),
                binding.expected_blocker.as_str(),
                binding.content_available,
                binding.fixture_id.replace(',', ";"),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let cleared = self
            .drill_bindings
            .iter()
            .filter(|binding| binding.is_cleared_handoff())
            .count();

        let mut out = String::new();
        out.push_str("# Historical-Evidence Drill Corpus: Fixtures and Regression Drills\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Drill bindings: {} ({} clear the live-target handoff)\n",
            self.drill_bindings.len(),
            cleared
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Drill bindings\n\n");
        for binding in &self.drill_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: object `{}` on `{}`, drill `{}`, state `{}`, outcome `{}`, blocker `{}`, content-available `{}`\n",
                binding.fixture_label,
                binding.binding_id,
                binding.object_class.as_str(),
                binding.consumer.as_str(),
                binding.drill.as_str(),
                binding.evidence_state.as_str(),
                binding.expected_handoff_outcome.as_str(),
                binding.expected_blocker.as_str(),
                binding.content_available,
            ));
        }
        out
    }

    /// Deterministic health dashboard JSON, minted from truth, so release / support can surface this corpus.
    pub fn render_health_dashboard(&self) -> String {
        let dashboard = DrillHealthDashboard {
            record_kind: M5_HISTORICAL_EVIDENCE_DRILL_DASHBOARD_RECORD_KIND,
            packet_id: &self.packet_id,
            support_export_ref: M5_HISTORICAL_EVIDENCE_DRILL_ARTIFACT_REF,
            corpus_schema_ref: M5_HISTORICAL_EVIDENCE_DRILL_SCHEMA_REF,
            matrix_schema_ref: M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
            drills: DrillScenario::ALL.iter().map(|d| d.as_str()).collect(),
            historical_reference_states: HistoricalReferenceDrillState::ALL
                .iter()
                .map(|s| s.as_str())
                .collect(),
            handoff_outcomes: HandoffOutcome::ALL.iter().map(|o| o.as_str()).collect(),
            exact_blockers: DrillBlocker::ALL.iter().map(|b| b.as_str()).collect(),
            fixture_families: M5HistoricalReferenceObject::ALL
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

#[derive(Serialize)]
struct DrillHealthDashboard<'a> {
    record_kind: &'a str,
    packet_id: &'a str,
    support_export_ref: &'a str,
    corpus_schema_ref: &'a str,
    matrix_schema_ref: &'a str,
    drills: Vec<&'a str>,
    historical_reference_states: Vec<&'a str>,
    handoff_outcomes: Vec<&'a str>,
    exact_blockers: Vec<&'a str>,
    fixture_families: Vec<DrillFixtureFamily<'a>>,
}

#[derive(Serialize)]
struct DrillFixtureFamily<'a> {
    object_class: &'a str,
    canonical_schema: &'a str,
}

/// Errors emitted when reading the checked-in drill-corpus export.
#[derive(Debug)]
pub enum M5HistoricalEvidenceDrillArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5HistoricalEvidenceDrillViolation>),
}

impl fmt::Display for M5HistoricalEvidenceDrillArtifactError {
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

impl Error for M5HistoricalEvidenceDrillArtifactError {}

/// Validation failures emitted by [`M5HistoricalEvidenceDrillCorpusPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5HistoricalEvidenceDrillViolation {
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
    /// A binding's non-live grammar values are incomplete.
    GrammarFacetIncomplete,
    /// A binding's historical-role word is not a frozen role token.
    HistoricalRoleWordOutsideVocabulary,
    /// A binding's gate-role dropped its mutation-blocked posture.
    MutationBlockedPostureMissingForGateRole,
    /// A binding's evidence state does not match its drill.
    EvidenceStateMismatch,
    /// A binding's handoff outcome does not match its drill.
    HandoffOutcomeMismatch,
    /// A binding's exact blocker does not match its drill.
    BlockerMismatch,
    /// A binding's exact blocker does not match its handoff outcome.
    BlockerOutcomeMismatch,
    /// A binding's parity state does not match its drill.
    ParityStateMismatch,
    /// A binding's content-available flag does not match its drill.
    ContentPresenceMismatch,
    /// Two surfaces show the same fixture with different non-live grammar.
    GrammarDriftAcrossSurfaces,
    /// A shared object class is not seeded by at least two distinct consumers.
    ObjectClassReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A binding is missing a stable drill label.
    DrillLabelMissing,
    /// A binding is missing its source snapshot descriptor join.
    SourceDescriptorJoinMissing,
    /// A binding's provenance join is incomplete.
    ProvenanceJoinIncomplete,
    /// A binding's live-target handoff ref presence does not match its drill.
    LiveTargetHandoffRefMismatch,
    /// A binding's metadata-only exit ref presence does not match its drill.
    MetadataOnlyExitRefMismatch,
    /// A binding's satisfy-prerequisite ref presence does not match its drill.
    SatisfyPrerequisiteRefMismatch,
    /// A binding is missing its blocker note.
    BlockerNoteMissing,
    /// A binding is missing its corpus evidence bindings.
    CorpusEvidenceBindingsMissing,
    /// A binding is missing the analysis-only base action set.
    BaseActionsMissing,
    /// A binding's action set is not the closed drill action set.
    ActionSetNotClosed,
    /// A binding's open-current-live-object action does not match its drill.
    OpenLiveActionDrillMismatch,
    /// A binding whose content is gone degrades to a generic dead link.
    MetadataFallbackMissing,
    /// A binding cannot discover its non-live boundary via keyboard focus and screen-reader announcement.
    AccessibilityStateUndiscoverable,
    /// A binding's non-live side is not mutation blocked / boundary not called out.
    NonLiveBoundaryNotCalledOut,
    /// A binding lets non-live evidence look live by omission.
    LooksLiveByOmission,
    /// A binding reopens a live target without validating identity, trust, route, and authority.
    ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
    /// A binding dead-links an expired or removed artifact.
    DeadLinksExpiredOrRemovedArtifact,
    /// A binding's non-live evidence is unjoined to its capture context.
    NonLiveEvidenceUnjoinedToCaptureContext,
    /// A binding presents non-live evidence as current or reopens through an ambiguous route.
    PresentsAsCurrentOrReopensThroughAmbiguousRoute,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared object class appears among the bindings.
    ObjectClassCoverageMissing,
    /// Not every drill appears among the bindings.
    DrillCoverageMissing,
    /// Fewer than four distinct historical-reference states appear.
    HistoricalReferenceStateCoverageInsufficient,
    /// Fewer than two distinct live-target handoff outcomes appear.
    HandoffOutcomeCoverageInsufficient,
    /// Not every exact blocker appears among the bindings.
    ExactBlockerCoverageMissing,
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

impl M5HistoricalEvidenceDrillViolation {
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
            Self::HistoricalRoleWordOutsideVocabulary => "historical_role_word_outside_vocabulary",
            Self::MutationBlockedPostureMissingForGateRole => {
                "mutation_blocked_posture_missing_for_gate_role"
            }
            Self::EvidenceStateMismatch => "evidence_state_mismatch",
            Self::HandoffOutcomeMismatch => "handoff_outcome_mismatch",
            Self::BlockerMismatch => "blocker_mismatch",
            Self::BlockerOutcomeMismatch => "blocker_outcome_mismatch",
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::ContentPresenceMismatch => "content_presence_mismatch",
            Self::GrammarDriftAcrossSurfaces => "grammar_drift_across_surfaces",
            Self::ObjectClassReuseUnproven => "object_class_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::DrillLabelMissing => "drill_label_missing",
            Self::SourceDescriptorJoinMissing => "source_descriptor_join_missing",
            Self::ProvenanceJoinIncomplete => "provenance_join_incomplete",
            Self::LiveTargetHandoffRefMismatch => "live_target_handoff_ref_mismatch",
            Self::MetadataOnlyExitRefMismatch => "metadata_only_exit_ref_mismatch",
            Self::SatisfyPrerequisiteRefMismatch => "satisfy_prerequisite_ref_mismatch",
            Self::BlockerNoteMissing => "blocker_note_missing",
            Self::CorpusEvidenceBindingsMissing => "corpus_evidence_bindings_missing",
            Self::BaseActionsMissing => "base_actions_missing",
            Self::ActionSetNotClosed => "action_set_not_closed",
            Self::OpenLiveActionDrillMismatch => "open_live_action_drill_mismatch",
            Self::MetadataFallbackMissing => "metadata_fallback_missing",
            Self::AccessibilityStateUndiscoverable => "accessibility_state_undiscoverable",
            Self::NonLiveBoundaryNotCalledOut => "non_live_boundary_not_called_out",
            Self::LooksLiveByOmission => "looks_live_by_omission",
            Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority => {
                "reopens_live_target_without_validating_identity_trust_route_and_authority"
            }
            Self::DeadLinksExpiredOrRemovedArtifact => "dead_links_expired_or_removed_artifact",
            Self::NonLiveEvidenceUnjoinedToCaptureContext => {
                "non_live_evidence_unjoined_to_capture_context"
            }
            Self::PresentsAsCurrentOrReopensThroughAmbiguousRoute => {
                "presents_as_current_or_reopens_through_ambiguous_route"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ObjectClassCoverageMissing => "object_class_coverage_missing",
            Self::DrillCoverageMissing => "drill_coverage_missing",
            Self::HistoricalReferenceStateCoverageInsufficient => {
                "historical_reference_state_coverage_insufficient"
            }
            Self::HandoffOutcomeCoverageInsufficient => "handoff_outcome_coverage_insufficient",
            Self::ExactBlockerCoverageMissing => "exact_blocker_coverage_missing",
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
pub fn current_stable_m5_historical_evidence_drill_corpus_export(
) -> Result<M5HistoricalEvidenceDrillCorpusPacket, M5HistoricalEvidenceDrillArtifactError> {
    let packet: M5HistoricalEvidenceDrillCorpusPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/support/m5-historical-evidence-drills/support_export.json"
        )))
        .map_err(M5HistoricalEvidenceDrillArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5HistoricalEvidenceDrillArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5HistoricalEvidenceDrillCorpusPacket,
    violations: &mut Vec<M5HistoricalEvidenceDrillViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_HISTORICAL_EVIDENCE_DRILL_SCHEMA_REF,
        M5_HISTORICAL_EVIDENCE_DRILL_DOC_REF,
        M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
        M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF,
    ];
    // The five object classes map to three canonical domain schemas; require every distinct one.
    let mut domains: BTreeSet<&str> = BTreeSet::new();
    for object_class in M5HistoricalReferenceObject::ALL {
        domains.insert(object_class.canonical_domain_schema_ref());
    }
    required.extend(domains);
    for reference in required {
        if !refs.contains(reference) {
            violations.push(M5HistoricalEvidenceDrillViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5HistoricalEvidenceDrillCorpusPacket,
    violations: &mut Vec<M5HistoricalEvidenceDrillViolation>,
) {
    if packet.drill_bindings.is_empty() {
        violations.push(M5HistoricalEvidenceDrillViolation::DrillBindingsMissing);
        return;
    }

    // One vocabulary: the non-live grammar must be identical for every binding that renders the same fixture.
    let mut fixture_grammar: BTreeMap<&str, &HistoricalEvidenceGrammar> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object class must be seeded by at least two distinct consumers.
    let mut object_consumers: BTreeMap<
        M5HistoricalReferenceObject,
        BTreeSet<M5HistoricalReferenceConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5HistoricalReferenceConsumerSurface> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5HistoricalReferenceObject> = BTreeSet::new();
    let mut seen_drills: BTreeSet<DrillScenario> = BTreeSet::new();
    let mut seen_states: BTreeSet<HistoricalReferenceDrillState> = BTreeSet::new();
    let mut seen_outcomes: BTreeSet<HandoffOutcome> = BTreeSet::new();
    let mut seen_blockers: BTreeSet<DrillBlocker> = BTreeSet::new();

    for binding in &packet.drill_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.fixture_id.trim().is_empty()
            || binding.fixture_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5HistoricalEvidenceDrillViolation::BindingIncomplete);
        }
        if binding.drill_label.trim().is_empty() {
            violations.push(M5HistoricalEvidenceDrillViolation::DrillLabelMissing);
        }
        if !binding.non_live_grammar.all_present() {
            violations.push(M5HistoricalEvidenceDrillViolation::GrammarFacetIncomplete);
        }
        if !binding
            .non_live_grammar
            .historical_role_word_in_vocabulary()
        {
            violations
                .push(M5HistoricalEvidenceDrillViolation::HistoricalRoleWordOutsideVocabulary);
        }
        if !binding
            .non_live_grammar
            .mutation_blocked_posture_satisfied()
        {
            violations
                .push(M5HistoricalEvidenceDrillViolation::MutationBlockedPostureMissingForGateRole);
        }

        let disclosure = binding.disclosure();

        if binding.evidence_state != disclosure.expected_state {
            violations.push(M5HistoricalEvidenceDrillViolation::EvidenceStateMismatch);
        }
        if binding.expected_handoff_outcome != disclosure.expected_handoff_outcome {
            violations.push(M5HistoricalEvidenceDrillViolation::HandoffOutcomeMismatch);
        }
        if binding.expected_blocker != disclosure.expected_blocker {
            violations.push(M5HistoricalEvidenceDrillViolation::BlockerMismatch);
        }
        if !binding.blocker_matches_outcome() {
            violations.push(M5HistoricalEvidenceDrillViolation::BlockerOutcomeMismatch);
        }
        if binding.parity_state != disclosure.parity {
            violations.push(M5HistoricalEvidenceDrillViolation::ParityStateMismatch);
        }
        if !binding.content_presence_matches_drill() {
            violations.push(M5HistoricalEvidenceDrillViolation::ContentPresenceMismatch);
        }

        // Provenance join: always present, always joined back to a source snapshot descriptor.
        let provenance = &binding.provenance_join;
        if provenance.source_snapshot_descriptor_ref.trim().is_empty() {
            violations.push(M5HistoricalEvidenceDrillViolation::SourceDescriptorJoinMissing);
        }
        if !provenance.all_present() {
            violations.push(M5HistoricalEvidenceDrillViolation::ProvenanceJoinIncomplete);
        }

        // Handoff expectation refs match the drill disclosure.
        let expectation = &binding.handoff_expectation;
        if expectation.expected_outcome != binding.expected_handoff_outcome {
            violations.push(M5HistoricalEvidenceDrillViolation::HandoffOutcomeMismatch);
        }
        if expectation.expected_blocker != binding.expected_blocker {
            violations.push(M5HistoricalEvidenceDrillViolation::BlockerMismatch);
        }
        if expectation.live_target_handoff_ref.is_some()
            != disclosure.requires_live_target_handoff_ref
        {
            violations.push(M5HistoricalEvidenceDrillViolation::LiveTargetHandoffRefMismatch);
        }
        if expectation.metadata_only_exit_ref.is_some()
            != disclosure.requires_metadata_only_exit_ref
        {
            violations.push(M5HistoricalEvidenceDrillViolation::MetadataOnlyExitRefMismatch);
        }
        if expectation.satisfy_prerequisite_ref.is_some()
            != disclosure.requires_satisfy_prerequisite_ref
        {
            violations.push(M5HistoricalEvidenceDrillViolation::SatisfyPrerequisiteRefMismatch);
        }
        if expectation.blocker_note.trim().is_empty() {
            violations.push(M5HistoricalEvidenceDrillViolation::BlockerNoteMissing);
        }

        // Corpus evidence bindings (screenshots, accessibility, CLI export, dashboard).
        if !binding.corpus_evidence.all_present() {
            violations.push(M5HistoricalEvidenceDrillViolation::CorpusEvidenceBindingsMissing);
        }

        // Action rules.
        if !binding.has_base_actions() {
            violations.push(M5HistoricalEvidenceDrillViolation::BaseActionsMissing);
        }
        if !binding.action_set_is_closed() {
            violations.push(M5HistoricalEvidenceDrillViolation::ActionSetNotClosed);
        }
        if !binding.open_live_action_matches_drill() {
            violations.push(M5HistoricalEvidenceDrillViolation::OpenLiveActionDrillMismatch);
        }

        // Dead-link guardrail: never degrade to a generic dead link when metadata / provenance / boundary
        // can be shown.
        if !binding.renders_metadata_instead_of_dead_link() {
            violations.push(M5HistoricalEvidenceDrillViolation::MetadataFallbackMissing);
        }

        // Accessibility discovery.
        if !binding.accessibility_state_discoverable() {
            violations.push(M5HistoricalEvidenceDrillViolation::AccessibilityStateUndiscoverable);
        }

        // Guardrail row-invariants.
        if !binding.non_live_boundary_explicitly_called_out {
            violations.push(M5HistoricalEvidenceDrillViolation::NonLiveBoundaryNotCalledOut);
        }
        if binding.looks_live_by_omission {
            violations.push(M5HistoricalEvidenceDrillViolation::LooksLiveByOmission);
        }
        if binding.reopens_live_target_without_validating_identity_trust_route_and_authority {
            violations.push(
                M5HistoricalEvidenceDrillViolation::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
            );
        }
        if binding.dead_links_expired_or_removed_artifact {
            violations.push(M5HistoricalEvidenceDrillViolation::DeadLinksExpiredOrRemovedArtifact);
        }
        if binding.non_live_evidence_unjoined_to_capture_context {
            violations
                .push(M5HistoricalEvidenceDrillViolation::NonLiveEvidenceUnjoinedToCaptureContext);
        }
        if binding.presents_as_current_or_reopens_through_ambiguous_route {
            violations.push(
                M5HistoricalEvidenceDrillViolation::PresentsAsCurrentOrReopensThroughAmbiguousRoute,
            );
        }

        // Support / export consumers must map an object class back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations.push(M5HistoricalEvidenceDrillViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match fixture_grammar.get(binding.fixture_id.as_str()) {
            None => {
                fixture_grammar.insert(binding.fixture_id.as_str(), &binding.non_live_grammar);
            }
            Some(existing) => {
                if **existing != binding.non_live_grammar && !drift_reported {
                    violations.push(M5HistoricalEvidenceDrillViolation::GrammarDriftAcrossSurfaces);
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
        seen_states.insert(binding.evidence_state);
        seen_outcomes.insert(binding.expected_handoff_outcome);
        seen_blockers.insert(binding.expected_blocker);
    }

    // Coverage: every consumer surface, object class, and drill must appear.
    for consumer in M5HistoricalReferenceConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5HistoricalEvidenceDrillViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for object_class in M5HistoricalReferenceObject::ALL {
        if !seen_objects.contains(&object_class) {
            violations.push(M5HistoricalEvidenceDrillViolation::ObjectClassCoverageMissing);
            break;
        }
    }
    for drill in DrillScenario::ALL {
        if !seen_drills.contains(&drill) {
            violations.push(M5HistoricalEvidenceDrillViolation::DrillCoverageMissing);
            break;
        }
    }
    for blocker in DrillBlocker::ALL {
        if !seen_blockers.contains(&blocker) {
            violations.push(M5HistoricalEvidenceDrillViolation::ExactBlockerCoverageMissing);
            break;
        }
    }

    // AC1: at least four distinct historical-reference states and two distinct live-target handoff outcomes.
    if seen_states.len() < 4 {
        violations
            .push(M5HistoricalEvidenceDrillViolation::HistoricalReferenceStateCoverageInsufficient);
    }
    if seen_outcomes.len() < 2 {
        violations.push(M5HistoricalEvidenceDrillViolation::HandoffOutcomeCoverageInsufficient);
    }

    // Reuse: every present object class must be seeded by two or more distinct consumers.
    for consumers in object_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5HistoricalEvidenceDrillViolation::ObjectClassReuseUnproven);
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
