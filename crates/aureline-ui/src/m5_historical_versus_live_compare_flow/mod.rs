//! Historical-versus-live compare flows that pair a preserved M5 snapshot with its current live object
//! across the shell / archive-viewer, help / docs, support, review / incident, runbook-archive,
//! release-center, companion / export, program-governance, and CLI / export surfaces at **one canonical
//! compare vocabulary and action-rule set**.
//!
//! This module is the B149 historical-vs-live compare-flow lane over the five non-live-evidence object
//! classes frozen in [`crate::m5_historical_reference_matrix`] and made machine-readable by the
//! historical-snapshot-descriptor implement lane
//! ([`crate::m5_historical_snapshot_descriptor_and_change_diff_registries`]). Where the archive-viewer lane
//! ([`crate::m5_archived_snapshot_viewer_and_analysis_only_banner_consumers`]) proves how a single preserved
//! snapshot is *shown* as non-live, this lane proves how a preserved snapshot is *compared against its live
//! target* without collapsing the two into one ambiguous view: every compare surface pairs a historical
//! snapshot with its current live object, labels identity / freshness / drift, and — when the pairing is
//! approximate, its target is missing, or the pairing is policy-blocked — narrows the comparison with an
//! explicit mismatch reason instead of failing silently or dead-ending.
//!
//! The core honesty axes are three, mirroring the batch acceptance criteria.
//!
//! 1. **Identity / freshness / drift, always labeled.** A seeded historical snapshot compares against a
//!    current live object with explicit identity ([`CompareIdentityMatchState`]), freshness / drift
//!    ([`CompareFreshnessDriftState`]), and a drift summary, and the historical-side grammar
//!    ([`CompareHistoricalGrammar`]) is identical across every surface that renders the same profile. The
//!    historical-role word must be a token from the frozen [`M5HistoricalReferenceRole`] vocabulary, so no
//!    surface rewrites `snapshot_labeling`, `capture_time_attribution`, `provenance_attribution`, or
//!    `mutation_blocked_posture` in its own words.
//! 2. **No dead end, no silent failure.** A missing or mismatched live target never produces a dead end: the
//!    user can still inspect the historical packet and read an explicit [`CompareMismatchNote`] naming *why*
//!    the live comparison narrowed or failed — [`CompareMismatchReason::MissingLiveTarget`],
//!    [`CompareMismatchReason::ChangedScope`], [`CompareMismatchReason::ChangedBranchOrWorktree`],
//!    [`CompareMismatchReason::RetiredCapability`], or [`CompareMismatchReason::UnsupportedSkew`].
//! 3. **Never implies apply / sync is safe.** The compare action set ([`CompareAction`]) is deliberately
//!    closed and analysis-only — there is no apply / sync / restore variant — so a compare flow can never
//!    imply that applying or syncing the historical snapshot is safe. The historical side stays mutation
//!    blocked while the flow still allows navigation to a validated current live object or export of the
//!    comparison packet, and only an explicit, reviewed mutation handoff ([`ReviewedMutationHandoff`]) may
//!    name a path that takes over.
//!
//! Every binding names the accessibility routes ([`M5HistoricalReferenceAccessibilityRoute`]) through which
//! the compare state, its provenance, and the open-live-target action can be discovered without pointer-only
//! chrome; keyboard focus and screen-reader announcement are mandatory. Narrowing is disclosed, never hidden:
//! an approximate, missing-target, or policy-blocked pairing carries an explicit note naming the reason, the
//! preserved historical grammar, and the next action, so a surface may narrow *which* live comparison remains
//! without ever rewording the underlying historical grammar or quietly implying the snapshot is live.
//!
//! The packet references upstream historical-reference contracts by id rather than embedding their content.
//! Raw secret values, credentials, and private endpoints stay outside the support boundary.
//!
//! The boundary schema is
//! [`schemas/program/m5-historical-versus-live-compare-flow.schema.json`](../../../../schemas/program/m5-historical-versus-live-compare-flow.schema.json).
//! The contract doc is
//! [`docs/support/m5_historical_versus_live_compare_flow.md`](../../../../docs/support/m5_historical_versus_live_compare_flow.md).
//! The protected fixture directory is
//! [`fixtures/recovery/m5-historical-versus-live-compare/`](../../../../fixtures/recovery/m5-historical-versus-live-compare/).

mod seed;
#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

pub use seed::{
    seeded_m5_historical_versus_live_compare_flow,
    seeded_m5_historical_versus_live_compare_flow_missing_target_narrowed,
    seeded_m5_historical_versus_live_compare_flow_policy_blocked_narrowed,
};

use crate::m5_historical_reference_matrix::{
    M5HistoricalReferenceAccessibilityRoute, M5HistoricalReferenceConsumerSurface,
    M5HistoricalReferenceObject, M5HistoricalReferenceRole, M5_HISTORICAL_REFERENCE_MATRIX_DOC_REF,
    M5_HISTORICAL_REFERENCE_MATRIX_SCHEMA_REF,
};

/// Stable record-kind tag carried by [`M5HistoricalVersusLiveCompareFlowPacket`].
pub const M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_RECORD_KIND: &str =
    "m5_historical_versus_live_compare_flow_registry";

/// Schema version for historical-versus-live compare-flow records.
pub const M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_SCHEMA_VERSION: u32 = 1;

/// Stable packet id for the checked-in export.
pub const M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_PACKET_ID: &str =
    "m5-historical-versus-live-compare-flow:stable:0001";

/// Repo-relative path of the boundary schema.
pub const M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_SCHEMA_REF: &str =
    "schemas/program/m5-historical-versus-live-compare-flow.schema.json";

/// Repo-relative path of the contract doc.
pub const M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_DOC_REF: &str =
    "docs/support/m5_historical_versus_live_compare_flow.md";

/// Repo-relative path of the checked support-export artifact.
pub const M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_ARTIFACT_REF: &str =
    "artifacts/support/m5-historical-versus-live-compare/support_export.json";

/// Repo-relative path of the checked matrix CSV.
pub const M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_CSV_REF: &str =
    "artifacts/support/m5-historical-versus-live-compare/matrix.csv";

/// Repo-relative path of the checked Markdown summary.
pub const M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_REPORT_REF: &str =
    "artifacts/support/m5-historical-versus-live-compare/summary.md";

/// Repo-relative path of the protected fixture directory.
pub const M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_FIXTURE_DIR: &str =
    "fixtures/recovery/m5-historical-versus-live-compare";

/// Proof-freshness SLO in hours for this lane.
pub const M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_PROOF_SLO_HOURS: u32 = 720;

/// Mutation-blocked-posture sentinel words a historical-side grammar may never fall back to; a compare flow
/// whose historical role must be present before surfacing as non-live evidence must always keep a real
/// mutation-blocked posture rather than implying the object is editable, live, writable, or the current
/// object.
const MUTATION_BLOCKED_POSTURE_ABSENT_SENTINELS: [&str; 5] = [
    "none",
    "editable",
    "live_object",
    "writable",
    "current_object",
];

/// Whether a consumer surface is an export / support path that must map an object class back to its
/// canonical contract by id.
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
///
/// This is the "one vocabulary" gate: a historical-side role word must be a controlled role token rather than
/// a per-surface synonym.
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

/// The outcome of pairing a historical snapshot with its current live object.
///
/// The outcome governs the discoverable action set, identity / freshness state, and narrowing disclosure —
/// never the historical-side grammar: a narrowed outcome still carries the same historical-role,
/// snapshot-label, capture-time, provenance, and mutation-blocked-posture words, and discloses the narrowing
/// through an explicit mismatch note.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareOutcome {
    /// The live target exists and is a confirmed identity match; identity, freshness, and drift are labeled
    /// and the flow offers a validated open-current-live-object action.
    LiveTargetPaired,
    /// The live target exists but the pairing is only approximate (partial identity, changed scope, changed
    /// branch / worktree, or an unsupported skew); the narrowing is disclosed.
    ApproximatePairing,
    /// No live target remains; the historical packet stays inspectable and the reason is disclosed instead of
    /// a dead end.
    LiveTargetMissing,
    /// A policy or lifecycle block prevents live comparison; the historical packet stays inspectable and the
    /// reason is disclosed instead of a dead end.
    PolicyBlockedPairing,
}

impl CompareOutcome {
    /// Every outcome, in declaration order.
    pub const ALL: [Self; 4] = [
        Self::LiveTargetPaired,
        Self::ApproximatePairing,
        Self::LiveTargetMissing,
        Self::PolicyBlockedPairing,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LiveTargetPaired => "live_target_paired",
            Self::ApproximatePairing => "approximate_pairing",
            Self::LiveTargetMissing => "live_target_missing",
            Self::PolicyBlockedPairing => "policy_blocked_pairing",
        }
    }

    /// Whether this outcome narrows below a full, confirmed live pairing.
    pub const fn is_narrowed(self) -> bool {
        !matches!(self, Self::LiveTargetPaired)
    }

    /// The mismatch reasons this outcome is allowed to name. A confirmed pairing names none; every narrowed
    /// outcome must name exactly one reason from its allowed set.
    pub const fn allowed_mismatch_reasons(self) -> &'static [CompareMismatchReason] {
        match self {
            Self::LiveTargetPaired => &[],
            Self::ApproximatePairing => &[
                CompareMismatchReason::ChangedScope,
                CompareMismatchReason::ChangedBranchOrWorktree,
                CompareMismatchReason::UnsupportedSkew,
            ],
            Self::LiveTargetMissing => &[
                CompareMismatchReason::MissingLiveTarget,
                CompareMismatchReason::RetiredCapability,
            ],
            Self::PolicyBlockedPairing => &[
                CompareMismatchReason::RetiredCapability,
                CompareMismatchReason::UnsupportedSkew,
            ],
        }
    }
}

/// The action a compare surface may expose.
///
/// The set is deliberately closed and analysis-only: there is no apply / sync / restore action variant, so a
/// compare flow can never imply that applying or syncing the historical snapshot is safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareAction {
    /// Inspect the preserved historical packet metadata-only.
    InspectHistorical,
    /// Export the historical-versus-live comparison packet.
    ExportComparison,
    /// Open the current live object — only when the live target exists and is validated.
    OpenCurrentLiveObject,
}

impl CompareAction {
    /// The analysis-only base action set present on every compare surface.
    pub const ANALYSIS_ONLY_BASE: [Self; 2] = [Self::InspectHistorical, Self::ExportComparison];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InspectHistorical => "inspect_historical",
            Self::ExportComparison => "export_comparison",
            Self::OpenCurrentLiveObject => "open_current_live_object",
        }
    }
}

/// The identity-match state a compare surface labels between the historical snapshot and the live object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareIdentityMatchState {
    /// The historical snapshot and the live object are the same validated object identity.
    SameObjectIdentity,
    /// The pairing is only an approximate identity match.
    ApproximateIdentity,
    /// No live object could be identified, so identity cannot be verified.
    IdentityUnverifiable,
}

impl CompareIdentityMatchState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SameObjectIdentity => "same_object_identity",
            Self::ApproximateIdentity => "approximate_identity",
            Self::IdentityUnverifiable => "identity_unverifiable",
        }
    }
}

/// The freshness / drift state a compare surface labels between the historical snapshot and the live object.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareFreshnessDriftState {
    /// The snapshot is in sync with the live object; no drift.
    InSyncNoDrift,
    /// The snapshot is behind the current live object.
    SnapshotBehindLive,
    /// The snapshot has diverged from the current live object.
    SnapshotDivergedFromLive,
    /// No live object is available to compare against, so freshness cannot be verified.
    FreshnessUnverifiable,
}

impl CompareFreshnessDriftState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InSyncNoDrift => "in_sync_no_drift",
            Self::SnapshotBehindLive => "snapshot_behind_live",
            Self::SnapshotDivergedFromLive => "snapshot_diverged_from_live",
            Self::FreshnessUnverifiable => "freshness_unverifiable",
        }
    }

    /// A never-empty human word summarizing the drift, so drift is always labeled and never omitted.
    pub const fn drift_summary_word(self) -> &'static str {
        match self {
            Self::InSyncNoDrift => "no_drift_snapshot_matches_live",
            Self::SnapshotBehindLive => "drift_snapshot_behind_live",
            Self::SnapshotDivergedFromLive => "drift_snapshot_diverged_from_live",
            Self::FreshnessUnverifiable => "live_comparison_unavailable",
        }
    }
}

/// Why a compare surface narrowed its live comparison below a confirmed live pairing.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareMismatchReason {
    /// The live target is missing / removed.
    MissingLiveTarget,
    /// The live target's scope changed relative to the snapshot.
    ChangedScope,
    /// The live target's branch or worktree changed relative to the snapshot.
    ChangedBranchOrWorktree,
    /// The snapshot describes a retired capability with no live counterpart.
    RetiredCapability,
    /// The snapshot and live object are on an unsupported version skew.
    UnsupportedSkew,
}

impl CompareMismatchReason {
    /// Every reason, in declaration order.
    pub const ALL: [Self; 5] = [
        Self::MissingLiveTarget,
        Self::ChangedScope,
        Self::ChangedBranchOrWorktree,
        Self::RetiredCapability,
        Self::UnsupportedSkew,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MissingLiveTarget => "missing_live_target",
            Self::ChangedScope => "changed_scope",
            Self::ChangedBranchOrWorktree => "changed_branch_or_worktree",
            Self::RetiredCapability => "retired_capability",
            Self::UnsupportedSkew => "unsupported_skew",
        }
    }
}

/// The next action a narrowed compare surface offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareNarrowNextAction {
    /// Open the approximate-pairing detail explaining what differs.
    OpenApproximatePairingDetail,
    /// Inspect the historical packet metadata-only when no live comparison can run.
    InspectHistoricalPacketOnly,
}

impl CompareNarrowNextAction {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenApproximatePairingDetail => "open_approximate_pairing_detail",
            Self::InspectHistoricalPacketOnly => "inspect_historical_packet_only",
        }
    }
}

/// Whether a binding preserves a full confirmed live pairing or discloses a narrowed comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompareParityState {
    /// Historical grammar and a confirmed live pairing are preserved and shown.
    PairPreserved,
    /// Historical grammar is preserved and a narrowed comparison is explicitly disclosed.
    PairNarrowedDisclosed,
}

impl CompareParityState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PairPreserved => "pair_preserved",
            Self::PairNarrowedDisclosed => "pair_narrowed_disclosed",
        }
    }
}

/// Downgrade trigger that can narrow this compare lane below its claimed parity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoricalVersusLiveCompareFlowDowngradeTrigger {
    /// Proof packet has gone stale.
    ProofStale,
    /// Policy or legal block applies.
    PolicyBlocked,
    /// Historical grammar drifted between surfaces for the same profile.
    CompareGrammarDriftDetected,
    /// A historical side dropped its mutation-blocked posture and began to imply the object is live.
    MutationBlockedPostureDropped,
    /// A compare flow implied that applying or syncing the historical snapshot is safe.
    ImpliesApplyOrSyncHistoricalSnapshotSafe,
    /// A surface reopened a live target without validating identity, trust, route, and authority.
    ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
    /// A surface dead-ended on a missing or mismatched target instead of keeping the historical packet.
    DeadEndsOnMissingOrMismatchedTarget,
    /// A surface collapsed the snapshot and the live object into one ambiguous view.
    CollapsesSnapshotAndLiveIntoOneAmbiguousView,
    /// An identity, freshness, or drift label was dropped.
    IdentityFreshnessOrDriftLabelDropped,
    /// An accessibility route for the compare state, provenance, or open-live-target action was dropped.
    AccessibilityRouteDropped,
    /// An export / support consumer lost its canonical contract reference.
    CanonicalRegistryReferenceMissing,
    /// An upstream historical-reference contract narrowed.
    UpstreamHistoricalReferenceNarrowed,
}

impl HistoricalVersusLiveCompareFlowDowngradeTrigger {
    /// Every trigger, in declaration order.
    pub const ALL: [Self; 12] = [
        Self::ProofStale,
        Self::PolicyBlocked,
        Self::CompareGrammarDriftDetected,
        Self::MutationBlockedPostureDropped,
        Self::ImpliesApplyOrSyncHistoricalSnapshotSafe,
        Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
        Self::DeadEndsOnMissingOrMismatchedTarget,
        Self::CollapsesSnapshotAndLiveIntoOneAmbiguousView,
        Self::IdentityFreshnessOrDriftLabelDropped,
        Self::AccessibilityRouteDropped,
        Self::CanonicalRegistryReferenceMissing,
        Self::UpstreamHistoricalReferenceNarrowed,
    ];

    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProofStale => "proof_stale",
            Self::PolicyBlocked => "policy_blocked",
            Self::CompareGrammarDriftDetected => "compare_grammar_drift_detected",
            Self::MutationBlockedPostureDropped => "mutation_blocked_posture_dropped",
            Self::ImpliesApplyOrSyncHistoricalSnapshotSafe => {
                "implies_apply_or_sync_historical_snapshot_safe"
            }
            Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority => {
                "reopens_live_target_without_validating_identity_trust_route_and_authority"
            }
            Self::DeadEndsOnMissingOrMismatchedTarget => {
                "dead_ends_on_missing_or_mismatched_target"
            }
            Self::CollapsesSnapshotAndLiveIntoOneAmbiguousView => {
                "collapses_snapshot_and_live_into_one_ambiguous_view"
            }
            Self::IdentityFreshnessOrDriftLabelDropped => {
                "identity_freshness_or_drift_label_dropped"
            }
            Self::AccessibilityRouteDropped => "accessibility_route_dropped",
            Self::CanonicalRegistryReferenceMissing => "canonical_registry_reference_missing",
            Self::UpstreamHistoricalReferenceNarrowed => "upstream_historical_reference_narrowed",
        }
    }
}

/// The controlled historical-side grammar a preserved-snapshot profile presents.
///
/// These five words describe the historical (non-live) side of the comparison and must be identical across
/// every consumer surface that shows the same profile. The historical-role word must be a frozen
/// [`M5HistoricalReferenceRole`] token; the rest are controlled words the snapshot carries. A surface may
/// narrow the live comparison, but it may never reword any of these values per surface. Identity, freshness,
/// and drift labels are per-binding and derived from the outcome, so they are intentionally not part of this
/// shared grammar.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareHistoricalGrammar {
    /// Historical-role word (must be a frozen [`M5HistoricalReferenceRole`] token).
    pub historical_role_word: String,
    /// The captured-evidence / archived-snapshot label word.
    pub snapshot_label_word: String,
    /// The capture-time word the snapshot is attributed to.
    pub capture_time_word: String,
    /// The provenance / capture-context word the snapshot is attributed to.
    pub provenance_word: String,
    /// The mutation-blocked-posture word (read-only, non-authoritative-for-mutation).
    pub mutation_blocked_posture_word: String,
}

impl CompareHistoricalGrammar {
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

    /// Whether the profile honours the mutation-blocked rule: a historical-side role that must be present
    /// before the object may be surfaced as non-live evidence must pair it with a real mutation-blocked
    /// posture word and never collapse to an editable / live / writable / current-object sentinel.
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

/// The explicit note a narrowed comparison shows.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareMismatchNote {
    /// Why the comparison narrowed.
    pub reason: CompareMismatchReason,
    /// A never-omitted explanation of why the live comparison narrowed or failed.
    pub explanation: String,
    /// Note naming the preserved historical grammar (never omitted).
    pub preserved_grammar_note: String,
    /// The next action offered.
    pub next_action: CompareNarrowNextAction,
    /// Human-readable next-action copy (never omitted).
    pub next_action_label: String,
}

/// An explicit, reviewed mutation path that takes over when — and only when — a mutation is genuinely safe.
///
/// A compare flow never implies apply / sync is safe by itself; this handoff names the separate, reviewed
/// path (for example a migration or restore review flow) that owns any actual mutation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReviewedMutationHandoff {
    /// Stable id of the reviewed mutation path.
    pub reviewed_path_id: String,
    /// Human-readable label of the reviewed mutation path.
    pub reviewed_path_label: String,
}

/// Disclosures a compare binding must carry, derived from its outcome.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompareRenderDisclosure {
    /// The parity state the outcome requires.
    pub parity_state: CompareParityState,
    /// The identity state the outcome requires.
    pub identity_match_state: CompareIdentityMatchState,
    /// The next action the mismatch note must offer, if any.
    pub narrow_next_action: Option<CompareNarrowNextAction>,
    /// Whether the binding must carry an explicit mismatch note.
    pub needs_mismatch_note: bool,
    /// Whether the binding requires a verifiable (non-`FreshnessUnverifiable`) freshness comparison.
    pub requires_live_freshness: bool,
    /// Whether the binding offers a validated open-current-live-object action.
    pub offers_open_live_target: bool,
}

/// Resolves the render disclosures a compare binding must carry from its outcome.
///
/// A confirmed live pairing renders the full analysis-only action set plus a validated
/// open-current-live-object action, a same-object identity, and a verifiable freshness comparison. An
/// approximate, missing-target, or policy-blocked pairing narrows the live comparison and discloses the
/// narrowing through an explicit note — but all three keep every historical grammar word.
pub const fn resolve_compare_render_disclosure(outcome: CompareOutcome) -> CompareRenderDisclosure {
    match outcome {
        CompareOutcome::LiveTargetPaired => CompareRenderDisclosure {
            parity_state: CompareParityState::PairPreserved,
            identity_match_state: CompareIdentityMatchState::SameObjectIdentity,
            narrow_next_action: None,
            needs_mismatch_note: false,
            requires_live_freshness: true,
            offers_open_live_target: true,
        },
        CompareOutcome::ApproximatePairing => CompareRenderDisclosure {
            parity_state: CompareParityState::PairNarrowedDisclosed,
            identity_match_state: CompareIdentityMatchState::ApproximateIdentity,
            narrow_next_action: Some(CompareNarrowNextAction::OpenApproximatePairingDetail),
            needs_mismatch_note: true,
            requires_live_freshness: true,
            offers_open_live_target: true,
        },
        CompareOutcome::LiveTargetMissing => CompareRenderDisclosure {
            parity_state: CompareParityState::PairNarrowedDisclosed,
            identity_match_state: CompareIdentityMatchState::IdentityUnverifiable,
            narrow_next_action: Some(CompareNarrowNextAction::InspectHistoricalPacketOnly),
            needs_mismatch_note: true,
            requires_live_freshness: false,
            offers_open_live_target: false,
        },
        CompareOutcome::PolicyBlockedPairing => CompareRenderDisclosure {
            parity_state: CompareParityState::PairNarrowedDisclosed,
            identity_match_state: CompareIdentityMatchState::IdentityUnverifiable,
            narrow_next_action: Some(CompareNarrowNextAction::InspectHistoricalPacketOnly),
            needs_mismatch_note: true,
            requires_live_freshness: false,
            offers_open_live_target: false,
        },
    }
}

/// One compare binding: a preserved-snapshot object class compared against its live target on one consumer
/// surface in one outcome for one preserved-snapshot profile.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompareFlowBinding {
    /// Stable binding id.
    pub binding_id: String,
    /// Stable preserved-snapshot-profile id (shared across surfaces that show the same profile).
    pub snapshot_profile_id: String,
    /// Human-readable preserved-snapshot-profile identity.
    pub snapshot_profile_label: String,
    /// Which preserved-snapshot object class this binding compares.
    pub object_class: M5HistoricalReferenceObject,
    /// Which consumer surface renders it.
    pub consumer: M5HistoricalReferenceConsumerSurface,
    /// The outcome of pairing this snapshot with its live target.
    pub outcome: CompareOutcome,
    /// The controlled historical-side grammar presented (identical across surfaces for one profile).
    pub historical_grammar: CompareHistoricalGrammar,
    /// The labeled identity-match state between the snapshot and the live object.
    pub identity_match_state: CompareIdentityMatchState,
    /// The labeled freshness / drift state between the snapshot and the live object.
    pub freshness_drift_state: CompareFreshnessDriftState,
    /// A never-empty drift summary word, so drift is always labeled.
    pub drift_summary: String,
    /// Whether a confirmed pairing is preserved or a narrowing is disclosed.
    pub parity_state: CompareParityState,
    /// The discoverable action set allowed on this compare surface.
    pub allowed_actions: Vec<CompareAction>,
    /// The accessibility routes through which the compare state, provenance, and open-live-target action can
    /// be discovered without pointer-only chrome.
    pub accessibility_routes: Vec<M5HistoricalReferenceAccessibilityRoute>,
    /// The explicit mismatch note; required and complete when the outcome narrows.
    pub mismatch_note: Option<CompareMismatchNote>,
    /// An explicit, reviewed mutation handoff that takes over when a mutation is genuinely safe; absent by
    /// default, since a compare flow never implies apply / sync is safe.
    pub reviewed_mutation_handoff: Option<ReviewedMutationHandoff>,
    /// The historical side stays mutation blocked. MUST be `true`.
    pub historical_side_mutation_blocked: bool,
    /// Guardrail: this surface collapses the snapshot and the live object into one ambiguous view. MUST be
    /// `false`.
    pub collapses_snapshot_and_live_into_one_ambiguous_view: bool,
    /// Guardrail: this surface implies that applying or syncing the historical snapshot is safe. MUST be
    /// `false`.
    pub implies_apply_or_sync_historical_snapshot_is_safe: bool,
    /// Guardrail: this surface reopens a live target without validating identity, trust, route, and
    /// authority. MUST be `false`.
    pub reopens_live_target_without_validating_identity_trust_route_and_authority: bool,
    /// Guardrail: this surface dead-ends on a missing or mismatched target instead of keeping the historical
    /// packet. MUST be `false`.
    pub dead_ends_on_missing_or_mismatched_target: bool,
    /// Guardrail: this surface leaves the historical side mutable or unlabeled. MUST be `false`.
    pub leaves_historical_side_mutable_or_unlabeled: bool,
    /// Source contract refs this binding points at.
    pub source_contract_refs: Vec<String>,
}

impl CompareFlowBinding {
    /// Disclosures this binding must carry, derived from its outcome.
    pub const fn disclosure(&self) -> CompareRenderDisclosure {
        resolve_compare_render_disclosure(self.outcome)
    }

    /// Whether this binding renders below a full confirmed live pairing.
    pub const fn is_narrowed(&self) -> bool {
        self.outcome.is_narrowed()
    }

    /// Whether every guardrail row-invariant holds (historical side mutation blocked, all guardrails false).
    pub const fn guardrails_hold(&self) -> bool {
        self.historical_side_mutation_blocked
            && !self.collapses_snapshot_and_live_into_one_ambiguous_view
            && !self.implies_apply_or_sync_historical_snapshot_is_safe
            && !self.reopens_live_target_without_validating_identity_trust_route_and_authority
            && !self.dead_ends_on_missing_or_mismatched_target
            && !self.leaves_historical_side_mutable_or_unlabeled
    }

    /// Whether the analysis-only base action set is present.
    pub fn has_analysis_only_base_actions(&self) -> bool {
        CompareAction::ANALYSIS_ONLY_BASE
            .iter()
            .all(|action| self.allowed_actions.contains(action))
    }

    /// Whether no apply / sync affordance leaked in (structurally guaranteed by the closed action enum, but
    /// checked so the invariant is explicit).
    pub fn action_set_is_analysis_only(&self) -> bool {
        self.allowed_actions.iter().all(|action| {
            matches!(
                action,
                CompareAction::InspectHistorical
                    | CompareAction::ExportComparison
                    | CompareAction::OpenCurrentLiveObject
            )
        })
    }

    /// Whether the open-current-live-object action is present exactly when the outcome offers it.
    pub fn open_live_action_matches_outcome(&self) -> bool {
        let offered = self.disclosure().offers_open_live_target;
        let present = self
            .allowed_actions
            .contains(&CompareAction::OpenCurrentLiveObject);
        offered == present
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
pub struct HistoricalVersusLiveCompareFlowTrustReview {
    /// Object-class reuse is proven by fixtures rather than inferred from screenshots.
    pub object_class_reuse_proven_by_fixtures: bool,
    /// The same profile presents the same historical grammar across surfaces.
    pub same_profile_same_historical_grammar_across_surfaces: bool,
    /// Every historical-role word is a frozen role token.
    pub historical_role_words_stay_in_frozen_vocabulary: bool,
    /// A historical side's mutation-blocked posture never masquerades as a live, writable, or current object.
    pub mutation_blocked_posture_never_masquerades_as_live: bool,
    /// A compare flow never implies that applying or syncing the historical snapshot is safe.
    pub compare_never_implies_apply_or_sync_is_safe: bool,
    /// An open-live-target action always validates identity, trust, route, and authority first.
    pub open_live_target_always_validates_identity_trust_route_authority: bool,
    /// A missing or mismatched target never dead-ends; the historical packet stays inspectable.
    pub missing_or_mismatched_target_never_dead_ends: bool,
    /// The snapshot and the live object are never collapsed into one ambiguous view.
    pub snapshot_and_live_never_collapsed_into_one_ambiguous_view: bool,
    /// Identity, freshness, and drift are always labeled.
    pub identity_freshness_and_drift_always_labeled: bool,
    /// Accessibility routes for the compare state, provenance, and open-live-target action are present.
    pub accessibility_routes_present_for_state_provenance_and_open_live_target: bool,
    /// Narrowing is disclosed across paired, approximate, missing, and policy-blocked outcomes.
    pub narrowing_disclosed_across_outcomes: bool,
    /// Support / export consumers point at the canonical contracts.
    pub support_export_point_canonical_contracts: bool,
    /// Downgrade narrows the claim rather than hiding the object class.
    pub downgrade_narrows_instead_of_hides: bool,
    /// Stale or underqualified bindings automatically block promotion.
    pub stale_or_underqualified_blocks_promotion: bool,
}

impl HistoricalVersusLiveCompareFlowTrustReview {
    /// Whether every invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.object_class_reuse_proven_by_fixtures
            && self.same_profile_same_historical_grammar_across_surfaces
            && self.historical_role_words_stay_in_frozen_vocabulary
            && self.mutation_blocked_posture_never_masquerades_as_live
            && self.compare_never_implies_apply_or_sync_is_safe
            && self.open_live_target_always_validates_identity_trust_route_authority
            && self.missing_or_mismatched_target_never_dead_ends
            && self.snapshot_and_live_never_collapsed_into_one_ambiguous_view
            && self.identity_freshness_and_drift_always_labeled
            && self.accessibility_routes_present_for_state_provenance_and_open_live_target
            && self.narrowing_disclosed_across_outcomes
            && self.support_export_point_canonical_contracts
            && self.downgrade_narrows_instead_of_hides
            && self.stale_or_underqualified_blocks_promotion
    }
}

/// Consumer projection block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalVersusLiveCompareFlowProjection {
    /// The shell / archive-viewer surface consumes the shared compare flow.
    pub shell_consumes_compare_flow: bool,
    /// The help / docs surface consumes the shared compare flow.
    pub help_docs_consumes_compare_flow: bool,
    /// The support bundle viewer consumes the shared compare flow.
    pub support_consumes_compare_flow: bool,
    /// The review / incident surface consumes the shared compare flow.
    pub review_incident_consumes_compare_flow: bool,
    /// The runbook-archive surface consumes the shared compare flow.
    pub runbook_archive_consumes_compare_flow: bool,
    /// The release-center retirement snapshot page consumes the shared compare flow.
    pub release_center_consumes_compare_flow: bool,
    /// The companion / export path consumes the shared compare flow.
    pub companion_export_consumes_compare_flow: bool,
    /// The program-governance review consumes the shared compare flow.
    pub program_governance_consumes_compare_flow: bool,
    /// The CLI / export path consumes the shared compare flow.
    pub cli_export_consumes_compare_flow: bool,
    /// Every object class is paired by two or more consumers.
    pub every_object_class_paired_by_two_or_more_consumers: bool,
    /// Historical grammar is identical for the same profile.
    pub historical_grammar_identical_for_same_profile: bool,
    /// Narrowing is disclosed rather than hidden.
    pub narrowing_disclosed_not_hidden: bool,
    /// Export maps a compare row back to one historical-reference object class.
    pub compare_maps_back_to_one_historical_reference_object: bool,
}

impl HistoricalVersusLiveCompareFlowProjection {
    /// Whether every projection invariant holds.
    pub const fn all_hold(&self) -> bool {
        self.shell_consumes_compare_flow
            && self.help_docs_consumes_compare_flow
            && self.support_consumes_compare_flow
            && self.review_incident_consumes_compare_flow
            && self.runbook_archive_consumes_compare_flow
            && self.release_center_consumes_compare_flow
            && self.companion_export_consumes_compare_flow
            && self.program_governance_consumes_compare_flow
            && self.cli_export_consumes_compare_flow
            && self.every_object_class_paired_by_two_or_more_consumers
            && self.historical_grammar_identical_for_same_profile
            && self.narrowing_disclosed_not_hidden
            && self.compare_maps_back_to_one_historical_reference_object
    }
}

/// Proof freshness block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoricalVersusLiveCompareFlowProofFreshness {
    /// Proof-freshness SLO in hours.
    pub proof_freshness_slo_hours: u32,
    /// RFC 3339 timestamp of the last proof refresh.
    pub last_proof_refresh: String,
    /// True when stale proof automatically narrows the lane.
    pub auto_narrow_on_stale: bool,
}

/// Constructor input for [`M5HistoricalVersusLiveCompareFlowPacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct M5HistoricalVersusLiveCompareFlowPacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Compare bindings.
    pub compare_bindings: Vec<CompareFlowBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<HistoricalVersusLiveCompareFlowDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5HistoricalReferenceConsumerSurface>,
    /// Trust review block.
    pub trust_review: HistoricalVersusLiveCompareFlowTrustReview,
    /// Consumer projection block.
    pub consumer_projection: HistoricalVersusLiveCompareFlowProjection,
    /// Proof freshness block.
    pub proof_freshness: HistoricalVersusLiveCompareFlowProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe historical-versus-live compare-flow packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct M5HistoricalVersusLiveCompareFlowPacket {
    /// Record kind; must equal [`M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable surface label.
    pub surface_label: String,
    /// Compare bindings.
    pub compare_bindings: Vec<CompareFlowBinding>,
    /// Downgrade triggers that apply to this lane.
    pub downgrade_triggers: Vec<HistoricalVersusLiveCompareFlowDowngradeTrigger>,
    /// Consumer surfaces this packet covers.
    pub consumer_surfaces: Vec<M5HistoricalReferenceConsumerSurface>,
    /// Trust review block.
    pub trust_review: HistoricalVersusLiveCompareFlowTrustReview,
    /// Consumer projection block.
    pub consumer_projection: HistoricalVersusLiveCompareFlowProjection,
    /// Proof freshness block.
    pub proof_freshness: HistoricalVersusLiveCompareFlowProofFreshness,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl M5HistoricalVersusLiveCompareFlowPacket {
    /// Builds a historical-versus-live compare-flow packet from stable-lane input.
    pub fn new(input: M5HistoricalVersusLiveCompareFlowPacketInput) -> Self {
        Self {
            record_kind: M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_RECORD_KIND.to_owned(),
            schema_version: M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_SCHEMA_VERSION,
            packet_id: input.packet_id,
            surface_label: input.surface_label,
            compare_bindings: input.compare_bindings,
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

    /// Validates the historical-versus-live compare-flow invariants.
    pub fn validate(&self) -> Vec<M5HistoricalVersusLiveCompareFlowViolation> {
        let mut violations = Vec::new();

        if self.record_kind != M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_RECORD_KIND {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::WrongRecordKind);
        }
        if self.schema_version != M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_SCHEMA_VERSION {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.surface_label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::MissingIdentity);
        }
        if self.downgrade_triggers.is_empty() {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::DowngradeTriggersMissing);
        }
        if self.consumer_surfaces.is_empty() {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::ConsumerSurfacesMissing);
        }

        validate_source_contracts(self, &mut violations);
        validate_bindings(self, &mut violations);

        if !self.trust_review.all_hold() {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::TrustReviewIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations
                .push(M5HistoricalVersusLiveCompareFlowViolation::ConsumerProjectionIncomplete);
        }
        if self.proof_freshness.proof_freshness_slo_hours == 0
            || self.proof_freshness.last_proof_refresh.trim().is_empty()
        {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::ProofFreshnessIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("compare-flow packet serializes"),
        ) {
            violations
                .push(M5HistoricalVersusLiveCompareFlowViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("compare-flow packet serializes")
    }

    /// Deterministic matrix CSV, one row per compare binding.
    pub fn render_matrix_csv(&self) -> String {
        let mut out = String::from(
            "object_class,consumer,outcome,identity_match_state,freshness_drift_state,historical_role_word,parity_state\n",
        );
        for binding in &self.compare_bindings {
            out.push_str(&format!(
                "{},{},{},{},{},{},{}\n",
                binding.object_class.as_str(),
                binding.consumer.as_str(),
                binding.outcome.as_str(),
                binding.identity_match_state.as_str(),
                binding.freshness_drift_state.as_str(),
                binding.historical_grammar.historical_role_word,
                binding.parity_state.as_str(),
            ));
        }
        out
    }

    /// Deterministic Markdown summary for support, review, or release handoff.
    pub fn render_markdown_summary(&self) -> String {
        let narrowed = self
            .compare_bindings
            .iter()
            .filter(|binding| binding.is_narrowed())
            .count();

        let mut out = String::new();
        out.push_str("# Historical-vs-Live Compare Flows: One Vocabulary Across Surfaces\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Surface: `{}`\n", self.surface_label));
        out.push_str(&format!(
            "- Compare bindings: {} ({} narrowed)\n",
            self.compare_bindings.len(),
            narrowed
        ));
        out.push_str(&format!(
            "- Proof freshness SLO: {} hours (last refresh: {})\n",
            self.proof_freshness.proof_freshness_slo_hours, self.proof_freshness.last_proof_refresh
        ));

        out.push_str("\n## Compare bindings\n\n");
        for binding in &self.compare_bindings {
            out.push_str(&format!(
                "- **{}** [`{}`]: object `{}` on `{}`, outcome `{}`, identity `{}`, freshness `{}`, role `{}`\n",
                binding.snapshot_profile_label,
                binding.binding_id,
                binding.object_class.as_str(),
                binding.consumer.as_str(),
                binding.outcome.as_str(),
                binding.identity_match_state.as_str(),
                binding.freshness_drift_state.as_str(),
                binding.historical_grammar.historical_role_word,
            ));
        }
        out
    }
}

/// Errors emitted when reading the checked-in compare-flow export.
#[derive(Debug)]
pub enum M5HistoricalVersusLiveCompareFlowArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<M5HistoricalVersusLiveCompareFlowViolation>),
}

impl fmt::Display for M5HistoricalVersusLiveCompareFlowArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(formatter, "compare-flow export parse failed: {error}")
            }
            Self::Validation(violations) => {
                let tokens = violations
                    .iter()
                    .map(|violation| violation.as_str())
                    .collect::<Vec<_>>()
                    .join(",");
                write!(formatter, "compare-flow export failed validation: {tokens}")
            }
        }
    }
}

impl Error for M5HistoricalVersusLiveCompareFlowArtifactError {}

/// Validation failures emitted by [`M5HistoricalVersusLiveCompareFlowPacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum M5HistoricalVersusLiveCompareFlowViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Source contract refs are incomplete.
    MissingSourceContracts,
    /// No compare bindings are present.
    CompareBindingsMissing,
    /// A compare binding is incomplete.
    BindingIncomplete,
    /// A binding's historical grammar values are incomplete.
    GrammarFacetIncomplete,
    /// A binding's historical-role word is not a frozen role token.
    HistoricalRoleWordOutsideVocabulary,
    /// A binding's gate-role dropped its mutation-blocked posture.
    MutationBlockedPostureMissingForGateRole,
    /// A binding's parity state does not match its outcome.
    ParityStateMismatch,
    /// A binding's identity-match state does not match its outcome.
    IdentityStateMismatch,
    /// A binding's freshness / drift state is invalid for its outcome.
    FreshnessStateInvalidForOutcome,
    /// A binding's drift summary is missing.
    DriftSummaryMissing,
    /// Two surfaces show the same profile with different historical grammar.
    CompareGrammarDriftAcrossSurfaces,
    /// A shared object class is not paired by at least two distinct consumers.
    ObjectClassReuseUnproven,
    /// A support / export binding does not point at the canonical contracts.
    SupportExportReferenceMissing,
    /// A narrowed binding is missing its explicit mismatch note.
    MismatchNoteMissing,
    /// A mismatch note's reason is not allowed for the outcome.
    MismatchReasonNotAllowedForOutcome,
    /// A mismatch note's next action does not match the required next action.
    MismatchNextActionMismatch,
    /// A mismatch note is missing its explanation.
    MismatchExplanationMissing,
    /// A mismatch note is missing its preserved-grammar note.
    MismatchNotePreservedGrammarMissing,
    /// A mismatch note is missing its next-action copy.
    MismatchNextActionLabelMissing,
    /// A confirmed-pairing binding carries a mismatch note it must not.
    UnexpectedMismatchNote,
    /// A binding is missing the analysis-only base action set.
    AnalysisOnlyBaseActionsMissing,
    /// A binding's action set is not analysis-only.
    ActionSetNotAnalysisOnly,
    /// A binding's open-current-live-object action does not match its outcome.
    OpenLiveActionOutcomeMismatch,
    /// A reviewed mutation handoff is present but incomplete.
    ReviewedMutationHandoffIncomplete,
    /// A binding cannot discover its compare state via keyboard focus and screen-reader announcement.
    AccessibilityStateUndiscoverable,
    /// A binding's historical side is not mutation blocked.
    HistoricalSideNotMutationBlocked,
    /// A binding collapses the snapshot and the live object into one ambiguous view.
    CollapsesSnapshotAndLiveIntoOneAmbiguousView,
    /// A binding implies that applying or syncing the historical snapshot is safe.
    ImpliesApplyOrSyncHistoricalSnapshotIsSafe,
    /// A binding reopens a live target without validating identity, trust, route, and authority.
    ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
    /// A binding dead-ends on a missing or mismatched target.
    DeadEndsOnMissingOrMismatchedTarget,
    /// A binding leaves the historical side mutable or unlabeled.
    LeavesHistoricalSideMutableOrUnlabeled,
    /// Not every consumer surface appears among the bindings.
    ConsumerCoverageMissing,
    /// Not every shared object class appears among the bindings.
    ObjectClassCoverageMissing,
    /// Not every compare outcome appears among the bindings.
    OutcomeCoverageMissing,
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

impl M5HistoricalVersusLiveCompareFlowViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::CompareBindingsMissing => "compare_bindings_missing",
            Self::BindingIncomplete => "binding_incomplete",
            Self::GrammarFacetIncomplete => "grammar_facet_incomplete",
            Self::HistoricalRoleWordOutsideVocabulary => "historical_role_word_outside_vocabulary",
            Self::MutationBlockedPostureMissingForGateRole => {
                "mutation_blocked_posture_missing_for_gate_role"
            }
            Self::ParityStateMismatch => "parity_state_mismatch",
            Self::IdentityStateMismatch => "identity_state_mismatch",
            Self::FreshnessStateInvalidForOutcome => "freshness_state_invalid_for_outcome",
            Self::DriftSummaryMissing => "drift_summary_missing",
            Self::CompareGrammarDriftAcrossSurfaces => "compare_grammar_drift_across_surfaces",
            Self::ObjectClassReuseUnproven => "object_class_reuse_unproven",
            Self::SupportExportReferenceMissing => "support_export_reference_missing",
            Self::MismatchNoteMissing => "mismatch_note_missing",
            Self::MismatchReasonNotAllowedForOutcome => "mismatch_reason_not_allowed_for_outcome",
            Self::MismatchNextActionMismatch => "mismatch_next_action_mismatch",
            Self::MismatchExplanationMissing => "mismatch_explanation_missing",
            Self::MismatchNotePreservedGrammarMissing => "mismatch_note_preserved_grammar_missing",
            Self::MismatchNextActionLabelMissing => "mismatch_next_action_label_missing",
            Self::UnexpectedMismatchNote => "unexpected_mismatch_note",
            Self::AnalysisOnlyBaseActionsMissing => "analysis_only_base_actions_missing",
            Self::ActionSetNotAnalysisOnly => "action_set_not_analysis_only",
            Self::OpenLiveActionOutcomeMismatch => "open_live_action_outcome_mismatch",
            Self::ReviewedMutationHandoffIncomplete => "reviewed_mutation_handoff_incomplete",
            Self::AccessibilityStateUndiscoverable => "accessibility_state_undiscoverable",
            Self::HistoricalSideNotMutationBlocked => "historical_side_not_mutation_blocked",
            Self::CollapsesSnapshotAndLiveIntoOneAmbiguousView => {
                "collapses_snapshot_and_live_into_one_ambiguous_view"
            }
            Self::ImpliesApplyOrSyncHistoricalSnapshotIsSafe => {
                "implies_apply_or_sync_historical_snapshot_is_safe"
            }
            Self::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority => {
                "reopens_live_target_without_validating_identity_trust_route_and_authority"
            }
            Self::DeadEndsOnMissingOrMismatchedTarget => {
                "dead_ends_on_missing_or_mismatched_target"
            }
            Self::LeavesHistoricalSideMutableOrUnlabeled => {
                "leaves_historical_side_mutable_or_unlabeled"
            }
            Self::ConsumerCoverageMissing => "consumer_coverage_missing",
            Self::ObjectClassCoverageMissing => "object_class_coverage_missing",
            Self::OutcomeCoverageMissing => "outcome_coverage_missing",
            Self::DowngradeTriggersMissing => "downgrade_triggers_missing",
            Self::ConsumerSurfacesMissing => "consumer_surfaces_missing",
            Self::TrustReviewIncomplete => "trust_review_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::ProofFreshnessIncomplete => "proof_freshness_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable compare-flow export.
pub fn current_stable_m5_historical_versus_live_compare_flow_export(
) -> Result<M5HistoricalVersusLiveCompareFlowPacket, M5HistoricalVersusLiveCompareFlowArtifactError>
{
    let packet: M5HistoricalVersusLiveCompareFlowPacket =
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../artifacts/support/m5-historical-versus-live-compare/support_export.json"
        )))
        .map_err(M5HistoricalVersusLiveCompareFlowArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(M5HistoricalVersusLiveCompareFlowArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &M5HistoricalVersusLiveCompareFlowPacket,
    violations: &mut Vec<M5HistoricalVersusLiveCompareFlowViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    let mut required: Vec<&str> = vec![
        M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_SCHEMA_REF,
        M5_HISTORICAL_VERSUS_LIVE_COMPARE_FLOW_DOC_REF,
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
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::MissingSourceContracts);
            return;
        }
    }
}

fn validate_bindings(
    packet: &M5HistoricalVersusLiveCompareFlowPacket,
    violations: &mut Vec<M5HistoricalVersusLiveCompareFlowViolation>,
) {
    if packet.compare_bindings.is_empty() {
        violations.push(M5HistoricalVersusLiveCompareFlowViolation::CompareBindingsMissing);
        return;
    }

    // One vocabulary: the historical grammar must be identical for every binding that renders the same
    // preserved-snapshot profile.
    let mut profile_grammar: BTreeMap<&str, &CompareHistoricalGrammar> = BTreeMap::new();
    let mut drift_reported = false;

    // Reuse: each object class must be paired by at least two distinct consumers.
    let mut object_consumers: BTreeMap<
        M5HistoricalReferenceObject,
        BTreeSet<M5HistoricalReferenceConsumerSurface>,
    > = BTreeMap::new();
    let mut seen_consumers: BTreeSet<M5HistoricalReferenceConsumerSurface> = BTreeSet::new();
    let mut seen_objects: BTreeSet<M5HistoricalReferenceObject> = BTreeSet::new();
    let mut seen_outcomes: BTreeSet<CompareOutcome> = BTreeSet::new();

    for binding in &packet.compare_bindings {
        if binding.binding_id.trim().is_empty()
            || binding.snapshot_profile_id.trim().is_empty()
            || binding.snapshot_profile_label.trim().is_empty()
            || binding.source_contract_refs.is_empty()
        {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::BindingIncomplete);
        }
        if !binding.historical_grammar.all_present() {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::GrammarFacetIncomplete);
        }
        if !binding
            .historical_grammar
            .historical_role_word_in_vocabulary()
        {
            violations.push(
                M5HistoricalVersusLiveCompareFlowViolation::HistoricalRoleWordOutsideVocabulary,
            );
        }
        if !binding
            .historical_grammar
            .mutation_blocked_posture_satisfied()
        {
            violations.push(
                M5HistoricalVersusLiveCompareFlowViolation::MutationBlockedPostureMissingForGateRole,
            );
        }

        let disclosure = binding.disclosure();

        if binding.parity_state != disclosure.parity_state {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::ParityStateMismatch);
        }
        if binding.identity_match_state != disclosure.identity_match_state {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::IdentityStateMismatch);
        }

        // Freshness / drift must be verifiable only when the outcome supports a live comparison.
        let freshness_verifiable =
            binding.freshness_drift_state != CompareFreshnessDriftState::FreshnessUnverifiable;
        if disclosure.requires_live_freshness != freshness_verifiable {
            violations
                .push(M5HistoricalVersusLiveCompareFlowViolation::FreshnessStateInvalidForOutcome);
        }
        if binding.drift_summary.trim().is_empty() {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::DriftSummaryMissing);
        }

        // Narrowing disclosure.
        if disclosure.needs_mismatch_note {
            match &binding.mismatch_note {
                None => {
                    violations
                        .push(M5HistoricalVersusLiveCompareFlowViolation::MismatchNoteMissing);
                }
                Some(note) => {
                    if !binding
                        .outcome
                        .allowed_mismatch_reasons()
                        .contains(&note.reason)
                    {
                        violations.push(
                            M5HistoricalVersusLiveCompareFlowViolation::MismatchReasonNotAllowedForOutcome,
                        );
                    }
                    if Some(note.next_action) != disclosure.narrow_next_action {
                        violations.push(
                            M5HistoricalVersusLiveCompareFlowViolation::MismatchNextActionMismatch,
                        );
                    }
                    if note.explanation.trim().is_empty() {
                        violations.push(
                            M5HistoricalVersusLiveCompareFlowViolation::MismatchExplanationMissing,
                        );
                    }
                    if note.preserved_grammar_note.trim().is_empty() {
                        violations.push(
                            M5HistoricalVersusLiveCompareFlowViolation::MismatchNotePreservedGrammarMissing,
                        );
                    }
                    if note.next_action_label.trim().is_empty() {
                        violations.push(
                            M5HistoricalVersusLiveCompareFlowViolation::MismatchNextActionLabelMissing,
                        );
                    }
                }
            }
        } else if binding.mismatch_note.is_some() {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::UnexpectedMismatchNote);
        }

        // Reviewed mutation handoff, when present, must be complete.
        if let Some(handoff) = &binding.reviewed_mutation_handoff {
            if handoff.reviewed_path_id.trim().is_empty()
                || handoff.reviewed_path_label.trim().is_empty()
            {
                violations.push(
                    M5HistoricalVersusLiveCompareFlowViolation::ReviewedMutationHandoffIncomplete,
                );
            }
        }

        // Action rules.
        if !binding.has_analysis_only_base_actions() {
            violations
                .push(M5HistoricalVersusLiveCompareFlowViolation::AnalysisOnlyBaseActionsMissing);
        }
        if !binding.action_set_is_analysis_only() {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::ActionSetNotAnalysisOnly);
        }
        if !binding.open_live_action_matches_outcome() {
            violations
                .push(M5HistoricalVersusLiveCompareFlowViolation::OpenLiveActionOutcomeMismatch);
        }

        // Accessibility discovery.
        if !binding.accessibility_state_discoverable() {
            violations
                .push(M5HistoricalVersusLiveCompareFlowViolation::AccessibilityStateUndiscoverable);
        }

        // Guardrail row-invariants.
        if !binding.historical_side_mutation_blocked {
            violations
                .push(M5HistoricalVersusLiveCompareFlowViolation::HistoricalSideNotMutationBlocked);
        }
        if binding.collapses_snapshot_and_live_into_one_ambiguous_view {
            violations.push(
                M5HistoricalVersusLiveCompareFlowViolation::CollapsesSnapshotAndLiveIntoOneAmbiguousView,
            );
        }
        if binding.implies_apply_or_sync_historical_snapshot_is_safe {
            violations.push(
                M5HistoricalVersusLiveCompareFlowViolation::ImpliesApplyOrSyncHistoricalSnapshotIsSafe,
            );
        }
        if binding.reopens_live_target_without_validating_identity_trust_route_and_authority {
            violations.push(
                M5HistoricalVersusLiveCompareFlowViolation::ReopensLiveTargetWithoutValidatingIdentityTrustRouteAndAuthority,
            );
        }
        if binding.dead_ends_on_missing_or_mismatched_target {
            violations.push(
                M5HistoricalVersusLiveCompareFlowViolation::DeadEndsOnMissingOrMismatchedTarget,
            );
        }
        if binding.leaves_historical_side_mutable_or_unlabeled {
            violations.push(
                M5HistoricalVersusLiveCompareFlowViolation::LeavesHistoricalSideMutableOrUnlabeled,
            );
        }

        // Support / export consumers must map an object class back to canonical contracts.
        if consumer_must_reference_canonical(binding.consumer)
            && !binding.points_at_canonical_contracts()
        {
            violations
                .push(M5HistoricalVersusLiveCompareFlowViolation::SupportExportReferenceMissing);
        }

        // Grammar-drift accumulation.
        match profile_grammar.get(binding.snapshot_profile_id.as_str()) {
            None => {
                profile_grammar.insert(
                    binding.snapshot_profile_id.as_str(),
                    &binding.historical_grammar,
                );
            }
            Some(existing) => {
                if **existing != binding.historical_grammar && !drift_reported {
                    violations.push(
                        M5HistoricalVersusLiveCompareFlowViolation::CompareGrammarDriftAcrossSurfaces,
                    );
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
        seen_outcomes.insert(binding.outcome);
    }

    // Coverage: every consumer surface, object class, and outcome must appear.
    for consumer in M5HistoricalReferenceConsumerSurface::ALL {
        if !seen_consumers.contains(&consumer) {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::ConsumerCoverageMissing);
            break;
        }
    }
    for object_class in M5HistoricalReferenceObject::ALL {
        if !seen_objects.contains(&object_class) {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::ObjectClassCoverageMissing);
            break;
        }
    }
    for outcome in CompareOutcome::ALL {
        if !seen_outcomes.contains(&outcome) {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::OutcomeCoverageMissing);
            break;
        }
    }

    // Reuse: every present object class must be paired by two or more distinct consumers.
    for consumers in object_consumers.values() {
        if consumers.len() < 2 {
            violations.push(M5HistoricalVersusLiveCompareFlowViolation::ObjectClassReuseUnproven);
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
