//! Project Doctor, support-export, and developer diagnostics for reactive state.
//!
//! Reactive-state failures — a consumer pinned to a stale epoch, a derived view
//! coalescing behind its producer, a subscription that needs to resubscribe, a
//! materialized view serving a partial scope, or a derived surface reading a
//! provider-unavailable overlay — must be diagnosable as first-class contract
//! failures, not as generic "UI weirdness" or "cache corruption" anecdotes.
//!
//! This module folds the canonical reactive-state objects this batch froze — the
//! cross-surface subscription envelope, the materialized-view classes, the
//! invalidation vocabulary, and the consumer-side recovery flows — into one
//! Project Doctor and support-export packet. It does **not** invent a second
//! diagnostics-only state model: every freshness, completeness, backpressure,
//! epoch, invalidation-reason, view-class, lag-condition, recovery-strategy, and
//! truth-claim token is the same one the product surfaces show, re-exported from
//! [`aureline_reactive_state`]; and every finding severity, confidence class, and
//! repair-availability class is the same one Project Doctor already uses,
//! re-exported from [`aureline_doctor::probes`].
//!
//! Six diagnostic sections make up the packet:
//!
//! - [`ActiveSubscriptionRow`] — every active `(binding, scope)` subscription,
//!   its authority epoch, the consumer's snapshot epoch, and whether the two
//!   have drifted apart.
//! - [`InvalidationHistoryRow`] — the ordered invalidation history, each entry
//!   naming the exact [`InvalidationReason`] and the epoch transition it drove.
//! - [`StaleMaterializationRow`] — the materialized-view classes currently stale
//!   and the narrow rebuild path for each.
//! - [`SlowConsumerRow`] — the lagging consumers, their backpressure mode, and
//!   the recovery strategy plus safe next step the recovery contract recommends.
//! - [`DoctorProbeRow`] — one Project Doctor probe per diagnosable condition,
//!   each carrying a stable finding code, a [`ReactiveStateReasonCode`], a
//!   severity, a confidence class, a recovery recommendation, and a repair
//!   availability — so support and Doctor flows can name the failure directly.
//! - [`ReactiveStateReasonCode`] — the exact reason codes and the
//!   [`SafeNextStep`] each one recommends when a consumer is stale, coalescing,
//!   resubscribe-required, or reading a provider-unavailable overlay.
//!
//! [`compile_support_export_envelope`] projects the Doctor probes into a
//! metadata-first [`ReactiveDiagnosticsSupportExportEnvelope`] that is reviewable
//! before it leaves the machine: it carries finding identity, reason codes, and
//! recovery recommendations but no raw state payloads, credentials, raw provider
//! responses, raw paths, or raw traces.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/support/reactive_diagnostics.schema.json`](../../../../schemas/support/reactive_diagnostics.schema.json)
//! - [`/docs/support/reactive_diagnostics.md`](../../../../docs/support/reactive_diagnostics.md)
//! - [`/artifacts/support/reactive_diagnostics.json`](../../../../artifacts/support/reactive_diagnostics.json)
//! - [`/artifacts/support/reactive_diagnostics.md`](../../../../artifacts/support/reactive_diagnostics.md)
//! - [`/artifacts/support/reactive_diagnostics_runbook.md`](../../../../artifacts/support/reactive_diagnostics_runbook.md)
//! - [`/fixtures/support/reactive_diagnostics/`](../../../../fixtures/support/reactive_diagnostics/)

use std::collections::BTreeSet;
use std::fmt;

use aureline_doctor::probes::{ConfidenceClass, FindingSeverity, RepairAvailabilityClass};
use aureline_reactive_state::{
    M5ReactiveBackpressureMode as BackpressureMode, M5ReactiveCompleteness as Completeness,
    M5ReactiveFreshness as Freshness, M5ReactiveInvalidationReason as InvalidationReason,
    M5ReactivePersistenceClass as PersistenceClass, M5ReactiveScopeClass as ScopeClass,
    M5ReactiveTruthClaim as TruthClaim, M5ReactiveViewClass as ViewClass,
    ReactiveRecoveryActionPosture as ActionPosture,
    ReactiveRecoveryConsumerSurface as ConsumerSurface,
    ReactiveRecoveryEpochPosture as EpochPosture, ReactiveRecoveryLagCondition as LagCondition,
    ReactiveRecoveryStrategy as RecoveryStrategy,
};
use serde::{Deserialize, Serialize};

/// Schema version stamped onto packets and fixtures.
pub const REACTIVE_DIAGNOSTICS_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const REACTIVE_DIAGNOSTICS_PACKET_RECORD_KIND: &str = "reactive_diagnostics_packet_record";

/// Stable record-kind tag carried by troubleshooting fixtures.
pub const REACTIVE_DIAGNOSTICS_FIXTURE_RECORD_KIND: &str = "reactive_diagnostics_fixture_record";

/// Stable record-kind tag carried by one support-export row.
pub const REACTIVE_DIAGNOSTICS_SUPPORT_EXPORT_ROW_RECORD_KIND: &str =
    "reactive_diagnostics_support_export_row";

/// Stable record-kind tag carried by the support-export envelope.
pub const REACTIVE_DIAGNOSTICS_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND: &str =
    "reactive_diagnostics_support_export_envelope";

/// Repo-relative schema ref.
pub const REACTIVE_DIAGNOSTICS_SCHEMA_REF: &str =
    "schemas/support/reactive_diagnostics.schema.json";

/// Repo-relative reviewer doc ref.
pub const REACTIVE_DIAGNOSTICS_DOC_REF: &str = "docs/support/reactive_diagnostics.md";

/// Repo-relative machine-readable artifact packet.
pub const REACTIVE_DIAGNOSTICS_PACKET_REF: &str = "artifacts/support/reactive_diagnostics.json";

/// Repo-relative reviewer artifact report.
pub const REACTIVE_DIAGNOSTICS_REPORT_REF: &str = "artifacts/support/reactive_diagnostics.md";

/// Repo-relative troubleshooting runbook.
pub const REACTIVE_DIAGNOSTICS_RUNBOOK_REF: &str =
    "artifacts/support/reactive_diagnostics_runbook.md";

/// Repo-relative fixture directory.
pub const REACTIVE_DIAGNOSTICS_FIXTURE_DIR: &str = "fixtures/support/reactive_diagnostics";

/// Repo-relative fixture manifest.
pub const REACTIVE_DIAGNOSTICS_FIXTURE_MANIFEST_REF: &str =
    "fixtures/support/reactive_diagnostics/manifest.yaml";

// ---------------------------------------------------------------------------
// Reason codes and safe next steps.
// ---------------------------------------------------------------------------

/// Exact reason code a reactive-state diagnosis names.
///
/// These are the failure classes the contract makes first-class: a consumer
/// pinned to a stale epoch, a consumer trailing the live epoch via coalesced
/// frames, a subscription that must resubscribe before acting, a derived surface
/// reading a provider-unavailable overlay, an epoch that has drifted from its
/// authority, an invalidation storm saturating a consumer, a bounded delta queue
/// that overflowed, and a partially loaded scope serving a stale view.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReactiveStateReasonCode {
    /// The consumer is pinned to a known-stale epoch behind its authority.
    ConsumerStale,
    /// The consumer trails the live epoch and applies coalesced frames.
    ConsumerCoalescing,
    /// The consumer must run a visible resubscribe before it can act.
    ResubscribeRequired,
    /// The derived surface is reading a provider overlay that is unavailable.
    ProviderOverlayUnavailable,
    /// The consumer's snapshot epoch has drifted behind the authority epoch.
    EpochDrift,
    /// A burst of invalidations is arriving faster than the consumer applies.
    InvalidationStorm,
    /// The bounded delta queue overflowed and dropped intermediate frames.
    BackpressureOverflow,
    /// A partially loaded scope is serving a stale or incomplete view.
    PartialScopeStale,
}

impl ReactiveStateReasonCode {
    /// Returns every reason code in stable diagnosis order.
    pub const fn all() -> [Self; 8] {
        [
            Self::ConsumerStale,
            Self::ConsumerCoalescing,
            Self::ResubscribeRequired,
            Self::ProviderOverlayUnavailable,
            Self::EpochDrift,
            Self::InvalidationStorm,
            Self::BackpressureOverflow,
            Self::PartialScopeStale,
        ]
    }

    /// Returns the four reason codes the spec requires a diagnosis to name
    /// directly: stale, coalescing, resubscribe-required, and
    /// provider-unavailable.
    pub const fn required_named() -> [Self; 4] {
        [
            Self::ConsumerStale,
            Self::ConsumerCoalescing,
            Self::ResubscribeRequired,
            Self::ProviderOverlayUnavailable,
        ]
    }

    /// Returns the stable snake_case token for this reason code.
    pub const fn as_token(self) -> &'static str {
        match self {
            Self::ConsumerStale => "consumer_stale",
            Self::ConsumerCoalescing => "consumer_coalescing",
            Self::ResubscribeRequired => "resubscribe_required",
            Self::ProviderOverlayUnavailable => "provider_overlay_unavailable",
            Self::EpochDrift => "epoch_drift",
            Self::InvalidationStorm => "invalidation_storm",
            Self::BackpressureOverflow => "backpressure_overflow",
            Self::PartialScopeStale => "partial_scope_stale",
        }
    }

    /// Returns the stable dotted finding code used by Project Doctor and support
    /// exports, e.g. `reactive.consumer_stale`.
    pub fn finding_code(self) -> String {
        format!("reactive.{}", self.as_token())
    }

    /// Returns the human-readable diagnosis title shown identically across the
    /// Doctor pane, CLI/headless rows, and support exports.
    pub const fn title(self) -> &'static str {
        match self {
            Self::ConsumerStale => "Consumer reading a stale epoch",
            Self::ConsumerCoalescing => "Consumer trailing the live epoch",
            Self::ResubscribeRequired => "Subscription needs to resubscribe",
            Self::ProviderOverlayUnavailable => "Provider overlay unavailable",
            Self::EpochDrift => "Consumer epoch drifted from authority",
            Self::InvalidationStorm => "Invalidation storm saturating consumer",
            Self::BackpressureOverflow => "Backpressure queue overflowed",
            Self::PartialScopeStale => "Partial scope serving a stale view",
        }
    }

    /// Returns the narrow, safe next step recommended for this reason code.
    pub const fn safe_next_step(self) -> SafeNextStep {
        match self {
            Self::ConsumerStale => SafeNextStep::RequestFreshSnapshot,
            Self::ConsumerCoalescing => SafeNextStep::WaitForCoalescedCatchUp,
            Self::ResubscribeRequired => SafeNextStep::Resubscribe,
            Self::ProviderOverlayUnavailable => SafeNextStep::ReconnectProvider,
            Self::EpochDrift => SafeNextStep::RequestFreshSnapshot,
            Self::InvalidationStorm => SafeNextStep::WaitForCoalescedCatchUp,
            Self::BackpressureOverflow => SafeNextStep::RequestFreshSnapshot,
            Self::PartialScopeStale => SafeNextStep::HoldLastKnownReadOnly,
        }
    }

    /// Returns the Project Doctor severity for this reason code.
    pub const fn severity(self) -> FindingSeverity {
        match self {
            // A consumer reading a stale epoch, a required resubscribe, and an
            // unavailable provider overlay block exact-truth actions until the
            // narrow recovery runs.
            Self::ConsumerStale | Self::ResubscribeRequired | Self::ProviderOverlayUnavailable => {
                FindingSeverity::Blocking
            }
            // The remaining conditions are degraded-but-usable: the surface
            // stays visible as last-known truth while it catches up.
            Self::ConsumerCoalescing
            | Self::EpochDrift
            | Self::InvalidationStorm
            | Self::BackpressureOverflow
            | Self::PartialScopeStale => FindingSeverity::Degraded,
        }
    }

    /// Returns the repair availability for this reason code. Every reactive
    /// recovery path is a reviewed, bounded, in-product action.
    pub const fn repair_availability(self) -> RepairAvailabilityClass {
        RepairAvailabilityClass::ReviewedRepairAvailable
    }
}

impl fmt::Display for ReactiveStateReasonCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_token())
    }
}

/// Narrow, safe next step a stale, coalescing, resubscribe-required, or
/// provider-unavailable consumer should take. Each step is a bounded recovery
/// action, never a silent retry or an exact-truth action while behind.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafeNextStep {
    /// Wait while buffered deltas coalesce into one consistent frame.
    WaitForCoalescedCatchUp,
    /// Drop the lossy stream and request a fresh consistent snapshot.
    RequestFreshSnapshot,
    /// Tear down and re-establish the subscription from a new snapshot epoch.
    Resubscribe,
    /// Re-establish the provider overlay; stay on last-known read-only until it
    /// returns.
    ReconnectProvider,
    /// Keep the last consistent projection visible read-only and do not act on
    /// it as exact current truth.
    HoldLastKnownReadOnly,
    /// Open the subscription inspector to see authority, epoch, and freshness.
    OpenSubscriptionInspector,
    /// Export the reactive diagnostics packet for support review.
    ExportReactiveDiagnostics,
}

impl SafeNextStep {
    /// Returns the stable snake_case token for this next step.
    pub const fn as_token(self) -> &'static str {
        match self {
            Self::WaitForCoalescedCatchUp => "wait_for_coalesced_catch_up",
            Self::RequestFreshSnapshot => "request_fresh_snapshot",
            Self::Resubscribe => "resubscribe",
            Self::ReconnectProvider => "reconnect_provider",
            Self::HoldLastKnownReadOnly => "hold_last_known_read_only",
            Self::OpenSubscriptionInspector => "open_subscription_inspector",
            Self::ExportReactiveDiagnostics => "export_reactive_diagnostics",
        }
    }

    /// Returns the support-safe instruction shown for this next step.
    pub const fn instruction(self) -> &'static str {
        match self {
            Self::WaitForCoalescedCatchUp => {
                "Wait for the coalesced frame to apply; the surface stays visible but is not yet current."
            }
            Self::RequestFreshSnapshot => {
                "Request a fresh consistent snapshot and narrow to last-known read-only until it applies."
            }
            Self::Resubscribe => {
                "Run the visible resubscribe to re-establish the subscription on a new snapshot epoch."
            }
            Self::ReconnectProvider => {
                "Reconnect the provider overlay; keep the view read-only as last-known until it returns."
            }
            Self::HoldLastKnownReadOnly => {
                "Keep the last consistent projection visible read-only and do not treat it as exact current truth."
            }
            Self::OpenSubscriptionInspector => {
                "Open the subscription inspector to confirm the authority, epoch, and freshness in effect."
            }
            Self::ExportReactiveDiagnostics => {
                "Export the metadata-safe reactive diagnostics packet for support review before sharing."
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Section: active subscriptions and current epoch.
// ---------------------------------------------------------------------------

/// One active `(binding, scope)` subscription and its epoch standing.
///
/// The authority epoch is the live epoch the publishing authority is on; the
/// snapshot epoch is the epoch the consumer's current frame belongs to. When the
/// snapshot epoch trails the authority epoch the consumer has drifted and may not
/// present its derived view as exact current truth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ActiveSubscriptionRow {
    /// Subscription id allocated for the `(binding, scope)` pair.
    pub subscription_id: u64,
    /// Binding the frame was published on.
    pub binding_id: String,
    /// Canonical query family.
    pub query_family: String,
    /// Subscription scope class.
    pub scope_class: ScopeClass,
    /// Concrete scope id.
    pub scope_id: String,
    /// Materialized-view class.
    pub view_class: ViewClass,
    /// Live epoch the publishing authority is on.
    pub authority_epoch: u64,
    /// Snapshot epoch the consumer's current frame belongs to.
    pub snapshot_epoch: u64,
    /// Delta sequence within the snapshot epoch.
    pub delta_seq: u64,
    /// Observed freshness.
    pub freshness: Freshness,
    /// Observed completeness.
    pub completeness: Completeness,
    /// Observed backpressure mode.
    pub backpressure_mode: BackpressureMode,
    /// Narrowed truth claim the surface may present.
    pub truth_claim: TruthClaim,
    /// Consumer surfaces subscribed to the binding, in stable order.
    pub consumer_surfaces: Vec<ConsumerSurface>,
    /// True when the snapshot epoch trails the authority epoch.
    pub epoch_drift: bool,
}

impl ActiveSubscriptionRow {
    /// Returns true when this subscription's epoch and truth claim are
    /// internally consistent and no derived view overclaims exact truth.
    pub fn is_consistent(&self) -> bool {
        self.epoch_drift == (self.snapshot_epoch < self.authority_epoch)
            // A derived surface may never claim exact current truth.
            && self.truth_claim != TruthClaim::ExactCurrentTruth
    }
}

// ---------------------------------------------------------------------------
// Section: invalidation history.
// ---------------------------------------------------------------------------

/// One ordered entry in the invalidation history.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct InvalidationHistoryRow {
    /// Strictly increasing sequence number within the history.
    pub sequence: u64,
    /// Scope the invalidation applied to.
    pub scope_id: String,
    /// Exact invalidation reason.
    pub invalidation_reason: InvalidationReason,
    /// Epoch the consumer was on before the invalidation.
    pub from_epoch: u64,
    /// Epoch the authority moved to.
    pub to_epoch: u64,
    /// Synthetic monotonic observation tag (no wall clock).
    pub observed_at: String,
    /// Redaction-safe narration of the invalidation.
    pub narration: String,
}

// ---------------------------------------------------------------------------
// Section: stale materialization classes.
// ---------------------------------------------------------------------------

/// One materialized-view class that is currently stale.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StaleMaterializationRow {
    /// Binding whose materialized view is stale.
    pub binding_id: String,
    /// Materialized-view class.
    pub view_class: ViewClass,
    /// Persistence class of the materialization.
    pub persistence_class: PersistenceClass,
    /// Scope the materialization covers.
    pub scope_id: String,
    /// Observed freshness (never authoritative for a stale row).
    pub freshness: Freshness,
    /// Invalidation reason that left the materialization stale.
    pub invalidation_reason: InvalidationReason,
    /// Narrow rebuild path for this materialization.
    pub rebuild_next_step: SafeNextStep,
    /// Redaction-safe narration of the stale materialization.
    pub narration: String,
}

impl StaleMaterializationRow {
    /// Returns true when this row is genuinely stale (never authoritative).
    pub fn is_stale(&self) -> bool {
        self.freshness != Freshness::Authoritative
    }
}

// ---------------------------------------------------------------------------
// Section: slow consumers and backpressure.
// ---------------------------------------------------------------------------

/// One lagging consumer, its backpressure state, and the recovery the contract
/// recommends.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlowConsumerRow {
    /// Consumer surface that fell behind.
    pub consumer_surface: ConsumerSurface,
    /// Condition that put the consumer behind.
    pub lag_condition: LagCondition,
    /// Epoch posture while recovering.
    pub epoch_posture: EpochPosture,
    /// Action posture applied to exact-truth actions while behind.
    pub action_posture: ActionPosture,
    /// Backpressure mode in effect.
    pub backpressure_mode: BackpressureMode,
    /// Reason code naming the failure.
    pub reason_code: ReactiveStateReasonCode,
    /// Recovery strategy the contract recommends.
    pub recommended_strategy: RecoveryStrategy,
    /// Safe next step the consumer should take.
    pub safe_next_step: SafeNextStep,
    /// True only when the consumer still offers exact-truth actions.
    pub offers_exact_truth_action: bool,
    /// True only when a silent automatic retry is allowed.
    pub silent_retry_allowed: bool,
    /// Redaction-safe narration of the lag and recovery.
    pub narration: String,
}

impl SlowConsumerRow {
    /// Returns true when this row honors the no-stale-exact-truth invariant:
    /// a consumer that is not on the live epoch never offers an exact-truth
    /// action and never retries silently.
    pub fn honors_truth_gate(&self) -> bool {
        if self.epoch_posture.is_current() {
            return true;
        }
        !self.offers_exact_truth_action && !self.silent_retry_allowed
    }
}

// ---------------------------------------------------------------------------
// Section: Project Doctor probes.
// ---------------------------------------------------------------------------

/// One Project Doctor probe for a reactive-state condition.
///
/// A probe binds a stable finding code to a [`ReactiveStateReasonCode`], a
/// severity, a confidence class, the evidence rows it rests on, the safe next
/// step, and a recovery recommendation. The Doctor pane, CLI/headless rows, and
/// support exports all bind to this one row so they name the failure and the
/// narrow recovery identically.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorProbeRow {
    /// Stable probe id.
    pub probe_id: String,
    /// Stable dotted finding code, e.g. `reactive.consumer_stale`.
    pub finding_code: String,
    /// Reason code this probe diagnoses.
    pub reason_code: ReactiveStateReasonCode,
    /// Diagnosis severity.
    pub severity: FindingSeverity,
    /// Confidence class for the diagnosis.
    pub confidence: ConfidenceClass,
    /// Repair availability for the recommended recovery.
    pub repair_availability: RepairAvailabilityClass,
    /// Safe next step the probe recommends.
    pub safe_next_step: SafeNextStep,
    /// Redaction-safe summary of the condition the probe detects.
    pub condition_summary: String,
    /// Redaction-safe recovery recommendation.
    pub recovery_recommendation: String,
    /// References to the evidence rows the probe rests on (section:key), never
    /// raw payloads.
    pub evidence_refs: Vec<String>,
}

impl DoctorProbeRow {
    /// Returns true when the probe's finding code, severity, and next step are
    /// consistent with its reason code.
    pub fn is_consistent(&self) -> bool {
        self.finding_code == self.reason_code.finding_code()
            && self.severity == self.reason_code.severity()
            && self.safe_next_step == self.reason_code.safe_next_step()
            && self.repair_availability == self.reason_code.repair_availability()
            && !self.evidence_refs.is_empty()
            && !self.condition_summary.trim().is_empty()
            && !self.recovery_recommendation.trim().is_empty()
    }
}

// ---------------------------------------------------------------------------
// Packet.
// ---------------------------------------------------------------------------

/// Shared source references for the packet.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceContractRefs {
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Packet ref.
    pub packet_ref: String,
    /// Report ref.
    pub report_ref: String,
    /// Runbook ref.
    pub runbook_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level Project Doctor and support-export packet for reactive state.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveDiagnosticsPacket {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable packet id.
    pub packet_id: String,
    /// Reviewer title.
    pub title: String,
    /// Shared refs.
    pub source_contract_refs: SourceContractRefs,
    /// Active subscriptions and current epoch standing.
    pub active_subscriptions: Vec<ActiveSubscriptionRow>,
    /// Ordered invalidation history.
    pub invalidation_history: Vec<InvalidationHistoryRow>,
    /// Stale materialization classes.
    pub stale_materializations: Vec<StaleMaterializationRow>,
    /// Slow-consumer and backpressure state.
    pub slow_consumers: Vec<SlowConsumerRow>,
    /// Project Doctor probes.
    pub doctor_probes: Vec<DoctorProbeRow>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// Troubleshooting scenario a fixture reproduces.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TroubleshootingScenario {
    /// A consumer's snapshot epoch drifts behind the authority epoch.
    EpochDrift,
    /// A burst of invalidations saturates a consumer.
    InvalidationStorm,
    /// A consumer lags and coalesces behind its producer.
    LaggingConsumer,
    /// A partially loaded scope serves a stale view.
    PartialScopeStale,
    /// A derived surface reads a provider-unavailable overlay.
    ProviderOverlayUnavailable,
    /// A dropped subscription requires a visible resubscribe.
    ResubscribeRequired,
    /// A consumer is pinned to a known-stale epoch.
    ConsumerStale,
    /// A bounded delta queue overflows.
    BackpressureOverflow,
}

impl TroubleshootingScenario {
    /// Returns the stable snake_case token for this scenario.
    pub const fn as_token(self) -> &'static str {
        match self {
            Self::EpochDrift => "epoch_drift",
            Self::InvalidationStorm => "invalidation_storm",
            Self::LaggingConsumer => "lagging_consumer",
            Self::PartialScopeStale => "partial_scope_stale",
            Self::ProviderOverlayUnavailable => "provider_overlay_unavailable",
            Self::ResubscribeRequired => "resubscribe_required",
            Self::ConsumerStale => "consumer_stale",
            Self::BackpressureOverflow => "backpressure_overflow",
        }
    }
}

/// Fixture pinning one troubleshooting scenario to its expected diagnosis.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveDiagnosticsFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Troubleshooting scenario under test.
    pub scenario: TroubleshootingScenario,
    /// Reason code the scenario must reproduce.
    pub expected_reason_code: ReactiveStateReasonCode,
    /// Finding code the scenario must reproduce.
    pub expected_finding_code: String,
    /// Severity the scenario must reproduce.
    pub expected_severity: FindingSeverity,
    /// Safe next step the scenario must reproduce.
    pub expected_safe_next_step: SafeNextStep,
    /// Short reviewer note.
    pub notes: String,
}

// ---------------------------------------------------------------------------
// Validation.
// ---------------------------------------------------------------------------

/// One validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationViolation {
    /// Stable check id.
    pub check_id: &'static str,
    /// Human-readable explanation.
    pub message: String,
}

/// Validation report for the packet or fixtures.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationReport {
    /// All detected violations.
    pub violations: Vec<ValidationViolation>,
}

impl ValidationReport {
    fn push(&mut self, check_id: &'static str, message: impl Into<String>) {
        self.violations.push(ValidationViolation {
            check_id,
            message: message.into(),
        });
    }

    fn is_empty(&self) -> bool {
        self.violations.is_empty()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "reactive-diagnostics validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

/// Validates the packet against the frozen reactive-diagnostics contract.
///
/// # Errors
///
/// Returns a [`ValidationReport`] listing every contract violation.
pub fn validate_reactive_diagnostics_packet(
    packet: &ReactiveDiagnosticsPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != REACTIVE_DIAGNOSTICS_PACKET_RECORD_KIND {
        report.push("packet_record_kind", "packet record kind is wrong");
    }
    if packet.schema_version != REACTIVE_DIAGNOSTICS_SCHEMA_VERSION {
        report.push("packet_schema_version", "packet schema version is wrong");
    }

    if packet.active_subscriptions.is_empty() {
        report.push("active_subscriptions_present", "no active subscriptions");
    }
    for row in &packet.active_subscriptions {
        if !row.is_consistent() {
            report.push(
                "subscription_consistent",
                format!(
                    "subscription {} has inconsistent epoch/claim state",
                    row.subscription_id
                ),
            );
        }
    }

    // Invalidation history must be strictly ordered and never roll an epoch
    // backward.
    let mut last_sequence: Option<u64> = None;
    for row in &packet.invalidation_history {
        if let Some(prev) = last_sequence {
            if row.sequence <= prev {
                report.push(
                    "invalidation_sequence_monotonic",
                    format!("invalidation sequence {} is not increasing", row.sequence),
                );
            }
        }
        last_sequence = Some(row.sequence);
        if row.to_epoch < row.from_epoch {
            report.push(
                "invalidation_epoch_forward",
                format!(
                    "invalidation sequence {} rolls the epoch backward",
                    row.sequence
                ),
            );
        }
    }

    for row in &packet.stale_materializations {
        if !row.is_stale() {
            report.push(
                "materialization_stale",
                format!(
                    "materialization {} is marked stale but authoritative",
                    row.binding_id
                ),
            );
        }
    }

    for row in &packet.slow_consumers {
        if !row.honors_truth_gate() {
            report.push(
                "slow_consumer_truth_gate",
                format!(
                    "slow consumer {} offers exact truth or a silent retry while behind",
                    row.consumer_surface.as_str()
                ),
            );
        }
        if row.reason_code == ReactiveStateReasonCode::ProviderOverlayUnavailable
            && row.action_posture != ActionPosture::Blocked
        {
            report.push(
                "provider_overlay_blocked",
                format!(
                    "provider-unavailable consumer {} must block exact-truth actions",
                    row.consumer_surface.as_str()
                ),
            );
        }
    }

    if packet.doctor_probes.is_empty() {
        report.push("doctor_probes_present", "no doctor probes");
    }
    for probe in &packet.doctor_probes {
        if !probe.is_consistent() {
            report.push(
                "probe_consistent",
                format!("doctor probe {} is internally inconsistent", probe.probe_id),
            );
        }
    }

    // Every reason code the spec requires a diagnosis to name must have a
    // Doctor probe.
    let covered: BTreeSet<_> = packet
        .doctor_probes
        .iter()
        .map(|probe| probe.reason_code)
        .collect();
    for required in ReactiveStateReasonCode::required_named() {
        if !covered.contains(&required) {
            report.push(
                "required_reason_code_probed",
                format!("no doctor probe names reason code {}", required.as_token()),
            );
        }
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

/// Validates one fixture against the packet it claims to reproduce.
///
/// # Errors
///
/// Returns a [`ValidationReport`] when the fixture's expected diagnosis is not
/// reproducible from the packet.
pub fn validate_reactive_diagnostics_fixture(
    packet: &ReactiveDiagnosticsPacket,
    fixture: &ReactiveDiagnosticsFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != REACTIVE_DIAGNOSTICS_FIXTURE_RECORD_KIND {
        report.push("fixture_record_kind", "fixture record kind is wrong");
    }
    if fixture.schema_version != REACTIVE_DIAGNOSTICS_SCHEMA_VERSION {
        report.push("fixture_schema_version", "fixture schema version is wrong");
    }

    let expected_code = fixture.expected_reason_code;
    if fixture.expected_finding_code != expected_code.finding_code() {
        report.push(
            "fixture_finding_code",
            format!(
                "fixture {} finding code does not match its reason code",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_severity != expected_code.severity() {
        report.push(
            "fixture_severity",
            format!(
                "fixture {} severity does not match its reason code",
                fixture.fixture_id
            ),
        );
    }
    if fixture.expected_safe_next_step != expected_code.safe_next_step() {
        report.push(
            "fixture_safe_next_step",
            format!(
                "fixture {} safe next step does not match its reason code",
                fixture.fixture_id
            ),
        );
    }

    // The scenario must be reproducible: the packet must carry a Doctor probe
    // that names the same reason code.
    let reproducible = packet
        .doctor_probes
        .iter()
        .any(|probe| probe.reason_code == expected_code);
    if !reproducible {
        report.push(
            "fixture_reproducible",
            format!(
                "fixture {} reason code {} is not reproducible from the packet",
                fixture.fixture_id,
                expected_code.as_token()
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

// ---------------------------------------------------------------------------
// Seeds.
// ---------------------------------------------------------------------------

// One row carries the full subscription contract; an argument list keeps the
// seed readable next to its sibling helpers.
#[allow(clippy::too_many_arguments)]
fn subscription(
    subscription_id: u64,
    binding_id: &str,
    query_family: &str,
    scope_class: ScopeClass,
    scope_id: &str,
    view_class: ViewClass,
    authority_epoch: u64,
    snapshot_epoch: u64,
    delta_seq: u64,
    freshness: Freshness,
    completeness: Completeness,
    backpressure_mode: BackpressureMode,
    truth_claim: TruthClaim,
    consumer_surfaces: Vec<ConsumerSurface>,
) -> ActiveSubscriptionRow {
    ActiveSubscriptionRow {
        subscription_id,
        binding_id: binding_id.to_owned(),
        query_family: query_family.to_owned(),
        scope_class,
        scope_id: scope_id.to_owned(),
        view_class,
        authority_epoch,
        snapshot_epoch,
        delta_seq,
        freshness,
        completeness,
        backpressure_mode,
        truth_claim,
        consumer_surfaces,
        epoch_drift: snapshot_epoch < authority_epoch,
    }
}

fn invalidation(
    sequence: u64,
    scope_id: &str,
    invalidation_reason: InvalidationReason,
    from_epoch: u64,
    to_epoch: u64,
    observed_at: &str,
    narration: &str,
) -> InvalidationHistoryRow {
    InvalidationHistoryRow {
        sequence,
        scope_id: scope_id.to_owned(),
        invalidation_reason,
        from_epoch,
        to_epoch,
        observed_at: observed_at.to_owned(),
        narration: narration.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn stale_materialization(
    binding_id: &str,
    view_class: ViewClass,
    persistence_class: PersistenceClass,
    scope_id: &str,
    freshness: Freshness,
    invalidation_reason: InvalidationReason,
    rebuild_next_step: SafeNextStep,
    narration: &str,
) -> StaleMaterializationRow {
    StaleMaterializationRow {
        binding_id: binding_id.to_owned(),
        view_class,
        persistence_class,
        scope_id: scope_id.to_owned(),
        freshness,
        invalidation_reason,
        rebuild_next_step,
        narration: narration.to_owned(),
    }
}

#[allow(clippy::too_many_arguments)]
fn slow_consumer(
    consumer_surface: ConsumerSurface,
    lag_condition: LagCondition,
    epoch_posture: EpochPosture,
    action_posture: ActionPosture,
    backpressure_mode: BackpressureMode,
    reason_code: ReactiveStateReasonCode,
    recommended_strategy: RecoveryStrategy,
    narration: &str,
) -> SlowConsumerRow {
    SlowConsumerRow {
        consumer_surface,
        lag_condition,
        epoch_posture,
        action_posture,
        backpressure_mode,
        reason_code,
        recommended_strategy,
        safe_next_step: reason_code.safe_next_step(),
        offers_exact_truth_action: false,
        silent_retry_allowed: false,
        narration: narration.to_owned(),
    }
}

fn doctor_probe(
    reason_code: ReactiveStateReasonCode,
    confidence: ConfidenceClass,
    condition_summary: &str,
    recovery_recommendation: &str,
    evidence_refs: Vec<&str>,
) -> DoctorProbeRow {
    DoctorProbeRow {
        probe_id: format!("probe.reactive.{}", reason_code.as_token()),
        finding_code: reason_code.finding_code(),
        reason_code,
        severity: reason_code.severity(),
        confidence,
        repair_availability: reason_code.repair_availability(),
        safe_next_step: reason_code.safe_next_step(),
        condition_summary: condition_summary.to_owned(),
        recovery_recommendation: recovery_recommendation.to_owned(),
        evidence_refs: evidence_refs.into_iter().map(str::to_owned).collect(),
    }
}

/// Returns the checked-in reactive-diagnostics packet this lane freezes.
pub fn seeded_reactive_diagnostics_packet() -> ReactiveDiagnosticsPacket {
    let active_subscriptions = vec![
        subscription(
            101,
            "shell.workspace_tree",
            "workspace_tree",
            ScopeClass::Workspace,
            "ws:alpha",
            ViewClass::DurableLocalMaterialization,
            12,
            12,
            4,
            Freshness::Authoritative,
            Completeness::Full,
            BackpressureMode::Realtime,
            TruthClaim::ConsistentSnapshot,
            vec![ConsumerSurface::DesktopShell],
        ),
        subscription(
            102,
            "search.results",
            "search_results",
            ScopeClass::Workspace,
            "ws:alpha",
            ViewClass::EphemeralProjection,
            9,
            7,
            2,
            Freshness::Stale,
            Completeness::Partial,
            BackpressureMode::Coalesced,
            TruthClaim::StaleSnapshot,
            vec![ConsumerSurface::DesktopShell, ConsumerSurface::CliHeadless],
        ),
        subscription(
            103,
            "graph.neighborhood",
            "graph_neighborhood",
            ScopeClass::Window,
            "win:3",
            ViewClass::EphemeralProjection,
            6,
            4,
            5,
            Freshness::Cached,
            Completeness::Partial,
            BackpressureMode::Coalesced,
            TruthClaim::CoalescedStream,
            vec![ConsumerSurface::DesktopShell],
        ),
        subscription(
            104,
            "review.workspace",
            "review_workspace",
            ScopeClass::ReviewWorkspace,
            "rw:42",
            ViewClass::ExportableSnapshot,
            3,
            3,
            0,
            Freshness::Stale,
            Completeness::Unavailable,
            BackpressureMode::SnapshotRequired,
            TruthClaim::ProviderUnavailable,
            vec![ConsumerSurface::ReviewWorkspace],
        ),
        subscription(
            105,
            "ai.context",
            "ai_context",
            ScopeClass::Workspace,
            "ws:alpha",
            ViewClass::EphemeralProjection,
            5,
            5,
            1,
            Freshness::Warming,
            Completeness::Partial,
            BackpressureMode::Coalesced,
            TruthClaim::PartialProjection,
            vec![ConsumerSurface::AiInspector],
        ),
        subscription(
            106,
            "companion.panel",
            "companion_panel",
            ScopeClass::CompanionSurface,
            "cmp:9",
            ViewClass::ManagedReplicatedView,
            4,
            2,
            3,
            Freshness::Stale,
            Completeness::Partial,
            BackpressureMode::SnapshotRequired,
            TruthClaim::StaleSnapshot,
            vec![ConsumerSurface::CompanionSnapshot],
        ),
    ];

    let invalidation_history = vec![
        invalidation(
            1,
            "ws:alpha",
            InvalidationReason::UpstreamInputStale,
            6,
            7,
            "tick:000001",
            "A workspace input changed upstream of the search index, so the projection fell behind its authority.",
        ),
        invalidation(
            2,
            "ws:alpha",
            InvalidationReason::QueueSaturation,
            7,
            9,
            "tick:000002",
            "A burst of edits saturated the bounded delta queue, coalescing several epochs into one catch-up frame.",
        ),
        invalidation(
            3,
            "win:3",
            InvalidationReason::CacheServed,
            4,
            6,
            "tick:000003",
            "The graph neighborhood was served from a local cache while the live producer advanced two epochs.",
        ),
        invalidation(
            4,
            "rw:42",
            InvalidationReason::WatcherDropped,
            3,
            3,
            "tick:000004",
            "The review provider overlay dropped; the watcher is gone and no current truth is observable for the scope.",
        ),
        invalidation(
            5,
            "cmp:9",
            InvalidationReason::AuthorityEpochRolled,
            2,
            4,
            "tick:000005",
            "The companion mirror's authority rolled two epochs while the panel held a stale replicated view.",
        ),
        invalidation(
            6,
            "ws:alpha",
            InvalidationReason::CausalityLost,
            7,
            9,
            "tick:000006",
            "A contiguous run of deltas was dropped, so causality cannot be proven without a fresh snapshot.",
        ),
    ];

    let stale_materializations = vec![
        stale_materialization(
            "search.results",
            ViewClass::EphemeralProjection,
            PersistenceClass::LocalCacheOrDb,
            "ws:alpha",
            Freshness::Stale,
            InvalidationReason::UpstreamInputStale,
            SafeNextStep::RequestFreshSnapshot,
            "The search projection is stale behind a changed upstream input; rebuild it from a fresh snapshot.",
        ),
        stale_materialization(
            "graph.neighborhood",
            ViewClass::EphemeralProjection,
            PersistenceClass::MemoryOnly,
            "win:3",
            Freshness::Cached,
            InvalidationReason::CacheServed,
            SafeNextStep::RequestFreshSnapshot,
            "The graph neighborhood is a cached projection two epochs behind; refresh it from the live producer.",
        ),
        stale_materialization(
            "companion.panel",
            ViewClass::ManagedReplicatedView,
            PersistenceClass::ServiceOrLocalMirror,
            "cmp:9",
            Freshness::Stale,
            InvalidationReason::AuthorityEpochRolled,
            SafeNextStep::Resubscribe,
            "The companion replicated view trails a rolled authority epoch; resubscribe to reconcile it.",
        ),
        stale_materialization(
            "review.workspace",
            ViewClass::ExportableSnapshot,
            PersistenceClass::SavedArtifact,
            "rw:42",
            Freshness::Stale,
            InvalidationReason::WatcherDropped,
            SafeNextStep::HoldLastKnownReadOnly,
            "The review snapshot's provider overlay is gone; hold the last-known snapshot read-only until it returns.",
        ),
    ];

    let slow_consumers = vec![
        slow_consumer(
            ConsumerSurface::DesktopShell,
            LagCondition::RapidInvalidationBurst,
            EpochPosture::Coalescing,
            ActionPosture::RevalidateBeforeAct,
            BackpressureMode::Coalesced,
            ReactiveStateReasonCode::InvalidationStorm,
            RecoveryStrategy::CoalesceDeltas,
            "Shell strips are coalescing an invalidation burst into one consistent frame and revalidate before any pending action commits.",
        ),
        slow_consumer(
            ConsumerSurface::DesktopShell,
            LagCondition::BackpressureOverflow,
            EpochPosture::SnapshotRecovering,
            ActionPosture::NarrowedToLastKnown,
            BackpressureMode::SnapshotRequired,
            ReactiveStateReasonCode::BackpressureOverflow,
            RecoveryStrategy::RequestFreshSnapshot,
            "The bounded delta queue overflowed; the card narrowed to last-known read-only and is reloading from a fresh snapshot.",
        ),
        slow_consumer(
            ConsumerSurface::CliHeadless,
            LagCondition::ConsumerLag,
            EpochPosture::Coalescing,
            ActionPosture::RevalidateBeforeAct,
            BackpressureMode::Coalesced,
            ReactiveStateReasonCode::ConsumerCoalescing,
            RecoveryStrategy::CoalesceDeltas,
            "Headless output is coalescing trailing deltas and stamps each record with its epoch and a not-current freshness flag.",
        ),
        slow_consumer(
            ConsumerSurface::CliHeadless,
            LagCondition::ReconnectAfterDrop,
            EpochPosture::ResubscribePending,
            ActionPosture::ResubscribeRequired,
            BackpressureMode::SnapshotRequired,
            ReactiveStateReasonCode::ResubscribeRequired,
            RecoveryStrategy::Resubscribe,
            "The subscription dropped and is resubscribing on a new snapshot epoch; exact-truth actions wait for the visible resubscribe.",
        ),
        slow_consumer(
            ConsumerSurface::AiInspector,
            LagCondition::InvalidationGap,
            EpochPosture::StaleEpoch,
            ActionPosture::NarrowedToLastKnown,
            BackpressureMode::SnapshotRequired,
            ReactiveStateReasonCode::ConsumerStale,
            RecoveryStrategy::RequestFreshSnapshot,
            "An invalidation gap broke causality for the AI context panel; it is pinned to a stale epoch until a fresh snapshot applies.",
        ),
        slow_consumer(
            ConsumerSurface::ReviewWorkspace,
            LagCondition::ProviderOverlayDisappeared,
            EpochPosture::StaleEpoch,
            ActionPosture::Blocked,
            BackpressureMode::SnapshotRequired,
            ReactiveStateReasonCode::ProviderOverlayUnavailable,
            RecoveryStrategy::MarkStaleEpoch,
            "The review workspace's provider overlay disappeared; exact-truth actions are blocked and the view holds last-known read-only.",
        ),
        slow_consumer(
            ConsumerSurface::CompanionSnapshot,
            LagCondition::ConsumerLag,
            EpochPosture::StaleEpoch,
            ActionPosture::NarrowedToLastKnown,
            BackpressureMode::SnapshotRequired,
            ReactiveStateReasonCode::PartialScopeStale,
            RecoveryStrategy::RequestFreshSnapshot,
            "The companion panel holds a partial, stale replicated scope; it stays read-only until a fresh snapshot reconciles the scope.",
        ),
    ];

    let doctor_probes = vec![
        doctor_probe(
            ReactiveStateReasonCode::ConsumerStale,
            ConfidenceClass::ObservedAuthoritative,
            "A consumer is pinned to a known-stale epoch after an invalidation gap broke causality.",
            "Request a fresh consistent snapshot; keep the view read-only as last-known until it applies. Do not act on the stale epoch as current truth.",
            vec!["active_subscription:105", "slow_consumer:ai_inspector", "invalidation_history:6"],
        ),
        doctor_probe(
            ReactiveStateReasonCode::ConsumerCoalescing,
            ConfidenceClass::ObservedAuthoritative,
            "A consumer is trailing the live epoch and applying coalesced frames behind its producer.",
            "Wait for the coalesced frame to apply and revalidate any pending action against the producer before it commits.",
            vec!["active_subscription:102", "slow_consumer:cli_headless", "invalidation_history:2"],
        ),
        doctor_probe(
            ReactiveStateReasonCode::ResubscribeRequired,
            ConfidenceClass::ObservedAuthoritative,
            "A dropped subscription is reconnecting and must resubscribe on a new snapshot epoch before acting.",
            "Run the visible resubscribe to re-establish the subscription; exact-truth actions stay withheld until the new snapshot applies.",
            vec!["slow_consumer:cli_headless", "invalidation_history:4"],
        ),
        doctor_probe(
            ReactiveStateReasonCode::ProviderOverlayUnavailable,
            ConfidenceClass::ObservedAuthoritative,
            "A derived surface is reading a provider overlay whose backing producer is unavailable.",
            "Reconnect the provider overlay; keep the view read-only as last-known truth and block exact-truth actions until it returns.",
            vec!["active_subscription:104", "stale_materialization:review.workspace", "slow_consumer:review_workspace"],
        ),
        doctor_probe(
            ReactiveStateReasonCode::EpochDrift,
            ConfidenceClass::ObservedAuthoritative,
            "A consumer's snapshot epoch has drifted behind its authority epoch.",
            "Request a fresh snapshot to realign the consumer to the authority epoch; narrow to last-known read-only meanwhile.",
            vec!["active_subscription:102", "active_subscription:103", "active_subscription:106"],
        ),
        doctor_probe(
            ReactiveStateReasonCode::InvalidationStorm,
            ConfidenceClass::ObservedAuthoritative,
            "A burst of invalidations is arriving faster than a consumer can apply, saturating its delta queue.",
            "Let the storm coalesce into one consistent frame; the surface stays visible but does not claim the burst already settled.",
            vec!["slow_consumer:desktop_shell", "invalidation_history:2"],
        ),
        doctor_probe(
            ReactiveStateReasonCode::BackpressureOverflow,
            ConfidenceClass::ObservedAuthoritative,
            "A bounded delta queue overflowed and dropped intermediate frames, breaking causality.",
            "Drop the lossy stream and request a fresh consistent snapshot; narrow to last-known read-only until it applies.",
            vec!["slow_consumer:desktop_shell", "invalidation_history:6"],
        ),
        doctor_probe(
            ReactiveStateReasonCode::PartialScopeStale,
            ConfidenceClass::InferredFromEvidence,
            "A partially loaded scope is serving a stale or incomplete view.",
            "Hold the last consistent projection read-only and request a fresh snapshot to reconcile the partial scope.",
            vec!["active_subscription:106", "slow_consumer:companion_snapshot", "invalidation_history:5"],
        ),
    ];

    ReactiveDiagnosticsPacket {
        record_kind: REACTIVE_DIAGNOSTICS_PACKET_RECORD_KIND.to_owned(),
        schema_version: REACTIVE_DIAGNOSTICS_SCHEMA_VERSION,
        packet_id: "packet.reactive_diagnostics".to_owned(),
        title: "Reactive-state Project Doctor and support diagnostics".to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: REACTIVE_DIAGNOSTICS_DOC_REF.to_owned(),
            schema_ref: REACTIVE_DIAGNOSTICS_SCHEMA_REF.to_owned(),
            packet_ref: REACTIVE_DIAGNOSTICS_PACKET_REF.to_owned(),
            report_ref: REACTIVE_DIAGNOSTICS_REPORT_REF.to_owned(),
            runbook_ref: REACTIVE_DIAGNOSTICS_RUNBOOK_REF.to_owned(),
            fixture_manifest_ref: REACTIVE_DIAGNOSTICS_FIXTURE_MANIFEST_REF.to_owned(),
        },
        active_subscriptions,
        invalidation_history,
        stale_materializations,
        slow_consumers,
        doctor_probes,
        invariants: vec![
            "Every diagnosis names an exact reason code and a narrow safe next step.".to_owned(),
            "A consumer that is not on the live epoch never offers an exact-truth action or a silent retry.".to_owned(),
            "A derived surface never presents exact current truth; provider-unavailable overlays block exact-truth actions.".to_owned(),
            "Export packets carry finding identity and recovery vocabulary only — never raw payloads, credentials, paths, or traces.".to_owned(),
        ],
    }
}

fn fixture(
    fixture_id: &str,
    scenario: TroubleshootingScenario,
    expected_reason_code: ReactiveStateReasonCode,
    notes: &str,
) -> ReactiveDiagnosticsFixture {
    ReactiveDiagnosticsFixture {
        record_kind: REACTIVE_DIAGNOSTICS_FIXTURE_RECORD_KIND.to_owned(),
        schema_version: REACTIVE_DIAGNOSTICS_SCHEMA_VERSION,
        fixture_id: fixture_id.to_owned(),
        scenario,
        expected_reason_code,
        expected_finding_code: expected_reason_code.finding_code(),
        expected_severity: expected_reason_code.severity(),
        expected_safe_next_step: expected_reason_code.safe_next_step(),
        notes: notes.to_owned(),
    }
}

/// Returns the checked-in troubleshooting fixtures this lane freezes.
///
/// The corpus reproduces epoch drift, invalidation storms, lagging consumers,
/// and partial-scope stale views — plus the provider-unavailable, resubscribe,
/// stale, and backpressure conditions — so reactive-state bugs are reproducible
/// from fixtures instead of tribal knowledge.
pub fn seeded_reactive_diagnostics_fixtures() -> Vec<ReactiveDiagnosticsFixture> {
    vec![
        fixture(
            "fixture.reactive_diagnostics.epoch_drift",
            TroubleshootingScenario::EpochDrift,
            ReactiveStateReasonCode::EpochDrift,
            "A consumer's snapshot epoch trails the authority epoch; the diagnosis names epoch drift and recommends a fresh snapshot.",
        ),
        fixture(
            "fixture.reactive_diagnostics.invalidation_storm",
            TroubleshootingScenario::InvalidationStorm,
            ReactiveStateReasonCode::InvalidationStorm,
            "An invalidation burst saturates a consumer; the diagnosis names an invalidation storm and recommends coalesced catch-up.",
        ),
        fixture(
            "fixture.reactive_diagnostics.lagging_consumer",
            TroubleshootingScenario::LaggingConsumer,
            ReactiveStateReasonCode::ConsumerCoalescing,
            "A consumer lags and coalesces behind its producer; the diagnosis names coalescing and recommends waiting for catch-up.",
        ),
        fixture(
            "fixture.reactive_diagnostics.partial_scope_stale",
            TroubleshootingScenario::PartialScopeStale,
            ReactiveStateReasonCode::PartialScopeStale,
            "A partial scope serves a stale view; the diagnosis names partial-scope stale and recommends holding last-known read-only.",
        ),
        fixture(
            "fixture.reactive_diagnostics.provider_overlay_unavailable",
            TroubleshootingScenario::ProviderOverlayUnavailable,
            ReactiveStateReasonCode::ProviderOverlayUnavailable,
            "A provider overlay disappears; the diagnosis names the unavailable overlay and recommends reconnecting the provider.",
        ),
        fixture(
            "fixture.reactive_diagnostics.resubscribe_required",
            TroubleshootingScenario::ResubscribeRequired,
            ReactiveStateReasonCode::ResubscribeRequired,
            "A dropped subscription must resubscribe; the diagnosis names resubscribe-required and recommends the visible resubscribe.",
        ),
        fixture(
            "fixture.reactive_diagnostics.consumer_stale",
            TroubleshootingScenario::ConsumerStale,
            ReactiveStateReasonCode::ConsumerStale,
            "A consumer is pinned to a stale epoch; the diagnosis names the stale consumer and recommends a fresh snapshot.",
        ),
        fixture(
            "fixture.reactive_diagnostics.backpressure_overflow",
            TroubleshootingScenario::BackpressureOverflow,
            ReactiveStateReasonCode::BackpressureOverflow,
            "A bounded delta queue overflows; the diagnosis names the overflow and recommends a fresh snapshot.",
        ),
    ]
}

// ---------------------------------------------------------------------------
// Support export.
// ---------------------------------------------------------------------------

/// One metadata-safe support-export row derived from a Doctor probe.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveDiagnosticsSupportExportRow {
    /// Stable row record kind.
    pub record_kind: String,
    /// Stable probe id.
    pub probe_id: String,
    /// Stable dotted finding code.
    pub finding_code: String,
    /// Reason code the row names.
    pub reason_code: ReactiveStateReasonCode,
    /// Diagnosis severity.
    pub severity: FindingSeverity,
    /// Confidence class.
    pub confidence: ConfidenceClass,
    /// Repair availability.
    pub repair_availability: RepairAvailabilityClass,
    /// Safe next step.
    pub safe_next_step: SafeNextStep,
    /// Redaction-safe condition summary.
    pub condition_summary: String,
    /// Redaction-safe recovery recommendation.
    pub recovery_recommendation: String,
    /// Count of evidence rows referenced (metadata only, no payloads).
    pub evidence_ref_count: usize,
    /// Raw payloads remain excluded.
    pub raw_payload_excluded: bool,
    /// Ambient authority remains excluded.
    pub ambient_authority_excluded: bool,
}

impl ReactiveDiagnosticsSupportExportRow {
    fn from_probe(probe: &DoctorProbeRow) -> Self {
        Self {
            record_kind: REACTIVE_DIAGNOSTICS_SUPPORT_EXPORT_ROW_RECORD_KIND.to_owned(),
            probe_id: probe.probe_id.clone(),
            finding_code: probe.finding_code.clone(),
            reason_code: probe.reason_code,
            severity: probe.severity,
            confidence: probe.confidence,
            repair_availability: probe.repair_availability,
            safe_next_step: probe.safe_next_step,
            condition_summary: probe.condition_summary.clone(),
            recovery_recommendation: probe.recovery_recommendation.clone(),
            evidence_ref_count: probe.evidence_refs.len(),
            raw_payload_excluded: true,
            ambient_authority_excluded: true,
        }
    }

    /// Returns true when the row remains metadata-safe and support-usable.
    pub fn is_export_safe(&self) -> bool {
        self.raw_payload_excluded
            && self.ambient_authority_excluded
            && self.finding_code == self.reason_code.finding_code()
            && !self.condition_summary.trim().is_empty()
            && !self.recovery_recommendation.trim().is_empty()
    }
}

/// Metadata-first support-export envelope reviewable before it leaves the
/// machine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveDiagnosticsSupportExportEnvelope {
    /// Stable envelope record kind.
    pub record_kind: String,
    /// Stable envelope id.
    pub envelope_id: String,
    /// Capture time supplied by the caller.
    pub captured_at: String,
    /// Reviewer doc ref.
    pub doc_ref: String,
    /// Schema ref.
    pub schema_ref: String,
    /// Reviewer report ref.
    pub report_ref: String,
    /// Runbook ref.
    pub runbook_ref: String,
    /// True when the envelope is shaped for review before any share step.
    pub reviewable_before_share: bool,
    /// Raw payloads remain excluded.
    pub raw_payload_excluded: bool,
    /// Ambient authority remains excluded.
    pub ambient_authority_excluded: bool,
    /// Export rows.
    pub rows: Vec<ReactiveDiagnosticsSupportExportRow>,
}

impl ReactiveDiagnosticsSupportExportEnvelope {
    /// Builds a metadata-safe envelope from a validated packet.
    pub fn from_packet(
        envelope_id: impl Into<String>,
        captured_at: impl Into<String>,
        packet: &ReactiveDiagnosticsPacket,
    ) -> Self {
        let mut rows: Vec<_> = packet
            .doctor_probes
            .iter()
            .map(ReactiveDiagnosticsSupportExportRow::from_probe)
            .collect();
        rows.sort_by(|a, b| a.finding_code.cmp(&b.finding_code));
        Self {
            record_kind: REACTIVE_DIAGNOSTICS_SUPPORT_EXPORT_ENVELOPE_RECORD_KIND.to_owned(),
            envelope_id: envelope_id.into(),
            captured_at: captured_at.into(),
            doc_ref: REACTIVE_DIAGNOSTICS_DOC_REF.to_owned(),
            schema_ref: REACTIVE_DIAGNOSTICS_SCHEMA_REF.to_owned(),
            report_ref: REACTIVE_DIAGNOSTICS_REPORT_REF.to_owned(),
            runbook_ref: REACTIVE_DIAGNOSTICS_RUNBOOK_REF.to_owned(),
            reviewable_before_share: true,
            raw_payload_excluded: true,
            ambient_authority_excluded: true,
            rows,
        }
    }

    /// Returns true when the envelope remains metadata-safe and in sync with the
    /// canonical packet refs.
    pub fn is_export_safe(&self) -> bool {
        self.reviewable_before_share
            && self.raw_payload_excluded
            && self.ambient_authority_excluded
            && self.doc_ref == REACTIVE_DIAGNOSTICS_DOC_REF
            && self.schema_ref == REACTIVE_DIAGNOSTICS_SCHEMA_REF
            && self.report_ref == REACTIVE_DIAGNOSTICS_REPORT_REF
            && self.runbook_ref == REACTIVE_DIAGNOSTICS_RUNBOOK_REF
            && !self.rows.is_empty()
            && self
                .rows
                .iter()
                .all(ReactiveDiagnosticsSupportExportRow::is_export_safe)
    }
}

/// Error returned when the support envelope cannot be compiled.
#[derive(Debug)]
pub enum ReactiveDiagnosticsSupportExportError {
    /// The canonical packet failed validation.
    PacketValidation(ValidationReport),
}

impl fmt::Display for ReactiveDiagnosticsSupportExportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PacketValidation(report) => {
                write!(f, "reactive-diagnostics packet invalid: {report}")
            }
        }
    }
}

impl std::error::Error for ReactiveDiagnosticsSupportExportError {}

impl From<ValidationReport> for ReactiveDiagnosticsSupportExportError {
    fn from(report: ValidationReport) -> Self {
        Self::PacketValidation(report)
    }
}

/// Compiles the metadata-first support-export envelope from the canonical
/// reactive-diagnostics packet.
///
/// # Errors
///
/// Returns a [`ReactiveDiagnosticsSupportExportError`] when the seeded packet
/// fails validation.
pub fn compile_support_export_envelope(
    envelope_id: impl Into<String>,
    captured_at: impl Into<String>,
) -> Result<ReactiveDiagnosticsSupportExportEnvelope, ReactiveDiagnosticsSupportExportError> {
    let packet = seeded_reactive_diagnostics_packet();
    validate_reactive_diagnostics_packet(&packet)?;
    Ok(ReactiveDiagnosticsSupportExportEnvelope::from_packet(
        envelope_id,
        captured_at,
        &packet,
    ))
}

#[cfg(test)]
mod tests;
