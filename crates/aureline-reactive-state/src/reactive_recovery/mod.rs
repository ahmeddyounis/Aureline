//! Consumer-side reactive recovery flows for lagging subscription consumers.
//!
//! When a derived consumer falls behind its producer — a rapid invalidation
//! burst, plain consumer lag, a backpressure overflow, an invalidation gap, a
//! reconnect after a dropped watcher, or a provider overlay that disappeared —
//! it must catch up without continuing to present its derived view as exact
//! current truth. The packet freezes one shared recovery vocabulary so the
//! desktop shell, the CLI/headless lane, AI inspectors, review workspaces, and
//! companion snapshots all coalesce, resubscribe, request fresh snapshots, and
//! mark stale epochs the same way, and all gate exact-truth actions identically
//! while they are behind.
//!
//! Three properties are frozen here:
//!
//! - **Recovery strategy per surface and lag condition.** Each
//!   ([`ConsumerSurface`], [`LagCondition`]) pair declares a primary
//!   [`RecoveryStrategy`] plus ordered fallbacks, so no surface invents a
//!   private catch-up path.
//! - **No stale exact-truth action.** A consumer that is not on the current
//!   epoch never offers an action that depends on exact current truth, and a
//!   materially changed action posture is never hidden behind a silent retry.
//! - **Visible, exportable drills.** [`RecoveryDrill`]s walk the rapid
//!   invalidation, consumer lag, reconnect, and provider-overlay-disappearance
//!   scenarios from detection through honest verification, proving recovery is
//!   user-visible and support-exportable rather than silent.
//!
//! The packet is mirrored by:
//!
//! - [`/schemas/state/reactive_recovery.schema.json`](../../../../schemas/state/reactive_recovery.schema.json)
//! - [`/docs/state/reactive_recovery.md`](../../../../docs/state/reactive_recovery.md)
//! - [`/artifacts/state/reactive_recovery.json`](../../../../artifacts/state/reactive_recovery.json)
//! - [`/artifacts/state/reactive_recovery.md`](../../../../artifacts/state/reactive_recovery.md)
//! - [`/artifacts/state/reactive_recovery_drills.md`](../../../../artifacts/state/reactive_recovery_drills.md)
//! - [`/fixtures/state/reactive_recovery/`](../../../../fixtures/state/reactive_recovery/)

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::{Deserialize, Serialize};

/// Schema version stamped onto packets and fixtures.
pub const REACTIVE_RECOVERY_SCHEMA_VERSION: u32 = 1;

/// Stable record-kind tag carried by the packet.
pub const REACTIVE_RECOVERY_PACKET_RECORD_KIND: &str = "reactive_recovery_packet_record";

/// Stable record-kind tag carried by fixtures.
pub const REACTIVE_RECOVERY_FIXTURE_RECORD_KIND: &str = "reactive_recovery_fixture_record";

/// Repo-relative schema ref.
pub const REACTIVE_RECOVERY_SCHEMA_REF: &str = "schemas/state/reactive_recovery.schema.json";

/// Repo-relative reviewer doc ref.
pub const REACTIVE_RECOVERY_DOC_REF: &str = "docs/state/reactive_recovery.md";

/// Repo-relative machine-readable artifact packet.
pub const REACTIVE_RECOVERY_PACKET_REF: &str = "artifacts/state/reactive_recovery.json";

/// Repo-relative reviewer artifact report.
pub const REACTIVE_RECOVERY_REPORT_REF: &str = "artifacts/state/reactive_recovery.md";

/// Repo-relative reviewer drill report.
pub const REACTIVE_RECOVERY_DRILLS_REF: &str = "artifacts/state/reactive_recovery_drills.md";

/// Repo-relative fixture directory.
pub const REACTIVE_RECOVERY_FIXTURE_DIR: &str = "fixtures/state/reactive_recovery";

/// Repo-relative fixture manifest.
pub const REACTIVE_RECOVERY_FIXTURE_MANIFEST_REF: &str =
    "fixtures/state/reactive_recovery/manifest.yaml";

/// Derived consumer surface that can fall behind its producer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsumerSurface {
    /// Desktop shell strips, cards, and panes.
    DesktopShell,
    /// CLI and headless machine-readable output.
    CliHeadless,
    /// AI context and route inspectors.
    AiInspector,
    /// Review and merge-queue workspaces.
    ReviewWorkspace,
    /// Companion mirror and follow snapshots.
    CompanionSnapshot,
}

impl ConsumerSurface {
    /// Returns the stable string vocabulary for this surface.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DesktopShell => "desktop_shell",
            Self::CliHeadless => "cli_headless",
            Self::AiInspector => "ai_inspector",
            Self::ReviewWorkspace => "review_workspace",
            Self::CompanionSnapshot => "companion_snapshot",
        }
    }
}

/// Condition that put a consumer behind its producer.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LagCondition {
    /// A burst of invalidations arrived faster than the consumer can apply them.
    RapidInvalidationBurst,
    /// The consumer is applying deltas but trailing the live epoch.
    ConsumerLag,
    /// The bounded delta queue overflowed and dropped intermediate frames.
    BackpressureOverflow,
    /// A contiguous run of deltas is missing, so causality cannot be proven.
    InvalidationGap,
    /// The subscription dropped and is reconnecting to the producer.
    ReconnectAfterDrop,
    /// A provider overlay the view depended on is no longer reachable.
    ProviderOverlayDisappeared,
}

impl LagCondition {
    /// Returns the stable string vocabulary for this lag condition.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RapidInvalidationBurst => "rapid_invalidation_burst",
            Self::ConsumerLag => "consumer_lag",
            Self::BackpressureOverflow => "backpressure_overflow",
            Self::InvalidationGap => "invalidation_gap",
            Self::ReconnectAfterDrop => "reconnect_after_drop",
            Self::ProviderOverlayDisappeared => "provider_overlay_disappeared",
        }
    }
}

/// Catch-up strategy a lagging consumer applies.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecoveryStrategy {
    /// Collapse buffered deltas into one consistent coalesced frame.
    CoalesceDeltas,
    /// Drop the delta stream and request a fresh consistent snapshot.
    RequestFreshSnapshot,
    /// Tear down and re-establish the subscription from a new snapshot epoch.
    Resubscribe,
    /// Mark the current epoch stale and hold until recovery is visible.
    MarkStaleEpoch,
}

impl RecoveryStrategy {
    /// Returns the stable string vocabulary for this strategy.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::CoalesceDeltas => "coalesce_deltas",
            Self::RequestFreshSnapshot => "request_fresh_snapshot",
            Self::Resubscribe => "resubscribe",
            Self::MarkStaleEpoch => "mark_stale_epoch",
        }
    }
}

/// Epoch standing of a consumer while it recovers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EpochPosture {
    /// The consumer is on the live epoch; exact current truth holds.
    Current,
    /// The consumer is applying coalesced frames and trailing the live epoch.
    Coalescing,
    /// The consumer is pinned to a known-stale epoch.
    StaleEpoch,
    /// A resubscription is in flight and the new snapshot has not yet applied.
    ResubscribePending,
    /// A fresh snapshot was requested and is still warming.
    SnapshotRecovering,
}

impl EpochPosture {
    /// Returns the stable string vocabulary for this epoch posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Current => "current",
            Self::Coalescing => "coalescing",
            Self::StaleEpoch => "stale_epoch",
            Self::ResubscribePending => "resubscribe_pending",
            Self::SnapshotRecovering => "snapshot_recovering",
        }
    }

    /// Returns true when this posture proves the consumer is on the live epoch.
    pub const fn is_current(self) -> bool {
        matches!(self, Self::Current)
    }
}

/// Posture applied to actions that depend on exact current truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActionPosture {
    /// Exact-truth actions are offered normally.
    ExactTruthAllowed,
    /// Actions revalidate against the producer before they commit.
    RevalidateBeforeAct,
    /// Only last-known read-only views remain; exact-truth actions are withheld.
    NarrowedToLastKnown,
    /// A visible resubscribe step is required before acting.
    ResubscribeRequired,
    /// Exact-truth actions are blocked until truth is re-established.
    Blocked,
}

impl ActionPosture {
    /// Returns the stable string vocabulary for this action posture.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactTruthAllowed => "exact_truth_allowed",
            Self::RevalidateBeforeAct => "revalidate_before_act",
            Self::NarrowedToLastKnown => "narrowed_to_last_known",
            Self::ResubscribeRequired => "resubscribe_required",
            Self::Blocked => "blocked",
        }
    }

    /// Returns true when this posture still offers unqualified exact truth.
    pub const fn allows_exact_truth(self) -> bool {
        matches!(self, Self::ExactTruthAllowed)
    }
}

/// Context a recovering consumer keeps visible and honest.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreservedContextClass {
    /// The layout slot and pane ordering remain intact.
    LayoutSlot,
    /// The last consistent projection stays visible as last-known truth.
    LastKnownProjection,
    /// The subscription scope identity remains attributable.
    ScopeIdentity,
    /// A freshness cue marks the view as not-yet-current.
    FreshnessCue,
    /// The invalidation reason that triggered recovery stays visible.
    InvalidationReason,
    /// The stale epoch marker stays visible until the consumer catches up.
    EpochMarker,
}

impl PreservedContextClass {
    /// Returns the stable string vocabulary for this context class.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::LayoutSlot => "layout_slot",
            Self::LastKnownProjection => "last_known_projection",
            Self::ScopeIdentity => "scope_identity",
            Self::FreshnessCue => "freshness_cue",
            Self::InvalidationReason => "invalidation_reason",
            Self::EpochMarker => "epoch_marker",
        }
    }
}

/// Phase of a recovery drill step.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DrillPhase {
    /// The consumer detects it has fallen behind.
    Detect,
    /// The consumer narrows exact-truth actions before catching up.
    NarrowAction,
    /// The consumer applies its recovery strategy.
    Recover,
    /// The consumer verifies the resulting posture is honest.
    Verify,
}

impl DrillPhase {
    /// Returns the stable string vocabulary for this drill phase.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Detect => "detect",
            Self::NarrowAction => "narrow_action",
            Self::Recover => "recover",
            Self::Verify => "verify",
        }
    }
}

/// One recovery-flow row keyed by surface and lag condition.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryFlowRow {
    /// Stable flow id.
    pub flow_id: String,
    /// Consumer surface this flow governs.
    pub consumer_surface: ConsumerSurface,
    /// Condition that put the consumer behind.
    pub lag_condition: LagCondition,
    /// Preferred catch-up strategy.
    pub primary_strategy: RecoveryStrategy,
    /// Narrower or later strategies that remain available.
    pub fallback_strategies: Vec<RecoveryStrategy>,
    /// Epoch standing while this flow runs.
    pub epoch_posture: EpochPosture,
    /// Posture applied to exact-truth actions while this flow runs.
    pub action_posture: ActionPosture,
    /// True only when the flow still offers exact-truth actions.
    pub offers_exact_truth_action: bool,
    /// True only when a silent automatic retry is allowed.
    pub silent_retry_allowed: bool,
    /// True when the recovery state is surfaced to the user.
    pub recovery_cue_visible: bool,
    /// True when the recovery state is support-exportable.
    pub support_exportable: bool,
    /// Context the recovering consumer keeps visible and honest.
    pub preserved_context: Vec<PreservedContextClass>,
    /// Support-safe summary of how the consumer recovers.
    pub recovery_summary: String,
    /// Support-safe summary of why the truth posture is honest.
    pub truth_posture_rationale: String,
    /// Contract or module refs that anchor the flow.
    pub source_contract_refs: Vec<String>,
    /// Product consumers that quote the flow directly.
    pub consumer_refs: Vec<String>,
    /// Short reviewer note.
    pub notes: String,
}

/// One ordered step inside a recovery drill.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DrillStep {
    /// Phase of this step.
    pub phase: DrillPhase,
    /// Epoch posture observed at this step.
    pub epoch_posture: EpochPosture,
    /// Action posture observed at this step.
    pub action_posture: ActionPosture,
    /// Redaction-safe narration of the step.
    pub narration: String,
}

/// One recovery drill walking a lag condition from detection to verification.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecoveryDrill {
    /// Stable drill id.
    pub drill_id: String,
    /// Reviewer title.
    pub title: String,
    /// Lag condition exercised by the drill.
    pub lag_condition: LagCondition,
    /// Flow row exercised by the drill.
    pub exercised_flow_id: String,
    /// Ordered drill steps.
    pub steps: Vec<DrillStep>,
    /// True when the drill proves no stale exact-truth action was offered.
    pub asserts_no_stale_exact_action: bool,
    /// True when the drill proves the recovery was user-visible.
    pub asserts_recovery_visible: bool,
    /// Epoch posture the drill ends on.
    pub expected_final_epoch_posture: EpochPosture,
    /// Action posture the drill ends on.
    pub expected_final_action_posture: ActionPosture,
    /// Short reviewer note.
    pub notes: String,
}

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
    /// Drill report ref.
    pub drills_ref: String,
    /// Fixture manifest ref.
    pub fixture_manifest_ref: String,
}

/// Top-level packet freezing the reactive-recovery contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveRecoveryPacket {
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
    /// Recovery-flow rows.
    pub flows: Vec<RecoveryFlowRow>,
    /// Recovery drills.
    pub drills: Vec<RecoveryDrill>,
    /// Short invariant summary.
    pub invariants: Vec<String>,
}

/// Fixture pinning one recovery flow to its expected posture.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReactiveRecoveryFixture {
    /// Stable record kind.
    pub record_kind: String,
    /// Schema version.
    pub schema_version: u32,
    /// Stable fixture id.
    pub fixture_id: String,
    /// Recovery flow under test.
    pub expected_flow_id: String,
    /// Consumer surface under test.
    pub consumer_surface: ConsumerSurface,
    /// Lag condition under test.
    pub lag_condition: LagCondition,
    /// Expected primary strategy.
    pub expected_primary_strategy: RecoveryStrategy,
    /// Expected epoch posture.
    pub expected_epoch_posture: EpochPosture,
    /// Expected action posture.
    pub expected_action_posture: ActionPosture,
    /// Expected exact-truth-action offering.
    pub expected_offers_exact_truth_action: bool,
    /// One consumer that would quote this scenario.
    pub consumer_ref: String,
    /// Short reviewer note.
    pub notes: String,
}

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
        writeln!(f, "reactive-recovery validation failed")?;
        for violation in &self.violations {
            writeln!(f, "- {}: {}", violation.check_id, violation.message)?;
        }
        Ok(())
    }
}

impl std::error::Error for ValidationReport {}

// One row carries the full flow contract; a builder struct would obscure the
// seed more than the argument list does.
#[allow(clippy::too_many_arguments)]
fn flow(
    flow_id: &str,
    consumer_surface: ConsumerSurface,
    lag_condition: LagCondition,
    primary_strategy: RecoveryStrategy,
    fallback_strategies: Vec<RecoveryStrategy>,
    epoch_posture: EpochPosture,
    action_posture: ActionPosture,
    preserved_context: Vec<PreservedContextClass>,
    recovery_summary: &str,
    truth_posture_rationale: &str,
    source_contract_refs: Vec<&str>,
    consumer_refs: Vec<&str>,
    notes: &str,
) -> RecoveryFlowRow {
    RecoveryFlowRow {
        flow_id: flow_id.to_owned(),
        consumer_surface,
        lag_condition,
        primary_strategy,
        fallback_strategies,
        epoch_posture,
        action_posture,
        offers_exact_truth_action: epoch_posture.is_current()
            && action_posture.allows_exact_truth(),
        silent_retry_allowed: false,
        recovery_cue_visible: true,
        support_exportable: true,
        preserved_context,
        recovery_summary: recovery_summary.to_owned(),
        truth_posture_rationale: truth_posture_rationale.to_owned(),
        source_contract_refs: source_contract_refs
            .into_iter()
            .map(str::to_owned)
            .collect(),
        consumer_refs: consumer_refs.into_iter().map(str::to_owned).collect(),
        notes: notes.to_owned(),
    }
}

fn step(
    phase: DrillPhase,
    epoch_posture: EpochPosture,
    action_posture: ActionPosture,
    narration: &str,
) -> DrillStep {
    DrillStep {
        phase,
        epoch_posture,
        action_posture,
        narration: narration.to_owned(),
    }
}

/// Returns the checked-in packet this lane freezes.
pub fn seeded_reactive_recovery_packet() -> ReactiveRecoveryPacket {
    let flows = vec![
        flow(
            "desktop_shell_rapid_invalidation_burst",
            ConsumerSurface::DesktopShell,
            LagCondition::RapidInvalidationBurst,
            RecoveryStrategy::CoalesceDeltas,
            vec![
                RecoveryStrategy::MarkStaleEpoch,
                RecoveryStrategy::RequestFreshSnapshot,
            ],
            EpochPosture::Coalescing,
            ActionPosture::RevalidateBeforeAct,
            vec![
                PreservedContextClass::LayoutSlot,
                PreservedContextClass::LastKnownProjection,
                PreservedContextClass::FreshnessCue,
                PreservedContextClass::InvalidationReason,
            ],
            "Shell strips coalesce the invalidation burst into one consistent frame and revalidate any pending exact-truth action against the producer before it commits.",
            "While coalescing the strip trails the live epoch, so it shows a freshness cue and revalidates instead of asserting the burst is already settled truth.",
            vec![
                "docs/adr/0005-subscription-envelope-and-invalidation-semantics.md",
                "schemas/runtime/subscription_envelope.schema.json",
            ],
            vec![
                "crates/aureline-shell/src/ai_truth_strip/mod.rs",
                "crates/aureline-shell/src/dashboard_truth/mod.rs",
            ],
            "A rapid invalidation burst coalesces in place; the strip stays visible but does not claim the burst already settled.",
        ),
        flow(
            "desktop_shell_backpressure_overflow",
            ConsumerSurface::DesktopShell,
            LagCondition::BackpressureOverflow,
            RecoveryStrategy::RequestFreshSnapshot,
            vec![RecoveryStrategy::Resubscribe],
            EpochPosture::SnapshotRecovering,
            ActionPosture::NarrowedToLastKnown,
            vec![
                PreservedContextClass::LayoutSlot,
                PreservedContextClass::LastKnownProjection,
                PreservedContextClass::FreshnessCue,
                PreservedContextClass::EpochMarker,
            ],
            "When the bounded delta queue overflows the card drops the lossy stream and requests a fresh consistent snapshot, narrowing to last-known read-only until it applies.",
            "Dropped frames break causality, so the card cannot prove current truth; it narrows to last-known and waits for the fresh snapshot rather than guessing.",
            vec![
                "docs/adr/0005-subscription-envelope-and-invalidation-semantics.md",
                "schemas/runtime/subscription_envelope.schema.json",
            ],
            vec![
                "crates/aureline-shell/src/drift_truth/mod.rs",
                "crates/aureline-shell/src/graph_state_card/mod.rs",
            ],
            "Backpressure overflow narrows the card to last-known truth and reloads from a fresh snapshot instead of replaying a lossy queue.",
        ),
        flow(
            "cli_headless_consumer_lag",
            ConsumerSurface::CliHeadless,
            LagCondition::ConsumerLag,
            RecoveryStrategy::CoalesceDeltas,
            vec![
                RecoveryStrategy::RequestFreshSnapshot,
                RecoveryStrategy::MarkStaleEpoch,
            ],
            EpochPosture::Coalescing,
            ActionPosture::RevalidateBeforeAct,
            vec![
                PreservedContextClass::LastKnownProjection,
                PreservedContextClass::ScopeIdentity,
                PreservedContextClass::FreshnessCue,
                PreservedContextClass::EpochMarker,
            ],
            "Headless output coalesces trailing deltas and stamps each emitted record with its epoch and a not-current freshness flag so scripts can revalidate before acting.",
            "Machine-readable output must never imply exact truth while trailing; the freshness flag and epoch stamp let a consumer decide instead of trusting a stale line.",
            vec![
                "docs/adr/0005-subscription-envelope-and-invalidation-semantics.md",
                "schemas/runtime/subscription_envelope.schema.json",
            ],
            vec![
                "crates/aureline-cli/src/stabilize_stable_cli_headless_schemas_machine_readable_output/mod.rs",
            ],
            "CLI lag coalesces and stamps freshness so headless consumers can revalidate rather than trust a trailing line as current.",
        ),
        flow(
            "cli_headless_reconnect_after_drop",
            ConsumerSurface::CliHeadless,
            LagCondition::ReconnectAfterDrop,
            RecoveryStrategy::Resubscribe,
            vec![RecoveryStrategy::RequestFreshSnapshot],
            EpochPosture::ResubscribePending,
            ActionPosture::ResubscribeRequired,
            vec![
                PreservedContextClass::ScopeIdentity,
                PreservedContextClass::FreshnessCue,
                PreservedContextClass::InvalidationReason,
                PreservedContextClass::EpochMarker,
            ],
            "After a dropped watcher the headless lane resubscribes from a new snapshot epoch and surfaces an explicit resubscribe-required state before exact-truth actions resume.",
            "A reconnect cannot silently resume the old epoch; the resubscribe-required state is emitted so the change in action posture is never hidden behind an automatic retry.",
            vec![
                "docs/adr/0005-subscription-envelope-and-invalidation-semantics.md",
                "schemas/runtime/subscription_envelope.schema.json",
            ],
            vec![
                "crates/aureline-cli/src/stabilize_stable_cli_headless_schemas_machine_readable_output/mod.rs",
            ],
            "A dropped subscription resubscribes from a fresh epoch and emits resubscribe-required rather than silently resuming the old stream.",
        ),
        flow(
            "ai_inspector_invalidation_gap",
            ConsumerSurface::AiInspector,
            LagCondition::InvalidationGap,
            RecoveryStrategy::MarkStaleEpoch,
            vec![
                RecoveryStrategy::RequestFreshSnapshot,
                RecoveryStrategy::Resubscribe,
            ],
            EpochPosture::StaleEpoch,
            ActionPosture::NarrowedToLastKnown,
            vec![
                PreservedContextClass::LastKnownProjection,
                PreservedContextClass::ScopeIdentity,
                PreservedContextClass::FreshnessCue,
                PreservedContextClass::InvalidationReason,
                PreservedContextClass::EpochMarker,
            ],
            "When the inspector detects a gap in the invalidation sequence it marks the epoch stale, narrows context to last-known, and requests a fresh snapshot before re-asserting any pin.",
            "A causality gap means the inspector cannot prove its context matches current truth; marking the epoch stale and narrowing keeps AI context honest rather than confidently wrong.",
            vec![
                "docs/adr/0005-subscription-envelope-and-invalidation-semantics.md",
                "schemas/runtime/subscription_envelope.schema.json",
            ],
            vec![
                "crates/aureline-ai/src/context_inspector/mod.rs",
                "crates/aureline-ai/src/stabilize_ai_route_and_spend_truth/mod.rs",
            ],
            "An invalidation gap marks the inspector epoch stale and narrows context instead of presenting a possibly-mismatched pin as current.",
        ),
        flow(
            "review_workspace_reconnect_after_drop",
            ConsumerSurface::ReviewWorkspace,
            LagCondition::ReconnectAfterDrop,
            RecoveryStrategy::Resubscribe,
            vec![RecoveryStrategy::RequestFreshSnapshot],
            EpochPosture::ResubscribePending,
            ActionPosture::ResubscribeRequired,
            vec![
                PreservedContextClass::LayoutSlot,
                PreservedContextClass::ScopeIdentity,
                PreservedContextClass::FreshnessCue,
                PreservedContextClass::EpochMarker,
            ],
            "A review workspace that lost its live link resubscribes to the merge-queue and pipeline streams and shows a resubscribe-required banner before approve or merge actions re-enable.",
            "Approve and merge depend on exact current state; the workspace requires a visible resubscribe before they re-enable so a reconnect never silently restores exact-truth actions.",
            vec![
                "docs/adr/0005-subscription-envelope-and-invalidation-semantics.md",
                "schemas/runtime/subscription_envelope.schema.json",
            ],
            vec![
                "crates/aureline-review/src/workspace/mod.rs",
                "crates/aureline-review/src/stabilize_review_workspace_anchors_stale_base_labels_approval/mod.rs",
            ],
            "Review reconnect requires a visible resubscribe before approve or merge re-enable; no exact-truth action resumes silently.",
        ),
        flow(
            "review_workspace_provider_overlay_disappeared",
            ConsumerSurface::ReviewWorkspace,
            LagCondition::ProviderOverlayDisappeared,
            RecoveryStrategy::MarkStaleEpoch,
            vec![RecoveryStrategy::Resubscribe],
            EpochPosture::StaleEpoch,
            ActionPosture::Blocked,
            vec![
                PreservedContextClass::LayoutSlot,
                PreservedContextClass::LastKnownProjection,
                PreservedContextClass::ScopeIdentity,
                PreservedContextClass::FreshnessCue,
                PreservedContextClass::InvalidationReason,
            ],
            "When the remote preview or pipeline provider overlay disappears the workspace marks the overlaid rows stale and blocks exact-truth actions that depended on the missing provider.",
            "The provider that backed the overlay is gone, so those rows cannot claim current truth; blocking the dependent actions is honest while the workspace waits for the overlay to return.",
            vec![
                "docs/adr/0005-subscription-envelope-and-invalidation-semantics.md",
                "schemas/runtime/subscription_envelope.schema.json",
            ],
            vec![
                "crates/aureline-review/src/workspace/mod.rs",
                "crates/aureline-review/src/stabilize_review_workspace_anchors_stale_base_labels_approval/mod.rs",
            ],
            "A disappeared provider overlay blocks the dependent review actions and keeps the rows marked stale instead of pretending the overlay is still live.",
        ),
        flow(
            "companion_snapshot_provider_overlay_disappeared",
            ConsumerSurface::CompanionSnapshot,
            LagCondition::ProviderOverlayDisappeared,
            RecoveryStrategy::RequestFreshSnapshot,
            vec![
                RecoveryStrategy::Resubscribe,
                RecoveryStrategy::MarkStaleEpoch,
            ],
            EpochPosture::SnapshotRecovering,
            ActionPosture::NarrowedToLastKnown,
            vec![
                PreservedContextClass::LastKnownProjection,
                PreservedContextClass::ScopeIdentity,
                PreservedContextClass::FreshnessCue,
                PreservedContextClass::EpochMarker,
            ],
            "When a followed provider overlay disappears the companion narrows its mirror to last-known and requests a fresh bounded snapshot rather than implying the followed session is still live.",
            "The companion mirror is read-bounded; if the overlay it followed is gone it shows last-known and waits for a fresh snapshot instead of asserting current followed state.",
            vec![
                "docs/adr/0005-subscription-envelope-and-invalidation-semantics.md",
                "schemas/runtime/subscription_envelope.schema.json",
            ],
            vec![
                "crates/aureline-companion/src/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty/mod.rs",
                "crates/aureline-companion/src/ship_managed_sync_maturity_with_snapshot_classes_conflict_review_device_registry_and_end_to_end_encrypted_storage/mod.rs",
            ],
            "A companion that loses its followed overlay narrows to last-known and reloads from a fresh snapshot rather than implying the session is still live.",
        ),
        flow(
            "companion_snapshot_consumer_lag",
            ConsumerSurface::CompanionSnapshot,
            LagCondition::ConsumerLag,
            RecoveryStrategy::CoalesceDeltas,
            vec![RecoveryStrategy::RequestFreshSnapshot],
            EpochPosture::Coalescing,
            ActionPosture::RevalidateBeforeAct,
            vec![
                PreservedContextClass::LastKnownProjection,
                PreservedContextClass::ScopeIdentity,
                PreservedContextClass::FreshnessCue,
                PreservedContextClass::EpochMarker,
            ],
            "A trailing companion snapshot coalesces deltas into its bounded mirror and revalidates any write-back inside scope before it commits while a freshness cue marks the lag.",
            "The companion may trail the live session; coalescing with a visible freshness cue keeps the mirror honest and revalidates scoped writes instead of trusting a trailing frame.",
            vec![
                "docs/adr/0005-subscription-envelope-and-invalidation-semantics.md",
                "schemas/runtime/subscription_envelope.schema.json",
            ],
            vec![
                "crates/aureline-companion/src/ship_session_follow_and_incident_awareness_surfaces_with_bounded_read_write_scope_and_stale_state_honesty/mod.rs",
            ],
            "Companion lag coalesces into the bounded mirror with a freshness cue and revalidates scoped write-back instead of trusting a trailing frame.",
        ),
    ];

    let drills = vec![
        RecoveryDrill {
            drill_id: "drill.reactive_recovery.rapid_invalidation_burst".to_owned(),
            title: "Rapid invalidation burst coalesces without claiming settled truth".to_owned(),
            lag_condition: LagCondition::RapidInvalidationBurst,
            exercised_flow_id: "desktop_shell_rapid_invalidation_burst".to_owned(),
            steps: vec![
                step(
                    DrillPhase::Detect,
                    EpochPosture::Coalescing,
                    ActionPosture::RevalidateBeforeAct,
                    "A burst of invalidations arrives faster than the strip can apply each delta.",
                ),
                step(
                    DrillPhase::NarrowAction,
                    EpochPosture::Coalescing,
                    ActionPosture::RevalidateBeforeAct,
                    "Pending exact-truth actions switch to revalidate-before-act and a freshness cue appears.",
                ),
                step(
                    DrillPhase::Recover,
                    EpochPosture::Coalescing,
                    ActionPosture::RevalidateBeforeAct,
                    "The buffered deltas coalesce into one consistent frame applied in epoch order.",
                ),
                step(
                    DrillPhase::Verify,
                    EpochPosture::Current,
                    ActionPosture::ExactTruthAllowed,
                    "The coalesced frame reaches the live epoch and exact-truth actions re-enable.",
                ),
            ],
            asserts_no_stale_exact_action: true,
            asserts_recovery_visible: true,
            expected_final_epoch_posture: EpochPosture::Current,
            expected_final_action_posture: ActionPosture::ExactTruthAllowed,
            notes: "Coalescing never offered an exact-truth action while behind; it re-enabled them only after reaching the live epoch.".to_owned(),
        },
        RecoveryDrill {
            drill_id: "drill.reactive_recovery.consumer_lag".to_owned(),
            title: "Consumer lag recovers via coalesced catch-up with visible freshness".to_owned(),
            lag_condition: LagCondition::ConsumerLag,
            exercised_flow_id: "cli_headless_consumer_lag".to_owned(),
            steps: vec![
                step(
                    DrillPhase::Detect,
                    EpochPosture::Coalescing,
                    ActionPosture::RevalidateBeforeAct,
                    "Headless output detects it is emitting records behind the live epoch.",
                ),
                step(
                    DrillPhase::NarrowAction,
                    EpochPosture::Coalescing,
                    ActionPosture::RevalidateBeforeAct,
                    "Each emitted record is stamped with its epoch and a not-current freshness flag.",
                ),
                step(
                    DrillPhase::Recover,
                    EpochPosture::Coalescing,
                    ActionPosture::RevalidateBeforeAct,
                    "Trailing deltas coalesce and the lane drains toward the live epoch.",
                ),
                step(
                    DrillPhase::Verify,
                    EpochPosture::Current,
                    ActionPosture::ExactTruthAllowed,
                    "Output reaches the live epoch and clears the not-current freshness flag.",
                ),
            ],
            asserts_no_stale_exact_action: true,
            asserts_recovery_visible: true,
            expected_final_epoch_posture: EpochPosture::Current,
            expected_final_action_posture: ActionPosture::ExactTruthAllowed,
            notes: "Every trailing record carried an epoch stamp and freshness flag, so no consumer could read a lagging line as current truth.".to_owned(),
        },
        RecoveryDrill {
            drill_id: "drill.reactive_recovery.reconnect_after_drop".to_owned(),
            title: "Reconnect requires a visible resubscribe before exact-truth actions resume"
                .to_owned(),
            lag_condition: LagCondition::ReconnectAfterDrop,
            exercised_flow_id: "review_workspace_reconnect_after_drop".to_owned(),
            steps: vec![
                step(
                    DrillPhase::Detect,
                    EpochPosture::ResubscribePending,
                    ActionPosture::ResubscribeRequired,
                    "The review workspace detects its live subscription dropped.",
                ),
                step(
                    DrillPhase::NarrowAction,
                    EpochPosture::ResubscribePending,
                    ActionPosture::ResubscribeRequired,
                    "Approve and merge disable behind a visible resubscribe-required banner.",
                ),
                step(
                    DrillPhase::Recover,
                    EpochPosture::ResubscribePending,
                    ActionPosture::ResubscribeRequired,
                    "The workspace resubscribes to the merge-queue and pipeline streams from a fresh snapshot epoch.",
                ),
                step(
                    DrillPhase::Verify,
                    EpochPosture::Current,
                    ActionPosture::ExactTruthAllowed,
                    "The fresh snapshot applies, the banner clears, and approve or merge re-enable.",
                ),
            ],
            asserts_no_stale_exact_action: true,
            asserts_recovery_visible: true,
            expected_final_epoch_posture: EpochPosture::Current,
            expected_final_action_posture: ActionPosture::ExactTruthAllowed,
            notes: "The resubscribe was never silent; the banner made the changed action posture visible until the fresh epoch applied.".to_owned(),
        },
        RecoveryDrill {
            drill_id: "drill.reactive_recovery.provider_overlay_disappeared".to_owned(),
            title: "Disappeared provider overlay blocks dependent actions without faking truth"
                .to_owned(),
            lag_condition: LagCondition::ProviderOverlayDisappeared,
            exercised_flow_id: "review_workspace_provider_overlay_disappeared".to_owned(),
            steps: vec![
                step(
                    DrillPhase::Detect,
                    EpochPosture::StaleEpoch,
                    ActionPosture::NarrowedToLastKnown,
                    "The remote preview provider overlay stops responding and its rows are flagged.",
                ),
                step(
                    DrillPhase::NarrowAction,
                    EpochPosture::StaleEpoch,
                    ActionPosture::Blocked,
                    "Exact-truth actions that depended on the missing overlay are blocked and the rows are marked stale.",
                ),
                step(
                    DrillPhase::Recover,
                    EpochPosture::StaleEpoch,
                    ActionPosture::Blocked,
                    "The workspace holds the stale marker and keeps trying to resubscribe to the provider.",
                ),
                step(
                    DrillPhase::Verify,
                    EpochPosture::StaleEpoch,
                    ActionPosture::Blocked,
                    "With the provider still gone the rows stay blocked and stale rather than reverting to an exact-truth claim.",
                ),
            ],
            asserts_no_stale_exact_action: true,
            asserts_recovery_visible: true,
            expected_final_epoch_posture: EpochPosture::StaleEpoch,
            expected_final_action_posture: ActionPosture::Blocked,
            notes: "Honest recovery here means staying blocked and stale while the provider is gone; the drill proves the workspace does not pretend nothing changed.".to_owned(),
        },
    ];

    ReactiveRecoveryPacket {
        record_kind: REACTIVE_RECOVERY_PACKET_RECORD_KIND.to_owned(),
        schema_version: REACTIVE_RECOVERY_SCHEMA_VERSION,
        packet_id: "state.reactive_recovery.v1".to_owned(),
        title: "Consumer-side reactive recovery flows for lagging subscription consumers"
            .to_owned(),
        source_contract_refs: SourceContractRefs {
            doc_ref: REACTIVE_RECOVERY_DOC_REF.to_owned(),
            schema_ref: REACTIVE_RECOVERY_SCHEMA_REF.to_owned(),
            packet_ref: REACTIVE_RECOVERY_PACKET_REF.to_owned(),
            report_ref: REACTIVE_RECOVERY_REPORT_REF.to_owned(),
            drills_ref: REACTIVE_RECOVERY_DRILLS_REF.to_owned(),
            fixture_manifest_ref: REACTIVE_RECOVERY_FIXTURE_MANIFEST_REF.to_owned(),
        },
        flows,
        drills,
        invariants: vec![
            "A consumer that is not on the current epoch never offers an action that depends on exact current truth.".to_owned(),
            "Every recovery flow keeps a visible freshness cue and is support-exportable; recovery is never silent.".to_owned(),
            "A materially changed action posture is never hidden behind an automatic silent retry.".to_owned(),
            "Each lagging surface coalesces, resubscribes, requests fresh snapshots, or marks the epoch stale from one shared vocabulary instead of a private cache.".to_owned(),
            "When a provider overlay disappears the dependent rows stay blocked or narrowed rather than reverting to an exact-truth claim.".to_owned(),
        ],
    }
}

/// Returns the checked-in fixtures this lane freezes.
pub fn seeded_reactive_recovery_fixtures() -> Vec<ReactiveRecoveryFixture> {
    seeded_reactive_recovery_packet()
        .flows
        .iter()
        .map(|row| ReactiveRecoveryFixture {
            record_kind: REACTIVE_RECOVERY_FIXTURE_RECORD_KIND.to_owned(),
            schema_version: REACTIVE_RECOVERY_SCHEMA_VERSION,
            fixture_id: format!("fixture.reactive_recovery.{}", row.flow_id),
            expected_flow_id: row.flow_id.clone(),
            consumer_surface: row.consumer_surface,
            lag_condition: row.lag_condition,
            expected_primary_strategy: row.primary_strategy,
            expected_epoch_posture: row.epoch_posture,
            expected_action_posture: row.action_posture,
            expected_offers_exact_truth_action: row.offers_exact_truth_action,
            consumer_ref: row.consumer_refs.first().cloned().unwrap_or_default(),
            notes: row.notes.clone(),
        })
        .collect()
}

/// Validates the checked-in packet contract.
pub fn validate_reactive_recovery_packet(
    packet: &ReactiveRecoveryPacket,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if packet.record_kind != REACTIVE_RECOVERY_PACKET_RECORD_KIND {
        report.push(
            "packet.record_kind",
            "packet record_kind does not match the frozen token",
        );
    }
    if packet.schema_version != REACTIVE_RECOVERY_SCHEMA_VERSION {
        report.push("packet.schema_version", "packet schema_version must be 1");
    }
    if packet.source_contract_refs.doc_ref != REACTIVE_RECOVERY_DOC_REF {
        report.push("packet.doc_ref", "doc_ref drifted from the frozen doc");
    }
    if packet.source_contract_refs.schema_ref != REACTIVE_RECOVERY_SCHEMA_REF {
        report.push(
            "packet.schema_ref",
            "schema_ref drifted from the frozen JSON schema",
        );
    }
    if packet.source_contract_refs.packet_ref != REACTIVE_RECOVERY_PACKET_REF {
        report.push(
            "packet.packet_ref",
            "packet_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.report_ref != REACTIVE_RECOVERY_REPORT_REF {
        report.push(
            "packet.report_ref",
            "report_ref drifted from the frozen artifact",
        );
    }
    if packet.source_contract_refs.drills_ref != REACTIVE_RECOVERY_DRILLS_REF {
        report.push(
            "packet.drills_ref",
            "drills_ref drifted from the frozen drill artifact",
        );
    }
    if packet.source_contract_refs.fixture_manifest_ref != REACTIVE_RECOVERY_FIXTURE_MANIFEST_REF {
        report.push(
            "packet.fixture_manifest_ref",
            "fixture_manifest_ref drifted from the frozen manifest",
        );
    }
    if packet.invariants.is_empty() {
        report.push("packet.invariants", "packet must declare invariants");
    }

    let mut flow_ids = BTreeSet::new();
    let mut covered_surfaces = BTreeSet::new();
    let mut covered_conditions = BTreeSet::new();
    let mut covered_primary_strategies = BTreeSet::new();

    for row in &packet.flows {
        if !flow_ids.insert(row.flow_id.as_str()) {
            report.push(
                "flow.id_unique",
                format!("duplicate flow_id {}", row.flow_id),
            );
        }
        validate_flow_row(&mut report, row);

        covered_surfaces.insert(row.consumer_surface);
        covered_conditions.insert(row.lag_condition);
        covered_primary_strategies.insert(row.primary_strategy);
    }

    for required in [
        ConsumerSurface::DesktopShell,
        ConsumerSurface::CliHeadless,
        ConsumerSurface::AiInspector,
        ConsumerSurface::ReviewWorkspace,
        ConsumerSurface::CompanionSnapshot,
    ] {
        if !covered_surfaces.contains(&required) {
            report.push(
                "packet.covered_surface",
                format!("packet must cover consumer surface {}", required.as_str()),
            );
        }
    }
    for required in [
        LagCondition::RapidInvalidationBurst,
        LagCondition::ConsumerLag,
        LagCondition::BackpressureOverflow,
        LagCondition::InvalidationGap,
        LagCondition::ReconnectAfterDrop,
        LagCondition::ProviderOverlayDisappeared,
    ] {
        if !covered_conditions.contains(&required) {
            report.push(
                "packet.covered_condition",
                format!("packet must cover lag condition {}", required.as_str()),
            );
        }
    }
    for required in [
        RecoveryStrategy::CoalesceDeltas,
        RecoveryStrategy::RequestFreshSnapshot,
        RecoveryStrategy::Resubscribe,
        RecoveryStrategy::MarkStaleEpoch,
    ] {
        if !covered_primary_strategies.contains(&required) {
            report.push(
                "packet.covered_strategy",
                format!(
                    "packet must use recovery strategy {} as a primary at least once",
                    required.as_str()
                ),
            );
        }
    }

    validate_drills(&mut report, packet, &flow_ids);

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

fn validate_flow_row(report: &mut ValidationReport, row: &RecoveryFlowRow) {
    if row.fallback_strategies.is_empty() {
        report.push(
            "flow.fallback_strategies",
            format!("flow {} must declare at least one fallback", row.flow_id),
        );
    }
    if row
        .fallback_strategies
        .iter()
        .any(|strategy| *strategy == row.primary_strategy)
    {
        report.push(
            "flow.fallback_strategies",
            format!(
                "flow {} fallback strategies must not repeat the primary",
                row.flow_id
            ),
        );
    }
    if row.preserved_context.is_empty() {
        report.push(
            "flow.preserved_context",
            format!("flow {} must preserve context", row.flow_id),
        );
    }
    if !row.recovery_cue_visible {
        report.push(
            "flow.recovery_cue_visible",
            format!("flow {} must keep its recovery cue visible", row.flow_id),
        );
    }
    if !row.support_exportable {
        report.push(
            "flow.support_exportable",
            format!("flow {} must remain support-exportable", row.flow_id),
        );
    }
    if row.recovery_summary.trim().is_empty() {
        report.push(
            "flow.recovery_summary",
            format!("flow {} must explain how it recovers", row.flow_id),
        );
    }
    if row.truth_posture_rationale.trim().is_empty() {
        report.push(
            "flow.truth_posture_rationale",
            format!(
                "flow {} must explain why its truth posture is honest",
                row.flow_id
            ),
        );
    }
    if row.source_contract_refs.is_empty() {
        report.push(
            "flow.source_contract_refs",
            format!("flow {} must cite source contract refs", row.flow_id),
        );
    }
    if row.consumer_refs.is_empty() {
        report.push(
            "flow.consumer_refs",
            format!("flow {} must cite at least one consumer ref", row.flow_id),
        );
    }

    // The central invariant: exact-truth actions are offered only on the live
    // epoch, and a stale or recovering epoch never claims exact truth.
    let expects_exact = row.epoch_posture.is_current() && row.action_posture.allows_exact_truth();
    if row.offers_exact_truth_action != expects_exact {
        report.push(
            "flow.exact_truth_gate",
            format!(
                "flow {} may offer exact-truth actions only on the current epoch with an exact-truth action posture",
                row.flow_id
            ),
        );
    }
    if !row.epoch_posture.is_current() && row.action_posture.allows_exact_truth() {
        report.push(
            "flow.no_stale_exact_action",
            format!(
                "flow {} must not keep an exact-truth action posture on a {} epoch",
                row.flow_id,
                row.epoch_posture.as_str()
            ),
        );
    }
    // Guardrail: silent retry only when the action posture did not change.
    if row.silent_retry_allowed && !row.action_posture.allows_exact_truth() {
        report.push(
            "flow.silent_retry_guardrail",
            format!(
                "flow {} must not allow a silent retry while the action posture is {}",
                row.flow_id,
                row.action_posture.as_str()
            ),
        );
    }

    // Strategy and epoch posture must agree so the catch-up path is legible.
    let expected_epoch = match row.primary_strategy {
        RecoveryStrategy::CoalesceDeltas => EpochPosture::Coalescing,
        RecoveryStrategy::RequestFreshSnapshot => EpochPosture::SnapshotRecovering,
        RecoveryStrategy::Resubscribe => EpochPosture::ResubscribePending,
        RecoveryStrategy::MarkStaleEpoch => EpochPosture::StaleEpoch,
    };
    if row.epoch_posture != expected_epoch {
        report.push(
            "flow.strategy_epoch_agreement",
            format!(
                "flow {} primary strategy {} expects epoch posture {} but found {}",
                row.flow_id,
                row.primary_strategy.as_str(),
                expected_epoch.as_str(),
                row.epoch_posture.as_str()
            ),
        );
    }
}

fn validate_drills(
    report: &mut ValidationReport,
    packet: &ReactiveRecoveryPacket,
    flow_ids: &BTreeSet<&str>,
) {
    if packet.drills.is_empty() {
        report.push("packet.drills", "packet must declare recovery drills");
    }

    let mut drill_ids = BTreeSet::new();
    let mut covered_conditions = BTreeSet::new();
    let flows_by_id: BTreeMap<_, _> = packet
        .flows
        .iter()
        .map(|row| (row.flow_id.as_str(), row))
        .collect();

    for drill in &packet.drills {
        if !drill_ids.insert(drill.drill_id.as_str()) {
            report.push(
                "drill.id_unique",
                format!("duplicate drill_id {}", drill.drill_id),
            );
        }
        if !flow_ids.contains(drill.exercised_flow_id.as_str()) {
            report.push(
                "drill.exercised_flow_id",
                format!(
                    "drill {} references unknown flow {}",
                    drill.drill_id, drill.exercised_flow_id
                ),
            );
        } else if let Some(flow_row) = flows_by_id.get(drill.exercised_flow_id.as_str()) {
            if flow_row.lag_condition != drill.lag_condition {
                report.push(
                    "drill.flow_condition_match",
                    format!(
                        "drill {} lag condition does not match flow {}",
                        drill.drill_id, drill.exercised_flow_id
                    ),
                );
            }
        }
        if !drill.asserts_no_stale_exact_action {
            report.push(
                "drill.asserts_no_stale_exact_action",
                format!(
                    "drill {} must assert no stale exact-truth action",
                    drill.drill_id
                ),
            );
        }
        if !drill.asserts_recovery_visible {
            report.push(
                "drill.asserts_recovery_visible",
                format!(
                    "drill {} must assert the recovery is visible",
                    drill.drill_id
                ),
            );
        }

        if drill.steps.is_empty() {
            report.push(
                "drill.steps",
                format!("drill {} must declare steps", drill.drill_id),
            );
            continue;
        }
        if drill.steps.first().map(|s| s.phase) != Some(DrillPhase::Detect) {
            report.push(
                "drill.first_phase",
                format!("drill {} must begin with a detect step", drill.drill_id),
            );
        }
        let last = drill.steps.last().expect("non-empty drill steps");
        if last.phase != DrillPhase::Verify {
            report.push(
                "drill.last_phase",
                format!("drill {} must end with a verify step", drill.drill_id),
            );
        }
        if drill.expected_final_epoch_posture != last.epoch_posture
            || drill.expected_final_action_posture != last.action_posture
        {
            report.push(
                "drill.final_posture",
                format!(
                    "drill {} expected-final posture must match its verify step",
                    drill.drill_id
                ),
            );
        }
        let has_narrow_action = drill
            .steps
            .iter()
            .any(|s| s.phase == DrillPhase::NarrowAction && !s.action_posture.allows_exact_truth());
        if !has_narrow_action {
            report.push(
                "drill.narrow_action_step",
                format!(
                    "drill {} must narrow exact-truth actions before recovering",
                    drill.drill_id
                ),
            );
        }
        for (index, drill_step) in drill.steps.iter().enumerate() {
            if drill_step.action_posture.allows_exact_truth()
                && !drill_step.epoch_posture.is_current()
            {
                report.push(
                    "drill.step_no_stale_exact_action",
                    format!(
                        "drill {} step {} claims exact truth on a {} epoch",
                        drill.drill_id,
                        index,
                        drill_step.epoch_posture.as_str()
                    ),
                );
            }
            if drill_step.narration.trim().is_empty() {
                report.push(
                    "drill.step_narration",
                    format!(
                        "drill {} step {} must narrate the step",
                        drill.drill_id, index
                    ),
                );
            }
        }

        covered_conditions.insert(drill.lag_condition);
    }

    // The four scenarios the recovery lane must drill explicitly.
    for required in [
        LagCondition::RapidInvalidationBurst,
        LagCondition::ConsumerLag,
        LagCondition::ReconnectAfterDrop,
        LagCondition::ProviderOverlayDisappeared,
    ] {
        if !covered_conditions.contains(&required) {
            report.push(
                "packet.drilled_condition",
                format!("packet must drill lag condition {}", required.as_str()),
            );
        }
    }
}

/// Validates one checked-in fixture against the frozen packet.
pub fn validate_reactive_recovery_fixture(
    packet: &ReactiveRecoveryPacket,
    fixture: &ReactiveRecoveryFixture,
) -> Result<(), ValidationReport> {
    let mut report = ValidationReport {
        violations: Vec::new(),
    };

    if fixture.record_kind != REACTIVE_RECOVERY_FIXTURE_RECORD_KIND {
        report.push(
            "fixture.record_kind",
            "fixture record_kind does not match the frozen token",
        );
    }
    if fixture.schema_version != REACTIVE_RECOVERY_SCHEMA_VERSION {
        report.push("fixture.schema_version", "fixture schema_version must be 1");
    }

    let rows: BTreeMap<_, _> = packet
        .flows
        .iter()
        .map(|row| (row.flow_id.as_str(), row))
        .collect();
    let row = match rows.get(fixture.expected_flow_id.as_str()) {
        Some(row) => *row,
        None => {
            report.push(
                "fixture.expected_flow_id",
                format!("fixture {} references an unknown flow", fixture.fixture_id),
            );
            return Err(report);
        }
    };

    if row.consumer_surface != fixture.consumer_surface {
        report.push(
            "fixture.consumer_surface",
            format!(
                "fixture {} drifted from flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }
    if row.lag_condition != fixture.lag_condition {
        report.push(
            "fixture.lag_condition",
            format!(
                "fixture {} drifted from flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }
    if row.primary_strategy != fixture.expected_primary_strategy {
        report.push(
            "fixture.primary_strategy",
            format!(
                "fixture {} drifted from flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }
    if row.epoch_posture != fixture.expected_epoch_posture {
        report.push(
            "fixture.epoch_posture",
            format!(
                "fixture {} drifted from flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }
    if row.action_posture != fixture.expected_action_posture {
        report.push(
            "fixture.action_posture",
            format!(
                "fixture {} drifted from flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }
    if row.offers_exact_truth_action != fixture.expected_offers_exact_truth_action {
        report.push(
            "fixture.offers_exact_truth_action",
            format!(
                "fixture {} drifted from flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }
    if !row
        .consumer_refs
        .iter()
        .any(|reference| reference == &fixture.consumer_ref)
    {
        report.push(
            "fixture.consumer_ref",
            format!(
                "fixture {} cites a consumer_ref not declared by flow {}",
                fixture.fixture_id, row.flow_id
            ),
        );
    }

    if report.is_empty() {
        Ok(())
    } else {
        Err(report)
    }
}

#[cfg(test)]
mod tests;
