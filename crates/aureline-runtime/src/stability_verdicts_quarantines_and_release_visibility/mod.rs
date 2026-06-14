//! Stability verdicts, flaky-state badges, and quarantine records with evidence
//! windows, confidence, owners, expiry, restore conditions, and release
//! visibility for the M5 test-intelligence lane.
//!
//! Where [`crate::session_plans_attempt_records_and_execution_lineage`] makes the
//! *execution* of a test selection attributable — a [`SessionPlan`] plus an
//! append-only history of [`AttemptRecord`]s — this module converts repeated
//! outcomes from that ledger into governed, evidence-based **stability verdicts**
//! and **quarantine records**. A flaky state is no longer a single opaque boolean
//! label: it is a [`StabilityVerdictRecord`] carrying a controlled
//! [`StabilityVerdictState`], a visible [`EvidenceWindow`], a
//! [`StabilityConfidenceClass`], an owner, and a [`ReleaseVisibilityClass`].
//!
//! [`SessionPlan`]: crate::session_plans_attempt_records_and_execution_lineage::SessionPlan
//! [`AttemptRecord`]: crate::session_plans_attempt_records_and_execution_lineage::AttemptRecord
//!
//! * a [`StabilityVerdictRecord`] ties a durable [`VerdictSubject`] to a
//!   controlled state, an evidence window (pass / fail / inconclusive counts over
//!   the attempts that produced it), a confidence class, an owner, and a release
//!   visibility class. Its [`VerdictEvidenceProvenance`] keeps an imported /
//!   provider-backed verdict from ever reading as a locally verified pass: an
//!   imported verdict is [`StabilityVerdictState::ImportedOnlyUnverified`], carries
//!   an `origin_provider_ref`, and can never be [`StabilityVerdictState::Stable`];
//! * a [`QuarantineRecord`] ties a verdict to an owner, an expiry, a
//!   [`RestoreConditionClass`], and a [`ReleaseVisibilityClass`]. Quarantines stay
//!   visible, filterable, countable, and exportable — never collapsed into local
//!   muting — and an expired quarantine reopens its scope
//!   ([`QuarantineState::ExpiredReopened`]) so it fails readiness instead of
//!   silently persisting as local UI state;
//! * a [`ReadinessImpactClass`] travels with every verdict and quarantine so a
//!   stale, quarantined, confirmed-flaky, or unknown row can never roll up behind a
//!   generic green state: only a [`StabilityVerdictState::Stable`] verdict may
//!   carry [`ReadinessImpactClass::NoImpact`].
//!
//! [`StabilityVerdictQuarantinePacket::validate`] refuses a packet that collapses a
//! parameterized template into a concrete invocation, lets an imported verdict read
//! as a locally verified pass, hides a stale or quarantined row behind a green
//! readiness state, drops a quarantine's owner / expiry / release visibility, or
//! records a verdict or quarantine that release and support packets cannot reopen
//! from the export alone.
//!
//! Raw test source, raw provider payloads, provider cursors, credentials, host
//! names, and raw artifact bodies never cross this boundary; the packet carries
//! only typed class tokens, booleans, opaque ids, fingerprint digests, and
//! redaction-aware reviewable labels.
//!
//! The boundary schema is
//! [`schemas/testing/stability-verdicts-quarantines-and-release-visibility.schema.json`](../../../../schemas/testing/stability-verdicts-quarantines-and-release-visibility.schema.json).
//! The contract doc is
//! [`docs/testing/m5/stability-verdicts-quarantines-and-release-visibility.md`](../../../../docs/testing/m5/stability-verdicts-quarantines-and-release-visibility.md).
//! The protected fixture directory is
//! [`fixtures/testing/m5/stability-verdicts-quarantines-and-release-visibility/`](../../../../fixtures/testing/m5/stability-verdicts-quarantines-and-release-visibility/).

#[cfg(test)]
mod tests;

use std::collections::BTreeSet;
use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::durable_test_items_and_partial_discovery::DurableTestNodeKind;
use crate::testing_identity::TestItemIdentityClass;

/// Stable record-kind tag carried by [`StabilityVerdictQuarantinePacket`].
pub const STABILITY_VERDICT_QUARANTINE_RECORD_KIND: &str =
    "test_stability_verdict_quarantine_packet";

/// Schema version for the stability-verdict / quarantine packet.
pub const STABILITY_VERDICT_QUARANTINE_SCHEMA_VERSION: u32 = 1;

/// Repo-relative path of the boundary schema.
pub const STABILITY_VERDICT_QUARANTINE_SCHEMA_REF: &str =
    "schemas/testing/stability-verdicts-quarantines-and-release-visibility.schema.json";

/// Repo-relative path of the contract doc.
pub const STABILITY_VERDICT_QUARANTINE_DOC_REF: &str =
    "docs/testing/m5/stability-verdicts-quarantines-and-release-visibility.md";

/// Repo-relative path of the checked support-export artifact.
pub const STABILITY_VERDICT_QUARANTINE_ARTIFACT_REF: &str =
    "artifacts/testing/m5/stability-verdicts-quarantines-and-release-visibility/support_export.json";

/// Repo-relative path of the checked Markdown summary.
pub const STABILITY_VERDICT_QUARANTINE_SUMMARY_REF: &str =
    "artifacts/testing/m5/stability-verdicts-quarantines-and-release-visibility.md";

/// Repo-relative path of the protected fixture directory.
pub const STABILITY_VERDICT_QUARANTINE_FIXTURE_DIR: &str =
    "fixtures/testing/m5/stability-verdicts-quarantines-and-release-visibility";

/// Controlled stability-verdict state. This is the badge vocabulary: a flaky
/// state is one of these controlled tokens backed by a visible [`EvidenceWindow`],
/// never a single opaque boolean label.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilityVerdictState {
    /// Locally verified stable: the evidence window holds no failures.
    Stable,
    /// Intermittent behavior is suspected but not yet reproduced.
    SuspectedFlaky,
    /// Intermittent behavior reproduced: the window holds both passes and failures.
    ConfirmedFlaky,
    /// The scope is quarantined; a [`QuarantineRecord`] governs it.
    Quarantined,
    /// Known failing: the window holds failures and no passes.
    KnownFailing,
    /// Imported / provider-backed evidence only; never a locally verified pass.
    ImportedOnlyUnverified,
    /// The evidence window is stale; the state must not roll up green.
    StaleEvidence,
    /// The state cannot be classified; an automatic green roll-up is blocked.
    UnknownRequiresReview,
}

impl StabilityVerdictState {
    /// Every state, in declaration order.
    pub const ALL: [Self; 8] = [
        Self::Stable,
        Self::SuspectedFlaky,
        Self::ConfirmedFlaky,
        Self::Quarantined,
        Self::KnownFailing,
        Self::ImportedOnlyUnverified,
        Self::StaleEvidence,
        Self::UnknownRequiresReview,
    ];

    /// Stable token recorded in schemas, fixtures, and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::SuspectedFlaky => "suspected_flaky",
            Self::ConfirmedFlaky => "confirmed_flaky",
            Self::Quarantined => "quarantined",
            Self::KnownFailing => "known_failing",
            Self::ImportedOnlyUnverified => "imported_only_unverified",
            Self::StaleEvidence => "stale_evidence",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Whether this state is the imported-only state (provider-backed, never a
    /// locally verified pass).
    pub const fn is_imported_only(self) -> bool {
        matches!(self, Self::ImportedOnlyUnverified)
    }

    /// Whether this state may carry [`ReadinessImpactClass::NoImpact`]. Only a
    /// locally verified stable verdict may; every flaky, quarantined, known-failing,
    /// imported-only, stale, or unknown state must visibly fail or narrow readiness.
    pub const fn permits_no_readiness_impact(self) -> bool {
        matches!(self, Self::Stable)
    }

    /// Whether the controlled state is consistent with the evidence window that
    /// produced it, so the badge stays evidence-based rather than asserted.
    pub fn consistent_with_window(self, window: &EvidenceWindow) -> bool {
        match self {
            Self::Stable => window.failed_attempts == 0 && window.inconclusive_attempts == 0,
            Self::SuspectedFlaky => {
                window.failed_attempts >= 1 || window.inconclusive_attempts >= 1
            }
            Self::ConfirmedFlaky => window.passed_attempts >= 1 && window.failed_attempts >= 1,
            Self::KnownFailing => window.failed_attempts >= 1 && window.passed_attempts == 0,
            // The quarantine record, imported provenance, staleness, and review
            // gate govern these states; the window need not constrain them.
            Self::Quarantined
            | Self::ImportedOnlyUnverified
            | Self::StaleEvidence
            | Self::UnknownRequiresReview => true,
        }
    }
}

/// Confidence the stability verdict carries, derived from the evidence window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StabilityConfidenceClass {
    /// High confidence: a wide evidence window backs the verdict.
    High,
    /// Moderate confidence.
    Moderate,
    /// Low confidence.
    Low,
    /// The evidence window is too small to classify with confidence.
    InsufficientEvidence,
}

impl StabilityConfidenceClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::High => "high",
            Self::Moderate => "moderate",
            Self::Low => "low",
            Self::InsufficientEvidence => "insufficient_evidence",
        }
    }

    /// Whether the confidence is strong enough to back a green readiness roll-up.
    /// A stable-green claim requires high or moderate confidence; low or
    /// insufficient confidence may not present as a clean green.
    pub const fn supports_green_rollup(self) -> bool {
        matches!(self, Self::High | Self::Moderate)
    }
}

/// Provenance of the evidence backing a verdict. This is the anchor that keeps an
/// imported / provider-backed verdict from masquerading as a local pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerdictEvidenceProvenance {
    /// Evidence came from local executions.
    LocalAuthoritative,
    /// Evidence came from remote-target executions.
    RemoteAuthoritative,
    /// Evidence came from notebook-kernel executions.
    NotebookAuthoritative,
    /// Imported / provider-backed evidence; read-only and never a local rerun.
    ImportedReadOnly,
}

impl VerdictEvidenceProvenance {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LocalAuthoritative => "local_authoritative",
            Self::RemoteAuthoritative => "remote_authoritative",
            Self::NotebookAuthoritative => "notebook_authoritative",
            Self::ImportedReadOnly => "imported_read_only",
        }
    }

    /// Whether the provenance is imported / read-only.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedReadOnly)
    }
}

/// Release visibility a verdict or quarantine carries. The same vocabulary lets
/// release and support packets see the verdict / quarantine truth the product
/// showed locally.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReleaseVisibilityClass {
    /// Blocks release widening or stable promotion.
    ReleaseBlocking,
    /// Claim text must narrow while the row is unresolved.
    ClaimNarrowingRequired,
    /// A governed waiver covers the row, but it stays visible as debt.
    WaiverLinked,
    /// Informational only after a verified recovery.
    InformationalRecovered,
    /// Visibility cannot be classified; treated as visible debt.
    UnknownRequiresReview,
}

impl ReleaseVisibilityClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ReleaseBlocking => "release_blocking",
            Self::ClaimNarrowingRequired => "claim_narrowing_required",
            Self::WaiverLinked => "waiver_linked",
            Self::InformationalRecovered => "informational_recovered",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Whether the row stays visible as debt on release and support surfaces (the
    /// non-recovered classes). A recovered row is the only one that may drop to an
    /// informational state.
    pub const fn is_visible_debt(self) -> bool {
        !matches!(self, Self::InformationalRecovered)
    }
}

/// Readiness impact a verdict or quarantine asserts. This is the gate that keeps a
/// stale, quarantined, or unknown row from rolling up behind a generic green
/// state: only a stable verdict may carry [`Self::NoImpact`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessImpactClass {
    /// Fails readiness: the row blocks the gate until resolved.
    FailsReadiness,
    /// Narrows the claim: the gate passes only with a narrowed claim.
    NarrowsClaim,
    /// No readiness impact: the row is clean and green.
    NoImpact,
}

impl ReadinessImpactClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::FailsReadiness => "fails_readiness",
            Self::NarrowsClaim => "narrows_claim",
            Self::NoImpact => "no_impact",
        }
    }

    /// Whether the impact blocks the readiness gate outright.
    pub const fn blocks_readiness(self) -> bool {
        matches!(self, Self::FailsReadiness)
    }
}

/// Kind of treatment a [`QuarantineRecord`] applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineTreatmentKind {
    /// A mute that suppresses noise but stays release-visible as debt.
    Mute,
    /// A quarantine that narrows execution or result counting.
    Quarantine,
}

impl QuarantineTreatmentKind {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Mute => "mute",
            Self::Quarantine => "quarantine",
        }
    }
}

/// Lifecycle state of a [`QuarantineRecord`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineState {
    /// Active and inside its expiry window.
    Active,
    /// Expired; the affected scope is reopened for owner review.
    ExpiredReopened,
    /// Resolved and retained as history.
    Resolved,
    /// Renewed under a new owner or expiry.
    Renewed,
}

impl QuarantineState {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::ExpiredReopened => "expired_reopened",
            Self::Resolved => "resolved",
            Self::Renewed => "renewed",
        }
    }

    /// Whether a record in this state still suppresses or narrows live execution
    /// (so it must stay release-visible and readiness-impacting).
    pub const fn is_suppressing(self) -> bool {
        matches!(self, Self::Active | Self::ExpiredReopened | Self::Renewed)
    }
}

/// Reason a quarantine or mute exists.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuarantineReason {
    /// Recent intermittent behavior is suspected.
    SuspectedFlaky,
    /// Comparable attempts reproduced flaky behavior.
    ReproducedFlaky,
    /// The row is known failing and under investigation.
    KnownFailing,
    /// The row depends on a target-environment condition.
    EnvironmentDependent,
    /// Imported provider evidence is incomparable locally.
    ImportedIncomparable,
    /// A local mute suppresses noise but does not hide debt.
    ManualNoiseReduction,
    /// Policy restricts execution or result publication.
    PolicyRestriction,
    /// The reason cannot be classified.
    UnknownRequiresReview,
}

impl QuarantineReason {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SuspectedFlaky => "suspected_flaky",
            Self::ReproducedFlaky => "reproduced_flaky",
            Self::KnownFailing => "known_failing",
            Self::EnvironmentDependent => "environment_dependent",
            Self::ImportedIncomparable => "imported_incomparable",
            Self::ManualNoiseReduction => "manual_noise_reduction",
            Self::PolicyRestriction => "policy_restriction",
            Self::UnknownRequiresReview => "unknown_requires_review",
        }
    }

    /// Whether the reason is an imported / provider-incomparable one.
    pub const fn is_imported(self) -> bool {
        matches!(self, Self::ImportedIncomparable)
    }
}

/// Condition that must be met to restore (lift) a quarantine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreConditionClass {
    /// A stable evidence window of consecutive passes is required.
    StableWindowRequired,
    /// The owner must sign off on lifting the quarantine.
    OwnerSignOff,
    /// A verified fix must be linked.
    FixVerified,
    /// Renewal is required before continued suppression.
    RenewalRequired,
    /// Manual review only; no automatic restore.
    ManualReviewOnly,
}

impl RestoreConditionClass {
    /// Stable token recorded in the packet.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::StableWindowRequired => "stable_window_required",
            Self::OwnerSignOff => "owner_sign_off",
            Self::FixVerified => "fix_verified",
            Self::RenewalRequired => "renewal_required",
            Self::ManualReviewOnly => "manual_review_only",
        }
    }
}

/// One durable subject (test item or family) a verdict or quarantine addresses,
/// keyed by a durable node id and a non-display fingerprint rather than a label.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerdictSubject {
    /// Durable node id of the subject.
    pub subject_id: String,
    /// Node kind, reusing the frozen durable-discovery vocabulary so a
    /// parameterized template never collapses into a concrete invocation.
    pub node_kind: DurableTestNodeKind,
    /// Non-display fingerprint token. Must differ from
    /// [`subject_id`](VerdictSubject::subject_id) so neither a label nor a bare id
    /// stands in for the durable fingerprint.
    pub subject_fingerprint_token: String,
    /// Identity stability, reusing the frozen identity vocabulary.
    pub identity_class: TestItemIdentityClass,
}

impl VerdictSubject {
    /// Whether this subject is imported / provider-owned and read-only.
    pub fn is_imported(&self) -> bool {
        self.identity_class == TestItemIdentityClass::ImportedReadOnly
    }

    /// Whether the fingerprint is a real non-display basis distinct from the id.
    pub fn fingerprint_independent_of_id(&self) -> bool {
        let token = self.subject_fingerprint_token.trim();
        !token.is_empty() && token != self.subject_id.trim()
    }

    /// Whether the subject carries the durable identity a reopen needs.
    pub fn is_valid(&self) -> bool {
        !self.subject_id.trim().is_empty()
            && self.fingerprint_independent_of_id()
            && self.identity_class != TestItemIdentityClass::DisplayTextOnlyDenied
    }
}

/// The visible evidence window backing a stability verdict. This is what turns a
/// flaky badge from an opaque boolean into an inspectable, evidence-based state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceWindow {
    /// Total attempts observed in the window.
    pub observed_attempts: u32,
    /// Attempts that passed.
    pub passed_attempts: u32,
    /// Attempts that failed.
    pub failed_attempts: u32,
    /// Attempts that were inconclusive (errored, imported, or unknown).
    pub inconclusive_attempts: u32,
    /// Opaque ref of the first attempt in the window.
    pub first_attempt_ref: String,
    /// Opaque ref of the latest attempt in the window.
    pub latest_attempt_ref: String,
    /// Attempt refs contributing to the window, in observation order.
    pub evidence_attempt_refs: Vec<String>,
    /// Window-open timestamp.
    pub window_opened_at: String,
    /// Window-close (latest observation) timestamp.
    pub window_closed_at: String,
}

impl EvidenceWindow {
    /// Whether the counts sum to the observed total.
    pub fn counts_sum(&self) -> bool {
        self.passed_attempts
            .checked_add(self.failed_attempts)
            .and_then(|partial| partial.checked_add(self.inconclusive_attempts))
            == Some(self.observed_attempts)
    }

    /// Whether the window carries a non-empty, well-formed evidence set.
    pub fn is_valid(&self) -> bool {
        self.observed_attempts >= 1
            && self.counts_sum()
            && self.evidence_attempt_refs.len() == self.observed_attempts as usize
            && self
                .evidence_attempt_refs
                .iter()
                .all(|r| !r.trim().is_empty())
            && !self.first_attempt_ref.trim().is_empty()
            && !self.latest_attempt_ref.trim().is_empty()
            && !self.window_opened_at.trim().is_empty()
            && !self.window_closed_at.trim().is_empty()
            && self.window_opened_at.as_str() <= self.window_closed_at.as_str()
    }
}

/// A governed, evidence-based stability verdict for one durable subject.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilityVerdictRecord {
    /// Stable verdict id.
    pub verdict_id: String,
    /// Durable subject the verdict addresses.
    pub subject: VerdictSubject,
    /// Session this verdict was last evaluated against (reconstructable elsewhere).
    pub session_ref: String,
    /// Controlled badge state.
    pub state: StabilityVerdictState,
    /// Confidence derived from the evidence window.
    pub confidence: StabilityConfidenceClass,
    /// Visible evidence window backing the state.
    pub evidence_window: EvidenceWindow,
    /// Provenance of the evidence (keeps imported from reading as local).
    pub evidence_provenance: VerdictEvidenceProvenance,
    /// Owner responsible for the verdict.
    pub owner_ref: String,
    /// Whether the verdict is imported / provider-backed only.
    pub imported: bool,
    /// Origin provider ref, present iff the verdict is imported.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_provider_ref: Option<String>,
    /// Quarantine record this verdict is bound to, present iff quarantined.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quarantine_ref: Option<String>,
    /// Release visibility class.
    pub release_visibility: ReleaseVisibilityClass,
    /// Readiness impact asserted by the verdict.
    pub readiness_impact: ReadinessImpactClass,
    /// Evidence packet refs backing the verdict.
    pub evidence_refs: Vec<String>,
    /// Export-safe verdict summary.
    pub support_summary: String,
}

impl StabilityVerdictRecord {
    /// Whether the verdict's imported markers all agree, so an imported verdict
    /// never reads as a locally verified pass and a local verdict never carries
    /// imported markers.
    pub fn imported_markers_consistent(&self) -> bool {
        if self.imported {
            self.state.is_imported_only()
                && self.evidence_provenance.is_imported()
                && self.origin_provider_ref.is_some()
        } else {
            !self.state.is_imported_only()
                && !self.evidence_provenance.is_imported()
                && self.origin_provider_ref.is_none()
        }
    }

    /// Whether the quarantine binding agrees with the state: a [`Quarantined`]
    /// verdict binds a quarantine ref, and a non-quarantined verdict does not.
    ///
    /// [`Quarantined`]: StabilityVerdictState::Quarantined
    pub fn quarantine_binding_consistent(&self) -> bool {
        (self.state == StabilityVerdictState::Quarantined) == self.quarantine_ref.is_some()
    }

    /// Whether the readiness impact respects the no-green-over-stale rule: only a
    /// stable verdict may carry [`ReadinessImpactClass::NoImpact`], and a green
    /// roll-up needs strong confidence and a clean window.
    pub fn readiness_impact_consistent(&self) -> bool {
        if self.readiness_impact == ReadinessImpactClass::NoImpact {
            self.state.permits_no_readiness_impact()
                && self.confidence.supports_green_rollup()
                && self.release_visibility == ReleaseVisibilityClass::InformationalRecovered
        } else {
            // Any non-green readiness impact must keep the row visible as debt.
            self.release_visibility.is_visible_debt()
        }
    }

    /// Whether every field required to record this verdict is present and its
    /// invariants hold.
    pub fn is_valid(&self) -> bool {
        !self.verdict_id.trim().is_empty()
            && self.subject.is_valid()
            && !self.session_ref.trim().is_empty()
            && !self.owner_ref.trim().is_empty()
            && self.evidence_window.is_valid()
            && self.state.consistent_with_window(&self.evidence_window)
            && self.imported_markers_consistent()
            && self.quarantine_binding_consistent()
            && self.readiness_impact_consistent()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.support_summary.trim().is_empty()
            && self
                .origin_provider_ref
                .as_ref()
                .map_or(true, |r| !r.trim().is_empty())
            && self
                .quarantine_ref
                .as_ref()
                .map_or(true, |r| !r.trim().is_empty())
    }
}

/// A governed quarantine (or mute) record tied to a stability verdict, an owner,
/// an expiry, a restore condition, and a release-visibility class.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuarantineRecord {
    /// Stable quarantine id.
    pub quarantine_id: String,
    /// Durable subject the record covers.
    pub subject: VerdictSubject,
    /// Stability verdict this record derives from.
    pub verdict_ref: String,
    /// Treatment kind.
    pub treatment_kind: QuarantineTreatmentKind,
    /// Lifecycle state.
    pub state: QuarantineState,
    /// Reason for the treatment.
    pub reason: QuarantineReason,
    /// Owner responsible for the record.
    pub owner_ref: String,
    /// Record creation timestamp.
    pub created_at: String,
    /// Expiry timestamp.
    pub expires_at: String,
    /// Condition that must be met to restore the scope.
    pub restore_condition: RestoreConditionClass,
    /// Release visibility class.
    pub release_visibility: ReleaseVisibilityClass,
    /// Readiness impact asserted by the record.
    pub readiness_impact: ReadinessImpactClass,
    /// Whether the record is imported / provider-incomparable.
    pub imported: bool,
    /// Attempt ref reopened when the record expired.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reopened_attempt_ref: Option<String>,
    /// Evidence packet refs supporting the treatment.
    pub evidence_refs: Vec<String>,
    /// Export-safe record summary.
    pub support_summary: String,
}

impl QuarantineRecord {
    /// Whether the record's imported markers agree.
    pub fn imported_markers_consistent(&self) -> bool {
        self.imported == self.reason.is_imported() && self.imported == self.subject.is_imported()
    }

    /// Whether an expired-reopened record carries the markers that keep it from
    /// silently persisting: a reopened attempt ref, a release-blocking visibility,
    /// and a readiness-failing impact.
    pub fn expiry_markers_consistent(&self) -> bool {
        if self.state == QuarantineState::ExpiredReopened {
            self.reopened_attempt_ref.is_some()
                && self.release_visibility == ReleaseVisibilityClass::ReleaseBlocking
                && self.readiness_impact == ReadinessImpactClass::FailsReadiness
        } else {
            true
        }
    }

    /// Whether a suppressing record stays visible as debt and impacts readiness —
    /// never collapsing into silent local muting behind a green state.
    pub fn suppression_visible(&self) -> bool {
        if self.state.is_suppressing() {
            self.release_visibility.is_visible_debt()
                && self.readiness_impact != ReadinessImpactClass::NoImpact
        } else {
            true
        }
    }

    /// Whether the expiry timestamp is strictly after creation.
    pub fn expiry_after_creation(&self) -> bool {
        !self.created_at.trim().is_empty()
            && !self.expires_at.trim().is_empty()
            && self.created_at.as_str() < self.expires_at.as_str()
    }

    /// Returns this record evaluated at `now`: an active record whose expiry has
    /// passed flips to [`QuarantineState::ExpiredReopened`], reopening its scope so
    /// it fails readiness instead of silently persisting as local state.
    pub fn evaluated_at(
        &self,
        now: &str,
        reopened_attempt_ref: Option<String>,
    ) -> QuarantineRecord {
        if self.state == QuarantineState::Active && self.expires_at.as_str() <= now {
            let mut reopened = self.clone();
            reopened.state = QuarantineState::ExpiredReopened;
            reopened.release_visibility = ReleaseVisibilityClass::ReleaseBlocking;
            reopened.readiness_impact = ReadinessImpactClass::FailsReadiness;
            reopened.reopened_attempt_ref = reopened_attempt_ref;
            reopened.support_summary = format!(
                "Expired {} reopened for owner review.",
                reopened.treatment_kind.as_str()
            );
            reopened
        } else {
            self.clone()
        }
    }

    /// Whether every field required to record this quarantine is present and its
    /// invariants hold.
    pub fn is_valid(&self) -> bool {
        !self.quarantine_id.trim().is_empty()
            && self.subject.is_valid()
            && !self.verdict_ref.trim().is_empty()
            && !self.owner_ref.trim().is_empty()
            && self.expiry_after_creation()
            && self.imported_markers_consistent()
            && self.expiry_markers_consistent()
            && self.suppression_visible()
            && !self.evidence_refs.is_empty()
            && self.evidence_refs.iter().all(|r| !r.trim().is_empty())
            && !self.support_summary.trim().is_empty()
            && self
                .reopened_attempt_ref
                .as_ref()
                .map_or(true, |r| !r.trim().is_empty())
    }
}

/// Guardrail invariants block.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilityGuardrails {
    /// Parameterized templates stay distinct from their concrete invocations.
    pub templates_distinct_from_invocations: bool,
    /// Imported / provider-backed verdicts never read as locally verified passes.
    pub imported_never_reads_as_local: bool,
    /// Flaky badges use controlled states with visible evidence windows.
    pub flaky_badges_evidence_based: bool,
    /// Quarantines stay visible, filterable, countable, and exportable.
    pub quarantines_visible_and_countable: bool,
    /// Expired or stale rows fail readiness instead of silently persisting.
    pub expiry_fails_readiness: bool,
    /// No stale / quarantined row hides behind a generic green state.
    pub no_green_over_stale_or_quarantine: bool,
}

impl StabilityGuardrails {
    /// Whether every guardrail invariant holds.
    pub fn all_hold(&self) -> bool {
        self.templates_distinct_from_invocations
            && self.imported_never_reads_as_local
            && self.flaky_badges_evidence_based
            && self.quarantines_visible_and_countable
            && self.expiry_fails_readiness
            && self.no_green_over_stale_or_quarantine
    }
}

/// Consumer projection block: the surfaces that read the same verdict / quarantine
/// truth the product showed locally, without re-deriving test state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilityConsumerProjection {
    /// Flaky-state badges normalize onto these verdict records.
    pub flaky_badges_normalized: bool,
    /// Quarantine UI normalizes onto these quarantine records.
    pub quarantine_ui_normalized: bool,
    /// Imported-run joins normalize onto these records without reading as local.
    pub imported_join_normalized: bool,
    /// Release and support exports read the same records.
    pub release_support_export_normalized: bool,
    /// The readiness gate reads this packet instead of scraping UI text.
    pub readiness_gate_reads_packet: bool,
}

impl StabilityConsumerProjection {
    /// Whether every consumer-projection invariant holds.
    pub fn all_hold(&self) -> bool {
        self.flaky_badges_normalized
            && self.quarantine_ui_normalized
            && self.imported_join_normalized
            && self.release_support_export_normalized
            && self.readiness_gate_reads_packet
    }
}

/// Constructor input for [`StabilityVerdictQuarantinePacket::new`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabilityVerdictQuarantinePacketInput {
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Stability verdicts across states and provenance.
    pub verdicts: Vec<StabilityVerdictRecord>,
    /// Quarantine records referencing the verdicts.
    pub quarantines: Vec<QuarantineRecord>,
    /// Guardrail invariants block.
    pub guardrails: StabilityGuardrails,
    /// Consumer projection block.
    pub consumer_projection: StabilityConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

/// Export-safe stability-verdict / quarantine packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StabilityVerdictQuarantinePacket {
    /// Record kind; must equal [`STABILITY_VERDICT_QUARANTINE_RECORD_KIND`].
    pub record_kind: String,
    /// Schema version; must equal [`STABILITY_VERDICT_QUARANTINE_SCHEMA_VERSION`].
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Human-readable packet label.
    pub label: String,
    /// Stability verdicts across states and provenance.
    pub verdicts: Vec<StabilityVerdictRecord>,
    /// Quarantine records referencing the verdicts.
    pub quarantines: Vec<QuarantineRecord>,
    /// Guardrail invariants block.
    pub guardrails: StabilityGuardrails,
    /// Consumer projection block.
    pub consumer_projection: StabilityConsumerProjection,
    /// Canonical source contract refs.
    pub source_contract_refs: Vec<String>,
    /// Packet redaction class token.
    pub redaction_class_token: String,
    /// Packet mint timestamp.
    pub minted_at: String,
}

impl StabilityVerdictQuarantinePacket {
    /// Builds a stability-verdict / quarantine packet.
    pub fn new(input: StabilityVerdictQuarantinePacketInput) -> Self {
        Self {
            record_kind: STABILITY_VERDICT_QUARANTINE_RECORD_KIND.to_owned(),
            schema_version: STABILITY_VERDICT_QUARANTINE_SCHEMA_VERSION,
            packet_id: input.packet_id,
            label: input.label,
            verdicts: input.verdicts,
            quarantines: input.quarantines,
            guardrails: input.guardrails,
            consumer_projection: input.consumer_projection,
            source_contract_refs: input.source_contract_refs,
            redaction_class_token: input.redaction_class_token,
            minted_at: input.minted_at,
        }
    }

    /// Verdict states represented by some verdict in this packet.
    pub fn represented_verdict_states(&self) -> BTreeSet<StabilityVerdictState> {
        self.verdicts.iter().map(|v| v.state).collect()
    }

    /// Subject node kinds represented across verdicts and quarantines.
    pub fn represented_subject_kinds(&self) -> BTreeSet<DurableTestNodeKind> {
        self.verdicts
            .iter()
            .map(|v| v.subject.node_kind)
            .chain(self.quarantines.iter().map(|q| q.subject.node_kind))
            .collect()
    }

    /// Resolves a verdict by its id.
    pub fn verdict(&self, verdict_id: &str) -> Option<&StabilityVerdictRecord> {
        self.verdicts.iter().find(|v| v.verdict_id == verdict_id)
    }

    /// Resolves a quarantine by its id.
    pub fn quarantine(&self, quarantine_id: &str) -> Option<&QuarantineRecord> {
        self.quarantines
            .iter()
            .find(|q| q.quarantine_id == quarantine_id)
    }

    /// Count of imported / provider-backed verdicts.
    pub fn imported_verdict_count(&self) -> usize {
        self.verdicts.iter().filter(|v| v.imported).count()
    }

    /// Count of quarantine records that still suppress execution.
    pub fn active_quarantine_count(&self) -> usize {
        self.quarantines
            .iter()
            .filter(|q| q.state == QuarantineState::Active)
            .count()
    }

    /// Count of quarantine records reopened by expiry.
    pub fn expired_reopened_count(&self) -> usize {
        self.quarantines
            .iter()
            .filter(|q| q.state == QuarantineState::ExpiredReopened)
            .count()
    }

    /// Count of rows (verdicts or quarantines) that fail readiness.
    pub fn fails_readiness_count(&self) -> usize {
        self.verdicts
            .iter()
            .filter(|v| v.readiness_impact.blocks_readiness())
            .count()
            + self
                .quarantines
                .iter()
                .filter(|q| q.readiness_impact.blocks_readiness())
                .count()
    }

    /// Whether the readiness gate is blocked by any verdict or quarantine.
    pub fn readiness_blocked(&self) -> bool {
        self.fails_readiness_count() > 0
    }

    /// Validates the stability-verdict / quarantine invariants.
    pub fn validate(&self) -> Vec<StabilityVerdictQuarantineViolation> {
        let mut violations = Vec::new();

        if self.record_kind != STABILITY_VERDICT_QUARANTINE_RECORD_KIND {
            violations.push(StabilityVerdictQuarantineViolation::WrongRecordKind);
        }
        if self.schema_version != STABILITY_VERDICT_QUARANTINE_SCHEMA_VERSION {
            violations.push(StabilityVerdictQuarantineViolation::WrongSchemaVersion);
        }
        if self.packet_id.trim().is_empty()
            || self.label.trim().is_empty()
            || self.redaction_class_token.trim().is_empty()
            || self.minted_at.trim().is_empty()
        {
            violations.push(StabilityVerdictQuarantineViolation::MissingIdentity);
        }

        validate_source_contracts(self, &mut violations);
        validate_coverage(self, &mut violations);
        validate_verdicts(self, &mut violations);
        validate_quarantines(self, &mut violations);

        if !self.guardrails.all_hold() {
            violations.push(StabilityVerdictQuarantineViolation::GuardrailsIncomplete);
        }
        if !self.consumer_projection.all_hold() {
            violations.push(StabilityVerdictQuarantineViolation::ConsumerProjectionIncomplete);
        }

        if json_contains_forbidden_boundary_material(
            &serde_json::to_value(self).expect("stability verdict quarantine packet serializes"),
        ) {
            violations.push(StabilityVerdictQuarantineViolation::RawBoundaryMaterialInExport);
        }

        violations
    }

    /// Deterministic export-safe JSON.
    ///
    /// # Panics
    ///
    /// Panics only if serializing this metadata-only packet fails.
    pub fn export_safe_json(&self) -> String {
        serde_json::to_string_pretty(self).expect("stability verdict quarantine packet serializes")
    }

    /// Deterministic Markdown summary for support, docs, or review handoff.
    pub fn render_markdown_summary(&self) -> String {
        let mut out = String::new();
        out.push_str("# M5 Stability Verdicts And Quarantine Records\n\n");
        out.push_str(&format!("- Packet: `{}`\n", self.packet_id));
        out.push_str(&format!("- Label: `{}`\n", self.label));
        out.push_str(&format!(
            "- Verdicts: {} ({} imported-only) across {} / {} states\n",
            self.verdicts.len(),
            self.imported_verdict_count(),
            self.represented_verdict_states().len(),
            StabilityVerdictState::ALL.len()
        ));
        out.push_str(&format!(
            "- Quarantines: {} ({} active, {} expired-reopened)\n",
            self.quarantines.len(),
            self.active_quarantine_count(),
            self.expired_reopened_count()
        ));
        out.push_str(&format!(
            "- Readiness: {} row(s) fail readiness ({})\n",
            self.fails_readiness_count(),
            if self.readiness_blocked() {
                "gate blocked"
            } else {
                "gate clear"
            }
        ));
        out.push_str("\n## Stability verdicts\n\n");
        for verdict in &self.verdicts {
            out.push_str(&format!(
                "- **{}** [{}] confidence `{}` → readiness `{}`, release `{}`\n",
                verdict.verdict_id,
                verdict.state.as_str(),
                verdict.confidence.as_str(),
                verdict.readiness_impact.as_str(),
                verdict.release_visibility.as_str()
            ));
            out.push_str(&format!(
                "  - subject `{}` ({}), provenance `{}`\n",
                verdict.subject.subject_id,
                verdict.subject.node_kind.as_str(),
                verdict.evidence_provenance.as_str()
            ));
            out.push_str(&format!(
                "  - window: {} observed ({} passed / {} failed / {} inconclusive)\n",
                verdict.evidence_window.observed_attempts,
                verdict.evidence_window.passed_attempts,
                verdict.evidence_window.failed_attempts,
                verdict.evidence_window.inconclusive_attempts
            ));
            if let Some(quarantine_ref) = &verdict.quarantine_ref {
                out.push_str(&format!("  - quarantine `{quarantine_ref}`\n"));
            }
        }
        out.push_str("\n## Quarantine records\n\n");
        for quarantine in &self.quarantines {
            out.push_str(&format!(
                "- **{}** [{} / {}] reason `{}` owner `{}`\n",
                quarantine.quarantine_id,
                quarantine.treatment_kind.as_str(),
                quarantine.state.as_str(),
                quarantine.reason.as_str(),
                quarantine.owner_ref
            ));
            out.push_str(&format!(
                "  - expires `{}` restore `{}` → readiness `{}`, release `{}`\n",
                quarantine.expires_at,
                quarantine.restore_condition.as_str(),
                quarantine.readiness_impact.as_str(),
                quarantine.release_visibility.as_str()
            ));
            if let Some(reopened) = &quarantine.reopened_attempt_ref {
                out.push_str(&format!("  - reopened attempt `{reopened}`\n"));
            }
        }
        out
    }
}

/// Errors emitted when reading the checked-in packet export.
#[derive(Debug)]
pub enum StabilityVerdictQuarantineArtifactError {
    /// Support export failed to parse.
    SupportExport(serde_json::Error),
    /// Support export failed validation.
    Validation(Vec<StabilityVerdictQuarantineViolation>),
}

impl fmt::Display for StabilityVerdictQuarantineArtifactError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SupportExport(error) => {
                write!(
                    formatter,
                    "stability verdict quarantine export parse failed: {error}"
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
                    "stability verdict quarantine export failed validation: {tokens}"
                )
            }
        }
    }
}

impl Error for StabilityVerdictQuarantineArtifactError {}

/// Validation failures emitted by [`StabilityVerdictQuarantinePacket::validate`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StabilityVerdictQuarantineViolation {
    /// Packet record kind is wrong.
    WrongRecordKind,
    /// Packet schema version is wrong.
    WrongSchemaVersion,
    /// Required identity field is missing.
    MissingIdentity,
    /// Required base source contract refs are incomplete.
    MissingSourceContracts,
    /// A parameterized template was collapsed into its concrete invocation.
    TemplateCollapsedWithInvocation,
    /// No verdict demonstrates a locally verified stable state.
    StableVerdictCaseMissing,
    /// No verdict demonstrates a controlled flaky badge state.
    FlakyVerdictCaseMissing,
    /// No verdict demonstrates a quarantined state.
    QuarantinedVerdictCaseMissing,
    /// No verdict demonstrates an imported-only verdict held read-only.
    ImportedVerdictCaseMissing,
    /// No quarantine demonstrates an expired-reopened readiness failure.
    ExpiredQuarantineCaseMissing,
    /// A verdict is incomplete.
    VerdictInvalid,
    /// A verdict's fingerprint stands in for its bare id.
    FingerprintSubstitutesIdentity,
    /// An imported verdict reads as a locally verified pass (or a local verdict
    /// carries imported markers).
    ImportedVerdictReadsAsLocal,
    /// A verdict's controlled state disagrees with its evidence window.
    VerdictStateWindowMismatch,
    /// A stale, quarantined, or unknown verdict rolls up behind a green state.
    GreenOverStaleOrQuarantine,
    /// A quarantined verdict does not bind a resolvable quarantine record.
    QuarantineBindingUnresolved,
    /// A quarantine is incomplete.
    QuarantineInvalid,
    /// A quarantine references a verdict absent from the packet.
    QuarantineVerdictUnresolved,
    /// A quarantine's subject disagrees with its bound verdict's subject.
    QuarantineSubjectMismatch,
    /// A suppressing quarantine collapsed into silent local muting.
    QuarantineSilentlyMuted,
    /// An expired quarantine did not reopen its scope / fail readiness.
    ExpiredQuarantinePersistsSilently,
    /// A verdict or quarantine lacks evidence refs.
    EvidenceMissing,
    /// Guardrail block does not satisfy required invariants.
    GuardrailsIncomplete,
    /// Consumer projection does not satisfy required invariants.
    ConsumerProjectionIncomplete,
    /// Export contains raw boundary material.
    RawBoundaryMaterialInExport,
}

impl StabilityVerdictQuarantineViolation {
    /// Stable token used in tests and support exports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongRecordKind => "wrong_record_kind",
            Self::WrongSchemaVersion => "wrong_schema_version",
            Self::MissingIdentity => "missing_identity",
            Self::MissingSourceContracts => "missing_source_contracts",
            Self::TemplateCollapsedWithInvocation => "template_collapsed_with_invocation",
            Self::StableVerdictCaseMissing => "stable_verdict_case_missing",
            Self::FlakyVerdictCaseMissing => "flaky_verdict_case_missing",
            Self::QuarantinedVerdictCaseMissing => "quarantined_verdict_case_missing",
            Self::ImportedVerdictCaseMissing => "imported_verdict_case_missing",
            Self::ExpiredQuarantineCaseMissing => "expired_quarantine_case_missing",
            Self::VerdictInvalid => "verdict_invalid",
            Self::FingerprintSubstitutesIdentity => "fingerprint_substitutes_identity",
            Self::ImportedVerdictReadsAsLocal => "imported_verdict_reads_as_local",
            Self::VerdictStateWindowMismatch => "verdict_state_window_mismatch",
            Self::GreenOverStaleOrQuarantine => "green_over_stale_or_quarantine",
            Self::QuarantineBindingUnresolved => "quarantine_binding_unresolved",
            Self::QuarantineInvalid => "quarantine_invalid",
            Self::QuarantineVerdictUnresolved => "quarantine_verdict_unresolved",
            Self::QuarantineSubjectMismatch => "quarantine_subject_mismatch",
            Self::QuarantineSilentlyMuted => "quarantine_silently_muted",
            Self::ExpiredQuarantinePersistsSilently => "expired_quarantine_persists_silently",
            Self::EvidenceMissing => "evidence_missing",
            Self::GuardrailsIncomplete => "guardrails_incomplete",
            Self::ConsumerProjectionIncomplete => "consumer_projection_incomplete",
            Self::RawBoundaryMaterialInExport => "raw_boundary_material_in_export",
        }
    }
}

/// Reads and validates the checked-in stable packet export.
///
/// # Errors
///
/// Returns an artifact error if the export cannot parse or fails validation.
pub fn current_stability_verdict_quarantine_export(
) -> Result<StabilityVerdictQuarantinePacket, StabilityVerdictQuarantineArtifactError> {
    let packet: StabilityVerdictQuarantinePacket = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../artifacts/testing/m5/stability-verdicts-quarantines-and-release-visibility/support_export.json"
    )))
    .map_err(StabilityVerdictQuarantineArtifactError::SupportExport)?;
    let violations = packet.validate();
    if violations.is_empty() {
        Ok(packet)
    } else {
        Err(StabilityVerdictQuarantineArtifactError::Validation(
            violations,
        ))
    }
}

fn validate_source_contracts(
    packet: &StabilityVerdictQuarantinePacket,
    violations: &mut Vec<StabilityVerdictQuarantineViolation>,
) {
    let refs: BTreeSet<&str> = packet
        .source_contract_refs
        .iter()
        .map(String::as_str)
        .collect();
    for required in [
        STABILITY_VERDICT_QUARANTINE_SCHEMA_REF,
        STABILITY_VERDICT_QUARANTINE_DOC_REF,
        STABILITY_VERDICT_QUARANTINE_ARTIFACT_REF,
    ] {
        if !refs.contains(required) {
            violations.push(StabilityVerdictQuarantineViolation::MissingSourceContracts);
            break;
        }
    }
}

fn validate_coverage(
    packet: &StabilityVerdictQuarantinePacket,
    violations: &mut Vec<StabilityVerdictQuarantineViolation>,
) {
    let states = packet.represented_verdict_states();

    if !states.contains(&StabilityVerdictState::Stable) {
        violations.push(StabilityVerdictQuarantineViolation::StableVerdictCaseMissing);
    }
    if !(states.contains(&StabilityVerdictState::SuspectedFlaky)
        || states.contains(&StabilityVerdictState::ConfirmedFlaky))
    {
        violations.push(StabilityVerdictQuarantineViolation::FlakyVerdictCaseMissing);
    }
    if !states.contains(&StabilityVerdictState::Quarantined) {
        violations.push(StabilityVerdictQuarantineViolation::QuarantinedVerdictCaseMissing);
    }
    if !states.contains(&StabilityVerdictState::ImportedOnlyUnverified) {
        violations.push(StabilityVerdictQuarantineViolation::ImportedVerdictCaseMissing);
    }

    let subject_kinds = packet.represented_subject_kinds();
    if !(subject_kinds.contains(&DurableTestNodeKind::ParameterizedTemplate)
        && subject_kinds.contains(&DurableTestNodeKind::ConcreteInvocation))
    {
        violations.push(StabilityVerdictQuarantineViolation::TemplateCollapsedWithInvocation);
    }

    if !packet
        .quarantines
        .iter()
        .any(|q| q.state == QuarantineState::ExpiredReopened)
    {
        violations.push(StabilityVerdictQuarantineViolation::ExpiredQuarantineCaseMissing);
    }
}

fn validate_verdicts(
    packet: &StabilityVerdictQuarantinePacket,
    violations: &mut Vec<StabilityVerdictQuarantineViolation>,
) {
    for verdict in &packet.verdicts {
        if !verdict.is_valid() {
            violations.push(StabilityVerdictQuarantineViolation::VerdictInvalid);
        }
        if !verdict.subject.fingerprint_independent_of_id() {
            violations.push(StabilityVerdictQuarantineViolation::FingerprintSubstitutesIdentity);
        }
        if !verdict.imported_markers_consistent() {
            violations.push(StabilityVerdictQuarantineViolation::ImportedVerdictReadsAsLocal);
        }
        if !verdict
            .state
            .consistent_with_window(&verdict.evidence_window)
        {
            violations.push(StabilityVerdictQuarantineViolation::VerdictStateWindowMismatch);
        }
        if !verdict.readiness_impact_consistent() {
            violations.push(StabilityVerdictQuarantineViolation::GreenOverStaleOrQuarantine);
        }
        if verdict.evidence_refs.is_empty()
            || verdict.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(StabilityVerdictQuarantineViolation::EvidenceMissing);
        }

        // A quarantined verdict must bind a quarantine record present in the packet.
        if verdict.state == StabilityVerdictState::Quarantined {
            let bound = verdict
                .quarantine_ref
                .as_ref()
                .and_then(|id| packet.quarantine(id));
            match bound {
                Some(quarantine) if quarantine.subject == verdict.subject => {}
                Some(_) => {
                    violations.push(StabilityVerdictQuarantineViolation::QuarantineSubjectMismatch);
                }
                None => {
                    violations
                        .push(StabilityVerdictQuarantineViolation::QuarantineBindingUnresolved);
                }
            }
        }
    }
}

fn validate_quarantines(
    packet: &StabilityVerdictQuarantinePacket,
    violations: &mut Vec<StabilityVerdictQuarantineViolation>,
) {
    for quarantine in &packet.quarantines {
        if !quarantine.is_valid() {
            violations.push(StabilityVerdictQuarantineViolation::QuarantineInvalid);
        }
        if !quarantine.suppression_visible() {
            violations.push(StabilityVerdictQuarantineViolation::QuarantineSilentlyMuted);
        }
        if !quarantine.expiry_markers_consistent() {
            violations.push(StabilityVerdictQuarantineViolation::ExpiredQuarantinePersistsSilently);
        }
        if quarantine.evidence_refs.is_empty()
            || quarantine.evidence_refs.iter().any(|r| r.trim().is_empty())
        {
            violations.push(StabilityVerdictQuarantineViolation::EvidenceMissing);
        }

        match packet.verdict(&quarantine.verdict_ref) {
            None => {
                violations.push(StabilityVerdictQuarantineViolation::QuarantineVerdictUnresolved);
            }
            Some(verdict) if verdict.subject != quarantine.subject => {
                violations.push(StabilityVerdictQuarantineViolation::QuarantineSubjectMismatch);
            }
            Some(_) => {}
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
